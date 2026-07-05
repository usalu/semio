---
name: Puzzle3D WGPU Feature Parity
overview: "Fix the empty puzzle3d viewport (a real culling/projection bug) and then rebuild, in the new WGPU renderer, the full legacy PlayCanvas visualization/interaction set that the current React world-3d-host.tsx no longer has: vortices, attractions, target volumes, reference image planes, brush placement preview, and object relocate with indirect/proximity connect."
todos:
  - id: phase0-frustum-fix
    content: Fix frustum_planes matrix indexing bug in ui/wgpu/rs/scene3d.rs with a real regression test
    status: completed
  - id: phase0-ndc-fix
    content: Fix Mat4::perspective to WGPU NDC z in [0,1] convention
    status: completed
  - id: phase0-layer-batch-fix
    content: Harden build_layer_batches to not drop scene-only layers
    status: completed
  - id: phase0-glb-id-fix
    content: Fix GLB mesh id mismatch in apply_glb_bytes
    status: completed
  - id: phase0-verify
    content: Rebuild WASM and verify puzzle3d/puzzle5d/lowpoly e2e show real geometry
    status: in_progress
  - id: phase1-schema
    content: Add references field + vortex radius to Puzzle3dFixture; extend World3dScene with vortices/attractions/targetVolumes/references JSON
    status: pending
  - id: phase1-plugin-wiring
    content: Build and wire world_vortices_json/world_attractions_json/world_target_volumes_json/world_references_json in puzzle3d plugin
    status: pending
  - id: phase2-line-pipeline
    content: Add WORLD3D_LINES_SHADER + pipeline for attraction lines and target-volume wireframes
    status: pending
  - id: phase2-vortex-markers
    content: Add vortex marker mesh/instances reusing world pipeline
    status: pending
  - id: phase2-brush-preview-pipeline
    content: Add translucent world pipeline variant for brush placement ghost preview
    status: pending
  - id: phase2-reference-planes
    content: Add textured 3D quad pipeline + image texture loading for reference planes
    status: pending
  - id: phase3-render-wiring
    content: Parse and render vortices/attractions/target-volumes/references in world3d.rs render_world_3d
    status: pending
  - id: phase3-vortex-picking
    content: Add vortex hit-testing for brush hover/click
    status: pending
  - id: phase3-brush-flow
    content: Wire brush candidate preview + placement command flow end-to-end
    status: pending
  - id: phase3-relocate-connect
    content: Implement MVP object translate-drag with proximity/indirect connect
    status: pending
  - id: final-verify
    content: Full e2e + manual browser verification of all restored features against concrete-forest example
    status: pending
isProject: false
---

# Puzzle3D WGPU Feature Parity

## Scope decision (confirmed with user)

"Premigration" = the archived PlayCanvas puzzle3d renderer (`.repo/🎫/26/06/25/PUZZLE-3D-MESH-HOVER-STYLE/play-canvas-before.txt`), not the current React `world-3d-host.tsx` (which already dropped these features). This is a genuinely large feature-parity effort, not a small bug fix. Work is organized in phases; Phase 0 is a hard blocker for everything else.

## Phase 0 — Fix the empty viewport (blocking bug, do first)

Root cause, verified independently: `frustum_planes()` in [ui/wgpu/rs/scene3d.rs](ui/wgpu/rs/scene3d.rs) (around line 406) extracts frustum planes with transposed row/column indices against the column-major `Mat4` (`cols[col][row]`, confirmed by `transform_point` at line 119-123). For row `r`, the correct plane component at index `c` is `m[c][r]`, not `m[r][c]` as currently written. This produces mathematically wrong plane equations, which the per-instance frustum cull in [framework/renderer/wgpu/rs/world3d.rs](framework/renderer/wgpu/rs/world3d.rs) line 254-288 then uses to discard the only object in the scene, leaving `culled_draws` empty — an empty scene pass, hence the black viewport (matches the screenshot in `.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/screenshot-puzzle3d.png`). The existing test `frustum_contains_origin_box` (scene3d.rs test module) passes only because it uses a symmetric huge box under a symmetric default camera, which happens to survive the transposed formula — it does not catch this class of bug.

