# Existing Record Equality Integration

The new owned UiComponentCompare cursor has native semantic/byte-grant/contention/cancel coverage, but is not yet installed in the old-record diff branch. That branch currently receives the old reconciler by shared reference, while the comparison owner requires both exact roots. Cloning the old Component wholesale to satisfy that API would reintroduce the work defect; moving it out of the old reconciler without an exact restore lease could make cancellation corrupt the retained old root.

The next integration must either retain an immutable old-record read root or explicitly transfer and restore the old component within the job's exclusive current-reconciler authority. It must preserve old-root reads and restore on cancellation/fault, not merely identify a borrowed pointer or reuse a cursor with arbitrary roots. The current job owns current/cursor/candidate separately in one retained state; that owner boundary is the appropriate place to bind the pair. No public borrowed-comparison escape or identity-only equality shortcut has been introduced.

Fresh-record Component copy is independently installed and tested, so this old-record authority issue does not block its bounded allocation/copy progression. It does block claiming the old-record semantic comparison path complete.

## Preferred Existing Domain Join

The UI contract already has exact UiDocumentLease ownership, generation-bound node ordinals, shared-alias handback and typed final retirement. The preferred root is that existing document domain, not a parallel Arc<Component> owner with a new allocation/last-drop protocol. A reconciler can retain one immutable document lease plus its id-to-ordinal index; a comparison captures one exact alias and ordinal with the incoming Component owner. Each comparison turn borrows that exact row under the existing arena try-lock and advances the typed comparator, without producing UiDocumentNodePage's current whole credited_clone.

Prerequisites before cutover: expose separately admitted document-node backing reservation/placement (current UiNodeTable.try_insert uses cold UiFixedList.try_push); bind the borrowed row operation to the exact lease/ordinal and stable node id; maintain fixed public wire order; use exact lease close on cancellation without waiting for the live old document; and join the new retained document root to every current/candidate/retirement/snapshot owner. Old-field equality/copy methods must remain visibly unproved until each is actually migrated. A temporary old-root mutation/restore or new generic callback read escape is not proposed.

The full resident meter must then charge actual document pages, id indexes, current/candidate/patch roots, pending field owners and retirement roots. Source tree allocation before admission remains separately open; replacing the old multiplier alone cannot prove Process fits.
