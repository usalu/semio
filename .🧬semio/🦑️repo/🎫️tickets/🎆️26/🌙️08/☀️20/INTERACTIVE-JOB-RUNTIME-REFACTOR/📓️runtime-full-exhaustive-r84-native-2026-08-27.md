# Full Runtime R84

Actual exhaustive native result: **121 passed, zero failed, zero skipped**, 1.632s, exit0. Existing no-fail-fast route, jobs2, retained target and unchanged grants; no exclusions. This includes the previous runtime acceptance laws and new direct output receiver law, not the Plugin live tracker, guest execution, or hard callback latency.

Selected pre-dispatch hashes: `📓️runtime-full-r84-source-inputs-2026-08-27.md` (not an atomic full repository closure).

## Exact Command

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='exhaustive --lib --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-runtime-full-exhaustive-r84-2026-08-27.txt'
```

## Full Output

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib --no-fail-fast -- --nocapture

> bun ./📜️script.ts test exhaustive --lib --no-fail-fast -- --nocapture

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] surface-ownership-oracle checks=40
────────────
[32;1m Nextest run[0m ID [1me6bb0167-dc34-4fd3-8437-b1e6482f05c8[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m121[0m tests across [1m1[0m binary
[32;1m       START[0m [         ] (  1/121) [35;1msemio-framework-ui-runtime[0m [36mdispatch::tests[0m[36m::[0m[34;1ma_zero_tolerance_makes_any_trailing_revision_stale[0m

running 1 test
test dispatch::tests::a_zero_tolerance_makes_any_trailing_revision_stale ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (  1/121) [35;1msemio-framework-ui-runtime[0m [36mdispatch::tests[0m[36m::[0m[34;1ma_zero_tolerance_makes_any_trailing_revision_stale[0m
[32;1m       START[0m [         ] (  2/121) [35;1msemio-framework-ui-runtime[0m [36mdispatch::tests[0m[36m::[0m[34;1man_intent_at_or_ahead_of_the_current_revision_is_never_stale[0m

running 1 test
test dispatch::tests::an_intent_at_or_ahead_of_the_current_revision_is_never_stale ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (  2/121) [35;1msemio-framework-ui-runtime[0m [36mdispatch::tests[0m[36m::[0m[34;1man_intent_at_or_ahead_of_the_current_revision_is_never_stale[0m
[32;1m       START[0m [         ] (  3/121) [35;1msemio-framework-ui-runtime[0m [36mdispatch::tests[0m[36m::[0m[34;1man_intent_exactly_at_the_tolerance_is_not_yet_stale[0m

running 1 test
test dispatch::tests::an_intent_exactly_at_the_tolerance_is_not_yet_stale ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (  3/121) [35;1msemio-framework-ui-runtime[0m [36mdispatch::tests[0m[36m::[0m[34;1man_intent_exactly_at_the_tolerance_is_not_yet_stale[0m
[32;1m       START[0m [         ] (  4/121) [35;1msemio-framework-ui-runtime[0m [36mdispatch::tests[0m[36m::[0m[34;1man_intent_trailing_by_more_than_the_tolerance_is_stale[0m

running 1 test
test dispatch::tests::an_intent_trailing_by_more_than_the_tolerance_is_stale ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (  4/121) [35;1msemio-framework-ui-runtime[0m [36mdispatch::tests[0m[36m::[0m[34;1man_intent_trailing_by_more_than_the_tolerance_is_stale[0m
[32;1m       START[0m [         ] (  5/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mdefer_effects_queue_rather_than_run_inline[0m

running 1 test
test entity::tests::defer_effects_queue_rather_than_run_inline ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (  5/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mdefer_effects_queue_rather_than_run_inline[0m
[32;1m       START[0m [         ] (  6/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mdropped_subscription_stops_delivering[0m

running 1 test
test entity::tests::dropped_subscription_stops_delivering ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (  6/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mdropped_subscription_stops_delivering[0m
[32;1m       START[0m [         ] (  7/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1meffects_queue_rather_than_run_inline[0m

running 1 test
test entity::tests::effects_queue_rather_than_run_inline ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (  7/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1meffects_queue_rather_than_run_inline[0m
[32;1m       START[0m [         ] (  8/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mnested_lease_is_rejected_not_aliased[0m

running 1 test

thread 'entity::tests::nested_lease_is_rejected_not_aliased' (8378335) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️entity.rs:304:17:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.015s] (  8/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mnested_lease_is_rejected_not_aliased[0m
[32;1m       START[0m [         ] (  9/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mread_during_lease_is_rejected[0m

running 1 test
test entity::tests::read_during_lease_is_rejected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (  9/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mread_during_lease_is_rejected[0m
[32;1m       START[0m [         ] ( 10/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mrelease_is_queued_until_flush_releases[0m

running 1 test
test entity::tests::release_is_queued_until_flush_releases ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 10/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mrelease_is_queued_until_flush_releases[0m
[32;1m       START[0m [         ] ( 11/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mspawn_local_queues_future_for_the_embedder[0m

running 1 test
test entity::tests::spawn_local_queues_future_for_the_embedder ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 11/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mspawn_local_queues_future_for_the_embedder[0m
[32;1m       START[0m [         ] ( 12/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mstale_entity_id_never_resolves_to_new_occupant[0m

running 1 test
test entity::tests::stale_entity_id_never_resolves_to_new_occupant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 12/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mstale_entity_id_never_resolves_to_new_occupant[0m
[32;1m       START[0m [         ] ( 13/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mvalue_restored_after_panicking_closure[0m

running 1 test

thread 'entity::tests::value_restored_after_panicking_closure' (8378358) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️entity.rs:402:39:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 13/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mvalue_restored_after_panicking_closure[0m
[32;1m       START[0m [         ] ( 14/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mweak_entity_upgrade_fails_after_last_strong_drops[0m

running 1 test
test entity::tests::weak_entity_upgrade_fails_after_last_strong_drops ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 14/121) [35;1msemio-framework-ui-runtime[0m [36mentity::tests[0m[36m::[0m[34;1mweak_entity_upgrade_fails_after_last_strong_drops[0m
[32;1m       START[0m [         ] ( 15/121) [35;1msemio-framework-ui-runtime[0m [36mgateway::tests[0m[36m::[0m[34;1mfull_backing_sink_returns_full_synchronously_without_dropping_the_command[0m

running 1 test
test gateway::tests::full_backing_sink_returns_full_synchronously_without_dropping_the_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 15/121) [35;1msemio-framework-ui-runtime[0m [36mgateway::tests[0m[36m::[0m[34;1mfull_backing_sink_returns_full_synchronously_without_dropping_the_command[0m
[32;1m       START[0m [         ] ( 16/121) [35;1msemio-framework-ui-runtime[0m [36mgateway::tests[0m[36m::[0m[34;1mfull_local_capacity_returns_full_synchronously_without_dropping_the_command[0m

running 1 test
test gateway::tests::full_local_capacity_returns_full_synchronously_without_dropping_the_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 16/121) [35;1msemio-framework-ui-runtime[0m [36mgateway::tests[0m[36m::[0m[34;1mfull_local_capacity_returns_full_synchronously_without_dropping_the_command[0m
[32;1m       START[0m [         ] ( 17/121) [35;1msemio-framework-ui-runtime[0m [36mgateway::tests[0m[36m::[0m[34;1mresolving_a_ticket_frees_capacity_for_a_new_submission[0m

running 1 test
test gateway::tests::resolving_a_ticket_frees_capacity_for_a_new_submission ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 17/121) [35;1msemio-framework-ui-runtime[0m [36mgateway::tests[0m[36m::[0m[34;1mresolving_a_ticket_frees_capacity_for_a_new_submission[0m
[32;1m       START[0m [         ] ( 18/121) [35;1msemio-framework-ui-runtime[0m [36mgateway::tests[0m[36m::[0m[34;1mticket_round_trips_to_acknowledged_and_to_rejected[0m

running 1 test
test gateway::tests::ticket_round_trips_to_acknowledged_and_to_rejected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 18/121) [35;1msemio-framework-ui-runtime[0m [36mgateway::tests[0m[36m::[0m[34;1mticket_round_trips_to_acknowledged_and_to_rejected[0m
[32;1m       START[0m [         ] ( 19/121) [35;1msemio-framework-ui-runtime[0m [36minbox::tests[0m[36m::[0m[34;1mdrain_into_on_an_empty_inbox_is_a_no_op[0m

running 1 test
test inbox::tests::drain_into_on_an_empty_inbox_is_a_no_op ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 19/121) [35;1msemio-framework-ui-runtime[0m [36minbox::tests[0m[36m::[0m[34;1mdrain_into_on_an_empty_inbox_is_a_no_op[0m
[32;1m       START[0m [         ] ( 20/121) [35;1msemio-framework-ui-runtime[0m [36minbox::tests[0m[36m::[0m[34;1mdrain_into_respects_limit_and_leaves_the_remainder_queued[0m

running 1 test
test inbox::tests::drain_into_respects_limit_and_leaves_the_remainder_queued ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 20/121) [35;1msemio-framework-ui-runtime[0m [36minbox::tests[0m[36m::[0m[34;1mdrain_into_respects_limit_and_leaves_the_remainder_queued[0m
[32;1m       START[0m [         ] ( 21/121) [35;1msemio-framework-ui-runtime[0m [36minbox::tests[0m[36m::[0m[34;1mpush_beyond_capacity_returns_overflow_without_dropping_existing_entries[0m

running 1 test
test inbox::tests::push_beyond_capacity_returns_overflow_without_dropping_existing_entries ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 21/121) [35;1msemio-framework-ui-runtime[0m [36minbox::tests[0m[36m::[0m[34;1mpush_beyond_capacity_returns_overflow_without_dropping_existing_entries[0m
[32;1m       START[0m [         ] ( 22/121) [35;1msemio-framework-ui-runtime[0m [36minbox::tests[0m[36m::[0m[34;1msame_key_pushes_coalesce_to_the_newest_value[0m

running 1 test
test inbox::tests::same_key_pushes_coalesce_to_the_newest_value ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 22/121) [35;1msemio-framework-ui-runtime[0m [36minbox::tests[0m[36m::[0m[34;1msame_key_pushes_coalesce_to_the_newest_value[0m
[32;1m       START[0m [         ] ( 23/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1ma_burst_of_same_key_own_presence_writes_coalesces_to_the_newest_value[0m

running 1 test
test presence::tests::a_burst_of_same_key_own_presence_writes_coalesces_to_the_newest_value ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 23/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1ma_burst_of_same_key_own_presence_writes_coalesces_to_the_newest_value[0m
[32;1m       START[0m [         ] ( 24/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1ma_burst_of_same_key_peer_writes_coalesces_to_one_update[0m

running 1 test
test presence::tests::a_burst_of_same_key_peer_writes_coalesces_to_one_update ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 24/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1ma_burst_of_same_key_peer_writes_coalesces_to_one_update[0m
[32;1m       START[0m [         ] ( 25/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1mdistinct_peers_on_one_key_are_all_reported_and_expire_independently[0m

running 1 test
test presence::tests::distinct_peers_on_one_key_are_all_reported_and_expire_independently ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 25/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1mdistinct_peers_on_one_key_are_all_reported_and_expire_independently[0m
[32;1m       START[0m [         ] ( 26/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1mown_presence_never_expires[0m

running 1 test
test presence::tests::own_presence_never_expires ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 26/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1mown_presence_never_expires[0m
[32;1m       START[0m [         ] ( 27/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1mpresence_entries_expire_exactly_at_their_ttl_and_a_flush_after_expiry_omits_them[0m

running 1 test
test presence::tests::presence_entries_expire_exactly_at_their_ttl_and_a_flush_after_expiry_omits_them ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 27/121) [35;1msemio-framework-ui-runtime[0m [36mpresence::tests[0m[36m::[0m[34;1mpresence_entries_expire_exactly_at_their_ttl_and_a_flush_after_expiry_omits_them[0m
[32;1m       START[0m [         ] ( 28/121) [35;1msemio-framework-ui-runtime[0m [36mpresent::tests[0m[36m::[0m[34;1ma_stateless_fn_item_satisfies_present_generically[0m

running 1 test
test present::tests::a_stateless_fn_item_satisfies_present_generically ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] ( 28/121) [35;1msemio-framework-ui-runtime[0m [36mpresent::tests[0m[36m::[0m[34;1ma_stateless_fn_item_satisfies_present_generically[0m
[32;1m       START[0m [         ] ( 29/121) [35;1msemio-framework-ui-runtime[0m [36mpresent::tests[0m[36m::[0m[34;1mdeep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close[0m

running 1 test
test present::tests::deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 29/121) [35;1msemio-framework-ui-runtime[0m [36mpresent::tests[0m[36m::[0m[34;1mdeep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close[0m
[32;1m       START[0m [         ] ( 30/121) [35;1msemio-framework-ui-runtime[0m [36mpresent::tests[0m[36m::[0m[34;1mduplicate_stale_cancel_and_deadline_fault_before_publication[0m

running 1 test
test present::tests::duplicate_stale_cancel_and_deadline_fault_before_publication ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 30/121) [35;1msemio-framework-ui-runtime[0m [36mpresent::tests[0m[36m::[0m[34;1mduplicate_stale_cancel_and_deadline_fault_before_publication[0m
[32;1m       START[0m [         ] ( 31/121) [35;1msemio-framework-ui-runtime[0m [36mpresent::tests[0m[36m::[0m[34;1mmounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate[0m

running 1 test
test present::tests::mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 31/121) [35;1msemio-framework-ui-runtime[0m [36mpresent::tests[0m[36m::[0m[34;1mmounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate[0m
[32;1m       START[0m [         ] ( 32/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::handback_entry_tests[0m[36m::[0m[34;1mretained_handback_maintenance_entry_does_not_wait_for_registry[0m

running 1 test
test reconcile::handback_entry_tests::retained_handback_maintenance_entry_does_not_wait_for_registry ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 32/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::handback_entry_tests[0m[36m::[0m[34;1mretained_handback_maintenance_entry_does_not_wait_for_registry[0m
[32;1m       START[0m [         ] ( 33/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::handback_entry_tests[0m[36m::[0m[34;1mretained_handback_poison_is_fault_without_mutating_queued_owner[0m

running 1 test

thread '<unnamed>' (8378443) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../🚪️handback/🧪️component.rs:66:102:
fixture registry poison
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::handback_entry_tests::retained_handback_poison_is_fault_without_mutating_queued_owner::{closure#0}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test reconcile::handback_entry_tests::retained_handback_poison_is_fault_without_mutating_queued_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.015s] ( 33/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::handback_entry_tests[0m[36m::[0m[34;1mretained_handback_poison_is_fault_without_mutating_queued_owner[0m
[32;1m       START[0m [         ] ( 34/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::handback_entry_tests[0m[36m::[0m[34;1mretained_handback_take_entry_does_not_wait_for_registry[0m

running 1 test
test reconcile::handback_entry_tests::retained_handback_take_entry_does_not_wait_for_registry ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 34/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::handback_entry_tests[0m[36m::[0m[34;1mretained_handback_take_entry_does_not_wait_for_registry[0m
[32;1m       START[0m [         ] ( 35/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::output::tests[0m[36m::[0m[34;1msurface_output_pool_contended_drop_preserves_reserved_entry_until_exact_drain[0m

running 1 test
[DEBUG] output-pool held-mutex-drop-waits=false exact-return-drained=true
test reconcile::output::tests::surface_output_pool_contended_drop_preserves_reserved_entry_until_exact_drain ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 35/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::output::tests[0m[36m::[0m[34;1msurface_output_pool_contended_drop_preserves_reserved_entry_until_exact_drain[0m
[32;1m       START[0m [         ] ( 36/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::output::tests[0m[36m::[0m[34;1msurface_output_pool_defers_reuse_and_rejects_stale_epoch_after_final_return[0m

running 1 test
[DEBUG] output-pool reuse-before-drain=false exact-epoch=2 explicit-close-no-second-return=true
test reconcile::output::tests::surface_output_pool_defers_reuse_and_rejects_stale_epoch_after_final_return ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 36/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::output::tests[0m[36m::[0m[34;1msurface_output_pool_defers_reuse_and_rejects_stale_epoch_after_final_return[0m
[32;1m       START[0m [         ] ( 37/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::output::tests[0m[36m::[0m[34;1msurface_output_pool_zero_grant_and_busy_registry_leave_authority_unchanged[0m

running 1 test
[DEBUG] output-pool busy-refusal-exact=true zero-grant-mutates=false static-bytes=125088
test reconcile::output::tests::surface_output_pool_zero_grant_and_busy_registry_leave_authority_unchanged ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 37/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::output::tests[0m[36m::[0m[34;1msurface_output_pool_zero_grant_and_busy_registry_leave_authority_unchanged[0m
[32;1m       START[0m [         ] ( 38/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::patch_handoff_tests[0m[36m::[0m[34;1mretained_patch_handoff_close_respects_all_grants_and_contended_exact_credit[0m

running 1 test
[DEBUG] patch-close grants=1,64,4096 exact-credit-contention=true exact-handback-contention=true
test reconcile::patch_handoff_tests::retained_patch_handoff_close_respects_all_grants_and_contended_exact_credit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 38/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::patch_handoff_tests[0m[36m::[0m[34;1mretained_patch_handoff_close_respects_all_grants_and_contended_exact_credit[0m
[32;1m       START[0m [         ] ( 39/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::patch_handoff_tests[0m[36m::[0m[34;1mretained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment[0m

running 1 test
[DEBUG] patch-handoff exact-slots=true occupied-target-preserved=true invalid-ack-preserved=true surface-bytes=4
test reconcile::patch_handoff_tests::retained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 39/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::patch_handoff_tests[0m[36m::[0m[34;1mretained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment[0m
[32;1m       START[0m [         ] ( 40/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::patch_handoff_tests[0m[36m::[0m[34;1mretained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority[0m

running 1 test

thread 'reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority' (8378474) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../🩹️patch/🧪️component.rs:100:13:
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

thread 'reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority' (8378474) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../🩹️patch/🧪️component.rs:100:13:
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

thread 'reconcile::patch_handoff_tests::retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority' (8378474) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../🩹️patch/🧪️component.rs:100:13:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 40/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::patch_handoff_tests[0m[36m::[0m[34;1mretained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority[0m
[32;1m       START[0m [         ] ( 41/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mabandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged[0m

running 1 test
test reconcile::tests::abandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 41/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mabandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged[0m
[32;1m       START[0m [         ] ( 42/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mallocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation[0m

running 1 test
test reconcile::tests::allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] ( 42/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mallocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation[0m
[32;1m       START[0m [         ] ( 43/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_completion_transfers_do_not_borrow_the_child_grant[0m

running 1 test
[DEBUG] parent-child-grants compare-final=4096 lease-close=4096 comparison-owner=2256 source-return=3096 candidate-physical=6416 separate-turns=true
test reconcile::tests::canonical_document_tests::surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.016s] ( 43/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_completion_transfers_do_not_borrow_the_child_grant[0m
[32;1m       START[0m [         ] ( 44/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_existing_pair_stays_structurally_owned_across_unwind[0m

running 1 test

thread 'reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind' (8378491) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1579:51:
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

thread 'reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind' (8378491) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1579:51:
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

thread 'reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind' (8378491) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1579:51:
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

thread 'reconcile::tests::canonical_document_tests::surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind' (8378491) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1376:43:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.03s

[32;1m        PASS[0m [   0.040s] ( 44/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_existing_pair_stays_structurally_owned_across_unwind[0m
[32;1m       START[0m [         ] ( 45/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_fresh_children_retain_completed_roots_for_a_separate_turn[0m

running 1 test
[DEBUG] fresh-child-completion [("component", true), ("bindings", true)]
test reconcile::tests::canonical_document_tests::surface_canonical_document_fresh_children_retain_completed_roots_for_a_separate_turn ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.017s] ( 45/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_fresh_children_retain_completed_roots_for_a_separate_turn[0m
[32;1m       START[0m [         ] ( 46/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_nine_live_reconcilers_share_the_original_root_with_readers[0m

running 1 test
[DEBUG] canonical-reconcilers actual-surfaces=9 exact-root-readers=9 roots-after-owner-close=9 typed-reader-close=true
test reconcile::tests::canonical_document_tests::surface_canonical_document_nine_live_reconcilers_share_the_original_root_with_readers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] ( 46/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_nine_live_reconcilers_share_the_original_root_with_readers[0m
[32;1m       START[0m [         ] ( 47/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_old_reader_keeps_original_credit_during_replacement[0m

running 1 test
[DEBUG] canonical-reader-replacement grant=1 original-root-unchanged=true original-credit-retained=true typed-terminal=true
[DEBUG] canonical-reader-replacement grant=64 original-root-unchanged=true original-credit-retained=true typed-terminal=true
[DEBUG] canonical-reader-replacement grant=4096 original-root-unchanged=true original-credit-retained=true typed-terminal=true
test reconcile::tests::canonical_document_tests::surface_canonical_document_old_reader_keeps_original_credit_during_replacement ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 47/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::canonical_document_tests[0m[36m::[0m[34;1msurface_canonical_document_old_reader_keeps_original_credit_during_replacement[0m
[32;1m       START[0m [         ] ( 48/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanged_component_with_unchanged_layout_emits_set_component_not_upsert[0m

running 1 test
test reconcile::tests::changed_component_with_unchanged_layout_emits_set_component_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 48/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanged_component_with_unchanged_layout_emits_set_component_not_upsert[0m
[32;1m       START[0m [         ] ( 49/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node[0m

running 1 test
test reconcile::tests::changing_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 49/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node[0m
[32;1m       START[0m [         ] ( 50/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_only_accessibility_emits_exactly_one_set_accessibility_not_upsert[0m

running 1 test
test reconcile::tests::changing_only_accessibility_emits_exactly_one_set_accessibility_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 50/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_only_accessibility_emits_exactly_one_set_accessibility_not_upsert[0m
[32;1m       START[0m [         ] ( 51/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_only_bindings_emits_exactly_one_set_bindings_not_upsert[0m

running 1 test
test reconcile::tests::changing_only_bindings_emits_exactly_one_set_bindings_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 51/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_only_bindings_emits_exactly_one_set_bindings_not_upsert[0m
[32;1m       START[0m [         ] ( 52/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_only_menu_emits_exactly_one_set_menu_not_upsert[0m

running 1 test
test reconcile::tests::changing_only_menu_emits_exactly_one_set_menu_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 52/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_only_menu_emits_exactly_one_set_menu_not_upsert[0m
[32;1m       START[0m [         ] ( 53/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_only_style_emits_exactly_one_set_style_not_upsert[0m

running 1 test
test reconcile::tests::changing_only_style_emits_exactly_one_set_style_not_upsert ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 53/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_only_style_emits_exactly_one_set_style_not_upsert[0m
[32;1m       START[0m [         ] ( 54/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops[0m

running 1 test
test reconcile::tests::changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 54/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mchanging_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops[0m
[32;1m       START[0m [         ] ( 55/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mduplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed[0m

running 1 test

thread 'reconcile::tests::duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed' (8378544) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:244:9:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 55/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mduplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed[0m
[32;1m       START[0m [         ] ( 56/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mevery_large_tree_cursor_slice_stays_below_eight_milliseconds[0m

running 1 test
test reconcile::tests::every_large_tree_cursor_slice_stays_below_eight_milliseconds ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.015s] ( 56/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mevery_large_tree_cursor_slice_stays_below_eight_milliseconds[0m
[32;1m       START[0m [         ] ( 57/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mfirst_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent[0m

running 1 test
test reconcile::tests::first_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 57/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mfirst_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent[0m
[32;1m       START[0m [         ] ( 58/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mfixed_runtime_owners_keep_bounded_state_off_the_stack[0m

running 1 test
test reconcile::tests::fixed_runtime_owners_keep_bounded_state_off_the_stack ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[DEBUG] canonical-owner-layout reconciler=760 cursor=48552 retained=65024
[32;1m        PASS[0m [   0.010s] ( 58/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mfixed_runtime_owners_keep_bounded_state_off_the_stack[0m
[32;1m       START[0m [         ] ( 59/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1midentifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation[0m

running 1 test
test reconcile::tests::identifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 59/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1midentifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation[0m
[32;1m       START[0m [         ] ( 60/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mids_are_never_reused_after_removal[0m

running 1 test
test reconcile::tests::ids_are_never_reused_after_removal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 60/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mids_are_never_reused_after_removal[0m
[32;1m       START[0m [         ] ( 61/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1minserting_a_middle_sibling_preserves_the_others_ids[0m

running 1 test
test reconcile::tests::inserting_a_middle_sibling_preserves_the_others_ids ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 61/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1minserting_a_middle_sibling_preserves_the_others_ids[0m
[32;1m       START[0m [         ] ( 62/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1minstance_lifetime_published_patch_close_retains_exact_handback_until_terminal[0m

running 1 test
[DEBUG] published-close owner-transitions=4 physical-turns=17 semantic-bytes=10 grant=1
test reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] ( 62/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1minstance_lifetime_published_patch_close_retains_exact_handback_until_terminal[0m
[32;1m       START[0m [         ] ( 63/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mmark_rejected_then_reconcile_emits_a_full_resend[0m

running 1 test
test reconcile::tests::mark_rejected_then_reconcile_emits_a_full_resend ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 63/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mmark_rejected_then_reconcile_emits_a_full_resend[0m
[32;1m       START[0m [         ] ( 64/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mopaque_surface_document_uses_aggregate_credits_instead_of_scalar_page[0m

running 1 test
test reconcile::tests::opaque_surface_document_uses_aggregate_credits_instead_of_scalar_page ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 64/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mopaque_surface_document_uses_aggregate_credits_instead_of_scalar_page[0m
[32;1m       START[0m [         ] ( 65/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind[0m

running 1 test

thread 'reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind' (8378587) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📤️output/🧪️component.rs:161:9:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 65/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind[0m
[32;1m       START[0m [         ] ( 66/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer[0m

running 1 test
[DEBUG] ready-revalidation actual=["fault", "fault", "pending", "pending"] exact-source-preserved=true
test reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 66/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer[0m
[32;1m       START[0m [         ] ( 67/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_admission_refuses_before_producer_when_only_one_handback_is_free[0m

running 1 test
[DEBUG] handback-admission one-free-accepted=false producer-invoked=false
test reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 67/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_admission_refuses_before_producer_when_only_one_handback_is_free[0m
[32;1m       START[0m [         ] ( 68/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_admission_transfers_after_seal_with_no_unreserved_handback[0m

running 1 test
[DEBUG] handback-admission post-seal-transfer=true late-slot-acquisition=false
test reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 68/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_admission_transfers_after_seal_with_no_unreserved_handback[0m
[32;1m       START[0m [         ] ( 69/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind[0m

running 1 test

thread 'reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind' (8378605) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📤️output/🧪️component.rs:231:9:
[DEBUG] direct pool receiver callback after actual transfer
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}
   3: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}>, ()>
   8: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind
   9: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}
  10: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] direct-pool receiver-bytes=8936 original-payload=true original-shell=true callback-unwind-retained=true
test reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 69/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind[0m
[32;1m       START[0m [         ] ( 70/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_keeps_exact_paired_ready_on_refusal_and_fifo_handoff[0m

running 1 test
[DEBUG] output-pool fifo=2 exact-rejected-pointer=true paired-credit=true close-grants=1,64,4096
test reconcile::tests::output_pool_tests::surface_output_pool_keeps_exact_paired_ready_on_refusal_and_fifo_handoff ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 70/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_keeps_exact_paired_ready_on_refusal_and_fifo_handoff[0m
[32;1m       START[0m [         ] ( 71/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_reserves_before_producer_and_refuses_the_sixty_fifth[0m

running 1 test
[DEBUG] output-pool preproducer=64 extra=false entry-limit=64 independent-payload-quota=false
test reconcile::tests::output_pool_tests::surface_output_pool_reserves_before_producer_and_refuses_the_sixty_fifth ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 71/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_reserves_before_producer_and_refuses_the_sixty_fifth[0m
[32;1m       START[0m [         ] ( 72/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_static_backing_joins_existing_ledger_once_without_a_root_slot[0m

running 1 test
[DEBUG] output-pool static-ledger contract=390800 runtime=143568 total=534368 additional-root-slots=0 final-release-retains-static=true
test reconcile::tests::output_pool_tests::surface_output_pool_static_backing_joins_existing_ledger_once_without_a_root_slot ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] ( 72/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_static_backing_joins_existing_ledger_once_without_a_root_slot[0m
[32;1m       START[0m [         ] ( 73/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_binding_clone_requires_bounded_backing_and_copy[0m

running 1 test
[DEBUG] surface-binding-clone turns=135 allocated=79744 initialized=66304 maximum-allocation=2072 maximum-placement=2072
test reconcile::tests::ownership::surface_ownership_binding_clone_requires_bounded_backing_and_copy ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 73/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_binding_clone_requires_bounded_backing_and_copy[0m
[32;1m       START[0m [         ] ( 74/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_binding_copy_cancel_keeps_all_original_and_partial_backings[0m

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.018s] ( 74/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_binding_copy_cancel_keeps_all_original_and_partial_backings[0m
[32;1m       START[0m [         ] ( 75/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_binding_copy_unwind_keeps_owners_outside_callback[0m

running 1 test

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (8378632) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1591:49:
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

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (8378632) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1591:49:
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

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (8378632) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1591:49:
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

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (8378632) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1591:49:
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

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (8378632) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1591:49:
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

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (8378632) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1591:49:
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

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (8378632) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1591:49:
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

thread 'reconcile::tests::ownership::surface_ownership_binding_copy_unwind_keeps_owners_outside_callback' (8378632) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1591:49:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.022s] ( 75/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_binding_copy_unwind_keeps_owners_outside_callback[0m
[32;1m       START[0m [         ] ( 76/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_component_copy_charges_actual_surface_backing_before_publication[0m

running 1 test
[DEBUG] surface-component-copy turns=18 reported=91805 ledger-allocation=32768 actual-allocation=32768
test reconcile::tests::ownership::surface_ownership_component_copy_charges_actual_surface_backing_before_publication ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] ( 76/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_component_copy_charges_actual_surface_backing_before_publication[0m
[32;1m       START[0m [         ] ( 77/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source[0m

running 1 test

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (8378647) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1585:51:
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

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (8378647) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1585:51:
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

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (8378647) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1585:51:
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

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (8378647) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1585:51:
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

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (8378647) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1585:51:
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

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (8378647) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1585:51:
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

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (8378647) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1585:51:
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

thread 'reconcile::tests::ownership::surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source' (8378647) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1585:51:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.03s

[32;1m        PASS[0m [   0.043s] ( 77/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source[0m
[32;1m       START[0m [         ] ( 78/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_existing_component_refuses_before_cloning_unadmitted_payload[0m

running 1 test
[DEBUG] existing-component-refusal rejected=true allocation-before-admission=0 source-unchanged=true
test reconcile::tests::ownership::surface_ownership_existing_component_refuses_before_cloning_unadmitted_payload ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 78/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_existing_component_refuses_before_cloning_unadmitted_payload[0m
[32;1m       START[0m [         ] ( 79/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_existing_component_retains_comparison_and_copy_between_turns[0m

running 1 test
[DEBUG] existing-component-copy turns=42 allocation-ledger=32768 old-unchanged=true
test reconcile::tests::ownership::surface_ownership_existing_component_retains_comparison_and_copy_between_turns ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.023s] ( 79/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_existing_component_retains_comparison_and_copy_between_turns[0m
[32;1m       START[0m [         ] ( 80/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_finalize_transfers_exact_record_and_index_allocations[0m

running 1 test
[DEBUG] surface-finalize-transfer exact-records=true exact-indexes=true replacement-bytes=0 closed=true
test reconcile::tests::ownership::surface_ownership_finalize_transfers_exact_record_and_index_allocations ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 80/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_finalize_transfers_exact_record_and_index_allocations[0m
[32;1m       START[0m [         ] ( 81/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_inline_fields_do_not_allocate_a_second_owner[0m

running 1 test
[DEBUG] surface-inline-footprint name="tree-item-icon" before=19368 after=19368 delta=0 items-before=14 items-after=15
[DEBUG] surface-inline-footprint name="reserved-binding" before=218280 after=218280 delta=0 items-before=12 items-after=14
test reconcile::tests::ownership::surface_ownership_inline_fields_do_not_allocate_a_second_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] ( 81/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_inline_fields_do_not_allocate_a_second_owner[0m
[32;1m       START[0m [         ] ( 82/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_native_backing_inventory_preserves_capacity[0m

running 1 test
test reconcile::tests::ownership::surface_ownership_native_backing_inventory_preserves_capacity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out[DEBUG] surface-physical-owner owner=tree-node bytes=6456
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
; finished in 0.00s

[32;1m        PASS[0m [   0.051s] ( 82/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_native_backing_inventory_preserves_capacity[0m
[32;1m       START[0m [         ] ( 83/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_patch_backing_is_admitted_in_separate_turns[0m

running 1 test
[DEBUG] surface-patch-allocation turns=2 largest=27672 operation-bytes=6416
test reconcile::tests::ownership::surface_ownership_patch_backing_is_admitted_in_separate_turns ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.030s] ( 83/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_patch_backing_is_admitted_in_separate_turns[0m
[32;1m       START[0m [         ] ( 84/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_patch_refusal_and_cancel_keep_exact_unallocated_owner[0m

running 1 test
[DEBUG] surface-patch-cancel stage=1 retained-before=0 terminal=true allocation-during-close=0
[DEBUG] surface-patch-cancel stage=2 retained-before=27672 terminal=true allocation-during-close=0
[DEBUG] surface-patch-cancel stage=3 retained-before=34088 terminal=true allocation-during-close=0
[DEBUG] surface-patch-cancel stage=4 retained-before=34088 terminal=true allocation-during-close=0
test reconcile::tests::ownership::surface_ownership_patch_refusal_and_cancel_keep_exact_unallocated_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.037s] ( 84/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_patch_refusal_and_cancel_keep_exact_unallocated_owner[0m
[32;1m       START[0m [         ] ( 85/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_resident_reservation_uses_one_shared_aggregate_ledger[0m

running 1 test
[DEBUG] runtime-resident-join expected-bytes=65536 observed-bytes=65536 expected-slots=1 observed-slots=1
test reconcile::tests::ownership::surface_ownership_resident_reservation_uses_one_shared_aggregate_ledger ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 85/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_resident_reservation_uses_one_shared_aggregate_ledger[0m
[32;1m       START[0m [         ] ( 86/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_resident_return_maintenance_preserves_contended_credit[0m

running 1 test
[DEBUG] runtime-resident-return mutex-busy-keeps-credit=true maintenance-resumes=true exact-return=65536
test reconcile::tests::ownership::surface_ownership_resident_return_maintenance_preserves_contended_credit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 86/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_resident_return_maintenance_preserves_contended_credit[0m
[32;1m       START[0m [         ] ( 87/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_transfer_preserves_backing_without_allocating_replacement[0m

running 1 test
[DEBUG] surface-backing-transfer source-bytes=0 moved-bytes=64
[DEBUG] surface-moved-source rejected-exact-payload=true payload-capacity=16 replacement-bytes=0
test reconcile::tests::ownership::surface_ownership_transfer_preserves_backing_without_allocating_replacement ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 87/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::ownership[0m[36m::[0m[34;1msurface_ownership_transfer_preserves_backing_without_allocating_replacement[0m
[32;1m       START[0m [         ] ( 88/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mpersistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement[0m

running 1 test
test reconcile::tests::persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 88/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mpersistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement[0m
[32;1m       START[0m [         ] ( 89/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mpublic_drop_handback_is_lossless_at_terminal_cap_and_plus_one[0m

running 1 test
test reconcile::tests::public_drop_handback_is_lossless_at_terminal_cap_and_plus_one ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.06s

[32;1m        PASS[0m [   0.070s] ( 89/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mpublic_drop_handback_is_lossless_at_terminal_cap_and_plus_one[0m
[32;1m       START[0m [         ] ( 90/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mremoving_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained[0m

running 1 test
test reconcile::tests::removing_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 90/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mremoving_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained[0m
[32;1m       START[0m [         ] ( 91/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mreordering_siblings_preserves_every_id_and_emits_only_set_children[0m

running 1 test
test reconcile::tests::reordering_siblings_preserves_every_id_and_emits_only_set_children ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 91/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mreordering_siblings_preserves_every_id_and_emits_only_set_children[0m
[32;1m       START[0m [         ] ( 92/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mresumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics[0m

running 1 test
test reconcile::tests::resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 92/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mresumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics[0m
[32;1m       START[0m [         ] ( 93/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mretained_map_page_advances_each_key_once_without_rewalking_prior_entries[0m

running 1 test
test reconcile::tests::retained_map_page_advances_each_key_once_without_rewalking_prior_entries ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 93/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mretained_map_page_advances_each_key_once_without_rewalking_prior_entries[0m
[32;1m       START[0m [         ] ( 94/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mround_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot[0m

running 1 test
test reconcile::tests::round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.017s] ( 94/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mround_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot[0m
[32;1m       START[0m [         ] ( 95/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1msemantic_aggregate_quota_faults_before_key_or_record_clone[0m

running 1 test
test reconcile::tests::semantic_aggregate_quota_faults_before_key_or_record_clone ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 95/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1msemantic_aggregate_quota_faults_before_key_or_record_clone[0m
[32;1m       START[0m [         ] ( 96/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1msemantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion[0m

running 1 test
test reconcile::tests::semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 96/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1msemantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion[0m
[32;1m       START[0m [         ] ( 97/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1msemantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged[0m

running 1 test
test reconcile::tests::semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 97/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1msemantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged[0m
[32;1m       START[0m [         ] ( 98/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mstale_cancel_and_drop_handoff_preserve_public_terminal_ownership[0m

running 1 test
test reconcile::tests::stale_cancel_and_drop_handoff_preserve_public_terminal_ownership ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 98/121) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests[0m[36m::[0m[34;1mstale_cancel_and_drop_handoff_preserve_public_terminal_ownership[0m
[32;1m       START[0m [         ] ( 99/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mfinish_rejects_mismatched_surface[0m

running 1 test

thread 'tracking::tests::finish_rejects_mismatched_surface' (8378796) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️tracking.rs:75:9:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 99/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mfinish_rejects_mismatched_surface[0m
[32;1m       START[0m [         ] (100/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mn_notifications_of_one_surface_coalesce_to_one_dirty_mark[0m

running 1 test
test tracking::tests::n_notifications_of_one_surface_coalesce_to_one_dirty_mark ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (100/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mn_notifications_of_one_surface_coalesce_to_one_dirty_mark[0m
[32;1m       START[0m [         ] (101/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mnested_present_scopes_attribute_reads_to_the_right_surface[0m

running 1 test
test tracking::tests::nested_present_scopes_attribute_reads_to_the_right_surface ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (101/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mnested_present_scopes_attribute_reads_to_the_right_surface[0m
[32;1m       START[0m [         ] (102/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mpresenter_reading_a_not_b_wakes_only_on_a[0m

running 1 test
test tracking::tests::presenter_reading_a_not_b_wakes_only_on_a ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (102/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mpresenter_reading_a_not_b_wakes_only_on_a[0m
[32;1m       START[0m [         ] (103/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mreads_outside_a_present_scope_are_not_recorded[0m

running 1 test
test tracking::tests::reads_outside_a_present_scope_are_not_recorded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (103/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mreads_outside_a_present_scope_are_not_recorded[0m
[32;1m       START[0m [         ] (104/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mstale_edge_disappears_after_next_present_without_the_read[0m

running 1 test
test tracking::tests::stale_edge_disappears_after_next_present_without_the_read ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (104/121) [35;1msemio-framework-ui-runtime[0m [36mtracking::tests[0m[36m::[0m[34;1mstale_edge_disappears_after_next_present_without_the_read[0m
[32;1m       START[0m [         ] (105/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1ma_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch[0m

running 1 test
test transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (105/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1ma_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch[0m
[32;1m       START[0m [         ] (106/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1ma_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction[0m

running 1 test
test transaction::tests::a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (106/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1ma_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction[0m
[32;1m       START[0m [         ] (107/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1ma_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command[0m

running 1 test
test transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (107/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1ma_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command[0m
[32;1m       START[0m [         ] (108/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1man_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics[0m

running 1 test
test transaction::tests::an_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (108/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1man_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics[0m
[32;1m       START[0m [         ] (109/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1man_entity_notified_but_not_read_by_any_surface_produces_no_patch[0m

running 1 test
test transaction::tests::an_entity_notified_but_not_read_by_any_surface_produces_no_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (109/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1man_entity_notified_but_not_read_by_any_surface_produces_no_patch[0m
[32;1m       START[0m [         ] (110/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1man_expired_wall_clock_budget_returns_before_consuming_input[0m

running 1 test
test transaction::tests::an_expired_wall_clock_budget_returns_before_consuming_input ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (110/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1man_expired_wall_clock_budget_returns_before_consuming_input[0m
[32;1m       START[0m [         ] (111/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1man_intent_mutates_entity_state_and_the_following_transact_emits_a_patch[0m

running 1 test
test transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (111/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1man_intent_mutates_entity_state_and_the_following_transact_emits_a_patch[0m
[32;1m       START[0m [         ] (112/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mcancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision[0m

running 1 test
test transaction::tests::cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (112/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mcancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision[0m
[32;1m       START[0m [         ] (113/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mdeterministic_surface_order_is_independent_of_hash_map_insertion_order[0m

running 1 test
test transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (113/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mdeterministic_surface_order_is_independent_of_hash_map_insertion_order[0m
[32;1m       START[0m [         ] (114/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mhard_credits_fault_before_any_candidate_snapshot_is_published[0m

running 1 test
test transaction::tests::hard_credits_fault_before_any_candidate_snapshot_is_published ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (114/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mhard_credits_fault_before_any_candidate_snapshot_is_published[0m
[32;1m       START[0m [         ] (115/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mnext_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending[0m

running 1 test
test transaction::tests::next_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (115/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mnext_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending[0m
[32;1m       START[0m [         ] (116/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mone_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output[0m

running 1 test
test transaction::tests::one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (116/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mone_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output[0m
[32;1m       START[0m [         ] (117/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mpresence_flushes_on_its_own_channel_and_never_appears_in_a_patch[0m

running 1 test
test transaction::tests::presence_flushes_on_its_own_channel_and_never_appears_in_a_patch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (117/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mpresence_flushes_on_its_own_channel_and_never_appears_in_a_patch[0m
[32;1m       START[0m [         ] (118/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mrepeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command[0m

running 1 test
test transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (118/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mrepeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command[0m
[32;1m       START[0m [         ] (119/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mthe_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget[0m

running 1 test
test transaction::tests::the_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (119/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mthe_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget[0m
[32;1m       START[0m [         ] (120/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mtransaction_canonical_job_preserves_independent_node_credit[0m

running 1 test
test transaction::tests::transaction_canonical_job_preserves_independent_node_credit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (120/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mtransaction_canonical_job_preserves_independent_node_credit[0m
[32;1m       START[0m [         ] (121/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mtwo_surfaces_are_independent_dirtying_one_does_not_re_present_the_other[0m

running 1 test
test transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (121/121) [35;1msemio-framework-ui-runtime[0m [36mtransaction::tests[0m[36m::[0m[34;1mtwo_surfaces_are_independent_dirtying_one_does_not_re_present_the_other[0m
────────────
[32;1m     Summary[0m [   1.632s] [1m121[0m tests run: [1m121[0m [32;1mpassed[0m, [1m0[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-ezcZlm[0m



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs



```

