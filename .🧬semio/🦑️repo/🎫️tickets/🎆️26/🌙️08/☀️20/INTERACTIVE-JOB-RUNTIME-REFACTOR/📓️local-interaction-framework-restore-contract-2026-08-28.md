# Local Interaction Framework Restore Contract

## Status And Chosen Boundary

This is the concrete next native contract, not mounted code or permission to begin a Store cutover. The canonical semantic fixture merge is separately actual RED→GREEN at `📓️canonical-restore-semantics-source-r1-red-2026-08-28.md` and `📓️canonical-restore-semantics-source-r2-green-2026-08-28.md`: fourteen cases, original twelve unchanged, existing full five-field identities preserved, independent Immer and jsonc-parser references. Native/local publication, cancellation and timing are unexecuted.

The command remains an ordinary registered framework-owned command using the existing invocation/retained command ingress and result/ACK protocol. No new AppChannel tag, WIT event, poll export, query token, numeric lifetime authority or restore-specific transport is introduced. The proposed registry verb is `restoreLocalInteraction`; its payload schema is the existing `semio:local-interaction#/$defs/restore`. Both `full` and `domains` are already canonical variants.

The prospective Plugin test domain remains `🕹️interaction/📡️live/🧪️tests/🔁️restore-full-sparse` with empty-leaf fixture/schema/native spellings selected by taxonomy. Its membership is not released, so this packet creates none of those files. The test domain will reference the existing canonical case IDs, not copy fourteen payload/state rows.

## Commands Are Not Historical Generation Authority

Two typed roles are necessary, not two transport routes:

- Proposed `RestoreLocalInteractionCommand` is a transparent Rust command wrapper around `protocol::LocalInteractionRestore`. Its `base` must match freshly captured live authority. It remains held with the original admitted raw input through decoding, rejection and publication.
- Proposed `ReplaceLocalInteractionState` is the direct persistent mutation leaf for the prepared result, transparently carrying the canonical three-map `LocalInteractionState` content. Its schema references `semio:local-interaction#/$defs/state`. The command's generation/revision authority is not copied into a historical inverse as permission to mutate a future runtime. Forward content is the exact complete candidate; inverse content comes from the exact captured original local root. This avoids a self-referential inverse identity/digest and preserves historical content-only semantics.

The leaf updates only persisted local `selection`, `activeMode` and `activeGranularity`. It does not replace hover, document, shared configuration, Presence, transient state or another instance. It is not an alias for the four-field cold SetInteractionState leaf. The live preparation must install the exact prebuilt retained root, not call SetState, clone a whole state, or call generic cold `Mutation::apply`.

This is a proposal for review: no new opcode/schema source is mounted yet. When admitted, the adjacent fourteen-field descriptor and real `dsl::MutationLeaf` derive must produce the direct-leaf provenance. The parent Interaction mutation roster includes that exact leaf and actual descriptor; no handwritten token, borrowed app owner witness, compatibility alias or binary discriminator invented outside the canonical codec is permitted. Text/binary history schema remains the single authored event representation across implementations.

## Actual Existing Pipeline And Required Join

The inspected production `dispatch_typed_command_inner` accepts `Box<A::Command>` and checks mounted instance, complete operation pipeline and admitted command identity before allocating the ordinary operation. `start_typed_command_operation` captures operation/cancellation/publication fields but explicitly rejects `QualifiedToolProof::FrameworkOwned` at `interactive-job.framework-route`. A framework restore must be selected by the exact existing framework registration before any A::Command-only decode. It cannot pass itself off as A::Command to get through this rejection.

The intended join is one typed framework payload arm in the same mounted operation owner, backed by a concrete `FrameworkLocalInteractionRestoreJobFactory<A>` registered through the current framework tool registry. Its exact owner/controller/tool/schema/factory tuple is checked by the existing proof machinery. Existing `ToolWireAdmission` and `RetainedToolWireInput` custody remain tied to that actual framework factory; app-command admission witnesses are not reused. The current synchronous writer or JSON decode does not become retained merely because this command has a factory: staged raw/partial-domain decode is a required producer dependency.

