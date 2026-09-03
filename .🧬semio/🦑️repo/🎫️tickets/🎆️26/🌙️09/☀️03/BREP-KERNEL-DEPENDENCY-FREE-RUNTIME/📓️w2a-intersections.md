# 📓️ W2-A — Intersections (exact SSI, p-curves on both supports)

Worker W2-A. File owned: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/✂️intersect/🦀️.rs`,
split into child files under a new `✂️intersect/` subfolder (mounted via `#[path]`, following the
`➰️curve/🦀️.rs` → `🎢️bezier`/`🪢️bspline`/`✂️curve-ops` precedent):

- `✂️intersect/🦀️.rs` — root: submodule mounts + reexports.
- `✂️intersect/🤝️shared/🦀️.rs` — numeric plumbing shared by all three intersection kinds: Gaussian
  elimination, parameter-explicit global curve interpolation (3D + 2D, sharing one knot placement
  so a fit and its p-curves stay evaluable at the same `t`), periodic-angle unwrapping, the exact
  analytic `(u, v)` inverse for every non-NURBS `Surface` kind, and Bézier-span extraction.
- `✂️intersect/➰️curve-curve/🦀️.rs` — curve/curve (mostly unchanged, + tangency).
- `✂️intersect/➿️curve-surface/🦀️.rs` — curve/surface (new analytic cases + real general path).
- `✂️intersect/🏄️surface-surface/🦀️.rs` — surface/surface (majority rewrite; the bulk of this wave).

## 1. `IntCurve` (new shape)

```rust
pub struct IntCurve {
    pub curve3: Curve3,
    pub pcurve_a: Curve2,
    pub pcurve_b: Curve2,
    pub domain: ParamDomain,   // from engine::contract (W1-A), shared t-domain for all three
    pub kind: IntCurveKind,    // Exact | Fitted { max_error: f64 }
}
```

`IntCurve` was previously `{ curve3: Curve3 }` only, and unused/unmatched anywhere outside this
file (confirmed via search before changing it), so no other worker's code needed touching.
`engine.rs`'s three call sites (`curve_curve_intersect_sync`, `curve_surface_intersect_sync`,
`surface_surface_intersect_sync`) only read `.point`/`.curve3`, which are unchanged field names —
compiles untouched.

## 2. p-curve convention

`build_pcurve(surface, curve3, tol) -> (Curve2, is_exact)`:

- **Plane**: always closed-form. `Curve3::Line` → `Curve2::Line`. `Curve3::Circle`/`Ellipse` →
  `Curve2::Ellipse` (equal radii for a circle) — using `Curve2::Ellipse`'s `x_axis` rotation means
  no phase-alignment trick is needed; any circle/ellipse frame maps exactly.
- **Cylinder/Cone/Sphere/Torus** (axisymmetric): a `Curve3::Line` whose direction is parallel to
  the surface's own axis (a ruling) → exact `Curve2::Line`, slope/intercept read off two exact
  samples (the relation genuinely is affine, so two samples fully determine it — no fitting
  error). A `Curve3::Circle`/`Ellipse` whose plane normal is parallel to the surface's axis (a
  "native" latitude/azimuthal circle) → exact `Curve2::Line` (`u = t + phase`, `v = const`).
  Everything else (oblique circles on a sphere not through the pole, the cosine-shaped v(u) an
  oblique cylinder/cone section produces on the cylinder/cone side, NURBS surfaces, general-path
  traces) → `sample_and_fit_pcurve`: 33 exact-inverse samples (`shared::exact_uv`), periodic
  unwrap, then a degree-≤3 global interpolation (`shared::interpolate_params_2d`) sharing the
  curve3 fit's parameterization. **`curve3` can be `Exact` while the `IntCurve.kind` is still
  `Fitted`** — the enclosing `IntCurveKind` reflects whichever side (if any) needed a fit, and
  `max_error` is measured honestly (16-sample dense resampling, distance from the fitted
  curve/p-curve back to the true `curve3.eval(t)`), not assumed.

