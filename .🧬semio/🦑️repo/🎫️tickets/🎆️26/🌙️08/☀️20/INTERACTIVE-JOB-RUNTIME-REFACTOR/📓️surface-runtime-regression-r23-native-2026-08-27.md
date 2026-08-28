# Runtime Regression R23 RED

Actual full runtime attempt excluding only the existing intentional inline-census RED:54 passed,1 failed,38 not run,1 excluded. New separate binding and Component copy fields exceeded the existing48KiB cursor footprint assertion. The correction uses one mutually-exclusive tagged retained field owner, not a raised limit, new heap allocation or weakened assertion. R24 verifies that correction.

```text
    
  stderr ───

    thread 'reconcile::tests::fixed_runtime_owners_keep_bounded_state_off_the_stack' (5430906) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:3586:9:
    assertion failed: size_of::<SurfaceReconcileCursor>() <= 48 * 1_024
    stack backtrace:
       0: __rustc::rust_begin_unwind
       1: core::panicking::panic_fmt
       2: core::panicking::panic
       3: semio_framework_ui_runtime::reconcile::tests::fixed_runtime_owners_keep_bounded_state_off_the_stack
       4: semio_framework_ui_runtime::reconcile::tests::fixed_runtime_owners_keep_bounded_state_off_the_stack::{closure#0}
       5: <semio_framework_ui_runtime::reconcile::tests::fixed_runtime_owners_keep_bounded_state_off_the_stack::{closure#0} as core::ops::function::FnOnce<()>>::call_once
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

  Cancelling due to test failure: 6 tests still running
────────────
     Summary [   0.585s] 55/93 tests run: 54 passed, 1 failed, 1 skipped
        FAIL [   0.059s] (49/93) semio-framework-ui-runtime reconcile::tests::fixed_runtime_owners_keep_bounded_state_off_the_stack
warning: 38/93 tests were not run due to test failure (run with --no-fail-fast to run all tests, or run with --max-fail)
error: test run failed
Warning: command "bun ./📜️script.ts test --lib -- --skip surface_ownership_inline_fields_do_not_allocate_a_second_owner" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.


```
