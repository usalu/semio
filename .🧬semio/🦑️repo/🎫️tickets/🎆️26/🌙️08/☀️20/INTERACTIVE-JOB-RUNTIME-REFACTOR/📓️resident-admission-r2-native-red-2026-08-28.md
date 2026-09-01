# Resident Admission R2 Structural Native RED

Actual canonical exhaustive inventory: **8 passed, 1 failed, 0 skipped**, nine executed in0.044s; Nx exit1. The last test `resident_admission_short_and_foreign_refusals_preserve_live_consumer` first fails the observed allocation assertion at test line191: actual1, expected0. Its live probe then reaches strict Drop at line128 during assertion unwind, causing SIGABRT. These are distinct first-failure and cleanup outcomes; no assertion or destructor was weakened. The runner's cancellation notice occurred after test9/9; all nine executed.

No live composition, over-allocation-error, foreign mutable consumer, unknown-fault disposal, or outer-root loss proof. Resident source hold released at terminal, diagnostic routed to its owner. This lane changed no resident implementation.

[20 selected inputs](./📓️resident-admission-r2-selected-inputs-2026-08-28.md). Actual production `6748e3961f82178ef59c9bb8ccc89117b12ab0c2014759bf5969353fa170ed83`; tests `4a9850d501948d9a994b40da42b9d1b4063b667e0bf219630b6e288f9886e92d`. Selected capture, not full atomic dependency closure.

The canonical noargs target chooses exhaustive through environment. It printed the failure and summary; passing-case stdout is not present in this tool output. Existing retained target/jobs2/budgets unchanged.

```sh
set -o pipefail
SEMIO_TEST_LEVEL=exhaustive SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-resident-admission-r2-2026-08-28.md'
```

## Complete Captured Tool Output

```text

> nx run @semio-tech/value-resident-rs:test

> bun ./📜️script.ts test

────────────
[32;1m Nextest run[0m ID [1m86dc9d8e-4d93-471b-9371-357b191d505a[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m9[0m tests across [1m1[0m binary
[31;1m     SIGABRT[0m [   0.041s] (9/9) [35;1msemio-framework-value-resident[0m [36mtests[0m[36m::[0m[34;1mresident_admission_short_and_foreign_refusals_preserve_live_consumer[0m
[31;1m [0m [31;1mstdout[0m [31;1m───[0m

    running 1 test[0m
[31;1m [0m [31;1mstderr[0m [31;1m───[0m

    thread 'tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer' (9478347) panicked at 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:191:59:
    assertion `left == right` failed
      left: 1
     right: 0
    stack backtrace:
       0: __rustc::rust_begin_unwind
       1: core::panicking::panic_fmt
       2: core::panicking::assert_failed_inner
       3: core::panicking::assert_failed::<usize, usize>
       4: semio_framework_value_resident::tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer
       5: semio_framework_value_resident::tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer::{closure#0}
       6: <semio_framework_value_resident::tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer::{closure#0} as core::ops::function::FnOnce<()>>::call_once
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

    thread 'tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer' (9478347) panicked at 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:128:9:
    the concrete parent must retire the original payload before its shell destructor
    stack backtrace:
       0:        0x1024e96f0 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
       1:        0x1024fa2e0 - core[c6c0a6c66382aec3]::fmt::write
       2:        0x1024eddb8 - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
       3:        0x1024d0c40 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
       4:        0x1024e32d0 - std[87758e35c17852a5]::panicking::default_hook
       5:        0x1024e35f8 - std[87758e35c17852a5]::panicking::panic_with_hook
       6:        0x1024d0cf4 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
       7:        0x1024c63ac - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
       8:        0x1024d1200 - __rustc[feecb8598a58626c]::rust_begin_unwind
       9:        0x1025075b8 - core[c6c0a6c66382aec3]::panicking::panic_fmt
      10:        0x102460080 - <semio_framework_value_resident[3988b7ec1dd18658]::tests::ResidentDropProbe as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
      11:        0x10245fd54 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_value_resident[3988b7ec1dd18658]::tests::ResidentDropProbe>
      12:        0x10245f68c - core[c6c0a6c66382aec3]::ptr::drop_glue::<core[c6c0a6c66382aec3]::option::Option<semio_framework_value_resident[3988b7ec1dd18658]::tests::ResidentDropProbe>>
      13:        0x10245f480 - core[c6c0a6c66382aec3]::ptr::drop_glue::<core[c6c0a6c66382aec3]::cell::UnsafeCell<core[c6c0a6c66382aec3]::option::Option<semio_framework_value_resident[3988b7ec1dd18658]::tests::ResidentDropProbe>>>
      14:        0x10245fb68 - core[c6c0a6c66382aec3]::ptr::drop_glue::<std[87758e35c17852a5]::sync::poison::mutex::Mutex<core[c6c0a6c66382aec3]::option::Option<semio_framework_value_resident[3988b7ec1dd18658]::tests::ResidentDropProbe>>>
      15:        0x102470374 - <alloc[659a9e145e4cda22]::sync::Arc<std[87758e35c17852a5]::sync::poison::mutex::Mutex<core[c6c0a6c66382aec3]::option::Option<semio_framework_value_resident[3988b7ec1dd18658]::tests::ResidentDropProbe>>>>::drop_slow
      16:        0x10246041c - <alloc[659a9e145e4cda22]::sync::Arc<std[87758e35c17852a5]::sync::poison::mutex::Mutex<core[c6c0a6c66382aec3]::option::Option<semio_framework_value_resident[3988b7ec1dd18658]::tests::ResidentDropProbe>>> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
      17:        0x10245f874 - core[c6c0a6c66382aec3]::ptr::drop_glue::<alloc[659a9e145e4cda22]::sync::Arc<std[87758e35c17852a5]::sync::poison::mutex::Mutex<core[c6c0a6c66382aec3]::option::Option<semio_framework_value_resident[3988b7ec1dd18658]::tests::ResidentDropProbe>>>>
      18:        0x102468018 - semio_framework_value_resident[3988b7ec1dd18658]::tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer
      19:        0x102470754 - semio_framework_value_resident[3988b7ec1dd18658]::tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer::{closure#0}
      20:        0x102460a0c - <semio_framework_value_resident[3988b7ec1dd18658]::tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
      21:        0x10248cd34 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
      22:        0x102498068 - test[ee52d9429afbedb2]::run_test::{closure#0}
      23:        0x102492f48 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
      24:        0x10249a530 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
      25:        0x1024e9044 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
      26:        0x188071c58 - __pthread_cond_wait

    [0m[31;1mthread 'tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer' (9478347) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:[0m
    [31;1mpanic in a destructor during cleanup[0m
    thread caused non-unwinding panic. aborting.[0m

    (test [31;1maborted[0m with signal [1m6[0m: SIGABRT)

[31;1m  Cancelling[0m due to [31;1mtest failure[0m: 
────────────
[31;1m     Summary[0m [   0.044s] [1m9[0m tests run: [1m8[0m [32;1mpassed[0m, [1m1[0m [31;1mfailed[0m, [1m0[0m [33;1mskipped[0m
[31;1m     SIGABRT[0m [   0.041s] (9/9) [35;1msemio-framework-value-resident[0m [36mtests[0m[36m::[0m[34;1mresident_admission_short_and_foreign_refusals_preserve_live_consumer[0m
[31;1merror[0m: test run failed
Warning: command "bun ./📜️script.ts test" exited with non-zero status code


 NX   Running target test for project @semio-tech/value-resident-rs failed

Failed tasks:

- @semio-tech/value-resident-rs:test

Hint: run the command with --verbose for more details.


```

