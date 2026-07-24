# Lower corner heads match negative-axis tips

## Request
Bottom (lower-hemisphere) corner hits should use the same small unlabeled circle as the opposite ends of the main axes, because under-views are less relevant than upper corners.

## Change
- **r3f** (`WorldProjectionGizmoViewport`): lower corners omit `label` so `WorldProjectionGizmoHitHead` uses radius 12 / scale 0.65 (same path as −X/−Y/−Z). Upper corners keep NE/NW/SE/SW labels and large heads.
- **wgpu** (`paint_world_orbit_view_gizmo`): lower corners empty label + `prominent=false` → solid tip radius 2; upper corners labeled + radius 3. Empty-label axis ends also get the small tip disk for parity.

## Verify
- `cargo check -p infinite_world` → `cargo-lower-corners.txt`
