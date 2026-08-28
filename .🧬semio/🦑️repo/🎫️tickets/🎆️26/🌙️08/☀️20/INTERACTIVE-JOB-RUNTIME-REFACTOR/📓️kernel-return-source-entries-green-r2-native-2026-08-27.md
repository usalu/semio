# Kernel Return Source Entries R2

Actual canonical `@semio-tech/framework-rs:test-wire-retirement-native --args='--lib return_source_entries_ -- --nocapture'`, shared target/profile, SEMIO_COVERAGE=0. Exit 0: four tests passed, 261 skipped, 0.138 seconds. This covers the four source-entry owner laws only, not a mounted return producer or Plugin guest runtime.

Raw output: `🧪️member-kernel-return-source-entries-green-r2-native-2026-08-27.txt`.

```text

> nx run @semio-tech/framework-rs:test-wire-retirement-native --args=--lib return_source_entries_ -- --nocapture

> bun ./📜️script.ts test-wire-retirement-native --lib return_source_entries_ -- --nocapture

warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID 72296e4f-75a6-4895-b95f-3a8504689142 with nextest profile: fundamental
    Starting 4 tests across 1 binary (261 tests skipped)
       START [         ] (1/4) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots

running 1 test

thread 'manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots' (7423038) panicked at 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🧪️component.rs:83:79:
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

thread 'manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots' (7423038) panicked at 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🧪️component.rs:83:79:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 264 filtered out; finished in 0.03s

        PASS [   0.052s] (1/4) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots
       START [         ] (2/4) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_incremental_freeze_preserves_exact_fifo_and_handoff

running 1 test
test manifest::kernel::return_source_entries_tests::return_source_entries_incremental_freeze_preserves_exact_fifo_and_handoff ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 264 filtered out; finished in 0.00s

        PASS [   0.028s] (2/4) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_incremental_freeze_preserves_exact_fifo_and_handoff
       START [         ] (3/4) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_over_admission_reports_and_retains_exact_empty_backing

running 1 test
test manifest::kernel::return_source_entries_tests::return_source_entries_over_admission_reports_and_retains_exact_empty_backing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 264 filtered out; finished in 0.00s

        PASS [   0.031s] (3/4) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_over_admission_reports_and_retains_exact_empty_backing
       START [         ] (4/4) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_reserve_before_placement_and_preserve_original_allocation

running 1 test
test manifest::kernel::return_source_entries_tests::return_source_entries_reserve_before_placement_and_preserve_original_allocation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 264 filtered out; finished in 0.00s

        PASS [   0.020s] (4/4) semio-framework manifest::kernel::return_source_entries_tests::return_source_entries_reserve_before_placement_and_preserve_original_allocation
────────────
     Summary [   0.138s] 4 tests run: 4 passed, 261 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-byS8Uj



 NX   Successfully ran target test-wire-retirement-native for project @semio-tech/framework-rs



 NX   Nx detected a flaky task

  @semio-tech/framework-rs:test-wire-retirement-native

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```
