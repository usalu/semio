# Path Node Marquee Selection

Dragging from empty canvas space in node editing mode now enters the existing marquee preview state. It selects anchors on the currently selected editable paths without changing layer selection. Rectangle edges are inclusive, and world transforms include rotation, scale, and shear. Handles remain directly pickable; marquee membership is based on anchors.

Replace, Shift-toggle, and Ctrl/Meta-add use a deduplicated region merge. The previous point selection remains intact throughout the drag and changes only on successful release. A cancelled release or Escape clears the gesture without replacing the previous selection. An empty click clears points in replace mode and preserves them in modified modes.

The retained point query now has a node-area traversal. It hashes one segment per work item and publishes one captured anchor identifier per work item, preserving geometry-bound point references without a whole-path hash at the end. The existing work-per-step and result/byte capacities bound each query. Invalid geometry or result overflow prevents selection publication. The query continues to skip hidden and locked ancestors.

The synchronous geometry-id helper now delegates to the same segment hash primitive, preserving the existing hash format. The TracePointerJob no longer derives unused Clone/Debug, allowing it to own an incremental first-party hasher.

## Verification

- RED 94569 exited 1 after adding fixture-driven marquee coverage before implementation.
- GREEN 54316 exited 0: 275 passed, one skipped, 148733 assertions across 28 files. Ajv validates the neutral fixture. Independent Three.js Matrix3 and Box2 computations confirm transformed anchor membership. The field audit passes 69 cases and the publication audit passes 37 commands.
- Added native tests for neutral rectangle/merge fixtures and equivalence of incremental and synchronous geometry hashes.
- Added a native retained-query test over 5000 anchors that checks the per-step work bound and exact captured point identifiers.
- Added a native gesture test that starts an empty-space marquee without changing point selection, then cancels it.
- Added an app journey covering replace, toggle, add, and cancellation; it checks unchanged document content, unchanged layer selection, unchanged point selection during dragging, and exact released membership.
- Native tests have not yet been observed executing. Handles 17529 and 93519 were polled and confirmed live after the changes. Their terminal source coverage must still be established. Browser marquee/keyboard acceptance awaits current component activation.
- Diff whitespace check passed for the changed picker and point-selection helper files.

Logs: `🗑️generated/tests-node-marquee-red.txt`, `🗑️generated/tests-node-marquee.txt`. The skipped PDF oracle still awaits native PDF output. Native run 17529 predates the latest features; do not assume it covers them without inspecting its terminal roster.

## Changed Files

Relative to `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/`:

- `✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs`
- `✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🎯️points/🦀️.rs`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🎯️points/🟦️.ts`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🎯️points/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🎯️points/🧫️fixtures/▧️marquee/🔣️.json`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🎯️points/🧫️fixtures/▧️marquee/🧬️schema/🔣️.json`

This note and the acceptance ledger also changed. The goal remains active. Lasso node selection, full native/browser acceptance, large deletion jobs, rendering/export fidelity, typography, complete import, and other end-user requirements remain unfinished.
