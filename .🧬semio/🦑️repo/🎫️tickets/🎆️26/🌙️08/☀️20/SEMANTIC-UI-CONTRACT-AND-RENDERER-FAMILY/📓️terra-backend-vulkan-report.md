# 📓️ terra-backend-vulkan-report

Packet `backend-vulkan`, wave W3.

## Done — milestone 1 reached in full, plus milestone 6 (device-loss/recovery bookkeeping,
## `backend-testing` scaffolding) and non-shader groundwork toward milestone 2. Milestones 2's
## *pipelines*, 3, 4, 5 are **not** reached.

`VulkanBackend` — a concrete hand-written Vulkan implementation of `ui_render::GraphicsBackend` — in
`🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/`, 1,930 lines across 7 files
(6 new region files + rewritten `📦️glue.rs`):

- `🦀️memory.rs` — `find_memory_type` (the textbook vulkan-tutorial scan, pure and tested against
  fabricated `vk::PhysicalDeviceMemoryProperties`/`vk::MemoryRequirements`), `as_bytes` (this crate's
  `bytemuck::cast_slice` substitute — see "Shader/dependency strategy" below for why), `GrowBuffer` (a
  capacity-doubling `HOST_VISIBLE|HOST_COHERENT` arena, the "grow-buffer arena per frame in flight" the
  ticket brief asks for — written, unit-tested for its growth-policy arithmetic, **not yet wired into
  `VulkanBackend`** since nothing replays per-frame instance data yet).
