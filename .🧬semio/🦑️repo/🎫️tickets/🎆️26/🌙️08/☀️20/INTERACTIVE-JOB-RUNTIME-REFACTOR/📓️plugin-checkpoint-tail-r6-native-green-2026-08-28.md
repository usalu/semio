# Plugin Checkpoint Tail R6 Native GREEN

## Actual Result

Exactly one full checkpoint/restore test passed,523 skipped,.057s,Nx0. Runtime DEBUG reports `outcome=Ok((1, 1, 1, 1, 7)), closed=true, close_fault=None`: one Artifact result,one UI result,one full-UI scope,one terminal receipt,and restored count7. The helper checks exact result ACKs and retires the original app before asserting the collected outcome. This is actual execution of the previously failing checkpoint tail on the corrected generated-tool roster, not full Plugin or guest lifecycle proof.

R4's primary generated_migrated=false catalog failure is superseded for this corrected fixture path. Its separate generic constructor-unwind/ArtifactStore Drop defect remains open; this successful constructor does not exercise or repair it. No per-callback timing or arbitrary payload bounded-close claim.

## Source and Command

Main SHA256 `2ad816977def25ded3175c87c0f7d03344f1bae57549689b17388adf871736ca`; Store0ed0d7… unchanged. [R6 input capture](./📓️plugin-checkpoint-r6-selected-inputs-2026-08-28.md) contains864 actual pre-dispatch selected hashes, including native Store; not an atomic full dependency closure. Existing retained target/jobs2,exhaustive/no-fail-fast,coverage disabled,unchanged grants/limits.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-checkpoint-tail-r6-2026-08-28.md'
```

Raw [R6 stream](./🧪️member-plugin-checkpoint-tail-r6-2026-08-28.md), nextest artifacts `🧪️native-artifacts/semio-nextest-AL4XxV`. The changed const-owner roster law follows separately on this same held source.

## Full Actual Tool Output

```text

> nx run @semio-tech/framework-plugin:test --args=exhaustive checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume --no-fail-fast -- --nocapture

> bun 📜️script.ts test exhaustive checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume --no-fail-fast -- --nocapture

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
[32;1m Nextest run[0m ID [1m9901de16-0fad-4480-9129-1738ee6e4164[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m1[0m test across [1m1[0m binary ([1m523[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/1) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume[0m

running 1 test
[DEBUG] restart retained publication outcome=Ok((1, 1, 1, 1, 7)), closed=true, close_fault=None
test component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 523 filtered out; finished in 0.04s

[32;1m        PASS[0m [   0.055s] (1/1) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume[0m
────────────
[32;1m     Summary[0m [   0.057s] [1m1[0m test run: [1m1[0m [32;1mpassed[0m, [1m523[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-AL4XxV[0m



 NX   Successfully ran target test for project @semio-tech/framework-plugin



 NX   Nx detected a flaky task

  @semio-tech/framework-plugin:test

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

