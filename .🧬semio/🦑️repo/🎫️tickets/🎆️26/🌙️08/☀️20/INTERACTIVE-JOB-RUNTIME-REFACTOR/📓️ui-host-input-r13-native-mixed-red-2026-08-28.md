# UI Host Input R13 Native Mixed RED

Actual canonical exhaustive/no-fail-fast result: **17 executed, 8 passed, 9 failed, 62 skipped**, 0.336s, Nx1. All17 selected cases ran; full actual44,102-character tool output below. Fresh selected capture [here](./📓️ui-host-input-r13-selected-inputs-2026-08-28.md).

Writer6 now all pass, including the two actual-byte scrub laws that failed in R12. Sealed String ownership first converts to Vec under a separately sufficient descriptor grant, preserving exact pointer/capacity and no byte mutation in that phase. Subsequent granted prefixes are actually zeroed; suffix bytes remain unchanged. Allocation release remains a separate physical-capacity grant. Original four validation/copy/capacity/unwind tests still pass. This is private-buffer evidence, not installed source, resident funding, callback or all-receiver publication proof.

Original admission5 still all fail at the original five assertions. Root5 remains one identity PASS/four actual backing/allocation RED; existing ordinary input-generation test passes. No constructor/enqueue behavior or limits changed. The prior R10 counter-only close claim is superseded only by this private-buffer repair and actual R12→R13 test sequence.

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
- ok enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release
- ok enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer
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
 Nextest run ID 7bf1c92f-8e4a-477c-9bfe-77303da60a2a with nextest profile: exhaustive
    Starting 17 tests across 1 binary (62 tests skipped)
       START [         ] ( 1/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing

running 1 test
[DEBUG] event-queue-constructor capacity=256 slot-bytes=56 physical=14336

thread 'enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing' (9637414) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:23:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        FAIL [   0.017s] ( 1/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing
       START [         ] ( 2/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation

running 1 test

thread 'enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation' (9637420) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:38:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        FAIL [   0.014s] ( 2/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation
       START [         ] ( 3/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap

running 1 test

thread 'enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap' (9637423) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:49:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        FAIL [   0.015s] ( 3/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap
       START [         ] ( 4/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue

running 1 test

thread 'enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue' (9637426) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:62:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        FAIL [   0.014s] ( 4/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue
       START [         ] ( 5/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing

running 1 test
[DEBUG] event-queue-terminal logical=8 original-payload-capacity=64 retained-queue-backing=14336 terminal=true

thread 'enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing' (9637439) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:86:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        FAIL [   0.015s] ( 5/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing
       START [         ] ( 6/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots

running 1 test

thread 'enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots' (9637443) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:145:9:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.01s

        FAIL [   0.046s] ( 6/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots
       START [         ] ( 7/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_equal_counters_moves_and_address_reuse_never_reuse_identity

running 1 test
test enqueue::input_root_tests::input_root_native_equal_counters_moves_and_address_reuse_never_reuse_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.01s

        PASS [   0.077s] ( 7/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_equal_counters_moves_and_address_reuse_never_reuse_identity
       START [         ] ( 8/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind

running 1 test

thread 'enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind' (9637478) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:102:9:
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

thread 'enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind' (9637478) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:114:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        FAIL [   0.018s] ( 8/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind
       START [         ] ( 9/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout

running 1 test

thread 'enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout' (9637483) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:175:9:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        FAIL [   0.015s] ( 9/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout
       START [         ] (10/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation

running 1 test

thread 'enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation' (9637490) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs:60:9:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        FAIL [   0.014s] (10/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation
       START [         ] (11/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release

running 1 test
test enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        PASS [   0.012s] (11/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release
       START [         ] (12/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer

running 1 test
test enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        PASS [   0.012s] (12/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer
       START [         ] (13/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing

running 1 test

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9637499) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
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

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9637499) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
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

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9637499) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
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

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9637499) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
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

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9637499) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
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

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9637499) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
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

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9637499) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
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

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9637499) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
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

thread 'enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing' (9637499) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:147:13:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        PASS [   0.016s] (13/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing
       START [         ] (14/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_one_byte_validation_copy_and_seal_are_separate

running 1 test
test enqueue::input_writer_tests::input_writer_native_one_byte_validation_copy_and_seal_are_separate ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        PASS [   0.012s] (14/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_one_byte_validation_copy_and_seal_are_separate
       START [         ] (15/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_refusal_and_bytewise_close_retain_physical_backing

running 1 test
test enqueue::input_writer_tests::input_writer_native_refusal_and_bytewise_close_retain_physical_backing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        PASS [   0.013s] (15/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_refusal_and_bytewise_close_retain_physical_backing
       START [         ] (16/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_utf8_matches_std_without_a_final_scan

running 1 test
test enqueue::input_writer_tests::input_writer_native_utf8_matches_std_without_a_final_scan ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        PASS [   0.012s] (16/17) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_utf8_matches_std_without_a_final_scan
       START [         ] (17/17) semio-framework-ui-host enqueue::tests::input_generation_increases_monotonically_and_survives_drain

running 1 test
test enqueue::tests::input_generation_increases_monotonically_and_survives_drain ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        PASS [   0.013s] (17/17) semio-framework-ui-host enqueue::tests::input_generation_increases_monotonically_and_survives_drain
────────────
     Summary [   0.336s] 17 tests run: 8 passed, 9 failed, 62 skipped
        FAIL [   0.017s] ( 1/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing
        FAIL [   0.014s] ( 2/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation
        FAIL [   0.015s] ( 3/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap
        FAIL [   0.014s] ( 4/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue
        FAIL [   0.015s] ( 5/17) semio-framework-ui-host enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing
        FAIL [   0.046s] ( 6/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_concurrent_single_attempts_preserve_busy_roots
        FAIL [   0.018s] ( 8/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_installed_owner_survives_actual_admission_unwind
        FAIL [   0.015s] ( 9/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_permanent_exhaustion_and_exact_fixed_layout
        FAIL [   0.014s] (10/17) semio-framework-ui-host enqueue::input_root_tests::input_root_native_vectors_refuse_before_mint_or_allocation
error: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib input_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-host-rs failed

Failed tasks:

- @semio-tech/ui-host-rs:test

Hint: run the command with --verbose for more details.
```

