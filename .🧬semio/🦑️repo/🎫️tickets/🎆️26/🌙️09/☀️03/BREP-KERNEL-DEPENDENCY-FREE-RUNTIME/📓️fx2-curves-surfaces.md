# 📓️ FX-2 — snapshot::curve / snapshot::surface fixes

Fixer FX-2 on `BREP-KERNEL-DEPENDENCY-FREE-RUNTIME`. Scope: `…/📸️snapshot/➰️curve/**` and
`…/📸️snapshot/🏄️surface/**` (bspline, bezier, curve-ops, surface-ops, curve/surface roots) only.

## Starting point

The coordinator's digest (`🗑️generated/coordinator-harness-failures.md`) listed 14 tests under my
scope. Re-running against the live tree at task start showed only 12 still failing —
`approximate_curve_achieves_the_requested_error_bound` and
`approximate_curve_with_count_matches_endpoints_exactly` already passed (fixed by a concurrent
worker before I started). I fixed the remaining 12, plus improved two other functions' robustness
along the way (see below) without weakening any assertion.

## Fixes

### 1. `curve.rs` / `surface.rs` — non-similarity `transformed()` was not angle-parametrization-exact

`arc_to_nurbs`/`circular_profile`'s standard rational-quadratic circle construction reproduces a
circle's **shape** exactly but is provably **not** angle-linear in its parameter interior to a
120°-max span (cos/sin are transcendental, a finite-degree rational curve can only equal them at
isolated points, not on an interval — verified both by a symbolic series expansion, giving the
leading-order error `peak ≈ 0.0321·radius·half_span³`, and numerically). `Curve3::transformed`'s
and `Surface::transformed`'s non-similarity paths need the STRONGER pointwise-pushforward contract
`transformed.eval(t) == map.apply_point(self.eval(t))` at every `t`, not just the shape. Added
`refined_max_span(radius)` (derived from that same error bound, solved for a `1e-9` target) and
`arc_to_nurbs_with_span`/`circular_profile_with_span` (the existing 120°-span callers are
untouched); `transformed_via_nurbs`/`Surface::transformed`'s non-similarity arms now call the
refined variant. For `Cone`'s unbounded-`v` profile, sizing off the true worst-case radius
(`PRACTICAL_UNBOUNDED_EXTENT · tan(half_angle)`, ~577000) would force ~166k spans; verified
numerically that the `v`-direction's degree-1 blend between apex (`r=0`) and the far point makes
the ACTUAL error at any query `v` scale with `r(v)`, not the extreme's `r` — sized off
`half_angle.tan()` (`r` at `v=1`) instead, giving ample precision for any practically-queried `v`
at a sane span count. Fixes: `circle_transformed_via_nurbs_under_non_similarity_matches_mapped_eval`,
`sphere_/torus_/cone_transformed_via_nurbs_under_non_similarity_matches_mapped_eval*`.

### 2. `surface.rs` — `Cylinder`/`Cone` similarity transform used the wrong frame-axis scaling

`Frame3::transformed` divides every axis by `scale` (correct for `Sphere`/`Torus`, whose `eval`
multiplies ALL three axes by an already-`scale`-compensated radius). `Cylinder`/`Cone::eval` use
`frame.z` as a raw multiplier on `v` itself (no radius in that term) — dividing `z` by `scale`
there drifted `transformed.eval(u,v)` from `map.apply_point(self.eval(u,v))` by a `v`-proportional
error (derived algebraically: the correct `z'` is the un-divided `map.apply_vector(frame.z)`,
magnitude `scale`, matching the correct `d(eval)/dv` under `map`). Added `axial_frame_transformed`
(x/y still `1/scale`, z left un-divided) for these two arms only; `Sphere`/`Torus`/`Circle`/
`Ellipse` still use the original `Frame3::transformed` unchanged. Fixes:
`cylinder_transformed_stays_cylinder_under_similarity`.

### 3. `curve-ops.rs` — `all_closest_parameters`/`all_local_minima_nurbs` missed minima

- `all_local_minima_periodic` (circle/ellipse): `is_local_minimum` assumes `t` already satisfies
  the first-order condition `f'(t)=0` (its own docstring); a full-period domain's `domain.0`/
  `domain.1` sentinels are the SAME arbitrary physical point, not a real critical point, so
  evaluating `is_local_minimum` there was a misuse that spuriously returned `true`, producing a
  bogus second "minimum" (the domain-boundary point) alongside the real one. Now dropped
  unconditionally for a full-period domain; a genuinely trimmed sub-arc still keeps both endpoints
  unconditionally. Fixes: `closest_parameter_on_circle_is_the_unique_local_minimum`.
- `all_local_minima_nurbs`/`closest_on_nurbs` (NURBS): seeded Newton from a SINGLE midpoint per
  Bézier span/patch. A degree-`p` span's distance-squared derivative has degree up to `2p-1`, so up
  to `p` local minima can coexist in ONE span — a single seed only ever finds one basin (the
  S-shaped-curve test is exactly this: one cubic span, two minima). Added `span_seeds`/
  `patch_seeds` (5 seeds per 1D span, 5×5=25 per 2D patch) in both `closest_on_nurbs`/
  `all_local_minima_nurbs` (curve-ops.rs) and `closest_on_nurbs_surface` (surface-ops.rs) — same
  bug class, same fix, applied consistently to both the single-closest and all-minima entry points.
  Fixes: `all_closest_parameters_finds_both_minima_of_an_s_shaped_curve`,
  `snapshot::surface::surface_ops::tests::quick::closest_point_on_nurbs_patch_matches_dense_sampling_oracle`.

