# Owned Context Paths and Test Documentation

Native604's Norm test compilation exposed that `ArtifactOwnedToolJobContext` is public under `semio_framework_plugin::app`, but is not re-exported at the crate root. Corrected the newly added Norm and Space reducer parameters to use that public module path. The type and callback position are unchanged.

The same run reported an unused documentation comment attached to the OS crate's `include!` invocation. Moved the explanation onto the actual included path-mount test, where rustdoc can attach it to a function. The test body is unchanged.

Changed files:

- `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🔬️standalone/🦀️.rs`

Parser and fresh compiler verification are pending.

Pass 619 parsed both context-path corrections, both documentation locations and the two framework test-import fixes with Rust 2021. All six files remained unchanged during parsing. Ticket TypeScript and the 339-law inventory also passed validation. Compiler rechecks remain pending.
