# Terra DB Committed-Witness Recovery Current Audit

Status: read-only current-source audit on 2026-09-06. No native build or law was run. The concurrently reported mount build failure is not used as evidence here.

## Verdict

The new DB wrapper closes the earlier *raw record → DB witness* hole at its own boundary. `ArtifactCommittedDurableGroupDecisionV1` has no public constructor, receives its record only from a consumed `WalCommittedTransaction`, and `into_store_owned_recovery(self, admission)` consumes it ([DB artifact](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:350)). The wrapper retains that witness through lower admission, Store recovery, and internal acknowledgement; it returns its receipt/applied outcome only through `take_terminal` after terminal Store ownership ([DB artifact](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:555)).

I found no current source path by which a raw `DurableOwnedGroupJournalRecordV1` plus caller-built receipt constructs that DB terminal/witness or a Hub `applied` acknowledgement. No source outside the Store/DB implementation currently consumes either recovery terminal type. Thus the lower Store route is not, by itself, a proven authoritative-ack forgery.

The DB recovery wrapper has since replaced that ordinary `Result` with a retained `Fault` outcome. The remaining concrete retained-owner P0 before any production mount is the **public lower Store host**: its `advance` and `capture_snapshot` still return ordinary `Result` while nonterminal Stores are held in `ManuallyDrop` and `Drop` only asserts. A direct caller using `?` on a recoverable error drops the host and leaks exact Store owners if the assertion is caught. This must not be left as a convention for a future per-document actor.

## What is now correctly witness-bound

`committed_durable_group_decision_from_transaction`:

1. takes the committed transaction by value;
2. drains and retires every record;
3. calls `finish()` before it admits the Event bytes;
4. accepts only one Event body, rejects a mixed/duplicate transaction and a foreign document; and
5. returns the opaque witness only after those checks ([DB artifact](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:601)).

An early decoder error drops `WalCommittedTransaction` with `finished == false`, which marks the retained cursor failed rather than producing a witness ([WAL](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2144)). That is fail-closed; the outer replay owner still has to execute its normal close path.

When lower admission rejects before any Store staging, the DB wrapper restores the same witness plus a reconstructed exact admission in `Admitting`, and `take_rejected_terminal` consumes the witness and returns only stores, error, document, and derived receipt ([DB artifact](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:482)). It cannot be retried after that terminal handoff.

When lower recovery reaches acknowledgement, the wrapper supplies its own internally derived receipt to the Store host and retains the host on every failure ([DB artifact](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:527)). No receipt argument or acknowledgement method is exported by the DB wrapper.

## Lower Store restoration is intentionally non-authoritative

The lower Store API remains public by design:

- callers can create `DurableOwnedMapRecoveryAdmissionV1` from three Stores and call `restore_untrusted_committed_record` with a canonical record/receipt ([Store](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:997));
- it can reconstruct/alter **caller-owned** stores and its public host can acknowledge the supplied lower receipt ([Store](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2099));
- it returns only `DurableOwnedMapRecoveryOwnersV1`, never an `ArtifactCommittedDurableGroupDecisionV1`, DB terminal, Hub approval receipt, or document-sync publication acknowledgement.

This is a valid kernel-local recovery capability; sealing it `pub(crate)` would be an invalid cross-crate fix because DB depends on Store. Its safety condition is **outlet typing**, not pretending raw data is unavailable: only the consuming DB wrapper may yield the authoritative recovery terminal, and future Hub/Map code must accept that terminal—not raw Store owners, record, receipt, or lower host—as evidence that a shared approval applied.

Required source/native acceptance when a Hub owner is added:

1. Drive a hand-built canonical record/receipt through the public lower route. It may restore the caller’s three stores, but cannot reach the Hub committer’s `applied`/sync/undo outlet because no DB terminal exists.
2. Drive the same decision through a physically replayed sole WAL Event. Only `committed_durable_group_decision_from_transaction` may produce the terminal accepted by the committer.
3. Assert a DB terminal is unavailable for command-only, aborted, duplicate/mixed Event, foreign document, and lower-only recovery. The already present physical negative law is a good start, but its API-shape check should become an actual consumer contract once the committer exists.

Do not turn the lower host’s `#[doc(hidden)] acknowledge_restoration` into a claimed durable approval acknowledgement. It is only the Store coordinator’s internal/lower completion step.

## P0: lower Store `Result` plus `ManuallyDrop` permits retained-owner loss through `?`

The following types deliberately use `ManuallyDrop<Option<_>>`:

- `DurableOwnedMapRecoveryAdmissionV1` ([Store](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:901));
- `DurableOwnedMapRecoveryHostV1` ([Store](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:962)); and
- `ArtifactCommittedDurableGroupRecoveryV1` ([DB artifact](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:424)).

Their `Drop` implementations assert that the state is empty but do not retire the manually held value ([Store host drop](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2128), [DB wrapper drop](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:587)). This avoids accidental rollback after a committed decision, but a caught panic leaves the `ManuallyDrop` payload un-dropped. The lower host exposes normal `Result`-returning `advance` and `capture_snapshot` methods ([Store snapshot](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2094), [advance](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2108)). This is an easy valid-looking outer call:

