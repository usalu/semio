# Verify log — empty canvas + duplicate example keys fix

## Root cause

- `Puzzle2dBoardHost` called `session.attachCanvas()` but `@semio-tech/puzzle-2d-rs` exports `attach_canvas` (snake_case wasm-bindgen).

## Fixes

1. `puzzle-2d-board-host.tsx`: `attach_canvas`
2. `os-shell.tsx`: corrected `Puzzle2dBoardWasmSession` type
3. `ExampleDefinition.app_id` + `register_app` tags examples per app; os-shell filters + dedupes; wgpu shell filters by app
4. Rebuilt puzzle plugin WASM

## Validation

- `cargo check` core/plugin/wgpu — ok
- vitest 23 passed
- Dev server `http://127.0.0.1:6012/` manual:
  - 3 puzzle2d-board canvases mount (1099×1159, 545×1159, 545×1159)
  - No `attachCanvas is not a function` after reload
  - Example dropdown: unique entries only (Empty, Concrete Forest, Nakagin)
  - Concrete Forest: all three panes render geometry (overview + detail + selection LOD)

## Smooth zoom reset follow-up

### Root cause

- `puzzle2d_pane_camera` recomputed the overview camera from fixture bounds and ignored the persisted zoom, so the board could briefly zoom and reset on the next scene sync.
- Asset-backed boards also triggered scene refreshes while `wheel_screen` updated the WASM camera silently, leaving React without a persisted camera update to protect the live zoom from stale scene state.

### Fix

- `puzzle/plugin/rs/d2/mod.rs`: overview now renders the persisted `fixture.camera` literally, while detail and selection keep their derived triptych framing.
- Added `apply_board_events_camera_round_trips_to_overview_scene` to cover the event -> document -> rendered overview camera loop.
- `framework/renderer/react/components/puzzle-2d-board-host.tsx`: wheel handling now dispatches `setCamera` from the live WASM `session.cameraJson()` before draining board events.

### Validation

- `bun ./script.ts test --run index.test.ts` in `framework/renderer/react`: 1 file passed, 25 tests passed.
- `cargo check -p puzzle-plugin --target wasm32-wasip2`: passed with existing warnings.
- `cargo test -p puzzle-plugin apply_board_events_camera_round_trips_to_overview_scene -- --exact`: blocked by existing native `component_export_anchor` macro target error before the test can run.
