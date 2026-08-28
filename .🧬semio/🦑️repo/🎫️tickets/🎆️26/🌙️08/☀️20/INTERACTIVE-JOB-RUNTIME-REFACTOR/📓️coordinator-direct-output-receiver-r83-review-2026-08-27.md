# Native Direct Output Receiver Review

## Executed Boundary

Root read the full R82 and R83 command/output report and the actual runtime implementation plus native test. R82 was a missing-API compiler RED with four E0599 diagnostics, not a runtime failure. R83 executed one selected native test:1PASS/120skipped,0.018s,exit0. The canonical task also executed its forty-check schema/Node Buffer oracle. Exact source and complete raw output remain in [the authored R82–R83 report](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️runtime-direct-output-receiver-r82-r83-2026-08-27.md).

The test really calls receive_job_into with a one-byte-short grant, an occupied current receiver, and the sufficient grant. It then panics after the actual successful transfer, outside the returned registry guard. It verifies the same original job shell and ready payload address, and explicitly closes the resulting roots. The declared transfer debit is8936bytes within the unchanged32768-byte grant. This is not a measured8ms certificate or instruction-level panic coverage.

## Source Review

The runtime's direct receiver validates the exact output queue/reservation/generation and empty receiver before taking the ready job. It places the ready patch directly into the pre-reserved pool entry, avoiding a detached intermediate whole ready result. The original job shell remains available for its separate close. Published and no-patch Empty are distinct results; Pending preserves ownership. Fixed64-entry/64-queue capacities are unchanged.

The actual producer path still must reserve this pool entry before producer invocation. The prior live Plugin R2 saturation test is still the clean RED (acceptedtrue instead of false after successful cleanup). The owner is changing that exact Plugin key/output admission region now; this review does not promote the standalone receiver into a mounted Plugin pass. Any newly introduced field or generation lifecycle must join the native close owner, not borrow current-route identity.

The test's caught callback panic occurs after the receiver returns. Registry poisoning during an internal panic, callback-tail quiescence, physical initialization/scan accounting and full final owner release are not established by this one selected test. They retain their separate protocol/test obligations. No quota, stack, timeout, pool capacity or compiler configuration was raised, and root started no native compiler.

## Next Gate

Run the unchanged original live saturation/retention cohort only after its source is coherent, through the sole compiler owner, then the full affected runtime suite and relevant Wasm width checks. Preserve the existing RED evidence and distinguish each actual scope.

