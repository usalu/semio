# 📓️ W1-Z — Kernel test-binary integration pass

Worker W1-Z (integrator). Goal: make `TICKET/🔬️harness`'s `cargo test` binary actually LINK and
RUN, fix genuine cross-worker integration bugs among the ~150 new wave-1/wave-2 unit tests, and
report the truthful pass/fail state. `B = ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/
🔖️v1/🪆️subsets/✳️brep/🧬️schema`.

**STOPPED per the coordinator's explicit instruction** once the binary linked and ran (per-family
fixers — intersect, curves/surfaces, offset/blend, sweep, inferences+primitives, boolean/euler —
were assigned to take the remaining failures forward). Everything below is what I actually did and
verified myself before that handoff; the "not fixed" items are named honestly, not silently
dropped.

## Final verified `cargo test` state (mine, foreground, in the harness)

Two runs, because the fleet-wide Cargo-manifest corruption (see "External blocker" below) sat
between them:

- **`RUSTC_WRAPPER="" cargo test`** (full suite, including `🔺️diff/{boolean,offset,blend,sweep,
  euler}` — files I do not own): **380 passed; 53 failed; 1 ignored; finished in 92.86s** —
  `🗑️generated`-equivalent log kept at `/tmp/w1z-full-1.txt` (outside the ticket folder, scratch
  only; not a committed artifact). This was the FIRST time the binary ever linked — see "What I
  changed" §1-3.
- **`RUSTC_WRAPPER="" cargo test -- --skip diff::boolean:: --skip diff::offset:: --skip
  diff::blend:: --skip diff::sweep:: --skip diff::euler:: --skip
  diff::intersect::surface_surface::tests::general_marching_skew_cylinders_closed_loop_on_both`**
  (after §4-9 below landed): **366 passed; 30 failed; 0 ignored; 52 filtered out; finished in
  36.08s**. The skip list excludes the 5 files I must not rewrite beyond compile-fixes, plus the
  one `#[ignore]`d hang (both runs still MOUNT and COMPILE those 5 files — only this second run's
  *execution* excludes them, to avoid the coordinator's other family-fixers' concurrent edits
  there racing my own verification).

I did **not** get a final, fully-fresh, all-modules-included re-run after my last three fixes
(§7's second pass, §8's `fitting_knot_vector`/`all_local_minima_periodic` fixes) — the coordinator's
stop-editing message arrived while I was mid-investigation of the `shell-orientation-inward`
false-positive on a clean tetrahedron (§9). Those fixes are on disk, type-check-consistent with
their call sites (I re-read them after writing), but their own runtime effect is only PARTIALLY
confirmed (see per-item notes below) — flagged honestly, not claimed as a clean pass.

## What I changed

### 1. Split `validate_body` into a kernel-only file (DO item 2a)

- **New**: `B/💡️inferences/✅validation-report/🧪️body/🦀️.rs` — `validate_body` and its 9
  `check_*` helpers, moved verbatim out of `✅validation-report/🦀️.rs`, plus the topology-only
  test module (`a_cleanly_built_tetrahedron_validates_with_no_issues` and its 11 siblings).
  Depends ONLY on kernel modules (`snapshot`/`diff::euler`/`diff::primitives`/
  `inferences::{bounding_volume,mass_properties}`) — no `SemioBrepSnapshot`, no
  `store::InferredField`, no artifact/STEP/plugin chain.
