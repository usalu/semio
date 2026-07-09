---
name: Fix CAD Interaction and Curve Bugs
overview: "Fix three verified root-cause bugs blocking CAD construction interactions end-to-end: pointer clicks never reach the engagement dispatcher, engagement preview items are never rendered, and curve/wall/column tube meshes fail to tessellate."
todos:
 - id: fix-pointer-gate
   content: Move engagementSessionActive branch ahead of the event.target===hostRef.current gate in handlePointerDown (world-3d-host.tsx)
   status: completed
 - id: add-preview-channel
   content: Add dedicated engagement_preview_json field to World3dScene (core/rs, plugin/rs world3d_scene_extended + all call sites, os-shell.tsx type)
   status: completed
 - id: wire-cad-preview
   content: Point cad/plugin/rs/lib.rs build_world_scene_for_pane preview output at the new engagement_preview_json slot
   status: completed
 - id: render-preview-layer
   content: Add WorldEngagementPreviewRecord parsing + EngagementPreviewLayer (point/segment/box-preview) rendering in world-3d-host.tsx
   status: completed
 - id: fix-curve-pipe
   content: Rewrite curve_mesh_from_wire in geometry_import.rs to pipe a real face profile (regular_polygon_wire_sync + planar_face_from_wire_sync) instead of a bare curve
   status: completed
 - id: extend-tests
   content: Extend existing cad-plugin/kernel tests to cover curve tube tessellation
   status: completed
 - id: verify-e2e
   content: cargo test, rebuild wasm, manual browser verification of box construction + structure-classic curves on hexagonal-cut-concrete-forest-left
   status: completed
 - id: reopen-ticket
   content: Reopen CAD-WGPU-PREMIGRATION-PARITY ticket before implementing, close with summary when done
   status: completed
isProject: false
---

# Fix CAD Interaction and Curve Bugs

## Diagnosis (verified against the running `bun run dev:cad` server on `:6020` with the CDP browser tools, plus direct code inspection)

### Bug A — clicks inside the 3D pane never advance the construction state machine

`framework/renderer/react/components/world-3d-host.tsx` `handlePointerDown` (~line 1377):

```tsx
if (event.button !== 0 || event.target !== hostRef.current) return;
if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
    ... dispatch("worldPointerDown", ...)
}
```

`hostRef` is the outer wrapper `<div>`; the actual click target is always the WebGL `<canvas>` that `WorldCanvas` mounts 4 DOM levels deep (`div > div > div > canvas`), which fills the div 100%. I confirmed this live: after starting the "Box" tool (engagement `Step: first_corner`), dispatching a real `pointerdown`/`pointerup` on the pane's canvas left `event.target.tagName === "CANVAS"` (never `=== hostRef.current`) and the step stayed frozen at `first_corner` — the `worldPointerDown` dispatch never runs.

### Bug B — construction preview (origin point / rubber-band box) never renders

`cad/plugin/rs/interaction.rs::preview_display_items()` computes `point`/`segment`/`box-preview` records and `cad/plugin/rs/lib.rs::build_world_scene_for_pane` serializes them into the `interaction_json` slot of `World3dScene` (via `world3d_scene_extended`). But that same slot is already used by the shared `World3dHost` component for puzzle-3d's unrelated `WorldInteractionRecord` (`activeTool`, `brushCandidateIndex`, `hoveredVortexFullId`). There is no code anywhere in `world-3d-host.tsx` that turns CAD's preview items into visible geometry — they're silently dropped.

### Bug C — Structure Classic (and any "curve" primitive) never shows a tube

`cad/plugin/rs/geometry_import.rs::curve_mesh_from_wire`:

```rust
let profile = kernel.circle_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.05).ok()?; // -> Entity::Curve
let solid = kernel.pipe_sync(&profile, wire, None).ok()?;
```

