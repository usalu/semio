# Procedural Integration Audit — 2026-09-29

Read-only source inspection; no builds, tests, browser sessions or Git mutations were performed by this lane. Only this report was written. Other lanes own validation and repairs.

## Confirmed Findings

1. **P1: Scalar/vector/text inspector edits bind the wrong lifecycle trigger.** `generation3d/.../editor/📌️panels/🔍️inspection/🦀️.rs`, `editable_input`, builds inputs with `commit("blur")` and a `Trigger::Change` binding. The actual React path is `treeItemToTreeData` → `renderTreeItemControls` → `UiNodeView` → `InputView`, in `framework/.../renderer/.../elements/🗣️Interpreter/🟦️.tsx`. `InputView` line 1324 dispatches `commit` when the authored mode is blur. `UiDocumentStore/🟦️.tsx:566` resolves the exact trigger and emits nothing without that binding. These edits therefore never reach the retained command. Bind `Trigger::Commit` for those inputs and keep checkbox `Trigger::Change`. Root was notified and is repairing this. Existing renderer tests at `Interpreter/🧪️tests/🪪️container-node-ids/🟦️.tsx:113` and `Interpreter/🧪️tests/🪟️tree-windows/🟦️.tsx:383` demonstrate commit bindings; the new inspector tests should assert the actual authored trigger and dispatch an edit through the real renderer path.

2. **Progress/cancellation is currently kernel-only for graph bevel and decimation.** `flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs:233` calls `mesh.bevel_edges`; line 259 calls `mesh.decimate`. Framework kernel lines 770–771 and 1416–1417 delegate to callback variants with unconditional `|_| true`. `Operator::evaluate` takes only the input dictionary. Thus the graph call cannot observe cancellation or report callback progress even though the kernel exposes and tests these capabilities. An eventual graph contract must carry execution context to these callback variants; kernel tests alone cannot establish user-facing cancellation. This is an integration limitation, not a claim that no framework job can cancel between operators.

3. **New graph operations do not extend the retained viewport component command.** `generation3d/.../editor/🎮️commands/🥽️edit-mesh-selection/🦀️.rs:29` accepts only extrude, inset, subdivide, flip, deleteFaces, moveVertices and loopCut; its granularity selector and parameter construction remain restricted to that list. New bevel/dissolve/merge/proportional/snap/mirror/decimate widgets are available through graph catalogue construction, not directly through that selection command. Do not describe these as all integrated into viewport component editing without extending its command/schema/UI/tests.

## Withdrawn Early Suspicions

The exported legacy `renderUiControl` helper renders blur inputs without a local draft and toggles with `{pressed}`, but **new inspector tree children do not use that helper**. The actual `InputView` already maintains a draft, and `ToggleView` emits a scalar boolean via `dispatchTrigger`; accessibility labels are carried there. The initial read-only-input and missing-boolean-value alerts based on `renderUiControl` were retracted after tracing the call chain. They are not defects attributed to this task.

## Positive Source Evidence and Verification Limits

`set-widget-input` validates widget existence, input availability, collection cardinality, compatible literal schema, finite numeric values and incoming connections before editing the temporary host. It emits document mutations through `commit_host_snapshot` and supports optional gesture coalescing. The command tests include publication, undo and redo, but this audit did not run them. Checkbox events on the actual renderer path are compatible with the action mapper’s scalar value conversion. Labels on new controls are authored and read by standard InputView/ToggleView.

Mesh graph errors are propagated using `map_err(mesh_error)` rather than discarded. Kernel report documents convex/closed bevel limits, one-sided mirror assumptions and approximate decimation honestly. No browser/runtime confirmation, full feature-completeness assertion or new passing-test assertion is made here.
