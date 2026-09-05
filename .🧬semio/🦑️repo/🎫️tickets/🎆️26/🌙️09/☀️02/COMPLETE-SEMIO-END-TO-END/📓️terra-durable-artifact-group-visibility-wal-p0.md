# P0: Durable Parent–Child Artifact-Group Visibility and WAL

## Decision

**RED — no current primitive durably and atomically publishes one parent plus one child.**

There are three valuable but non-substitutable green pieces:

| Existing piece | Proven scope | Why it does not close this P0 |
| --- | --- | --- |
| `ArtifactGroupVisibilityOwner` | One-process atomic read selection for staged VCS history and `ArtifactCursor` roots | It is an `Arc` plus `AtomicU8`, has no stable identity, serialization, journal, recovery, or fanout. |
| `prepare_one_item_publication` | Reserves one member's history/retirement capacity while leaving every member externally unchanged | It deliberately rejects ordinary publication once reserved and exposes no group commit/adopt operation. |
| `db_wal::ArtifactWal` | Crash-recoverable, hash-chained transaction frames for **one** `ArtifactId` | Its storage and `ArtifactEngine` are per document; no participant-set record, cross-document recovery, or group frontier exists. |

Do not connect GIS typed group work to `TransactionCoordinator` as a durability shortcut. Its phase 2 dispatches children sequentially and then the parent, and relies on best-effort reverse-order `Undo` compensation on a late error. That is useful interaction compensation, not durable atomic visibility.

This audit inspected framework-only sources. It does not modify or require changes to the hub event page, React `ShellHost`, GIS group-work, or Stdio taxonomy.

## Validated source map

### 1. Present in-memory history/cursor barrier — green, but narrow

- `ArtifactGroupVisibilityOwner` owns only `Arc<ArtifactGroupVisibility>`; `commit` and `abort` are one-way `0 → 1` / `0 → 2` CAS transitions and `Drop` aborts the still-pending owner. There is no group id or durable field: [🌿️vcs/🦀️.rs:188](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:188), [🌿️vcs/🦀️.rs:206](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:206), [🌿️vcs/🦀️.rs:228](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:228).
- `ArtifactHistoryLedger` can reserve/stage a suffix under that exact pointer, then either expose it after `commit`, adopt it into the base ledger, or return each staged owner after `abort`: [🌿️vcs/🦀️.rs:452](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:452), [🌿️vcs/🦀️.rs:478](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:478), [🌿️vcs/🦀️.rs:489](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:489).
- `ArtifactCursor` has the analogous staged root, exact-authority adoption, aborted-owner return, and immutable read selection: [🏪️store/🦀️.rs:2144](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2144), [🏪️store/🦀️.rs:2164](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2164), [🏪️store/🦀️.rs:2172](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2172), [🏪️store/🦀️.rs:2180](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2180).
- `ArtifactEnvelopeOwners::capture_read` requires history and cursor to name the same pointer, captures exactly once, and reads both roots through that capture. The injected-serializer test proves that a captured envelope cannot tear across a concurrent decision: [🏪️store/🦀️.rs:2399](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2399), [🏪️store/🦀️.rs:23335](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:23335).
- The VCS test is an honest in-memory law: two ledgers expose the suffix at one decision and reject foreign decisions. It is not a restart/recovery test: [🌿️vcs/🦀️.rs:255](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:255).

### 2. The current root barrier is incomplete for typed projection visibility — red

`ArtifactStore::snapshot`, `snapshot_ref`, `snapshot_read`, and `snapshot_root` always read `self.current`; none captures `ArtifactGroupVisibility`: [🏪️store/🦀️.rs:14312](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14312), [🏪️store/🦀️.rs:14324](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14324), [🏪️store/🦀️.rs:14341](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14341).

Consequently, the existing pointer gate protects only VCS/cursor serialization. It cannot make a member's materialized snapshot, generation, content revision, applied/redo identities, clock, tail cache, and history appear as one group-owned root. A parent/child projection reader can therefore not use it as a full typed-state atomicity primitive.

The symbol census is also decisive: all `ArtifactGroupVisibility*`, `stage_group_owned`, `reserve_group_one`, `stage_group_reserved`, `adopt_group*`, and `abort_group*` occurrences are the two defining modules and their unit tests; there is no production parent/child coordinator that owns this visibility authority.

### 3. Existing retained member preparation — green reservation seam, not commit