- `🦀️vk_error.rs` — `VulkanGraphicsError` (this crate's internal error set, richer than the contract's
  `BackendError` needs at the trait boundary — mirrors the Metal target's `MetalGraphicsError`) and
  `classify_vk_result`, the one place every `vk::Result` failure becomes a `BackendError` variant.
- `🦀️swapchain_support.rs` — pure swapchain-configuration decisions: `choose_surface_format` (prefers
  `B8G8R8A8_SRGB`/`SRGB_NONLINEAR`, matching the other three backends' sRGB preference),
  `choose_present_mode` (`MAILBOX` else `FIFO`), `choose_extent` (handles the `u32::MAX` "surface
  defers to us" sentinel and clamps to capabilities), `choose_image_count`, and `is_parked` (the
  zero-size predicate every `resize`/`render` call consults).
- `🦀️descriptor_layout.rs` — pure translation from `ui_render::shader_contract`'s backend-neutral
  `PipelineSpec`/`BindGroupSpec` into Vulkan value structs: `descriptor_type_for`, `stage_flags_for`,
  `descriptor_set_layout_bindings`, `vk_format_for`, `vertex_input_state`, and `batch_scissor` (the
  ticket brief's explicit "per-batch scissor maps 1:1 to dynamic scissor state" instruction, translated
  and unit-tested against `ui_render::UI_CONTENT_PIPELINE`/`VECTOR_PIPELINE`'s real vertex-buffer specs
  — **not yet called from `render`**, see "Milestone 2" below).
- `🦀️resources.rs` — `GpuResources`: real GPU-resident `ResourceOp` application. Textures/atlases go
  through a host-visible staging buffer into a `DEVICE_LOCAL` `vk::Image` (`UNDEFINED →
  TRANSFER_DST_OPTIMAL → SHADER_READ_ONLY_OPTIMAL`, one-time command buffer, `queue_wait_idle`);
  meshes go through the same staging pattern into two `DEVICE_LOCAL` `vk::Buffer`s. Atlas routing by
  upload byte density mirrors the Metal target's `🦀️resources.rs` exactly (1 byte/pixel → glyph,
  4 → icon). `apply`/`drain_known`/`knows_texture`/`knows_mesh` are the same shape the Metal target's
  table exposes.
- `🦀️backend.rs` — `VulkanBackend` itself: `ash::Entry::load` → instance (via `ash-window`'s
  `enumerate_required_extensions`) → physical/logical device + single graphics-and-present queue →
  `khr::surface`/`khr::swapchain` loaders → swapchain + image views → one render pass (clear/store,
  `UNDEFINED → PRESENT_SRC_KHR`) → one framebuffer per swapchain image → a command pool +
  `FRAMES_IN_FLIGHT` command buffers and per-frame semaphore/fence triples. `resize` parks on zero-size
  and recreates the swapchain otherwise (`old_swapchain` chaining, destroy-after-create ordering).
  `render` waits its frame's fence, acquires (recreating on `ERROR_OUT_OF_DATE_KHR`), records a single
  clear-and-present render pass, submits, presents (recreating on `ERROR_OUT_OF_DATE_KHR`, reporting
  `Presented` on `SUBOPTIMAL_KHR` with recreation deferred to the next call). `GraphicsBackend` is fully
  implemented — `device_status`/`recover`/`debug_force_device_loss`/`read_back` included — plus a
  `Drop` that waits idle and destroys every handle in dependency order.
- `📦️glue.rs` — mounts all of the above, **with every `mod` declaration itself
  `#[cfg(target_os = "linux")]`-gated**, not just the top `compile_error!` banner (see "Deviations from
  the scaffold" below — this was necessary, not cosmetic).

### Milestones, against the ticket's six

1. **Instance + device + swapchain + resize + clear-colour frame** — done. 2 frames in flight with
   per-frame fences/semaphores; zero-size park/restore; `ERROR_OUT_OF_DATE_KHR`/`SUBOPTIMAL_KHR`
   recreation on both acquire and present.
2. **UI quad + vector pipelines, glyph/icon/raster images, per-batch scissor from `ResourceOp`** —
   **stopped short of real pipeline objects.** What *is* real: `apply_resources` uploads
   textures/atlases/meshes to actual `DEVICE_LOCAL` GPU memory (not bookkeeping-only), and
   `🦀️descriptor_layout.rs` has the exact, tested translation from `PipelineSpec`/`BindGroupSpec` to
   `vk::DescriptorSetLayoutBinding`/vertex-input-state/dynamic-scissor `vk::Rect2D` that pipeline
   creation needs. What is missing: no `vk::ShaderModule`, no `vk::Pipeline`, no descriptor pool/set,
   no sampler — `render` does not replay `RenderPacket::batches` at all. Root cause: no SPIR-V. See
   "Shader strategy" below.
3. **Stencil silhouette clip** — not reached (depends on milestone 2's pipelines).
4. **Offscreen scene target → mip blur → glass composite → blit** — not reached.
5. **World3d mesh/lines/textured + dynamic-offset uniform ring** — not reached. (Mesh *upload* — the
   GPU-resident vertex/index buffers — is done in `🦀️resources.rs`; the dynamic-offset ring buffer and
   the pipeline to draw them are not.)
6. **`DeviceStatus::Lost` + `recover()` + `backend-testing`** — mostly done. `debug_force_device_loss`/
   `device_status`/`recover` are real and match the Metal target's shape (a *simulated* recovery: drops
   every `GpuResources` table, returns the dead generations, does not actually destroy/recreate the
   Vulkan device — see the doc comment on `VulkanBackend::recover`). `render`'s real
   `ERROR_DEVICE_LOST` path is wired (`render_inner` → `VulkanGraphicsError::Vk(ERROR_DEVICE_LOST)` →
   `DeviceStatus::Lost`) but obviously unexercised without hardware. `read_back` is **not** implemented
   for real — it honestly returns `BackendError::Timeout` (or `ZeroSizeSurface` when parked) rather than
   fabricating pixel data; see its doc comment for exactly what staging-copy work is missing.

## Acceptance: UNRUN (ruling U4 — I do not run cargo)

Commands for `sol` to run:
```
CARGO_TARGET_DIR=<session scratchpad>/target cargo check -p semio-framework-ui-backend-vulkan --lib
CARGO_TARGET_DIR=<session scratchpad>/target cargo check -p semio-framework-ui-backend-vulkan --all-targets
CARGO_TARGET_DIR=<session scratchpad>/target cargo check -p semio-framework-ui-backend-vulkan --lib --target x86_64-unknown-linux-gnu
CARGO_TARGET_DIR=<session scratchpad>/target cargo check -p semio-framework-ui-backend-vulkan --all-targets --target x86_64-unknown-linux-gnu
```
**Expected native (macOS) result**: a single `compile_error!` ("...builds only on Linux.") and nothing
else — see "Deviations from the scaffold" for why every `mod` is `target_os`-gated to make this true.
**Expected Linux-target result**: the real crate compiles (my claim, unverified — every `ash` 0.38 API
used was cross-checked against the vendored source at
`~/.cargo/registry/src/index.crates.io-*/ash-0.38.0+1.3.281/src/` and against a downloaded
`ash-window-0.13.0.crate` — see "Verification method" below — but nothing here has actually been fed
through `rustc`). `#[cfg(test)]` tests will *compile* under the Linux target (`--all-targets`) but
cannot *execute* there from this host (cross-compiled binaries don't run on macOS) — running them for
real needs a Linux box or CI runner, ideally against lavapipe per the ticket brief.

## Verification method (since nothing here executes)

Every `ash`/`ash-window` API surface used — builder methods, `unsafe fn` signatures, struct field
names, `Default`/`PartialEq`/`Debug` derives, module paths (`ash::khr::surface::Instance`,
`ash::khr::swapchain::Device`), the `khr::*::Instance::new(&entry, &instance)` /
`khr::*::Device::new(&instance, &device)` construction pattern — was read directly from
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/{entry,instance,device,
vk/definitions,vk/enums,vk/macros}.rs`, matching the ticket's explicit warning that 0.38 moved to
lifetime-bearing `vk::…::default()` builder chains and recall reproduces the old `.build()` pattern
wrongly. `ash-window` was **not** present in the local registry cache (never fetched on this machine),
so I downloaded `https://static.crates.io/crates/ash-window/ash-window-0.13.0.crate` directly (a plain
`curl`, not a `cargo` invocation — U4 forbids the latter, not source lookup) and read its
`create_surface`/`enumerate_required_extensions` signatures from the real 0.13.0 source rather than
from memory.

## Decisions

**Memory-type selection.** No VMA (ticket brief). `🦀️memory.rs::find_memory_type` is the textbook
`vkGetPhysicalDeviceMemoryProperties` linear scan — first type whose `memory_type_bits` bit is set *and*
whose `property_flags` contains every requested flag. Two allocation shapes use it: (1)
`🦀️resources.rs`'s textures/atlases/meshes get **one dedicated `vkAllocateMemory` each**, always
`DEVICE_LOCAL`, populated via a `HOST_VISIBLE|HOST_COHERENT` staging buffer + one-time command buffer +
`queue_wait_idle` (simple and correct first, not throughput-optimized — a transfer queue / async upload
ring is explicitly future work, noted in `🦀️resources.rs`'s header); (2) `🦀️memory.rs::GrowBuffer` is
the "one grow-buffer arena per frame in flight" the brief asks for — a single `HOST_VISIBLE|
HOST_COHERENT` capacity-doubling buffer, permanently mapped (no `flush_mapped_memory_ranges` needed,
mirrors the Metal target's `Shared`-storage reasoning) — **written and unit-tested but not yet wired**
into `VulkanBackend` since nothing replays per-frame `quad_instances`/`vector_vertices` yet.

**Sync strategy.** 2 frames in flight, the standard vulkan-tutorial shape: per-frame-in-flight
`image_available`/`render_finished` semaphores plus an `in_flight` fence (created `SIGNALED` so the
first wait never blocks). **Documented imprecision, called out in `🦀️backend.rs`'s `FRAMES_IN_FLIGHT`
doc comment**: `render_finished` is indexed by frame-in-flight rather than by swapchain image, which is
technically insufficient when the swapchain has more images than frames-in-flight and present order can
outrun submission order. This is the well-known minimal tutorial synchronization scheme, not a
from-scratch design choice — a per-swapchain-image semaphore array is the documented fix, left as
follow-up since verifying it needs a real device/validation layer, not a `Cargo.toml` change.

**Shader strategy — this is why milestone 2 stops at "groundwork".** The ticket's intended route is
build-time `naga` WGSL→SPIR-V in a `build.rs` (naga as a build-dependency only, keeping the wgpu-boundary
`cargo tree` gate green) — that needs a `Cargo.toml`/`build.rs` change this packet cannot make (U7,
registrar-only). The fallback the brief offers is hand-written GLSL/SPIR-V, embedded and marked interim
— which the Metal target's analog (hand-written MSL, transcribed line-for-line from the canonical WGSL)
did successfully. I did not do the SPIR-V equivalent: **no shader compiler is available in this
environment** (`glslc`, `glslangValidator`, `spirv-dis`, `spirv-val`, a `naga` CLI — none installed;
checked with `which`), and hand-assembling the UI megashader's SPIR-V binary by hand (9 `kind` branches,
SDF rounded-rect math, atlas sampling) with no `spirv-val` to check it against would mean shipping
unverifiable bytecode and calling it done — exactly what CLAUDE.md's "you MUST NOT assume, you MUST
validate your assumptions" and the ticket's own "never claim a check passed without its output" forbid.
Rather than fabricate SPIR-V I cannot validate, I built every *other* piece milestone 2 needs
(descriptor layouts, vertex-input state, scissor mapping, real texture/mesh residency) and stopped at
the shader boundary — see "Registrar-requests" for the actual unblock.

**Every `unsafe` block and its soundness argument** is written as an inline `// 🔓️ SAFETY:` comment at
the call site (per the ticket's "keep blocks tight and comment each with the invariant making it
sound" instruction) — there are ~45 of them across `🦀️backend.rs`/`🦀️resources.rs`/`🦀️memory.rs`. The
recurring invariants: (a) handle validity — every handle passed to a `destroy_*`/`free_*`/`cmd_*` call
was created on the same `device`/`instance` earlier in the same file and is still alive; (b) lifetime
ordering — children destroyed before parents, enforced by `Drop`'s explicit sequencing and by
`recreate_swapchain` creating the new swapchain before destroying the old one; (c) no concurrent GPU
access — every `destroy_*` in `🦀️resources.rs`/`🦀️backend.rs::Drop` is preceded by a `device_wait_idle`
or (for the one-time upload path) a `queue_wait_idle`; (d) pointer/buffer bounds — every `map_memory`+
`copy_nonoverlapping` pair copies at most the just-allocated capacity.

**Where Vulkan diverges from Metal's shape, and why.**
1. **Per-batch dynamic scissor, not stencil-only clipping.** The ticket brief explicitly instructs
   "per-batch scissor maps 1:1 to dynamic scissor state — use it rather than baking scissor into
   pipelines," where the Metal target instead resets to full-viewport every batch and clips entirely
   through the stencil silhouette mask (see `📓️terra-backend-metal-report.md`'s "Decisions"). I followed
   the brief: `🦀️descriptor_layout.rs::batch_scissor` translates `DrawBatch::layer_state.scissor` into a
   `vk::Rect2D` for `vkCmdSetScissor`, ready for `render` to call once pipelines exist.
2. **Manual handle lifetime, no ARC.** Metal's `objc2::rc::Retained<T>` reference-counts every GPU
   object; `ash` handles are plain `Copy` integers/pointers with no destructor, so every owning struct
   here (`VulkanImage`, `VulkanMesh`, `GrowBuffer`, `FrameSync`, `VulkanBackend` itself) carries an
   explicit `destroy`/`Drop` and this crate is responsible for getting destruction *order* right, not
   just *completeness* — a class of bug Metal's ARC makes structurally impossible.
3. **Explicit swapchain image count / present mode / format negotiation.** `CAMetalLayer` hides this
   behind `nextDrawable`; Vulkan's `VK_KHR_surface`/`VK_KHR_swapchain` require querying capabilities/
   formats/present-modes and choosing among them — `🦀️swapchain_support.rs` is the Vulkan-only layer
   with no Metal analog.
4. **A render pass + framebuffer, not a `MTLRenderPassDescriptor` per call.** Metal builds its pass
   descriptor inline per `render`; Vulkan's render pass/framebuffer objects are built once and reused,
   only rebuilt on `recreate_swapchain`.

## What is unverified without a Vulkan device

Everything that touches `ash::Entry::load`/`vkCreate*`/`vkCmd*`/`vkQueue*` — i.e. essentially all of
`🦀️backend.rs` and the non-pure half of `🦀️resources.rs`/`🦀️memory.rs`. Concretely: whether
`ash::Entry::load()` finds a loader at all on a target machine; whether physical-device/queue-family
selection (`pick_physical_device`) actually finds a graphics+present-capable family on real hardware;
whether the render pass/framebuffer/swapchain image-view combination is accepted by validation layers
(none were enabled — see "Deviations" below); the swapchain recreation path's correctness under a real
resize event; the staging-buffer upload path's correctness (barrier stage/access masks, buffer-to-image
copy region math) with real pixel/vertex data; `Drop`'s destruction ordering under a real multi-frame
in-flight state. **None of this has been exercised even once** — this machine has no Vulkan loader.

## Registrar-requests

1. **`build.rs` + `[build-dependencies] naga = "…"` for `semio-framework-ui-backend-vulkan`** — the
   actual unblock for milestone 2's pipelines. Compile the canonical WGSL in
   `ui_render::shader_contract::ALL_SHADERS` to SPIR-V at build time (naga's WGSL front end → SPIR-V
   back end), embed via `include_bytes!` from `OUT_DIR`. This keeps naga out of the runtime dependency
   graph (the wgpu-boundary `cargo tree` gate stays green) exactly as the ticket brief specifies. Until
   this lands, milestone 2 cannot progress past what is already here.
2. Everything else needed (`ash`, `ash-window`, `raw-window-handle`, `backend-testing` feature) is
   already declared in the existing `Cargo.toml` — no other registrar-request.

## Deviations from the scaffold

**`📦️glue.rs`'s `mod` declarations are individually `#[cfg(target_os = "linux")]`-gated**, not just the
top `compile_error!` banner the scaffold shipped with. Reasoning: `Cargo.toml` puts `ash`/`ash-window`/
`raw-window-handle` behind `[target.'cfg(target_os = "linux")'.dependencies]`, so on macOS those crates
are absent from the dependency graph entirely. A `compile_error!` macro does not stop the rest of the
crate from being parsed and name-resolved — if the `mod backend;` etc. declarations underneath it were
ungated, a native `cargo check` on macOS would report the intentional `compile_error!` *plus* a cascade
of "can't find crate `ash`" errors from every `use ash::…` in the newly-written modules, which is not
what "a single expected compile_error" (the scaffold's evident intent, and what the Metal target's own
`#[cfg(not(target_os = "macos"))]` guard achieves for *its* platform by symmetry) means. Gating each
`mod` line restores that: native macOS sees exactly the one intentional error, the Linux target sees the
real crate. The `compile_error!` text itself is unchanged, and the guard is `keep`-compliant per the
ticket brief ("scaffold with a `compile_error!` Linux-only guard: **keep it**").

## Files touched

Created:
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🦀️memory.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🦀️vk_error.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🦀️swapchain_support.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🦀️descriptor_layout.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🦀️resources.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🦀️backend.rs`

Modified:
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/📦️glue.rs` (mounted the
  above; kept and per-module-gated the `compile_error!` guard)

Not touched (registrar-only, per U7): `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/Cargo.toml`
