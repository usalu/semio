# 📓️ terra-backend-webgpu-report

Packet `backend-webgpu`, wave W3. Anchor commit `cb9bcce7a4`.

## Done

`WebGpuBackend` — a concrete `wgpu 27.0.1` implementation of `ui_render::GraphicsBackend` — in
`🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/`, 2,526 lines across 12
files (11 new region files + rewritten `📦️glue.rs`):

- `🦀️surface_state.rs` — `SurfaceState` (park/unpark), `DeviceHealth`, and `ScenePhase`/
  `classify_batch_phase` (which of the four replay passes a `DrawBatch` belongs to). Pure, no `wgpu`.
- `🦀️gpu_types.rs` — pure `shader_contract` → `wgpu` enum/struct translations (`VertexFormat`,
  `BlendMode`, `CompareFunction`, `StencilOperation`, `DepthStencilSpec`, `BindingKind`, …).
- `🦀️gpu_uniforms.rs` — byte-exact `UiGlobals`/`BlurGlobals`/`World3dGlobals`/`World3dGpuInstance`/
  `WorldLineGpuVertex` (the WGSL uniform/instance shapes `shader_contract.rs` deliberately doesn't
  mirror in Rust).
- `🦀️pipelines.rs` — `Pipelines`: every render pipeline built generically from `ui_render::
  PipelineSpec` + `ui_render::ALL_SHADERS`'s WGSL via `crate::gpu_types`, plus the five hand-built
  bind group layouts (`ui_globals`, `world_globals`, `blur`, `scene_sample`, glass reuses two of them).
- `🦀️resources.rs` — `GpuResources`: `apply_resources`'s whole implementation. Typed `HashMap
  <TextureId, _>`/`HashMap<MeshId, _>` tables (no `String` key, no per-frame clone), plus the two
  fixed glyph/icon atlas slots the canonical WGSL bakes in.
- `🦀️scene_target.rs` — `SceneColorTarget`: the 5-level mip chain + blur-scratch offscreen target.
- `🦀️buffers.rs` — `GrowBuffer` (doubling-capacity upload buffer), `WorldGlobalsRing` (dynamic-offset
  per-`SurfacePass` uniform ring), `FrameBuffers`.
- `🦀️frame.rs` — `render()`: buckets `RenderPacket::batches` by `ScenePhase`, replays each `DrawBatch`
  verbatim, gathers/replays `SurfacePass` 3D content, runs the blur chain, blits + composites glass,
  renders foreground content/overlay onto the swapchain.
- `🦀️gpu_context.rs` — `GpuContext::new` (the async instance→surface→adapter→device chain),
  format/capability translation, shared surface-configure/depth-texture helpers.
- `🦀️backend.rs` — `WebGpuBackend` itself: the `GraphicsBackend` impl, device-lost callback wiring,
  `backend-testing`'s `debug_force_device_loss`/`read_back`.
- `📦️glue.rs` — mounts all of the above; the `compile_error!` browser-only guard is untouched.

64 `#[cfg(test)]` unit tests across the pure-logic files (surface state, phase classification, gpu-type
translation, uniform byte layouts, mip-extent math, buffer growth math, atlas-upload classification,
batch bucketing, world-pass gathering, surface-format picking, loss-flag decode, padded-row alignment).

## Acceptance: UNRUN

Per U4, I ran no cargo command. Commands for `sol`, `timeout: 600000`, `CARGO_TARGET_DIR` in the
session scratchpad:

```
cargo check -p semio-framework-ui-backend-webgpu --target wasm32-unknown-unknown
cargo check -p semio-framework-ui-backend-webgpu --target wasm32-unknown-unknown --all-targets
cargo check -p semio-framework-ui-backend-webgpu --target wasm32-unknown-unknown --features backend-testing
cargo tree -p semio-framework-ui-backend-webgpu --invert wgpu --target wasm32-unknown-unknown
```

