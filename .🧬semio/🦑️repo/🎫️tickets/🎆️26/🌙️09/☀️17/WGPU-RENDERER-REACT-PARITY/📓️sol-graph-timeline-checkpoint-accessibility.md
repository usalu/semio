# GraphTimeline Checkpoint Keyboard and Accepted Accessibility

## Shared Contract

The existing schema-first `GraphTimelineHost/🎯️checkpoint-hit` contract now covers the checkpoint's stable host key, accessible name, button role, and exact `checkoutCheckpoint` descriptor in addition to its six pointer regions. The selectable control is the labels-and-graph shell. The description cell is a sibling outside that control and remains inert.

The accessible name is the visible checkpoint labels joined in their authored order, falling back to the stable checkpoint id when no label exists. The action remains `{ controllerId, action: "checkoutCheckpoint", args: { checkpointId } }`; it does not gain a surface id.

## React Oracle

The actual `HistoryTable` shell now exposes `role="button"`, `tabIndex={0}`, and an explicit accessible name only when `onSelectCheckpoint` exists. Click, Enter, and Space call the same callback with the checkpoint id. Enter and Space prevent their native/default propagation. The description sibling has no role, tab stop, or handler.

The mounted `GraphTimelineHost` test validates the schema, all six physical regions, the button's role/name/tab stop, two exact keyboard activations, and description inertness. The browser accessibility mirror publishes the same virtual button and sends one fully addressed `accessibility-activate` event containing the accepted window generation, node id, and stable key.

## Accepted WGPU Presentation

`GraphTimelineAccessibilityControl` carries the stable `{host}.history.{checkpointId}` key, checkpoint id, accessible name, vertically clipped labels-and-graph rectangle, and exact pointer action. `render_graph_timeline` derives it from the same shared selectable-width helper, row pitch, scroll authority, and surface bounds as paint. Rows outside the viewport and empty rectangles are omitted.

Controls are staged with the candidate frame, then sealed, acknowledged, or discarded with the shell's presentation epoch. The interpreter publishes accepted controls only for visible, nonretiring GraphTimeline scenes as focusable, tabbable, actionable virtual buttons. Activation requires the current window generation, node hash and key, retained GraphTimeline kind and host, accepted control, live checkpoint id, unchanged label, and unchanged action. A removed checkpoint or accepted empty successor makes the old address inert.

## Validation Receipts

Root's pre-accessibility pointer partition run passed the schema plus six actual React hit cases: 1 file and 7 tests, Vitest 115.03 seconds and Nx 2 minutes 25 seconds.

The first accessibility extension run passed the schema, six physical cases, and mounted keyboard law:

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/sol-graph-timeline/tmp' bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --excludeTaskDependencies -- --run '../../../../🧱️elements/🌳️GraphTimelineHost/🧪️tests/🎯️checkpoint-hit/🟦️.tsx' --silent=false --reporter=verbose
```

Result: 1 file passed, 8 tests passed; Vitest 69.78 seconds, Nx 1 minute 24 seconds.

After adding the addressed browser mirror law, the same command passed 1 file and 9 tests; Vitest 70.58 seconds, Nx 1 minute 20 seconds. Both runs emitted only the existing multiple-Three.js-instance warning.

The focused WGPU Scenes and Interpreter laws are source-complete. The Scenes law verifies fixture-derived selectable geometry, description exclusion, exact descriptor, accepted activation, and live-checkpoint refusal. The Interpreter law verifies the published virtual button, wrong generation/key refusal, exact activation, and stale successor refusal. Their Nx execution was queued behind the shared Rust compiler lane; no passing native result is claimed until that command terminates.

Two setup-only attempts did not execute a test: the first supplied a relative `TMPDIR`, which nextest could not use after changing its working directory; the second supplied libtest's `--exact`, which the repository's nextest wrapper rejects. The active command uses an absolute ticket-local `TMPDIR` and the supported positional name filter:

```sh
CARGO_BUILD_JOBS=2 NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/sol-graph-timeline/tmp' bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit --skip-nx-cache --excludeTaskDependencies -- accepted_checkpoint_accessibility_uses_selectable_geometry_and_exact_action
```

## Current Limits

The browser receipt uses the real DOM mirror. Native assistive-technology publication is covered by the retained accessibility projection law rather than external screen-reader automation. Root owns the full native, WASM, and paired-shell runtime gates.
