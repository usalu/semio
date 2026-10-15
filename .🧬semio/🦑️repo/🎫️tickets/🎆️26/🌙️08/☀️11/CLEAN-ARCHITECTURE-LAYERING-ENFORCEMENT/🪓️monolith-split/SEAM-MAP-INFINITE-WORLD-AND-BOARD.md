# Seam Map: Infinite World And Board Monoliths

Read-only audit. No source file was edited, no git command ran, no cargo/nx build ran.
Scratch outputs (coupling tables, item indexes) live in `🗑️generated/seam-scratch/` of this monolith-split folder.

Absolute root for all paths below:
`/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/`
Abbreviated as `INF/` below.

| Target | Lines | Owner |
|---|---|---|
| A. `INF/🌍️world/🦀️.rs` | 17,408 | `semio-framework-os-infinite` |
| B. `INF/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` | 13,836 | same crate |
| C. `INF/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` | 5,430 | same crate |

---

## 0. Crate ownership, wiring, cfg gates

### 0.1 Owning crate

- Manifest: `INF/📦️packages/🦀️rust/Cargo.toml`, package `semio-framework-os-infinite`, `[lib] path = "🦀️.rs"` (lib root = `INF/📦️packages/🦀️rust/🦀️.rs`, 84 lines).
- `INF/🦀️.rs` (87 bytes) is a one-line glue file: `pub use crate::world::*;`. It is mounted as `component` by the lib root, not compiled as a crate root.
- Other manifests in the infinite tree: `INF/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml` (package `semio-framework-artifact-infinite-dag`) and `INF/🎲️board/.../🧬️schema/🖼️presentation/📦️packages/🦀️rust/Cargo.toml` (package `semio-framework-os-infinite-directed-presentation`). Neither mounts A, B or C.

### 0.2 How A, B and C are mounted (lib root `INF/📦️packages/🦀️rust/🦀️.rs`)

```
L22-24  #[path = "../../../../../../🔨️modules/🗺️surface/🏔️terrain/🦀️.rs"] pub mod framework_surface_terrain;
L26-29  #[cfg(any(test, not(all(target_arch="wasm32", target_env="p2"))))] #[path="../../🦀️.rs"] mod component; pub use component::*;
L31-34  #[cfg(any(test, not(all(target_arch="wasm32", target_env="p2"))))] #[path="../../🌍️world/🦀️.rs"] pub mod world;
L37-62  #[path="."] pub mod board { #[path="../../🎲️board/🦀️.rs"] mod component; #[path="."] pub mod ports { ... 
          #[path="../../🎲️board/🔌️ports/➡️directed/🦀️.rs"] pub mod directed;          // 958 lines
          #[path="../../🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs"] pub mod directed_normal; // B
          #[path="../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs"] pub mod directed_dag;   // C
          ... } #[path="."] pub mod normal { #[path="../../🎲️board/➕️normal/➡️directed/🦀️.rs"] pub mod directed; // 12-line stub
          #[path="../../🎲️board/➕️normal/↔️undirected/🦀️.rs"] pub mod undirected; } }   // 6-line stub
L77     pub use board::ports::directed_normal::*;
L83     pub use board::ports::directed_dag::{fit_node_size, note_widget_size, preview_widget_size, would_create_cycle, DagHost};
```

Findings from the wiring:

- The lib root uses the `#[path = "."]` inline-module trick, so inline module names do not add directory levels. The B file relies on this pattern once its inline `board_host` module gets child files (see section 3).
- `board::normal::directed` and `board::normal::undirected` mount two 12- and 6-line stubs in `🎲️board/➕️normal/`. This is a second "normal" tree beside `🎲️board/🔌️ports/➡️directed/➕️normal/`. Both are compiled. Flag for cleanup, out of scope for the split.
- Plugins consume the crate through `extern crate ... as infinite_board_port_directed(_normal)` per the doc comment at lib root L75. The name `infinite_canvas` in that comment does not match the package name `semio-framework-os-infinite`. Naming drift, verify before relying on it.

### 0.3 cfg gates (target matrix)

| Module | native | wasm32-unknown (browser) | wasm32-wasip2 |
|---|---|---|---|
| `world` (A) + `component` | yes | yes | no (test only) |
| `board` incl. `directed`, `directed_normal` (B), `directed_dag` (C) | yes | yes | yes (ungated at lib root) |
| `framework_surface_terrain` | yes | yes | yes |

Inside the files:

- A: the `ui_wgpu` imports (L13-72) are `cfg(not(all(wasm32, p2)))`. Many items carry `#[cfg(not(test))]` or `#[cfg(test)]`. Full list via `/usr/bin/grep -n '#\[cfg'` in the scratch index.
- C: `mod wasm_session` (L5221-5398) is `cfg(all(target_arch="wasm32", not(target_env="p2")))`, browser only. The `#[cfg]` pair at L5221 and L5397 wraps the inline module.
- B: `#[cfg(test)]` at L3014, 3065, 3546, 3583, 8319, 11508, 12426 (legacy differential, test only).

---

## 1. File A: `INF/🌍️world/🦀️.rs` (17,408 lines)

Structure: flat file with `//#region` markers. Counts: 376 `fn`, 128 `struct`, 98 `impl`, 45 `enum`, 96 const/static, 7 `mod`, 0 `macro_rules`. Regions are at column 0. Gaps between regions hold free functions and records that have no region marker.

### 1.1 Top-level sections

Line counts are inclusive (`end - start + 1`). Gaps are blank or attribute-only.

