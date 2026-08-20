# 📓️ terra-backend-iface-report

Packet `backend-iface` — the `GraphicsBackend` contract. Anchor commit `5e7b8046be`.

## Done

Replaced the scaffold body of
`🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️backend.rs` (owned file, only edit) with:

- `PhysicalSize` (physical-pixel surface size; `ZERO`, `new`, `is_zero`).
- `DeviceCapabilities` + `SurfaceFormat` / `MemoryClass` / `GpuTier`.
- `LossReason`, `ResourceKind`, `BackendError` (the full failure set from the packet spec: surface
  out-of-date, surface lost, device lost, timeout, out of memory, unsupported format, zero-size
  surface, browser canvas replaced, shader compilation failure — plus `UnknownResource(ResourceKind)`
  for the "referenced but never uploaded" case the TESTS section requires).
- `FrameStats`, `RenderReport { Presented, SkippedZeroSize, SkippedOutOfDate }`.
- `DeviceStatus { Healthy, Suboptimal, Lost(LossReason) }`.
- `RecoveredResources { lost_textures, lost_meshes, lost_atlases }`.
- `ReadbackImage` (gated `#[cfg(feature = "backend-testing")]`).
- `pub trait GraphicsBackend` — exact signature from the packet spec, full invariant docstring
  (sync/no-reentrancy, `apply_resources`-before-`render` ordering, zero-size parking, verbatim batch
  replay, only-construction-is-async).
- `NullBackend` — a real working no-op implementation: tracks resource residency via `HashSet<TextureId
  /MeshId/AtlasId>` populated from `apply_resources`' `ResourceOp` stream, validates every id a
  `RenderPacket`'s batches/surface-passes reference before "rendering", reports `FrameStats`, and
  implements the `backend-testing` hooks (`debug_force_device_loss` / `read_back`).
- `pub type ActiveBackend = NullBackend;` plus the file's top docstring, which is the full U3
  explanation and hand-off note (see Decisions below).
- `#[cfg(test)] mod tests` with the five scenarios the packet's TESTS section names, plus a
  `drive_backend<B: GraphicsBackend>(&mut B)` helper proving the trait is exercised generically, never
  through `dyn`.

## Acceptance: UNRUN (ruling U4 — coordinator runs cargo)

Exact commands for `sol`:

```
CARGO_TARGET_DIR=<session-scratchpad>/target cargo check -p semio-framework-ui-render --lib --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo check -p semio-framework-ui-render --all-targets --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo test -p semio-framework-ui-render --lib --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo test -p semio-framework-ui-render --lib --features backend-testing --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo tree -p semio-framework-ui-render -i wgpu --timeout 600000   # expect: package not found (no wgpu in the dep graph)
CARGO_TARGET_DIR=<session-scratchpad>/target cargo tree -p semio-framework-ui-render -i winit --timeout 600000  # expect: package not found
```

Expected to still be RED at this point per the packet brief: the crate does not compile yet because a
sibling contract packet (`shader-repair`/`render-scene`, this crate's `element.rs`/`frame.rs`/`layout.rs`
/`schedule.rs`/`dispatch.rs`/`text.rs`/`surface.rs` scaffolds) had not landed as of this read. That is
expected and not this packet's regression.

