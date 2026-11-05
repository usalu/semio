# 🗃 Durable Local Catalog Recorder Audit

Current LocalCatalogConfigMutation is admitLocalDocument carrying LocalDocument fields documentId/schema/name/storage/target/admittedAtMs, or retireLocalDocument carrying documentId. Admit diff stores all six explicit fields and replaces by key; retire removes by key. If owner metadata is added, direct admit/diff/codec twins and equality predicates must all carry it.

Specific mutation reflected predicate: for admit find matching documentId and compare every stored field exactly; for retire require no entry with that documentId. Additional concurrent unrelated documents and ordering must not prevent acknowledgment. Whole-catalog equality would incorrectly fail concurrent additions.

Current worker dispatch returns local catalog events before localDocumentOwnersRef subscribers. handleLocalCatalogEventMessage must forward catalog-client status and documentArchiveReplaced archive to the observer while retaining normal projection updates. remoteMutations and optimistic state are not durability acknowledgments. Current status subscriber receives persisted boolean only; use full status requiring persisted,pendingMutations0,no writeFailure before requesting externalChanged readback. The decoded readback must satisfy specific reflected predicate before resolution. A status alone may describe prior writes.

Capture worker and localCatalogClientInstanceId per operation, and verify current generation in callbacks. Serialize operations with recovered promise tail; failure/cancellation must not poison queue. One watcher slot exists per client: cleanup must compare observer identity before deleting or restoring predecessor. Settle once, clear timer, remove abort listener, and reject on generation replacement/unmount. Update localCatalogRef when adopting authoritative decoded archive, not only React state; otherwise queued diff may observe stale projection. Additional actor-side concurrent docs must survive sparse mutations.

Existing writeLocalDocumentRef demonstrates status->externalChanged->byte-identical archive readback and deadline cleanup, but it creates separate client per document. Catalog uses shared long-lived client so subscription lifetime/ownership needs stronger guards. Current ordinary document status dispatch also omits writeFailure, which can hide typed failed-save state independently of catalog changes.

Root notified of these exact seams. No product edits or tests in this audit.
