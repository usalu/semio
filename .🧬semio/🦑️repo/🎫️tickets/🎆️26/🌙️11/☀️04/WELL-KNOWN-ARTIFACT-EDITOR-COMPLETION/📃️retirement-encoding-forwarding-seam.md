# Retirement Encoding Forwarding Seam

Read-only current-source map; no source change or execution.

NativeRetirementDecodeControl (value/decode/retirement) contains the borrowed original NativeDecodeControl, paid hop capacity, spent capacity, and previous maximum. ledger_identity and retirement_recipient_identity read the unchanged original. charge checks capacity-spent and the original cumulative maximum before invoking the actual forwarded allocation port; it observes the same callback without allowing original cancellation to reject already-owned retirement.

StepContext::with_native_retirement currently calls the same OriginalJobDecodeOwner::retire with a callback derived from original_retirement or the actual deadline. It does not inspect cancellation. Inside with_original_retirement the callback follows the actual retirement override, while finite fuel and independently admitted retained currencies remain required. No new observer or maximum is necessary for accepted-result framing.

The smallest public seam is a synchronous with_encoding_receiver method on NativeRetirementDecodeControl, shaped like the existing decode encoding receiver: supply remaining capacity, the original-progress forwarding observer, and original-allocation forwarding port to the caller. Encoder new_forwarded then consumes those borrowed ports for one action. Remaining capacity must be min(paid hop capacity minus spent, original cumulative maximum minus owned). Allocations must pass through the equivalent of retirement charge so both ceilings and spent are updated. Simply calling original.with_encoding_receiver would bypass the wrapper's spent counter and is insufficient.

Existing scoped_encoding_receiver macro (value/receiving) already validates encoder owned offsets, cumulative ceiling, progress bounds, stage transitions and forwards the original allocation callback. It retains first refusal through Mutex and always preserves physical charges, including error/unwind. Reuse those validation semantics, but bind the retirement hop limit and spent field explicitly rather than minting a fresh native control with an accepting callback. Initial checkpoint may use the actual retirement observer, not the normal cancelled observer.

The method should preserve returned primary and forwarding refusals independently at the caller, along with actual written prefix/work/copy progress. No borrowed reference should escape the synchronous closure. Native regression needs same ledger+recipient identity before/after, cancelled original caller completing accepted finite JSON, denied capacity preserving offsets, and partial error/unwind retaining actual charge. This review supplies the API seam only; it does not claim those native laws have executed.


## Minimal reuse refinement

Root proposed direct reuse of original.with_encoding_receiver with a before/after owned-byte delta reconciled into retirement spent. This is sufficient under the actual admit_turn_capacity implementation: original.maximum_bytes is bounded by loan-start owned plus admitted capacity (and any prior whole ceiling); owned only grows. Prior charge increments owned, so the existing macro's remaining ceiling already enforces capacity-spent without an additional duplicate allocation port. The synchronous callback cannot access the wrapper to make a competing charge.

Therefore the previous warning applies only to unreconciled direct delegation. Delegation plus same-loan delta reconciliation is the smaller valid implementation. Reconcile on both Ok and Err, and use a nonallocating restoration guard for unwind so spent remains coherent with already-preserved original owned bytes. Any checked delta/add refusal must not discard an owning primary result/refusal. No counterexample beyond missing unwind reconciliation was identified.


## Permanent neutral accounting contract

Created JSON/schema/Bun test in value/decode/retirement/tests/encoding. Six rows cover original owned32, hop8, prior charge2, successful encoded3 leaving3, overspend refusal, accepted partial-error2 and unwind1 retained, observer denial before allocation, and exhausted authority refusing another allocation. Strict Ajv validates the neutral input; independent SQLite SUM and remaining-ceiling queries derive output accounting and preserve exact ledger/recipient identity. Actual test result: 1 pass, 0 fail, 14 assertions, 251 ms. This is the neutral accounting oracle; actual Rust guard and original-observer behavior remain subject to root native tests.

## Production Bridge

The actual NativeRetirementDecodeControl now forwards with_encoding_receiver to its borrowed original decoder. The existing original encoder bridge still checks observer cancellation, child allocation shape, original allocation port and cumulative ceiling. The actual Job retirement issuer supplies its own retirement observer; the page layer does not replace it. A nonallocating Drop guard charges the original owned-byte delta back to this same retirement hop on success, error and unwind. Its ceiling was already admitted as at most hop-start owned plus capacity, so prior charges and nested encoding cannot restore remaining authority.

A native Rust regression was authored before this API. It retains actual String births across successful output, partial refusal and unwind, verifies same ledger and recipient, exhausted capacity and refused observer, and closes the strings through paid controlled retirement. AST parsing passed for both Rust files. Native execution is queued with the single editor validation driver; no native pass is claimed.


## Landed Rust source review

The landed with_encoding_receiver delegates to the original bridge under RetirementEncodingSpend. Its Drop guard reconciles original owned-byte delta on success, error and unwind. Under current internal invariants original owned is monotone and bounded by loan-start owned plus capacity; spent plus that delta cannot exceed the admitted hop, so the subtraction/addition are valid without introducing another limit. The macro's offset/progress/refusal logic is unchanged.

The new native regression retains actual strings from successful encoding, partial error and unwind, then closes each through quoted ControlledRetirement steps with observed allocation/release parity. Observer refusal performs no allocation and preserves owned bytes. This is source review, not an executed native result.

A lightweight recursive explicit #[path] inventory from the actual value-native-decoding lib root, including cfg(test) paths and the new test, found 22 files and120421 bytes; manifest caps are32 files/262144 bytes/4000 active lines. The inventory counted684 nonblank/noncomment physical lines, not the authoritative active-line metric. No registered authoritative source-reader tool was available in this task context. These bounded source counts reveal no size reason to extract the new test or raise limits. Cargo's actual admission remains authoritative.
