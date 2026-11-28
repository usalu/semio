# Wall-clock ceiling turned machine load into `plugin.internal.zero-progress`

## Where it was enforced
- `🏃️runtime/📦️instances/🔁️cleanup-turns` : `runtime_close_stall_verdict` and `runtime_live_cleanup_nonterminal_status` faulted `ZeroProgress` only when the fruitless count reached its limit AND `now - stall_since >= 8000us` (`RUNTIME_CLOSE_STALL_CREDIT_US`, `RUNTIME_LIVE_STALL_CREDIT_US`). A loaded machine satisfies the time half by itself, so load decided between yield and fault. Without a clock the live verdict never faulted.
- `🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps` : `RuntimeCloseCleanupJob::advance` classified a zero-progress reading as "starved" with `cx.retirement_should_yield()`. Inside the retirement loan that ignores the deadline, so a close step that read empty because its clock ran out was judged a funded stall (the live job already used `should_yield`).
- Classification site of the fault: `runtime_cleanup_reading_is_starved` / `runtime_maintenance_reading` (`Fruitless` is the only reading that buys debt).

## Contract now
- Expired budget with zero progress = starved: yields, buys no debt, retries on a later turn (close job now uses `cx.should_yield()`, deadline or fuel).
- The only fault input is a progress witness: `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT` (8 -> 64) / `RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT` (256) consecutive funded fruitless turns with no progress receipt (live: and no instance-epoch advance). Count is per turn (close ladder steps once per reactor turn; live repeat pumps read as starved).
- Wall time is only measured for the fault message (`measured_stall_us`); the deadline stays a scheduling hint. `RUNTIME_*_STALL_CREDIT_US` are reporting references.

## Files
- `cleanup-turns/🦀️.rs`, `🚪️opening/🦀️.rs` (limit 64, docs), `cleanup-pumps/🦀️.rs` (starved witness).
- Laws: `📦️instances/🧪️tests/🦀️.rs` (`a_starved_clock_completes_the_cleanup_across_turns_without_a_fault`, `a_starved_clock_never_faults_the_live_pump_across_turns`, `a_funded_body_that_never_progresses_faults_after_the_deterministic_bound`); `🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs` (`structural_zero_progress_is_decided_by_the_count_whatever_the_clock_reads`, `live_zero_progress_is_decided_by_the_count_whatever_the_clock_reads`, `live_repeat_pumps_buy_no_stall_debt_under_any_clock`; the three wall-clock-credit laws replaced).
- Vector `🧫️fixtures/🩻️live-stall-reading/🔣️.json`: the "fruitless count reached inside one frame" row encoded the wall-clock fault (`faults:false`); now `faults:true`, plus a frozen-clock row and a below-limit long-span row; `stallCreditUs` dropped. This fixed `live_readings_are_judged_by_their_cause_over_language_agnostic_vectors` (coordinator item).

## Status
Native, private `C:\t\zp`: `semio-plugin-runtime-instances close_stall_laws` 7/7; `semio-plugin-tests-01-2 runtime_close_budget_tests` 20/20 (includes the vector laws and the sticky prior-outcome vector law). `C:\t\zp*` deleted.
NOT re-run (build cost, hours per target on this machine): bin-05, reactor-jobs, the 2D/3D replacement laws and `selection-turn-cost`. Bin-05 and reactor-jobs do not depend on the edited crates; the replacement laws run the production cleanup and should be rerun once by the coordinator.

## Caveats
- I could not reproduce the browser fault; the starved-classification fix is the identified root cause, the count-only verdict is the structural guard.
- A wedged ladder under a permanently expired clock is "starved" and never faults (unfunded by definition); only funded turns count.
- Close limit raised to 64 because the time half no longer damps legitimately slow ladders. If a TS pin of the vector exists, update `faults` for the same row.
