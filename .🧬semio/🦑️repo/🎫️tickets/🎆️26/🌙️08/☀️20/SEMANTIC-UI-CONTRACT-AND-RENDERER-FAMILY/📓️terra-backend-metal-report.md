# 📓️ terra-backend-metal-report

Packet `backend-metal`, wave W3.

## Done — reached milestone 6 (all six), with one documented interleaving limitation

`MetalBackend` — a concrete hand-written Metal implementation of `ui_render::GraphicsBackend` — in
`🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/`, 2,668 lines across 9 files
(8 new region files + rewritten `📦️glue.rs`):

- `🦀️msl.rs` — hand-written MSL for all five shader families (UI megashader, vector, world3d
  mesh/lines, blur downsample + scene blit, glass), transcribed line-for-line from the canonical WGSL
  in `ui_render::shader_contract`. Interim path — see "Shader strategy" below.
- `🦀️types.rs` — GPU-layout structs this crate needs beyond what `ui_render` already exports as
  `Pod` (`QuadInstance`/`VectorVertex`/`GlassInstance` are used directly): `WorldGlobalsGpu`,
  `World3dGpuInstance`, `World3dGpuVertex`, `WorldLineGpuVertex`, `WORLD_GLOBALS_SLOT_SIZE`,
  `UNIT_QUAD_CORNERS`, `BlurMipGpu`.
