# 📓️ W1-E — Exact analytic primitives

Files touched: `✳️brep/🧬️schema/🔺️diff/🧱️primitives/🦀️.rs` (full rewrite of `make_sphere`/
`make_cylinder`/`make_cone`/`make_torus`/`make_convex_hull`, new helpers, new tests) and
`✳️brep/🧬️schema/⚙️engine/🦀️.rs` `#region 🔖️SyncApi` (4 one-line edits: dropped the trailing
`segments` argument from the `make_sphere`/`make_cylinder`/`make_cone`/`make_torus` calls in
`sphere_prim_sync`/`cylinder_prim_sync`/`cone_prim_sync`/`torus_prim_sync`; `BrepKernel` trait and
its public `_prim`/`_prim_sync` signatures were already segments-free, so no trait edit was
needed). `make_box` untouched.

## Representation chosen, per primitive

All four use `Frame3::WORLD`-aligned frames for their main analytic surface and hand-built
(not `Frame3::from_normal`-derived) frames for every circle/line edge, chosen so p-curves reduce
to simple affine maps (usually `p = t` directly) instead of needing runtime trig fitting.

### Sphere — ONE face, one seam (used twice) + two degenerate pole edges

Chose the OCCT-style single-seam sphere over the old two-hemisphere+equator split, so sphere joins
cylinder/cone/torus' own "one seam" convention (**coordination note for W1-F**: if the classifier/
tessellator special-cases sphere topology anywhere, it should expect this shape, not two
hemisphere faces).

- 2 vertices (south pole, north pole).
- 3 edges: `e_seam` (meridian great-circle arc, curve domain `[-π/2, π/2]`, `v0=south, v1=north`),
  `e_north`/`e_south` — **degenerate**: `v0==v1==pole`, `curve = Curve3::Line{origin: pole, dir:
  Vec3::ZERO}` (a standard OCCT device: the whole `u` range collapses to one point, encoded here
  as a constant-valued line since the schema has no dedicated "degenerate" flag).
- 1 face (`Surface::Sphere`), loop = `[seam@u=0 (fwd), north-deg (fwd), seam@u=2π (rev), south-deg
  (rev)]` — the seam edge used twice closes the `(u,v)=(0,2π)×(-π/2,π/2)` rectangle.
- **Euler**: naive `V−E+F = 2−3+1 = 0`. Degenerate edges are a schema artifact, not real
  1-cells, so the convention here (and the test `euler_excluding_degenerate`) **excludes edges
  where `v0==v1` and the curve is a zero-direction `Line`** from the `E` count: `2−1+1 = 2`,
  matching every other genus-0 solid. `is_degenerate_edge` (pub(crate)) implements this predicate.

### Cylinder / Cone — topology unchanged, only added p-curves + dropped `segments`

The pre-existing topology (3 faces for cylinder: lateral + 2 caps; 2 for cone: lateral + base) was
already exact/analytic and already used a single seam — nothing to restructure. Cone: **no
separate degenerate apex edge** — the one seam edge (base→apex) is used twice (up, down) in the
lateral loop, both traversals sharing the apex vertex as an endpoint, which already closes the
`v=0` (apex) side of the parameter rectangle without needing a third, degenerate edge (unlike the
sphere, whose two poles are NOT both endpoints of one edge).

### Torus — ONE face, fundamental-polygon topology (two seams, each used twice)

Replaced the old major×minor triangulated grid with the textbook "torus = square with opposite
sides glued" identification: `e_meridian` (u=0 tube cross-section circle) and `e_equatorial` (v=0
main circle), each used twice, all four coedges sharing the single vertex `(u,v)=(0,0)`.
`V=1, E=2, F=1`, no degenerate edges, `χ = 1−2+1 = 0` (genus 1 — the existing test's own
expectation, kept as-is).

### Convex hull — coplanar-triangle merge, planar polygon faces

`merge_coplanar_triangles`: buckets Quickhull's triangles by a `1e-7`-quantized `(normal, d)` key,
then for each bucket cancels every directed edge against its in-bucket opposite-direction twin
(present exactly when two triangles in the bucket share that edge) and walks what's left into one
ordered boundary loop. A box's 8-corner hull now yields 6 quad faces (`V=8,E=12,F=6,χ=2`), not 12
triangles; a tetrahedron (no coplanar pairs) is unchanged (`V=4,E=6,F=4`). Hull faces do **not**
carry p-curves (same as `make_box`'s planar faces — validation only requires a p-curve on
non-planar surfaces) — flagged here rather than silently scoped out. `solid_from_triangle_soup` is
untouched and still used only for mesh import / as the (now less likely to be needed) hull
fallback path — nothing else in this file calls it.

