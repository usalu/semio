# Pending Child Group Publication Atomicity Audit

Status: **RED — source inspection only, 2026-09-04.** No build, native test, or runtime process was run for this audit.

## Decisive boundary

`PendingChildGroupPublication` is an exact retained *operation and result-page* owner. It is not a durable or visible parent-and-child transaction. Its `Committed` phase is set only **after** `VcsArtifactApp::dispatch_emit_group` returns; the latter calls the compensation-based legacy `CompositionCoordinator::dispatch_group`. Thus a `Child` result page/ACK can truthfully mean “the old sequential route returned,” but cannot mean one atomic parent-and-child publication occurred.

The smallest honest P0 is a private, retained **one parent + one existing child** group stage, not a rewrite of all `dispatch_group` users. It must prepare every Store participant and the app child-content root behind one `ArtifactGroupVisibilityOwner`, then perform one no-`await`, pre-reserved decision/adoption. It excludes genesis, N-child batches, and current group undo/redo until each has its own retained all-participant protocol.

## Current source route and exact gaps

| Boundary | Current behavior | Why it is not atomic |
| --- | --- | --- |
| Pending owner | [`PendingChildGroupPublication`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16084) owns parent mutations, `ChildEmit`s, description, receipt, bounded fault, and its local phase. `begin_dispatch` is only `Ready → Dispatching`; `commit` only stores a receipt. | It owns no cursor/history/root/graph candidate, no visibility token, and no pre-reserved app-root publication. Its phase cannot hide prior Store writes. |
| Publisher | [`publish_mounted_typed_child_operation_unit`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22504) takes the owner, checks captured parent/root freshness, calls `pending.begin_dispatch()`, awaits `dispatch_emit_group`, then calls `pending.commit`. | The cancellation lease is claimed before the await, but `dispatch_group` is not a retained/cancellation-aware operation. Cancellation after `begin_dispatch` is not a precommit abort point; the old route may mutate one or more members before the publisher next observes cancellation. |
| Wire ownership | The source-only law at [`plugin/🦀️.rs:18093`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:18093) says “never clone publishes,” but [`dispatch_emit_group`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21122) constructs `ChildDispatch { ops: child_emit.ops.clone(), … }`. | The Store receives copied wire vectors while the pending owner retains the originals. This is not a single retained operation handoff and makes a future group abort/return protocol ambiguous. The P0 stage must move exact wires into the retained group candidate, or retain an explicit reversible request owner before any preparation begins. |
| Parent encoding | [`plugin/🦀️.rs:21104`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21104) converts each parent mutation through `OpBinary::encode_op(op).unwrap_or_default()`. | An encoding failure is silently changed into an empty byte vector. Atomic preparation must reject the original parent mutation before allocating or preparing any participant; it must never turn an encode fault into a different command. |
| Legacy coordinator | [`CompositionCoordinator::dispatch_group`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19361) delegates to [`dispatch_relation_group`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19441). Phase 2 dispatches each child, stamps its tail, then dispatches/stamps the parent. | Each dispatch/stamp awaits and mutates a live Store before the next participant. `compensate` calls ordinary `undo` in reverse order ([`store/🦀️.rs:19338`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19338)); it is observable, append-only, and may skip a foreign tail. It is compensation, not all-or-nothing publication. |
| Immutable child view | On a successful legacy dispatch, [`dispatch_emit_group`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21146) asynchronously rebuilds the next `ChildContentView`. On a later capture failure it invokes `undo_group` ([`…:21159`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21159)). | Durable member edits are already live before the app’s immutable view changes. Failed capture is handled by best-effort compensation and can leave a mixed store/view state. |
| Genesis | [`dispatch_relation_group`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19499) creates a member, stamps its owner, and inserts its graph edge before later creation/dispatch work. [`absorb_created_children`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21046) later inserts map/root independently. | A late failure can leave a graph edge, an owner-stamped strict member, or a partial app map/root. Current child emit uses `Vec::new()` genesis ([`plugin/🦀️.rs:21143`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21143)); genesis must remain outside P0. |

