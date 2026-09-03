# 📓️ W2-B — Exact boolean pipeline (imprint → classify → select → stitch)

Worker W2-B. Files owned and edited:
`D = ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff`
- `D/🔀️boolean/🦀️.rs` — full rewrite.
- `D/🔺️euler/🦀️.rs` — extended (new imprint primitives + a real pre-existing bug fix).
- `D/🧵️sew/🦀️.rs` — unchanged this pass (see §6).
- `⚙️engine/🔖️contract/🦀️.rs` — one targeted edit (`operation_quality` table, item 7 of the brief).
- `💡️inferences/🏷classification/🦀️.rs` — THREE functions fixed, outside nominal ownership; see §5.

## 1. Pipeline

`boolean_solid(body, a, b, op, tol, rec)`:
1. **Trivial fast path** (`trivial_topology_fast_path`): disjoint (AABB gap ≥ tol) or fully
   contained (AABB containment + a real `point_in_solid` boundary-point proof) — works for ANY
   solid shape; both branches are genuine analytic results (a union-of-shells or an outer+void
   solid), never mesh-derived.
2. **Box fast path** (`box_fast_path`): when BOTH operands are confirmed axis boxes (6 faces,
   volume == AABB volume), `Unite`/`Intersect` reduce to a single fresh `make_box` — exact
   analytic, cheaper than the general engine on 12 coplanar face pairs. `Cut` is left to the
   general engine. Subsumes the old `aabb_fast_path`, with one correctness fix: the old
   `Intersect` branch built a box from the AABB overlap unconditionally, even for non-box operands
   (wrong for e.g. sphere ∩ box) — now gated behind the same box-confirmation `Unite` already used.
3. **General exact engine** (`exact_imprint_boolean`), replacing the tessellate-and-classify
   pipeline as the default for everything else:
   - Coincident-face detection (`find_coincident_face_pairs`) collapses same-surface,
     same-boundary duplicate face pairs to one kept face before imprinting.
   - Face-pair candidates via AABB overlap (`bounding_volume::face_aabb`); SSI per pair via
     `intersect_surface_surface` (W2-A's `IntCurve`).
   - `clip_intcurve_to_faces` samples the shared domain, testing `point_in_face_uv` on both
     supports (periodic `u`/`v` wrapped first). A periodic curve whose valid runs cover ~100% of
     the period (allowing for a narrow gap where it grazes a seam at an ARBITRARY phase, not just
     `t=lo`) collapses to one closed range, carrying every gap's own touch parameter forward (not
     assumed at `t=lo`) so the per-face `Interior`/`SeamCrossing` decision samples the real touch
     point(s). Otherwise every maximal valid run becomes an open range, refined by bisection.
   - One imprint edge is built ONCE per clipped range and spliced into BOTH originating faces, so
     stitching needs no fuzzy vertex welding — the two resulting boundary pieces already share the
     identical `EdgeId`/`VertexId`s.
   - Three per-face imprint kinds: `Interior` (`euler::split_face_by_interior_curve` — a closed
     curve bounding a small sub-region: hole + new face), `SeamCrossing`
     (`euler::split_face_by_seam_crossing` — a closed curve spanning a periodic surface's FULL
     width, grazing a doubly-used seam edge at one physical point rather than bounding a
     sub-region — a genuinely different topological case from `Interior`, not a variant of it),
     `Open` (`euler::split_face_by_edge`, generalizing the pre-existing
     `split_planar_face_by_line` to any surface/curve pair — crosses the boundary at two distinct
     points).
   - Every resulting face piece is classified against the OTHER solid via a representative
     interior UV sample (`interior_point_of_face`) → `classification::point_in_solid`; cross-face
     comparisons go through `point_in_face_uv_periodic` (tries every small integer `2π` shift on
     both `u` and `v`, not just a single canonically-wrapped value — see §5c for why).
   - Selection (`keep_face`): `Unite` keeps `Outside|OnBoundary` from both; `Intersect` keeps
     `Inside|OnBoundary` from both; `Cut` keeps A's `Outside|OnBoundary` and B's `Inside` (B's kept
     pieces get `flipped` toggled).
   - Stitch (`stitch_selected_faces`): connected components over selected faces via shared
     `EdgeId`, one `add_shell`/`add_solid` per component, `shell_signed_volume` decides whether to
     flip every face in a component. The largest-|volume| solid is returned (unchanged
     single-handle signature); other components stay live in the body, unreturned.
   - Cleanup: `remove_solid_and_orphans` drops both original solids' superseded shell/solid
     wrappers and any unselected face; `gc_orphan_edges_and_vertices` removes edges/vertices no
     longer referenced.
   - `validate_body` must be clean, or the call returns `KernelError::Boolean(InvalidResult(...))`
     with the first issue's code/message — never a silently-wrong result.
