# 📓 W1-G — Tessellation (CDT, seams, poles, error certificate)

**File owned (only file touched):**
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs`
(1342 lines, full rewrite of the previous 758-line ear-clip+fan version).

## What changed

Replaced ear-clipping + fan fallback with a real constrained Delaunay pipeline:

1. **Edge sampling** (`sample_edge_points`/`sample_curve_adaptive`/`subdivide_curve_segment`):
   circle/ellipse use the exact formula `n = ceil(arc_range / min(2·acos(1 − d/r), angular_tol))`
   (`segments_for_chord_deviation`, no more ad-hoc `n_min` heuristic); NURBS uses recursive
   chordal+angular bisection (`MAX_CURVE_SUBDIV_DEPTH = 12`) instead of a coarse-grid-then-rescale
   heuristic. Every edge is discretized once, keyed by `EdgeId`, cached, and reused by both
   coedges (reversed for the opposite sense) — crack-free by construction.
2. **UV resolution + seam/pole handling** (`collect_loop_uv`/`coedge_point_uv`/`unwrap_uv`): uses
   the coedge's stored `Curve2` p-curve (`Coedge::pcurve`/`prange`, reparametrized from the edge's
   own `t` via `Edge::range`) when present, else `surface_ops::closest_point`. Periodic surfaces
   are unwrapped by picking the UV branch `u ± k·period` nearest the previous sample
   (`nearest_branch`); at a pole/apex (`surface.normal(...) == None`) `u` is pinned to the previous
   branch instead of unwrapped, since it's meaningless there.
3. **Triangulation** (`Tri`, `bowyer_watson`, `insert_point_into`, `recover_edge`/
   `try_flip_towards`/`find_opposite`/`is_convex_quad`/`segments_properly_intersect`,
   `build_constrained_triangulation`): incremental Bowyer–Watson Delaunay over the outer+hole ring
   points, Sloan-style edge-flip constraint recovery for every ring edge, then point-in-polygon
   trimming (outside outer / inside any hole). Replaces the old bridge-holes-then-ear-clip hack.
4. **Adaptive interior refinement** (`refine_adaptive`/`triangle_needs_refine`): for non-planar
   surfaces, iteratively (`MAX_REFINE_ITERS = 8`) finds triangles whose edge-midpoint chordal
   deviation or vertex-normal angular deviation exceeds tolerance and inserts a Steiner point at
   that triangle's UV-centroid surface point via the same incremental Bowyer–Watson insertion
   (safe on an already-trimmed mesh — a constrained ring edge, owned by exactly one triangle post
   trim, can never be the shared edge BW removes). Planar faces get zero interior points.
5. **Pole collapse** (`weld_and_compact`): a spatial-hash weld (tol `1e-7`) merges UV-distinct
   samples that evaluate to the same 3D point (sphere poles, cone apexes) into one vertex and drops
   the resulting degenerate triangles — this *is* the "fan around the pole", no bespoke cap-mesher
   needed.
6. **Winding** (`fix_winding_per_triangle`): per-triangle, judged against the surface's own normal
   at that triangle's own UV centroid — no more single-triangle-then-global-flip heuristic.
7. **Output**: `face_groups`/(new) `face_infos` `entity_id` and edge `entity_id` are now the
   entity's `PersistentLabel` (`label.0.to_string()`), never a raw arena index.
   `MeshTransfer::{edge_groups,face_infos,edge_infos}` (real fields, landed by W1-A concurrently
   in `⚙️engine/🔖️contract/🦀️.rs`) are filled directly — `edge_groups`/`edge_infos` in
   `pack_edge_segments_with_info` (solid level) and `tessellate_face_with_report` (face level),
   `face_infos` per-face in `append_face_mesh` (`surface_kind`/`curve_kind` via `SurfaceKind`/
   `CurveKind`, `area` from triangle-sum, `normal` from the first boundary UV sample).
8. **Error certificate**: new `TessellationReport { max_chordal, max_angular }`, computed by
   `measure_report` with the same metric `triangle_needs_refine` converges against. New
   `tessellate_solid_with_report`/`tessellate_face_with_report` return `(MeshTransfer,
   TessellationReport)`; `tessellate_solid`/`tessellate_face` are now thin wrappers over them —
   **signatures unchanged** (still `Result<MeshTransfer, KernelError>`), since `⚙️engine/🦀️.rs`,
   `🔺️diff/↔️offset`, `🔺️diff/🔀️boolean`, and `⚙️engine/📦️mesh-io` all call `tessellate_solid`
   directly and must keep compiling.

**Blocked note (resolved mid-task):** the ticket brief anticipated `MeshTransfer`'s
`edge_groups`/`face_infos`/`edge_infos` fields might not exist yet when I started (W1-A's
concurrent work). They landed in `⚙️engine/🔖️contract/🦀️.rs` partway through this session —
confirmed via the harness's own mount list (`📓️h0-harness.md`) and by reading the contract file
directly. No local mirror types were needed; the real `EdgeGroup`/`FaceInfo`/`EdgeInfo`/
`SurfaceKind`/`CurveKind` are imported from `crate::…::brep::schema::engine`.

## Verification

**Coordination note:** the ROOT workspace `cargo check -p semio-s-plugin-stdio` gate was
abandoned per an explicit coordinator instruction mid-session (20+ duplicate root checks were
queued on the shared build-directory lock across concurrent workers; W1-C owns that single root
check). All verification below uses the H0 standalone harness
(`TICKET/🔬️harness`, own workspace/target-dir, mounts the kernel layer verbatim via `#[path]` —
see `📓️h0-harness.md`).

