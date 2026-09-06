# Artifact Authority Durable Group Journal

## Boundary

The Store kernel now exposes an opaque `DurableOwnedGroupJournalRecordV1` only after decoding one canonical fixed-three decision Pack and matching its decision SHA-256. The DB artifact authority accepts that record through its existing actor mailbox and existing retained `ArtifactWal`; it creates one `WalRecord::Event`, preflights the full framed transaction, and submits it with `DurabilityClass::Fsync`.

The Store sink construction is non-I/O. Its retained commit states distinguish pre-handoff absence or rejection from an awaiting/failed append whose external state remains uncertain. A failed or awaiting owner cannot be terminally emptied, preventing an uncertain append from being converted into a false absence.

The authority now retains the same clone-shared `WorkerPoolUse` cell as its Database document-mount owner. Pool shutdown is therefore `Busy` while the journal mailbox/authority can still make progress. Database mount scheduling retains transient submissions behind one coalesced retry, but a hard scheduler refusal parks the exact job as `NonRunnable` with no timer and exposes an executor shutdown block. Neither path can turn an uncertain journal owner into absence.

Replay now has a crate-private `ArtifactCommittedDurableGroupDecisionV1` boundary. It is constructed only by consuming a `WalCommittedTransaction`; it accepts exactly one committed `Event` body, re-admits the canonical Store record, checks its document against the replay document, captures the transaction and segment identities from the WAL gate, and finishes the transaction before returning. Transactions without an Event are ignored. An Event mixed with another body or a second Event is rejected before any projection escapes. No API accepts an Event plus caller-supplied receipt or transaction metadata.

## Exact ownership and admission

- No second WAL writer is acquired.
- The decision anchor document must equal the authority's document before WAL mutation.
- The canonical Store limit is 491,520 bytes.
- The independently calculated successor header is 165 bytes, the largest Store event transaction is 491,650 bytes, and their combined segment span is 491,815 bytes under the 507,904-byte storage limit.
- The exact physical maximum for a single event under the same framing is 507,609 bytes; one more byte is rejected.
- Cancellation before mailbox/WAL transfer proves `Absent`; canonical/hash/document/capacity refusal proves `Rejected`; append or sync failure remains a retained error.

## Laws and receipts

The strict fixture and schema are under `🗿️artifact/🧪️fixtures/📓️durable-group-journal`. The independent Bun oracle uses AJV 2020-12, Node SHA-256, UTF-8 byte lengths, and an independent canonical-varint/framing calculation.

- Direct oracle: GREEN, `AJV=1 cases=6 witnesses=6 max-event=507609 store-margin=16089`.
- Registered Nx target `@semio-tech/framework-os-kernel:durable-group-journal-check`: GREEN, exit 0 with the same receipt.
- Rust parser gate for Store, WAL, artifact, and engine sources: GREEN.
- Native exact group is registered as `durable-group-journal-native-check` with four laws; the newest law is native-pending.

The first exact native law constructs the opaque witness from the committed transaction and checks its document, transaction, segment, canonical record, and decision identities. A fourth law executes all six neutral witness rows against physical WAL: one canonical Event is admitted, an ordinary Command and an aborted Event are invisible, and Event-plus-Command, two-Event, or foreign-document transactions are rejected only after their borrowed owners are finished. The remaining laws cover cancellation before handoff and forged hash rejection before mailbox transfer.

The first native attempt `bUcnaf/00` built and listed the group, then exposed
that the deliberately unrelated control transaction used memory durability and
was absent from physical replay. The fixture now submits that control
transaction with `DurabilityClass::Fsync`, preserving the intended two committed
transactions while admitting exactly one decision witness. This correction is
source-qualified and awaits the exact native rerun.

Root receipt `ZJRrWu/00` is GREEN for the three-law boundary, executable SHA-256
`0349c09335867ba4d64bb04258c7483bdafc674a302616822c6877a789bd64a5`.
It qualifies the two-transaction/one-witness Fsync path and the retained journal
shutdown cursor. The six-vector committed/aborted/mixed/foreign witness law was
registered after that run and remains native-pending.

## Nonclaims

This boundary proves durable decision-journal append ownership and an opaque committed-transaction read witness. It is not yet an inference approval witness or atomic three-Store publication: the retained GIS owner must still pass the witness's private record through `recover_store_owned` against the exact three live Store frontiers, publish through the mounted fixed host, and reopen through the same recovery path before Hub acceptance.
