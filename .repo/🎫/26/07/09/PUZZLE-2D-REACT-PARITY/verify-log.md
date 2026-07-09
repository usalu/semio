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
- The React `Puzzle2dBoardHost` correctly drained wheel camera events into `applyBoardEvents`.
- `puzzle2d_pane_camera` then recomputed the overview camera from fixture bounds and ignored the persisted wheel zoom, so the board briefly zoomed and reset on the next scene sync.

### Fix
- `puzzle/plugin/rs/d2/mod.rs`: overview now renders the persisted `fixture.camera` literally, while detail and selection keep their derived triptych framing.
- Added `apply_board_events_camera_round_trips_to_overview_scene` to cover the event -> document -> rendered overview camera loop.

### Validation
- `bun ./script.ts test --run index.test.ts` in `framework/renderer/react`: 1 file passed, 24 tests passed.
- `cargo check -p puzzle-plugin --target wasm32-wasip2`: passed with existing warnings.
- `cargo test -p puzzle-plugin apply_board_events_camera_round_trips_to_overview_scene -- --exact`: blocked by existing native `component_export_anchor` macro target error before the test can run.
