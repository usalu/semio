---
name: Wgpu Infinite World Parity
overview: Port chunking/view-radius streaming, refcounted object pooling, distance-based LOD, and the progressive world grid from the premigration `@semio-tech/infinite-world-r3f` engine into the Rust wgpu world canvas (`infinite/world/rs`), as generic opt-in infrastructure usable by every `world-3d` program.
todos:
 - id: ticket
   content: Open ticket via repo MCP under goal puzzle3d/puzzle3dplay
   status: completed
 - id: phase1-math
   content: Port LOD + progressive-grid pure math and floating_origin_rebase into kernel/3d/scene/rs/lib.rs with tests
   status: completed
 - id: phase2-chunking
   content: Add chunk-key bucketing + hysteresis visibility, wire into World3dState/sync_world3d_state/render_world_3d
   status: completed
 - id: phase3-pooling
   content: Add refcounted mesh pool tied to chunk load/unload; evict meshes/GPU cache at refcount zero
   status: completed
 - id: phase4-lod-grid-render
   content: Resolve per-mesh LOD URLs; generate progressive grid line draws anchored at orbit target, excluded from ray-pick
   status: completed
 - id: phase5-schema-wiring
   content: Add WorldLodRecord/WorldChunkingRecord/WorldMeshLodEntry to framework/core World3dScene; wire puzzle3d + cad plugins
   status: completed
 - id: phase5-tests-verify
   content: Extend existing test modules, run cargo tests + wasm build + smoke plays, verify via console logs, close ticket
   status: in_progress
isProject: false
---

# Wgpu Infinite World Parity (Chunking, Pooling, LOD, Grid)

## Context

`infinite/world/rs/lib.rs` (the wgpu port of the world-3d component) already has strong parity for interaction: orbit camera, click/marquee/lasso/component selection, gumball, paint-on-mesh (see `infinite/world/rs/lib.rs:1–3568`, closed tickets `MARQUEE-CROSSING-WINDOW-SELECTION`, `MAP-WGPU-RENDERER-PARITY`, `FIX-WGPU-WORLD3D-EMPTY-PREVIEW`).

It has **zero** of the infinite-world infrastructure that the premigration reference engine [infinite/world/r3f/index.tsx](infinite/world/r3f/index.tsx) provides and that CAD + puzzle3d already consume there (per the closed `UNIFY-CAD-AND-PUZZLE3D-INFINITE-WORLD` ticket):

- **Chunking / view-radius streaming** — `chunkKey`, hysteresis visibility, `WorldChunks` (r3f lines 478–600)
- **Object pooling** — `createRefCountPool` / `createTemplatePool` refcounted GPU-object reuse (r3f lines 615–704)
- **LOD** — `lodFromCameraDistance`, `pickClosestLod`/`pickClosestMeshUrl`, LOD context/bridge (r3f lines 706–1011)
- **Progressive grid** — `lodProgressiveGridLayers`, quantum bands, `WorldLodGridHelper` (r3f lines 780–894)
- **Precision** — `floatingOriginRebase`, orbit-anchored grid placement (r3f lines 146–223)

Confirmed via full-file review: current `infinite/world/rs/lib.rs` iterates the **entire** `state.draws` every frame with no spatial cells (`1456–1488`), has no mesh eviction (`meshes: HashMap` only grows, `204`), and draws no grid lines at all. `World3dScene` in `framework/core/rs/lib.rs:2062–2081` has no fields for any of this.

Selection-mechanism gaps (e.g. `append_component_vertex_spheres` stub at `infinite/world/rs/lib.rs:562-564`) are already tracked by `DRAW-POINTS-INSTEAD-OF-SPHERES-FOR-LOWPOLY-VERTICES` and `PER-COMPONENT-HOVER-AND-VERTEX-SPHERES` — **out of scope here** per confirmation.

## Design: generic opt-in schema

Extend `World3dScene` (`framework/core/rs/lib.rs:2062`) with new optional JSON fields, all `#[serde(default, skip_serializing_if = "Option::is_none")]` so every existing program (`lowpoly`, `cad`, `procedural/3d`, `shooting`, `s`, `puzzle/5d`, `puzzle/3d`, `forms`) keeps compiling untouched:

- `lod_json: Option<String>` → `WorldLodRecord { automatic, manual, distance_reference, depth_variable, grid_factor, grid_snap_enabled, show_grid, grid_datum }` — defaults mirror the r3f "framework host" composition (`automatic: true`, `show_grid: true`, `distance_reference: 100.0`, `grid_factor: 10.0`) so grid/LOD parity is visible immediately without touching every program.
- `chunking_json: Option<String>` → `WorldChunkingRecord { chunk_size, max_distance }` — absent = unbounded (today's behavior, matching r3f's opt-in `ViewRadiusLayer`).
- `WorldMeshRecord` (`infinite/world/rs/lib.rs:68–72`) gains `lods: Option<Vec<WorldMeshLodEntry { lod: f64, url: String }>>` mirroring `LodMeshEntry`/`pickClosestMeshUrl`.

Wire `puzzle/3d/plugin/rs/lib.rs` and `cad/plugin/rs/lib.rs` to set `chunking_json` with the same constants CAD uses today (`chunkSize=256`, `maxDistance=8000`, per `cad/renderer/js/index.tsx:2974-2975`), demonstrating real chunk streaming parity for the two consumers that use it upstream.

## Phase 1 — Pure math: LOD + progressive grid (`kernel/3d/scene/rs/lib.rs`)

Port as dependency-free Rust functions (mirrors r3f lines 724–810), colocated with existing camera/culling math (`kernel/3d/scene/rs/lib.rs:238–541`):

