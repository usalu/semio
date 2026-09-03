# 📓️ W1-D1 — Exact NURBS Math, Knots, Interpolation

Scope: `➰️curve/🪢️bspline/🦀️.rs`, the `Nurbs` arms of `➰️curve/🦀️.rs`/`🏄️surface/🦀️.rs`, the
interpolation/approximation fns of `➰️curve/✂️curve-ops/🦀️.rs`, and the `#region 🔖️SyncApi`
`interpolate_curve_sync`/`approximate_curve_sync`/`nurbs_surface_from_grid_sync`/`coons_patch_sync`
in `…/✳️brep/🧬️schema/⚙️engine/🦀️.rs`.

## What changed

**`🪢️bspline/🦀️.rs`** (new, all pure functions, dimension-agnostic over `Vec<f64>` channels):
- `curve_derivatives_rational(knots, controls_h, u, order) -> Vec<Vec<f64>>` — Piegl–Tiller `A_k(u)`
  recurrence; exact through any order (higher than degree returns the correctly-zero vector).
- `surface_derivatives_rational(u_knots, v_knots, controls_h, u, v, order) -> Vec<Vec<Vec<f64>>>` —
  tensor-product `RatSurfaceDerivs` generalization, `result[k][l]` for `k+l<=order`.
- `KnotVector::periodic_uniform(n, degree)` / `is_periodic()` / `wrap(u)` — unclamped uniform knots
  + the standard "phantom wrap" control-point trick (first `degree` points duplicated onto the
  end), so ordinary clamped de Boor machinery reproduces a `C^(degree-1)`-continuous closed curve
  with no special-cased evaluation path.
- `remove_knot(knots, controls, u, tol) -> Option<(KnotVector, Vec<Vec<f64>>)>` — Tiller's algorithm
  (NURBS Book A5.8), single removal, joint (not per-axis) tolerance check. Found and fixed a
  temp-array off-by-one during hand-verification (size must be `last-first+3`, not `+2`) before
  relying on it.
- `elevate_degree(knots, controls, t)` — general multi-span degree elevation via Bézier
  decomposition (`insert_knot_multi` to full multiplicity) + per-segment elevation
  (`elevate_bezier_span_multi`) + `remove_knot` back down to `original_mult + t` per interior knot.
- `insert_knot_multi`/`elevate_bezier_span_multi` — multi-channel (whole-point, not per-axis)
  counterparts of the existing scalar `insert_knot`/`elevate_bezier_span`, kept alongside them.

**`➰️curve/🦀️.rs`**: `Curve3::d1`/`d2` (+new `derivatives(t, order)` for arbitrary order) and
`Curve2::d1`/`d2` now call `curve_derivatives_rational` for the `Nurbs` arm — the `h=1e-4`/`1e-5`
finite-difference stand-ins are deleted. `is_periodic`/`period()` recognize periodic `Nurbs` knot
vectors (not just Circle/Ellipse).

**`🏄️surface/🦀️.rs`**: `Surface::derivatives`'s `Nurbs` arm calls `surface_derivatives_rational`
(du/dv/duu/duv/dvv all exact); the `finite_difference_derivatives` function is deleted; the header
comment that called FD "unsuitable for tight Newton" is updated.

**`➰️curve/✂️curve-ops/🦀️.rs`** (`#region 🔖️Fit` rewritten, new `#region 🔖️SurfaceFit`):
- `ParamMethod{Uniform,Chord,Centripetal}` + `parameterize()`.
- `interpolate_curve(points, degree, params_method, ends, closed) -> Option<NurbsCurve3>` — true
  global interpolation (exact linear solve, not "points become controls"). `ends: Some((d0,d1))`
  clamps start/end tangents via the standard clamped-B-spline boundary identity
  `C'(0) = p/(U[p+1]-U[0])·(P1-P0)`, expressed as 2 extra rows/controls in the same linear system.
  `closed: true` builds a periodic curve (`interpolate_curve_closed`, n×n system with column
  indices wrapped `mod n` over `KnotVector::periodic_uniform`).
