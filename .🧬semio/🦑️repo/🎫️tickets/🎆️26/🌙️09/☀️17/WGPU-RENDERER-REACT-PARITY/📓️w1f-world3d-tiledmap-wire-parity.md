# 🌍️🧭️ W1f — World3d + TiledMap wire/behaviour parity

Packet W1f of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Every claim below was re-verified against
the live tree before editing; line numbers are post-edit.

---

## 1 — P0: World3d terrain tiles never loaded in production

### Confirmed defect

`sync_terrain_state` inserted every visible-but-uncached DEM tile URL into
`World3dState.pending_terrain_tile_urls`, and **nothing drained that map**. The only consumer was a
unit test that called `state.terrain_session.upload_elevation_tile(...)` by hand. The docstring even
referenced a `fetch_pending_terrain_tiles` in `♾️infinite/🦀️.rs` that does not exist anywhere in the
repo. Terrain was therefore dead on both browser and native: the tile meshes could never build,
because `terrain_tile_mesh_json` stayed `"null"` forever.

### Fix — terrain rides the existing bounded world-asset pipeline

| site | change |
| --- | --- |
| `♾️infinite/🌍️world/🦀️.rs:6949` | new `reserve_terrain_tile_fetch` — admits the tile into `WorldAssetIoAuthority` as `WorldAssetRequestKind::Terrain { z, x, y }` |
| `♾️infinite/🌍️world/🦀️.rs:6973` | new `pub fn apply_world3d_terrain_tile_bytes` — hands fetched bytes to `TerrainSessionCore::upload_elevation_tile`, clears the pending entry, records a miss on refusal |
| `♾️infinite/🌍️world/🦀️.rs:1382`, `:1637` | new `terrain_tile_misses: HashSet<(u32,u32,u32)>` — port of React's `TerrainTileRenderer.tileMiss`, so a tile whose bytes the session refuses is never re-requested for this style (cleared with the style signature at `apply_terrain_style_if_changed_state`) |
| `♾️infinite/🌍️world/🦀️.rs:7421` | `sync_terrain_state` calls `reserve_terrain_tile_fetch` where it used to only `insert` into the pending map |
| `📺️renderer/…/🧊️renderer/🦀️.rs:10330` | new `WorldAssetRequestKind::Terrain` arm in `pump_renderer_asset_decode_step`'s `Ready` phase — collects the sealed pages and calls `apply_world3d_terrain_tile_bytes` |
| `📺️renderer/…/🧊️renderer/🦀️.rs:149` | `collect_world3d_asset_bytes` + `apply_world3d_terrain_tile_bytes` imported unconditionally (was gated `#[cfg(not(wasm32-p2))]` behind the reference-image arm) |

Nothing else was needed: `RendererAssetFetchOwner::World` is already drained generically by the
browser worker (`🌐️browser-worker/🦀️.rs:116`) and by the native fetch pump
(`🧊️renderer/🦀️.rs:10484`), the response cursor for `Terrain` was already `Opaque`
(`validate_utf8: false`, so binary PNG passes), and `reserve` de-duplicates by `(kind, url)` so the
per-frame re-offer is free. `semio_framework_pixels::decode_png` is our own decoder — no `image`
crate — so the apply path is valid on `wasm32-wasip2` too, unlike the reference-image arm.

Back-pressure is **not** a fault: `ItemCapacity`/`ByteCapacity`/`Closing`/`Stale` leave the tile
un-pended so the next frame re-offers it; only a genuine structural refusal records a miss.

### Continuous hypsometric ramp — implemented, layout change was contained

The 10-band bucketing (`TERRAIN_COLOR_BANDS`) existed only because `World3dVertex` had no colour
channel. It does now, and the whole change stayed inside the wgpu target plus the terrain mesh build:

| site | change |
| --- | --- |
| `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:230` | `World3dVertex` gains `color: [f32; 4]` (stride 24 → 40) |
| `🖍️draw/🦀️.rs:2428`, `:2480` | opaque + translucent world pipelines gain `{ offset: 24, shader_location: 2, Float32x4 }` (location 2 was free; instance data starts at 3) |
| `🖍️draw/🦀️.rs:431` | upload cursor reads `lease.vec4(Mesh3dField::Colors, vertex)`, falling back to opaque white — a mesh that declares no colours shades exactly as before |
| `🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs:229` | `WORLD3D_SHADER` `VertexInput` gains `@location(2) color`, and `out.color = instance.color * vertex.color` |
| `🌍️world/🦀️.rs:7083` | `WorldTerrainMeshCursor::vertex_color` replaces `triangle_band` — samples `hypsometric_color(uvs[i*2+1])` per vertex |
| `🌍️world/🦀️.rs` cursor | phases `Count`/`NextBand` deleted, `Colors` added; one mesh per tile instead of ten band meshes; `terrain_band_mesh_key` → `terrain_tile_mesh_key` |
| `🌍️world/🦀️.rs:7376` | `TerrainBandDraw` → `TerrainTileDraw`; the instance tint is now neutral white |

