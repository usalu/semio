# 🦴️ API: weighted straight skeleton and pitched roof surfaces (r7, label `z-depth`)

Crate `semio-framework-geometry` (`use semio_framework_geometry::{skeleton, roof}`), files
`🧰️framework/🔨️modules/📐️geometry/🦴️skeleton/🦀️.rs` and `…/🏠️roof/🦀️.rs`. Metres and radians, f64, counter-clockwise positive, no
`unsafe`, no panics on valid input, deterministic, `wasm32-wasip2` checked. Domain-neutral: nothing in it names a building.

## 1. `skeleton` — the weighted straight skeleton

Every input edge moves inward (to its left once the outer ring is counter-clockwise and the holes clockwise; the module normalises
either orientation) at its own constant `speed` per unit of time. Wavefront vertices ride the bisecting velocity that keeps them on
both adjacent edge lines; the skeleton is the trace of those vertices. Time read as height and `speed = 1 / tan(pitch)` is a hipped
roof; `speed = 0` is a vertical edge (a gable end); all speeds equal is the classic straight skeleton.

```rust
pub struct Ring { pub points: Vec<Point>, pub speeds: Vec<f64>, pub tags: Vec<u32> }   // edge i = points[i] -> points[i+1]
Ring::uniform(points, speed)

pub fn straight_skeleton(rings: &[Ring]) -> Result<Skeleton, SkeletonError>                       // ring 0 = outer, others = holes
pub fn straight_skeleton_until(rings, horizon: Option<f64>, control: &mut dyn FnMut() -> bool)    // stop at a time; false = cancel

pub struct Skeleton { nodes: Vec<SkeletonNode{point,time}>, arcs: Vec<SkeletonArc{from,to,reflex}>, faces: Vec<SkeletonFace>,
                      wavefront: Vec<WavefrontRing{points,speeds,tags}>, input_nodes, horizon }
pub struct SkeletonFace { ring, edge, tag, speed, nodes: Vec<u32> }    // the region swept by input edge (ring, edge), CCW node ids
Skeleton::face_area(&face)   Skeleton::peak()
```

* **Nodes** `0..n` are the input vertices at time 0 (`input_nodes[ring][vertex]` maps an original vertex to its node); the rest are
  event points `(x, y, time)`. Coincident event points (within `1e-9 * extent`, positions and times) share one node, so the arcs
  and faces are a proper planar graph.
* **Faces**: exactly one per input edge, indexed by the ORIGINAL ring and edge (orientation normalisation is hidden), tagged with the
  caller's `tag` and the edge `speed`. For `speed == 0` the face is the vertical gable end: `[a, b, top nodes from b back to a]`,
  heights = node times, plan width below `VERTICAL_SPEED = 1e-8` of drift per unit height.
* **Arcs**: `reflex = true` is the trace of a reflex vertex (a valley of a roof). Overlapping or T-touching arcs are noded.
* **Horizon**: `straight_skeleton_until(rings, Some(h), …)` stops at time `h`; the faces are closed by the wavefront pieces at `h`
  and `wavefront` holds the rings left, oriented so they can be fed back as input `Ring`s (this is how a mansard is built).
* **Cost / cancellation**: O(n^2 log n) in the edge count (200 edges: well under 0.3 s in debug); `control` is polled once per event,
  return `false` to get `SkeletonError::Cancelled` (no partial result).
* **Errors** `SkeletonError`: `NoRings`, `TooFewVertices`, `NonFinite`, `InvalidSpeed` (negative / non-finite), `DegenerateEdge`
  (zero length), `Spike` (zero-width spike at input), `SelfIntersecting` (also: rings touching or crossing), `SpeedMismatch` (two
  collinear adjacent edges with different speeds: the vertex between them has no velocity), `Cancelled`, `NoConvergence` (event cap).
* **Degeneracies handled**: simultaneous events (squares, plus/H/T shapes, orthogonal staircases), zero-width whiskers (a frozen
  vertex between antiparallel edges retracts immediately), rings that shrink to two vertices, reflex vertices hitting a hole.
* **Known limits** (documented, tested as refusals or as continuity only): straight edges only (flatten bulged loops first);
  a vertical edge next to a reflex corner is a legal skeleton but not a physical roof (the reflex vertex slides along the wall line
  into the footprint and the surface steps; the roof inference should avoid choosing such gable ends — see the handoff);
  collinear neighbours must share their speed.

## 2. `roof` — hip, gable and mansard surfaces

```rust
pub enum Roof { Hip{pitch}, Gable{pitch, ridge_direction}, Mansard{lower_pitch, upper_pitch, break_height}, Pitched{pitches: Vec<Vec<f64>>} }
pub const VERTICAL: f64 = FRAC_PI_2;            // pitch of a gable end;  GABLE_END_TOLERANCE = 0.02 (sine, ~1 degree)
pub fn roof_surface(footprint: &[Vec<Point>], roof: &Roof) -> Result<RoofSurface, RoofError>
pub fn roof_surface_controlled(footprint, roof, control)                    // cancellable
pub fn gable_pitches(rings, pitch, ridge_direction, tolerance) -> Vec<Vec<f64>>   // pitch per edge; VERTICAL across the ridge

pub struct RoofSurface { faces: Vec<RoofFace{ring,edge,zone,vertical,vertices:[x,y,z]}>, lines: Vec<RoofLine{from,to,kind}>, height }
RoofLineKind::{Ridge, Hip, Valley, Verge, Break}
RoofSurface::{mesh(), shell(thickness), plan_area(), surface_area()}
```

