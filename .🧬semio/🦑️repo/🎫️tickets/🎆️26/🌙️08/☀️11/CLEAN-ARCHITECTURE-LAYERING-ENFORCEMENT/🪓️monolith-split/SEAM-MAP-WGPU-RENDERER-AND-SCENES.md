# 🪓️ Seam Map: wgpu Renderer (19,807 lines) and Scenes (12,343 lines)

Read-only audit. No source file was edited and no cargo, nx or build was run. Intermediate outputs (scripts and `seam-*.txt`) live in `🗑️generated/` beside this file and are to be deleted when the ticket closes.

Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` (abbreviated `E/`).

- **A** = `E/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (19,807 lines, `wc` reports 19,807, the file has no trailing blank line).
- **B** = `E/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (12,343 lines).
- Crate manifest: `E/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`, `[lib] path = "📚️library/🦀️.rs"`.

Verification status:
- Line ranges come from `grep -n` and `awk`. A Python overlap and coverage check reports 0 overlaps and 0 uncovered non-blank lines for both files.
- Every `#[path]`, `include!`, `include_str!` and `include_bytes!` literal (58 in A, 16 in B) was resolved against the disk. The new literal (one extra `../` per directory level added) resolves to the same file for every occurrence. 0 mismatches.
- Coupling is token-level (whole-word identifier match). It over-approximates: names that also occur in comments or strings are counted. Method-level coupling is name-only. The compiler is the final arbiter and is not run here.

---

## 1. Wiring (verified)

1. **A is textually included into the crate root, not declared as a module.** `📦️packages/🦀️rust/📚️library/🦀️.rs:11-12`:
   ```
   #[cfg(not(target_os = "wasi"))]
   include!("../../../🧊️renderer/🦀️.rs");
   ```
   Consequences:
   - Every top-level item of A is a crate-root item (`crate::name`). Private items at crate root are visible to the whole crate.
   - A's `use` block (lines 219–266) is a crate-root import block. Child modules do **not** inherit it.
   - Every `#[path]` written inside A resolves relative to A's directory (`E/🎯️targets/🧊️wgpu/🧊️renderer/`). This is confirmed by the existing literals, e.g. A line 3 `../../../🧪️tests/...`.
   - The `cfg(not(target_os="wasi"))` gate on the `include!` applies to everything A declares, so children declared inside A inherit the WASI exclusion. No cfg change is needed for them.
   - The lib root already mounts a child of A's directory: line 4, `#[path = "../../../🧊️renderer/📇️registry/🦀️.rs"] mod renderer_registration;`. Existing siblings in A's directory: `📇️registry/`, `🪶️sqlite/`.

2. **B is a real module.** A line 113: `#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod scenes;`. Its own `#[path]` literals resolve relative to B's directory (`E/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/`).

3. **Child-module convention (verified in the Shell element).** `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` lines 1815–1816:
   ```
   #[path = "📤️icon-export/🦀️.rs"]
   mod icon_export;
   ```
   The child `📤️icon-export/🦀️.rs` begins with a `//!` line whose first character is the folder emoji. The path is relative to the parent file's directory, and the child folder sits next to the parent file.
   Caveat: the child's own import on its line 3 reads `use crate::scenes::icon_export::...`, which does not match its mount point (`shell::icon_export`). Do not copy that line.

4. **Child path depth rule.** A `#[path]`, `include!`, `include_str!` or `include_bytes!` literal moved from a file into a child file one directory deeper gains one `../` prefix. A child two levels deep (under `📦️kernel-runtime/<child>/`) gains two. Applied and validated in section 7.

5. **Public API surface.** Free `pub` items in the partitions that move: A has 6 (`scale_bench`, `run` in it, `run_socket_grant_probe`, `run_native`, `run_smoke`, `upload_icon_atlas`). B has 109 in the Scenes partitions (full list in appendix B). Each must keep its path via a `pub use` re-export in the parent (A's root or B's root `scenes`).

---

## 2. File A: top-level sections (current)

Columns: line range (inclusive), section, contents, line count.

| Lines | Section | Contents | Count |
|---|---|---|---|
| 1–267 | header | `extern crate` aliases (25–32), `#[path]` `mod` declarations (36–217: dock, mesh_assets, agent_*, hub_*, canvas_presence, engine_canvas, interpreter, media_slots, program_bridge, plugin_catalog, scenes, canvas2d_gumball/paint, native_temporal, shell, icon_atlas, deadlines, frame_latency, os_host, native_caller, render_snapshot, browser_original_output, runtime_mailbox_core, frame_job, frame_authority, surface_lane, input_wire, socket_door, directory_door, browser_worker, winit_app, native_accessibility, parallel_runtime), `cfg(test)` fixtures and `include!` (2–14), crate-root `use` block (219–266) | 267 |
| 268–1020 | `//#region 📡️RendererAssetAuthority` (part) | renderer asset statics (269–270), request/response/seal functions (272–329), `RendererAssetFetchOwner` (339–405), size consts (406–421), `RendererAssetPageIndex` / `PageCursor` / `FormatCursor` (423–583), GLB/PNG/JPEG/text/protobuf structure cursors (584–1009), `protobuf_varint_byte` (1010) | 753 |
| 1021–1216 | GLB JSON lexer | consts (1021–1023), `GlbJsonAtom` (1026–1068), token/lex enums (1069–1087), `GlbJsonTokenCursor` (1088–1216) | 196 |
| 1217–1387 | GLB schema structs | `GlbAccessorSchema` … `GlbPrimitiveSchema`, material/texture/sampler/mesh/node/scene schemas, `GlbNumericArray`, `GlbSchemaSection` | 171 |
| 1388–1616 | GLB schema output | `GlbSchemaOutput` struct (1388) and impl (1418–1616) | 229 |
| 1617–2249 | GLB schema cursor | `GlbSchemaCursor` struct (1617) and impl (1648–2249, large state machine), `exact_u16` (2250) | 633 |
| 2257–2459 | GLB instance plan | `GlbInstanceOutput`, `GlbNodePlanFrame`, `GlbInstancePlanCursor` (impl 2301–2459) | 203 |
| 2460–2535 | GLB math | `glb_world_frame`, matrix helpers, TRS, transforms, normalize | 76 |
| 2536–2635 | GLB outline | `GLB_OUTLINE_SCALE`, edge key/record, `GlbOutlineAccumulator` impl | 100 |
| 2636–3333 | GLB materialize | `GlbMaterializePhase`, `GlbMaterializeCursor` impl (2689–3333) | 698 |
| 3334–3435 | GLB helpers | `glb_texture_wrap`, `glb_mag_filter`, `glb_min_filter`, `glb_mesh_fault`, `glb_component_bytes`, `glb_read_component` | 102 |
| 3436–3677 | renderer asset probe | `RendererAssetProbePhase`, `RendererAssetProbe` impl (3473–3677) | 242 |
| 3678–4003 | decode job and session | `RendererAssetDecodeResult/Turn/TurnKind`, `RendererAssetDecodeJob` + `InteractiveJob` impl (3729), `RendererAssetDecodeWake` (3800–3813), `RendererAssetDecodeSession` impl (3814–4003) | 326 |
| 4004–4027 | prefix validators | `renderer_asset_svg_prefix_is_valid`, `…ui_image…`, `…reference…` | 24 |
| 4028–4029 | region end | `//#endregion 📡️RendererAssetAuthority` | 2 |
| 4030–4406 | `//#region 🧵️RendererWorkerPool` | `renderer_worker_pool` (4032–4042), `RendererIoHandle` + Future + Drop (4043–4108), IO slots and nodes (4109–4276), IO statics (4277–4283), `renderer_io_with_node` / `pump_renderer_io_sessions` / `submit_renderer_io` (4284–4397), `cfg(all(test))` mod (4403–4405) | 377 |
| 4407–4420 | kernel doc | `//#region 🎠️KernelRuntime` and its doc block | 14 |
| 4421–4423 | kernel header | `#[cfg(not(target_arch = "wasm32"))]`, `#[path = "."]`, `pub(crate) mod kernel_runtime {` | 3 |
| 4424–4446 | kernel use block | 23 `use` lines, private to `kernel_runtime` | 23 |
| 4447–4475 | kernel native read | `pub(super) async fn read_native_component` | 29 |
| 4476–4820 | kernel core | renderer sequence authority (4476–4547), job progress presentation (4548–4720), `next_seq`, `decode_actor_turn_result` (4721–4760), `TURN_BUDGET` and consts (4761–4783), test static (4784–4785), turn helpers (4795–4820) | 345 |
| 4821–4901 | `🔖️ExtensionIndex` | `PluginDescriptorJson`, `ExtensionRecord`, `ExtensionIndex`, `find_wasm_artifact` | 81 |
| 4903–5164 | `🔖️Requests/Outcomes` | `CreateAppRequestOwner`, `QueuedKernelEvent(Kind)`, `RejectedKernelEvents`, `KernelRequest` | 262 |
| 5165–5351 | kernel close | `KERNEL_CLOSE_*` states, `KernelCloseStatus`, `KernelCloseSubmissionRegistry`, `KernelCloseSubmission` (+`Wake`), `KernelCloseHandle` | 187 |
| 5352–6282 | mounted product replay | `PRODUCT_REPLAY_*` consts, `RawSpawnJobOwner`, replay recovery registry, `MountedProductReplayRequest`, refusal registry, admission, claim, authority | 931 |
| 6283–6373 | `📬️TypedOperationResultPage` | page and token, `cfg(test)` mod (6369–6371) | 91 |
| 6375–6471 | exchange outcome | `ExchangeSurfaceDocument`, `ExchangeOutcome`, `KernelOutcome` | 97 |
| 6472–6553 | `🔖️KernelFuture` | `ResponseSlot`, `KernelFuture` and `Future` impl | 82 |
| 6555–6736 | `🔖️KernelClient` | `KernelClient` global client and impl | 182 |
| 6738–6753 | `📄️RetainedDocumentExchange` | `RetainedPatchApply`, `RetainedDocumentBuild` | 16 |
| 6755–7131 | retained surface | `RetainedSurface` impl (6772–7055), slots and registry, `RETAINED_SURFACE_CAPACITY` | 377 |
| 7133–7220 | pending rejection | `PendingSurfaceRejection(Registry)` | 88 |
| 7221–7461 | mounted replay routes | `PendingJobProgressPresentation`, `MountedReplayRouteSeed`, `MountedJobReplay` impl, recovery registry, `ClosingKernelApp` (7445–7461) | 241 |
| 7462–7604 | `📦️CommandDocumentRetirement` | capacity const, reservation tokens, `CommandDocumentRetirementRegistry` impl | 143 |
| 7605–7673 | pool state head | `ReservedToolJob`, `KernelPoolState` struct (7619) | 69 |
| 7674–9503 | `impl KernelPoolState` | pool logic (1,830 lines) | 1,830 |
| 9504–9755 | request queue | `KERNEL_REQUEST_QUEUE_CAPACITY`, `KernelRequestQueue(State)` and impl | 252 |
| 9756–9920 | pool futures and runner | `KernelPoolFuture` + `Wake` (9756–9806), `yield_kernel_maintenance_turn`, `run_kernel_pool` (9821), second `impl KernelPoolState` (9851–9920) | 165 |
| 9921–9923 | kernel test include | `#[cfg(test)] include!(…semantic-document…)` (9921–9922), `//#endregion` | 3 |
| 9924 | kernel close brace | `}` closing `mod kernel_runtime` | 1 |
| 9925 | region end | `//#endregion 🎠️KernelRuntime` | 1 |
| 9927–9944 | `🔖️ActorBudgetBridge` | `actor_budget_from_turn_budget` (9941) | 18 |
| 9946–10841 | `🔖️ScaleBench` | `pub mod scale_bench { … }` (9958–10840) with sub-regions Registry JSON, Row, Env, Budget2–Budget8 tests | 896 |
| 10842–10916 | task and log | `spawn_app_task` (10858/10866), `log_debug_diagnostic_once_per_transition`, `log_debug_once_per_transition`, `log_debug_diagnostic`, `log_debug` (10908/10913) | 75 |
| 10917–11005 | `🌓️HostAppearance` | `HostAppearancePreference`, `HostAppearance`, appearance setters/getters | 89 |
| 11007–11028 | `🎓️HostIntroduction` | suppression flag | 22 |
| 11030–11125 | `⌨️HostPlatform` | keybinding platform functions, `compiled_host_platform`, platform setters | 96 |
| 11127–11191 | `🗣️HostLocale` | `host_locale_from_tag`, setters, `resolve_theme`, `appearance_is_dark`, `app_now_ms` (wasm/native) | 65 |
| 11193–11222 | test mod declarations | five `cfg(test)` `#[path]` mods (async_boundary, frame_action_ledger, settle_pump, wheel_application_point, boot_axis_parity) | 30 |
| 11223–11764 | `📮️RuntimeMailbox` (part) | `RuntimeDispatchCursor` (11225) and dispatch source, `FrameActionEnvelope`, `FrameActionReceiptOwner`, `FrameActionBatchOwner`, `FrameActionOwners` impl (11396–11581), `FrameInputActionStep`, `FrameDeferredCursor` impl (11672–11764) | 542 |
| 11765–12019 | frame maintenance | `FrameMaintenanceOwner`, `FrameDeferredExecutionOwner`, execution cell and registry, `FrameMaintenanceAuthority` (11974) | 255 |
| 12020–12326 | `RuntimeApply` | `RuntimeApply` enum and impl (12041–12326) | 307 |
| 12327–12470 | presentation authorities | `RUNTIME_COMPLETION_CAPACITY`, `RuntimePresentationWitness`, `RuntimePresentationAuthority` impl, `RuntimeRasterOperationAuthority` impl | 144 |
| 12471–12514 | completion types | `RuntimeHostWaker` (cfg pair), `RuntimeCompletion`, `RuntimeCompletionQueue`, `returned_completion` | 44 |
| 12488–12490 | test mod | `runtime_publication_tests` (cfg test, inside 12471–12514 block) | 3 |
| 12500–12514 | enqueue | `enqueue_runtime_completion` | 15 |
| 12515–12610 | staged reference image | `StagedReferenceImage`, `StagedReferenceImageAuthority` impl | 96 |
| 12611–12838 | native reference decode | `NativeReferenceDecodeOutput/CloseStep/Job`, `NativeReferenceDecodeJob` impl | 228 |
| 12839–12917 | native asset transport | `NativeAssetHandoff`, `NativeAssetTransportLease/CloseStep/Guard`, `NativeAssetStreamFailure` + From impls, test barrier (12871) | 79 |
| 12918–12992 | mailbox state | `RuntimeMailboxInner` (+impl), `pub(crate) struct RuntimeMailbox(Arc<…>)` | 75 |
| 12994–13160 | deferred envelopes | `FrameMaintenanceExecutionEnvelope/Guard`, `RuntimeDispatchReturn`, `FrameDeferredExecutionEnvelope/Guard` | 167 |
| 13161–13168 | interaction step | `PresenterInteractionStep` | 8 |
| 13169–14772 | `impl RuntimeMailbox` | the single large mailbox impl (1,604 lines) | 1,604 |
| 14773–14849 | app handles | `AppHandle`, `AppTextStream`, `AppRuntime` struct, `Deref` / `DerefMut` | 77 |
| 14811–14835 | interaction state | `AppInteractionState` struct | 25 |
| 14850–14940 | `impl AppRuntime` (part 1) | small impl | 91 |
| 14941–15018 | frame engine packets | `FRAME_ENGINE_PACKET_*` consts, `FrameEnginePackets` impl | 78 |
| 15019–15046 | frame build structs | `AppFrameBuild`, `AppFrameAfterChrome` | 28 |
| 15047–15256 | frame build cursors | `FrameBuildCursor` (+phase, impl 15113–15207), `FrameFinishCursor` (+impl) | 210 |
| 15257–15369 | frame boundary fns | `frame_prepared_abandonment_complete`, `frame_original_primitive_close`, `frame_child_step`, `frame_inline_*`, `frame_worker_child_step`, `frame_receive_close`, `frame_received_child` | 113 |
| 15370–16156 | frame transaction | `FrameTransaction`, `AppFrameTransactionStep`, `FrameTransactionStage`, `AppFrameTransactionPhase`, `impl FrameTransaction` (15450–16156) | 787 |
| 16157–16227 | presentation and build impl | `AppFramePresentation` struct (16157), `discard_frame_input_candidate` (16171), `impl AppFrameBuild` (16182–16227) | 71 |
| 16228–16453 | frame preparation | `FramePreparationStep`, `AppFramePreparation` struct and impl | 226 |
| 16454–16659 | presenter state | `AppPresenter` struct (16454), stall watch, input wait, progress tuple, `AppSurfaceResizePhase`, `AppPresentPhase`, `AppPresentCursor`, `AppPresentedRetirement` struct (16622), `RasterCandidateRetirement`, `AppPresentStep` | 206 |
| 16660–16832 | presentation impls | `impl AppFramePresentation` (16660), `impl AppPresentedRetirement` (16703–16832) | 173 |
| 16833–17654 | `impl AppPresenter` | presenter impl (822 lines) | 822 |
| 17655–17801 | native streams | const assert, `native_renderer_asset_path`, `stream_native_renderer_asset`, `push_renderer_asset_page`, `stream_native_renderer_http_asset`, `world_rejection_fault` | 147 |
| 17802–18398 | `impl AppRuntime` (part 2) | runtime impl (597) | 597 |
| 18399–18880 | `impl AppInteractionState` | interaction impl (482) | 482 |
| 18881–18898 | region | `//#region 🔖️OsHostDecomposition — SemioApp deletion` | 18 |
| 18899–18984 | world3d traces | `step_world3d_camera_fit_after_snapshot`, `world3d_ingest_trace`, `world3d_interaction_trace` | 86 |
| 18985–19108 | boot | `boot_runtime` (native, async) | 124 |
| 19109–19334 | socket probe types | `NativeSocketProbeSnapshot`, `NativeSocketProbeDiff`, `NativeSocketProbeMutation`, `cfg(test)` mods 19113–19119 | 226 |
| 19335–19423 | socket probe run | `pub async fn run_socket_grant_probe` | 89 |
| 19424–19457 | native runners | `pub fn run_native`, `pub async fn run_smoke` | 34 |
| 19458–19526 | smoke with axes | `pub(crate) async fn run_smoke_with_axes` | 69 |
| 19527–19541 | atlas upload | `pub fn upload_icon_atlas` (wasm) | 15 |
| 19542–19803 | `🧭️BootDescriptor` | `WGPU_BOOT_FIELD_CAPACITY`, `WgpuBoot*` structs, `apply_boot_descriptor`, `boot_*` getters, runtime diagnostics | 262 |
| 19805–19807 | wasm export | `semio_wgpu_set_plugin_catalog` (wasm) | 3 |

Rows are listed by starting line. Rows that are nested inside another row (test mods, `cfg(test)` items, the struct and impl of one type) are noted in their own text. Section 4 uses contiguous ranges with no gaps.

Region-marker hazards in A: `//#region 📮️RuntimeMailbox` (11223) is closed by `//#endregion 🎮️AppInteractionState` at 14939, which is a mismatched label. The AppFrame/Presenter block 14941–18880 has no region marker of its own.

---

## 3. File B: top-level sections (current)

