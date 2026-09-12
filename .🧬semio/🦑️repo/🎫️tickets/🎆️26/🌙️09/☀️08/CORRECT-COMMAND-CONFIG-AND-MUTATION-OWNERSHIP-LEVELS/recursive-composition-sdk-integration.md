# Recursive Composition SDK Integration

## Verified current boundaries

- Store now owns the recursive topology validator in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🦀️.rs`. `OwnedDocumentClosure` retains only fixed scalar cursor state, validates the source generation on every step, and borrows child projections from an `OwnedDocumentClosureSource`.
- A concrete `ArtifactStore<P, Mutation>` already exposes its immutable typed value through `snapshot_ref()`. That is the smallest read adapter required by the closure source: `ChildRestoreProjection` borrows only schema fields during a step, so an erased snapshot lease would add retirement work without strengthening authority.
- `SpaceMember` and `space_members!` in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` are the closed heterogeneous member boundary. They did not previously expose the typed snapshot's child projection.
- `MemberOpenRequest`, `MemberOpenOperation`, and the retained page input live in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs`. `MemberFactory::begin_open` is the bounded construction boundary. The convenience `M::open` path materializes the whole input and must not be used by recursive restore.
- The original `ChildMemberRegistry` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` stored `((slot, child_id), (dialect, member))`. It assumed every member was directly owned by the root and could not provide a recursive `OwnerRef` or indexed source row without scanning.
- `VcsArtifactApp::open_child` validates only the live root projection and commits one child immediately. `child_packs` emits a flat list. `load_document_text` and `load_document_pack` replace the parent and reset the document-scoped window state, but leave the child registry separate.
- Protocol `ChildPackEntry` in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` and its TypeScript mirror in `🧰️framework/🛍️products/💻️os/🟦️.ts` carry only a root-relative slot, child ID, dialect, and envelope bytes. The channel handles `LoadDocument` and `LoadChildren` as separate commands, so it cannot make a whole closure visible atomically.
- `SpaceMember::envelope_pack_bytes` already retains the authoritative full envelope representation by combining `print_document_pack` output with `encode_document_pack_bytes`; it preserves the child snapshot, SPR history, and operations.
- `ActiveArtifactStoreReplacement` in the plugin is the established retained replacement lifecycle. It builds a candidate parent store, checks live authority, commits the document-scoped window reset, swaps `self.store`, and incrementally retires the displaced or rejected store. It currently has no candidate child registry or closure validation phase.

## Smallest prerequisite completed

`SpaceMember` now exposes:

```rust
fn child_restore_projection(
    &self,
) -> Result<ChildRestoreProjection<'_>, ChildRestoreProjectionError>;
```

The `ArtifactStore<P, Mutation>` blanket implementation requires `P: ArtifactCompositionFields` and calls `ChildRestoreProjection::from_snapshot(self.snapshot_ref())`. `NoMembers`, `space_members!`, and the Store's manual test member delegate the method. The existing closed-dialect native fixture now reads the borrowed projection before bounded member close. A repository-wide implementation search found no other manual `SpaceMember` implementation.

The plugin registry prerequisite is also implemented. Each occupied row now retains its exact `ArtifactRef`, recursive `OwnerRef`, and closed-enum member. A preallocated dense ordinal table maps closure indices to physical hash slots in O(1), and `take_at` repairs the moved ordinal in O(1). `DocumentClosureSourceView` implements `OwnedDocumentClosureSource` by borrowing the root snapshot and those exact rows; child projections delegate to `SpaceMember::child_restore_projection`. Existing live operations still look up `(slot, child_id)` but read the stored reference and owner rather than reconstructing closure authority.

## Required candidate model

The retained replacement must own one unpublished candidate bundle:

1. Candidate parent `ArtifactStore<A, A::Mutation>`.
2. Complete candidate child registry with one dense row per member. Each row stores the exact `ArtifactRef`, exact recursive `OwnerRef`, and closed-enum member `M`.
3. Candidate `ChildContentView` and composition ownership graph built from those rows without publishing either.
4. Frozen input/source generation and the existing operation, cancellation, expiry, and retirement authority.

The dense rows provide O(1) `member_reference(index)`, `member_owner(index)`, and `child_projection(Some(index))` for `OwnedDocumentClosureSource`. A separate fixed-capacity lookup can preserve runtime lookup by child identity. Closure validation must use the root snapshot and each actual decoded member snapshot; input entries are transport claims and must never supply authoritative projections.

Every member must be opened with `M::begin_open(MemberOpenRequest)` and driven through `MemberOpenOperation::step` with the current `StepContext`. Rejection, cancellation, staleness, expiry, and post-open closure failure all drive the exact open operation, retained request pages, candidate member, and remaining candidate bundle through bounded close. No branch may call `M::open` or discard a candidate registry through an unbounded destructor.

## Atomic publication

Extend `ActiveArtifactStoreReplacement` rather than creating a second parent-replacement path. Its states should decode the parent, retain and open each child, validate `OwnedDocumentClosure`, build the unpublished content/ownership views, admit all displaced-state retirement capacity, and then reach one non-suspending commit boundary.

That boundary swaps together:

- `self.store`;
- the entire `self.children` registry;
- `self.child_content_root`;
- the composition ownership graph and its generation;
- the prepared document-scoped window replacement and document generation.

The live parent generation and candidate source generation must still match immediately before the swap. The old parent, old child registry, old content view, and old graph transfer into one incremental retirement owner. A failed or stale candidate transfers the equivalent unpublished state into bounded retirement. Render and command paths continue to observe only the old bundle until this boundary and only the new bundle afterward.

## Export and channel contract

Export needs a retained generation-fenced cursor over the root and every candidate member. Each exported child row must carry its exact recursive `OwnerRef`, exact target dialect, and full envelope bytes. The current flat `slot` field cannot distinguish equal slot names under different parents. Deterministic order should use the recursive owner coordinate plus child identity, while work remains page/grant bounded; one call must not clone or serialize the entire registry.

The current separate `LoadDocument` then `LoadChildren` protocol cannot express an atomic closure restore. The SDK/channel boundary eventually needs one retained root-plus-members candidate command, with the leaf case represented by zero members. This audit does not invent a replacement archive encoding: the existing full envelope pack and SPR data remain authoritative, and the protocol schema should change only when the retained candidate input shape is fixed.

## Validation required for integration

- Valid nested Stdio Kit → Object → Mesh/Brep/Value closure, including unordered transport entries.
- Missing, extra, duplicate, cyclic, wrong-parent, wrong-slot, wrong-child, and wrong-dialect members.
- More than 64 total descendants distributed across valid parents, while enforcing 64 child refs per parent and 1024 descendants per closure.
- Zero and small grants through input framing, member open, closure validation, candidate construction, and retirement.
- Cancellation, expiry, source-generation change, and live-generation change at every retained phase.
- Failure leaves parent, children, content view, composition graph, window state, and document generation unchanged.
- Success makes all six authorities visible together and resets document-scoped windows once.
- Export and reload preserve exact parent/member envelope packs, SPR histories, and recursive owners.
- App close during member open or closure validation retires the exact candidate and displaced owners once.

## Integration order

1. Compile and validate the completed `SpaceMember::child_restore_projection` prerequisite.
2. Compile and validate the completed exact reference/owner/member registry and O(1) ordinal index.
3. Compile and validate the completed immutable source view over parent `snapshot_ref()` plus member projections.
4. Extend `ActiveArtifactStoreReplacement` with retained member-open and closure-validation phases.
5. Commit and retire the complete bundle atomically.
6. Add retained recursive export, then change the protocol/SDK input boundary with its Rust and TypeScript schema facets and independent oracle.

## Implemented Checkpoint, 2026-09-12

All six integration steps above are now present in source. `ActiveArtifactStoreReplacement` retains the parent store, exact member ingress, decoded member registry, closure cursor, unpublished immutable content root and composition coordinator. Its non-suspending publication boundary swaps the parent, recursive members, child content, coordinator, pins and document-window generation together, then bounded-retires the displaced bundle. The ordinary public-maintenance success, rejection, cancellation and stale-authority laws pass on the configured normal stack in `🗑️generated/recursive-replacement-suite-native-36.log` (three laws); the archive law is the only failing test in that run.

Channel version 16 and `DocumentArchivePack` provide the atomic input/export boundary. Browser folder persistence now carries exact versioned archive bytes through ShellHost, worker, SQLite/file storage and plugin load/read. The native Store actor mirrors the archive actor vocabulary, storage and WGPU load route in source. Focused TypeScript and production storage checks pass in `🗑️generated/recursive-document-archive-typescript-5.log` and `🗑️generated/recursive-document-archive-storage-1.log`.

Suite 37 compiled and passed nine retained-window laws; only the archive law failed. Its preserved primary fault showed the parent had not reached `AwaitingMembers`, and cleanup exhausted without a replacement owner. The internal archive command was driving the envelope worker while omitting the field-decoder and completed-record return pumps required by that same retained ladder. Archive progress and exact cleanup now use the existing bounded three-owner decoder driver. Native verification remains pending.

Suite 38 again passed nine laws and isolated the archive failure to the parent decoder itself. Source inspection then established the structural mismatch without another diagnostic run: `load_document_archive` framed authoritative `.pack` + `.spr` bytes with `encode_document_pack_bytes` and submitted those binary bytes to `artifact_envelope_owned_schema`, whose tokenizer accepts a JSON object. That fresh JSON field decoder also intentionally constructs only fresh metadata, whereas the persisted SPR carries the exact dialect, owner, cursor, history messages, conflicts and checkpoint pins required by reload. The temporary worker-payload print was removed after this finding.

The corrected transport path keeps both contracts separate. The paged archive command decoder retains and transfers the bounded `DocumentArchivePack`; Store `parse_document_pack` reconstructs and validates its authoritative typed parent envelope; `begin_persisted_document_store_replacement` validates the exact app schema and dialect and transfers that envelope directly into `ArtifactApp::build_document_store_initialization_job`. The existing `ActiveArtifactStoreReplacement` then owns initializer progress, complete recursive member admission, closure validation and atomic publication. A rejected decoded parent is returned unchanged and cursor-retired through the app's exact `ArtifactEnvelopeDecodeOwnerBundle`. The composition fixture supplies a real initialization factory using `ArtifactStoreInitializationRuntime` and exact store owners. Its archive law additionally submits malformed Pack and foreign-schema parent candidates and requires the previously published recursive archive to remain byte-for-byte unchanged.

Suite 39 compiled this transport boundary and the neutral archive oracle, then overflowed the normal stack inside `ActiveArtifactStoreReplacement::drive_initializer` while taking the prepared parent candidate. The initializer trait now provides an out-of-line `take_boxed_candidate`; replacement ownership stays heap-resident, and the atomic commit swaps the boxed candidate with the live Store in place. The same box becomes the displaced Store retirement owner, avoiding another complete Store move through the maintenance frame. This correction is source-ready and awaits the coordinated native rerun.

The remaining execution boundary is larger than the transport fix. `parse_document_pack` still materializes and replays a complete history outside a retained operation, and `plugin_exchange` still invokes archive load synchronously while exclusively borrowing the app. Production completion requires a sequence-keyed archive-load operation admitted by the command, advanced under maintenance fuel, cancellable by an independent command, and acknowledged only after terminal cleanup. Its parser phase must own valid-prefix snapshots, mutations, edits and replay projections behind exact retirement factories. Both the parent and every member need nonempty-history coverage; `InitialMemberStoreOpen` currently admits genesis history only.

The native archive target now also owns exact nonempty event-log and text-folder round trips plus an actor refusal law: trailing, unsupported-version and over-limit candidates must leave the last accepted recursive archive unchanged. Native actor compilation, these persistence laws and the direct/paged archive codecs run next through the registered `recursive-document-archive-native` target. Hub bootstrap remains a root-pair descriptor and therefore publishes an empty-member archive until its server descriptor/assembler owns the complete closure.