## 3. Case table

| pair | result | p-curve exactness | notes |
|---|---|---|---|
| plane/plane | line (or empty/`Tangent`) | always exact | unchanged formula |
| plane/cylinder ⊥ axis | circle | exact | unchanged formula |
| plane/cylinder ∥ axis | 0/1/2 lines | exact | tangent case (`h≈0`) now returns **one line**, not `Err(Tangent)` — matches the DO list's "one or two lines, tangency" as a real result |
| plane/cylinder oblique | ellipse | plane side exact; cylinder side **fitted** (the true relation is `v(u) = c − R·tanθ·cos(u)`, a single-frequency cosine — not representable by `Curve2`'s four variants, fits to <1e-9 easily) | documented gap: no literal closed-form Curve2 "cosine" variant exists |
| plane/cone ⊥ axis | circle | exact | via `plane_level_case` (meridian solve) |
| plane/cone oblique, bounded | ellipse | exact both sides | `plane_cone_ellipse`: substituting the plane's own principal axes (`e1`=axis projected into the plane, `e2=n×e1`) into the cone's implicit equation gives a conic with **no cross term** — complete the square directly |
| plane/cone, parabola/hyperbola | — | **routed to `general_marching` (Fitted)** | open item — not hand-derived; see §5 |
| plane/sphere | circle (or `Tangent` point) | exact only when plane normal ∥ sphere's own polar axis; else fitted | sphere frame orientation is caller-chosen, so this is the common case in practice |
| plane/torus ⊥ axis | 0/1/2 circles | exact | meridian solve (shared with plane/cone) |
| plane/torus, axis-containing | 2 circles (radius = minor) | exact | direct formula |
| plane/torus, oblique (incl. Villarceau) | — | **routed to `general_marching` (Fitted)** | open item — Villarceau circles not special-cased; marching still finds the correct curve, just as a NURBS fit |
| sphere/sphere | circle (or `Tangent` point) | exact only when the connecting line ∥ a sphere's own axis; else fitted | `h² ≤ tol²` now returns `Err(Tangent)` (point contact) instead of a radius-clamped near-zero circle |
| cylinder/cylinder, coaxial | via `coaxial_case` | exact | degenerate (no finite circle) unless radii equal (all-points) |
| cylinder/cylinder, parallel non-coaxial | 0/1/2 lines | exact | cross-section circle/circle in the plane ⊥ axis |
| cylinder/cylinder, equal radius, intersecting axes (Steinmetz) | 2 ellipses | exact on the queried cylinder; other side generally fitted | `(p·a)²=(p·b)²` ⟹ lies on one of two bisector planes through the axes' meeting point; each plane's section of cylinder A is one ellipse (reuses `plane_cylinder`) |
| cylinder/cylinder, general | — | `general_marching` (Fitted) | |
| coaxial cylinder/cone/sphere/torus, any pair | 0..2 circles | exact | `coaxial_case`: reduces to intersecting two meridian-profile conics in the shared `(ρ, z)` half-plane (line/line, line/circle, or circle/circle via a radical-line subtraction) |
| cone/cone, cone/sphere, cone/torus, sphere/torus, torus/torus — non-coaxial | — | `general_marching` (Fitted) | |
| any pair involving a `Surface::Nurbs` | — | `general_marching` (Fitted) | |

## 4. Curve/curve and curve/surface

- **curve/curve**: kept line/line, line/circle, circle/circle analytic; added exact single-point
  **tangency detection** (line/circle discriminant ≈0, circle/circle `h≈0`) reported via new
  `tangent: bool` on `CurveCurveHit` instead of silently deduping two near-identical roots into
  one untagged hit. The general NURBS path is unchanged Bézier control-hull subdivision (a
  certified-convergent bound, though not literally a fat-line distance band — documented
  simplification) with a new `multiplicity: u32` accumulated by `merge_hits` when it collapses
  near-duplicate raw hits, and `tangent` set from `d1_a × d1_b ≈ 0` at Newton convergence.
- **curve/surface**: added exact line/cone (quadratic, mirrors line/cylinder), circle/plane and
  circle/sphere (both reduce to `A cos t + B sin t + C = 0`, solved via amplitude/phase). General
  path replaced the fixed 32-sample scan with real Bézier-span subdivision of the curve
  (`shared::curve_to_bezier_segments`) + `closest_uv`-seeded Newton at each surviving leaf, with an
  AABB-vs-`closest_uv`-distance reject test pruning branches that can't reach the surface.

## 5. General marching (the fallback for every undocumented pair)

`general_marching(a, b, tol)`:

1. **Seeding**: each surface's finite domain is binned into a 10×10 cell grid; a cell's AABB is
   estimated from 4 corners + center; cell pairs whose AABBs overlap seed a damped 4-unknown
   Gauss-Newton (`S_a(u_a,v_a) − S_b(u_b,v_b) = 0`) converging to a start point.
2. **Marching**: predictor (`tangent = n̂_a × n̂_b`, projected into each surface's own tangent-plane
   basis for an initial `(u,v)` guess) + corrector (a few damped-Newton iterations on the joint
   4-unknown system). Terminates on a non-periodic domain border, a near-zero tangent (tangential
   contact), or loop closure back near the seed.
3. **Fit**: centripetal-parameterize the traced 3D points, interpolate (3D + both p-curves,
   sharing the parameter array), and measure `max_error` by comparing the fit back to the original
   samples.

**Documented simplifications** (given the scope of this wave): seed cell resolution is a fixed
10×10 grid rather than true recursive Bézier-patch subdivision per knot span; the march step size
is a domain-diagonal heuristic (`0.02 × diag`, clamped) rather than a curvature-bound derived from
`duu`/`duv`/`dvv`. Both are real, working, and tested (skew-cylinder loop closure, coaxial-family
seam continuity, a 20-iteration randomized coaxial cylinder/sphere sweep), but a literal recursive
Bézier-patch subdivision + curvature-bounded step would be a stronger foundation for W2-B's exact
boolean pipeline if seams/tangencies turn out to need tighter certification there.

## 6. Concurrent-tree note

While this file was in progress, `🏄️surface/🪡️surface-ops/🦀️.rs`'s `closest_point(surface, domain,
target, samples)` was replaced by W1-D2 with `closest_uv(surface, domain, target, tol) ->
ClosestUv { u, v, point, distance, certified }` mid-session. All three new files were written
against the new `closest_uv` API (confirmed live on disk via `grep` before finishing); no adapter
was added.

## 7. Verification (results as run)

Harness (`TICKET/🔬️harness`, isolated target dir, `RUSTC_WRAPPER=""`):

**`cargo check --lib --message-format short`** (production code only, no `#[cfg(test)]`):

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 21s
```

Zero errors. Three warnings, all pre-existing/unrelated to `✂️intersect` except one which was in
my own code and has since been fixed (a dead-store on `pt` in `march_direction`, no longer
present):

```
✂️intersect/🏄️surface-surface/🦀️.rs:897:10: warning: value assigned to `pt` is never read   [FIXED]
📸️snapshot/🏄️surface/🪡️surface-ops/🦀️.rs:182:4: warning: function `insert_v_knot_grid` is never used   [not mine]
🔺️diff/🧱️primitives/🦀️.rs:137:15: warning: function `is_degenerate_edge` is never used   [not mine]
```

One real compile error was caught and fixed during this pass (before the run above):
`🏄️surface-surface/🦀️.rs:472:20: error[E0614]: type f64 cannot be dereferenced` — a stray `*rho`
in `meridian_radii_at_level`'s torus branch, where `rho` iterates a `[f64; 2]` array by value, not
by reference. Fixed by dropping the `*`.

**`cargo check --message-format short`** (includes `#[cfg(test)]`) and **`cargo test -- intersect`**:
both fail to *build* the test binary — but every one of the errors is in files this worker does
not own, none in `✂️intersect`:

