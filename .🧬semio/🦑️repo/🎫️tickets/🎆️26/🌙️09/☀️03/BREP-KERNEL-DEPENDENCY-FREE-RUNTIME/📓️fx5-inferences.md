# 📓 FX-5 — Inferences fixer (tessellation, validation-report, mass-properties, bounding-volume, primitives)

Ticket: `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME`. Scope: the 9 target tests listed in the
coordinator's `🗑️generated/coordinator-harness-failures.md`, plus (mid-task) 3 extra targets the
coordinator flagged from `📓️fx4-sweeps.md`. All runs via the H0 standalone harness
(`TICKET/🔬️harness`), foreground, `bun ./📜️script.ts sync` before every `cargo test`.

## Files touched

- `💡️inferences/🧩tessellation/🦀️.rs`
- `💡️inferences/📏mass-properties/🦀️.rs`
- `💡️inferences/✅validation-report/🧪️body/🦀️.rs`
- `💡️inferences/🌳bounding-volume/🦀️.rs`
- `🔺️diff/🧱️primitives/🦀️.rs` — read only, no changes needed there; the volume bug was entirely in
  mass-properties.rs's own boundary-sampling helpers.

## Root causes and fixes

### 1. `tessellation.rs` — 4 failing tests, all from the same class of bug

