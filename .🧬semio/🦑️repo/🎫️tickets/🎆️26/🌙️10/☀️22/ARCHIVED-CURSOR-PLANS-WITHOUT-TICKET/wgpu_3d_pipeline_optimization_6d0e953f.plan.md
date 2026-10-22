---
name: Wgpu 3D Pipeline Optimization
overview: Fix the per-pass camera uniform bug with dynamic offsets, eliminate per-frame GPU buffer allocation with persistent grow-only buffers, cache scene parsing, add AABB culling/early-out to picking and rendering, and raise device limits to full WebGPU defaults.
todos:
 - id: dynamic-uniforms
   content: "Dynamic-offset world globals buffer: one aligned slot per scene pass, fixes multi-viewport camera bug"
   status: completed
 - id: persistent-buffers
   content: Persistent grow-only vertex buffers for world instances, UI instances, vector vertices; single combined instance upload per frame
   status: completed
 - id: device-limits
   content: Request full WebGPU default limits instead of downlevel webgl2 defaults
   status: completed
 - id: scene-cache
   content: Cache scene JSON hashes on World3dState, skip re-parse when unchanged; fix triple mesh_from_kind; content-versioned MeshGpuStore
   status: completed
 - id: picking-aabb
   content: AABB slab early-out in ray picking and projected-AABB pre-test in marquee selection
   status: completed
 - id: frustum-culling
   content: Frustum plane extraction + per-instance AABB culling before building ScenePass3d draws
   status: completed
 - id: safe-glb-fetch
   content: Replace unsafe raw-pointer GLB polling with Rc<RefCell> spawn_local pattern
   status: completed
 - id: cleanup-verify
   content: Remove dead fields/imports, run cargo tests, wasm build, browser verification with puzzle3d and puzzle5d
   status: completed
isProject: false
---

# WGPU 3D Pipeline Optimization

## Confirmed issues in the current implementation

- **Camera bug (correctness)**: in [ui/wgpu/rs/draw.rs](ui/wgpu/rs/draw.rs) `render_world_passes`, `update_world_globals` writes the single `world_globals_buffer` per pass during encoding. `queue.write_buffer` executes before the submitted command buffer, so with 2+ world-3d surfaces every pass renders with the last camera.
- **Per-frame allocations**: `create_buffer_init` each frame for world instance data (once per draw call), `ui_instances`, and `vector_vertices`.
- **Restrictive limits**: `GpuContext::from_canvas` requests `Limits::downlevel_webgl2_defaults()` on a `BROWSER_WEBGPU`-only backend ([ui/wgpu/rs/gpu.rs](ui/wgpu/rs/gpu.rs)).
- **Per-frame CPU work**: [framework/renderer/wgpu/rs/world3d.rs](framework/renderer/wgpu/rs/world3d.rs) `sync_world3d_state` re-parses camera/meshes/instances/selection JSON and regroups instances every frame; `mesh_from_kind` is invoked three times per missing-mesh instance.
- **Unculled picking**: `ray_pick_instance` tests all triangles with no AABB slab test despite `Mesh3d` carrying `aabb_min`/`aabb_max`; `screen_select_instances` projects every triangle of every instance.
- **Unsafe GLB polling**: `AppRuntime::frame` in [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) spawns an async task via a raw `*mut AppRuntime`.
- Dead fields in `UiPipelines` (`glyph_view`, `surface_format`, bind group layouts) generate warnings.

wgpu 27 scope note: mesh shaders, `multi_draw_indirect`, and `PipelineCache` are native-only; on browser WebGPU the effective techniques are instancing (already in place), dynamic uniform offsets, persistent buffers, and CPU-side culling.

## 1. Dynamic-offset world globals (fixes multi-viewport camera bug)

In [ui/wgpu/rs/draw.rs](ui/wgpu/rs/draw.rs):

- Make the world globals bind group layout use `has_dynamic_offset: true` with `min_binding_size` = size of `World3dGlobals`.
- Allocate a grow-only uniform buffer with one 256-byte-aligned slot per scene pass (alignment from `device.limits().min_uniform_buffer_offset_alignment`).
- Before encoding, write all pass globals in one loop; during encoding bind with `set_bind_group(0, &bg, &[offset])` per pass.