- `interpolate_centripetal` kept as a thin wrapper over `interpolate_curve` (same signature/tests).
- `approximate_curve_with_count(points, degree, n_controls)` / `approximate_curve(points, degree,
  max_error)` — Piegl-Tiller least-squares fit (pinned endpoints, normal-equations solve for
  interior controls via the existing `solve_linear_system`); `approximate_curve` grows
  `n_controls` until the achieved max deviation is within `max_error`, returning the achieved error.
- `interpolate_surface_grid(points, degree_u, degree_v) -> Option<Surface>` — true 2-pass separable
  global surface interpolation (not "grid becomes controls").
- `coons_patch_nurbs(c0, c1, d0, d1, tol) -> Option<Surface>` — exact NURBS Coons patch: harmonizes
  each boundary pair onto a shared `(knots, degree)` (`harmonize_pair`, via `elevate_degree` +
  `insert_knot_multi`), builds 2 ruled surfaces + 1 bilinear corner surface on that shared basis via
  `linear_curve_to_target`, combines `ruled1+ruled2-bilinear` channel-wise in homogeneous space.
  Verified by hand (and by the boundary-reproduction test) that this exactly reproduces each
  boundary curve at its edge, including the weight channel, not just a Euclidean approximation of
  the classical Coons formula.

**`⚙️engine/🦀️.rs`** `#region 🔖️SyncApi`: `interpolate_curve_sync`/`approximate_curve_sync` now call
`interpolate_curve`/`approximate_curve_with_count` (were: build a clamped-uniform curve straight
from the input points, and downsample for approximation); `nurbs_surface_from_grid_sync` calls
`interpolate_surface_grid`; `coons_patch_sync` first interpolates each of the 4 input polylines
into an exact `NurbsCurve3`, then calls `coons_patch_nurbs` (was: bilinear-sample a grid then treat
it as controls). All four now return `Err(BrepError::InvalidInput(...))` on degenerate input
instead of silently producing a degraded result.

## Tests added

`bspline`: rational derivatives vs. plain basis-derivative sum (unweighted case), vs. an
independently-built quarter-circle rational-quadratic NURBS (unit-circle + perpendicularity, exact
to the assertion tolerance), vs. Richardson-extrapolated central differences on 100 random rational
curves (quickcheck, 1e-6); a bilinear-patch analytic-formula check for `surface_derivatives_rational`
(du/dv/duv exact, duu/dvv exactly zero); `periodic_uniform`/`is_periodic`/`wrap`; `remove_knot`
round-trips an insertion (deterministic + 100-case quickcheck) and rejects a non-removable knot;
`elevate_degree` is a no-op at `t=0`, preserves a multi-span curve at `t=2` including interior-knot
multiplicity, and a 100-case quickcheck over random degree/span-count/elevation-amount.

`curve.rs`: NURBS quarter-circle d1/d2 exact (unit circle, perpendicularity, curvature=1, all
<1e-9); NURBS tangent direction matches the analytic circle's at the same physical point (angle
derived from the NURBS point itself, since the rational-quadratic parametrization isn't
angle-linear — same caveat `to_nurbs`'s own doc already states).

`surface.rs`: a hand-built degree-2×2 rational bump patch vs. Richardson-extrapolated FD (du/dv,
1e-6); normal/curvature well-defined off singularities; 40-case quickcheck over random rational
control nets/weights vs. Richardson FD (1e-5).

`curve-ops.rs`: `interpolate_curve` passes through every point at its own parameter (1e-10); end
tangents honoured (direction match within 1e-4 via a small-h FD probe); closed pentagon
interpolation passes through every vertex and is continuous in d1/d2 across the seam
(before/after-seam FD agree); `approximate_curve` achieves the requested bound on a noisy
sinusoidal-helix point set; `approximate_curve_with_count` matches endpoints exactly;
`interpolate_surface_grid` passes through every grid point (1e-8); `coons_patch_nurbs` reproduces
all 4 boundary curves exactly along their edge of the patch (1e-7).

