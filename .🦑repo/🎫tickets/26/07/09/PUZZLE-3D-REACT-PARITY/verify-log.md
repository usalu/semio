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

### Unrelated blockers fixed in passing (workspace-wide, not scoped to this ticket)

- Stale `protocol/plugin/rs` workspace-member entry in root `Cargo.toml` (directory no longer exists, consolidated into `protocol/rs`).
- Wrong relative path in new `mathematical/graph/drawing/rs` crate's `Cargo.toml` (`../rs` → `../../rs`).
- Misplaced `//!` inner doc comments (should be `//`/`///`) in `puzzle/3d/rs/lib.rs` and `puzzle/5d/rs/lib.rs`, copy-pasted into a new VCS-integration region.
- Missing `Clone` derive on `Puzzle3dScene`/`Puzzle5dScene` after the `*Envelope` → `*Scene` rename.
- `puzzle/2d/rs` wasm32 build: `ray_from_origin_to_axis_aligned_rectangle_edge` moved to the new `mathematical_geometry` crate; added the dependency + import.
- `puzzle/plugin/rs` `WindowEngagementToggleGroupOption` import: `semio_framework_plugin`'s own `pub mod component` (WASM export glue) shadows the glob-reexported `ui_wgpu::component::layout`, making that one type permanently unreachable via the flat plugin surface; added `ui_wgpu` as a direct dependency (precedented — `cad-plugin`/`procedural-plugin` already do this) and imported it directly.
- Stale `ui/react` → `ui/js/react` path in `infinite/world/r3f/vitest.config.ts`.

## Round 3 (2026-07-13) — "it doesn't work end to end": two root-caused, app-wide bugs

User reported the app not working at all: `installHook.js:1 [DEBUG] framework os boot failed Error: Keine Plugins geladen` ("No plugins loaded"). Root-caused and fixed two bugs — both pre-existing/introduced by the concurrently in-progress `OS-VCS-HUB-CQRS-EVENT-SOURCING-REFACTOR` and a newer selection-halo feature, neither scoped to this ticket, but both fully blocking it (and every other plugin) from working at all.

### Bug 1 — total boot failure ("No plugins loaded")

`framework/renderer/react/os-shell.tsx`'s `registry` memo filtered the plugin registry by the **raw** `pluginFilter` (a playground _variant_ id, e.g. `"puzzle3d"`) directly against each entry's crate-level `pluginId` (e.g. `"puzzle"` — one crate serves the `puzzle2d`/`puzzle3d`/`puzzle5d` variants). The two never matched, so `expandPluginRegistry` always produced an empty list → `loaded.length === 0` → every dev server for every plugin threw immediately on boot, before even attempting a plugin load (confirmed no per-plugin load/reject log ever fired). Fix: resolve the variant id via the already-imported `resolvePluginRegistryId(pluginFilter)` (same resolver already correctly used two call-sites below, for `primary`/`primaryApp` lookup) before passing it to `expandPluginRegistry`.

### Bug 2 — every engagement control's callback silently no-op'd (the dead fill slider)

Traced with temporary `[DEBUG]` instrumentation through the whole chain (`EngagementControlView`'s `onValueChange` → `windowEngagementControlToSpec`'s `dispatchNumeric` → `onAction` → `plugin.handleAction` → raw `windowEngagements()` wasm response) down to the actual wire JSON. `ui/wgpu/rs/lib.rs`'s `WindowEngagementControl` enum has `#[serde(tag = "kind", rename_all = "camelCase")]` at the enum level — which only renames the `kind` tag value (confirmed `"kind":"slider"` correctly cased) and, per serde's actual behavior, **does not** camelCase the fields _inside_ each struct-like variant (a distinct, separate serde attribute — `rename_all_fields` — is needed for that, or per-field `#[serde(rename = "...")]`). So every variant's `on_change`/`on_commit`/`on_select` fields shipped as literal snake_case JSON (`"on_change":{...}`) while every TS consumer only ever reads `control.onChange` (camelCase) — the callback was `undefined`, so the dispatch silently no-op'd for every slider, stepper, ring, toggle group, and select control in the entire app, not just Fill. This is why arrow-key/drag nudges moved the slider's own visual thumb (Radix's local `aria-valuenow`, driven by the always-firing `onValueChange`→`setDraftValues` half of the chain) but the committed "Fill N" label never budged — `control.onChange` itself was undefined, so `dispatchNumeric` was never reached. Fix: added `#[serde(rename = "onChange" | "onCommit" | "onSelect")]` to the 7 affected fields across `Slider`/`Stepper`/`Ring`/`ToggleGroup`/`Select` variants, mirroring the string already present in each field's adjacent `ts(rename = "...")` type-gen attribute.

