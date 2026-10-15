# Seam Map: OS Rust tier-2 monolith split (4 files)

Sources (all under `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/`):

| Id | File | Lines | Bytes | Crate |
|---|---|---|---|---|
| A | `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | 7,651 | 383,408 | `semio-framework-os-renderer-wgpu` |
| B | `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` | 6,860 | 358,271 | `semio-framework-os-renderer-wgpu` |
| C | `🖥️host/🦀️.rs` | 4,671 | 266,024 | `semio-framework-os` |
| D | `🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | 5,831 | 384,928 | `semio-framework-os-mcp` |

Mode: read-only audit. No source file was edited. No git, cargo, nx or build was run. Raw `/usr/bin/grep` and `python3` analysis scripts were run from the session scratchpad only. Line numbers refer to the files as audited (mtimes Oct 9 to Oct 15 2026); re-verify each anchor line before cutting.

## 0. Summary

- Every one of the four files is above the 262,144-byte per-crate gate of the compiler-ingress source-boundary check (`📓️EXECUTOR-RULES.md`). None can become a crate as it stands. All proposed children are below both the 2,500-active-line cap and the byte gate (largest child: 72,333 bytes in A, 95,763 in B, 82,675 in C, 81,120 in D).
- A (EngineCanvas): parent keeps 1-35 (imports) and the `wgpu-standalone` `include!` at 2398-2399. Nine children (`🗂️`, `📦️`, `🎓️`, `🧩️`, `🕸️`, `🗺️`, `🎲️`, `🖌️`, `✍️` folders). Largest child 1,180 active lines. 31 private items widened to `pub(super)`. 7 test mounts gain one `../`.
- B (Interpreter): parent keeps 1-90 (imports and `WorkerCell`, which target A imports by path) plus the worker-cell test mount. Ten children. Largest child 1,499 active lines. 71 private items widened (the largest set of the four). 4 test mounts move or gain one `../`.
- C (OS host): the file is already a set of inline modules (`host`, `backbone`, `instance`, `media_export_raster`, `media_export_simple`, `workflow`, `codec_abi`, `registry`). Each becomes `#[path]` file module. Module paths do not change, so no visibility is widened. Only the 9 `include!`/`include_str!` paths change (`../`).
- D (MCP workspace): parent keeps 1-69 and the quick/long test mounts. Twelve children. Largest child 813 active lines. 41 private items widened. Three `#[path]` mounts gain one `../`. `remote` and `workspace_identity` stay put.
- Cross-target: the renderer element cluster is cyclic. EngineCanvas and Interpreter reference each other; Interpreter and Scenes reference each other (see section 6). Scenes already split its wgpu monolith into children (`🧭️node-graph-and-tiled-map`, `🧩️board2d-and-vfs`, `✍️text-editor`, `🧮️table-and-paint2d`, ...). Reuse its emoji for the same concepts.
- Two repo-policy hazards must be handled in the same change: `POLICY_SPLIT_SOURCE_ROOTS` in the repo policy library does not list any of the four parents, and `📜️script.ts` allowlists and audits read these files as single files (section 6.3).

## 1. Measurement method

- Active line: non-blank and not starting with `//` after trimming. Doc comments and region markers count as comments. Attributes, `#[cfg]` lines and block comments count as active.
- Bytes: UTF-8 bytes of every line plus its newline.
- Cut points: blank line, `//#region` / `//#endregion` boundary, or the closing `}` of a top-level item. Each cut was verified at audit time (listed in each section).
- Cross-references: for each private top-level `fn`, `struct`, `enum`, `const`, `static`, `type`, `trait`, the whole-word identifier was searched across all lines not starting with `//`. Uses are counted per proposed child. Names used from another child are listed as widening candidates. Private methods were not listed item by item because generic names (`new`, `get`, `drop`, `default`, `close`, `token`) collide with other types; the blanket rule in section 4.1 of the example applies (widen every private method called from another child).
- Assumption check: `super::` and `self::` appear 0 times in all four files. `macro_rules!` appears 0 times in all four. `#![...]` inner attributes appear 0 times in all four.

---

## 2. Target A: EngineCanvas wgpu target (7,651 lines)

### A.1 Owning crate and targets

- Crate: `semio-framework-os-renderer-wgpu`, manifest `🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`, lib root `📦️packages/🦀️rust/📚️library/🦀️.rs`.
- Compiled as `include!("../../../🧊️renderer/🦀️.rs");` at the lib root (lib line 12, under `cfg(not(target_os = "wasi"))`). The renderer file is the crate's real root module tree.
- Targets: native (linux, macOS, windows: `cfg(not(target_arch = "wasm32"))`) and browser `wasm32-unknown-unknown`. Not `wasi`. wasm-only lines: 33 (`use js_sys`) and 6945-6946 (`#[wasm_bindgen(js_name = semioWgpuHostPageHidden)]`). Native-only: 1118. Paired native and wasm `cfg` at 1959 and 1964 (`text_editor_now_ms`).

### A.2 How it is mounted

- `🧊️renderer/🦀️.rs` lines 92-93: `#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod engine_canvas;`. Module path `crate::engine_canvas` (82 uses in Scenes, 24 in Interpreter, plus the renderer frame modules).
- Within the file, relative paths resolve from `⚙️EngineCanvas/🎯️targets/🧊️wgpu/`. After the split, each child sits one folder deeper (`🎯️targets/🧊️wgpu/<child>/`), so every relative path gains one `../` (section A.6).

### A.3 Section table (current file)

| Lines | Content (line of item) | Goes to |
|---|---|---|
| 1-35 | `//!` doc; imports; `#[cfg(target_arch = "wasm32")] use js_sys;` (33) | P |
| 36-838 | `//#region Registry`: `NodeGraphEngine` (37), `NodeGraphSyncCache` (43), `EngineSurface` (59), capacities (99-104), `retire_string_page` (107), `retire_bytes_page` (120), `EngineSurfaceId` (129), `EngineSurfaceToken` (168), identity/snapshot/freshness/slot (174-193), `EngineSurfaceRegistry` (201-354), `EngineSurfaceClosePhase` (355), `NodeGraphEngineRetirement` (375), `EngineSurfaceRetirement` (396-838, Drop 833) | C01 |
| 839-1589 | `//#region PreparedEngineCanvas`: `EngineCanvasPacket` (843), `StagedEngineScene(s)` (875, 885), `STAGED_ENGINE_SCENES` (934), `stage_engine_packet_step` (938), `EngineCanvasBuildContext` (966-1117), `EngineGpuCandidate` (1142), `EngineGpuSlot` (1249), `EngineCanvasPresenter` (1297-1588) | C02 |
| 1590-1678 | `MapSyncCache` (1592), `BoardSyncCache` (1616), `RasterSyncCache` (1654), `EditorSyncCache` (1673) | C01 |
| 1679-1917 | `//#region EditorSceneDocument`: `EditorSceneRange`...`EditorSceneHoverField` (1686-1838), `text_editor_scene_member` (1847), `text_editor_scene_document` (1871), `synchronize_text_editor_host` (1906) | C09 |
| 1918-2157 | `TEXT_EDITOR_PENDING_ECHO_CAPACITY` (1919), `TextEditorDeliverySnapshot` (1922), `TextEditorTypingEnd` (1943), `text_editor_now_ms` cfg pair (1960, 1965), `TextEditorDelivery*` (1969-1982), `TextEditorDeliveryState` (1990-2157) | C09 |
| 2158-2231 | `raster_sync_terminal` (2158), `editor_sync_terminal` (2171), `node_graph_sync_terminal` (2175), `map_sync_terminal` (2191), `board_sync_terminal` (2205), `MAP_TILE_ASSET_FAULT` (2228), `ENGINE_SURFACES` (2230) | C01 |
| 2232-2397 | Tutorial geometry: `TutorialSurfaceGeometry` (2233), `tutorial_polyline_point` (2240, pub), `tutorial_geometry_from_json` (2262), `tutorial_graph_*` (2286-2319), `resolve_tutorial_surface_point` (2334, pub) | C03 |
| 2398-2399 | `#[cfg(test)] include!("../../🧪️tests/🧊️wgpu-standalone/🦀️.rs");` | P (path unchanged) |
| 2401-2415 | Test mounts: `engine_surface_attach_tests` (2401-2403), `node_graph_attach_tests` (2405-2407), `node_graph_gesture_tests` (2409-2411), `metrics_invalidation_drain_tests` (2413-2415) | C04 (2401-2403, 2413-2415); C05 (2405-2411) |
| 2417-2569 | `empty_engine_surface` (2417), `ensure_engine_surface` (2458), `engine_surface_wire_id` (2496), `engine_surface_token` (2500), `engine_surface_live_freshness` (2504), `engine_surface_token_at` (2520), `begin_engine_surface_close_token` (2530), `close_engine_surface_step` (2547), `engine_surface_terminal_nonopaque_is_empty` (2551), `create_target_texture` (2555), region marker (2568-2569) | C01 |
| 2570-3679 | `//#region EngineSurfaceAttach`: `EngineSurfaceKindDetail` (2580), `EngineSurfaceRegistration` (2598), `AttachedSurfaceRegistry` (2613-2652), `take_engine_surface_registrations` (2658), `node_graph_engine_from_scene` (2680), `sync_node_graph_engine` (2754), `sync_node_graph_scene` (2902), `register_engine_surface` (2948), `sync_map_engine` (3009), `sync_tiled_map_scene` (3073), `sync_board_engine` (3110), `sync_board2d_scene` (3322), `sync_raster_engine` (3377), `sync_paint2d_scene` (3462), `sync_text_editor_scene` (3491), `sync_engine_scene` (3578), `stage_engine_scene_paint` (3591), `RasterCompositeProgress` (3644), `engine_raster_key` (3676) | C04 |
| 3680-4999 | `//#region NodeGraph`: `node_graph_fit_camera` (3784), `node_graph_set_catalogue_json` (3802), drag ghost and drop actions (3822-3955), `node_graph_wheel_into` (3986), pointer down/move/up/cancel (4045-4136), `GraphInteraction*` (4152-4190), connection cursor (4217), screen pointer (4255-4462), graph interaction publish (4551-4690), labels (4745-4900), overlays (4903-4998). Test mounts `node_graph_attach/gesture_tests` (2405-2411) and `catalogue_workflow_drop_tests` (3982-3984) | C05 |
| 5000-5297 | `//#region TiledMap`: `take_map_tile_asset_fault` (5003), tile fetch (5009-5084), `with_map_host(_mut)` (5105-5114), camera (5124-5134), interaction (5143-5296) | C06 |
| 5298-6164 | `//#region Board2d`: `CoalescedBoardEvents` (5301), `with_board_host(_mut)` (5314-5335), `puzzle_board_*` (5345-6131), board event queue and pointer plan (5388-5768), board pointer settle (5768-5834). Test mount `board2d_engine_tests` (6162-6164) | C07 |
| 6165-6765 | `//#region Paint2d`: constants (6178-6179), `with_raster_host_mut` (6181), `paint2d_pick_layer` (6195), `//#region ContextMenuTargets` (6204-6318), `Paint2dMarquee` (6399-6477), `paint2d_pointer_button_into` (6500), `paint2d_pointer_move_into` (6656), `paint2d_wheel_into` (6735). Test mount `paint2d_engine_tests` (6166-6168) | C08 |
| 6766-7651 | `//#region TextEditor`: `TextEditorExplicitDraftSettings` (6769), `text_editor_wheel_into` (6807), `emit_text_editor_actions` (6821), `TextEditorActionOutcome` (6883), `drive_text_editor_*` (6954-6978), `settle_text_editor_action_receipt` (7071), explicit draft fns (7175-7396), public text editor API (7396-7636), `//#region ScenesInteropAdditions` (7577-7650), `semioWgpuHostPageHidden` (6945-6946) | C09 |

