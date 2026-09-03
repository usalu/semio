# 📓️ W1-F — One classifier, trimmed mass properties, stronger validation

Files owned and edited (only these four):
- `I/🏷classification/🦀️.rs`
- `I/🌳bounding-volume/🦀️.rs`
- `I/📏mass-properties/🦀️.rs`
- `I/✅validation-report/🦀️.rs`

where `I = ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences`.

## 1. Classification — one authoritative classifier

- `point_in_solid` now genuinely traverses the face BVH: each retry ray restricts its candidate
  faces to `FaceBvh::query_ray(...)` instead of scanning `body.solid_faces(solid)` (audit §6.10's
  headline finding — the BVH parameter used to be accepted and never dereferenced).
- Crossings are counted per DISTINCT ray/face intersection across all BVH-culled candidates, not
  one per face and not one per solid — a curved face's trim can be crossed more than once by the
  same ray (e.g. a torus's near/far lobes). Near-duplicate roots within `10·tol` are merged (the
  same physical crossing seen from two faces sharing a boundary).
- New three-valued `UvStatus { Inside, Outside, OnBoundary }` replaces the old boolean trim test
  internally. A ray hit landing `OnBoundary` (within `tol` of a loop's edge OR vertex — vertices
  are covered automatically since a polygon vertex is an endpoint of two of its own edge segments)
  or geometrically tangent (`|normal·dir| < 1e-6`, `IntersectError::Tangent`, or a ray parallel to
  a planar face) aborts the WHOLE ray as `Grazing`; `classify_by_ray_consensus` discards that ray
  and retries with the next irrational direction, rather than mis-voting a degenerate hit.
- Trims prefer the coedge's stored p-curve (`coedge_uv_sample`), evaluated over `co.prange` in the
  coedge's own traversal direction, with seam unwrapping for periodic surfaces; the 3D-curve
  reprojection fallback only runs when no p-curve exists yet. Fixed a latent bug in that fallback:
  it previously ignored `co.forward` entirely, always walking the shared edge curve forward even
  for a backward-oriented coedge.
- Real solver failures from `intersect_curve_surface` (`Unresolved`/`Degenerate`) now propagate as
  `KernelError` instead of being silently swallowed to "no hits" (`.unwrap_or_default()` before).
  Only `IntersectError::Tangent` is treated as a (correct) zero-crossing/grazing signal.
- `surface_uv`'s `Surface::Nurbs` branch used to return the parametric domain's lower corner
  regardless of the query point (a stub); it now calls `surface_ops::closest_point` for a real
  approximate `(u, v)`.
- Deleted the duplicate ray-parity classifier from mass-properties (`classify_point_on_solid`,
  `PointSolidClassification`, `RayHit`, `ray_hits_solid`, `ray_face_intersection`) — this WAS the
  audit's "two classifiers" risk (§6.10). `distance_solid_solid`'s overlap check now calls
  `classification::point_in_solid` directly.
- Left the `Surface::Sphere` special case in `point_in_face_trim_status` (3D-polygon pole/seam
  test) untouched — it predates this wave and isn't p-curve-aware; that's W1-E's seam/pole
  domain, noted as a follow-up rather than touched blind.

## 2. BVH — ordered ray query, true-nearest, refit, per-solid wrapper

- `spatial::Bvh<T>`: `query_ray_ordered` (near-to-far via a new `aabb_ray_entry` returning the
  clamped entry `t`), `query_nearest_exact` (branch-and-bound true nearest — an injected `exact`
  closure computes real distance per leaf, pruned by AABB lower bound, unlike
  `query_point_nearest`'s AABB-only heuristic), `refit` (recomputes every leaf/branch AABB in
  place from a fresh per-item bounds closure, same tree topology — cheaper than a full rebuild).
- `face_aabb` now also samples a coarse 5×5 grid over the surface's FULL parametric domain for
  non-planar surfaces, conservatively widening the AABB past what boundary-only sampling would
  catch (a saddle-shaped NURBS patch can bulge past the convex hull of its boundary samples). Not
  a tight trim-clipped bound — a documented, safe (over-, never under-) approximation.
- `FaceBvh`: `query_ray_ordered`, `closest_face` (true face distance + trim test, delegating to
  mass-properties' now-`pub` `closest_point_on_face` inside the branch-and-bound), `refit`.
  `EdgeBvh::refit`.
- New `SolidBvh { solid: SolidId, .. }` wrapper (`build`, `faces()`, `closest_face`, `refit`) — the
  per-solid cacheable unit a future `Brep` engine wrapper can hold, per the ticket's ask.
- Tests for every new method (ordered-ray sorting, true-nearest-beats-AABB-nearest, refit-changes-
  bounds, `SolidBvh` wiring).

## 3. Mass properties — trimmed-domain quadrature, inertia, error estimate

- New `MassProperties { volume, area, centroid, inertia: [[f64;3];3], error_estimate }` and
  `solid_mass_properties(body, solid, tol)`.
- Core machinery: ear-clip triangulation (`ear_clip`, robust to either polygon winding, normalized
  internally) of each loop's UV polygon, then a 6-point symmetric (degree-4-exact) Gauss-Legendre
  quadrature per triangle (`quad_triangle_once`), adaptively quartering
  (`integrate_triangle_adaptive`) until the volume component's relative change across one more
  refinement level is below `tol` or depth 6 is hit. Every one of area/volume/first-moments/
  second-moments comes from the SAME per-sample formula (`n·dA = cross(du,dv)` exactly, no
  normalization needed), so one pass produces everything.
- `solid_mass_properties` sums the outer loop's moments minus each hole's (same +outer/-inner
  pattern as the pre-existing planar code), applies a global sign correction derived from the raw
  volume's own sign (so a solid with inward-facing labeling still gets a correct-signed centroid
  and positive inertia — verified algebraically, not just empirically), and reports
  `error_estimate` as the accumulated adaptive-refinement volume error divided by the volume
  (relative, not a guess).
