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

## Round 2 (2026-07-13) — suggestions, context menu, Alt+right-click, fill progress

Added: per-vortex brush-candidate suggestion popup (`openVortexSuggestions`/`closeVortexSuggestions`/`hoverSuggestion`/`acceptSuggestion`), select-then-open context menu for any entity kind (`contextMenuAt`), Alt+right-click → suggestion popup instead of orbit, `fillBuildTick` auto-start-at-1, fill slider draft-while-sliding fix, fill build progress label, brush ghost color parity, target-volume hide/lock flag toggle.

### Verified
- `cargo check -p puzzle-plugin` — clean (native and `wasm32-wasip2`).
- `cargo test -p puzzle-plugin --lib d3::` — **13 passed** (7 new: `context_menu_at_selects_vortex_and_prepends_suggest_objects`, `context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden`, `open_vortex_suggestions_opens_the_suggestion_popup`, `close_vortex_suggestions_clears_the_menu`, `accept_suggestion_appends_an_object_and_closes_the_menu`, `fill_build_tick_auto_starts_fill_when_active_and_done`, `fill_count_control_shows_building_progress_while_precompute_incomplete`).
- `bunx vitest run` in `framework/renderer/react` — **105 passed** (incl. new `resolveWorldContextMenuTarget` priority/null test).
- `bunx vitest run` in `infinite/world/r3f` — **70 passed** (incl. new `shouldAssignWorldOrbitRightMouse` test).
- Puzzle wasm rebuilt clean: `SEMIO_PLUGIN=puzzle bun ./script.ts plugin` → `puzzle_plugin_component.core.wasm` produced.

### NOT verified — live browser check blocked
Live interaction verification (Alt+right-click popup, brush ghost cycling, context-menu select-then-open, fill slider drag + auto-count-1) could **not** be completed: `http://127.0.0.1:6013/` (and independently, the unrelated `lowpoly` dev server on `:6078`) both hang on a blank `#root` after `bootFrameworkOs()` — no console errors, no render, no plugin-load network activity — even after 60s+ waits and a `node_modules/.vite/deps` cache clear. Confirming a *different, unrelated* plugin fails identically rules out a regression in this ticket's changes; this is a repo-wide dev-environment breakage from the concurrently in-progress `OS-VCS-HUB-CQRS-EVENT-SOURCING-REFACTOR` ticket (large `PluginApp`/`DocumentApp` trait redesign + VCS backbone wiring touching `framework/core`, `framework/plugin`, `framework/product/os`, `vcs` all session). Recommend re-running the live checklist once that refactor lands and dev servers boot again.

### Unrelated blockers fixed in passing (workspace-wide, not scoped to this ticket)
- Stale `protocol/plugin/rs` workspace-member entry in root `Cargo.toml` (directory no longer exists, consolidated into `protocol/rs`).
- Wrong relative path in new `mathematical/graph/drawing/rs` crate's `Cargo.toml` (`../rs` → `../../rs`).
- Misplaced `//!` inner doc comments (should be `//`/`///`) in `puzzle/3d/rs/lib.rs` and `puzzle/5d/rs/lib.rs`, copy-pasted into a new VCS-integration region.
- Missing `Clone` derive on `Puzzle3dScene`/`Puzzle5dScene` after the `*Envelope` → `*Scene` rename.
- `puzzle/2d/rs` wasm32 build: `ray_from_origin_to_axis_aligned_rectangle_edge` moved to the new `mathematical_geometry` crate; added the dependency + import.
- `puzzle/plugin/rs` `WindowEngagementToggleGroupOption` import: `semio_framework_plugin`'s own `pub mod component` (WASM export glue) shadows the glob-reexported `ui_wgpu::component::layout`, making that one type permanently unreachable via the flat plugin surface; added `ui_wgpu` as a direct dependency (precedented — `cad-plugin`/`procedural-plugin` already do this) and imported it directly.
- Stale `ui/react` → `ui/js/react` path in `infinite/world/r3f/vitest.config.ts`.