`cargo test` cannot run for this crate on the host — it only compiles for `wasm32-unknown-unknown`
(U-program rule, this packet's own brief). The `#[cfg(test)]` modules are real and will run once a
`wasm32-unknown-unknown` test runner (or a real browser) is available; until then their coverage is
unexercised, not just unverified.

**Cheap non-cargo checks performed instead, both green:**
- Brace/paren/bracket balance, comment- and lifetime-aware, over every file: all zero.
- `//#region`/`//#endregion` balance per file: all equal (see table below).

| file | region/endregion |
|---|---|
| `🦀️backend.rs` | 5/5 |
| `🦀️buffers.rs` | 5/5 |
| `🦀️frame.rs` | 9/9 |
| `🦀️gpu_context.rs` | 5/5 |
| `🦀️gpu_types.rs` | 2/2 |
| `🦀️gpu_uniforms.rs` | 2/2 |
| `🦀️pipelines.rs` | 3/3 |
| `🦀️resources.rs` | 6/6 |
| `🦀️scene_target.rs` | 2/2 |
| `🦀️surface_state.rs` | 5/5 |
| `📦️glue.rs` | 1/1 |

I also read the vendored `wgpu 27.0.1`/`wgpu-types 27.0.1` source (`~/.cargo/registry/src/index.crates
.io-*/wgpu{,-types}-27.0.1/src/`) for every non-obvious API surface before using it — `SurfaceTarget::
Canvas`, `Instance::create_surface`/`request_adapter`/`request_device` sync-vs-async split, `Surface
Error` variants, `Device::set_device_lost_callback`'s **unconditional** `Send` bound (not `WasmNotSend`
— real constraint, see Decisions), `BufferSlice::map_async`'s `WasmNotSend` bound, `BufferView: Deref
<Target=[u8]>`, `ShaderStages::VERTEX_FRAGMENT`, `ExperimentalFeatures::disabled()`,
`TextureFormat::{is_srgb,add_srgb_suffix}`, `COPY_BYTES_PER_ROW_ALIGNMENT` — rather than from recall,
per U4's explicit instruction (a sibling packet already lost a cycle guessing at an API).

## Decisions

1. **Pipelines built generically from `PipelineSpec`, bind group layouts hand-built.** The ticket asks
   for the former explicitly ("so the shader source is shared... rather than duplicated here").
   `crate::pipelines::build_pipeline` reads any `&PipelineSpec` + `&ShaderModule` + `&PipelineLayout`
   and produces a `wgpu::RenderPipeline` — no per-pipeline hand-copy. The five `wgpu::BindGroupLayout`s
   stay hand-built (`Pipelines::new`) rather than derived per-`PipelineSpec`, because a `wgpu::
   BindGroup` must be created against the *same* layout object its pipeline's `PipelineLayout` used —
   building a fresh layout per spec would risk producing structurally-identical-but-distinct layout
   objects for the same semantic bind group (`UI_GLOBALS_BIND_GROUP` backs `UI_MASK_PIPELINE`,
   `UI_CONTENT_PIPELINE`, `VECTOR_PIPELINE` and glass's group 0 — one object, four consumers).
2. **`DrawBatch` replay is uniform across all four pipeline kinds it ever carries** (`UiQuad`/
   `UiRasterTextured`/`Vector`/`Glass`) — one `replay_batch` function, mask draw + pipeline/bind-group/
   vertex-buffer selection purely off `batch.pipeline`/`batch.texture`. `Scene::finish`'s `batch()`
   already appends one `DrawBatch` **per glass instance** (`instance_range: (index, 1)`), not one batch
   covering the whole `glass_instances` array the way `draw.rs`'s `composite_glass_regions` issued a
   single instanced draw — I replay them exactly as given (N small draws, same pass) rather than
   re-deriving `draw.rs`'s single-instanced-draw shape, per the contract's "replay batches verbatim,
   make no batching decision" invariant.
3. **Typed ids bought**: no `String` key anywhere in this crate's resource tables (`HashMap<TextureId,
   _>`/`HashMap<MeshId, _>` directly), no per-instance `String` clone in the raster/mesh hot path
   (`resource.rs`'s own stated motivation), and a stale id is now a `None` lookup + a clean
   `BackendError::UnknownResource`, never a string comparison against a possibly-stale key.
4. **Atlas upload routed by pixel-byte-length, not arrival order.** `ResourceOp::UploadAtlas` carries
   no channel/format tag, but the canonical WGSL bakes in exactly two fixed texture slots (`glyph_atlas`
   R8Unorm, `icon_atlas` Rgba8UnormSrgb — bindings 1/2 and 3/4 of `UI_GLOBALS_BIND_GROUP`). `pixels.len()
   == width*height` routes to the glyph slot, `== width*height*4` to the icon slot — deterministic and
   order-independent, unlike tracking "first/second atlas id seen." See registrar-requests #2.
5. **3D `SurfacePass` content renders as one block, not interleaved at `quad_watermark`/
   `vector_watermark`.** `draw.rs` interleaves each `SurfacePass` between its originating 2D layer's
   watermark offsets via `render_interleaved_layers`. `RenderPacket` doesn't expose `Scene::finish`'s
   internal `ordered_layers` (by design — `render-scene`'s own report explains why), so a backend has
   no way to recover *which* `DrawBatch` a given `SurfacePass.layer_index` used to sit inside, or where
   in its instance range the watermark split falls. All `surface_passes` now render as one dedicated
   block right after the backdrop's 2D content, each still scoped to its own `viewport` rect via a
   synthesized two-quad silhouette mask (reset-to-full-screen at ref 0, then the pass's own viewport at
   ref 1) rather than inheriting whatever stencil state the last 2D batch happened to leave — the best
   achievable correctness given the information `RenderPacket` currently carries. See registrar-requests
   #1 for the fix that would let a future revision restore exact interleaving.
6. **Pass/attachment structure**: every render pass Clears stencil to 0 at its start (`draw.rs` does
   this for *every* sub-pass, including ones that Load color/depth — confirmed by re-reading its five
   `depth_stencil_attachment` call sites; only the very first pass also Clears depth to 1.0, every later
   one Loads depth) — because each of the four `ScenePhase` streams runs its own independent stencil
   mask chain (`Scene::finish::batch`'s `previous_bounds` resets to `None` at each stream boundary), and
   a stale ref=1 region from a *different* stream must not leak into the next one's mask math.
7. **`WORLD3D_TEXTURED_PIPELINE`/`SurfaceTexturedDraw` not wired into `render()`.** `shader_contract.rs`
   already documents this pipeline as "inferred, not wired in `draw.rs`" (dead code upstream, confirmed
   by that packet's own grep). Building it would need a per-texture bind group shape `ResourceOp` has no
   equivalent for (no texture-coordinate channel in `CreateOrUpdateMesh`), so it stays unbuilt and
   `SurfaceTexturedDraw` content is silently skipped, matching the reference implementation's own status
   quo rather than inventing a rendering path nothing upstream exercises.
8. **Fixed a real bug found while porting, not present in `draw.rs`**: my first draft fed `RenderPacket::
   viewport` (documented — `backend.rs`'s own `PhysicalSize` docstring — as *logical*-pixel) straight
   into the screen-size uniform / scissor / viewport math, while every rect inside `quad_instances`/
   `vector_vertices`/`glass_instances` is already dpr-snapped to *physical* pixels by `Scene::finish`'s
   `snap()` step. At dpr ≠ 1.0 this would have rendered content into roughly `1/dpr` of the surface.
   Fixed: `crate::frame::render` now takes explicit `physical_width`/`physical_height` from the backend's
   own tracked `SurfaceState` (set via `resize`), never derived from the packet.

## What is unverified without a browser

- **Every `wgpu` call site's exact argument shape.** I cross-checked struct/enum literals against the
  vendored 27.0.1 source, but a device build (`create_render_pipeline`, bind group layout compatibility,
  shader compilation) can only be validated by an actual adapter/device — none of this ran.
- **`recover()`'s real-world usefulness.** Documented directly on the method: a genuine WebGPU device
  loss leaves the old `wgpu::Device` permanently dead, and rebuilding one needs `request_adapter`/
  `request_device` again — a real async round-trip `recover()` (a plain sync `fn` per the trait) cannot
  perform. It resets bookkeeping and reports the ids that were resident, matching `NullBackend::
  recover`'s own shape, but a caller still needs a fresh `WebGpuBackend::new` for rendering to resume.
- **`debug_force_device_loss`/`read_back`'s round trip.** `read_back` depends on `map_async`'s callback
  firing between two separate calls into this crate (a real microtask/animation-frame boundary) — this
  can only be exercised by an actual conformance-harness run in a browser, never by a synchronous test.
- **The atlas byte-length classification heuristic** (decision #4) — untested against a real glyph/icon
  atlas upload from the text/raster pipeline upstream of this crate.
- **3D `SurfacePass` visual correctness** under decision #5's simplification — clipping to each pass's
  own `viewport` is a reasoned approximation, not a proven match against `draw.rs`'s exact pixel output.
- **Device capability values** (`DeviceCapabilities::gpu_tier`/`memory_class` thresholds in
  `gpu_context.rs`) — plausible but arbitrary buckets; no real adapter was queried to calibrate them.

## registrar-requests

1. **`ui_render::resource::ResourceOp::UploadAtlas` should carry an explicit channel/format tag**
   (e.g. `channels: u8` or a small `AtlasFormat` enum) instead of leaving a backend to infer R8Unorm-vs-
   Rgba8UnormSrgb from `pixels.len()`. Works today (decision #4) but is a landmine for a future atlas
   kind whose byte-per-pixel count collides with an existing one.
2. **`ui_render::scene::SurfacePass` should carry enough to let a backend replay it in the correct
   `DrawBatch` position** — either a `foreground_of: Option<usize>`/`overlay: bool` pair mirroring
   `LayerState`'s, or (cleaner) a sentinel `DrawBatch` entry (e.g. a `PipelineKind::World3dMesh` batch
   whose `instance_range` indexes into `surface_passes` instead of `quad_instances`) marking exactly
   where in `batches` order a pass belongs. Today's `RenderPacket` makes decision #5's simplification
   the best any backend can do without this.
3. **This crate's `Cargo.toml` needs, at minimum:**
   ```toml
   [dependencies]
   bytemuck = "1.24.0"

   [features]
   backend-testing = ["ui_render/backend-testing"]
   ```
   `bytemuck` is used throughout (`pipelines.rs`, `buffers.rs`, `gpu_uniforms.rs`, `resources.rs`) for
   `cast_slice`/`bytes_of` over `ui_render`'s `Pod`/`Zeroable` GPU types — Cargo dependencies are not
   transitive for `use` purposes, so depending on `ui_render` (which depends on `bytemuck` itself) does
   not make `bytemuck::` resolvable here. The `backend-testing` feature is required for `#[cfg(feature =
   "backend-testing")]` in this crate (gating `debug_force_device_loss`/`read_back`, required by `ui_render
   ::GraphicsBackend`'s own same-named feature-gated methods) to exist and stay in lockstep with `ui_render`'s
   own feature — without it, `impl GraphicsBackend for WebGpuBackend` is missing required trait methods
   whenever `ui_render`'s `backend-testing` feature is unified on by anything else in the same build.
4. **`web-sys` feature list** (`Window`, `HtmlCanvasElement`, already present) looks sufficient for this
   packet's scope (canvas width/height read directly off the passed `HtmlCanvasElement`, dpr supplied by
   the caller through `resize`, never queried from `window()`) — flagging only because I could not
   compile-verify it; no change requested unless `cargo check` disagrees.

## Deviations

- **No separate raster render pass.** `draw.rs` ran UI/vector content and raster-textured quads as two
  passes with two independent stencil-mask chains (`render_interleaved_layers` then `draw_raster_layers`).
  `Scene::finish::batch` already interleaves raster-texture-run `DrawBatch`es into the *same* per-layer
  mask chain as the quad/vector batches for that layer (confirmed by re-reading `scene.rs`'s `batch()`);
  replaying them in one pass with one mask chain is both correct per the new contract and simpler than
  reproducing the old two-pass split.
- **`FrameStats` timing fields are always `0.0`.** `wgpu::Features::TIMESTAMP_QUERY` was not requested
  (`DeviceCapabilities::supports_timestamp_queries` correctly reports `false`), so encode/submit/present
  durations are not measured — matches the trait's own allowance (`FrameStats` fields are per-frame
  counts/timings a backend *may* report; nothing requires nonzero timings).

## Files touched

Created (all under `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/`):
`🦀️surface_state.rs`, `🦀️gpu_types.rs`, `🦀️gpu_uniforms.rs`, `🦀️pipelines.rs`, `🦀️resources.rs`,
`🦀️scene_target.rs`, `🦀️buffers.rs`, `🦀️frame.rs`, `🦀️gpu_context.rs`, `🦀️backend.rs`.

Rewritten: `📦️glue.rs` (kept the `compile_error!` guard verbatim, added the module mounts + `pub use
backend::WebGpuBackend`).

Not touched: `Cargo.toml` (registrar-owned, see registrar-requests #3), everything outside this crate's
own directory.
