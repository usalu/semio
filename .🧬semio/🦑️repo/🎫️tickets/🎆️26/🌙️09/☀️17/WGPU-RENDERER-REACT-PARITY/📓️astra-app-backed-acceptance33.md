# App-Backed Renderer Acceptance

The next gate verifies real app actions and accepted visible consequences for each registered scene family. Generic structural parity treats component scenes as visual leaves, so it cannot establish internal interaction parity.

## Current Work Split

- Astra: current native/WASM integration, then paired physical Puzzle3d window and control checks at the fresh artifact.
- Sol Select: Draw Canvas2d and Note InkCanvas gesture/cancellation contracts, app receipts, and React/WGPU acceptance paths.
- Sol Tree: finish accepted-caret verification, then Flow NodeGraph selection and persisted movement using its real controller `s.flow.flow@1/*#editor`.
- Terra: read-only audit of production contracts, exact existing targets, fixture/activation availability, and acceptance evidence.

## Cancellation Policy Decision

The reference is actual React app behavior. A generic cancellation fixture must not override production semantics shared by React and WGPU.

Canvas2d Draw commits at gesture completion. Its cancelled pointer-up receipt carries `cancelled: true`, and the unfinished draft must produce no document mutation.

InkCanvas Note accepts and persists begin/live events during the gesture. Both current renderers do this. Pointer cancellation clears the active local gesture and preview; already accepted ink remains. The acceptance law must require that later stale move/end events cannot continue publishing from the cancelled gesture.

This packet therefore corrects the inaccurate generic Ink expectation with an explicit per-surface terminal policy. It does not introduce a new Note rollback protocol or change the React reference's accepted-event semantics. Such a rollback would be a separate product behavior change.

## Validation Boundary

The source-level disagreement was verified by Terra against React InkCanvasHost, WGPU Scenes, and Note's production handler and tests. This report is a coordination decision, not a successful live acceptance result.

Full renderer native run 4 has been launched without a test filter, so it includes the new deadline, animation, tutorial, BlockList and caret-adjacent laws. Full UI run 4, scoped native Cargo check 5, and WASM build 13 are also in progress. No current full pass is claimed.

## September 27 Integration Gate Receipts

All four jobs have now exited with code 1. WASM build 13 compiled the renderer successfully in 9m36s, then Nx output failed with ENOSPC before successful artifact publication was confirmed. Native shape check 5 finished one Cargo pass in 13m28s, then its subsequent dependency execution ran out of disk while writing Puzzle metadata. Neither complete command is a pass.

Full UI run 4 stopped before assertions on two caret fixture include paths. Sol Tree corrected both from `../../../` to `../../` and verified the fixture files resolve. Full native renderer run 4 stopped before assertions on `details::Labels.locale`; direct current-source inspection confirms another concurrent editor has already supplied that field and both constructor values, so no duplicate edit was made.

The disk reported 2.1 GiB available. This ticket’s generated runtime data is only about 14 MiB; deleting its receipts cannot resolve compiler-cache pressure. A read-only ownership audit is running before any cleanup. Shared active caches and other tickets remain untouched.

The fresh actual React Flow scoped-domain oracle passed all five tests in 8.49s after using a temporary directory within the ticket. Sol Tree additionally strengthened the Flow app movement law to read the published `NodeGraphScene.hostSnapshotJson` and require the requested persisted position. This still needs its native execution and physical WGPU verification.

Sol Select confirmed that no Note rollback edits remain. The Canvas/Ink packet preserves the per-surface cancellation semantics above.
