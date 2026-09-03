# 🔁 W1-B — Exact Affine Transforms

## Design

**`Affine3`** (`B/🧬️schema/📸️snapshot/➡️vector/🔢️matrix/🦀️.rs`, new `#region 🔖️Affine`, after `Trsf`):
full affine group `p ↦ linear·p + translation` (`linear: Mat3`, general — mirror/non-uniform
scale/shear all representable). `Trsf` (rigid+uniform-scale only) is kept unchanged since it is
genuinely used elsewhere (`inferences::classification`/`mass-properties`'s `Sdf::placement`, W1-F
territory) — `Affine3::from_trsf` lifts one into the other. Constructors: `translation`,
`rotation_axis_angle` (about world origin), `rotation_about(origin,...)`, `scaling(center,factors)`
(non-uniform), `mirror(origin,normal)` (Householder). Ops: `apply_point`/`apply_vector`,
`apply_normal` (cofactor matrix, robust to near-singular `linear`, never divides by `det`),
`determinant`, `compose`, `inverse` (`Option`, `None` iff singular), `is_similarity() ->
Option<(Quat, uniform_scale, is_reflection)>` (decomposes `linear` via `|det|^(1/3)` + an
orthogonality check; reflection factored as `rotation · diag(1,1,-1)` so the returned `Quat` is
always a proper rotation), `max_singular_value` (`= uniform_scale` for a similarity, else the
largest root of `MᵀM`'s cubic characteristic polynomial via the existing `polynomial::solve_cubic`
— the correct tolerance-scaling factor). Added `Mat3::{from_diagonal, scaled, sub, cofactor}` and
`Quat::from_mat3` (Shepperd's method) as supporting primitives. Also added `Frame3::transformed
(&self, map, scale)`: maps `origin`/`x`/`y`/`z` DIRECTLY through `map` (not re-deriving `z` from
`x × y`) — a reflection therefore flips the frame's handedness, which is deliberate (see below).

**`Curve3::transformed`/`Surface::transformed`** (append-only `#region 🔁️Transform` at the end of
`curve.rs`/`surface.rs`, per instructions — never touched the NURBS-arm/closest-point regions other
workers own): `Line` and `Plane` stay exact under ANY invertible affine map (a line/plane's image
under any invertible linear map is again a line/plane — no orthonormality or domain constraint
applies to either; `Plane`'s `x`/`y` are mapped directly, unnormalized, `z` re-derived as `x × y`
since `Plane` places no orthonormality requirement on its own axes, only linear independence).
`Circle`/`Ellipse`/`Cylinder`/`Cone`/`Sphere`/`Torus` stay their own analytic kind under a
*similarity* (frame via `Frame3::transformed`, radii/major/minor scaled uniformly, `half_angle`
angle-invariant); under a non-similarity map they convert to the exact equivalent NURBS: `Circle`/
`Ellipse` via the existing `Curve3::to_nurbs` (bounded natural domain) plus a direct control-point
transform (weights unchanged — provably exact for ANY affine map, since a rational curve/surface's
weighted-average structure is affine-invariant); `Sphere`/`Torus` (bounded in both `u`,`v`) via a
new `revolve_to_nurbs` sweep builder (reuses the same per-≤120°-span rational-quadratic technique
`Curve3::to_nurbs` uses for `Circle`, generalized to an off-origin circle for the torus meridian,
and shared as the angular sweep direction for every revolved kind); `Cylinder`/`Cone` (unbounded
`v`) use the same builder over a documented finite practical window (`PRACTICAL_UNBOUNDED_EXTENT =
1e6`) — exact within that window, the same class of necessary finite-domain choice `Line`'s own
`to_nurbs` already requires. `Nurbs` always just transforms control points. A reflection
(`det(map) < 0`) is NOT compensated inside `transformed()` itself — it is handled once, at the
topology level (`Face::flipped`), per the ticket's instruction to make orientation handling
explicit. `Curve2` p-curves are untouched by design (parameter-space, invariant under the surface's
own map) — asserted directly in `surface.rs`'s
`pcurve_stays_unchanged_when_surface_is_transformed_by_the_same_map` test.

