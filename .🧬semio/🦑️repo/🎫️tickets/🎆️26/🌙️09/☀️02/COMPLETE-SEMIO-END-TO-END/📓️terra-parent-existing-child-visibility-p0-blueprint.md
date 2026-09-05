# Parent + Existing-Child Visibility P0

Status: **RED by current-source inspection, 2026-09-05.** This is a read-only implementation blueprint. No build, native test, or runtime process was run.

## Decision

The smallest honest transaction is one optional parent one-item mutation plus **one already registered child** one-item mutation, both staged behind a single `ArtifactGroupVisibilityOwner`. It is not the current `dispatch_group` route, not child genesis, not a multi-child batch, and not atomic undo/redo.

The P0 must make one captured read see either every old plane or every new plane:

| Plane | Required participant |
| --- | --- |
| Parent | post snapshot, generation/revision, VCS edit suffix, cursor |
| Existing child | same four Store planes |
| App | `ChildContentView` and `child_content_generation` |
| Result | one committed receipt only after the shared decision |

The current visibility primitive can hide a VCS suffix and `ArtifactCursor`, but no current Store-root or app-root overlay exists. That is the decisive missing seam.

## Current evidence

- [`ArtifactGroupVisibilityOwner`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:206) supplies exactly one pending `0` to committed `1` or aborted `2` CAS. Its `Drop` merely marks a still-pending decision aborted; it does **not** retire staged payloads.
- [`reserve_group_one` / `stage_group_reserved` / `adopt_group` / `abort_group_one`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:452) stage one invisible history suffix under that shared Arc. `ArtifactCursor` has matching [`stage_group_owned` / adopt / abort](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2164) mechanics.
- [`ArtifactEnvelopeOwners::capture_read`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2399) captures one history/cursor decision, but does not cover `ArtifactStore.current`, scalar generation/content revision, the child registry, or `ChildContentView`. It cannot by itself provide cross-document snapshot atomicity.
- [`ArtifactStoreOneItemPrepared`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13190) is the correct sealed typed candidate: a private edit, post root, exact authority and address/digest seal. Its preparation factory already offers bounded progress/cancel/close at [`ArtifactStoreOneItemPreparation`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13228).
- [`SpaceMember::prepare_one_item_publication`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17728) reaches `Publishing`, reserves 12 displaced-owner slots and an ordinary history slot, then [`advance_one_item_publication`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17717) correctly refuses it: “requires an atomic group visibility authority.” P0 must replace those two ad-hoc reservation fields with one stage token; it must not create a second reservation side channel.
- The direct [`ArtifactStore::advance_apply_one`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15444) path immediately replaces the current root, history, cursor and scalar state. It is unsuitable once another participant is prepared.
- [`PendingChildGroupPublication`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16084) owns only `ChildEmit`s, parent mutations, receipt/fault and bounded local close. It owns no Store candidate, root overlay, visibility owner or reservation.
- [`publish_mounted_typed_child_operation_unit`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22536) freshness-checks then awaits `dispatch_emit_group`; it commits its local receipt only afterward. [`dispatch_emit_group`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21110) passes raw child pointers across that await, uses `OpBinary::encode_op(...).unwrap_or_default()` at line 21138, and only rebuilds `ChildContentView` after the Store group returns at line 21179.
- The legacy [`CompositionCoordinator::dispatch_relation_group`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19441) sequentially dispatches/stamps child edits, then parent edit, and compensates with ordinary undo on error. It is observable and can encounter foreign tails; it is not a visibility transaction.

