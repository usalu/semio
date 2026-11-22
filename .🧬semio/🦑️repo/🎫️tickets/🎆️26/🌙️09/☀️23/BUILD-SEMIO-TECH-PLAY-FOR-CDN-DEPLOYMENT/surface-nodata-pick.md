# Surface NoData pick failure

## Cause (hypothesis, not reproduced)
The browser test builds each query point from `terrain_tile_mesh_json` positions. JSON carries the shortest decimal of each f32 vertex coordinate, which JS parses to an f64 that differs from the f32 value by up to about 1e-7 relative. The Rust pick (`TerrainSession::pick_elevation_grid`) tests against the exact f32 vertices with a barycentric tolerance of `16 * f64::EPSILON` (about 3.5e-15). Queries on shared edges or vertices (xy [1,0.5], [2,1], [2,2]) therefore fall just outside both triangles in the browser. The native Rust test feeds exact f32 values, so it passes there. Fixture and test are self-consistent.

## Fix
`🏔️terrain/🎯️pick/🦀️.rs` `triangle_hit`: tolerance is now `4 * f32::EPSILON` (about 4.8e-7), matching the f32 mesh resolution. The gap query [1.5,0.5] is far from any triangle and stays null.

## Verification
Not completed. `repo ./📜️script.ts wasm` needs the release driver. `repo run ...:wasm:release` and `repo cargo test` failed on repo-tooling problems: a missing `@semio-tech/schema-validator-ts` package in an unrelated plugin package (other agents' concurrent edits), and an internal cache-owner error. Native cargo is refused by the compiler ingress. Rerun the surface-wasm job once the tooling is healthy.

## Root cause (reproduced natively)
New terrain unit test `json_published_mesh_points_pick_the_surviving_face_exactly` (`🏔️terrain/🧪️tests/🔬️unit/🦀️.rs`) mirrors the browser test: `set_project_origin`, `set_elevation_grid_json`, `terrain_tile_mesh_json(0,0,0)` positions parsed as f64 from JSON, bilinear point, `pick_elevation_grid_json` at z=3e6. Before the test fix it reproduced `z = 0.026397673413157463` exactly.

Hypotheses: (a) false, pick and JSON use the same tile/mesh. (c) false, `distance` is exact (2999999.9736 vs 3e6). (b) not a tolerance leak. Diagnostics showed the ray origin x = 17795.588 (JSON decimal, f64) while vertex 10 x = 17795.587890625 (f32), 1.1e-4 m east of the vertex. That point is strictly inside face [7,10,11] whose vertex 11 is 2,147,483.75 m high (slope about 240 m/m over 8.9 km), so the surface at that xy truly is 0.0264 m. Pick takes the highest hit (nearest from above), so it correctly returned 0.0264. The pick is exact; the test queried an f64 point that is not the f32 mesh vertex, so the slope amplified the 1e-4 m offset.

## Fix
Test inputs, not pick: query points are now rounded to f32 (the mesh and Three raycaster resolution), `as f32` in Rust and `Math.fround` in `🕸️browser/🟦️.ts`. Assertions unchanged (0.01 m). The pick's `4*f32::EPSILON` barycentric tolerance stays justified: f32-rounded points on shared edges differ from the f64 edge by about 1e-7 relative, so strict tolerance returned null hits. Pick source is unchanged (debug logging removed).

## Results
`repo cargo ... test --manifest-path 🏔️terrain/📦️packages/🦀️rust/Cargo.toml`: 30 passed, 0 failed. Browser wasm test not rerun here (same logic as the Rust test).
