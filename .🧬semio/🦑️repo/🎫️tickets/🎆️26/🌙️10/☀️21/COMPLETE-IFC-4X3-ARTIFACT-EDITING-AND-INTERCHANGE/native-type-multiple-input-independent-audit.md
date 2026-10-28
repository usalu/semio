# Multiple Required Reference Guidance Independent Audit

Source review of the worker's frozen host-type-multiple-input-implementation checkpoint found one P2 candidate custody defect, reported before finalization. No native execution/typecheck was performed. Initial review also observed the worker's conservative UTF8 quote correction128→192, accounting64zero-init+up-to64source-to-buffer+up-to64buffer-to-retained writes; the corrected source is being used for the continuing review.

## P2: local born String drops on construction/posting refusal

In details/type-guidance.rs phase28 (lines753–763 at the reviewed checkpoint), a new String is constructed by admit_original_construction into a local Option output. The call uses `?`; moving output into retained row_parts.strings[part] occurs only afterward on success. The actual framework construction API deliberately stores the born candidate into its receiver before quote/allocation/receipt reconciliation, and may return Err while that receiver owns the allocation (job/construction/🦀️.rs24–52). A refusal therefore unwinds the local Option<String>, dropping its owned allocation instead of retaining it for the original paid close ladder.

Use the actual retained Work string slot as receiver, or move any born local output into that slot before propagating BOTH success and failure. Add an original construction/posting-refusal law confirming ownership stays retained and then retires under original grants. This is a concrete source-contract defect, not a claimed executed native failure. Other reviewed local outputs contain unit only and mutate backing already retained inside Field/target, so the same finding is not automatically applied to those sites.

## Continuing checks

The progressive required roster and sorted raw-row merge use exact target indices, retain later raw payload text and keep fields editable when ready. Phase4 rejects extra/descending rows; placeholders carry per-slot original text-derived IDs, and readiness ANDs each slot before strict full tuple validation. Completed accessors borrow exact original row by ordinal rather than scanning the source graph. Missing bindings construct explicit sorted roster Set and Text, preserving layout replacement authority; present fields retain Preserve/Text.

RowParts uses separately funded String/outer/inner backing and a fixed closed schema, without a whole graph/request clone. Copy chunks end on valid UTF8 boundaries and retain source text without numeric normalization. Partial row/vector/edit/plan/field owners are present in the typed close roster; the phase28 local-owner exception above must be corrected before final source closure. Enclosing snapshot/fullhash/profile/caller/controller authority is unchanged from the earlier audited checkpoint.

Full final hash check and bounded no-additional-findings statement await the worker's custody correction. Native compiler ingress still refuses before rustc; all runtime/physical receipt/public-guided capability gates remain false.
## Final source closure

Worker corrected phase28 through construct_row_string receiving directly into retained RowParts.strings[part]. Only bounded fixed/index literals cross a20-byte stack carrier; raw sibling text reserves backing but is copied later in UTF8 chunks. On either construction or posting refusal, the retained slot already owns any born String before the error propagates and the ordinary failure latch prevents semantic retries. The original typed close roster retires RowParts before request/source owners. Source-adjacent type-guidance-tests.rs forces an actual construction overquote19 versus reserved20, asserts the born String stays in the real retained slot with no heap release, then retires through original close. That law is authored/parsed and unexecuted; the P2 is closed at source-contract level only.

Reviewed corrected quotes: phase28 includes20-byte literal initialization plus literal copy and retained String copy; phase20 includes192 for64zero-init+64copyin+64copyout. Published demand and actual construction/copy debit agree structurally; physical sufficiency remains unexecuted. Outer/inner row vectors each reserve separately; assembly consumes all7 Strings and both vectors into retained copied_row before receipt posting. Subsequent copied_rows and resident Set edit move under retained owners. No remaining local born heap receiver was found in this slice.

Final refreshed manifest independently matches11/11 current file hashes, recorded in generated/type-multiple-independent-hashes.json. No additional confirmed P1/P2 in scope. The initial finding above is retained as audit history and superseded by this closure.

## Independent actual TypeScript observations

Authored type-multiple-independent.ts uses the actual current form/bindings reducer/mutation builder against the shared declared two/four-reference source roles. All4 neutral edit sequences reach readiness while keeping every2/4 field and binding present, sorted exact input indices, source unchanged and expected4/5 central mutation count. Four-field reverse entry yields[3]→[1,3]→[1,2,3]→[0,1,2,3], not an append-only dead end. A145-byte raw invalid sibling draft with a multibyte character crossing64 bytes remains byte-for-byte unchanged after inserting the earlier field; next response retains2fields and ready=false. Actual output generated/type-multiple-independent.json. These are original TypeScript endpoint observations, not native row-copy execution or physical complex-chain proof.

Worker's selected237pass/711assert/21files, strict and external direct-tuple IfcOpenShell evidence remain separately attributed. No full repeated sweep or native execution was attempted here. All audit processes exited; no live source mappings remain. Native/type/physical receipt/public transport flags remain false.
