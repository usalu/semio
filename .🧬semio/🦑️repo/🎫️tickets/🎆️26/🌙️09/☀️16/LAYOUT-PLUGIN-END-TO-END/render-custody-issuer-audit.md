# Selected Checkpoint Render Read Issuer Audit

2026-10-28. Read-only support coordinated with mutations owner. Inspected actual checkpoint/media preparation, Store derived preview/read APIs and snapshot registry implementation. No implementation, tests or builds.

## Exact Root and Issuer

NativeCheckpointExportSource in plugin/instance/docs/staging/checkpoint owns the actual ArtifactDerivedHistoryPreview until replay finishes. The preview captures the Store snapshot_read_leases handle, generation_now and content_revision_now in Store engine/lifted/store/checkpoint-read begin_derived_checkpoint_preview. Its state is the selected checkpoint Arc, not the live Store head. ArtifactDerivedHistoryPreview::take_finished_snapshot_owner (Store/read/derived-snapshots... lines141-143) only takes that Arc and preserves the planner/input residues; it does not issue read custody.

The correct read must therefore be issued from the finished preview state and its captured registry before preview closure and Arc transfer. Calling live snapshot_read_original would issue an authenticated read of the live head; it cannot authenticate a different selected-checkpoint Arc. Creating a new registry or constructing a synthetic derived wrapper would lose the actual captured issuer.

Existing genuine fixed-slot APIs:

- SnapshotReadRegistryHandle::try_issue_original in Store/read/registry lines20-25 accepts the exact Arc and returns the original lease; refusal returns both its allocation-free SnapshotReadLeaseRefusal and the unchanged Arc. It checks lock, vacant/resident slot and generation limits.
- SnapshotRead::new(owner,lease), exported by store::os_store::snapshot_read, constructs the typed captured read without heap allocation (value/snapshot/read lines533-534).
- SnapshotRead::<P>::capture_copy_byte_demand (same module531) quotes the typed read frame plus the extra Arc header.
- SnapshotRead::belongs_to, lease_identity and commit_authority_matches preserve issuer identity/frontier checks; its get reference must match the selected checkpoint snapshot pointer.

The proposed narrow checkpoint advance handoff into the existing preparation original_read slot can use these exact APIs and the actual preview fields. Keep the slot empty before issue; price both the read capture and any snapshot/bool/Option frame transfers within the same grant before side effects; account the receipt with cx.consume_retained/consume_fuel. On undergrant, do not issue or take state. On issuer denial, preserve the preview original state and reconcile the fixed alias/header work; do not replace the issuer or fall back to a live read.

## Authority Freshness

try_issue_original does not validate canonical generation/revision. Before issuance validate preview.registry.authority_matches(preview.generation,preview.revision). The initialized Store publishes initial authority in draft-config-store-and-checkpoint-identity line55; ordinary publication/commit paths publish updated authority. The finished replay step itself does not publish a new authority merely because it reaches preview.finished.

Media preparation/render line137 already compares the current Store generation/revision to its captured canonical key on every unpublished turn, including before checkpoint.advance at140. Preserve that guard. The captured preview authority check additionally fails closed if an already-finished preview is used after its event frontier changes. Do not republish a stale authority to make the check pass.

## Capacity and Closure

The actual registry is prefunded once through SnapshotReadRegistryBirth in Store/read/registry/birth. Its advance quotes/charges the Arc frame and paged slot/generation backing through the original native decoder ledger; it cannot issue while partial. Snapshot issue uses one of1024prefunded slots and requires no per-export new registry capacity. Selected checkpoint export still owes the existing bounded returned-read/registry-alias closure and preview/input/Arc retirement; transfer the typed read into the actual request slot rather than dropping it during preview cleanup.

Current checkpoint close_demands/close_step already retains and closes replay/input/decoder residues under the actual Store and original retirement control. Keep its snapshot alias until original.snapshot_source has retained the same selected Arc. Source retirement does not authorize substituting another root.

## Existing APIs That Are Different Doors

Store::retained_derived_read_demands/admit_retained_derived_read (engine/lifted/store lines145-160) produces an ErasedSnapshotRead from ArtifactDerivedSnapshot, using the same registry but checking only Store registry pointer in its demand. It is not the typed original read required by the new request field and should not be used to avoid the checkpoint original handoff.

StoreCursor::snapshot_read_derived (store-cursor-operations lines76-86) validates same Store/frontier, republishes commit authority and issues an erased read. It demonstrates which identities are required, but republishing/erasing is unnecessary for this original checkpoint seam. Existing live snapshot_read_original checks already-published authority and issues a typed read of snapshot_owner; its shape is useful precedent, while its live root is unsuitable for selected checkpoint export.

## Result

A genuine derived checkpoint typed issuer exists through the original preview registry and state. No new public arbitrary issuer, synthetic authority, snapshot clone, replacement context or live-head read is required. Concrete authority/funding recommendations were sent to mutations and root. This audit makes no claim that the pending implementation or native compiler has passed.

## AppOperationContext Read-only Capability Audit

Read-only follow-up requested by root; no code/tests/builds. Inspected declaration/view/operation-contexts, all matching SDK plugin construction sites, retained-command producer/read consumers, ArtifactView and child lookup, plus Layout consumers.

AppOperationContext is a public data struct carrying actual app/document/operation/generation/canonical revision/grant and authoring_seed. There is no constructor that rejects empty seed. Its sole seed-sensitive method entity_id explicitly returns the framework fault entity creation requires an admitted authoring seed when empty. The native declaration/view/tests/entity-id law constructs an empty-seed variant and asserts entity_id fails. Thus absence of entity-authoring capability is deliberate and tested; empty string is not a minted authoring seed.

This does not establish that every empty-seed metadata struct is an authenticated admitted command. Type documentation calls AppOperationContext durable authority captured at public command admission. The production retained-command producer builds it by moving the actual ArtifactOwnedToolJobRequest fields, including its real authoring_seed. In the bounded SDK construction scan no preexisting production empty-seed constructor or dedicated readonly AppOperationContext constructor was found. Test fixtures elsewhere predominantly supply explicit fixture seeds. The new media request declares an operation_context field, so populating it is a current integration responsibility rather than evidence that a historical constructor already exists.

Existing explicit readonly context support is represented by AppRenderOperationContext and ArtifactView.operation=None. ArtifactView.operation() fails if no public command operation is bound; operation_optional documents render/test views that do not enter command authority. retained_grant chooses actual command grant or render mounted maintenance grant. child_editor_read and OriginalChildMemberOperation::from_view distinguish authentic Command and Render variants and fence real parent identity/canonical revision. These paths do not invent a command admission to read a document.

Read-only consumers of AppOperationContext, including OriginalChildMemberLookup and ArtifactCommandSnapshotRead, use actual app/document/operation/generation/revision plus original parent/read/source binding. They do not demand a nonempty authoring_seed merely to read. ArtifactCommandSnapshotRead::borrow_original checks its SnapshotRead commit_authority_matches; take_current additionally validates OriginalToolSourceBinding against the actual StepContext and exact generation/revision. Therefore an empty-seed metadata value cannot independently bypass read custody or physical caller authority, and entity creation remains refused.

For the media seam, an empty seed can truthfully express no entity-authoring capability if all remaining metadata comes from the actual admitted media operation and the original snapshot/child/source binding is retained. It must not be described as a fabricated admitted command or used to manufacture an authoring seed, replacement StepContext, operation ID, or Store read. The safest interpretation of current evidence is no-authoring capability is supported; a broad claim of an established standalone readonly command-admission constructor is not supported. Exact evidence and this distinction were sent to mutations/root.
