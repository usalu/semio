# Closure check: demonstrator, sourcing, procedural components

Packages: `semio-hub-demonstrator-component`, `semio-hub-sourcing-component`, `semio-hub-procedural-component` (release, wasm32-wasip2, `check --lib`).

## Causes found
- Procedural (17:24): not a Rust error. `Cargo.lock` refresh refused: `semio-s-artifact-puzzle-2d-board-host` manifest inherited `semio-s-artifact-stdio-dwg`, absent from the puzzle-2d workspace. Already gone from that manifest (mtime 17:30) by another agent.
- Sourcing (17:23): `semio-os-dsl` E0599 `close_work_demand` on `RetainedRecordReceiving`. The method now exists (`pack/value/decode/receiving`, `pub fn close_work_demand`); the 18:2x run passed `semio-os-dsl` and reached `semio-hub-sourcing-component`.
- 19:11 cycle `pdf-page-editor-authoring -> schema-4/tests-2`: not mine; manifest already rewritten at 19:21 (deps now document, io-export, mut-family-8, snapshot-2, no cycle).

## Gate blockers met (not source errors)
- `hostUnderPressureV1` missing from host-capacity decision module (re-added by another agent ~18:14).
- 19:24+: `ENOENT .../🔗️derivative/🔏️index/🧊️stamps`: index `🟦️.ts` imports `./🧊️stamps/🟦️.ts` which does not exist yet (another agent's in-flight split).

## Status
- Not verified clean. The last full run (1h6m) compiled all dependencies up to `semio-hub-sourcing-component`, but its tail was truncated before the final verdict (exit 101, `[DEBUG] views body done`).

## Update after stamps module appeared
- Run 1 (13 min): real errors. `semio-s-artifact-procedural-2d-viewer` E0432 `semio_plugin::WindowLayoutRoot/StackNode/WindowNode` (and 4 errors in `semio-s-artifact-puzzle-3d-editor-modes`). Cause: another agent's `WindowLayoutNode` migration (old Root/Axis/Stack/Window union replaced by recursive `WindowLayoutNode`, `Axis::{Horizontal,Vertical}`, `active_window_id`).
- Fixed by me: `layout()` in `🌀️procedural/…/🧊️generation3d/…/👁️viewer/🎭️modes/👁️view/🦀️.rs` (now `WindowLayoutNode::Stack{…Window{…}}`). The other viewer/editor `layout()` files (procedural 2d, puzzle 2d/3d, sourcing curation) were migrated concurrently by the other agent. Remaining old-API users in the closure: only test files (puzzle 3d/5d editor unit tests).
- Run 2 (23 min): exit 101, errors were in the truncated middle of the output; visible tail clean.
- Run 3 (`--message-format=short`, 54 min): aborted by the gate, `Insufficient physical compiler initial headroom: 3.0 GiB free of 4.0 GiB` after 1812 s.
- Status: NOT verified clean for any of the three packages.
