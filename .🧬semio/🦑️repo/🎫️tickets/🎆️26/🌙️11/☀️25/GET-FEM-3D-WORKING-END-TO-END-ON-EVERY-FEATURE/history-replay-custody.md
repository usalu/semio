# History replay custody and close zero-progress

## Task A: sealed source requires original granted handback
Root cause: the owner is NOT the replay/preview/displaced operation. It is the instance projection cache `CommandHistory.cache`
(`AppProjectionCache` = key, Arc<Snapshot>, Arc<Config>, Arc<HistoryView>). Every invalidation did `history.cache = None` (29 sites, e.g.
`drive_time_travel_finalize_outcome` after a history-edit finalization and `step_reprojection_turn` after adoption). When the store swaps its
document the cache holds the LAST `Arc<Fem3dSnapshot>`; the plain drop runs `SealedShared::drop` (debug assertion). Only apps with sealed lists
(FEM 3D) trip it. (Backtrace frame 9 is `drop_glue<Option<AppProjectionCache>>`; frame 10 is a mis-resolved nearby symbol.)

Fix (existing retirement machinery, assertion untouched):
- `CommandHistory::invalidate_cache()` replaces all 29 plain drops: a cache that is the last alias of its snapshot/config parks in the new
  `cache_parked`; otherwise it drops (a non-last alias is safe).
- `retire_cache_step` / `cache_retirement_demand` / `cache_parked_pending` hand the parked cache through `admit_owned_retirement` into the
  existing `cache_retirement` ticket (same logic the close ladder used, now shared; `CacheRetirementRoots` type moved here).
- The maintenance ladder `CommandPrune` rung (position 0) drains parked caches under a funded effect (`plugin_effect_close`), quote included.
- The close ladder uses the same helper (also drains `cache_parked`; terminal check and frontier name updated).
- `refresh_cache` parks a stale last-alias snapshot/config instead of dropping it when the store generation moved.

Files: `🧵️instance/📜️history/📥️staging/🧱️components/🦀️.rs`, `🪜️maintenance-ladder/🦀️.rs`, `🪜️maintenance-ladder/📥️staging/🔒️close-ladder/🦀️.rs`,
`🧵️instance/🖼️render/📥️staging/📜️history-view/🦀️.rs`, plus the one-line `invalidate_cache()` swaps in 13 instance/time-travel files.

Result (`hr-law.sh a1 context::history_edits_end_to_end`): the `sealed source requires original granted handback` panic is gone. The law
still FAILS later: `the instance never settled within its original turn bound`, History stage `Finalizing`, `step_commit stage=0 closing=true
pub=true resident_empty=false` repeating (the history-replay agent's `[DEBUG] step_commit` lines are in the store). That is a separate
commit-publisher close stall, not custody; not fixed here.

## Task B: zero-progress close cleanup (analysis only, not fixed)
Pump: `⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns` `runtime_close_stall_verdict`: faults when `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT` (8)
consecutive `Progress(0,0)` readings AND wall time since the first one >= `RUNTIME_CLOSE_STALL_CREDIT_US` (8000us). 30100us over 8 readings
means ~4 ms steps that were NOT host-starved (that case is already ignored) but returned an empty `Progress` while the ladder waited.
Suspect: a FEM 3D mounted-job close rung (`✏️editor/🧵️session/🧹️recovery-and-close`, `retire_one` -> child `close_step`
`session_pending(0,0)` / `InteractiveJob::close_step`) reporting a zero receipt while a running solve/mesh job drains, instead of
`AwaitingInput`/`Blocked`. Not reproduced natively (needs a running mounted job being closed); an existing TS-side fix for the actor
`closeJobs` is reported in `import-examples.md`. Next step: a law in the session unit tests that closes a child with a live mesh/pcg job
and asserts no run of 8 consecutive zero receipts.

## Update (coordinator follow-ups)
- `context::history_edits_end_to_end` PASSES (1267 s, then again 1313 s together with `model_window::transform_utility*`). The earlier
  `Finalizing` stall did not reproduce on the current tree (it was seen once on a tree carrying another agent's `[DEBUG] step_commit` lines); no
  commit-publisher change was needed.
- REGRESSION fixed: wiring parked caches into the LIVE maintenance ladder (`CommandPrune` rung) made the live cleanup pump see owed work it
  could not drain (rung quote vs the small live grant), giving `plugin.internal.zero-progress` at boot; and parking on `strong_count(config)==1`
  parked a cache on almost every invalidation. Now: parking only when the SNAPSHOT Arc is unique (`invalidate_cache`, `refresh_cache`), and parked
  caches drain ONLY in the close ladder's retained-fields rung (`retire_cache_step(live=true)`); the live ladder is untouched. Trade-off: a
  parked (rare, last-alias) cache is held until close.
  Missing: a native boot law for the live pump (needs the browser/pump harness); verified only by the two editor_app laws above.
- Task B law `fem3d_numerical_child_close_with_live_solve_never_reports_a_run_of_empty_receipts` (visual-job unit tests) PASSES on first run, i.e.
  it does NOT reproduce the Import Document zero-progress: the numerical child close ladder reports real receipts for 1..600 solve turns.
  The zero-progress source is therefore elsewhere (other close rungs of the old document's instance, or the actor `closeJobs` TS path); no
  close-step change was made. The law is kept as a regression guard.
- Disk: build dir `C:\t\fx7b` hit 100 % disk twice; stale older-hash dirs under `debug/build` were pruned (not other agents' dirs).
