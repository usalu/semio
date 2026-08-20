# 📓️ Packet `render-scene` — report

Anchor commit `5e7b8046be`. Source ported from `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs` (read-only, not modified). Files owned and rewritten wholesale:

- `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️scene.rs`
- `…/🦀️tessellate.rs`
- `…/🦀️resource.rs`

## Done

**`resource.rs`** (region `🔖️Resource`): generational `TextureId`/`MeshId`/`AtlasId` (shared shape via a `resource_id!` macro over a crate-private `Slot`/`pub(crate) trait ResourceId`), `ResourceState`, `ResourceOp` (`UploadAtlas`/`UploadTexture`/`CreateOrUpdateMesh`/`EvictTexture`/`EvictMesh`), and `ResourceRegistry` — string interning, upload-request dedup (no-op once `Resident`), `mesh_content_hash` (ported FNV-1a from `mesh_content_version`) keyed mesh interning, `drain_ops`, and `report_device_loss` which re-marks surviving ids `Requested` **without** a generation bump (identity preserved) while `evict_*` bumps the generation (identity retired, slot reused). 8 tests.

**`tessellate.rs`** (region `🔖️Tessellate`): pure functions only — `snap_to_device_pixels`/`snap_rect`/`snap_point`, `thick_line_positions` (extracted from `push_line`'s body), `dashed_line_segments` (verbatim), `triangle_fan_positions` (extracted from `push_triangle_fan`'s body), `ear_clip_polygon` + `point_in_triangle`/`sign` (verbatim), and the silhouette-mask chain `union_scissors`/`merge_scissor_bounds`/`layer_scissors`/`mask_instances` (verbatim, `UiInstance::solid` → `QuadInstance::solid`). 9 tests including the ported `SilhouetteClipTests` mask-reset/empty-clip cases.

**`scene.rs`** (regions `🔖️Primitives`, `🔖️SceneBuilder`, `🔖️Finish`, `🔖️RenderPacket`): `LayoutRect`/`ScissorRect`/`ClipRegion`, `QuadInstance` (byte-identical to `UiInstance`: `rect`/`color`/`params`/`uv_rect`, 64 bytes) with all 9 `KIND_*` constants and constructors, `VectorVertex`, `GlassStyle`/`GlassRegion`/`GlassInstance`, `StencilPolicy` (plain replacement for `wgpu::StencilState`), the `Surface*` family (`SurfacePass`/`SurfaceMeshDraw`/`MeshInstance`/`SurfaceLineDraw`/`LineVertex3`/`SurfaceTexturedDraw`/`TexturedMeshInstance`), `PipelineKind`; `SceneLayer`/`SceneBuilder` (full `DrawList` API surface, all sync); `Scene::finish` as validate → snap → order → batch → hash (implemented as private free functions called from the one public `Scene::finish`); `RenderPacket`. 22 tests: the ported `SilhouetteClipTests`, layer-watermark/scissor-split tests, a `QuadInstance` byte-layout test (`size_of` + `offset_of!` for all 4 fields), content-hash stability/sensitivity tests, pixel-snapping determinism at dpr 1.0/1.5/2.0, stack-balance validation, empty-layer dropping, layer merging, and mask/batch-range correctness.

No `wgpu`, no `winit`, no `async fn` anywhere in the three files. Every non-trivial `fn` carries the `// 🚫️async: U1 …` tag.

## Acceptance

