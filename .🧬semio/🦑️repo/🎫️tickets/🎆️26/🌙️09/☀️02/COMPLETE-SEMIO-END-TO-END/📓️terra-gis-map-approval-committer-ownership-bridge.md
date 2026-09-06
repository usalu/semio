# GIS Map approval: concrete owned-committer bridge

## Decision

There is no production per-document three-member Map actor to wire into `GisMapApprovalCommitterV1`.  The one existing document serializer in `HubInferenceRuntimeV1` is only a `DocumentScope -> tokio::Mutex<()>` gate; the Hub has no `GisMapStore`, drawing store, or value store (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs:161-170`).  `MapHost` is a pure visual projection, not an owner of persistence.  The generic DB `ArtifactHandle` is also expressly forbidden by the committer contract and does not own the three typed Stores.

The smallest honest bridge is therefore a new **typed, per-`DocumentScope` actor** which owns all three Stores and the fixed Store host for the duration of a commit.  It cannot be replaced by a direct implementation that calls the generic socket/`ArtifactHandle::submit` path.

## Concrete input and Store types

The existing typed unit of work is already exactly the input needed by that actor:

| Role | Concrete source type | Work field | Fixed member identity |
| --- | --- | --- | --- |
| parent | `GisMapStore = ArtifactStore<GisMapSnapshot, GisMapMutation>` | `GisMapCreateRegionGroupWorkV1.parent` / `parent_inverse` | selected map document |
| drawing | `ArtifactStore<SemioDrawingSnapshot, SemioDrawingMutation>` | `drawing` / `drawing_inverse` | `gismap-drawing` |
| value | `ArtifactStore<SemioValueSnapshot, SemioValueMutation>` | `value` / `value_inverse` | `gismap-value` |

Evidence:

- `GisMapCreateRegionGroupWorkV1` carries the complete parent/drawing/value forward and inverse mutations at [`inferences/🦀️.rs:47`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:47), and `create_region_group_work` rejects a supplied image and non-exact child handles before returning it at [line 93](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:93).
- `GisMapStore` is the parent alias at [`mutations/🦀️.rs:42`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:42); child aliases and fixed handles are at [`gismap/🦀️.rs:78`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:78) and [line 83](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:83).
- The parent has a retained production initializer, `gis_map_document_store_initialization_job`, at [`binary/🦀️.rs:1223`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1223); its candidate uses `from_initialized_runtime_with_owners` at [line 1127](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1127).

There is no corresponding current Hub source reference to any of these Store types.  Parent bootstrap alone is insufficient: the persisted parent snapshot carries handles, not owned drawing/value Stores.  A real opener must construct/open all three from the verified parent pair plus retained group decisions, then keep them together.  The existing internal recovery does exactly the required all-pre/post check (`recover_store_owned`) at [`durable-group/🦀️.rs:1735`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1735), but it is `pub(super)`, so it is not currently callable by a Hub opener.

## Exact ownership bridge

Add one Store-owned, generic *assembly* surface; do not expose the coordinator, seals, or prepared fields.  Its role is just to bridge three member preparations into the already-tested fixed host:

```rust
// Store crate, durable-group module: public opaque construction surface.
pub struct DurableOwnedThreeStoreMapAssemblyV1<P, PM, D, DM, V, VM> { /* private */ }

impl<P, PM, D, DM, V, VM> DurableOwnedThreeStoreMapAssemblyV1<P, PM, D, DM, V, VM> {
    pub fn begin(
        parent: ArtifactStore<P, PM>,
        drawing: ArtifactStore<D, DM>,
        value: ArtifactStore<V, VM>,
        // three already-admitted, retained Store preparation owners
        parent_preparation: ArtifactStoreOneItemPublication<P, PM>,
        drawing_preparation: ArtifactStoreOneItemPublication<D, DM>,
        value_preparation: ArtifactStoreOneItemPublication<V, VM>,
        sink: Box<dyn DurableOwnedGroupJournalSinkV1>,
    ) -> Self;

    pub async fn advance(&mut self, grant: ArtifactStoreOneItemGrant)
        -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError>;
    pub fn cancel(&mut self) -> bool;
    pub async fn close_step(&mut self, grant: ArtifactStoreOneItemGrant)
        -> Result<SnapshotRetirementStep, DurableOwnedGroupDecisionError>;
    pub fn take_terminal_owners(&mut self)
        -> Option<DurableOwnedMapCommitOwnersV1<P, PM, D, DM, V, VM>>;
}
```

`begin` must take the Stores by value and retain all three preparation owners.  It advances all preparations to `Prepared`, takes their exact sealed candidates, performs the existing private `from_store_prepared -> bind_store_owned -> begin_retained_commit -> mount_map` sequence, and thereafter delegates to `DurableOwnedMapCommitHostV1`.  This is deliberately an assembly state machine rather than a new public constructor for `DurableOwnedThreeStorePreparedV1`:

- the present private sequence is at [`durable-group/🦀️.rs:743`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:743), [line 852](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:852), and [line 1836](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1836);
- the public fixed-slot host already supplies `capture_snapshot`, `cancel`, `acknowledge`, `advance`, and one-time terminal owner handoff at [lines 1163-1185](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1163);
- individual Store preparation is admitted through `begin_member_apply_one` at [`store/🦀️.rs:15285`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15285), which requires the installed domain factory and checks exact current generation/revision before it lends the snapshot.

The typed Hub actor owns the assembly, never an exposed host operation:

```text
Closed
  -> Opening { verified pair + three typed Store initializers/recovery }
  -> Ready { parent, drawing, value }
  -> Preparing { assembly }          // no visibility change
  -> JournalUncertain { assembly }   // retry same owner or cancel then await Absent
  -> CommittedUnpublishedPair { assembly }
  -> PublishedAwaitingAck { assembly }
  -> Ready { owners returned exactly once }
  -> Closing
```

Store it as `HashMap<document_key, Arc<tokio::sync::Mutex<TypedMapActor>>>`, bounded exactly like the existing runtime gates.  Do **not** hold that mutex across the journal I/O future; take the actor's exclusive turn/state, await, then re-enter only if its actor epoch and `InferenceDocumentFenceV1` generation still match.  The existing `HubInferenceRuntimeV1::document_gate` may serialize admission, but it does not retain the state above and is not a substitute for it.

## Current preparation admission is a blocking P0

The `begin_member_apply_one` route named above is **not currently admitted by the GIS Map production owner catalog**.  `MemberStoreOwners::new` initializes `one_item_preparation` to `None` ([`store/🦀️.rs:2028-2045`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2028)); `begin_member_apply_one` forwards that optional installed factory to `begin_apply_one`, which explicitly rejects its absence ([`store/🦀️.rs:15264-15298`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15264)).  The actual GIS initializer calls only `MemberStoreOwners::new` at [`binary/🦀️.rs:709`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:709) and then transfers that catalog into the Store at [line 1127](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1127).  It never calls `with_one_item_preparation`.

`GisMapEditor` can construct a parent `Gis2dOneItemPreparationFactory` ([`editor/🦀️.rs:601-603`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:601)), but that capability is not installed by the binary Store initializer.  It also does not supply the independently typed drawing/value factories.  Consequently, a new committer must not pretend `begin_member_apply_one` works today.

For the smallest compileable bridge, put three concrete, domain-owned factory values inside the typed `TypedMapActor` constructor and call the existing `begin_apply_one(..., Some(&factory))` path for each member.  The actor must accept only `GisMapCreateRegionGroupWorkV1`, choose the three factories itself, and keep the factory references private; it must not turn this into a generic Hub mutation API.  A later cleanup can instead install the same concrete factories in all three `MemberStoreOwners` catalogs via `with_one_item_preparation`, after those child catalogs exist.  Either form still needs a real drawing and value factory, so this is a prerequisite for the first commit law rather than an incidental wiring detail.

## Journal and witness are the real blocking seam

The current journal trait is synchronous:

```rust
fn advance(&mut self, grant: ArtifactStoreOneItemGrant)
  -> Result<DurableOwnedGroupJournalAdvanceV1, String>;
```

at [`durable-group/🦀️.rs:253`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:253).  The only existing implementation is the in-module `FakeJournalSink` at [line 2479](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2479).  By contrast, the actual exclusive durable primitive is asynchronous `ArtifactWal::submit` at [`db/wal/🦀️.rs:2479`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2479), and `ArtifactWal` retains its non-cloneable `WalWriterPermit` at [line 2300](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2300).

The earlier conclusion that a synchronous `advance` necessarily forces blocking was too strong.  It may safely be a **retained, nonblocking poll boundary**, but only if the actual I/O is owned by a pre-existing document WAL actor.  `DbIoTaskOperation` already illustrates this shape: it retains a fixed task handle, registers a waker on `poll`, and hands back an exact result lease only after terminal ownership ([`storage/🦀️.rs:4154-4158`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4154), [lines 4334-4403](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4334)).  In contrast, `WorkerPool` jobs are `FnOnce` closures, so using one to call `block_on(ArtifactWal::submit)` is not an equivalent retained async owner and is unsafe.

The minimal no-blocking design therefore keeps the existing Store trait but gives the Hub actor a private, fixed per-document WAL mailbox.  `HubGisMapJournalCommitV1` has `NotSubmitted | Awaiting(ReplyReceiver<...>) | Committed | Failed` state.  Its first `advance(grant)` transfers only the already-bounded, request-bound envelope bytes to that mailbox and returns `Pending`; later turns inspect the retained receiver and return `Committed` only with a forced-Fsync `WalAppendReceipt`.  The mailbox actor—not the Store journal object—owns `ArtifactWal`, the backend facet, construction/retirement of `WalRecordBatch`, and the `await` of `ArtifactWal::submit`.  It returns the `ArtifactWal` only through its own terminal/close state.  This avoids a self-referential future over both `&mut ArtifactWal` and `WalRef<'_>` (the latter is explicitly borrowed from `DbBackend::wal` at [`storage/🦀️.rs:4853-4857`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4853)).

That mailbox/actor does not exist yet.  Changing the Store trait to `async fn` is also valid, but is a wider generic change and still must solve the same owned-`ArtifactWal`/borrowed-facet issue.  Do not use a fire-and-forget Hub task that borrows either owner.  The commit object retains:

```text
ArtifactWal + WalRecordBatch + validated decision pack + request-bound canonical command
  + scope + fence generation + cancellation/control + pending receipt/error state
```

It submits exactly one forced-`Fsync` transaction through its actor-owned `ArtifactWal`.  The batch must include a canonical approval command that the witness verifier can see.  Today `InferenceWalVerifierV1` only recognises a matching `WalRecord::Command` in a committed transaction ([`inference/wal/🦀️.rs:289`](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🧾️wal/🦀️.rs:289)); it rejects a fabricated witness by replaying the physical WAL.  A decision-only `Event` cannot reconcile the Hub ledger.

Use a request-bound `HubGisMapApprovalJournalEnvelopeV1` inside one `WalRecord::Event` **or** put the canonical command and decision in the same batch, but do not write them in different transactions.  The former is preferable because it avoids duplicate authority bytes; it requires the verifier to decode and validate that tagged envelope rather than looking only for `Command`.

The envelope must carry: `scope`, active authorization/fence generation, `job_id`, `proposal_hash`, `mutation_id`, `command_hash`, exact canonical command bytes, `decision_sha256`, and exact canonical decision bytes.  It must independently check both hashes before append.  Replay accepts it only if scope/generation/fence, command identity, and decision all match; its receipt uses the WAL `tx_id` and segment.  This is the only route by which `GisMapApprovalReceiptV1.witness` should be created.

There is a hard size constraint: Store allows a decision event up to `491_520` bytes (`DURABLE_OWNED_GROUP_EVENT_MAX_BYTES` at [`durable-group/🦀️.rs:13`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:13)), while a physical WAL segment includes framing and has a 496-KiB read cap.  A decision at that maximum cannot be batched with a command.  The sink/envelope must preflight the **complete physical transaction** against the WAL reservation before Store visibility changes and reject over-capacity input.  Do not rely on the Store decision bound alone.

## Approval method in concrete order

1. The Hub route has already authenticated and selected its scope.  The committer obtains the typed actor for the exact `DocumentScope` and re-reads the verified active pair (`VerifiedRebootstrapSource::active_pair` at [`lag-rebootstrap/🦀️.rs:219`](/Users/ueli/Documents/semio/🌎️hub/🛰️lag-rebootstrap/🦀️.rs:219)).  Reject if descriptor, canonical pack digest, or frontier differs from `request.base_frontier`.
2. Open/recover the three Stores together.  The parent uses the existing GIS initializer; the opener also has to materialize the two derived members and replay admitted group decisions before it lends a `Ready` actor.  The current Store recovery primitive is private, so include a typed Store-owned `open_or_recover_fixed_map_three` facade with the assembly change above.  There is no valid shortcut that starts a new drawing/value Store after a persisted parent has prior group decisions.
3. Decode the request's canonical server command in the Hub inference module, recompute `GisMapInference::infer(parent_snapshot).create_region_group_work(parent_snapshot, job_id)`, and require exact equality of the work's parent forward/inverse to the decoded command.  Do not accept the request command as the child mutation source.
4. After the preparation-factory P0 above is resolved, obtain the three private `begin_apply_one(..., Some(&factory))` preparation owners against the actor's live Store generations/revisions; create the opaque Store assembly with a fresh request-bound journal sink.  A stale preparation returns `Conflict` before journal begin.
5. Drive the retained journal/host until `Committed(receipt)`, then capture the triple snapshot and run an explicit verified checkpoint-pair publication turn before acknowledgement.  The current Hub field is named `_artifact_publication` and is only constructed at startup; it is not a Map actor/pair transition.  Do not imply that a Store-memory flip makes `active_pair` reopenable.  If journal I/O is uncertain, retain the same assembly; cancellation means `cancel` then wait only for `Absent` before restoring pre-state.  If the journal is committed but checkpoint-pair publication is uncertain, the actor enters `CommittedUnpublishedPair`, does **not** cancel or reuse the host, and retries/reconciles that exact pair publication before acknowledgement; a completed journal cannot be rolled back by Store cancellation.
6. Create `InferenceWalTargetV1` from request values and verify the emitted transaction using `InferenceWalVerifierV1`.  Its `CommittedInferenceWalWitnessV1` is private and matches scope, generation, job, proposal, mutation, command hash, and transaction id ([`inference/wal/🦀️.rs:65`](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🧾️wal/🦀️.rs:65)).  Only then return `GisMapApprovalReceiptV1`; `commit_prepared_approval` already reconciles the ledger only through that receipt at [`runtime/🦀️.rs:350`](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:350).

## First executable laws

1. **Approved exact group:** materialize a verified Map pair, derive group work, approve, and assert a single committed journal envelope, the parent/drawing/value post snapshots, one common decision id, and a verifier-minted witness.
2. **Hostile parent-only command:** retain valid parent command bytes but alter the inferred child outcome or group decision; exact parent/inverse comparison or decision validation rejects it before journal begin and all three Store frontiers remain unchanged.
3. **Journal uncertainty:** inject append/sync failure after journal begin.  Dropping the request leaves the actor in `JournalUncertain`; retry uses the same decision/command and neither exposes a partial member nor creates a second transaction.
4. **Restart:** close actor after commit, reopen from the active parent pair plus journal record, apply Store-owned three-member recovery, then assert all three snapshots/frontiers match pre-close.  A missing/foreign drawing or value recovery pack must fail the whole opener.
5. **Over-capacity envelope:** decision at max plus command/framing over physical transaction capacity is rejected before journal begin/visibility.  This specifically prevents the current 491,520-byte Store event bound from becoming an invalid WAL claim.

## Current integration points

- Replace the startup injection of `UnavailableGisMapApprovalCommitterV1` at [`hub bin:6588`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6588) only after the typed actor, retained journal mailbox, opener, and verifier envelope law exist.
- Keep `HubInferenceRuntimeV1` as the route/job authority; put the actual implementation behind its existing `Arc<dyn GisMapApprovalCommitterV1>` seam ([`runtime/🦀️.rs:124`](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:124)).
- Do not attach it to the generic WebSocket submit at [`hub bin:3229`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3229).  That route is a single generic engine command and cannot own the typed triple or Store seals.

No build or source edit was performed for this audit.
