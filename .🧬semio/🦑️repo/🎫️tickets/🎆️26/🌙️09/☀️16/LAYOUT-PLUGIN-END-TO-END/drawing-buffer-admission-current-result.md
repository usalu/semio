# Actual Affected Admission and Caller Result

One captured normal owning run4516 completed EXIT1:49 actual tests,46 pass/3 fail,9096 assertions,634ms. Sequential captured strict16 session97211 completed EXIT2:two diagnostics solely in the ticket actual-registration importer, where expression-body callbacks return the original helper summary instead of void. All54 exact pre/test-post/strict-post/current hashes match. No source fix or retry has been applied. Exact logs and full hashes are in generated/drawing-ts-buffer-admission-current/launch-receipt.json.

The actual runtime failures are:
- close driver records actual paid blocked admission while a denied admission stays zero: turn/tests/work.test.ts:9 expected paid work1 but actual0.
- absolute cached nested demand keeps held copy and depth stable: absolute/absolute.test.ts:12 parent slot expectedtrue but actualfalse.
- nested retirement driver preserves its absolute pending allocator and close quotes: absolute/absolute.test.ts:17 enter parent slot expectedtrue but actualfalse.

Source diagnosis: real page body birth quotes depth3. The old turn fixture wrapper still supplies depth2, before the retirement driver adds its real frame. Its successful-body turn therefore legitimately refuses. Both absolute seed constructors still give depth1; seed.contiguous correctly refuses and leaves zero capacity, so later parent slot cannot hold a row. These are retained old scalar-depth assumptions, not evidence that denial or CPU guards should be weakened. Once a body is genuinely admitted, its next slot quote can drop from birth depth3 to existing-row depth1; the absolute test's post-admission comparison must use that actual next quote, while all repeated denied quotes remain unchanged.

Proposed narrow correction for Root review only: preserve all shared JSON/stock/fuel and all denial/expiry assertions; derive seed body depth from parent.nextStorageDemand().depth; derive successful driver body depth from its actual discovered pending quote; make capacity-only denied body test supply its independently required absolute depth. Assert the actual next slot depth after the successful birth rather than the old birth depth. Add braces to the two ticket test callbacks to preserve execution but return void. No production change is indicated. All54 held until Root independently reviews the terminal evidence.

Direct admission/body/link/row CPU and provenance cases passed, including error-after-effect, expired loans and same-ledger resumed holds. All page, helper, Byte5/PNG5, numeric7 and original two primitive laws passed in this run. This does not establish whole Font/Drawing readiness or make a red affected suite green.
