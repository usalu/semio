# UI Host Input R10 Native Mixed RED

Actual canonical exhaustive/no-fail-fast cohort: **15 executed, 6 passed, 9 failed, 62 skipped**, 0.200s, Nx1. All selected tests ran. Fresh selected capture [here](./📓️ui-host-input-r10-selected-inputs-2026-08-28.md). The filter also selected one existing input-generation regression, so the actual count is15, not the planned14.

- Original admission5: all5 still fail at the intended existing defects (constructor14336B, full-refusal generation257/256, two MAX→0 wraps, terminal retaining14336B).
- Root5: one equal-counter/move/address-reuse law passes; four fail at actual existing constructor backing/allocation assertions. Their later assertions are not all reached, so no complete concurrency/MAX/layout credit is inferred.
- Writer4: all4 pass, including actual13 UTF-8 vectors and nine actual partial-copy panics caught with original source and buffer outside the callback. The close law currently checks only inspection counters and capacity, not actual byte retirement; this is a known source defect, not bytewise-close proof. A new actual-byte scrub law is required before that claim.
- Existing input_generation_increases_monotonically_and_survives_drain passes.

No funded queue, outer source/facade, all-receiver commit, real callback wall-clock timing or whole UI-host GREEN is claimed. No original failure was excluded or weakened.

## Actual Executed Roster

- FAILED enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing
- FAILED enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation
- FAILED enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap
- FAILED enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue
- FAILED enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing
- FAILED enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots
- ok enqueue::input_root_tests::input_root_native_equal_counters_moves_and_address_reuse_never_reuse_identity
- FAILED enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind
- FAILED enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout
- FAILED enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation
- ok enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
- ok enqueue::input_writer_tests::input_writer_native_one_byte_validation_copy_and_seal_are_separate
- ok enqueue::input_writer_tests::input_writer_native_refusal_and_bytewise_close_retain_physical_backing
- ok enqueue::input_writer_tests::input_writer_native_utf8_matches_std_without_a_final_scan
- ok enqueue::tests::input_generation_increases_monotonically_and_survives_drain

## Complete Captured Tool Output