- `🦀️pipelines.rs` — `Pipelines`: every `MTLRenderPipelineState`/`MTLDepthStencilState`/
  `MTLSamplerState` this backend needs, built from `🦀️msl.rs`. 9 pipelines, 4 depth-stencil states
  (Metal's state split collapses two of the wgpu target's six depth-stencil configs onto one shared
  object — see the file's header for why).
- `🦀️resources.rs` — `GpuResources`: `ResourceOp` application (atlas/texture/mesh upload+evict),
  typed `HashMap<TextureId,_>`/`HashMap<MeshId,_>` tables, glyph/icon atlas routing by upload byte
  density (the contract has no `AtlasKind` — see the file's header), 1x1 dummy atlas textures seeded
  at construction so the UI megashader always has something bound.
- `🦀️scene_target.rs` — `SceneTarget`: the offscreen scene-color target + blur-scratch texture, both
  full 5-level mip chains. No per-mip `TextureView`s (Metal's explicit-LOD `sample()` and
  per-attachment `level` property make them unnecessary — see `🦀️msl.rs`'s header).
- `🦀️frame_buffers.rs` — `GrowBuffer` (capacity-doubling `Shared`-storage upload buffer),
  `FrameBuffers`. Five buffers, not the wgpu target's per-batch-group set — `RenderPacket::
  quad_instances`/`vector_vertices` are already flat arrays every `DrawBatch::instance_range`/
  `mask_range` indexes into, so this backend uploads each once per frame and slices by byte offset.
- `🦀️world3d.rs` — `WorldGlobalsRing` (a plain byte-offset ring buffer — Metal has no "dynamic
  offset bind group" the way wgpu does), `upload_world_passes`, `encode_passes` (opaque/translucent
  mesh + lines). Textured mesh pipeline not implemented — `shader_contract.rs` itself documents
  `WORLD3D_TEXTURED_PIPELINE` as inferred, never wired to a real pipeline in the reference.
- `🦀️backend.rs` — `MetalBackend` itself: device/queue/`CAMetalLayer` construction (real + headless),
  resize (zero-size park/restore), the two-pass `render()` (offscreen scene pass → blur/blit/glass/
  foreground composite pass), the `GraphicsBackend` impl, `backend-testing`'s
  `debug_force_device_loss`/`recover`/`read_back`. 5 `#[cfg(test)]` tests, all device-gated.
- `📦️glue.rs` — mounts all of the above; the `compile_error!` macOS-only guard kept from the scaffold.

### Milestones, against the ticket's six

1. **Device + layer + swapchain** — done. `MTLDevice`/queue, `CAMetalLayer` attached to an AppKit
   `NSView` via `raw_window_handle`, `resize` (zero-size parks: `size` is recorded but
   `scene_target`/`depth_texture` are left at their last valid size, and `render` refuses to draw
   while parked), `nextDrawable`/present, clear-colour frame.
2. **UI quads + vector** — done. The SDF megashader (all 9 `kind` branches transcribed), vector
   triangles, glyph/icon atlas + per-texture raster upload, per-batch scissor is a no-op by design
   (see "Decisions" — clipping is entirely stencil-driven, matching the wgpu reference).
3. **Stencil silhouette clip** — done. `mask_depth_stencil`/`content_depth_stencil` mirror
   `mask_stencil_state()`/`content_stencil_state()` exactly; `draw_silhouette_mask` replays
   `mask_range` verbatim (reset-bounds quad at ref 0, then every "piece" quad at ref 1).
4. **Offscreen scene target → mip blur → glass composite → blit** — done. `SCENE_MIP_LEVELS = 5`,
   blit-copy-then-downsample per level, `scene_blit_pipeline` to the drawable, glass composited as one
   instanced draw (see "Decisions").
5. **World3d** — done for mesh (opaque+translucent) and lines, with a real depth buffer
   (`Depth32Float_Stencil8`) and the byte-offset globals ring. **Not interleaved with 2D content
   layer-by-layer** — see "Known limitation" below; this is the one place I stopped short of the wgpu
   reference's exact behaviour, and it is a contract gap, not a shortcut.
6. **Device loss + `backend-testing`** — done. `debug_force_device_loss`/`recover` are necessarily
   *simulated* (Metal has no programmatic "lose the device" API); `read_back` blits the just-presented
   drawable into a `Shared`-storage texture and `getBytes`.

## Known limitation — world3d is not interleaved with 2D content layer-by-layer

The wgpu reference (`🎯️targets/🧊️wgpu/🦀️draw.rs`) interleaves a `SurfacePass`'s 3D draws between the 2D
UI/vector content painted immediately before/after it *within the same scissor layer*, using
`SurfacePass::layer_index` plus `quad_watermark`/`vector_watermark` to find the exact split point in
its own `DrawList::layers`. The backend-neutral `ui_render::scene::DrawBatch` — the only per-batch data
a `GraphicsBackend` actually receives — carries **no `layer_index` and no watermark**, and
`Scene::finish`'s `order()` step remaps `SurfacePass::layer_index` to a position in an internal, merged
layer list that `RenderPacket` never exposes. By the time a `RenderPacket` reaches this crate, that
index names nothing a backend can look up. I confirmed this by re-reading `🦀️scene.rs`'s `DrawBatch`,
`LayerState`, and `RenderPacket` field lists in full — there is no substitute field.

This is a gap in `ui_render::scene` (packet `render-scene`), not in this crate's `OWNS` list, so I did
not — and could not — fix it here. Given that, `🦀️world3d.rs::encode_passes` renders every
`SurfacePass` for a frame as one group, inside the offscreen scene pass, after all backdrop-normal 2D
batches and before backdrop-overlay 2D batches. For typical UI content (3D viewports occupying their
own region, not interleaved pixel-for-pixel with unrelated 2D chrome) this produces the correct visual
result; it only diverges from the reference when 2D content is deliberately painted *between* two
different 3D passes within one scissor layer, which no current fixture exercises. Flagging for `sol`:
the clean upstream fix is either putting `layer_index`/watermark fields on `DrawBatch` itself, or
replaying `SurfacePass` draws inline as their own `DrawBatch`-like entry in `Scene::finish`'s ordered
output — I'd rather someone who owns `scene.rs` pick the shape than have a `backend-metal`-only patch
drift from what the other three backends will need too.

## Acceptance: UNRUN

Per U4, I ran no cargo command. Commands for `sol`, `timeout: 600000`, `CARGO_TARGET_DIR` in the
session scratchpad:

```
cargo check -p semio-framework-ui-backend-metal --target aarch64-apple-darwin
cargo check -p semio-framework-ui-backend-metal --target aarch64-apple-darwin --all-targets
cargo check -p semio-framework-ui-backend-metal --target aarch64-apple-darwin --features backend-testing
cargo test -p semio-framework-ui-backend-metal --target aarch64-apple-darwin --features backend-testing
cargo tree -p semio-framework-ui-backend-metal --invert wgpu --target aarch64-apple-darwin
```

**Cheap non-cargo checks performed instead, both green:**
- Brace/paren/bracket balance over every file (including the MSL string literals): all equal.
- `//#region`/`//#endregion` balance per file: all equal.

| file | region/endregion |
|---|---|
| `🦀️msl.rs` | 6/6 |
| `🦀️types.rs` | 4/4 |
| `🦀️pipelines.rs` | 4/4 |
| `🦀️resources.rs` | 1/1 |
| `🦀️scene_target.rs` | 1/1 |
| `🦀️frame_buffers.rs` | 2/2 |
| `🦀️world3d.rs` | 1/1 |
| `🦀️backend.rs` | 7/7 |

I read the vendored `objc2 0.6.4`/`objc2-metal 0.3.2`/`objc2-quartz-core 0.3.2`/`objc2-foundation
0.3.2`/`raw-window-handle 0.6.2` source (`~/.cargo/registry/src/index.crates.io-*/<crate>-<version>/
src/`) for every non-obvious API surface before using it — every method's exact `unsafe`/safe marker
(Metal's own binding is inconsistent about this: `setOffset`/`setBufferIndex`/`setWidth`/`setHeight`
are `unsafe fn`, `setFormat`/`setPixelFormat`/`setStride`/`setStepFunction` are plain `fn`, checked
individually rather than assumed uniform), every struct field name/order (`MTLScissorRect`,
`MTLViewport`, `MTLClearColor`, `MTLRegion`/`MTLOrigin`/`MTLSize`), every enum's numeric layout via its
`pub const` list, the `AsRef<ProtocolObject<T>> for ProtocolObject<P>` blanket impl that makes
`drawable.as_ref()` a valid protocol upcast, and the `MTLDrawable`/`CAMetalDrawable` supertrait
relationship it relies on. One general (non-Metal) language pattern — the hand-rolled `block_on` for
the test module's `async fn` construction calls — I did compile and run standalone with plain `rustc`
in the session scratchpad (`/private/tmp/.../scratchpad/wakercheck.rs`, not touching this crate or its
Cargo workspace) to confirm the mutually-referential `const VTABLE`/`const RAW_WAKER` `RawWaker`
pattern is legal Rust; it printed `42` as expected. No objc2/Metal call was compiled anywhere.

## Decisions

**Shader strategy — interim hand-written MSL, not the planned `naga` build-time cross-compile.** The
plan needs a `build.rs` + `[build-dependencies] naga` line in this crate's `Cargo.toml`, which is
registrar-only (U7) — see Registrar-requests below. Every MSL string in `🦀️msl.rs` is transcribed
directly from the corresponding WGSL constant in `ui_render::shader_contract`, function-by-function,
branch-by-branch (the UI megashader's 9 `kind` branches, the exact ring-animation constants —
`two_pi`, durations 1.6/3.2s, dash period 12, pulse curves — all copied verbatim, not re-derived).

**Metal's state split changes the pipeline/state count vs. wgpu, in a way worth flagging.** In wgpu
(and the WGSL contract's `PipelineSpec`), blend mode, colour-write-mask, depth-compare, depth-write and
stencil test/ops are all baked into one `RenderPipeline`. In Metal only shader functions, vertex
layout, and per-attachment format/blend/write-mask live in `MTLRenderPipelineState`; depth
compare/write and stencil test/ops live in a separate `MTLDepthStencilState` bound at encode time; cull
mode, winding, depth bias and viewport/scissor are pure encoder state with **no object at all**.
Concretely: the wgpu target's `world_pipeline_translucent` and `world_line_pipeline` have identical
depth/stencil behaviour and differ only in depth *bias* (encoder state in Metal) — they collapse onto
one shared `world3d_translucent_depth_stencil` here, with the bias applied/reset around the translucent
mesh draws specifically (`🦀️world3d.rs::encode_passes`).

**No per-batch hardware scissor rect.** I re-read the wgpu reference's `render_interleaved_layers`/
`draw_ui_instances` closely and confirmed it never calls `set_scissor_rect` to a batch's specific clip
rect — `push_scissor`'s plain rectangle is folded into the *same* silhouette-mask/stencil machinery as
`begin_silhouette_clip` (`mask_instances` takes `scissor` and `clip` together). Hardware scissor is
only ever reset to the full viewport (inside `draw_silhouette_mask` itself, and once per batch group).
I ported that exactly — `encode_2d_batches` never narrows the scissor rect per batch.

**One flat upload per array, not one per batch group.** `RenderPacket::quad_instances`/
`vector_vertices` are already single flat arrays covering every batch (backdrop, foreground, overlay,
and the mask quads `Scene::finish` appended) with every `DrawBatch::instance_range`/`mask_range`
already an offset into them. Rather than re-collecting each filtered batch group into a fresh buffer
(the wgpu target's approach, needed because its own `DrawLayer`s are the unit of storage), this backend
uploads each array once per frame and reads every batch's slice by byte offset — see `🦀️frame_buffers.
rs`'s header. This also means one encoder covers a whole logical pass (backdrop-normal + world3d +
backdrop-overlay in one `MTLRenderCommandEncoder`) where the wgpu target used several, since its reason
for splitting (buffer-reuse ordering) doesn't apply here.

**Glass regions: one instanced draw, not one draw per region.** Glass has no stencil mask and every
instance shares one pipeline/state, so `instanceCount = glass_instances.len()` in a single draw call is
pixel-identical to the reference's per-region loop (GPUs rasterize/blend a draw call's instances in
submission order). One API round-trip instead of N.

**No per-mip `TextureView`s.** Metal's `sample(sampler, uv, level(lod))` takes an explicit LOD directly
against the whole mip chain, and `MTLRenderPassColorAttachmentDescriptor.level` picks a render-target
mip directly on the source texture — so `SceneTarget` never allocates the per-mip view array the wgpu
target's `SceneColorTarget` needs. Same pixels, fewer objects.

**`unsafe` blocks and their soundness** (every one in the crate; file:region → invariant):
- `🦀️resources.rs::create_texture`/`replace_region`/`new_buffer_with_bytes` — plain dimension/mip
  setters that Metal validates rather than reading OOB; `replaceRegion`/`newBufferWithBytes` pointers
  are valid Rust slice pointers for exactly the byte count passed, never retained past the call.
- `🦀️world3d.rs::WorldGlobalsRing::write_passes` — writes are bounds-checked against
  `ensure_slots`'s just-established capacity before this runs.
- `🦀️world3d.rs::encode_passes`/`draw_mesh` — buffer-offset binds and indexed draws whose
  counts/offsets come straight from `SurfacePass`/`SurfaceMeshDraw` data the caller (`Scene::finish`)
  already validated.
- `🦀️backend.rs::new` — the one `msg_send!` pair (`setWantsLayer:`/`setLayer:`) on the `NSView*`
  `raw_window_handle::AppKitWindowHandle` guarantees is live.
- `🦀️backend.rs::set_drawable_size` — the `CGSizeShim`/`msg_send!` workaround for not depending on
  `objc2-core-foundation` (see Registrar-requests); its `Encode` matches `CGSize`'s real `{CGSize=dd}`
  encoding field-for-field. **This is the single highest-risk unverified call in the crate** — I could
  not cross-check the ObjC runtime's exact acceptance of a locally-defined `Encode` impl standing in
  for the real `CGSize` type without compiling, only reason about it from the `objc2::Encode` trait's
  documented contract. If `sol`'s first build shows a `setDrawableSize:` failure or an `Encode`
  assertion panic, this is the first place to look — the fallback is the registrar-request below.
- `🦀️backend.rs::encode_2d_batches`/`draw_silhouette_mask`/`bind_ui_textures`/`run_blur_chain`/
  `blit_scene_to_drawable`/`composite_glass`/`capture_readback`/`read_back` — buffer/texture/sampler
  binds and draw calls whose ranges/offsets come from `RenderPacket` data `Scene::finish` already
  produced in-bounds, or from this backend's own just-allocated, exactly-sized buffers.

**Tests needing a live device**: all 5 in `🦀️backend.rs`'s `#[cfg(all(test, feature =
"backend-testing"))] mod tests` — `constructing_a_headless_backend_succeeds_or_skips_cleanly`,
`zero_size_resize_parks_and_restores`, `apply_resources_before_render_succeeds_and_an_unapplied_id_
errors_cleanly`, `forced_device_loss_reports_lost_and_recover_names_the_dead_generation`,
`read_back_reports_zero_size_cleanly_before_any_frame_is_presented`. Each constructs via
`MetalBackend::new_headless` (a `CAMetalLayer` not attached to any view — Metal drawables work
independent of window presence) and treats construction failure as the clean-skip signal, since
`MTLCreateSystemDefaultDevice` returning `None` is exactly the "no device" case a headless CI runner
can hit. **Unverified assumption**: that `nextDrawable()` on an unattached `CAMetalLayer` succeeds the
same way it would on a window-backed one — a widely-used technique for headless Metal testing, but not
something I could confirm without running the suite.

**Deviation — no `Result` propagation for pipeline/shader-compile/texture-allocation failure.**
`🦀️pipelines.rs::Pipelines::new` and the `allocate_*` helpers in `🦀️backend.rs`/`🦀️scene_target.rs`
panic (`.expect(...)`) on failure rather than threading a `Result` up through `MetalBackend::new`. The
wgpu reference has the same gap (`create_shader_module`/`create_texture` don't return `Result` either).
Given these failures only occur if the hand-written MSL itself has a bug or the device is truly out of
memory during setup — never from user/runtime data — I judged the panic acceptable for a first cut, but
it is a real deviation from "every construction path returns an error cleanly" and worth revisiting if
`sol` disagrees.

## Registrar-requests

Two independent asks — either can land without the other:

1. **The planned shader route** (`build.rs` + build-time `naga` MSL cross-compile, replacing
   `🦀️msl.rs`'s hand-written strings):
   ```toml
   [build-dependencies]
   naga = { version = "27", features = ["wgsl-in", "msl-out"] }
   ```
   plus a `build.rs` in this crate that iterates `ui_render::ALL_SHADERS`, runs naga's WGSL→MSL
   backend, and emits the result via `include_str!`/`OUT_DIR` for `🦀️pipelines.rs` to consume instead
   of the `_MSL` constants in `🦀️msl.rs`.
2. **Replacing the `CGSizeShim`/`msg_send!` workaround** in `🦀️backend.rs::set_drawable_size` with the
   typed `CAMetalLayer::setDrawableSize` call:
   ```toml
   objc2-core-foundation = "0.3"
   ```
   (already a transitive dependency via `objc2-quartz-core`'s `CAMetalLayer` feature — this just makes
   it nameable from this crate). This is the fix for this report's single highest-risk unsafe call.

## Deviations (summary)

- World3d is not interleaved with 2D content layer-by-layer — see "Known limitation" above (upstream
  contract gap, not fixable from this crate).
- `WORLD3D_TEXTURED_PIPELINE` not implemented — dead/unwired surface in the reference itself.
- Construction/allocation failures panic rather than returning `Result` — see "Deviations" in
  Decisions.
- Shader route is interim hand-written MSL, not the planned `naga` build-time cross-compile — needs
  the `build.rs`/`[build-dependencies]` registrar-request above.
- `setDrawableSize:` goes through a hand-rolled `CGSizeShim` + `msg_send!` rather than the typed
  `objc2_quartz_core` call, because `objc2-core-foundation` isn't a declared dependency — see
  registrar-request 2.

## Files touched

Created (all new, under `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/`):
`🦀️msl.rs`, `🦀️types.rs`, `🦀️pipelines.rs`, `🦀️resources.rs`, `🦀️scene_target.rs`,
`🦀️frame_buffers.rs`, `🦀️world3d.rs`, `🦀️backend.rs`.

Rewritten: `📦️glue.rs` (was the packet's scaffold placeholder).

Not touched: `Cargo.toml` (per OWNS — dependency needs listed under Registrar-requests instead),
everything outside this crate.

## cgsize-correction

Both reported errors (`E0599: no associated function or constant named 'ENCODING' found for struct
'CGSizeShim'` and `E0308: mismatched types`) were the same call — `set_drawable_size`'s `msg_send!
[layer, setDrawableSize: size]`. Root cause of the `E0599`, reasoned from source rather than compiled:
`CGSizeShim` did get a real `unsafe impl objc2::Encode for CGSizeShim { const ENCODING = ...; }`, but
`🦀️backend.rs` never had `use objc2::Encode;`/`use objc2::RefEncode;` in scope — the `msg_send!` macro's
generated code resolves `<T as Encode>::ENCODING`-shaped associated-const lookups against whatever
`Encode`/`RefEncode` names are in lexical scope at the invocation site, not merely "implemented
somewhere in the crate", and I only ever referenced `objc2::Encode` fully-qualified inside the `impl`
line itself. The `E0308` was very likely the macro's consequent fallback/placeholder type once that
resolution failed, not an independent bug.

Deleted `CGSizeShim` and the `msg_send!` call outright — no shim needed. Replaced with the real typed
API, confirmed against the now-declared `objc2-core-foundation = "0.3"`:

- `objc2_core_foundation::CGSize` — `objc2-core-foundation-0.3.2/src/geometry.rs:112-119`:
  `pub struct CGSize { pub width: CGFloat, pub height: CGFloat }` (`#[repr(C)]`), with
  `CGFloat = f64` on 64-bit targets (`geometry.rs:6-8,18`) and `unsafe impl Encode`/`RefEncode`
  provided at `geometry.rs:120-127`, gated behind the crate's `"objc2"` feature — confirmed present in
  `objc2-core-foundation`'s own `default = [...]` list (`Cargo.toml`, `"objc2"` entry), so it is on
  without any extra feature flag on our side.
- `CGSize::new(width: CGFloat, height: CGFloat) -> Self` — `geometry.rs:150-153`, `pub const fn`, safe.
- `CAMetalLayer::setDrawableSize` — `objc2-quartz-core-0.3.2/src/generated/CAMetalLayer.rs:99-107`:
  `pub fn setDrawableSize(&self, drawable_size: CGSize);`, no `unsafe` marker, gated
  `#[cfg(feature = "objc2-core-foundation")]` — confirmed that feature is in `objc2-quartz-core`'s own
  `default = [...]` list (`Cargo.toml` line 188) alongside `"CAMetalLayer"` (also default), so our
  `objc2-quartz-core = { version = "0.3", features = ["CAMetalLayer"] }` entry (which does not disable
  default features) compiles the method in.

New body, no `unsafe` block at all:

```rust
fn set_drawable_size(layer: &CAMetalLayer, width: u32, height: u32) {
    layer.setDrawableSize(objc2_core_foundation::CGSize::new(width as f64, height as f64));
}
```

Also updated the module-wide `msg_send!` safety-note comment in `🦀️backend.rs` (it previously described
a `CGSize`-shaped call among its examples) to reflect that only the two `NSView` calls
(`setWantsLayer:`/`setLayer:` in `MetalBackend::new`) still go through `msg_send!`.

This closes registrar-request 2 from this report's original "Registrar-requests" section — the
`objc2-core-foundation` addition is exactly what it asked for. Registrar-request 1 (the planned
`naga`/`build.rs` shader route) is unaffected and still open.

Re-checked brace/paren/bracket balance and `//#region`/`//#endregion` balance on `🦀️backend.rs` after
the edit: both still equal (7/7 regions, all bracket kinds balanced). Acceptance remains UNRUN per U4.
