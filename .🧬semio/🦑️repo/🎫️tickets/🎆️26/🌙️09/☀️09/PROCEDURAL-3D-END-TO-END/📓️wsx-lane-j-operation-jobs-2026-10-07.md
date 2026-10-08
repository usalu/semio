# ⏱️ Lane J — Resumable, cancellable B-Rep operation jobs

Crate `semio-framework-3d` (`🧰️framework/🔨️modules/🧊️3d/📐️brep/`). Paths below are relative to that folder. The framework is now its own Cargo workspace: run tests from `🧰️framework/` (`cargo test -p semio-framework-3d --lib operation_jobs`).

## 1. Design

* **One staged contract** (`🛠️operations/⏱️staged/🦀️.rs`): `StagedOperation::advance` runs exactly one unit of the algorithm's own loop; `Plan` holds counted phases; `drive*` is the one-shot façade. Every kernel one-shot (`fillet_edges`, `offset_solid`, `revolve_face`, `loft_profiles`, `make_convex_hull`, `compound_cut`, …) is now `drive_*(Job::new(..))` — one algorithm per operation, no duplicate.
* **Isolation instead of rollback**: the engine job runs inside a private working copy (`Body::extract`) of exactly what its inputs reach. The session is not touched until the job is `Ready`, when only the result is appended (`Body::absorb`, labels re-minted so no two live entities share a label; label numbering equals the old one-shot's when nothing else interleaved). Cancel/failure therefore leave the session byte-identical, no orphans, inputs never flipped/consumed.
* **Progress**: `done` is monotone, `total` is only revised upward while running (nested boolean/blend/hull reveal their size on admission); on the last step it may shrink once (`Plan::settle`) when a nested boolean planned pessimistically (it skips imprint rows of coincident faces). `done == total` at `Ready`.

## 2. Public API (exact signatures)

Engine (`⚙️engine/⏱️operation-jobs/🦀️.rs`, re-exported from `⚙️engine/🦀️.rs` region `OperationJobs`):

* `pub enum BrepOperation { Fillet, FilletVariable, FilletEdges, Chamfer, ChamferAsymmetric, ChamferEdges, Shell, Draft, OffsetSolid, OffsetFace, ThickenFace, Defeature, Extrude, ExtrudeWire, Revolve, Sweep, Pipe, HelicalSweep, Loft, Section, Split, CompoundCut, LinearPattern, CircularPattern, GridPattern, ConvexHull }` (:35, struct-like variants carrying handles + parameters; `tag() -> &'static str` :64 = the fixture's camelCase kind).
* `pub struct BrepOperationProgress { pub done: usize, pub total: usize, pub phase: &'static str }` (:107)
* `pub enum BrepOperationAdmission { Answered(Vec<GeometryHandle>), Job(BrepOperationJob) }` (:121) — `Answered` only for patterns of < 2 instances.
* `pub enum BrepOperationStep { Working(BrepOperationProgress), Ready(Vec<GeometryHandle>), Cancelled(BrepOperationProgress) }` (:129)
* `pub struct BrepOperationJob` (:156): `pub fn progress(&self) -> BrepOperationProgress` (:165), `pub fn cancel(&mut self)` (:172), `pub fn is_terminal(&self) -> bool` (:182)
* `Brep::operation_job_sync(&self, operation: BrepOperation) -> Result<BrepOperationAdmission, BrepError>` (:506)
* `Brep::step_operation_job_sync(&mut self, job: &mut BrepOperationJob, budget: usize) -> Result<BrepOperationStep, BrepError>` (:545)
* `Brep::run_operation_sync(&mut self, operation: BrepOperation) -> Result<Vec<GeometryHandle>, BrepError>` (:586) — all 26 `*_sync` methods (`fillet_sync` … `split_sync`, `convex_hull_sync`) now call it (:611-716), signatures unchanged.
* `Body::extract(&self, roots: &[EntityRef]) -> (Body, MergeMap)` and `Body::absorb(&mut self, scratch: &Body, roots: &[EntityRef], scratch_base: u64) -> MergeMap` (`📸️representation/🕸️topology/🦀️.rs:972/987`); `merge_selected` now delegates to the shared private `copy_in` walk (:883/893).

Kernel jobs (all `: StagedOperation`, all `::new(&Body, …) -> Result<Self, KernelError>`):
`BlendJob::{fillet,variable_fillet,chamfer}` (`🎨️blend:1044`), `OffsetFaceJob, OffsetSolidJob::{new,with_default_corner}, ThickenJob, ShellJob (Closed/Open), DraftJob` (`↔️offset/⏱️jobs`), `DefeatureJob` (`🧵️sew:178`), `ExtrudeJob, ExtrudeWireJob, StationSweepJob, SweepJob::{pipe,helical}` (`➡️sweep:41-343`), `RevolveJob` (`🌀️revolve:250`), `LoftJob` (`🥞️loft:61`), `ConvexHullJob` (`🧱️primitives:657`), `SectionJob, SplitJob, BooleanFoldJob::{compound_cut,pattern}` (`🔀️boolean/⏱️jobs`). `BooleanJob`/`BrepBooleanJob` API untouched (`BooleanStage` wraps it).

## 3. Step granularity (one unit = one `advance`)

| op | units |
|---|---|
| fillet / fillet_variable / fillet_edges / chamfer* | per selected edge patch, per reached-vertex corner, per patch trim, per corner vertex mint, per carried edge, per patch seam, per original face rebuild, per patch face, per corner face, +1 shell/solid (box 1 edge: 28, 12 edges: 79) |
| offset_solid, closed shell | per face surface offset, per vertex / edge / face of the shared `Rebuild` pass, +1 close; `Round` corner nests a `BlendJob` (its units appended) |
| open shell | surfaces, rebuild passes, per kept face copy, per open-face rim coedge, +1 close |
| draft | per drafted face, rebuild passes, +1 close |
| offset_face | 1 surface, per boundary coedge, 1 face, 1 p-curves |
| thicken_face | offset_face units, per ruled side, +1 |
| defeature | per removed face, +1 commit |
| extrude / extrude_wire | (1 face) + 1 setup + per profile coedge + 1 close |
| revolve | partial: 1 setup + per coedge + 1; full: per coedge + 1 |
| sweep / pipe | straight line → extrude, arc → revolve; general: per path edge sampled, 1 RMF, per guide frame, 1 align, per segment (1 + per profile coedge), 1 close |
| helical_sweep | 1 frames, 1 align, `⌈24·turns⌉·(1+coedges)`, 1 close |
| loft | per (loop, edge position) lateral, +1 close |
| section | per face, per edge, +1 |
| split | tessellation units (nested `TessellationJob`), per 256 classified triangles, per side soup/hull (`ConvexHullJob` units on fallback) |
| compound_cut, linear/circular/grid pattern | the nested `BooleanStage` units of every operand (imprint pairs, apply, classify, stitch, validate, +1 finishing) |
| convex_hull | per input point dedupe, 1 seed, per point insert, per hull vertex, 1 coplanar merge, per polygon, 1 close |

## 4. Tests (all run by me)

Fixture `⚙️engine/🧫️fixtures/⏱️operation-jobs/🔣️.json` (language-agnostic: 32 cases covering all 26 operations incl. expected volume/area/face count/unit bounds, 25 refusals/failures, 15 independent parry3d oracles (cuboids, cylinder, hull), exact pre-refactor integrals for curved cases). Tests `⚙️engine/⏱️operation-jobs/🧪️tests/🔬️unit/🦀️.rs`:

* `oneshot_facade_matches_fixture`, `job_driven_one_unit_at_a_time_matches_oneshot` (budget 1: session byte-identical after every `Working` step, monotone progress, `done==total`, calls == units, unit bounds, parry3d oracle), `job_budget_does_not_change_the_result` (budgets 1/3/1000/MAX leave a byte-identical session), `cancel_at_every_sampled_step_leaves_the_session_untouched` (exhaustive at every unit for the 25 cheap cases, sampled 0/1/mid/last-1 otherwise; also cancel-then-rerun == never-started), `refusals_and_failures_precede_any_mutation` (25), `fixture_covers_every_operation`, `committed_results_never_share_a_label_with_existing_entities`, `extract_and_absorb_isolate_a_working_copy`, `single_instance_patterns_answer_immediately`; `#[ignore] exact_integrals_of_curved_results_match_the_original_implementation` (slow adaptive integration).

Results (nightly-2026-07-20, debug):
* `cargo test -p semio-framework-3d --lib operation_jobs`: **9 passed, 1 ignored**, 3–6 s; repeated 10× on the built binary with default threads: 10/10 green (no flake).
* `brep::engine`: **68 passed, 0 failed, 1 ignored** (196 s) — includes the pre-existing engine tests.
* `brep::operations` (before the workspace split): **134 passed, 0 failed** — blend/offset/sweep/sew/primitives/boolean/transform regression.
* Stdio-plugin kernel integration tests run from a scratch crate: `brep_extrude_orientation` 13/13, `brep_shell_orientation` 2/2, `brep_tessellation_jobs` 10/10, `brep_procedural_example_booleans` 3/3, `brep_analytic_blend` 6/7 — the 7th (`…stays_inside_the_interactive_budget`) is a 200 ms wall-clock gate that failed only under machine load ≈70 (fillet alone 1.3–2 ms; tessellation 296–900 ms; two of three attempts were inside budget).
* `#[ignore] exact_integrals_of_curved_results…` was started and stopped after 6.5 CPU-minutes without finishing (lane A's integrator makes exact NURBS/revolve volumes minutes-long in debug); it is **not verified** — the mesh-measured fixture values (which equal the pre-refactor volumes within tessellation error, e.g. sweep 0.091669044 vs 0.091669048) are what is verified.
* The earlier "4 failures" reported by lane A did not reproduce; most likely cause was the not-yet-fixed `merge_coplanar_triangles` HashMap iteration order (convex-hull inputs differed run to run, which the byte-identity tests detect). Fixed (clusters and boundary start now ordered); see §5.

## 5. Behaviour changes worth knowing

* Inputs are never mutated: `extrude/revolve/sweep/pipe/loft/helical/thicken` no longer flip or absorb the session's profile faces; `defeature_sync` returns a **new** solid (it used to edit the input in place and return the same handle); patterns no longer leave intermediate fused solids/copies as live handles.
* `make_convex_hull` is now deterministic (plane clusters sorted by key, boundary walk starts at the smallest vertex); section samples edges in sorted order (was HashSet order).
* Offset/shell/draft rebuild passes iterate sorted ids (deterministic ids/labels).
* Removed dead helpers: `copy_faces`/`FaceCopy`, `build_prism`, `frame_stations`, `sample_path`, `rebuild_topology`, `convex_hull_3d` (all crate-private).

## 6. Remaining gaps

* `fuse/cut/intersect` (`BrepBooleanJob`) still run in-session and, when cancelled, leave unreachable garbage until the next compaction (not in the requested list; could be re-expressed as a `BrepOperation::Boolean` over `BooleanStage`).
* Kernel faults surfaced, not fixed: `chamfer_asymmetric` over all edges fails at corners (box) / seams (cylinder); `grid_pattern` 2×2 with overlapping boxes fails late inside the boolean engine (`missing entity: edge …`, fixture `fail-grid-pattern-overlapping-boxes`; the job fails at unit ≈ mid-run and leaves the session untouched); `defeature` discards its `sew_faces` result; `section` builds one face from unordered edge/vertex hits (convex-order not guaranteed, curved sections still outside the exact path).
* Genuinely monolithic units remain: boolean stitch, `solid_from_triangle_soup`, `propagate_rmf`, coplanar merge of the hull, `TessellationJob` face meshing, each one honest unit.
* Mesh-measured fixture cases (curved/free-form results) use tessellation volume because lane A's stricter adaptive integrator makes exact volumes of NURBS sweeps take minutes in debug; their exact pre-refactor integrals sit in the fixture and are checked by the `#[ignore]`d test.
* `BrepOperation` has no `ToValue`/`FromValue` derive yet (fixture is the schema); working copy drop is a plain `Drop`, not the `PayloadRetirement` path.