No blocker was hit: `Mesh3dField::Colors`, `Mesh3dSchema.colors`, `mesh3d_write_vec4` and
`Mesh3dLease::vec4` all already existed in `🖱️ui/🎬️scene/📐️math/🦀️.rs` — the lease layer supported
per-vertex colour the whole time; only the GPU vertex struct did not.

Side effect: the cursor is now ~5× cheaper. The banded writer ran `TERRAIN_COLOR_BANDS` full
counting passes plus a per-band position/normal sweep; the new one writes each vertex exactly once
per field.

**Divergence to note.** There are two `WORLD3D_SHADER` copies: `🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders`
(the one this renderer compiles) and `🖱️ui/🖌️render/✨️shader-contract` (the webgpu/d3d12/metal/vulkan
backends, with HLSL/MSL mirrors and a second private `World3dVertex` in
`🖌️render/🎯️targets/🧊️webgpu/🗃️resources/🦀️.rs:320`). No test ties them together, so only the wgpu
copy was changed. Those backends keep position+normal and stay self-consistent, but they will not
show the terrain ramp until someone ports the same three-line change.

---

## 2 — P2: World3d `setCamera` wire shape

### What each renderer sent

| | React `worldCameraSetCameraDispatchArgs` | wgpu **before** | wgpu **after** |
| --- | --- | --- | --- |
| address | `windowId` | bounded writer: `windowId` · `orbit_camera_action`: `surfaceId` | `windowId` (both) |
| `camera.position` | `[x,y,z]` | `[x,y,z]` | `[x,y,z]` |
| `camera.target` | `[x,y,z]` | `[x,y,z]` | `[x,y,z]` |
| `camera.zoom` | number | — | `1.0` |
| `camera.up` | `[x,y,z]` when reported | — | `[0,0,1]` |
| `camera.fov` | — (guest camera value has no such member) | degrees | **removed** |

React's `buildWorldCameraDispatchArgs` (🌐️World3dHost/🟦️.tsx ~768) deliberately sends only
`{position, target, zoom, up?}`; its own docstring records that a stray key (there, a bare
`projection` string) fails deserialization of the whole `camera` value and silently drops the
dispatch. `fov` was exactly that kind of stray key.

**Orthographic zoom semantics.** React derives `zoom` from the live three.js camera:
`captureNavigationSnapshot` reports `camera instanceof ThreeOrthographicCamera ? camera.zoom : 1`
(`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3584`), and `orbitCameraZoomForProjection` only substitutes the
orthographic default of `50` when a projection switch happens. The wgpu orbit is perspective-only —
`OrbitController` carries `fov_y`, never an orthographic frustum — so the identity zoom `1.0` is the
faithful value, pinned as `WORLD3D_PERSPECTIVE_CAMERA_ZOOM` (`🌍️world/🦀️.rs:9174`) with the
derivation written down. `up` is `WORLD3D_ORBIT_UP = [0,0,1]` (`:9179`) because
`OrbitController::to_camera` hard-codes Z-up.

### Sites

- `🌍️world/🦀️.rs:6280`, `:6306` — `plan_world3d_wheel`/`plan_world3d_drag` put the zoom scalar in
  flat-action slot 6 where the fov used to sit.
- `🌍️world/🦀️.rs:6390`–`6425` — the production bounded writer emits `zoom` + `up`, not `fov`.
- `🌍️world/🦀️.rs:9181` — `orbit_camera_action` (the non-bounded twin used by the wheel-settle sweep)
  switched from `surfaceId` to `windowId` and to the same camera value.
- Pinned tests updated: `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:334`+ (now asserts
  `windowId`, absence of `surfaceId`, `zoom == 1.0`, a 3-axis `up`, and absence of `fov`) and
  `🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs:208`.

---

## 3 — P2: TiledMap selection/hover verbs

### Guest evidence

