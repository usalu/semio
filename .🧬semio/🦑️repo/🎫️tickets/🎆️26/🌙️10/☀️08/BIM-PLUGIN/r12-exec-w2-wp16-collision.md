# 💥️ r12 exec: w2-wp16-collision (framework mesh collision)

## Built
- `🧰️framework/🔨️modules/📐️geometry/💥️collision/🦀️.rs` (mounted `pub mod collision;` in `📦️packages/🦀️rust/🦀️.rs`; Rust path `semio_framework_geometry::collision`).
- Unit tests `💥️collision/🧪️tests/🔬️unit/🦀️.rs` (15), fixture `🧫️fixtures/💥️collision/🔣️.json` (13 cases, shared with the Rust unit test and the parry oracle; reusable by the BIM python oracle), parry3d oracle `🧪️tests/💥️collision-oracles/🦀️.rs` (5), `[[test]] collision_oracles` row in `📦️packages/🦀️rust/Cargo.toml`.

## API (as built)
```rust
#[derive(Clone, Copy, Debug, Default, PartialEq)] pub struct Aabb { pub min: Xyz, pub max: Xyz }
impl Aabb { point(p), from_points(&[Xyz])->Option, including(p), union(&Aabb), overlaps(&Aabb, margin:f64)->bool, distance(&Aabb)->f64, centre(), extents(), min_extent() }
pub fn mesh_aabb(mesh:&TriMesh)->Option<Aabb>
#[derive(Clone, Debug, Default, PartialEq)] pub struct Bvh { .. }   // median split on longest centroid axis, leaves <= 4 triangles, ties by index
impl Bvh { pub fn build(mesh:&TriMesh)->Self /*total; empty mesh -> empty Bvh*/; pub fn bounds(&self)->Option<Aabb>; pub fn triangle_count(&self)->usize }
pub fn triangle_contact(a:[Xyz;3], b:[Xyz;3]) -> Option<Vec<Xyz>>   // all contact points (segment ends, or coplanar overlap polygon vertices)
pub fn triangles_intersect(a:[Xyz;3], b:[Xyz;3]) -> Option<[Xyz;2]> // segment; equal ends = point touch; coplanar: the overlap polygon's two mutually farthest vertices
pub fn triangle_distance(a:[Xyz;3], b:[Xyz;3]) -> (f64, Xyz, Xyz)   // (0, m, m) with the contact-segment midpoint when intersecting
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum ClashKind { Hard, Clearance }
#[derive(Clone, Copy, Debug, PartialEq)] pub struct MeshClash { pub kind, pub distance:f64, pub point:Xyz, pub bounds:Aabb, pub pairs:usize }
pub fn clash(a:&Bvh, a_mesh:&TriMesh, b:&Bvh, b_mesh:&TriMesh, tolerance:f64, clearance:f64, cancel:&dyn Fn()->bool) -> Option<MeshClash>
```
Epsilons: plane/contact `1e-10`, hard threshold `max(tolerance, 1e-9)`, degenerate triangle = |cross| <= 1e-18 (-> `triangles_intersect`/`triangle_contact` None; `triangle_distance` still works on them).

## Semantics, with ONE deviation from the spec (BIM oracle must mirror it)
1. Intersection set = contact points of all intersecting pairs (coplanar overlap contributes all polygon vertices, not just the extreme segment) PLUS every vertex of either mesh strictly inside the other (parity +x ray with a fixed shift of (0, 2.718281828e-7, 3.141592653e-7)). Reason: for a wall sunk 1 mm into a slab the pure crossing set is a flat loop on the slab top (z extent 0), so the specified "smallest extent of the crossing AABB" would give 0 instead of 1 mm; adding embedded vertices makes the AABB equal to the overlap box for axis-aligned boxes (fixtures: expected -0.001). Extent = smallest AABB axis extent; Hard iff pairs>0 and extent > max(tolerance,1e-9): distance=-extent, point=AABB centre, bounds=AABB, pairs=count. pairs>0 but not hard (touching, within tolerance) returns None, also with clearance (distance 0).
2. pairs==0: parity ray from first vertex of each mesh against the other (closed meshes assumed); contained mesh -> Hard, extent = its mesh_aabb min extent, point = its centre, pairs 0 (no tolerance threshold).
3. clearance>0: BVH branch-and-bound (nearer child first, prune by AABB gap >= best, best starts at `clearance`); 0<d<clearance -> Clearance, point = midpoint of closest points, bounds = AABB of them. d >= clearance: None.
4. `cancel` polled once per BVH node visit in all phases; cancelled => None (indistinguishable from no clash; callers must check their own flag).

## Verification (all through the gate, `--manifest-path 🧰️framework/Cargo.toml`; the geometry crate is NOT a member of the root `Cargo.toml` workspace, the framework workspace is `🧰️framework/Cargo.toml`)
- `cargo test -p semio-framework-geometry` : lib 190 passed (15 collision), `aec_geometry_oracles` 9, `collision_oracles` 5, `first_party_geometry` 5, doc 0; 0 failed.
- `cargo check -p semio-framework-geometry --lib --target wasm32-wasip2` : OK, no warnings in collision.
- `bun T/r3-f1-check-names.ts`: no collision-related finding (5 pre-existing BIM DUPLICATE EMOJI problems of peers).
- Oracle facts (parry3d 0.17.6, f32): `query::distance` is WRONG for cuboid pairs separated on >1 axis (returns 1.36 instead of 0.5), so the oracle uses `closest_points`; its GJK triangle distance overestimates by up to ~3e-4, so tests assert `ours <= parry + 1e-4` and `parry - ours <= 1e-3`. Triangle intersection boolean: 4000 seeded random pairs agree (mismatch only tolerated below 1e-3 gap). Box pairs: 400 axis-aligned vs analytic overlap/gap (exact, 1e-9), 600 rotated vs parry cuboid intersection/closest points.
- Debug logs: none left; generated logs under `T/🗑️generated/w2-wp16-collision/` (all.txt, lib/or logs) to delete when the ticket closes.

## Open items
- Open (non-closed) meshes: containment/embedded-vertex steps assume closed meshes.
- Rotated-box hard-clash distance is a conservative proxy (checked only as negative and bounded), not the exact MTD.