| Lines | Section | Contents | Count |
|---|---|---|---|
| 1–28 | header | `cfg` (20), `use` block (21–28, 7 lines) | 28 |
| 29–316 | `//#region SceneRuntime` | `AdmittedSurfaceMap` (generic map, 30–316, impl 84–307), tokens/faults/rejections (30–66) | 288 |
| 317–319 | test mod | `admitted_surface_map_tests` (`#[path]` 318) | 3 |
| 322–386 | viewport and drag | `Viewport` (322–353), `SceneDragMode`, `SceneDrag`, `InkEditKind`, `InkEditState` | 65 |
| 387–489 | host temporal | `HostTemporalPresentation` impl (402–489) | 103 |
| 490–587 | accessibility controls | `VfsAccessibilityControl(Kind)`, `BlockList/EventFeed/World3d/GraphTimelineAccessibilityControl`, `SceneAccessibilityPresentation` impl | 98 |
| 588–708 | scene surface state | `SceneSurfaceState`, `SceneValueRetirement`, `SceneSurfaceRetirement`, retire helpers (662–708) | 121 |
| 709–1064 | `impl SceneSurfaceRetirement` | retirement logic (356) | 356 |
| 1066–1078 | retired close | `close_retired_scene_surface_one` | 13 |
| 1079–1156 | canvas gestures | `CanvasClick`, `CanvasGestureKind`, `CanvasGesture`, `CanvasGestureSlots` impl, `CanvasCatalogueHover` | 78 |
| 1157–1556 | pending raster (no region marker) | `RASTER_*` consts, `RasterUploadRefusal`, `PendingRasterQueue` impl, `PendingRasterSurface`, `PendingRasterUploadStep/CheckedOut/Cursor`, `PendingRasterSurfaceRetirement`, `PendingRasterAuthorityClose` | 400 |
| 1557–1610 | cell and statics | `WorkerCell<T>` (1559–1581), statics `SCENE_STATE`, `SCENE_SURFACE_RETIREMENT`, `CANVAS_GESTURE`, `CANVAS_CATALOGUE_HOVER`, `PENDING_RASTER_*`, `SCENE_CAMERA_DISPATCH_*` (1595–1610) | 54 |
| 1612–1631 | native temporal formatter | `NativeHostTemporalFormatter` trait, install and get | 20 |
| 1633–1675 | scene list transfer | `SceneListTransferSource/Session/Authority`, `SCENE_LIST_TRANSFER` (1673) | 43 |
| 1676–1724 | VFS pub fns | `toggle_vfs_row_expanded`, `seed_vfs_expanded`, `vfs_selection_for_click` | 49 |
| 1725–1851 | scene identity | `scene_host_retiring`, `scene_surface_retirement_can_admit`, `mount_scene_identity`, `bind_scene_engine_token`, `mount_canvas_camera`, `retire_scene_identity` | 127 |
| 1852–1864 | state accessors | `scene_state`, `mutate_scene_state` | 13 |
| 1865–2064 | `//#region SceneCameraDispatch` | consts, `SceneCameraOwnerMetadata`, camera record fns, `schedule_scene_camera_dispatch`, `SceneCameraDispatchStep/Cursor` impl | 200 |
| 2062–2063 | cfg test include | `include!("../../🧪️tests/🧊️wgpu-standalone/🦀️.rs")` | 2 |
| 2066–2195 | camera helpers and misc | `scene_pointer_edge_state`, scroll key/offset, `scene_action`, `now_ms` (wasm/native), theme, action byte helpers, list row helpers | 130 |
| 2196–2893 | `//#region SceneInput` (part 1) | canvas pointer wire and encodings (2206–2300), `canvas_pointer_move_into` (2521), `canvas_pointer_button_into` (2590), cancellation (2321–2520, 2406–2508), catalogue drag (2709–2893) | 698 |
| 2894–3061 | `//#region SceneRowTransfer` | drag start/arm/release, `SceneListTransferRelease` | 168 |
| 3062–3264 | `SceneInput` (part 2) | `SceneModifiers` (3067–3109), `passive_scene_pointer_button` (3110), move (3189), `SceneListHit`, list hit, double click | 203 |
| 3266–3476 | `//#region 🖱️SurfaceContextMenu` | `SceneContextMenuTarget`, `scene_context_menu_target` and per-kind targets, test mod (3443–3445), `context_menu_surface_kind_id` (3456) | 211 |
| 3477–3969 | `//#region RenderEntry` | `render_placeholder`, chrome label structs, `SceneEngineHosts`, `render_component_scene_step` (3556), list, canvas2d, ink, icon, world3d render steps, debug logging, test mod (3966–3968) | 493 |
| 3971–4038 | `//#region Paint2d` (+Navigator) | cfg-test paint2d helpers (3976–4016), navigator padding | 68 |
| 4040–4782 | `//#region Table` | `TableColumn`, `TableCellPayload`, table helpers, focus/commit/accessibility cells, `render_table` (4692) | 743 |
| 4784–4788 | `//#region TableTests` | `table_tests` (cfg test, 4786) | 5 |
| 4790–5274 | `//#region BlockList` (+BlockListTests) | `BlockListPaint/Role/Target/Plan`, `block_list_plan` (4870), transfer, `render_block_list`, tests (5272) | 485 |
| 5276–5545 | `//#region DiffView` (+DiffViewTests) | diff view rendering and metrics, tests (5543) | 270 |
| 5547–6138 | `//#region EventFeed` (+tests) | event feed model and rendering, tests (6136) | 592 |
| 6140–6538 | `//#region GraphTimeline` (+tests) | graph timeline model and rendering, tests (6536) | 399 |
| 6540–7715 | `//#region Canvas2d` | Canvas2d paint, `Canvas2dFillBlend` (7610), `Canvas2dShapes` (7674), tests (7711–7713) | 1,176 |
| 7717–9778 | `//#region InkCanvas` (model and state) | `InkCanvasModel` (7720–8165), `InkCanvasState` (8167–9777), ink item and camera JSON, `InkInteractionJob` | 2,062 |
| 9779–10128 | `InkCanvasRender` (+raster cost, ink tests) | `render_ink_canvas`, `RasterFrameCostTests` (10117), `InkCanvasTests` (10123) | 350 |
| 10129–10141 | `//#region NodeGraph` | `NodeGraphSurface`, render glue | 13 |
| 10143–10588 | `//#region TiledMap` | `TiledMapSurface`, pointer and touch handling, context menu target | 446 |
| 10589–11040 | `//#region IconRender` (+tests) | icon render frame and layout, `icon_render` ingress, path include (11034), tests (11038) | 452 |
| 11042–11224 | `//#region Board2d` | `Board2dSurface`, pointer/touch/wheel handlers, `Puzzle2dSelectionMenu` (11143–11220) | 183 |
| 11226–11762 | `//#region VirtualFileSystem` (+tests) | VFS model, schema, hit testing, rendering, tests (11760) | 537 |
| 11764–12343 | `//#region TextEditor` | `State` (11765–11892, no end marker), `Popups` (11893–12029), `PopupDrive` (12031–12338), tests (12341) | 580 |

Region-marker hazards in B: the `//#region State` at 11765 has no end marker of its own (the TextEditor region closes at 12343 which covers `State`, `Popups` and `PopupDrive`). `Puzzle2dSelectionMenu` (11143) is nested inside `Board2d`.

---

## 4. Proposed partitions

### 4.1 File A (19,807 lines)

Target: each parent below about 3,000 lines. After the split the parent `🧊️renderer/🦀️.rs` is about 930 lines plus new `#[path]` declarations.

Child folders sit next to A (`E/🎯️targets/🧊️wgpu/🧊️renderer/<folder>/🦀️.rs`). Emojis were checked for uniqueness within that directory (existing: `📇️registry`, `🪶️sqlite`). All are RGI emoji with U+FE0F, the form used by the repo.

| Id | Folder (A-relative) | Module | Lines | Range (inclusive) | Items |
|---|---|---|---|---|---|
| P0 | (parent, `🧊️renderer/🦀️.rs`) | crate root | 930 | 1–267, 9927–9944, 10842–11222, 19542–19807 | header and `mod` decls; `ActorBudgetBridge`; task/log, host appearance/introduction/platform/locale, `app_now_ms`, test mod decls; `BootDescriptor` |
| A1 | `🗂️asset-authority/` | `asset_authority` | 753 | 268–1020 | renderer asset statics, request/seal fns, fetch owner, size consts, page/format/structure cursors |
| A2 | `🦴️glb-decode/` | `glb_decode` | 2,415 | 1021–3435 | GLB JSON lexer, schema, instance plan, math, outline, materialize, helpers |
| A3 | `🔁️asset-decode-jobs/` | `asset_decode_jobs` | 971 | 3436–4406 | asset probe, decode job/session, prefix validators, `renderer_worker_pool`, IO handles/slots/pump, `cfg(test)` mod |
| K0 | `📦️kernel-runtime/` | `kernel_runtime` (file root) | 68 | 4408–4475 | kernel doc, `#[cfg(not(wasm32))]`, use block (4424–4446), `pub(super) read_native_component`; declares K1–K4 |
| K1 | `📦️kernel-runtime/🧮️kernel-core/` | `kernel_core` | 876 | 4476–5351 | renderer sequence, job progress presentation, `next_seq`, turn budget consts, `ExtensionIndex`, requests/outcomes, close submission |
| K2 | `📦️kernel-runtime/📬️kernel-replay-client/` | `kernel_replay_client` | 1,386 | 5352–6737 | product replay, typed-result page (+test mod 6369), exchange outcome, `KernelFuture`, `KernelClient` |
| K3 | `📦️kernel-runtime/🗃️kernel-retained-state/` | `kernel_retained_state` | 936 | 6738–7673 | retained document exchange, retained surface and registry, pending rejection, mounted replay routes, command document retirement, pool-state head struct |
| K4 | `📦️kernel-runtime/🏊️kernel-pool/` | `kernel_pool` | 2,252 | 7674–9923 and 9925 | `impl KernelPoolState` (7674–9503 and 9851–9920), request queue, `KernelPoolFuture`, `run_kernel_pool`, kernel semantic test include (9921–9922). Closing brace 9924 is removed. |
| S1 | `📊️scale-bench/` | `scale_bench` | 896 | 9946–10841 | `pub mod scale_bench`: registry JSON, row/env, budget tests (keep `pub`) |
| R1 | `🛎️frame-actions/` | `frame_actions` | 1,104 | 11223–12326 | dispatch cursor, frame action envelope/owners, input action step, deferred cursor, maintenance owners and authority, `RuntimeApply` |
| R2 | `🎨️presentation-raster/` | `presentation_raster` | 842 | 12327–13168 | presentation and raster authorities, completion types, `RuntimeMailboxInner`, `RuntimeMailbox` struct, deferred envelopes, `PresenterInteractionStep`, staged reference image, native reference decode, native asset transport (+test mod 12488–12490, test barrier 12871) |
| R3 | `📮️mailbox-impl/` | `mailbox_impl` | 1,604 | 13169–14772 | `impl RuntimeMailbox` |
| R4 | `🧾️frame-build/` | `frame_build` | 1,455 | 14773–16227 | `AppHandle`, `AppTextStream`, `AppRuntime` struct, `AppInteractionState` struct, `impl AppRuntime` (part 1), frame engine packets, build cursors, boundary fns, `FrameTransaction` (15370–16156), `AppFramePresentation` struct, `impl AppFrameBuild` |
| R5 | `🖥️frame-presentation/` | `frame_presentation` | 1,427 | 16228–17654 | `AppFramePreparation`, `AppPresenter` struct and state types, `impl AppFramePresentation`, `impl AppPresentedRetirement`, `impl AppPresenter` |
| R6 | `🧠️app-runtime/` | `app_runtime` | 1,454 | 17655–19108 | native asset streams, `impl AppRuntime` (part 2), `impl AppInteractionState`, `SemioApp` deletion region, world3d traces, `boot_runtime` |
| R7 | `🔬️native-socket-probe/` | `native_socket_probe` | 433 | 19109–19541 | socket probe types and run fns, `run_native`, `run_smoke`, `run_smoke_with_axes`, `upload_icon_atlas` (wasm) |

Coverage check: the listed ranges plus P0 sum to 19,808 lines, which is 19,807 plus the file's terminal newline counted by the script. No gaps, no overlaps.

Parent `#[path]` declarations to add to A (after line 217, same cfg lines as the code they replace):
- `#[path = "🗂️asset-authority/🦀️.rs"] mod asset_authority;` and the same pattern for every A and R child (`R`s use `pub(crate)` where the item is used cross-partition, see 6).
- K0: `#[cfg(not(target_arch = "wasm32"))] #[path = "📦️kernel-runtime/🦀️.rs"] pub(crate) mod kernel_runtime;` (replaces lines 4421–4423; the `#[path = "."]` and inline body are removed).
- S1: keep `#[cfg(not(target_arch = "wasm32"))] pub mod scale_bench;` with its doc block (9947–9956) as outer `///` on the declaration.
- R7: keep `#[cfg(not(target_arch = "wasm32"))]` gates on the items inside its child file; the module declaration itself is ungated.

### 4.2 File B (12,343 lines)

Parent `Q0` (the B file after the split) keeps 1–28 (header), 1557–1864 (cell, statics, temporal formatter, transfer authority, VFS fns, identity fns, state accessors) and new `mod` declarations. About 340 lines.

Child folders sit next to B (`E/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/<folder>/🦀️.rs`). The directory currently holds only `🦀️.rs`, so there is no collision.

