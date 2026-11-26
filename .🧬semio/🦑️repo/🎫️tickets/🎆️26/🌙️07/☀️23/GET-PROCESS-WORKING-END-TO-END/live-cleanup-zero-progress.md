# Live cleanup zero-progress fault at boot

## Cause
- The browser message `zero-progress ... (elapsed 0us, ceiling 8000us)` is the signature of the count-only live stall rule: 256 fruitless readings (`Progress(default)` or `Blocked`) faulted regardless of real time, and the message quoted the last turn's cost (0us on a coarse browser clock). A booting document load keeps the live pump Blocked/zero-progress for hundreds of microsecond-long pressure steps per turn (`LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN` = 32), so the instance faulted at boot and every later turn re-returned the stored fault.
- The tree now already prices the live verdict in both currencies (`runtime_live_cleanup_nonterminal_status`: 256 readings AND `RUNTIME_CLOSE_STALL_CREDIT_US` of real time; message quotes the credit spent). `cleanup-turns/🦀️.rs` was last edited 11:53 and `app-cell/🦀️.rs` 11:51, so the tested hub was built before that fix. With the fix, the fault message could not read `elapsed 0us` for a live ZeroProgress (it must be at least 8000us).
- Lead 1 (cancel restart loop) is ruled out: a `Cancelled` outcome records no reading (`outcome_progress = None`), which `runtime_live_cleanup_nonterminal_status` treats as "not zero progress" and clears both the count and the stall instant. A session restarted every turn after a cancel therefore never accumulates stall.
- Lead 2 (time-travel effects, snapshot read drains, CancelPreview) does not touch the live pump's stall counters; not implicated.

## Fix
No further production change needed beyond the credit rule already in the tree. Rebuild the hub from the current tree.

## Laws
- Existing: `live_zero_progress_cannot_fault_before_its_stall_credit_elapses`, `permanently_blocked_live_cleanup_faults_after_its_stall_credit_without_claiming_released_ownership` (a genuinely stuck pump still faults, quoting >= the credit).
- New: `cancelled_live_cleanup_outcomes_clear_the_stall_run_and_never_fault` (close-budget law file, run in crate `tests-01-2`): after 255 stalled readings, 1000 cancelled (None) readings leave count and instant at 0 and stay Ready.

## Results
`cargo test -j 2 -- cancelled_live_cleanup live_zero_progress_cannot_fault permanently_blocked_live_cleanup` in `tests-01-2`: 3 passed, 0 failed (new cancelled-outcome law, credit-before-fault law, permanently-blocked-still-faults law).

## Changed files
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs` (new law)
- ticket: `live-cleanup-zero-progress.md`