Child totals (section A.4): P 26, C01 1,018, C02 656, C03 158, C04 954, C05 1,087, C06 236, C07 708, C08 489, C09 1,180; total 6,512. Every non-blank line is assigned to exactly one child (checked by script).

### A.4 Proposed children

Folder names reuse existing repo emoji for the same concept (`🕸️` NodeGraph, `🎲️` board, `🗺️` tiled map, `🖌️` paint, `✍️` text editor, `🎓️` tutorial, `🗂️` catalog). None repeats inside `⚙️EngineCanvas/🎯️targets/🧊️wgpu/`, which has no other children. Longest proposed child path (`🎓️tutorial-surface-geometry/🦀️.rs`) is 163 characters and 225 UTF-8 bytes, under the 240-byte rule.

| Child | Folder (`🦀️.rs` inside) | Ranges (inclusive) | Active | Bytes | Consumed by (cross-reference rows) |
|---|---|---|---|---|---|
| C01 | `🗂️engine-surface-registry` | 36-838, 1590-1678, 2158-2231, 2417-2569 | 1,018 | 45,404 | C02, C03, C04, C05, C06, C07, C08, C09 |
| C02 | `📦️prepared-engine-canvas` | 839-1589 | 656 | 36,848 | C01, C04 |
| C03 | `🎓️tutorial-surface-geometry` | 2232-2397 | 158 | 8,290 | none outside (leaf) |
| C04 | `🧩️engine-surface-attach` | 2401-2403, 2413-2415, 2570-3679 | 954 | 57,819 | C07, C08 (`engine_camera_from_json`) |
| C05 | `🕸️node-graph` | 2405-2411, 3680-4999, 3982-3984 | 1,087 | 68,956 | C01, C02, C04, C09 |
| C06 | `🗺️tiled-map` | 5000-5297 | 236 | 14,401 | C04 |
| C07 | `🎲️board2d` | 5298-6164 | 708 | 44,897 | C04 |
| C08 | `🖌️paint2d` | 6165-6765 | 489 | 32,685 | C07 |
| C09 | `✍️text-editor` | 1679-1917, 1918-2157, 6766-7651 | 1,180 | 72,333 | C01, C04 |
| P | `🦀️.rs` (kept) | 1-35, 2398-2399 | 26 | 1,897 | none (wiring only) |

Parent wiring (added to `🎯️targets/🧊️wgpu/🦀️.rs` after the imports; `pub use` for the public API, plain `use` for `pub(super)` names; use explicit name lists generated from the cross-reference, not `use child::*`, to follow the executor rule against glob ambiguity):

```rust
#[path = "🗂️engine-surface-registry/🦀️.rs"]
mod engine_surface_registry;
pub use engine_surface_registry::{/* public names */};
use engine_surface_registry::{/* pub(super) names used by siblings */};
// repeat for C02..C09
```

### A.5 Visibility widening (from the cross-reference)

Items private at definition and used from another child. Each becomes `pub(super)`:

- C01 (18): `NodeGraphEngine`, `NodeGraphSyncCache`, `EngineSurface`, `retire_string_page`, `EngineSurfaceId`, `EngineSurfaceIdentity`, `EngineSurfaceSnapshot`, `EngineSurfaceLiveFreshness`, `EngineSurfaceRegistry`, `MapSyncCache`, `BoardSyncCache`, `RasterSyncCache`, `MAP_TILE_ASSET_FAULT`, `ENGINE_SURFACES`, `ensure_engine_surface`, `engine_surface_wire_id`, `engine_surface_live_freshness`, `create_target_texture`.
- C02 (2): `StagedEngineScene`, `STAGED_ENGINE_SCENES`.
- C04 (1): `engine_camera_from_json`.
- C05 (5): `engine_canvas_fault_log`, `engine_canvas_debug_log`, `PublishedGraphInteraction`, `log_graph_geometry_census`, `published_graph_interaction`.
- C06 (1): `reserve_map_tile_fetches`. C07 (1): `board2d_granularity_by_id`. C08 (1): `write_interaction_hover`.
- C09 (2): `synchronize_text_editor_host`, `TextEditorDeliveryState`.
- Blanket rule for methods: every private method of an `impl` in one child that is called from another child (for example `EngineSurfaceRegistry` methods called from C04 and C09) becomes `pub(super)`.
- `ENGINE_SURFACES` (static, `WorkerCell`) is read by six children. Keep it in C01 and widen; do not duplicate it.

### A.6 Relative paths, includes and test mounts

| Current line | Kind | Current text | Child | Text after move |
|---|---|---|---|---|
| 2399 | `include!` (cfg test) | `"../../🧪️tests/🧊️wgpu-standalone/🦀️.rs"` | P | unchanged |
| 2402 | `#[path]` mod | `"../../🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs"` | C04 | `"../../../🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs"` |
| 2406 | `#[path]` mod | `"../../🧪️tests/🕸️wgpu-node-graph/🦀️.rs"` | C05 | `"../../../🧪️tests/🕸️wgpu-node-graph/🦀️.rs"` |
| 2410 | `#[path]` mod | `"../../🧪️tests/🫳️node-graph-gestures/🦀️.rs"` | C05 | `"../../../🧪️tests/🫳️node-graph-gestures/🦀️.rs"` |
| 2414 | `#[path]` mod | `"../../🧪️tests/🩺️metrics-invalidation-drain/🦀️.rs"` | C04 | `"../../../🧪️tests/🩺️metrics-invalidation-drain/🦀️.rs"` |
| 3983 | `#[path]` mod | `"../../🧪️tests/🔬️wgpu-catalogue-workflow-drop/🦀️.rs"` | C05 | `"../../../🧪️tests/🔬️wgpu-catalogue-workflow-drop/🦀️.rs"` |
| 6163 | `#[path]` mod | `"../../🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs"` | C07 | `"../../../🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs"` |
| 6167 | `#[path]` mod | `"../../🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs"` | C08 | `"../../../🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs"` |

Seven test mounts and one `include!` (kept in P). The mounted test files use `use super::*;` and reach the module through the parent glob chain; they keep working when P re-exports every child name.

Note: the `wgpu-standalone` test file (`🧪️tests/🧊️wgpu-standalone/🦀️.rs:63`) reads this monolith with `include_str!("../../../⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs")` into `ENGINE_CANVAS_SOURCE`. After the split that string holds only the parent, so any assertion over children fails silently. Such readers need a module-tree read (section 6.3).

### A.7 `#[cfg(test)]` and macros

- Test-only mounts: 8 (listed above). Test-only items gated with `#[cfg(test)]` also exist: `EngineCanvasPacketDestination` (950) and `EngineCanvasPacketReservation` (957) in C02; `board2d_tool_run_trace_state` (3311) in C04; `component_scene_begin_note_edit_fixture` (3695) in C05; `pick_target_hits_for_test` (6308) and `selection_domains_for_test` (6314) in C08. They move with their chunks.
- `macro_rules!`: none. No macro hazard.
- `#[wasm_bindgen]` export: `semioWgpuHostPageHidden` (6946) moves with C09 and keeps its `js_name`.
- `thread_local!`: none in this file. State lives in `WorkerCell` statics.

### A.8 Wrong-direction dependencies