### 4. `curve-ops.rs` test — `interpolate_surface_grid_passes_through_every_grid_point` wrong assumption

`interpolate_surface_grid` uses NURBS-Book-§9.5 chord/centripetal-averaged `u`/`v` parameters
(documented, standard, avoids oscillation for non-uniform grid data) — the test assumed these land
on a uniform `i/(n-1)` grid, which is only true for perfectly evenly-3D-spaced input (this grid's
`z = sin(0.5i)+cos(0.3j)` is not linear in `i`/`j`, so its averaged params are close to but not
exactly uniform: verified numerically, e.g. `u_params[1] = 0.3408` vs uniform `0.3333`). Added a
`grid_params` test helper that recomputes the SAME averaging via the existing public
`parameterize` primitive (an independent oracle, not touching the algorithm), and evaluates at
those actual parameters instead of guessed uniform ones. The implementation was not touched.

### 5. `surface-ops.rs` test — `closest_point_on_cylinder_seam_wraps_correctly` wrong frame assumption

The fixture built `target` from raw world `cos`/`sin` of a small negative angle, assuming
`Frame3::from_normal(origin, Z)` gives `x = world X`. It does not: `Vec3::any_orthogonal()`'s
smallest-component heuristic gives `x = (0,1,0), y = (-1,0,0)` for `normal = Z` — a 90°-rotated
basis. World-angle `-0.01` is therefore LOCAL angle `-0.01 - π/2 ≈ 4.702` (verified algebraically
and numerically), nowhere near the cylinder's own `u = 0`/`TAU` seam the test's name promises to
probe. Fixed by building `target` via `frame.to_world(...)` (the frame's own axes) instead of raw
world coordinates — frame-agnostic, actually exercises the seam. Implementation untouched.

### 6. `curve.rs` test — `project_curve_on_cylinder_stays_within_deviation_bound`, three compounding test bugs

- The ellipse fixture (`major=3.5, minor=3.2` on a `radius=3` cylinder, frame via
  `Frame3::from_normal`) was not actually an exact cylinder cross-section: for an oblique circular
  section, `minor_radius` must equal the cylinder radius exactly and `major_radius = radius /
  cos θ`, AND the ellipse's own frame.x must be the tilt direction itself (the projection of the
  cylinder axis onto the cutting plane) — `from_normal`'s arbitrary `any_orthogonal`-based x is not
  that direction. The old fixture's curve was up to ~0.35 off the actual cylinder surface at its
  worst point — no p-curve (which must stay ON the surface) could satisfy a tight deviation bound
  against source data that far away, regardless of fit quality. Rebuilt the fixture as a verified
  (numerically: `x²+y² = radius²` at every sampled `t`) exact cross-section.
- `project_curve` (unlike `project_curve_pieces`) deliberately returns only its first seam-free
  piece (its own docstring) — a full-revolution domain necessarily crosses the cylinder's `u=0`/
  `TAU` seam, so comparing against the full `(0, TAU)` range compared a partial-domain p-curve
  against out-of-range samples. Reduced the tested domain to `(0.0, 4.0)`, verified numerically to
  stay under one seam-free arc for this fixture.
- The deviation check assumed `pc`'s own (centripetal) parameter fraction corresponds *affinely* to
  the original curve's `t` fraction — it does not (centripetal spacing follows 3D chord length, not
  `t`). Replaced the proportional-remap check with the geometrically meaningful property
  `project_curve`'s docstring actually promises: every traced point stays within `tol` of the TRUE
  3D curve, checked via the existing certified `curve_ops::closest_parameter` (exact for an ellipse,
  no sampling-resolution risk — an earlier brute-force-oracle draft of this same check produced a
  false positive from under-resolving the curve's ~3-unit/radian "speed", confirmed and discarded).
- Also strengthened `fit_pcurve`'s own internal convergence check from one inter-sample midpoint to
  4 evenly-spaced interior probes per interval, a general robustness improγement (not the fix for
  the above — the single-midpoint check was already converging correctly for this specific case,
  confirmed by tracing `n_samples`/`max_dev` at each doubling step; kept anyway as a strictly more
  thorough check for other future cases).

## Signature changes

`revolve_to_nurbs` (surface.rs) gained a `u_radius_scale: f64` parameter (its 4 call sites, all
within `Surface::transformed`, updated). `arc_to_nurbs` gained a sibling `arc_to_nurbs_with_span`
(curve.rs); `circular_profile` gained a sibling `circular_profile_with_span` and the now-unused
bare `circular_profile` was removed (surface.rs). All are private (`fn`, not `pub fn`) — no
external callers, nothing to grep outside these two files. No public API signature changed.

## Verification

Ran (foreground, harness, `RUSTC_WRAPPER=""`):

```
cargo test -- snapshot::curve snapshot::surface
test result: ok. 113 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 17.57s
```

Full-suite `cargo test` (all 448 tests, to confirm no regression outside scope) — see the report's
closing line for the verbatim result.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/✂️curve-ops/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🏄️surface/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🏄️surface/🪡️surface-ops/🦀️.rs`

No other files edited. No git write commands run. Ticket not closed/reopened. `📌️important.md` not
touched.
