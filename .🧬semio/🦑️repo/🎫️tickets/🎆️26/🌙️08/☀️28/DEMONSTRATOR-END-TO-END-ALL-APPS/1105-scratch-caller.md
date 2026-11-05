# Scratch Caller Owned Result Repair

Static review found the existing shared ScratchCaller::turn<R> passed generic owned R directly through StepContext::with_original_work_loan, whose actual result requires Copy. The helper now retains R in its outer Option and returns only unit through the same original one-fuel loan. The actual callback, caller, decoder, deadline, financial grant, heap/CPU receipts, stock and full-family 200-call assertions remain unchanged. Callback failures still propagate through the existing original caller result; expected refusal tests must continue retaining their complete nested Result as the owned callback output. This is a test helper typing correction, not new runtime behavior.

The existing owned literal, native parameter patch, scratch, scene and gesture laws exercise owned helper results. They have not compiled or run in the current native pass. One Rust file parsed with zero syntax errors. No native RED or GREEN is claimed.
