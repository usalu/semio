# Native XLSX history green: status (incomplete)

## Result
Not green. Base was last executed at 46 tests with `history_edits_end_to_end` and `conflict_history_edits_end_to_end` failing (the other 44 not re-verified after later edits). Strict, Transitional, Strict mutations and Base mutations were not run. The repo cargo MCP tool stopped answering (PreToolUse hook timeout) after a 2h38m build, so no further runs were possible.

## Environment repairs (concurrent-edit collisions)
- `OriginalOperationReceivingAdmission::binary` built the struct with `comparison_source`; the field is `pack_value`. Fixed in `...operation-wire/receiving/binary`.
- `plugin-app-impl/rs` had been overwritten by a stale copy that declared five modules that now live in `members/staging`. Restored from the index (stale copy kept at `generated/plugin-app-impl-stale-working.rs.txt`).
- `pack/value/decode/receiving/rs` lost its `close_work_demand` quote and call site that `semio-os-dsl` uses; re-added, other agent's two unrelated hunks kept.
- Xlsx editor unit tests lost three imports (`XlsxCell`, `build_minimal_xlsx`, `apply_xlsx_mutation`); added at the file top. Not yet compiled.

## Diagnosis so far
Both failures are the same. The captured run shows stage Finalizing, pending history work, and a close ladder quote of release 848, depth 3 with `sessionRetired=false`, `sessionOwner=false`, `closing=true`. The session retirement demand (4224 copy bytes, depth 1) is never reached because `time_travel_close_demands` and `time_travel_close_step_context` take the `owners().any(finalizing)` branch first and only drive `StepCommit`. In `step_commit`, `close_commit` runs only when `commit_closing` is set (cancel, failure or stage 4 completion). A close (not a cancel) while a finalize is in flight therefore keeps running the forward stage machine, which never completes. The primary test also stalls in Finalizing before any close, so the stage machine itself stalls; which stage is not yet known.

## Next step
A `[DEBUG] step_commit` line (first 12 calls, then every 2000th) is in `time-travel/store-state/finalize/rs`. Run the base suite with `-- history_edits_end_to_end`, read the stage and publisher state, fix, then remove that line and `commit_debug`.

## Update: close-safety change (unverified)
Added `TimeTravelStoreCommand::CloseCommit` (store-state `rs`): sets `commit_closing=true` and drives `step_commit`, so `close_commit` retires any in-flight finalization. The close ladder's finalizing branch (`time_travel_close_step_context`) now issues `CloseCommit` instead of `StepCommit`. This guarantees a close arriving mid-finalize completes, but it is not compiled or tested: every `mcp__repo__*` call has failed with a PreToolUse hook timeout for over 10 minutes, including after three 2-minute waits. The primary-test stall in Finalizing (before any close) is still undiagnosed; the `[DEBUG] step_commit` print and `commit_debug` remain and must be removed when done.
