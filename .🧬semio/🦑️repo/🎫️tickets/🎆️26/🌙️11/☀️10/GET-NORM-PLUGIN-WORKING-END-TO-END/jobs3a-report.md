# jobs3a report: worker, worker-authority, preview, publication test targets

Logs: `🗑️generated/jobs3a-*.txt`. Build: `repo cargo --store=shared --configuration=prototype --parallelism=4 -- check|test ...`.

## Status
| crate | check --all-targets | test |
|---|---|---|
| job-worker | 0 errors (91 -> 0) | lib 6/30 pass; original-normal-failure 0/1, original-payload 0/1, original-worker-turn 1/3 (before the last fixes below) |
| job-worker-authority | 0 errors | 2/11 pass before fixture fix; fix applied (caller clock = params.now_us), not re-run |
| job-preview | 0 errors (20 -> 0) | 0/3; admission turn grant fix applied, not re-run |
| job-publication | 0 errors (5 -> 0) | 1/2; production fix applied (below), not re-run |

## Changes
- Cargo: preview and publication gained `testkit` feature, `semio-framework-job` testkit dev-dep, `semio-framework-value-testing-allocator` dev-dep (publication test target `required-features=["testkit"]`); worker gained dev-deps `semio-framework-value-testing-effect-bank` and `semio-framework-job-testing-caller`.
- Tests re-expressed under the post-merge API (3-arg `step` + `InteractiveJobStep::from_callback`, `close_original_retained_effect_step` / `close_original_effect_step`, `try_step_on_original_caller`, `pump_one_for_interactive_turn`, `step_on_original_caller`, `acknowledge_*` via caller turns, staged `WorkerJobSessionAdmission` instead of `WorkerJobAdmissionContext`/`try_admit_owned`, effect close via `bank_run` for wake owners).
  - Pool-submission tests (mounted submit, contention, pool rejection, panic wake) re-expressed on caller-run steps (pool/rejected/panic-capture states have no production entry any more). Quarantine-output page assertions dropped because jobs can no longer mint payload inside `step` (needs staged builder protocol).
  - erasure test rewritten on `OriginalJobFixtureCaller`; the case row `rawReceiptAccepted=false` is skipped (sealed `WorkerJobAdmissionControl`, `StepContext` cannot refuse a fitting receipt).
  - preview test rewritten on the fixture caller; stock derived as (admission+step grant)*maximumTurns from the fixture.
- Production fix: `RetainedJobPublication::close_step` now charges its metadata work through `effect.performed(1)` (it reported `work:1` without performing it, so fuel was never charged; cause of the publication `fuel` assertion 0 != quote).
- Stale fixture fix: worker-authority `Fixture::admit` used `logical_now_us` as caller clock while params carry `now`; admission binds the clock fn ("worker admission input differs from its actual source").

## Open findings (root causes not yet fixed)
1. Worker close ladder: `receive_original_worker_close_effects` fails with "reported fewer effects than its original recipient" (most worker lib failures). DEBUG probe showed one close step that posts copied_bytes=118 on the context while the returned step reports 0 (session_return frame test, 5th close turn after BeginClose). Production-side accounting mismatch in the worker close ladder (suspects: stage 2 `remove_terminal` / stage 3 params / retained wake close); fix in `worker/🎟️authority/🦀️.rs` or the close helpers. Probe removed.
2. `BeginClose` close turn reports `copied_items:0` now (old test expected 1); mounted-close fixture (`🧪️tests/🧫️fixtures/♻️mounted-close/🔣️.json`, schema, TS test) still lists `PreadmittedFault`/`PayloadLedger` phases and must be regenerated from the real ladder once item 1 is fixed (cross-language contract).
3. staged-projection test: "original effect close performed more work than it settled" for the payload builder `close_step` standalone effect close (job-payload area; not touched).
4. worker `original-normal-failure`, `original-payload`, `original-worker-turn` and lib wake/slots tests still failing; slots/wake "requires its actual admitting source" errors are cascades from earlier tests leaving parked retirement nodes (shared process-wide slots).
5. preview/publication need a re-run of `test` after the fixes above.
