# Encoded Record Refused Payload Witness Review

Read-only current branch and proposed future witness. No source edits or runtime claim.

Actual insert branch returns InvariantViolated only for a new unique ID when len equals actual admitted capacity; it retires incoming payload before returning. A replacement identity at full capacity is accepted, transfers the new payload and retires the previous value. Root's eleven-cardinality allocator law always inserts declared-count distinct primitive fields and therefore never reaches this branch.

Meaningful narrow-stack witness should iteratively construct a deep single-child Block/Record payload, create a full one-slot EncodedRecord, replace the existing ID at capacity, then refuse a distinct ID carrying that deep owner. It should assert replacement success, precise refusal kind, original committed field/len preserved after refusal, unchanged control owned ledger and successful narrow-stack thread completion. Rejecting duplicate ID is not the overflow branch; count and identity differ and must be authored explicitly. Dropping or asserting the deep payload after its transfer must not accidentally introduce recursive cleanup in the test.

No zero-allocation cleanup assertion is valid for current retire_field: its explicit pending Vec frontier and error-message allocation are separate requests. If observing requests, distinguish previously measured record backing layout from retirement frontier/error storage and exclude fixture construction/assertions. This supports no unadmitted record growth and iterative stack behavior, not allocator-refused retirement. A future allocator-refused law requires a separate actual retirement implementation authority.

Shared/OS codegen typed insert propagation is already reviewed in the paired report. No staged payload law was present in borrowed-object file at this read time. Existing small-five logs visible here remain older compiler-only receipts; no newer post-port feature verdict inferred. Root owns current selection and execution.