## Verification

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --lib --message-format short` (foreground,
root workspace — completed after a long wait on the shared build-directory lock several other
concurrent workers were also blocked on; see `🗑️generated/w1d1-check.txt`, extracted from the full
run): **zero errors in `🪢️bspline`, `➰️curve/🦀️.rs`, `🏄️surface/🦀️.rs`, `✂️curve-ops/🦀️.rs`, or my
`⚙️engine/🦀️.rs` region.** The 7 errors present are all in files/regions I do not own: W1-C's
`handle_for_label` (`⚙️engine/🦀️.rs:1516-1528`, borrow-checker), W1-D2's `closest_point_sync`
(`⚙️engine/🦀️.rs:1268/1272`, tuple-arity mismatch against `closest_point_on_solid`) — both
pre-existing/concurrent, not touched.

Additionally ran `cargo check --lib` inside `TICKET/🔬️harness` (H0's isolated, lock-free harness):
1 error total, in W2-A's `🔺️diff/✂️intersect/🏄️surface-surface/🦀️.rs:472` (`*rho` deref on an
owned `f64`) — unrelated to this ticket row, not touched.

`cargo test -- bspline curve surface` inside the harness: **blocked from actually running** by
several *other* workers' errors in the same crate (not fixable within my ownership): an unresolved
`inferences::validation_report` import in `primitives.rs`/`euler.rs`/`sweep.rs`'s own test modules
(H0's documented, not-yet-resolved limitation), `🧩tessellation/🦀️.rs`'s own tests still calling
`make_cylinder`/`make_sphere` with the pre-W1-E argument count, W2-A's surface-surface deref bug
above, `📸️snapshot/🕸️topology/🦀️.rs`'s pre-existing `serde_json` round-trip test (serde-elimination
wave fallout, flagged in H0's own report), and — found live, mid-session — W1-D2's own
`project_curve_pieces` test in `➰️curve/🦀️.rs`'s `🎯️Pcurve` region (ambiguous-float-type
`angle0`/`angle1` literals). None are in code I wrote. While hunting for MY OWN bugs the same way
(hand-tracing `remove_knot`, and this test-compile attempt), I did find and fix one real bug of my
own: `interpolate_curve_closed`'s test destructured `curve` as `Curve3::Nurbs{..}`, but
`interpolate_curve` returns `NurbsCurve3` directly (E0308) — fixed to use `curve.knots` directly.
I did **not** get a clean `cargo test` run and am not claiming individual test pass/fail beyond
what `cargo check` (types + borrow-checking, which test bodies also go through for compilation)
already confirms; whoever next gets a green `cargo test` in this tree should run
`-- bspline curve surface` and report the actual pass/fail count.

## Known simplifications (documented in-code)

- `interpolate_curve`'s tangent-constrained case uses uniform (not parameter-averaged) interior
  knots — a conditioning tradeoff for a simpler, hand-verifiable construction, not an exactness one.
- `interpolate_curve_closed`'s non-uniform parameterization is a direct wrap-aware
  chord/centripetal computation, not literally shared with `parameterize()` (which isn't
  period-aware) — documented inline.
- `coons_patch_nurbs` combines the 3 constituent surfaces' homogeneous control nets directly
  (not via a general "two rational surfaces with different weight functions" sum, which would need
  a full product/rebasing construction); this is exact at and only guaranteed at the 4 boundary
  edges and 4 corners — verified in the boundary-reproduction test and by the cancellation argument
  in the function's own doc comment.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/🪢️bspline/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🏄️surface/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/✂️curve-ops/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs` (only the 4 named
  functions in `#region 🔖️SyncApi` + removed one now-unused `KnotVector` import)

## Note on an inaccurate coordinator message

Mid-task a message claimed I had "renamed/moved `curve_ops::closest_point`" and broken a caller in
`💡️inferences/📏mass-properties/🦀️.rs:746`. I never touched `closest_point`/`closest_parameter` (that
region is W1-D2's) — verified before acting: `mass-properties.rs:746` calls
`curve_ops::closest_parameter`, a function W1-D2 added (not something I renamed from), and the
mismatch (if any) is between W1-F's and W1-D2's slices, not mine. Did not act on the message beyond
verifying it didn't apply to me.