## p-curve convention (binding on every consumer, not just this file)

Confirmed against the ALREADY-LIVE validator (`✅validation-report/🦀️.rs`
`check_same_parameter`, lines ~191–222): it samples `s∈[0,1]`, maps `p = prange.0 +
(prange.1−prange.0)·s` and, **independently**, `t = edge.range.0 + (edge.range.1−edge.range.0)·s`,
and checks `surface.eval(pcurve.eval(p)) ≈ curve3.eval(t)`. **`forward` never enters this
calculation.** So: a p-curve is always parametrized in the edge's OWN curve order — never
reparametrized/reversed to match a particular coedge's traversal direction. When the SAME edge is
used twice in one loop (every seam here), BOTH coedges get the identical `(pcurve, prange)`
(cylinder/cone actually reuse the same `Curve2Id` for both). This is the standard STEP/OCCT
PCurve+SameSense model — **W1-F/W1-G**: any boundary-walk consumer (tessellation, the
`loop_uv_polygon`-style boundary sampler) needs to read a coedge's pcurve **backwards** (from
`prange.1` to `prange.0`) when `coedge.forward == false`, to get a continuous physical trace.

Cap p-curves (cylinder bottom cap, cone base) use `Curve2::Circle`, not `Curve2::Line`: the cap's
plane frame is a deliberate reflection of the circle's own frame (`x` kept, `y`/`z` negated) so the
mapping is the closed form `p = −t` (`prange` reversed relative to `edge.range`, e.g. `(0, −2π)`)
— documented inline at each cap in the source.

## ⚠️ Finding for W1-F (not fixed here — out of file-ownership scope)

`💡️inferences/📏mass-properties/🦀️.rs`'s `loop_positions`/`loop_uv_polygon` (used by
`face_area`/`face_volume_contribution`/`solid_volume`/`solid_surface_area` for every non-planar-or-
polygonal-with-few-vertices face) sample **one point per coedge — its start vertex only** — never
along the p-curve/edge curve. For a straight-edge polygon (box, hull) that's exact; for ANY curved-
edge loop with few vertices (every lateral/cap/spherical/toroidal face built here) it produces a
degenerate 1–4-point "polygon" (e.g. the cylinder's 4-coedge lateral loop samples only 2 distinct
UV points — `v_bot`'s and `v_top`'s — since 2 of its 4 coedges share each vertex), so
`point_in_uv_polygon` is unreliable and quadrature under/over-integrates. `try_analytic_sphere_
volume` (a `Surface::Sphere`-only special case) sidesteps this for sphere volume specifically and
still matches my one-face representation trivially. Everything else routes through the generic
path. **Suggested fix (not mine to make): sample N points along each coedge's p-curve (respecting
`forward`, see above) instead of just its start vertex.**

## Test results (verbatim, foreground)

`cd /Users/ueli/Documents/semio && RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --lib
--message-format short` — heavy concurrent-build contention from other wave-1 workers checking the
same crate simultaneously (8+ parallel `cargo check -p semio-s-plugin-stdio` processes observed via
`ps aux`); see `🗑️generated/w1e-check.txt` for the raw log. Result: **[FILL IN AFTER RUN]**

Unit tests: harness not yet READY per `📓️h0-harness.md` (file did not exist at time of writing) —
`🔬️harness/` scaffolding (Cargo.toml/lib.rs/Cargo.lock/.cargo) exists but no readiness note was
posted. Re-run `cd 🔬️harness && RUSTC_WRAPPER="" cargo test -- primitives` (plus existing boolean/
offset/sweep tests) once ready; results will be appended here, not fabricated.

## ⚠️ Breaking change for another worker's file (not fixed here — out of scope)

`💡️inferences/🏷classification/🦀️.rs` (W1-F-owned) calls the free functions `make_sphere`/
`make_cylinder` directly (not through the trait) with the old `segments` argument, at:
- line 519: `make_sphere(&mut body, r, 24, &mut rec)` → drop the `24,`
- line 539: `make_cylinder(&mut body, radius, height, 32, &mut rec)` → drop the `32,`

Trivial 2-line fix, left for W1-F since it's their file.
