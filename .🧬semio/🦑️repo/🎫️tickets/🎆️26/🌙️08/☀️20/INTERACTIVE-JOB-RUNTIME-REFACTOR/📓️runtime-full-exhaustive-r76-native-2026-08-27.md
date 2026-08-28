# Runtime Full Exhaustive R76

Actual canonical `@semio-tech/ui-runtime-rs:test --args='exhaustive --lib --no-fail-fast -- --nocapture'`, unchanged shared target/profile, SEMIO_COVERAGE=0. Exit 1: all 119 tests executed, 113 passed, 6 failed, zero skipped, 4.889 seconds. The runner flag repair prevented fail-fast omission without excluding or weakening any test.

All six failures are in the cfg(test) standalone transaction oracle: bulk projection coalescing; stale revision; intent-to-patch; deterministic surface order; repeated superseding input; independent surfaces. The oracle still drives the raw cursor and commits an unsealed candidate, instead of the canonical admitted Job/paired output path. This source observation explains a likely join gap but is not a new passing oracle or live-runtime claim. The original inline census and canonical root/read pressure cases passed in this actual full run. Full physical accounting and live PatchTracker output/producer admission remain open.

Raw output: `🧪️member-runtime-full-exhaustive-r76-native-2026-08-27.txt`.

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib --no-fail-fast -- --nocapture

> bun ./📜️script.ts test exhaustive --lib --no-fail-fast -- --nocapture

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
 Nextest run ID 95fb3ef4-96e4-4129-9053-683c7d333b68 with nextest profile: exhaustive
    Starting 119 tests across 1 binary
       START [         ] (  1/119) semio-framework-ui-runtime dispatch::tests::a_zero_tolerance_makes_any_trailing_revision_stale

running 1 test
test dispatch::tests::a_zero_tolerance_makes_any_trailing_revision_stale ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.016s] (  1/119) semio-framework-ui-runtime dispatch::tests::a_zero_tolerance_makes_any_trailing_revision_stale
       START [         ] (  2/119) semio-framework-ui-runtime dispatch::tests::an_intent_at_or_ahead_of_the_current_revision_is_never_stale

running 1 test
test dispatch::tests::an_intent_at_or_ahead_of_the_current_revision_is_never_stale ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.016s] (  2/119) semio-framework-ui-runtime dispatch::tests::an_intent_at_or_ahead_of_the_current_revision_is_never_stale
       START [         ] (  3/119) semio-framework-ui-runtime dispatch::tests::an_intent_exactly_at_the_tolerance_is_not_yet_stale

running 1 test
test dispatch::tests::an_intent_exactly_at_the_tolerance_is_not_yet_stale ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.017s] (  3/119) semio-framework-ui-runtime dispatch::tests::an_intent_exactly_at_the_tolerance_is_not_yet_stale
       START [         ] (  4/119) semio-framework-ui-runtime dispatch::tests::an_intent_trailing_by_more_than_the_tolerance_is_stale

running 1 test
test dispatch::tests::an_intent_trailing_by_more_than_the_tolerance_is_stale ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.013s] (  4/119) semio-framework-ui-runtime dispatch::tests::an_intent_trailing_by_more_than_the_tolerance_is_stale
       START [         ] (  5/119) semio-framework-ui-runtime entity::tests::defer_effects_queue_rather_than_run_inline

running 1 test
test entity::tests::defer_effects_queue_rather_than_run_inline ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.020s] (  5/119) semio-framework-ui-runtime entity::tests::defer_effects_queue_rather_than_run_inline
       START [         ] (  6/119) semio-framework-ui-runtime entity::tests::dropped_subscription_stops_delivering

running 1 test
test entity::tests::dropped_subscription_stops_delivering ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.017s] (  6/119) semio-framework-ui-runtime entity::tests::dropped_subscription_stops_delivering
       START [         ] (  7/119) semio-framework-ui-runtime entity::tests::effects_queue_rather_than_run_inline

running 1 test
test entity::tests::effects_queue_rather_than_run_inline ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.018s] (  7/119) semio-framework-ui-runtime entity::tests::effects_queue_rather_than_run_inline
       START [         ] (  8/119) semio-framework-ui-runtime entity::tests::nested_lease_is_rejected_not_aliased

running 1 test

thread 'entity::tests::nested_lease_is_rejected_not_aliased' (7605273) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️entity.rs:304:17:
🚫️ nested update/read of a leased entity — rejected, not aliased
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::entity::EntityStore>::take_for_lease::<i32>
   3: semio_framework_ui_runtime::entity::tests::nested_lease_is_rejected_not_aliased::{closure#0}
   4: <semio_framework_ui_runtime::entity::tests::nested_lease_is_rejected_not_aliased::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::entity::tests::nested_lease_is_rejected_not_aliased::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::entity::tests::nested_lease_is_rejected_not_aliased::{closure#0}>, alloc::boxed::Box<i32>>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::entity::tests::nested_lease_is_rejected_not_aliased::{closure#0}>, alloc::boxed::Box<i32>>
   9: semio_framework_ui_runtime::entity::tests::nested_lease_is_rejected_not_aliased
  10: semio_framework_ui_runtime::entity::tests::nested_lease_is_rejected_not_aliased::{closure#0}
  11: <semio_framework_ui_runtime::entity::tests::nested_lease_is_rejected_not_aliased::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test entity::tests::nested_lease_is_rejected_not_aliased ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.025s] (  8/119) semio-framework-ui-runtime entity::tests::nested_lease_is_rejected_not_aliased
       START [         ] (  9/119) semio-framework-ui-runtime entity::tests::read_during_lease_is_rejected

running 1 test
test entity::tests::read_during_lease_is_rejected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.017s] (  9/119) semio-framework-ui-runtime entity::tests::read_during_lease_is_rejected
       START [         ] ( 10/119) semio-framework-ui-runtime entity::tests::release_is_queued_until_flush_releases

running 1 test
test entity::tests::release_is_queued_until_flush_releases ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.018s] ( 10/119) semio-framework-ui-runtime entity::tests::release_is_queued_until_flush_releases
       START [         ] ( 11/119) semio-framework-ui-runtime entity::tests::spawn_local_queues_future_for_the_embedder

running 1 test
test entity::tests::spawn_local_queues_future_for_the_embedder ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.022s] ( 11/119) semio-framework-ui-runtime entity::tests::spawn_local_queues_future_for_the_embedder
       START [         ] ( 12/119) semio-framework-ui-runtime entity::tests::stale_entity_id_never_resolves_to_new_occupant

running 1 test
test entity::tests::stale_entity_id_never_resolves_to_new_occupant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.018s] ( 12/119) semio-framework-ui-runtime entity::tests::stale_entity_id_never_resolves_to_new_occupant
       START [         ] ( 13/119) semio-framework-ui-runtime entity::tests::value_restored_after_panicking_closure

running 1 test