**`B/🧬️schema/🔺️diff/🔁️transform/🦀️.rs`** (new file, mounted in the crate's module tree exactly
like its `🔀️boolean`/`🧱️primitives` siblings, alphabetically after `text`): `transform_solid`
deep-copies every reachable vertex/edge/coedge/p-curve/loop/face/shell into fresh entities (fresh
`PersistentLabel`s via `euler::make_vertex`/`make_edge`/`make_loop`/`add_face`/`add_shell`/
`add_solid`, all recorded generated), transforming each geometric support via `Curve3`/
`Surface::transformed` and copying p-curves unchanged; shared geometry (same `Curve3Id`/`SurfaceId`
reached from multiple edges/faces) is transformed exactly once via an id-remap cache
(`CopyCtx`). Tolerances scale by `map.max_singular_value()`. `Face::flipped` is toggled when
`map.determinant() < 0`. `copy_solid` = `transform_solid` under `Affine3::IDENTITY`.
`transform_face`/`transform_wire` are the same walk scoped to one face / one `Wire`.
`transform_solid_in_place` is the destructive variant (touches existing entities, records
`modified`, dedups shared geometry the same way, never generates new ids).

**`B/🧬️schema/⚙️engine/🦀️.rs`**: deleted `transform_solid_mesh` (tessellate → soup →
`solid_from_triangle_soup`) and its now-dead `rotate_point_around_axis` helper. Added `rotate_about
(shape, origin, axis, angle)` to the `BrepKernel` trait (`// #region Transforms`) + impl
delegation — `rotate` itself still rotates about the WORLD origin only (documented explicitly; the
old code used the shape's bounding-box center, which the audit flagged as a real bug — §6.2
"rotation uses the bounding-box center because the public API lacks an explicit origin"). New
private dispatcher `transform_shape_sync(shape, map: &Affine3)` resolves the handle's entity kind
(Solid → `transform_solid`+`register_solid`; Face → `transform_face`+`register_face`; Wire →
`transform_wire`+`register_wire`; Curve/Surface → `.transformed(map)` directly + `register_curve`/
`register_surface`; anything else → `InvalidInput`) — so translate/rotate/rotate_about/scale/mirror
now support faces/wires/curves/surfaces, not just solids, per the DO item. `translate_sync`/
`rotate_sync`/`rotate_about_sync`/`scale_sync`/`mirror_sync` are now one-line `Affine3` constructor
+ dispatch calls. `copy_shape_sync` uses `copy_solid` directly for solids (its dedicated identity
entry point), falls back to the dispatcher under `Affine3::IDENTITY` otherwise.
`linear_pattern_sync`/`circular_pattern_sync`/`grid_pattern_sync` are UNCHANGED (they already just
call `translate_sync`/`rotate_sync` + `fuse_sync`, so they now transform exactly and fuse via
whatever the boolean module does today — matches the DO item's "patterns transform then fuse, fuse
stays whatever the boolean module does today").

**`B/🧬️schema/⚙️engine/🔖️contract/🦀️.rs`**: `rotate_about` added to `BREP_KERNEL_OPERATIONS`
(trait-declaration order, right after `rotate`); `translate`/`rotate`/`rotate_about`/`scale`/
`mirror`/`copy_shape` set to `OpQuality::ExactAnalytic` (was `MeshDerivedBRep`).
`linear_pattern`/`circular_pattern`/`grid_pattern` left at `MeshDerivedBRep` since they still end
each step with `fuse_sync` (boolean, not yet exact — W2-B's territory).

## Files touched

- `B/🧬️schema/📸️snapshot/➡️vector/🔢️matrix/🦀️.rs` — `Affine3`, `Mat3::{from_diagonal,scaled,
  sub,cofactor}`, `Quat::from_mat3`, `Frame3::transformed`, + tests.
- `B/🧬️schema/📸️snapshot/➰️curve/🦀️.rs` — appended `Curve3::transformed` + tests (own
  `#region`/`#endregion`, after the existing Tests region; did not touch NURBS-derivative code).
- `B/🧬️schema/📸️snapshot/🏄️surface/🦀️.rs` — appended `Surface::transformed` + `revolve_to_nurbs`/
  `circular_profile` + tests (own region, after the existing Tests region and before W1-D2's
  concurrently-appended `🧭️Isocurve` region — both landed cleanly, sequential, non-conflicting).
- `B/🧬️schema/🔺️diff/🔁️transform/🦀️.rs` — NEW: `transform_solid`/`copy_solid`/`transform_face`/
  `transform_wire`/`transform_solid_in_place` + tests.
- `B/🧬️schema/⚙️engine/🦀️.rs` — deleted `transform_solid_mesh`/`rotate_point_around_axis`; added
  `rotate_about` (trait + impl); rewired `transform_shape_sync`/`translate_sync`/`rotate_sync`/
  `rotate_about_sync`/`scale_sync`/`mirror_sync`/`copy_shape_sync`; two new `use` lines
  (`diff::transform::{copy_solid, transform_face, transform_solid, transform_wire}`, `Affine3`
  added to the existing `vector::matrix` import).
- `B/🧬️schema/⚙️engine/🔖️contract/🦀️.rs` — `rotate_about` in `BREP_KERNEL_OPERATIONS`; quality
  table updates above.
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs` — mounted `diff::transform` (one `#[path]` +
  `pub mod transform;`, alphabetically after `text`, matching every sibling's exact form).
- `TICKET/🔬️harness/lib.rs` — mounted `diff::transform` there too (same pattern), so the isolated
  harness can typecheck/test it; this was additive only, did not touch H0's own engine.rs-mount
  work landing concurrently in the same file.

## Verification

**Root-workspace `cargo check -p semio-s-plugin-stdio --lib`**: per coordinator instruction, ABANDONED
— 20+ duplicate concurrent `cargo check` invocations from other workers were queued on the shared
target-dir lock; my own foreground/background attempts sat on `Blocking waiting for file lock on
build directory` for the ticket's whole remaining time budget and were killed rather than left
running in the background.

**Gate used instead — H0's isolated harness** (`TICKET/🔬️harness`, own target dir, no lock
contention), run FOREGROUND, both commands below:

### `RUSTC_WRAPPER="" cargo check --lib --message-format short`

Full output: `🗑️generated/w1b-harness-check-2.txt`. Verbatim error section (only `error`-level
lines; framework dependency warnings elided — none reference brep):

```
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/📦️mesh-io/🦀️.rs:22:23: error[E0432]: unresolved import `crate::artifacts::dwg`: could not find `dwg` in `artifacts`
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs:62:80: error[E0432]: unresolved import `crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report`: could not find `validation_report` in `inferences`
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🧵️sew/🦀️.rs:14:80: error[E0432]: unresolved import `crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report`: could not find `validation_report` in `inferences`
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs:1291:22: error[E0433]: cannot find module or crate `serde_json` in this scope: use of unresolved module or unlinked crate `serde_json`
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs:1267:27: error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields: expected 2 fields, found 1
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs:1271:29: error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields: expected 2 fields, found 1
error: could not compile `brep-kernel-harness` (lib) due to 6 previous errors
```

All 6 errors are OUTSIDE my region and pre-exist/concurrent-peer:
- `mesh-io` dwg import — H0's own documented, expected gap (harness doesn't mount `crate::dwg`).
- `validation_report` (×2, `engine.rs:62` + `sew.rs:14`) — H0's own documented gap (validate_report
  pulls the whole cross-artifact schema/STEP stack; not mounted).
