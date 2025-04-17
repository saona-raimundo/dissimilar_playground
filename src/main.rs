#![forbid(unsafe_code)]

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    // Input
    let (old_text, set_old_text) = signal(String::new());
    let (new_text, set_new_text) = signal(String::new());
    //
    // // TODO: Show text lines
    // // TRY: HTML editor instead of textarea elements
    // // - https://codemirror.net/
    // // - https://ace.c9.io/
    //
    // Output
    let output = view! {
        <table>
            <tbody>
                {move || {
                    let old = old_text.get();
                    let new = new_text.get();
                    let diff = dissimilar::diff(&old, &new);
                    diff.into_iter()
                        .map(|chunk| {
                            let (sign, color, text) = match chunk {
                                dissimilar::Chunk::Equal(s) => ("=", "gray", s),
                                dissimilar::Chunk::Delete(s) => ("-", "red", s),
                                dissimilar::Chunk::Insert(s) => ("+", "green", s),
                            };
                            let marker = match sign {
                                "=" => " border-top:medium dashed black; width: 100%",
                                _ => "",
                            };
                            // Computing
                            view! {
                                <tr>
                                    <td style=format!("color:{color}")>{format!("{sign}")}</td>
                                    <td style=format!(
                                        "color:{color}; white-space:pre-wrap;{}",
                                        marker,
                                    )>{format!("{text}")}</td>
                                </tr>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </tbody>
        </table>
        <hr />
    };

    view! {
        <h1>"Diffing text"</h1>
        <div>
            <h2>"Input"</h2>
            <div class="flex-container" style="display:flex; flex-wrap: wrap;">
                //
                // // TODO: Show line numbers in the input
                //
                <textarea
                    autofocus
                    rows="10"
                    style="flex: 1000;"
                    placeholder="Old text."
                    on:input=move |ev| {
                        set_old_text.set(event_target_value(&ev));
                    }
                    prop:value=old_text
                >
                    {move || old_text.get()}
                </textarea>
                <textarea
                    rows="10"
                    style="flex: 1000;"
                    placeholder="New text."
                    on:input=move |ev| {
                        set_new_text.set(event_target_value(&ev));
                    }
                    prop:value=new_text
                >
                    {move || new_text.get()}
                </textarea>
            </div>
        </div>
        <div>
            <h2>"Output"</h2>
            <div>{output}</div>
        </div>

        // Description
        <footer>
            <h2>"About"</h2>

            <h3>"Diffing implementation"</h3>
            <p>
                <a href="https://github.com/dtolnay/dissimilar">"Dissimilar"</a>
                " is a port of the Diff component of "
                <a href="https://github.com/google/diff-match-patch">"Diff Match Patch"</a>
                " to Rust. The diff implementation is based on "
                <a href="https://neil.fraser.name/writing/diff/myers.pdf">
                    "Myers' diff algorithm"
                </a>
                " but includes some "
                <a href="https://neil.fraser.name/writing/diff/">"semantic cleanups"</a>
                " to increase human readability by factoring out commonalities which are likely to be coincidental."
            </p>

            <h3>"This playground"</h3>
            <p>
                "Source code: "
                <a href="https://github.com/saona-raimundo/dissimilar_playground">"GitHub"</a>
            </p>
            <p>
                "Lisence: "
                <a rel="lisence" href="https://creativecommons.org/publicdomain/zero/1.0/">
                    <img
                        alt="Creative Commons Licence"
                        style="border-width:0"
                        src="https://i.creativecommons.org/l/by/4.0/80x15.png"
                    />
                </a> <a rel="lisence" href="https://creativecommons.org/publicdomain/zero/1.0/">
                    {"CC0 1.0 Universal"}
                </a>
            </p>
            <address>
                {"Author: 🧑🏼‍💻"}
                <a href="href=https://saona-raimundo.github.io/">{"Raimundo Saona"}</a>
            </address>
        </footer>
    }
}