| Lines | Count | Contents |
|---|---|---|
| 1-87 | 87 | `//!` doc, cfg-gated imports (`crate::framework_surface_terrain::TerrainSessionCore` L6, `ui_wgpu::wgpu::*` L13-72), `#[cfg(test)]` imports |
| 88-503 | 416 | Region `PreparedWorldResources`: consts L89-98, `WorldSlots` type L92, `world_close_string*` L100-101, `World3dBuildRejected` enum L110, `World3dBuildContext` struct L137 and impl L256-502, `WorldCursorWakeToken/Fault/State/Authority` L155-255, `Drop` L502 |
| 513-1122 | 610 | Region `SceneRecords` (nested region markers at 761, 992, 1073): serde records `WorldLodRecord` L535, `WorldChunkingRecord` L571, `WorldSelectionTargets` L591, `WorldVortexRecord` L610, `WorldAttractionRecord` L627, `WorldTutorialGeometry` L639 + `world3d_tutorial_*` L645-685, `WorldTargetVolumeRecord` L689, `WorldReferenceRecord` L703, `WorldEngagementPreviewRecord` L738. Nested `PickTargets` L761-922 (consts, `WorldPickTargetRecord` L812, pick key fns L847-921). `WorldCatalogueDropPreviewRecord` L925, `WorldBrushPreviewRecord` L936 with `ToValue/FromValue` impls L947-990. `Environment` records L992-1071. `TerrainStyle` L1073-1121 |
| 1124-2510 | 1387 | Region `World3dState`: `WorldDynamicToken/Fault/Entry/Rejected/Registry` L1145-1186, `WorldMeshSource`/`WorldMeshRegistry` L1190-1227 (Deref impls), `WorldNativeMeshAdmission` L1228, `WorldDynamicInsertPlan` L1245, `WorldDrawRegistry` L1395 + impl L1408-1508, `WorldDrawRebuildDescriptor/Step/Draft/Cursor` L1509-1666, `WorldSceneRaster` L1715, `World3dPrimitiveMaterial` L1721, `World3dMeshAppearance` L1728, `World3dMeshAsset` L1787, `WorldOpaqueOwner/Quarantine` L1819-1856 with statics L1857-1858, `World3dScalarFieldStatus` L1878, `World3dState` struct L1898 + impl L2131-2484, `Drop` L2505 |
| 2512-2617 | 106 | Region `WorldCameraSync`: `WorldCameraSync` L2545, `WorldNavigationSnapshot` L2562, navigation consts and fns L2570-2616 |
| 2619-2861 | 243 | Region `World3dDynamicRetirement`: struct L2621, impl L2626-2739, `begin_/step_/terminal/cursor_work_pending` fns L2740-2860 |
| 2863-3049 | 187 | Region `WorldDrawRebuild`: `begin_world3d_draw_rebuild` L2864 through `world3d_draw_rebuild_terminal_is_empty` L3046 (free fns only) |
| 3051-8332 | 5282 | Region `WorldInteractionTransaction`. Largest block in A. Detail in 1.2 |
| 8334-8376 | 43 | Region `Pool`: `RefCountPool` L8335 |
| 8378-8435 | 58 | Region `Chunking`: `chunk_*` and `update_visible_chunks` fns L8379-8434 |
| 8437-8824 | 388 | Region `LodGrid`: LOD consts, `default_lod_record`, `scene_lod`, `resolve_physical_mesh_id`, `append_lod_grid_lines`, `MeshStyleKind/MeshStyleState/MeshStylePaint`, `world_instance_style`, `resolve_mesh_style`, `world3d_style_paint`, `retain_ensured_world_*` L8676-8680, translucent sort L8685-8738 (cfg gated), `sync_mesh_pool` L8751, `reserve_world3d_mesh_fetch` L8785, `offer_missing_mesh_fetches` L8799, `queue_lod_mesh_fetch` L8813 |
| 8826-8972 | 147 | Region `Environment`: `environment_*` fns L8830-8964, `World3dShadowProfile/Geometry` L8904-8949 (cfg gated) |
| 8974-9576 | 603 | Region `Terrain`: `TerrainVisibleTileRow`, `TerrainTileMeshPayload`, tile URL/key fns, `WorldTerrainMeshPhase/Source/Cursor/Step` L9060-9310 (Drop L9311), `step_world3d_terrain` L9331, `begin_world_terrain_pending` L9392, `TerrainMeshOracleData` + `build_terrain_tile_mesh_oracle` L9401-9418 (cfg test), `apply_terrain_style_*` L9442-9473, `TerrainTileDraw` L9484, `sync_terrain_state` L9498, `sync_terrain` L9557 |
| 9578-9629 | 52 | Region `Gumball`: `GumballHandle` enum L9580, impl L9595-9629 |
| 9630-10252 | 623 | Region `MeshHelpers` / `PlaceholderMeshAuthority`: `WorldPlaceholderKind` L9633, phase/owner/input enums L9664-9698, `WorldPlaceholderMeshCursor` L9726 + impl L9767-10145 (Drop L10146), `placeholder_triangle*` L10152-10233, `begin/step_world_placeholder_mesh` L10233-10251 |
| 10254-10267 | 14 | Region `WorldMeshBuffers`: type alias `WorldMeshBuffers = semio_framework::MeshData`, trait `WorldMeshSurface` L10257, impl L10262 |
| 10269-10428 | 160 | Region `PlaceholderMesh`: `placeholder_*` mesh generators; most are `#[cfg(test)]` L10275-10389 |
| 10430-11730 | 1301 | No region. Pick and overlay helpers: `selection_mode_label`, `component_mode_active`, consts L10442-10445, `world_pick_rect`, `pointer_in_pick_rect`, `pick_targets_instance`, `is_click_gesture`, vertex marker consts L10495-10515, `world3d_sub_element_paint_native`, `append_component_overlays` L10786, `WorldFaceOverlay*` enums/structs L10911-10943, `WorldFaceOverlayMeshCursor` impl L11063-11281 (Drop L11282), `face_overlay_*` L11312-11360, `pick_gumball_handle_at` L11432, `append_gumball_geometry` L11511, `apply_gumball_preview` and gumball preview fns L11584-11649, `gumball_commit_action` L11656 |
| 11731-11913 | 183 | No region. `world3d_camera_zoom`, `advance_world3d_view_revision`, projection apply/spec fns L11753-11812, `world3d_pending_camera_settle` L11843, `world3d_camera_projection` L11848, `world3d_live_camera_json` L11856, `orbit_camera_action` L11888 |
| 11914-12493 | 580 | No region. Native mesh admission and close steps L11914-11960 (cfg gated), `publish_world3d_mesh_lease` L11967, `store_mesh`, `retain_*` L11989-12008, `set_world_mesh_version`, `mark_world_dynamic_fault`, `retire_world_mesh` L12027, `publish_world_raster` L12041, `DecodedMeshPaintImage` L12060, raster upload fns L12094-12124, `World3dPreparedStatus` L12133, `World3dSnapshotApplyCursor` L12156 + impl L12171-12478, `World3dSnapshotApplyStep` L12187, `step_world3d_snapshot` L12224 (about 255 lines), close fn L12479 |
| 12494-13871 | 1378 | Region `World3dSceneBridge`: `World3dSceneMeshEntry/ComponentSource/InstanceEntry/CameraRecord/FitRecord`, camera-fit cursor L12792-12912, `step_world3d_camera_fit` L12912, selection/gumball/hover records L13017-13148, `World3dSceneBridgeCursor` L13195, `stage_world3d_scene_bridge` L13238, `step_world3d_scene_bridge` L13317 (about 170 lines), `publish_world3d_scene_bridge_snapshot` L13565, `sync_world3d_scene_*` L13698-13854, reference URL fns L13854-13870 |
| 13873-13924 | 52 | No region. `sync_world3d_state` L13873, `world3d_presentation` L13914 |
| 13925-14079 | 155 | No region. `sync_world3d_tool_run_trace` L13925, `world3d_tool_run_trace_cursor(s)` L13943-13955 (cfg gated), `append_tool_run_trace_draws` L13956, `apply_runtime_draw_flags` L13984, `append_world3d_section_cap` L14017 |
| 14080-14273 | 194 | Region `Modelling overlay`: consts L14081-14089, `world3d_*_points` and `world3d_overlay_color` L14092-14121, `paint_world3d_annotations` L14123, legend L14179-14221, `World3dAccessibilityEntry` L14234, `world3d_modelling_accessibility` L14245 |
| 14284-14697 | 414 | No region. `render_world_3d` (one function, cfg gated L14284) |
| 14697-14698 | 2 | `#[path = "⏯️tool-run-trace/🦀️.rs"] pub mod tool_run_trace;`. Existing child module |
| 14700-14719 | 20 | Region `WorldOrbitViewGizmo`: `update_world_orbit_view_gizmo_hover` L14706 |
| 14721-15060 | 340 | No region. Pointer and wheel entry points: `world3d_hit_target`, `handle_world3d_pointer_move` L14726 (cfg test), `handle_world3d_paint_actions` L14765, `handle_world3d_pointer_button` L14795, `resolve_world_context_menu_target` L14955, `handle_world3d_pointer_drag` L14989, `handle_world3d_wheel` L15024, `merge_string_ids` L15034 |
| 15061-15228 | 168 | Region `WorldInteractionDomain`: domain/granularity consts, `world_interaction_definition`, resolved-id fns, merge mode wire helpers |
| 15245-15781 | 537 | No region. Action and pick fns: `world_component_preview_target`, `apply_world_action_preview`, `pick_hover_action`, `pick_select_action`, `merge_u32_ids`, `marquee_select_action`, `gumball_drag_update`, `start_gumball_drag`, `pick_component_at`, `pick_paint_hit`, `world_marquee_overlay_points`, `marquee_local_polygon`, `update_marquee_preview`, `scene_geometry`, `pick_instance_at`, `pick_surface_at` |
| 15782-15893 | 112 | Region `VortexArrow`: mesh consts, `VortexArrowLayout`, quaternion helpers, `vortex_arrow_layout`, `ensure_primitive_mesh`, `append_vortex_arrow_draws` |
| 15895-15996 | 102 | No region. `pick_vortex_at`, `pick_reference_at`, `object_world_position`, `update_dragged_instance_position`, `world3d_ground_plane_pick` |
| 15997-16134 | 138 | Region `WorldRelocateGesture`: `World3dRelocateSession` L16003, relocate fns L16018-16133 |
| 16136-16249 | 114 | No region. `snap_world_point_to_grid`, catalogue drop preview fns, `preview_scale`, `reference_image_aspect`, `world_reference_plane_size`, `brush_preview_mesh_id`, `ghost_mesh_id`, `append_box_wireframe`, `parse_color`, `srgb_to_linear` |
| 16250-16899 | 650 | Region `WorldAssetIoAuthority`: metadata id, request/response tokens and faults, `WorldAssetFetchOwner` + Drop L16479, `WorldAssetClaim`, `WorldAssetIoAuthority` struct L16499 + impl L16513-16777, step/close fns L16777-16893, Drop L16894 |
| 16901-16949 | 49 | No region. `mesh_id_from_url`, `publish_world3d_asset_*` and `publish_world3d_inline_asset` |
| 16950-17251 | 302 | Region `WorldBrushMeshAnnounce`: consts L16956-16976, `WorldBrushMeshRun/Entry/Registry` L16982-17023, static `WORLD_BRUSH_MESH_REGISTRY` L17038, feedback fns L17041-17111, `step_world3d_brush_mesh_announce` L17172 |
| 17260-17365 | 106 | No region. Reference image decode: consts L17268-17269, `bounded_reference_image`, `DecodedReferenceImage`, `decode_*` and `apply_*reference*` L17272-17362 |
| 17366-17408 | 43 | Test module declarations (all `#[cfg(test)]` + `#[path]`): `tests` L17368 (`🧪️tests/🔬️unit/🦀️.rs`), `pointer_gesture_tests` L17374, `scene_input_residency_tests` L17379, `journal_sequence_tests` L17385, `surface_verb_tests` L17391, `bridge_empty_build_tests` L17397, plus L17400 `world_test_mesh_query` |

