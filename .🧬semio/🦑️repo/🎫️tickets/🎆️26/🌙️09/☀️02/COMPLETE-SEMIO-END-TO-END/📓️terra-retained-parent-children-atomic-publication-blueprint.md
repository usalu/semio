# Retained Parent-and-Children Atomic Publication Blueprint

Status: **RED by current-source inspection.** This is a read-only blueprint; no build or native gate was run.

## Decisive boundary

The existing one-item APIs are a useful retained *preparation* boundary, not a group commit. `SpaceMember::prepare_one_item_publication` drives a candidate to `Publishing`, reserves a normal edit-history slot and a displaced-owner slot, then intentionally makes `advance_one_item_publication` reject it as requiring an atomic visibility authority. It neither stages a post snapshot nor uses `ArtifactGroupVisibilityOwner`. [`store/🦀️.rs:17717`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17717) [..:17783](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17783)

`CompositionCoordinator::dispatch_relation_group` remains a separate legacy route: it dispatches each child, stamps its tail after dispatch, dispatches the parent, and compensates with sequential undo on error. The comment accurately describes compensation rather than a visibility transaction. [`store/🦀️.rs:19441`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19441) [..:19566](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19566)

The minimal honest packet is a private `VcsArtifactApp` transaction for **one optional parent one-item edit plus one existing child one-item edit**. It must stage every post-root, cursor, edit history, and child-content root behind one `ArtifactGroupVisibilityOwner`, then flip one bit. It excludes `ChildGenesis`, multi-child batches, and undo/redo initially. That is enough to make a Flow child-only `addWidget` a real consumer later; it is not enough to claim all `ChildEmit` groups atomic.

## Reusable current authorities

| Existing authority | Current proof | Required use in the packet |
| --- | --- | --- |
| `ArtifactGroupVisibilityOwner` | Its only transitions are pending `0` to committed `1` or aborted `2`; `Drop` aborts a pending owner. [`vcs/🦀️.rs:206`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:206) | One owner per parent/child operation; never one per participant. The retained transaction, not an event handler, owns it. |
| Group history suffix | `reserve_group_one`/`stage_group_reserved` retain a suffix that reads old until the shared decision commits; `adopt_group` and `abort_group_one` return exact owners. [`vcs/🦀️.rs:452`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:452) | Replace the current normal `reserve_edit_history_slot()` branch in group preparation. Stage the prepared `Edit` before the decision, with `group_id` already sealed in its `MutationMeta`. |
| Cursor overlay | `ArtifactCursor::stage_group_owned`, `adopt_group_owned`, and `abort_group_owned` already bind an exact visibility Arc and do not expose mutable access while staged. [`store/🦀️.rs:2164`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2164) | Build each next cursor during staging, including applied/redo/checkpoint state. Do not push directly to the live cursor in a participant loop. |
| Coherent single-envelope read | `ArtifactEnvelopeOwners::capture_read` captures one group decision and refuses different history/cursor views. [`store/🦀️.rs:2401`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2401) | Extend the same decision to the Store current root, generation, and content revision; cursor/history alone cannot make current snapshots atomic. |
| Sealed one-item candidate | `ArtifactStoreOneItemPrepared` has private edit/post-root/seal fields, and the Store validates exact authority before moving them. [`store/🦀️.rs:13190`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13190) | Move, never clone, it from the preparation into a private staged participant. This is the correct typed parent/child mutation boundary. |
| App actor and child root | `VcsArtifactApp` has exclusive `&mut self`, a child registry, `child_content_root`, and `child_content_generation`. [`plugin/🦀️.rs:18920`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:18920) | It is the sole coordinator and should own the group operation. No mutex, global queue, or generic cross-app coordinator is needed. |

The visibility primitives are currently exercised only by low-level unit tests, not by a production group operation. [`vcs/🦀️.rs:270`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:270) [`store/🦀️.rs:23339`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:23339)

## Exact missing Store seam

The current `Publishing` turn directly replaces `current`, cursor vectors, history, generation, and revision. [`store/🦀️.rs:15431`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15431) It cannot be used after another participant has been prepared.

Add a **private typed staged root** beside `ArtifactStoreOneItemPublication`, conceptually:

```text
ArtifactStoreOneItemGroupStage<P, Mutation>
  visibility: Arc<ArtifactGroupVisibility>
  prepared: ArtifactStoreOneItemPrepared<P, Mutation>
  next_cursor: ArtifactCursorOwners
  next_generation / next_revision / next_clock / next edit sequence
  pre-reserved history rejection and displaced-owner capacity
  exact participant/store authority
```

