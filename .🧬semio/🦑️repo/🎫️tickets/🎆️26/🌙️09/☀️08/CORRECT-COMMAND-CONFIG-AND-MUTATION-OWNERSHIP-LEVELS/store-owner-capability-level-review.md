# Store Owner Capability Level Review

## Finding

The catalog renamed from MemberStoreOwners to DocumentStoreOwners is shared by more than persisted artifact documents. Its present fields describe Store lifecycle authority: current/initial snapshot retirement, mutation retirement, disposal, and optional typed/wire preparation. A required persisted-artifact snapshot factory must not be added blindly to every occurrence in the 87-source rename ledger.

This is a source inspection finding. No native test or production change is claimed for it. The lower retained Pack physical-source repair can proceed independently.

## Concrete Evidence

| Source owner | Observed use | Capability implication |
| --- | --- | --- |
| OS Store `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2156` | DocumentStoreOwners has four retirement/disposal fields plus optional typed and member-wire preparation. Its type bounds do not require ArtifactPack or artifact identity. | The existing bundle is generic Store lifecycle authority. |
| Same Store source, installer near 15250 | install_document_store_owners_exact installs those lifecycle fields into generic ArtifactStore. | Installation alone does not admit an archive or document identity. |
| NoConfig `🔌️plugin/🎚️config/🚫️none/♻️retirement/🦀️.rs:6` | no_config_store_owners returns DocumentStoreOwners<NoConfig, NoConfigMutation>. | An empty configuration lane needs lifecycle ownership without a persisted-artifact factory. |
| NoDraft `🔌️plugin/📝️draft/🚫️none/♻️retirement/🦀️.rs:6` | no_draft_store_owners uses the same bundle for DraftStore. | Draft publication/retirement does not establish archive artifact identity. |
| Interaction `🔌️plugin/🕹️interaction/♻️retirement/🦀️.rs:151` | interaction_store_owners constructs it for InteractionState and InteractionConfigMutation. | Interaction lifecycle remains distinct from artifact archive loading. |
| Window framework `🔌️plugin/🪟️window/🎚️config/🦀️.rs:201` | bounded_window_config_store_owners returns the bundle for each exact WindowConfigOwner. | Window config lifecycle must stay reusable without document-only capabilities. |
| FEM3D Model window `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🦀️.rs:127` | build_store_owners returns DocumentStoreOwners for exact model-window config and delegates to the generic window helper. | A real current consumer demonstrates the distinction. |
| Store aliases near 3740 and 10455 | DraftStore and ConfigStore are aliases of ArtifactStore, with their own semantic boundaries. | Shared underlying algebra does not make their external persistence capabilities identical. |

Framework-relative plugin paths in this table are under `🧰️framework/🛍️products/💻️os/🔨️modules`. Line positions are observations from this review and may shift with concurrent edits.

## Required Refinement Before Factory Integration

The next Store design should separate generic lifecycle ownership from persisted-document decode authority. A generic Store lifecycle bundle can retain the common retirement/disposal/preparation capabilities. A document-specific bundle composes that lifecycle bundle with the required typed persisted-snapshot decoder. The common root/member hydrator accepts the document-specific capability and therefore has no optional decoder or synchronous fallback. Config, draft, interaction, and exact window consumers retain only the capabilities their own lane requires.

Names and physical folder placement need a focused Terra audit before another broad rename. This report proposes the boundary, not a compatibility alias or a runtime type test. Existing valid artifact factories, compressed canonical input and custom Flow formats remain required. The 87-source rename ledger is an occurrence inventory, not a catalog of 87 archive-reachable document grammars.

The audit should classify constructor sites by actual return type/call path, distinguish parent/member/document from config/draft/interaction/test owners, determine which wire-preparation capability is generic versus member-specific, and propose a schema-first compile/native law that prevents installing document decode authority into a lane that does not possess the required identity. It must also verify whether config persistence needs a separate retained snapshot factory on its own config boundary; no capability should be removed merely because it is not an artifact archive.

## Relation To Existing Proposal

This refines executable slices 4–5 of retained-artifact-pack-hydration-architecture-audit.md and the corresponding architecture proposal. Their lower Pack/page/catalog/value/codec repairs and final activation gate remain valid. Do not add a dummy rejection factory to every non-document catalog merely to satisfy a new generic field.

## Review Guardrails From The Terra Trace

HistoryLane and ArtifactToolPublicationLane are different axes. Store HistoryLane::Document is the ordinary Undo/Redo history within that exact store; Interaction is a persisted replayable side history skipped by ordinary undo. Plugin ArtifactToolPublicationLane selects the artifact/config/draft/exact-window target store and guards emissions before dispatch. Config and draft correctly record ordinary history in their own stores. A generic Primary name may express the first axis better, but adding Config/Draft variants to HistoryLane or redirecting these edits would conflate the axes and change behavior incorrectly.

The encoded one-item mutation request contains operation, expected revision/generation, actor, group, schema and bytes, with no ArtifactRef, OwnerRef or parent/child relation. Its existing Member naming reflects its first composition surface rather than a member-specific parser input. The actual publication is bound afterward to its exact store's lease registry, and another store rejects it. Source inspection found no non-test caller of that wire-entry method. A config alias acquiring the blanket SpaceMember method surface is therefore a mixed type-surface concern, not evidence of active cross-store mutation or accepted artifact archives. Without an envelope dialect, artifact_ref() returns None; ordinary config lifecycle owners do not install the wire factory.

The refined design must keep generic lifecycle, typed/encoded mutation preparation, exact-store publication binding, store-local history policy, and identity-bearing document composition/loading separate. Any claim that an existing route admits the wrong owner requires an actual counterexample through the registry/admission path. The audit must not manufacture a runtime failure from an overloaded name alone.

## Concrete Window-Load Validation Gap To Exercise

Root confirmed the current TypedWindowConfigStoreOwner::load implementation at plugin/window/config source lines457–470 parses the complete Pack/SPR, compares only envelope.schema with O::SCHEMA, reconstructs applied/redo IDs, then resets the partition requested by the outer window_id. It does not compare the decoded envelope id with the partition's generated window-config:{kind}:{window_id} identity. This is a source-level missing check; no foreign-inner-id native counterexample has yet been run.

The retained window-load slice should add that exact mismatch row and preserve the previous partition, input ownership and bounded decoded-candidate retirement on refusal. A schema-only happy-path reopen is insufficient evidence. Do not add a synchronous drop of a rejected large decoded envelope merely to make an identity assertion pass; validation and retained retirement belong in the window-load capability already proposed by the audit.