`SpaceMember` provides begin/advance/prepare/abort for an erased one-item publication: [🏪️store/🦀️.rs:17510](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17510). `prepare_one_item_publication` revalidates generation/revision, reserves displaced-owner slots and an edit-history slot, then refuses normal `advance_one_item_publication` until an atomic authority exists: [🏪️store/🦀️.rs:17717](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17717), [🏪️store/🦀️.rs:17728](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17728), [🏪️store/🦀️.rs:17762](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17762).

Its two-member law proves real reservation, stale-owner refusal, exact-member abort, and bounded close while no snapshot/history becomes visible. It stops before a group commit: [🏪️store/🦀️.rs:22671](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:22671).

The single-member publisher is not reusable as the group commit. Its `Publishing` phase moves the snapshot, cursor, history, generation, revision and lease authority for one store, then awaits an ACK: [🏪️store/🦀️.rs:15358](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15358), [🏪️store/🦀️.rs:15444](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15444). Cancellation is effective only before that local publication; ACK/close then retain and dispose exact owners: [🏪️store/🦀️.rs:13426](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13426), [🏪️store/🦀️.rs:13443](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13443), [🏪️store/🦀️.rs:15493](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15493).

### 4. Existing composition is compensating, not atomic or durable — red

`TransactionCoordinator::dispatch_relation_group` performs child creation, child `dispatch_wire_with_policy` calls in input order, then parent dispatch. A late failure invokes `compensate`, which calls `Undo` on the parent then already-applied children in reverse order: [🏪️store/🦀️.rs:19332](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19332), [🏪️store/🦀️.rs:19509](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19509).

Its own commentary explicitly permits `CompensationFailed` when rollback cannot fully complete: [🏪️store/🦀️.rs:19431](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19431). This must remain separate from the new primitive; no re-labelling or extension of compensation yields a crash-safe all-or-nothing commit.

### 5. Existing durable authorities and their hard boundary — green per document, red group

- `WalStorage` is explicitly a sequence of segments for one `document`; every API takes one `&ArtifactId`: [🗄️storage/🦀️.rs:4563](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4563).
- `ArtifactWal` writes `TxBegin … TxCommit` through one active per-document segment and forces `commit()+sync` for `Fsync`: [📝️wal/🦀️.rs:1622](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1622), [📝️wal/🦀️.rs:1732](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1732).
- It recovers a torn active tail, verifies the chain, rebuilds the active suffix, and surfaces `WalRecoveryReport`; torn sealed history is a hard error: [📝️wal/🦀️.rs:1216](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1216), [📝️wal/🦀️.rs:1654](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1654), [📝️wal/🦀️.rs:2072](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2072).
- `ArtifactEngine` owns one `document`, one `ArtifactWal`, one `Frontier`, one outbox and one in-memory `commit_log`: [🗿️artifact/🦀️.rs:1153](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1153). Its submit path WAL-commits, then updates a single frontier and only then appends the local notification/outbox: [🗿️artifact/🦀️.rs:1468](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1468), [🗿️artifact/🦀️.rs:1511](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1511).
- `WalRecord::Event` exists but is opaque, and the current engine replay only applies `Command` and `Frontier`; it has no parent/child group-frame decoder: [📝️wal/🦀️.rs:423](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:423), [🗿️artifact/🦀️.rs:1251](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1251).

There is intentionally no kernel → db dependency: the kernel manifest has no `db` dependency, while the db package depends on the kernel. The executor must therefore consume a kernel-owned journal port; `os_store` must not import db implementation types: [💻️os/Cargo.toml:33](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:33), [🛢️db/Cargo.toml:25](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml:25).

### 6. Recovery, cancellation, and fault seams worth reusing

- `WalCursorControl` checks cancellation, deadline, and finite fuel at every grant: [📝️wal/🦀️.rs:173](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:173).
- The retained WAL replay test proves cancellation, resume, bounded close, and terminal emptiness: [📝️wal/🦀️.rs:2237](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2237).
- `db_testkit::FaultStorage` provides fail-nth append, torn-write, fsync-lie, and catalog-CAS injection; `CrashHarness` restarts after every write boundary: [🧪️testkit/🦀️.rs:254](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️.rs:254), [🧪️testkit/🦀️.rs:489](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️.rs:489).
- Existing cursor retirement already demonstrates the required bounded owner discipline: return the displaced old root only after adoption and drain it under an item/byte grant: [🏪️store/🦀️.rs:526](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:526), [🏪️store/🦀️.rs:23409](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:23409).