It is created only after the existing preparation reaches `Publishing`, with all validation, vector capacity, retirement reservation, and candidate sealing complete. `stage_one_item_group` then:

1. verifies the immutable live authority, generation, revision, member lease identity, and cancellation fence;
2. takes the sealed candidate exactly once;
3. stages the edit through `reserve_group_one`/`stage_group_reserved` and the cursor through `stage_group_owned` with the same visibility;
4. stages the post snapshot plus scalar Store state in a new private Store root overlay using that same visibility; and
5. leaves the old live root untouched and returns only a private stage token to the retained operation.

`MemberStoreOneItemPublication.group_history`/`group_displaced` are insufficient: they currently hold a normal reservation and no candidate/root visibility state. Replace them with the group-stage token rather than adding a second reservation side channel. [`store/🦀️.rs:13287`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13287)

The new Store read root must cover **current snapshot, generation, content revision, applied/redo state, and cursor/history**. It must either take an explicit shared `ArtifactGroupReadDecision` or capture its exact visibility once, as `capture_read` does. A history/cursor-only overlay would allow a new edit to be visible with the old `current` snapshot.

After the one decision flips, participant adoption must be non-fallible. Existing `adopt_group`/`adopt_group_owned` return `Result`; calling a fallible method after visibility is committed would make an error visible. Make a private `adopt_committed_exact(stage-token)` operation that has no allocation, I/O, await, or recoverable branch because the token has already proved matching visibility and reserved capacities. An invariant failure here is internal corruption, not an application error. The old root/old cursor/history owners go only to slots reserved before staging.

The object-safe `SpaceMember` surface needs parallel private methods for stage, commit-adopt, and abort-close of an erased one-item group participant. The macro at [`store/🦀️.rs:18264`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18264) can delegate them per closed arm; do not expose a raw candidate or raw visibility handle to plugins.

## VcsArtifactApp transaction and ownership transitions

Add private `RetainedParentChildOneItemGroup<A, M>` owned by `PendingChildGroupPublication`, replacing `Ready → Dispatching` only for the new one-parent/one-child shape. It owns:

- `ArtifactGroupVisibilityOwner` and immutable group id;
- optional typed parent one-item publication and exactly one erased child one-item publication;
- exact parent `ArtifactRef`, child `(slot, id, dialect, OwnerRef)`, expected parent/child generation and revision, and captured child-content generation;
- pre-admitted `ChildContentView` post-root overlay and the exact old-root retirement reservation; and
- the eventual immutable group receipt / durable route, not naked mutation bytes.

### Progress state machine

1. **Admission.** Validate parent and child coordinates against the captured `ChildContentView` and registry, factory support, every one-item wire size/schema, and one optional parent plus exactly one child operation. `ChildGenesis`, an empty child op, duplicate child, multiple children, and any `AmendLast` route fail closed. Mint the group id before calling either factory so `ArtifactStoreOneItemLiveAuthority::validate_semantic_edit` verifies it in the candidate rather than relying on legacy `stamp_tail_group_id` after visibility.
2. **Prepare.** Drive the existing parent and child factories one bounded grant at a time. At every yield the retained group owns all inputs and both partial cursors. A cancellation, timeout, stale expected generation/revision, factory rejection, or capacity failure calls group abort; it never invokes `dispatch_group` or ordinary `apply_one`.
3. **Stage.** Stage parent then child through the new Store group method. Build the next `ChildContentView` from the child *staged post-root*, not `ChildContentView::with_member`: that helper awaits `snapshot_read_erased` and therefore cannot run after the first participant is observable. [`plugin/🦀️.rs:8505`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8505) Reserve the old child-content retirement before this phase.
4. **Final fence.** Recheck cancellation plus exact parent/child generations, revisions, registry leases, child root identity/generation, and visibility pending state. All checks occur before a root is moved. This is defense in depth under `&mut self`; it also protects a participant invalidated by a retained owner before its final turn.
5. **Linearize.** Install the staged app child-content overlay and all Store overlays, then call `visibility.commit()` exactly once. At that point every capture using the shared read decision sees all pre-state or all post-state. Immediately run the private infallible adoption moves: promote staged histories/cursors/current roots, queue old roots in already-reserved retirements, advance generations, and mark group receipt committed. No allocation, await, future poll, or fallible call is permitted between the first root move and receipt commit.
6. **Delivery / close.** The typed `Child` result page carries only an immutable receipt token and can retry without rerunning the group. ACK retires the retained operation. Cancellation after step 5 only stops delivery; it may not undo the committed group.

