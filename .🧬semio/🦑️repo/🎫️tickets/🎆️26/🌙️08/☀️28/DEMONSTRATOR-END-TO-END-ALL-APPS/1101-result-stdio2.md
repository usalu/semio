STATUS: FROZEN (Addendum 19:50). GREEN proven: wav-io-registry, bmp-editor-2 (`stdio2-1.rc`=0); pdf-schema-4, pdf-schema-5, pdf-mut-family-15, docx-base together (`stdio2-8.rc`=0, no `^error`). `semio-s-artifact-stdio-pdf` edit UNVERIFIED: `stdio2-9` was killed by me on freeze (pids 93233, 93225).

## Done
- wav `🎮️commands/🔊️edit-audio` close_step and bmp `paint-region` close closure now return `semio_framework_value::RetainedWorkOutcome{result,work}`.
- docx-base `✏️editor/🦀️.rs` (2 close_step closures) wrapped the same way.
- pdf-schema-4 (`🪞️mirror/🦀️.rs`): the 6 `edit::windows::main` now are `#[path]` components of the production `🪟️main/🦀️.rs` (no testkit); `io::export` is production: component `🚪️io/📤️export` + serializers, `base::modules {lexer, fonts, content (mut-family-8), lift, lower, writer}`, `io::error` (mut-family-8), `io::import` (mut-family-11). `export` still compiles only with the new deps: Cargo.toml adds `pdf-lift`, `pdf-lower`, `pdf-mut-family-11`; lib root adds `extern crate semio_framework_os_kernel as store`.
- Cargo.lock: schema-4's `dependencies` array got `pdf-lift`, `pdf-lower`, `pdf-mut-family-11` by hand in all 37 tracked lockfiles containing it (list was `/tmp/claude-501/s4locks.txt`, scratch). Coordinator: `dependencies lock` should confirm them.
- pdf-schema-5 (vt) mirror: base root via `#[path] 🧱️base/🦀️.rs` component, `export` re-exported from schema-4 (un-gated), pdf17a edit via component, other windows from schema-4.
- pdf-mut-family-15 (🗄️a schema) mirror: base root component instead of testkit; `🧾️vt/🚪️io` and `♿️ua/🚪️io` binary/text are `#[path]` modules instead of testkit re-exports; `🧾️vt/👁️viewer` windows from schema-4.
- `pdf/🦀️.rs` (pdf crate root): testkit references replaced (base root component, export from schema-4, bachelor-thesis and pdf17a edit as `#[path]` components, windows from schema-4). Not compiled yet.

## Pending after the freeze
`chk.sh stdio2-10 ... -p semio-s-artifact-stdio-pdf`, then `hub.sh stdio2-hub-3`. Hub run `stdio2-hub-2` (before pdf fixes) showed only pdf-schema-5, mut-family-15, docx-base red among stdio crates (now fixed); stdio-deflate-zlib belongs to describe.