## Smallest executable framework packet

### Tight scope

Implement **one existing parent + one existing child**, each receiving exactly one typed one-item publication. Exclude child genesis, peer groups, N-member groups, UI plumbing, websocket fanout, and ordinary `TransactionCoordinator` dispatch. They are separate packets after this primitive proves recovery.

The public boundary must comprise only:

```rust
ArtifactGroupCommitRequestV1
ArtifactGroupCommitReceiptV1
ArtifactGroupRecoveryWitnessV1
trait ArtifactGroupJournalPort
```

All owner, staged-root, journal-frame, state-machine, and retirement types stay private to framework crates. In particular, neither a caller nor GIS receives `ArtifactGroupVisibility`, its `Arc`, an `ArtifactHistoryReservation`, or an internal journal handle.

### Exact private types and file ownership

| File | Add or change | Private types / responsibility |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs` | Preserve `ArtifactGroupVisibilityOwner` as the in-process decision, but add no persistence here. | `ArtifactGroupReadDecision` remains the sole same-process captured-decision witness. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | Add the kernel-side port/request/receipt plus the executor entrypoint; forward it through `SpaceMember` and `space_members!`. | `ArtifactStoreGroupCandidate<P, Mutation>`, `ArtifactStoreGroupRoot<P, Mutation>`, `ArtifactGroupCommitOwner<P, Mutation>`, `ArtifactGroupCommitPhase`, `ArtifactGroupRetirement`. A candidate owns every post-state root, not only VCS/cursor. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧩️group/🦀️.rs` (new) | Implement `ArtifactGroupJournalPort` against a canonical parent-anchor group journal. Encode/decode the V1 frame and replay it. | `ArtifactGroupWalFrameV1`, `ArtifactGroupWalMemberV1`, `ArtifactGroupJournal`, `ArtifactGroupJournalRecovery`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/🦀️.rs` | Mount the new `db_group` module only. | No new facade-wide group API. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️.rs` | Extend the existing fault/crash harness for a two-member group-journal workload. | Test-only `GroupCrashScenario` and assertion helpers. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧩️group/🧪️group-commit-v1.json` (new) | Language-agnostic canonical request/frame/recovery/failure oracle. Test it with the first-party decoder and test-only `serde_json` oracle. | Fixture only; no runtime dependency. |

`ArtifactStoreGroupRoot` must retain and route reads through one captured decision for: current snapshot, snapshot lease generation/revision, applied/redo IDs, cursor, all four VCS ledgers, local actor, clock, tail undo cache, revision accumulator, and projection cause. Merely reusing `ArtifactHistoryGroupSuffix` + `ArtifactCursorGroupRoot` is invalid because the live snapshot APIs above bypass them.

`ArtifactGroupWalMemberV1` must contain the canonical parent/child artifact references, expected pre-generation/revision, resulting generation/revision, deterministic group sequence, and a **canonical recovery payload for the already-prepared typed post-state**. It may not record only `MemberStoreOneItemWire`: the existing preparation factory is arbitrary domain code, so recomputing it after a crash is not a valid deterministic-recovery assumption. Add a private prepared-candidate recovery encoder/decoder seam to the one-item factory path and prove round-trip equality before the journal accepts a frame.

Use the existing `WalRecord::Event` carrier only behind the new typed `ArtifactGroupWalFrameV1` codec. Do not pretend its current opaque bytes are a typed group authority. Submit the one group frame to a **parent-anchor** `ArtifactWal` with `DurabilityClass::Fsync`; that one durable record is the commit point. A separate parent WAL plus child WAL transaction cannot provide the requested atomicity through today's `WalStorage` API.

### State machine and ownership order

```text
New
  → Preparing(parent, child; cancellable, bounded)
  → Prepared(all roots + exact recovery frames retained)
  → JournalCommitting(Fsync; cancellation becomes “resolve journal”)
  → DurableCommitted(group frame is the authority)
  → Visible(one in-memory owner.commit; non-cancellable)
  → Adopted(parent then child; old roots moved to reserved retirement)
  → FanoutQueued(parent then child, same group sequence/frontier)
  → Retiring(bounded close steps)
  → Complete

