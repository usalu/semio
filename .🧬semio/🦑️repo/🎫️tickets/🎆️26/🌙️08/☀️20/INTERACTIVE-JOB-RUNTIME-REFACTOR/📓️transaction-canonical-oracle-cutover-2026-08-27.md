# Transaction Canonical Oracle Cutover

The entire transaction module is mounted only under `cfg(test)` in UI runtime glue. This packet does not add a production transaction route. Original native R76 executed all 119 tests and found six unchanged transaction assertions failing; those assertions and semantic inputs remain unchanged.

The old transaction path drove `SurfaceReconcileCursor` directly and committed its unsealed candidate. The authored replacement reserves the existing paired reconciliation authority, aliases the same canonical document root, copies only id→ordinal/key→id metadata, and drives the real `SurfaceReconcileJob`. Completion uses the granted in-place `take_ready_into` with structural current/output targets on a separate turn. The exact old runtime root stays published until transaction commit.

The test-owned `TransactionPatch` keeps `UiPendingPatch` and `SurfaceReconcilePublishedPatch` together. Read/serde oracles borrow the exact payload; cleanup drains payload descendants before releasing paired publication metadata. Active jobs, candidate roots, superseded outputs, and registered surface roots explicitly use the existing typed close paths during cold test cleanup. There is no general SnapshotClone, second payload map, or production compatibility API.

These cold test helpers allocate/copy metadata and may drain multiple bounded close steps synchronously. They are not interactive admission, callback latency, or live publication evidence. Existing neutral canonical-document/ownership fixtures and independent Node Buffer/Ajv oracles retain the same-root/paired ownership contract. The six Rust semantic cases remain decisive, with full exhaustive execution required after the focused gate.

R77 focused transaction gate completed: 16 passed, 103 skipped, 0 failed, 0.315s, exit 0. All six formerly failing semantic cases passed with their original assertions unchanged. This is test-oracle coverage only. Raw output: `🧪️member-runtime-transaction-r77-native-2026-08-27.txt`.

## Actual R77 Output

Follow-up full R78 executed all 119 tests: 119 passed, 0 failed, 0 skipped, 1.874s, exit 0. Complete output is preserved in `📓️runtime-full-exhaustive-r78-native-2026-08-27.md`; the original R76 failure report remains unchanged.

```text
> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib transaction:: --no-fail-fast -- --nocapture

> bun ./📜️script.ts test exhaustive --lib transaction:: --no-fail-fast -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=40
────────────
 Nextest run ID 0849b389-13ea-4071-ae85-93ea3369f592 with nextest profile: exhaustive
    Starting 16 tests across 1 binary (103 tests skipped)
       START [         ] ( 1/16) semio-framework-ui-runtime transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch

running 1 test
test transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.021s] ( 1/16) semio-framework-ui-runtime transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch
       START [         ] ( 2/16) semio-framework-ui-runtime transaction::tests::a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction

running 1 test
test transaction::tests::a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.020s] ( 2/16) semio-framework-ui-runtime transaction::tests::a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction
       START [         ] ( 3/16) semio-framework-ui-runtime transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command

running 1 test
test transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.018s] ( 3/16) semio-framework-ui-runtime transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command
       START [         ] ( 4/16) semio-framework-ui-runtime transaction::tests::an_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics

running 1 test
test transaction::tests::an_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.014s] ( 4/16) semio-framework-ui-runtime transaction::tests::an_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics
       START [         ] ( 5/16) semio-framework-ui-runtime transaction::tests::an_entity_notified_but_not_read_by_any_surface_produces_no_patch

running 1 test
test transaction::tests::an_entity_notified_but_not_read_by_any_surface_produces_no_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.016s] ( 5/16) semio-framework-ui-runtime transaction::tests::an_entity_notified_but_not_read_by_any_surface_produces_no_patch
       START [         ] ( 6/16) semio-framework-ui-runtime transaction::tests::an_expired_wall_clock_budget_returns_before_consuming_input

running 1 test
test transaction::tests::an_expired_wall_clock_budget_returns_before_consuming_input ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.015s] ( 6/16) semio-framework-ui-runtime transaction::tests::an_expired_wall_clock_budget_returns_before_consuming_input
       START [         ] ( 7/16) semio-framework-ui-runtime transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch

running 1 test
test transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.015s] ( 7/16) semio-framework-ui-runtime transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch
       START [         ] ( 8/16) semio-framework-ui-runtime transaction::tests::cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision

running 1 test
test transaction::tests::cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.018s] ( 8/16) semio-framework-ui-runtime transaction::tests::cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision
       START [         ] ( 9/16) semio-framework-ui-runtime transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order

running 1 test
test transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.020s] ( 9/16) semio-framework-ui-runtime transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order
       START [         ] (10/16) semio-framework-ui-runtime transaction::tests::hard_credits_fault_before_any_candidate_snapshot_is_published

running 1 test
test transaction::tests::hard_credits_fault_before_any_candidate_snapshot_is_published ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.016s] (10/16) semio-framework-ui-runtime transaction::tests::hard_credits_fault_before_any_candidate_snapshot_is_published
       START [         ] (11/16) semio-framework-ui-runtime transaction::tests::next_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending

running 1 test
test transaction::tests::next_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.017s] (11/16) semio-framework-ui-runtime transaction::tests::next_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending
       START [         ] (12/16) semio-framework-ui-runtime transaction::tests::one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output

running 1 test
test transaction::tests::one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.023s] (12/16) semio-framework-ui-runtime transaction::tests::one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output
       START [         ] (13/16) semio-framework-ui-runtime transaction::tests::presence_flushes_on_its_own_channel_and_never_appears_in_a_patch

running 1 test
test transaction::tests::presence_flushes_on_its_own_channel_and_never_appears_in_a_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.020s] (13/16) semio-framework-ui-runtime transaction::tests::presence_flushes_on_its_own_channel_and_never_appears_in_a_patch
       START [         ] (14/16) semio-framework-ui-runtime transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command

running 1 test
test transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.031s] (14/16) semio-framework-ui-runtime transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command
       START [         ] (15/16) semio-framework-ui-runtime transaction::tests::the_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget

running 1 test
test transaction::tests::the_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.024s] (15/16) semio-framework-ui-runtime transaction::tests::the_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget
       START [         ] (16/16) semio-framework-ui-runtime transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other

running 1 test
test transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.023s] (16/16) semio-framework-ui-runtime transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other
────────────
     Summary [   0.315s] 16 tests run: 16 passed, 103 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-RXPl2R



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs
```
