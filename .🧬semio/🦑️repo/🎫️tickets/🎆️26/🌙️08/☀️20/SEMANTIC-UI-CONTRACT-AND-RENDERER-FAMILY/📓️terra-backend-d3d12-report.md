# 📓️ terra-backend-d3d12-report

Packet `backend-d3d12`, wave W3.

## Done — reached milestone 6 (all six), with documented known limitations and one sizing/lifetime bug caught and fixed in review

`D3d12Backend` — a concrete hand-written Direct3D 12 implementation of `ui_render::GraphicsBackend` —
in `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/`, ~3,294 lines across 9
files (8 new region files + rewritten `📦️glue.rs`), structured 1:1 against the Metal backend
(`backend-metal`, ~2,668 lines/9 files) with the divergences below called out.

- `🦀️types.rs` — GPU-layout structs (`WorldGlobalsGpu`, `World3dGpuInstance`, `World3dGpuVertex`,
  `WorldLineGpuVertex`, `BlurMipGpu`, `WORLD_GLOBALS_SLOT_SIZE = 256`, `UNIT_QUAD_CORNERS`) plus, unlike
  Metal, the shared low-level D3D12 plumbing every other file needs: `transition_barrier` (the one
  `D3D12_RESOURCE_BARRIER` constructor in the crate), `wait_for_fence_value` (a polling fence wait —
  see Decisions), `DEFAULT_HEAP`/`UPLOAD_HEAP`/`READBACK_HEAP` heap-properties constants,
  `buffer_desc`/`texture2d_desc`/`create_upload_buffer`/`create_default_texture2d` resource-creation
  helpers.
- `🦀️hlsl.rs` — hand-written HLSL (Shader Model 5.0) for all five shader families, transcribed
  line-for-line from the canonical WGSL in `ui_render::shader_contract`, mirroring `🦀️msl.rs`'s
  structure and its "interim, not the planned cross-compile" framing.
- `🦀️pipelines.rs` — the one root signature every pipeline shares (1 root CBV + 2 descriptor tables —
  see Decisions for the exact wgpu-binding mapping), `SamplerHeap` (4 fixed sampler descriptors, built
  once), 9 `ID3D12PipelineState`s. Depth/stencil and rasterizer state (cull mode, depth bias) are baked
  into each PSO directly — D3D12 needs no Metal-style separate depth-stencil-state object or
  encoder-time cull/bias push-pop (see Decisions, "genuinely simpler than Metal").
- `🦀️resources.rs` — `GpuResources`: `ResourceOp` application, a growable `ResidentSrvTable` (CPU-only
  descriptor heap, glyph@0/icon@1 fixed, raster textures free-listed from 2), glyph/icon atlas routing
  by upload byte density (same inference as Metal — the contract has no `AtlasKind`), synchronous
  texture upload via a dedicated small command allocator/list/fence. **Caught and fixed in review**: an
  initial draft dropped each upload's staging buffer at the end of the function that created it, before
  the command list was ever executed — see Decisions/"the staging-buffer lifetime bug" for the full
  story; fixed via a `pending_staging: Vec<ID3D12Resource>` field cleared only after the fence confirms
  GPU completion.
- `🦀️scene_target.rs` — `SceneTarget`: the offscreen scene-color target + blur-scratch texture (5-mip
  chains), an RTV heap (one descriptor per mip) and a small resident SRV heap for both textures'
  full-chain SRVs. `ensure` now returns whether it actually recreated the textures, so `🦀️backend.rs`
  knows whether to reset its per-mip resource-state tracking.
