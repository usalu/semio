# Window Owner Factory Export

The completed wasm1040 check found seven dead-code diagnostics within one factory implementation. Fresh source inspection found the concrete factory and three external calls. Expose the existing factory through the app and crate public API so those consumers can resolve it. No implementation or lint policy changed.

Verified consumers:
- ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🦀️.rs
- ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🦀️.rs
- ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🫧️transient/🦀️.rs

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