The existing native [`retained_member_group_preparation_reserves_real_history_without_partial_visibility_and_aborts_stale_owners`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:22672) proves retained preparation/reservation and abort only. The plugin law [`retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34223) proves the legacy success/ACK/undo route, not all-or-nothing visibility.

## Exact P0 source packet

### 1. Store-owned participant stage

Add private `ArtifactStoreOneItemGroupStage<P, Mutation>` next to [`ArtifactStoreOneItemPublication`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13392), plus erased `SpaceMember` delegates and macro forwarding beside [`space_members!`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18267). It is constructible only by the exact Store that drove the candidate to `Publishing`.

It owns:

- the common `Arc<ArtifactGroupVisibility>`;
- the exact Store lease/one-item authority, expected generation and revision;
- moved `ArtifactStoreOneItemPrepared<P, Mutation>`;
- the staged edit/history reservation using `reserve_group_one` and `stage_group_reserved`, with the shared group id sealed in `MutationMeta` before staging;
- staged `ArtifactCursorOwners` via `stage_group_owned`;
- a new private Store live-root overlay for post snapshot, generation, revision, clock/tail and applied/redo state under the *same* Arc; and
- all displaced-root and metadata retirement capacity reserved before the decision.

The new overlay requires a private explicit shared-read API, conceptually `capture_group_read(&ArtifactGroupReadDecision)`, that selects the current-root/scalar plane as well as history/cursor. A read of parent and child must retain one common decision before obtaining either value. Re-capturing decisions separately permits parent-before/child-after observations around the CAS.

`stage_one_item_group` performs all identity, seal, slot-capacity and stale checks while pending. `adopt_committed_exact` is private and infallible by construction: no allocation, await, I/O, or recoverable branch after the CAS. `abort_group_step` returns staged post roots/cursors/edits to existing bounded Store retirement, drains the history suffix with `abort_group_one`, and returns reservations. Neither method exposes raw roots or the visibility handle to an app.

### 2. One-parent/one-existing-child retained coordinator

Replace the `Ready → Dispatching → dispatch_emit_group` leg of [`PendingChildGroupPublication`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16084) only for this closed shape with private `RetainedExistingChildVisibilityGroup<A, M>`:

- `ArtifactGroupVisibilityOwner` and its immutable group id;
- zero-or-one typed parent candidate and exactly one erased child candidate;
- full parent `ArtifactRef`; child `ArtifactRef`; exact `OwnerRef(parent, slot, child_id)`; selected factory/declaration schema; and captured parent/child generation/revision;
- captured `ChildContentView` plus its generation/digest; a staged next child view built from the child **post-root**; and an already admitted retirement slot for the old app view;
- cancellation/deadline/operation authority and only an immutable receipt after commit.

Admission is closed: one `ChildEmit` with nonempty op bytes, at most one parent mutation, no `ChildGenesis`, no N-child batch, no config/draft/effect/event lane, no `AmendLast`, no duplicate target. The selected `MemberFactory` must prove the child `op_schema` maps to its exact one-item preparation schema and decoder; `ChildEmit`'s semantic schema string alone is not a wire-decoder authority. Parent encoding is a fallible admission step; remove the current `unwrap_or_default()` behavior from this route so an encoding error cannot become an empty command.

### 3. Ordered retained transition

1. **Admit.** Validate full refs, owner, graph ownership, selected schema, support/footprint, byte caps, registry/store lease identities and the captured root. Mint the group id before either factory seals its edit.
2. **Prepare.** Advance parent and child one bounded grant per runtime turn through current one-item factories. Retain every partially prepared owner in the coordinator.
3. **Stage.** Create one Store group-stage per participant. Build a new child-content root from the staged child post snapshot, not from live `with_member`; reserve its previous-root retirement now.
4. **Final fence.** Before moving a root, recheck cancellation/deadline, parent/child generation/revision, child root digest/generation, exact registry entry/owner/slot/dialect, and `visibility.pending()`.
5. **Linearize.** With no await/allocation/fallible action remaining, install all private Store/app overlays, call `visibility.commit()` once, then execute infallible adoption of every Store history/cursor/root and the app child-root. Advance both Store generations and app child generation. Only then create the receipt and Child page.
6. **Deliver/close.** ACK only acknowledges page delivery. Post-commit cancellation may suppress or retry delivery; it never reruns, undoes, or compensates the committed transaction.

Before the CAS, a cancellation, deadline, second-stage failure, stale participant, capacity fault or page failure must call `visibility.abort()` and run the bounded stage abort/retirement sequence. After the CAS there is no abort. Do not use `CompositionCoordinator::undo_group` before or after the decision as transaction cleanup: it is a legacy foreign-tail-sensitive compensation route.

## Bounded test packet

### Neutral corpus

Add `retained-parent-existing-child-visibility-v1` schema/fixture and an independent Bun/AJV transition oracle beside the existing Store neutral script. Model visible planes and owner outcomes, not Rust private fields.

Required rows:

1. parent+child success and child-only success: all five planes old while pending, all new after one decision;
2. parent staged then child stage/child-view/history/cursor/retirement failure: no visible write, no `undo_group`, reservations restored;
3. zero grant, cancellation/deadline at each precommit state, stale parent, stale child, changed app child root, wrong slot/id/dialect/owner/schema, duplicate target, unsupported factory and capacity exhaustion;
4. captured pre- and post-decision reads never mix parent/child planes;
5. cancellation after commit and delayed ACK: exactly one group id/receipt/edit pair, no second publish; and
6. parent encoding failure: no empty wire substitution and no participant admission.

### Native focused laws

Place Store laws adjacent to line 22672 and the end-to-end app law adjacent to line 34223:

1. `retained_parent_existing_child_stage_keeps_every_store_plane_old_until_shared_commit`;
2. `retained_parent_existing_child_second_stage_failure_aborts_exact_owners_without_compensation`;
3. `retained_parent_existing_child_shared_read_decision_never_mixes_parent_and_child`;
4. `retained_parent_existing_child_post_commit_cancel_retries_one_receipt_only`; and
5. `mounted_typed_child_publication_routes_one_existing_child_visibility_group_then_delayed_acks_and_terminal_close`.

The last law must use actual `VcsArtifactApp`, actual `ArtifactStore` retained factories and the public driver (`plugin_step_live_cleanup`, `plugin_continue_typed_operations`, `plugin_acknowledge_typed_operation_result`). It must drive `Worker → Publishing → Child ACK → Terminal ACK → Retiring`, use grants `0`, `1`, and `4096`, prove terminal emptiness, and avoid direct cursor/root/map mutation or a mocked dispatcher.

## Explicit nonclaims

P0 does **not** make current `CompositionCoordinator::dispatch_group` atomic; does not cover child genesis, child registration/map/graph changes, peer links, N-child batches, config/draft/effects, global composition history/undo/redo, durable CQRS/socket broadcast/restart recovery, or database atomicity. Those require separately staged all-participant graph/history/event protocols.
