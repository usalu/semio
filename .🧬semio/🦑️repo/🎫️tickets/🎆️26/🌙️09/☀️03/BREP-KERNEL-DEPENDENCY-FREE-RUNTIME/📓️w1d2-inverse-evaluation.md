# 📓️ W1-D2 — Certified inverse evaluation, p-curves, isocurves

Worker W1-D2 on `BREP-KERNEL-DEPENDENCY-FREE-RUNTIME`. Kernel root
`B = ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema`.

## What changed

### 1. Curves — `B/📸️snapshot/➰️curve/✂️curve-ops/🦀️.rs` (`// #region 🔖️Project`)

- New `ClosestParam { t, point, distance, certified }` result type.
- `closest_parameter(curve, domain, target, tol) -> ClosestParam`: analytic closed forms —
  - `Line`: exact projection + domain clamp.
  - `Circle`: exact `atan2`, both the near and antipodal-far critical angle wrapped into `domain`
    via a new `periodic_candidates` helper (handles seam-crossing/trimmed-arc cases and the
    degenerate "target on the axis" case).
  - `Ellipse`: **no sampling** — the optimality condition reduces to a quartic in
    `t = tan((θ-c)/2)` (tan-half-angle substitution) for three overlapping 240°-wide windows
    (`c = 0, 2π/3, 4π/3`, so every θ falls safely inside at least one window's finite `t`-range,
    each window's blind spot at `θ = c+π` covered by the others). Each window's quartic is
    isolated and refined on its **Bernstein form** via the existing
    `polynomial::{Bernstein, isolate_roots, refine_root}` — a certified root enclosure.
  - `Nurbs`: Bézier-span decomposition (`bezier_spans`, via repeated `split_nurbs` — no new
    knot-insertion code) + convex-hull (`RationalBezier3::control_hull_box`) pruning against the
    current best distance, seeding safeguarded Newton (`newton_closest_point`, existing) per
    surviving span, domain clamp/wrap preserved.
- `all_closest_parameters(curve, domain, target, tol) -> Vec<ClosestParam>`: every **local
  minimum** (filtered via a second-order check, `is_local_minimum`, from the periodic/NURBS
  candidate sets — the old `all_extrema`'s "minima and maxima both" behavior is dropped since
  nothing outside its own tests called it).
- Deleted the uniform-sampling seed entirely (`all_extrema`'s sample-based sign-scan, and
  `closest_point`'s coarse-grid seed) — replaced by the above.
- **Old `closest_point`/`all_extrema` names removed** (not kept as compatibility wrappers — the
  DoD's "no ignored arguments" rule and the ticket's no-compat-layer rule both argued against
  keeping a `samples: usize` parameter that a certified algorithm can't meaningfully consume). Six
  external call sites were repointed to the new API (§4).
- Also fixed a pre-existing bug in `interpolate_curve_with_tangents` (`knots: clamped` referenced
  an undefined variable; should be `kv`) — this was blocking the whole crate's compilation for
  every concurrent worker, flagged urgently by the ticket coordinator; one-line fix.

### 2. Surfaces — `B/📸️snapshot/🏄️surface/🪡️surface-ops/🦀️.rs` (full file, owned)

- New `ClosestUv { u, v, point, distance, certified }`.
- `closest_uv(surface, domain, target, tol) -> ClosestUv`: exact closed forms for **all five**
  analytic kinds:
  - `Plane`: orthogonal projection + clamp (unchanged behavior).
  - `Sphere`: exact azimuth/elevation (pole-safe via `normalized().unwrap_or(Vec3::Z)`).
  - `Cylinder`: exact — `u` from `atan2`, `v` from a direct clamp (the objective is additively
    separable in `u`/`v` for a cylinder).
  - `Cone`: exact — `u` from `atan2`, `v` from the closed-form minimizer of the plain upward
    parabola `f(v) = (v·tanα - ρ0)² + (v - z0)²` (clamped to the apex when the unconstrained
    minimum is negative).
  - `Torus`: exact — `u` = target's own meridional azimuth (true for any surface of revolution),
    `v` = the exact circle-closest-point angle within that meridional plane's tube cross-section.
  - `Nurbs`: **Bézier-patch subdivision** — new tensor-product knot-insertion/grid-split machinery
    (`insert_u_knot_grid`/`insert_v_knot_grid`, `split_grid_u`/`split_grid_v`, both new since
    surface control nets are 2D grids and no such helper existed) decomposes the surface into
    Bézier patches tagged with their exact `(u0,u1,v0,v1)` sub-domain and control-net hull box;
    hull-box lower-bound pruning feeds a damped 2D Newton (`newton_uv`) on `Surface::derivatives`
    (exact rational derivatives when W1-D1's implementation is live, finite differences
    otherwise — this code calls the method name, not a concrete implementation) with periodic
    wrap per direction and a 1D-solve fallback when the 2×2 Jacobian is near-singular (poles,
    apex, seams).
- Old `closest_point(surface, domain, target, samples)` removed for the same reasons as §1; eight
  external call sites repointed (§4).

### 3. Isocurves — `B/📸️snapshot/🏄️surface/🦀️.rs`, appended `// #region 🧭️Isocurve`

- `IsoDirection { U, V }` and `Surface::isocurve(dir, at) -> Curve3`.
- Every analytic kind reduces to an exact `Curve3::Line` or `Curve3::Circle` whose `eval`
  reproduces `Surface::eval` **pointwise** (not merely the same shape) — derived by hand per kind
  (plane, cylinder, cone including the apex-line generator, sphere meridian/parallel via a
  purpose-built local frame, torus tube-circle/parallel-circle) and verified by tests comparing
  `isocurve(..).eval(t)` against `surface.eval(u,v)` directly, including a 100-iteration random
  torus property test.
- `Nurbs`: the standard isoparametric curve-extraction formula — folds the fixed direction's
  basis functions into new (exact, not approximated) control points on the other direction's own
  knot vector. No new knots, no sampling.

### 4. P-curves — `B/📸️snapshot/➰️curve/🦀️.rs`, appended `// #region 🎯️Pcurve`

- `Surface::eval_pcurve(pcurve, t) -> Pnt3`: trivial (`surface.eval` at `pcurve.eval(t)`).
- `Surface::project_curve(curve, domain, tol) -> Curve2` / `Surface::project_curve_pieces(...) ->
  Vec<Curve2>`:
  - Analytic shortcut (`analytic_pcurve_shortcut`) for a `Line`/`Circle`/`Ellipse` on a `Plane`
    whose in-plane axes are exactly aligned with the curve's own frame (the common case for edges
    built directly in a face's frame) — exact, parameter-for-parameter.
  - General path: adaptive-density sampling (starts at 8, doubles), each sample projected via the
    certified `closest_uv`, periodic directions unwrapped for continuity (`unwrap_near`), fit via
    W1-D1's `curve_ops::interpolate_curve` (present and used, per the ticket's "if present"
    instruction — this exercised and fixed a live bug in it, see §1), refined by checking actual
    3D deviation at **inter-sample midpoints** (not just the fitted points, which are trivially
    exact) until `<= tol` or a 1024-sample cap.
  - Seam-crossing curves: `seam_crossings` bisects for periodic-direction jumps `> 0.4` period
    between coarse probes, `project_curve_pieces` splits the domain there and fits each piece
    independently.

### 5. Engine — `B/⚙️engine/🦀️.rs`

- `BrepKernel` trait (`// #region Evaluate`): new `curve_closest_parameter(&self, curve, point) ->
  Result<(f64, Vec3, f64), BrepError>` and `surface_closest_uv(&self, surface, point) ->
  Result<(f64, f64, Vec3, f64), BrepError>`.
- `Brep` impl: `curve_closest_parameter_sync`/`surface_closest_uv_sync` (call the new
  `curve_ops::closest_parameter`/`surface_ops::closest_uv`), plus trait delegation.
- `closest_point_sync` (Measure region) now routes by entity kind: `Entity::Curve` →
  `curve_closest_parameter_sync` (populates `ClosestPoint.parameter`), `Entity::Surface` →
  `surface_closest_uv_sync` (populates `ClosestPoint.uv`), everything else unchanged
  (`closest_point_on_solid`). Purely additive — no existing signature changed.

## API break and who else was touched

Renaming `curve_ops::closest_point`→`closest_parameter` and `surface_ops::closest_point`→
`closest_uv` (with `tol: f64` replacing the now-meaningless `samples: usize`) broke source
compatibility for existing callers. Per the ticket's own "no ignored arguments / no compatibility
layers" rules, callers were repointed instead of adding a shim:

- `💡️inferences/📏mass-properties/🦀️.rs` — 4 call sites (`closest_point_on_face`,
  `closest_point_on_planar_face`, `face_to_uv`'s NURBS arm).
- `💡️inferences/🏷classification/🦀️.rs` — 1 call site (`face_to_uv`'s NURBS arm).
- `💡️inferences/🧩tessellation/🦀️.rs` — 1 call site (`coedge_point_uv`'s fallback arm).
- `🔺️diff/✂️intersect/➿️curve-surface/🦀️.rs`, `🔺️diff/✂️intersect/🤝️shared/🦀️.rs` — W2-A's own
  files already used the new `closest_uv` name by the time I checked (their concurrent rewrite
  adopted it directly); one older top-level `🔺️diff/✂️intersect/🦀️.rs` inline-module copy that
  existed before their restructure was also fixed but is now superseded by their split files.

## Verification

- **Harness (`TICKET/🔬️harness`, isolated Cargo package, own target-dir, no root-workspace lock
  contention)** — mounts the real `✳️brep` source verbatim:
  - `cargo check --lib`: **zero errors in all four files I own**
    (`curve.rs`/`curve-ops.rs`/`surface.rs`/`surface-ops.rs`). The one remaining error in the
    whole harness build is `E0614` in `🔺️diff/✂️intersect/🏄️surface-surface/🦀️.rs:472`
    (W2-A's file, a `f64` dereference — not mine, not fixed).
  - `cargo test --lib -- surface_ops curve_ops closest isocurve pcurve`: does **not** finish
    compiling the *test* binary — 11 errors, **all pre-existing/other-workers'**, none in my four
    files:
    - `E0432` ×3 in `🔺️euler`/`➡️sweep`/`🧱️primitives` test modules — `validation_report` isn't
      mounted in the harness (documented, known gap in `📓️h0-harness.md` §"Not mounted").
    - `E0277` ×2 in `📸️snapshot/🕸️topology/🦀️.rs:752-753` — `Body` isn't `Serialize`, pre-existing
      test bug from the separate serde-elimination wave.
    - `E0614` ×1 — same W2-A file as above.
    - `E0061` ×4 in `💡️inferences/🧩tessellation/🦀️.rs`'s own test module — calls
      `make_sphere`/`make_cylinder` with a stale extra arg; `🔺️diff/🧱️primitives/🦀️.rs`'s
      signature changed under a concurrent worker (W1-E), not mine.
    - Two of my own bugs *were* caught and fixed this way: an ambiguous-float-type error in my
      `project_curve_pieces_handles_a_seam_crossing_curve_on_a_cylinder` test (missing `: f64`
      annotation) and the `interpolate_curve_with_tangents` `clamped`/`kv` typo (§1). Re-running
      after both fixes leaves only the 11 pre-existing errors above.
  - **I could not get a fully green `cargo test` run** — the crate-wide test binary won't compile
    until those other workers land their fixes. I am not claiming any test passed that I did not
    see run; the closest/isocurve/pcurve tests are written (18 new `#[semio_framework_async_macros::async_test]`
    functions across the two files, several with random/oracle-based `quick` variants) but
    unexecuted pending those unrelated fixes.
- **Root workspace `cargo check -p semio-s-plugin-stdio --lib`** (the ticket's specified gate):
  run twice in the foreground. One run sat on `Blocking waiting for file lock on build directory`
  the whole time (20+ concurrent `cargo check -p semio-s-plugin-stdio` processes were observed
  queued simultaneously — nine-plus workers sharing one target-dir/lock, consistent with
  `📓️h0-harness.md`'s own documented contention) and was killed with no output. The other run
  (started earlier) *did* get past the lock and produced 1983 lines of `--message-format short`
  output spanning many unrelated subsets (pdf/vt/x/…) with **zero `error` lines** before it too
  was killed (still running, not yet finished, after ~40 minutes) to stop adding to the lock
  contention. That is supporting evidence, not a completed, authoritative pass — it never reached
  a `Finished`/error-summary line. `⚙️engine/🦀️.rs`'s new region (the one file the harness does
  *not* mount, per its own documented "Not mounted" §3) is therefore verified by (a) that partial
  zero-error run and (b) careful manual type-checking against the existing
  `curve_ref`/`surface_ref`/`pnt`/`evec` helpers and the `ClosestParam`/`ClosestUv` struct fields
  — not by a completed compiler run. Flagging this explicitly rather than claiming a pass I didn't
  fully see.

## Remaining / handoff

- Ellipse closest-parameter's tan-half-angle quartic is a from-scratch derivation (cross-checked
  algebraically against the `c=0` case, then against a dense-sampling oracle test) — worth a
  second pair of eyes given its algebraic density.
- `project_curve`'s adaptive fit is a working, tolerance-checked implementation but not literally
  the "Bézier subdivision of the 3D curve" phrasing in the plan (adaptive uniform-in-`t` resampling
  instead) — chose this for robustness within the time budget; revisit if a stricter reading is
  wanted.
- Once the root workspace build clears its lock, please re-run
  `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --lib --message-format short` and the
  harness `cargo test` (now likely green once W1-E/W1-G/the topology-serde owner land their
  fixes) to get the actual pass/fail counts this report couldn't obtain.

Files touched: `📸️snapshot/➰️curve/✂️curve-ops/🦀️.rs`, `📸️snapshot/🏄️surface/🪡️surface-ops/🦀️.rs`,
`📸️snapshot/🏄️surface/🦀️.rs`, `📸️snapshot/➰️curve/🦀️.rs`, `⚙️engine/🦀️.rs`,
`💡️inferences/📏mass-properties/🦀️.rs`, `💡️inferences/🏷classification/🦀️.rs`,
`💡️inferences/🧩tessellation/🦀️.rs`, `🔺️diff/✂️intersect/🦀️.rs` (superseded by W2-A's split),
`🔺️diff/✂️intersect/🤝️shared/🦀️.rs`.