- Element cycle: `use crate::interpreter::{FrameworkWidgetContext, ScenePointerTarget};` (line 10) and `use crate::interpreter::WorkerCell;` (line 25). The EngineCanvas element imports a sibling element. Interpreter in turn uses `engine_canvas::` 24 times. Move `WorkerCell`, `FrameworkWidgetContext` and `ScenePointerTarget` into a neutral module (for example the existing `🧵️worker-cell` concept and a scene-pointer contract) so the arrow points one way.
- Concrete plugin crates imported into a framework element: `flow::{dag::...}` and `flow::FlowHost` (lines 11-14, plugin `🌊️flow`), `semio_framework_os_node_graph::GraphHost` (16, plugin `🕸️dag`/NodeGraph), `framework_editor::EditorHost` (15, `✍️editor` module), `infinite_world::world::tool_run_trace` (21). The file also contains plugin-named identifiers: `puzzle_board_*` and `puzzle3d_catalogue_drag_payload*` (`🧩️puzzle`), `last_note_click` and `editing_note_id` (`🗒️note` state) and `FLOW_WIDGET_DRAG_MIME` (`application/x-flow-widget`). A framework element should receive plugin behaviour through an interface rather than name the plugin.
- `tutorial_*` geometry is framework-level and stays. It is cleaner in its own child (C03) as proposed.

### A.9 Ordered extraction sequence

1. Baseline (not run here): `cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-renderer-wgpu --message-format=short`, plus the `--tests` and wasm32 variants, recorded before the first cut.
2. Wire empty P: add `mod`/`pub use`/`use` lines for the nine children to P (children not yet created).
3. Extract leaves first, in this order: C03 (leaf), C06, C08, C07, C09, C05, C04, C02, C01. Each step cuts by content anchors (`assert text.count(anchor) == 1`), moves the range verbatim, adds the `pub(super)` names from A.5, rewrites the test `#[path]`s (A.6), and runs `git diff --stat` (read-only) plus the cargo check.
4. After C01 moves, P keeps only imports, wiring, the `include!` and nothing else.
5. Update the reader sources (A.6 note and section 6.3) in the same change; a split that leaves them stale is not complete.

### A.10 Coupling notes

- C01 is the hub: `ENGINE_SURFACES` (static) is read by all of C04-C09. Keep it in C01.
- C04 (attach) and C05 (node graph) are the heaviest pair: C05 uses `ENGINE_SURFACES` 25 times. C04 holds the node-graph scene sync (`sync_node_graph_engine`, `sync_node_graph_scene`, 2754-2948) and C05 holds node-graph interaction. Both use `NodeGraphEngine` from C01. Keep them as separate children with `pub(super)` names; moving the scene sync into C05 is the alternative.
- C09 owns the text-editor delivery state (`TextEditorDeliveryState`), which C01 reads only for the terminal check (3 uses). Keep.

---

## 3. Target B: Interpreter wgpu target (6,860 lines)

### B.1 Owning crate and targets

- Same crate as A: `semio-framework-os-renderer-wgpu`. Mounted from `🧊️renderer/🦀️.rs` lines 95-96: `#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod interpreter;`. Module path `crate::interpreter`.
- Targets: native and `wasm32-unknown-unknown` (109 `cfg(wasm32)`-tagged lines, many paired native/wasm implementations). Not `wasi`.
- `#[wasm_bindgen]` exports (all in the introspection region): `dumpStructure` (6780), `dumpFrameStats` (6788), `dumpMeshStats` (6805), `dumpBoard2d` (6815), `dumpChrome` (6839), `dumpAccessibility` (6846).

### B.2 Section table

| Lines | Content (line of item) | Goes to |
|---|---|---|
| 1-90 | Imports (10-21), `FrameworkWidgetContext` (23), `WorkerCell` (32, pub(crate)), `test_worker_cell` (58) and its thread-local (49-57) | P (consumed by A and Scenes) |
| 91-291 | `//#region RenderPlanValidator`: `RenderPlanLimits` (93), `RENDER_PLAN_LIMITS` (109), `validate_component_scene` (147), `walk_ui_node` (252), `validate_window_body_surface` (284) | C01 |
| 293-1966 | `//#region RetainedEngineCutover` head: block comment (294-319, must move with `UI_ENGINE`), `UI_ENGINE` (320) and `UiEngineCell` (331-378), `UiDocumentCloseQueue` (387-443), `SceneIntentQueue` (514-676), `ScenePointerOwners` (742-746), retained hit registry (947-1000), `dispatch_ui_event` (1117), `apply_ui_commands` (1505), retained input receipts (1557-1605), clipboard IO (1741-1900), `apply_scene_ui_command` (1921) | C02 |
| 1967-2312 | `//#region TextEditorFocus`: `FocusedTextEditor` (1975), focus and caret fns (1988-2072), `apply_focused_text_editor_*` (2121-2259) | C03 |
| 2314-2481 | `//#region TableEditableTextFocus` (2315-2481) | C03 |
| 2483-2542 | `//#region TableStepperFocus` (2484-2542) | C03 |
| 2544-2598 | `//#region VirtualFileSystemFocus` (2545-2598) | C04 |
| 2600-2694 | `//#region InkEditorFocus` (2601-2694) | C04 |
| 2696-3301 | `//#region InkClipboardFocus` (2698-3301): ink clipboard address, pending native clipboard slots (2771-2858), stream fns (2970-3096), `close_window_focus_clipboard_one` (3105), `close_retiring_focus_clipboard_one` (3209) | C04 |
| 3302-3423 | `drive_scene_interaction_step` (3303), `PresentedInputCandidate*` (3334-3340), `//#region SurfaceContextMenu` (3362-3423) | C05 |
| 3424-3779 | `SceneIntentProgress` (3425), `process_scene_interaction` (3438), `FrameworkSceneHost` (3693-3762), `layout_step_is_window_work` (3763), `drive_mounted_layout_text_one` (3772) | C05 |
| 3780-4125 | `//#region RetainedDocumentConsumer` (3780-4124): `UiDocumentFrameCursor` (3804), `set_ui_document_flow` (3868), `render_ui_document_step` (3955) | C06 |
| 4126-4132 | `#[cfg(all(test, not(wasm32)))]` mounts `ui_command_wiring_tests` (4127), `app_catalogue_tests` (4131) | C02 (recommended; the tests exercise `apply_ui_commands`) |
| 4133 | `//#endregion RetainedEngineCutover` | (marker) |
| 4135-4509 | `//#region UiImageLoading` (4135-4420): `UI_IMAGE_*` caches (4136-4143), `apply_ui_image_bytes` (4154), `rasterize_svg_to_rgba` (4231), data-url decoding (4258-4279). Then `render_ui_image_step` (4424), `framework_widget_context` (4494) | C07 |
| 4510-4514 | `//#region RenderPlanValidatorTests`: `#[cfg(test)]` mount `render_plan_validator_tests` (4511-4513) | C01 |
| 4516-5791 | `//#region Introspection`: `IntrospectionTypes` (4528-4701), `IntrospectionPathGrammar` (4703-4772), `IntrospectionVisualFields` (4774-4860), `IntrospectionWalk` (4862-4920), `IntrospectionWindowSelection` (4922-4944), `IntrospectionBuilders` head (4946-5791, incl. accessibility dump builders) | C08 |
| 5792-6006 | `//#region MeshStats` (5792-5916), `//#region Board2dStats` (5918-6004) | C09 |
| 6008-6767 | `//#region ChromeLedger`: `CHROME_LEDGER` (6099), `CHROME_ACCESSIBILITY` (6207), `ACCESSIBILITY_VISIBLE_WINDOWS` (6217), `accessibility_window_is_visible` (6621), `project_chrome_dump` (6742) | C10 |
| 6769-6856 | `//#region IntrospectionExports` (6769-6851): six `#[wasm_bindgen]` dump exports (6776-6846). `introspection_tests` mount (6853-6855) | C08 |
| 6857-6860 | `#[cfg(test)] #[path = "../../🧵️worker-cell/🧪️tests/🦀️.rs"] mod worker_cell_tests;` | P (path unchanged) |

Child totals (section B.3): P 60, C01 182, C02 1,499, C03 520, C04 699, C05 440, C06 261, C07 345, C08 1,121, C09 182, C10 639; total 5,948 active lines. Every non-blank line is assigned (checked by script).

### B.3 Proposed children

Folder names reuse repo emoji (`🖋️` ink, `🖼️` image, `🖱️` pointer, `📄️` document, `🔬️` introspection, `🧱️` block, `🎯️` ledger, `🧮️` table/paint). None repeats; `🗣️Interpreter/🎯️targets/🧊️wgpu/` currently holds only `🦀️.rs`.

| Child | Folder | Ranges | Active | Bytes | Consumed by |
|---|---|---|---|---|---|
| C01 | `🧮️render-plan-validator` | 91-291, 4510-4514 | 182 | 13,447 | none (leaf; test only) |
| C02 | `🧩️retained-ui-engine` | 293-1966, 4126-4132 | 1,499 | 95,763 | C03, C04, C05, C06, C08, C10 (`UI_ENGINE` 14 uses in C10) |
| C03 | `✍️text-and-table-focus` | 1967-2312, 2314-2481, 2483-2542 | 520 | 27,852 | C02, C04, C05, C08, C10 |
| C04 | `🖋️ink-and-vfs-focus` | 2544-2694, 2696-3301 | 699 | 35,612 | C02, C05, C08, C10 |
| C05 | `🖱️scene-interaction` | 3302-3779 | 440 | 27,232 | C06 |
| C06 | `📄️retained-document-consumer` | 3780-4125 | 261 | 18,998 | C02 |
| C07 | `🖼️ui-image-loading` | 4135-4509 | 345 | 17,489 | C05 |
| C08 | `🔬️introspection` | 4516-5791, 6769-6856 | 1,121 | 65,564 | C02, C09, C10 |
| C09 | `🧱️mesh-and-board2d-stats` | 5792-6006 | 182 | 11,191 | C08 |
| C10 | `🎯️chrome-ledger` | 6008-6767 | 639 | 40,414 | C02, C08, C09 |
| P | `🦀️.rs` (kept) | 1-90, 6857-6860 | 60 | 4,699 | consumed by A (`WorkerCell`, `FrameworkWidgetContext`) and by all children |