- `serde_json` at `engine.rs:1291` — inside `validate_sync` (W1-F's `#region Measure`), a harness
  Cargo.toml dependency gap (`serde_json` is dev-only there), not a real-crate bug.
- Both `E0023` at `engine.rs:1267`/`1271` — inside `closest_point_sync` (W1-D2's `#region
  Evaluate`), matching `Entity::Curve(_)`/`Entity::Surface(_)` with 1 field against W1-C's
  concurrently-widened `Entity::Curve(Curve3, PersistentLabel)`/`Entity::Surface(Surface,
  PersistentLabel)` (2 fields) — a real but NOT-mine bug, cross-worker fallout, reported not fixed.

None of the 6 errors are in the `// #region Transforms` trait block, `transform_shape_sync`,
`translate_sync`/`rotate_sync`/`rotate_about_sync`/`scale_sync`/`mirror_sync`/`copy_shape_sync`,
the `// #region 🧮Convert` block, or the `BrepKernelImpl` Transforms delegation — i.e. **zero
errors in every line I own**, confirmed by `grep` against the full error list. `matrix.rs`/
`curve.rs`/`surface.rs`/`diff/🔁️transform/🦀️.rs` independently produce **zero warnings and zero
errors** in this same run (confirmed via an earlier narrower run before H0 widened the harness to
the full `engine.rs`, and again with `grep -E "🔁️transform|error"` on this run).

### `RUSTC_WRAPPER="" cargo test -- transform`

Full output: `🗑️generated/w1b-harness-test-2.txt`. The whole crate's test binary fails to build
(one binary for the whole harness, so no test — mine or anyone else's — actually ran). Verbatim
error blocks:

```
error[E0432]: unresolved import `crate::artifacts::dwg`
  --> .../⚙️engine/📦️mesh-io/🦀️.rs:22:23
   |
22 | use crate::artifacts::dwg::{dwg_drawing_to_mesh, dwg_from_bytes, dwg_to_bytes, mesh_to_dwg_drawing};
   |                       ^^^ could not find `dwg` in `artifacts`

error[E0432]: unresolved import `...inferences::validation_report` (×5: engine.rs:62,
sew.rs:14, primitives.rs:752 [test mod], euler.rs:426 [test mod], sweep.rs:439 [test mod])

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> .../⚙️engine/🦀️.rs:1267:27
     |
 433 |     Curve(Curve3, PersistentLabel),
     |           ------  --------------- tuple variant has 2 fields
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> .../⚙️engine/🦀️.rs:1271:29
     |
 434 |     Surface(Surface, PersistentLabel),
     |             -------  --------------- tuple variant has 2 fields
error: could not compile `brep-kernel-harness` (lib) due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
error[E0277]: the trait bound `topology::Body: serde::Serialize` is not satisfied
    --> .../📸️snapshot/🕸️topology/🦀️.rs:752:42
     |
 752 |         let json = serde_json::to_string(&body).unwrap();
     |                    --------------------- ^^^^^ unsatisfied trait bound
error[E0277]: the trait bound `topology::Body: serde::Deserialize<'de>` is not satisfied
    --> .../📸️snapshot/🕸️topology/🦀️.rs:753:26
error: could not compile `brep-kernel-harness` (lib test) due to 10 previous errors
```

Same 6 root causes as the `check` run plus 2 test-only ones already documented in
`📓️h0-harness.md` ("Known failing"): `primitives.rs`/`euler.rs`/`sweep.rs`'s own
`#[cfg(test)]` modules import `validation_report` too, and `topology.rs`'s
`serde_round_trips_a_whole_body` test needs `Body: Serialize`/`Deserialize`, which it currently
lacks (unrelated serde-elimination-wave fallout, flagged by H0 already). **None of these are in
any file I own**, so none of my own tests (in `matrix.rs`, `curve.rs`'s/`surface.rs`'s
`transform_tests`, `diff/🔁️transform/🦀️.rs`'s `tests`) ran — the shared compilation unit means one
peer's broken file blocks everyone's tests, not just mine. I did NOT modify any of the 6 blocking
files to work around this (out of scope, would touch other workers' regions).

**Consequence for confidence**: my code is type-checked correct (zero errors in every region I
own, across two separate full-harness runs) but NOT yet runtime-verified — I have not personally
observed my own unit tests pass. This is an honest gap, not a claimed pass. Whoever unblocks the 6
listed cross-worker issues (H0/W1-C/W1-D2/W1-F, already individually flagged in their own
`📓️w1*.md` files or above) should re-run `cd TICKET/🔬️harness && RUSTC_WRAPPER="" cargo test --
transform` — my ~25 new tests across `matrix.rs` (Affine3 unit tests), `curve.rs`
(`transform_tests`, 6 tests incl. a non-similarity + reflection + inverse-round-trip case),
`surface.rs` (`transform_tests`, 6 tests incl. the pcurve-invariant assertion and one non-similarity
case per analytic kind), and `diff/🔁️transform/🦀️.rs` (`tests`, 7 tests incl. face/edge-count
preservation, analytic-kind preservation, volume-scales-by-`|det|`, rotate∘inverse round trip,
`copy_solid`, and `OpDelta` generated/modified correctness) should then all run and their pass/fail
be reported honestly by whoever runs them next.

**`engine.rs` wiring specifically**: type-checked (zero errors in my regions) against the FULL
`Brep`/`BrepKernel` façade — H0 widened the harness's `⚙️engine` mount from contract-only to the
full file mid-session (see harness `lib.rs`'s "Engine" region comment), so this is a real, if
partial (no test execution), verification of the dispatcher/trait/impl wiring, not merely of the
math primitives. Still fully untested at runtime for the reasons above.

## Open items / honest gaps

1. **No test execution yet** — see "Consequence for confidence" above. Re-run `cargo test --
   transform` in the harness once the 6 blocking cross-worker issues clear.
2. **`Cylinder`/`Cone` non-similarity transform is exact only within `±1e6`** (documented constant
   `PRACTICAL_UNBOUNDED_EXTENT` in `surface.rs`) — a mathematically necessary finite-domain choice
   for an unbounded-`v` analytic surface converted to NURBS (same root cause as `Curve3::Line`
   needing an explicit domain for `to_nurbs`); not reachable from any current `engine.rs` call site
   (`scale_sync`'s `factor: f64` is always uniform, so `is_similarity()` is always `Some` on every
   path this ticket wave wires through the engine).
3. **Root-workspace `cargo check -p semio-s-plugin-stdio --lib`** was never completed (abandoned
   per coordinator instruction due to fleet lock contention) — the harness result above is offered
   as the best available substitute per that same instruction, but it is not literally the same
   build the ticket's own `📋️master-plan.md`/`📓️plan.md` verification line names.
4. Root workspace's own `serde_json` dependency status for `s.stdio.semio.brep` was not
   independently confirmed (the harness's dependency gap noted above is a harness-Cargo.toml issue,
   not necessarily a real-crate one — but I did not check the real crate's `Cargo.toml`).

## Rules followed

TDD (tests written alongside every new function). Exact math throughout — no mesh/tessellation
fallback anywhere in `transform_solid`/`transform_face`/`transform_wire`/`Curve3::transformed`/
`Surface::transformed`. Every new public function's docstring starts with a unique emoji, no
comments inside definitions. Never ran a git write command. Never touched `📌️important.md`. Never
closed/reopened the ticket. Ignored unrelated concurrent-peer churn (W1-A/C/D1/D2/E/F/G/H0/W2-A all
had live edits in the same tree during this session) except where their landed API changes required
matching call-site fixes in my OWN new test file (`make_cylinder`/`make_sphere` dropped their
`segments` parameter, `solid_volume` gained a `chord_tol` parameter — both fixed in
`diff/🔁️transform/🦀️.rs`'s tests to match the current signatures).