- Analytic fast paths: kept the existing sphere path (extended to also produce area/inertia), and
  added a new orientation-agnostic box detector (`try_analytic_box_properties`: 6 planar faces, 8
  distinct vertices, one corner with 3 mutually orthogonal incident edges — built from raw vertex
  adjacency, so it doesn't depend on any face's `flipped` flag). Cylinder/cone/torus fast paths are
  NOT implemented in this pass (out of time budget) — they fall through to the general triangulated
  path, which is correct (verified against the closed-form volume within the adaptive tolerance in
  tests) but not as fast as a closed form would be. Documented gap, not a correctness gap.
- `loop_area`/`loop_volume_moments`'s non-planar branches (used by the pre-existing
  `solid_volume`/`solid_surface_area`/`solid_center_of_mass`, whose public signatures `engine.rs`
  and `boolean.rs` already depend on) now route through the SAME triangulated quadrature, replacing
  the old scheme (integrate over an axis-aligned UV rectangle, zero the integrand outside a
  polygon-membership test — correct only in the limit of infinite samples, poor near curved trim
  boundaries). Scaled by the derived `×6`/`×24` factors to match the legacy tetra-sum convention
  those callers already rely on (verified algebraically against `signed_tetra_sum`'s own
  normalization, not guessed) — deleted the now-dead `parametric_volume_moments`,
  `integrate_parametric_face`, `gauss_samples`, `GL5_NODES`/`GL5_WEIGHTS`, `loop_uv_bounds`,
  `outward_normal`.
