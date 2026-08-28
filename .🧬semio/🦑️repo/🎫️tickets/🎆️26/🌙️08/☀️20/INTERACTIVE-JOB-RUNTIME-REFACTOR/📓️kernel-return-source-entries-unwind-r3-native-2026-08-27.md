# Kernel Source Entries Actual Callback Unwind R3

Actual canonical common-framework filter `return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots`: exit 0, one passed, 264 skipped, 0.038 seconds. This strengthens the earlier R2 fixture: actual reserved payload placement/partial freeze occurs inside the panic callback, while ownership roots stay outside and are drained afterward. Production source unchanged by this fixture revision.

Raw `🧪️member-kernel-return-source-entries-unwind-r3-native-2026-08-27.txt`.

```text

> nx run @semio-tech/framework-rs:test-wire-retirement-native --args=--lib return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots -- --nocapture

> bun ./📜️script.ts test-wire-retirement-native --lib return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots -- --nocapture

warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID b14bc3c6-da3f-46ca-bbf8-1acd85757c3d with nextest profile: fundamental
    Starting 1 test across 1 binary (264 tests skipped)
       START [         ] (1/1) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots

running 1 test

thread 'manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots' (7563132) panicked at 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🧪️component.rs:94:13:
fixture producer failed after owned placement
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}
   3: <semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}>, ()>
   8: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots
   9: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}
  10: <semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots' (7563132) panicked at 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🧪️component.rs:94:13:
fixture producer failed after owned placement
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}
   3: <semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}>, ()>
   8: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots
   9: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}
  10: <semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 264 filtered out; finished in 0.02s

        PASS [   0.035s] (1/1) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots
────────────
     Summary [   0.038s] 1 test run: 1 passed, 264 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-NuHLWX



 NX   Successfully ran target test-wire-retirement-native for project @semio-tech/framework-rs



```
