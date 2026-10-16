# 🎞️ Result RENDERER-WP2: EngineCanvas, Scenes, IconRenderHost and Canvas2dHost carved into 14 crates; icon-host reads its icons from the runtime asset port

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Plan: `PLAN-RENDERER-WGPU-CRATES.md` (6.2 WP2 crates, 5.2 item moves, 10.4 hand-off contract). Tools: `rwp2-tools/` (see section 9). Logs: `🗑️generated/rwp2-*`. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `T` = `E/🎯️targets/🧊️wgpu`, `EL` = `E/🧱️elements`.

## 0. Verdict

All 14 production crates of WP2 are physically carved: `engine-registry`, `engine-text`, `engine-board`, `engine-graph`, `engine-attach`, `scene-state`, `canvas2d-host`, `scene-canvas2d`, `scene-ink`, `scene-feeds`, `scene-tables`, `scene-text-vfs`, `icon-host`, `scene-render`. Each has a library root in its `T/<folder>` that mounts the source files with `#[path]` (the files stay in the element trees, the pets/canvas precedent; every file has exactly one owning crate, `verify_static.py`), explicit manifests, and the old parents `EngineCanvas/🦀️.rs` and `Scenes/🦀️.rs` are thin facades that re-export the crates' names. The hub `🎞️wp2-scenes` now says `extern crate … as canvas2d_gumball;` and `extern crate … as icon_atlas;`; `engine_canvas` and `scenes` stay mounted as facade modules (10.4.4 item 3).

**IconRenderHost no longer embeds anything**: the atlas reads the 250 catalog SVGs from the icon pack that every host installs through Canvas's runtime asset port; the generated `🧩️icon-embeds` file, its generator output root and its mount are retired. `icon-host` is **10 source files / 108,494 B / 2,469 lines** (was 259 files in the monolith closure), well inside the gate.

Nothing was compiled by cargo. The launcher runs now (`update --workspace --offline` succeeded, the lock moved) but `check -p …-engine-registry` is refused before rustc by the oversized frontier (`Rust crate Source boundary exceeded before compiler byte allocation`), the same as every crate above `os-kernel`. Evidence is the union differential (section 6), the stub harness for the new atlas code, census, definition scans and static gates.

## 1. The icon-host decision (coordinator: no raw SVG embeds)

* **Before**: `IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs` mounted `🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs` (250 `include_str!`, `ICON_SVGS`), 259 files.
* **Now**: `IconAtlasRebuild::new` takes the sorted catalog ids of `semio_framework_canvas::icon_pack()` once and rasterises `pack.plain(id)` in the same order, then the logo (`SEMIO_LOGO_SVG`, 682 B, still `include_str!`). `icon_atlas_source_count()` is `plain_len() + 1`. Without an installed pack only the logo is rasterised (the host already reports "icons will not render").
* **Port extended** (the channel carried the sources, only an ordered accessor was missing): `IconPack::plain_ids()` (byte order, `C/🔌️runtime-assets/🦀️.rs`) and the facade `semio-framework-canvas` re-exports `icon_pack` and `IconPack` (`C/🦀️.rs`). Tests: `the_catalog_ids_come_in_byte_order_whatever_the_document_order_was` and an ascending-ids assertion in `installed_assets_serve_every_consumer…`.
* **Retired**: `renderRust` no longer emits `🧩️icon-embeds/🦀️.rs` (`🔣️icons/🏗️builder/📽️projection/🟦️.ts`), `assetOutputRoots` and the assets `📋️project.json` output lost the root, the generated file and its folder are deleted. `bun canvas-carve-tools/assets-check.ts check` -> `[assets] fresh 262` (was 263). `RESULT-ICON-RENDER-GENERATED.md` is history.
* **Data proof** (python, `🗑️generated`-free): the pack's `plain` map equals the former `ICON_SVGS` exactly: 250 ids, same byte order, 0 byte mismatches against the 250 source files, `semio-logo` not a catalog id.
* **Test** (`IconRenderHost/🧪️tests/🔬️wgpu-unit/🦀️.rs`): `a_catalog_icon_is_rasterised_into_the_atlas_from_the_installed_pack` installs the repository assets through the port (`RuntimeAssets::from_root` + `install_runtime_assets`, no testkit feature needed), asserts the source count equals `plain_len()+1`, the `plus` cell is in the atlas with the cell size and has non-transparent pixels, the first eight pack ids and the logo are present; the four existing rebuild laws install the pack first. Run through a stub harness (`rwp2-tools/icon-stub/`: real `IconAtlasRebuild` code, stubbed `usvg`/`resvg`/`tiny_skia`/Canvas, the 250 real SVG files): **6 passed, 0 failed**; a first run failed because my stub keyed ids with the emoji prefix, fixed in the stub, not in the code. Not a cargo run.
* Hosts: native `run_native`/`run_smoke_with_axes` install the assets first, the browser Worker installs them before `semioWgpuWorkerBootstrap`, whose `step()` builds the atlas later. No host change was needed.