| Id | Folder (B-relative) | Module | Lines | Range (inclusive) | Items |
|---|---|---|---|---|---|
| Q1 | `🗃️scene-state/` | `scene_state` | 1,128 | 29–1156 | `AdmittedSurfaceMap` and tests (317–319), `Viewport`, drag/ink state, `HostTemporalPresentation`, accessibility controls, `SceneSurfaceState`, `SceneSurfaceRetirement` impl, canvas gesture types |
| Q2 | `🪣️pending-raster/` | `pending_raster` | 400 | 1157–1556 | pending raster queue and surface types, upload cursor, retirement, authority close. Renamed from 🖼️ (see 4.3). |
| Q3 | `📷️scene-camera-dispatch/` | `scene_camera_dispatch` | 331 | 1865–2195 | camera dispatch region, `cfg(test)` include (2062–2063), pointer edge state, scroll helpers, `scene_action`, `now_ms`, theme, action byte helpers, list helpers |
| Q4 | `⌨️scene-input/` | `scene_input` | 1,070 | 2196–3265 | canvas pointer wire, move/button/wheel/catalogue, row transfer, `SceneModifiers`, passive pointer, list hit, double click |
| Q5 | `🎨️scene-render-entry/` | `scene_render_entry` | 704 | 3266–3969 | surface context menu (+test 3443–3445), render entry (+test 3966–3968) |
| Q6 | `🧮️table-and-paint2d/` | `table_and_paint2d` | 819 | 3970–4788 | `Paint2d` helpers, `Table` and `TableTests` (test 4786) |
| Q7 | `🧱️block-list-and-diff/` | `block_list_and_diff` | 757 | 4789–5545 | `BlockList` (+tests 5272), `DiffView` (+tests 5543) |
| Q8 | `📰️event-feed-and-timeline/` | `event_feed_and_timeline` | 993 | 5546–6538 | `EventFeed` (+tests 6136), `GraphTimeline` (+tests 6536) |
| Q9 | `🖌️canvas2d/` | `canvas2d` | 1,177 | 6539–7715 | `Canvas2d` (+fill blend, shapes, tests 7711) |
| Q10 | `✒️ink-canvas-model-state/` | `ink_canvas_model_state` | 2,063 | 7716–9778 | `InkCanvasModel`, `InkCanvasState` |
| Q11 | `🖋️ink-canvas-render/` | `ink_canvas_render` | 350 | 9779–10128 | `InkCanvasRender`, raster cost tests (10117–10121), ink tests (10123–10127) |
| Q12 | `🧭️node-graph-and-tiled-map/` | `node_graph_and_tiled_map` | 460 | 10129–10588 | `NodeGraph`, `TiledMap` |
| Q13 | `🖼️icon-render/` | `icon_render` | 453 | 10589–11041 | `IconRender` (+path include 11034, tests 11038) |
| Q14 | `🧩️board2d-and-vfs/` | `board2d_and_vfs` | 721 | 11042–11762 | `Board2d` (+`Puzzle2dSelectionMenu`), `VirtualFileSystem` (+tests 11760) |
| Q15 | `✍️text-editor/` | `text_editor` | 581 | 11763–12343 | `TextEditor` (`State`, `Popups`, `PopupDrive`, tests 12341) |

Coverage check: 12,343 lines, covered with no gaps and no overlaps (Q0 + Q1–Q15).

Parent `scenes` declarations to add: `#[path = "📷️scene-camera-dispatch/🦀️.rs"] mod scene_camera_dispatch;` and the same for each Q (keeping the existing `#[cfg]`s, none in B except the top `cfg` at line 20, which is a file-level gate and stays in Q0).

### 4.3 Naming correction

`🖼️pending-raster` was considered for Q2 but `🖼️` is already used by Q13 (`🖼️icon-render`) in the same directory. Q2 uses `🪣️pending-raster`.

---

## 5. Coupling

Counts from the token-level analysis (`🗑️generated/seam-free.txt`, `seam-external.txt`):

- A, cross-partition free-item edges: 346. Densest hubs: `RuntimeMailbox` (R2 owns the type, R3 owns its impl), `AppRuntime` (R4 owns the struct, R6/R5/R2 use it), `KernelPoolState` (K3 type, K4 impl), `ResponseSlot`/`KernelClient` (K2), `renderer_worker_pool`/`run_renderer_io` (A3, used by K1, K4, R1, R3, R5, R6).
- A, external references: many free items are referenced by test files that are `include!`d or `#[path]`-mounted into the crate root. Those tests refer to crate-root names unqualified (e.g. `AppInteractionState`, `AppPresenter`, `AppRuntime`). The export list in appendix A covers them.
- B, cross-partition free-item edges: 271. Hub: the statics and accessors in Q0 (`SCENE_STATE`, `mutate_scene_state`, `scene_state`, `WorkerCell`). Children reach Q0 items through descendant visibility (no change needed). Child-to-child edges (e.g. Q4 → Q3 `now_ms`, Q1 → Q0 `CANVAS_CATALOGUE_HOVER`, Q5 → all) need the visibility rule in section 6.
- B items used from A: `scenes` is `pub mod` in A and A's code imports `crate::scenes::AdmittedSurfaceToken` (A line 219). Whatever Q0 re-exports keeps that path.

Per-partition import lines from A's crate-root `use` block (219–266) that each child needs, from the word-match analysis. Copy only these lines, then delete unused ones after the compiler reports them.

| Child | Needed header `use` lines (A 219–266) |
|---|---|
| A1 | 219, 243, 244, 245, 250, 251 |
| A2 | 219, 220, 222, 224, 225, 226, 233, 241, 243, 244, 245, 250, 251 |
| A3 | 219, 240, 242, 243, 244, 245, 250, 251 |
| K0 | 241, 242, 243, 244, 245, 251, 263, 265, 266 |
| K1 | 234, 236, 238, 239, 241, 242, 243, 244, 250, 265, 266 |
| K2 | 234, 236, 238, 239, 242, 243, 244, 250 |
| K3 | 239, 241, 244, 250, 265, 266 |
| K4 | 234, 236, 238, 241, 242, 243, 244, 245, 250, 251, 265, 266 |
| S1 | 234, 236, 238, 241, 243, 244, 245, 250, 251, 263, 265, 266 |
| R1 | 234, 236, 238, 239, 240, 242, 243, 244, 245, 250, 251 |
| R2 | 219, 220, 222, 224, 225, 226, 233, 241, 243, 244, 245, 250, 251, 265, 266 |
| R3 | 219, 220, 222, 224, 225, 226, 233, 239, 240, 241, 242, 243, 244, 245, 250, 251, 265, 266 |
| R4 | 219, 220, 222, 224, 225, 226, 233, 239, 241, 242, 243, 244, 245, 250, 251, 265, 266 |
| R5 | 220, 222, 224, 225, 226, 233, 239, 240, 243, 244, 245, 250, 251, 263, 265, 266 |
| R6 | 219, 220, 222, 224, 225, 226, 233, 234, 236, 238, 239, 240, 241, 242, 243, 244, 245, 250, 251, 263, 265, 266 |
| R7 | 219, 234, 236, 239, 240, 241, 243, 244, 245, 250, 251, 257, 263, 265, 266 |

Scenes children need B header lines 22–27 (`use ui_wgpu::…`), line 21 for Q10 and line 27 for Q2. In practice every Q child needs 22–27 (Q2 needs only 27).

Simpler alternative (only if the compiler agrees): each child starts with `use super::*;` to inherit the parent's imports. This brings in all crate-root names and hides dependencies, so explicit lines are preferred.

---

## 6. Visibility rules after moving code

Applied per moved item, the compiler then confirms each step:

1. **Items used only inside their own partition:** no change.
2. **Items used from another partition or from another crate file:** `pub(crate)` on the item and a `pub(crate) use <child>::{…};` re-export in the parent (crate root for A children, `kernel_runtime` root for K children, `scenes` root for Q children). The re-export names are the lists in Appendix A (A and B sections). Consumers keep their existing unqualified names.
3. **Items that were `pub` in the original file (B's 109 public items and A's 6):** keep `pub` and add `pub use <child>::{…};` in the parent, so that paths such as `crate::scenes::Viewport` and `crate::run_native` do not change. A's `run_native` and the others in R7 are at crate root, so the re-export lives in the crate root. `upload_icon_atlas` keeps its `#[cfg(target_arch = "wasm32")]` on the `pub use` line.
4. **Private methods in impls that are called from another partition:** `pub(crate)` on that method only. Candidate lists (over-approximate, name-only) are in appendix C. The compiler errors (E0624 private method, E0616 private field) identify the exact set. Fields accessed across partitions get the same treatment.
5. **Private items in a parent used by children:** no change. Descendant modules see parent-private items.
6. **Split impl blocks:** `impl AppRuntime` appears in R4 (14850) and R6 (17802). `impl KernelPoolState` appears twice in K4. Rust allows inherent impls in multiple modules of one crate. Methods called across the two R-partitions follow rule 4.
7. **`super::`:** the only two uses (A 10081, 10099) are inside `scale_bench`. Its parent stays the crate root, so they stay correct. No `self::` and no `pub(super)` change except `read_native_component` (K0, `pub(super)`), which keeps its meaning because K0 stays the `kernel_runtime` root.

---

## 7. Hazards

### 7.1 Path literals that must change

Rule: the new literal is `'../'*depth + old`, where depth is 1 for a child folder directly under the parent's directory and 2 for a child under `📦️kernel-runtime/`. K0 is the `kernel_runtime` root file and has depth 1. Each new literal was checked: it resolves to the same file as the old one. 0 mismatches.

Moved literals in A (P0 literals on lines 3–217 are unchanged):

| Old line | Partition | Kind | Old literal (relative to A dir) | New literal |
|---|---|---|---|---|
| 4404 | A3 | `#[path]` | `../../../🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs` | `../../../../🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs` |
| 4422 | K0 | `#[path = "."]` on inline mod | `.` | removed. The file-level `#[path = "📦️kernel-runtime/🦀️.rs"]` on the declaration in A replaces it. |
| 6370 | K2 | `#[path]` | `../../../🧪️tests/🗞️typed-result-page/🦀️.rs` | `../../../../../🧪️tests/🗞️typed-result-page/🦀️.rs` |
| 9822 | K4 | `include_str!` | `../../../../../🔌️plugin/🖥️host/🧵️shard/🪪️identity/⚙️configuration/🔣️.json` | `../../../../../../../🔌️plugin/🖥️host/🧵️shard/🪪️identity/⚙️configuration/🔣️.json` |
| 9922 | K4 | `include!` (cfg test) | `../../../🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs` | `../../../../../🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs` |
| 10099 | S1 | `include_str!` inside a function body | `../../../../../🔌️plugin/…/🔣️.json` | `../../../../../../🔌️plugin/…/🔣️.json` |
| 12489 | R2 | `#[path]` | `../../../../../../../../🔨️modules/🖱️ui/…/🧪️tests/📥️enqueue/🦀️.rs` | `../../../../../../../../../🔨️modules/🖱️ui/…/🧪️tests/📥️enqueue/🦀️.rs` |
| 19015 | R6 | `include_bytes!` | `../../../../../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf` | `../../../../../../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf` |
| 19114 | R7 | `#[path]` | `🪶️sqlite/🧪️tests/🦀️.rs` | `../🪶️sqlite/🧪️tests/🦀️.rs` |
| 19118 | R7 | `#[path]` | `🪶️sqlite/🦀️.rs` | `../🪶️sqlite/🦀️.rs` |

Note on 10099: the literal sits inside a function body in S1; it gains one `../` because S1 moves one level down.

Moved literals in B (all child modules are depth 1; `Q0` literals are unchanged):