Note: the 4126-4132 test mounts sit inside the `RetainedEngineCutover` region in the file, between `RetainedDocumentConsumer` and `UiImageLoading`. Moving them to C02 keeps them with the function they exercise. The byte totals include the `cfg` lines.

### B.4 Visibility widening (cross-reference)

71 private items used from another child. Full list in Appendix A. The main groups:

- `UI_ENGINE` (static, read by 6 children) and the scene-intent statics (`SCENE_INTENTS`, `SCENE_POINTER_OWNERS`) and the intent types (`SceneIntentEvent`, `SceneInteractionIntent`) stay in C02, with `pub(super)`.
- Focus state (`FocusedTextEditor` family, table/VFS/ink focus fns) is read by C02 and C05 and by C04's clipboard path. Widen all of them.
- Introspection accessibility builders (`*_accessibility_entries`, `build_accessibility_dump`) in C08 are called from C02 (`scene_virtual_accessibility_node_id` 8 uses); widen.

### B.5 Relative paths, includes and test mounts

| Current line | Kind | Child | Text after move |
|---|---|---|---|
| 4127 | `#[path]` test | C02 | `"../../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs"` |
| 4131 | `#[path]` test | C02 | `"../../../🧪️tests/🛍️wgpu-app-catalogue/🦀️.rs"` |
| 4512 | `#[path]` test | C01 | `"../../../🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs"` |
| 6854 | `#[path]` test | C08 | `"../../../🧪️tests/🔬️wgpu-introspection/🦀️.rs"` |
| 6859 | `#[path]` test | P | unchanged `"../../🧵️worker-cell/🧪️tests/🦀️.rs"` |

No `include!` or `include_str!` in this file (the one `include` hit is in a comment). `#[cfg(test)]` gates: 34. `#[cfg(all(test, not(wasm32)))]` gates on the mounts at 4126 and 4130 move with them.

### B.6 Macro and attribute hazards

- No `macro_rules!`. `thread_local!` appears 12 times. Line 50 (P) is the test-only worker-cell thread-local. The other 11 (lines 1837-4145) hold focus and clipboard state in C02, C03, C04 and C07 and move with them. `WorkerCell` must stay in P (or in a neutral module) because target A imports it.
- The block comment at 294-319 is `/* ... */` and attaches to `static UI_ENGINE` at 320. The cut at 292-293 must keep it with the static (C02).
- `#[derive(serde::Serialize)]` on introspection types at 4532-6077 (C08, C09, C10). They move with their structs. No derive-level path changes.

### B.7 Wrong-direction dependencies

- Module cycle: Interpreter imports `crate::scenes::{queue_canvas_image_upload_sized, queue_canvas_image_upload_with, render_component_scene_step, RasterUploadRefusal}` (line 10) and calls `crate::scenes::...` 142 times. Scenes imports `crate::interpreter` 26 times. Scenes imports `engine_canvas::` 82 times. EngineCanvas imports `crate::interpreter` twice. So the renderer's three element modules form one cycle. Extract the shared contracts (`FrameworkWidgetContext`, `ScenePointerTarget`, `WorkerCell`, `RasterUploadRefusal`, the scene-intent types) into a neutral module before any child crate boundary can be drawn.
- Introspection and diagnostic exports (`#[wasm_bindgen] dump*`, `IntrospectionBuilders`, `ChromeLedger`, `*_accessibility_entries`) are test/diagnostic tooling compiled into the production element. They belong in a test-support or devtools module behind a feature.
- Concrete plugin and product names in production code: `gisMap.*` labels in the render-plan validator (lines 228-232, plugin `🗺️surface` tiled-map), `world3d` (`infinite_world::world::World3dState`, comment at 140; direct import of `infinite_world` at line 11), `layout`/`draw` mounted-layout identifiers. The `svg` raster path (`rasterize_svg_to_rgba`, 4231) is renderer-owned and acceptable, but names the `usvg`/`resvg` stack directly.

### B.8 Ordered extraction sequence

1. Baseline check as in A.9.1.
2. Leaves first: C01 (leaf, test-only mount), C09 (consumed only by C08), C10 (consumed by C08 and C02 only through 2-3 names), C07, C06, C05, C04, C03, C08, C02 last (the hub; consumed by almost everything).
3. Move the block comment (294-319) with `UI_ENGINE` (320). Do not separate them.
4. Move the test mounts with their children (section B.5); fix each `../`.
5. Keep `WorkerCell`, `test_worker_cell` and `FrameworkWidgetContext` in P, widen to `pub(crate)` (already `pub`/`pub(crate)`), and let A import them unchanged.

---

## 4. Target C: OS host (`🖥️host/🦀️.rs`, 4,671 lines)

### C.1 Owning crate and targets

- Crate: `semio-framework-os`, manifest `🖥️host/📦️packages/🦀️rust/Cargo.toml`. Lib root `🖥️host/📦️packages/🦀️rust/🦀️.rs` (`[lib] path`).
- Mount: packages root lines 40-41: `#[path = "../../🦀️.rs"] mod host_core; pub use host_core::*;`. Modules keep their paths (`crate::host`, `crate::instance`, `crate::workflow`, `crate::registry`, ...) through the glob re-export.
- Macro in packages root: `#[cfg(test)] macro_rules! test_identity` at line 11, and `mod host_core;` at line 41. The macro stays textually before `mod host_core;`, so it remains in scope for every child file declared under `host_core`. Keep the order.
- Targets: native, `wasm32-unknown-unknown` (42 wasm-gated lines) and `wasm32-wasip2` (`target_env = "p2"`, 21 gated lines: SVG raster and the store sync are excluded there).
- `#[path]` attributes inside this file: none. `include!` 8 times, `include_str!` once.

### C.2 Section table

The file is a set of inline modules. Each block becomes a file module with the same name, so visibility and module paths do not change.

| Lines | Content | Active | Goes to |
|---|---|---|---|
| 1-3 | `//!` doc, `#[cfg(test)] use semio_framework_artifact_reference::...` | 1 | P |
| 4-1365 | `pub mod host {`: `ProgramHotSwapEvent` (55), `LoadedProgram` (63), `PluginHost` (69-363), `BackboneDocument` (366-656), workflow reconcile (657-842), `OsWorkflowStore` (845-975), `OsBackbonePort` (976-1031), `OsSpaceCatalogEntry` (1060-1255), extension install (1257-1306), `PluginHost` extension impl (1307-1361), `include!` host-unit test (1363) | 990 | C01 |
| 1367-1533 | `pub mod backbone {`: `SpaceBackbonePort` (1402-1523), `open_folder_space_backbone` (1524), `open_file_space_backbone` (1529), `decode_os_space_pack_payload` (1498) | 130 | C02 |
| 1536-1990 | `pub mod instance {`: `OsParameter*` types (1581-1618), `create_os_id` (1653), parameter patching (1659-1906), `build_configure_config` (1916), `materialize_os_app_instance_document_json` (1951), `include!` instance-unit test (1987) | 336 | C03 |
| 1992-2212 | `pub mod media_export_raster {`: `rasterize_svg_to_png_base64` (2010), `svg_to_polylines` (2045), `register_2d_export_handlers` (2150), `register_mesh_exporter/importer` (2163-2177), `include!` wasip2 (2186) and unit (2189) tests | 162 | C04 |
| 2214-2296 | `pub mod media_export_simple {`: `wrap_svg` (2225), `title_card_svg` (2231), `pages_rects_svg` (2238), `map_points_svg` (2259) | 70 | C05 |
| 2298-3116 | `pub mod workflow {`: `negotiate_media_contract` (2344), `negotiate_wire_format` (2366), `registry_shared_stdio_dialect` (2390), `validate_workflow` (2408), `OsWorkflowNodeGraphPayload` (2560), `OsWorkflowChannelSpec`/`OperatorInfo` (2646-2661), media export and import registries (2779-3101), `include!` tests (2583, 3113) | 574 | C06 |
| 3118-4131 | `pub mod codec_abi {`: schema `include_str!` (3122), `OsHostCodecOperation` (3131), error codes (3159), `OsHostCodecSession` (3644-3937), `RetainedOsHostCodecService` (3967-4070), `OsHostCodecService` (4070-4114), `include!` codec-abi test (4129) | 920 | C07 |
| 4133-4636 | `pub mod registry {`: `OsArtifactDescriptor` (4154), `seed_builtin_artifact_kinds` (4235-4324), `register_artifact_descriptors` (4329), `OsAppRegistration` (4406), `register_app_io` (4443), `resolve_os_app_definition` (4503), `workflow_palette` (4579), `PluginRegistry` (4602-4631), `include!` registry test (4633) | 362 | C08 |
| 4638-4671 | Crate-root re-exports (`pub use crate::space::*`, `crate::workflow::{...}`, `host::{...}`, `registry::{...}`, `semio_framework::*`, `ui_wgpu::wgpu::*`, `vcs::{...}`) | 34 | P |