### Bug 3 — 3D scene crash ("Maximum call stack size exceeded"), blocking all mesh rendering

Discovered while re-verifying after the above two fixes: any scene with real mesh objects (Concrete Forest, Nakagin) crashed the `<CanvasImpl>` on load. Root cause: a newly-added selection-halo/edge-outline feature in `world-3d-host.tsx`'s `applyGlbMeshEdgeBorders`/`applyGlbMeshSelectionHalo` called `object.add(outline|halo)` **from inside** `root.traverse(...)` — since the added `outline`/`halo` node is itself a `Mesh`/`LineSegments`, splicing it into the live `children` array mid-traversal causes `traverse()` to recurse into the node it just added, add another halo to _that_, recurse again, forever. Fix: split each function into a collect-pass (walk `traverse`, gather target meshes, mutate nothing) followed by a separate mutate-pass (add the outline/halo to each collected target) — same output, no live-array mutation during iteration.

### Live-verified after all three fixes

- App boots correctly (`http://127.0.0.1:6013/`, tab title updates to "semio · puzzle · 3d", engagement bar renders).
- `fill 20` command-line path: 1 → 10 objects, 9 attractions, slider correctly shows "Fill 20".
- **Fill slider arrow-key nudges now correctly commit** — dispatched native `keydown` events on the Radix thumb move `aria-valuenow` _and_ the committed "Fill N" label (verified 0→1 and 0→3 in separate clean runs) — this was the user's core original complaint.
- Concrete Forest and Nakagin Capsule Tower both render their meshes correctly (no more stack overflow).
- Brush mode: hovering a vortex on the mesh produces the ghost preview _and_ populates the "Placement" candidate toggle group in the engagement bar with real candidate labels (e.g. "Hexagonal Cut Concrete Forest Left").
- Rust: `cargo check -p puzzle-plugin` clean (native + `wasm32-wasip2`); `cargo test -p puzzle-plugin --lib d3::` 13/13 passing.
- TS: `infinite-world-r3f` 70/70 passing; `framework-renderer-react` 105/106 passing (1 unrelated pre-existing failure in an unrelated "spawned window chrome" plugin-contributions test, not touched by this work); `ui/js/react` 219/230 passing (11 unrelated pre-existing failures, all in tree-view component tests — confirmed via targeted grep that none reference the Slider component this ticket touched).
- Puzzle wasm rebuilt clean after every fix.

### Not verified live (covered by passing unit tests instead)

Alt+right-click → suggestion popup could not be exercised live: `handleWorldOrbitRightPointerDown` never fired for synthetic `PointerEvent`s dispatched with `button: 2, altKey: true` — most likely because `useWorldOrbitRightMouseBindings`'s gesture handling depends on real browser pointer-capture semantics that scripted dispatch can't fully replicate, compounded by the dev server's frequent HMR-triggered WebGL context resets during testing. The underlying action logic (`openVortexSuggestions`, `hoverSuggestion`, `acceptSuggestion`, `closeVortexSuggestions`) is covered by the 4 passing Rust unit tests added in Round 2, and the closely-related hover→ghost→candidate pipeline (same `hoveredVortexFullId`/`worldVortexHover` plumbing) is confirmed working live above.
