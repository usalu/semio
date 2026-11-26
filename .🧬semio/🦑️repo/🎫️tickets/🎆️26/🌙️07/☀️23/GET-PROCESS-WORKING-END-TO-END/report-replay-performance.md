# Report replay performance (long history, 3061 operations)

## Measured (temporary `[DEBUG]` counters, all removed)

- Per operation the Report replay costs a constant ~190 `step_original_report_replay` calls, independent of history length: ~78 `operation_retirement.close_step` (retiring the prepared operation), ~60 `ArtifactReplayPreparationTask::step` (preparation), ~50 other phases. About 562k calls for 3061 operations.
- Each call was one driver turn and cost ~50 us: half is the per-call original receiving scaffolding (`receive_step` -> decode scope -> effect) plus `memmove` of the large async futures (drive future ~100 KB) per turn.
- After batching: ~16.8k driver turns (about 5.5 per operation, ~15-33 units per turn); the replay of all 3061 operations ends after ~9-11 s (debug build) instead of never finishing within 30 s.

## Fixes

1. `step_replay` (time-travel store-state) loops `step_derived_report_replay` inside one receiving scope while the turn grant (restricted by the grant the receiving scope issued, via the new `receive_step_issued`), the work claim (1024 charged per unit, floor 2048), the wall deadline closure and `!finished` allow. Receipts are summed so the turn receipt still fits the grant; fairness, progress and cancellation are unchanged.
2. Lost completion (latent bug exposed by the speed-up): `step_time_travel_replay` recorded the command receipt before applying `ReplayCompleted`; `apply_time_travel_event` answers `Busy` while a command receipt is pending, so a completed replay with a non-zero final receipt never left `Replaying`. The event is now applied first.

## Law

`the_report_replay_takes_a_constant_number_of_driver_turns_per_operation` (acceptance-blocked-review-and-long-history): replays 100 and 1000 downstream operations, at most 10 driver turns per operation (measured ~5.5).

## Finalize (second pass)

Profile of the finalize of 3061 operations (temporary counters, removed):

- Publication phase 4 (`advance_metadata`, per-edit digests) took ~2M `advance` calls (~650 per operation): `ArtifactCanonicalRecordDigest` hashed and counted one byte per unit and `NativeEditDigest` hashed one framed byte per unit. Now a unit batches up to 64 encoder steps / 64 framed bytes inside the same grant (work stays one unit per call); phase 4 dropped to ~110k advances.
- The close of the publication (`close_commit`, retiring the Report result) ran one unit per driver turn, ~70 turns per operation. `step_commit` now batches up to `min(grant items, 64)` close units per turn, stopping on `should_yield`, a settled outcome or no progress (same shape as the stage-1 loop).
- The authoring (undo) replay stepped one unit per turn and, unlike the Report replay, never drained returned snapshot reads: `step_derived_report_replay_batched` (new, in the lifted Store) now serves both paths, and the driver drains returned reads while an authoring replay runs.

Law: `the_finalize_takes_a_constant_number_of_driver_turns_per_operation` (marginal cost: turns(1000 ops) - turns(100 ops) at most 4 per operation); passes.

## Result

`time_travel_tests_1`: 31 pass, 1 fail (32 with the two new laws). `the_undo_of_a_finalize_over_a_long_history...` now gets through the first replay, finalize and commit (about 14 s) and fails in the undo's authoring replay at about cursor 20 of 28: the job's retained capacity stock runs out (the replay retains ~20 KB per operation, ~63 MB for 3061 operations, and the grant's capacity shrinks to ~3 KB), so the step makes no progress. This is a resource-accounting limit, not a turn count: next step is to cut the retained capacity per replayed operation (or release it as retire units complete) so two full replays fit the stock.

Not run: unit tests of `semio-os-store-canonical-json` and `semio-os-store-ledger` (their test targets do not compile in the current shared tree for unrelated reasons); the digests are exercised by the laws above.