`MountedTypedCommandFullOperation<A>` is the existing structural operation home. Its session, cancellation lease, terminal outcome, pending publication and result page must continue to exist there outside any fallible callback. The additional domain owner is not a second global restore queue or a transient async local. The concrete payload/completion arm must preserve the exact retained restore candidate on error. Existing `ArtifactToolCompletionValue` currently contains only Emit/Download, so the required domain-owned completion cannot be claimed already present.

The internal publication contract gains an exact Interaction arm, and `PendingArtifactStorePublication` gains the actual typed Interaction Store publication owner. These are native ownership variants, not new result-wire tags. The existing terminal result lane/token remains the external ACK path. An ACK authorizes retirement only of the exact operation's held publication and result; it is not a local-interaction identity or a query ACK. The command does not require a new result lane just to return private state. UI invalidation uses the existing admitted UI/result path and must retain its own native patch authority.

## Actual Store Limitation, Not An Implicit Ready Seam

`ArtifactStore::begin_apply_one` and `ArtifactStoreOneItemPreparationFactory` provide the appropriate typed preparation/admission shape. However `begin_apply_one_owned` explicitly rejects every lane except `HistoryLane::Document`, and `advance_apply_one` repeats that restriction during commit preflight. It also refuses an outbound backbone. Simply passing `HistoryLane::Interaction`, labeling the factory HostOnly, or treating this separate Store as a document would be false authority.

The next Store packet must implement the actual retained Interaction side-lane preparation/metadata/commit within that existing one-item publication state machine. It needs its own expected Interaction Store generation/revision, exact history/cursor ownership, preadmitted displaced roots and fixed receipt, plus unchanged document-lane laws. The generic prepared seal and canonical digest checks remain intact. Existing `prepared`/`take_prepared` transfers must remain structurally recoverable under errors; no source-take before a fallible target admission.

There is also a representation boundary: current `protocol::InteractionState` stores BTreeMaps, while `LocalInteractionRoot` stores canonical retained OrderedMaps. A retained candidate cannot be followed by a whole BTreeMap conversion. The chosen long-term join is a single canonical retained local root for these three persisted fields, with unchanged external schema and separately preserved hover ownership; it is not a sidecar cache with a second truth. This representation/decoder/serializer adoption is a prerequisite for the live factory and must be separately reviewed/tested across existing consumers. No such adoption was performed in this slice.

## Retained Candidate State And Phases

The proposed `LocalInteractionRestorePreparation` implements the existing one-item preparation interface after the actual lane/representation prerequisites. It structurally owns the original admitted restore input, document/config/interaction reads, fixed full identity, exact registry/topology inputs, one unexposed candidate root, one current domain key/patch, one `LocalInteractionRootUpdate`, and all pending retirement/serialization/history owners. A borrowed method receiver or ManuallyDrop shell alone is not the enclosing recovery owner.

The sequence is:

1. Preadmit the original operation/decoder/candidate/retirement/result storage before producers and capture exact current roots under the real exclusive app owner. Verify the request's full base and live instance before candidate mutation.
2. Decode with the canonical retained raw-input owner. Each consumed page transfers bytes/values into independently admitted semantic storage before input ACK. Retain partial values and the original fault on later field/EOF rejection.
3. For full restore, initialize an empty unexposed three-map root and traverse the union of selection, mode and granularity keys. Do not enumerate selection alone. For sparse restore, capture the original immutable three-map root and visit only supplied domain patches. Null removes that domain in the respective field; empty selection is not absence; an empty sparse patch preserves the original content.
4. Reuse `begin_domain_patch`/`LocalInteractionRootUpdate::advance` for one domain's three-field candidate. Store each completed domain back into the same private outer candidate, and close the prior update/retirement cursor before reusing its field. Do not expose a partially applied multi-domain root.
5. Validate actual registry domain, supported mode/granularity, selected IDs and anchor policy against captured topology in retained steps. A non-member selection anchor is allowed where the existing schema/semantic law permits it; do not silently normalize or split literal comma IDs. The fixture's private label is not a real SelectionSpec.broadcast=false declaration.
6. Prepare exact persistent forward/inverse content, metadata, canonical digest and history state in retained phases. Capture fresh publication authority again under the original app owner and revalidate all five public identity fields plus the actual document/config read authorities and captured registry/topology source.
7. Commit one exact complete Interaction root in the existing one-item publication owner. Keep displaced roots, prepared event/input and fixed receipt structurally held until their own retirement and exact result ACK. Never infer completion from an empty source slot.

