---
name: Marquee Crossing Window Selection
overview: Restore the canonical drag-direction selection convention (drag left-to-right = window/total inclusion, drag right-to-left = crossing/partial inclusion) for the 3D world marquee (mesh, vertex, edge, face granularities), which is currently missing entirely after the WGPU migration.
todos:
  - id: kernel-helpers
    content: Add marquee_is_crossing/marquee_is_crossing_from_path + segment intersection helpers in kernel/3d/scene/rs/lib.rs
    status: completed
  - id: kernel-components
    content: "Extend screen_select_components with crossing param: edge (both endpoints) and face (all 3 vertices) window vs crossing tests"
    status: completed
  - id: kernel-instances
    content: "Extend screen_select_instances with crossing param: window = all mesh vertices inside; upgrade crossing per-triangle test to vertex/edge based"
    status: completed
  - id: world-thread-crossing
    content: Extend marquee_local_polygon to compute and return crossing; thread through update_marquee_preview and marquee_select_command
    status: completed
  - id: world-visual
    content: "Optional: vary marquee overlay fill alpha by crossing, matching node-graph convention"
    status: completed
  - id: tests
    content: Add/extend tests in kernel_3d_scene and infinite_world for direction helpers and window-vs-crossing selection
    status: completed
  - id: build-verify
    content: Run cargo tests, rebuild WGPU wasm, browser-verify both drag directions, open/close ticket
    status: completed
isProject: false
---

## Root cause

The 3D world marquee selection (`infinite/world/rs/lib.rs` + `kernel/3d/scene/rs/lib.rs`) has **no drag-direction concept at all**. It always does a single overlap-style test regardless of whether the user drags left-to-right or right-to-left, so "total inclusion" (window) never differs from "partial inclusion" (crossing).

This distinction exists elsewhere in the codebase and was **lost for the 3D world during the WGPU port**:
- `gis/2d/rs/lib.rs` `features_in_rect_json`/`features_in_polygon_json` take a `crossing: bool` and branch: window mode requires `screen_pts.iter().all(...)` (full containment), crossing mode requires intersection.
- `framework/renderer/wgpu/rs/lib.rs` has `map_marquee_crossing(method, start_x, end_x) -> bool { end_x < start_x }`, i.e. drag right-to-left = crossing.
- The pre-migration reference implementation (recovered via `git show 5ecbe3dbfb^:puzzle/3d/react/index.tsx`, since `puzzle/3d/react` and `ui/react` were deleted when these apps were rewritten in Rust) defines the exact canonical algorithm:
  - `marqueeIsCrossing(startX, endX) = endX < startX`
  - `marqueeIsCrossingFromPath(path, method)`: for lasso, walks the path for the first point whose `|dx| >= 2px` and returns `dx < 0`; otherwise falls back to comparing first/last point x.
  - `marqueeCandidateSelected`: crossing → hull intersects marquee (any point inside, or any edge crosses the boundary); window → **every** hull point must be inside the marquee.

## Design

Add a `crossing: bool` parameter alongside the existing `rectangle: bool` parameter (mirrors the existing `gis/2d` convention rather than a full polygon-unification rewrite, minimizing risk).

### `kernel/3d/scene/rs/lib.rs` (pure geometry, testable)

1. Add direction helpers matching the recovered reference 1:1:
```rust
pub fn marquee_is_crossing(start_x: f32, end_x: f32) -> bool {
    end_x < start_x
}
pub fn marquee_is_crossing_from_path(path: &[[f32; 2]], is_lasso: bool) -> bool {
    // lasso: first point with |dx| >= 2px decides; else fall back to start/end
}
```
2. Add segment-intersection primitives (needed for correct edge/face/window crossing tests, not currently present):
```rust
fn segments_intersect(a0, a1, b0, b1) -> bool
fn segment_intersects_rect(a, b, rect: [f32; 4]) -> bool
fn segment_intersects_polygon(a, b, polygon: &[[f32; 2]]) -> bool
```
3. Extend `screen_select_components(..., active_instance_id, crossing: bool)`:
   - `vertex`: unaffected (a point cannot be "partial" - matches reference, which treats point candidates identically either way).
   - `edge`: project **both endpoints** (currently only the midpoint is projected). `window` requires both endpoints inside; `crossing` requires either endpoint inside OR the segment crosses the marquee boundary.
   - `face`: project **all 3 triangle vertices** (currently only the centroid). `window` requires all 3 inside; `crossing` requires any vertex inside OR any triangle edge crosses the boundary.
4. Extend `screen_select_instances(..., crossing: bool)`:
   - Keep the existing AABB broad-phase + per-triangle refine for `crossing` (upgrade the per-triangle test from "centroid inside" to "any vertex inside OR edge crosses boundary", reusing the new segment helpers).
   - Add `window` mode: instance selected only if **every** mesh vertex projects inside the marquee (iterate `mesh.positions` once - cheaper and more exact than an AABB-corner approximation, and reuses geometry already available).

### `infinite/world/rs/lib.rs`

1. Extend the `marquee_local_polygon(state, rect)` helper (added earlier this session) to also return `crossing`, computed from the **raw global** `state.marquee_points` (direction is translation-invariant, no need to localize first) via `marquee_is_crossing_from_path(&state.marquee_points, state.selection_method == "lasso")`.
2. Thread `crossing` through both call sites: `update_marquee_preview` and `marquee_select_command`, passing it to `screen_select_components(...)` and `screen_select_instances(...)`.
3. Optional visual parity with the node-graph marquee (`paint_node_graph_selection_marquee` already varies fill alpha 0.08 crossing / 0.12 window - note: neither this app nor `gis/2d` implement dashed strokes, so no new dash primitive is needed): add a translucent `ctx.draw.push_solid_overlay(...)` fill under the existing border strokes in the marquee-paint block (~line 1697-1716), alpha varying by `crossing`.

### Tests (extend existing test modules only, no new files)

- `kernel/3d/scene/rs/lib.rs`: direction helpers (`marquee_is_crossing`, `marquee_is_crossing_from_path` incl. the lasso first-horizontal-step case), `screen_select_instances` window-vs-crossing on a partially overlapping instance, `screen_select_components` edge/face window-vs-crossing.
- `infinite/world/rs/lib.rs`: a drag-direction test asserting a partially overlapping vertex/edge/face is included when dragging right-to-left but excluded when dragging left-to-right (and vice versa for a fully enclosed one).

## Process

- Open a new ticket (this is a distinct concern from the just-closed `PER-COMPONENT-HOVER-AND-VERTEX-SPHERES` ticket) under goal `🎯r2602/🎯runningsketchpad`.
- `cargo test -p kernel_3d_scene -p infinite_world`, then rebuild WGPU wasm (`bun framework/renderer/wgpu/script.ts wasm`).
- Browser-verify in Vertex/Edge/Face and Mesh granularity: drag left-to-right over a partially overlapping selection → only fully-enclosed items are selected; drag right-to-left → partially overlapping items are also selected.
- Close ticket with summary and touched files.

## Out of scope

- GIS map (`gis/2d`) and node graph (`flow_core`/`framework/renderer/wgpu`) already implement the crossing/window distinction correctly and are not touched.
- No dashed-stroke rendering primitive will be added (matches existing precedent - neither GIS map nor node graph use dashes in the current WGPU renderer, only alpha variation).
