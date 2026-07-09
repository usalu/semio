# PUZZLE-3D-REACT-PARITY verify log

## Root cause (fill/brush broken in browser)

1. **Concurrent wasm plugin calls** — `refreshUi` used `Promise.all` for `render`/`tools`/`windowEngagements` while `registerBrushMesh`/`setHover` commands were in flight → `RefCell already borrowed` at `framework/plugin/rs/lib.rs:1281` → wasm abort poisoned the instance.

2. **WASI P2 + wasm-bindgen mismatch** — `puzzle/3d/rs` used `#[cfg(target_arch = "wasm32")]` for `js_sys::Date::now()` and `#[wasm_bindgen]` exports. The puzzle **plugin component** (`wasm32-wasip2`) hit `cannot call wasm-bindgen imported functions on non-wasm targets` during Concrete Forest precompute → abort before fill/brush could run.

## Fixes

| Layer                                   | Change                                                                                                |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `framework/core/js/index.ts`            | `withSerializedPluginWasmHandle`, per-module handle cache, busy retry                                 |
| `framework/product/os/dev/script.ts`    | Generated bridge: module-level `runSerialized`, `createPluginApi` singleton                           |
| `framework/plugin/rs/lib.rs`            | `InstanceGuard` — reject re-entrant instance access with `plugin instance busy` instead of panicking  |
| `framework/renderer/react/os-shell.tsx` | Sequential `refreshUi` wasm reads                                                                     |
| `puzzle/3d/rs/lib.rs`                   | `target_env = "p2"` cfgs: native precompute session for component wasm; js-sys only for web wasm-pack |
| `puzzle/plugin/rs/d3/mod.rs`            | `setActiveTool` drives precompute for brush/fill                                                      |

## Verification (2026-07-09)

- `bun nx run @semio-tech/framework-renderer-react:test` — 27 passed
- `.repo/🎫/26/07/09/PUZZLE-3D-REACT-PARITY/wasm-verify.ts` — fill round-trip value 4
- Headless browser (playwright): Concrete Forest load **0** panics; fill engagement shows slider (`data-control-kind="slider"`)
- Puzzle wasm rebuilt: `cargo build -p puzzle-plugin --target wasm32-wasip2 --release` + jco transpile + `script.ts build puzzle`

## Dev notes

- Hard-refresh `http://127.0.0.1:6013/` after wasm rebuild (Vite does not always hot-swap `.wasm`).
- Run puzzle-3d dev in isolation (`dev:puzzle:3d` only) to avoid wasm-pack races.