New|Preparing|Prepared --cancel/fault before journal commit--> Aborting
Aborting --owner.abort, reverse child→parent return, bounded close--> Complete
JournalCommitting --I/O uncertainty--> RecoverJournal
RecoverJournal --commit frame found--> DurableCommitted
RecoverJournal --no trusted commit frame--> Aborting
```

The required transfer order is:

1. Validate both exact member authorities; reserve every history, snapshot, fanout and retirement slot before moving a candidate.
2. Build/stage both complete candidate roots while the shared visibility is pending. A pre-commit reader sees both old roots.
3. Encode, hash, and validate both recovery payloads; append the single parent-anchor group frame with `Fsync`.
4. If the write result is uncertain, recover the anchor before choosing abort versus commit. Never abort blindly after attempting the durable boundary.
5. Once the commit frame is trusted, call `ArtifactGroupVisibilityOwner::commit()` exactly once. Cancellation is no longer an abort authority.
6. Adopt parent then child staged roots into their ordinary roots, placing every displaced owner into pre-reserved bounded retirement. No ordinary writer can interleave while either candidate is staged.
7. Enqueue non-fallible group notifications in canonical order: parent first, child second. Persist/derive their shared `group_sequence` from the journal, so a crash before delivery replays rather than drops or duplicates them.
8. Retire old roots under caller grants after adoption. A held old snapshot remains valid until that bounded retirement consumes it.

The recovery witness must report at least `{ group_id, anchor_frontier, parent_frontier, child_frontier, member_count: 2, recovered: bool }`. Opening the parent or either child through this group path must reject a frame whose two expected bases do not match, and must either apply both exact payloads idempotently or expose neither. A standalone child path is not part of this P0; it must fail closed until it resolves the parent anchor rather than silently showing a partial child projection.

### Native acceptance laws

1. **No pre-commit visibility.** At every preparation yield, parent and child `snapshot`, `snapshot_read`, `snapshot_root`, VCS serialization, cursor serialization, document pack, and group fanout remain the old pair.
2. **One captured root decision.** A read captured before visibility remains entirely old even if the group commits while serializing; a fresh read is entirely new for current snapshot **and** VCS/cursor. This extends the existing envelope-only law.
3. **Atomic durable prefix.** Crash/torn/fail at every journal append and sync boundary: recovery has either neither member's new group frame or both under one group id; never one parent/child update.
4. **Idempotent recovery.** Reopen/replay twice or replay the same committed group frame twice: both member frontiers and history counts are unchanged after the first application; the receipt/group sequence is identical.
5. **Stale all-or-nothing.** Change either member generation/revision after its preparation but before journal commit. No group frame is durable, the untouched member is still old, and all reserved owners reach terminal emptiness.
6. **Cancellation boundary.** Before durable commit, cancellation returns both staged candidates/slots in reverse ownership order and leaves no frame. After a trusted commit (including an uncertain write resolved by recovery), cancellation reports the committed receipt and completes adoption; it cannot hide a durable group.
7. **Fanout order and replay.** No notification precedes the durable frame; emitted/replayed notifications are exactly `parent(group_sequence)`, then `child(group_sequence)`, with no duplicate after crash between adoption and delivery.
8. **Retirement.** Adopted old parent and child roots retire only through bounded item/byte grants; every returned owner reaches `terminal_is_empty`, and no `Drop` assertion fires.
9. **Malformed frame.** Bad version, duplicated participant, wrong anchor, mismatched expected base, oversized field, hash failure, torn active tail, and torn sealed segment all fail closed; no member becomes visible from an untrusted frame.

Run the language-agnostic JSON fixture through both the first-party V1 codec and test-only `serde_json` oracle. Drive the same two-member workload through `db_testkit::FaultStorage` (`fail_nth_write`, torn write, fsync lie) and `CrashHarness` rather than inventing a second failure injector.

## Explicit nonclaims

- Current `ArtifactGroupVisibilityOwner` is not a durable group transaction, even though its in-memory reader law is correct.
- `ArtifactWal` group-commit batching is fsync batching, not a cross-artifact transaction.
- `ArtifactEngine::commit_log` and `drain_outbox` are per-document/in-memory delivery mechanisms; they do not supply restart-safe group fanout.
- `TransactionCoordinator`'s preflight and reverse `Undo` compensation do not survive process death and must not be used as the group WAL.
- This packet intentionally does not solve group child genesis, N-way/peer transactions, cross-process UI delivery, or reconnect ACK semantics.

## Handoff

Implement the five-file framework packet above, starting with the V1 fixture and the crash laws. Keep the GIS caller unchanged until the parent-anchor journal, complete staged root, recovery witness, and all nine laws are green.
