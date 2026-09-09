# Boolean/Offset Kernel — Failures, Root Causes, Repairs (2026-09-09)

Scope: the BREP subset of `semio-s-artifact-stdio-semio`
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep`), specifically
`🧬️schema/🔺️diff/🔀️boolean/🦀️.rs`, `🧬️schema/🔺️diff/↔️offset/🦀️.rs`,
`🧬️schema/🔺️diff/🔺️euler/🦀️.rs`, `🧬️schema/💡️inferences/✅validation-report/🧪️body/🦀️.rs` and
`🧬️schema/⚙️engine/🦀️.rs`, driven by the two boolean procedural-3d examples
(`🧲️sphere-box-fuse`, `🍩️sphere-cut-with-torus`).

## 0. How the tests are declared and run

The boolean and offset families are **in-crate unit tests**, not `[[test]]` integration binaries:
`🔀️boolean/🦀️.rs:1512` and `↔️offset/🦀️.rs:1276` each end with a `#[path] … mod tests;` mount of
their own `🧪️tests/🔬️unit/🦀️.rs`, and the engine family likewise from
`⚙️engine/🦀️.rs`. They therefore run out of the crate's `--lib` test binary:

```sh
CARGO_INCREMENTAL=0 RUSTC_WRAPPER="" cargo test -p semio-s-artifact-stdio-semio --lib --no-run
./target/debug/deps/semio_s_artifact_stdio_semio-<hash> brep::schema::diff::boolean
./target/debug/deps/semio_s_artifact_stdio_semio-<hash> brep::schema::diff::offset
./target/debug/deps/semio_s_artifact_stdio_semio-<hash> brep::schema::engine
```

No feature flag is needed: `default = []`, and no boolean/offset/engine module is feature-gated
(`conversion-brep` only adds the DWG/STEP *codecs*).

## 1. Baseline (before any change)

| Family | Filter | Result |
| --- | --- | --- |
| boolean | `brep::schema::diff::boolean` | **8 passed, 5 failed** (24.6 s) |
| offset | `brep::schema::diff::offset` | **9 passed, 4 failed** (~30 min — two of them are pathologically slow, see §3) |
| engine | `brep::schema::engine` | **26 passed, 1 failed** (190 s) |

Raw logs: `🗑️generated/boolean-baseline.txt`, `🗑️generated/offset-baseline.txt`,
`🗑️generated/engine-baseline.txt`.

### 1.1 Boolean failures (verbatim)

```
---- …::diff::boolean::tests::tangent_spheres_union_volume_is_exact_sum ----
called `Result::unwrap()` on an `Err` value: Boolean(InvalidResult("exact boolean result failed
validation: 10 issue(s), first: shell-3-edge-2:shell-not-closed:edge is used 1 time(s) within this
shell (a closed shell needs exactly 2)"))

---- …::diff::boolean::tests::sphere_union_sphere_lens_exact_volume_and_validates ----
… "exact boolean result failed validation: 4 issue(s), first: shell-3-edge-1:shell-not-closed:
edge is used 1 time(s) within this shell (a closed shell needs exactly 2)"

---- …::diff::boolean::tests::union_and_intersect_are_commutative_by_volume ----
… "exact boolean result failed validation: 4 issue(s), first: shell-3-edge-1:shell-not-closed: …"

---- …::diff::boolean::tests::box_union_cylinder_through_exact_volume_and_validates ----
… "exact boolean result failed validation: 5 issue(s), first: edge-30:non-manifold-edge:edge is
used by 3 coedges (2-manifold shapes use at most 2)"

---- …::diff::boolean::tests::box_minus_cylinder_bore_exact_volume_and_validates ----
… "exact boolean result failed validation: 4 issue(s), first: edge-30:non-manifold-edge:edge is
used by 3 coedges (2-manifold shapes use at most 2)"
```

### 1.2 Offset failures (verbatim)

```
---- …::diff::offset::tests::offset_cylinder_matches_closed_form ----
v=12.271412208249995 expected=9.817477042468104

---- …::diff::offset::tests::shell_box_one_open_face_matches_closed_form ----
v=2.9813333333311696 expected=3.3919999999999986

---- …::diff::offset::tests::draft_box_side_face_matches_trapezoid_magnitude ----
v_plus=0.932429988162589 v0=1 expected_delta=0.10135501775433624

---- …::diff::offset::tests::thicken_planar_face_matches_box_volume ----
(FAILED only after ~30 minutes of wall time — see §3.4)
```

`offset_solid_box_round_matches_minkowski_closed_form` also ran for >15 minutes without a verdict
in the same batch; both slow cases are pathological-runtime defects, not merely wrong numbers.

### 1.3 Engine failure (verbatim)

```
---- …::engine::tests::sphere_torus_cut_produces_preview_mesh ----
cut: Operation("operation failed: imprint point does not lie on the face's boundary loop")
```

This is `euler::splice_boundary_vertex`'s only error return, reached from the seam-crossing /
chord imprint of `sphere_prim(2.2)` against `torus_prim(2.0, 0.5)` — the `🍩️sphere-cut-with-torus`
example's own configuration.

## 2. Root causes

### 2.1 Point (pole) edges were counted as an open shell — 3 of the 5 boolean failures

`make_sphere` (`🔺️diff/🧱️primitives/🦀️.rs:243`) builds the OCCT-style single-seam sphere: one
`Surface::Sphere` face whose outer ring is `[(seam, fwd), (north, fwd), (seam, rev), (south, rev)]`,
where `north`/`south` are **point edges** — `Curve3::Line { dir: ZERO }` with `v0 == v1` — that
close the `(u, v)` rectangle along `v = ±π/2`. Each appears exactly ONCE in that ring, which is the
standard representation (OCCT marks it with `BRep_Builder::Degenerated`).