| Old line | Partition | Old literal (relative to B dir) | New literal |
|---|---|---|---|
| 318 | Q1 | `../../🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs` |
| 2063 | Q3 | `../../🧪️tests/🧊️wgpu-standalone/🦀️.rs` (`include!`) | `../../../🧪️tests/🧊️wgpu-standalone/🦀️.rs` |
| 3444 | Q5 | `../../🧪️tests/🔬️wgpu-surface-context-menu/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-surface-context-menu/🦀️.rs` |
| 3967 | Q5 | `../../🧪️tests/🔬️wgpu-render-entry/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-render-entry/🦀️.rs` |
| 4786 | Q6 | `../../🧪️tests/🔬️wgpu-table/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-table/🦀️.rs` |
| 5272 | Q7 | `../../🧪️tests/🔬️wgpu-block-list/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-block-list/🦀️.rs` |
| 5543 | Q7 | `../../🧪️tests/🔬️wgpu-diff-view/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-diff-view/🦀️.rs` |
| 6136 | Q8 | `../../🧪️tests/🔬️wgpu-event-feed/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-event-feed/🦀️.rs` |
| 6536 | Q8 | `../../🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs` |
| 7713 | Q9 | `../../🧪️tests/🔬️wgpu-canvas2d/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-canvas2d/🦀️.rs` |
| 10119 | Q11 | `../../🧪️tests/🔬️wgpu-raster-frame-cost/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-raster-frame-cost/🦀️.rs` |
| 10125 | Q11 | `../../🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs` |
| 11034 | Q13 | `../../../🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/🦀️.rs` | `../../../../🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/🦀️.rs` |
| 11038 | Q13 | `../../🧪️tests/🔬️wgpu-icon-render/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-icon-render/🦀️.rs` |
| 11760 | Q14 | `../../🧪️tests/🔬️wgpu-virtual-file-system/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-virtual-file-system/🦀️.rs` |
| 12341 | Q15 | `../../🧪️tests/🔬️wgpu-text-editor/🦀️.rs` | `../../../🧪️tests/🔬️wgpu-text-editor/🦀️.rs` |

Literals in the lib root (`📚️library/🦀️.rs`) and the Shell element do not change, since A and B stay at their current directories.

### 7.2 Source-text law tests that read A or B by path

These tests read the renderer or Scenes source as text or include it with `include_str!`. They break (or silently check nothing) when code moves:

- `E/🧱️elements/🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs:306`: `read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"))`
- `E/🧱️elements/🐚️Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs:17`: `include_str!("../../../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")`
- `E/🧱️elements/🐚️Shell/🧪️tests/🔄️sync-owner-lifetime/🦀️.rs:6`: `include_str!` of A (`KERNEL_SOURCE`)
- `E/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs:269`: `read(…renderer…).contains("js_name = semioWgpuSetHostLocale")` (the string sits in P0 host locale, so it stays in A)
- `E/🧱️elements/🐚️Shell/🧪️tests/🔀️wgpu-document-relay/🦀️.rs:236`: `include_str!` of A
- `E/🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs:95`: `read_to_string` of A

Each of these must read the whole renderer tree (A plus its child files) or be re-pointed at the child that owns the searched text. Do this in the same step as the move that changes its target.

Comments with line numbers into A (`🧊️renderer/🦀️.rs:12233`, `:15530`, `:15631`, `🧊️renderer/🦀️.rs` region references in Shell, EngineCanvas, frame-job, ui-prefs-themes, wgpu-navbar tests) become stale. They are documentation only, but refresh them.

### 7.3 Test modules

- A `cfg(test)`: fixture mod (3–4), test use/include (7–14), `renderer_io_retained_tests` (4403–4405, A3), `typed_result_page_tests` (6369–6371, K2), `kernel semantic-document` include (9921–9922, K4), five test mods (11193–11221, P0), `runtime_publication_tests` (12488–12490, R2), `NativeAssetPageTestBarrier` (12871–12877, R2), `native_socket_sqlite_tests` (19113–19115, R7).
- B `cfg(test)`: `admitted_surface_map_tests` (317–319, Q1), `standalone` include (2062–2063, Q3), `surface_context_menu_tests` (3443–3445, Q5), `render_entry_tests` (3966–3968, Q5), `Paint2d*` helpers (3976–4016, Q6), `table_tests` (4784–4788, Q6), `block_list` tests (5270–5274, Q7), `diff_view` tests (5541–5545, Q7), `event_feed` tests (6134–6138, Q8), `graph_timeline` tests (6534–6538, Q8), `canvas2d` tests (7711–7715, Q9), `raster_frame_cost`/`ink_canvas` tests (10117–10127, Q11), `icon_render` tests (11033–11040, Q13), `virtual_file_system` tests (11758–11762, Q14), `text_editor` tests (12341, Q15).
- Test mods that `use super::*;` inherit their parent's names. Moving a test mod under a child changes which names it sees. Check the test's imports after the move.

### 7.4 Other

- No `macro_rules!` anywhere in A or B.
- `#[cfg]` blocks: each child file keeps its own `#[cfg(...)]` on items. The `mod` declaration in the parent keeps the `cfg` only where the original inline module had one (kernel_runtime 4421, scale_bench 9957, `ActorBudgetBridge` 9940).
- `///` doc blocks above the inline `kernel_runtime` (4408–4420) and `scale_bench` (9947–9956) move with the module. Keep them as `///` on the `mod` declaration, or convert to `//!` inside the child file, which is the repo convention. Every new child file starts with a `//!` line whose first character is its folder emoji.
- Inline-to-file conversion of kernel_runtime: its body is indented four spaces (module level). Dedent is optional; do it in a separate commit.
- Path length: the deepest new file (`📦️kernel-runtime/🏊️kernel-pool/🦀️.rs`) is 123 characters (186 bytes) from the repo root, or 151 characters with `/Users/ueli/Documents/semio/` in front. Below 256 under either measure.
- The `WorkerCell` pattern in B (`#[cfg(not(target_arch = "wasm32"))]` at 1558–1610) is used by every Q child. Its statics stay in Q0; children reach them through descendant visibility.

---

## 8. Ordered extraction sequence

Each step moves one partition, adds its `mod` and re-exports, fixes its path literals (section 7.1), bumps visibility (section 6) and updates any source-text test (7.2). Keep the parent compiling after every step. Build and check are run by the executor, not by this audit.

### 8.1 File B (scenes) first

Leaves first: each step moves a partition whose inbound uses are mostly from Q0.

1. Q15 `text_editor` (11763–12343)
2. Q14 `board2d_and_vfs` (11042–11762)
3. Q13 `icon_render` (10589–11041)
4. Q12 `node_graph_and_tiled_map` (10129–10588)
5. Q11 `ink_canvas_render` (9779–10128)
6. Q10 `ink_canvas_model_state` (7716–9778)
7. Q9 `canvas2d` (6539–7715)
8. Q8 `event_feed_and_timeline` (5546–6538)
9. Q7 `block_list_and_diff` (4789–5545)
10. Q6 `table_and_paint2d` (3970–4788)
11. Q5 `scene_render_entry` (3266–3969)
12. Q4 `scene_input` (2196–3265)
13. Q3 `scene_camera_dispatch` (1865–2195)
14. Q2 `pending_raster` (1157–1556), folder `🪣️pending-raster`
15. Q1 `scene_state` (29–1156)
16. Final: Q0 (1–28, 1557–1864) plus `mod` declarations and `pub use`/`pub(crate) use` re-exports for each child from Appendix A (B section).

### 8.2 File A (renderer) second

Consumers before providers, so that the foundations (asset, kernel) move last. Kernel is converted to a file first, then split from the leaves.

