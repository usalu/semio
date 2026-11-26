# Live cleanup pump `zero-progress` on a fresh xlsx import (elapsed 0us)

## Symptom
`natural-file.import-and-retirement-failed: original-turn.native-return-refused: ... plugin.internal.zero-progress: runtime live cleanup faulted for instance 2 ... (elapsed 0us, ceiling 8000us)` with the stdio-office wasm built 2026-11-26 11:36; the 2026-11-25 23:48 wasm opened the same files.

## Where it lives
- Verdict: `runtime_live_cleanup_nonterminal_status` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs`.
- Fault text: `RuntimeCleanupFaultVector "zero-progress"` (`🧹️cleanup-vocabulary`), thrown by `plugin_step_live_cleanup_cell` from `cell.maintenance_status`.
- Caller: the reactor turn `drive_turn_retirement` (`⚛️reactor/🔄️turn/🪜️entry-lifecycle-and-retirement/🦀️.rs`).

## Root cause
1. The CLOSE ladder's zero-progress verdict was already repaired to be priced in two currencies (8 fruitless steps AND `RUNTIME_CLOSE_STALL_CREDIT_US` = 8000us of real time; see the doc of `RUNTIME_CLOSE_STALL_CREDIT_US`, "a count is not a duration"). The LIVE (maintenance) pump kept the count-only rule: 256 consecutive `Progress(0)` or `Blocked` readings fault, whatever the clock says. The message quotes the elapsed time of the last turn (microseconds), not a stall duration, hence "elapsed 0us, ceiling 8000us".
2. The working tree changed `drive_turn_retirement` (uncommitted, today): the live pump used to run once per turn (more only while `plugin_live_cleanup_under_pressure`); it now repeats up to `LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN` (32) times on an ingress-free turn while a funded document archive is runnable (`archive_maintenance_slice_open`). A natural-file import is exactly such a document-archive load, so 256 fruitless readings (an unfunded ladder pass, or a `Blocked` "waits for its exact live media owner") are now reachable in about 8 quiet turns, i.e. within microseconds, long before any real stall.
3. Other uncommitted work that touches the same path and was in the 11:36 build but is not the cause: typed-operation retirement batching (`retire_granted_typed_operation_run`), parked projection cache (`invalidate_cache`, `cache_parked`; retired only by the close ladder), latest-wins `context_ready`. None of them changes the live verdict; they only change how many readings per turn are fruitless.
4. Not proven natively: which exact reading (zero receipt vs `Blocked`) the import produced. The `[DEBUG] lcsj` traces of the old wasm are gone from the source (no match in the tree), so they could not be compared. The fix is independent of which reading it was.

## Fix
The live verdict now uses the same two currencies as the close verdict: `RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT` (256) consecutive fruitless readings AND `RUNTIME_CLOSE_STALL_CREDIT_US` of real time since the pump last moved. A real, permanent stall still faults (after 8 ms); a burst of microsecond-long repeats cannot. A missing clock never faults. Progress, `AwaitingInput` and contention clear or ignore the run as before. A `ZeroProgress` fault now quotes the credit it spent, not the turn cost.

## Files changed
- `⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs`: new signature `runtime_live_cleanup_nonterminal_status(contended, progress, stalled, stall_since_us, now_us) -> (status, credit_us)`, caller stores the credit, `runtime_live_cleanup_publish_turn` quotes it.
- `⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🛎️app-cell/🦀️.rs`: `maintenance_stall_since_us`, `maintenance_stall_credit_spent_us`.
- `🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs`: new laws `live_zero_progress_cannot_fault_before_its_stall_credit_elapses`, `live_zero_progress_without_a_clock_never_faults`; the permanent-Blocked, host-input-wait, contention and progress-currency laws now run on the new signature with a virtual clock.
- `🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs`: caller of the verdict updated.

Everything under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`.

## Tests
Command (guarded, `-j 2`, private target dir): `claude-cleanup-pump-test.sh` -> `cargo test -p semio-plugin-tests-01-2 --lib runtime_close_budget_tests`. Result: 16 passed, 0 failed (`runtime_close_budget_tests`, includes both new live laws and the updated permanent-Blocked, input-wait, contention and progress-currency laws). Build 3m44s; the private target dir was deleted afterwards. The registration-test caller was only compile-checked (it is in the same crate build), not executed.

## Rebuild
Yes. The live verdict is compiled into every plugin wasm: stdio-office (and every other plugin component) must be rebuilt after this change; the 11:36 wasm still faults.

## Round 2 (13:46 wasm: `elapsed 8000us, ceiling 8000us`, instance 2 = the import instance)
The time credit worked (the clock now reads 8000us) but the verdict still fired, so the readings it counted were not evidence.

