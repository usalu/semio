STATUS: EDITS COMPLETE, NOT FULLY VERIFIED — last check chk11 (generation3d --tests, 14 crates) stopped at FOREIGN framework errors (semio-framework-2d-booleans path flatten/boolean API drift, semio-framework-plugin-tool-run unresolved window_config_original); zero errors in any generation3d file. Earlier chk6: all touched crate libs compiled.

## Change
- `WidgetJob::step(&mut self, cx: &mut StepContext) -> Result<Option<WidgetStep>, OriginalTurnFailure>` is the only stepping API (fuel `step(usize)` and `step_on_original` deleted; `None` = grant admitted no progress). Implementers: `PhasedJob`, `ReadyJob` (source check only when the job was admitted under an original source), `OriginalFixedOutputJob`, `UnitJob`, `MachineJob`, `Interference`.
- Deleted `with_cold_step_context` and `cold_turn_fault` (the ignored-argument shim, shimaudit row 8/9). New in `⚙️compute`: `StepFault {Widget, Turn}` (a step stops by widget fault or by the funded context's refusal, never swallowed), `fuel_budget(cx)` / `spend(cx, units)` (units the context funds, derived from `cx.fuel_remaining()`; no local context issuing), re-exports `StepContext`/`OriginalTurnFailure`.
- `Pipeline::step(|work, cx| …)` returns `Result<Flow, StepFault>`; tessellate / operations / export_groups / brep-boolean `Fold::advance` / analysis-check `intersect` are driven by `cx` (`step_boolean_job_sync(job, cx)` directly; budgeted kernel calls take `fuel_budget(cx)` and charge `cx` the budget while Working, one unit when finished).
- Cold-oracle `Generation3dGeometry::compute` (cx-less trait method) drives `compute_cold_step(.., cx)` through the framework's `SelfFundedCloseCaller`; a refused turn becomes `InferenceFault` `…step-refused` carrying the refusal (`turn_fault`), not a constant.
- Test driver: `geometry_compute::testing::step_with_fuel` (cfg test/`testkit`, enabled by the 4 geometry test-support crates); ~25 test call sites rewritten.
- generation2d has no widget-job fuel path (nothing to change).

## Boolean law
`brep-boolean` fixture law (`🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json`, TS twin `🟦️.ts`) runs fuse/cut/intersect/compoundCut at fuel 1 and unbounded fuel against the fixture oracle (12 cases), plus determinism and cancellation laws; unchanged and now exercises `step_boolean_job_sync(job, cx)` through `Fold::advance`.

## Verification
- chk6: only errors were in io-geometry (5; 1 mine, fixed). Test targets and the boolean fixture law NOT run (blocked by the pre-existing io-geometry brep-interchange breakage and the pre-existing compute `testkit` feature that needs dev-only deps in lib builds).
- Tools: `C/1030-tools/stepctx/{chk.sh,both.sh,c*.py}`.


## Review fix (zero fuel / one charging rule)
- `fuel_budget` no longer `max(1)`; new `unfunded(cx)` (= `should_yield`), `settle(cx, budget, working)` (the one charging rule: working call spends its budget, finished call one unit). `PhasedJob`, `UnitJob`, `Interference`, `MachineJob` answer `Ok(None)` and do no work when unfunded; all budgeted kernel calls (tessellate, operations, CutAll, mesh machines, brep-interchange mesh import) charge via `settle`.
- Law test `an_unfunded_turn_does_no_work_and_a_funded_turn_does_at_most_its_fuel` (phased-job unit tests): 0 fuel runs 0 units, N fuel runs <= N.

## Other fixes made to unblock io-geometry
- `brep-interchange`: `crate::brep_interchange::{export,import}_step` (sweep damage, resolved to the local widget fns) -> `semio_s_artifact_stdio_step_geometry::geometry::*`; the stdio step geometry crate stub `…/📐️step/…/🚪️io/📐️geometry/📦️packages/🦀️rust/🦀️.rs` lost its `#[path = "../../🦀️.rs"] pub mod geometry;` line (restored). Not compile-verified.
- mesh-interchange `Machine::advance` x3 and brep-interchange mesh-import step rebased on `cx`.

## Still unverified
Test targets and the brep-boolean fixture law (fuse/cut/intersect/compoundCut, 12 cases) did not run; rerun `C/1030-tools/stepctx/chk.sh N --tests -p …` (see both.sh) once framework-2d-booleans / plugin-tool-run compile. Also pre-existing: compute `testkit` feature builds dev-only test code in the lib (blocks `-p semio-s-artifact-procedural-3d-geometry --tests`).
## Rerun after RESUME
- chk12: stopped at foreign `semio-framework-pixels` (CoverageJob API drift). chk13: stopped at foreign `semio-framework-2d-booleans` (3 errors at paths/🦀️.rs:43, peer mid-edit) + store-components. No error in any generation3d file. Test targets / boolean fixture law / zero-fuel law still not run. Rerun: `C/1030-tools/stepctx/chk.sh N --tests -p <14 crates, see both.sh>` once framework 2d is green.

## Port wave (Addendum 11:55)
- My stepping crates (phased-job, brep-boolean, analysis-check/-measure, mesh-support/-convert, mesh-interchange, brep-interchange, compute `StepFault/fuel_budget/settle/unfunded`) use none of the changed 🌱️value APIs (`close_step`/`RetirementStep::{Failure,Progress}`/`receive_birth_demand`/`begin_receive`/`RetainedWorkLedger`): nothing to pre-port. The only generation3d geometry files that do are the original path (`⚙️compute/🎟️original`, `🔌️inputs/🏗️constructor`, `🚂️engine`, `🚀️original-field`), not part of this lane.
- chk14 (--tests, 14 crates): still stops in the framework value layer, not in generation3d: `semio-framework-value-retained-map` (`next_return_demand` missing on `RetainedCloneBorrowAuthorityAdmission`, 🎟️lease:25) and `semio-framework-value-edit` (`admit_owned` missing on `RetainedCloneSource`, 📇️index/♻️transfer:29 and 🌳️root/🛬️receive:48). Per-crate status: all red/unchecked only because of these upstream errors; last compiled-green evidence = chk6 libs.
- Pending: rerun `chk.sh N --tests …` + brep fixture law + zero-fuel law when the value layer compiles.

## semio-framework-mesh-engine port (coordinator request)
- Edited to the peer's `RetainedWorkOutcome` / `RetirementStep(…, u64)` API (mirrored from `ControlledRetirement::close_step`): `mesh-engine/🌲️canonical/📥️decode` (close closure returns `RetainedWorkOutcome{result, work: close.last_turn_work()}`), `mesh-engine/🚪️io/📝️text` (`admit_owned_retirement(..).result`, `close_factory_ticket(..).result.map_err`), and upstream `semio-framework-async/🔔️wake/🧵️thread` (`Complete(0)`, `Progress(.., 1)`).
- Check mesh3 (`-p semio-framework-mesh-engine --lib -p semio-framework-async`): refused BEFORE compiling — `semio-framework-job-context` source file `🧵️job/⏱️context/📦️return/🎟️…` exceeds the crate source byte cap (foreign peer file). mesh-engine/async therefore UNVERIFIED (red until that file shrinks); my 5 edits are all single-expression ports.
- gen3d --tests laws still pending on upstream.
