# 🧊️ W2a — Scene remainder and asset lanes

Packet W2a of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Every claim below was re-verified against
the live tree; every line number is post-edit. Items are numbered as the packet brief numbers them.

**Landed with tests: 1, 2, 3, 4, 8, 9, and 6's lane half (6c).**
**Not landed: 5, 6a (marquee), 6b (navigator), 7 — see §10 for exactly what is left and why.**

---

## 1 — P0: the three dead asset lanes

### 1.1 The shared lane decoded NOTHING

`pump_renderer_asset_decode_step`'s `Ready` phase resolved the owning World3d surface FIRST and
`return false`d for every `RendererAssetFetchOwner::Shared(_)`. That one line parked the whole shared
lane permanently: the probe stayed `Ready`, nothing closed it, and no later shared request was ever
admitted. Both shared kinds — `UiImage` and `MapTile` — were therefore unreachable even when their
request side worked.

| site | change |
| --- | --- |
| `📺️renderer/…/🧊️renderer/🦀️.rs:10320` | new `take_shared_asset_bytes` — drains a `Ready` probe whose kind applies without a `World3dState`; a collect failure answers empty bytes so the lane still CLOSES instead of parking |
| `🧊️renderer/🦀️.rs:10346` | the `Ready` phase resolves `UiImage`/`MapTile` **before** the surface lookup, calling `interpreter::apply_ui_image_bytes` / `engine_canvas::apply_map_tile_bytes` and then `probe.begin_close()` |

`queue_ui_image_url_fetch` (`🗣️Interpreter/…/🦀️.rs:1668`) already reserved
`WorldAssetRequestKind::UiImage` from two production call sites, and the probe already accepted PNG,
JPEG and SVG prefixes for that kind (`🧊️renderer/🦀️.rs:418`, `:2692`) — so the request and structure
halves needed nothing. Only the apply half was missing.

### 1.2 Map tiles: nothing reserved a `MapTile` request anywhere in the repo

React reference: `MapRenderer.uploadTileRow` / `refreshRasterTiles` / `refreshVectorTiles`
(`🧭️TiledMapHost/🟦️.tsx:608-661`).