Every phase consumes the existing one-item/byte grant, including comparator bytes, key/value copies, actual page allocations, descriptor movement, final shell release and tail clock. A page count or fixed number of domain phases is not physical work proof. No higher budgets, whole-state serde credit, unbounded union allocation or live cold conversion is proposed.

## Identity And Cancellation

The exact public identity remains appInstanceId, decimal generation, revision, documentRevision and topologyRevision. The current producer derives generation/revision from the Interaction Store and topologyRevision from the existing document/config/UI-generation authority. Configuration authority is already included there; do not add a sixth wire field or reuse a historical tutorial identity.

`LocalInteractionInputReads` already retains document/config reads and validates their original commit authorities, but its own transfer/recovery edges still need actual mounted checks. Interaction adds its original Store read and exact publication seal. A topology or registry change between candidate turns and commit rejects the whole candidate while the original Store remains authoritative.

Prepublication cancellation begins close of the retained decoder, current update, candidate, semantic input and all original reads. The actual root stays unchanged. Postpublication cancellation cannot undo an accepted commit: it preserves the already-issued exact receipt/result/ACK and the displaced-root retirement. Callback panic/clock fault must leave every owner in the same mounted operation slot; callback-tail quiescence and resident funding remain prerequisites, not inferred from a caught panic or a COMPLETE flag.

## Next Test-First Packet

After taxonomy releases the test domain and resident/Opening integration is coherent, stage native tests before implementation:

- All fourteen canonical semantic IDs, preserving all five stale dimensions and the large Unicode case, through the real typed framework registration, no A::Command compatibility path.
- Actual direct-leaf descriptor/codec/forward/inverse bijection and absence of generation authority in persistent content, with independent schema/codec reference calculation.
- Genuine current-API rejection of Interaction lane, followed by exact lane-owned preparation/commit; preserve document-lane refusal and seal laws.
- Full restore clearing selection-only, mode-only and granularity-only absent domains; sparse empty/no-op and explicit private replacement; exact hover preservation and no private Presence emission.
- Grants zero/one-byte-short/1/64/4096, allocation refusal before producer, one domain crossing multiple pages, and no candidate exposure between domains.
- Real cancellation/unwind/fault after decoder placement, within each map update, after private candidate transfer and before/after publication; original pointers and final typed root/ACK retirement observed before assertions.
- Wrong operation/instance/generation/sequence/attempt ACK, stale document/config/topology/registry, live source replacement and original callback-tail ownership.

The first live packet must use actual Interaction Store/root/registry types, not a synthetic successful restore shell. All of these native/live obligations remain unexecuted.

## Read-Only Source Observations

Observed source hashes after the scoped reads; these are not a native compiled snapshot:

| Source | SHA-256 |
| --- | --- |
| Plugin main | `2ad816977def25ded3175c87c0f7d03344f1bae57549689b17388adf871736ca` |
| Store main | `0ed0d7a78c833c1081825c598de3a5dde36ecc858a2e1448c5695899358efd0d` |
| LocalInteractionRoot | `fb9069928312a57213d9a2dbe67ab5a55a1fa48b127e5e84817155714c312e82` |
| LocalInteractionRootUpdate | `d4f9c7d2259f7962a3206f268fd64c3b5c399aba0ee8238eaf2d251025e6ccdc` |
| Local topology authority | `f450c8eca21d2bf66d35d6f968808423472a71b70ec66196f2280fb7aae0c59f` |
| Local document/config reads | `298d7c2ab53e9a2b0bb1d38bbab298f7e103d4bddf3a344689f9d42eea41cf9e` |

Resident source `508b7872…` and tests `ebde45c9…` were rechecked unchanged. No Plugin, Store, native root, resident or new-domain source was changed by this contract.
