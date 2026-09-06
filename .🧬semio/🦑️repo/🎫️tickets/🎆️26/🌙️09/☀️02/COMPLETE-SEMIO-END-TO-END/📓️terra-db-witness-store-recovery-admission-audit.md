# DB Witness / Exact-Three Store Recovery Admission Audit

## Verdict

WG's proposed split is the correct narrow authority boundary **provided the
DB wrapper, rather than the lower Store host, is the only public source of an
authoritative recovered/acknowledged outcome**.  It does not require moving
WAL into Store.

The current source has already improved the prior recovery frontier:
`begin_store_owned_recovery` is private, so a raw public record and receipt
cannot currently start publication by themselves.  The newly declared
`DurableOwnedMapRecoveryAdmissionV1` is the right owner shape, but has no
constructor, transition, or `Drop`/terminal protocol yet.  It must not become
an unretained `ManuallyDrop` carrier while the DB bridge is added.

This was source-only; no build or runtime test was run.

## Current Authority Facts

The sole WAL parser is already narrow.  It consumes a
`WalCommittedTransaction`, records its immutable `(transaction_id,
segment_index)`, finishes the transaction, requires one sole `Event`, admits
the canonical Store Pack, and fences `record.document` to the replay document
([`artifact/🦀️.rs:380`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:380)).
It rejects duplicate/mixed bodies and a foreign document before it returns
`ArtifactCommittedDurableGroupDecisionV1` ([`artifact/🦀️.rs:407`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:407)).

That witness has private fields, but its crate-internal `record()` and
`into_record()` escape hatches remain ([`artifact/🦀️.rs:350`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:350),
[`artifact/🦀️.rs:370`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:370)).
They are not Hub-public, but retaining them after the recovery bridge exists
would make the sole consuming operation a convention within the DB crate,
rather than its unique transition.

Store correctly treats canonical record and journal receipt as **data**, not
as WAL proof: the record can be publically re-admitted from canonical Pack and
the receipt is cloneable, with public hash/transaction/segment fields
([`durable-group/🦀️.rs:237`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:237),
[`durable-group/🦀️.rs:289`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:289)).
That is safe only because the former public recovery entry is now private
([`durable-group/🦀️.rs:355`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:355)).
The Store script enforces this precise non-public condition
([`durable-group/📜️script.ts:102`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️script.ts:102)).

`DurableOwnedMapRecoveryAdmissionV1` owns three private `ManuallyDrop<Option<ArtifactStore<…>>>` slots
([`durable-group/🦀️.rs:899`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:899)).
There is currently no implementation or Drop implementation for it.  In
contrast, the existing recovery host keeps every Store/coordinator private and
asserts on nonterminal drop ([`durable-group/🦀️.rs:2021`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2021),
[`durable-group/🦀️.rs:2071`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2071)).

## Required Exact Transition

Use a one-way sequence, with no raw record/receipt in any Hub-facing result:

```text
WalCommittedTransaction
  -> ArtifactCommittedDurableGroupDecisionV1 (DB-private constructor)
  +  DurableOwnedMapRecoveryAdmissionV1 (three exact Store owners)
  -> ArtifactCommittedMapRecoveryV1 (DB-owned retained wrapper)
  -> lower Store host (private field of wrapper)
  -> internal lower acknowledgement
  -> terminal exact-three Store owners
```

`ArtifactCommittedDurableGroupDecisionV1::into_store_owned_recovery(self,
admission)` must consume both operands.  It must construct the lower receipt
itself from the witness's own record hashes and immutable transaction/segment
fields.  It must not accept a caller-provided receipt, transaction, segment,
record, or decision hash.  Before it transfers a Store, it must check all of:

- witnessed `document == admission/decision parent document`;
- record decision and anchor hashes match the internally derived receipt;
- receipt `transaction_id == witness.transaction_id` and
  `segment_index == witness.segment_index`;
- the lower fixed-three/frontier verification accepts the owned parent,
  drawing, and value Stores.