### `cargo check --lib` (production code only, no `#[cfg(test)]`)

```
cd TICKET/🔬️harness && RUSTC_WRAPPER="" cargo check --lib --message-format short
```
Result: **zero errors, zero warnings from `🧩tessellation/🦀️.rs`.** Full run:
`Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 05s`, only pre-existing warnings
elsewhere (framework crates' own lints) plus one warning each in two OTHER brep files
(`🔺️diff/✂️intersect/🏄️surface-surface/🦀️.rs`, unrelated to this file — W2-A's territory).

One fix was needed and applied: the harness flagged `unused import: ArenaId` (only used inside
`#[cfg(test)]`, so dead under a `--lib`-only check) — moved the `ArenaId` import from the file's
top-level `use` block into `mod tests`'s own imports. Confirmed clean on re-run.

### `cargo check` (includes `#[cfg(test)] mod tests`)

Result: **zero errors, zero warnings from `🧩tessellation/🦀️.rs`.** `Finished` in 0.95s
(incremental). Same two pre-existing warnings elsewhere as above.

### `cargo test -- tessellation`

Could not reach a running test binary — the harness's shared crate fails to build its **test**
profile due to unrelated, actively-churning concurrent work in OTHER files, confirmed across four
separate attempts over several minutes (peer error sets *changed* between attempts, proving it's
live concurrent editing elsewhere, not a stable bug in this file):

- Attempt 1: `⚙️engine/🧱️primitives`-adjacent signature drift — my own test call sites broke
  because `🔺️diff/🧱️primitives/🦀️.rs`'s `make_cylinder`/`make_sphere` dropped their `segments: usize`
  parameter mid-session (W1-E). **This one WAS mine to fix** (it broke my own file's call sites,
  not "unrelated churn") — fixed all 5 call sites (lines 1206, 1233, 1251, 1261, 1295) to the new
  4-arg/3-arg signatures. Confirmed no more E0061 from my file on re-run.
- Attempt 2: `⚙️engine/🦀️.rs` — `Entity::Curve(_)`/`Entity::Surface(_)` pattern arity errors (E0023,
  that file's own `PersistentLabel`-carrying enum variants, not mine).
- Attempt 3: `crate::artifacts::dwg` and
  `…::brep::schema::inferences::validation_report` unresolved imports (E0432) — both are
  deliberately-excluded-from-the-harness modules per `📓️h0-harness.md` §"Not mounted", pulled in
  transitively by some OTHER file's `#[cfg(test)]` module (not this file's).
- Attempt 4: `📸️snapshot/🕸️topology/🦀️.rs:752-753` — `topology::Body: serde::Serialize`/
  `Deserialize` unsatisfied (E0277) in THAT file's own test, `⚙️engine/📦️mesh-io/🦀️.rs`,
  `🔺️diff/{➡️sweep,🔺️euler,🧱️primitives,🧵️sew}/🦀️.rs`.

Ran a 5th attempt after the above (in case peers had converged): still blocked, this time by only
`📸️snapshot/🕸️topology/🦀️.rs:752-753`'s own `serde_json::to_string(&body)`/`from_str` test (E0277,
`topology::Body` doesn't derive `Serialize`/`Deserialize`) plus the same `⚙️engine/🦀️.rs` E0023s —
i.e. this looks like a **persistent**, not transient, harness gap (topology.rs's own test needs a
derive nobody has added), separate from the file-churn seen in attempts 1-4.

**In every one of the 5 attempts, `grep -n "🧩tessellation" <full compiler output>` returned zero
matches** — this file has never been the source of a build failure once the
`make_cylinder`/`make_sphere` argument-count fix above landed. I did not fabricate or infer a pass:
the test binary genuinely never finished linking, so `-- tessellation`'s actual pass/fail
per-test results were never produced. This is consistent with `📓️h0-harness.md`'s own status
line: "READY (with known-failing tests, all pre-existing / concurrent-work, not harness bugs)".

## Tests added (in this file, currently unexecuted pending a green harness build)

- `unit_box_has_six_face_groups_and_unit_normals` — 6 face groups, 12 triangles, 12 edge groups,
  unit normals, `face_groups`/`face_infos` `entity_id` parses as `u64` (`PersistentLabel`),
  `face_infos[i].surface_kind == SurfaceKind::Plane`, area == 1.
- `tessellate_face_matches_one_box_face`, `sample_edge_polyline_returns_line_endpoints`,
  `shared_edge_samples_are_identical_across_adjacent_faces`, `missing_solid_returns_missing_entity`,
  `tessellate_rectangle_wire_emits_edge_segments` — carried over from the old suite, adapted to the
  new algorithm's stable public surface.
- `circle_edge_samples_respect_deflection` — deflection 0.05 vs 0.005 (chosen so the chordal
  criterion dominates over `DEFAULT_ANGULAR_TOL`, unlike the old test's 0.2/0.02 which would now
  both be angular-tol-capped under the corrected exact formula).
- `cylinder_shared_edge_vertices_are_reused_exactly` — crack-free: every `sample_edge_polyline`
  point for the cylinder's circle edge is found verbatim (< 1e-6) among the solid mesh's vertices.
- `cylinder_lateral_face_area_matches_analytic_across_the_seam` — seam-crossing loop: lateral area
  within 2% of `2πrh`, `face_infos[0].surface_kind == SurfaceKind::Cylinder`, unit normals.
- `tighter_deflection_yields_more_triangles` — convergence: 0.02 deflection yields strictly more
  triangles than 0.2 on the same cylinder.
- `report_max_chordal_respects_deflection` — error certificate: `TessellationReport.max_chordal <=
  deflection * 1.05`.
- `face_with_circular_hole_triangulates_inside_the_trim` — hole: every triangle centroid lies
  inside the 4×3 outer rectangle and outside the r=0.5 hole circle.
- `sphere_caps_collapse_pole_to_single_vertex` — pole fan: for both hemisphere faces, exactly one
  mesh vertex sits within 1e-4 of the true pole point.
- `coedge_uv_prefers_stored_pcurve_when_present` — white-box test of `collect_loop_uv`/
  `coedge_point_uv` with a manually-set `Coedge::pcurve`/`prange` (no primitive populates pcurves
  yet — W1-E concurrent work) proving the p-curve path is taken over projection when present.

## Remaining / follow-up

- `cargo test -- tessellation` needs to actually run once the peer files above compile again —
  none of these tests have been executed; results above are compile-cleanliness only, not
  pass/fail.
- `try_flip_towards`'s constraint recovery is best-effort (bounded guard, silently leaves partial
  recovery on pathological input) — adequate for the box/cylinder/sphere/hole cases tested, not
  proven against arbitrary concave polygons.
- `weld_and_compact`'s spatial hash is O(n) average but the constraint-recovery flip search
  (`try_flip_towards`) is O(triangle count) per attempt — fine at current test scale, not tuned for
  very fine deflections on large faces.
