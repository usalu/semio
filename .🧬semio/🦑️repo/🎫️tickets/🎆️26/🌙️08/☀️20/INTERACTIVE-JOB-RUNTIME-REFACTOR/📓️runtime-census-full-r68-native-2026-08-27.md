# Runtime Full R68

Canonical exhaustive runtime run, `SEMIO_COVERAGE=0`, unchanged shared target/profile, no exclusions. Actual exit 1: 99 passed, 1 failed, 15 not run of 115, 2.260 seconds. All tests before the transaction group passed, including the original inline census and canonical resident/read-pressure gates. This is not a full-suite pass.

The first remaining failure is the test-only transaction oracle `a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch`, expected one patch and received zero. No assertion or production limit was changed. The module remains cfg(test)-gated; live PatchTracker admission remains the production priority. Cause is not yet diagnosed.

Raw output: `🧪️member-runtime-census-full-r68-native-2026-08-27.txt`.

```text
running 1 test
test tracking::tests::n_notifications_of_one_surface_coalesce_to_one_dirty_mark ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s

        PASS [   0.018s] ( 95/115) semio-framework-ui-runtime tracking::tests::n_notifications_of_one_surface_coalesce_to_one_dirty_mark
       START [         ] ( 96/115) semio-framework-ui-runtime tracking::tests::nested_present_scopes_attribute_reads_to_the_right_surface

running 1 test
test tracking::tests::nested_present_scopes_attribute_reads_to_the_right_surface ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s

        PASS [   0.016s] ( 96/115) semio-framework-ui-runtime tracking::tests::nested_present_scopes_attribute_reads_to_the_right_surface
       START [         ] ( 97/115) semio-framework-ui-runtime tracking::tests::presenter_reading_a_not_b_wakes_only_on_a

running 1 test
test tracking::tests::presenter_reading_a_not_b_wakes_only_on_a ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s

        PASS [   0.015s] ( 97/115) semio-framework-ui-runtime tracking::tests::presenter_reading_a_not_b_wakes_only_on_a
       START [         ] ( 98/115) semio-framework-ui-runtime tracking::tests::reads_outside_a_present_scope_are_not_recorded

running 1 test
test tracking::tests::reads_outside_a_present_scope_are_not_recorded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s

        PASS [   0.017s] ( 98/115) semio-framework-ui-runtime tracking::tests::reads_outside_a_present_scope_are_not_recorded
       START [         ] ( 99/115) semio-framework-ui-runtime tracking::tests::stale_edge_disappears_after_next_present_without_the_read

running 1 test
test tracking::tests::stale_edge_disappears_after_next_present_without_the_read ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s

        PASS [   0.018s] ( 99/115) semio-framework-ui-runtime tracking::tests::stale_edge_disappears_after_next_present_without_the_read
       START [         ] (100/115) semio-framework-ui-runtime transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch

running 1 test

thread 'transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch' (7387079) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs:902:9:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.01s

        FAIL [   0.018s] (100/115) semio-framework-ui-runtime transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch
  Cancelling due to test failure: 
────────────
     Summary [   2.260s] 100/115 tests run: 99 passed, 1 failed, 0 skipped
        FAIL [   0.018s] (100/115) semio-framework-ui-runtime transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch
warning: 15/115 tests were not run due to test failure (run with --no-fail-fast to run all tests, or run with --max-fail)
error: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.


```