The existing retained owner does have truthful close behavior: child emits, parent mutations, description, receipt, and fault are drained under caller grants ([`plugin/🦀️.rs:16144`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16144)). That proves bounded retirement of its local fields only; it does not retire Store-owned staged state because none exists today.

## Existing pieces to reuse, and the missing join

`ArtifactGroupVisibilityOwner` already provides exactly one pending → committed or aborted decision, with `Drop` aborting only a still-pending view ([`vcs/🦀️.rs:206`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:206)). The history ledger can reserve and stage an invisible suffix with that same view, then adopt only after commit or return exact entries after abort ([`vcs/🦀️.rs:452`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:452)). `ArtifactCursor` has parallel `stage_group_owned`/adopt/abort support ([`store/🦀️.rs:2164`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2164)). These primitives are currently covered by low-level unit tests around [`store/🦀️.rs:23345`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:23345), not by a real parent/child group.

The object-safe `SpaceMember` seam already provides retained one-item admission, incremental preparation, and bounded abort ([`store/🦀️.rs:17510`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17510)). `ArtifactStore` preparation verifies the exact generation/revision and reserves displaced-root plus history capacity ([`store/🦀️.rs:17728`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17728)). It deliberately rejects `advance_one_item_publication` once group reservations exist because no atomic group visibility authority is bound ([`store/🦀️.rs:17717`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17717)). This is the correct seam to extend; adding another independent publication queue is not.

What is missing is a private **participant stage** that owns the sealed one-item candidate, one common visibility view, staged history/cursor/current-root/generation/revision state, and all reserved displaced-owner slots. Neither `MemberStoreOneItemPublication.group_history/group_displaced` nor `PendingChildGroupPublication` presently contains those identities. In particular, stage/adopt must cover the Store’s current snapshot and scalar generation/revision as well as cursor/history; hiding only ledger suffixes would still expose a mixed snapshot/history read.

## Smallest implementation sequence

### P0 — existing parent plus one existing child only

1. Add a private `ArtifactStoreOneItemGroupStage<P, Mutation>` and erased `SpaceMember` delegates beside the existing one-item publication APIs. The stage accepts a common `ArtifactGroupVisibilityOwner::view()`, validates the exact participant/store authority and sealed candidate, and stages the post-root, cursor overlay, edit with its group id already sealed, revision/generation, and displaced-owner reservation. It must use `reserve_group_one`/`stage_group_reserved` and `stage_group_owned`, not the normal history reservation path.
2. Add a private retained `PreparedParentChildGroup<A, M>` owned by `PendingChildGroupPublication`. It contains exactly one optional parent candidate and one child candidate, parent/child full refs and owner coordinate, source/captured root generations, one visibility owner, a prebuilt next `ChildContentView`, and a pre-admitted previous-root retirement slot. It owns moved original operation bytes; no `ChildEmit.ops.clone()` publication.
3. Drive participant preparation one bounded grant per publisher turn. Before every next turn and again immediately before commit, check cancellation/deadline, member/store lease identities, parent and child generation/revision, child root digest/generation, registry entry, slot/id/dialect/owner, and visibility pending. Any wait or rejection keeps the complete retained group owner; cancellation/timeout calls one bounded participant abort path that returns staged entries/reservations and drains every owner to terminal.
4. After both Store candidates and the app child-root candidate are fully staged, perform one exclusive no-`await`, no-allocation, no-recoverable-error linearization: install the private app overlay, flip `visibility.commit()` once, then run private infallible adoption of every staged Store/app owner. A failed invariant here is internal corruption; all capacity/identity work belongs before the bit flip. Only then construct `ChildPublicationResultV1` and expose the Child result page.
5. ACK is delivery acknowledgment only. A cancellation before the bit flip aborts; a cancellation after it may suppress/retry presentation but can never compensate or rerun the committed group.

This packet leaves `CompositionCoordinator::dispatch_group` unchanged for legacy callers and routes only the exact P0 shape away from it. That is more honest than calling the current legacy method atomic. `ChildGenesis`, multiple children, config/draft/effects, peer links, and undo/redo need a later batch stage that includes graph/map/root candidates under the same decision.