Total active: 3,579. Largest child 990 active lines and 82,675 bytes.

### C.3 Proposed children

| Child | Folder (beside the file) | Source module | Active | Bytes |
|---|---|---|---|---|
| C01 | `🔌️plugin-host` | `host` | 990 | 82,675 |
| C02 | `🗃️document-backbone` | `backbone` | 130 | 10,513 |
| C03 | `🧱️instance` | `instance` | 336 | 23,487 |
| C04 | `🖼️media-export-raster` | `media_export_raster` | 162 | 13,590 |
| C05 | `🧾️media-export-simple` | `media_export_simple` | 70 | 4,352 |
| C06 | `🔀️workflow` | `workflow` | 574 | 55,668 |
| C07 | `📡️codec-abi` | `codec_abi` | 920 | 44,136 |
| C08 | `🗂️registry` | `registry` | 362 | 27,508 |
| P | `🖥️host/🦀️.rs` | (kept) | 35 | 4,086 |

Existing sibling folders `🎠️activation`, `💾️persistence`, `📦️packages`, `🧩️component`, `🧪️tests`, `🧫️fixtures`, `🧬️schema` do not collide with these emoji. The child folder names are the existing emoji-plus-kebab convention. Full path for the deepest child is about 80 characters.

Wiring in `📦️packages/🦀️rust/🦀️.rs`: no change (`host_core` stays). Inside `🖥️host/🦀️.rs`, replace each `pub mod X { ... }` by `#[path = "<child>/🦀️.rs"] pub mod X;` and keep the existing `pub use` block (4638-4671) unchanged.

### C.4 Visibility

No widening. Each module keeps its privacy because each block already was a module. Private items inside a block are only visible to that block (as before). Items used across blocks already use `crate::x::...` paths.

### C.5 Relative paths and includes

Each child file is one folder deeper than the host file, so `include!` and `include_str!` paths gain `../`:

| Current line | Child | Current | After |
|---|---|---|---|
| 1363 | C01 | `include!("🧪️tests/🔬️host-unit/🦀️.rs")` | `include!("../🧪️tests/🔬️host-unit/🦀️.rs")` |
| 1987 | C03 | `include!("🧪️tests/🔬️instance-unit/🦀️.rs")` | `include!("../🧪️tests/🔬️instance-unit/🦀️.rs")` |
| 2186 | C04 | `include!("🧪️tests/🔬️media-export-raster-wasip2/🦀️.rs")` | `include!("../🧪️tests/🔬️media-export-raster-wasip2/🦀️.rs")` |
| 2189 | C04 | `include!("🧪️tests/🔬️media-export-raster-unit/🦀️.rs")` | `include!("../🧪️tests/🔬️media-export-raster-unit/🦀️.rs")` |
| 2583 | C06 | `include!("🧪️tests/🔬️workflow-standalone/🦀️.rs")` | `include!("../🧪️tests/🔬️workflow-standalone/🦀️.rs")` |
| 3113 | C06 | `include!("🧪️tests/🔬️workflow-unit/🦀️.rs")` | `include!("../🧪️tests/🔬️workflow-unit/🦀️.rs")` |
| 3122 | C07 | `include_str!("🧬️schema/🔣️.json")` | `include_str!("../🧬️schema/🔣️.json")` |
| 4129 | C07 | `include!("🧪️tests/🔬️codec-abi-unit/🦀️.rs")` | `include!("../🧪️tests/🔬️codec-abi-unit/🦀️.rs")` |
| 4633 | C08 | `include!("🧪️tests/🔬️registry-unit/🦀️.rs")` | `include!("../🧪️tests/🔬️registry-unit/🦀️.rs")` |

The `#[cfg(test)]` gates travel with each block. The test files are fed through `include!`, so they keep the crate scope of their child. Child-level `//!` doc is the existing inner doc of the inline `host` block (line 5).

### C.6 Wrong-direction dependencies

- Product hard-wires artifact kinds of other plugins: `seed_builtin_artifact_kinds` (4235-4324) seeds `space`, `workflow`, `collection`, `parameter.value` and `data`; it imports `crate::space`, `crate::workflow` (lines 10-11 of `host`) and the artifact crates `semio_framework_artifact_workflow_workflow` and `semio_framework_artifact_space_space` (Cargo.toml). The doc comments name plugin kinds `cad`, `puzzle3d`, `gismap` (2733-2743).
- Product owns a native SVG rasterizer: `resvg`, `tiny-skia`, `usvg` are native dependencies of this crate (Cargo.toml) and `rasterize_svg_to_png_base64` lives in `media_export_raster` (2010). The rasterizer belongs in a renderer-side or media plugin.
- Legacy path present: `registry_export_media_legacy` (2941) and `registry_import_media_legacy` (3053). The repo rule (AGENTS.md) forbids legacy paths; both should be removed in the split change, with callers updated at once.
- Hard-coded product identifiers: `SPACE_FOLDER_DOCUMENT_ID = "studio"` (1388), `OS_SPACE_BACKBONE_URI_PREFIX = "space://"` (1058). These are product defaults; keep them out of framework modules.
- The root policy file allowlists a `block_on` in this file at `📜️script.ts:9047`, keyed on `lineHint 1985`, which falls in the `instance` module (1536-1990). The key must move to the child that contains the call. See section 6.3.

### C.7 Ordered extraction sequence

1. Baseline check (repo gate, package `semio-framework-os`, `--tests`, native and wasm32-unknown-unknown and wasm32-wasip2 variants).
2. Extract the leaves first (C05, C02, C04, C03), then C08, C06, C07, C01. Each block is moved verbatim; only the `include!` paths change.
3. Re-verify the packages-root macro `test_identity` remains before `mod host_core;`.
4. Update the root-policy allowlist key (6.3) in the same change.

---

## 5. Target D: MCP workspace (`🏠️workspace/🦀️.rs`, 5,831 lines)

### D.1 Owning crate and targets

- Crate: `semio-framework-os-mcp` (role `product`, id `os-mcp`), manifest `🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml`, lib root `📦️packages/🦀️rust/🦀️.rs`.
- Mount: lib root lines 85-86: `#[path = "../../🏠️workspace/🦀️.rs"] pub mod workspace;`. Module path `crate::workspace`. The binary `semio-os-mcp` (`🏗️bootstrap/⌨️entrypoint/🦀️.rs`) depends on this crate.
- Targets: native (most items gated `cfg(not(target_arch = "wasm32"))`, about 170 gate lines) and `wasm32` (only the ungated items: plugin paths, probe document, binding). Not `wasi`-specific.

### D.2 Section table

| Lines | Content (line of item) | Goes to |
|---|---|---|
| 1-14 | `//!` module doc | P |
| 16-18 | `#[cfg(test)] #[path = "../../../../../🔨️modules/🏃️process/🧭️routing/🏠️workspace/🦀️.rs"] mod workspace_identity;` | P (path unchanged) |
| 21-23 | `is_repository_root` (21) | P |
| 25-49 | `use` block (crate, `semio_framework_plugin_host`, os-kernel `os_directory`, std) | P |
| 51-56 | `#[path = "🔗️remote/🦀️.rs"] pub mod remote;` and remote `use` (54-56) | P (path unchanged) |
| 61-65 | `now_ms` (61), `workspace_worker_pool` (65) | P |
| 70-138 | `//#region PluginPaths`: `PluginRegistryEntry` (73), `find_repo_root` (91), `load_plugin_registry` (110), `plugin_registry_entries` (115), `find_plugin_entry` (124), `resolve_plugin_wasm_path` (130) | C01 |
| 140-156 | `//#region Descriptor`: `load_package_descriptor` (151) | C01 |
| 158-446 | `//#region ProbeDocument`: `PROBE_SCHEMA` (166), `ProbeSnapshot` (175), `probe_json` mount (181-182), `probe_sqlite` mount (184-185), `impl` blocks for ArtifactDsl/Pack/Store (187-233), `ProbeDiff` (253), `ProbeMutation` (297-384), `ProbeStore` (385), `ensure_probe_codec_registered` (391), `probe_store_owners` (408), `close_probe_store_to_terminal` (423) | C02 |
| 448-497 | `//#region Binding`: `WorkspaceOrigin` (454-512) | C03 |
| 499-564 | `//#region PluginActivation`: `PluginActivationOutcome` (513), `activate_plugin_instance` (532) | C03 |
| 566-902 | `//#region ArtifactChannel` head: `PluginArtifactChannel` (584), `PluginInferenceRoute` (643), `PendingExchange` (648), `PendingResponsePage` (675-753), `headless_agent_ops` (754), `PendingExchangeRegistry` (784-830), inference and open/command budget constants (831-920) | C05 |
| 903-1300 | `shared_plugin_runtime` (921; its doc block starts at 903), component caches (941-1050), `ComponentBytes`, `activation_fault`, `scoped_compiled_component` (1001-1050), compile worker (1102-1141), `//#region CompileFlight` (1151-1299) | C04 |
| 1301-2145 | `impl PluginArtifactChannel` (1307-2145, cfg not wasm32) | C05 |
| 2146-2744 | App-frame helpers (2146-2588; `app_command_seq_mut` doc block starts at 2146): `app_command_seq_mut` (2154), frame tags (2211-2259), `shell_app_frame_payload` (2321), `document_backbone_payload` (2337), `effect_shape` (2365), `CommandTurn` (2431), `decode_guest_fault` (2455), `bind_inference_document` (2552); `impl ArtifactChannel for PluginArtifactChannel` (2589-2744) | C06 |
| 2745-2906 | `distinct_plugin_ids` (2754; doc block starts at 2745), `AppRoute` (2766), route helpers (2776-2906) | C07 |
| 2907-3293 | `//#region GuestDocumentCodec`: `guest_codec_routes` (3010), `guest_print_mirror`/`guest_apply_ops_binary`/`guest_replay_envelopes` (3028-3072), `guest_compile_dsl`, `guest_edit_text_from_envelope` (3102-3107), `guest_sqlite_*` (3114-3255), `#[cfg(test)] #[path="🪶️sqlite/🧪️tests/guest-input-slot.rs"] mod guest_sqlite_original;` (3168-3170) | C08 |
| 3294-3718 | `PluginComponentSource` (3303), `HubPluginComponents` (3315), `WorkspaceCatalog` (3348), `RoutingArtifactChannel` (3409-3718, `impl ArtifactChannel` 3626) | C07 |
| 3751-3808 | `//#region ShellRouted`: `ShellRoutedArtifactChannel` (3760-3808) | C07 |
| 3809-3810 | `//#endregion ArtifactChannel` and blank | P (marker) |
| 3811-3863 | `//#region HeadlessWorkspace`: `HeadlessWorkspace` struct (3813) | C10 |
| 3864-4215 | `InstalledArtifactKind` (3867; doc block starts at 3864), `SessionDocumentPair` (3880), `HubRelay` (3898), `HubInbound*` (3909-3950), `plan_hub_inbound` (3950), hub edit refusal (3992), hub link helpers (4037-4100), `HubRelay` impl (4103-4200) | C09 |
| 4216-4296 | `PluginArtifactBinding` (4217-4271; derive at 4216), `impl Drop for HeadlessWorkspace` (4273-4296) | C10 |
| 4297-5452 | `impl HeadlessWorkspace` (4297-5201), `impl GatewayBackend for HeadlessWorkspace` (5202-5412), `impl GatewayBackend for Arc<HeadlessWorkspace>` (5413-5452) | C11 |
| 5453-5608 | `impl HeadlessWorkspace` (5453-5608), `directory_json_value` (5590), `base64_encode` (5595) | C10 |
| 5609-5611 | `//#endregion HeadlessWorkspace` and blank | P (marker) |
| 5612-5810 | `//#region Tests` wrapping `//#region Inference`: `use DocumentScope` (5613), `impl HeadlessWorkspace` inference facade (5619-5810). Production code under a Tests region | C12 |
| 5811-5817 | blank and region marker | P |
| 5818-5820 | `#[cfg(test)] #[path = "🧪️tests/🔬️quick/🦀️.rs"] mod quick;` | P (path unchanged) |
| 5827-5830 | `#[cfg(test)] #[path = "🧪️tests/🔬️long/🦀️.rs"] mod long;` | P (path unchanged) |
| 5831 | `//#endregion Tests` | P |