thread 'entity::tests::value_restored_after_panicking_closure' (7605294) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️entity.rs:402:39:
boom
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure::{closure#0}::{closure#0}
   3: <semio_framework_ui_runtime::entity::EntityStore>::update::<i32, (), semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure::{closure#0}::{closure#0}>
   4: semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure::{closure#0}
   5: <semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   6: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   7: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure::{closure#0}>, ()>
   8: ___rust_try
   9: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure::{closure#0}>, ()>
  10: semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure
  11: semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure::{closure#0}
  12: <semio_framework_ui_runtime::entity::tests::value_restored_after_panicking_closure::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test entity::tests::value_restored_after_panicking_closure ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.021s] ( 13/119) semio-framework-ui-runtime entity::tests::value_restored_after_panicking_closure
       START [         ] ( 14/119) semio-framework-ui-runtime entity::tests::weak_entity_upgrade_fails_after_last_strong_drops

running 1 test
test entity::tests::weak_entity_upgrade_fails_after_last_strong_drops ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.020s] ( 14/119) semio-framework-ui-runtime entity::tests::weak_entity_upgrade_fails_after_last_strong_drops
       START [         ] ( 15/119) semio-framework-ui-runtime gateway::tests::full_backing_sink_returns_full_synchronously_without_dropping_the_command

running 1 test
test gateway::tests::full_backing_sink_returns_full_synchronously_without_dropping_the_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.020s] ( 15/119) semio-framework-ui-runtime gateway::tests::full_backing_sink_returns_full_synchronously_without_dropping_the_command
       START [         ] ( 16/119) semio-framework-ui-runtime gateway::tests::full_local_capacity_returns_full_synchronously_without_dropping_the_command

running 1 test
test gateway::tests::full_local_capacity_returns_full_synchronously_without_dropping_the_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.018s] ( 16/119) semio-framework-ui-runtime gateway::tests::full_local_capacity_returns_full_synchronously_without_dropping_the_command
       START [         ] ( 17/119) semio-framework-ui-runtime gateway::tests::resolving_a_ticket_frees_capacity_for_a_new_submission

running 1 test
test gateway::tests::resolving_a_ticket_frees_capacity_for_a_new_submission ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.019s] ( 17/119) semio-framework-ui-runtime gateway::tests::resolving_a_ticket_frees_capacity_for_a_new_submission
       START [         ] ( 18/119) semio-framework-ui-runtime gateway::tests::ticket_round_trips_to_acknowledged_and_to_rejected

running 1 test
test gateway::tests::ticket_round_trips_to_acknowledged_and_to_rejected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.019s] ( 18/119) semio-framework-ui-runtime gateway::tests::ticket_round_trips_to_acknowledged_and_to_rejected
       START [         ] ( 19/119) semio-framework-ui-runtime inbox::tests::drain_into_on_an_empty_inbox_is_a_no_op

running 1 test
test inbox::tests::drain_into_on_an_empty_inbox_is_a_no_op ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.019s] ( 19/119) semio-framework-ui-runtime inbox::tests::drain_into_on_an_empty_inbox_is_a_no_op
       START [         ] ( 20/119) semio-framework-ui-runtime inbox::tests::drain_into_respects_limit_and_leaves_the_remainder_queued

running 1 test
test inbox::tests::drain_into_respects_limit_and_leaves_the_remainder_queued ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.021s] ( 20/119) semio-framework-ui-runtime inbox::tests::drain_into_respects_limit_and_leaves_the_remainder_queued
       START [         ] ( 21/119) semio-framework-ui-runtime inbox::tests::push_beyond_capacity_returns_overflow_without_dropping_existing_entries

running 1 test
test inbox::tests::push_beyond_capacity_returns_overflow_without_dropping_existing_entries ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.017s] ( 21/119) semio-framework-ui-runtime inbox::tests::push_beyond_capacity_returns_overflow_without_dropping_existing_entries
       START [         ] ( 22/119) semio-framework-ui-runtime inbox::tests::same_key_pushes_coalesce_to_the_newest_value

running 1 test
test inbox::tests::same_key_pushes_coalesce_to_the_newest_value ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.020s] ( 22/119) semio-framework-ui-runtime inbox::tests::same_key_pushes_coalesce_to_the_newest_value
       START [         ] ( 23/119) semio-framework-ui-runtime presence::tests::a_burst_of_same_key_own_presence_writes_coalesces_to_the_newest_value

running 1 test
test presence::tests::a_burst_of_same_key_own_presence_writes_coalesces_to_the_newest_value ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.020s] ( 23/119) semio-framework-ui-runtime presence::tests::a_burst_of_same_key_own_presence_writes_coalesces_to_the_newest_value
       START [         ] ( 24/119) semio-framework-ui-runtime presence::tests::a_burst_of_same_key_peer_writes_coalesces_to_one_update

running 1 test
test presence::tests::a_burst_of_same_key_peer_writes_coalesces_to_one_update ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.017s] ( 24/119) semio-framework-ui-runtime presence::tests::a_burst_of_same_key_peer_writes_coalesces_to_one_update
       START [         ] ( 25/119) semio-framework-ui-runtime presence::tests::distinct_peers_on_one_key_are_all_reported_and_expire_independently

running 1 test
test presence::tests::distinct_peers_on_one_key_are_all_reported_and_expire_independently ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.025s] ( 25/119) semio-framework-ui-runtime presence::tests::distinct_peers_on_one_key_are_all_reported_and_expire_independently
       START [         ] ( 26/119) semio-framework-ui-runtime presence::tests::own_presence_never_expires

running 1 test
test presence::tests::own_presence_never_expires ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.025s] ( 26/119) semio-framework-ui-runtime presence::tests::own_presence_never_expires
       START [         ] ( 27/119) semio-framework-ui-runtime presence::tests::presence_entries_expire_exactly_at_their_ttl_and_a_flush_after_expiry_omits_them

running 1 test
test presence::tests::presence_entries_expire_exactly_at_their_ttl_and_a_flush_after_expiry_omits_them ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.025s] ( 27/119) semio-framework-ui-runtime presence::tests::presence_entries_expire_exactly_at_their_ttl_and_a_flush_after_expiry_omits_them
       START [         ] ( 28/119) semio-framework-ui-runtime present::tests::a_stateless_fn_item_satisfies_present_generically

running 1 test
test present::tests::a_stateless_fn_item_satisfies_present_generically ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.020s] ( 28/119) semio-framework-ui-runtime present::tests::a_stateless_fn_item_satisfies_present_generically
       START [         ] ( 29/119) semio-framework-ui-runtime present::tests::deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close

running 1 test
test present::tests::deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.029s] ( 29/119) semio-framework-ui-runtime present::tests::deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close
       START [         ] ( 30/119) semio-framework-ui-runtime present::tests::duplicate_stale_cancel_and_deadline_fault_before_publication

running 1 test
test present::tests::duplicate_stale_cancel_and_deadline_fault_before_publication ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.023s] ( 30/119) semio-framework-ui-runtime present::tests::duplicate_stale_cancel_and_deadline_fault_before_publication
       START [         ] ( 31/119) semio-framework-ui-runtime present::tests::mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate

