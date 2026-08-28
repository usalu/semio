# Runtime Patch Handoff R27 Native GREEN

Canonical UI runtime selector `retained_patch_handoff_` completed **2 passed, 94 skipped**, 0.173 seconds. Source schema/Node Buffer oracle completed 23 assertions.

Ready publication and exact ACK now preflight borrowed source/destination slots and caller-admitted fixed initialization/move bytes before detaching any source. Ready, Published, and ACK expose exact-grant typed close; busy admission and handback mutexes return no progress with the exact private proof retained. Poison and invalid authority are errors, not discarded proofs.

The retained state uses one phase-qualified `UiPendingPatch` for either the unpublished patch or the published header, replacing the separate optional published surface root. Partial typed retirement survives handback without extracting a half-closed patch. Existing arbitrary Drop/global handback paths and other runtime retirement tails remain separate obligations; this is not full callback lifecycle or resident accounting completion.

The earlier four-call published-close fixture predates typed header traversal and still requires its exact source/native join. An additional structural callback-unwind law is queued. No full runtime pass is claimed here.

Actual output:

```text
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=23
warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID 99b48aef-938a-4715-bb57-438bdea374c3 with nextest profile: fundamental
    Starting 2 tests across 1 binary (94 tests skipped)
       START [         ] (1/2) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_close_respects_all_grants_and_contended_exact_credit

running 1 test
[DEBUG] patch-close grants=1,64,4096 exact-credit-contention=true exact-handback-contention=true
test reconcile::patch_handoff_tests::retained_patch_handoff_close_respects_all_grants_and_contended_exact_credit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 95 filtered out; finished in 0.02s

        PASS [   0.084s] (1/2) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_close_respects_all_grants_and_contended_exact_credit
       START [         ] (2/2) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment

running 1 test
[DEBUG] patch-handoff exact-slots=true occupied-target-preserved=true invalid-ack-preserved=true surface-bytes=4
test reconcile::patch_handoff_tests::retained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 95 filtered out; finished in 0.01s

        PASS [   0.080s] (2/2) semio-framework-ui-runtime reconcile::patch_handoff_tests::retained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment
────────────
     Summary [   0.173s] 2 tests run: 2 passed, 94 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-GSFZiL



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs
```
