# Intermittent `plugin.internal.zero-progress` while retiring the predecessor

## Contract of the rule

`runtime_close_stall_verdict` (`runtime/instances/cleanup-turns`) judges each close-ladder step. A step is stalled only when the ladder was read and answered `Pending { 0, 0 }` (`PluginLifecycleStep::Progress(default)`). The fault needs `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT` (8) consecutive stalled steps and `RUNTIME_CLOSE_STALL_CREDIT_US` (8 ms) of wall time since the first one. `contended`, `Blocked`, `AwaitingInput`, `None` and any real progress buy no debt.

## Root cause

A step whose own budget (deadline or fuel) the host had already spent still produced a `Progress(default)` reading: `StepContext::retirement_should_yield()` makes the ladder (`close_step`, `advance_lifecycle`, `advance_close_batch`) return an empty yield before it does any work. The job reported that as an observed stall. In a loaded browser the turn is stretched past its deadline by host latency, so eight such starved steps over 10.5 ms faulted a ladder that was merely waiting (`elapsed 10500us, ceiling 8000us`). The wall-clock half of the rule cannot tell guest time from host time, so the fix is at the reading, not at the clock.

The ladder at that point waits on the retiring instance's own drain (result pages, maintenance session, mounted jobs); it is not waiting on anything unrecoverable, and when it genuinely waits on an external owner it already reports `Blocked`/`AwaitingInput` (ExternalWait, never faults). The hit is therefore a false reading, not a ladder that should not wait.

## Fix

- `runtime_close_reading_is_starved(progress, budget_spent)` (`runtime/staging/reactor-lifecycle/cleanup-pumps`): true for an empty `Progress` reading taken with the budget spent.
- `RuntimeCloseCleanupJob::advance` marks such a step `contended` (the one reading the verdict ignores without touching debt). A wedged ladder polled with full budget still faults; real progress and external waits are never reclassified.

## Files

- `🧰️framework/…/🔌️plugin/⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs` (helper, `advance`).
- `🧰️framework/…/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs` (no net change; helper lives in the lifecycle crate that owns the job).
- Tests: `🧰️framework/…/📦️instances/🧪️tests/🦀️.rs`, module `close_stall_laws`: `host_starved_empty_close_steps_never_buy_structural_stall_credit` (injected clock past every deadline, 1024 steps spaced 1.5 ms, never faults) and `full_budget_empty_close_steps_still_fault_and_other_readings_are_never_starved`.

## Tests

Run: `native-cargo.sh test -p semio-plugin-runtime-instances --lib close_stall` (log `🗑️generated/zp-2.log`). The companion crate `semio-plugin-tests-01-2` that hosts the older close-budget laws does not compile today (25 pre-existing errors: missing imports of `runtime_close_stall_verdict`, `TimeTravelExtension`, `OperationId` type drift), so the new laws live in the owning crate. The first red run (`zp-1.log`) failed only on the missing helper plus those pre-existing errors.

## Browser

The fix is guest/framework Rust compiled into the plugin runtime wasm. A wasm rebuild and re-staging is required for the browser to pick it up. No TS change.

## Follow-up: companion crate `semio-plugin-tests-01-2` compiles and is green

Result: `native-cargo.sh test -p semio-plugin-tests-01-2 --lib` gives 17 passed, 0 failed (after one run lost to a full disk, `No space left on device`, unrelated to the code).

The 25 errors were test-host drift, not product bugs:

- `runtime_close_stall_verdict` and `runtime_live_cleanup_nonterminal_status` were not imported into the `runtime-instances` test host: imports added to `🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/📦️runtime-instances/🦀️.rs` (`inst_cleanup_turns`).
- `preparation_receipt` carries a plain `u64` operation id now, not `OperationId`: `🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs` uses `77`.
- `TimeTravelExtension` lives in the `semio-plugin-time-travel-turns` crate, which the companion did not depend on: dependency added to the companion `Cargo.toml` (feature `artifact-app-testing`), host import in `🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/⏳️time-travel-turns/🧱️unit-1/🦀️.rs`, and `use super::TimeTravelExtension;` in `⏪️time-travel/🧵️instance/🧪️tests/🦀️.rs`.
- `RuntimeCleanupFaultVector` was private but is returned by the public `RuntimeCleanupFault::vector()`: now `pub` with `#[doc(hidden)]` (`🧹️cleanup-vocabulary/🦀️.rs`).
- The 3 errors from my own draft starvation tests are gone: the tests live in `semio-plugin-runtime-instances` (`close_stall_laws`, 2 of 2 green).

No genuine red test remained once it compiled. No wasm rebuilt.

## Follow-up 2: `semio-plugin-tests-06-2` (not started)

Stopped before any edit or build: `df` showed 19 GB free on `/System/Volumes/Data` (guard: 20 GB). Nothing changed for this crate.
