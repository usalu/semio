# Retained GIS Map Assembly and Document WAL Bridge: Current Packet

## Decision

The next coherent change is Store-owned `DurableOwnedThreeStoreMapAssemblyV1`, instantiated only by a typed per-document GIS Map actor.  It must own the three concrete Stores, three exact GIS factories, and the journal sink from admission until it returns terminal owners.  It is not an `ArtifactHandle::submit` adapter and it must not block a worker while awaiting WAL I/O.

This is a source audit only.  No build was started.  The current database single-flight suite has a live capacity/shutdown leak investigation; it is not marked qualified here.

## Current retained document mount

`Database::mount_document` now publishes one `Opening` owner before it awaits, and joining callers receive a bounded waiter slot.  `DatabaseDocumentMountWait::drop` removes only its exact waiter, not the owner ([engine](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7475>)).  A failed engine open retains its original `ArtifactEngineOpenRejected`; `DatabaseDocumentMountOwner` repeatedly drives its same `retry_close` future and parks the owner if cleanup faults ([engine](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7489>), [engine](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7649>)).  `shutdown_step` continues an `Opening` owner instead of dropping it ([engine](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8242>)).

Therefore the Map actor must be constructed **only after** `ensure_document` / `document` returns a ready `ArtifactHandle`; it must not independently open a second WAL or retain an ad-hoc copy of the writer.  A rejected first open already has a retained-close protocol.  Test-only fixtures likewise consume `ArtifactEngineOpenRejected` through `retry_close`, rather than dropping it ([testkit](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️.rs:571>)).

The useful current database law selectors are:

- `database_concurrent_ensure_mounts_one_actor_and_one_writer`
- `database_published_opening_joins_without_actor_overwrite`
- `database_cancelled_ensure_waiter_does_not_cancel_mount_owner`
- `database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation`
- `database_mount_owner_emits_before_ready_and_survives_elected_waiter_cancellation`
- `database_mount_waiter_capacity_rejects_33_and_reuses_one_cancelled_slot`
- `database_shutdown_interrupt_retains_waiterless_opening_owner_until_ready`
- `database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume`

Use those unchanged as the database acceptance prerequisite.  A currently observed shutdown-capacity failure must be fixed in that owner/fanout slice, not worked around by making a second Map actor or releasing a live handle early.

## Exact Store assembly surface

Keep the Store module domain-neutral but make the call site exact:

```rust
type Parent = ArtifactStore<GisMapSnapshot, GisMapMutation>;
type Drawing = ArtifactStore<SemioDrawingSnapshot, SemioDrawingMutation>;
type Value = ArtifactStore<SemioValueSnapshot, SemioValueMutation>;
```

The input is one server-recomputed `GisMapCreateRegionGroupWorkV1`: its parent, drawing, and value forwards have fixed child identities and it rejects image participation ([GIS inference](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:45>)).  The Map actor owns the work only until its own validation/admission completes; it recomputes it from the currently verified parent snapshot and never accepts child mutations from an HTTP/browser caller.

`DurableOwnedThreeStoreMapAssemblyV1<P, PM, D, DM, V, VM>` should have private states:

```text
AdmittingParent → AdmittingDrawing → AdmittingValue
  → PreparingParent → PreparingDrawing → PreparingValue
  → Mounted(DurableOwnedMapCommitHostV1)
  → Closing | Terminal
```

Its constructor owns, by value:

1. all three Stores;
2. the three forward typed mutations and actor/revision/generation fence values;
3. three role-specific `Arc<dyn ArtifactStoreOneItemPreparationFactory<_, _>>`; and
4. `Box<dyn DurableOwnedGroupJournalSinkV1>`.

For each role it uses `ArtifactStore::begin_apply_one` with the explicit factory ([Store](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15264>)), then advances only until the individual publication has its prepared candidate.  It must **not** take the individual `Publishing` branch: the candidate is transferred internally to the current private `from_store_prepared → bind_store_owned → begin_retained_commit → mount_map` chain ([durable group](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:743>), [durable group](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1827>)).  Only the existing fixed host then exposes capture/cancel/advance/ack/one-time terminal-owner handoff ([durable group](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1159>)).

### Mandatory ownership rules

- Retain a factory `Arc` only until that role's `begin_apply_one` transfers the preparation owner.  Do not return a mutation or factory already accepted by the Store; close/retire its publication.  An immediately rejected later role has not transferred ownership and its work can be recomputed from the verified snapshot.
- Keep all three initial authority/edit `group_id` values `None`.  The Store-private binding operation is the sole `None → decision_sha256` transition; accepting a caller-supplied group id violates the current one-time seal check.
- On a late role rejection, cancellation before journal start, or preparation error, retain and close every accepted publication before terminally returning the three Stores and the sink.  Dropping a live publication or host is an assertion failure by design.
- After journal submission, retain the same mounted host through all uncertainty.  `cancel` only becomes a rollback after the exact sink proves `Absent`; a `Committed` decision must move forward to reconciliation/publication rather than recreate a new group.
- The assembly stays a single-owner actor turn.  It may return `Pending` but must not permit another request to borrow a Store during `Admitting*`, `Preparing*`, or a nonterminal mounted host.

