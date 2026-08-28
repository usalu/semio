# Component Copy and Comparison R39

Canonical exact `retained_component_` selector: five passed,130 skipped,0.223s,exit0. R38 failed before tests on two sibling-private progress constructors; replaced only with existing public progress fields, without visibility changes. R37 actual4096-work Surface RED is now GREEN: inlineComponent3096, max initialized/copied step4096. Allocation still separately pre-admits32768; private Vec retains uninitialized capacity, fills bounded slices, and moves intoBox only after exactlen==capacity, so transfer cannot reallocate. Pending buffer remains in the root owner and closes bytewise.

Retained equality passes all18 variants plus7 explicit value cases at1/64/4096 bytes, using bytewise operand accounting and fixed-depth immutable arena-page traversal with try_lock. Equal values on distinct roots are compared semantically, not by identity. Contention and seven cancellation frontiers retain both roots and close terminally.

No active runtime adoption/resident-meter/Process-fit/full-regression/Wasm claim follows from this scoped gate. The existing runtime physical page constant is32768; this packet did not alter it and separately verifies4096 initialized-byte work.

```text
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] fixed-list-page-oracle checks=35
warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID c9bfff79-1649-48e1-9753-44a97f1ff1b4 with nextest profile: fundamental
    Starting 5 tests across 1 binary (130 tests skipped)
       START [         ] (1/5) semio-framework-ui-contract action::component_compare_tests::retained_component_compare_cancellation_and_arena_contention_keep_both_roots

running 1 test
[DEBUG] retained-component-compare cancel-frontiers=7 contended-owner-preserved=true no-wait=true
test action::component_compare_tests::retained_component_compare_cancellation_and_arena_contention_keep_both_roots ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 134 filtered out; finished in 0.00s

        PASS [   0.014s] (1/5) semio-framework-ui-contract action::component_compare_tests::retained_component_compare_cancellation_and_arena_contention_keep_both_roots
       START [         ] (2/5) semio-framework-ui-contract action::component_compare_tests::retained_component_compare_matches_all_native_variants_and_hostile_values

running 1 test
[DEBUG] retained-component-compare variants=18 hostile-values=7 byte-grants=1,64,4096 exact-serde=true
test action::component_compare_tests::retained_component_compare_matches_all_native_variants_and_hostile_values ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 134 filtered out; finished in 0.00s

        PASS [   0.014s] (2/5) semio-framework-ui-contract action::component_compare_tests::retained_component_compare_matches_all_native_variants_and_hostile_values
       START [         ] (3/5) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_all_variants_match_native_serde

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
[DEBUG] retained-component-copy type="surface" turns=8 allocated=32768 exact-serde=true
test action::component_copy_tests::retained_component_copy_all_variants_match_native_serde ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 134 filtered out; finished in 0.00s

        PASS [   0.012s] (3/5) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_all_variants_match_native_serde
       START [         ] (4/5) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_large_list_cancel_preserves_source_and_partial_candidate

running 1 test
[DEBUG] retained-component-cancel list-items=32 payload-bytes=32768 frontiers=8 close-grants=3 terminal=true
test action::component_copy_tests::retained_component_copy_large_list_cancel_preserves_source_and_partial_candidate ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 134 filtered out; finished in 0.16s

        PASS [   0.169s] (4/5) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_large_list_cancel_preserves_source_and_partial_candidate
       START [         ] (5/5) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_surface_advances_under_real_4096_work_grant

running 1 test
[DEBUG] component-copy-real-grant inline=3096 work-max=4096 complete=true
test action::component_copy_tests::retained_component_copy_surface_advances_under_real_4096_work_grant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 134 filtered out; finished in 0.00s

        PASS [   0.012s] (5/5) semio-framework-ui-contract action::component_copy_tests::retained_component_copy_surface_advances_under_real_4096_work_grant
────────────
     Summary [   0.223s] 5 tests run: 5 passed, 130 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-taAekN



 NX   Successfully ran target test for project @semio-tech/ui-contract-rs



 NX   Nx detected a flaky task

  @semio-tech/ui-contract-rs:test

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```
