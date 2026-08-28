# Plugin Awaited Completion R1

## Evidence Availability Correction

The checkpoint and idempotent raw .txt files disappeared before the report's later file-copy step. The two literal `cat: ... No such file or directory` blocks below are failed artifact reads, **not test output**. Their exit codes were not checked by that copy step; this was a report-generation error. No unknown output is reconstructed and no raw file is recreated. The generated-conformance output was read successfully.

The actual live tool transcript still contains checkpoint completion chunk `04b669` (reported 3,632 original tokens, truncated to 3,000; primary failure and footer visible) and idempotent completion chunk `084a01` (complete 318-token tail). The following are exact visible excerpts copied from those existing tool results, with terminal color escapes omitted. They are partial transcript evidence, not recovered complete raw logs.

```text
the SAME command dispatched directly must succeed: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'applyCountFromTask' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
artifact store reached Drop without its exact terminal-empty shallow-shell witness
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
     Summary [   0.091s] 1 test run: 0 passed, 1 failed, 521 skipped
```

```text
test component::subset_macro_tests::subset_macro_derived_register_is_idempotent ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.01s
     Summary [   0.063s] 1 test run: 1 passed, 521 skipped
```

The idempotent nextest artifact directory `🧪️native-artifacts/semio-nextest-SraKrs` still contains only `binaries-metadata.json`; it is not a recovery copy of stdout. No cleanup, deletion, or relocation was performed by this lane. The disappearance cause is not established.

## Reported Results

All three canonical exhaustive/no-fail-fast selectors executed separately with unchanged budgets. Idempotent registration passed 1/1 (521 skipped, 0.063s); generated conformance passed 1/1 (521 skipped, 0.016s). Checkpoint failed 0/1 (521 skipped, 0.091s) with SIGABRT: primary `interactive-job.missing-factory` for `applyCountFromTask` at line 36042, then strict ArtifactStore Drop failures during unwind. It progressed beyond the previous parked-task count failure; no complete checkpoint success is claimed. Source holds were released after all three processes terminated. Dag owns the further fixture correction.

## checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume

Exit 1.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-awaited-checkpoint-green-r1-2026-08-27.txt'
```

```text
cat: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-awaited-checkpoint-green-r1-2026-08-27.txt: No such file or directory

```

## subset_macro_derived_register_is_idempotent

Exit 0.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive subset_macro_derived_register_is_idempotent --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-awaited-idempotent-green-r1-2026-08-27.txt'
```

```text
cat: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-awaited-idempotent-green-r1-2026-08-27.txt: No such file or directory

```

## subset_macro_derived_validator_registers

Exit 0.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive subset_macro_derived_validator_registers --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-awaited-generated-green-r1-2026-08-27.txt'
```

```text

> nx run @semio-tech/framework-plugin:test --args=exhaustive subset_macro_derived_validator_registers --no-fail-fast -- --nocapture

> bun 📜️script.ts test exhaustive subset_macro_derived_validator_registers --no-fail-fast -- --nocapture

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
[32;1m Nextest run[0m ID [1m3b4fd316-f84c-4774-80be-eb313808f6cd[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m1[0m test across [1m1[0m binary ([1m521[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/1) [35;1msemio-framework-plugin[0m [36mcomponent::subset_macro_tests::__subset_registration::conformance[0m[36m::[0m[34;1msubset_macro_derived_validator_registers[0m

running 1 test
test component::subset_macro_tests::__subset_registration::conformance::subset_macro_derived_validator_registers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 521 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (1/1) [35;1msemio-framework-plugin[0m [36mcomponent::subset_macro_tests::__subset_registration::conformance[0m[36m::[0m[34;1msubset_macro_derived_validator_registers[0m
────────────
[32;1m     Summary[0m [   0.016s] [1m1[0m test run: [1m1[0m [32;1mpassed[0m, [1m521[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-5taprc[0m



 NX   Successfully ran target test for project @semio-tech/framework-plugin



```