`footprint` = outer ring then holes, either orientation, straight edges. Heights are relative to the footprint plane (the caller
adds the eave elevation); a pitch must lie in `(0, pi/2]` (`RoofError::InvalidPitch` otherwise; `InvalidBreak` for a mansard whose
break height is not positive). Faces carry `(ring, edge)` of the FOOTPRINT edge they rise from, also for the second zone of a
mansard (`zone 1`). A mansard whose skeleton closes below the break is a plain hip (no zone-1 faces).

* `Hip`: every edge the same pitch. `Gable`: edges across `ridge_direction` (within tolerance) vertical, the rest pitched — right
  for footprints whose ridge-perpendicular edges are real outer ends (rectangles, L/U with one ridge direction per call). For
  cross gables or any other choice use `Pitched` with `VERTICAL` on exactly the edges the author wants as gable ends.
* `mesh()`: sloped faces facing up and gable ends facing outward (triangulated with the framework triangulator).
* `shell(thickness)`: the sloped faces thickened vertically downward into a CLOSED solid (top, bottom, a vertical wall on every
  boundary edge: eaves and verges); `volume == thickness * plan_area()`. A roof layer `j` is `shell(t_j)` translated down by the
  sum of the thicknesses above it. Gable ends (vertical faces) are not roof material; their polylines give the top profile for the
  wall below.
* `lines`: ridges (horizontal), hips (descend from ridge/apex), valleys (reflex traces), verges (sloped edge over a gable end),
  breaks (the crease between the two slopes of a mansard, at `break_height`).

## 3. Fixtures and third-party validation

* Fixtures (language agnostic): `…/📐️geometry/🧫️fixtures/🦴️skeleton/🔣️.json` (schema `…/🦴️skeleton/🧬️schema/🔣️.json`), 54 cases:
  15 hand shapes (rectangle, square, triangle, L, U, T, plus, H, comb, arrow, frame, off-centre hole, L with hole, two holes,
  clockwise input), 3 weighted convex polygons (one with a vertical edge), 24 random star polygons, 12 random orthogonal
  histograms. Each carries its expected per-edge face areas, the peak and the unswept area at nine times.
* Oracles (never our code): `py_straight_skeleton` 0.1.0 (an independent straight-skeleton implementation; new TEST dependency in
  `pyproject.toml` group `test`, locked in `uv.lock`, never a runtime dependency) for face areas; `shapely` mitred buffer for the
  unswept area where the buffer topology equals the wavefront (it differs on 28 of 300 random stars, where the skeleton library
  decides); `shapely` half-plane intersection for weighted convex polygons.
  Python: `…/🧪️tests/🦴️skeleton-oracles/🐍️.py` (104 tests; the fixture generator `r7-z-depth-skeleton-fixtures.py` imports it).
  Rust: `skeleton` module tests (22) and `roof` module tests (12).
* Result of the cross-check of 300 random stars: our faces equal `py_straight_skeleton` on 290, the library fails on 1 and differs on 9, where `shapely`
  agrees with us (the generator discards any library answer that breaks the partition / plane invariants); the 28 stars where `shapely` differs are
  the buffer semantic (library agrees with us). Hence every case in the fixture is confirmed by at least one oracle, 49 of 54 by two.
* Commands: `cargo test --manifest-path 🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/Cargo.toml` (166 lib + 9 + 5 tests),
  `--target wasm32-wasip2 --lib` check, `.venv/Scripts/python.exe -m pytest -p no:cacheprovider "<…/🦴️skeleton-oracles/🐍️.py>"`.

## 4. Using it from the BIM roof solids

```rust
let rings: Vec<Vec<Point>> = loops_to_points(&roof.footprint_eave /* footprint grown by overhang, straight edges */);
let roof = match shape { Hip{pitch} => Roof::Hip{pitch}, Gable{pitch, ridge_direction} => Roof::Gable{pitch, ridge_direction},
                         Mansard{lower_pitch, upper_pitch, break_height} => Roof::Mansard{..}, Shed/Flat => (existing planes) };
let surface = roof_surface_controlled(&rings, &roof, &mut || !cancelled())?;       // heights relative to the eave plane
for (j, layer) in layers.iter().enumerate() { let solid = surface.shell(layer.thickness).translated([0, 0, eave_z - above]); … }
```

Error mapping for diagnostics (F14): `RoofError::InvalidPitch|InvalidBreak` -> `roof.invalid-pitch`; `Skeleton(SelfIntersecting|
DegenerateEdge|Spike|TooFewVertices)` -> `roof.degenerate-footprint`; `Skeleton(Cancelled)` -> cancellation; everything else ->
`roof.fallback-flat.skeleton`. Curved footprints (any `bulge != 0`) still fall back to the existing planes or a flattened ring
(`loops::flatten` within the display tolerance) — the skeleton has no arcs.
