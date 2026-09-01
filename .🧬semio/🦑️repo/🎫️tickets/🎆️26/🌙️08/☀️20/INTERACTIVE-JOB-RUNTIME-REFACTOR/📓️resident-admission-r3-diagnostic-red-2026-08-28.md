# Resident Admission R3: First-Access Diagnostic RED

Canonical no-argument target, explicit exhaustive profile, existing retained target and jobs2. Fresh 20 selected inputs are in [capture](./📓️resident-admission-r3-selected-inputs-2026-08-28.md); this is not a complete atomic dependency closure. Production stayed `6748e3961f82178ef59c9bb8ccc89117b12ab0c2014759bf5969353fa170ed83`; tests `8682c03fdfd70347a3ccd298268eaf8e32c7c507fd2fd3d0ddc2a29514aaf2ba`.

Actual result: **10 executed, 8 passed, 2 failed, 0 skipped**, 0.071s; Nx exit1. The canonical runner announced cancellation after the first failure, but all ten executed. No no-fail-fast argument is accepted by this target; no runner/profile override was added. No SIGABRT or secondary destructor panic occurred.

- Existing refusal law reached its unchanged post-cleanup assertion: observed `[1,0]`, required `[0,0]`. The original consumer and both roots were explicitly drained before asserting.
- New first-access law observed constructor0, first refused access1, repeated refused access0. Exact recorded allocation is phase2, size64, alignment8. Unchanged assertion observed `[0,1,0]`, required `[0,0,0]`.
- These observations identify the first-access boundary, not by themselves an allocator call stack or a live funding authority. No prewarm, exemption, production repair, limit change, or live producer adoption was performed.

Source/compiler hold released on terminal result. Dag owns the production repair. Input/metrics funding remains unavailable; a numeric work grant is not substituted for it.

Command:
```sh
SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache
```

## Complete Captured Tool Output

```text
> nx run @semio-tech/value-resident-rs:test

> bun ./📜️script.ts test

────────────
 Nextest run ID 2433bfa1-436b-48a7-8361-2fcdf94ff182 with nextest profile: exhaustive
    Starting 10 tests across 1 binary
        FAIL [   0.042s] ( 5/10) semio-framework-value-resident tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer
  stdout ───

    running 1 test
    test tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer ... FAILED

    failures:

    failures:
        tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.01s
    
  stderr ───

    thread 'tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer' (9536661) panicked at 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:225:5:
    assertion `left == right` failed: refused admission must not allocate; original consumer was retired before this assertion
      left: [1, 0]
     right: [0, 0]
    stack backtrace:
       0: __rustc::rust_begin_unwind
       1: core::panicking::panic_fmt
       2: core::panicking::assert_failed_inner
       3: core::panicking::assert_failed::<[usize; 2], [usize; 2]>
       4: semio_framework_value_resident::tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer
       5: semio_framework_value_resident::tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer::{closure#0}
       6: <semio_framework_value_resident::tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer::{closure#0} as core::ops::function::FnOnce<()>>::call_once
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

  Cancelling due to test failure: 5 tests still running
        FAIL [   0.054s] (10/10) semio-framework-value-resident tests::resident_admission_first_access_refusal_allocation_boundary
  stdout ───

    running 1 test
    test tests::resident_admission_first_access_refusal_allocation_boundary ... FAILED

    failures:

    failures:
        tests::resident_admission_first_access_refusal_allocation_boundary

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s
    
  stderr ───
    [DEBUG] resident first-access root=0 first=1 second=0 layouts=[(2, 64, 8), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0)]

    thread 'tests::resident_admission_first_access_refusal_allocation_boundary' (9536667) panicked at 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:338:5:
    assertion `left == right` failed
      left: [0, 1, 0]
     right: [0, 0, 0]
    stack backtrace:
       0: __rustc::rust_begin_unwind
       1: core::panicking::panic_fmt
       2: core::panicking::assert_failed_inner
       3: core::panicking::assert_failed::<&[usize], &[usize]>
       4: semio_framework_value_resident::tests::resident_admission_first_access_refusal_allocation_boundary
       5: semio_framework_value_resident::tests::resident_admission_first_access_refusal_allocation_boundary::{closure#0}
       6: <semio_framework_value_resident::tests::resident_admission_first_access_refusal_allocation_boundary::{closure#0} as core::ops::function::FnOnce<()>>::call_once
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

────────────
     Summary [   0.071s] 10 tests run: 8 passed, 2 failed, 0 skipped
        FAIL [   0.042s] ( 5/10) semio-framework-value-resident tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer
        FAIL [   0.054s] (10/10) semio-framework-value-resident tests::resident_admission_first_access_refusal_allocation_boundary
error: test run failed
Warning: command "bun ./📜️script.ts test" exited with non-zero status code


 NX   Running target test for project @semio-tech/value-resident-rs failed

Failed tasks:

- @semio-tech/value-resident-rs:test

Hint: run the command with --verbose for more details.
```

