# Layer Stack Editing

Added Bring Forward and Send Backward to the selection command, its schema, the action form, and the English/German inspector. Selected sibling layers move one stack position while preserving both selected and unselected relative order. Contiguous selections at the front/back boundary stay unchanged. Front/back extremes now share the same stable planner and return no mutations when the requested order already holds.

The implementation uses existing semantic reorder mutations. Planning uses adjacent swaps for one-step movement and calculated source positions for extremes, avoiding repeated full-array splices during planning. Rust and TypeScript implementations consume the same eleven neutral cases. The input schema was updated before implementation; the initial test reproduced the missing planner. Immer independently applies the returned moves and checks the handwritten expected order, selection stability and source immutability.

Native tests exercise the real command and mutation application at root and inside a group. Inspector tests check both localized labels and semantic action payloads. Registered app tests cover both directions as one undo/redo history entry. These native additions are authored but not yet verified.

## Evidence

- `tests-layer-stack-red.txt`: missing implementation reproduced, exit 1.
- `tests-layer-stack-current.txt`: **189 tests / 19,139 assertions / 26 files passed**, exit 0, plus 48 field-patch cases and the 36-command publication audit.
- Final run 43880 after the planning optimization passed **189 tests / 19,139 assertions / 26 files**, exit 0 (`tests-layer-stack-final.txt`), including the 48 field-patch and 36-command audits.
- Existing native 88159 (`tests-native-svg-gradients.txt`) and component build 93519 (`build-draw-inspector-rows.txt`) are live. The native run began before these sources; confirm coverage before treating its eventual result as current evidence.

Browser verification remains pending. Ungrouping, complete import/image workflows, large-document cancellation, native renderer parity and the rest of the full acceptance list remain unfinished. Goal and ticket remain active.

## Changed Sources

All paths below are relative to `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`:

- `🎮️commands/🎛️edit-selection/🧬️schema/🔣️.json`
- `🎮️commands/🎛️edit-selection/🗂️stack/🦀️.rs`, `🟦️.ts`, and `🧫️fixtures/🔣️.json`
- `🎮️commands/🎛️edit-selection/🦀️.rs` and its Rust/TypeScript unit tests
- `🗣️terminology/🦀️.rs`
- `📌️panels/🔍️properties/🦀️.rs` and its selection tests
- `🦀️.rs` and `🧪️tests/🔬️unit/🦀️.rs`