`check_shell_closure_and_orientation` required *every* edge in a shell to be used exactly twice,
and `check_degenerate_geometry` additionally flagged any edge shorter than its own tolerance. A
bare sphere therefore fails `validate_body` on both counts. The boolean only surfaces it because
`exact_imprint_boolean` re-validates and its `issues_scoped_to_new_solids` filter keys on
`shell-<id>-edge-<id>` strings: the STITCHED shell has a fresh id, so the operand's pre-existing
(and in fact legitimate) pole issues reappear as new ones.

This is why the three sphere-based unions fail and why `self_boolean_identities_on_a_sphere`
passes — the latter is answered by `trivial_topology_fast_path`'s containment branch, which never
validates.

### 2.2 `box_fast_path` returned a wrong-shaped and wrong-placed box

Two independent defects in `🔀️boolean/🦀️.rs`'s box shortcut:

* `BooleanOp::Unite` returned `aabb_union(bb_a, bb_b)` for ANY pair of axis boxes that the trivial
  path had not already answered — i.e. exactly the partially-overlapping case, whose union is an
  L/cross shape, never the AABB union. The result was strictly too large.
* Both branches built the result with `make_box`, which always anchors at the ORIGIN. Whenever the
  union/intersection's min corner was not `(0,0,0)`, the shortcut returned a correctly-sized box in
  the wrong place. `overlapping_aabb_intersect_volume_matches_dims` could not see this because it
  asserts on volume only.

### 2.3 The imprint stage cannot express a multi-segment or coincident boundary

`exact_imprint_boolean` queues each clipped intersection segment independently and
`apply_pending_imprints` splices them one at a time, so `euler::split_face_by_edge` only accepts a
segment that crosses the face's own boundary at exactly TWO points. Two configurations in the
examples violate that:

