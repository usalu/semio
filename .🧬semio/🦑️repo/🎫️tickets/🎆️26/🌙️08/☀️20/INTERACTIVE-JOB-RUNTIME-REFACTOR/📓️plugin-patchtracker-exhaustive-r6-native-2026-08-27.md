# Plugin PatchTracker Exhaustive R6

Preservation note: the raw .txt path subsequently returned ENOENT. The complete 354-line output below was successfully read and copied before that disappearance; its actual footer remains present. No raw file is recreated, and this lane performed no cleanup, deletion, or relocation.

Actual canonical exhaustive/no-fail-fast run: 30 tests selected, 29 passed, 1 failed, 492 skipped; elapsed 0.646s, Nx exit 1. All seven `mounted_output_admission_` laws passed, including the actual two-thread shared-pool saturation/restore test. This is not a full Plugin, guest, or callback-timing gate.

The sole failure is `mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner`: its queued admission at source line 1609 returns the tree before the final cap-plus-one assertion. The fixture calculates aggregate bytes / default maximum job bytes without accounting for newly preadmitted static output backing. Exact cause and correction are under inspection; this report retains the failure, not a weakened capacity claim.

The previous R5 same-process 2-pass/4-fail and six individually passing diagnostic processes remain separately recorded in [R5 combined and isolated report](./📓️plugin-mounted-output-r5-combined-isolated-2026-08-27.md).

## Command

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive component::reactor::patches::tests:: --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-patchtracker-exhaustive-r6-2026-08-27.txt'
```

## Complete Captured Output

```text

> nx run @semio-tech/framework-plugin:test --args=exhaustive component::reactor::patches::tests:: --no-fail-fast -- --nocapture