`pipe_sync` (`kernel/3d/brep/rs/lib.rs:636`) does `self.face_id(profile)?`, which only succeeds for `Entity::Face` (`kernel/3d/brep/rs/lib.rs:152-157`); a `Curve` entity always errors with `"... is not a face"`. So every wall/column/beam curve tessellation attempt fails and silently falls back to a generic bounding-box mesh in `object_mesh_data` — this is why Structure Classic shows boxes/nothing instead of real wall/column tubes. Confirmed the fixture (`cad/asset/play/hexagonal-cut-concrete-forest-left.model.json`) has 11 structure-classic objects, all single-`curve`-primitive walls/columns referencing simple 2-point line wires.

## Fixes

### 1. Unblock engagement pointer dispatch

In `handlePointerDown` (`framework/renderer/react/components/world-3d-host.tsx`), move the `selection.engagementSessionActive` branch before/independent of the `event.target !== hostRef.current` check, so any primary-button pointerdown inside the pane raycasts to the ground plane and dispatches `worldPointerDown` while a session is active. Leave the existing gate untouched for the non-engagement marquee/paint path.

### 2. Render CAD engagement preview items

Add a dedicated `engagement_preview_json` channel instead of overloading `interaction_json`:

- `framework/core/rs/lib.rs`: add `engagement_preview_json: Option<String>` field to `World3dScene`.
- `framework/plugin/rs/lib.rs`: append `engagement_preview_json: Option<String>` parameter to `world3d_scene_extended(...)`, thread into the struct; update `world3d_scene(...)` to pass `None`.
- Update the other two call sites to pass `None` for the new trailing arg: `puzzle/plugin/rs/d3/mod.rs`, `puzzle/plugin/rs/d5/mod.rs`.
- `cad/plugin/rs/lib.rs::build_world_scene_for_pane`: pass the existing `preview` value into the new `engagement_preview_json` slot (keep `interaction_json` as `None` for CAD).
- `framework/renderer/react/os-shell.tsx`: add `engagementPreviewJson?: string` to the `World3dScene` type.
- `framework/renderer/react/components/world-3d-host.tsx`: add a `WorldEngagementPreviewRecord` type + parser matching `preview_display_items`'s shape (`kind: "point" | "segment" | "box-preview"`, with `position`/`from`,`to`/`cornerA`,`cornerB`), and an `EngagementPreviewLayer` component rendering points as small spheres, segments as lines, and `box-preview` as a translucent/wireframe box between the two corners; mount it inside the canvas tree.

### 3. Fix curve/tube tessellation

Rewrite `curve_mesh_from_wire` in `cad/plugin/rs/geometry_import.rs` to build a real face profile before piping:

```rust
fn curve_mesh_from_wire(kernel: &mut BrepkitKernel, wire: &GeometryHandle) -> Option<MeshData> {
    let profile_wire = kernel.regular_polygon_wire_sync(0.08, 8).ok()?;
    let profile_face = kernel.planar_face_from_wire_sync(&profile_wire).ok()?;
    let solid = kernel.pipe_sync(&profile_face, wire, None).ok()?;
    let mesh = block_on(kernel.tessellate(&solid, 0.1)).ok()?;
    let _ = kernel.dispose_sync(&solid);
    let _ = kernel.dispose_sync(&profile_face);
    let _ = kernel.dispose_sync(&profile_wire);
    Some(mesh_from_indexed(&mesh.position, &mesh.normal, &mesh.index))
}
```

Extend the existing `cad-plugin` test module (`cad/plugin/rs/lib.rs` / `geometry_import.rs`) to cover this path directly (assert a 2-point wire produces a non-degenerate mesh) rather than adding a new test file, per repo conventions.

## Verification

- `cargo test -p kernel_3d_brepkit -p cad-plugin`.
- Rebuild the CAD wasm plugin and restart `bun run dev:cad`; reload `hexagonal-cut-concrete-forest-left`, click "Box" in the Shape pane, click twice inside the viewport, confirm `Step:` advances past `first_corner` and a preview marker/box shows; confirm Structure Classic now renders tube-shaped walls/columns.
- Work happens inside the existing (closed) ticket `.repo/🎫/26/07/06/CAD-WGPU-PREMIGRATION-PARITY` — reopen it via `ticket_reopen` before implementing, per repo convention.