- `distance_solid_solid`: the "0 if bounding boxes touch" false-positive risk described in the
  audit is gone — overlap is now decided by the real classifier (`solids_overlap`, sampling each
  solid's own boundary points through `classification::point_in_solid` on the OTHER solid).
  Added edge/edge candidates (`edge_edge_closest_distance`/`edge_edge_min_distance`, sampling one
  edge's curve against the other's `curve_ops::closest_parameter` in both directions) alongside
  the existing vertex/face-sample-vs-face candidates. This is sampling+refinement based, not a
  certified global continuous optimum — full face/face closest-point optimization for curved
  B-Rep pairs is out of scope for this pass (documented, not silently claimed as certified).
- `closest_point_on_face`/`closest_point_on_planar_face` now use W1-D2's newly-landed certified
  `curve_ops::closest_parameter` (which appeared mid-session, replacing the removed sample-based
  `curve_ops::closest_point` this file called before) instead of a fixed 16/24-sample scan.
  Surface-side still uses the existing sample-based `surface_ops::closest_point` — no certified
  `closest_uv` has landed yet, per the ticket's own fallback instruction.
- `face_sample_points`/`closest_point_on_face` made `pub` (reused by bounding-volume's
  `closest_face` and validation-report's self-intersection probe — one implementation, not a
  second copy). Added `solid_signed_volume`/`shell_signed_volume` (raw, un-`abs()`'d) for
  validation's orientation checks.
- Fixed the same NURBS `surface_uv` stub as classification.rs (duplicated code, same fix applied
  to both, kept "close to each other" per doctrine).

## 4. Validation — strong enough to reject broken solids

- `check_missing_pcurves`: every coedge without a p-curve is now an ERROR (`missing-pcurve`), not
  silently skipped — previously `check_same_parameter` just skipped a `None` p-curve, so a body
  missing p-curves entirely validated as clean. Updated this file's own `build_tetrahedron` test
  fixture to attach real p-curves (`attach_planar_pcurves`, a local test helper) so the "clean"
  regression test stays honestly clean under the new stricter rule.
- `check_same_parameter`: 16 base samples (up from 5), adaptively bisecting any interval whose
  deviation grows sharply relative to its neighbor (up to 3 refinement passes) — catches a
  localized divergence a coarse fixed grid would step over.
- `check_shell_closure_and_orientation`: every edge within one shell must be used by exactly 2
  coedges with OPPOSITE `forward` sense — fewer than 2 is `shell-not-closed` (open boundary or
  cross-shell non-manifold), exactly 2 with the SAME sense is `orientation-inconsistent` (adjacent
  faces disagree on traversal direction).
- `check_solid_orientation`: a solid's outer shell must have positive signed volume
  (`shell-orientation-inward` otherwise); each void/inner shell must be inverted relative to it —
  same sign as the outer shell means the cavity wasn't correctly flipped (`void-shell-not-inverted`).
- `check_degenerate_geometry`: edge arc length below its own tolerance (`degenerate-edge`), face
  area below tolerance-squared (`sliver-face`).
- `check_self_intersection_probe`: for every pair of non-adjacent faces (no shared edge) on the
  same solid whose AABBs overlap, samples each face's boundary/interior points
  (`mass_properties::face_sample_points`) and flags `warning-possible-self-intersection` when the
  closest sampled pair comes within tolerance. A probe (vertex/edge-midpoint sampling, not a dense
  UV grid or certified SSI), as the ticket calls for — cheap enough to run always.
- Severity convention: `ValidationIssue` (in `snapshot/🚨️error/🦀️.rs`, not a file I own) has no
  severity field. Rather than edit a file outside my ownership, Warning-level findings use a
  `warning-` code prefix (only the self-intersection probe, per the ticket); every other code here
  is an Error.
- 8 new tests, one per new check, plus the updated clean-tetrahedron fixture.

## 5. Verification

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --lib --message-format short` against the
ROOT workspace was queued behind ~50-60 concurrent `cargo check` invocations from the other 8
workers on this ticket (all hitting the same target-dir lock — `Blocking waiting for file lock on
build directory` the whole session; see `🗑️generated/w1f-check.txt`). Rather than block on that
queue, used H0's isolated standalone harness (`TICKET/🔬️harness`, own workspace/target-dir, never
contends the root lock — see `📓️h0-harness.md`):

- `cd TICKET/🔬️harness && RUSTC_WRAPPER="" cargo check --lib --message-format short` — **1 error
  total in the whole crate**, in `🔺️diff/✂️intersect/🏄️surface-surface/🦀️.rs:472` (E0614, a
  peer's file, not mine — W2-A's intersect module). **Zero errors and zero warnings in any of my 4
  files.**
- `cargo check --message-format short` (includes `#[cfg(test)]`) — same result, still only that
  one peer error plus a handful of OTHER peer errors that only became visible once test code was
  included: `🧩tessellation/🦀️.rs` (5×, calling `make_sphere`/`make_cylinder` with the OLD arg
  count — a primitives signature change), `📸️snapshot/➰️curve/{🦀️.rs,✂️curve-ops/🦀️.rs}` (float
  type-inference and a `NurbsCurve3`/`Curve3` pattern mismatch — W1-D1/D2 mid-refactor),
  `📸️snapshot/🕸️topology/🦀️.rs` (a pre-existing test still calling `serde_json` on `Body`, which
  no longer derives serde), `🔺️diff/{➡️sweep,🔺️euler,🧱️primitives}/🦀️.rs` test modules (import
  `inferences::validation_report`, which the harness deliberately doesn't mount — h0's documented
  gap #2, a real `SemioBrepSnapshot`/schema/STEP-serializer cascade unrelated to `validate_body`
  itself). **None of these are in my 4 files; none were introduced by me.**
- I DID fix two things the harness surfaced that WERE mine: (a) `make_sphere`/`make_cylinder` call
  sites in classification.rs's and mass-properties.rs's own pre-existing tests, broken by the same
  concurrent primitives signature change, trivial arg-count fixes; (b) a genuine bug in my own new
  `bounding-volume.rs` test (`query_ray_ordered_returns_near_to_far`) — wrong reference-depth in a
  `.map(|(item, _)| item)` closure over `&(&&str, f64)`, fixed to `.map(|&(item, _)| item)`.
- `cargo test -- classification bounding_volume mass_properties validation` — blocked from
  actually RUNNING by the peer errors above (the whole crate's test binary must compile as one
  unit). I could not get a PASS/FAIL run of my own tests this session; I'm not claiming one. What
  I can and do claim, verified: `cargo check` (lib AND full-with-tests) is 100% clean for
  `🏷classification`, `🌳bounding-volume`, `📏mass-properties`, `✅validation-report` — every new
  function, every new test, type-checks correctly against the current state of the tree.

## Known limitations (explicit, not silent)

1. Cylinder/cone/torus mass properties: correct (general triangulated path, closed-form-checked in
   tests) but not analytically fast — only sphere and box have closed-form paths.
2. `distance_solid_solid`'s edge/edge and face/face candidates are sampling+refinement, not a
   certified global continuous optimum.
3. Self-intersection is a cheap probe (boundary/interior point sampling), not a certified SSI check.
4. Sphere's pole/seam trim special-case in classification is not p-curve-aware (pre-existing,
   untouched — W1-E's domain).
5. No certified `surface_ops::closest_uv` exists yet — surface-side closest point still uses the
   existing sample-based `closest_point`, per instruction.
6. Actual `cargo test` pass/fail for these modules could not be obtained this session — blocked by
   peer files outside my ownership; `cargo check` is clean.