`setFeatureSelection` / `setHover` / `setSelection` / `setSelectionMethod` / `setSelectionMode` were
**deleted** from the GIS app in ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM. The app
now declares `InteractionDefinition { id: "features", granularities: [layer, feature], … }`
(`✏️s/🔌️plugins/🌍️gis/…/✏️editor/🦀️.rs:1127`), which auto-injects
`interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`. Its own context menu builds
`select_feature_action_args` (`:712`) in exactly React's shape. So **every** wgpu tiled-map selection
and hover dispatch was being refused by the guest.

### Payload table

| gesture | React `🧭️TiledMapHost` | wgpu **before** | wgpu **after** |
| --- | --- | --- | --- |
| pick / marquee hit | `interactionSelect {surfaceId, domainId:"features", targets:"<json>", merge, method}` | `setFeatureSelection {surfaceId, positions[], routes[], mode}` | same as React |
| pick / marquee miss | `clearSelection {surfaceId}` | `setFeatureSelection` with empty arrays | `clearSelection {surfaceId}` |
| hover change | `interactionHover {surfaceId, domainId:"features", channel:"pointer", targets:"<json>"}` | `setHover {surfaceId, hover:{kind,id}\|null}` | same as React |
| pan / wheel | `setCamera {surfaceId, camera:{x,y,zoom}}` | `setCamera` + `setFeatureSelection` + `setHover` every tick | `setCamera` only |

`targets` is JSON **text**, not a node array — the wire shape both
`mapFeatureSelectionActionArgs` and the guest's `select_feature_action_args` build. `granularity` is
`"feature"`; the hover channel is `"pointer"`.

### Sites

- `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5823` — `MAP_INTERACTION_DOMAIN`/`MAP_FEATURE_GRANULARITY`/
  `MAP_HOVER_CHANNEL` constants; `:5829` `map_feature_targets_json`.