> bun 📜️script.ts test exhaustive component::reactor::patches::tests:: --no-fail-fast -- --nocapture

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] plugin-runner-oracle cases=6
────────────
[32;1m Nextest run[0m ID [1mc2c25113-c019-4468-9a2b-64cf3065ecd6[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m30[0m tests across [1m1[0m binary ([1m492[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] ( 1/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mactor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot[0m

running 1 test
test component::reactor::patches::tests::actor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] ( 1/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mactor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot[0m
[32;1m       START[0m [         ] ( 2/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mcap_plus_one_returns_the_exact_tree_owner[0m

running 1 test
test component::reactor::patches::tests::cap_plus_one_returns_the_exact_tree_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 2/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mcap_plus_one_returns_the_exact_tree_owner[0m
[32;1m       START[0m [         ] ( 3/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mclose_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish[0m

running 1 test
test component::reactor::patches::tests::close_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] ( 3/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mclose_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish[0m
[32;1m       START[0m [         ] ( 4/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1meffects_publish_in_admission_order_even_when_later_tree_finishes_first[0m

running 1 test
test component::reactor::patches::tests::effects_publish_in_admission_order_even_when_later_tree_finishes_first ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] ( 4/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1meffects_publish_in_admission_order_even_when_later_tree_finishes_first[0m
[32;1m       START[0m [         ] ( 5/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mgeneration_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation[0m

running 1 test
test component::reactor::patches::tests::generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 5/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mgeneration_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation[0m
[32;1m       START[0m [         ] ( 6/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_publishes_every_section_beyond_thirty_two_nodes[0m

running 1 test
test component::reactor::patches::tests::mounted_catalogue_publishes_every_section_beyond_thirty_two_nodes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.03s

[32;1m        PASS[0m [   0.046s] ( 6/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_publishes_every_section_beyond_thirty_two_nodes[0m
[32;1m       START[0m [         ] ( 7/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_reports_producer_failure_once_before_cleanup[0m

running 1 test
test component::reactor::patches::tests::mounted_catalogue_reports_producer_failure_once_before_cleanup ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] ( 7/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_reports_producer_failure_once_before_cleanup[0m
[32;1m       START[0m [         ] ( 8/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_reports_reconcile_capacity_without_leaking_owners[0m

running 1 test
test component::reactor::patches::tests::mounted_catalogue_reports_reconcile_capacity_without_leaking_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.03s

[32;1m        PASS[0m [   0.045s] ( 8/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_reports_reconcile_capacity_without_leaking_owners[0m
[32;1m       START[0m [         ] ( 9/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_document_tree_publishes_nested_interactive_rows[0m

running 1 test
test component::reactor::patches::tests::mounted_document_tree_publishes_nested_interactive_rows ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.029s] ( 9/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_document_tree_publishes_nested_interactive_rows[0m
[32;1m       START[0m [         ] (10/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_cancel_and_drop_keep_the_original_close_generation[0m

running 1 test
[DEBUG] mounted-uncommitted-close drop=false generation=1 preserved=true revision=0 terminal=true
[DEBUG] mounted-uncommitted-close drop=true generation=1 preserved=true revision=0 terminal=true
test component::reactor::patches::tests::mounted_output_admission_cancel_and_drop_keep_the_original_close_generation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (10/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_cancel_and_drop_keep_the_original_close_generation[0m
[32;1m       START[0m [         ] (11/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_close_waits_for_the_original_uncommitted_grant[0m

running 1 test
[DEBUG] live-output close-waits-for-original-grant=true rejected-tree-pointer-preserved=true
test component::reactor::patches::tests::mounted_output_admission_close_waits_for_the_original_uncommitted_grant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (11/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_close_waits_for_the_original_uncommitted_grant[0m
[32;1m       START[0m [         ] (12/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_concurrent_trackers_share_one_fixed_pool_without_overadmission[0m

running 1 test
[DEBUG] live-output same-process-workers=2 preoccupied=63 accepted=1 exact-refusal=true full64-restored=true
test component::reactor::patches::tests::mounted_output_admission_concurrent_trackers_share_one_fixed_pool_without_overadmission ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (12/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_concurrent_trackers_share_one_fixed_pool_without_overadmission[0m
[32;1m       START[0m [         ] (13/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots[0m

running 1 test

thread 'component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots' (8540204) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:960:82:
[DEBUG] actual mounted direct-output transfer unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_plugin::component::reactor::patches::tests::after_output_transfer::{closure#0}
   3: <std::thread::local::LocalKey<core::cell::Cell<bool>>>::try_with::<semio_framework_plugin::component::reactor::patches::tests::after_output_transfer::{closure#0}, ()>
   4: <std::thread::local::LocalKey<core::cell::Cell<bool>>>::with::<semio_framework_plugin::component::reactor::patches::tests::after_output_transfer::{closure#0}, ()>
   5: semio_framework_plugin::component::reactor::patches::tests::after_output_transfer
   6: semio_framework_plugin::component::reactor::patches::drive_job_one
   7: <semio_framework_plugin::component::reactor::patches::PatchTracker>::drive_one
   8: semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#1}
   9: <semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#1} as core::ops::function::FnOnce<()>>::call_once
  10: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
  11: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#1}>, bool>
  12: ___rust_try
  13: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#1}>, bool>
  14: semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots
  15: semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#0}
  16: <semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots' (8540204) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1211:13:
[DEBUG] actual Pending receiver callback retains exact Ready
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#3}
   3: <semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#3} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#3}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#3}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#3}>, ()>
   8: semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots
   9: semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#0}
  10: <semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] live-output exact-lifetime=true admission-generation=1 producer-callback-roots=true occupied-busy-zero-refusal=true pending-callback-retained=true terminal=true
test component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.05s

[32;1m        PASS[0m [   0.063s] (13/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots[0m
[32;1m       START[0m [         ] (14/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_incomplete_producer_sources_preserve_remaining_owners[0m

running 1 test
[DEBUG] mounted-producer-incomplete reservation-retained=true tree-retained=true
test component::reactor::patches::tests::mounted_output_admission_incomplete_producer_sources_preserve_remaining_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (14/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_incomplete_producer_sources_preserve_remaining_owners[0m
[32;1m       START[0m [         ] (15/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box[0m

running 1 test

thread 'component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box' (8540280) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:964:79:
[DEBUG] actual mounted producer partial-step unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_plugin::component::reactor::patches::tests::after_producer_step
   3: <semio_framework_plugin::component::reactor::patches::PatchTracker>::drive_one
   4: semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box::{closure#1}
   5: <semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box::{closure#1} as core::ops::function::FnOnce<()>>::call_once
   6: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
   7: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box::{closure#1}>, bool>
   8: ___rust_try
   9: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box::{closure#1}>, bool>
  10: semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box
  11: semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box::{closure#0}
  12: <semio_framework_plugin::component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] mounted-producer-unwind exact-slot-and-box-retained=true
test component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.030s] (15/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box[0m
[32;1m       START[0m [         ] (16/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full[0m

running 1 test
[DEBUG] mounted-output-admission accepted=false tree-constructed=false shared-entries=64
test component::reactor::patches::tests::mounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (16/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full[0m
[32;1m       START[0m [         ] (17/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_path_advances_one_reconcile_opportunity_per_grant[0m

running 1 test
test component::reactor::patches::tests::mounted_path_advances_one_reconcile_opportunity_per_grant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (17/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_path_advances_one_reconcile_opportunity_per_grant[0m
[32;1m       START[0m [         ] (18/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner[0m

running 1 test

thread 'component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner' (8540327) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1609:90:
fixed unadmitted slot: ("2:queued", ComponentTree { root: BuiltNode { key: "root", component: Text(TextProps { value: Label("queued"), emphasize: None, data_attributes: None }), layout: Leaf(LeafLayout { width: Hug, height: Hug }), style: StyleSpec { variant: Solid, size: Md, density: Standard, tone: Neutral, emphasis: Regular }, activity: Idle, disabled: false, accessibility: AccessibilitySpec { label: None, description: None, live: Off, shortcut: None, hidden: false }, bindings: [], menu: None, children: BuiltChildren { len: 0 }, rejected_children: BuiltChildren { len: 0 } } })
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<u64, (alloc::string::String, semio_framework_ui_runtime::present::ComponentTree)>>::expect
   4: semio_framework_plugin::component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner
   5: semio_framework_plugin::component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner::{closure#0}
   6: <semio_framework_plugin::component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner ... FAILED

failures:

failures:
    component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.02s

[31;1m        FAIL[0m [   0.031s] (18/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner[0m
[32;1m       START[0m [         ] (19/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_settings_controls_publish_with_authored_fields[0m

running 1 test
test component::reactor::patches::tests::mounted_settings_controls_publish_with_authored_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.019s] (19/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_settings_controls_publish_with_authored_fields[0m
[32;1m       START[0m [         ] (20/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_sources_publish_every_window_and_panel_tree[0m

running 1 test
test component::reactor::patches::tests::mounted_sources_publish_every_window_and_panel_tree ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.028s] (20/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_sources_publish_every_window_and_panel_tree[0m
[32;1m       START[0m [         ] (21/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mone_active_surface_does_not_wait_behind_sixty_three_empty_slots_between_steps[0m

running 1 test
test component::reactor::patches::tests::one_active_surface_does_not_wait_behind_sixty_three_empty_slots_between_steps ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (21/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mone_active_surface_does_not_wait_behind_sixty_three_empty_slots_between_steps[0m
[32;1m       START[0m [         ] (22/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mpublished_owner_first_ack_rejects_early_stale_duplicate_wrong_instance_and_aba_without_authority_loss[0m

running 1 test
test component::reactor::patches::tests::published_owner_first_ack_rejects_early_stale_duplicate_wrong_instance_and_aba_without_authority_loss ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (22/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mpublished_owner_first_ack_rejects_early_stale_duplicate_wrong_instance_and_aba_without_authority_loss[0m
[32;1m       START[0m [         ] (23/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mresize_storm_coalesces_to_one_deferred_surface_owner[0m

running 1 test
test component::reactor::patches::tests::resize_storm_coalesces_to_one_deferred_surface_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (23/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mresize_storm_coalesces_to_one_deferred_surface_owner[0m
[32;1m       START[0m [         ] (24/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mstale_generation_fault_is_publicly_retrievable[0m

running 1 test
test component::reactor::patches::tests::stale_generation_fault_is_publicly_retrievable ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (24/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mstale_generation_fault_is_publicly_retrievable[0m
[32;1m       START[0m [         ] (25/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_rejected_advances_capacity_before_conversion[0m

running 1 test
test component::reactor::patches::tests::terminal_full_plus_matching_rejected_advances_capacity_before_conversion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.024s] (25/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_rejected_advances_capacity_before_conversion[0m
[32;1m       START[0m [         ] (26/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_surface_advances_capacity_before_conversion[0m

running 1 test
test component::reactor::patches::tests::terminal_full_plus_matching_surface_advances_capacity_before_conversion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.024s] (26/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_surface_advances_capacity_before_conversion[0m
[32;1m       START[0m [         ] (27/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_unadmitted_advances_capacity_before_conversion[0m

running 1 test
test component::reactor::patches::tests::terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.023s] (27/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_unadmitted_advances_capacity_before_conversion[0m
[32;1m       START[0m [         ] (28/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation[0m

running 1 test
test component::reactor::patches::tests::terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.020s] (28/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation[0m
[32;1m       START[0m [         ] (29/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed[0m

running 1 test
test component::reactor::patches::tests::terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.021s] (29/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed[0m
[32;1m       START[0m [         ] (30/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mtracker_initialization_fits_the_component_stack_budget[0m

running 1 test
test component::reactor::patches::tests::tracker_initialization_fits_the_component_stack_budget ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (30/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mtracker_initialization_fits_the_component_stack_budget[0m
────────────
[31;1m     Summary[0m [   0.646s] [1m30[0m tests run: [1m29[0m [32;1mpassed[0m, [1m1[0m [31;1mfailed[0m, [1m492[0m [33;1mskipped[0m
[31;1m        FAIL[0m [   0.031s] (18/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner[0m
[31;1merror[0m: test run failed
Warning: command "bun 📜️script.ts test exhaustive component::reactor::patches::tests:: --no-fail-fast -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-plugin failed

Failed tasks:

- @semio-tech/framework-plugin:test

Hint: run the command with --verbose for more details.


```