4. `boolean_solid_mesh_preview` — the pre-rewrite tessellate→centroid-classify→triangle-soup
   pipeline, kept verbatim as an explicit opt-in; never called by `boolean_solid`/`compound_cut`.

`compound_cut` unchanged (folds `boolean_solid(..., Cut, ...)`).

## 2. Euler extensions (`🔺️euler/🦀️.rs`)

- **Bug fix in `split_edge`**: the two new coedges used to always get `pcurve: None`. Refactored
  into `split_edge_with_vertex` (binds a caller-supplied vertex — needed so a shared imprint
  vertex is the SAME `VertexId` on both sides) + `split_edge` (thin wrapper, same signature).
  Both now split each affected coedge's `prange` proportionally alongside the 3D `range`.
- `split_edge_at_params`, `splice_boundary_vertex` (idempotent "ensure this vertex is on this
  loop's ring", splitting whichever boundary edge's span contains the position via any curve
  kind), `split_face_by_edge`, `split_face_by_interior_curve`, `split_face_by_seam_crossing`,
  `kill_edge_merge_faces` (unit-tested, not called by the boolean pipeline itself — duplicate
  faces are dropped, not merged into a neighbor).
- **Second bug fix, found via live debugging, in `split_face_by_edge`/`split_face_by_seam_crossing`
  themselves**: both rebuild a loop's ring via `make_loop` for the pre-existing "chain" members
  (e.g. a cylinder's own `e_bot`/`e_top`/seam pieces) plus the new chord — but `make_loop` always
  mints fresh coedges with `pcurve: None`, silently discarding every chain member's REAL p-curve
  (only the chord's own p-curve was being restored afterward). Confirmed via instrumentation: a
  post-split cylinder lateral piece's loop showed only ONE surviving coedge (the chord), because
  the other three had gone `pcurve: None` and were silently skipped by every p-curve-only
  consumer. Fixed with new `loop_walk_pc`/`member_chain_pc`/`make_loop_pc` helpers that carry each
  member's own `(pcurve, prange)` through the rebuild and restore it verbatim.

## 3. Tests (in-file `#[cfg(test)]`)

Kept all 6 pre-existing `boolean.rs` tests (pass unchanged — box scenarios never leave the fast
paths). Added, exercising the general exact engine: `box_union_cylinder_through_exact_volume_and_
validates`/`box_minus_cylinder_bore_exact_volume_and_validates` (cylinder through a box, `Unite`
and `Cut`, closed-form volume check), `sphere_union_sphere_lens_exact_volume_and_validates`
(spheres translated along their own shared polar axis — the exact, non-fitted SSI case — spherical-
cap lens closed form), `self_boolean_identities_on_a_sphere` (A∪A/A∩A = A via the coincident-face
path), `union_and_intersect_are_commutative_by_volume` (A∪B==B∪A and A∩B==B∩A, general sphere
pairs), `tangent_spheres_union_volume_is_exact_sum`.

## 4. `operation_quality` (item 7, minimal edit to a shared file)

`fuse`/`cut`/`intersect`/`compound_cut` → `ExactNumericalWithinTolerance` (the imprint domain-clip
is sampling+bisection-based, matching `curve_surface_intersect`/`surface_surface_intersect`'s own
existing rating, not a certified closed-form clip). `section`/`split` left untouched (still the
pre-existing vertex/tessellation-based implementations — not upgraded this pass).

## 5. Three cross-cutting bugs found and fixed outside nominal ownership

All three live in `💡️inferences/🏷classification/🦀️.rs` (W1-F's file), all found via live
instrumented debugging of the curved-boolean tests (never assumed), all confirmed by reading the
broken code directly, all small and unambiguous enough to fix rather than duplicate the whole
classifier locally per the ticket's own "work around locally" instruction — the alternative would
have meant reimplementing large parts of the "one classifier" the audit's whole point was to stop
duplicating.

**(a) `coedge_uv_sample` ignored `forward` for the stored-p-curve branch.** Its own no-p-curve
fallback three lines below correctly reverses direction for a backward coedge
(`t1 - (t1 - t0) * s`); the p-curve branch evaluated `p0 + (p1 - p0) * s` regardless of `forward`.
Since `prange` is documented (and constructed, e.g. in `🧱️primitives/🦀️.rs`) in the edge's OWN
curve order — never reparametrized per coedge — a `forward == false` coedge with a stored p-curve
produced a DISCONNECTED/malformed trim polygon: nearly every non-planar face has at least one
reversed p-curve-bearing coedge by construction. This silently affected `point_in_face_uv`
**and** `point_in_solid` (via `classify_by_ray_consensus` → `point_in_face_uv_status`), i.e. sew,
mass-properties, and validation's self-intersection probe too, not just this file. One-line fix
mirroring the fallback branch's own correct handling.

**(b) `point_in_loop`/`point_in_face_uv_status` special-cased `Surface::Plane` to 1 sample per
coedge** (its own corner only), assuming every planar face's boundary is a straight-edge polygon —
true for `make_box`, false for a planar face bounded by an imprinted CIRCLE/ELLIPSE p-curve (a
cylinder-through-plane hole, or the small disk face on the far side of it): a single-coedge
circular loop degenerated to ONE sample point, `poly.len() < 3`, unconditionally `Outside`.
Switched both call sites to the uniform 16-samples/edge path always used for non-planar surfaces
(exact for a straight edge too — extra collinear points don't change the polygon).

**(c) Not a single bug but a genuine limitation worked around locally, not "fixed" upstream**:
`classification::loop_uv_polygon_sampled` unwraps a periodic surface's `u`/`v` continuously
starting from each loop's own first sample — two DIFFERENT faces' polygons can legitimately settle
on different multiples of `2π` even for the "same" physical seam. A single canonically-`[0, 2π)`-
wrapped query point compared against a polygon that drifted outside that window silently reads as
outside everywhere. Added `boolean::point_in_face_uv_periodic` (tries every small integer `2π`
shift on `u` and `v`, accepts the first that lands inside) for every CROSS-face UV comparison in
`🔀️boolean/🦀️.rs`; left `classification.rs` itself untouched since the drift is an inherent
property of per-loop independent unwrapping, not a bug in it.

## 6. What's NOT done / NOT passing this pass (explicit, not silent)

Verbatim `test result:` lines, both from runs I executed in the FOREGROUND via
`TICKET/🔬️harness`, `bun ./📜️script.ts sync && RUSTC_WRAPPER="" cargo test -- boolean euler`:

Before the three classification.rs fixes (§5) and the euler.rs chain-p-curve fix (§2):
```
test result: FAILED. 24 passed; 8 failed; 0 ignored; 0 measured; 407 filtered out; finished in 0.06s
```
(8 failures: my 5 new curved tests + 3 pre-existing, unrelated to this file — see below.)

After all fixes in this report, latest clean run:
```
test result: FAILED. 18 passed; 6 failed; 0 ignored; 0 measured; 424 filtered out; finished in 0.19s
```
(count differs run-to-run only because the filter also matches whichever OTHER modules' tests
happen to be linked in at that moment, depending on concurrent peers' own build state — the actual
pass/fail set for MY tests is stable across runs.)

Of the 6 failing:
- `diff::euler::tests::split_rectangle_face_into_two` — pre-existing, NOT this file's bug:
  `🧱️primitives`'s `make_planar_face_from_wire` doesn't attach p-curves to its planar faces
  (confirmed: unrelated to the classification.rs fixes above, which fixed `make_box`'s own
  sibling test `make_box_euler_and_validate` — that one now passes — but not this one, which uses
  a different construction path that still produces bare planar faces with no p-curves at all).
  Flagged for `🧱️primitives`'s owner.
- `diff::boolean::tests::sphere_union_sphere_lens_exact_volume_and_validates` /
  `union_and_intersect_are_commutative_by_volume` — fail with
  `Boolean(ClassificationAmbiguous("no interior UV sample found for face face-3-0"))`. Diagnosed
  via live debug: this face is a small `Interior`-split disk on one sphere from a LATITUDE circle
  (the exact, axis-parallel SSI case) — its own centroid genuinely tests `Outside` even after
  fixes (a)/(b)/(c) above, meaning there is at least a FOURTH distinct gap in this area not yet
  isolated (my best working theory, not confirmed: the same "multiple independent seam-touch
  gaps" issue §1 already handles for the DOMAIN-clip decision may also need to propagate into
  which candidate UV points `interior_point_of_face`'s own grid-scan tries, for a loop whose own
  polygon has drifted far from its analytic center in the same way `point_in_face_uv_periodic`
  was built to tolerate for CROSS-face queries — `interior_point_of_face`'s candidates are
  generated in the SAME face's own drifted representation already, so this theory is incomplete;
  not resolved this pass).
- `diff::boolean::tests::box_union_cylinder_through_exact_volume_and_validates` /
  `box_minus_cylinder_bore_exact_volume_and_validates` — progressed significantly across this
  session's fixes (from failing at imprint-splicing, to failing at classification, to now reaching
  full selection+stitch) but fail at `validate_body` with `non-manifold-edge: edge is used by 3
  coedges (2-manifold shapes use at most 2)`. This is a SELECTION bug: one imprint edge ends up
  referenced by 3 selected faces instead of 2, meaning either a coincident/duplicate face slipped
  past `find_coincident_face_pairs`, or the SAME edge is legitimately shared by 3 geometrically
  distinct kept pieces in a way this pass's selection logic doesn't dedupe. Root cause not
  isolated within this session's time budget.
- `diff::boolean::tests::tangent_spheres_union_volume_is_exact_sum` — fails at `validate_body`
  with `shell-not-closed: edge is used 1 time(s)`. The near-tangent (not quite disjoint, not quite
  overlapping) case falls through both fast paths into the general engine, where a genuinely
  edge-case SSI result (a near-degenerate contact circle) produces an open boundary somewhere in
  selection. Not isolated within this session's time budget.

None of the 5 remaining `diff::boolean` failures produce a WRONG result silently — every one is
caught by `validate_body`/the interior-sample check and surfaces as a `KernelError`, per the
ticket's "never a silently-wrong result" requirement; they are diagnosed-but-unresolved gaps, not
undetected corruption.

**Also not done**: `🧵️sew/🦀️.rs` untouched (no tolerance-escalation retry, no additional
orientation-propagation beyond what the imprint pipeline's shared-edge construction already
guarantees, no dedicated closed-shell detection beyond `stitch_selected_faces`'s connected-
components grouping, which lives in `🔀️boolean/🦀️.rs` since it's boolean-specific — `sew_faces`/
`heal_solid` unchanged from the prior pass). `section_solid_by_plane`/`split_solid_by_plane`
unchanged (still vertex-sampling/tessellation-based, not routed through the new imprint engine —
a design using two `boolean_solid(..., Intersect, ...)` calls against half-space box tools was
sketched but not implemented, since it needs a genuine deep-copy helper the pre-existing
`clone_solid_shells` doesn't provide — it aliases the same faces rather than copying them). `Open`
(`split_face_by_edge`) imprint path compiles and is unit-testable in principle but not exercised
by any test this pass (every scenario tested reduces to `Interior`/`SeamCrossing` on every
support involved). General *offset* box/box booleans (an imprint chord landing exactly on an
existing box edge/corner) are not routed through the general engine — covered by the pre-existing,
still-exact `box_fast_path` instead; full T-junction/edge-coincident topology merging was scoped
out in favor of the curved-surface engine the audit calls out by name. Random-placement
Monte-Carlo volume oracle test not written.

Also checked (assigned by the ticket coordinator, `⚙️engine/🦀️.rs`, not this worker's file to fix
but exercising this worker's pipeline): `engine::tests::sphere_torus_cut_produces_preview_mesh`
(a sphere ∖ torus, coaxial SSI) —
```
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 447 filtered out; finished in 0.07s
```
fails with the SAME `ClassificationAmbiguous("no interior UV sample found for face ...")` symptom
as the sphere/sphere lens failures above — confirms this is a general "Interior-split disk face on
a periodic surface" gap, not specific to sphere/sphere, and is the single highest-value remaining
fix (would likely resolve 3 of the 5 open `diff::boolean` failures plus this `engine::tests` one
at once).

## 7. Verification

`TICKET/🔬️harness`, FOREGROUND, per `📓️h0-harness.md` and the ticket coordinator's mid-session
`bun ./📜️script.ts sync` recipe (a tool the coordinator/H0 added mid-session to survive a large,
repo-wide, unrelated automated codemod that was toggling framework crate directory names — see
below).

`cargo check --lib --message-format short`: clean (zero errors in this worker's files) at every
point this session where the wider tree itself compiled; the LAST such clean run predates a
concurrent peer's (FX-5, per the ticket coordinator's own fixer-assignment message)
still-in-progress edit to `💡️inferences/🧩tessellation/🦀️.rs` (an unrelated arg-count mismatch,
confirmed by reading the file: not anything this worker touched), which blocks the full crate
build as of the final `cargo test` attempt in this report.

`cargo test -- boolean euler`: see the two verbatim `test result:` lines in §6 above (before/after
this session's fixes). A final confirming re-run after the FX-5 tessellation.rs churn resolves is
recommended but was not obtainable in this session's remaining time.

## Open items / requests for other workers

- `🧱️primitives/🦀️.rs`'s `make_planar_face_from_wire` still doesn't attach p-curves to its planar
  faces (its sibling `make_box` was fixed by another concurrent worker mid-session, confirmed via
  `make_box_euler_and_validate` now passing; `make_planar_face_from_wire` was not).
- `🧵️sew/🦀️.rs` tolerance-escalation/orientation-propagation/closed-shell-detection (§6) and
  `section_solid_by_plane`/`split_solid_by_plane` exactness remain open for a future pass.
- The 3 remaining `diff::boolean` test failures (§6) are diagnosed to the symptom level
  (non-manifold selection for box/cylinder; open boundary for tangent spheres; a fourth
  UV-representation gap for sphere/sphere lens) but not to root cause — a good next session's
  starting point, with the exact repro (this file's own tests) and the debugging techniques used
  this session (temporary `[DEBUG]`-prefixed `eprintln!` gated behind `SEMIO_DEBUG_*` env vars,
  removed before this report) both directly reusable.