### 1.2 Region `WorldInteractionTransaction` (L3051-8332, 5282 lines) internal clusters

| Lines | Count | Contents |
|---|---|---|
| 3051-3814 | 764 | Ids and registries: `WorldInteractionId` L3062, mesh registry L3083-3227, object registry L3235-3371, `WorldInteractionRegistryBuildCursor` L3379-3530, phase/cancel enums, intent queue L3568-3711, flat action L3713-3761, plan L3750-3761, active census L3762-3792, `WorldInteractionAuthority` struct L3793 |
| 3815-4745 | 931 | Marquee: `WorldMarqueeGesture` L3815, result pages L3823-3889, `WorldMarqueePickCursor` L3890 + impl L3920-4388, `WorldMarqueePublishJob` L4389 + impl L4414-4623, `WorldComponentMarqueePublishJob` L4624-4709, `impl WorldMarqueeGesture` L4710-4745 |
| 4746-5598 | 853 | Pick cursors: `WorldInteractionAuthorityStep` L4746, `WorldInteractionStep` L4829, `WorldRayPickCursor` L4857 + impl L4884-5238, `WorldObjectPickCursor` L5246 + impl L5258-5385, `WorldComponentKind/Hit/PickCursor` L5386-5425 + impl L5426-5598 |
| 5599-6603 | 1005 | Component and gumball: `WorldComponentTarget` L5601, `WorldComponentAddress` L5675, `WorldGumballTarget` L5644, `WorldGumballPickCursor` L5716 + impl L5788-6039, `WorldGumballPhase` L6040, `WorldGumballGesture` L6072-6232, `WorldGumballCommitJob` L6233, `WorldBrushCommitJob` L6252-6453 |
| 6604-6823 | 220 | Free fns: `world_mesh_vertex`, `world_mesh_triangle`, `world_mesh_component_id`, `world_ray_triangle_barycentric`, `enqueue_world3d_intent`, `enqueue_world3d_event(s)`, `world3d_interaction_front_generation`, `step_world3d_interaction`, `close_world3d_interaction_step`, `world3d_interaction_terminal_is_empty` |
| 6824-7739 | 916 | `impl WorldInteractionAuthority` (main authority impl) |
| 7740-7948 | 209 | Free fns: `world3d_hover_is_published`, `plan_world3d_pointer_leave`, `plan_world3d_camera_settle`, `plan_world3d_wheel`, `plan_world3d_non_primary_press_pick`, `plan_world3d_drag`, `plan_world3d_paint_release`, `plan_world3d_paint_abort`, `INTERACTION_TARGETS_*` consts L7937-7942 |
| 7949-8028 | 80 | `WorldInteractionText` + `world_write_json_string`, `world_write_interaction_target`, `world_interaction_targets` |
| 8029-8331 | 303 | `publish_world3d_plan_step` |

### 1.3 Child modules already present under `INF/🌍️world/`

- `⏯️tool-run-trace/🦀️.rs` (mounted at L14697) with `🧪️tests/`.
- `🎨️r3f/`, `🏔️terrain/` (contains only `🚦️ownership/`, no `🦀️.rs`), `♻️resources/` (own tests + `🧬️schema`), `🧬️schema/`, `🧪️tests/`, `🧫️fixtures/`.
- Test mods of A mount through `#[path = "🧪️tests/<emoji-name>/🦀️.rs"]` (L17367-17398).

---

## 2. File B: `INF/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` (13,836 lines)

Structure: the whole file is one inline module `pub mod board_host { ... }` from L3 to L13830 (`// #endregion board_host`). The file body is indented by 4 spaces. Tail L13831-13836 are re-exports. Inner `#![allow(...)]` at L6-7. `macro_rules!` count 3 (all function-local, see section 4).

