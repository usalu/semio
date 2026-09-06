# Store-Owned Map Recovery Current Audit

## Outcome

The current recovery core has the right *state* semantics, but it does not yet
have the required exclusive durable-admission boundary.  A committed WAL
transaction can be narrowed to an `ArtifactCommittedDurableGroupDecisionV1`,
and the Store host retains all three Stores through publish, acknowledgement,
and the one terminal handoff.  However, the same Store publication path is
also publicly callable with a caller-built canonical record and a caller-built
receipt.  That makes the DB witness optional rather than authoritative.

This was a read-only source audit.  No build or test was run.

## What Is Sound Today

`committed_durable_group_decision_from_transaction` accepts a
`WalCommittedTransaction`, consumes its records, closes each record, and
finishes the transaction before it admits a record.  It accepts exactly one
`Event`, rejects a mixed/duplicate body, and fences the decision's document
against the replay document ([`artifact/🦀️.rs:410`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:410)).  The existing database law exercises sole-event,
mixed, duplicate, foreign-document, and aborted-transaction admission
([`artifact/🦀️.rs:6165`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:6165)).

At the Store boundary, `recover_store_owned` verifies all three member
references and owners, then accepts only all-base or all-post frontiers.  A
post frontier additionally checks the tail edit, group id, bound/unbound
hashes, edit digest, canonical edit bytes, and post snapshot bytes
([`durable-group/🦀️.rs:2659`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2659)).  Mixed state is rejected with all three
owned Stores returned; the current Store law covers all-post idempotence,
one post plus two base, and a hash-mismatched receipt
([`durable-group/🦀️.rs:3997`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:3997)).

The recovery host correctly has no cancellation path and no journal sink:
after the decision is committed it stages the three sealed outcomes, commits
only the shared visibility owner, then publishes/adopts/clears the three
members.  It returns `AwaitingAck` until its internally retained receipt is
acknowledged ([`durable-group/🦀️.rs:1450`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1450),
[`durable-group/🦀️.rs:1664`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1664)).  A capture fails closed if a
committed shared visibility has a missing or different member root
([`durable-group/🦀️.rs:804`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:804)).  The host owns the exact three
Stores until `take_terminal_owners`; dropping a nonterminal host asserts
([`durable-group/🦀️.rs:2010`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2010)).

## P0: Public Record And Receipt Can Manufacture Recovery Admission

`DurableOwnedGroupJournalRecordV1::admit_canonical` is public, as is
`begin_store_owned_recovery`; `DurableOwnedGroupJournalReceiptV1` exposes all
four fields publicly ([`durable-group/🦀️.rs:235`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:235),
[`durable-group/🦀️.rs:289`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:289),
[`durable-group/🦀️.rs:354`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:354)).  The recovery method verifies matching decision and anchor
hashes plus only `transaction_id != 0`; it does not itself verify a WAL
transaction or even give `segment_index` semantic weight.  Therefore a Hub
caller that has a canonical decision Pack can build a matching positive
receipt and start Store publication without a physical committed WAL
transaction.

The DB wrapper proves the intended source, but does not close this gap:
`ArtifactCommittedDurableGroupDecisionV1::begin_store_owned_recovery` takes
`&self` and delegates to that public Store method
([`artifact/🦀️.rs:349`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:349)).  The same witness may consequently be reused after its
first publication attempt, rather than requiring a fresh committed-cursor
scan for a replay attempt.

The planned `DurableOwnedMapRecoveryHostV1` must consequently be entered
only through a **consuming** DB-owned operation, for example
`ArtifactCommittedDurableGroupDecisionV1::into_store_owned_recovery(self,
parent, drawing, value)`.  It belongs in the per-document DB/Map owner; that
owner takes a fresh `WalCommittedTransaction`, mints one witness, transfers it
once, and retains the returned host until exact terminal handoff.  Hub must
receive neither `DurableOwnedGroupJournalRecordV1`,
`DurableOwnedGroupJournalReceiptV1`, raw transaction, nor segment identity.

Changing only the DB method to consume `self` is necessary but not sufficient:
the public Store recovery method remains a bypass.  Preserve public canonical
record admission/projection for the journal and inference verifier, but move
the *publication* entry behind a DB-owned privileged recovery adapter.  That
requires resolving the present kernel-to-DB visibility split deliberately;
an `rg`/convention that Hub does not call the public Store method is not an
authority boundary.

