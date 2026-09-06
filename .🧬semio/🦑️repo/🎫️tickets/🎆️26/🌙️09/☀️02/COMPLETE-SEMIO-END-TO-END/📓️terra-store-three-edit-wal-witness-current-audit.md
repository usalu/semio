# Store Three-Edit WAL Witness Boundary

Read-only audit, 2026-09-06. No source or runtime changes were made.

## Current result

`DurableOwnedGroupVerifiedThreeEditsV1` is a useful **read-only canonical projection**, not a durable-publication or replay authority. It cannot itself mint the Store's pointer-sealed `ArtifactStoreOneItemPrepared` authority, but it has no WAL transaction witness and no live three-Store frontier check. It must therefore never be the object accepted from Hub/browser input, nor the object that applies an approval.

The narrow safe next boundary is an opaque, DB-owned `ArtifactCommittedDurableGroupDecisionV1` constructed only while consuming one `WalCommittedTransaction`. It owns the admitted Store record plus the exact replayed `{document, transaction_id, segment_index}`. Its constructor is crate-private; Hub receives only its verified projection/read-model result, never a caller-supplied Event, receipt, or transaction identifier.

## What the Store code does prove

- `DurableOwnedGroupJournalRecordV1::verify_fixed_three_edits` re-decodes the canonical Pack, rechecks decision/anchor/document equality, then verifies all three bound member outcomes before producing the three `Edit`s. See [durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:289).
- Each projected edit is tied to the decision hash, member base/post frontier, canonical unbound hash, semantic metadata, edit digest, and a canonical decodable post snapshot at [durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1995). The fixed parent/drawing/value ownership and schema identities are checked in `validate` at [durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2767).
- The projected values are not sealed Store owners. `Edit` is public and cloneable, but the live authority has private fields and actual publication validates an address-bound private seal at [store/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13073) and [store/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13148). A projection therefore cannot directly manufacture an `ArtifactStoreOneItemPrepared` seal.
- The correct recovery path is already private and Store-owned: `recover_store_owned` checks the identities of all three live Stores and requires all three to be at the same base or all three at the exact post frontier; it rejects mixed state. In the `Apply` case it reconstructs fresh private sealed prepared owners. See [durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2347), especially [line 2362](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2362). It also recalculates the real Store post revision before sealing at [line 2275](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2275).

## Critical current holes

### P0 — event-only projection has no committed transaction or receipt binding

`verify_fixed_three_edits` accepts only `&self`; neither it nor `DurableOwnedGroupJournalRecordV1` accepts a `WalCommittedTransaction`, transaction id, segment index, or committed Event span. `DurableOwnedGroupJournalReceiptV1` is four public fields at [durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:235), so it is an asynchronous journal acknowledgement, not a replay-proof token.

The WAL has exactly the needed source of truth: `WalTransactionGate` only exposes bodies after matching `TxBegin`/`TxCommit` and exact `record_count` at [wal/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1360); `WalCommittedTransaction` exposes the authoritative `transaction_id`, `segment_index`, and body count at [wal/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1899). No current Store API joins them.

Consequences if Hub treats the current projection as approval authority:

1. A canonical Event copied from transaction A can be paired with a forged/reused `JournalReceiptV1` for transaction B; neither the Store record nor projection detects that mismatch.
2. A historical valid canonical Pack can be re-admitted directly: `JournalRecordV1::admit` is public at [durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:289). It validates format and consistency, not an unforgeable live Store provenance.
3. The current document authority accepts that record, checks only `document.artifact_id`, and writes a one-body `WalRecord::Event` transaction. It does not reject a replayed decision id or validate the three current Store frontiers. See [artifact/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1720). Its public direct append route is at [line 4319](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4319).

This does **not** let the projection forge a private Store seal. It does mean that Event bytes alone are not authority to approve, replay, or publish a Map group.

### P0 — parent edit decoding is not parent-state reconstruction

`verify_bound_member_edit` reconstructs an `ArtifactStoreOneItemLiveAuthority` from values in the Pack solely to validate the edit's semantic shape, and merely checks that the encoded post snapshot decodes/canonicalizes; it does not have a live parent Store or recompute its post revision ([durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2012)).

Thus an inference/replay integration must not reconstruct and apply only `verified.parent()`. It must run the existing `recover_store_owned(parent, drawing, value)` under the one retained three-Store owner. That is the only present code path that:

- binds the parent artifact and both required child owner identities;
- compares each live generation/revision to the decision's base or post frontier;
- rejects a one-or-two-member partial application; and
- computes the actual parent/drawing/value post revisions before reconstructing private seals.

## Bounded witness and handoff

Keep Store independent of DB/WAL. Place the witness extractor beside the document authority, not in `store`:

```rust
// db/artifact: constructor crate-private; no public receipt/bytes constructor.
pub(crate) struct ArtifactCommittedDurableGroupDecisionV1 {
    document: ArtifactId,
    transaction_id: u64,
    segment_index: u64,
    record: DurableOwnedGroupJournalRecordV1,
}
```

The extractor consumes one `WalCommittedTransaction` before `finish()`:

1. Require `record_count() == 1`.
2. Decode exactly its sole `WalRecord::Event`; reject every other body kind, a second Event, and all mixed `Command`/`Frontier`/`Outbox` bodies. The current writer already emits exactly that grammar at [artifact/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1744).
3. Retain/copy the bounded Event bytes, construct the Store record with `admit`, and require `record.document().artifact_id == replay_document` before retiring the WAL record.
4. Capture `transaction.transaction_id()` and `transaction.segment_index()` in the opaque witness, retire the body, then `finish()` the transaction. Never accept either identifier, an Event body, or `DurableOwnedGroupJournalReceiptV1` from Hub/browser input.
5. Hub may use `witness.project_fixed_three_edits` only for an event/read model. The approval/recovery side passes its internal record directly to Store's `recover_store_owned` while it owns all three live stores. `Apply` proceeds through the fixed host; `AlreadyApplied` is idempotent; any mixed/stale/altered frontier fails closed.

For a clean authority surface, make `JournalRecordV1::admit`, raw Pack extraction, and `ArtifactAuthority::append_durable_group_decision_retained` crate-private or otherwise internal to the Store coordinator/DB actor. The public journal sink should transfer an already Store-owned record, rather than taking arbitrary `(decision_pack, decision_sha256)`. This is a focused API shrink, not a new generic auth layer.

## First executable laws

1. **Committed single Event only.** A physical valid WAL transaction with one canonical fixed-three `Event` yields an opaque witness whose `{document, tx, segment}` equals `WalCommittedTransaction`; `TxAbort`, torn tail, an Event outside a transaction, or record-count mismatch yields none. The WAL gate already provides the structural acceptance at [wal/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1360).
2. **No cross-transaction substitution.** Two committed transactions containing distinct valid Event Packs must create two different opaque witnesses. There is no API that accepts event-A plus transaction-B/receipt-B; an event replay in a later transaction carries the later tx id and must go through the live frontier recovery, not inherit A's authority.
3. **No event-only partial grammar.** A committed transaction containing canonical Event plus `Command`, `Frontier`, `Outbox`, or a second Event is rejected by the durable-group witness extractor before it exposes a projection. This stops accidental reuse of a generic ArtifactEngine transaction as the Map decision witness.
4. **Altered parent and mixed group states fail before publication.** Starting from a valid Pack, advance only the parent Store or alter its generation/revision; `recover_store_owned` must return `InvalidFrontier` and hand back no bound apply owner. Repeat with all three exact post states and require `AlreadyApplied` only. The existing Store recovery law around [durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:3576) is the closest fixture seam.
5. **Direct raw append is unavailable.** Outside Store/DB internal visibility, no caller can construct a journal record from captured bytes or submit one directly to an `ArtifactAuthority`; only the fixed-three coordinator reaches the journal sink. Keep the current projection test at [durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:3459) as the decoding baseline, but it is not this authority law.

## Qualification status

The existing journal native law proves one exact Fsync Event and reports its receipt, but its replay merely collects Event bytes and asserts equality; it does not construct a witness, test a transaction mismatch, or call live Store recovery ([artifact/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:5320)). No Hub inference, browser, or user approval claim follows from the current Store projection.