| site | change |
| --- | --- |
| `⚙️EngineCanvas/…/🦀️.rs:4226` | `map_tile_url` — React's `{z}/{x}/{y}` substitution chain |
| `⚙️EngineCanvas/…/🦀️.rs:4240` | `reserve_map_tile_fetch` — admits ONE tile as `WorldAssetRequestKind::MapTile { surface, key, vector, z, x, y }`; back-pressure (`ItemCapacity`/`ByteCapacity`/`Closing`/`Stale`) leaves it un-pended so the next frame re-offers, only a structural refusal records a miss (React's `tileMiss`) |
| `⚙️EngineCanvas/…/🦀️.rs:4266` | `reserve_map_tile_fetches` — walks `visible_raster_tile_cursor`/`visible_vector_tile_cursor`, obeys `render_mode_str()` exactly as React's `needsRasterTiles`/`needsVectorTiles` gates do, skips tiles `has_tile`/`has_vector_tile` already holds, and clears a lane's miss set when that lane's `visible_tiles_revision` changes |
| `⚙️EngineCanvas/…/🦀️.rs:2585` | `sync_tiled_map_scene` calls it after `sync_map_engine`, every frame |
| `⚙️EngineCanvas/…/🦀️.rs:1647` | `MapSyncCache` gains `tile_pending`/`tile_misses`/`raster_tiles_revision`/`vector_tiles_revision` |
| `⚙️EngineCanvas/…/🦀️.rs:4301` | `apply_map_tile_bytes` now clears the pending slot, records a miss when the host refuses the bytes, and bumps `scene_revision` so the next frame repaints |

### 1.3 `UiNode::Image`: data URIs faulted, and the ui crate's ledger had no production caller

Two separate defects, one per renderer path.

**os-renderer (scene-slot path).** `render_ui_image_step` phase 2 answered
`ScenePaintStep::Fault` for every `src` starting with `data:`, and `resolve_ui_image`,
`resolve_ui_image_svg`, `resolve_ui_image_url`, `percent_decode_basic` and
`parse_svg_data_url_bytes` were ALL `#[cfg(test)]`. So a `Component::Image` carrying an inline bitmap
faulted the whole paint step, on browser and native alike, while React's `ImageView` is a plain
`<img src>` the browser decodes natively.

| site | change |
| --- | --- |
| `🗣️Interpreter/…/🦀️.rs:1756` | new production `resolve_ui_image_data_url` — `data:image/svg+xml` through `resvg`, every other `data:` media type through the renderer's png/jpeg decode (`queue_canvas_image_upload_sized`), publishing into the SAME three caches a fetched URL lands in so the draw phase reads one cache either way |
| `🗣️Interpreter/…/🦀️.rs:1856` | phase 2 decodes instead of faulting |
| `🗣️Interpreter/…/🦀️.rs:1721`, `:1683`, `:1705` | `resolve_ui_image_svg`, `percent_decode_basic`, `parse_svg_data_url_bytes` promoted out of `#[cfg(test)]` |

Deliberate: an unresolved data URI records **no** miss. `queue_canvas_image_upload_*` answers `None`
both for a busy raster queue and for an undecodable payload; a permanent miss on the former would
blank the image forever, so the source is simply re-offered next paint, bounded by the raster queue's
own admission.

**ui crate (no-scene-host path).** `admit_ui_image`/`take_ui_image_upload`
(`🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:2559`, `:2605`) had only test callers, so the ledger stayed
empty forever, `ui_image_natural_size` always answered `None`, and EVERY `UiNode::Image` painted the
`alt` placeholder however decodable its `src` was.

| site | change |
| --- | --- |
| `🖌️paint/🦀️.rs:1100-1111` | the `UiNode::Image` arm admits its own source (idempotent: an already-decoded `src` is a ≤64-entry ledger lookup, never a second decode) |
| `🎞️Scenes/…/🦀️.rs:3879` | new `queue_decoded_raster_upload` — everything `queue_canvas_image_upload_with` does minus the decode and minus the `canvas-image:<surface>:<layer>` key convention, because the retained paint keys its quad by the `src` itself |
| `🧊️renderer/🦀️.rs:12225` | the frame's `RasterUploads` phase drains `take_ui_image_upload()` into it — the "host's per-frame drain" that function's own docstring names and never had. It answers `None` whenever a `SceneHost` is registered (the renderer's normal case), so it is free there and is what makes the no-scene-host path show a real bitmap |

### Tests

- `engine_surface_attach_tests::tiled_map_paint_reserves_the_visible_tiles_react_fetches` — a painted map offers its visible tiles, never offers one the host already holds, and a second paint does not double-reserve.
- `engine_surface_attach_tests::map_tile_url_substitutes_the_same_three_placeholders_react_does`.
- `render_plan_validator_tests::render_ui_image_step_paints_an_inline_data_url_instead_of_faulting` — drives the PRODUCTION cursor to completion and asserts a raster quad plus the decoded natural size (so `object-contain` matches React's intrinsic ratio).
- `wgpu::paint::tests::the_paint_arm_admits_its_own_source_without_a_separate_admit_call` — no explicit `admit_ui_image`, which is exactly what a host that only paints does.

### Gaps

- A `.svg` **URL** (not data URI) reaches `apply_ui_image_bytes`, which sniffs SVG and rasterises — that path was already live. An inline SVG on the **ui crate's** own ledger is still `Unsupported` (that crate is PNG-only by design, `🖌️paint/🦀️.rs:2512`).
- `MapTile` is admitted per visible tile with no prefetch ring; React additionally fetches `prefetchTilesJson`/`prefetchVectorTilesJson`. The Rust cursors exist (`prefetch_tiles_json`) — wiring them is a one-function follow-up.

---

## 2 — InkCanvas `documentJson` / `snapshotJson`

`InkCanvasScene` had TWO encoders disagreeing on the document payload's key: serde
(`rename_all = "camelCase"`, the pack wire every producer actually travels, and the key React's
`InkCanvasHost` reads — `parseInkScene(scene?.documentJson)`, `🖋️InkCanvasHost/🟦️.tsx:971`) said
`documentJson`; the hand-written `ToValue`/`FromValue` said `snapshotJson`. A scene handed to the
DSL-value transport carried a key nothing read.

Fixed to `documentJson` at all four sites:
`🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:2283`, `:2297` (ToValue/FromValue),
`🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json:31`,
`🖱️ui/🎬️scene/🟦️.ts:997` (the TS type declared `snapshotJson` while its own docstring said
`documentJson` — React was reading a key the type denied).

Test: `scenes::tests::ink_canvas_document_payload_is_document_json_on_both_encoders` — both encoders
spell it the same and neither emits `snapshotJson`. Repo-wide grep now shows zero `snapshotJson` for
this field (the remaining hits are the unrelated machine-derive/flow-ABI export of that name).

---

## 3 — `SurfaceKind` wire tag: `virtualFileSystem` → `virtual-file-system`

The contract crate had already made the rename; the wgpu target's own `SurfaceKind` and the schema
projection had not, which is why two conflicting generated TS unions existed.

| site | change |
| --- | --- |
| `🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2908-2910`, `:2946` | `#[serde(rename)]`, `#[value(rename)]` and `as_str` all kebab-case |
| `🔨️modules/🧬️schema/📽️projection/🦀️.rs:1167` | the projected TS union regenerated (it was the copy that still said `virtualFileSystem`) |
| `🖱️ui/🧬️contract/🗺️surface/🦀️.rs:74`, `🧬️contract/🧬️schema/🦀️.rs:727`, `🎬️scene/🎬️scenes/🦀️.rs:30`, `🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:354` | four docstrings that still described the rename as *pending* now describe it as *done* |
| `🎞️Scenes/…/🦀️.rs:6703` | the placeholder label |
| `🖱️ui/🧪️tests/…-ui-node-wire-format/🦀️.rs:312` | the golden union |

New law: `ui_node_wire_format_tests::surface_kind_wire_tags_agree_between_contract_and_wgpu_target`
walks all fifteen variant pairs and asserts `serde_json` agreement AND `as_str` agreement.
`🔀️reconcile`'s `surface_kind` maps variant-to-variant, so a tag that drifts on one side silently
re-splits the wire in two without failing to compile — that is what this law catches.

---

## 4 — Board2d: keyboard shortcuts, `domain_id`, event-table drift

### 4.1 Zero keyboard handling existed

React has two listeners: `Escape` → `session.cancelAreaSelect()` (`🖥️Board2dHost/🟦️.tsx:1292-1313`)
and a CAPTURE-phase `Tab`/`shift+Tab` → `session.brushCycleCandidate(!shiftKey)` (`:1338-1339`, in
capture so its `preventDefault` beats the shell's chord dispatcher).

| site | change |
| --- | --- |
| `⚙️EngineCanvas/…/🦀️.rs:4951` | `puzzle_board_key_into` — `Escape` cancels an in-flight area-select, `Tab`/`shift+Tab` walk the armed brush slot's candidates; gated on `board_pointer_inside` (this target's equivalent of React's `hoverActiveRef`) and on the active utility for `Tab`; returns `true` when the chord is consumed |
| `⚙️EngineCanvas/…/🦀️.rs:4880` | `puzzle_board_flush_events_into` — the standalone drain+coalesce+dispatch (React's `flushBoardEvents`) the chords flush through; `puzzle_board_wheel_into` inlines the same three steps because it must ride the camera action's reservation |
| `🧊️renderer/🦀️.rs:14128` (`board2d_key`), `:14172` | the renderer's `handle_key` offers `Escape`/`Tab` to every board pane the pointer is inside BEFORE the shell's chord table, mirroring React's capture-phase precedence |

### 4.2 `domain_id` was carried and read nowhere

React's only use of `Board2dScene.domainId` is one coalesced `interactionHover` per hover change,
with the granularity resolved from the fixture and defaulting to `"node"`
(`🖥️Board2dHost/🟦️.tsx:728`, `:774-781`).

| site | change |
| --- | --- |
| `⚙️EngineCanvas/…/🦀️.rs:4562` | `board2d_granularity_by_id` — the Rust twin of `board2dGranularityById`, including its rule that an id the fixture never carries reads as `node` rather than being dropped |
| `⚙️EngineCanvas/…/🦀️.rs:1648-1657` | `BoardSyncCache` gains `domain_id` + `granularity_by_id` (rebuilt only when the fixture changes), because the pointer seams are addressed by surface id, not by scene |
| `⚙️EngineCanvas/…/🦀️.rs:70` | `EngineSurface.board_hover_published: Option<Option<String>>` — the coalescing witness (outer `None` = never published, `Some(None)` = the explicit clear) |
| `⚙️EngineCanvas/…/🦀️.rs:4909` | `puzzle_board_hover_into` — publishes `interactionHover` on the declared domain; an app that declares no domain publishes nothing (React's own `if (!interactionDomainId) return undefined`), and an unchanged target costs no round trip |
| `⚙️EngineCanvas/…/🦀️.rs:5039`, `:5052` | published after a pointer move, and cleared on pointer leave |

### 4.3 Both event tables had drifted from React's sets

`board_event_transient` (`:4532`) was missing `Hover` and `TransformPreview` — so every board
pointermove carried a hover row into `applyBoardEvents`, i.e. a whole-surface republish per move,
where React explicitly keeps hover off the batch. `board_event_flush_now` (`:4540`) was missing
`NodeRotate`, `RegionCreate`, `RegionMove`, `RegionResize` — so a rotate or a region edit sat in the
buffer until some later event happened to flush it. Both now name exactly what
`PUZZLE2D_TRANSIENT_EVENT_NAMES`/`PUZZLE2D_FLUSH_NOW_EVENT_NAMES` (`🟦️.tsx:293-294`) name.

### Tests

`board2d_engine_tests::transient_and_flush_now_tables_match_the_react_sets`,
`::coalesce_drops_hover_rows_so_a_pointermove_never_republishes_the_surface`,
`::board_granularity_classification_matches_the_react_table`.

### Gaps

- The hover row that reaches `applyBoardEvents` on a pointer MOVE comes from
  `BoardPointerPlan::events_json()`, built inside `infinite_canvas`, not from the drain queue the
  coalescer filters. Dropping it there needs a `BoardHost::plan_pointer` change (one `push` guard in
  `🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs`); until then the republish-per-move cost survives on
  the plan path even though `interactionHover` is now published alongside it.
- React's third Escape listener (a pane that does NOT own the suggestion popup still dismisses it,
  `🟦️.tsx:713-721`) has no wgpu counterpart — the wgpu suggestion menu is `suggestion_menu_json` on
  the scene with no cross-pane dismissal path.
- The bespoke `BoardEventKind::Select` selection model is unchanged (WP-5's open decision). React
  dispatches no generic `interactionSelect` for Board2d either, so this is not a divergence today;
  only hover is domain-scoped on both sides.

---

## 6c — `Paint2dScene` lanes

`Paint2dScene` had no `SceneLane`/`split_lanes`/`merge_lane` override at all, so a raster document
larger than `UI_FIXED_BYTES` (32 KiB) made `scene_surface.encode` refuse the WHOLE surface — the
paint-2d window silently stopped updating past that size while Canvas2d and Board2d paged fine.

| site | change |
| --- | --- |
| `🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:1629` | `lanes: Vec<SceneLaneRef>` on the spine (skipped when empty, on serde AND `ToValue`) |
| `🎬️scenes/🦀️.rs:1640` | `SceneDoc::split_lanes`/`merge_lane` |
| `🎬️scenes/🦀️.rs:1673` | `Paint2dSceneLane { DocumentSync, Assets }` + the five `PAINT2D_SCENE_LANE_*` tables |
| `🖱️ui/🎬️scene/🧫️fixtures/🚚️paint2d-scene-lanes/🔣️.json` | new language-neutral declaration, same shape as the canvas2d/board2d twins |
| `🖱️ui/🎬️scene/🟦️.ts:892`, `:583-606` | `lanes?` on the TS type plus `PAINT2D_SCENE_LANE_KEY_PREFIX`/`PAINT2D_SCENE_LANES`/`paint2dSceneLaneForBodyKey`/`paint2dSceneFromLanes` |
| `🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:478` | the Paint2d arm merges its lanes back, like World3d/Canvas2d/Board2d |

Both lanes always publish (neither field is optional): `document_sync_json` is the whole
`RasterSession` sync channel and `assets_json` carries one entry per imported bitmap, so either
outgrows the doc on a real painting. Everything else (camera, selection, hover, active utility, brush
size/opacity, view mode, composite viewport) is a bounded per-frame descriptor and stays in the spine.

Tests: `scenes::tests::paint2d_scene_lanes_mirror_the_language_neutral_declaration`,
`::paint2d_scene_splits_into_the_declared_lanes_and_merges_back`,
`::paint2d_spine_stays_inside_the_fixed_surface_doc_for_an_oversized_document` (128 KiB of payload,
spine still < 4 KiB and admitted by `UiFixedBytes`).

**Side effect I had to repair:** the new `Vec` grew `Option<UiSurfaceSlot>` by 24 bytes, which the
committed slot-table budget guards. `⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json:85` updated
160592 → 160616 (`ui::wgpu_engine` / `wgpu::engine::UiSurfaceRegistry`), which is what
`ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack` asserts. Consistent with
Canvas2d/Board2d/World3d, which all already carry `lanes` in the same slot.

There is no TS lane *collection* counterpart to wire: `canvas2dSurfaceLaneTexts`/
`board2dSurfaceLaneTexts` do not exist in production TS either — the only references are injected
test dependencies in `🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx`. Paint2d is exactly as
wired as Canvas2d and Board2d on the React side.

---

## 8 — IconRender pending/error state

React renders the shot offscreen through `iconRenderPort` and has three visible states: the error
text, the finished `<img>`, and `ui.host.rendering` while the promise is in flight
(`🖼️IconRenderHost/🟦️.tsx:55-61`). The wgpu twin draws the GLB straight into the frame, so it had
exactly ONE: a silently empty shot frame for the whole fetch, and forever if the fetch never landed.

| site | change |
| --- | --- |
| `🎞️Scenes/…/🦀️.rs:6668` | `IconRenderStatus { Ready, Rendering, Failed }` |
| `🎞️Scenes/…/🦀️.rs:6678` | `icon_render_status(mesh_resident, faulted)` — takes the two predicates rather than the state so the mapping is testable headlessly; the call site feeds it `state.mesh_lease(&mesh_id).is_some()` and `state.snapshot_fault().is_some()` |
| `🎞️Scenes/…/🦀️.rs:6690` | `render_icon_render_status` — centres the line INSIDE the shot frame, like React's text inside `IconShotFrame` |
| `🎞️Scenes/…/🦀️.rs:6600` | `render_icon_render` paints it over the (still empty) world draw |

Divergence recorded in the code: the two messages are hard-coded English
(`"Rendering…"`, `"Shot failed"`) exactly like this region's existing `"No shot"` — the wgpu scene
painters carry no `useLabel` equivalent yet. Wiring `LocalizedLabel` into scene chrome is its own
packet. React also shows `iconRenderPort.render`'s own rejection message; this twin has only the
state's snapshot fault, so it names the failure instead of inventing a string.

Tests: `icon_render_tests::icon_render_status_reports_rendering_until_the_subject_mesh_is_resident`
(all four predicate combinations, including "residency wins"),
`::icon_render_status_line_paints_inside_the_frame`.

Still open from W1b's list: IconRender needs a `/mesh/` public-id → transport-path resolution
equivalent. It HAS one — `crate::mesh_assets::mesh_asset_transport_url` at `🎞️Scenes/…/🦀️.rs:6524`,
the same call React's host makes. That W1b gap is already closed; no action.

---

## 9 — The two `WORLD3D_SHADER` copies, and the law that ties them

**Both copies were already aligned when I got here** — `🖌️render/✨️shader-contract/🦀️.rs:538` carries
`@location(2) color` and `out.color = instance.color * vertex.color`, `WORLD3D_MESH_VERTEX_BUFFERS`
is stride 40 with the Float32x4 at offset 24, the HLSL (`🪟️d3d12/✨️hlsl/🦀️.rs:222`) and MSL
(`🍎️metal/✨️msl/🦀️.rs:221`) mirrors both read `vertex_color : ATTRIB2` / `[[attribute(2)]]`, and
`🧊️webgpu/🗃️resources/🦀️.rs:325`'s private `World3dVertex` has the `color: [f32; 4]`. W1f's gap 4
was closed by the W1 integration pass (`📓️w1-integration.md` §2.7). Verified byte-identical, not
assumed.

What was still missing is the thing W1f's report asked for: *nothing tied the two copies together*.
`ui_render` must not depend on `semio-framework-ui` (the dependency runs the other way for every
other seam), so the law reads the wgpu target's SOURCE TEXT:

- `🖌️render/🧪️tests/🔬️shader-contract-unit/🦀️.rs` — `WGPU_TARGET_SHADERS_SOURCE` `include_str!`s
  `🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs`, `raw_string_const` lifts a named raw-string const out of it.
- `world3d_wgsl_is_byte_identical_in_the_wgpu_target_and_the_shader_contract` — both
  `WORLD3D_SHADER` and `WORLD3D_LINES_SHADER`, byte for byte.
- `world3d_vertex_locations_are_all_fed_by_the_declared_vertex_buffers` — every `@location` the
  WGSL's `VertexInput` declares has a per-vertex attribute in BOTH world-3d pipelines, so a shader
  that gains a channel cannot ship without the layout that feeds it. That is the exact shape of the
  W1f divergence (`@location(2) color` present, stride 24).

---

## 10 — Not landed, and exactly what is left

I stopped implementing when the fleet's churn made the target files unverifiable (see §11). These
four are stated precisely enough to pick up cold.

1. **Item 5 — World3d relocate drag in production + Escape-cancels-relocate/suggestion.**
   The finding in the brief is understated: ALL THREE World3d pointer handlers are `#[cfg(test)]` —
   `handle_world3d_pointer_move` (`♾️infinite/🌍️world/🦀️.rs:10676`),
   `handle_world3d_pointer_button` (`:10741`, which is where the `worldRelocate` dispatch lives at
   `:10853`) and `handle_world3d_pointer_drag` (`:10951`). Production goes through the bounded
   `WorldInteractionAuthority` instead (`:5515`-`5790`), whose planners are only
   `plan_world3d_wheel`/`plan_world3d_drag`/`plan_world3d_paint_stroke` (`:6269`, `:6284`, `:6310`).
   Promoting relocate therefore is NOT un-gating a function: it needs a new
   `WorldFlatActionKind::Relocate` + `plan_world3d_relocate`, a `WorldRayPickPurpose` arm to resolve
   the dragged object and its plane-projected position per move, a `publish_world3d_plan_step` writer
   arm emitting `worldRelocate { surfaceId, objectId, position }`, and the `PointerButton`/
   `PointerMove` dispatch arms. The state fields it needs already exist and are already written by
   the test-only drag (`drag_object_id` `:1310`, `drag_last_position` `:1314`, `drag_object_z`,
   `press_object_id` `:1358`). Escape-cancel is meaningless until that exists, and the vortex
   suggestion menu (`World3dHost` ~6167-6180) has no wgpu counterpart at all.
2. **Item 6a — Paint2d marquee/lasso overlay + `marqueeHitsJson` commit.**
   `RasterHost::marquee_hits_json` and the merge helpers are wired; the wgpu side still dispatches a
   single-point pick on release. Needs the drag-rectangle overlay draw plus the commit, against
   React's `marqueeRef`/`SelectionMarquee`.
3. **Item 6b — Paint2d navigator "you are here" + navigator-drives-content-camera.**
   `navigator_fit_camera_json` is applied (the overview renders fitted);
   `navigator_viewport_overlay_json` and the navigator's own pan/wheel (which must dispatch the
   CONTENT pane's camera) are unwired, and the navigator refuses pointer input.
4. **Item 7 — Pointer modifiers + row drag sources.**
   `UiEvent::PointerDown`/`PointerUp`/`PointerMove`/`Scroll`
   (`🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:45-65`) carry NO modifier field, while `KeyDown`/`KeyUp`
   do (`EventModifiers`). Until they do, `vfs_selection_for_click(…, false, false)` can only ever
   resolve a plain replace, and the same holds for Table/BlockList/NodeGraph multi-select. The change
   is mechanical but wide (every construction site and match arm in the ui crate, the renderer and
   the tui host), and `⚡️events/🦀️.rs` is one of the files a peer is editing right now — it wants to
   be one packet on its own, landed when the lane is quiet. Row drag sources (`row_drag_mime`,
   `TableScene::drop_action_json`, the BlockList palette drag) depend on the same event shape for
   their modifier-qualified drops.

---

## 11 — Verification

All logs under `🗑️generated/w2a-*.txt`.

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | ✅ 0 errors, reached `Checking semio-framework-ui` (`w2a-ui-check-5.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ✅ 0 errors, reached `Checking semio-framework-os-renderer-wgpu` (`w2a-renderer-check-1.txt`) |
| `cargo check -p semio-framework-ui-scene --lib -j 4` | ✅ (`w2a-scene-check-1.txt`) |
| `cargo check … --target wasm32-unknown-unknown` (both crates) | ✅ 0 errors, both reached their own `Checking` line (`w2a-renderer-wasm-check.txt`, `w2a-ui-wasm-check.txt`) |
| `cargo test -p semio-framework-ui-render --lib` | ✅ 131/131 — includes both new shader laws |
| `cargo test -p semio-framework-ui-scene --lib` | 126 passed / 3 failed — all three failures are **pre-existing and not mine**, see below |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib` | 512 passed / 8 failed after I fixed the one that WAS mine (the slot-table budget); the rest are order-dependent or peer-lane, see below |
| targeted renderer filters (`board2d_engine_tests`, `icon_render`, `engine_surface_attach_tests::tiled_map`, `map_tile_url`, `render_ui_image_step`) | ✅ all green |
| `cargo test -p semio-framework-os-renderer-wgpu --lib engine_canvas:: -- --test-threads=1` | 55 passed / 4 failed — all four are named as pre-existing/other-lane in `📓️w1-integration.md` §5.1 |

### The one failure that was mine, and its fix

`ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack` measured 160616 against a
committed 160592 — the 24 bytes `Paint2dScene.lanes` adds to `Option<UiSurfaceSlot>`. Budget updated
(§6c). Re-run is green.

### Failures I did NOT cause (evidence, not assertion)

- **`semio-framework-ui-scene` ×3** (`board2d_scene_lanes_mirror_…`,
  `board2d_scene_splits_…`, `pack::tests::typed_scene_neutral_catalog_matches_native_serde_contracts`):
  `Board2dScene` gained `grid_visible`/`selectable_nodes`/`selectable_edges`/`selectable_handles`
  without `🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json`, `🧫️fixtures/🧾️typed-scene/🔣️.json` or the
  typed catalog's `Board2dScene` row being updated. `git show HEAD:…board2d-scene-lanes/🔣️.json |
  grep -c gridVisible` is `0` and `grid_visible` is already at HEAD in `🎬️scenes/🦀️.rs`, so this
  predates the packet. The fix is four field names in the fixture's `spineFields` + `roundTrip.spine`
  and four entries in the catalog row — it belongs to whoever added the fields, and it is the same
  fixture family I extended for Paint2d, so it is a one-minute integrator fix.
- **`semio-framework-ui` prepared ×3** (`preparation_yields_at_the_configured_item_budget`,
  `receiver_survives_worker_ownership_of_the_job`,
  `retained_codec_source_moves_once_and_retires_one_page_per_governed_step`): each PASSES alone and
  fails in-suite with `test prepared input must fit fixed process permits`
  (`🎟️prepared/🦀️.rs:1866`) — a process-wide permit pool the test binary leaks across tests. The set
  that fails changes between runs. Matches `📓️w1-integration.md` §5.4(a).
- **`semio-framework-ui` engine/reconcile ×2** (`max_plus_one_stale_aba_…` → `ArenaFull`,
  `large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms` → 174 ms):
  `⚙️engine/🦀️.rs`, `🌳️tree/🦀️.rs` and `🌳️document-tree-reconcile/🦀️.rs` carry 413 lines of
  uncommitted peer edits right now (W1l/W1g lanes); the perf slice is load-dependent under a live
  fleet.
- **renderer `engine_canvas` ×4** (`paint2d_engine_tests` ×3,
  `saturated_graph_and_board_wheel_queues_preserve_cameras`): all four named in
  `📓️w1-integration.md` §5.1 as "in files their packet owners were actively writing".
- **Parallel-only, in this module**: `tiled_map_and_board_windows_attach_…`,
  `a_board_window_paints_its_tool_run_trace_lane_…` and
  `node_graph_window_attaches_the_flow_engine_…` fail with `--test-threads` default and pass serially
  — `ENGINE_SURFACES` and `take_engine_surface_registrations()` are process-wide. My new tiled-map
  test now drains the registration list it appends to so it leaves that shared list as it found it,
  but the module is inherently order-dependent and wants its own serialisation ticket.

### Re-verification note

After the green checks above, a peer's in-flight edits started breaking BOTH crates at
HEAD-of-working-tree, in files this packet never touched. Polled four times over the session; the
error kept MOVING, i.e. the author is iterating live:

1. `resolve_overlay_placement_side` gained a sixth `FlowInline` argument, `⚙️engine/🦀️.rs:1851`
   still called it with five.
2. `write_canvas_pointer_action` in `🎞️Scenes` now takes a `&CanvasPointerWire`,
   `🎞️Scenes/…/🦀️.rs:1236` still passes five scalars (breaks the renderer lib and lib-test).
3. Current state at the end of this packet: `⚙️engine/🦀️.rs:726`
   `self.windows.get(id)` is handed a `SurfaceId` where `&str` is expected (E0308) — the
   `UiFlow`/router lane, W1l.

**Every gate in the table above was measured before that churn, on a tree carrying all of this
packet's edits.** The ONE edit made after the last green renderer check is a single test-only line —
`let _ = take_engine_surface_registrations();` in
`⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`, a symbol that same test module already calls
at `:437`. Re-run

    cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4
    cargo check -p semio-framework-os-renderer-wgpu --lib -j 4
    cargo test  -p semio-framework-os-renderer-wgpu --lib engine_surface_attach_tests -- --test-threads=1

once the three peer call sites above are repaired.