## 2. Persistent grow-only vertex buffers

Add a small `FrameBuffers` struct owned by `GpuContext` (world instances, UI instances, vector vertices):

- Each is a `wgpu::Buffer` with `VERTEX | COPY_DST`, grown (power-of-two) only when the frame's data exceeds capacity, otherwise reused via `queue.write_buffer`.
- World instances: build one combined `Vec<World3dGpuInstance>` for all passes/draws before encoding; each draw call records its `(offset, count)` range and uses `set_vertex_buffer(1, buffer.slice(range))`. One upload per frame, zero allocations in steady state.
- Same pattern replaces the per-frame `ui_instances` and `vector_vertices` `create_buffer_init` calls.

## 3. Full WebGPU device limits

In [ui/wgpu/rs/gpu.rs](ui/wgpu/rs/gpu.rs): request `wgpu::Limits::default()` clamped with `.using_resolution(adapter.limits())` instead of `downlevel_webgl2_defaults()`, so large meshes (kit GLBs) and bigger uniform ranges work.

## 4. Scene sync caching in the world-3d host

In [framework/renderer/wgpu/rs/world3d.rs](framework/renderer/wgpu/rs/world3d.rs):

- Store content hashes (or the raw strings) of `camera_json` / `meshes_json` / `instances_json` / `selection_json` on `World3dState`; skip `sync_world3d_state` parsing entirely when unchanged (the common case at 60fps). Camera changes stay renderer-local through the orbit controller.
- Fix the triple `mesh_from_kind` call: generate the primitive once and insert.
- `MeshGpuStore::ensure_mesh` keyed by `(id, version)` where version is a hash of the inline mesh data, so plugin-side mesh edits re-upload instead of being ignored.

## 5. AABB early-out for picking and marquee

In [ui/wgpu/rs/scene3d.rs](ui/wgpu/rs/scene3d.rs), all pure and unit-tested:

- `ray_pick_instance`: transform the ray into local space (restore inverse-transform path), slab-test against `aabb_min`/`aabb_max`, and only then run Moller-Trumbore over triangles. Keep returning world-space distance.
- `screen_select_instances`: project the 8 AABB corners first; if the projected AABB misses the marquee polygon/rect bounding box, skip the instance without touching triangles.

## 6. CPU frustum culling per instance

When building `ScenePass3d` draws in the world-3d host: test each instance's transformed AABB against the camera frustum (extract planes from view-proj in `scene3d.rs`, `frustum_planes(view_proj)` + `aabb_intersects_frustum`) and drop culled instances from the draw list. Matters for puzzle3d/design scenes with many instances.

## 7. Safe GLB fetch scheduling

In [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs): replace the raw-pointer `spawn_local` with the existing `Rc<RefCell<AppRuntime>>` clone pattern (fetch bytes outside the borrow, decode + `ingest_glb_mesh` inside a short borrow). Remove the `asset_poll_pending` unsafe block.

## 8. Cleanup

Remove dead `UiPipelines` fields (`glyph_view`, `surface_format`, unused layouts), the leftover unused-import warnings in `world3d.rs`/`scenes.rs`, and the now-unused `ear_clip_polygon` import in scenes if applicable.

## Verification

- `cargo test -p ui_wgpu` (new tests: dynamic offset slot math, AABB slab test, frustum culling, projected-AABB marquee early-out).
- `cargo test -p semio-framework-core` and plugin crates unchanged-green.
- `cargo build -p semio-framework-renderer-wgpu --target wasm32-unknown-unknown --release` + `bun ./📜️script.ts wasm` artifact regen.
- Browser check with `SEMIO_RENDERER=wgpu` `?plugin=puzzle3d` (many instances, one draw call per mesh) and `?plugin=lowpoly`: `[DEBUG]` logs for buffer growth events, culled instance counts, and per-frame re-sync skips; verify two simultaneous world-3d surfaces (puzzle5d) get distinct cameras.