```text
> nx run @semio-tech/ui-host-rs:test --args=exhaustive --lib input_ -- --nocapture

> bun ./📜️script.ts test exhaustive --lib input_ -- --nocapture

────────────
 Nextest run ID 3d7b46bc-4cd3-4efc-96ec-119f68a5cac4 with nextest profile: exhaustive
    Starting 15 tests across 1 binary (62 tests skipped)
       START [         ] ( 1/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing

running 1 test
[DEBUG] event-queue-constructor capacity=256 slot-bytes=56 physical=14336

thread 'enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing' (9611251) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:23:5:
assertion `left == right` failed
  left: 14336
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<u64, u64>
   4: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing
   5: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing ... FAILED

failures:

failures:
    enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        FAIL [   0.015s] ( 1/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing
       START [         ] ( 2/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation

running 1 test

thread 'enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation' (9611256) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:38:5:
assertion `left == right` failed
  left: InputGeneration(257)
 right: InputGeneration(256)
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<semio_framework_ui_host::enqueue::InputGeneration, semio_framework_ui_host::enqueue::InputGeneration>
   4: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation
   5: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation ... FAILED

failures:

failures:
    enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        FAIL [   0.015s] ( 2/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation
       START [         ] ( 3/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap

running 1 test

thread 'enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap' (9611260) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:49:5:
assertion `left == right` failed
  left: InputGeneration(0)
 right: InputGeneration(18446744073709551615)
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<semio_framework_ui_host::enqueue::InputGeneration, semio_framework_ui_host::enqueue::InputGeneration>
   4: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap
   5: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap ... FAILED

failures:

failures:
    enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        FAIL [   0.014s] ( 3/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap
       START [         ] ( 4/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue

running 1 test

thread 'enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue' (9611264) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:62:5:
assertion `left == right` failed
  left: InputGeneration(0)
 right: InputGeneration(18446744073709551615)
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<semio_framework_ui_host::enqueue::InputGeneration, semio_framework_ui_host::enqueue::InputGeneration>
   4: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue
   5: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue ... FAILED

failures:

failures:
    enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        FAIL [   0.015s] ( 4/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue
       START [         ] ( 5/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing

running 1 test
[DEBUG] event-queue-terminal logical=8 original-payload-capacity=64 retained-queue-backing=14336 terminal=true

thread 'enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing' (9611267) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:86:5:
assertion failed: !terminal || physical == 0
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing
   4: semio_framework_ui_host::enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing::{closure#0}
   5: <semio_framework_ui_host::enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing ... FAILED

failures:

failures:
    enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        FAIL [   0.014s] ( 5/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing
       START [         ] ( 6/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots

running 1 test

thread 'enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots' (9611271) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:145:9:
assertion `left == right` failed
  left: 256
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots
   5: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots ... FAILED

failures:

failures:
    enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        FAIL [   0.014s] ( 6/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots
       START [         ] ( 7/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_equal_counters_moves_and_address_reuse_never_reuse_identity

running 1 test
test enqueue::input_root_tests::input_root_native_equal_counters_moves_and_address_reuse_never_reuse_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        PASS [   0.011s] ( 7/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_equal_counters_moves_and_address_reuse_never_reuse_identity
       START [         ] ( 8/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind

running 1 test

thread 'enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind' (9611286) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:102:9:
[DEBUG] after actual queue root installation
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind
   9: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind' (9611286) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:114:5:
assertion `left == right` failed
  left: 256
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind
   5: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind ... FAILED

failures:

failures:
    enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        FAIL [   0.014s] ( 8/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind
       START [         ] ( 9/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout

running 1 test

thread 'enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout' (9611292) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:175:9:
assertion `left == right` failed
  left: 256
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout
   5: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout ... FAILED

failures:

failures:
    enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        FAIL [   0.013s] ( 9/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout
       START [         ] (10/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation

running 1 test

thread 'enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation' (9611295) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:60:9:
assertion `left == right` failed: root admission must not allocate queue backing
  left: 1
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation
   5: semio_framework_ui_host::enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation ... FAILED

failures:

failures:
    enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        FAIL [   0.013s] (10/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation
       START [         ] (11/15) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing

running 1 test

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9611298) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
[DEBUG] after actual partial input byte copy
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
   9: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9611298) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
[DEBUG] after actual partial input byte copy
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
   9: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9611298) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
[DEBUG] after actual partial input byte copy
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
   9: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9611298) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
[DEBUG] after actual partial input byte copy
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
   9: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9611298) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
[DEBUG] after actual partial input byte copy
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
   9: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9611298) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
[DEBUG] after actual partial input byte copy
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
   9: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9611298) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
[DEBUG] after actual partial input byte copy
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
   9: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9611298) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
[DEBUG] after actual partial input byte copy
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
   9: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9611298) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
[DEBUG] after actual partial input byte copy
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
   3: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}>, ()>
   8: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
   9: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0}
  10: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        PASS [   0.015s] (11/15) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
       START [         ] (12/15) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_one_byte_validation_copy_and_seal_are_separate

running 1 test
test enqueue::input_writer_tests::input_writer_native_one_byte_validation_copy_and_seal_are_separate ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        PASS [   0.011s] (12/15) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_one_byte_validation_copy_and_seal_are_separate
       START [         ] (13/15) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_refusal_and_bytewise_close_retain_physical_backing

running 1 test
test enqueue::input_writer_tests::input_writer_native_refusal_and_bytewise_close_retain_physical_backing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        PASS [   0.011s] (13/15) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_refusal_and_bytewise_close_retain_physical_backing
       START [         ] (14/15) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_utf8_matches_std_without_a_final_scan

running 1 test
test enqueue::input_writer_tests::input_writer_native_utf8_matches_std_without_a_final_scan ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        PASS [   0.011s] (14/15) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_utf8_matches_std_without_a_final_scan
       START [         ] (15/15) semio-framework-ui-host enqueue::tests::input_generation_increases_monotonically_and_survives_drain

running 1 test
test enqueue::tests::input_generation_increases_monotonically_and_survives_drain ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s

        PASS [   0.013s] (15/15) semio-framework-ui-host enqueue::tests::input_generation_increases_monotonically_and_survives_drain
────────────
     Summary [   0.200s] 15 tests run: 6 passed, 9 failed, 62 skipped
        FAIL [   0.015s] ( 1/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing
        FAIL [   0.015s] ( 2/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation
        FAIL [   0.014s] ( 3/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap
        FAIL [   0.015s] ( 4/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue
        FAIL [   0.014s] ( 5/15) semio-framework-ui-host enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing
        FAIL [   0.014s] ( 6/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots
        FAIL [   0.014s] ( 8/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind
        FAIL [   0.013s] ( 9/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout
        FAIL [   0.013s] (10/15) semio-framework-ui-host enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation
error: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib input_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-host-rs failed

Failed tasks:

- @semio-tech/ui-host-rs:test

Hint: run the command with --verbose for more details.
```