## 2. Crate graph and census

Edges are the plan's (Appendix F) plus the downward edges the code needed: `scene-state -> engine-registry, native-platform`, `scene-canvas2d -> canvas2d-host`, `icon-host -> asset-authority`, `scene-render -> host-base`. `edges_check.py`: no undeclared edge, no upward edge (`verify_static.py`: rank order, DAG, every path dependency resolves). Census (`census_wp2.ts`, canonical counter, host cfg; lib / test, files / bytes / active lines; limits 28 / 230,000 / 3,500):

| crate | lib | test selection |
|---|---|---|
| engine-registry | 4 / 101,242 / 2,187 | 5 / 101,339 / 2,270 |
| engine-text | 2 / 65,086 / 1,145 | 3 / 65,178 / 1,148 |
| engine-board | 4 / 93,368 / 1,776 | 5 / 93,462 / 1,785 |
| engine-graph | 2 / 69,723 / 1,307 | 3 / 69,817 / 1,322 |
| engine-attach | 2 / 58,941 / 1,111 | 3 / 59,036 / 1,122 |
| scene-state | 6 / 115,056 / 2,675 | 7 / 115,149 / 2,695 |
| canvas2d-host | 2 / 21,884 / 445 | 5 / 61,579 / 548 |
| scene-canvas2d | 2 / 55,009 / 1,084 | 6 / 103,289 / 1,744 |
| scene-ink | 3 / 109,894 / 2,431 | 9 / 161,831 / 3,209 |
| scene-feeds | 3 / 71,917 / 1,456 | 11 / 124,484 / 2,323 |
| scene-tables | 3 / 86,900 / 1,538 | 14 / 155,591 / 2,639 |
| scene-text-vfs | 3 / 72,443 / 1,323 | 8 / 112,860 / 2,047 |
| icon-host | 10 / 108,494 / 2,469 | 24 / 174,751 / 3,229 |
| scene-render | 3 / 88,481 / 1,660 | 6 / 101,916 / 1,893 |

All 28 rows are `ok` (wasm32 rows are within ±25 lines, `🗑️generated/rwp2-census-final.log`). The first scene-state test row was 4,289 lines (OVER): the 1,123-line camera fixture slice (`Scenes/🧪️tests/🧊️wgpu-standalone`, plan: scene-ink-tests) was `include!`d by the camera dispatch file; it is now included from the `scenes` facade like EngineCanvas's slice, and the `AdmittedSurface*` tests left with the lift (section 4). `ONLY=… bun census.ts` for the five engine crates (`rwp2-census-engine-1.log`, before the later moves): `overLimit false`, `closureCompilable false` because of the oversized frontier (3 oversized deps), as for every renderer crate.

## 3. What moved where (plan 5.2, executed against the live files)

Item moves inside WP2 (cut with `rwp2-tools/mvtool.py`: closure-checked, docs/attributes/regions kept, impl blocks of the named types follow):