### 2.1 Top-level sections (indent 4 inside `board_host`)

| Lines | Count | Contents |
|---|---|---|
| 1-333 | 333 | Module doc L1, wrapper open L3, `#![allow]` L6-7, imports, `NormalPortError` enum L40-112 + Display/Error/From impls, consts L114-149 (`GRID_WORLD_*`, `PUZZLE_2D_LODS` L140, `PUZZLE_2D_LOD_SCALE` L149), `BoardDrawLod` enum + impl L151-196, `puzzle_2d_lod_scale_json` L197, `StyleChromePass` L214, `NodeHandlePaintLayer` L221, `BrushCandidateEntry/Ref/Page` L229-318, `BrushPreviewSnapshot` L320 |
| 334-1550 | 1217 | Fill types and commit codec: `BOARD_FILL_*` consts L334-344, `BoardFillText` L346-412, fixed-page containers L413-532, `BoardFillShape` and snapshot/capture/handle/template/kind/rule snapshot and capture types L534-658, `BoardFillSnapshot` + Drop L659-672, `BoardFillCaptureFault` L674, `BoardFillSnapshotCapture/Ingress` L699-722, ingress text enums L724-750, `BoardFillCaptureStep` L752, source/candidate/preview types L759-834, `BoardFillStage` + impl L831-892, `BoardFillJobState` L893, `BoardFillPreview`, `BoardFillCandidateVerdict/Event`, `BoardFillPlacement` + impl + Drop L972-1050, `BoardFillCheckpoint` + impl L1051-1093, `BoardFillResult`, `BoardFillCommitHandle/Shape/Placement/Candidate` L1102-1144, commit magic/offset consts L1145-1168, `BoardFillCommitEncodeStage` L1171, `BoardFillCommitEncoder` + impl L1202-1392, `BoardFillCommitPayload` L1393, `impl BoardFillCommitCandidate` L1429-1528, `BoardFillJob` L1529, `BoardFillOffer` L1546 |
| 1551-2512 | 962 | Drag/paint state types L1551-1596 (`BoardTransformDrag`, `BoardRegionDrag`, `BoardRegionPaint`, `DropPreviewSnapshot`), `BoardPickTargetJson` L1598, `BOARD_EVENT_*` consts L1607-1610, `BoardEventKind` + impl L1615-1697, `board_pointer_offset`, `BoardGestureMotion`, `BoardEventFault`, `BoardOwnedEvent` L1722 + impl L1868-2347, `BoardEventReservation`, `BoardEventBatchReservation`, `BoardEventQueue` L2348 + impl L2364-2512 |
| 2513-3667 | 1155 | `NodeIconSpan` L2513, `WorldContentCache` L2523, `BoardHost` struct L2530-2651, `BoardHostRetirement` + impl + Drop L2652-2685, `BoardHostClosePhase` L2687, `BoardWheelPlan` L2716, descriptor/pointer consts L2726-2737, `BoardPointerPhase/Intent/PlanFault/Span/Delta` L2738-2774, `BoardDeleteKind/Entry/Plan/Operation/PlanningPhase/PlanningOperation` L2776-2841, `BoardPropertyAudit*` L2842-2884, `BoardPointerPublication` L2885, `BoardPointerCommitOperation` + impl L2890-2960, `BoardAuthorityStep` L2962, `impl BoardDeletePlan` L2969-3064, `#[cfg(test)] impl BoardPropertyAudit` L3068-3216, `impl BoardPropertyRetirement` L3217, `impl BoardEntityRetirement` L3295, `impl BoardPointerPublication` L3378, `BoardPointerPlanKind`, `BoardPointerPlan` + impl L3443-3630, json/id helpers L3631-3661, `impl BoardWheelPlan` L3662 |
| 3668-5668 | 2001 | `impl Default for BoardHost` L3668. `impl BoardHost` L3759-4230: `draw_lod_for_frame`, `board_draw_lod_label`, opaque-scene retirement fns L3775-3830, close fns L3834-3944, `nonopaque_terminal_is_empty`, `viewport`, `camera_content_affine`, grid-step fns L4232-4244, `new`/`new_normal` L4245-4255, `set_grid_*`, `set_external_link_preview_json`, `set_size`, `set_camera*`, `set_selection_options`, `set_handle_link_compat_from_json`, `set_board_kind_catalogs_from_json` L4478, `validate_against_manifest` L4602. `impl BoardHost` L4231-5668: CSS color parsers L4651-4779, style resolution fns L4780-5157 (`resolve_*_style_kind`, `resolve_*_color`), edge tip fns L5159-5249, link compat and default-kind fns L5285-5413, brush helper fns L5414-5668 (`brush_*`) |
| 5669-7640 | 1972 | Region `Fill` (L5669-5675): `begin_board_fill_snapshot` L5671. Region `FillJob` (L5677-7642): `impl BoardFillSnapshotCapture` L5680-6097 (Drop L6098), `impl BoardFillSnapshotIngress` L6104-6532 (Drop L6533), `impl BoardFillJob` L6539-7548 (large state machine, `macro_rules! retire_page` at L7521), `impl InteractiveJob for BoardFillJob` L7549-7624, Drop L7625, `const _: fn()` assert L7631 |
| 7641-10131 | 2491 | `impl BoardHost` continued: `brush_preview_snapshot_from_session_json` L7644 (free fn), brush session mirror and slot fns L7679-7933, `brush_*` candidate and slot fns, `append_brush_node_icon_paint`, drop-preview paint, `set_world_raster_tiling`, `set_original_element_style`, `set_selection_screen_preview`, `set_canvas_palette`, reserve/publish event fns L8158-8229, Gestures region L8230-8317, `drain_events_json`, `step_event_authority`, `step_pointer_commit_operation` L8413, `step_selection_preview_commit` L8560, `step_selection_commit` L8691, `step_brush_pointer_commit` L8789, `set_selection_ids*` L8972-9018, `commit_area_select_from_initial`, `interaction_json` L9120, `handle_positions_json`, world/screen conversions, scaled-size fns L9238-9266, indirect-handle fns L9290-9330, link drag ring fns L9330-9626, `resolve_hover_world` L9846, `resolve_pick_targets_world` L9950, `pick_targets_at_screen_json` L10017, `resolve_hit_world` L10022, `resolve_node_hit_world` L10108 |
| 10132-12425 | 2294 | `sync_descriptor` L10132 and `descriptor_admission` L10140, `sync_descriptor_with` L10179-10392, `set_node_positions*` L10393-10425, `clear_scene` L10426, `load_board_snapshot` L10442, cull fns L10490-10512, paint helpers L10540-10951 (`paint_node_geometry`, `append_handle_marker`, `append_nodes_and_handles*`, `append_edges_wires_and_link`), icon bake fns L10951-11001, `append_cached_world_content` L11002, `set_wheel_zoom_active`, `build_vector_scene` L11091, hover/highlight setters L11144-11209, `wheel_screen`/`plan_wheel`/`commit_wheel` L11209-11228, delete planning L11229-11668 (`begin_delete_property_audit`, `step_delete_planning` L11260 incl. cfg, `plan_delete_selection`, `step_pending_delete_operation`, `delete_selection` L11642), link snap fns L11669-11841, `try_commit_link_edge` L11842, selection/link pointer plan fns L11902-12256, `plan_pointer` L12200, `plan_drag_pointer` L12240, `begin/step_pointer_commit` L12257-12343, publication and cancel fns L12344-12414 |
| 12426-13124 | 699 | `#[cfg(test)] commit_pointer_legacy_differential` L12426-12578, `commit_pointer` L12415 (kept with this block), `pointer_down_screen` L12579, `pointer_move_screen` L12717, `pointer_up_screen` L12877, `pointer_leave_screen` L12990, `pointer_cancel_screen` L13010, `cancel_area_select` L13050, `node_world_bounds`, `selection_draggable_node_members`, `selection_union_bounds_world` L13096-13124 |
| 13125-13623 | 499 | Region `TransformGumball` L13125-13317: `set_transform_flags`, `transform_flags`, `transform_gumball_geometry`, `transform_gumball_json`, `try_begin_transform_drag_at`, update/commit/cancel fns, `append_transform_gumball_paint` L13296. Region `TargetRegions` L13319-13621: `set_area_brush_extent`, `area_brush_extent`, region snap fns, `target_regions_json` L13366, region drag and paint fns, `append_target_region_paint` L13570 |
| 13624-13830 | 207 | `try_begin_bounded_selection_drag_at`, `selection_drag_shape_world`, `selection_contains_node/handle/edge`, `resolve_area_selection_with_initial` L13722, `test_resolve_node_style_kind` L13760, `build_scene` L13766, `impl CanvasContent for BoardHost` L13765 (ends L13830 with the wrapper close) |
| 13831-13836 | 6 | Tail: `pub use crate::infinite::board::ports::directed::*;`, `pub use semio_framework_canvas as canvas;`, `pub use board_host::*;` |