Child totals (section D.3): P 48, C01 53, C02 209, C03 76, C04 289, C05 753, C06 398, C07 454, C08 281, C09 243, C10 193, C11 813, C12 168; total 3,978 active lines (checked by script).

### D.3 Proposed children

Sibling folders in `🏠️workspace/` today: `🔗️remote`, `🚪️io`, `🧪️tests`, `🧫️fixtures`, `🧬️schema`, `🪶️sqlite`. Proposed names avoid all of them, and the emoji differ from each other.

| Child | Folder | Ranges | Active | Bytes | Used by |
|---|---|---|---|---|---|
| C01 | `🧭️plugin-paths-and-descriptor` | 70-156 | 53 | 5,946 | pub items only |
| C02 | `🔬️probe-document` | 158-446 | 209 | 17,045 | C03, C10, C11 |
| C03 | `🔖️plugin-binding-and-activation` | 448-564 | 76 | 7,020 | none (leaf) |
| C04 | `🧊️compiled-components` | 903-1300 | 289 | 22,734 | C03, C05, C07, C08, C11 |
| C05 | `📨️plugin-artifact-channel` | 566-902, 1301-2145 | 753 | 80,507 | C03, C06, C08 |
| C06 | `🧩️artifact-app-frames` | 2146-2744 | 398 | 39,381 | C05 |
| C07 | `🔀️routing-and-shell-routed` | 2745-2906, 3294-3808 | 454 | 42,777 | C11 |
| C08 | `🗂️guest-document-codec` | 2907-3293 | 281 | 30,567 | C05, C11 |
| C09 | `🤝️hub-relay` | 3864-4215 | 243 | 19,758 | C07 |
| C10 | `🪪️headless-workspace-state` | 3811-3863, 4216-4296, 5453-5608 | 193 | 20,439 | C11 |
| C11 | `🛂️gateway-backend` | 4297-5452 | 813 | 81,120 | none (leaf) |
| C12 | `💡️inference-facade` | 5612-5810 | 168 | 11,810 | C11 |
| P | `🦀️.rs` (kept) | 1-69, 3809-3810, 5609-5611, 5811-5831 | 48 | 5,821 | (wiring) |

Largest child by active lines and by bytes: C11 (813 lines, 81,120 bytes). Every child is well under the 262,144-byte gate; the whole file (384,928 bytes) is above it today.

### D.4 Visibility widening (cross-reference)

41 private items used from another child. Full list in Appendix A. Notable:

- `is_repository_root` (P, 21) used by C01 and C11: `pub(super)`.
- `now_ms` (P, 61) used by C07, C11, C12; `workspace_worker_pool` (P, 65) used by C11: `pub(super)`.
- `shared_plugin_runtime` (C04) used by C05, C07, C08, C11; `activation_fault` (C04) used by C11 4 times.
- App-frame helpers (C06) used by C05 (CommandTurn 11 times, decode_guest_fault 4 times): widen.
- `directory_json_value` (C10) used by C11 twice: widen.
- `hub_socket_reactor` (C08) used by C11: widen. `register_guest_document_codec` (C08) used by C11.

### D.5 Relative paths and test mounts

| Current line | Kind | Child | Text after move |
|---|---|---|---|
| 181 | `#[path]` mod `probe_json` | C02 | `"../🚪️io/🔤️json/🦀️.rs"` |
| 184 | `#[path]` mod `probe_sqlite` | C02 | `"../🪶️sqlite/🦀️.rs"` |
| 3169 | `#[path]` mod `guest_sqlite_original` (cfg test) | C08 | `"../🪶️sqlite/🧪️tests/guest-input-slot.rs"` |
| 16-18 | `#[path]` mod `workspace_identity` (cfg test) | P | unchanged |
| 51-52 | `#[path]` mod `remote` | P | unchanged |
| 5818-5820 | `#[path]` mod `quick` (cfg test) | P | unchanged |
| 5827-5830 | `#[path]` mod `long` (cfg test) | P | unchanged |

`probe_json::` and `probe_sqlite::` are used only inside the ProbeDocument region (lines 191-347), so they move with C02 without cross-child references. No `include!` or `include_str!` in this file.

Cut placement: each cut lands on the first line of an item's attribute and doc block, so no `#[cfg]`, `#[derive]` or `///` line is separated from its item. Verified cuts: C05 ends at 902 (the `const` above the doc of `shared_plugin_runtime`, which starts at 903); `app_command_seq_mut` starts at 2146 (doc block); `distinct_plugin_ids` starts at 2745 (doc block); `InstalledArtifactKind` starts at 3864 (doc block); `PluginArtifactBinding` starts at 4216 (derive). A script confirmed that no chunk begins or ends on an attribute or doc line that belongs to the other side. The `cfg(not(target_arch = "wasm32"))` gates travel with their items.

### D.6 Macro hazards

- No `macro_rules!`. `dyn_enum_close` is imported (line 34) but not invoked here.
- `block_on` is used 31 times (for example 548, 559, 924, 976, 1076). These are sync calls into async (`semio_framework_async::block_on`). The root policy file allowlists `block_on` for other files; it does not allowlist this file. Keep the `use`s in P so children see them through `use super::{...}`.

### D.7 Wrong-direction dependencies

- Product logic depends on the plugin host directly: `semio_framework_plugin_host::{GuestRuntime, GuestRuntimes, SharedEngineConfig, WasmtimeRuntime}` (line 39) and `store::SpaceMember`. The product layer may depend on the host; the issue is that the probe document (`ProbeSnapshot`, `ProbeStore`, `ProbeMutation`) is a test fixture living in production code (C02, 158-446) and is wired into `ensure_probe_codec_registered`, which runs in production. Move the probe behind `#[cfg(test)]` or into a test-support crate.
- Plugin names in doc comments and identifiers: `note` (`🗒️note`, module doc at line 9, and a doc comment near line 5827), `wfc` (`🀄️wfc`, 961 and 2546 docs), `lowpoly` (2494 doc), `block` (2760 doc). All are doc-only, no code coupling. Keep docs generic or name the plugin in a fixture.
- `HeadlessWorkspace` implements `GatewayBackend` (5202-5452) and an inference facade (5612-5810) in the same type, so a product crate owns both the gateway and the inference client. Split the inference facade to its own child (C12, proposed) and keep the trait impl in C11.
- Test under production: the `Tests` region (5611) wraps production inference code. Rename the region when the file is split.

### D.8 Ordered extraction sequence

