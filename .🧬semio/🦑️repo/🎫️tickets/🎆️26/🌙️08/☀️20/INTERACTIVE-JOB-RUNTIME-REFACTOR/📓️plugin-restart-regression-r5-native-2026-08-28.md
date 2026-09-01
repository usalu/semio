# Plugin Restart Regression R5

Exactly2 tests passed,522 skipped,.137s,Nx0 on the held R4 snapshot (main04f85d…). The intentional panic after exact transient-store handoff is caught and later ownership/close assertions pass. This regression does not repair or supersede the separate full-checkpoint R4 catalog failure and constructor-unwind SIGABRT.

Both source holds were released immediately at this terminal result. Existing retained target/jobs2/exhaustive/no-fail-fast; no limit or profile change. Input provenance is [R4 capture](./📓️plugin-checkpoint-r4-selected-inputs-2026-08-28.md), with its explicit pre-dispatch versus during-held-compile distinction.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive checkpoint_restart_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-restart-regression-r5-2026-08-28.md'
```

Raw [R5 stream](./🧪️member-plugin-restart-regression-r5-2026-08-28.md); nextest artifacts `🧪️native-artifacts/semio-nextest-lu4LNv`. Full untruncated tool output:

```text

> nx run @semio-tech/framework-plugin:test --args=exhaustive checkpoint_restart_ --no-fail-fast -- --nocapture

> bun 📜️script.ts test exhaustive checkpoint_restart_ --no-fail-fast -- --nocapture

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] plugin-runner-oracle cases=6
────────────
[32;1m Nextest run[0m ID [1m151e3da2-b4ab-4f0c-b61a-589c298e9559[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m2[0m tests across [1m1[0m binary ([1m522[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_mode_requires_its_exact_concrete_factory_owner[0m

running 1 test
test component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_mode_requires_its_exact_concrete_factory_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 523 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (1/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_mode_requires_its_exact_concrete_factory_owner[0m
[32;1m       START[0m [         ] (2/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_transient_close_retains_the_exact_store_until_granted[0m

running 1 test

thread 'component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted' (9183725) panicked at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:220:9:
injected after the original transient store entered its structural retirement owner
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted::{closure#0}
   3: <semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted::{closure#0}>, ()>
   8: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted
   9: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted::{closure#0}
  10: <semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 523 filtered out; finished in 0.04s

[32;1m        PASS[0m [   0.056s] (2/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_transient_close_retains_the_exact_store_until_granted[0m
────────────
[32;1m     Summary[0m [   0.137s] [1m2[0m tests run: [1m2[0m [32;1mpassed[0m, [1m522[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-lu4LNv[0m



 NX   Successfully ran target test for project @semio-tech/framework-plugin



 NX   Nx detected a flaky task

  @semio-tech/framework-plugin:test

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