The wrapper alone maps the lower `AwaitingAcknowledgement` state to the
internal host acknowledgement.  The lower API must continue to expose neither
receipt nor acknowledgement authority; its existing private acknowledgement
already uses its internally retained receipt ([`durable-group/🦀️.rs:1990`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1990)).
After the bridge lands, delete the production `record()` and `into_record()`
methods.  Test-only byte assertions should derive their expectation before
creating the witness, not dismantle it afterwards.

The Store's public admission type need not itself prove WAL commitment: it is
only an exclusive owner of three already-authorized Stores.  That is safe when
the per-document actor is the only component allowed to retain or construct
it; a Hub request, raw event, journal receipt, or inference proposal must
never contain an `ArtifactStore` or admission.  The authoritative success
type must originate only from the consuming DB wrapper.

## P0 Before Exposing The Admission Constructor

`DurableOwnedMapRecoveryAdmissionV1` currently leaks its three Stores if it
is dropped because its `ManuallyDrop` fields have no terminal invariant or
retirement path.  Add its retained state and `Drop` protocol in the same patch
as a public constructor:

- `Pending` has no extraction/rebind API and asserts if dropped;
- pre-transfer validation rejection returns a typed rejected wrapper that
  retains the **DB witness and all three Stores**, or terminally closes all
  four; it may not silently discard the witness;
- once a lower host exists, all error/blocked states retain the exact DB
  wrapper and host; no `take_owners` exists before terminal completion;
- `AlreadyApplied` is a terminal, verified all-post result that returns its
  three owners once.  It is not a new publication or acknowledgement;
- cancellation after the DB witness is consumed is forbidden.  Expose no
  abort transition; shutdown retains/drives the same wrapper to terminal
  handoff.

The existing lower coordinator already has the correct no-cancel condition
for recovery (`recovery_committed` makes `cancel` return false)
([`durable-group/🦀️.rs:1519`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1519)).

## Minimum Native Laws

Add these to the established OS Rust exact-law selector
[`📦️packages/🦀️rust/📜️script.ts:824`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:824),
using the physical-WAL carrier and neutral corpus already used by
`committed_durable_group_decision_accepts_only_one_exact_event_transaction`
([`artifact/🦀️.rs:6148`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:6148)).

1. `artifact_committed_group_witness_consumes_into_exact_three_store_recovery`
   writes one forced-Fsync sole Event, scans the committed cursor, consumes its
   witness plus all-base Stores, publishes/acks once, and verifies no second
   WAL Event.  A reopened scan creates a fresh witness and all-post Stores
   report only `AlreadyApplied`.
2. `artifact_committed_group_recovery_rejects_raw_foreign_and_nonsole_before_store_stage`
   covers command-only, duplicate Event, Event-plus-command, aborted Event,
   foreign replay document, mismatched admission/document, and a receipt with
   mismatched tx or segment.  No row may stage a root or yield an authoritative
   wrapper.
3. `artifact_committed_group_recovery_retries_exact_owner_pre_and_post_visibility_faults`
   injects drawing faults at staging and adoption.  The first shows all-old,
   the second all-new; both retain the identical wrapper/three owners, produce
   no reappend, and end in precisely one handoff.
4. `artifact_committed_group_recovery_forbids_cancel_and_no_witness_escape`
   requests cancel after witness consumption and verifies it cannot abort,
   extract/rebind Stores, construct a DB receipt, or ACK.  Completion may only
   occur through the wrapper's internal acknowledgement.
5. `artifact_committed_group_recovery_mixed_or_tampered_post_is_terminal_rejection`
   gives one base/two post or a forged tail group-id/edit digest/snapshot Pack.
   It returns no success and preserves every entry Store byte-for-byte.  It
   must not downgrade to `AlreadyApplied`.

The present Store carrier law still deliberately hand-creates a receipt
([`durable-group/🦀️.rs:4022`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:4022)); retain it as lower-state coverage only, not as the authoritative DB-recovery proof.

## Scope

No real GIS per-document actor instantiates the fixed-three assembly today;
Hub remains `UnavailableGisMapApprovalCommitterV1`.  This packet validates the
next Store/DB ownership seam only and makes no production GIS approval or
recovery claim.