* **Chains.** In `🧲️sphere-box-fuse` the sphere (r = 1.2, centred on the origin) meets the box
  (`[0, 1.5]³`, i.e. centred on the sphere's own centre at its corner) in three quarter great
  circles. On the sphere face those arcs are `u ∈ [0, π/2]` at `v = 0` and `v ∈ [0, π/2]` at
  `u = π/2`: each has ONE end on the face boundary and one end on the OTHER arc. Neither is a valid
  two-crossing chord alone; together they are one chain that is.
* **Coincidence with existing topology.** The third arc (from the box's `y = 0` face) lies exactly
  along the sphere's own seam edge (`u = 0`, `v ∈ [0, π/2]`). Imprinting it as a new edge duplicates
  existing topology; the file's own module doc already records this as an out-of-scope case that
  "surfaces as a `BooleanError` rather than silently producing a wrong result".

In `🍩️sphere-cut-with-torus` the two intersection circles (`ρ = 2.1475`, `z = ±0.4777`, derived
from `ρ² + z² = 2.2²` and `(ρ − 2)² + z² = 0.5²`) each graze BOTH operands' seams at the same
physical point `(2.1475, 0, ±0.4777)`, which is where `splice_boundary_vertex` reports "imprint
point does not lie on the face's boundary loop".

### 2.4 Offset

* `offset_cylinder_matches_closed_form` — **the test's expectation was wrong, not the kernel.** A
  SHARP solid offset moves every face along its own normal, so a cylinder grown by `d` has radius
  `r + d` AND height `h + 2d`. Closed form `π(r+d)²(h+2d) = 12.27185`; the kernel returned
  `12.271412` (4.4e-4 off, well inside the test's own 1 % band). The test asserted
  `π(r+d)²h = 9.8175`.
* `shell_box_one_open_face_matches_closed_form` (2.98133 vs 3.392) and
  `draft_box_side_face_matches_trapezoid_magnitude` (|Δ| = 0.06757 = `tan θ / 3`, expected
  `tan θ / 2`) are genuine kernel defects in `shell_solid_with_open_faces` / `draft_angle`; the
  draft ratio (a third rather than a half of the wedge) points at the drafted face being rebuilt as
  a corner pyramid rather than a full trapezoidal prism.
* `thicken_planar_face_matches_box_volume` and `offset_solid_box_round_matches_minkowski_closed_form`
  are runtime bombs: NURBS ruled side faces measured at `chord_tol = 1e-6` drive
  `segments_for_chord_deviation` into thousands of boundary samples and `ear_clip`'s O(n³) worst
  case — the same hazard `stitch_selected_faces` already documents for its own sign probe.

## 3. Repairs

All paths below are relative to
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/`.

| # | File:region | Change |
| --- | --- | --- |
| R1 | `🧬️schema/💡️inferences/✅validation-report/🧪️body/🦀️.rs` — new `is_point_edge`, used in `check_shell_closure_and_orientation` and `check_degenerate_geometry` | A POINT edge (`v0 == v1` and a curve whose whole range has zero arc length — a sphere's collapsed poles) is exempt from the two-uses-per-shell rule and from the sliver check. Derived intrinsically from the geometry rather than persisted, so no schema change; a full-period seam edge (also `v0 == v1`, but non-zero length) stays under the ordinary rule. |
| R2 | `🧬️schema/💡️inferences/📏mass-properties/🦀️.rs` — `loop_uv_polygon` + new `UV_WELD` | Consecutive duplicate UV vertices (and a duplicated closing vertex) are welded out. A repeat is fatal, not redundant: `ear_clip`'s blocking test is `point_in_or_on_triangle`, so a duplicate of an ear's OWN corner blocks that ear — with every ear blocked the triangulation returns empty and the face measures ZERO area. |
| R3 | `🧬️schema/🔺️diff/🔺️euler/🦀️.rs` — `split_face_by_edge` → `split_face_by_chain` (+ new `oriented_endpoints`) | The chord splitter now takes a CHAIN of oriented `ParametricEdge`s instead of one edge, splices the chain's two END vertices into the boundary, and closes both new rings with the whole chain (reversed on one side). A single-segment chain behaves exactly as before. |
| R4 | `🧬️schema/🔺️diff/🔀️boolean/🦀️.rs` — new `welded_imprint_vertex`, `build_imprint_edge` reworked, `weld` seeded from both operands' boundary vertices in `exact_imprint_boolean` | Imprint endpoints are welded against each other and against existing operand vertices, so arcs that meet physically share one `VertexId` (the precondition for R3's chains) and an arc ending on a pole reuses that pole's vertex. |
| R5 | `🧬️schema/🔺️diff/🔀️boolean/🦀️.rs` — new `chain_open_pendings`, `locate_active_piece`; `apply_pending_imprints` rewritten | Open segments are assembled into maximal end-to-end chains before splitting; closed ones keep the interior / seam-crossing route unchanged. A component whose every vertex has degree 2 (a piecewise-closed imprint) is reported rather than mis-split. |
| R6 | `🧬️schema/🔺️diff/🔀️boolean/🦀️.rs` — new `coincident_boundary_edge`, `boundary_vertex_at`, `boundary_subedge`, `oriented_prange`; call site in `exact_imprint_boolean` | When a clipped segment runs ALONG one operand's own boundary edge, that edge is subdivided and REUSED as the shared imprint edge and the coincident side queues no imprint — instead of minting a duplicate, non-manifold edge. |
| R7 | `🧬️schema/🔺️diff/🔀️boolean/🦀️.rs` — `box_fast_path` (+ new `boxes_union_is_a_box`) | `Unite` now returns early ONLY when the two axis boxes' union really is a box (they agree on two axis intervals and merely extend the third); everything else goes to the general engine. Both branches now build the result AT the union/intersection's min corner (`make_box` always builds at the origin, so the shortcut used to return a right-sized box in the wrong place — invisible to a volume-only assertion). |
| R8 | `🧬️schema/🔺️diff/🔀️boolean/🦀️.rs` — `stitch_selected_faces` | Assembles ONE solid from all connected groups: a group a containment test places inside another becomes an inverted void shell, everything else joins the outer shell. It used to return only the largest group and leave the rest live-but-unreturned, which silently dropped every other lump. |
| R9 | `🧬️schema/🔺️diff/🔀️boolean/🦀️.rs` — `LOCAL_RAY_RETRY_DIRS` | All six retry directions had a POSITIVE `z`, so every ray left the query point through the same hemisphere and one damaged face there lost the whole vote at once. Half are negated so the set spans both hemispheres. |
| R10 | `🧬️schema/🔺️diff/🔀️boolean/🦀️.rs` — `exact_imprint_boolean`'s validation error; `…/🧪️body/🦀️.rs` — `same-parameter-violated` message | The boolean's failure now lists EVERY validation issue rather than only the first, and the same-parameter issue names the face, edge, `prange` and `range`. Both were needed to diagnose §2 at all and are kept. |
| R11 | `🧬️schema/🔺️diff/↔️offset/🧪️tests/🔬️unit/🦀️.rs` — `offset_cylinder_matches_closed_form` | Corrected the test's own closed form to `π(r+d)²(h+2d)`; see §2.4. |
| R12 | `🧬️schema/💡️inferences/📏mass-properties/🦀️.rs` — `ear_clip` | When no ear can be found the loop used to `break`, silently returning a PARTIAL triangulation whose every integrated moment is short by the abandoned tail. It now clips the FLATTEST remaining corner instead (its own triangle contributes ~nothing to the integral, and removing it always makes progress), so the polygon is always fully consumed. This is the companion to R2: once R2 stopped duplicate vertices from blocking every ear, the runs of exactly-collinear boundary samples that every dense curve discretisation produces became the next thing that could stall the clip. |

New tests:

* `🧬️schema/🔺️diff/🔀️boolean/🧪️tests/🔬️unit/🦀️.rs` — `analytic_primitives_are_structurally_valid`:
  every analytic primitive must pass `validate_body` on its own. This is the regression guard for R1
  and R2 (a sphere used to report `shell-not-closed` ×2, `degenerate-edge` ×2 and `sliver-face`).
* `🧪️tests/🧲️procedural-example-booleans/🦀️.rs` (declared as `[[test]] brep_procedural_example_booleans`
  in the crate's `Cargo.toml`) — the two example configurations at kernel level, each asserting
  `validate_body` silence, χ = 2, our own volume against an independent oracle, and `parry3d`'s own
  `trimesh_signed_volume_and_center_of_mass` over the tessellated result (positive, so an inverted
  or open shell fails). `parry3d = "0.17"` was added to `[dev-dependencies]` — test-only, the same
  crate and version `semio-framework-3d` already uses as its collision oracle, so
  `🔒️dependencies.json`'s freeze check (which only rejects NEW dependency names) still passes.
  Expected volumes: fuse `V_sphere + a³ − V_sphere/8 = 9.70845` (closed form — with `r = 1.2 < a = 1.5`
  the box contains exactly one octant of the sphere, which is centred on the box's own corner);
  cut `4/3·π·2.2³ − 6.76110 = 37.84114` (axisymmetric quadrature with the radial extent integrated
  in closed form per `z` slice). Both tests currently FAIL, on the boolean defect in §4.3 — they are
  the executable statement of what is still missing.

### 3.1 The standalone kernel harness

`🔬️harness/{Cargo.toml,lib.rs}` in this ticket folder mounts the real kernel sources at exactly the
module positions the crate root gives them (`crate::standards::v1::subsets::brep::schema::…`) and
supplies its OWN `snapshot`/`diff`/`inferences` roots, which is the whole point: only those roots
pull `semio-framework-plugin`/`-schema`, and neighbouring sessions left those crates uncompilable
for most of this ticket's window (`MEMBER_OPEN_IDENTITY_BYTES`, `MemberFactory::Open`,
`ChildMemberRegistry`, `ArtifactChildOwner` — none of them BREP). It also sidesteps the root
`target/` lock, which was continuously held by other sessions' `cargo test` runs.

Result: a full kernel edit→test cycle costs **~30 s of compile** instead of the 20–60 min the real
crate needed, and it never blocks on a peer's half-finished refactor. Successor to
`26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME/🔬️harness`, whose `#[path]`s (`✳️brep`,
`crate::artifacts::semio::…`) no longer match the tree.

```sh
cd '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🔬️harness'
CARGO_TARGET_DIR=<scratch>/target-harness RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 \
  cargo test --lib --no-run
<scratch>/target-harness/debug/deps/brep_kernel_harness-<hash> --test-threads 4 diff::boolean
```

The harness is a MOUNT, never a copy — it always compiles whatever is on disk.

## 4. After

| Family | Before | After |
| --- | --- | --- |
| boolean | 8 passed / **5 failed** | 12 passed / **2 failed** |
| offset | 9 passed / **4 failed** | 10 passed / **3 failed** (`offset_cylinder_matches_closed_form` fixed) |
| engine | 26 passed / **1 failed** | 26 passed / **1 failed** |
| new `analytic_primitives_are_structurally_valid` | — | passes (would have failed before R1/R2) |
| new `brep_procedural_example_booleans` (2 tests) | — | fails, see §4.3 |

(The boolean family's total changes from 13 to 14 because the three throw-away debug probes used
during the investigation were replaced by one permanent primitive-validity test.)

Fixed by these repairs: `sphere_union_sphere_lens_exact_volume_and_validates`,
`union_and_intersect_are_commutative_by_volume`, `tangent_spheres_union_volume_is_exact_sum`,
`offset_cylinder_matches_closed_form`. Raw logs: `🗑️generated/boolean-after.txt`,
`🗑️generated/engine-after.txt`.

### 4.1 Compilation

* `cargo check -p semio-s-artifact-stdio-semio --lib --keep-going` — **exit 0, 0 warnings**, and the
  log shows the crate actually recompiled (`Checking semio-s-artifact-stdio-semio`), not a cache
  hit. Receipt: `🗑️generated/native-check-lib-after.txt`.
* `cargo check -p semio-s-artifact-stdio-semio --test brep_procedural_example_booleans` — exit 0,
  no warnings: the new `[[test]]` target and its `parry3d` oracle type-check against the real crate.
* `cargo check -p semio-s-artifact-stdio-semio --all-targets` reports 3 errors, all in the
  `📦️object` subset's own tests (`no method named expect found for Vec<u8>`, `FaultCode doesn't
  implement Display`) — a neighbouring session's in-flight `pack`/`FaultCode` change, no BREP file
  involved.
* `cargo check -p semio-s-artifact-stdio-semio --lib --keep-going --target wasm32-wasip2 --profile
  wasm-dev` did NOT reach this crate: it stops in `semio-framework-plugin` with
  `error[E0639]: cannot create non-exhaustive variant using struct expression` at
  `🌐host/🦀️.rs:558` (`Effect::InvokeExtension { … }`) — again a neighbouring session's in-flight
  change, no BREP file involved. Receipt: `🗑️generated/wasm-check-blocked.txt`. The wasm gate has to
  be re-run once that crate compiles again; nothing in this ticket's diff is target-conditional
  (no `#[cfg(target_arch)]` was added or touched), so the native check is the strongest evidence
  currently obtainable.

### 4.2 Remaining: box ∪ / − cylinder (2 boolean tests)

Both fail identically, and the cause is now pinned down rather than guessed:

```
edge-30:non-manifold-edge:edge is used by 3 coedges
coedge-60:same-parameter-violated:pcurve and 3D curve disagree by 1 at s=1
    (tol 1e-7; face-6 edge-30 prange (4.71238898038469, 10.995574287564276)
                          range  (4.71238898038469, 10.995574287564276))
shell-4-edge-30:shell-not-closed:edge is used 3 time(s) within this shell
```

Instrumented trace (temporary `[DEBUG]` probes, since removed): the box's bottom face splits
correctly into an annulus and a disc, and the disc's own classification sample point is exactly the
disc centre `(0, 0, -0.5)` — but `local_point_in_solid` against the already-imprinted cylinder
returns `Outside`, so BOTH halves are kept and the imprint edge ends up with three coedges. Ray by
ray:

```
ray face-15 t=0.5338 uv=(0.9118,1.6873) in_trim=false     ← v=1.687 ⇒ z=-0.31, the MIDDLE piece
ray face-20 t=0.5338 uv=(0.9118,1.6873) in_trim=false
ray face-21 t=0.5338 uv=(0.9118,1.6873) in_trim=false
ray face-20 t=0.7436 uv=(5.2614,0.9493) in_trim=true      ← v=0.949 ⇒ z=-1.05, the LOWER piece: OK
```

So the three lateral pieces do not cover a point that is plainly on the cylinder: the piece produced
by the SECOND `split_face_by_seam_crossing` has a UV trim that rejects everything, and the two
`same-parameter-violated` coedges on the imprint circle say its p-curve and 3D curve part company at
exactly the closure parameter (deviation `1.0` = the circle's own diameter, `2r`). The defect is
therefore in the seam-crossing splitter's handling of a SECOND split on an already-split periodic
face — not in classification, not in the ray directions, and not in the parts repaired above. R9
made the symptom symmetric (both discs now misclassify) rather than accidentally right for one of
them, which is the honest state.

### 4.3 Remaining: the two procedural examples

Neither example works yet. The new `[[test]]` target builds and runs against the real crate
(`target/debug/deps/brep_procedural_example_booleans-…`, receipt
`🗑️generated/example-tests-after.txt`):

```
test sphere_cut_with_torus_example_is_a_closed_oriented_solid ... FAILED
test sphere_box_fuse_example_is_a_closed_oriented_solid ... FAILED
  cut:  Operation("imprint point does not lie on the face's boundary loop")
  fuse: Operation("imprint point does not lie on the face's boundary loop")
test result: FAILED. 0 passed; 2 failed
```

Both die in `euler::splice_boundary_vertex`. The instrumented imprint trace for
`🧲️sphere-box-fuse` explains exactly why, and the reasons are specific:

```
fa=sphere fb=box-bottom(z=0) t=(3.1416,4.7124) p0=(1e-6, 1.2, 0)          p1=(1.2, 1.2e-6, 0)
fa=sphere fb=box-front (y=0) along_a=Some(edge-0)  t=(4.7134,4.9051)      ← fragment 1 of 3
fa=sphere fb=box-front (y=0) along_a=Some(edge-18) t=(4.9126,5.0925)      ← fragment 2 of 3
fa=sphere fb=box-front (y=0) along_a=Some(edge-20) t=(5.1211,5.2599)      ← fragment 3 of 3
fa=sphere fb=box-left  (x=0) t=(3.2398,4.7124) p0=(0, 0.1176, 1.1942)     p1=(0, 1.2, 1e-6)
```

1. **The clip's endpoints are only tolerance-accurate.** `refine_boundary` bisects an inside/outside
   predicate, so the shared corner of two arcs comes out as `(1e-6, 1.2, 0)` from one face pair and
   `(0, 1.2, 1e-6)` from the other — `1.4e-6` apart, just outside R4's weld radius, so the chain
   never joins. The fix is to solve for the boundary crossing rather than bisect a predicate (or,
   less cleanly, to widen the weld to the clip's own accuracy).
2. **The pole is never reached.** The `x = 0` arc stops at `(0, 0.1176, 1.1942)` instead of the north
   pole `(0, 0, 1.2)`, because the trim predicate fails in the pole's neighbourhood — so the chain's
   far end is not on the face boundary at all.
3. **A coincident arc fragments.** The `y = 0` plane's arc lies exactly on the sphere's own seam;
   R6 correctly recognises it (`along_a=Some`), but the clip returns it as THREE short runs rather
   than one, because the trim predicate is borderline all along a curve that sits on the boundary.

R3–R6 are the machinery those examples need (chains, welded vertices, coincident-edge reuse) and
they are in place; what is still missing is a clip stage that is exact at the trim boundary, at a
periodic seam, and at a pole. `🍩️sphere-cut-with-torus` fails at the same splice call: its two
intersection circles (`ρ = 2.1475`, `z = ±0.4777`) graze both operands' seams at the same physical
point, and the imprint vertex placed there does not land on either seam edge within tolerance.

### 4.4 Remaining: offset

* `shell_box_one_open_face_matches_closed_form` — `v = 2.98133` vs `3.392`; the shelled cavity comes
  out as `5.01867` instead of `4.608`. Untouched by this ticket.
* `draft_box_side_face_matches_trapezoid_magnitude` — `|Δv| = 0.06757 = b·c²·tanθ/3`, expected
  `b·c²·tanθ/2`. A third rather than a half of the wedge is the signature of the drafted region
  being rebuilt as a corner PYRAMID rather than a trapezoidal prism, i.e. the vertex displacement
  moves the top corners in two axes where it should move them in one.
* `thicken_planar_face_matches_box_volume` and
  `offset_solid_box_round_matches_minkowski_closed_form` are runtime bombs, not just wrong numbers:
  the first took ~30 minutes of wall time before failing, the second never returned inside a
  15-minute window. NURBS ruled side faces measured at `chord_tol = 1e-6` drive
  `segments_for_chord_deviation` into thousands of boundary samples and `ear_clip`'s O(n³) worst
  case — the same hazard `stitch_selected_faces` already documents for its own sign probe, which
  solved it by dropping the probe tolerance to `1e-3`.

### 4.5 Two further primitive defects found in passing

Both surfaced from the new `analytic_primitives_are_structurally_valid` probe and are recorded here
rather than fixed, being outside the boolean path:

* **`make_cone` builds an inward-oriented shell.** `validate_body` on a bare
  `make_cone(1.0, 2.0)` reports `solid-0:shell-orientation-inward: outer shell's signed volume is
  negative (-2.0944)`; the magnitude is exactly `π·r²·h/3`, so the geometry is right and only the
  orientation is inverted. The cone is not on either example's op chain.
* **`OPERATION_QUALITY` still tags `sphere_prim`/`cylinder_prim`/`cone_prim`/`torus_prim` as
  `MeshDerivedBRep`** (`⚙️engine/🔖️contract/🦀️.rs:241-244`) although the W1-E rewrite made all four
  exact analytic — already noted as gap 6 in `📓️kernel-and-preview-audit-2026-09-09.md`, still
  unfixed, and still misleading anyone auditing kernel fidelity from the table alone.

## 5. Resumed 18:00 — both examples exact

The lane above was killed by a machine reboot. Everything it left on disk was verified present
(`git status --porcelain` / `git diff HEAD --stat` on the paths §3 names) and re-measured before
anything new was built on it; §4's after-state reproduced exactly (boolean 12/2, engine 26/1,
examples 0/2), so the numbers below are relative to that.

The harness (§3.1) was extended to mount the crate's own `[[test]]` example target as a unit-test
module (`extern crate self as semio_s_artifact_stdio_semio;` makes the file's absolute `use` paths
resolve unchanged), which brought the two examples into the same ~20 s edit→test cycle as
everything else, plus a `ticket_probe` module with the face-by-face dumps the offset work needed.

### 5.1 After

| Family | §4 after | Now |
| --- | --- | --- |
| `brep_procedural_example_booleans` (the two examples) | 0 passed / **2 failed** | **2 passed / 0 failed** |
| boolean | 12 passed / **2 failed** | **14 passed / 0 failed** |
| engine | 26 passed / **1 failed** | **27 passed / 0 failed** |
| offset | 10 passed / **3 failed** (+2 runtime bombs) | **12 passed / 1 failed** (one bomb left, §5.6) |
| euler | — | **10 passed / 0 failed** |
| primitives | — | **13 passed / 0 failed** |
| inferences (classification, mass-properties, tessellation, validation) | — | **72 passed / 0 failed** |
| snapshot (arena, curve, surface, topology, tolerance, …) | — | **219 passed / 0 failed** |
| sweep / intersect / blend / sew / transform | — | **13 / 38 / 7 / 8 / 7 passed, 0 failed** |

`sphere_box_fuse_example_is_a_closed_oriented_solid` and
`sphere_cut_with_torus_example_is_a_closed_oriented_solid` now assert and get: `validate_body`
silent, χ = 2, our own `solid_volume` within 5e-3 of the independent oracle, and `parry3d`'s
`trimesh_signed_volume_and_center_of_mass` over the tessellated result POSITIVE and within 2e-2 —
i.e. closed, 2-manifold, coherently oriented, outward, and the right size, cross-checked by a
third-party integrator. No soup, no hull, no fallback: `boolean_solid` reaches
`exact_imprint_boolean` for both.

### 5.2 The clip stage (§4.3's three symptoms, one cause)

`point_in_face_uv` returns `status == Inside`, so a point ON the trim boundary reads as OUTSIDE.
Every one of §4.3's symptoms follows from the clip having been built on it:

* a sphere's POLE *is* its own `v = ±π/2` boundary, so the clip stopped one sampling cell short of
  it (`(0, 0.1176, 1.1942)` instead of `(0, 0, 1.2)`);
* a periodic SEAM is the ring's own edge, so an arc could never reach it;
* an arc running exactly ALONG an operand's boundary is on it at every sample, so it shredded into
  three borderline fragments.

| # | Change | File |
| --- | --- | --- |
| R13 | New `point_in_face_uv_closure` (`status != Outside`) and its periodic wrapper; the clip's `valid` predicate now tests the trim's CLOSURE. Membership of the closure is what "still on this face" means; deciding which SIDE of a shared boundary something is on stays the open test's job. | `💡️inferences/🏷️classification`, `🔀️boolean` |
| R14 | New `snap_clip_endpoint`: alternating projection (project the bisected endpoint onto the boundary curve it is nearest, re-solve the intersection curve's own parameter for that point, repeat) pulls each clip endpoint onto the boundary EXACTLY. `refine_boundary` could only ever locate a crossing to within `tol`, and the two faces of a pair flip in opposite directions, so the same physical corner arrived from two face pairs `1.4e-6` apart — outside the `1e-6` weld radius, which is why the arcs never chained. Endpoints now land at 1e-16. | `🔀️boolean` |
| R15 | New `boundary_touch_parameters` + `golden_section_minimum`: a closed curve's seam touches are now SOLVED for (distance to the supports' boundary curves, local minima refined by golden section) instead of read off gaps in the trim sampling — which the closure predicate no longer produces, and which were never better than the sampling that made them. | `🔀️boolean` |
| R16 | New `pcurve_for_clip` + `affine_pcurve_through`: an SSI p-curve is certified over the curve's FULL domain and a boolean only ever imprints a clipped SUB-range. A global interpolation can be arbitrarily wrong on one — a great circle through a sphere's poles has a `u` that jumps by π there, so the fit oscillated by 0.14 on an r = 1.2 sphere while still passing through every one of its own 33 nodes (hence reporting ~1e-16 to the sampling that produced it). Measured on the range actually used and, when it misses, rebuilt there by exact inversion (poles' undefined `u` carried from the neighbour, periodics unwrapped) and an AFFINE fit — which is not a simplification but the exact answer for every sub-arc this stage produces (latitude circle, meridian branch, ruling). A genuinely non-affine sub-range keeps the original rather than trading one wrong answer for another. | `🔀️boolean` |

### 5.3 The chord splitter and the periodic ring (§4.2 and §4.3 shared this)

| # | Change | File |
| --- | --- | --- |
| R17 | `edge_param_at_point`'s `Curve3::Line` branch returned the projection parameter WITHOUT checking the point is on the line. `splice_boundary_vertex` splits the first ring edge whose parameter lands strictly inside, so the sphere's north pole was spliced into the box's `x = 1.5` face — a face it is nowhere near — because it happens to project onto that face's own edge at an interior parameter. The non-line branch had always checked its distance; this one now does too. | `🔺️euler` |
| R18 | New `member_uv`, `ring_index_at_uv`, `splice_boundary_vertex_at_uv`, `chain_endpoint_uv`; `split_face_by_chain` locates its chord ends by `(u, v)` instead of by vertex id. A periodic face's seam vertex legitimately appears TWICE on its own ring (`u = 0` and `u = 2π`), and so does a pole; taking the first occurrence partitioned the face the wrong way round half the time, handing the piece on the `u > 0` side the `u = 2π` boundary. The p-curve says unambiguously which occurrence the chord meets. The UV route also handles a chord ending on a DEGENERATE edge (a pole), where the 3D splice has nothing to solve and stops: in `(u, v)` that edge is an ordinary segment and the arrival is an ordinary interior parameter on it. | `🔺️euler` |
| R19 | `split_face_by_seam_crossing` pushed its closed imprint edge as `forward = false` on one piece and `true` on the other, FIXED. Topology cannot choose (a closed edge has `v0 == v1`, so both senses connect) but the p-curve can, and must: the edge has to continue from where the boundary chain it closes left off. Fixing it was right half the time; the other half handed one piece a reversed ring, which then traversed every edge it shared with the other operand the same way that operand did. | `🔺️euler` |
| R20 | New `align_pcurve_branch` + `Curve2::translated`: a closed imprint's p-curve is re-expressed in the same periodic BRANCH as the target face's own ring before splitting. `🍩️sphere-cut-with-torus`'s second intersection circle arrived at `u ∈ [2π, 4π]`, `v = −1.271` against a ring written on `u ∈ [0, 2π]`, `v ∈ [0, 2π]`; imprinted unchanged, the resulting face's UV polygon had its pieces sitting periods apart — a shape with no interior at all, which `interior_point_of_face` then correctly refused to sample. Applied to closed imprints only: an open chain's members can legitimately have been born in different branches from EACH OTHER, so a single group shift would tear it; those are aligned per traversal at split time instead. | `🔀️boolean`, `📸️snapshot/➰️curve` |

### 5.4 Orientation, poles, and the p-curve sense (found by finishing the two examples)

| # | Change | File |
| --- | --- | --- |
| R21 | `make_sphere`/`make_torus` wound their outer ring CLOCKWISE in `(u, v)` while `make_box`/`make_cylinder`/`make_cone` wind counter-clockwise, and every one of those surfaces has `du × dv` pointing outward. Nothing caught it: a solid whose faces agree WITH EACH OTHER validates whichever convention it picked, and a sphere has one face. It surfaces the instant a boolean makes a sphere piece adjacent to a box piece. Both rewound (members and p-curves reordered, no geometry changed). | `🧱️primitives` |
| R22 | `make_cone`'s lateral frame paired `z = −Z` with an unmirrored `y = Y` — a LEFT-handed frame, which negates `du × dv` and built the whole face with an inward normal (`shell-orientation-inward`, signed volume exactly `−πr²h/3`). Mirrored `y`, the same right-handed reflection its own base cap already used; the base circle's p-curve now states the resulting backwards azimuth explicitly, and the two seam traversals get their own `u = 0` / `u = 2π` p-curves like the sphere's. §4.5's first item. | `🧱️primitives` |
| R23 | `check_shell_closure_and_orientation` compared raw `forward` flags. A coedge's `forward` is stated in its SURFACE's sense; the face's outward normal is that sense only when `flipped` is false, so the flag to compare is `forward XOR flipped`. Every boolean `Cut` reported `orientation-inconsistent` — a cut flips the tool's faces to face into the cavity and their rings quite correctly stay put. | `✅validation-report/🧪️body` |
| R24 | New `Surface::is_degenerate_uv`, used everywhere a pole was previously detected via `normal(u, v).is_none()`. That asks a different question with an ABSOLUTE threshold (`Vec3::normalized` rejects at `f64::EPSILON`), so a pole was detected on a UNIT sphere (`\|du × dv\| ≈ 6e-17`) and MISSED on a radius-2.2 one (`≈ 3e-16`) — the same geometry, decided by scale. A missed pole makes the two `u` branches meeting there unwrap onto one another and the face measure ZERO area: the last thing standing between `🍩️sphere-cut-with-torus` and a clean `validate_body`. | `📸️snapshot/🏄️surface` + `📏mass-properties`, `🧩tessellation`, `🔀️boolean` |
| R25 | New `planar_pcurve_sense`: `build_pcurve`'s `(Plane, Circle/Ellipse)` shortcut ignored handedness. `Curve2::Ellipse::eval` derives its second axis as a FIXED right-handed `x_axis.perp()`, so when the conic's normal runs anti-parallel to the plane's — a cylinder's intersection circle against a box's bottom face — every parameter mapped to the ANTIPODE, `2r` away. Negating `minor_radius` flips exactly that term, the same signed-radius convention `offset::exact_pcurve_for_circle_on_plane` already documents. This was §4.2's `same-parameter-violated ... disagree by 1`. | `✂️intersect/🏄️surface-surface` |
| R26 | `analytic_primitives_are_structurally_valid` now covers the CONE too and asserts each primitive's closed-form volume WITH ITS SIGN — a positive magnitude alone proves nothing about orientation, and the inward cone had exactly the right magnitude. | `🔀️boolean/🧪️tests/🔬️unit` |
| R27 | `OPERATION_QUALITY` now tags `sphere_prim`/`cylinder_prim`/`cone_prim`/`torus_prim` `ExactAnalytic`; the table's own header paragraph was rewritten to describe the current pipeline rather than the pre-W1-E/W2-B one (patterns/defeature/split/mesh IO are what is still `MeshDerivedBRep`). §4.5's second item. | `⚙️engine/🔖️contract` |

### 5.5 Offset (§4.4)

§4.4's diagnosis — `ear_clip`'s O(n³) on dense `chord_tol = 1e-6` boundaries — was not what the
profiler found. A live `sample(1)` of the hung `thicken_planar_face_matches_box_volume` puts 100 %
of its time in `set_face_pcurves → Surface::project_curve → fit_pcurve`, which on a NURBS support
inverts up to 1025 samples through a `1e-9` Bézier-subdivision `closest_uv` and re-interpolates the
lot, once per doubling, per edge, per face.

| # | Change | File |
| --- | --- | --- |
| R28 | New `exact_pcurve_on_ruled_boundary`, first in the `exact_pcurve` chain: a boundary that is an ISO-LINE of its own NURBS patch — which every boundary of a ruled/lofted patch is, those patches being built ALONG their boundaries — has a straight `(u, v)` image and needs no fit at all. Ten candidates, each VERIFIED against the real curve before it is returned, so a boundary whose parameter is not affine in the patch's (a circular rail, whose `to_nurbs` parameter is a Möbius map of angle) still falls through to the fit. `thicken_planar_face_matches_box_volume` went from ≫30 min to **0.01 s**. | `↔️offset` |
| R29 | New `offset_point_map`; `offset_face` now REBUILDS its boundary through it instead of sharing the original face's edges. Sharing them left a face whose surface had moved and whose rim had not — `thicken_face` then ruled every side between an edge and ITSELF, so all four sides were degenerate and zero-area and the thickened box measured a volume of exactly 0. The map is exact and affine for `Plane` (translation), `Sphere` (uniform scale about the centre), `Cylinder` (scale about the AXIS — affine, not a similarity) and `Cone` (apex translation); a `Torus` tube offset is not affine, and `offset_face` reports rather than guessing. | `↔️offset` |
| R30 | `thicken_face` reversed the FAR cap. The solid grows along the original face's outward normal, so the offset cap already faces outward and it is the ORIGINAL that now faces into the new material; reversing the wrong one left both caps pointing the same way and their contributions cancelled (a 2 × 1 × 0.5 thickened quad measured 1/3 of its volume). | `↔️offset` |
| R31 | `rebuild_topology` skipped every edge whose two faces were untouched — even when one of its ENDS had moved, so the vertex it stopped at no longer existed on the solid. A drafted unit box came out with TEN vertices, its far edges still ending at the original corner while the drafted face ended at the new one; the volume landed between the two shapes (0.9324 where the trapezoid is 0.8986 — §4.4's "pyramid rather than prism", which was really "half of each"). Edges are now rebuilt when a face changed OR an end moved, and the vertex lookups fall back to the current position for the end that did not move. | `↔️offset` |
| R32 | `shell_solid_with_open_faces` offset the OPEN faces' surfaces too. An opening is not a wall: the cavity runs up to it. A 2³ box shelled at 0.2 with its top open came out with a 1.6-tall cavity instead of 1.8 and a slanted ruled rim instead of the flat frame the opening actually is. Open faces keep their surface (and still trim the inner faces that meet them), and the vertex target now solves each touched face's OWN final plane — displaced for a wall, not displaced for an opening — instead of dragging every rim corner inward along all three normals. | `↔️offset` |

Fixed by R28–R32: `thicken_planar_face_matches_box_volume`, `shell_box_one_open_face_matches_closed_form`,
`draft_box_side_face_matches_trapezoid_magnitude` — offset is now 12 passed / 1 failed.

### 5.6 Remaining: `offset_solid_box_round_matches_minkowski_closed_form`

Still a runtime bomb (> 15 min, no verdict), and for a reason R28 cannot reach.
`offset_solid_with_corner(Round)` delegates its corners to `blend::fillet_edges`, whose rolling-ball
patches are `Surface::Nurbs` whose boundaries are NOT affine iso-lines, so every one of them goes
through `fit_pcurve`. A `sample(1)` of the hung run puts its time in
`bspline::basis_function_derivatives` / `surface_derivatives_rational` /
`surface_ops::closest_on_nurbs_surface` under `set_face_pcurves` — the same fit, now unavoidable.

The exact surgery is the one the ticket names: a rolling-ball edge blend between two planes IS a
cylinder and a vertex blend IS a sphere, and built as `Surface::Cylinder`/`Surface::Sphere` they
need no p-curve fit at all (`exact_pcurve_for_circle_on_cylinder` / `_line_on_cylinder` already
cover their boundaries) and the Minkowski closed form falls out exactly. That is a rewrite of
`diff::blend`'s patch construction, not of `diff::offset`, and is left as the one open item.

### 5.7 Compilation

All three in a private `CARGO_TARGET_DIR` seeded by an APFS clone of the repo's `target/debug`,
`RUSTC_WRAPPER=""`, and each log shows the crate genuinely recompiling (`Checking
semio-s-artifact-stdio-semio`), not a cache hit:

* `cargo check -p semio-s-artifact-stdio-semio --lib --keep-going` — **exit 0, 0 errors, 0 warnings**.
* `cargo check -p semio-s-artifact-stdio-semio --lib --keep-going --target wasm32-wasip2 --profile
  wasm-dev` with `CARGO_PROFILE_WASM_DEV_DEBUG=false` — **exit 0, 0 errors, 0 warnings**. (§4.1's
  blocker, `semio-framework-plugin`'s `E0639`, is gone — that neighbouring session's change landed.)
* `cargo check -p semio-s-artifact-stdio-semio --test brep_procedural_example_booleans` — **exit 0,
  0 errors, 0 warnings**: the `[[test]]` target and its `parry3d` oracle type-check against the real
  crate, which is where the two example tests live for CI.

### 5.8 Raw logs

`🗑️generated/resumed-*.txt` — one per family (`resumed-boolean-after.txt`, `resumed-ex-after.txt`,
`resumed-engine-after.txt`, …), the three compilation receipts
(`resumed-native-check.txt`, `resumed-wasm-check.txt`, `resumed-example-test-check.txt`) and the two
`sample(1)` profiles that pinned the offset runtimes (`resumed-thicken-profile.txt`,
`resumed-round-minkowski-profile.txt`). `resumed-offset-after.txt` is truncated at the one test that
never returns (§5.6).

### 5.9 Files changed in this pass

`🧬️schema/🔺️diff/🔀️boolean/🦀️.rs`, `…/🔀️boolean/🧪️tests/🔬️unit/🦀️.rs`,
`🧬️schema/🔺️diff/🔺️euler/🦀️.rs`, `🧬️schema/🔺️diff/🧱️primitives/🦀️.rs`,
`🧬️schema/🔺️diff/↔️offset/🦀️.rs`, `🧬️schema/🔺️diff/✂️intersect/🏄️surface-surface/🦀️.rs`,
`🧬️schema/💡️inferences/🏷️classification/🦀️.rs`, `🧬️schema/💡️inferences/📏mass-properties/🦀️.rs`,
`🧬️schema/💡️inferences/🧩tessellation/🦀️.rs`,
`🧬️schema/💡️inferences/✅validation-report/🧪️body/🦀️.rs`,
`🧬️schema/📸️snapshot/🏄️surface/🦀️.rs`, `🧬️schema/📸️snapshot/➰️curve/🦀️.rs`,
`🧬️schema/⚙️engine/🔖️contract/🦀️.rs` (all under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/`), plus this
ticket's `🔬️harness/lib.rs`.
