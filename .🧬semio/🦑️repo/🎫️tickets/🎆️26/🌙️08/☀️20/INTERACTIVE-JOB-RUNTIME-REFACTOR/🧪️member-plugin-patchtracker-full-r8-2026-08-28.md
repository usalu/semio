
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
[32;1m Nextest run[0m ID [1m70c75831-a2e3-42c6-9819-209a0b43f65b[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m30[0m tests across [1m1[0m binary ([1m492[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] ( 1/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mactor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot[0m

running 1 test
test component::reactor::patches::tests::actor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.032s] ( 1/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mactor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot[0m
[32;1m       START[0m [         ] ( 2/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mcap_plus_one_returns_the_exact_tree_owner[0m

running 1 test
test component::reactor::patches::tests::cap_plus_one_returns_the_exact_tree_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.026s] ( 2/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mcap_plus_one_returns_the_exact_tree_owner[0m
[32;1m       START[0m [         ] ( 3/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mclose_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish[0m

running 1 test
test component::reactor::patches::tests::close_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.070s] ( 3/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mclose_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish[0m
[32;1m       START[0m [         ] ( 4/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1meffects_publish_in_admission_order_even_when_later_tree_finishes_first[0m

running 1 test
test component::reactor::patches::tests::effects_publish_in_admission_order_even_when_later_tree_finishes_first ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.035s] ( 4/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1meffects_publish_in_admission_order_even_when_later_tree_finishes_first[0m
[32;1m       START[0m [         ] ( 5/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mgeneration_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation[0m

running 1 test
test component::reactor::patches::tests::generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.026s] ( 5/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mgeneration_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation[0m
[32;1m       START[0m [         ] ( 6/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_publishes_every_section_beyond_thirty_two_nodes[0m

running 1 test
test component::reactor::patches::tests::mounted_catalogue_publishes_every_section_beyond_thirty_two_nodes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.04s

[32;1m        PASS[0m [   0.053s] ( 6/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_publishes_every_section_beyond_thirty_two_nodes[0m
[32;1m       START[0m [         ] ( 7/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_reports_producer_failure_once_before_cleanup[0m

running 1 test
test component::reactor::patches::tests::mounted_catalogue_reports_producer_failure_once_before_cleanup ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] ( 7/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_reports_producer_failure_once_before_cleanup[0m
[32;1m       START[0m [         ] ( 8/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_reports_reconcile_capacity_without_leaking_owners[0m

running 1 test
test component::reactor::patches::tests::mounted_catalogue_reports_reconcile_capacity_without_leaking_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.09s

[32;1m        PASS[0m [   0.115s] ( 8/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_catalogue_reports_reconcile_capacity_without_leaking_owners[0m
[32;1m       START[0m [         ] ( 9/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_document_tree_publishes_nested_interactive_rows[0m

running 1 test
test component::reactor::patches::tests::mounted_document_tree_publishes_nested_interactive_rows ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.045s] ( 9/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_document_tree_publishes_nested_interactive_rows[0m
[32;1m       START[0m [         ] (10/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_cancel_and_drop_keep_the_original_close_generation[0m

running 1 test
[DEBUG] mounted-uncommitted-close drop=false generation=1 preserved=true revision=0 terminal=true
[DEBUG] mounted-uncommitted-close drop=true generation=1 preserved=true revision=0 terminal=true
test component::reactor::patches::tests::mounted_output_admission_cancel_and_drop_keep_the_original_close_generation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (10/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_cancel_and_drop_keep_the_original_close_generation[0m
[32;1m       START[0m [         ] (11/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_close_waits_for_the_original_uncommitted_grant[0m

running 1 test
[DEBUG] live-output close-waits-for-original-grant=true rejected-tree-pointer-preserved=true
test component::reactor::patches::tests::mounted_output_admission_close_waits_for_the_original_uncommitted_grant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (11/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_close_waits_for_the_original_uncommitted_grant[0m
[32;1m       START[0m [         ] (12/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_concurrent_trackers_share_one_fixed_pool_without_overadmission[0m

running 1 test
[DEBUG] live-output same-process-workers=2 preoccupied=63 accepted=1 exact-refusal=true full64-restored=true
test component::reactor::patches::tests::mounted_output_admission_concurrent_trackers_share_one_fixed_pool_without_overadmission ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.028s] (12/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_concurrent_trackers_share_one_fixed_pool_without_overadmission[0m
[32;1m       START[0m [         ] (13/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots[0m

running 1 test

thread 'component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots' (8702650) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:960:82:
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

thread 'component::reactor::patches::tests::mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots' (8702650) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1211:13:
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

[32;1m        PASS[0m [   0.068s] (13/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots[0m
[32;1m       START[0m [         ] (14/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_incomplete_producer_sources_preserve_remaining_owners[0m

running 1 test
[DEBUG] mounted-producer-incomplete reservation-retained=true tree-retained=true
test component::reactor::patches::tests::mounted_output_admission_incomplete_producer_sources_preserve_remaining_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (14/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_incomplete_producer_sources_preserve_remaining_owners[0m
[32;1m       START[0m [         ] (15/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box[0m

running 1 test

thread 'component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box' (8702657) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:964:79:
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

[32;1m        PASS[0m [   0.034s] (15/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box[0m
[32;1m       START[0m [         ] (16/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full[0m

running 1 test
[DEBUG] mounted-output-admission accepted=false tree-constructed=false shared-entries=64
test component::reactor::patches::tests::mounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.034s] (16/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full[0m
[32;1m       START[0m [         ] (17/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_path_advances_one_reconcile_opportunity_per_grant[0m

running 1 test
test component::reactor::patches::tests::mounted_path_advances_one_reconcile_opportunity_per_grant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.019s] (17/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_path_advances_one_reconcile_opportunity_per_grant[0m
[32;1m       START[0m [         ] (18/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner[0m

running 1 test
[DEBUG] mounted-resident-capacity fixed=534368 per=8388608 accepted=3 full=25700192 cap-plus-one=false exact-refusal=true restored=534368
test component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.021s] (18/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner[0m
[32;1m       START[0m [         ] (19/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_settings_controls_publish_with_authored_fields[0m

running 1 test
test component::reactor::patches::tests::mounted_settings_controls_publish_with_authored_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.021s] (19/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mmounted_settings_controls_publish_with_authored_fields[0m
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

[32;1m        PASS[0m [   0.014s] (23/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mresize_storm_coalesces_to_one_deferred_surface_owner[0m
[32;1m       START[0m [         ] (24/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mstale_generation_fault_is_publicly_retrievable[0m

running 1 test
test component::reactor::patches::tests::stale_generation_fault_is_publicly_retrievable ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (24/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mstale_generation_fault_is_publicly_retrievable[0m
[32;1m       START[0m [         ] (25/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_rejected_advances_capacity_before_conversion[0m

running 1 test
test component::reactor::patches::tests::terminal_full_plus_matching_rejected_advances_capacity_before_conversion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.025s] (25/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_rejected_advances_capacity_before_conversion[0m
[32;1m       START[0m [         ] (26/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_surface_advances_capacity_before_conversion[0m

running 1 test
test component::reactor::patches::tests::terminal_full_plus_matching_surface_advances_capacity_before_conversion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.026s] (26/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_surface_advances_capacity_before_conversion[0m
[32;1m       START[0m [         ] (27/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_unadmitted_advances_capacity_before_conversion[0m

running 1 test
test component::reactor::patches::tests::terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.028s] (27/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_full_plus_matching_unadmitted_advances_capacity_before_conversion[0m
[32;1m       START[0m [         ] (28/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation[0m

running 1 test
test component::reactor::patches::tests::terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.024s] (28/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation[0m
[32;1m       START[0m [         ] (29/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed[0m

running 1 test
test component::reactor::patches::tests::terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.026s] (29/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mterminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed[0m
[32;1m       START[0m [         ] (30/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mtracker_initialization_fits_the_component_stack_budget[0m

running 1 test
test component::reactor::patches::tests::tracker_initialization_fits_the_component_stack_budget ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (30/30) [35;1msemio-framework-plugin[0m [36mcomponent::reactor::patches::tests[0m[36m::[0m[34;1mtracker_initialization_fits_the_component_stack_budget[0m
────────────
[32;1m     Summary[0m [   0.933s] [1m30[0m tests run: [1m30[0m [32;1mpassed[0m, [1m492[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-IhiiJR[0m



 NX   Successfully ran target test for project @semio-tech/framework-plugin



 NX   Nx detected a flaky task

  @semio-tech/framework-plugin:test

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks

