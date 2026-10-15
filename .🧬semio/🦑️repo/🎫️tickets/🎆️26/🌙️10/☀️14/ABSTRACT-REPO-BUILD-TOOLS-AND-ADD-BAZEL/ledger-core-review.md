# Incremental Optional Ledger: Independent Core Review

This is a read-only Source review of High's durable ledger and its real storage/record/observation callsites. Core executed no test, producer, profile or SQLite probe for this review. High's focused runtime evidence remains separately owned and is not independently rerun here.

## Durable Ordering

The separate operation-gate database holds exclusive custody while the accounting database commits `dirty=1`, the synchronous filesystem callback runs, and the accounting database commits clean exact usage. Releasing the gate does not roll back the already committed dirty intent. A process failure before intent commit cannot have entered the physical callback; a failure after it leaves durable dirty state for the next handle. A surviving handle checks that state before admission and inventories physical optional leaves before making it clean. Callback failure or failed clean commit likewise leaves dirty recovery pending. Successful clean publication applies the declared retained delta while the operation gate is still held.

Every opener also performs an unconditional physical reconciliation. This covers older rollback-only state and quiescent image maintenance performed outside an already open ledger. Initialization is independently serialized through the existing original resource lease; short synchronous gate/accounting transactions provide cross-handle publication serialization. Busy refusal remains an optional persistence denial, not an invented accepted publication.

Retirement takes the same operation gate, commits dirty intent before removing the owned namespace, then measures actual remaining optional bytes and commits them clean. Global optional-cache maintenance retains genuine image-exclusive custody, which excludes live shared-image ledger users; a subsequent opener reconciles actual bytes. It does not reclaim immutable pins, mandatory publications or arbitrary inference files.

## Control, Physical Bounds and Integration

Admission checks fixed pool usage plus the complete declared write peak before callback entry. Filesystem headroom additionally reserves both bounded SQLite databases and their maximum DELETE-journal peaks. Configured page caps, FULL synchronous mode, disabled cache spill and no WAL/SHM constrain metadata growth. Successful updates avoid an inventory scan; dirty recovery and initialization retain bounded inventories with original control checks.

The optional NP and record callsites retain their exact byte/temporary/replacement plans. Local reservations are released in finally; the shared ledger remains the cross-handle admission authority. Computed values and Source/mandatory publications remain outside optional-denial suppression. Existing original control, live image identity, writer namespace checks and record-descriptor registration remain in place.

Core found the SQLite sidecars were initially checked only after SQLite open/configuration could touch them. The final ledger reread, SHA `ef157658c605f54a316c1cb6d0cb09e4985ada9dd908f97f86a8aa53be8cf347`, resolves this with pre-open bounds at both opens. It also checks original control immediately after both awaited database opens, before configuration. Explicit failure flags preserve unrelated primary errors during gate rollback, including a thrown undefined value; postcommit bounds/control checks are retained. Both database close attempts run, with the primary close failure preserved; failed opening also attempts both closes without replacing its original error.

Ledger initialization now requires the bounded unavoidable metadata capacity. Optional publication still checks the original free-space floor plus declared physical peak and metadata reserve. This does not change the surrounding local quota constructor or image-maintenance admission policies; it is not a claim that every FactStore can enter below its existing floor.

## Resolved Record Cleanup Finding

The earlier record Source accounted cleanup for every old `*.tmp` entry, although the quota inventory counts only exact current pack/index temporary spellings. Deleting an uncounted `other.tmp` with a negative retained delta could understate the valid optional files left behind. Core reported this to High and Root.

The final record reread, SHA `fcd3af70ca6f745503b233292cf15c6b39d19c22101fe79e8bdfe46c0cf1c6c6`, defines exact `PACK_TEMPORARY` spelling and skips unmatched names before stat, deletion or accounted publication whenever that publication interface is present. The admitted spelling matches the existing quota whitelist and the current pack/index temporary writer. The remaining Core blocker is resolved. No further material blocker was found in this bounded review.

High reports its final focused gate as seven passing tests / 185 assertions / 14.16 seconds in `🗑️generated/ledger-incremental-final2.log`, including actual Node API and crash/cleanup cases. Core did not execute or independently aggregate that gate; this report's final verdict is Source review only.

## Reviewed Paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/📒️ledger/🟨️.mjs`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/📒️ledger.json`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗃️records/🟦️.ts`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs`.

No current graph acceptance, crash-runtime result or end-to-end performance claim follows from this Source review alone.
