# Artifact Authority Durable Group Journal

## Boundary

The Store kernel now exposes an opaque `DurableOwnedGroupJournalRecordV1` only after decoding one canonical fixed-three decision Pack and matching its decision SHA-256. The DB artifact authority accepts that record through its existing actor mailbox and existing retained `ArtifactWal`; it creates one `WalRecord::Event`, preflights the full framed transaction, and submits it with `DurabilityClass::Fsync`.

The Store sink construction is non-I/O. Its retained commit states distinguish pre-handoff absence or rejection from an awaiting/failed append whose external state remains uncertain. A failed or awaiting owner cannot be terminally emptied, preventing an uncertain append from being converted into a false absence.

The authority now retains the same clone-shared `WorkerPoolUse` cell as its Database document-mount owner. Pool shutdown is therefore `Busy` while the journal mailbox/authority can still make progress. Database mount scheduling retains transient submissions behind one coalesced retry, but a hard scheduler refusal parks the exact job as `NonRunnable` with no timer and exposes an executor shutdown block. Neither path can turn an uncertain journal owner into absence.

## Exact ownership and admission

- No second WAL writer is acquired.
- The decision anchor document must equal the authority's document before WAL mutation.
- The canonical Store limit is 491,520 bytes.
- The independently calculated successor header is 165 bytes, the largest Store event transaction is 491,650 bytes, and their combined segment span is 491,815 bytes under the 507,904-byte storage limit.
- The exact physical maximum for a single event under the same framing is 507,609 bytes; one more byte is rejected.
- Cancellation before mailbox/WAL transfer proves `Absent`; canonical/hash/document/capacity refusal proves `Rejected`; append or sync failure remains a retained error.

## Laws and receipts

The strict fixture and schema are under `🗿️artifact/🧪️fixtures/📓️durable-group-journal`. The independent Bun oracle uses AJV 2020-12, Node SHA-256, UTF-8 byte lengths, and an independent canonical-varint/framing calculation.

- Direct oracle: GREEN, `AJV=1 cases=6 max-event=507609 store-margin=16089`.
- Registered Nx target `@semio-tech/framework-os-kernel:durable-group-journal-check`: GREEN, exit 0 with the same receipt.
- Rust parser gate for Store, WAL, artifact, and engine sources: GREEN.
- Native exact group is registered as `durable-group-journal-native-check` with three laws; native qualification is pending.

The exact native laws cover one canonical forced-Fsync event and replay, cancellation before handoff, and forged hash rejection before mailbox transfer.

## Nonclaims

This boundary proves only durable decision-journal append ownership. An Event record is not yet a durable decision recovery decoder, an inference WAL witness, or atomic three-Store publication. Those require a typed facade decoder and verified same-transaction replay before Hub acceptance.