Fixes, all in `ui/wgpu/rs`:
1. **`frustum_planes`** (`scene3d.rs`) — rewrite the six plane-row extractions using `m[c][r]` (component `c` from column `c`, row `r`), matching `transform_point`'s established convention. Add a regression test with a small offset box and a non-trivial (translated/rotated-via-orbit) camera — both a case that must stay visible and a case that must be culled — since the existing symmetric test cannot detect this bug class.
2. **`Mat4::perspective`** (`scene3d.rs` line 80-90) — currently emits OpenGL-style NDC z ∈ [-1, 1]; WGPU (like D3D/Metal) expects z ∈ [0, 1] for its hardware clip/depth test. Adjust the two affected matrix entries (`cols[2][2]` and `cols[3][2]`) to the D3D/WGPU convention. Add a unit test asserting `transform_point` maps a point at `near` to z≈0 and at `far` to z≈1.
3. **`build_layer_batches`** (`ui/wgpu/rs/draw.rs` line 772-775) — currently skips a layer if it has no UI/vector content, silently dropping any `ScenePass3d` whose `layer_index` points at that layer. Include layers that are referenced by any `draw.scene_passes[..].layer_index` even when `ui_instances`/`vector_vertices` are empty, so 3D content never depends on an incidental background quad being pushed first.
4. **`apply_glb_bytes`** (`framework/renderer/wgpu/rs/world3d.rs` line 588-597) — uses `format!("url:{url}")` as the mesh id, but every other call site (`world3d_mesh_id_from_url`, `framework/plugin/rs/world3d_host.rs` line 27-36) uses `mesh:{slug}`. Fix so fetched GLBs actually bind to their instances (currently dead on arrival for any mesh using a real URL).

Verify: `cargo test -p ui_wgpu`, WASM rebuild, then the existing `.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts --plugin puzzle3d` (and `puzzle5d`, `lowpoly` for regression) should show real geometry, not just a colored background.

## Phase 1 — Data plumbing: fixture → scene payload

The current Rust engine (`puzzle/3d/rs/lib.rs`) and plugin (`puzzle/3d/plugin/rs/lib.rs`) already model attractions, vortices, target volumes, and brush/fill logic — none of it reaches the renderer today. `world3d_scene()` only carries camera/meshes/instances/selection.

1. **`Puzzle3dVortex`** (`puzzle/3d/plugin/rs/lib.rs` line 61-71) — add missing `radius: Option<f64>` field (present in `concrete-forest.3d.json` line 384 `"radius": 0.36` but currently dropped by serde).
2. **`Puzzle3dFixture`** (`lib.rs` line 122-138) — add `references: Vec<Puzzle3dReference>` (new struct: `id`, `source: {url, media_kind}`, `origin: [f64;3]`, `width_world: f64`, `locked: bool`, `hidden: bool`) matching `concrete-forest.3d.json` line 708-724 — this data is silently dropped today.
3. **`World3dScene`** (`framework/core/rs/ui.rs` line 618-625) — extend with optional JSON fields: `vortices_json`, `attractions_json`, `target_volumes_json`, `references_json`, `brush_preview_json` (all `#[serde(default)]` so other plugins/components are unaffected).
4. **`world3d_scene()`** (`framework/plugin/rs/world3d_host.rs` line 95-107) — extend signature/builder to accept the new optional JSON strings.
5. **puzzle3d plugin** (`puzzle/3d/plugin/rs/lib.rs`) — add builder functions alongside `world_instances_json`/`world_meshes_json` (line 294-327):
   - `world_vortices_json`: flatten `object.vortices` into world-space records `{fullId, objectId, vortexKind, position, direction, radius, color}` (color resolved from `meta.kindCatalogs.vortices[].color`), with object transform applied (origin + orientation quaternion).
   - `world_attractions_json`: resolve each `Puzzle3dAttraction.attracting`/`attracted` (vortex full-ids) to world positions for line endpoints.
   - `world_target_volumes_json`: pass through `origin/orientation/scale` per target volume.
   - `world_references_json`: pass through reference records.
   - Wire all four into the `world3d_scene(...)` call in `render()` (line 763-772), plus a brush-preview JSON populated from `Puzzle3dPrecomputeSession` (Phase 3).

## Phase 2 — New WGPU render primitives

All in `ui/wgpu/rs`:

1. **Line pipeline for 3D space** (`shaders.rs` + `draw.rs`) — add `WORLD3D_LINES_SHADER`: same `Globals{view_proj, light_dir}` uniform/bind-group-layout as `WORLD3D_SHADER` (reuse `world_bind_group_layout`/`WorldGlobalsRing`, no new bind group needed), vertex format `{position: vec3, color: vec4}`, `PrimitiveTopology::LineList`, no depth write but depth test enabled so lines occlude correctly behind objects. Used for attraction links and target-volume wireframe edges (12 edges per box, computed on CPU from origin/orientation/scale).
2. **Vortex markers** — reuse the existing `world_pipeline` instance path: a small built-in sphere/octahedron primitive (add a `mesh_from_kind("vortex-marker")` case or a tiny inline mesh constant), instanced with translation-only model matrices and per-vortex color, drawn as an additional `SceneDraw3d` inside the same `ScenePass3d`. No shader changes needed.
3. **Brush preview ghost** — a second instance draw using the candidate mesh with a translucent tint; needs an alpha-blended variant of the world pipeline (`world_pipeline` currently uses `BlendState::REPLACE` — add a second pipeline `world_pipeline_translucent` with `BlendState::ALPHA_BLENDING` and `depth_write_enabled: false`, sharing the same shader/layout).
4. **Reference image planes** — new `WORLD3D_TEXTURED_SHADER` (position transformed by `view_proj`, uv sampled from a texture) + pipeline, plus a small texture-loading path in `ui/wgpu/rs/gpu.rs`/`draw.rs` (`RasterTextureStore` already loads raw pixel buffers via `ensure_raster` — extend or add a sibling store for 3D-plane textures fed by decoded image bytes fetched the same way GLBs are fetched in `framework/renderer/wgpu/rs/world3d.rs`). This is the single largest new subsystem in this plan (image fetch + decode + GPU upload + world-space quad draw) — flag as separable/optional if scope needs to be trimmed, since `concrete-forest.3d.json`'s references are `hidden: true`/background floor-plan aids, not core puzzle-solving visuals.
5. **`ScenePass3d`** (`scene3d.rs` line 388-396) — extend with `line_draws: Vec<LineDraw3d>` (position+color vertex list) and keep vortex/brush-preview as additional `SceneDraw3d` entries so the existing watermark/interleaving logic (Phase-0-fixed) covers them for free.