Non-cargo checks I *did* run (cheap, no build): `rustfmt --config-path ./rustfmt.toml --check` on
`🦀️backend.rs` — clean after one formatting fix (a struct-literal line rustfmt wanted joined). A clean
rustfmt run also confirms the file parses as valid Rust. Grepped the file for `async fn`, `\bdyn\b`,
`wgpu`, `winit` — the only hits are inside doc-comment prose explaining the rules (e.g. "never `async
fn`", "why this trait carries zero `dyn`", a doc-comment cross-reference to `wgpu::StencilState` by
name), never in actual code. Brace/paren counts balance (78/78, 236/236). Grepped for stray `//`
comments inside function bodies — none beyond the mandated `// 🚫️async:` tags, doc comments and
`//#region` markers.

## Decisions

**The cfg-selected concrete-backend mechanism (U3).** `GraphicsBackend` has exactly one compiled impl
per target because the four backend crates are `cfg`-exclusive (browser wasm32 → webgpu, macOS →
Metal, Windows → D3D12, Linux → Vulkan) — there is never a runtime choice to erase, so the seam is a
`pub type ActiveBackend = …;` alias to a **concrete** type, not an enum/box/vtable. The key subtlety:
the four backend crates each *depend on* `semio-framework-ui-render` (to implement `GraphicsBackend`
for their own device-backed struct) — they can never be a dependency *of* this crate without inverting
the graph. So `ActiveBackend` **in this file** cannot ever alias to a real backend; it resolves to
`NullBackend` unconditionally, which is this crate's only `GraphicsBackend` impl. The real per-target
alias belongs one layer up, in `semio-framework-ui-host` (packet `ui-host`), which is the crate that
legitimately depends on all four backend crates. I wrote out the exact pattern that packet must follow
in `🦀️backend.rs`'s top docstring:
```rust
#[cfg(target_arch = "wasm32")]
pub type ActiveBackend = semio_framework_ui_backend_webgpu::WebGpuBackend;
#[cfg(target_os = "macos")]
pub type ActiveBackend = semio_framework_ui_backend_metal::MetalBackend;
#[cfg(target_os = "windows")]
pub type ActiveBackend = semio_framework_ui_backend_d3d12::D3d12Backend;
#[cfg(all(unix, not(target_os = "macos")))]
pub type ActiveBackend = semio_framework_ui_backend_vulkan::VulkanBackend;
```
Each backend crate's context type is `pub` and implements `GraphicsBackend` directly — nothing about
the trait or this file changes when the four backend packets land. Generic code (frame drivers, the
conformance harness) is written as `fn drive<B: GraphicsBackend>(backend: &mut B)`, never
`Box<dyn GraphicsBackend>`; the in-file test `drive_backend` demonstrates the shape.

**`RecoveredResources`' shape was found, not invented.**
`crate::resource::ResourceRegistry::report_device_loss(&mut self, lost_textures: &[TextureId],
lost_meshes: &[MeshId], lost_atlases: &[AtlasId])` already exists (packet `render-scene`/sibling). I
matched `RecoveredResources`'s three field names and order to it exactly so a caller does
`registry.report_device_loss(&r.lost_textures, &r.lost_meshes, &r.lost_atlases)` with no impedance
mismatch — documented in `RecoveredResources`'s own docstring.

**No type collisions with siblings.** Checked `🦀️scene.rs` (`RenderPacket`, `DrawBatch`,
`PipelineKind`, `QuadInstance`, `LayoutRect`, …) and every other region file in the crate (`element.rs`,
`frame.rs`, `layout.rs`, `schedule.rs`, `dispatch.rs`, `text.rs`, `surface.rs` are all still 7-line
scaffolds, `tessellate.rs` is landed but unrelated) before adding `PhysicalSize` and `ReadbackImage` —
neither existed anywhere in the crate, so both are defined fresh here, not duplicated.

**`BackendError::UnknownResource`** is an addition beyond the packet's named failure-set bullet list,
required by the TESTS section ("render referencing an id never uploaded is a clean error not a
panic"). Carries a `ResourceKind` (`Texture`/`Mesh`/`Atlas`) so a caller can tell which table missed.

**`ZeroSizeSurface` is deliberately unused by `resize`/`render`.** Per the invariant ("a zero-size
surface parks rather than erroring"), no compliant backend returns it from those two methods —
`NullBackend` never does. It is still part of the enum (documented as such) because a
`backend-testing` `read_back()` on a parked surface has nothing to read back and legitimately returns
it; `NullBackend::read_back` does exactly that.

## Registrar-requests

None. Nothing outside `🦀️backend.rs` needed a change for this packet.

## Deviations

None from the packet brief. `DeviceCapabilities`'s `preferred_surface_format` / `memory_class` /
`gpu_tier` fields needed backing enum types (`SurfaceFormat`, `MemoryClass`, `GpuTier`) that the brief
named by field but not by shape — defined narrowly (3 variants each) to cover what `NullBackend` and
the four real backends plausibly need, same "backend maps this onto its own device type" posture as
`crate::scene::StencilPolicy`. Not a deviation from anything specified, just a gap the brief left to
this packet's judgment.
