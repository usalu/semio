# Input Actual-Byte Scrub R12 Native RED

Actual canonical exhaustive/no-fail-fast result: **0 passed, 2 failed, 77 skipped**, 0.077s, Nx1. Fresh selected capture [here](./📓️ui-host-scrub-r12-selected-inputs-2026-08-28.md). Both tests performed exact owner cleanup before the failing assertions; no abort.

The unsealed buffer kept byte97 at the first one-byte retirement frontier, where the neutral oracle requires0 and unchanged suffix. The sealed-text law returned Inspected under a short descriptor grant instead of Blocked, confirming no separate ownership conversion phase. This establishes real semantic failures beyond the prior counter-only law. Production close was unchanged for this snapshot. No queue/root failure was repaired or reclassified.

```text
> nx run @semio-tech/ui-host-rs:test --args=exhaustive --lib input_writer_native_close_scrubs_ -- --nocapture

> bun ./📜️script.ts test exhaustive --lib input_writer_native_close_scrubs_ -- --nocapture

────────────
 Nextest run ID 06ea3746-4292-424f-86fd-59ba3944378f with nextest profile: exhaustive
    Starting 2 tests across 1 binary (77 tests skipped)
       START [         ] (1/2) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release

running 1 test

thread 'enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release' (9628659) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:179:9:
assertion `left == right` failed: actual bytes at close frontier 1
  left: [97, 0, 195, 169, 240, 159, 167, 170]
 right: [0, 0, 195, 169, 240, 159, 167, 170]
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<alloc::vec::Vec<u8>, alloc::vec::Vec<u8>>
   4: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release
   5: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release ... FAILED

failures:

failures:
    enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.01s

        FAIL [   0.051s] (1/2) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release
       START [         ] (2/2) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer

running 1 test

thread 'enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer' (9628662) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs:207:5:
assertion `left == right` failed
  left: Inspected
 right: Blocked
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<semio_framework_ui_host::enqueue::input_writer::InputWriteKind, semio_framework_ui_host::enqueue::input_writer::InputWriteKind>
   4: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer
   5: semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer::{closure#0}
   6: <semio_framework_ui_host::enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer ... FAILED

failures:

failures:
    enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.00s

        FAIL [   0.015s] (2/2) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer
────────────
     Summary [   0.077s] 2 tests run: 0 passed, 2 failed, 77 skipped
        FAIL [   0.051s] (1/2) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release
        FAIL [   0.015s] (2/2) semio-framework-ui-host enqueue::input_writer_tests::input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer
error: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib input_writer_native_close_scrubs_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-host-rs failed

Failed tasks:

- @semio-tech/ui-host-rs:test

Hint: run the command with --verbose for more details.
```

