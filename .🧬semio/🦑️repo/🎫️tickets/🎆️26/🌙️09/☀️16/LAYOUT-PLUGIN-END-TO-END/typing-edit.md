# Original Typing Edit Boundary

Schema-first unmounted source packet. The existing typing commit owner is frozen and unchanged. No native compiler request is issued here.

The defining TypingLedger.send can yield one prior committed or empty transaction followed by the new edit's Open/Idle effect. A tool change, elapsed idle deadline, buffer change or Split fold commits the previous original transaction before the new run; a Net fold preserves its transaction. Empty replacement keeps a pre-existing transaction open but an empty first edit has no transaction. Exact canonical persisted configuration is root followed by typing.

The new neutral contract covers these nine cases including Unicode buffer identity, retracting overwritten keys via a shorter net, an empty original commit, and an empty new edit. Denial/cancellation/foreign caller+decoder/old-row retention assertions are also required of the native implementation.

Planned naturally independent package: semio-framework-tool-typing-edit at tool-machine/typing/edit. It depends on the first-party tool-machine ledger, job/native controls and value ownership; no external runtime library. The original ledger needs sorted Vec resident addresses for fundable growth/shift/release (same defining technique already used by ScrubLedger); the current BTreeMap cannot expose exact original allocation receipts for a first window. This defining change remains unmounted until the sole native snapshot terminates.

The fold seam must retain both the borrowed original entry slice and the incoming original Vec. An owned native decision is Split, ReplaceIncoming, or Net with an actually owned Vec. No callback may invoke the old Clone-bound fold. Writer is the sole concrete custom fold override and requires its actual native splice composition producer; other existing declarations use Split. The writer must stage all leaf keys, canonical state strings and transaction metadata before atomic slot publication, keep old metadata/entries for ordered output and paid retirement, and never retire a Vec IntoIter.

## Draft API and Atomicity Audit

The new source is unmounted and not source-ready. No Cargo manifest, workspace member or existing source dependency was modified. Current independent Ajv/SQLite/JSONPatch reference reports nine cases, nativeCompared false, zero native records.

Pure request is OriginalTypingEditRequest with actual owned window/tool/ActorId/buffer/Vec leaves and HybridLogicalTimestamp. Native fold decisions are Split, ReplaceIncoming or Net with an actual owned Vec. The fold provider borrows original keyed entries and actual incoming leaves under the same source/decoder and must physically close before pure accept_fold moves its decision.

A fixed-header exchange cannot create Vec<M> from the original Vec<(String,M)> in one bounded turn. Therefore the corrected draft stages the new row and only final take_result exchanges the resident slot into an actual receiving result containing the previous owned TypingState plus Open/Idle. The SDK must project its old entries boundedly into the actual Emit and keep every metadata/key/vector owner until paid retirement.

Postexchange cancellation requires an explicit rollback or irrevocable acceptance contract. The proposed rollback retains original keys, metadata and entry Vec capacity during projection, reconstructs the exact previous row boundedly, then uses the fresh resident generation to restore it under genuine original retirement. Reuse folds retain displaced original entries and deadline for a direct restoration. Pure close must refuse silent retirement while an unacknowledged exchange remains. This protocol is under coordination; the draft must not be mounted or claimed complete before implemented and tested.