* into `engine-registry` (all in `🗂️engine-surface-registry`): the text editor delivery state (9 items, `TextEditorDeliveryState` and 8 siblings), `engine_canvas_fault_log`/`engine_canvas_debug_log`/`PublishedGraphInteraction`/`published_graph_interaction` (from node-graph), `engine_camera_from_json` (from attach). Regions `✍️TextEditorDelivery`, `🪵️EngineCanvasDiagnostics`, `📷️EngineCamera`.
* into `scene-state` (regions in `🗃️scene-state`): the list/input kit of scene-input (9 items incl. `SceneListHit`, `SceneModifiers`, `request_canvas_pointer_gesture_cancel_for_host`), the chrome label kit and `render_placeholder` (6), the table accessibility kit (5), `retire_tiled_map_scene_identity`, `TextEditorUiState`/`TextEditorRenameDraft`/`text_editor_ui`, `InkBoundsF`, `draw_ink_rect_outline`, `VfsAccessibilityFocusTarget`, `CanvasLayer` + 5 JSON types (130 lines), and **the gumball state** (`GumballSlot`/`GUMBALL`/`request_cancel` and their 12-item closure, 182 lines): `request_canvas_pointer_gesture_cancel_for_host` calls `request_cancel`, which the plan had left in canvas2d-host, an upward edge; the press hit test, dispatch and paint stay in canvas2d-host. The Scenes root items (`SceneRuntime` region, `WorkerCell` renamed `SceneCell`, statics, identity mount/retire) are now the body of the `scene-state` library root.
* **Lifts out of WP2 files, requested by WP1 (RESULT-RENDERER-WP1.md 4a)**: `🤝️contracts/📥️lifted/🗃️scene-state/🦀️.rs` (the `AdmittedSurface*` family, 285 lines incl. the missing `IntoIterator for &AdmittedSurfaceMap` impl and its 159-line test mount, whose path is rebased) and `🖥️native-platform/📥️lifted/🎞️Scenes/🦀️.rs` (`NativeHostTemporalFormatter` port, 4 items, cfg `not(wasm32)`). Fields the scene crates read were widened to `pub`. scene-state and the other scene crates now import them from `element_contracts::` / `native_platform` and the `scenes` facade re-exports them from those crates. **WP1 must add the two mount lines** (`mod admitted_surfaces; pub use admitted_surfaces::*;` in contracts, the lifted file in native-platform); the lift uses `semio_framework_async::boxed_fixed_slots` and the native one `ui_contract`, so both manifests need those dependencies.
* Lifts into WP2 crates mounted by `scene-state`: `📥️lifted/🏠️os-host` (component surface close bridge, 112 lines) and `📥️lifted/🎯️chrome-ledger` (26 lines), both WP3's. I replaced `crate::engine_canvas::EngineSurfaceToken` in the os-host file by the registry crate path (valid in both mounts). WP3 still mounts the os-host file in `os_host` (`scene_state_lift`): until it re-exports from the crate instead, the bridge static exists twice. The chrome ledger is mounted once (scene-state).
* Not done (deferred on purpose): `EngineSurfaceToken` stays in engine-registry (the 5-line lift to contracts would break scene-state until WP1 mounts it; scene-state depends on engine-registry instead); `🧩️ui-image-loading` (301 lines) was not lifted by WP3, see section 7.
* `crate::infinite::board::…` in engine-attach was a dangling path left by a peer rename (baseline E0433); it is now `infinite_canvas::board::…`.

Cross-crate rewriting: `super::<sibling>` -> `semio_framework_os_renderer_wgpu_<crate>::<module>` for the engine family (`pub mod` per old module, e.g. `engine_registry::engine_surface_registry`); `engine_canvas::NAME` -> the owning crate/module; `crate::interpreter|shell|app_now_ms|…` -> `crate::element_contracts|host_base::…` aliases declared by `extern crate … as …;` in each library root (10.4.4 item 5); the Scenes children keep their flat `use super::*` namespace, now supplied by the library root (external leaves, explicit upstream `use` lists generated from the exported names, glob-mounted children).

Visibility (`widen.py`): items, fields, methods, re-exports and `thread_local!` statics that code in another crate, the facades or the tests name were widened from `pub(super)`/`pub(crate)`/private to `pub`, everything else kept (several hundred declarations over the 14 crates, `widen.py` counts per run; field and method decisions are name-and-type token based and can over-widen a field whose name another crate also uses). `fixtrait.py` removed visibility qualifiers my widening had put on trait-impl fns (E0449 caught by the oracle).

## 4. Tests

* In-file test mounts stay where they were (they travelled with their files into the crates). Three engine files mounted tests that reach into the upper crates (`board2d-engine`, `catalogue-workflow-drop`, `metrics-invalidation-drain`): hoisted into the `engine_canvas` facade with their test-only imports. The facade keeps the five EngineCanvas/Scenes standalone slices, `ENGINE_CANVAS_SOURCE`, `SCENES_SOURCE`.
* The plan's companion crates (`engine-attach-tests`, `scene-ink-tests`) are still empty skeletons: the big cross-crate suites stay in the facades (which belong to the monolith test unit) until WP0 phase 2. Tests that read fixtures through `CARGO_MANIFEST_DIR/../../../../…` only work from the monolith package directory.

## 5. Registries and manifests

No registry row was added (WP0 froze them). Manifests: dependencies were completed from the code (`deps_check.py`, specs copied one directory deeper from the old top manifest; `add_dep.py`): e.g. `ui_gpu`, `vello`, `js-sys` (registry), `serde`, `base64`, `image`, `ui_render` (ink, tables, canvas2d), `semio-framework-canvas` + `ui_viewport` + `ui_gpu` + `semio-framework-async` + `serde` (icon-host). `Cargo.lock`: `bun repo cargo … update --workspace --offline` ran (log `rwp2-cargo-update.log`); the lock also took other peers' pending crates.

