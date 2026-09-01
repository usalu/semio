# Resident Foreign Consumer R6 Native RED

Actual canonical no-argument exhaustive result: **13 executed, 12 passed, 1 failed, 0 skipped**, 0.050s, Nx1. All13 executed despite the runner's cancellation announcement. Fresh selected capture: [inputs](./📓️resident-foreign-consumer-r6-selected-inputs-2026-08-28.md). No production repair in this executor.

The new real empty-check/release interlock law printed accepted=true, consumerDropsDuringRelease=1, originalRootTerminal=true. It reached the unchanged expected0 destructor assertion after exact cleanup; actual1 failed. There was no secondary abort. This is a real foreign mutable consumer ownership failure; the existing12 passes do not establish a private consumer/parent authority. Passing stdout was not captured by this invocation.

Source/compiler hold released to Dag on terminal result. No Wasm rerun for this intentional native RED.

```sh
SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache
```

## Complete Captured Tool Output

```text
> nx run @semio-tech/value-resident-rs:test

> bun ./📜️script.ts test

────────────
 Nextest run ID f6003d18-4ddc-41c1-ba68-4fd090da14ff with nextest profile: exhaustive
    Starting 13 tests across 1 binary
        FAIL [   0.029s] (10/13) semio-framework-value-resident tests::resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop
  stdout ───

    running 1 test
    test tests::resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop ... FAILED

    failures:

    failures:
        tests::resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s
    
  stderr ───
    [DEBUG] resident foreign repopulation accepted=true consumerDropsDuringRelease=1 originalRootTerminal=true

    thread 'tests::resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop' (9616440) panicked at 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:406:5:
    assertion `left == right` failed
      left: 1
     right: 0
    stack backtrace:
       0: __rustc::rust_begin_unwind
       1: core::panicking::panic_fmt
       2: core::panicking::assert_failed_inner
       3: core::panicking::assert_failed::<u64, u64>
       4: semio_framework_value_resident::tests::resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop
       5: semio_framework_value_resident::tests::resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop::{closure#0}
       6: <semio_framework_value_resident::tests::resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop::{closure#0} as core::ops::function::FnOnce<()>>::call_once
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

  Cancelling due to test failure: 3 tests still running
────────────
     Summary [   0.050s] 13 tests run: 12 passed, 1 failed, 0 skipped
        FAIL [   0.029s] (10/13) semio-framework-value-resident tests::resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop
error: test run failed
Warning: command "bun ./📜️script.ts test" exited with non-zero status code


 NX   Running target test for project @semio-tech/value-resident-rs failed

Failed tasks:

- @semio-tech/value-resident-rs:test

Hint: run the command with --verbose for more details.
```