### 2.2 Other files under `INF/🎲️board/🔌️ports/➡️directed/➕️normal/`

- `🧪️tests/🔬️board-host-standalone/🦀️.rs` (only the test fn name `board_host_nonopaque_close_is_interruptible_and_terminal_witnessed` at L659 mentions `board_host`; it is not a path reference).

---

## 3. File C: `INF/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` (5,430 lines)

Structure: flat file with `//#region`/`// #region` markers. One inherent block, `impl DagHost { ... }` L1599-5216 (3,617 lines). Children already mounted:

- L10-11 `#[path="🎛️input/🦀️.rs"] pub mod input_application;` (93 lines)
- L1515-1516 `#[path="🧹️retirement/🦀️.rs"] mod retirement;` (32 lines)
- L5221-5398 inline `mod wasm_session { use super::*; ... }` (browser only)
- L5403-5412 `tests`, `wire_edit_tests`, `node_graph_edit_rows_tests` via `#[path]` into `🧪️tests/`

### 3.1 Top-level sections

| Lines | Count | Contents |
|---|---|---|
| 1-98 | 98 | Module doc, imports, `DagBoardEngine` alias L24, `pub use crate::infinite::board::ports::directed::{self as graph, ...}` L16-22, `use graph::{...}` L23 |
| 26-75 | 50 | Region `Errors`: `DagError` enum L29 + Display/Error/From impls |
| 77-98 | 22 | Region `PortSide`: `DagPortSide` enum L80, impl L85 |
| 99-262 | 164 | Layout constants L99-110, `ComputationChannelRowSide` L112, computation/IO geometry fns L118-262 |
| 264-503 | 240 | Region `PreviewContent`: `DAG_PREVIEW_*` consts L265-270, `PreviewTreeRow*` L273-287, `WidgetPointerKind` L288, preview-tree fns L296-505 |
| 504-851 | 348 | Region `IoNode`: `DAG_CLUSTER_*`, slider/select/action control bounds, `io_node_handle_angles`, computation column fns L631-750, `fit_node_size` L750, variadic insert fns L792-803, port angle fns L814-854 |
| 853-887 | 35 | Regions `Acyclicity` (`would_create_cycle` L855), `Layout` (`apply_dag_layout` L864), `GraphExtension` (`DagExtension` L878, impls L880-889) |
| 889-1099 | 211 | Region `Lod`: `DAG_LODS` L892, LOD scale L901, zoom fns, handle/stroke consts L926-939, node body and stroke fns L941-1063, `dag_draw_lod`, `dag_lod_scale_json` L1089 |
| 1101-1123 | 23 | Regions `Grid` (consts L1102-1106), `ChannelRef` (L1109-1112), `NoteEdit` (`NoteEditState` L1117-1123) |
| 1125-1598 | 474 | Region `DagHost` (types first): `DAG_INTERACTION_*` consts L1127-1128, `DagInteractionPlanFault`, `DagPointerPhase/Intent`, `DagNodeMove`, `DagDragOffset`, `DagProjectionGesture`, `DagInteractionProjection` L1182 + impl L1237, `DagPointerSnapshot`, `DagPointerPlan` L1198 + impl L1263, `impl DagSelectionSource for DagHost` L1210, `DagHost` struct L1286, `DagGraphEdit` L1346, `DagJournalRefusal` L1357, `DagGraphEditRowSink` trait L1364, row writer fns L1378-1505, `DAG_GRAPH_EDIT_CAPACITY` L1513, `DagNodeEvalStatusKind` L1520, `MinimapWidgetLayout` L1528, `pointer_event_now_ms` L1541, screen/world conversions L1553-1563, canvas color fns, `DagNodePaintChrome` + impl L1575-1598 |
| 1599-2224 | 626 | `impl DagHost` (host snapshot constructors L1600-1672, node id maps, hover/selection predicates L1692-1736, projection and pointer plan fns L1776-1950, selection getters, `hover_facts`, `screen_geometry_census_json` L2092, screen hit fns L2114-2224) |
| 2225-2710 | 486 | `impl DagHost` regions: `ContentFraming` L2225-2285 (content bounds, fit camera), `MinimapWidget` L2287-2404 (layout, hit, paint), `SelectionAlign` L2406-2710 (union bounds, `entity_screen_json`, `align_selection`) |
| 2711-3997 | 1287 | `impl DagHost` mutators and controllers: `set_widget_position`, selection and hover setters L2713-2800, computing progress L2800-2851, port insert hit L2856-2890, viewport/ghost/LOD setters L2891-3115, `focus_selection_camera`, grid/LOD paint L3016-3105, `load_host_snapshot_json`, `reorganize`, `rebuild_engine_with_layout` L3134, edge/port parsing fns L3226-3280, `sync_*_from_engine` L3246-3300, `process_engine_events`, journal fns L3334-3437, screen pointer gesture and hit fns L3438-3493, handle-hit fns L3494-3611, `try_node_rectangle_pointer_down`, `widget_hit_at`, `toggle_preview_tree_path`, note edit fns L3759-3933, `take_pending_*` L3964-3969, `pointer_down`/`pointer_down_screen` L3986-4068 |
| 3998-5216 | 1219 | `impl DagHost` paint: `pointer_move*`, `pointer_up*`, `pointer_cancel_screen`, `set_canvas_palette`, `node_overlays_json`, handle/edge path fns L4204-4241, `paint_node_handles_for_spec`, label overlay rows and JSON L4283-4463, variadic and name painters L4487-4800, computing border fns L4780-4814, `paint_node_visual` L4874, `paint_scene` L5084, candidate count fns L5418-5420 |
| 5217-5399 | 183 | `#[cfg(all(wasm32, not(p2)))] mod wasm_session { use super::*; ... }`, browser session paint |
| 5401-5413 | 13 | Region `Tests`: three `#[cfg(test)] #[path]` test mods |
| 5414-5430 | 17 | `impl infinite::board::schema::dag_input::output::DagOutputCandidates` L5418 (implements a trait from `schema/dag_input`) and trailing fns |