- `:5839` `write_tiled_map_selection` — now emits `interactionSelect` or, on a miss, `clearSelection`
  (React's split exists because `next_selection` treats empty `targets` as a no-op, not a clear). It
  gained a `method` argument: `"pick"` for a click, the marquee's own method for a drag, mirroring
  `emitFeatureSelection`'s two call sites.
- `:5879` `write_tiled_map_hover` — `interactionHover`; a cleared hover publishes an empty `targets`
  array, which is what clears the guest hover.
- `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4310` — `write_map_interaction_actions` (3 actions per
  gesture) replaced by `write_map_camera_action` (1). A wheel tick no longer re-publishes the whole
  selection and hover; `with_map_interaction_into` reserves 1 action instead of 3.
- `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4234` — `map_marquee_mode`'s unmodified case was `"default"`,
  which is in **no** `MergeMode` taxonomy (`replace | additive | subtractive | invertive | range`);
  React's `marqueeModeFromModifiers` returns `"replace"`. Fixed, with the pinned test in
  `🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs:603`.

### Wheel-zoom reachability — confirmed live

`🧊️renderer/🦀️.rs:12071` calls `engine_canvas::tiled_map_wheel_into(...)` on a real wheel event with
the surface bounds and controller id; it reaches `with_map_interaction_into` →
`MapInteractionIntent::Wheel` → `map_wheel_screen` + `clamp_camera_to_world_bounds` → a published
`setCamera`. The path is live, not orphaned. The only behaviour change here is that it no longer
drags two refused actions along with the camera.

### Map context menu — implemented

The orphaned `/** @emoji 🗺️ Pushes GIS map context-menu items for a screen-space hit. */` comment in
`🎞️Scenes` had no body. It does now:

- `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5793` — `tiled_map_context_menu_target(surface_id, inner, x, y)`
  returns `(Vec<ContextMenuHit>, Vec<ContextMenuSelectionGroup>)`: the feature under the pointer
  (`hit_test_feature_json`, domain `"route"` or `"position"`) plus the live selection split into its
  two domain groups — the port of `onContextMenu` in `🧭️TiledMapHost/🟦️.tsx:1194`.
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9296` — `resolve_context_menu_surface` now returns a 4-tuple and
  calls the above for a tiled-map surface; `open_context_menu` forwards the groups as the request's
  `surface.selection`, which was hardcoded to `Vec::new()`.

Those are exactly the domains `gis2d_context_menu_items` matches on (`feature`/`position`/`route`)
and the groups that decide whether its `clearSelection` row renders enabled.

The Board2d `board_context_menu_items` the packet pointed at does not exist under that name; the
wgpu context menu is generic (`resolve_context_menu_surface` → `program.context_menu(request)`), so
the fix is the hit/selection resolution above rather than a bespoke item list.

---

## 4 — P1: World3d drag-and-drop, Escape, cursor

### Catalogue drag-and-drop — already wired, no change needed

Contrary to the packet, the World3d drop path exists and is live in
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`:

- `sync_world3d_catalogue_drop_preview` (`:8844`) — the ghost, from `puzzle3d_catalogue_drag_payload`
  (React's `parsePuzzle3dCatalogueDragPayload`), with a retained-tree variant at `:8863` for panel
  catalogues.
- `world3d_catalogue_drop_action` (`:8884`) — dispatches `addObjectKind {objectKind, origin}`.
- `finish_tree_drag` (`:8897`) tries the flow-widget drop, then this, then the node-graph drop.

React dispatches the identical `dispatch("addObjectKind", { objectKind, origin })`
(`🌐️World3dHost/🟦️.tsx:7112`). The wgpu version is at parity; only the `dragEnter/Leave/Over` DOM
plumbing differs, which is a DOM-vs-immediate-mode difference, not a wire one.

### Escape-cancels-relocate — blocked, and the blocker is bigger than the packet

The wgpu World3d **has no production relocate drag to cancel**. `drag_object_id`,
`drag_last_position`, `update_dragged_instance_position` and the `worldRelocate` dispatch all live
inside `#[cfg(test)] fn handle_world3d_pointer_button` / `handle_world3d_pointer_drag`
(`🌍️world/🦀️.rs:10760`, `:10950`), which is a deliberate test-only oracle — `🔬️unit/🦀️.rs:1484`
asserts the production source does **not** contain `handle_world3d_pointer_button`. Repo-wide,
`worldRelocate` is emitted from that one `cfg(test)` function and nowhere else.

Adding an Escape rung would guard state no production gesture ever sets. The real gap is the missing
production relocate path (React `World3dHost` `beginRelocateDrag`/`endRelocateDrag`, ~6773–6848),
which is a packet of its own. **Not implemented; recommend a follow-up packet.**

The second half of that bullet, the suggestion menu (`World3dHost:6221`, `closeVortexSuggestions`),
has no wgpu counterpart at all — vortex suggestion menus are not implemented in the wgpu renderer.
Also a follow-up.

### Hover cursor policy — the premise was wrong; wgpu is already at parity

`👆️cursor/🦀️.rs:79`'s `HitKind::World3d => SemioCursor::Default` is **correct**. React's
`🌐️World3dHost/🟦️.tsx` sets no cursor anywhere: greps for `cursor`, `style.cursor`, `cursor:` and
`grab` over the whole `🌐️World3dHost` folder and `♾️infinite/🌍️world/🎨️r3f` return only docstring
prose and an unrelated `ToolRunTraceCursor`. There is no grab/pointer-over-gizmo policy to mirror.
**No change made** — changing it would have introduced a divergence, not removed one.

---

## Verification

Run in the foreground; logs under `🗑️generated/w1f-*.txt`.

| command | result |
| --- | --- |
| `cargo check -p semio-framework-ui --lib --keep-going` | **0 errors** (covers the `World3dVertex` + `WORLD3D_SHADER` change) |
| `cargo test -p semio-framework-os-infinite --lib terrain` | **26 passed, 0 failed** |
| `cargo test -p semio-framework-os-infinite --lib pointer_gesture` | **7 passed, 0 failed** — includes `a_camera_gesture_addresses_the_window_that_owns_the_surface` |
| `cargo test -p semio-framework-os-infinite --lib world::` | 137 passed, **7 failed — all pre-existing, none in this packet's surface** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | 12 errors, **all in worker W1b's Canvas2d block**, none in this packet's regions (see below) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going` | the **same** 12–13 errors and no others — this packet's code, including the now-uncfg'd terrain apply path, compiles for wasm |

### The 7 pre-existing `world::` failures

`live_renderer_retains_generation_wake_…`, `prepared_world_resources_are_send_and_deduplicate_uploads`,
`world_authority_retains_front_plan_across_output_saturation_…`,
`world_component_marquee_cursor_matches_legacy_vertex_edge_face_geometry`,
`world_component_marquee_publish_merges_before_one_atomic_set_selection`,
`world_object_registry_enforces_capacity_revision_and_aba`,
`world_saturation_owner_blocks_new_ingress_until_exact_fifo_transfer`.

They fail identically single-threaded, so it is not shared-authority interference. `git diff` over
`🌍️world/🦀️.rs` shows this packet touched only the terrain block, the camera flat-action slot and
`orbit_camera_action` — none of these tests reach any of that. They exercise the object registry,
the component-marquee cursor and action/mesh capacity, all of which live in `🖱️ui` and are being
refactored by peers this session (one failure is a component-id format change:
`numeric component id: ParseIntError`). Attributed to concurrent work, not to W1f.

### Renderer-crate check blocked

Every remaining error, on **both** targets, sits in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` in regions
owned by worker W1b and mid-edit across the whole session. Watched over ~12 build attempts it went
18 → 17 → 14 → 13 → 12 as W1b landed fixes: `IconName` unimported (`:1886`, `:2274`) — fixed;
`InkDocumentJson` missing `grid_*` fields, then an `f32`/`f64` mismatch at `:5698` — fixed; still
open are the `canvas_lum`/`canvas_sat`/`canvas_set_lum`/`canvas_set_sat` blend helpers at
`:3280`–`:3283`, which have no definition in the crate.

**Zero** errors were reported in any region this packet edited, on either target: the TiledMap block
of `🎞️Scenes` (`:5793`–`:5900`), the TiledMap block of `⚙️EngineCanvas` (`:4234`, `:4310`),
`🐚️Shell`'s context-menu resolution (`:9296`) or `🧊️renderer`'s asset pump (`:149`, `:10330`).
Warnings were emitted from `⚙️EngineCanvas:2603`+ and `🎞️Scenes:1606`, and a typeck error landed at
`🎞️Scenes:5698`, so resolution and per-body type checking did run across the whole crate — the
absence of errors in this packet's bodies is a real signal, not an early abort.

Still outstanding once W1b's `canvas_*` helpers land: one clean crate-level run on both targets, plus
`cargo test -p semio-framework-os-renderer-wgpu --lib world3d_orbit_and_wheel` (the engine-surfaces
`setCamera` assertions this packet rewrote).

Build attempts were repeatedly OOM-killed (exit 137, load average 57, swap 4.3/5.1 GB) — the fleet is
saturated; `-j 2`/`-j 3` got through on alternate attempts.

A late re-run of the two green `semio-framework-os-infinite` test commands failed to *build* because a
peer had just broken `semio-framework-ui`'s `📌️mounted_layout`/`⚙️engine` (`:588`, `:589`, `:681`,
`:571`) — untouched by this packet, whose only `🖱️ui` edits are in `🖍️draw` and `🎨️shaders` and were
green in `cargo check -p semio-framework-ui --lib`. The recorded pass counts above are from the runs
made before that breakage landed.

---

## Remaining gaps

1. **Re-run the renderer crate check** (native + `wasm32-unknown-unknown`) and the engine-surfaces
   test once W1b's `🎞️Scenes` `canvas_lum`/`canvas_sat`/`canvas_set_lum`/`canvas_set_sat` helpers
   exist. Nothing in this packet is expected to fail; the assertions in
   `🧩️wgpu-engine-surfaces/🦀️.rs` were updated to the new `setCamera` shape.
2. **Production World3d relocate drag** — absent behind `#[cfg(test)]`; Escape-cancel is meaningless
   until it exists (item 4).
3. **Vortex suggestion menu** — no wgpu counterpart at all (item 4).
4. **The other `WORLD3D_SHADER`** (`🖌️render/✨️shader-contract`, plus HLSL/MSL mirrors and the second
   `World3dVertex` in `🎯️targets/🧊️webgpu/🗃️resources`) still has no vertex colour, so the
   webgpu/d3d12/metal/vulkan backends will render terrain untinted. Nothing ties the two copies
   together — that duplication is itself worth a ticket.
5. **`apply_map_tile_bytes` has no caller** (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3885`), and
   nothing reserves a `WorldAssetRequestKind::MapTile` anywhere in the repo. Tiled-map raster/vector
   tiles are dead exactly the way terrain was — the same class of defect as item 1, one surface over.
   Out of this packet's scope; recommend it as the next P0.
6. **`apply_ui_image_bytes` likewise has no caller**, and `pump_renderer_asset_decode_step` returns
   early for every `RendererAssetFetchOwner::Shared(_)` in its `Ready` phase
   (`🧊️renderer/🦀️.rs:10327`) — so the whole shared (non-World) asset lane decodes nothing. Same
   family as 5.
7. **wgpu's NodeGraph `interactionSelect` omits `surfaceId`** while React's generic dispatch injects
   it; the map writers added here follow React. Harmless (the guest reads `domainId`), but the two
   wgpu surfaces now spell the same verb slightly differently.
