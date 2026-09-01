# Action-Bus Short-Close R2 Native Gate

Actual canonical nextest exhaustive result: 4 passed, 0 failed, 262 skipped, 0.070s; Nx exit 0. The unchanged production close code now agrees with the corrected registered fixture and the new exact logical/physical conservation law. Actual DEBUG: seven-plus-one initialized bytes; physical page-vector capacity remains one until its distinct empty-backing release, then zero. This is not a callback timing proof.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-rs:test-wire-retirement-native --skip-nx-cache --args='exhaustive --lib retained_wire_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-actionbus-short-close-native-r2-2026-08-28.md'
```

## Complete Captured Output

```text

> nx run @semio-tech/framework-rs:test-wire-retirement-native --args=exhaustive --lib retained_wire_ --no-fail-fast -- --nocapture

> bun ./📜️script.ts test-wire-retirement-native exhaustive --lib retained_wire_ --no-fail-fast -- --nocapture

────────────
[32;1m Nextest run[0m ID [1m0a12f86f-ce43-4242-b7d6-a06ee981bd49[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m4[0m tests across [1m1[0m binary ([1m262[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/4) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mretained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation[0m

running 1 test
test action_bus::tests::retained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (1/4) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mretained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation[0m
[32;1m       START[0m [         ] (2/4) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mretained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes[0m

running 1 test
[DEBUG] retained-number-close zero-items=blocked zero-bytes=blocked logical=7+1 backing-logical=0 terminal=true
test action_bus::tests::retained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.020s] (2/4) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mretained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes[0m
[32;1m       START[0m [         ] (3/4) [35;1msemio-framework[0m [36maction_bus::wire_retirement_tests[0m[36m::[0m[34;1mretained_wire_input_small_grants_retire_initialized_bytes_and_backing_allocation[0m

running 1 test
test action_bus::wire_retirement_tests::retained_wire_input_small_grants_retire_initialized_bytes_and_backing_allocation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.015s] (3/4) [35;1msemio-framework[0m [36maction_bus::wire_retirement_tests[0m[36m::[0m[34;1mretained_wire_input_small_grants_retire_initialized_bytes_and_backing_allocation[0m
[32;1m       START[0m [         ] (4/4) [35;1msemio-framework[0m [36maction_bus::wire_retirement_tests[0m[36m::[0m[34;1mretained_wire_short_close_conserves_logical_bytes_and_physical_backing[0m

running 1 test
[DEBUG] wire-short-close released=8 backing-capacity=1->0 zero-grants-preserve=true
test action_bus::wire_retirement_tests::retained_wire_short_close_conserves_logical_bytes_and_physical_backing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (4/4) [35;1msemio-framework[0m [36maction_bus::wire_retirement_tests[0m[36m::[0m[34;1mretained_wire_short_close_conserves_logical_bytes_and_physical_backing[0m
────────────
[32;1m     Summary[0m [   0.070s] [1m4[0m tests run: [1m4[0m [32;1mpassed[0m, [1m262[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-5QWwxi[0m



 NX   Successfully ran target test-wire-retirement-native for project @semio-tech/framework-rs



```