`begin_member_apply_one` is unsuitable now.  It consults an installed owner-catalog factory ([Store](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15285>)), while the current GIS document owner catalog only installs retirement/disposal owners ([GIS binary](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:709>)).  The existing `Gis2dOneItemPreparationFactory` is private editor implementation ([GIS editor](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:377>)).  GIS must expose three narrow typed factory builders to its Map actor, not export the private generic factory and not add a generic Hub mutation API.

The existing fixed-host laws to reuse are:

- `durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once`
- `durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots`
- `durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty`
- `durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change`
- `durable_map_fixed_host_slot_retains_every_live_owner_across_request_error_until_terminal_handoff`
- `durable_map_fixed_host_slot_cancellation_after_uncertain_io_waits_for_trusted_absence`

Add four assembly laws around those, not a generic Store submission law:

1. `durable_map_three_store_assembly_uses_exact_gis_factories_and_binds_one_decision` — exact parent/drawing/value inputs reach the fixed host with one common decision identity and all three edits sealed only after bind.
2. `durable_map_three_store_assembly_late_member_rejection_closes_prior_publications_before_owner_handoff` — drawing/value rejection causes no root change and no nonterminal `Drop`.
3. `durable_map_three_store_assembly_cancellation_before_journal_restores_all_three_frontiers` — no journal begins and each Store returns exactly once.
4. `durable_map_three_store_assembly_uncertain_journal_retains_same_host_until_committed_or_proven_absent` — no re-begin or member visibility change while the journal answer is uncertain.

## Per-document journal / WAL addition

The eventual actor bridge is one **typed mailbox request**, not generic `ArtifactHandle::submit`:

```rust
ArtifactMessage::AppendDurableGroupDecision {
    record: DurableGroupJournalRecordV1,
    reply: ReplySender<Result<DurableGroupJournalReceiptV1, DbError>>,
}

ArtifactHandle::append_durable_group_decision_retained(record)
    -> AskFuture<ArtifactMessage, Result<DurableGroupJournalReceiptV1, DbError>>
```

`ArtifactAuthority` already serializes a retained `ArtifactEngine` around every message turn ([artifact actor](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3838>)) and thereby already owns the exclusive `ArtifactWal` writer.  Its currently public retained path is only generic `submit_retained` ([artifact actor](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4077>)); do not expose a raw batch/event append beside it.

The typed `record` must be accepted by a Store-owned admission facade, not by public decision fields: `DurableOwnedThreeMemberDecisionV1::decode_canonical_pack` is currently crate-private ([durable group](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2094>)).  That facade should consume bounded canonical bytes and yield only verified `{ decision_sha256, anchor_sha256, canonical_pack }` to DB.  DB validates the exact document binding plus pack/hash/anchor and preflights the entire one-transaction frame cost before it begins journal I/O.  Store's `491_520`-byte decision ceiling is not enough to admit an envelope plus framing under the WAL's 496-KiB readable-segment cap.

The journal sink transfers one request-bound, canonical envelope to `append_durable_group_decision_retained` on its first `advance`, returns `Pending`, and retains the same `AskFuture` on subsequent advances.  No `WorkerPool::block_on`, borrowed `ArtifactWal`, `ArtifactHandle::submit`, or second writer permit is valid.  Failure/receiver closure is uncertain and leaves the journal owner live; a cancellation request can only complete if the actor returns a durable `Absent` result.

The first record must use `WalRecord::Event` internally plus forced `Fsync`; its returned receipt includes decision/anchor hash, transaction id and segment.  This alone is **not recovery**: current `ArtifactEngine::open_retained` ignores `Event` during state replay.  Before claiming reopenable Map approval, add the same typed envelope decoder to the Map actor/recovery reader and make `InferenceWalVerifierV1` recognize the canonical typed envelope.  The current verifier accepts a matching committed command rather than an arbitrary event, so an Event-only journal is not yet a Hub approval witness.

## Hub transition after the bridge

`commit_prepared_approval` currently refuses every child-bearing Map before invoking a committer ([Hub runtime](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:393>)); it must remain fail-closed until this actor is live.  Once live, replace the empty-child condition with an exact ordered `{gismap-drawing, gismap-value}` check and pass only a server-recomputed `GisMapCreateRegionGroupWorkV1` to the per-document Map actor.  Do not simply delete the check.

The actor owns `Ready { three stores } | Busy { assembly } | Closing` under a scope/document/fence generation.  It releases no Store while busy, and it reacquires its map after a journal poll only if the exact actor generation and verified Hub fence still match.  A request cancellation removes only the caller's reply interest; it cannot abandon an accepted assembly or the document WAL owner.

## First complete acceptance chain

1. database single-flight opens one document actor/writer and reaches `Ready`;
2. typed GIS actor opens/reconstructs all three Stores, verifies the parent child handles, and recomputes region work;
3. assembly admits all three exact role factories, binds exactly one decision, and a sink receives one forced-Fsync decision transaction;
4. injected caller cancellation or post-submit I/O error retains the same assembly; a retry neither re-begins nor exposes a partial Store;
5. after durable commit, the typed recovery reader replays the same canonical envelope, rebuilds all three post-frontiers, and the Hub verifier mints the receipt only from that transaction.

Until steps 3–5 exist together, the current `UnavailableGisMapApprovalCommitterV1` behavior is correct and no Map approval/reopen claim is warranted.