1. Baseline (repo gate, package `semio-framework-os-mcp`, `--tests`, native and wasm32 variants).
2. Leaves first: C12, C03, C01, C09, C10, C06, C08, C07, C02, C04, C05, C11. Each cut uses anchors (`//#region ...` and the item lines in D.2).
3. Re-verify each `#[cfg]` travels with its item.
4. Move the probe mounts (181-185) with C02 and rewrite them (D.5). Move 3169-3170 with C08.
5. Update `📜️script.ts:6271` (`policyReadFileSafe(root, mcp workspace)`) to a module-tree read in the same change (section 6.3).

---

## 6. Cross-target findings

### 6.1 Renderer element cycle (A, B, Scenes)

| From | To | References |
|---|---|---|
| A (EngineCanvas) | B (Interpreter) | 2 (`crate::interpreter::{FrameworkWidgetContext, ScenePointerTarget}`, `crate::interpreter::WorkerCell`) |
| B (Interpreter) | A (EngineCanvas) | 24 (`engine_canvas::...`) |
| B (Interpreter) | Scenes | 142 (`crate::scenes::...`) |
| Scenes | B (Interpreter) | 26 (`crate::interpreter`) |
| Scenes | A (EngineCanvas) | 82 (`engine_canvas::...`) |
| A (EngineCanvas) | Scenes | 0 |

The three element modules form a cycle. Splitting each file into children does not remove the cycle. The shared contracts (`WorkerCell`, `FrameworkWidgetContext`, `ScenePointerTarget`, `RasterUploadRefusal`, scene-intent types) must move into a neutral module first. This is the largest structural finding of this audit and the precondition for any cross-crate boundary inside the renderer.

Scenes has already split its own wgpu monolith (children listed in the summary). The node-graph and tiled-map children are named `🧭️node-graph-and-tiled-map`, the board children `🧩️board2d-and-vfs`, the text editor `✍️text-editor`, the table and paint `🧮️table-and-paint2d`. Use the same emoji for the same concept when naming A and B children, unless that emoji is already taken in the same folder.

### 6.2 Visibility and test mount rules (all four)

- Widen only the names in Appendix A to `pub(super)`. Do not widen to `pub` (the external API of each file is kept through explicit re-exports).
- Use `use super::{...};` with explicit name lists generated by the cross-reference, not `use super::*;` (executor rule). A glob would also hide the ambiguity this audit is designed to catch.
- Glob re-export from a parent module (`pub use child::*`) is valid only for names that are `pub` at definition. `pub use` of a `pub(super)` or `pub(crate)` item fails (E0364). Use plain `use` for those.

### 6.3 Repo-policy readers that must change in the same change

Line numbers in `📜️script.ts` were read at audit time. They move while other agents edit that file, so search by identifier before editing.

Found in `📜️script.ts` and in the repo policy library. These treat each file as a single source file or key a rule on a file path and a line number. After the split they would silently miss the moved code.

1. Split-root registry: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` lines 110-125 define `POLICY_SPLIT_SOURCE_ROOTS`, which `policyReadRustPolicySource` uses to decide whether to read the module tree. It lists the renderer (`🧊️renderer/🦀️.rs`), Scenes and Shell wgpu parents, and the plugin host (`🖥️host/🦀️.rs` under `🔌️plugin`), but none of these four parents. Add the four new parent paths:
   - A: `${OS_ENGINE}/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
   - B: `${OS_ENGINE}/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
   - C: `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs` (not the plugin-host path above)
   - D: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
2. `📜️script.ts:8967`: `INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE` is read with `policyReadFileSafe` at 8111 (single file) and `policyReadRustPolicySource` at 9310 (uses the split registry in item 1).
3. `📜️script.ts:8952`: `INTERACTIVITY_AUDIT_INTERPRETER_RASTER_FILE` is read with `policyReadRustPolicySource` at 9255 and `policyReadFileSafe` at 9301 (`mountedInterpreter`).
4. `📜️script.ts:6271`: `policyReadFileSafe(root, mcp workspace)` for D. Change to `policyReadModuleTreeSource`.
5. `📜️script.ts:6270`: `policyReadModuleTreeSource(root, plugin host)` (not ours) is the correct pattern; use it for C.
6. Allowlist key: `📜️script.ts:9047` `file: "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs", lineHint: 1985, pattern: "block_on"`. The `lineHint` is invalidated by the split. Re-key to `🧱️instance` (the module that contains line 1985), and verify at the time of the cut.
7. Other `policyReadModuleTreeSource` callers already exist for the other split roots (e.g. `📜️script.ts:5523` for the Shell parent). Mirror them.

### 6.4 Source-text readers (repo-wide, `/usr/bin/grep -rn --exclude-dir=node_modules --exclude-dir=.🧬semio -F`)

Full list in Appendix B. Summary by kind:

- Code readers that load the parent file as text and assert on content (must become a module-tree read): 
  - A: `🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs:63` (`include_str!`, `ENGINE_CANVAS_SOURCE`); `🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:11` (`include_str!`); `🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts:10` (`?raw` import); `🧪️tests/🫳️node-graph-gestures/🟦️.ts:30`; `🧪️tests/🧪️node-graph-edit-rows/🟦️.ts:58`; `🧪️tests/🖼️wgpu-raster-residency/🟦️.ts:181`; `🧪️tests/🔗️node-graph-wire-edit/🟦️.ts:118,144`; `🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:24-25` (asserts a substring of the file).
  - B: `🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs:62` (`INTERPRETER_SOURCE`, `include_str!`); `🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts:11` (`?raw`); `🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts:22`; `🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🟦️.ts:23`; `🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts:15`; `🔨️modules/🖱️ui/🧪️tests/♻️physical-job-close/🟦️.ts:36` (readFileSync of the Interpreter file; `:73` already uses `readModuleTreeSource` for the renderer tree).
  - C: `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:849` (`readFileSync` of the host); `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:318` (`readSources` of the host and its test dir).
  - D: `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts:208` and `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts:398` (readFileSync of the workspace file); `🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🟦️.ts:85` (`Bun.file`, reads the probe).
- Fixtures that embed the path (data, regenerate): `🔨️modules/🖼️canvas/🧪️tests/🧫️fixtures/🏷️label-fit/🔣️.json:9` (A); `...🧪️tests/🧫️fixtures/🖼️wgpu-raster-witness/🔣️.json:6` (A, key `engineCanvasSource`); `...🧫️fixtures/🖌️wgpu-document-owner-move/🔣️.json:4` and `🌳️wgpu-document-reconcile/🔣️.json:7` (B, `interpreterSource`); `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🔣️.json:75` and `🧪️tests/🧫️fixtures/🖊️dwg-artifact-ownership/🔣️.json:12` (C); `✏️s/🔌️plugins/🗄️stdio/.../🔮️oracles/🔣️.json:24,56` (C).
- Build inputs and fixtures that enumerate D's files (update in the same change): `🌎️hub/📦️packages/🦀️rust/📋️project.json:65` lists `🏠️workspace/🦀️.rs` and `🏠️workspace/🔗️remote/🦀️.rs` in an nx input glob, and `🌎️hub/🧪️tests/🧫️fixtures/🧱️foundation-source/🔣️.json:371` repeats the same brace list. Add the new child files to both, or nx input hashing and the foundation-source check will miss them.
- Distribution manifest entries: `🧑‍💻dev/📤️distribution/🧾️manifest.json:10937` (A), `:11157` (B). Regenerate after the split (the manifest lists the parent file path).
- Doc or comment-only references (no code change, may be updated for accuracy): `✏️s/🔌️plugins/🧩️puzzle/.../⚙️engine/🦀️.rs:20` (puzzle plugin doc naming the renderer file), `🌎️hub/🧩️compositions/🪐️space/⚙️engine/.../🦀️.rs:23` (space engine doc naming the host file); A: `Board2dHost/🟦️.tsx:550`, `wgpu-paint2d-engine/🦀️.rs:49`, `frame-worker/🟦️.ts:95`, `Cargo.toml:76`; B: `TaskManager/🟦️.tsx:11,14`, `time-travel/🟦️.ts:568`, `ShellHost/🎯️input-ledger/🟦️.ts:231`, `browser-frame-transport/🟦️.ts:316`, `parity/🏗️structure/🟦️.ts:130`; C: `✏️s/.../🚪️io/🦀️.rs:16,913` (doc); D: `🌉️mcp/🟦️.ts:354,781` (doc).
- Mounts that keep pointing at the parent (unchanged): `🧊️renderer/🦀️.rs:92-96` for A and B; `📦️packages/🦀️rust/🦀️.rs:40-41` for C; `📦️packages/🦀️rust/🦀️.rs:85-86` for D.

Coverage note: `🧰️framework`, `✏️s`, `🌎️hub` and `📜️script.ts` were searched with the tails named in the appendix. For C and D the tails were `💻️os/🖥️host/🦀️.rs` and `🏠️workspace/🦀️.rs`, which do not match relative doc references such as `🖥️host/🦀️.rs` written from another folder, so those relative references were not searched in `🧰️framework`. `✏️s` (4.1 GB) and `🌎️hub` (11 GB) were searched with source-type filters only (ts, tsx, rs, json, toml, md, mjs), so binary and other types were not searched. `📜️script.ts` was searched in full.

### 6.5 Legacy and naming

- Legacy in C: `registry_export_media_legacy` (2941), `registry_import_media_legacy` (3053) (AGENTS.md: no legacy support). Remove in the split change with callers updated.
- Node-graph naming in A: the node graph is called `flow`, `dag` and `Graph` in different places. Keep `🕸️node-graph` for the child and rename identifiers only in a later pass.

---

## 7. Verification (to be run by the executor; not run here)

For each target, after each cut (compile-atomic):

- `cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-renderer-wgpu --message-format=short` (A and B), and the `--tests` and `--target wasm32-unknown-unknown` variants. Package `semio-framework-os` for C, `semio-framework-os-mcp` for D (native and `wasm32` for D).
- Per the executor rules: `📓️RESOLUTION-ORACLE.md` full-resolution differential (`--emit=metadata -o /dev/null`) from the crate lib root against a sandbox holding the original; line-multiset losslessness; rustfmt parsing; path resolution.
- Child active lines ≤ 2,500 and child bytes ≤ 262,144: all proposed children pass (largest: A C09 1,180 / 72,333 bytes; B C02 1,499 / 95,763; C C01 990 / 82,675; D C11 813 / 81,120).
- Reader checks after the change: the test readers in section 6.4 must pass; the repo policy audit must run with the four parents registered in `POLICY_SPLIT_SOURCE_ROOTS`.
- Nothing above was compiled or run. These are the checks the executor must run and report.

## 8. Hazards (consolidated)

1. Cut order: extracting top-down invalidates later ranges. Use the leaf-first order from each section and re-locate every anchor by text before cutting.
2. Each cut must keep its `#[cfg]` and doc attributes with the item (blank-line boundaries were verified, see D.5).
3. Relative `#[path]` and `include!` paths gain one `../` in each child (A.6, B.5, C.5, D.5). A missed one fails only at compile time.
4. Private methods called across children fail with E0624 until widened (blanket rule, section A.5).
5. Struct fields of moved structs accessed across children need `pub(super)` on the field (apply the blanket rule to fields of each moved struct; not enumerated here).
6. Glob cycles: parent re-exports children and children import the parent. Legal, but any name defined in two children is ambiguous. The cross-reference found no duplicate top-level name across children in any of the four plans.
7. `pub use` of a `pub(crate)` or `pub(super)` item fails (E0364). Use plain `use` for those.
8. The static `ENGINE_SURFACES` (A), `UI_ENGINE` (B) and `SCENE_INTENTS` (B) are per-thread worker cells: moving them to another module must not duplicate the instance.
9. `WorkerCell` must not be duplicated. A and Scenes import it from B; keep one definition.
10. `#[wasm_bindgen(js_name = ...)]` exports keep their names (A 6946, B 6780-6846). `pub` stays `pub`.
11. Reader and allowlist keys: section 6.3. The policy keys (`file` plus `lineHint`) must be re-keyed in the same change.
12. Docs with stale `file:line` references across the repo (for example `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2801` in `wgpu-paint2d-engine/🦀️.rs:49`) are not code, but become wrong. Update the ones that name a function.