### P1 — batch/genesis and grouped history

`ChildGenesis` requires a retained batch that holds every `Mc::create` owner and graph/map/root admission before any owner stamp or graph insert. Existing `GroupReceipt::created_children` is post-hoc naked member ownership and is unsuitable. The batch must validate `ChildSlotSpec.many`, stage graph changes and app registry capacity before the decision, and transfer candidates through existing bounded child-member retirement on abort. A viewer opens an existing relation only; it never creates durable genesis.

Group undo/redo is separate. Current `undo_group` accepts foreign tails by skipping them, which is correct for legacy best-effort history but cannot be advertised as atomic route recovery. A future grouped-history route must preflight every tail, stage every reverse/redo candidate under one new visibility decision, and reject all if one tail is foreign/stale.

## Failure and await boundaries

Before P0’s visibility decision, every fallible/awaiting action is permissible only while the retained group owns all candidates:

- parent encoding and wire/schema admission;
- preview/policy and full parent/child coordinate validation;
- one-item incremental preparation, capacity reservations, staged root capture, and graph checks;
- cancellation/deadline/freshness rechecks.

After the decision, no fallible call, allocation, `await`, raw map-pointer reconstruction, or `undo_group` may occur. Existing [`dispatch_emit_group`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21129) uses raw child pointers across the coordinator await; the P0 stage should instead retain exact child candidates and never hold/construct raw pointers through multi-step asynchronous work.

## Required non-vacuous acceptance matrix

Add a schema-first neutral corpus (for example `retained-parent-child-one-item-group-v1`) and an independent Bun/AJV transition model. It must model externally visible states and receipt/retirement outcomes, not Rust internals.

| Law | Required assertion |
| --- | --- |
| Success | Parent + child histories/cursors/current roots plus app `ChildContentView` remain all-old while pending; one shared decision yields all-new; exactly one group id and one Child page/ACK result. Include parent-only mutation bytes plus child bytes to prove the parent is not silently omitted. |
| Visibility interleaving | A reader that captures before decision sees only pre-state across all planes; one after sees only post-state. No parent-new/child-old or history-new/root-old observation is legal. |
| Late prepare failure | Inject child candidate/root-capture/history/cursor/retirement capacity failure after a prepared parent. Assert no visible edit/root/generation change, no `undo_group` call, original group owner drains under grants `0`, `1`, `4096`, and every reservation is returned. |
| Stale/cancel | Parent stale, child stale, changed child root, timeout/cancel before commit, and duplicate/wrong slot/id/dialect/owner all keep pre-state and retain/close exact owners. Cancellation after commit produces no second mutation and keeps one retryable receipt. |
| Encoding | An `OpBinary` failure rejects before any participant admission; it never dispatches an empty replacement byte vector. |
| Legacy contrast | A foreign-tail/undo case remains classified legacy best-effort, not a P0 atomic law. |
| Runtime driver | Drive actual Worker → Publishing → Child page → delayed ACK → Terminal page → Retiring using `plugin_step_live_cleanup`, `plugin_continue_typed_operations`, and `plugin_acknowledge_typed_operation_result`; assert actual callbacks ran, not merely scheduled. |

Place Store laws beside the existing visibility tests around [`store/🦀️.rs:23345`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:23345) and plugin lifecycle laws beside [`retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34176). The latter currently proves a successful legacy parent/child change, delayed Child ACK, undo/redo, and close; it does not inject a post-child/pre-parent or post-Store/pre-view failure, so it is not atomicity evidence.

## Acceptance and nonclaims

P0 is accepted only after the neutral model and real Store/plugin lifecycle laws execute with actual retained factories and terminal-empty close witnesses. Current source has low-level visibility unit coverage and a successful legacy child-publication runtime law, but no executed all-or-nothing parent-and-child transaction law.

This audit does not claim atomic `dispatch_group`, child genesis, N-child publication, grouped undo/redo, graph mutation, restart recovery, socket broadcast, or a database transaction.
