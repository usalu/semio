# Plugin Restart Two-Law R3 Native Gate

## Actual Result

Canonical exhaustive selection completed with **2 passed, 0 failed, 522 skipped**, 0.087s; Nx exit 0. This is exactly the const-owner and transient-disposer pair, not the complete checkpoint/restore test or full Plugin suite. The historical R2 1/1 semantic RED remains preserved.

## Source and Command

751 selected nested source inputs were captured in [R3 input manifest](./📓️plugin-restart-r3-selected-inputs-2026-08-28.md). This is a selected input capture, not an atomic full dependency closure. Plugin main SHA256 `099212bbc6eb4bb2a5205fe109588989f71f36d89313fb1b449da9838c4a821f`; Store SHA256 `0ed0d7a78c833c1081825c598de3a5dde36ecc858a2e1448c5695899358efd0d`.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive checkpoint_restart_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-restart-two-r3-2026-08-28.md'
```

Existing retained target, CARGO_BUILD_JOBS=2, exhaustive profile, no-fail-fast, coverage disabled; no grant, time, stack, thread or quota changes. Source holds were released immediately after terminal.

## Executed Scope

The const-owner law passes. The transient law now reaches and passes zero-item and 4095-byte refusal, original root identity before admission, the granted 4096-byte structural handoff, and the deliberately caught panic after the method returns. It then checks the original root remains alive in the retained disposer, differs from the replacement root, is released on the next granted phase, and terminal completion rejects a foreign store.

The printed panic at completion/🦀️.rs:156 is intentional and caught; the test subsequently passes. It is not an internal allocation-panic, poisoned-mutex, arbitrary transient payload, or callback timing proof. The fixture's declared 4096-byte phases are not an allocator measurement. No complete checkpoint tail was selected.

Raw stream: [R3 raw Markdown](./🧪️member-plugin-restart-two-r3-2026-08-28.md). Original nextest artifacts: `🧪️native-artifacts/semio-nextest-wUUP7v`. The full untruncated tool output is also preserved below. Nx's historical flaky-task notice does not change the observed successful run into a retry claim.

## Actual Tool Output

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
[32;1m Nextest run[0m ID [1mc5f4054e-53d7-416c-a73c-c486528da47e[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m2[0m tests across [1m1[0m binary ([1m522[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_mode_requires_its_exact_concrete_factory_owner[0m

running 1 test
test component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_mode_requires_its_exact_concrete_factory_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 523 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (1/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_mode_requires_its_exact_concrete_factory_owner[0m
[32;1m       START[0m [         ] (2/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_transient_close_retains_the_exact_store_until_granted[0m

running 1 test

thread 'component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted' (9131413) panicked at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:156:9:
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 523 filtered out; finished in 0.05s

[32;1m        PASS[0m [   0.065s] (2/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_transient_close_retains_the_exact_store_until_granted[0m
────────────
[32;1m     Summary[0m [   0.087s] [1m2[0m tests run: [1m2[0m [32;1mpassed[0m, [1m522[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-wUUP7v[0m



 NX   Successfully ran target test for project @semio-tech/framework-plugin



 NX   Nx detected a flaky task

  @semio-tech/framework-plugin:test

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

