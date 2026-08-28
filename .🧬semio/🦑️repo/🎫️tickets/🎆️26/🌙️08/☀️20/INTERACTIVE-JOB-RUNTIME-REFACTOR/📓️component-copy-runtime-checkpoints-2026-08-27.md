# Component Copy Runtime Checkpoints

## R34 Native Exact Copy

Canonical `@semio-tech/ui-contract-rs:test --args='--lib retained_component_copy_ -- --nocapture'` exited 0: two tests passed, 130 skipped, 0.182 seconds. This follows R33's three missing-API compile errors. Strict Ajv plus independent Node Buffer oracle checks run before Rust (25 total including existing paging/bindings checks).

All 18 component variants match native serde. Select32 cancellation retains exact source/partial candidate across eight frontiers and grants1/64/4096. Partial candidates are hidden until complete. The shared typed field roster drives both copy and retirement; no domain-specific parallel field list was added. Physical allocations and initialized/copied bytes are separate, with UiFixedBytes' actual32KiB allocation admitted and charged as32KiB initialization, not its shorter semantic length.

This does not yet prove active runtime Component adoption, retained equality, complete resident accounting, Process fit, full UI regression or fresh Wasm execution.

## Captured Native Output

```text
> nx run @semio-tech/ui-contract-rs:test --args=--lib retained_component_copy_ -- --nocapture

> bun ./📜️script.ts test --lib retained_component_copy_ -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] fixed-list-page-oracle checks=25
warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID be5830fd-8f1c-4782-ad3d-7adb66933a8c with nextest profile: fundamental
    Starting 2 tests across 1 binary (130 tests skipped)
       START [         ] (1/2) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_all_variants_match_native_serde

running 1 test
[DEBUG] retained-component-copy type="container" turns=15 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="text" turns=10 allocated=3852 exact-serde=true
[DEBUG] retained-component-copy type="button" turns=3 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="separator" turns=2 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="input" turns=12 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="select" turns=10 allocated=3852 exact-serde=true
[DEBUG] retained-component-copy type="toggle" turns=5 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="keyValueList" turns=7 allocated=3852 exact-serde=true
[DEBUG] retained-component-copy type="slider" turns=7 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="numberStepper" turns=4 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="ring" turns=3 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="iconSelect" turns=4 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="tree" turns=3 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="treeSection" turns=4 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="treeItem" turns=33 allocated=8108 exact-serde=true
[DEBUG] retained-component-copy type="image" turns=4 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="extension" turns=3 allocated=0 exact-serde=true
[DEBUG] retained-component-copy type="surface" turns=6 allocated=32768 exact-serde=true
test action::component_copy_tests::retained_component_copy_all_variants_match_native_serde ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 131 filtered out; finished in 0.00s

        PASS [   0.017s] (1/2) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_all_variants_match_native_serde
       START [         ] (2/2) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_large_list_cancel_preserves_source_and_partial_candidate

running 1 test
[DEBUG] retained-component-cancel list-items=32 payload-bytes=32768 frontiers=8 close-grants=3 terminal=true
test action::component_copy_tests::retained_component_copy_large_list_cancel_preserves_source_and_partial_candidate ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 131 filtered out; finished in 0.15s

        PASS [   0.163s] (2/2) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_large_list_cancel_preserves_source_and_partial_candidate
────────────
     Summary [   0.182s] 2 tests run: 2 passed, 130 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-b1GNiF



 NX   Successfully ran target test for project @semio-tech/ui-contract-rs



```