0. Convert `kernel_runtime` inline body to `📦️kernel-runtime/🦀️.rs` without splitting. Apply one extra `../` to the K literals at 6370, 9822 and 9922 (the file sits one level below A, so depth 1). The `#[path = "."]` at 4422 is replaced by the file-level `#[path]`. Each later step adds one more `../` when the code moves into its child folder; section 7.1 gives the final literals.
1. S1 `scale_bench` (9946–10841)
2. R7 `native_socket_probe` (19109–19541)
3. K4 `kernel_pool` (7674–9923 and 9925)
4. K3 `kernel_retained_state` (6738–7673)
5. K2 `kernel_replay_client` (5352–6737)
6. K1 `kernel_core` (4476–5351)
7. R6 `app_runtime` (17655–19108)
8. R5 `frame_presentation` (16228–17654)
9. R4 `frame_build` (14773–16227)
10. R3 `mailbox_impl` (13169–14772)
11. R2 `presentation_raster` (12327–13168)
12. R1 `frame_actions` (11223–12326)
13. A3 `asset_decode_jobs` (3436–4406)
14. A2 `glb_decode` (1021–3435)
15. A1 `asset_authority` (268–1020)
16. Final: P0 (1–267, 9927–9944, 10842–11222, 19542–19807) plus `mod` declarations and crate-root `pub use` re-exports (R7's `run_*`, `upload_icon_atlas` with its wasm cfg).

Step 0 is needed because the kernel body is an inline module and cannot be split without first being a file.

---

## 9. Result summary

- A: 19,807 lines → P0 about 930 lines and 16 child modules (A1–A3, K0–K4, S1, R1–R7) of 68–2,415 lines each. All children are at or below 2,415 lines, and the parent is far under 3,000.
- B: 12,343 lines → Q0 about 340 lines and 15 children of 331–2,063 lines each.
- The six source-text tests in 7.2 and the path-literal rewrites in 7.1 (10 rows in A, 16 in B) are the only edits outside the two files and their new children.
- Verification still required by the executor (not run here): `cargo check` for the crate on native and wasm targets, the source-text tests in 7.2, and the public-API surface (section 6 rule 3).

---

## Appendix A: export lists (A)

Names defined in each A child that are referenced by another partition or another crate file (token-level, over-approximate). They become `pub(crate)` and are re-exported from the crate root, or from `kernel_runtime` for K children.

Generated by `🗑️generated/seam_exports.py` into `🗑️generated/seam-exports.md`. Full content:

## A export lists (re-export in crate root (P0))

- `A1` `🗂️asset-authority`: 22 names: `renderer_asset_io`, `reserve_renderer_asset_request`, `take_next_renderer_asset`, `reserve_renderer_asset_response`, `RendererAssetSealStep`, `seal_renderer_asset_response`, `return_renderer_asset`, `take_next_completed_renderer_asset_step`, `finish_renderer_asset`, `retire_cancelled_renderer_asset_step`, `close_renderer_asset_step`, `cancel_renderer_map_tile_assets`, `RendererAssetFetchOwner`, `RENDERER_ASSET_PROBE_BYTES`, `RENDERER_ASSET_PARSE_BLOCK_BYTES`, `RENDERER_ASSET_PIXEL_BYTES`, `RendererAssetPageIndex`, `RendererAssetPageCursor`, `RendererAssetFormatCursor`, `PngStructureCursor`, `JpegStructureCursor`, `ProtobufStructureCursor`
- `A2` `🦴️glb-decode`: 9 names: `GLB_SCHEMA_ITEM_CAPACITY`, `GLB_SCHEMA_OUTPUT_BYTES`, `GlbSchemaCursor`, `GlbMatrix`, `glb_world_frame`, `GLB_OUTLINE_SCALE`, `GlbOutlineAccumulator`, `GlbMaterializePhase`, `GlbMaterializeCursor`
- `A3` `🔁️asset-decode-jobs`: 22 names: `RendererAssetProbePhase`, `RendererAssetProbe`, `RendererAssetProbeStep`, `RendererAssetDecodeResult`, `RendererAssetDecodeTurnKind`, `RendererAssetDecodeTurn`, `RendererAssetDecodeJob`, `RendererAssetDecodeSession`, `renderer_asset_svg_prefix_is_valid`, `renderer_asset_reference_prefix_is_valid`, `renderer_worker_pool`, `RendererIoHandle`, `RENDERER_IO_SESSION_SLOTS`, `RENDERER_IO_LIVE`, `RENDERER_IO_CHECKED_OUT`, `RENDERER_IO_SLOTS`, `RENDERER_IO_WAKE`, `renderer_io_with_node`, `renderer_io_generation_live`, `pump_renderer_io_sessions`, `submit_renderer_io`, `run_renderer_io`
- `K0` `📦️kernel-runtime`: 1 names: `read_native_component`
- `K1` `📦️kernel-runtime/🧮️kernel-core`: 35 names: `RendererSequenceAuthority`, `reserve_seq`, `JOB_PROGRESS_PRESENTATION_CAPACITY`, `JobProgressPresentationBridge`, `JobProgressPresentationToken`, `JobProgressPresentationLease`, `job_progress_presentation_bridge`, `take_job_progress_presentation`, `next_seq`, `decode_actor_turn_result`, `TURN_BUDGET`, `RUN_TURN_OUTCOME_TIMEOUT`, `RUN_TURN_SETTLE_BUDGET`, `TURN_REQUESTS_BETWEEN_SLICES`, `COMPILED_COMPONENT_CAPACITY`, `CANCELLED_KERNEL_TURNS`, `run_turn_continuation_turns`, `run_turn_quiescent_continuations`, `native_shard_count`, `extension_index`, `find_wasm_artifact`, `CreateAppRequestOwner`, `QueuedKernelEventKind`, `QueuedKernelEvent`, `RejectedKernelEvents`, `KernelRequest`, `kernel_request_interleavable`, `KERNEL_CLOSE_SUBMISSION_CAPACITY`, `KERNEL_CLOSE_UNADMITTED`, `KERNEL_CLOSE_READY`, `KERNEL_CLOSE_QUEUED`, `KernelCloseStatus`, `KernelCloseSubmissionRegistry`, `KernelCloseSubmission`, `KernelCloseHandle`
- `K2` `📦️kernel-runtime/📬️kernel-replay-client`: 29 names: `PRODUCT_REPLAY_KIND_BYTES`, `PRODUCT_REPLAY_INPUT_BYTES`, `PRODUCT_REPLAY_PROFILE_COUNT`, `RawSpawnJobDisposition`, `MountedProductReplayRecoveryToken`, `MountedProductReplayRecoveryOwner`, `mounted_product_replay_recovery_registry`, `MountedProductReplayRequest`, `MountedProductReplayRefusalToken`, `MountedProductReplayRefusalCause`, `mounted_product_replay_refusal_registry`, `MountedProductReplayAdmissionPermit`, `RefusedProductReplay`, `MountedProductReplayAdmission`, `MountedProductReplayCheckpoint`, `MountedProductReplayClaim`, `MountedProductReplayExpected`, `MountedProductReplayAuthority`, `mounted_product_replay_profile`, `mounted_product_replay_worker_slot`, `TYPED_OPERATION_RESULT_PAGE_BYTES`, `TypedOperationResultToken`, `TypedOperationResultPage`, `ExchangeSurfaceDocument`, `ExchangeOutcome`, `KernelOutcome`, `ResponseSlot`, `KernelFuture`, `KernelClient`
- `K3` `📦️kernel-runtime/🗃️kernel-retained-state`: 18 names: `RetainedSurface`, `RETAINED_SURFACE_CAPACITY`, `RetainedSurfaceSlot`, `RetainedSurfaceRegistry`, `PendingSurfaceRejection`, `PendingSurfaceRejectionRegistry`, `PendingJobProgressPresentation`, `MountedReplayRouteSeed`, `MountedJobReplay`, `MountedReplayRecoveryRegistry`, `mounted_replay_recovery_registry`, `ClosingKernelApp`, `COMMAND_DOCUMENT_RETIREMENT_CAPACITY`, `CommandDocumentPageReservation`, `CommandDocumentRetirementState`, `CommandDocumentRetirementRegistry`, `ReservedToolJob`, `KernelPoolState`
- `K4` `📦️kernel-runtime/🏊️kernel-pool`: 4 names: `KERNEL_REQUEST_QUEUE_CAPACITY`, `KernelRequestQueue`, `KernelPoolFuture`, `run_kernel_pool`
- `S1` `📊️scale-bench`: 3 names: `PUMP_OUTCOME_TIMEOUT`, `turn_budget_of`, `skipped`
- `R1` `🛎️frame-actions`: 18 names: `RuntimeDispatchCursor`, `RuntimeDispatchSource`, `settle_renderer_action_receipt`, `FrameActionReceiptOwner`, `FrameActionOwners`, `FrameInputActionStep`, `transfer_frame_input_action`, `FrameDeferredCursor`, `SHELL_SYNC_PUMP_INTERVAL_MS`, `FrameDeferredWork`, `FrameMaintenanceOwner`, `FrameDeferredExecutionOwner`, `FrameExecutionOwnerCell`, `FrameMaintenanceRefusal`, `FrameExecutionRegistry`, `FrameMaintenanceAuthority`, `frame_maintenance_terminal_fault`, `RuntimeApply`
- `R2` `🎨️presentation-raster`: 28 names: `RUNTIME_COMPLETION_CAPACITY`, `WORLD3D_DEADLINE_CAPACITY`, `WORLD3D_DEADLINE_ID_BYTES`, `EFFECT_STORM_BUDGET`, `RuntimePresentationWitness`, `RuntimePresentationAuthority`, `RuntimeRasterOperationAuthority`, `RuntimeHostWaker`, `RuntimeCompletion`, `RuntimeCompletionQueue`, `returned_completion`, `enqueue_runtime_completion`, `StagedReferenceImageAuthority`, `NativeReferenceDecodeOutput`, `NativeReferenceDecodeCloseStep`, `NativeReferenceDecodeJob`, `NativeAssetHandoff`, `NativeAssetTransportLease`, `NativeAssetTransportCloseStep`, `NativeAssetTransportGuard`, `NativeAssetPageTestBarrier`, `NativeAssetStreamFailure`, `RuntimeMailboxInner`, `RuntimeMailbox`, `FrameMaintenanceExecutionEnvelope`, `RuntimeDispatchReturn`, `FrameDeferredExecutionEnvelope`, `PresenterInteractionStep`
- `R4` `🧾️frame-build`: 31 names: `AppHandle`, `TEXT_STREAM_CAPACITY`, `AppTextStream`, `AppRuntime`, `AppInteractionState`, `FRAME_ENGINE_PACKET_CAPACITY`, `FRAME_ENGINE_PACKET_PAGE_SLOTS`, `FRAME_ENGINE_PACKET_PAGES`, `FrameEnginePackets`, `AppFrameBuild`, `AppFrameAfterChrome`, `FrameBuildCursor`, `FrameBuildPhase`, `FrameFinishCursor`, `FrameFinishPhase`, `FrameBuildBoundaryStep`, `frame_prepared_abandonment_complete`, `FrameFinishBoundaryStep`, `frame_unpriced_child`, `frame_child_step`, `frame_inline_gate`, `frame_inline_progress`, `frame_worker_child_step`, `frame_receive_close`, `frame_received_child`, `FrameTransaction`, `AppFrameTransactionStep`, `FrameTransactionStage`, `AppFrameTransactionPhase`, `AppFramePresentation`, `discard_frame_input_candidate`
- `R5` `🖥️frame-presentation`: 14 names: `FramePreparationStep`, `AppFramePreparation`, `AppPresenter`, `AppPresentStallWatch`, `AppPresentInputWait`, `AppPresentProgress`, `APP_PRESENT_STALL_STEPS`, `note_present_stall_signature`, `AppSurfaceResizePhase`, `AppPresentPhase`, `AppPresentCursor`, `AppPresentedRetirement`, `RasterCandidateRetirement`, `AppPresentStep`
- `R6` `🧠️app-runtime`: 6 names: `stream_native_renderer_asset`, `push_renderer_asset_page`, `step_world3d_camera_fit_after_snapshot`, `world3d_ingest_trace`, `world3d_interaction_trace`, `boot_runtime`
- `R7` `🔬️native-socket-probe`: 9 names: `NativeSocketProbeSnapshot`, `native_snapshot_registration`, `NativeSocketProbeDiff`, `NativeSocketProbeMutation`, `run_socket_grant_probe`, `run_native`, `run_smoke`, `run_smoke_with_axes`, `upload_icon_atlas`

Total A export names: 249

## B export lists (re-export in scenes root (Q0))

- `Q1` `🗃️scene-state`: 27 names: `SCENE_SURFACE_CAPACITY`, `SCENE_SURFACE_ID_BYTE_CAPACITY`, `AdmittedSurfaceToken`, `AdmittedSurfaceCloseOwner`, `AdmittedSurfaceEntry`, `AdmittedSurfaceMap`, `Viewport`, `SceneDragMode`, `SceneDrag`, `InkEditKind`, `InkEditState`, `HostTemporalPresentation`, `VfsAccessibilityControlKind`, `VfsAccessibilityControl`, `BlockListAccessibilityControl`, `EventFeedAccessibilityControl`, `World3dAccessibilityControl`, `GraphTimelineAccessibilityControl`, `SceneSurfaceState`, `SceneValueRetirement`, `SceneSurfaceRetirement`, `close_retired_scene_surface_one`, `CanvasClick`, `CanvasGestureKind`, `CanvasGesture`, `CanvasGestureSlots`, `CanvasCatalogueHover`
- `Q2` `🖼️pending-raster`: 12 names: `RASTER_UPLOADS_PER_SURFACE_CAPACITY`, `RASTER_UPLOAD_KEY_BYTE_CAPACITY`, `RASTER_UPLOAD_BYTE_CAPACITY`, `RasterUploadRefusal`, `PendingRasterQueue`, `PendingRasterQueueToken`, `PendingRasterSurface`, `PendingRasterUploadStep`, `PendingRasterUploadCursor`, `PendingRasterSurfaceRetirement`, `PendingRasterAuthorityClose`, `begin_pending_raster_authority_close`
- `Q3` `📷️scene-camera-dispatch`: 25 names: `SCENE_CAMERA_DISPATCH_CAPACITY`, `SceneCameraOwnerMetadata`, `camera_owner_matches`, `remove_scene_camera_deadline`, `install_scene_camera_authority`, `close_scene_camera_storage`, `scene_camera_storage_is_empty`, `schedule_scene_camera_dispatch`, `SceneCameraDispatchStep`, `SceneCameraDispatchCursor`, `scene_camera_action`, `set_scene_last_pointer_pos`, `scroll_key`, `scroll_offset`, `set_scroll_offset`, `scene_action`, `now_ms`, `remember_scene_theme`, `scene_input_theme`, `write_scene_action`, `scene_action_bytes`, `write_scene_action_to_batch`, `reserve_list_rows`, `list_visible_row_capacity`, `scene_hovered_control_id`
- `Q4` `⌨️scene-input`: 34 names: `MAP_MARQUEE_THRESHOLD_PX`, `canvas_state_snapshot`, `canvas_active_utility`, `cancel_canvas_pointer_gesture`, `cancel_canvas_pointer_gesture_for`, `retire_canvas_window_step`, `request_canvas_pointer_gesture_cancel_for_window`, `request_canvas_pointer_gesture_cancel_for_host`, `request_stale_canvas_authority_cancel`, `canvas_pointer_terminal_step`, `cancel_stale_canvas_authority`, `canvas_pointer_move_into`, `canvas_pointer_button_into`, `CANVAS_CATALOGUE_DRAG_MIME`, `write_scene_action_batch`, `canvas_catalogue_drag_over_into`, `cancel_canvas_catalogue_drag`, `canvas_catalogue_drop_into`, `cancel_canvas_interactions`, `canvas_wheel_into`, `passive_scene_wheel`, `BLOCK_LIST_PALETTE_DRAG_MIME`, `SceneListTransferStart`, `cancel_scene_list_transfer`, `cancel_scene_list_transfer_for_pointer`, `cancel_scene_list_transfer_for_window`, `cancel_scene_list_transfer_if_stale`, `SceneModifiers`, `passive_scene_pointer_button`, `passive_scene_pointer_move`, `SceneListHit`, `scene_kind_is_list`, `scene_list_hit`, `scene_double_click_action`
- `Q5` `🎨️scene-render-entry`: 12 names: `SceneContextMenuTarget`, `scene_context_menu_target`, `context_menu_surface_kind_id`, `render_placeholder`, `VirtualFileSystemChromeLabels`, `BlockListChromeLabels`, `IconRenderChromeLabels`, `SceneChromeLabels`, `SceneEngineHosts`, `render_component_scene_step`, `debug_log`, `scene_has_bespoke_pointer_dispatch`
- `Q6` `🧮️table-and-paint2d`: 30 names: `Paint2dDocSyncJson`, `Paint2dFlatLayer`, `PAINT2D_NAVIGATOR_PADDING`, `TableColumn`, `merge_action_args`, `table_row_id`, `table_segment_rect`, `table_visible_rect`, `table_metrics`, `table_transfer_handle_rect`, `table_transfer_start`, `table_cell_hit`, `TableStepperFocusTarget`, `TableEditableTextFocusTarget`, `TableEditableTextCommitOutcome`, `TableStepperAccessibilityCell`, `TableEditableTextAccessibilityCell`, `TableButtonAccessibilityCell`, `TableStepperKeyOutcome`, `table_editable_text_focus_target`, `table_editable_text_commit`, `table_editable_text_commit_owned`, `table_stepper_focus_target`, `table_stepper_accessibility_cells`, `table_editable_text_accessibility_cells`, `table_button_accessibility_cells`, `table_button_accessibility_activate`, `table_stepper_apply_key`, `table_hit`, `render_table`
- `Q7` `🧱️block-list-and-diff`: 15 names: `BlockListRole`, `BlockListPlan`, `block_list_plan`, `block_list_transfer_start`, `block_list_transfer_drop_action`, `block_list_accessibility_controls`, `render_block_list`, `block_list_hit`, `block_list_accessibility_action_is_current`, `block_list_accessibility_activate`, `DiffLineOperation`, `DIFF_GUTTER_COLUMN_W`, `diff_view_metrics`, `diff_lines`, `render_diff_view`
- `Q8` `📰️event-feed-and-timeline`: 74 names: `EventFeedEntryJson`, `host_temporal_now_ms`, `event_feed_time_id`, `event_feed_time_request`, `publish_host_temporal_reply`, `host_temporal_reply`, `seal_host_temporal_candidates`, `acknowledge_host_temporal_candidates`, `discard_host_temporal_candidates`, `stage_vfs_accessibility_controls`, `stage_block_list_accessibility_controls`, `stage_event_feed_accessibility_controls`, `stage_graph_timeline_accessibility_controls`, `stage_table_stepper_accessibility_cells`, `stage_table_editable_text_accessibility_cells`, `stage_table_button_accessibility_cells`, `accepted_table_stepper_accessibility_cells`, `accepted_table_editable_text_accessibility_cells`, `accepted_table_button_accessibility_cells`, `seal_table_stepper_accessibility_candidates`, `seal_table_editable_text_accessibility_candidates`, `seal_table_button_accessibility_candidates`, `acknowledge_table_stepper_accessibility_candidates`, `acknowledge_table_editable_text_accessibility_candidates`, `acknowledge_table_button_accessibility_candidates`, `discard_table_stepper_accessibility_candidates`, `discard_table_editable_text_accessibility_candidates`, `discard_table_button_accessibility_candidates`, `accepted_vfs_accessibility_controls`, `accepted_block_list_accessibility_controls`, `accepted_event_feed_accessibility_controls`, `accepted_graph_timeline_accessibility_controls`, `vfs_accessibility_control_survives_candidate`, `seal_vfs_accessibility_candidates`, `acknowledge_vfs_accessibility_candidates`, `discard_vfs_accessibility_candidates`, `seal_block_list_accessibility_candidates`, `acknowledge_block_list_accessibility_candidates`, `discard_block_list_accessibility_candidates`, `stage_world3d_accessibility_controls`, `accepted_world3d_accessibility_controls`, `seal_world3d_accessibility_candidates`, `acknowledge_world3d_accessibility_candidates`, `discard_world3d_accessibility_candidates`, `seal_event_feed_accessibility_candidates`, `acknowledge_event_feed_accessibility_candidates`, `discard_event_feed_accessibility_candidates`, `seal_graph_timeline_accessibility_candidates`, `acknowledge_graph_timeline_accessibility_candidates`, `discard_graph_timeline_accessibility_candidates`, `event_feed_tone_color`, `event_feed_layout`, `event_feed_row_height`, `event_feed_hit`, `event_feed_accessibility_label`, `event_feed_accessibility_controls`, `event_feed_accessibility_activate`, `render_event_feed`, `HistoryColumnAuthorJson`, `HistoryColumnJson`, `history_avatar_image_id`, `history_lane_count`, `history_graph_width`, `history_lane_x`, `history_row_lane_guides`, `graph_timeline_avatar_initials`, `graph_timeline_label_track_width`, `graph_timeline_scene_label_track`, `graph_timeline_painted_label_track`, `graph_timeline_layout`, `graph_timeline_accessibility_controls`, `graph_timeline_accessibility_activate`, `render_graph_timeline`, `graph_timeline_hit`
- `Q9` `🖌️canvas2d`: 21 names: `CanvasFillJson`, `CanvasGradientStopJson`, `CanvasLayer`, `canvas_layer_should_render`, `decode_canvas_image_bytes`, `queue_canvas_image_upload_sized`, `queue_canvas_image_upload`, `draw_checkerboard`, `canvas_color_channels`, `canvas_gradient_color_at`, `canvas_apply_blend_mode`, `push_linear_gradient_fill`, `push_radial_gradient_fill`, `render_canvas_shape_fill`, `apply_canvas_framing`, `render_canvas_2d`, `screen_points`, `CANVAS_2D_EMPTY_LABEL`, `queue_canvas_image_upload_with`, `CANVAS2D_SELECTION_RING`, `CANVAS2D_SELECTION_GLOW`
- `Q10` `✒️ink-canvas-model-state`: 49 names: `InkCameraF`, `InkCameraJson`, `InkDocumentJson`, `InkBoundsF`, `ink_item_str`, `ink_item_id`, `ink_item_kind`, `ink_item_locked`, `ink_item_num`, `ink_item_bounds`, `scale_ink_item`, `ink_resize_bounds`, `ink_snap_coordinate`, `ink_maybe_snap_fields`, `create_ink_item`, `ink_points`, `ink_hits_point`, `erase_ink_stroke_points_in_item`, `ink_screen_to_world`, `ink_edit_for_point`, `ink_edit_control_id`, `ink_edit_screen_rect`, `INK_INTERACTION_DOCUMENT_BYTE_CAPACITY`, `INK_INTERACTION_ITEM_CAPACITY`, `INK_EVENT_JSON_BYTE_CAPACITY`, `InkInteractionDocument`, `collect_ink_document_block_spans`, `InkEventJsonPages`, `InkRawPages`, `ink_gesture_abort_into`, `clear_ink_pointer_state`, `ink_edit_key_into`, `ink_edit_blur_into`, `cancel_ink_edit`, `ink_edit_active`, `InkClipboardContentKind`, `ink_clipboard_copy_text`, `ink_clipboard_target`, `ink_clipboard_paste_into`, `ink_edit_paste_into`, `ink_pointer_cancel_into`, `ink_pointer_state_is_clear`, `validate_ink_value`, `ink_wheel_into`, `InkInteractionEvent`, `InkInteractionStep`, `InkBlockCursor`, `InkInteractionJob`, `INK_RESIZE_HANDLES`
- `Q11` `🖋️ink-canvas-render`: 9 names: `ink_effective_bounds`, `flatten_ink_items`, `ink_selection_bounds`, `ink_text_plain`, `ink_world_to_screen`, `ink_resize_handle_screen_pos`, `draw_ink_item`, `draw_ink_rect_outline`, `render_ink_canvas`
- `Q12` `🧭️node-graph-and-tiled-map`: 20 names: `NodeGraphSurface`, `TiledMapSurface`, `query_map_feature_hits`, `tiled_map_context_menu_target`, `write_tiled_map_selection`, `write_tiled_map_hover`, `touch_pointer_down`, `touch_pointer_move`, `touch_pointer_up`, `pinch_camera`, `tiled_map_touch_pointer_down`, `tiled_map_touch_pointer_move`, `tiled_map_touch_pointer_up_into`, `tiled_map_pointer_down_into`, `retire_tiled_map_scene_identity`, `tiled_map_pointer_move_into`, `tiled_map_pointer_cancel_into`, `tiled_map_pointer_leave_into`, `tiled_map_pointer_up_into`, `tiled_map_drag_active`
- `Q13` `🖼️icon-render`: 21 names: `IconRenderCameraFields`, `IconRenderProjection`, `IconRenderMaterialFields`, `IconRenderFormat`, `IconRenderRequestFields`, `icon_render_camera_json`, `icon_render_environment_json`, `icon_render_shadow_profile`, `icon_render_footer_layout`, `icon_render_frame`, `icon_render_content_frame`, `icon_render_badge_layout`, `with_icon_render_mask`, `icon_render_world_component_scene`, `render_icon_render`, `IconRenderStatus`, `icon_render_status`, `icon_render_subject_status`, `icon_render_status_message`, `render_icon_render_status`, `paint_icon_render_chrome`
- `Q14` `🧩️board2d-and-vfs`: 30 names: `Board2dSurface`, `puzzle_board_pointer_down_into`, `puzzle_board_touch_pointer_down_into`, `puzzle_board_touch_pointer_move`, `puzzle_board_touch_pointer_up_into`, `puzzle_board_pointer_move_into`, `puzzle_board_pointer_up_into`, `puzzle_board_pointer_leave_into`, `board2d_drag_active`, `puzzle_board_wheel_into`, `VfsSchema`, `vfs_glyph_icon`, `vfs_double_click_action`, `build_vfs_visible_rows`, `vfs_descriptor_label`, `VfsDescriptorValue`, `vfs_descriptor_value`, `vfs_avatar_initials`, `vfs_temporal_request`, `vfs_metrics`, `vfs_row_id`, `VfsAccessibilityFocusTarget`, `VfsAccessibilityKeyOutcome`, `vfs_accessibility_focus_target`, `vfs_accessibility_activate`, `vfs_accessibility_apply_key`, `vfs_hit`, `hit_double_click_target`, `double_click_action`, `render_vfs`
- `Q15` `✍️text-editor`: 21 names: `TextEditorUiState`, `text_editor_ui`, `text_editor_completions`, `text_editor_rename_info`, `identifier_prefix_start`, `text_editor_line_range`, `multi_span_replace`, `TEXT_EDITOR_OVERLAY_ITEMS`, `render_text_editor_overlays`, `text_editor_open_completions`, `text_editor_completions_open`, `text_editor_close_completions`, `text_editor_move_completion`, `text_editor_apply_completion`, `text_editor_start_rename`, `text_editor_rename_active`, `text_editor_cancel_rename`, `text_editor_local_menu_action`, `text_editor_queue_menu_action`, `text_editor_popup_key`, `text_editor_popup_pointer`

Total B export names: 400


## Appendix B: public items in moved partitions

Every `pub` declaration whose line falls in a partition that moves (excluding `P0`/`Q0`). Column 0 items are free items; indented ones are methods of a type that moves with its partition. Each free item needs a `pub use` re-export in the parent. Methods need nothing extra.

- A `S1`: scale_bench (mod, L9958);   run (fn, L10776, method)
- A `R7`: run_socket_grant_probe (fn, L19336); run_native (fn, L19425); run_smoke (fn, L19449); upload_icon_atlas (fn, L19529)
- B `Q1`: SCENE_SURFACE_CAPACITY (const, L30); SCENE_SURFACE_ID_BYTE_CAPACITY (const, L31); AdmittedSurfaceToken (struct, L34); AdmittedSurfaceFault (enum, L40); AdmittedSurfaceRejected (struct, L49); AdmittedSurfaceCloseOwner (struct, L55); AdmittedSurfaceMap (struct, L66);   try_insert (fn, L110, method);   retain_rejected (fn, L130, method);   retain_first_rejected (fn, L138, method);   admission_blocked (fn, L143, method);   set_external_reservations (fn, L148, method);   get_or_insert_with (fn, L157, method);   id_at (fn, L166, method);   get_mut_at (fn, L171, method);   len (fn, L176, method);   is_empty (fn, L180, method);   contains_key (fn, L184, method);   token (fn, L188, method);   get_token (fn, L193, method);   get_token_mut (fn, L197, method);   get (fn, L201, method);   get_mut (fn, L205, method);   iter (fn, L210, method);   keys (fn, L217, method);   values (fn, L221, method);   iter_mut (fn, L225, method);   values_mut (fn, L229, method);   remove (fn, L233, method);   take_exact_to_retirement (fn, L248, method);   acknowledge_retired_owner (fn, L265, method);   clear (fn, L269, method);   take_fault (fn, L274, method);   record_fault (fn, L278, method);   begin_close (fn, L284, method);   close_step (fn, L288, method);   terminal_is_empty (fn, L303, method)
- B `Q2`: PendingRasterUploadStep (enum, L1301); PendingRasterCheckedOut (struct, L1308);   take (fn, L1315, method); PendingRasterUploadCursor (struct, L1334);   step (fn, L1340, method);   retain_rejected (fn, L1398, method);   close_step (fn, L1407, method); PendingRasterAuthorityClose (struct, L1511); begin_pending_raster_authority_close (fn, L1515);   close_step (fn, L1521, method);   terminal_is_empty (fn, L1551, method)
- B `Q3`: SceneCameraDispatchStep (enum, L1961); SceneCameraDispatchCursor (struct, L1963);   begin (fn, L1970, method);   step (fn, L1983, method);   close_step (fn, L2024, method);   terminal_is_empty (fn, L2051, method)
- B `Q4`: canvas_pointer_move_into (fn, L2521); canvas_pointer_button_into (fn, L2590); canvas_wheel_into (fn, L2863); SceneModifiers (struct, L3067);   additive (fn, L3083, method);   interaction_merge (fn, L3092, method)
- B `Q5`: SceneContextMenuTarget (struct, L3278); scene_context_menu_target (fn, L3304); context_menu_surface_kind_id (fn, L3456); VirtualFileSystemChromeLabels (struct, L3485); BlockListChromeLabels (struct, L3493); IconRenderChromeLabels (struct, L3500); SceneChromeLabels (struct, L3507);   english (fn, L3516, method); SceneEngineHosts (struct, L3531); render_component_scene_step (fn, L3556)
- B `Q12`: NodeGraphSurface (struct, L10132); TiledMapSurface (struct, L10145); tiled_map_context_menu_target (fn, L10174); tiled_map_touch_pointer_down (fn, L10302); tiled_map_touch_pointer_move (fn, L10315); tiled_map_touch_pointer_up_into (fn, L10336); tiled_map_pointer_down_into (fn, L10349); retire_tiled_map_scene_identity (fn, L10390); tiled_map_pointer_move_into (fn, L10431); tiled_map_pointer_cancel_into (fn, L10491); tiled_map_pointer_leave_into (fn, L10513); tiled_map_pointer_up_into (fn, L10528); tiled_map_drag_active (fn, L10584)
- B `Q14`: Board2dSurface (struct, L11043); puzzle_board_pointer_down_into (fn, L11051); puzzle_board_touch_pointer_down_into (fn, L11056); puzzle_board_touch_pointer_move (fn, L11069); puzzle_board_touch_pointer_up_into (fn, L11090); puzzle_board_pointer_move_into (fn, L11103); puzzle_board_pointer_up_into (fn, L11117); puzzle_board_pointer_leave_into (fn, L11131); board2d_drag_active (fn, L11135); puzzle_board_wheel_into (fn, L11139); Puzzle2dSelectionMenuItem (struct, L11144); build_puzzle2d_selection_menu_items (fn, L11159)
- B `Q15`: text_editor_open_completions (fn, L12035); text_editor_completions_open (fn, L12049); text_editor_close_completions (fn, L12053); text_editor_move_completion (fn, L12061); text_editor_apply_completion (fn, L12078); text_editor_start_rename (fn, L12101); text_editor_rename_active (fn, L12114); text_editor_update_rename (fn, L12121); text_editor_commit_rename (fn, L12140); text_editor_cancel_rename (fn, L12169); text_editor_local_menu_action (fn, L12188); text_editor_queue_menu_action (fn, L12221); text_editor_popup_key (fn, L12241); text_editor_popup_pointer (fn, L12302)


## Appendix C: private method candidates (name-level, over-approximate)

A method is listed when its name occurs in another partition. Bump only the methods the compiler reports (E0624 or E0616) after the move. Listing here is a superset.

```text
A:
  A1 (14): admit, advance_decode_page, begin_close, close_step, decode_page, feed, finish, generation, owner_mut, read_block, read_byte, received_bytes, revision, rewind_decode_pages
  A2 (11): begin_close, close_step, feed, finish, instance, primitive, replay, restore_asset, step, take_asset, validate
  A3 (17): begin_close, cancel, close_step, complete, finish_ready_mesh, owner, owner_mut, pump_one, reject, restore_ready_mesh_asset, step, take_boundary, take_ready_mesh_asset, take_terminal_owner, terminal_is_empty, try_take, vacant
  K1 (21): can_cancel, cancel, close_step, command_credits, commit, extensions_of, finish, generation, into_event, into_parts, presented, publish, release_presented, reserve, return_lease, terminal_is_empty, try_admit, try_from_events, try_reserve, try_schedule, vacant
  K2 (32): abandoned, absorb, acknowledge_mount, acknowledge_qualification, acknowledge_raw_retirement_boundary, carries_nothing, close_instance_one, close_one, close_step, decode_guest_message, deliver, encode_ack, has_any, has_close_work, has_instance, has_work, instance, job_kind, kind, publish, raw_mut, refuse, reject, release, request_owner, reserve, retire, retry, step_one, submit, try_acknowledge_job_progress, validate
  K3 (28): accept_replay_submission, admit, admit_patch, advance_document_one, advance_instance_close_one, advance_patch_one, advance_realm_close_one, begin_close_batch, begin_close_instance, close_one, close_step, compose, get_mut, has_close_work, instance_is_empty, pending_replay_start_is_exact, publish, publish_batch, release, replay_submission_sequence, reserve, take_any_one, take_instance_one, terminal_is_empty, try_admit, try_alias, try_reserve_page, try_upsert
  K4 (15): activate_extensions_of, advance_product_replay, apply_turn_result, compile_serving_others, create_app, enqueue_retained, exchange, exchange_commands, mount_product_replay, retire_cancelled_turn, run_turn, try_push, turn_cancelled, wake_consumer, yield_slice
  S1 (5): activate, drain, ordinal, send_payload, unregister
  R1 (32): abandon, admit, begin_close, cancel, cancel_correlation, cancel_current, close_returned_interaction_step, close_step, complete, generation, is_live, new_for_generation, pop_front, reclaim_rejected, release, remaining, restore_taken, restores_interaction, settle, staged_batch_remaining, start_frame_deferred, take_next, take_pair, terminal_is_empty, try_begin, try_publish, try_push, try_push_queued, try_push_queued_pair, try_reserve, try_take, try_take_abandoned
  R2 (23): admit_build, begin, cancel, close_step, complete, completion, discard, enqueue, finish, mark_scene_changed, matches, observe_input_generation, owner_mut, rearm, release, restore, restore_output, retire_payload_step, seal, take_output, try_lock, try_schedule, witness_for
  R3 (23): admit_build_presentation_witness, cancel_interaction_future_reservation, downgrade, frame_inputs, has_pending_text_work, presentation_authority, presentation_witness_for, raster_operation_authority, reserve_future, reserve_interaction_future, retain_native_asset_transport, spawn_dispatch_reserved, spawn_frame_deferred_reserved, spawn_frame_maintenance_reserved, spawn_interaction_reserved, try_lock, try_spawn_frame_deferred_reserved, try_spawn_frame_maintenance_reserved, try_take_abandoned_frame_deferred, try_take_abandoned_frame_maintenance, update_frame_inputs, wait_native_asset_page_test_barrier, world_cursor_wake_authority
  R4 (10): check_out_interaction, close_backing_step, close_step, drive_icon_raster_step, interaction_available, prepare_step, return_interaction, submit_interaction, terminal_is_empty, try_push
  R5 (6): abort, close_step, commit, latency_stage, step, terminal_is_empty
  R6 (3): frame_after_input_step, frame_before_input_step, has_pending_text_work
  R7 (2): absorb, apply

B:
  Q0 (2): format_temporal_values, with
  Q1 (14): acknowledge, close_step, discard, get_mut, has_capacity_for, into_iter, observe, publish, refuse, screen_to_world, seal, stage, visible, world_to_screen
  Q2 (5): admits_upload, close_step, park_refusal, push_back, terminal_is_empty
  Q4 (3): button, release, sampled
  Q9 (2): close, open
  Q10 (5): as_str, block, front, publish, seal
  Q11 (1): visit
  Q14 (3): deserialize, values, visit

```

## Appendix D: generated sources

Scripts and raw outputs (delete when the ticket closes): `🗑️generated/seam_analyze.py`, `seam_free.py`, `seam_external.py`, `seam_exports.py`, `seam-A-analysis.txt`, `seam-B-analysis.txt`, `seam-free.txt`, `seam-external.txt`, `seam-regions.txt`, `seam-A-items.txt`, `seam-A-toplevel.txt`.
