# Runtime Regression R24

Canonical runtime library gate with only existing `surface_ownership_inline_fields_do_not_allocate_a_second_owner` excluded:93 passed,0 failed,1 excluded,0.564s,exit0. The previously failed48KiB fixed cursor assertion remains unchanged and passes after replacing mutually-exclusive binding/Component copy slots with one tagged owner. No additional allocation or ceiling increase was introduced.

The exclusion remains an intentional actual accounting RED, so this is not a full all-tests pass or Process quota acceptance. New Component copy and old binding copy both retain exact typed cleanup through the tagged owner.

```text
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=19
────────────
 Nextest run ID 6b2f6ad0-ffec-4c75-8b35-70f638e89c4b with nextest profile: fundamental
    Starting 93 tests across 1 binary (1 test skipped)
────────────
     Summary [   0.564s] 93 tests run: 93 passed, 1 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-2ObBsn



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs



```
