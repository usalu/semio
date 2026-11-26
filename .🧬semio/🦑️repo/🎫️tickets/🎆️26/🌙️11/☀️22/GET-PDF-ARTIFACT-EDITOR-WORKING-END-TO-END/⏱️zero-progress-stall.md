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

## Follow-up 2: `semio-plugin-tests-06-2` compiles; 23 of 27 laws green, 4 open

All compile errors of the crate (extension retirement, invoke, invoke-failure, inference gateway) are fixed against the current API, with no shims. Run single-threaded through the gate (`--test-threads=1`).

### Drift fixed

- Close frames now run inside an original effect (`close_original_effect_step`, `step_granted(body, effect)`, `begin_granted(value, effect)`): `🧪️tests/🔬️extension-retirement` `Resource::close_step` rewritten (also falls through from an empty begin to the step in the same effect); invoke and invoke-failure tests use `close_original_effect_step(.., |effect| owner.step(copy, effect))`.
- Fault-reply and gateway owners quote their real minimum copy (about 424 bytes) and need an original parent source for effects: grants are `copy.max(demand.copy_bytes)`, productive turns run under `OriginalJobFixtureCaller::bare_turn` (`job_caller`, `paid_turn` helpers), cancellation through `caller.cancel()`; gateway test widens its grant per quote and captures non-`Copy` results outside the loan.
- Receiving-failure test builds the cause with `work_refusal` and `retirement_failure` that the shared fixture expects; context-failure fuel check compares the fuel debit with `original_work_units_spent()` (performed work is charged, not 1).
- Refusal kinds are camelCase on the wire: `🧫️fixtures/🧩️extension-retirement/🔣️.json` (`invariantViolated`).
- Suspend/exported-poll test: the retained turn input is built from the context's own operation, generation and grant (`retained_turn(cx)`), as the kernel requires.
- Product visibility: `ExtensionRequestHandler` is `pub` and `#[doc(hidden)]`; `semio_framework_job` re-exports `CommitPolicy` and `validate_commit_with` (another agent's in-flight API).

### Open (4)

1. `original_inference_gateway_failure_all_owned_causes`: external blocker. `intrinsic Pack source changed its declared value authority` from the borrowed Pack cursor (`🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor`) that another session is editing. Retry once the Pack files are quiet 15 minutes; fix at the root if it still fails on a quiet tree.
2. `original_extension_context_failure_preserves_complete_canonical_wire`: after the full close, born versus freed bytes differ by 15 (22143 against 22128) in the effect-based close loop of the output owner. Not yet explained (effect machinery allocation not released inside the observed window, or a real leak in the close).
3. `extension_bundle_resource_retirement_runs_through_suspend_and_exported_poll`: with the corrected retained input the kernel turn now stops at `UI arena has no accepting lifetime registration`; the fixture context needs the UI arena lifetime registration the old fixture step context provided. The panic during unwind then aborts the process (`extension bundle requires terminal-empty retirement before drop`); running this test last or alone avoids losing the others.
4. `package_descriptor_lists_its_io_mechanism_rows_with_their_native_side` (describe test, another agent's files, not touched).

The `extends_*_panics` and `refuses_implicit_live_drop` laws print their expected panics and pass.

## Follow-up 3: the 15-byte gap and the suspend/poll law

### (2) The 15-byte alloc/free gap in `context_failure` is accounted machinery, not a leak

Bisected with an idle `OriginalFixtureCaller` (no effects run) measured in four windows (birth, context lend, context drop, retire): birth requests 21511 and releases 680, retire requests 632 and releases 21448; lend and drop are (0, 0). The 15 bytes are `CallerPolicy.source_kind`, the String `"original-parent"` (exactly 15 bytes), allocated in `admit` and freed when the caller struct itself drops, after `retire`. The law now also observes `drop(caller)` and balances birth, retire and drop together (`🧩️extension/📨️invoke/⚠️failure/🧪️tests/🦀️.rs`). My effect-based close was not involved (the gap is identical with and without an effect).

The turn-level `born == freed` of the output close balances exactly, for every copy grant.

A second symptom in the same law, a 64-byte allocation in one advance step that its receipt does not carry (turn 380, phase 0, copy 1), is contamination from another session's temporary `eprintln!("[DEBUG] symbols ...")` in the Pack borrowed cursor (`🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🦀️.rs`): the print allocates inside the allocation-observed window (the full-crate run then shows `(304, 112)` against `(0, 0)` in five unit-1 laws). To re-verify once that print is removed, together with blocker (1).

### (3) Suspend/exported-poll law: original assertion kept, passes

Contract check, not a redefinition: `admit_turn_entry` (`⚛️reactor/🔄️turn/🪜️entry-lifecycle-and-retirement`) only requires that the retained input names the context's own operation and generation and that the context's held grant lies inside the input grant; the turn steps treat a zero-item grant as "nothing ran, retry on the next funded turn" (`grant.maximum_items == 0` returns `Ok(None)` in the extension-invocation and event-dispatch lanes). So an unfunded turn suspends, it is not refused. The test now builds a correctly formed unfunded context: the parent-bearing fixture context narrowed to the zero-item grant through `with_retained_loan`, polling inside it (`resolve_ready`); the original assertion (the poll succeeds, demands unchanged) is kept. Other fixes in that law: the UI arena lifetime registration through `FixtureUiArena::admit()` (`semio_plugin_testing_laws`, the fixture every renderer law uses), and the retained turn input built from the context's own operation, generation and grant in `poll_extension_response` and the other budgets (`retained_turn(cx)`).

Result: `extension_bundle_resource_retirement_runs_through_suspend_and_exported_poll` passes.

### Found on the full run (unit 5, `🧩️extension/🧪️tests/📨️invoke`)

- `original_extension_admit_keeps_whole_failure_pair_and_paid_output_close`: now passes (effect-driven close through `OriginalFixtureCaller`, grant sized to the quoted minimum).
- `original_extension_resource_close_preserves_three_causes_same_caller_and_single_debit`: the ladder loan is now sized from the bundle's own quote; it then fails at the last step: `drop(bundle)` still releases 64 bytes after `terminal_is_empty()` (law expects `(0, 0)`). Open, not yet attributed (which allocation of `ExtensionBundle` the close ladder does not pay for).

## Follow-up 4: the 64 bytes after terminal-empty, and the final tally

### Root cause: a lazily boxed platform mutex outside any receipt (product fix)

Instrumented the drop of `ExtensionBundle` field by field (each `ManuallyDrop` field dropped in its own allocation window): `manifest`, `handlers`, `resource_owner` and `metadata_retirement` release (0, 0); `inference_gateway` releases (0, 64). The field was `std::sync::Mutex<Option<ArtifactInferenceGateway>>`, and `terminal_is_empty` and the close ladder lock it: on this platform the std mutex is boxed lazily (64 bytes) at the first lock, outside any receipt, and freed only when the bundle drops, after the ladder declared itself terminal-empty.

Contract: `RetainedOwnerGate` (`🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️gate`) is documented as "one non-allocating retained-owner gate shared by every layer that must not box a lock lazily"; the UI arena registry states the same rule for its own lock. A terminal-empty ladder with a hidden platform allocation violates it, so the ladder should not owe those bytes (not an intentionally drop-owned shell).

Fix: `ExtensionBundle.inference_gateway` is now `RetainedOwnerGate<Option<ArtifactInferenceGateway>>` (`🧩️extension/🛬️lifted/🏃️runtime/🦀️.rs`, `🧩️extension/🚪️retirement/🦀️.rs`; dependency `semio-framework-value-control-contract` added to the extension crate). Contention is refused without parking or allocating: `artifact_infer` returns an empty step (retry), `terminal_is_empty` reads not-empty, `retirement_demands` answers an explicit busy refusal. The law `original_extension_resource_close_preserves_three_causes_same_caller_and_single_debit` keeps its original `(0, 0)` assertion for `drop(bundle)` and passes.

### Final tally

`semio-plugin-tests-06-2`, whole crate, single-threaded through the gate (Pack debug print gone): 25 passed, 2 failed of 27.

- `original_inference_gateway_failure_all_owned_causes`: external blocker (1), unchanged: `intrinsic Pack source changed its declared value authority` from the borrowed Pack cursor, whose files were edited 7 minutes before the run (not quiet 15 minutes yet). Retry once quiet; fix at the root if it still fails.
- `package_descriptor_lists_its_io_mechanism_rows_with_their_native_side`: describe test in another agent's files, not touched.

All other laws pass, including `context_failure` (accounted 15-byte caller policy string; the 64-byte unreceipted allocation was the other session's debug print) and `suspend_and_exported_poll`.

## Follow-up 5: final tally 27 of 27, and one consistent `retainedProgress` shape

`semio-plugin-tests-06-2`, whole crate, single-threaded through the gate: **27 passed, 0 failed**.

- `original_inference_gateway_failure_all_owned_causes`: the `Execution` cause carries a `domain` on the wire (`{code, message, retainedProgress, cause, terminal}`, emitted by `extras()` whenever `domain_loan(cause)` exists); the law now derives that expectation from the cause, like `source` (`🧩️extension/📨️invoke/⚠️failure/🧪️tests/🦀️.rs`).
- The describe test of unit 2 passes again (its owner fixed it).

### Naming of `retainedProgress` (schema-first)

The wire carried the long names at top level (`copiedItems`, `copiedBytes`, `retainedCapacityBytes`, `releasedBytes`) and short names inside the inference `domain` (`items`, `copyBytes`, `capacityBytes`, `releaseBytes`). Both are the same type (`RetainedCloneProgress`). The job failure schema (`🔨️modules/🧵️job/⚠️failure/🧬️schema/📦️domain.json`, `semio.job.original-domain-failure`) defines the domain's `retainedProgress` with the long names, and no schema declares the short form for it. Fixed in the product: `ArtifactInferenceExecutionError`'s domain projection keys (`🧩️declaration/🛬️lifted/💡️inference/🦀️.rs`, `projection_key` of `[2]`) are now the long names; the law expects them. No other reader or fixture consumed the short names (searched rust, json, ts).

Left as is, on purpose: `🌊️flow/🌐️geometry/🦀️.rs` also uses `items`, `copyBytes`, `capacityBytes`, `releaseBytes`, but under `work.retained`: that is a different type (a retained grant of the work step, not a progress receipt), so the short grant naming is correct there.