## Required Retained-Recovery Laws

Add these to the existing Store and DB native law families.  Each uses the
exact parent/drawing/value owners and a DB-derived committed witness, never a
hand-constructed `DurableOwnedGroupJournalReceiptV1`.

1. **`artifact_committed_group_recovery_consumes_one_sole_event_witness_without_reappend`**
   
   Append one forced-Fsync, sole canonical decision Event with the existing
   `ArtifactWal` fixture.  Obtain the witness solely via
   `committed_durable_group_decision_from_transaction`; move it into the
   recovery host, drive to `AwaitingAck`, acknowledge, and take owners.  The
   physical WAL transaction count and its `(tx_id, segment_index)` must remain
   unchanged.  Reopen/replay to mint a *new* witness; the three all-post
   Stores must return `AlreadyApplied`, never a new host.

2. **`artifact_committed_group_recovery_rejects_nonsole_foreign_or_raw_admission_before_store_stage`**
   
   Reuse the DB fixture cases for command-only, duplicate Event, Event plus
   body, foreign replay document, and aborted Event.  None may produce the
   recovery capability or stage a member.  Add a compile/source-negative law
   at the Hub-facing API: there is no public constructor taking a raw
   `DurableOwnedGroupJournalRecordV1` or receipt.  This law closes the P0
   above rather than merely re-testing canonical Pack parsing.

3. **`committed_group_recovery_mixed_frontier_returns_all_three_exact_owners_without_mutation`**
   
   Derive a DB witness, give it one exact post Store and two exact base Stores,
   and require `InvalidFrontier` plus all three owners.  Their generations,
   revisions, edit tails, and current snapshot Packs are byte-identical to
   entry.  Repeat with an all-post Store whose tail group id, edit digest, or
   snapshot Pack is tampered; it is neither `AlreadyApplied` nor partially
   published.

4. **`committed_group_recovery_previsibility_fault_retries_the_same_host_all_old`**
   
   With all-base owners and a DB witness, inject a recoverable drawing-stage
   failure after parent staging.  `advance` may return `Err`, but the same
   retained host remains in `StagingDrawing`; all triple captures remain old.
   Restore the dependency, drive that *same host* to acknowledgement and one
   owner handoff.  A request cancellation cannot drop or replace this host.

5. **`committed_group_recovery_postvisibility_fault_retries_the_same_host_all_new`**
   
   Inject a recoverable failure while adopting drawing after the shared
   visibility committed and parent adopted.  Captures must be either rejected
   for hostile root tampering or return the complete new parent/drawing/value
   tuple—never a mix.  Restore the dependency and advance the exact host;
   assert no journal append, no second group edit, one acknowledgement, and
   one terminal owner handoff.  This is the missing proof for retained failure
   after durable commitment.

The current Store law is a useful carrier proof, but it hand-builds a receipt
and starts from `DurableOwnedGroupJournalRecordV1`
([`durable-group/🦀️.rs:4001`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:4001)); it cannot substitute for laws 1–2.

## Ownership Rule For The New Host

The document actor—not a request/Hub handler—must retain
`DurableOwnedMapRecoveryHostV1`.  `advance` errors deliberately leave the
three Stores and coordinator in place, and nonterminal `Drop` asserts
([`durable-group/🦀️.rs:1970`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1970),
[`durable-group/🦀️.rs:2046`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2046)).  After a committed decision, forced
cancel/abort is not an alternative; the only legal path is retry/resume,
internal receipt acknowledgement, then `take_terminal_owners` exactly once.

## Separate Pending DB Shutdown P0s

These remain unrepaired and unqualified; they are not Map recovery claims:

- `Database::hello_retained` releases the DB-owned `WorkerPoolUse` after a
  precheck, then `DatabaseSyncHelloFuture::try_submit` reacquires independently.
  A live hello/session can therefore outlast `DatabaseShutdownProgress::Complete`.
- `DatabaseCreateCatalogState::release_success` can run while the public
  completion still retains `state: Some(self.clone())`; dropping an
  unconsumed result then parks a self-cycle after `finished`, rather than
  retiring it.

Their exact source evidence and repair direction remain in
[`📓️terra-db-retained-use-shutdown-current-audit.md`](./📓️terra-db-retained-use-shutdown-current-audit.md).
