# Plugin Restart Two-Law R2 Native Gate

Actual canonical exhaustive result: 2 run, 1 passed, 1 failed, 522 skipped, 0.232s; Nx exit1. Const-owner law passes. The transient-store law fails at its intended first assertion, completion/🦀️.rs:149: actual `terminal_is_empty` true versus required false. No secondary abort occurred in the captured output. None of that test's later byte-grant/unwind/root-retirement assertions were reached. Complete checkpoint continuation and all other tests were deliberately unselected; no broader runtime proof.

Mutation/Dag source holds released at terminal. Source capture: `📓️plugin-restart-r2-selected-inputs-2026-08-28.md` (751 selected current inputs, not an atomic full closure). Main SHA matched release `15b212adb4d81e65eb7600e02068493812756a698e7e29aac8150022c1b4a261`. Same ticket target/jobs2, no changed profile/budget/serialization. Complete untruncated tool output immediately stored and copied below.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive checkpoint_restart_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-restart-two-r2-2026-08-28.md'
```

## Complete Captured Output

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
[32;1m Nextest run[0m ID [1m92751cad-98c0-4734-9cdb-47ff3f1dd810[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m2[0m tests across [1m1[0m binary ([1m522[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_mode_requires_its_exact_concrete_factory_owner[0m

running 1 test
test component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_mode_requires_its_exact_concrete_factory_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 523 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.166s] (1/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_mode_requires_its_exact_concrete_factory_owner[0m
[32;1m       START[0m [         ] (2/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_transient_close_retains_the_exact_store_until_granted[0m

running 1 test

thread 'component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted' (9114909) panicked at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:149:5:
assertion `left == right` failed
  left: true
 right: false
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<bool, bool>
   4: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted
   5: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted::{closure#0}
   6: <semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted ... FAILED

failures:

failures:
    component::plugin_runtime::plugin_builder_contract_tests::checkpoint_restart_transient_close_retains_the_exact_store_until_granted

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 523 filtered out; finished in 0.05s

[31;1m        FAIL[0m [   0.063s] (2/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_transient_close_retains_the_exact_store_until_granted[0m
────────────
[31;1m     Summary[0m [   0.232s] [1m2[0m tests run: [1m1[0m [32;1mpassed[0m, [1m1[0m [31;1mfailed[0m, [1m522[0m [33;1mskipped[0m
[31;1m        FAIL[0m [   0.063s] (2/2) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_restart_transient_close_retains_the_exact_store_until_granted[0m
[31;1merror[0m: test run failed
Warning: command "bun 📜️script.ts test exhaustive checkpoint_restart_ --no-fail-fast -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-plugin failed

Failed tasks:

- @semio-tech/framework-plugin:test

Hint: run the command with --verbose for more details.


```