- **Edited**: `B/💡️inferences/✅validation-report/🦀️.rs` — kept `BrepValidationDiagnostic`/
  `BrepValidationReport` (the real `InferredField<SemioBrepSnapshot>`) and its own
  `SemioBrepSnapshot`-based tests; mounts the new file via `#[path = "🧪️body/🦀️.rs"] mod body;
  pub use body::validate_body;` so every existing `inferences::validation_report::validate_body`
  call site (`⚙️engine/🦀️.rs:62`, `🔺️diff/🧵️sew/🦀️.rs:14`, and `🧱️primitives`/`🔺️euler`/
  `➡️sweep`'s own `#[cfg(test)]` modules) resolves unchanged — zero call-site edits needed there.
- **`TICKET/🔬️harness/lib.rs`**: mounts `inferences::validation_report::body` directly (a
  harness-only shim module — NOT the real root file, which still needs the unmountable
  `SemioBrepSnapshot`/STEP chain) with `pub use body::validate_body;`, so
  `inferences::validation_report::validate_body` resolves in the harness exactly as it does in the
  real crate.

### 2. DWG codec made caller-supplied, not kernel-imported (DO item 2b)

- **`B/⚙️engine/📦️mesh-io/🦀️.rs`**: removed `use crate::artifacts::dwg::{...}`. `export_dwg`/
  `import_dwg`/`export_solid_dwg`/`import_dwg_to_body` now take `exporter: &impl MeshExporter` /
  `importer: &impl MeshImporter` (the SAME `semio_framework_mesh_engine` traits `GlbExporter`/
  `GlbImporter` already use in this file) instead of calling `crate::artifacts::dwg` directly —
  the kernel layer no longer imports another artifact.
- **`B/⚙️engine/🦀️.rs`**: new `// #region 🔖️DwgCodec` — `DwgExporter`/`DwgImporter` structs
  implementing `MeshExporter`/`MeshImporter` over the real `crate::artifacts::dwg::{
  mesh_to_dwg_drawing, dwg_to_bytes, dwg_from_bytes, dwg_drawing_to_mesh}` functions (this file IS
  the contract façade that legitimately bridges artifacts — same precedent as its STEP handling).
  `export_dwg_sync`/`import_dwg_sync` pass `&DwgExporter`/`&DwgImporter` at their two call sites.
- **`TICKET/🔬️harness/lib.rs`**: added a harness-only `pub mod dwg { ... }` stub (4 function
  signatures matching the real `crate::artifacts::dwg` API, bodies return
  `Err("dwg codec is not mounted in the isolated kernel test harness")` / a default `MeshData`) —
  not a copy of any real source line, just enough surface for `DwgExporter`/`DwgImporter` (which
  ARE the real, verbatim-mounted `engine.rs`'s own code) to compile. Mounting the REAL `dwg`
  artifact would pull `crate::registry` and ~69 other artifact types (H0's own documented,
  unchanged rationale for the ORIGINAL exclusion — only which file carries the cross-artifact
  import moved, not the mountability of the real `dwg` artifact itself).

### 3. `topology.rs`'s serde test converted to the first-party codec

- **`B/📸️snapshot/🕸️topology/🦀️.rs`**: `serde_round_trips_a_whole_body` → renamed
  `json_round_trips_a_whole_body`, body switched from `serde_json::{to_string,from_str}` to
  `pack::{to_json_string,from_json_str}` (same pattern as `📸️snapshot/🏟️arena/🦀️.rs`'s existing
  `TestId` round-trip test) — `Body` already derives `value_derive::ToValue`/`FromValue`, no
  serde needed.

### 4. Hung test marked `#[ignore]` (last resort, per instructions)

- **`B/🔺️diff/✂️intersect/🏄️surface-surface/🦀️.rs`**:
  `general_marching_skew_cylinders_closed_loop_on_both` — confirmed genuinely hung, not slow: ran
  it in isolation twice, both times still consuming real CPU (`ps` showed the test-binary process
  actively running, 90s+ and 5+ minutes of wall time respectively) with zero output; killed both
  manually. `find_seeds`/`gauss_newton_seed`/`march_direction` are each individually
  iteration-bounded (30/400/8), so this should finish in well under a second by inspection —
  something is defeating a bound rather than terminating it, or `Surface::derivatives`/
  `gauss_elim` pathologically degrades for this skew-cylinder configuration. Marked
  `#[ignore = "hangs indefinitely (5+ min, real CPU burn) on skew-cylinder general_marching —
  owner: W2-A, needs profiling; see comment above"]` with the investigation notes left in a doc
  comment directly above it. **Owner: W2-A / the intersect family-fixer.**

### 5. `validation_report` body's own test fixture — p-curve convention bug (my file)

- **`B/💡️inferences/✅validation-report/🧪️body/🦀️.rs`**, `attach_planar_pcurves` (test helper):
  was `let prange = if co.forward { (0.0,1.0) } else { (1.0,0.0) };` — this CONTRADICTS W1-E's own
  binding convention (`📓️w1e-primitives.md` §"p-curve convention": a p-curve's `prange` is
  ALWAYS the edge's own curve order, `co.forward` is never baked into it; `set_outer_pcurves` in
  `🧱️primitives/🦀️.rs` documents the identical rule). Fixed to `let prange = (0.0, 1.0);`
  unconditionally. This alone eliminated every `same-parameter-violated` finding on the clean
  tetrahedron fixture (6 spurious findings, one full edge-length off at each backward coedge's far
  end) — **confirmed by re-run**: gone from both `a_cleanly_built_tetrahedron_validates_with_no_
  issues` and `shell_orientation_inward_is_detected_on_a_globally_reversed_tetrahedron`'s failure
  output.

### 6. `mass-properties.rs`'s `loop_uv_polygon` — the flagged single-sample-per-coedge bug

Both W1-E's and W1-F's own reports independently flagged this (`📓️w1e-primitives.md` §"Finding
for W1-F", `📓️w1f-...md` — not fixed by either, explicitly deferred to "whoever gets a green
`cargo test`"):

- **`B/💡️inferences/📏mass-properties/🦀️.rs`**: `loop_uv_polygon` sampled only each coedge's
  START vertex — exact for straight-edge polygons, degenerate (1-4 point "polygon") for any
  curved-edge loop with few coedges (every lateral/cap/spherical/toroidal face `🧱️primitives`
  builds). Added `coedge_uv_sample(body, co, surface, s)` (mirrors `🏷classification/🦀️.rs`'s own
  already-correct `coedge_uv_sample` — kept close per doctrine rather than cross-imported: pcurve
  when present, read in the edge's own order per §5's convention; 3D-curve reprojection fallback,
  `forward`-aware since THAT path has no stored prange to lean on) and rewrote `loop_uv_polygon` to
  sample `EDGE_SAMPLES=8` points per coedge instead of 1.
  - **Caught my own bug on first pass**: used `s = i/EDGE_SAMPLES` (never reaching `s=1.0`, so
    every edge was sampled over only 7/8 of its own span) — re-verified against
    `🏷classification/🦀️.rs`'s proven `loop_uv_polygon_sampled` pattern and corrected to
    `s = i/(n-1)` with the same last-sample-skip-except-final-coedge rule that pattern already
    uses. This second fix landed but I did **not** get a fresh full-suite run afterward (stop
    instruction arrived first) — last observed state (first-pass version) reduced
    `primitives::tests::closed_form_volumes_via_mass_properties`'s cylinder-volume error from
    "got 0" to "got 6.77 (expected 28.27)" and `mass_properties::tests::solid_mass_properties_
    cylinder_general_path_...`'s from grossly wrong to "got 8.33 (expected 9.42)" — real
    improvement, not yet fully correct, and the corrected (`i/(n-1)`) version is UNVERIFIED by any
    test run. **Owner: inferences/primitives family-fixer — please re-run `cargo test --
    mass_properties primitives::` first, before anything else, since the fix-on-disk is newer than
    any run that exercised it.**
  - `loop_positions` (the sibling 3D-point sampler) was **not** changed — confirmed it is only
    ever called for `Surface::Plane` faces, where vertex-only sampling is exact (W1-E's own
    observation); no bug there.

### 7. `make_box`/`make_convex_hull`/`make_planar_face_from_wire` never attached p-curves

The real cross-wave gap: W1-E's own report explicitly says "Hull faces do NOT carry p-curves (same
as `make_box`'s planar faces — validation only requires a p-curve on non-planar surfaces)" — true
of the OLD validator, false of W1-F's NEW one (`check_missing_pcurves` requires EVERY coedge to
have a p-curve, no planar exemption). Confirmed via the failing tests: every coedge of every
`make_box`/`make_convex_hull` solid reported `missing-pcurve`.

- **`B/🔺️diff/🧱️primitives/🦀️.rs`**: new `attach_planar_face_pcurves(body, face, frame, members,
  tol)` — for each coedge's edge, calls `Surface::project_curve(&curve3, edge.range, tol)`
  (W1-D2's existing method: exact for `Line`/`Circle`/`Ellipse` on a `Plane` via its
  `analytic_pcurve_shortcut`, tolerance-checked fit otherwise) and stamps the result via the
  existing `set_outer_pcurves`, `prange` always `(0.0, 1.0)` (§5's convention). Wired into:
  - `make_box` — captured each cap's `Frame3` before inserting the `Surface::Plane` (previously
    discarded via `plane_at`'s return-only-`Surface` signature), attached pcurves to all 6 faces.
  - `make_convex_hull` — same, per merged-coplanar-triangle-group face.
  - `make_planar_face_from_wire` (covers `make_planar_face_from_points` too, which delegates to
    it) — same.
  - **Not touched**: the OTHER planar-face builder inside `solid_from_triangle_soup` (the
    mesh-import fallback, `🔺️diff/🧱️primitives/🦀️.rs` around what W1-E's report calls "the
    (now less likely to be needed) hull fallback path") — same latent gap, but no currently-failing
    test exercises it and it's explicitly out of this wave's "exact primitives" scope per W1-E's
    own report; flagged here for whoever looks at it next, not fixed.
- **Confirmed fixed by re-run** (present in `/tmp/w1z-full-1.txt`'s failure list, ABSENT from
  `/tmp/w1z-full-3.txt`'s): `primitives::tests::make_box_euler_and_validate`,
  `primitives::tests::make_convex_hull_tetrahedron`, `primitives::tests::make_convex_hull_box_
  merges_coplanar_triangles_into_six_faces`, `sew::heal_tests::heal_solid_noop_on_valid_box`,
  `engine::tests::box_shell_produces_positive_volume`, `engine::tests::validate_returns_
  structured_json_report` — six tests, all resolved by this one fix.

### 8. `curve-ops.rs` — two W1-D1/W1-D2 bugs found and fixed while investigating failures

- **`fitting_knot_vector`** (`approximate_curve`/`approximate_curve_with_count`'s knot-vector
  builder): the interior-knot loop was `for j in 1..=(n - p - 1)`; the standard Piegl-Tiller
  averaging formula (NURBS Book Eq. 9.68) needs `n - p` interior knots, not `n - p - 1`. For the
  common `n_controls = degree + 2` case (`n - p == 1`), the old bound produced the EMPTY range
  `1..=0`, leaving the sole interior knot at its zero-initialized default — colliding with the
  clamped-start knots and pushing that knot's multiplicity past `degree + 1`, which
  `KnotVector::new` correctly rejected, which the `?` in `approximate_curve_with_count` silently
  turned into `None`, which every `.unwrap()`ing caller then panicked on. Fixed the loop bound to
  `1..=(n - p)`. **Not re-verified by a test run** (found and fixed after the stop instruction's
  window closed for further runs) — but the bug mechanism is unambiguous by inspection (empty
  range → unset array slot → invalid knot multiplicity), and it is the exact, sole cause of both
  `approximate_curve_achieves_the_requested_error_bound` and `approximate_curve_with_count_
  matches_endpoints_exactly`'s "unwrap() on a None value" panics.
- **`all_local_minima_periodic`**: for a domain spanning exactly one full period (e.g. a whole
  untrimmed circle's own `(0, TAU)`), `domain.0`/`domain.1` are the SAME physical point, not two
  independent constrained-optimum candidates — but the function force-included both unconditionally
  (correct for a genuinely-trimmed arc, where a boundary CAN be the constrained optimum even if not
  an unconstrained critical point). Added a `full_period` guard (skip the forced-endpoint disjunct
  when `domain` spans exactly one period) plus a closing dedup-pop. **Confirmed partial effect,
  NOT a full fix**: re-ran `closest_parameter_on_circle_is_the_unique_local_minimum` after this
  change — candidate count dropped from 3 to 2 (the duplicate `t=0.0` domain-start representative
  is now correctly gone), but the antipodal/farthest point (`t≈4.712`, should be a local MAXIMUM)
  is still surviving `is_local_minimum`'s filter alongside the still-present `t=TAU` endpoint. I
  hand-verified `is_local_minimum`'s formula (`d1·d1 + delta·d2 > 0`) against a 1D projection of
  the circle/target geometry and it SHOULD reject the antipodal point — the discrepancy is
  unresolved. **Owner: curves/surfaces family-fixer** — this file (`✂️curve-ops`) is now assigned
  there; I stopped per the coordinator's instruction rather than keep iterating on an assigned
  file. Affected tests still failing: `closest_parameter_on_circle_is_the_unique_local_minimum`,
  `all_closest_parameters_finds_both_minima_of_an_s_shaped_curve` (same root function).

### 9. Investigated, NOT fixed: `shell-orientation-inward` false positive on a clean tetrahedron

`validation_report::body::tests::a_cleanly_built_tetrahedron_validates_with_no_issues` and
`shell_orientation_inward_is_detected_on_a_globally_reversed_tetrahedron` both still fail with
`shell-orientation-inward` reported on the SUPPOSEDLY-correctly-wound `build_tetrahedron` fixture
(signed volume computed as -0.1667, expected +0.1667) — this is independent of §5's fix (already
applied, confirmed by the same-parameter findings being gone) and independent of §6 (that function
only affects non-planar `loop_uv_polygon`; a tetrahedron's four faces are all planar, routing
through the untouched `loop_positions`). Traced as far as: `build_tetrahedron`'s first face
`[0,1,2]` (positions `(0,0,0),(1,0,0),(0,1,0)`, with vertex 3 at `(0,0,1)` "above" it) computes
`normal = (p1-p0)×(p2-p0) = (0,0,1)` — pointing TOWARD vertex 3, i.e. into the solid's interior, not
outward — which by hand-derivation should make `shell_signed_volume` negative for THIS "clean"
fixture too, consistent with what's observed, but inconsistent with the test's own name/intent and
with `build_tetrahedron_globally_reversed` being the one specifically designed to trigger this
finding. I could not resolve whether the bug is in `build_tetrahedron`'s own fixture-construction
math, in `Frame3::from_normal`, or in `shell_signed_volume`'s sign convention within my remaining
time — flagged here with the exact hand-trace above so whoever picks it up doesn't have to
re-derive it. **Not introduced by me** — this fixture and check predate my split (moved verbatim);
confirmed via `/tmp/w1z-full-1.txt`'s failure list that this same finding was ALREADY present
(alongside the now-fixed same-parameter noise) in the very first successful run.

## Existing harness-mounted boolean/sweep/blend/offset/euler tests (DO item 4 — report, not fix)

Per instructions, I did not edit these five files beyond the two mechanical call-site fixes below
(both required just to keep `engine.rs` compiling, not algorithm changes):

- **`⚙️engine/🦀️.rs`**: `draft_sync`'s call into `offset::draft_angle` and `chamfer_sync`/
  `chamfer_asymmetric_sync`/`chamfer_edges_sync`'s calls into `blend::chamfer_edges` needed their
  argument lists updated to match live signature changes landing concurrently in
  `↔️offset/🦀️.rs`/`🎨️blend/🦀️.rs` (W2-D's active work). `draft_sync` had already been fixed by
  someone else by the time I got to it (re-verified, unchanged by me). `chamfer_edges` I DID fix:
  it's currently symmetric-only (`distance: f64`, no separate `d1`/`d2`), so
  `chamfer_asymmetric_sync`'s two distances now collapse to their average
  (`0.5 * (d1 + d2)`) — documented inline as a minimal compile-fix, not a claim of real asymmetric
  behavior; **owner: blend family-fixer**, once `chamfer_edges` grows a second distance parameter
  this collapse should be removed.
- In the first full run (`/tmp/w1z-full-1.txt`, before any family-fixer started), the following
  were failing in these five files — **verbatim, unmodified by me**:
  - `diff::boolean::tests`: `box_minus_cylinder_bore_exact_volume_and_validates`,
    `box_union_cylinder_through_exact_volume_and_validates`,
    `sphere_union_sphere_lens_exact_volume_and_validates`,
    `tangent_spheres_union_volume_is_exact_sum`, `union_and_intersect_are_commutative_by_volume`
  - `diff::euler::tests`: `split_rectangle_face_into_two`
  - `diff::offset::tests`: `draft_box_side_face_matches_trapezoid_magnitude`,
    `offset_cylinder_matches_closed_form`, `offset_determinism_face_count_and_volume`,
    `offset_nurbs_surface_within_bound`, `offset_solid_box_round_matches_minkowski_closed_form`,
    `offset_solid_box_sharp_matches_closed_form`, `offset_sphere_matches_closed_form`,
    `shell_box_fully_closed_matches_closed_form`, `shell_box_one_open_face_matches_closed_form`
  - `diff::blend::tests`: none failing in that first run (blend's own tests passed); by my second,
    partial run (`/tmp/w1z-full-3.txt` excluded these five files entirely from execution, so no
    fresh blend read — but an UNLOGGED interim full run I killed mid-way for contention showed
    `fillet_plane_cylinder_junction_decreases_volume` failing, `fillet_one_box_edge_matches_
    closed_form`/`fillet_determinism`/`variable_fillet_is_monotone_in_radius` passing — not a
    verified final state, mentioned for continuity only.
  - `diff::sweep::tests`: not observed failing in the one complete run I captured.
  - Since then, `↔️offset/🦀️.rs` and `🔺️diff/🔀️boolean/🦀️.rs` were both under LIVE, active
    concurrent edits for a long stretch of my session (root Cargo.toml corruption + a mid-edit
    `finish_rebuild`/`Pnt3::max` compile break I waited out rather than patched, per the
    "ignore/wait for concurrent churn" rule) — their CURRENT state is unknown to me; the
    boolean/offset/sweep/euler family-fixers should treat the list above as a snapshot, not current
    truth, and re-run fresh.

## External blocker encountered (not mine, not touched)

For roughly 25 minutes mid-session, `cargo check`/`cargo test` in the harness failed with
`failed to load manifest for dependency ...` / `No such file or directory` against several
framework crates (`semio-framework-mesh-engine`, `semio-framework-number`, and the root
`Cargo.toml` itself briefly showing doubled path segments like `📦️📦️📦️packages`). The coordinator
identified this as a foreign session's repo-wide codemod duplicating emoji path segments, being
repaired live by a peer session (semio-2f). Per the coordinator's instruction I did **not** edit
any `Cargo.toml` myself and simply waited/retried — confirmed resolved (`cargo check --lib` clean)
before resuming. No ticket files or harness Cargo.toml needed changes for this.

## Files touched (all under `B` unless noted)

- `💡️inferences/✅validation-report/🧪️body/🦀️.rs` — **new**
- `💡️inferences/✅validation-report/🦀️.rs`
- `💡️inferences/📏mass-properties/🦀️.rs`
- `🔺️diff/🧱️primitives/🦀️.rs`
- `🔺️diff/✂️intersect/🏄️surface-surface/🦀️.rs` (one `#[ignore]` + doc comment only)
- `📸️snapshot/➰️curve/✂️curve-ops/🦀️.rs`
- `📸️snapshot/🕸️topology/🦀️.rs`
- `⚙️engine/🦀️.rs`
- `⚙️engine/📦️mesh-io/🦀️.rs`
- `TICKET/🔬️harness/lib.rs`

## Rules followed

TDD where the fix was itself a test-fixture bug (§5); every fix targets the callee, not the call
site, except the two documented, minimal, owner-flagged compile-only shims (§2's DWG codec
indirection is a real architectural fix, not a shim). No comments inside function bodies beyond
what was already there; every new function's docstring starts with a unique emoji. Never ran a git
write command. Never touched `📌️important.md`. Never closed/reopened the ticket. Never edited any
`Cargo.toml`. Stopped editing kernel files immediately on the coordinator's instruction, with
in-flight investigation (§9) left as a documented trace rather than a guessed fix. All `cargo
test`/`cargo check` runs were FOREGROUND with an explicit `timeout` (killed and re-ran rather than
leaving backgrounded, twice caught and corrected a stray background process from an earlier
mis-timed call before continuing).