running 1 test
test present::tests::mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.022s] ( 31/119) semio-framework-ui-runtime present::tests::mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate
       START [         ] ( 32/119) semio-framework-ui-runtime reconcile::handback_entry_tests::retained_handback_maintenance_entry_does_not_wait_for_registry

running 1 test
test reconcile::handback_entry_tests::retained_handback_maintenance_entry_does_not_wait_for_registry ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.026s] ( 32/119) semio-framework-ui-runtime reconcile::handback_entry_tests::retained_handback_maintenance_entry_does_not_wait_for_registry
       START [         ] ( 33/119) semio-framework-ui-runtime reconcile::handback_entry_tests::retained_handback_poison_is_fault_without_mutating_queued_owner

running 1 test

thread '<unnamed>' (7605383) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../🚪️handback/🧪️component.rs:66:102:
fixture registry poison
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::handback_entry_tests::retained_handback_poison_is_fault_without_mutating_queued_owner::{closure#0}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test reconcile::handback_entry_tests::retained_handback_poison_is_fault_without_mutating_queued_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.024s] ( 33/119) semio-framework-ui-runtime reconcile::handback_entry_tests::retained_handback_poison_is_fault_without_mutating_queued_owner
       START [         ] ( 34/119) semio-framework-ui-runtime reconcile::handback_entry_tests::retained_handback_take_entry_does_not_wait_for_registry

running 1 test
test reconcile::handback_entry_tests::retained_handback_take_entry_does_not_wait_for_registry ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.027s] ( 34/119) semio-framework-ui-runtime reconcile::handback_entry_tests::retained_handback_take_entry_does_not_wait_for_registry
       START [         ] ( 35/119) semio-framework-ui-runtime reconcile::output::tests::surface_output_pool_contended_drop_preserves_reserved_entry_until_exact_drain

running 1 test
[DEBUG] output-pool held-mutex-drop-waits=false exact-return-drained=true
test reconcile::output::tests::surface_output_pool_contended_drop_preserves_reserved_entry_until_exact_drain ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.025s] ( 35/119) semio-framework-ui-runtime reconcile::output::tests::surface_output_pool_contended_drop_preserves_reserved_entry_until_exact_drain
       START [         ] ( 36/119) semio-framework-ui-runtime reconcile::output::tests::surface_output_pool_defers_reuse_and_rejects_stale_epoch_after_final_return

running 1 test
[DEBUG] output-pool reuse-before-drain=false exact-epoch=2 explicit-close-no-second-return=true
test reconcile::output::tests::surface_output_pool_defers_reuse_and_rejects_stale_epoch_after_final_return ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.025s] ( 36/119) semio-framework-ui-runtime reconcile::output::tests::surface_output_pool_defers_reuse_and_rejects_stale_epoch_after_final_return
       START [         ] ( 37/119) semio-framework-ui-runtime reconcile::output::tests::surface_output_pool_zero_grant_and_busy_registry_leave_authority_unchanged

running 1 test
[DEBUG] output-pool busy-refusal-exact=true zero-grant-mutates=false static-bytes=125088
test reconcile::output::tests::surface_output_pool_zero_grant_and_busy_registry_leave_authority_unchanged ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.029s] ( 37/119) semio-framework-ui-runtime reconcile::output::tests::surface_output_pool_zero_grant_and_busy_registry_leave_authority_unchanged
       START [         ] ( 38/119) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_close_respects_all_grants_and_contended_exact_credit

running 1 test
[DEBUG] patch-close grants=1,64,4096 exact-credit-contention=true exact-handback-contention=true
test reconcile::patch_handoff_tests::retained_patch_handoff_close_respects_all_grants_and_contended_exact_credit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.020s] ( 38/119) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_close_respects_all_grants_and_contended_exact_credit
       START [         ] ( 39/119) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment

running 1 test
[DEBUG] patch-handoff exact-slots=true occupied-target-preserved=true invalid-ack-preserved=true surface-bytes=4
test reconcile::patch_handoff_tests::retained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.019s] ( 39/119) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment
       START [         ] ( 40/119) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority

running 1 test

thread 'reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority' (7605416) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../🩹️patch/🧪️component.rs:100:13:
fixture callback failure with all source and output slots retained
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}
   3: <semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}>, ()>
   8: semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority
   9: semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}
  10: <semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority' (7605416) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../🩹️patch/🧪️component.rs:100:13:
fixture callback failure with all source and output slots retained
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}
   3: <semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}>, ()>
   8: semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority
   9: semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}
  10: <semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority' (7605416) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../🩹️patch/🧪️component.rs:100:13:
fixture callback failure with all source and output slots retained
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}
   3: <semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}>, ()>
   8: semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority
   9: semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0}
  10: <semio_framework_ui_runtime::reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] patch-handoff unwind-frontiers=3 exact-payload-pointer=true exact-authority-count=1 publish-bytes=3514 ack-bytes=3680
test reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.026s] ( 40/119) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority
       START [         ] ( 41/119) semio-framework-ui-runtime reconcile::tests::abandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged

running 1 test
test reconcile::tests::abandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.028s] ( 41/119) semio-framework-ui-runtime reconcile::tests::abandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged
       START [         ] ( 42/119) semio-framework-ui-runtime reconcile::tests::allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation

running 1 test
test reconcile::tests::allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.019s] ( 42/119) semio-framework-ui-runtime reconcile::tests::allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation
       START [         ] ( 43/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant

running 1 test
[DEBUG] parent-child-grants compare-final=4096 lease-close=4096 comparison-owner=2256 source-return=3096 candidate-physical=6416 separate-turns=true
test reconcile::tests::canonical_document_tests::surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.025s] ( 43/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant
       START [         ] ( 44/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind

running 1 test

thread 'reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind' (7605435) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1560:51:
injected existing component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}
   4: <semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind
  10: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] existing-pair-unwind phase=comparison exact-current-unchanged=true retained-close=true

thread 'reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind' (7605435) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1560:51:
injected existing component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}
   4: <semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind
  10: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] existing-pair-unwind phase=copy exact-current-unchanged=true retained-close=true

thread 'reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind' (7605435) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1560:51:
injected existing component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}
   4: <semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind
  10: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] existing-pair-unwind phase=source-returned exact-current-unchanged=true retained-close=true

thread 'reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind' (7605435) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1357:43:
injected retained component output callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}
   4: <semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#1}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind
  10: semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] existing-pair-unwind phase=candidate-returned exact-current-unchanged=true retained-close=true
test reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.04s

        PASS [   0.062s] ( 44/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind
       START [         ] ( 45/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_fresh_children_retain_completed_roots_for_a_separate_turn

running 1 test
[DEBUG] fresh-child-completion [("component", true), ("bindings", true)]
test reconcile::tests::canonical_document_tests::surface_canonical_document_fresh_children_retain_completed_roots_for_a_separate_turn ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.029s] ( 45/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_fresh_children_retain_completed_roots_for_a_separate_turn
       START [         ] ( 46/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_nine_live_reconcilers_share_the_original_root_with_readers

running 1 test
[DEBUG] canonical-reconcilers actual-surfaces=9 exact-root-readers=9 roots-after-owner-close=9 typed-reader-close=true
test reconcile::tests::canonical_document_tests::surface_canonical_document_nine_live_reconcilers_share_the_original_root_with_readers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.029s] ( 46/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_nine_live_reconcilers_share_the_original_root_with_readers
       START [         ] ( 47/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_old_reader_keeps_original_credit_during_replacement

running 1 test
[DEBUG] canonical-reader-replacement grant=1 original-root-unchanged=true original-credit-retained=true typed-terminal=true
[DEBUG] canonical-reader-replacement grant=64 original-root-unchanged=true original-credit-retained=true typed-terminal=true
[DEBUG] canonical-reader-replacement grant=4096 original-root-unchanged=true original-credit-retained=true typed-terminal=true
test reconcile::tests::canonical_document_tests::surface_canonical_document_old_reader_keeps_original_credit_during_replacement ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.028s] ( 47/119) semio-framework-ui-runtime reconcile::tests::canonical_document_tests::surface_canonical_document_old_reader_keeps_original_credit_during_replacement
       START [         ] ( 48/119) semio-framework-ui-runtime reconcile::tests::changed_component_with_unchanged_layout_emits_set_component_not_upsert

running 1 test
test reconcile::tests::changed_component_with_unchanged_layout_emits_set_component_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.024s] ( 48/119) semio-framework-ui-runtime reconcile::tests::changed_component_with_unchanged_layout_emits_set_component_not_upsert
       START [         ] ( 49/119) semio-framework-ui-runtime reconcile::tests::changing_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node

running 1 test
test reconcile::tests::changing_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.021s] ( 49/119) semio-framework-ui-runtime reconcile::tests::changing_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node
       START [         ] ( 50/119) semio-framework-ui-runtime reconcile::tests::changing_only_accessibility_emits_exactly_one_set_accessibility_not_upsert

running 1 test
test reconcile::tests::changing_only_accessibility_emits_exactly_one_set_accessibility_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.024s] ( 50/119) semio-framework-ui-runtime reconcile::tests::changing_only_accessibility_emits_exactly_one_set_accessibility_not_upsert
       START [         ] ( 51/119) semio-framework-ui-runtime reconcile::tests::changing_only_bindings_emits_exactly_one_set_bindings_not_upsert

running 1 test
test reconcile::tests::changing_only_bindings_emits_exactly_one_set_bindings_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.021s] ( 51/119) semio-framework-ui-runtime reconcile::tests::changing_only_bindings_emits_exactly_one_set_bindings_not_upsert
       START [         ] ( 52/119) semio-framework-ui-runtime reconcile::tests::changing_only_menu_emits_exactly_one_set_menu_not_upsert

running 1 test
test reconcile::tests::changing_only_menu_emits_exactly_one_set_menu_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.021s] ( 52/119) semio-framework-ui-runtime reconcile::tests::changing_only_menu_emits_exactly_one_set_menu_not_upsert
       START [         ] ( 53/119) semio-framework-ui-runtime reconcile::tests::changing_only_style_emits_exactly_one_set_style_not_upsert

running 1 test
test reconcile::tests::changing_only_style_emits_exactly_one_set_style_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.025s] ( 53/119) semio-framework-ui-runtime reconcile::tests::changing_only_style_emits_exactly_one_set_style_not_upsert
       START [         ] ( 54/119) semio-framework-ui-runtime reconcile::tests::changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops

running 1 test
test reconcile::tests::changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.023s] ( 54/119) semio-framework-ui-runtime reconcile::tests::changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops
       START [         ] ( 55/119) semio-framework-ui-runtime reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed

running 1 test

thread 'reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed' (7605490) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:244:9:
🚫️ duplicate sibling key "a" under parent UiNodeId(0) — reconciliation keys must be unique among siblings
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::assert_unique_child_keys
   3: <semio_framework_ui_runtime::reconcile::SurfaceReconcileOracle>::diff_children
   4: <semio_framework_ui_runtime::reconcile::SurfaceReconcileOracle>::diff_node
   5: <semio_framework_ui_runtime::reconcile::SurfaceReconcileOracle>::reconcile
   6: <semio_framework_ui_runtime::reconcile::SurfaceReconciler>::reconcile
   7: semio_framework_ui_runtime::reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed::{closure#0}
   8: <semio_framework_ui_runtime::reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   9: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
  10: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed::{closure#0}>, core::option::Option<semio_framework_ui_contract::document::UiPatch>>
  11: ___rust_try
  12: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed::{closure#0}>, core::option::Option<semio_framework_ui_contract::document::UiPatch>>
  13: semio_framework_ui_runtime::reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed
  14: semio_framework_ui_runtime::reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed::{closure#0}
  15: <semio_framework_ui_runtime::reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.026s] ( 55/119) semio-framework-ui-runtime reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed
       START [         ] ( 56/119) semio-framework-ui-runtime reconcile::tests::every_large_tree_cursor_slice_stays_below_eight_milliseconds

running 1 test
test reconcile::tests::every_large_tree_cursor_slice_stays_below_eight_milliseconds ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.030s] ( 56/119) semio-framework-ui-runtime reconcile::tests::every_large_tree_cursor_slice_stays_below_eight_milliseconds
       START [         ] ( 57/119) semio-framework-ui-runtime reconcile::tests::first_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent

running 1 test
test reconcile::tests::first_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.019s] ( 57/119) semio-framework-ui-runtime reconcile::tests::first_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent
       START [         ] ( 58/119) semio-framework-ui-runtime reconcile::tests::fixed_runtime_owners_keep_bounded_state_off_the_stack

running 1 test
[DEBUG] canonical-owner-layout reconciler=760 cursor=48552 retained=65024
test reconcile::tests::fixed_runtime_owners_keep_bounded_state_off_the_stack ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.018s] ( 58/119) semio-framework-ui-runtime reconcile::tests::fixed_runtime_owners_keep_bounded_state_off_the_stack
       START [         ] ( 59/119) semio-framework-ui-runtime reconcile::tests::identifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation

running 1 test
test reconcile::tests::identifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.018s] ( 59/119) semio-framework-ui-runtime reconcile::tests::identifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation
       START [         ] ( 60/119) semio-framework-ui-runtime reconcile::tests::ids_are_never_reused_after_removal

running 1 test
test reconcile::tests::ids_are_never_reused_after_removal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.019s] ( 60/119) semio-framework-ui-runtime reconcile::tests::ids_are_never_reused_after_removal
       START [         ] ( 61/119) semio-framework-ui-runtime reconcile::tests::inserting_a_middle_sibling_preserves_the_others_ids

running 1 test
test reconcile::tests::inserting_a_middle_sibling_preserves_the_others_ids ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.021s] ( 61/119) semio-framework-ui-runtime reconcile::tests::inserting_a_middle_sibling_preserves_the_others_ids
       START [         ] ( 62/119) semio-framework-ui-runtime reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal

running 1 test
[DEBUG] published-close owner-transitions=4 physical-turns=17 semantic-bytes=10 grant=1
test reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.030s] ( 62/119) semio-framework-ui-runtime reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal
       START [         ] ( 63/119) semio-framework-ui-runtime reconcile::tests::mark_rejected_then_reconcile_emits_a_full_resend

running 1 test
test reconcile::tests::mark_rejected_then_reconcile_emits_a_full_resend ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.037s] ( 63/119) semio-framework-ui-runtime reconcile::tests::mark_rejected_then_reconcile_emits_a_full_resend
       START [         ] ( 64/119) semio-framework-ui-runtime reconcile::tests::opaque_surface_document_uses_aggregate_credits_instead_of_scalar_page

running 1 test
test reconcile::tests::opaque_surface_document_uses_aggregate_credits_instead_of_scalar_page ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.037s] ( 64/119) semio-framework-ui-runtime reconcile::tests::opaque_surface_document_uses_aggregate_credits_instead_of_scalar_page
       START [         ] ( 65/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind

running 1 test

thread 'reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind' (7605534) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📤️output/🧪️component.rs:161:9:
[DEBUG] actual ready transfer callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}
   3: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}>, ()>
   8: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind
   9: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}
  10: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] ready-transfer bytes=8776 shell-preserved=true refused-payload-preserved=true unwind-targets-retained=true
test reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.02s

        PASS [   0.054s] ( 65/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind
       START [         ] ( 66/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer

running 1 test
[DEBUG] ready-revalidation actual=["fault", "fault", "pending", "pending"] exact-source-preserved=true
test reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.043s] ( 66/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer
       START [         ] ( 67/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free

running 1 test
[DEBUG] handback-admission one-free-accepted=false producer-invoked=false
test reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.028s] ( 67/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free
       START [         ] ( 68/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback

running 1 test
[DEBUG] handback-admission post-seal-transfer=true late-slot-acquisition=false
test reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.028s] ( 68/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback
       START [         ] ( 69/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_pool_keeps_exact_paired_ready_on_refusal_and_fifo_handoff

running 1 test
[DEBUG] output-pool fifo=2 exact-rejected-pointer=true paired-credit=true close-grants=1,64,4096
test reconcile::tests::output_pool_tests::surface_output_pool_keeps_exact_paired_ready_on_refusal_and_fifo_handoff ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.043s] ( 69/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_pool_keeps_exact_paired_ready_on_refusal_and_fifo_handoff
       START [         ] ( 70/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_pool_reserves_before_producer_and_refuses_the_sixty_fifth

running 1 test
[DEBUG] output-pool preproducer=64 extra=false entry-limit=64 independent-payload-quota=false
test reconcile::tests::output_pool_tests::surface_output_pool_reserves_before_producer_and_refuses_the_sixty_fifth ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.025s] ( 70/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_pool_reserves_before_producer_and_refuses_the_sixty_fifth
       START [         ] ( 71/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_pool_static_backing_joins_existing_ledger_once_without_a_root_slot

running 1 test
[DEBUG] output-pool static-ledger contract=390800 runtime=143568 total=534368 additional-root-slots=0 final-release-retains-static=true
test reconcile::tests::output_pool_tests::surface_output_pool_static_backing_joins_existing_ledger_once_without_a_root_slot ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.021s] ( 71/119) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_pool_static_backing_joins_existing_ledger_once_without_a_root_slot
       START [         ] ( 72/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_binding_clone_requires_bounded_backing_and_copy

running 1 test
[DEBUG] surface-binding-clone turns=135 allocated=79744 initialized=66304 maximum-allocation=2072 maximum-placement=2072
test reconcile::tests::ownership::surface_ownership_binding_clone_requires_bounded_backing_and_copy ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.035s] ( 72/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_binding_clone_requires_bounded_backing_and_copy
       START [         ] ( 73/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_binding_copy_cancel_keeps_all_original_and_partial_backings

running 1 test
[DEBUG] surface-binding-cancel frontier=0 retained=79744 terminal=true allocation-during-close=0
[DEBUG] surface-binding-cancel frontier=1 retained=79744 terminal=true allocation-during-close=0
[DEBUG] surface-binding-cancel frontier=2 retained=80128 terminal=true allocation-during-close=0
[DEBUG] surface-binding-cancel frontier=3 retained=80512 terminal=true allocation-during-close=0
[DEBUG] surface-binding-cancel frontier=4 retained=80896 terminal=true allocation-during-close=0
[DEBUG] surface-binding-cancel frontier=5 retained=82968 terminal=true allocation-during-close=0
[DEBUG] surface-binding-cancel frontier=12 retained=85808 terminal=true allocation-during-close=0
[DEBUG] surface-binding-cancel frontier=64 retained=117736 terminal=true allocation-during-close=0
[DEBUG] surface-binding-cancel frontier=100 retained=139840 terminal=true allocation-during-close=0
[DEBUG] surface-binding-cancel frontier=132 retained=159488 terminal=true allocation-during-close=0
test reconcile::tests::ownership::surface_ownership_binding_copy_cancel_keeps_all_original_and_partial_backings ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.02s

        PASS [   0.061s] ( 73/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_binding_copy_cancel_keeps_all_original_and_partial_backings
       START [         ] ( 74/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback

running 1 test

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (7605622) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1572:49:
injected binding callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-binding-unwind frontier=1 same-owner=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (7605622) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1572:49:
injected binding callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-binding-unwind frontier=2 same-owner=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (7605622) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1572:49:
injected binding callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-binding-unwind frontier=3 same-owner=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (7605622) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1572:49:
injected binding callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-binding-unwind frontier=4 same-owner=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (7605622) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1572:49:
injected binding callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-binding-unwind frontier=5 same-owner=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (7605622) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1572:49:
injected binding callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-binding-unwind frontier=6 same-owner=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (7605622) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1572:49:
injected binding callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-binding-unwind frontier=7 same-owner=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (7605622) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1572:49:
injected binding callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-binding-unwind frontier=8 same-owner=true terminal-close=true
test reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.05s

        PASS [   0.118s] ( 74/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback
       START [         ] ( 75/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_component_copy_charges_actual_surface_backing_before_publication

running 1 test
[DEBUG] surface-component-copy turns=18 reported=91805 ledger-allocation=32768 actual-allocation=32768
test reconcile::tests::ownership::surface_ownership_component_copy_charges_actual_surface_backing_before_publication ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.031s] ( 75/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_component_copy_charges_actual_surface_backing_before_publication
       START [         ] ( 76/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source

running 1 test

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (7605646) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1566:51:
injected component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-component-unwind frontier=0 retained-outside-callback=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (7605646) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1566:51:
injected component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-component-unwind frontier=1 retained-outside-callback=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (7605646) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1566:51:
injected component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-component-unwind frontier=2 retained-outside-callback=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (7605646) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1566:51:
injected component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-component-unwind frontier=3 retained-outside-callback=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (7605646) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1566:51:
injected component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-component-unwind frontier=4 retained-outside-callback=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (7605646) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1566:51:
injected component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-component-unwind frontier=5 retained-outside-callback=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (7605646) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1566:51:
injected component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-component-unwind frontier=8 retained-outside-callback=true terminal-close=true

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (7605646) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1566:51:
injected component callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_ui_runtime::reconcile::SurfaceReconcileCursor>::step
   3: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
   4: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   5: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   6: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   7: ___rust_try
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}>, semio_framework_ui_runtime::reconcile::SurfaceReconcileStep>
   9: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source
  10: semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0}
  11: <semio_framework_ui_runtime::reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] surface-component-unwind frontier=12 retained-outside-callback=true terminal-close=true
[DEBUG] surface-component-unwind frontier=16 retained-outside-callback=true terminal-close=true
[DEBUG] surface-component-unwind frontier=32 retained-outside-callback=true terminal-close=true
[DEBUG] surface-component-refusal actual-allocation=0 source-retained=true terminal=true
test reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.06s

        PASS [   0.141s] ( 76/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source
       START [         ] ( 77/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_existing_component_refuses_before_cloning_unadmitted_payload

running 1 test
[DEBUG] existing-component-refusal rejected=true allocation-before-admission=0 source-unchanged=true
test reconcile::tests::ownership::surface_ownership_existing_component_refuses_before_cloning_unadmitted_payload ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.037s] ( 77/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_existing_component_refuses_before_cloning_unadmitted_payload
       START [         ] ( 78/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_existing_component_retains_comparison_and_copy_between_turns

running 1 test
[DEBUG] existing-component-copy turns=42 allocation-ledger=32768 old-unchanged=true
test reconcile::tests::ownership::surface_ownership_existing_component_retains_comparison_and_copy_between_turns ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.04s

        PASS [   0.082s] ( 78/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_existing_component_retains_comparison_and_copy_between_turns
       START [         ] ( 79/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_finalize_transfers_exact_record_and_index_allocations

running 1 test
[DEBUG] surface-finalize-transfer exact-records=true exact-indexes=true replacement-bytes=0 closed=true
test reconcile::tests::ownership::surface_ownership_finalize_transfers_exact_record_and_index_allocations ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.094s] ( 79/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_finalize_transfers_exact_record_and_index_allocations
       START [         ] ( 80/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_inline_fields_do_not_allocate_a_second_owner

running 1 test
[DEBUG] surface-inline-footprint name="tree-item-icon" before=19368 after=19368 delta=0 items-before=14 items-after=15
[DEBUG] surface-inline-footprint name="reserved-binding" before=218280 after=218280 delta=0 items-before=12 items-after=14
test reconcile::tests::ownership::surface_ownership_inline_fields_do_not_allocate_a_second_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.083s] ( 80/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_inline_fields_do_not_allocate_a_second_owner
       START [         ] ( 81/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_native_backing_inventory_preserves_capacity

running 1 test
[DEBUG] surface-physical-owner owner=tree-node bytes=6456
[DEBUG] surface-physical-owner owner=record bytes=6416
[DEBUG] surface-physical-owner owner=patch-op bytes=6416
[DEBUG] surface-physical-owner owner=action-binding bytes=2072
[DEBUG] surface-physical-owner owner=row-action bytes=3104
[DEBUG] surface-physical-owner owner=flat-backing bytes=834560
[DEBUG] surface-physical-owner owner=retained-backing bytes=822272
[DEBUG] surface-physical-owner owner=key-index-backing bytes=69632
[DEBUG] surface-physical-owner owner=traversal-backing bytes=4096
[DEBUG] surface-physical-owner owner=postorder-backing bytes=2048
[DEBUG] surface-physical-owner owner=seen-backing bytes=68608
[DEBUG] surface-physical-owner owner=ids-backing bytes=2048
[DEBUG] surface-physical-owner owner=removal-backing bytes=3072
[DEBUG] surface-physical-owner owner=semantic-value-stack bytes=101888
[DEBUG] surface-physical-owner owner=tree-retirement-stack bytes=229432
[DEBUG] surface-physical-owner owner=hypothetical-inline-patch-op-backing bytes=7397648
[DEBUG] surface-physical-owner owner=patch-directory-backing bytes=27672
[DEBUG] surface-physical-owner owner=patch-first-payload-backing bytes=6416
[DEBUG] surface-physical-owner owner=generic-list-owner bytes=48
[DEBUG] surface-physical-owner owner=binding-copy-owner bytes=2752
[DEBUG] surface-physical-owner owner=pending-patch-owner bytes=7000
[DEBUG] surface-physical-owner owner=cursor bytes=48552
[DEBUG] surface-physical-owner owner=retained-job-allocation bytes=65024
[DEBUG] surface-physical-owner owner=reconciler bytes=760
test reconcile::tests::ownership::surface_ownership_native_backing_inventory_preserves_capacity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.03s

        PASS [   0.139s] ( 81/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_native_backing_inventory_preserves_capacity
       START [         ] ( 82/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_patch_backing_is_admitted_in_separate_turns

running 1 test
[DEBUG] surface-patch-allocation turns=2 largest=27672 operation-bytes=6416
test reconcile::tests::ownership::surface_ownership_patch_backing_is_admitted_in_separate_turns ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.049s] ( 82/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_patch_backing_is_admitted_in_separate_turns
       START [         ] ( 83/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_patch_refusal_and_cancel_keep_exact_unallocated_owner

running 1 test
[DEBUG] surface-patch-cancel stage=1 retained-before=0 terminal=true allocation-during-close=0
[DEBUG] surface-patch-cancel stage=2 retained-before=27672 terminal=true allocation-during-close=0
[DEBUG] surface-patch-cancel stage=3 retained-before=34088 terminal=true allocation-during-close=0
[DEBUG] surface-patch-cancel stage=4 retained-before=34088 terminal=true allocation-during-close=0
test reconcile::tests::ownership::surface_ownership_patch_refusal_and_cancel_keep_exact_unallocated_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.053s] ( 83/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_patch_refusal_and_cancel_keep_exact_unallocated_owner
       START [         ] ( 84/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_resident_reservation_uses_one_shared_aggregate_ledger

running 1 test
[DEBUG] runtime-resident-join expected-bytes=65536 observed-bytes=65536 expected-slots=1 observed-slots=1
test reconcile::tests::ownership::surface_ownership_resident_reservation_uses_one_shared_aggregate_ledger ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.027s] ( 84/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_resident_reservation_uses_one_shared_aggregate_ledger
       START [         ] ( 85/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_resident_return_maintenance_preserves_contended_credit

running 1 test
[DEBUG] runtime-resident-return mutex-busy-keeps-credit=true maintenance-resumes=true exact-return=65536
test reconcile::tests::ownership::surface_ownership_resident_return_maintenance_preserves_contended_credit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.042s] ( 85/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_resident_return_maintenance_preserves_contended_credit
       START [         ] ( 86/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_transfer_preserves_backing_without_allocating_replacement

running 1 test
[DEBUG] surface-backing-transfer source-bytes=0 moved-bytes=64
[DEBUG] surface-moved-source rejected-exact-payload=true payload-capacity=16 replacement-bytes=0
test reconcile::tests::ownership::surface_ownership_transfer_preserves_backing_without_allocating_replacement ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.027s] ( 86/119) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_transfer_preserves_backing_without_allocating_replacement
       START [         ] ( 87/119) semio-framework-ui-runtime reconcile::tests::persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement

running 1 test
test reconcile::tests::persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.025s] ( 87/119) semio-framework-ui-runtime reconcile::tests::persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement
       START [         ] ( 88/119) semio-framework-ui-runtime reconcile::tests::public_drop_handback_is_lossless_at_terminal_cap_and_plus_one

running 1 test
test reconcile::tests::public_drop_handback_is_lossless_at_terminal_cap_and_plus_one ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.59s

        PASS [   0.640s] ( 88/119) semio-framework-ui-runtime reconcile::tests::public_drop_handback_is_lossless_at_terminal_cap_and_plus_one
       START [         ] ( 89/119) semio-framework-ui-runtime reconcile::tests::removing_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained

running 1 test
test reconcile::tests::removing_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.045s] ( 89/119) semio-framework-ui-runtime reconcile::tests::removing_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained
       START [         ] ( 90/119) semio-framework-ui-runtime reconcile::tests::reordering_siblings_preserves_every_id_and_emits_only_set_children

running 1 test
test reconcile::tests::reordering_siblings_preserves_every_id_and_emits_only_set_children ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.081s] ( 90/119) semio-framework-ui-runtime reconcile::tests::reordering_siblings_preserves_every_id_and_emits_only_set_children
       START [         ] ( 91/119) semio-framework-ui-runtime reconcile::tests::resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics

running 1 test
test reconcile::tests::resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.034s] ( 91/119) semio-framework-ui-runtime reconcile::tests::resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics
       START [         ] ( 92/119) semio-framework-ui-runtime reconcile::tests::retained_map_page_advances_each_key_once_without_rewalking_prior_entries

running 1 test
test reconcile::tests::retained_map_page_advances_each_key_once_without_rewalking_prior_entries ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.084s] ( 92/119) semio-framework-ui-runtime reconcile::tests::retained_map_page_advances_each_key_once_without_rewalking_prior_entries
       START [         ] ( 93/119) semio-framework-ui-runtime reconcile::tests::round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot

running 1 test
test reconcile::tests::round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.02s

        PASS [   0.062s] ( 93/119) semio-framework-ui-runtime reconcile::tests::round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot
       START [         ] ( 94/119) semio-framework-ui-runtime reconcile::tests::semantic_aggregate_quota_faults_before_key_or_record_clone

running 1 test
test reconcile::tests::semantic_aggregate_quota_faults_before_key_or_record_clone ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.03s

        PASS [   0.149s] ( 94/119) semio-framework-ui-runtime reconcile::tests::semantic_aggregate_quota_faults_before_key_or_record_clone
       START [         ] ( 95/119) semio-framework-ui-runtime reconcile::tests::semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion

running 1 test
test reconcile::tests::semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.029s] ( 95/119) semio-framework-ui-runtime reconcile::tests::semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion
       START [         ] ( 96/119) semio-framework-ui-runtime reconcile::tests::semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged

running 1 test
test reconcile::tests::semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.090s] ( 96/119) semio-framework-ui-runtime reconcile::tests::semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged
       START [         ] ( 97/119) semio-framework-ui-runtime reconcile::tests::stale_cancel_and_drop_handoff_preserve_public_terminal_ownership

running 1 test
test reconcile::tests::stale_cancel_and_drop_handoff_preserve_public_terminal_ownership ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.045s] ( 97/119) semio-framework-ui-runtime reconcile::tests::stale_cancel_and_drop_handoff_preserve_public_terminal_ownership
       START [         ] ( 98/119) semio-framework-ui-runtime tracking::tests::finish_rejects_mismatched_surface

running 1 test

thread 'tracking::tests::finish_rejects_mismatched_surface' (7606190) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️tracking.rs:75:9:
assertion `left == right` failed: present scope nesting mismatch
  left: SurfaceId("a")
 right: SurfaceId("b")
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<semio_framework_ui_contract::document::SurfaceId, semio_framework_ui_contract::document::SurfaceId>
   4: <semio_framework_ui_runtime::tracking::DependencyTracker>::finish
   5: semio_framework_ui_runtime::tracking::tests::finish_rejects_mismatched_surface
   6: semio_framework_ui_runtime::tracking::tests::finish_rejects_mismatched_surface::{closure#0}
   7: <semio_framework_ui_runtime::tracking::tests::finish_rejects_mismatched_surface::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test tracking::tests::finish_rejects_mismatched_surface - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.02s

        PASS [   0.049s] ( 98/119) semio-framework-ui-runtime tracking::tests::finish_rejects_mismatched_surface
       START [         ] ( 99/119) semio-framework-ui-runtime tracking::tests::n_notifications_of_one_surface_coalesce_to_one_dirty_mark

running 1 test
test tracking::tests::n_notifications_of_one_surface_coalesce_to_one_dirty_mark ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.032s] ( 99/119) semio-framework-ui-runtime tracking::tests::n_notifications_of_one_surface_coalesce_to_one_dirty_mark
       START [         ] (100/119) semio-framework-ui-runtime tracking::tests::nested_present_scopes_attribute_reads_to_the_right_surface

running 1 test
test tracking::tests::nested_present_scopes_attribute_reads_to_the_right_surface ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.043s] (100/119) semio-framework-ui-runtime tracking::tests::nested_present_scopes_attribute_reads_to_the_right_surface
       START [         ] (101/119) semio-framework-ui-runtime tracking::tests::presenter_reading_a_not_b_wakes_only_on_a

running 1 test
test tracking::tests::presenter_reading_a_not_b_wakes_only_on_a ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.051s] (101/119) semio-framework-ui-runtime tracking::tests::presenter_reading_a_not_b_wakes_only_on_a
       START [         ] (102/119) semio-framework-ui-runtime tracking::tests::reads_outside_a_present_scope_are_not_recorded

running 1 test
test tracking::tests::reads_outside_a_present_scope_are_not_recorded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.037s] (102/119) semio-framework-ui-runtime tracking::tests::reads_outside_a_present_scope_are_not_recorded
       START [         ] (103/119) semio-framework-ui-runtime tracking::tests::stale_edge_disappears_after_next_present_without_the_read

running 1 test
test tracking::tests::stale_edge_disappears_after_next_present_without_the_read ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.021s] (103/119) semio-framework-ui-runtime tracking::tests::stale_edge_disappears_after_next_present_without_the_read
       START [         ] (104/119) semio-framework-ui-runtime transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch

running 1 test

thread 'transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch' (7606222) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs:902:9:
assertion `left == right` failed: a burst of same-key deltas must still yield exactly one patch
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_runtime::transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch
   5: semio_framework_ui_runtime::transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch::{closure#0}
   6: <semio_framework_ui_runtime::transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch ... FAILED

failures:

failures:
    transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.02s

        FAIL [   0.060s] (104/119) semio-framework-ui-runtime transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch
       START [         ] (105/119) semio-framework-ui-runtime transaction::tests::a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction

running 1 test
test transaction::tests::a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.042s] (105/119) semio-framework-ui-runtime transaction::tests::a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction
       START [         ] (106/119) semio-framework-ui-runtime transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command

running 1 test

thread 'transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command' (7606263) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs:879:9:
assertion `left == right` failed
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_runtime::transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command
   5: semio_framework_ui_runtime::transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command::{closure#0}
   6: <semio_framework_ui_runtime::transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command ... FAILED

failures:

failures:
    transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.02s

        FAIL [   0.058s] (106/119) semio-framework-ui-runtime transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command
       START [         ] (107/119) semio-framework-ui-runtime transaction::tests::an_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics

running 1 test
test transaction::tests::an_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.082s] (107/119) semio-framework-ui-runtime transaction::tests::an_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics
       START [         ] (108/119) semio-framework-ui-runtime transaction::tests::an_entity_notified_but_not_read_by_any_surface_produces_no_patch

running 1 test
test transaction::tests::an_entity_notified_but_not_read_by_any_surface_produces_no_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.053s] (108/119) semio-framework-ui-runtime transaction::tests::an_entity_notified_but_not_read_by_any_surface_produces_no_patch
       START [         ] (109/119) semio-framework-ui-runtime transaction::tests::an_expired_wall_clock_budget_returns_before_consuming_input

running 1 test
test transaction::tests::an_expired_wall_clock_budget_returns_before_consuming_input ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.030s] (109/119) semio-framework-ui-runtime transaction::tests::an_expired_wall_clock_budget_returns_before_consuming_input
       START [         ] (110/119) semio-framework-ui-runtime transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch

running 1 test

thread 'transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch' (7606286) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs:853:9:
assertion `left == right` failed
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_runtime::transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch
   5: semio_framework_ui_runtime::transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch::{closure#0}
   6: <semio_framework_ui_runtime::transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch ... FAILED

failures:

failures:
    transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        FAIL [   0.032s] (110/119) semio-framework-ui-runtime transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch
       START [         ] (111/119) semio-framework-ui-runtime transaction::tests::cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision

running 1 test
test transaction::tests::cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.024s] (111/119) semio-framework-ui-runtime transaction::tests::cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision
       START [         ] (112/119) semio-framework-ui-runtime transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order

running 1 test

thread 'transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order' (7606295) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs:1115:9:
assertion `left == right` failed
  left: "[{\"surface\":\"a\",\"baseRevision\":0,\"revision\":1,\"ops\":[{\"type\":\"upsert\",\"id\":0,\"key\":\"root\",\"component\":{\"type\":\"text\",\"value\":\"0:0\"},\"layout\":{\"kind\":\"leaf\",\"width\":\"hug\",\"height\":\"hug\"},\"style\":{},\"activity\":\"idle\",\"accessibility\":{}},{\"type\":\"setRoot\",\"id\":0}]},{\"surface\":\"b\",\"baseRevision\":0,\"revision\":1,\"ops\":[{\"type\":\"upsert\",\"id\":0,\"key\":\"root\",\"component\":{\"type\":\"text\",\"value\":\"0:0\"},\"layout\":{\"kind\":\"leaf\",\"width\":\"hug\",\"height\":\"hug\"},\"style\":{},\"activity\":\"idle\",\"accessibility\":{}},{\"type\":\"setRoot\",\"id\":0}]}]"
 right: "[]"
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<alloc::string::String, alloc::string::String>
   4: semio_framework_ui_runtime::transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order
   5: semio_framework_ui_runtime::transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order::{closure#0}
   6: <semio_framework_ui_runtime::transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order ... FAILED

failures:

failures:
    transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.02s

        FAIL [   0.044s] (112/119) semio-framework-ui-runtime transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order
       START [         ] (113/119) semio-framework-ui-runtime transaction::tests::hard_credits_fault_before_any_candidate_snapshot_is_published

running 1 test
test transaction::tests::hard_credits_fault_before_any_candidate_snapshot_is_published ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.021s] (113/119) semio-framework-ui-runtime transaction::tests::hard_credits_fault_before_any_candidate_snapshot_is_published
       START [         ] (114/119) semio-framework-ui-runtime transaction::tests::next_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending

running 1 test
test transaction::tests::next_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.017s] (114/119) semio-framework-ui-runtime transaction::tests::next_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending
       START [         ] (115/119) semio-framework-ui-runtime transaction::tests::one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output

running 1 test
test transaction::tests::one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.035s] (115/119) semio-framework-ui-runtime transaction::tests::one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output
       START [         ] (116/119) semio-framework-ui-runtime transaction::tests::presence_flushes_on_its_own_channel_and_never_appears_in_a_patch

running 1 test
test transaction::tests::presence_flushes_on_its_own_channel_and_never_appears_in_a_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.022s] (116/119) semio-framework-ui-runtime transaction::tests::presence_flushes_on_its_own_channel_and_never_appears_in_a_patch
       START [         ] (117/119) semio-framework-ui-runtime transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command

running 1 test

thread 'transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command' (7606325) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs:1069:9:
assertion `left == right` failed
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_runtime::transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command
   5: semio_framework_ui_runtime::transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command::{closure#0}
   6: <semio_framework_ui_runtime::transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command ... FAILED

failures:

failures:
    transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        FAIL [   0.030s] (117/119) semio-framework-ui-runtime transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command
       START [         ] (118/119) semio-framework-ui-runtime transaction::tests::the_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget

running 1 test
test transaction::tests::the_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.022s] (118/119) semio-framework-ui-runtime transaction::tests::the_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget
       START [         ] (119/119) semio-framework-ui-runtime transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other

running 1 test

thread 'transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other' (7606366) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs:1011:9:
assertion `left == right` failed
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_runtime::transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other
   5: semio_framework_ui_runtime::transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other::{closure#0}
   6: <semio_framework_ui_runtime::transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other ... FAILED

failures:

failures:
    transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        FAIL [   0.033s] (119/119) semio-framework-ui-runtime transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other
────────────
     Summary [   4.889s] 119 tests run: 113 passed, 6 failed, 0 skipped
        FAIL [   0.060s] (104/119) semio-framework-ui-runtime transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch
        FAIL [   0.058s] (106/119) semio-framework-ui-runtime transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command
        FAIL [   0.032s] (110/119) semio-framework-ui-runtime transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch
        FAIL [   0.044s] (112/119) semio-framework-ui-runtime transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order
        FAIL [   0.030s] (117/119) semio-framework-ui-runtime transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command
        FAIL [   0.033s] (119/119) semio-framework-ui-runtime transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other
error: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib --no-fail-fast -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.


```