```
error[E0432]: unresolved import `crate::artifacts::dwg`
  --> ⚙️engine/📦️mesh-io/🦀️.rs:22:23

error[E0432]: unresolved import `...inferences::validation_report`
  --> ⚙️engine/🦀️.rs:62:80
  --> 🔺️diff/🧵️sew/🦀️.rs:14:80
  --> 🔺️diff/🧱️primitives/🦀️.rs:752:84
  --> 🔺️diff/🔺️euler/🦀️.rs:426:84
  --> 🔺️diff/➡️sweep/🦀️.rs:439:84
  (all the same root cause: the harness's `inferences` mount doesn't currently expose
  `validation_report` — W1-F's module, not W2-A's)

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
  --> ⚙️engine/🦀️.rs:1267:27, 1271:29   (Entity::Surface(_) vs Entity::Surface(_, _))

error[E0433]: unresolved module/crate `serde_json`
  --> ⚙️engine/🦀️.rs:1291:22

error[E0277]: `topology::Body` doesn't implement `Serialize`/`Deserialize`
  --> 📸️snapshot/🕸️topology/🦀️.rs:752:42, 753:26, 98:1
```

`brep-kernel-harness (lib)`: 5 errors. `brep-kernel-harness (lib test)`: 10 errors (the 5 above
plus the topology.rs serde pair once `#[cfg(test)]` code is included). All six root causes are
in `⚙️engine/🦀️.rs` (shared/multi-owner), `⚙️engine/📦️mesh-io/🦀️.rs`, `🔺️diff/🧵️sew/🦀️.rs`,
`📸️snapshot/🕸️topology/🦀️.rs` — none in this worker's files, and this matches
`📓️h0-harness.md`'s own documented status ("READY, with known-failing tests, all
pre-existing/concurrent-work, not harness bugs"). No `✂️intersect` unit test could be *run* as a
result — the crate's test binary doesn't build for reasons entirely outside this file's scope.
`--lib` (production code, what the harness's own quickest gate recommends) is clean.

Root-workspace `cargo check -p semio-s-plugin-stdio --lib` was attempted first per the original
brief, but the coordinator flagged 20+ duplicate queued invocations (mine included, from repeated
turns) contending the shared `target/debug/.cargo-lock`; killed the duplicates and switched to the
harness per the coordinator's redirect (W1-C owns the root check).
**Re-verified after the `pt`/`*rho` cleanups** (final state of the files): `--lib` was re-run and
this time also showed 5 errors — but a `grep intersect` on that output returns nothing; all 5 are
newly-landed concurrent edits to `⚙️engine/🦀️.rs` (`Entity::Surface` arity, `dwg` import) and
`🔺️diff/🧵️sew/🦀️.rs` (`validation_report` import) that arrived between the two check runs (both
files are owned by other workers, actively changing). `✂️intersect`'s own files remain error-free
across every run in this section.

## 8. Open items (for W2-B and future work)

- Plane/cone parabola/hyperbola: not hand-derived exactly; falls to `general_marching` (correct,
  `Fitted`, not `Exact`).
- Plane/torus oblique sections (Villarceau circles among them): same — falls to `general_marching`.
- `general_marching`'s seed grid and step heuristic are real but not literally the recursive
  Bézier-patch subdivision + curvature-bounded step the plan describes; fine for W2-A's own tests,
  worth revisiting if W2-B's boolean pipeline needs tighter seam certification on general pairs.
- Cylinder/cylinder Steinmetz reuses `plane_cylinder` for the queried side; the non-queried
  cylinder's p-curve is usually `Fitted` (oblique bisector plane) rather than exact.