## 6. Verification (commands run, real results)

| check | result |
|---|---|
| union differential, native lib (`oracle2.py`: the 14 crates mounted as modules of the live renderer tree, crate-name paths rewritten, `pub(crate)` kept crate-private, lifted files copied per mount, compared with the tree whose WP2 element trees and hub are the pre-carve snapshot; `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`, sandbox patches of the oracle note) | `all5` (final state): A 5,482 errors, B 5,351, only-new 135, only-old 4. Every only-new row is an unresolved external or pending-crate name: `ui_wgpu`, `serde`, `serde_json`, `semio_framework`, `infinite_canvas`, `semio_framework_trace`, `semio_framework_os_renderer_camera` (not in the sandbox) and `crate::element_contracts::…` (the sandbox's contracts is an empty module after WP1's flip) plus the pending names of section 7. **No E0412/E0603/E0616/E0624/E0432 for an in-crate item.** Only-old: the dangling `crate::infinite` (fixed) and two contracts import rows of the old layout. |
| same, wasm32-unknown-unknown lib | `all5` wasm: A 5,311, B 5,178, only-new 137, only-old 4: the same classes as native plus `crate::host_base::host_io_call` (wasm-only branch; WP4's host-io lift into host-base) and `js_sys` (external). No in-crate resolution error. |
| earlier stages | engine family alone `eng1` (lib): only-new = 3 external rows, only-old = the `infinite` row; `eng2` wasm lib: same; the first scene runs (`ss1`..`ss3`, `sc1`, `all1`, `all2`) found and I fixed: missing facade names (computation had skipped the Shell and renderer trees), private statics/enums in the library roots, E0449, mod `scene_state` vs fn `scene_state`, `CanvasLayer`/`canvas2d_paint` back edges, `TextFace`, engine names imported as flat instead of by module, `slots`/`visible` stray imports |
| seeded sensitivity | making `engine_camera_from_json` private: **3 new E0603** (board, paint2d, facade), otherwise identical (`eng1seed`, `rwp2-oracle-eng1seed.log`) |
| definition scan | `widen.py` is the scan (cross-crate token usage per declaration); after the last apply it reports 0 more for all 14 crates |
| `verify_static.py` | manifests parse (Bun TOML), names/boundary keys/lib paths exact, every path dependency resolves, no upward edge, every `#[path]`/`include_str!` resolves, longest path < 240 B, each file owned by one crate (lifted files excepted), 0 orphan `.rs` in the four element trees, rustfmt parses all 14 library roots: `ALL STATIC CHECKS PASSED` |
| `edges_check.py` / `deps_check.py` | no undeclared first-party edge, no missing external dependency; declared-but-unused edges of the frozen plan remain (engine-text `contracts`, engine-graph `media-slots`, engine-attach `canvas2d-paint`/`media-slots`, scene-canvas2d `asset-authority`/`media-slots`, scene-tables `media-slots`, icon-host `media-slots`) |
| icon stub harness | 6 passed (section 1) |
| assets freshness | `[assets] fresh 262` |
| launcher | `update --workspace --offline` ok; `check -p semio-framework-os-renderer-wgpu-engine-registry --all-targets`: first `cargo fetch --locked … cannot update the lock file` (before the update), then after it `Rust crate Source boundary exceeded before compiler byte allocation` (logs `rwp2-cargo-check-start.log`, `rwp2-cargo-check-registry.log`) |
| `--cfg test` and wasm32 test differentials | not obtained: the full-monolith test configuration took 25+ minutes per side under load and the WP1 hubs were mid-edit (`couldn't find file …` in a `cfg(test)` mount at my first attempt); see "Open" below |

## 7. Hand-offs and pending names (nothing in another package's file was edited)

* **WP1**: mount `📥️lifted/🗃️scene-state` in `contracts` and `📥️lifted/🎞️Scenes` in `native-platform` (section 3); add `semio-framework-async` / `ui_contract` dependencies there; `asset-authority` still needs `IconRenderHost/📤️export/📥️asset` (138 lines) lifted by me: I did **not** lift it because `scenes::icon_export::asset` is also named by `mailbox-impl` (WP3) and the file keeps its single owner `icon-host` until `asset-authority` exists; then one `mod` line moves and icon-host imports it (`icon-host -> asset-authority` is already declared). `host_io_call`, `mesh_assets`, `app_now_ms`, `log_debug_diagnostic`, `spawn_app_task`, `action_args_json!` are written against `host-base` (`crate::host_base::…`, `crate::mesh_assets::…` aliases, `pub(crate) use …host_base::action_args_json;`), `renderer_asset_io`/`reserve_renderer_asset_request`/`cancel_renderer_map_tile_assets` against `asset-authority`.
* **WP3**: (1) `component_scene_is_presented` (still in `Interpreter/🧩️retained-ui-engine`) must move to `contracts`; `scene-state` names `element_contracts::component_scene_is_presented` (`ui_document_close_pending_for` and `framework_widget_context` already sit in contracts); (2) lift `🖼️ui-image-loading`'s `current_ui_image_key` and the `UI_IMAGE_*` statics into `🎨️scene-canvas2d/📥️lifted/…` (then one `mod` line in the scene-canvas2d root; scene-feeds and scene-text-vfs already call `scene_canvas2d::current_ui_image_key`, 2 pending rows in the oracle); (3) the `os_host` mount `scene_state_lift` of the close bridge must become a `pub(crate) use semio_framework_os_renderer_wgpu_scene_state::{ComponentSurfaceClose…}` re-export (duplicate static otherwise); (4) re-read before editing: I changed only the lifted os-host file, `Interpreter` is untouched.
* **WP4**: `host_io_call` (named as `crate::host_base::host_io_call` by `scene-feeds`), `ShellFindItem` and `try_push_find_item` are imported from `element_contracts` by `scene-feeds`/`scene-render` (the lift of `🔖️identity-and-types` into contracts).
* **WP0 phase 2**: the facades `EngineCanvas/🦀️.rs` and `Scenes/🦀️.rs` keep the test mounts and the `*_SOURCE` consts until the companions exist; the hub keeps `engine_canvas`/`scenes` as mounts.

## 8. Residual risks

1. No real compile. The oracle cannot see trait scoping, method resolution on external types or macro-generated items; privacy of fields and methods is checked by token heuristics plus the resolution errors the oracle does produce. First cargo check of the crates (once the frontier is carved) is the real verdict; likeliest faults: a missed `pub` on a field/method of an external-typed value, an `unused import` flood (the Scenes library roots import every leaf their children use, some are unused), `use` order for rustfmt.
2. The Scenes crates keep the flat `use super::*` namespace of the old module (explicit upstream imports, glob-mounted children); a future cleanup should turn the child-level globs into explicit imports.
3. Over-widening: some fields/methods are `pub` because another crate has the same name in a file mentioning the same type.
4. Two crates (`contracts`, `native-platform`) must wire lifts before `scene-state` can resolve `AdmittedSurfaceMap`; until then the `scenes` module resolves only in the union simulation (the shim mounts both lifted files as `wp2_contracts`/`wp2_native_platform` in the sandbox and the differential is clean for them).
5. The icon atlas is empty of catalog icons on a host that fails to install the pack (logged by the host); before, the catalog was compiled in.

## 9. Files

Created: the 14 library roots `T/<folder>/🦀️.rs` (rewritten from the WP0 stubs), lifted files `T/🤝️contracts/📥️lifted/🗃️scene-state/🦀️.rs`, `T/🖥️native-platform/📥️lifted/🎞️Scenes/🦀️.rs`; `rwp2-tools/` (`model.py`, `mvtool.py`, `imports.py`, `widen.py`, `facade_names.py`, `regen_facade.py`, `carve_engine.py`, `carve_scene.py`, `stage_*.py`, `fixtrait.py`, `add_dep.py`, `deps_check.py`, `edges_check.py`, `graph.py`, `oracle2.py`, `reparse.py`, `verify_static.py`, `census_wp2.ts`, `toml.ts`, `outline.py`, `icon-stub/`); this file.
Edited: the four element trees (`EL/⚙️EngineCanvas`, `EL/🎞️Scenes`, `EL/🖼️IconRenderHost`, `EL/📐️Canvas2dHost`: moves, imports, widening, facades), the hub `T/🧊️renderer/📥️mounts/🎞️wp2-scenes/🦀️.rs` (two `extern crate` units), 14 crate `Cargo.toml`, `🖼️canvas/🔌️runtime-assets/{🦀️.rs,🧪️tests/🦀️.rs}`, `🖼️canvas/🦀️.rs`, the assets generator (`🔣️icons/🏗️builder/📽️projection/🟦️.ts`, `🏗️builder/📦️publication/🟦️.ts`, `📦️packages/🟦️typescript/📋️project.json`), `Cargo.lock` (launcher update).
Deleted: `🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds/` (generated, gitignored).
Longest new path: the lifted-file mounts stay below 240 B (`verify_static.py`).