## 9. Target outcome

- A: parent ~26 active lines (imports and two test mounts); nine children, 158 to 1,180 active lines. Crate-level: the file leaves the 262,144-byte gate range.
- B: parent ~60 active lines (imports, `WorkerCell`); ten children, 182 to 1,499 active lines. The cycle in section 6.1 must be broken first to allow an independent crate boundary.
- C: parent ~35 active lines (crate-root re-exports and the `use` for the `cfg(test)` identity); eight inline modules become files, 70 to 990 active lines. The product-level legacy functions and hard-coded artifact kinds (C.6) should be removed in the same change.
- D: parent ~48 active lines; twelve children, 53 to 813 active lines. The probe fixture leaves production code (D.7).

---

## Appendix A: visibility lists (names used across children, by defining child)

### A (EngineCanvas), 31 items

- C01 (18): `NodeGraphEngine NodeGraphSyncCache EngineSurface retire_string_page EngineSurfaceId EngineSurfaceIdentity EngineSurfaceSnapshot EngineSurfaceLiveFreshness EngineSurfaceRegistry MapSyncCache BoardSyncCache RasterSyncCache MAP_TILE_ASSET_FAULT ENGINE_SURFACES ensure_engine_surface engine_surface_wire_id engine_surface_live_freshness create_target_texture`
- C02 (2): `StagedEngineScene STAGED_ENGINE_SCENES`
- C04 (1): `engine_camera_from_json`
- C05 (5): `engine_canvas_fault_log engine_canvas_debug_log PublishedGraphInteraction log_graph_geometry_census published_graph_interaction`
- C06 (1): `reserve_map_tile_fetches`
- C07 (1): `board2d_granularity_by_id`
- C08 (1): `write_interaction_hover`
- C09 (2): `synchronize_text_editor_host TextEditorDeliveryState`

### B (Interpreter), 71 items

- C02 (11): `UI_ENGINE SceneIntentEvent SceneInteractionIntent SCENE_INTENTS SCENE_POINTER_OWNERS apply_presented_scene_caret validate_clipboard_source submit_clipboard_io deliver_clipboard_recipient pump_clipboard_io_one write_owned_os_clipboard`
- C03 (18): `FocusedTextEditor text_editor_focus_in install_text_editor_focus reset_text_editor_caret blur_text_editor focused_text_editor focused_text_editor_matches with_live_text_editor apply_text_editor_text_at FocusedNodeGraphCaret clear_node_graph_caret focused_table_editable_text_address focused_table_editable_text_draft set_focused_table_editable_text_draft commit_focused_table_editable_text_with_engine clear_focused_table_editable_text focus_table_editable_text focus_table_stepper`
- C04 (14): `focus_vfs_control focused_vfs_control focus_ink_editor InkClipboardAddress focus_ink_surface PendingNativeInkClipboardToken PENDING_NATIVE_INK_CLIPBOARD complete_pending_native_ink_clipboard PendingNativeTextEditorClipboardToken PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD complete_pending_native_text_editor_clipboard queue_text_editor_clipboard_read close_window_focus_clipboard_one close_retiring_focus_clipboard_one`
- C05 (3): `FrameworkSceneHost layout_step_is_window_work drive_mounted_layout_text_one`
- C06 (1): `document_debug_log`
- C07 (1): `render_ui_image_step`
- C08 (15): `DumpMesh DumpMeshInstance DumpMeshSurface DumpMeshStats dump_window_ids scene_virtual_accessibility_node_id table_stepper_accessibility_entries table_editable_text_accessibility_entries table_button_accessibility_entries vfs_accessibility_entries block_list_accessibility_entries event_feed_accessibility_entries graph_timeline_accessibility_entries text_editor_accessibility_entries build_accessibility_dump`
- C09 (2): `build_mesh_stats build_board2d_dump`
- C10 (6): `CHROME_LEDGER world_camera_ledger_row CHROME_ACCESSIBILITY ACCESSIBILITY_VISIBLE_WINDOWS accessibility_window_is_visible project_chrome_dump`

### C (OS host)

No items: each block was already a module, so privacy does not change.

### D (MCP workspace), 41 items

- P (3): `is_repository_root now_ms workspace_worker_pool`
- C02 (3): `PROBE_SURFACE_ID probe_store_owners close_probe_store_to_terminal`
- C04 (4): `shared_plugin_runtime ComponentBytes activation_fault scoped_compiled_component`
- C05 (3): `headless_agent_ops headless_open_budget shared_compiled_component`
- C06 (16): `app_command_seq_mut app_frame_reply_seq app_frame_is_transaction_terminal app_frame_tag ingress_fault_text shell_app_frame_payload document_backbone_payload shell_lane_fault effect_shape named_shapes CommandTurn persistent_command_completion_port_ready decode_guest_fault app_media_out_ports child_head_dependencies bind_inference_document`
- C07 (6): `distinct_plugin_ids plugin_instance_slot app_instance_slot resolve_plugin_for_capability_in open_plugin_artifact_channel open_plugin_artifact_channel_scoped`
- C08 (3): `DOCUMENT_BACKBONE_BINDING_GENERATION hub_socket_reactor register_guest_document_codec`
- C09 (2): `HubInboundDelivery commits_with_nothing_relayed`
- C10 (1): `directory_json_value`

Cross-check line numbers in each row against section 2, 3 and 5 before editing.

## Appendix B: source-text readers (verified this audit)

Verified by the framework-wide search (`🧰️framework`, A and B complete under their tail; C and D partial, see 6.4), and by the root script search (`📜️script.ts`, complete for all four tails):

- A: 19 hits in `🧰️framework` (see 6.4 for code readers, fixtures, manifest and comments); `📜️script.ts:8967` (constant) and `:9310`/`:8111` (readers).
- B: 22 hits across `🧰️framework`, `✏️s`, `🌎️hub` and `📜️script.ts` (see 6.4); `📜️script.ts:8952`, `:9255`, `:9301`.
- C: 10 hits in `✏️s` and `🌎️hub` (filtered pass, plus the earlier all-roots pass). Code readers at `🌎️hub/...stdio/.../📜️script.ts:849`, `🌎️hub/...space/.../📜️script.ts:318`, `📜️script.ts:9047` (allowlist, `lineHint` 1985) and `📜️script.ts:6270` (already tree-aware).
- D: 4 hits in `🌎️hub` (2 code readers: `🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts:208` and `🧪️tests/🧱️foundation-source/🟦️.ts:398`; 1 fixture: `🧫️fixtures/🧱️foundation-source/🔣️.json:371`; 1 nx input glob: `📦️packages/🦀️rust/📋️project.json:65`), plus `🧰️framework` hits in section 6.4, and `📜️script.ts:6271` (single-file reader to change).

Filtered `✏️s` and `🌎️hub` search (ts, tsx, rs, json, toml, md, mjs) completed with 15 hits for the four tails (10 for C, 4 for D, 1 doc for A or B). Unfiltered search of those two roots (4.1 GB and 11 GB) was not run; re-run it before the cut if binary or other file types matter.
