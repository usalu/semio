# F3 root cause (sync-solver lead falsified; cause is per-unit turn cost in the numerical children)

## Lead verdict: FALSE
- `mounted_results_frames_never_invoke_synchronous_solvers` (editor_app_2, run alone, `🗑️generated/f3/law-d.log`): `assert_eq!(results_solve_count(), 0)` PASSED. It failed only on the first `close_step` assertion (expected `Progress`, a nothing-mounted close returns `Complete`). That is a law-premise bug, relaxed concurrently by the coordinator to `.expect("... without a fault")`. After that, `mounted_results_` (both mounted laws) PASS (`law-e.log`: 2 passed).
- Code reading agrees: with an operation, `results::render` only calls `live.with_result_fields` (borrow of the job's `Fem3dResultBorrow`) or `render_pending`; `with_static_results`/`with_mode_values` (the only inline solvers, `note_solve`) are reached only when `operation` is `None`. `fem3d_scene_parts` meshes solids per frame (`mesh_solids_partial`: triangulate plus extrude, cheap) and never solves.

## Real cause (found by the coordinator, confirmed by my audit)
The caption `Preparing analysis` is `progress_stage() -> "fem3d.preparation"`, the fall-through for every Solid*/Reserve*/Element* stage of `Fem3dNumericalStage`. The browser wedge after Add Solid, and with House, is the numerical child burning an enormous number of turns in the delegated children: the default slab needed 249,763 turns in `AssemblyJob` (`✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🔶️csr-build-and-run`), which did ONE cell/merge per `run` and returned `Yield` while fuel remained. Every turn costs a render plus a job step round, so it looks like a permanent wedge (21+ min) and healthy solves take 33-41 s. Fix (coordinator, not mine to touch): loop in `AssemblyJob::run` until `should_yield()`, cancellation, preview/checkpoint due or Complete.

## Audit of the other delegated children (read-only, no edit)
- Driver `MountedJob::step` (`🧵️session/⚙️mounted-job`) already drives up to `UNITS_PER_STEP=65536` units until `STEP_CEILING_MS=8` ms, but breaks after ONE unit when `default_now_us()` is `None` (`deadline.is_none()`), so a missing clock degrades every child to one unit per turn.
- `PcgJob::run` (`🔢️sparse/🔁️pcg/🏁️run-and-entry`): already loops `while units < batch_units && !should_yield() ...`. OK.
- `MeshJob::run` (`🕸️mesh/🕹️mesh-job-run`): one bounded unit per `run`, `consume_fuel(1)`, `Yield` after each stage unit, and `step_plain` returns `None` on `Yield`. Same one-unit-per-turn shape as the old assembly. Candidate to loop in `step_plain` until `should_yield()` or a non-`Yield` run.
- `LdltJob::run` / `SubspaceIterationJob::run` (`🔺️ldlt/🚀️run-and-close`, `🎵️subspace/🏃️run-and-publish`): `run` wraps `run_admitted`, charging `consume_fuel(1)` only when no work was spent; each call advances one rung and returns `Yield`. Same candidate fix. Not changed here (modal/buckling only, not on the Add Solid path).
- The `PrepareAssembly`/mount stages are advanced one item per `advance_one` call in `🪜️solve-step/🍓️staged-step`; the per-unit cost there is the same driver loop.

## Files touched by me
- `✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🧪️tests/🔬️unit/🦀️.rs`: new law `add_solid_settles_the_results_3d` (demo example settles, add a 3x3x0.3 solid, results must settle with no Preparing/queued/Solving caption within 600 pump rounds). No production code edited.
- Script: `f3-law.sh` (needs `CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false`, otherwise `semio-os-spr` fails with os error 206 even with equal target and build dir).

## Law verdicts
| law | verdict |
|---|---|
| mounted_results_frames_never_invoke_synchronous_solvers | PASS (after the close_step relaxation) |
| mounted_results_render_borrows_jobs_without_synchronous_solves | PASS (`law-e.log`) |
| add_solid_settles_the_results_3d (new) | RUNNING at write time on a binary built BEFORE the assembly fix; still pending after 10+ min, consistent with the turn-cost cause; rerun after the fix lands |
| superseded_solve_settles_on_the_latest_revision_3d | NOT RUN |
| burst_of_edits_with_undo_redo_settles_3d | NOT RUN |
| example_switch_during_a_running_solve_settles_on_the_new_example_3d | NOT RUN |

Reasons for NOT RUN: two full rebuilds (about 25 min each, os error 206 then disk-full 112 then other agents' mid-edit framework breaks) consumed the session. The three settle laws pump 600+ rounds and are only meaningful on a build that contains the assembly loop fix.

## Next
1. Rebuild with the assembly fix, run the four settle laws one filter at a time with `f3-retry.sh`.
2. Apply the same loop-until-`should_yield()` in `MeshJob::step_plain`, `LdltJob`, `SubspaceIterationJob`, and make `MountedJob::step` keep driving when `default_now_us()` is `None` (use fuel only).
3. Execute-disabled is not solve-related (blank required fields, other agent).
