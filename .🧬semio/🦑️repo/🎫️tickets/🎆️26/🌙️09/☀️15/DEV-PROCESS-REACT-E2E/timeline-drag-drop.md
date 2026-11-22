# Process3d Timeline Drag and Drop

The Steps section publishes a drag handle per materialized step. The `application/x-semio-tree-row-reorder` payload carries the step ID and the live store envelope ID from the mounted artifact view. Stock and section rows carry no timeline drag payload. The tree retains its existing selection and the inspector retains its index-based move controls.

The mounted React interpreter uses native row drag events, displays a before/after marker, and dispatches the tree's authored action through its surface-addressed intent with the target row and position. The mounted WGPU event router resolves the row's authored drop binding and sends the same action arguments with the position sampled at release. Both use the top and bottom quarters as insertion edges.

The retained `dropStep` command checks the source document against its admitted operation. It resolves both IDs in the current step list and delegates the index-based move to `moveStep`, producing the existing invertible `ReorderSteps` mutation on the artifact lane. Missing rows, self drops, centre drops, cross-document drops and duplicates produce no mutation.

The JSON Schema and neutral fixture cover both move directions, both insertion edges, the last position, missing entries and no-op cases. The Rust test calls the host-action bridge, retained command reducer and mutation preparation. The TypeScript test checks the same fixture with AJV and an independent Lodash implementation. The artifact projection test verifies the tree and row bindings and the exact drag payload.

The repository's light Bun command passed both editor TypeScript tests. The mounted React interpreter suite passed all 203 tests, including the canonical target ID and release position check. Scoped `git diff --check` passed, and a filesystem count found 17 touched Rust owners with a maximum of 494 lines. The JSON encoder is already owned by the Process3d editor kit, so dependency manifests and locks are unchanged. Native compilation was reserved for the parent task's isolated scope.

## Renderer compatibility audit

BIM and Norm publish row-level drop bindings and their roots carry no drop binding. Norm combines a JSON `id` with the row's authored `path` and `over` for `moveEntity`. BIM carries a raw element ID and authors `field` and `value` on the receiving storey row's `setField`. Position-aware handling is now reserved for the shared reorder MIME. React requires that MIME and a tree-level Drop binding. WGPU requires it on both source and target and keeps the existing argument merge for other row and stack drops. Both focused React tests passed after the change. The WGPU stack boundary has regression assertions awaiting native execution.

## WGPU insertion feedback

The existing shell drag overlay now paints a 2 px accent line inside the top or bottom edge of the presented target row, clipped to its retained ancestors. The overlay reads the active retained drag, and release uses the same pointer-to-edge function. The centre, drag exit, cancellation and release clear the marker. The native `retained_tree_reorder_tests::retained_tree_reorder_marker_follows_the_physical_pointer_and_clears` test in `semio-renderer-wgpu-shell-tests-input-2` exercises real pointer input and inspects the overlay and committed action. Native execution is assigned to the main verification run.

The exact repository source census measured these owners:

| Owner | Files | Active lines | Bytes |
| --- | ---: | ---: | ---: |
| Shell input tests 1, baseline | 24 | 2,661 | 272,810 |
| Shell input tests 1, after partition | 21 | 2,570 | 259,239 |
| Shell input tests 2, after partition | 13 | 2,379 | 183,651 |
| Production shell view panels | 13 | 1,985 | 114,837 |
| UI WGPU engine | 8 | 2,989 | 180,388 |
| Renderer UI engine adapter | 8 | 2,455 | 150,598 |

The input-1 closure exceeded its 262,144-byte limit by 10,666 bytes before the marker work. The two locale-label tests and the accepted dock-tab focus and activation test moved intact to input-2. The immediate-select wheel test remains in input-1. Both test suites now meet the closure limits.
