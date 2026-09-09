# Puzzle Layout Test Integration

Puzzle 2D unit tests now import their utility action constant directly. Layout fixture tests use the existing OS Infinite engine exports after their move from the graph module. The existing directed-layout and edge-snap implementations remain the test subjects. Added the affected fixture selectors to the ticket runtime catalog.

Rust and TypeScript syntax checks passed. Runtime execution is pending the active full compiler pass.

Changed files:
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/📜️script.ts

Puzzle 2D/5D context-menu and engagement tests now supply the current view-model parameter. Puzzle 5D imports its utility action constant directly in the test leaf. Registered the affected UI test selectors. Removed the compiler-reported unused Serde import from the Remodeling schema root; descendant schemas retain their own imports. Syntax checks passed for three Rust files and the runtime catalog. Runtime execution pending.

Corrected selector extraction for the custom async test attributes: runtime coverage targets the five actual context-menu, engagement, and utility-switch tests whose calls changed, with no incidental static-test selectors.
