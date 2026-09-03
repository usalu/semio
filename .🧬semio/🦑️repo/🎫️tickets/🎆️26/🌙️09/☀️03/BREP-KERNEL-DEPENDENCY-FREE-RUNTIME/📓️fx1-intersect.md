# 📓️ FX-1 — `diff::intersect::surface_surface` failures fixed

Fixer FX-1, scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/✂️intersect/**`
only. All 8 assigned failing tests now pass; found and fixed 3 distinct root-cause bugs (one of
them a shared systemic bug, matching the ticket's hint) plus one bad test fixture.

## Root causes

### 1. `IntCurve.domain` was `curve3.domain()` verbatim — infinite for any `Curve3::Line` result

`finish_intcurve` (`🏄️surface-surface/🦀️.rs`) set `IntCurve.domain` directly from
`curve3.domain()`. `Curve3::Line`'s natural domain is `(-∞, ∞)` by design (`➰️curve/🦀️.rs:59`,
not mine to change and correctly documented there — every consumer is expected to pick its own
finite window). But `finish_intcurve` never did that: any plane/plane, plane/cylinder-parallel, or
cylinder/cylinder-parallel result (all produce `Curve3::Line`) got an infinite `IntCurve.domain`.
Every consumer that samples across it (the test file's own `assert_on_both`, and presumably any
future edge-trim code) computes `t0 + (t1-t0)*frac` on `±∞` → `NaN` at every sample. This is
exactly the "IntCurve domain" convention gap the ticket brief flagged as a suspect.

Fixed by adding `finite_curve3_domain` (`🏄️surface-surface/🦀️.rs:227-247`): for a `Curve3::Line`,
bound it against **both** supports via the existing `line_domain_against_surface` bounding
strategy `curve_surface::curve_sample_domain` already used for a single surface — moved that
helper (and its `finite_surface_domain` companion) into `🤝️shared/🦀️.rs` so both `curve_surface`
and `surface_surface` share one implementation instead of two, per the codebase's own "keep
repeated code close together" rule. While moving `finite_surface_domain`, fixed a real (if
previously unexercised) latent bug in it: the old infinite-fallback was `other_end ± τ`, which
stays infinite whenever **both** ends of the same axis are infinite at once — true for
`Surface::Plane`'s `u` *and* `v` simultaneously, never hit before because every `Curve3`/`Plane`
pair in `curve_surface` has its own closed form, but a real gap now that `surface_surface` calls it
for a `Curve3::Line` bounded against a `Surface::Plane`. Replaced with a plain `±10` constant on
each side independently.

Affects (of the 8): `orthogonal_planes_intersect_in_line`, `plane_cylinder_parallel_two_lines`,
`cylinder_cylinder_parallel_two_lines`.

### 2. `fit_basis`'s clamped knot padding was hardcoded `[0, 1]`, not `[params[0], params[n-1]]`

The systemic bug matching the ticket's "p-curve parametrisation convention" hint.
`shared::fit_basis` (`🤝️shared/🦀️.rs`) — used by both `interpolate_params_2d` and
`interpolate_params_3d` — built the clamped-knot-vector ends as literal `0.0`/`1.0`:

```rust
let mut knots = vec![0.0; degree + 1];
...
knots.extend(std::iter::repeat_n(1.0, degree + 1));
```

This is only correct when the caller's `params` array is itself normalized to `[0, 1]`, which
`centripetal_params` (used by `general_marching`) always guarantees but `sample_and_fit_pcurve`
(`🏄️surface-surface/🦀️.rs`) never did — it interpolates at the curve's own raw domain-scaled `t`
(e.g. `[0, 2π]` for any `Curve3::Circle`/`Ellipse`). With interior knots averaged from `[0, 2π]`-
scale `params` but end-clamped to `0.0`/`1.0`, the resulting knot sequence is non-monotonic
(interior knots up to `~6.09` follow a final clamp of `1.0`), so `KnotVector::new`'s
non-decreasing check rejects it, `interpolate_params_2d` returns `None`, and every caller silently
fell back to `Curve2::Line { origin: pts2[0], dir: Vec2::ZERO }` — a **constant** p-curve, exact at
`t=0` and wrong everywhere else (confirmed with a temporary debug print: `pcurve_a.eval(t)` for
`sphere_sphere_circle` returned the identical `(u,v)` at both `t=0` and `t=π/8`).

Fixed by using `params[0]`/`params[n-1]` as the clamp values instead of hardcoded `0.0`/`1.0`
(`🤝️shared/🦀️.rs`, `fit_basis`). Strict generalization — `centripetal_params`'s output always has
`params[0]==0.0` and `params[n-1]==1.0`, so `general_marching`'s existing callers are unaffected;
verified via `shared::tests::interpolate_params_3d_passes_through_samples` (still passing) and the
full `general_marching`/coaxial test set (all still `ok`).

Affects: `sphere_sphere_circle`, `plane_cone_oblique_is_exact_ellipse`,
`plane_sphere_general_is_fitted_but_accurate`.

### 3. Torus axis-containing (meridian) circles had no exact p-curve case

`plane_torus`'s axis-containing branch (§3 of `📓️w2a-intersections.md`) is documented as
**exact**, and correctly builds an exact `Curve3::Circle` — but `build_pcurve`'s
`Circle`/`Ellipse`-on-axisymmetric-surface branch only special-cased the *latitude* alignment
(circle's plane normal ∥ the surface's axis — a native azimuthal circle). A **meridian** circle
(circle's plane *contains* the axis, i.e. normal ⊥ axis) fell through to `sample_and_fit_pcurve`
unconditionally, making the whole `IntCurve` `Fitted` even though the geometry is exact.
Mathematically, any circle lying on a torus within an axis-containing plane must be one of the
torus's own two meridian tube circles (a torus's cross-section by any axis-containing plane is
always exactly that pair — the same fact `plane_torus`'s direct formula already relies on), so
`(u, v)` is affine in the circle's own parameter (`u` constant, `v` linear, slope `±1`) — the same
kind of affine relationship `linear_pcurve_on_axisymmetric` already exploits for cylinder/cone
rulings, just for the meridian direction instead of the ruling direction.

Added `meridian_pcurve_on_torus` (`🏄️surface-surface/🦀️.rs`) and wired it into `build_pcurve`'s
Circle/Ellipse branch, gated on `Surface::Torus` + `n·axis_dir ≈ 0`. Derives the affine slope from
two samples `1e-4` rad apart (not a full unit step like the ruling case — `v`'s slope is `±1` over
the entire `[0, 2π)` domain, so a unit-step sample could itself wrap; a small step avoids that
ambiguity).

Affects: `plane_torus_axis_containing_two_circles` (was `Fitted { max_error: 3.0 }`, now `Exact`).

### 4. Bad test fixture — `plane_cylinder_tangent_is_one_line_not_error`

Not an implementation bug. The plane's frame is `Frame3::from_x_z(origin, Vec3::Y, Vec3::X)`, so
its normal (`z_hint`) is `X`. The test offset `origin` to `(0.0, 2.0, 0.0)` — along **Y**, the
plane's own in-plane `x_hint` direction, not its normal. That leaves the plane's signed distance
from the cylinder's axis at exactly `0` (the plane still contains the axis), which is precisely the
"two diametrically opposite lines" case `plane_cylinder_parallel_two_lines` already covers with an
(un-offset) origin — not tangency. Tangency requires the plane to sit at distance `radius` from the
axis *along its own normal*. Fixed the fixture: origin `(2.0, 0.0, 0.0)` (offset along `X`, the
actual normal, by exactly `radius=2.0`) — verified algebraically (`signed = -2.0`, `h² = r² -
dist² = 0`) and by the passing run below. The assertions themselves (`curves.len() == 1`, result is
a `Curve3::Line`) were never touched.

## Verification

Harness (`TICKET/🔬️harness`), foreground, `RUSTC_WRAPPER=""`, after `bun ./📜️script.ts sync`:

```
$ cargo test -- intersect
running 46 tests
... (44 ok, 1 ignored: general_marching_skew_cylinders_closed_loop_on_both — pre-existing,
     documented hang, not mine, unrelated to this fix)
failures:
    artifacts::...::diff::boolean::tests::union_and_intersect_are_commutative_by_volume

test result: FAILED. 44 passed; 1 failed; 1 ignored; 0 measured; 402 filtered out; finished in 0.04s
```

All 8 assigned tests pass:
`cylinder_cylinder_parallel_two_lines`, `orthogonal_planes_intersect_in_line`,
`plane_cone_oblique_is_exact_ellipse`, `plane_cylinder_parallel_two_lines`,
`plane_cylinder_tangent_is_one_line_not_error`, `plane_sphere_general_is_fitted_but_accurate`,
`plane_torus_axis_containing_two_circles`, `sphere_sphere_circle` — all now `... ok`.

The single remaining failure under the `intersect` name-substring filter,
`diff::boolean::tests::union_and_intersect_are_commutative_by_volume`
(`ClassificationAmbiguous("no interior UV sample found for face face-3-0")`), is in
`🔺️diff/🔀️boolean/🦀️.rs` — outside my file scope, owned by W2-B per the ticket brief. Not touched.

## Files changed (all within my scope)

- `✂️intersect/🤝️shared/🦀️.rs` — `fit_basis` end-clamp fix; added `finite_surface_domain` and
  `line_domain_against_surface` (moved from `➿️curve-surface/🦀️.rs`, with the `Plane`-both-axes-
  infinite fallback bug fixed in the move).
- `✂️intersect/➿️curve-surface/🦀️.rs` — removed the two functions moved to `shared`; call sites now
  `super::shared::finite_surface_domain` / `super::shared::line_domain_against_surface`. No
  behavior change for any currently-exercised path (verified: no new/changed warnings, all
  `curve_surface` tests still `ok`).
- `✂️intersect/🏄️surface-surface/🦀️.rs` — `finish_intcurve` now calls new `finite_curve3_domain`
  instead of `curve3.domain()` directly; added `meridian_pcurve_on_torus` and wired it into
  `build_pcurve`; fixed the `plane_cylinder_tangent_is_one_line_not_error` fixture's plane origin.

## Nothing found outside my file scope

The suspected shared bug turned out to live entirely inside `✂️intersect/**` (not in
`📸️snapshot/🏄️surface/🦀️.rs`'s `Surface::eval`/domain conventions, which I checked carefully and
found internally consistent — `exact_uv`'s inverse formulas round-trip exactly against
`Surface::eval`/`derivatives` for every analytic kind). Nothing to flag to other workers beyond the
one pre-existing, already-documented, out-of-scope boolean failure above.
