# UI Host Baseline Five R3 Native RED

## Actual Result

Exactly **0 passed,5 failed,63 skipped**, .070s,Nx1. All five unchanged baseline assertions executed; no secondary abort. This follows the separate R2 compile-only cfg-import failure and seven test-only cfg joins. EventQueue production and all five test expectations remained unchanged.

| Law | Actual failure |
| --- | --- |
| No unadmitted constructor backing | capacity256 × actual DiscreteEvent size56 =14336 bytes,expected0 |
| Full refusal preserves generation |257 versus original256 |
| Event generation never wraps |0 versus u64MAX |
| Metrics generation never wraps |0 versus u64MAX |
| Terminal requires empty backing |terminal=true while14336queue bytes remain; original payload logical8/capacity64 |

The String pointer/capacity assertions passed before that final law's cleanup. These laws do not yet execute a retained refused-String transfer, zero/short bytewise String retirement, actual Watchdog verdict, mutex contention, three-receiver metrics commit, or root identity. Their production APIs and missing-API tests remain upcoming; no such proof is borrowed from the independent22-case source oracle.

## Source and Command

[27 selected inputs](./📓️ui-host-baseline-selected-inputs-r3-2026-08-28.md), including TSVs, were captured before dispatch. Window cfg-join SHA02cdfea796b72c5ff244068b095280dc9c1fcee70f019e04918c65912002549f. Queue production SHA36d9370c16e9d4251b82f87b1133cdc353bd843c67e58122226cc6c14667373c unchanged. Not a full atomic dependency closure. Existing retained target/jobs2/exhaustive,router's existing no-fail-fast,coverage disabled; no limit/thread/stack changes.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/ui-host-rs:test --skip-nx-cache --args='exhaustive --lib input_admission_ -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-ui-host-baseline-five-r3-2026-08-28.md'
```

## Full Actual Tool Output

Raw [R3 stream](./🧪️member-ui-host-baseline-five-r3-2026-08-28.md). Full untruncated output also retained below. The existing browser envelope test now compiles, but its separate actual execution remains pending.

```text

> nx run @semio-tech/ui-host-rs:test --args=exhaustive --lib input_admission_ -- --nocapture

> bun ./📜️script.ts test exhaustive --lib input_admission_ -- --nocapture

────────────
[32;1m Nextest run[0m ID [1m1165bc47-2b4f-4575-a396-5a33725c68c3[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m5[0m tests across [1m1[0m binary ([1m63[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_constructor_has_no_unadmitted_backing[0m

running 1 test
[DEBUG] event-queue-constructor capacity=256 slot-bytes=56 physical=14336

thread 'enqueue::input_admission_tests::input_admission_constructor_has_no_unadmitted_backing' (9316718) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:23:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 67 filtered out; finished in 0.00s

[31;1m        FAIL[0m [   0.015s] (1/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_constructor_has_no_unadmitted_backing[0m
[32;1m       START[0m [         ] (2/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_full_refusal_preserves_generation[0m

running 1 test

thread 'enqueue::input_admission_tests::input_admission_full_refusal_preserves_generation' (9316721) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:38:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 67 filtered out; finished in 0.00s

[31;1m        FAIL[0m [   0.014s] (2/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_full_refusal_preserves_generation[0m
[32;1m       START[0m [         ] (3/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_generation_exhaustion_does_not_wrap[0m

running 1 test

thread 'enqueue::input_admission_tests::input_admission_generation_exhaustion_does_not_wrap' (9316724) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:49:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 67 filtered out; finished in 0.00s

[31;1m        FAIL[0m [   0.014s] (3/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_generation_exhaustion_does_not_wrap[0m
[32;1m       START[0m [         ] (4/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_metrics_generation_exhaustion_preserves_queue[0m

running 1 test

thread 'enqueue::input_admission_tests::input_admission_metrics_generation_exhaustion_preserves_queue' (9316727) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:62:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 67 filtered out; finished in 0.00s

[31;1m        FAIL[0m [   0.013s] (4/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_metrics_generation_exhaustion_preserves_queue[0m
[32;1m       START[0m [         ] (5/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_terminal_requires_empty_backing[0m

running 1 test
[DEBUG] event-queue-terminal logical=8 original-payload-capacity=64 retained-queue-backing=14336 terminal=true

thread 'enqueue::input_admission_tests::input_admission_terminal_requires_empty_backing' (9316730) panicked at 🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/../../📥️input/🎟️admission/🧪️component.rs:86:5:
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

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 67 filtered out; finished in 0.00s

[31;1m        FAIL[0m [   0.014s] (5/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_terminal_requires_empty_backing[0m
────────────
[31;1m     Summary[0m [   0.070s] [1m5[0m tests run: [1m0[0m [32;1mpassed[0m, [1m5[0m [31;1mfailed[0m, [1m63[0m [33;1mskipped[0m
[31;1m        FAIL[0m [   0.015s] (1/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_constructor_has_no_unadmitted_backing[0m
[31;1m        FAIL[0m [   0.014s] (2/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_full_refusal_preserves_generation[0m
[31;1m        FAIL[0m [   0.014s] (3/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_generation_exhaustion_does_not_wrap[0m
[31;1m        FAIL[0m [   0.013s] (4/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_metrics_generation_exhaustion_preserves_queue[0m
[31;1m        FAIL[0m [   0.014s] (5/5) [35;1msemio-framework-ui-host[0m [36menqueue::input_admission_tests[0m[36m::[0m[34;1minput_admission_terminal_requires_empty_backing[0m
[31;1merror[0m: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib input_admission_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-host-rs failed

Failed tasks:

- @semio-tech/ui-host-rs:test

Hint: run the command with --verbose for more details.


```

