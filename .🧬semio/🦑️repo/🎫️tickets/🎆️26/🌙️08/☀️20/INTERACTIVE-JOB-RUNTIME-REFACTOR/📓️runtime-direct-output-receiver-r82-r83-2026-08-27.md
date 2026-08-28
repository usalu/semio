# Direct Reserved Output Receiver R82–R83

R82: actual compile RED, four E0599 diagnostics for the missing direct receiver/required-byte API; no native test execution. R83: actual native 1 PASS, 120 skipped, 0.018s, exit0. The permanent schema and independent Node Buffer ownership oracle ran through the canonical target (reported40 checks).

The actual transfer debits 8,936 bytes, keeps the original Job shell and canonical current receiver structurally outside the caught callback, and moves Ready directly into its pre-reserved pool entry. Under-credit and occupied current refuse before mutation. A caught panic after the actual transfer leaves the original payload address owned by that entry. This is runtime-library scope, not live Plugin preadmission, internal instruction-level panic injection, producer allocation, guest execution, or watchdog timing.

The result distinguishes Pending, Empty and Published so no-op reconciliation cannot strand an empty output queue. Existing64-entry/64-queue and32KiB grants remain unchanged.

## R82 Exact Command and Full Output

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='exhaustive --lib surface_output_pool_direct_job_receiver_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-runtime-direct-output-red-r82-2026-08-27.txt'
```

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib surface_output_pool_direct_job_receiver_ --no-fail-fast -- --nocapture

> bun ./📜️script.ts test exhaustive --lib surface_output_pool_direct_job_receiver_ --no-fail-fast -- --nocapture

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] surface-ownership-oracle checks=40
error[E0599]: no associated function or constant named `required_job_transfer_bytes` found for struct `output::SurfaceReconcileOutputs` in the current scope
error[E0599]: no method named `receive_job_into` found for struct `output::SurfaceReconcileOutputs` in the current scope
error[E0599]: no method named `receive_job_into` found for struct `output::SurfaceReconcileOutputs` in the current scope
error[E0599]: no method named `receive_job_into` found for struct `output::SurfaceReconcileOutputs` in the current scope
error: could not compile `semio-framework-ui-runtime` (lib test) due to 4 previous errors; 10 warnings emittedWarning: command "bun ./📜️script.ts test exhaustive --lib surface_output_pool_direct_job_receiver_ --no-fail-fast -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.


```

## R83 Exact Command and Full Output

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='exhaustive --lib surface_output_pool_direct_job_receiver_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-runtime-direct-output-green-r83-2026-08-27.txt'
```

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib surface_output_pool_direct_job_receiver_ --no-fail-fast -- --nocapture

> bun ./📜️script.ts test exhaustive --lib surface_output_pool_direct_job_receiver_ --no-fail-fast -- --nocapture

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] surface-ownership-oracle checks=40
────────────
[32;1m Nextest run[0m ID [1me78464ae-6072-471f-856f-b1ecf54f81f5[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m1[0m test across [1m1[0m binary ([1m120[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/1) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind[0m

running 1 test

thread 'reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind' (8249250) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📤️output/🧪️component.rs:231:9:
[DEBUG] direct pool receiver callback after actual transfer
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}
   3: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}>, ()>
   8: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind
   9: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0}
  10: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] direct-pool receiver-bytes=8936 original-payload=true original-shell=true callback-unwind-retained=true
test reconcile::tests::output_pool_tests::surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.017s] (1/1) [35;1msemio-framework-ui-runtime[0m [36mreconcile::tests::output_pool_tests[0m[36m::[0m[34;1msurface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind[0m
────────────
[32;1m     Summary[0m [   0.018s] [1m1[0m test run: [1m1[0m [32;1mpassed[0m, [1m120[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-PBFT0h[0m



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs



```

