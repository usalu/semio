# Runtime Component Copy R21 GREEN

Actual canonical fresh-record Surface test:1 passed,92 skipped,0.032s,exit0 after R20 semantic RED. The actual runtime now retains UiComponentCopy inside RecordDiffCursor before entering the callback; separate turns pre-admit physical allocation, copy/initialize at4096 bytes, and restore original/candidate roots. Surface32KiB copies in15 turns; allocation ledger exactly32768, reported total81781 includes fixed owner initialization and final inline root transfers as distinct existing32KiB-page work. No budget was increased.

Cancellation now retires active Component source/candidate/buffer through the exact typed cursor, insteadnot whole Component assignment. Full runtime regression, explicit active unwind/cancellation tests, existing-record equality adoption and complete resident accounting remain pending. The original inline-census RED remains unchanged.

```text
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=16
warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID 0bd98734-4082-479d-bd29-7778b5851491 with nextest profile: fundamental
    Starting 1 test across 1 binary (92 tests skipped)
       START [         ] (1/1) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_component_copy_charges_actual_surface_backing_before_publication

running 1 test
[DEBUG] surface-component-copy turns=15 reported=81781 ledger-allocation=32768 actual-allocation=32768
test reconcile::tests::ownership::surface_ownership_component_copy_charges_actual_surface_backing_before_publication ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 92 filtered out; finished in 0.01s

        PASS [   0.028s] (1/1) semio-framework-ui-runtime reconcile::tests::ownership::surface_ownership_component_copy_charges_actual_surface_backing_before_publication
────────────
     Summary [   0.032s] 1 test run: 1 passed, 92 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-lHP5x7



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs



```
