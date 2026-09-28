# Keyboard Selection Deletion

Delete and Backspace now target a registered, localized `deleteSelection` input action. In node editing mode, the action removes selected anchors or handles across selected paths. In layer selection/move modes, it removes selected layers, with a selected ancestor owning descendant deletion once. Drawing tools do not delete layers in response to this action.

The schema-first `deletePoints` geometry operation has Rust and TypeScript implementations. All references address the original geometry, avoiding index shifts when several anchors are removed. Surviving contours begin with a move; a closed contour retains its close only when at least two anchors survive. Empty contours disappear. Removing every anchor deletes the empty path layer. Removing a cubic handle collapses it to its associated anchor; removing a quadratic control converts the segment to a line. Styles and transforms remain unchanged.

The editor validates every selected path against the geometry identifier captured by point selection, validates targets and inherited locks/visibility, prepares all changes before publishing, commits one mutation group, and clears obsolete point references. Layer selection is preserved for surviving paths. A stale or unavailable selected target rejects the whole operation. The bounded planner refuses work beyond 4096 selection/traversal items; a resumable large-document deletion workflow remains needed.

## Verification

- RED session 91244 exited 1. The shared geometry test failed with Missing path node on the first new deletePoints fixture because the operation was not implemented.
- GREEN session 59617 exited 0: 274 passed, one skipped, 148721 assertions across 28 files. The language-neutral corpus adds eleven deletion cases: multiple anchors, first anchors, entire contour, all anchors, cubic handles, quadratic handle, duplicates, no selection, missing node, absent handle, and close-marker rejection.
- Ajv validates the new operation against its schema. Independent Immer and Three.js comparisons verify the cubic-handle result and straight-line length. Existing shared fixture tests check expected geometry, immutable sources, and reversal laws in both languages; the Rust test execution remains pending.
- The independent publication audit now passes 37 commands, including deleteSelection; the field audit passes 69 cases.
- Native planner tests were added for multi-path edits and inverse restoration, empty-path removal, stale/locked/hidden/missing targets, ancestor normalization, and tool context.
- An app-level test was added to verify a single artifact publication, point-selection clearing, retained path selection, and exact undo/redo. The command also participates in the existing typed command coverage roster.
- Native session 17529 and component session 93519 remain live after these edits. These new native tests have not yet been observed executing. Current browser key dispatch, focus behavior, and user-visible undo remain unverified until the component is built and activated.

Logs: `🗑️generated/tests-node-delete-red.txt`, `🗑️generated/tests-node-delete.txt`. The skipped test is still the native PDF raster oracle awaiting PDFs.

## Changed Files

Relative to `✏️s/🔌️plugins/🖍️draw/`:

- `🧫️fixtures/🧪️publication-authority/🔣️.json`
- `🗿️artifacts/🖍️drawing/🦀️.rs`
- `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⌫️delete-selection/🦀️.rs`
- `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⌫️delete-selection/🧪️tests/🔬️unit/🦀️.rs`
- `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/✏️editing/🧬️schema/🔣️.json`
- `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/✏️editing/🧫️fixtures/🔣️.json`
- `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/✏️editing/🦀️.rs`
- `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/✏️editing/🟦️.ts`
- `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/✏️editing/🧪️tests/🔬️unit/🟦️.ts`

This note and the acceptance ledger also changed. The full editor goal remains active; this increment does not establish feature completeness.