- **`weld_and_compact` welded non-pole seam-duplicate vertices.** A cylinder's `u=0`/`u=2π`
  boundary corner is the SAME 3D point (periodicity) but must stay TWO distinct UV vertices for a
  correct rectangular boundary. The weld pass only checked 3D coincidence, so it merged them,
  making a nearby triangle's edge span the WHOLE `u` range (its midpoint landed on the opposite
  side of the cylinder) — `max_chordal` came out as `2` (≈ the cylinder's height) against a
  `0.05` deflection request. Fix: thread an `is_pole: Vec<bool>` (from `coedge_point_uv`'s existing
  `surface.normal(...).is_none()` check) through `collect_loop_uv`/`remove_closing_duplicate_uv`/
  `refine_adaptive`, and only weld pairs where BOTH are poles.
- **Sphere pole-crossing collapsed the whole lune onto one meridian.** `unwrap_uv`'s "pin `u` to
  `prev.0` at a pole" rule correctly pins the pole's OWN sample, but the code also let that pinned
  value anchor the NEXT (departing) sample's branch-unwrap, dragging the descending seam back onto
  the ascending seam's `u=0` branch instead of letting it land on `u=2π`. Fixed by tracking
  `prev_is_pole` and skipping the unwrap-against-`prev` step for the sample immediately after a
  pole.
- **`face_with_circular_hole_triangulates_inside_the_trim`'s own test fixture was inconsistent, not
  the algorithm**: `build_plane_face_with_hole` placed the hole's vertex at
  `hole_center + (0.5,0,0)` (world +X) while the hole's `Curve3::Circle` used
  `Frame3::from_normal(hole_center, Z)`, whose `x`/`y` axes are `Vec3::any_orthogonal`-derived —
  NOT world X/Y. The edge's topological endpoint disagreed with its own curve's `t=0` evaluation,
  fracturing the closed ring at that one sample and letting a triangle centroid land inside the
  hole. Fixed the fixture to derive the vertex from `hole_frame.to_world(radius, 0, 0)`, matching
  `make_cylinder`/`make_cone`'s own convention.
- **`DEFAULT_ANGULAR_TOL = 0.35`** silently overrode the caller's `deflection` for any deflection
  above ~1.5% of the radius (forcing the same ~18-segment circle for both `0.2` and `0.02`
  deflection on a unit cylinder) — `tighter_deflection_yields_more_triangles` saw `204 vs 204`.
  Raised to `1.4` (chosen empirically to give both circle-segment-count AND
  `triangle_needs_refine`'s angular-refinement criterion enough headroom that neither resonates
  with common segment counts — `π/3` was tried first and made a COARSER deflection produce MORE
  triangles due to `2π/(π/3) = 6` exactly, an integer-division razor's edge that left zero slack
  before `triangle_needs_refine` re-triggered).

### 2. `mass-properties.rs` — the primitives volume test + the cylinder general-path test

- **`coedge_uv_sample` never reversed p-curve sampling for a backward coedge** (`p0 + (p1-p0)*s`
  regardless of `co.forward`) — the exact same bug class W2-B already found and fixed in
  `classification.rs`'s own `coedge_uv_sample`, just not mirrored here. Half of any non-planar
  loop's coedges are reversed by construction, so this silently traced them backwards, producing a
  self-crossing UV boundary whose quadrature integrated the wrong region (cylinder volume came out
  at ~24% of closed-form). Fixed identically to classification.rs's fix.
- **`loop_uv_polygon` sampled a fixed `EDGE_SAMPLES = 8` points per coedge**, tolerance-blind: an
  8-gon's sagitta is already 7.6% of the radius for `r=1.5`, capping accuracy regardless of the
  caller's `chord_tol`. Replaced with a curvature-adaptive per-coedge count
  (`coedge_sample_count`/`segments_for_chord_deviation`, closed-form chordal-deviation formula,
  same shape as tessellation.rs's own but without its angular-tol term — this feeds a quadrature
  boundary, not a rendered mesh). Threaded `chord_tol` through `loop_uv_polygon`'s signature and
  all 6 call sites (a small fixed `1e-4` for the two classification-only call sites in
  `point_in_face_plane`, which have no caller-supplied tolerance).
- **Cone/sphere apex handling was missing entirely** in `loop_uv_polygon` (no pole-awareness at
  all, unlike tessellation.rs): the "skip the last sample of a non-final coedge" dedup rule dropped
  the apex's OWN pole-branch sample, and unwrapping the surviving final-coedge sample against the
  arriving branch closed the loop via a spurious diagonal across the full angular range instead of
  the true (apex-collapsed) rectangle — cone lateral volume came out at exactly `0`. Fixed with the
  same `is_pole`/`prev_was_pole` pattern as tessellation.rs, plus keeping (not skipping) a
  non-final coedge's last sample when it IS a pole, so both angular branches of the apex get their
  own polygon vertex.
- **`loop_area`/`loop_volume_moments`'s `Surface::Plane` fast path degenerated for curved-boundary
  planar faces** (a cylinder/cone cap's single full-circle coedge collapsed to ONE point via
  `loop_positions`, zeroing that face's area/volume contribution). Added
  `loop_has_only_straight_edges` and route through the general (curvature-adaptive) path whenever
  the loop has any non-`Line` edge; the straight-edge fast path is otherwise unchanged.
- **Coordinator's extra item (A): the `Plane` volume branch ignored `face.flipped`** — the general
  branch negates via `quad_triangle_once`'s `cross` flip, the fast `signed_tetra_sum` path didn't.
  Fixed to negate all four returned components when `flipped`.
- Also removed `Curve3`'s stray `#[cfg(test)]` gate (needed outside tests now for
  `coedge_sample_count`'s curve-type match).

### 3. `validation-report/🧪️body/🦀️.rs` — tetrahedron fixtures had their windings swapped

Verified algebraically per-face (not by trusting the validator): for the fixture's vertices
`0=(0,0,0), 1=(1,0,0), 2=(0,1,0), 3=(0,0,1)`, the OLD `build_tetrahedron`'s winding
(`[[0,1,2],[0,3,1],[1,3,2],[2,3,0]]`) has every face's `(v1-v0)×(v2-v0)` normal pointing TOWARD
that face's own excluded (4th) vertex — i.e. genuinely INWARD — while the OLD
`build_tetrahedron_globally_reversed` (each face's last two vertices swapped) is the genuinely
OUTWARD tetrahedron. The two fixtures were simply mislabeled; `check_solid_orientation`'s
`shell-orientation-inward` detection was correct both times. Swapped which winding each function
uses (validator untouched, stays strict).

### 4. `bounding-volume/🦀️.rs` — test used an unbounded ray, not a segment

`refit_updates_bounds_in_place_without_rebuilding`'s "before" assertion put item 0's ORIGINAL box
at `[0,1]³`, on the exact `y=0.5, z=0.5` sightline the probe ray (`origin=[10,0.5,0.5],
dir=[-1,0,0]`) walks. Since `query_ray` is a genuinely unbounded ray (`t ∈ [0,∞)`, not a clipped
segment), that ray already crosses `x∈[0,1]` at `t∈[9,10]` regardless of `refit` — the "before"
assertion asked for something geometrically false, not a BVH bug. Moved item 0's initial box to
`x∈[20,21]` (behind the ray's origin in the direction it travels, unreachable at any `t≥0`) so
"before" is genuinely empty; `refit` (unchanged) moves it into the ray's actual path afterward.

## Coordinator's extra targets (from `fx4-sweeps.md`)

Items (B) `loop_positions` degenerating for curved-boundary planar loops and (C) `coedge_uv_sample`
never reversing for backward coedges were both already fixed above (they're the same root causes as
the primitives/cylinder volume bug). Item (A) `Plane` branch ignoring `flipped` fixed above too.
`cargo test -- sweep` after these fixes: **10 passed, 2 failed** (down from the reported 5) — the
remaining 2 (`sweep_circle_along_line_is_a_cylinder`, `revolve_annulus_full_turn_is_analytic_and_
exact_volume`) fail inside `🔺️diff/➡️sweep/🦀️.rs` itself (lines 413, 336), outside my file
ownership — not touched.

## Verification (verbatim, foreground, harness)

`cd TICKET/🔬️harness && bun ./📜️script.ts sync && RUSTC_WRAPPER="" cargo test -- inferences::tessellation inferences::validation_report inferences::mass_properties inferences::bounding_volume diff::primitives`:

```
test result: ok. 77 passed; 0 failed; 0 ignored; 0 measured; 373 filtered out; finished in 15.53s
```

All 9 originally-assigned target tests confirmed individually passing as part of that run:
`tessellation::tests::{face_with_circular_hole_triangulates_inside_the_trim,
report_max_chordal_respects_deflection, sphere_caps_collapse_pole_to_single_vertex,
tighter_deflection_yields_more_triangles}`, `validation_report::body::tests::
{a_cleanly_built_tetrahedron_validates_with_no_issues,
shell_orientation_inward_is_detected_on_a_globally_reversed_tetrahedron}`,
`mass_properties::tests::solid_mass_properties_cylinder_general_path_matches_closed_form_within_
error_estimate`, `bounding_volume::spatial::tests::refit_updates_bounds_in_place_without_rebuilding`,
`diff::primitives::tests::closed_form_volumes_via_mass_properties`.

`cargo test -- sweep`:

```
test result: FAILED. 10 passed; 2 failed; 0 ignored; 0 measured; 438 filtered out; finished in 2.95s
```

Final confirmation, all 9 originally-assigned target tests named individually in one run:

```
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 441 filtered out; finished in 27.45s
```

A full unfiltered `cargo test` was also tried, to check for regressions outside my own files, but
was killed after exceeding the foreground time budget (whole-crate build+run against the 448-test
suite, most of it outside my ownership) — not re-attempted, since every test in my own file scope
is independently confirmed green above via the filtered runs, and the ticket's own rule is to never
leave background work running.

## Known limitations / not touched

- `🔺️diff/➡️sweep/🦀️.rs`'s own remaining 2 failures (sign/topology bugs in that file) — out of
  ownership scope, reported only.
- `coedge_sample_count`'s NURBS fallback (fixed 32 points) has no curvature-adaptive bisection like
  tessellation.rs's `sample_curve_adaptive` — acceptable for this dependency-free kernel's own test
  suite (no NURBS-boundary mass-property test currently exercises it), flagged as a documented gap
  in the function's own docstring rather than silently left unfinished.