---

## 4. Hazards

### 4.1 In-file macros

- B: three `macro_rules!` inside method bodies, function-local, so they move with their function: `retire_snapshot_page` L6071, `retire_ingress_page` L6506, `retire_page` L7521 (all inside the Fill job/snapshot code of section 2.1 row 5669-7640).
- A: none. C: none.

### 4.2 Source-scanning tests (highest hazard for A)

These tests read production source as text. They break when code moves:

- `INF/🌍️world/🧪️tests/🔬️unit/🦀️.rs` L1917 `include_str!("../../🦀️.rs")` reads A itself.
- Same file L3172 `include_str!("../../🦀️.rs")` reads A itself.
- Same file L3196 `include_str!("../../../🦀️.rs")` reads the infinite root glue file.
- Same file L236-239, L1379, L1906-1907 read renderer sources under `📺️renderer/...` (see wrong-direction 5.4).
- `INF/🌍️world/🧪️tests/📜️journal-sequences/🦀️.rs` L19 and `📇️surface-verbs/🦀️.rs` L19 read renderer files through `../../../../📺️renderer/...`.

Action before/with the split: each source-scanning assertion must be re-pointed to the child files (or to a concatenation of the world subtree).

### 4.3 `#[path]` rewrites and inline-module resolution

- A: L14697 `#[path = "⏯️tool-run-trace/🦀️.rs"] pub mod tool_run_trace;` is relative to A's directory. Keep `tool_run_trace` declared in A's root, or rewrite the path if it moves.
- A: L17366-17398 test mods (`🧪️tests/...`) are relative to A's directory. They stay in the root.
- B: the inline `pub mod board_host` (L3-13830) will need children. Inside an inline module of a file loaded via `#[path]` (not `mod.rs`), a child `#[path]` is relative to the file's directory plus the inline module name as a directory. Two ways to avoid that: (a) put `#[path = "."]` on the inline `board_host` (the repo's existing lib-root convention), or (b) unwrap `board_host` so its body becomes the file root and children sit next to it. Option (b) also changes the public path `directed_normal::board_host::*`. The only in-tree reference to that path is the test fn name, and the lib root reaches it only via `directed_normal::*` (glob). Option (b) is the cleaner one.
- C: inline `mod wasm_session { use super::*; }` at L5222 and its `cfg` pair at L5221/L5397. Moving it to a file needs `#[path = "🌐️wasm-session/🦀️.rs"] mod wasm_session;`. Its `use super::*;` keeps working because the parent is unchanged.
- C: `pub use crate::infinite::board::ports::directed::{self as graph, ...}` at L16-22 and `use crate::infinite::board::schema::...` at L861-1111 use absolute `crate::infinite` paths. Those keep working in children.
- Existing child files whose own `#[path]` targets are relative to their own folders: `🕸️dag/🎛️input/🦀️.rs`, `🕸️dag/🧹️retirement/🦀️.rs`, `🕸️dag/🧪️tests/...`. Unaffected unless the dag root moves.

### 4.4 Visibility (private items)

Private items are visible inside their module and its descendants. The split therefore needs pub(super) or pub(in path) only where code crosses a module boundary sideways or upward. Counts below come from an identifier-level heuristic over the proposed partitions (section 5). They are approximations. They count distinct private items defined in a non-ancestor group and referenced from another group.

- A (world), proposed partition: about 218 distinct items need widening (sum over children). Root-defined items need no change. Biggest pressure from `World3dState` hub (L1124-2510) and from `WorldInteractionAuthority` (L6824-7739), which reaches into the registries and the marquee and pick cursors.
- B (board_host), proposed partition: about 157 distinct items.
- C (dag), proposed partition: about 37 distinct items.
- Test modules in A (`tests`, `pointer_gesture_tests`, and others) reach into A-internal privates. Their references are not counted above. Measure after the move.

Heuristic cross-reference totals, for scale: A about 6,491 identifier occurrences across proposed groups; B: `B-FILLJOB` to `B-FILLTYPES` 1,132 occurrences; C: `DX3-PAINT` to `D-HEAD` 180 occurrences.

### 4.5 Other

- `#![allow(...)]` at B L6-7 applies to the whole `board_host` module. Moving it to the file root widens its scope to the file (acceptable, or keep it in the children).
- `#[cfg]` and `#[cfg(test)]` attributes must move with the item they gate. The pairs in A (for example L1388, L1667, L1706, L2504, L9310, L9400-9417, L11941-11960) must stay adjacent to their items.
- A: `use` lines at L1-87 are not inherited by children. Each child needs its own `use super::*;` plus specific imports, including the `#[cfg(not(all(wasm32, p2)))]` gating on ui_wgpu items.
- The B and C per-child `use` lists must keep the `#[cfg(test)] use ...` lines (for example `#[cfg(test)] use crate::infinite::board::schema::layout::{DagLayoutOrientation};` at C L862).
- Glob re-exports (`pub use child::*`) can create name ambiguity if two children export the same name. Keep names unique.

---

## 5. Proposed partition

Target: each parent below about 3,000 lines. Child folder names are proposals; each emoji must be RGI and unique within its parent. Check the taxonomy before use.

### 5.1 File A: world

Root `INF/🌍️world/🦀️.rs` keeps lines 1-2510 (header, records, `World3dState`) plus the test mods at 17366-17408 and the tool-run-trace mod declaration. That is about 2,510 lines.

| Child folder (under `🌍️world/`) | Module | Lines | Count | Items |
|---|---|---|---|---|
| `🧹️lifecycle/` | `lifecycle` | 2512-3049 | 538 | `WorldCameraSync`, `World3dDynamicRetirement`, `WorldDrawRebuild` (three sub-regions, three child files or one) |
| `🧵️interaction/` (wrapper, see 5.2) | `interaction` | 3051-8332 | 5282 | `WorldInteractionTransaction` (split into five children) |
| `🧱️scene/` | `scene` | 8334-10428 | 2095 | `Pool`, `Chunking`, `LodGrid`, `Environment`, `Terrain`, `Gumball` handle, `MeshHelpers`/placeholder authority, `WorldMeshBuffers`, `PlaceholderMesh` |
| `🖌️overlay/` | `overlay` | 10430-12493 | 2064 | pick/overlay/gumball paint, camera/projection, native mesh admission, raster publish, `World3dSnapshotApplyCursor` |
| `🌉️bridge/` | `bridge` | 12494-13871 | 1378 | `World3dSceneBridge` |
| `🎛️frame/` | `frame` | 13873-14719 | 847 | `sync_world3d_state`, `world3d_presentation`, tool-run-trace draws, modelling overlay, `render_world_3d`, `WorldOrbitViewGizmo` (declare `tool_run_trace` in the root, not in `frame`) |
| `🖱️pointer/` | `pointer` | 14721-16249 | 1529 | pointer and wheel handlers, `WorldInteractionDomain`, action and pick fns, `VortexArrow`, pick helpers, `WorldRelocateGesture`, snap and catalogue fns |
| `📡️assets/` | `assets` | 16250-17365 | 1116 | `WorldAssetIoAuthority`, asset publish helpers, `WorldBrushMeshAnnounce`, reference image decode |