`PendingChildGroupPublication` is the appropriate per-operation owner today. [`plugin/🦀️.rs:16079`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16079) Its current `publish_mounted_typed_child_operation_unit` still calls legacy `dispatch_emit_group` after merely checking freshness. [`plugin/🦀️.rs:22478`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22478) Route only the bounded new shape to this transaction; leave other groups on the legacy route marked non-atomic until batch work exists.

## Abort, failure, and retirement order

Before commit, the transaction owns every staged candidate. On any failure:

1. call `visibility.abort()` once (or let the unique owner abort on terminal close);
2. use each participant’s exact `abort_group` method to retrieve staged post-root/edit/cursor owners and release group reservations;
3. move candidate snapshots/mutations into the already-reserved Store and child-content retirement owners;
4. incrementally close parent/child preparation cursors and the transaction under caller grants; and
5. verify terminal emptiness before the group owner or pending publisher may drop.

An abort must preserve every pre-group root/history/cursor/generation and must not use `undo_group` as cleanup. `undo_group` is intentionally best-effort and may skip a foreign tail; it is inappropriate before a group commits. [`store/🦀️.rs:19599`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19599)

After commit, no abort is legal. If event/result delivery fails, retain a committed receipt for retry; do not create a compensating mutation in the publication owner.

## Durable history / redo reconciliation

The P0 receipt must contain a private `CompositionGroupRouteV1`: group id, parent full ref, exact child coordinate and owner, both member edit ids, pre/post generation/revision fingerprints, and route schema/version. It must become durable with the same group visibility decision or parent composition event; a `ChildPublicationResultV1` containing only id/count/generation cannot recover or authorize redo. [`plugin/🦀️.rs:16061`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16061)

P0 records the route but does **not** expose group undo/redo. Current `undo_group`/`redo_group` independently attempt each tail and return skipped members, so they preserve legacy/foreign-tail behavior but cannot undo a route atomically. [`store/🦀️.rs:19599`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19599) [..:19636](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19636)

P1 adds a retained group-history transition: preflight every recorded exact tail (undo) or redo tail (redo), stage every next root/history/cursor behind one visibility owner, and reject the whole route if one member is foreign/stale. Legacy best-effort actions remain a distinct route and must not be relabelled atomic. A loaded-parent fallback must resolve the durable `CompositionGroupRouteV1`, not infer children from the current map.

## Native and neutral acceptance packet

Add a schema-first `retained-parent-child-one-item-group-v1` fixture plus independent Bun/AJV model. It describes states and ownership outcomes, never Rust internal layouts.

Required neutral vectors:

- parent+child and child-only one-item success; before the decision all four visible planes (parent Store, child Store, child view, durable route) are old, and after it all are new;
- cancellation, deadline, zero grant, stale parent, stale child, changed child root, wrong slot/id/dialect/owner, duplicate target, group-id mismatch, factory rejection, and history/cursor/root-retirement capacity failure; all keep old state and drain exact owners;
- stage one participant then fail the second; no partial append/root/route and no direct strict-owner drop;
- ACK retry and cancellation after commit: one group id, one pair of edit ids, no second commit;
- read-decision interleaving: captured pre decision returns every pre root; captured post decision returns every post root, never a mixed parent/child pair;
- P1-only foreign-tail/redo cases: reject all routed transition participants, while a legacy best-effort route remains visibly classified separately.

Real focused native laws, to be placed beside current Store group-preparation tests ([`store/🦀️.rs:22672`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:22672)) and mounted typed child publication tests, should be:

1. `retained_parent_child_one_item_group_stages_without_visible_member_state`;
2. `retained_parent_child_one_item_group_flips_one_visibility_decision_for_roots_history_and_cursor`;
3. `retained_parent_child_one_item_group_aborts_second_participant_failure_with_terminal_owners`;
4. `mounted_typed_child_publication_routes_one_item_group_and_replays_one_ack_receipt`; and
5. later, `retained_composition_group_route_rejects_foreign_tail_without_partial_undo`.

Each native law must use real `ArtifactStore`/`VcsArtifactApp` retained factories and grants `0`, `1`, and `4096`; it must drive the existing incremental retirement paths to terminal-empty. A mock `dispatch_group`, direct cursor edits, or a raw drop cannot qualify.

## Explicit nonclaims

This blueprint does not make `ChildGenesis`, N-child groups, current `dispatch_emit_group`, or existing group undo/redo atomic. It does not claim a database transaction, multi-user authorization, socket broadcast, or restart recovery until the durable route and event projection are implemented and executed.