### Real root cause: starved readings counted as stalls
- `drive_turn_retirement` runs inside `with_original_retirement`, where `retirement_should_yield()` ignores the turn DEADLINE (fuel only). The funded natural-file decode job runs for milliseconds on the same turn and spends the deadline; the repeated live pump (now up to 32x per quiet turn while an archive is runnable) still runs.
- `maintenance_ladder_step` / `maintenance_ladder_unit_step` start with `if cx.should_yield() { return Progress(empty) }` (`should_yield` = fuel OR deadline). Every repeat after the decode job returned an empty receipt without the ladder having run a single unit.
- The close job already knows this class: `runtime_close_reading_is_starved` turns an empty reading taken with the budget spent into `contended`, which the verdict ignores (the 2026-11-25 fix, "elapsed 10500us ceiling 8000us"). `RuntimeLiveCleanupJob::advance` never did, so 256 starved readings plus 8 ms of the decode job's own wall time (the clock keeps running while the job works) satisfied both currencies.
- Pump semantics decided: a reading taken while the turn's budget was already spent is scheduling latency, never a stall; the live pump is not polled differently, and a genuinely wedged pump (empty or Blocked with its budget held) still faults after 256 readings and 8 ms. The ceiling is unchanged.

### Fix
- `life_cleanup_pumps` (`RuntimeLiveCleanupJob::advance`): after the source step, `runtime_cleanup_reading_is_starved(progress, cx.should_yield())` sets `contended`. The helper is renamed from `runtime_close_reading_is_starved` (now shared by close and live; close still passes `retirement_should_yield`).
- Test: `📦️instances/🧪️tests/🦀️.rs` law `host_starved_empty_live_steps_never_buy_structural_stall_credit` (1 024 starved readings over about 1.5 s of virtual clock never move the counters; the same pump with its budget held faults at 256 readings and the credit).
- Left alone (other agent's uncommitted change): the 32x repeat in `drive_turn_retirement`; it only wastes starved passes now. A later cleanup could stop repeating once `should_yield()`.

### Run
Guarded: `claude-cleanup-pump-test.sh` (`semio-plugin-tests-01-2` close-budget laws and `semio-plugin-runtime-instances` `stall_laws`). Result below.

Result: guarded run exit 0; `semio-plugin-tests-01-2` close-budget laws and `semio-plugin-runtime-instances` `stall_laws` (3 passed, including the new live starvation law). Private target dir deleted. stdio-office rebuild required; the split agent was told.

## Round 3 (14:38 / 14:47 wasm still faults: `stage=14 credit_us=64000`)
- Stage 14 is the store-replacement (document load) unit. Reasoning from the code: the live pump repeats up to 32 times per quiet turn and spends the turn's retained ITEM stock on its first pass; `try_step_on_original_caller` then refuses to step (item or depth stock 0) and the unit reads empty. Round 2 only covered a spent deadline or fuel.
- Fix: `runtime_live_pass_budget_spent` (`cleanup-pumps`) = deadline, fuel, item stock or depth stock spent; `RuntimeLiveCleanupJob::advance` uses it for the starved-reading rule. Law `live_passes_with_a_spent_item_or_depth_stock_are_starved_but_funded_passes_are_not`.
- Not the actual cause of the 15:11 failure (see round 4): the trace showed funded stock.

## Round 4 (15:11 wasm with the `[DEBUG] store replacement zero reading` trace)
Trace: state `Initializing`, grant 7 to 64 items, 1 MiB copy, 16 MiB capacity, quote `capacity 25088 depth 1`, 50 M fuel, no yield, no cancel, run 1 then run 33. Funded and not starved, yet zero readings in a stuck state.
- Cause (inferred from the code and trace, not observed directly): `drive_initializer` polls the decode job's worker session. `try_step_on_original_caller` answers `Idle` (receipt zero) while the session is not in its idle phase, i.e. while the step is running on its worker. The initializer mapped `Idle` to a zero-receipt `Progress`, so every poll during the decode counted as a stall, and 256 polls fit in 64 ms because the pump repeats up to 32 times per turn. The typed-operation worker path already maps the same `Idle` to `AwaitingInput`.
- Fix (`🗃️document/🧰️store-replacement-initializer-and-retirement/🦀️.rs`): `Idle` returns `AwaitingInput` (an external wait the live verdict clears); `Submitted` keeps its receipt.
- Bound so a dead worker cannot wait forever: the replacement records the instant of its first `Idle` in an unbroken run (`worker_idle_since_us`, reset by every other poll result). After `STORE_INITIALIZER_WORKER_IDLE_CEILING_US` (30 s of real time) of unbroken `Idle` it returns a framework fault, which the pump surfaces as an instance-scoped cleanup failure. A lost clock never faults.
- Laws: `an_idle_worker_is_waited_for_until_its_ceiling_and_then_declared_dead` (virtual clock: 1 024 idle polls over ms never fault; the ceiling faults; a reset restarts; no clock never faults) in `worker_idle_laws`; the existing input-wait law covers the verdict side.
- Known gap: the `Idle` to `AwaitingInput` mapping inside `drive_initializer` itself has no law, because it needs a live store-replacement job with a worker session (no fixture exists). The pure idle-ceiling helper and the verdict laws are tested; the mapping is only exercised by the browser open.

## Files changed overall (under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`)
`⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs`, `⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs`, `.../🛎️app-cell/🦀️.rs`, `⚛️reactor/🏃️runtime/📦️instances/🧪️tests/🦀️.rs`, `🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs`, `🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs`, `🗃️document/🧰️store-replacement-initializer-and-retirement/🦀️.rs`, `🗃️document/🧾️store-replacement-vocabulary/🦀️.rs`, `🗃️document/🔁️store-replacement-admission-and-member-drive/🦀️.rs`, and the temporary `[DEBUG]` trace in `🪜️maintenance-ladder/🦀️.rs` (remove once the open is confirmed).