- `🦀️frame_buffers.rs` — `GrowBuffer` (D3D12 upload-heap analog of Metal's `Shared`-storage
  `GrowBuffer`), `FrameBuffers` (same one-flat-array-per-kind simplification Metal documents, plus two
  more `GrowBuffer`s — `world_globals`/`blur_globals` — since D3D12's root CBV needs no dedicated ring
  type; see `🦀️world3d.rs`'s header), and `FrameDescriptors` — the per-frame shader-visible SRV bump
  allocator this backend needs that Metal's per-draw argument-table binding never did (see Decisions).
- `🦀️world3d.rs` — `upload_world_passes`/`encode_passes`: packs every pass's globals into the
  256-byte-strided root-CBV ring, uploads instances/lines, replays opaque/translucent mesh + line
  draws. No separate ring-buffer type (unlike Metal's `WorldGlobalsRing`) and no encoder-side depth-bias/
  cull-mode push-pop (unlike Metal's `encode_passes`) — both are D3D12-shape simplifications, explained
  in the file's own header. Same documented interleaving limitation as Metal (below).
- `🦀️backend.rs` — `D3d12Backend` itself: device/factory/direct-queue/flip-model-swapchain construction
  (real `HWND` + headless `CreateSwapChainForComposition`), resize (zero-size park/restore), the
  two-pass `render()` (offscreen scene pass → blur/blit/glass/foreground composite pass), explicit
  per-mip resource-state tracking (D3D12 has no render-pass load/store model — see Decisions), the
  `GraphicsBackend` impl, `backend-testing`'s `debug_force_device_loss`/`recover`/`read_back`. 5
  `#[cfg(test)]` tests, all device-gated, structurally identical to Metal's.
- `📦️glue.rs` — mounts all of the above; the `compile_error!` Windows-only guard kept from the scaffold.

### Milestones, against the ticket's six

1. **Device + DXGI flip-model swapchain + direct queue + per-frame fence** — done.
   `D3D12CreateDevice`(default adapter)/`CreateDXGIFactory2`, a `D3D12_COMMAND_LIST_TYPE_DIRECT` queue,
   a 3-buffer `DXGI_SWAP_EFFECT_FLIP_DISCARD` swapchain via `CreateSwapChainForHwnd`, `resize`
   (zero-size parks: `ResizeBuffers` is skipped entirely rather than passed zeros — see Decisions), a
   clear-colour frame, `Present`.
2. **UI quads + vector** — done. The SDF megashader (all 9 `kind` branches transcribed), vector
   triangles, glyph/icon atlas + per-texture raster upload, per-batch scissor is a no-op by design
   (same reasoning as Metal — clipping is entirely stencil-driven).
3. **Stencil silhouette clip** — done. `ui_mask`/`ui_content` PSOs' baked `D3D12_DEPTH_STENCIL_DESC`
   mirror the contract's mask/content stencil specs exactly; `draw_silhouette_mask` replays `mask_range`
   verbatim (reset-bounds quad at ref 0, then every "piece" quad at ref 1).
4. **Offscreen scene target → mip blur → glass composite → blit** — done. `SCENE_MIP_LEVELS = 5`,
   copy-then-downsample per level via explicit per-mip barrier tracking, `scene_blit_pipeline` to the
   back buffer, glass composited as one instanced draw (same simplification Metal documents).
5. **World3d** — done for mesh (opaque + translucent) and lines, with a real depth buffer
   (`DXGI_FORMAT_D24_UNORM_S8_UINT`) and the 256-byte-aligned root-CBV ring. **Not interleaved with 2D
   content layer-by-layer** — identical upstream contract gap to the one Metal's report documents (see
   below); not re-derived from scratch here, just confirmed still true by re-reading the same
   `ui_render::scene::DrawBatch`/`LayerState`/`RenderPacket` field lists Metal's report cites.
6. **Device loss + `backend-testing`** — done, and **genuinely stronger than Metal here**: D3D12
   exposes real device-removal detection (`ID3D12Device::GetDeviceRemovedReason`), so `device_status()`
   checks it for real rather than being purely simulated. `recover()`/`debug_force_device_loss` still
   need the `backend-testing`-only simulated path for testability, and `recover()`'s honest limits
   against a *real* device-removed event are documented in its own doc comment and below.
   `read_back()` copies the just-presented back buffer into a `READBACK`-heap buffer and `Map`s it.

## Known limitation — world3d is not interleaved with 2D content layer-by-layer

Identical finding to the Metal backend's report, not re-derived: `ui_render::scene::DrawBatch` — the
only per-batch data a `GraphicsBackend` receives — carries no `layer_index`/watermark a backend could
use to interleave a `SurfacePass`'s draws between the 2D content immediately before/after it within one
scissor layer (`Scene::finish`'s `order()` step remaps `SurfacePass::layer_index` into an internal,
unexposed merged layer list). This is a gap in `ui_render::scene` (packet `render-scene`), not in this
crate's `OWNS` list. `🦀️world3d.rs::encode_passes` renders every `SurfacePass` for a frame as one group,
after backdrop-normal 2D content and before backdrop-overlay 2D content — the same place Metal put it,
for the same reason. Flagging again for `sol`: both hand-written backends now independently hit this
same wall, which is a stronger signal that the upstream fix (either a `layer_index`/watermark field on
`DrawBatch`, or replaying `SurfacePass` draws inline as their own ordered entry in `Scene::finish`'s
output) is worth prioritizing rather than left backend-by-backend.

## Acceptance: UNRUN

Per U4, I ran no cargo command — this machine is macOS. Commands for `sol`, `timeout: 600000`,
`CARGO_TARGET_DIR` in the session scratchpad, target `x86_64-pc-windows-msvc`:

```
cargo check -p semio-framework-ui-backend-d3d12 --target x86_64-pc-windows-msvc
cargo check -p semio-framework-ui-backend-d3d12 --target x86_64-pc-windows-msvc --all-targets
cargo check -p semio-framework-ui-backend-d3d12 --target x86_64-pc-windows-msvc --features backend-testing
cargo test -p semio-framework-ui-backend-d3d12 --target x86_64-pc-windows-msvc --features backend-testing
cargo tree -p semio-framework-ui-backend-d3d12 --invert wgpu --target x86_64-pc-windows-msvc
```

**Cheap non-cargo checks performed instead, both green:**
- Brace/paren/bracket balance over every file (including the HLSL string literals): all equal.
- `//#region`/`//#endregion` balance per file: all equal.

| file | region/endregion |
|---|---|
| `🦀️types.rs` | 7/7 |
| `🦀️hlsl.rs` | 6/6 |
| `🦀️pipelines.rs` | 9/9 |
| `🦀️resources.rs` | 5/5 |
| `🦀️scene_target.rs` | 1/1 |
| `🦀️frame_buffers.rs` | 4/4 |
| `🦀️world3d.rs` | 1/1 |
| `🦀️backend.rs` | 7/7 |

**Every non-obvious `windows`-crate API used here was checked against the vendored `windows-0.62.2`
source**, fetched read-only from `static.crates.io` into the session scratchpad
(`/private/tmp/.../scratchpad/windows-0.62.2/`, `windows-core-0.62.2/`, `windows-strings-0.5.1/`, none
touching this crate's `Cargo.lock`/workspace) because this machine has no vendored copy of the real
`windows` crate under `~/.cargo/registry` — only its small support crates (`windows-targets`,
`windows-implement`, `windows-strings`, `windows-result`) were present, not the ~1MB-per-module
generated bindings themselves. I read the actual generated source for: `D3D12CreateDevice`,
`D3D12SerializeRootSignature`, `D3DCompile`, every `ID3D12Device`/`ID3D12GraphicsCommandList`/
`ID3D12CommandQueue`/`ID3D12Fence`/`ID3D12Resource`/`ID3D12DescriptorHeap` method I call (exact
`unsafe`/safe marker, exact parameter `Option<...>`/`*const`/reference shape, exact generic-`T`-from-
return-type-inference pattern), every struct/union field name and layout (`D3D12_RESOURCE_BARRIER`'s
`ManuallyDrop<Option<ID3D12Resource>>` union member, `D3D12_TEXTURE_COPY_LOCATION`'s union, root-
parameter/descriptor-range/root-signature-desc shapes, `D3D12_GRAPHICS_PIPELINE_STATE_DESC`'s full
field list), every enum's numeric layout via its `pub const` list, `windows_core::Param<T>`'s actual
blanket impls (`param.rs`, confirming `Option<&T>: Param<T>` for interface args and the `CopyType`
blanket for value types like `PCSTR`), `IDXGIFactory2::CreateSwapChainForComposition` (confirming a
window-free swapchain path exists at all — this is what let `new_headless` avoid needing
`Win32_UI_WindowsAndMessaging`), and `HRESULT::is_err`. `CreateEventW`'s `#[cfg(feature =
"Win32_Security")]` gate (not declared in this crate's `Cargo.toml`) is what drove the polling
`wait_for_fence_value` decision below — found by checking, not assumed.

## Decisions

**Root signature mapping — the wgpu reference's 5-entry UI bind group maps to 1 root CBV + 2
descriptor tables, exactly as the ticket specified**: `@binding(0)` (uniform) → root param 0, a root
CBV at `b0` (no descriptor object needed — every family shares this one slot, bound to a different GPU
virtual address per draw, mirroring how Metal reuses its `buffer(2)` slot across families).
`@binding(1)`/`@binding(3)` (glyph/icon textures) → root param 1, one `DESCRIPTOR_TABLE` with a
2-descriptor SRV range at `t0..t1`. `@binding(2)`/`@binding(4)` (glyph/icon samplers) → root param 2,
one `DESCRIPTOR_TABLE` with a 2-descriptor Sampler range at `s0..s1`. These have to be *two* tables,
not one four-entry one, because D3D12 forbids mixing SRV and Sampler ranges within a single descriptor
table (they live in different heap *types*) — confirmed by reasoning from the vendored
`D3D12_DESCRIPTOR_RANGE_TYPE`/heap-type enums, not assumed. Full detail and the "how each wgpu binding
landed" table live in `🦀️pipelines.rs`'s header.

**Shader strategy — interim hand-written HLSL compiled at runtime via `D3DCompile`, not the planned
build-time `naga` cross-compile.** Unlike Metal (whose interim MSL path needed no extra dependency —
`newLibraryWithSource` compiles at construction time with zero build-system involvement), this crate's
*interim* route also needs no `Cargo.toml` change: `D3DCompile` lives behind the already-declared
`Win32_Graphics_Direct3D_Fxc` feature. Only the *eventual* `naga`/`build.rs` swap (WGSL→HLSL at build
time, `d3dcompiler_47.dll` still doing the HLSL→bytecode step at runtime per the ticket's "zero-touch,
no DXC install" requirement) needs a registrar request, listed below.

**Frame descriptor bump allocator — the one thing this backend needs that Metal's shape never did.**
D3D12 command lists only *record* work; nothing executes until `ExecuteCommandLists`. Rewriting the
*same* shader-visible descriptor slot for every draw needing a different texture (mirroring Metal's
per-draw `setFragmentTexture_atIndex`) would mean every draw's GPU read sees whichever texture was
written *last* at CPU record time, not the one that draw was meant to use. `🦀️frame_buffers.rs::
FrameDescriptors` fixes this with a per-frame bump allocator: every draw needing its own texture pair
gets a fresh, never-reused-this-frame pair of slots. Sized once per `render()` call from
`packet.batches.len() + 8` (8 covers the fixed non-batch-driven allocations — see the inline comment at
the call site) so no growth is ever needed mid-recording — growing a *shader-visible* heap mid-list is
legal in D3D12 (re-`SetDescriptorHeaps` partway through works, as long as the old heap outlives every
draw already recorded against it) but adds bookkeeping this backend's exactly-knowable per-frame demand
makes unnecessary. Full reasoning in that file's header.

**No per-batch hardware scissor rect** — same as Metal, confirmed by the same re-read of the wgpu
reference's `render_interleaved_layers`/`draw_ui_instances`: hardware scissor is only ever reset to the
full viewport, never narrowed per batch; all clipping is stencil-driven.

**Depth/stencil, cull mode and depth bias are baked into each PSO — genuinely simpler than Metal here,
not harder.** In Metal, only shader functions/vertex layout/blend live in the pipeline state object;
depth-stencil is a separate bound object, and cull mode/depth bias are pure encoder state requiring
push/pop around specific draws (`world3d.rs::encode_passes`'s `setCullMode`/`setDepthBias` calls). D3D12
folds ALL of that (`DepthStencilState`, `RasterizerState.CullMode`/`DepthBias`/`SlopeScaledDepthBias`)
into one immutable `D3D12_GRAPHICS_PIPELINE_STATE_DESC` per PSO — so `world3d_translucent`'s bias
(`-2`/`-1.0`) and back-face culling are simply always-on facts about that PSO, and `🦀️world3d.rs::
encode_passes` never touches either at encode time. This is the one place this backend's shape
diverges from Metal's by being *simpler*, called out explicitly per the ticket's ask.

**Explicit per-subresource resource-state tracking — D3D12 has no render-pass load/store model.**
Metal's render-pass descriptor (`MTLLoadAction::{Clear,Load,DontCare}`) implicitly handles what D3D12
requires explicit `D3D12_RESOURCE_BARRIER`s for. `D3d12Backend` tracks `scene_state: [D3D12_RESOURCE_STATES; 5]`
(one entry per scene-target mip) and one scalar `blur_scratch_state` (sound because every mip of
`blur_scratch` this crate ever touches is processed identically and left in the same state every frame
— see `run_blur_chain`'s doc comment) plus one state per swapchain back buffer. Every barrier this crate
issues reads the tracked "before" state rather than guessing it, and updates the tracker immediately
after. "Clear vs. Load" itself needed no analog of Metal's `make_render_pass_descriptor` at all: D3D12's
classic (non-`BeginRenderPass`) API treats "don't call `Clear*View`" as implicitly "Load" — genuinely
simpler than Metal's descriptor-based model for this one axis.

**The staging-buffer lifetime bug — caught in review, fixed before this report.** An earlier draft of
`🦀️resources.rs::record_texture_upload` created its upload-heap staging buffer as a plain local
variable, which dropped (released the COM object) at the end of that function — long before the command
list recording the `CopyTextureRegion` referencing it was ever closed, executed, or fenced. D3D12 does
**not** keep resources referenced by a not-yet-executed command list alive for you (documented Microsoft
guidance, not something the `windows` bindings enforce at compile time) — so this would have freed the
staging buffer's memory while a recorded GPU command still named it, a genuine use-after-free class bug
that only a real device run (or very careful review) would surface. Fixed by adding `GpuResources::
pending_staging: Vec<ID3D12Resource>`, pushed to at the end of `record_texture_upload` and drained only
after `execute_and_wait` confirms the fence signaled. Flagging this explicitly rather than only fixing
it silently, since it is exactly the kind of defect the ticket's "verify against vendored source, don't
guess" instruction exists to catch, and it very nearly shipped.

**Synchronization model: one allocator, one list, one fence, full per-frame CPU/GPU serialization.**
`render()` waits for the *previous* frame's fence value before resetting/recording the next one — this
is what makes reusing the single `FrameDescriptors`/`GrowBuffer` set across frames sound without a
double/triple-buffered pool of per-frame resources. A correctness-first choice, not an accident;
explicitly not the deepest-possible pipelining, same spirit as Metal's "panics instead of `Result`
propagation" simplification.

**Texture upload is synchronous, blocking the caller inside `apply_resources`.** Same "correctness
first, not the fastest possible" framing as the previous point — see `🦀️resources.rs`'s header.

**Swapchain format vs. RTV format** — DXGI flip-model swapchains reject sRGB `Format` values directly
(documented D3D11/D3D12 restriction, not visible in the `windows` bindings themselves, so flagged as
unverified-from-source below); this crate creates the swapchain as `DXGI_FORMAT_B8G8R8A8_UNORM` and
creates every back-buffer RTV with an explicit `DXGI_FORMAT_B8G8R8A8_UNORM_SRGB` format override — the
standard technique for automatic sRGB write curves on a flip-model back buffer, matching Metal's
`BGRA8Unorm_sRGB` `capabilities().preferred_surface_format` report.

**`unsafe` blocks and their soundness** (every non-trivial one; file:region → invariant):
- `🦀️types.rs::transition_barrier` — the one `D3D12_RESOURCE_BARRIER` constructor in the crate. Builds
  a "borrowed" `ID3D12Resource` inside the barrier's `ManuallyDrop<Option<...>>` field via
  `transmute_copy` (a bitwise copy, no `AddRef`) rather than `.clone()`; sound because the barrier is
  always consumed synchronously by `ResourceBarrier` within the same function that built it and then
  dropped as a plain stack value (its `ManuallyDrop`'d field's real destructor never runs), while the
  actual owning `ID3D12Resource` reference outlives that call at every one of this crate's call sites.
  Same technique `windows_core::Param<T>`'s own `Borrowed` variant uses internally (confirmed by reading
  `windows-core-0.62.2/src/param.rs`), not invented here.
- `🦀️types.rs::wait_for_fence_value` — a plain, side-effect-free `GetCompletedValue` poll loop; no
  resource-lifetime concern, just a CPU busy-wait substituting for the `CreateEventW`/
  `WaitForSingleObject` pattern this crate cannot use (see registrar-requests).
- `🦀️types.rs::create_upload_buffer`/`create_default_texture2d` — `CreateCommittedResource`/`Map`/
  `Unmap` calls whose pointers/lengths are all derived from the same caller-supplied byte slice or
  just-allocated resource; no aliasing or bounds hazard.
- `🦀️resources.rs::record_texture_upload` — the `D3D12_TEXTURE_COPY_LOCATION` construction uses the
  same borrowed-pointer-without-`AddRef` technique as `transition_barrier`; **the actual lifetime-
  soundness of the resources those locations name** (not the locations themselves) is what
  `pending_staging` exists to guarantee — see the dedicated decision above.
- `🦀️pipelines.rs::build_pso`/`build_root_signature` — the `pRootSignature`/`pParameters`/
  `pDescriptorRanges` pointers all borrow stack-local values alive for exactly the synchronous
  `CreateGraphicsPipelineState`/`D3D12SerializeRootSignature` call that consumes them; `build_pso`'s
  borrowed-root-signature `ManuallyDrop` field is explicitly `mem::forget`'d afterward (documented
  inline) to avoid a phantom `Release` for an `AddRef` that never happened.
- `🦀️backend.rs::run_blur_chain`/`capture_readback` — `D3D12_TEXTURE_COPY_LOCATION` pairs borrowing
  long-lived `self`-owned resources (`scene_target`'s textures, swapchain back buffers, the readback
  buffer moved into `self.readback` immediately after) — no lifetime hazard, unlike the staging-buffer
  case above, precisely because none of these are function-local temporaries.
- Every `OMSetRenderTargets`/`RSSetViewports`/`IASetVertexBuffers`/`ResourceBarrier`/draw call
  throughout `🦀️backend.rs`/`🦀️world3d.rs` — ranges/offsets/counts all come from `RenderPacket` data
  `Scene::finish` already produced in-bounds, or from this crate's own just-computed, exactly-sized
  buffers/heaps.

## What is unverified without Windows

- **The single highest-risk item**: whether every generic `windows`-crate call's type inference
  resolves the way I read it from source (e.g. `device.CreateRootSignature::<ID3D12RootSignature>(...)`
  inferred from the enclosing function's return type, `None` resolving to `Option<&ID3DInclude>` for
  `D3DCompile`'s `pinclude` parameter). I traced every one of these against the vendored source's actual
  generic bounds and the `Param<T>` blanket impls in `param.rs`, but I cannot rule out a rustc inference
  edge case without compiling. If `sol`'s first build shows a type-inference error, `🦀️pipelines.rs`'s
  `compile_shader`/`build_root_signature` and `🦀️resources.rs`'s `GpuResources::new` (the
  `CreateCommandList`/`CreateFence` calls) are the first places to look.
- **The sRGB-override-on-a-UNORM-swapchain-buffer technique** (Decisions, above) is documented D3D11/
  D3D12 SDK behaviour, not something visible in the `windows` crate's Rust bindings — a runtime/driver
  semantic fact, not a binding fact, so vendored-source review cannot confirm it the way it confirmed
  everything else in this report.
- **Whether `IDXGIFactory2::CreateSwapChainForComposition` genuinely needs zero window/composition-
  target setup to succeed** (no `IDCompositionTarget`/`IDCompositionVisual` binding) on a real Windows
  test runner — I could only confirm the *API surface* accepts no `HWND`, not that the runtime accepts
  a swapchain with no compositor ever consuming it. If `new_headless` fails to construct on `sol`'s
  runner, this is the first thing to check; the fallback is the `Win32_UI_WindowsAndMessaging`
  registrar-request below (a hidden real window instead).
- **Root-signature/PSO validation acceptance** — whether the D3D12 debug/validation layer accepts the
  exact `D3D12_SAMPLER_DESC.ComparisonFunc = D3D12_COMPARISON_FUNC_NEVER` choice for non-comparison
  samplers, and whether every PSO's declared root-signature compatibility (a PSO whose shader uses a
  strict subset of the bound root signature's parameters) is accepted the way I reasoned it would be.
- **Descriptor-heap growth/reuse across frames** — `ResidentSrvTable`'s grow-by-doubling path
  (`🦀️resources.rs`) re-issues every `CreateShaderResourceView` against a new heap; I could not verify
  on-device that stale GPU-side references from a *previous* frame's already-executed draws are truly
  unaffected (they should be — the old heap object is simply replaced, and nothing reads it again — but
  this is exactly the kind of thing a GPU validation layer would catch immediately and I cannot run one).
- All 5 device-gated `#[cfg(test)]` tests in `🦀️backend.rs` — same "construction failure is the clean-
  skip signal" shape as Metal's, entirely unrun here.

## Registrar-requests

Three independent asks — any can land without the others:

1. **The planned shader route** (`build.rs` + build-time `naga` WGSL→HLSL cross-compile, replacing
   `🦀️hlsl.rs`'s hand-written strings — the runtime `D3DCompile` step stays either way):
   ```toml
   [build-dependencies]
   naga = { version = "27", features = ["wgsl-in", "hlsl-out"] }
   ```
   plus a `build.rs` that iterates `ui_render::ALL_SHADERS`, runs naga's WGSL→HLSL backend, and emits
   the result via `include_str!`/`OUT_DIR` for `🦀️pipelines.rs` to consume instead of the `_HLSL`
   constants in `🦀️hlsl.rs`.
2. **`Win32_Security`**, to replace `🦀️types.rs::wait_for_fence_value`'s CPU-polling loop with the
   idiomatic `CreateEventW`/`SetEventOnCompletion`/`WaitForSingleObject` pattern:
   ```toml
   windows = { version = "0.62", features = [
       # ...existing entries...
       "Win32_Security",
   ] }
   ```
   `CreateEventW` is `#[cfg(feature = "Win32_Security")]` in the vendored source; confirmed by reading
   it, not assumed.
3. **`Win32_UI_WindowsAndMessaging`** (optional, only needed if the `CreateSwapChainForComposition`
   headless path from "what is unverified without Windows" turns out not to work in practice on the
   real test runner): would let `new_headless` create a real hidden window via
   `RegisterClassExW`/`CreateWindowExW` instead. Not requested as a blocking need — the composition
   swapchain path is the primary design, this is only the documented fallback.

## Deviations (summary)

- World3d is not interleaved with 2D content layer-by-layer — identical upstream contract gap to the
  Metal backend's report (not fixable from this crate; see "Known limitation" above).
- `WORLD3D_TEXTURED_PIPELINE` not implemented — dead/unwired surface in the reference itself, same call
  Metal made.
- Construction/allocation failures panic (`.expect(...)`) rather than returning `Result` — same
  deviation Metal's report documents, same reasoning (failures here only occur from a bug in the
  hand-written HLSL or true out-of-memory during setup, never from user/runtime data).
- Shader route is interim hand-written HLSL + runtime `D3DCompile`, not the planned build-time `naga`
  cross-compile — needs registrar-request 1.
- Fence waits poll `GetCompletedValue` in a sleep loop instead of using an OS wait event — needs
  registrar-request 2 (`Win32_Security` for `CreateEventW`).
- Texture upload in `apply_resources` blocks synchronously on a GPU fence rather than pipelining —
  documented correctness-first simplification, see Decisions.
- `capabilities()`'s `memory_class`/`gpu_tier` are fixed conservative defaults, not queried from the
  real `IDXGIAdapter1` — this backend keeps no adapter reference (passed `None` to `D3D12CreateDevice`
  for the default adapter). Unlike Metal, which does query `hasUnifiedMemory()`/`isLowPower()` for real.
- `recover()` cannot actually heal a *real* `DEVICE_REMOVED` event (only the `backend-testing` simulated
  path) — the trait's `&mut self` / `RecoveredResources` signature has no way to express "reconstruct
  the whole backend," which a real device loss would require. Documented in the method's own doc
  comment and in "what is unverified without Windows."

## Files touched

Created (all new, under `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/`):
`🦀️types.rs`, `🦀️hlsl.rs`, `🦀️pipelines.rs`, `🦀️resources.rs`, `🦀️scene_target.rs`,
`🦀️frame_buffers.rs`, `🦀️world3d.rs`, `🦀️backend.rs`.

Rewritten: `📦️glue.rs` (was the packet's scaffold placeholder).

Not touched: `Cargo.toml` (per OWNS — dependency needs listed under Registrar-requests instead),
everything outside this crate.