```rust
recovery.advance(grant)?; // a recoverable Store error
// scope unwinds: Drop asserts, then leaks the retained witness/host/Stores
```

The current physical negative law intentionally exercises this exact recoverable `InvalidFrontier` path, but then manually calls `take_rejected_terminal`; it does not prove that a normal outer error path cannot lose ownership ([DB law](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:6448)). It also closes returned Stores without capturing the entry frontiers, so it proves handback reachability but not byte-identical no-mutation of every member.

### Minimum coherent repair

The DB recovery API already uses a retained `Fault(DurableOwnedGroupDecisionError)` outcome: its wrapper stays in `Admitting` or `Recovering`, so callers cannot use `?` to discard the only owner. Apply the same shape to the public lower host, including snapshot failure, or bind it to an actor-private retained driver. Keep actual programmer-invariant failures fail-stop.

Then put the wrapper in the per-document actor’s retained slot before the first turn. The actor may only:

- retry/resume the same wrapper;
- take `take_rejected_terminal` before any mutation; or
- take the sole terminal handoff after Store completion.

There is deliberately no cancellation after a committed witness. If shutdown cannot finish it, the actor must retain/park the exact wrapper for its existing maintenance/close mechanism; it cannot `Drop` it. A `Drop` assertion is an invariant backstop, not the liveness protocol.

The Store host can remain internal to that wrapper. Its own result-returning `advance` is not safe for a general caller until it receives the same retained-fault or actor-slot discipline.

## Exact laws still missing

1. **`committed_recovery_fault_is_a_retained_outcome_not_a_drop_path`** — inject the current pre-visibility `InvalidFrontier`; assert the wrapper reports a retained fault, remains driveable, and an actor-slot-style holder can retry or take rejected owners without panic/leak.
2. **`committed_recovery_rejected_terminal_returns_byte_identical_three_store_frontiers`** — capture all three base snapshots/generation/revision before lower admission, induce rejection, take the rejected terminal, and compare all fields before closing each owner.
3. **`lower_recovery_cannot_enter_authoritative_committer_outlet`** — use a hand-built lower record/receipt to reach a lower terminal, then require the committer/sync receipt constructor to reject it at its typed boundary. The companion sole-WAL witness case alone may enter that outlet.
4. **`committed_recovery_all_post_is_authoritative_but_nonreappending`** — physical sole-event replay, all three post stores, wrapper returns `already_applied`, no Store publication/journal append, and exactly one DB terminal handoff.
5. **`committed_recovery_post_visibility_fault_retains_same_witness_and_host`** — after shared visibility but before all adoption, inject the existing drawing fault. Require an all-new-or-rejected capture, same wrapper identity, no second Event, and terminal completion only after the retained retry.

## Current nonclaim

There is no per-document GIS actor, no Hub approval committer, and no Hub consumer of the DB terminal today. The source boundary is consequently a sound direction for authoritative recovery, but it is neither live Map recovery nor an approval acceptance claim until the retained actor drives these outcomes and only it translates the DB terminal into document sync/history acknowledgement.

## 2026-09-06 current delta: DB fault ownership repaired; lower host and physical identity proof remain

The DB wrapper has now made the critical direction change: `ArtifactCommittedDurableGroupRecoveryV1::advance` returns `ArtifactCommittedDurableGroupRecoveryAdvanceV1`, including `Fault(error)`, rather than `Result` ([DB artifact](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:437)). Every recoverable path restores the same `Admitting` or `Recovering` state before it yields that fault. A caller can no longer write `recovery.advance(grant)?`; the wrapper’s normal failure result retains its exact witness/admission/host. This resolves the wrapper-specific `?` escape above.

The updated physical rejection law also now proves the DB-derived transaction, anchor digest, decision digest, typed failure, all three generations, and all three materialised `latest_hash` values ([DB law](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:6448)). It still does **not** prove exact no-mutation/identity handback:

- capture each Store’s `snapshot_pack().pack` and `.spr` before it enters the admission, then compare both bytes after `take_rejected_terminal` ([Store snapshot API](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16059));
- capture and compare `SpaceMember::artifact_ref()` and `owner_ref()` for the parent, drawing, and value ([Store member API](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17566)); and
- retain the present generation/hash checks as complementary cursor-state proof.

The lower public `DurableOwnedMapRecoveryHostV1` still has fallible `advance` and `capture_snapshot` methods while retaining stores in `ManuallyDrop` ([Store host](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2081)). A direct Store caller can still use `host.advance(grant)?` or `host.capture_snapshot()?`, then invoke its assert-only `Drop` with retained stores. Either give the lower host the same retained `Fault` outcome family (including a snapshot-fault outcome) or make it strictly actor-private with a retained driver. The DB wrapper is protected because it catches the lower `Result` and restores `host` before returning its own `Fault`; the public lower route is not.

The admission itself remains an explicit linear owner: the DB wrapper correctly consumes it immediately and, on rejection, reconstructs it from all three rejected owners before returning `Fault`. A standalone caller that creates it and abandons it still trips its `ManuallyDrop` assertion; retain its `into_owners` handback in every pre-admission close branch. This is an owner-liveness contract, distinct from authoritative DB acknowledgement.