Per binding ruling **U4** (`📌️important.md`, this ticket): executors write code and mark acceptance **UNRUN** — only `sol` runs cargo gates in this tree (subagent builds cannot report across the turn boundary, and the U-program's concurrent ~13 agents make even a 600s window unreliable). I ran no cargo command.

**UNRUN** — exact commands for `sol` to run, `timeout: 600000`, from `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust`:
```
cargo test -p semio-framework-ui-render --lib
bun ./📜️script.ts boundaries
```
Sibling packets (`backend-iface`'s claim on `resource.rs` per its scaffold header vs. this packet's explicit brief — see Deviations; `shader-repair`, `render-elements`, etc.) are editing other files in this crate concurrently; unresolved names from `🦀️backend.rs`/`🦀️shader_contract.rs`/`🦀️element.rs`/`🦀️frame.rs`/`🦀️layout.rs`/`🦀️schedule.rs`/`🦀️dispatch.rs`/`🦀️text.rs`/`🦀️surface.rs` are not this packet's.

I did do cheap non-cargo sanity checks: `//#region`/`//#endregion` balance (26/26, 7/7, 11/11), brace/paren/bracket balance per file (all equal), and hand-verified the FNV-1a prime constant against the original `0x100000001b3` literal with `python3 -c` (caught and fixed a transcription bug — see Dropped-future bugs / arithmetic below).

## Dropped-future bugs found while porting

Confirmed by grepping `draw.rs` for `self.push_line(...)`/`self.push_line_overlay(...)` called as a bare statement (no `.await`) inside another `async fn`:

- `draw.rs:520` — `DrawList::push_dashed_line` calls `self.push_line(sx0, sy0, sx1, sy1, color, width);` with no `.await`. `push_line` is `pub async fn`; the call constructs a `Future` and drops it unread — every dashed line silently draws nothing.
- `draw.rs:527` — `DrawList::push_dashed_line_overlay` has the identical shape against `push_line_overlay`.

Both disappear by construction here: `tessellate::dashed_line_segments` is a plain sync `fn`, and `SceneBuilder::push_dashed_line`/`push_dashed_line_overlay` call the now-sync `push_line`/`push_line_overlay` directly — there is no `async fn` anywhere in the call chain for a bare call to silently no-op.

I checked every other function I ported (the full `DrawList` impl block, `ear_clip_polygon`, the mask/batch helpers) for the same shape and found no further instances of *that specific pattern* (a discarded bare statement call to a would-be-async fn) within the code this packet actually ports. Separately, and out of this packet's scope: essentially every method in `draw.rs` is `pub async fn` with call sites written for a sync fn (e.g. `UiInstance::solid(rect, color)` passed directly into `.push(...)` without `.await`, which would be a type error, not a silent no-op) — consistent with master.md's note that the committed wgpu-engine path does not currently compile. `draw.rs` is being deleted by a later packet; I did not attempt to catalogue every such site since none of them are code this packet ports.

## Decisions

- **`PipelineKind` landed here.** `shader_contract.rs` (packet `shader-repair`) was still the 8-line scaffold with no `PipelineKind` at the time this packet ran (grepped the whole `🖼️render` tree — no hits). Defined in `scene.rs`'s `🔖️Primitives` region per the ticket's own fallback instruction. `shader-repair` should treat this as the canonical definition, not redefine it.
- **`SurfacePass` (not `kernel_3d_scene::ScenePass3d`) models `RenderPacket::surface_passes`.** The crate has no `Cargo.toml` dependency on the sibling `🖱️ui/🎬️scene` module, and that module's own constructors (`Camera3d::view_proj`, `Instance3d::model_from_trs`, …) are `pub async fn` under R2 — calling them from this crate's literal-sync functions would need an `.await` this crate cannot have (ruling U1). `SurfacePass`/`SurfaceMeshDraw`/`MeshInstance`/`SurfaceLineDraw`/`LineVertex3`/`SurfaceTexturedDraw`/`TexturedMeshInstance` mirror `ScenePass3d`'s shape exactly (viewport/view_proj/light_dir/draws/translucent_draws/line_draws/textured_draws/layer_index/watermarks) but with `MeshId`/`TextureId` in place of `String` keys.
- **Stencil mask quads live in `RenderPacket::quad_instances`, not a separate field.** The ticket's `RenderPacket` field list has no dedicated mask-instance array; `DrawBatch.mask_range` is a sub-range into the same flat `quad_instances` a backend already has, appended after each batch's own content range. This keeps the packet's field list exactly as specified.
- **`Scene::finish`'s five steps are private free functions**, not five public `Scene::` associated functions — `Scene::finish` is the only public entry point. Matches "validate → snap → order → batch → hash" as an internal pipeline, not five separately-callable API surfaces.
- **`order`'s merge rule: drop empty layers first, then merge consecutive survivors with identical `(scissor, clip, foreground_of)`.** A naive "merge only literally-adjacent original layers" would never coalesce anything, because `push_scissor`/`pop_scissor`/`begin_silhouette_clip`/`end_silhouette_clip` always insert a fresh (usually empty) boundary layer between same-state runs. A layer referenced by a `SurfacePass.layer_index` is an unmergeable barrier in both directions (its watermark offsets must stay exactly valid); `scene_passes[i].layer_index` is remapped after ordering.
- **Did not port:** `paint_selection_marquee`/`selection_marquee_stroke`/`selection_marquee_fill`/`push_marquee_segment`/`SELECTION_MARQUEE_*` constants and their tests. These are product-policy code keyed on `Theme` (`theme.selected`), not backend-neutral scene primitives, and this crate has no `Theme` dependency (`ui_contract`'s `style.rs` only has semantic tokens, no RGBA type) — the caller is expected to resolve a stroke/fill `[f32;4]` itself and call `SceneBuilder::push_dashed_line`/`push_triangle_fan`/`push_solid` directly, same as `push_glass` now takes a pre-resolved `GlassStyle`. Also did not port `IconAtlas`, `RasterTexture`, `RasterTextureTable`, `MeshGpuTable`/`GpuMeshBuffers`, `WorldGlobalsRing`, `GrowBuffer`, `FrameBuffers`, `UiPipelines` — all either hold `wgpu::` types directly or are string-keyed GPU caches superseded by `ResourceRegistry`.

## Registrar-requests

None. All three owned files already existed as scaffolds; no `Cargo.toml`/`project.json`/taxonomy change was needed.

## Deviations

- **`resource.rs`'s own scaffold header said "owned by packet `backend-iface`."** My direct brief lists `resource.rs` as one of exactly three files this packet owns. I followed the direct brief (more specific and current than a placeholder comment) and rewrote it; the stale scaffold text is gone as part of the rewrite. Flagging this explicitly in case `backend-iface` was also told it owns this file.
- **Dropped the wgpu target's `screen_h`/`set_screen_height` plumbing entirely.** In the source, `ScissorRect::from_rect(rect, _screen_h)` never used the parameter (already prefixed `_screen_h` in `draw.rs`), and `ClipRegion::from_rects(rects, screen_h)` only forwarded it into that dead parameter. `LayoutRect`/`ScissorRect::from_rect`/`ClipRegion::from_rects` here take no screen-height argument at all; the ported `draw_list_push_scissor_splits_layers` test simply omits the `set_screen_height` call it no longer needs (that call was a no-op in the original too).
- **New local `LayoutRect` type instead of `semio_framework_geometry::Rect`.** `semio_framework_geometry::Rect::new`/accessors are `pub async fn` (R2) — not in U1's override scope — so calling them from this crate's literal-sync `fn`s isn't possible without an await point this crate can't have. `LayoutRect` is a plain `{x, y, w, h}: f32` struct, same shape as the wgpu target's own local `crate::wgpu::geometry::Rect`.
- **`GlassStyle` is a new plain struct** (`{tint, alpha, blur_px, saturate}: [f32;4]/f32`), replacing the wgpu target's dependency on `Theme::glass(Level)` — this crate has no theme/level type to resolve one itself, matching the ticket's own instruction ("callers derive `style` from `Theme::glass(level)` themselves").
- **`mask_range`/batch granularity is per-`SceneLayer`, shared across that layer's quad/raster/vector `DrawBatch`es**, matching the source's semantics where one `LayerBatch` (per layer) fed a single mask range consumed by both its UI-instance and vector-vertex draws. Raster batches are additionally split into contiguous same-`TextureId` runs within a layer (the source's per-key `RasterTextureTable` bind-group-per-texture requirement), each reusing that layer's one mask range.
- **Found and fixed a hand-transcription bug of my own before finalizing**: the FNV-1a 64-bit prime, first written as `0x0000_0001_0000_01b3` (= `0x1000001b3`), did not equal the source's `0x100000001b3`. Verified both the bug and the fix numerically (`python3 -c 'print(hex(0x100000001b3), hex(0x0000_0100_0000_01b3))'`) before writing the corrected `0x0000_0100_0000_01b3` into both `resource.rs::mesh_content_hash` and `scene.rs::fnv1a64`.
