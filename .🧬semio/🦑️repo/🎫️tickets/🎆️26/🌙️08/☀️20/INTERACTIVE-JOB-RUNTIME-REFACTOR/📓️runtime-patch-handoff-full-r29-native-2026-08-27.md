# Runtime Patch Handoff Regression R29

Canonical UI runtime library regression completed **96 passed, 1 skipped**, 0.600 seconds, exit 0. The one exclusion remains the intentionally failing physical inline-census test `surface_ownership_inline_fields_do_not_allocate_a_second_owner`; there are no new exclusions.

This includes the new structural handoff/unwind law, exact occupied-target and ACK rejection laws, contended credit/handback close, and published metadata-owner phase join. The unchanged actor fixture logical transitions now explicitly declare `owner-transition` granularity and name the typed first root `metadata`; the native test drives actual one-byte steps and verifies all nine surface bytes before those transitions.

Existing fixed runtime owner size assertions (48 KiB cursor, 64 KiB retained state) pass unchanged. This is not full resident-accounting or Process acceptance: the explicit inline-census failure is still open.

Actual output:

```text
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=23
────────────
 Nextest run ID 035db9bc-43ee-41eb-a104-e7f5426f9cd3 with nextest profile: fundamental
    Starting 96 tests across 1 binary (1 test skipped)
────────────
     Summary [   0.600s] 96 tests run: 96 passed, 1 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-0LahpV



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs
```