## Phase 3 — world3d.rs: parse, render, and pick

All in `framework/renderer/wgpu/rs/world3d.rs`:

1. **`World3dState`** — add fields for parsed vortices/attractions/target-volumes/references (mirroring the existing `scene_*_json` cache-and-diff pattern at line 76-79/122-136) so unchanged data is skipped, matching existing perf discipline.
2. **`render_world_3d`** — after building `culled_draws` (line 254-289), also build: attraction line segments (vortex→vortex), target-volume wireframe edges, vortex marker instances, and (if references phase is included) reference plane quads — push them into the extended `ScenePass3d`.
3. **Vortex hit-testing** — extend `pick_instance_at` (line 522-542) or add a sibling `pick_vortex_at` using `ray_aabb_slab`/sphere-ray test against each vortex world position + radius, for brush-target hover and click.
4. **Brush tool integration** — when `state` (or a new `active_tool` field threaded from the plugin's `setActiveTool`) is `"brush"`: on hover over a vortex, call into `Puzzle3dPrecomputeSession::brush_candidates` (already exposed, `puzzle/3d/rs/lib.rs` line 1390/1549) via a WASM-safe path from the plugin (the plugin already owns the `Puzzle3dPrecomputeSession`; renderer needs the *result* — likely the plugin computes preview JSON in `handle_command`/`render` and passes it as `brush_preview_json`, avoiding giving the generic renderer knowledge of puzzle3d's engine). On click, existing `addBrushObject` command path (`puzzle/3d/plugin/rs/lib.rs` line 689-703) already applies the placement — just needs the renderer to emit that command with the right `BrushPlacePayload` (target vortex, candidate kind) instead of it being unreachable today.
5. **Object relocate + connect gestures** — implement MVP single-axis-free translate drag (screen-space delta projected onto the ground plane through the dragged object's current Z, no full 3-axis gizmo initially — flag full gumball as a stretch/follow-up): on drag end, if the new position brings a vortex within `proximityRadius` of a compatible vortex (reuse engine's `vortices_attraction_compatible_for_drag`/host-accepts logic, again invoked plugin-side), auto-create the attraction (proximity connect) or require an explicit indirect-connect drop target (indirect connect) — both call a new plugin command (e.g. `worldRelocate`/`worldConnect`) mirroring the old `onRelocate`/`onIndirectConnect`/`onProximityConnect` callbacks.
6. **Fill visualization** — no new rendering needed: `setFillCount` (`puzzle/3d/plugin/rs/lib.rs` line 704-719) already returns an updated fixture whose new objects/attractions flow through the normal instance/attraction pipeline once Phases 1-2 land.

## Explicit non-goals / deferred (flag during execution, not silently dropped)

- Full multi-axis drag gumball (visual axis handles) — MVP is free-drag-on-ground-plane; a true gizmo is a follow-up.
- Fixture drag-and-drop from the OS file system onto the canvas — DOM-level concern in `framework/renderer/wgpu/js/index.ts`, unrelated to the render bug, lowest priority.
- Kind-catalog hover cross-highlighting from the left panel into the 3D view — needs a hover-state channel between `scenes.rs` panel rendering and `world3d.rs`; deferred after core visualization lands.

## Verification plan

- `cargo test -p ui_wgpu` and `cargo test -p semio-framework-renderer-wgpu` after each phase.
- WASM rebuild (`bun ./framework/renderer/wgpu/script.ts wasm`) + `.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts --plugin puzzle3d` for visual regression, plus a manual browser check (via the `cursor-ide-browser` MCP tools) confirming: object renders, vortex markers visible at both concrete-forest vortex positions, reference plane toggle-able (once implemented), brush tool shows a ghost preview and places on click, dragging an object near a compatible vortex creates a visible attraction line.
- Re-run `puzzle5d` and `lowpoly` E2E smoke (they share the same wgpu world3d path) to catch regressions from the shared-pipeline changes.