- `lod_from_camera_distance(distance, reference) -> f64`
- `pick_closest_lod(available: &[f64], desired: f64) -> Option<f64>` (log-space nearest, ties favor smaller/more-detailed)
- `pick_closest_mesh_url<'a>(entries: &'a [(f64, &'a str)], desired: f64, fallback: Option<&'a str>) -> Option<&'a str>`
- `lod_grid_band_steps_world(grid_factor: f64) -> [f64; 4]` (quanta `10, 2.5, 0.5, 0.1 × factor`)
- `lod_progressive_grid_layers(lod: f64, grid_factor: f64) -> Vec<(f64 /*step*/, f32 /*opacity*/)>` — same thresholds as r3f (`≤50`, `≤10`, `≤2`, cutoff `>1000`)
- `floating_origin_rebase(world: Vec3, anchor: Vec3) -> Vec3` (plain subtraction, ported from r3f `211–212`, for future camera-relative rendering)

Add unit tests alongside existing `kernel_3d_scene` tests mirroring the r3f test cases (r3f `2973–3008`: distance→LOD, band thresholds, layer-key stability).

## Phase 2 — Chunking / view-radius streaming (`infinite/world/rs/lib.rs`)

- `chunk_key(origin: Vec3, chunk_size: f64) -> (i64, i64, i64)` — floor-division bucketing (r3f `480–484`).
- `chunk_visible(cam_pos, chunk_center, chunk_size, max_dist, was_visible) -> bool` — same hysteresis band as r3f `499–513` (`enter = maxDist + 0.866·chunkSize`, `exit = enter + 0.5·chunkSize`).
- Extend `World3dState` (`198–324`) with `chunking: Option<WorldChunkingConfig>` and `visible_chunks: HashSet<(i64,i64,i64)>` (persisted across frames for hysteresis).
- In `sync_world3d_state` (`1133–1397`) / `render_world_3d` (`1435–1748`): when chunking is configured, bucket instances by `chunk_key(instance.position, chunk_size)`, recompute `visible_chunks` from camera position each frame, and only push instances in visible chunks into `culled_draws` (extends the existing per-instance frustum cull at `1456–1488` with a chunk-level pre-filter). When chunking is absent, behavior is unchanged (all instances considered, current path).

## Phase 3 — Object pooling (`infinite/world/rs/lib.rs`)

- Add a small `RefCountPool<K>` (mirrors r3f `createRefCountPool`/`createTemplatePool`, `617–698`): `acquire`, `release` (delete at 0), `keys`.
- Track mesh refcounts keyed by `mesh_id` as chunks/instances referencing them enter/leave the visible set (Phase 2); when a mesh's refcount hits zero, remove it from `state.meshes` / `state.mesh_versions` (`204–209`) so `ui_wgpu::GpuContext::ensure_mesh`'s backing `MeshGpuStore` entry can also be evicted (add an `evict_mesh(mesh_id)` on `GpuContext` if none exists — check `ui/wgpu/rs/lib.rs` `ensure_mesh`/`MeshGpuStore`).
- GLB/reference-image pending-fetch state (`pending_glb_urls`, `pending_image_urls`) also gets pool-aware: don't re-fetch/re-decode assets already resident for a still-referenced key.

## Phase 4 — LOD-driven mesh resolution + progressive grid rendering

- Resolve `WorldMeshRecord.lods` via `pick_closest_mesh_url` against the current scene LOD (computed each frame via `lod_from_camera_distance(camera_distance, lod_config.distance_reference)`, feeding `mesh_from_kind`/GLB URL selection at mesh-sync time, `1133–1397` / `2865–2915`).
- Each frame in `render_world_3d`, when `lod_config.show_grid`: compute `lod_progressive_grid_layers`, generate grid `LineVertex3d`/`LineDraw3d` segments (finite footprint, e.g. 12,000 world units per r3f `867`) anchored at the orbit target XY / `grid_datum` Z (r3f `gridPlacementAnchorCad`, `216–221`), push into `ScenePass3d.line_draws` (`kernel/3d/scene/rs/lib.rs:426–457`) with per-band opacity, and **exclude grid lines from ray-pick/hit-testing** (grid must not intercept `pick_instance_at`/`ground_plane_pick`, matching r3f's `raycast = worldRaycastNone`, `840–854`).

## Phase 5 — Wiring, tests, verification

- `framework/core/rs/lib.rs`: add `WorldLodRecord`, `WorldChunkingRecord`, `WorldMeshLodEntry` structs + defaults next to `World3dScene` (`2062–2099`).
- `puzzle/3d/plugin/rs/lib.rs`, `cad/plugin/rs/lib.rs`: set `chunking_json` (chunk_size 256, max_distance 8000) when constructing `World3dScene`.
- Extend existing test module in `infinite/world/rs/lib.rs` (`2987+`) — do not create new test files: chunk key bucketing/hysteresis, pool refcount eviction, LOD mesh URL resolution, grid layer generation/exclusion-from-pick.
- Extend `kernel_3d_scene` tests for the ported pure functions.
- Run `cargo test -p kernel_3d_scene -p infinite_world -p semio-framework-core --lib`, then `bun ./framework/renderer/wgpu/script.ts wasm` and smoke puzzle3d/cad wgpu plays (grid visible, LOD bands change with zoom, chunks unload at distance, meshes re-appear on re-approach) — confirm via console logs per repo rules before closing the ticket.

## Ticket

Open a new ticket via repo MCP (`ticket_open`) under goal `🎯puzzle3d🎯puzzle3dplay` (same goal as the closed `UNIFY-CAD-AND-PUZZLE3D-INFINITE-WORLD` ticket this continues) titled "Wgpu Infinite World Chunking Pooling Lod Grid Parity", and do all work/temp files inside its ticket folder.
