# Store-Private Committed Three-Store Recovery

## Outcome

The Store fixed-three coordinator now has a committed-recovery mode for an
already verified decision. It reconstructs the exact parent, drawing, and
value prepared owners with the existing `recover_store_owned` fence, stages
them under one shared visibility root, skips journal submission, and requires
the original retained receipt acknowledgement before returning all three Store
owners exactly once. Cancellation is unavailable after commit.

The language-neutral fixture classifies the four required states:

- all base frontiers apply once;
- all post frontiers are already applied;
- any mixed base/post tuple is rejected;
- a foreign receipt is rejected.

The exact Store law additionally exercises all four classifications and proves
that the recovery coordinator has no journal sink or reappend path. It now
injects a missing drawing retirement factory after the parent has staged and
again after shared visibility commits while the parent is already adopted.
The first fault leaves the same host at `StagingDrawing` with the complete old
triple visible; the second leaves it at `AdoptingDrawing` with the complete new
triple visible. Restoring each exact factory lets that same noncancellable host
reach acknowledgement and one terminal owner handoff.

## Authority Boundary

The kernel now exposes a one-shot lower restoration admission that owns exactly
three Stores. Its advance projection exposes no receipt or approval ACK, and an
unconsumed admission has an explicit `into_owners` handback. This lower seam is
not a committed-WAL authority: its record and receipt remain caller-readable
data. Its public bounded turn now reports an explicit retaining `Fault` rather
than `Result`, and its read observation is `Option`; neither lower operation
offers a `?` path that can unwind across its ManuallyDrop Store owners.

The DB opaque witness is minted only from one consumed
`WalCommittedTransaction` whose sole body is the canonical Event. Its former
`record()` and `into_record()` escapes are removed. The sole consuming route
combines it with the three-Store admission in a noncancellable retained state
machine. DB derives the lower receipt from the witness's admitted hashes,
transaction, and segment; a lower rejection restores the same witness and all
three Stores for exact retry. A caller that elects fail-closed retirement after
that pre-mutation rejection can consume the wrapper through
`take_rejected_terminal`; this destroys the witness and returns only the three
Stores, typed rejection, and derived metadata. A later recovery fault retains
the same lower host. The wrapper's bounded `advance` surface reports that state
as an explicit typed `Fault` turn, not a `Result`: an outer `?` cannot unwind
over the retained witness and Stores. Only a terminal DB handoff carries the
three Store owners and committed metadata.

This is a production-shaped ownership boundary, not live Hub acceptance. No
GIS per-document actor owns the wrapper yet, and Hub must never pass a record,
receipt, Event, transaction id, or segment id. The
`UnavailableGisMapApprovalCommitterV1` remains fail-closed until such an actor
mounts and drives this exact wrapper.

## Verification

- `rustfmt --edition 2021 --emit stdout` parsed the strengthened Store source.
- `@semio-tech/framework-os-kernel:durable-owned-group-decision-check` is GREEN:
  `AJV=1 SHA256=1 roles=3 cases=8 event=491520 whole=491779 member=162000`.
- `@semio-tech/framework-os-kernel:durable-group-journal-check` is GREEN:
  `AJV=1 cases=6 witnesses=6 recovery=4 max-event=507609 store-margin=16089`.
- The journal source oracle also rejects any raw `record()`/`into_record()`
  escape from the DB witness and requires the consuming recovery state. Its
  new physical negative law writes the canonical decision as the sole Fsync
  Event, mints the opaque witness only while consuming WAL replay, denies a
  mismatched three-Store frontier before mutation, destroys the witness, and
  returns all three exact Store owners for bounded terminal close. It compares
  every returned Store's Pack, SPR, artifact reference, owner reference,
  generation, and folded projection against its pre-admission value.
- The current Store native gate is GREEN15 at
  `durable-owned-group-decision-exact/exact-cargo-laws-zMvcm3/00`, executable
  SHA-256 `b98e404757d5f0f3e5b7ee667308e1d33cf0846e7a122b1952b132c9f767f75a`.
  This includes the private committed-record recovery law. The first current
  attempt exposed that the checked binary Pack vectors predated the canonical
  encoder; all three unbound outcomes, their independent hashes, and the
  unsigned decision digest were regenerated together. The AJV/Node/WebCrypto
  source oracle and native decode/re-encode law both pass the new corpus.
- The current DB journal gate is GREEN5 at
  `durable-group-journal-exact/exact-cargo-laws-ySopjz/00`, executable SHA-256
  `e22ac7171980ae9710b5281856fb87c0afcb8b3be0f6847c58c637b4e37ea259`.
  It includes the hostile sole-Event transaction law and the physical
  pre-mutation rejection handback law. An immediately preceding receipt,
  `bpQjEL/00`, passed its first four laws but did not terminate the authority in
  the hash-before-mailbox cleanup law. The current rerun passed all five after
  adding an exact retained-driver failure witness; because that diagnostic did
  not change runtime semantics, the earlier cleanup miss remains an unresolved
  transient rather than a claimed fix.

## Nonclaims

- No Hub approval or live per-document production recovery consumer exists.
- The public production-shaped assembly accepts real typed Stores, factories,
  and a real journal sink, but no GIS per-document actor constructs it today;
  its executable laws still use Demo stores and `FakeJournalSink`.
- The Store-only receipt fixture does not prove a physical WAL commit; only the
  separate DB opaque-witness law derives transaction and segment identity from
  `WalCommittedTransaction`.