`🧵️interaction/` wrapper, `🧵️interaction/🦀️.rs` (mount `#[path = "🧵️interaction/🦀️.rs"] mod interaction;` from A's root):

| Child folder | Module | Lines | Count | Items |
|---|---|---|---|---|
| `📇️registry/` | `registry` | 3051-3814 | 764 | ids, mesh/object registries, registry build cursor, phase and cancel enums, intent queue, flat action, plan, active census, `WorldInteractionAuthority` struct |
| `🔍️marquee/` | `marquee` | 3815-4745 | 931 | `WorldMarqueeGesture`, result pages, marquee pick cursor, publish jobs |
| `🧲️picking/` | `picking` | 4746-5598 | 853 | interaction authority step/step enums, ray/object/component pick cursors |
| `🕹️gumball/` | `gumball` | 5599-6603 | 1005 | component target/address, gumball target and cursor, gesture, commit jobs, brush commit job |
| `⚙️authority/` | `authority` | 6604-8332 | 1729 | free fns 6604-7948, `impl WorldInteractionAuthority` 6824-7739, text helpers, `publish_world3d_plan_step` |

Note: `impl WorldInteractionAuthority` (6824-7739) and its free fns are the main coupling point. Keeping it in one child avoids splitting the impl.

### 5.2 File B: board_host

Unwrap `board_host` (option (b) in 4.3). The root `INF/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` keeps lines 1-2512 (header, fill types, drag and event types, event queue) plus the tail re-exports and the child mounts. That is about 2,512 lines.

| Child folder (under `➕️normal/`) | Lines | Count | Items |
|---|---|---|---|
| `🧭️host/` | 2513-3667 | 1155 | `NodeIconSpan`, `WorldContentCache`, `BoardHost` struct, retirement, pointer and delete plan types and impls |
| `🎨️style/` | 3668-5668 | 2001 | `impl Default for BoardHost`, `impl BoardHost` (camera, LOD, CSS color, style resolution, edge tips, link compat, brush helpers) |
| `🪣️fill-job/` | 5669-7640 | 1972 | `begin_board_fill_snapshot`, `BoardFillSnapshotCapture`, `BoardFillSnapshotIngress`, `BoardFillJob` (three macro_rules come along) |
| `🎛️selection-events/` | 7641-10131 | 2491 | `impl BoardHost`: brush session, event authority, selection commits, interaction JSON, hit and pick fns |
| `🧹️scene-delete-links/` | 10132-12425 | 2294 | `impl BoardHost`: descriptor sync, paint, hover and wheel, delete planning, link snap, pointer planning, pointer commit |
| `🖱️pointer-gestures/` | 12426-13830 | 1405 | `#[cfg(test)] commit_pointer_legacy_differential`, `commit_pointer`, `pointer_down/move/up/leave/cancel_screen`, transform gumball, target regions, bounded selection, `build_scene`, `CanvasContent` impl |

Mount pattern for B children, in the B root (no inline module):
`#[path = "🧭️host/🦀️.rs"] mod host;` and so on, each child beginning `//!` with its own emoji.

### 5.3 File C: dag

Root `INF/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` keeps lines 1-1598 (errors, port side, preview content, IO node geometry, lod, grid, note state, `DagHost` types and graph edit rows). Existing child mounts (`🎛️input`, `🧹️retirement`, tests) stay.

| Child folder (under `🕸️dag/`) | Lines | Count | Items |
|---|---|---|---|
| `🧭️host-selection/` | 1599-2224 | 626 | `impl DagHost`: constructors, selection and pointer plans, screen hits |
| `🧱️frame-minimap-align/` | 2225-2710 | 486 | `impl DagHost`: content framing, minimap widget, selection align |
| `🛠️mutate/` | 2711-3997 | 1287 | `impl DagHost`: setters, LOD and grid, journal, screen pointer and widget hits, note edits, pointer I/O |
| `🖌️paint/` | 3998-5216 | 1219 | `impl DagHost`: paint helpers, label overlays, `paint_node_visual`, `paint_scene`, `impl DagOutputCandidates` (5418-5430 can stay in the root instead) |
| `🌐️wasm-session/` | 5217-5399 | 183 | `#[cfg(all(wasm32, not(p2)))]` session paint (keep `use super::*;`) |

### 5.4 Coupling summary

- A: root hub is `World3dState`. Strongest pairs (approximate identifier references): `scene` -> root 683, `overlay` -> root 549, `bridge` -> root 299, `assets` -> root 280, `pointer` -> root 191. Sideways: `authority` -> `registry` 261, `authority` -> `picking` 187, `overlay` -> `scene` 165, `pointer` -> `scene` 115.
- B: sideways pressure is `selection-events` -> types 447, `scene-delete-links` -> types 485 and parent 373, `style` -> parent 338.
- C: `paint` -> types 120 and host 180 (parent `D-HEAD`), `paint` -> `host-selection` 24, `mutate` -> types 70.

---

## 6. Wrong-direction dependencies and concrete-plugin leaks

Ordered by severity.

1. HIGH. B is a generic framework module that names a concrete plugin in code (not only docs). Code-level symbols:
   - `PUZZLE_2D_LODS` L140, `PUZZLE_2D_LOD_SCALE` L149, `pub fn puzzle_2d_lod_scale_json` L197 (exposed to plugins).
   - Node and edge id prefix `puzzle2d.brush.{serial}` L5639-5640.
   - Fill stage action ids `puzzle2d-fill-*` L6662-6687 (26 lines, one per `BoardFillStage` variant).
   - Other puzzle strings at L7044, L7051, L7081, L7142, L12178-12179.
   The `directed_normal` module is re-exported through `pub use board::ports::directed_normal::*;` (lib root L77) for plugins. Move the puzzle2d-specific LOD and fill-action vocabulary to a puzzle2d plugin crate, or parametrize it by the consumer's catalog.

2. HIGH. Infinite mounts a module that belongs to a higher crate. Lib root L22-24 mounts `🔨️modules/🗺️surface/🏔️terrain/🦀️.rs` through `#[path]`, with the comment "surface depends on infinite — avoid cargo cycle". The same source is compiled into two crates, and the duplicated dependency is recorded in the manifest comment. Inverted layering: fix by moving the terrain core into a crate that infinite can depend on, or invert the dependency.

3. MEDIUM. Infinite's tests read renderer source files as text (world tests L236-239, L1379, L1906-1907 read `📺️renderer/.../🧊️wgpu/...` and `🐚️Shell/...`). A framework test depends on renderer file layout. Replace with tests in the renderer crate or with a contract file owned by infinite.

4. MEDIUM. A (world) names plugin products in doc comments: `puzzle3d`, `generation3d`, `lowpoly`, `fem`, `terrarium` (lines 6511, 7751, 8488, 8493, 8668, 9028, 10512, 11599, 12205-12208, 12551-12552, 12623, 13140, 16847, 16871, 16958, 17110, 17156). Code-level symbols in A are generic (`WORLD_BRUSH_MESH_*`). The brush-mesh announce section (16950-17365) describes "the puzzle guest" protocol. Wording fix, low effort. The `WorldBrushMeshAnnounce` protocol itself is product-specific and could be a plugin-owned contract.

5. MEDIUM to LOW. C (dag) owns note-widget vocabulary: `NoteEditState` L1117, `note_widget_size` L588, `note_text_origin_x` L374, `note_*` calls, `begin_note_edit` L3759. If "note" is a plugin (memory notes a Note React boot recipe), the note widget belongs to that plugin. Needs a decision before the split. Not asserted as a violation until checked.

6. LOW. The stub mount `board::normal::{directed,undirected}` (12 and 6 lines) duplicates the naming of `ports::directed_normal`. It is a second "normal" tree. Clean up as part of the split, not as a layering fix.

7. LOW. Naming drift: lib root doc L75 says `infinite_canvas` while the crate is `semio-framework-os-infinite`. Confirm the plugin-side `extern crate` name.

---

## 7. Ordered extraction sequence

Preconditions before any edit: confirm no concurrent edits to A, B, C (git status shows other agents' changes under the ticket folder; do not stash or checkout). Record a baseline `cargo check` for native and wasm32-wasip2 in the ticket `🗑️generated` folder, run by the implementer, not by this audit.

1. C (dag), lowest risk, one impl block.
   a. Move `mod wasm_session` (L5217-5399) to `🌐️wasm-session/🦀️.rs`, keep `use super::*;`, keep the cfg pair.
   b. Move `impl DagHost` slices to `🧭️host-selection`, `🧱️frame-minimap-align`, `🛠️mutate`, `🖌️paint` in that order. Each slice is a complete `impl DagHost { ... }` block between method boundaries. Private helpers used across slices get `pub(super)`.
   c. Re-run the cross-slice check; the paint slice references the most private host helpers.
2. B (board_host), after C.
   a. Decide option (b): unwrap `board_host` into the file root. Replace the `#![allow]` at L6-7 with a file-level one. Rewrite the three function-local `macro_rules!` only if they move out of their function (they do not).
   b. Move slices in this order: `🖱️pointer-gestures` (12426-13830), `🧹️scene-delete-links` (10132-12425), `🎛️selection-events` (7641-10131), `🪣️fill-job` (5669-7640), `🎨️style` (3668-5668), `🧭️host` (2513-3667).
   c. Keep fill types and event queue (root, 1-2512) in the root.
3. A (world), after B. Leaves first, hub last.
   a. `📡️assets/` (16250-17365): few root privates.
   b. `🌉️bridge/` (12494-13871).
   c. `🖱️pointer/` (14721-16249).
   d. `🎛️frame/` (13873-14719), keep `mod tool_run_trace` in the root.
   e. `🖌️overlay/` (10430-12493).
   f. `🧱️scene/` (8334-10428).
   g. `🧵️interaction/` wrapper and children in this order: `⚙️authority` last among them, after `🕹️gumball`, `🧲️picking`, `🔍️marquee`, `📇️registry`. Each step reduces the set of cross-child references that still need `pub(super)`.
   h. `🧹️lifecycle/` (2512-3049) last, because `scene` and `bridge` reference it (about 69 and 42 references).
4. Re-point source-scanning tests (section 4.2) to the new child files in the same change, because they read A, B, C text directly.
5. Path length check: after each move, verify no path reaches 256 characters (AGENTS rule). The deepest proposed path is `🎲️board/🔌️ports/➡️directed/➕️normal/🖱️pointer-gestures/🦀️.rs` under `INF/`. Measure with the real checker; this audit did not measure character counts.

Target parents after the sequence: A root about 2,510 lines; B root about 2,512 lines; C root about 1,598 lines. All under about 3,000.

---

## 8. Child-module convention (repo pattern, verified in this audit)

- Parent: `#[path = "📤️icon-export/🦀️.rs"] mod icon_export;` (verified at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` L1815-1816).
- Child folder next to the parent file, named `<emoji>️<kebab-name>/`, containing `🦀️.rs`. Verified: `📤️icon-export/🦀️.rs`, `⏪️time-travel/`, `📎️local-folders/`, `🪟️tree-windows/`.
- Child file header: `//!` doc whose first character is a unique emoji (verified: `//! 📤️ Frame-pumped icon export batches ...`). Children reference the parent with `use super::{...}` and sibling crate paths with `crate::...`.
- Infinite already uses this pattern: `🌍️world/⏯️tool-run-trace/🦀️.rs` (L14697), `🕸️dag/🎛️input/🦀️.rs` (L10), `🕸️dag/🧹️retirement/🦀️.rs` (L1515), and test mods under `🧪️tests/<emoji-name>/🦀️.rs`.
- Emoji uniqueness within a parent: existing siblings in `🌍️world/`: `🎨️r3f`, `🏔️terrain`, `⏯️tool-run-trace`, `♻️resources`, `🧬️schema`, `🧪️tests`, `🧫️fixtures`. In `🎲️board/`: `🚪️io`, `➕️normal`, `🔌️ports`, `♻️retirement`, `🧬️schema`, `🧪️tests`, `🦀️.rs`. In `🕸️dag/`: `🎛️input`, `🧹️retirement`, `🧪️tests`, `🧫️fixtures`.
- Proposed names in 5.x avoid those emoji where they clash (for example `🖌️overlay` instead of `🎨️overlay`). Verify RGI status with the taxonomy checker before use.
- `🏔️terrain/` already exists under `🌍️world/` without a `🦀️.rs`. The Terrain section (8974-9576) could go there instead of `🧱️scene/`. Option, not decided here.

---

## 9. Limits of this audit

- No compile or test was run (read-only). All cfg and visibility conclusions are from reading and from identifier heuristics, not from rustc.
- The identifier coupling numbers come from an awk heuristic: it counts identifier occurrences, uses the first definition of each name, and does not parse scopes. Use them for ranking, not as exact counts. Scratch: `🗑️generated/seam-scratch/coupling.awk`, `distinct-priv.awk`, `world-proposed-coupling.txt`.
- Dependents of the infinite crate outside its tree were not fully enumerated. Targeted greps over `products/os` timed out because of `node_modules`. Repo-wide Cargo dependents: not verified. Known facts: the artifacts dag crate depends on the presentation sub-crate, not on the monolith; the surface crate depends on infinite (per comment at lib root L22).
- Scratch files are in `🗑️generated/seam-scratch/` of this monolith-split folder. Ticket tooling should clean them up. Nothing outside the ticket folder was written except one temporary file at `/tmp/x`, created and removed during the audit.
- The `🪓️monolith-split/📓️FLEET.md` file was not read (not in scope; written by another agent at 01:36).
