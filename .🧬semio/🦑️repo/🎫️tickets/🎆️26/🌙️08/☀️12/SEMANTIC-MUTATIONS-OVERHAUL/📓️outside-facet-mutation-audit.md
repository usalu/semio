# Outside-Facet Mutation Audit

## Scope and Evidence

The baseline remains `d03b1fdb6da7c4ea97043e5618d8f4098a43dff7`. Luna performed read-only anchored-source searches outside the excluded root and build/cache/ticket trees. These counts are a qualified lexical census, not an exhaustive AST proof. No production conversion is accepted from this audit alone.

The first census found 115 `Mutation` implementation declarations (114 concrete declarations and one macro template), eight `CompositeMutationKind` implementations (one production and seven tests), one outside-facet `MutationKind`, one `Mutations` derive, and one `CompositeMutation` derive. These categories overlap and must not be summed into a mutation count. Aggregate variants still require individual classification.

The coordinator explicitly rejects exemptions based solely on an implementation lacking the current `MutationKind` trait, being presence/configuration/transient/session state, or being called generic infrastructure. The user's definition covers operations accepted by commands/codecs, applied to snapshots, emitted by diff, or exposed in schema/union surfaces.

## Production Aggregate Candidates

These central operation families require owner/leaf inventory and a coordinated consumer cutover:

| Type | Source |
| --- | --- |
| WorkflowMutation, RunMutation | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` |
| FlowMutation | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` |
| SpaceMutation, CollectionMutation | `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs` |
| SpaceHistoryMutation | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` |
| DagMutation | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs` |
| ModulePayloadMutation | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs` |
| CreateBuildingStorey | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🦀️component.rs` |

## Runtime State Is Included

Luna identified 80 outside-leaf runtime-state implementations: 76 editor presence/configuration/transient implementations, plus Norm and Space presence/configuration. This is an additional cutover queue, not an approved exception list.

The 76 editor declarations cover architect, block2d/3d/5d, cad, draw, fem2d/3d, flow, forms, gis2d/3d, home, imperative, jack, layout, lowpoly, mathematical, note, playbook, present, procedural2d/3d, process3d, puzzle2d/3d/5d, raster, remodel, rewrite, sequence, shooting, sourcing-curate, space-index, vcs-demo, wires, and writer.

Concrete source evidence returned by Luna:

- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs`: public `Block2dPresenceMutation::Snapshot`, apply/inverse, `DslOps`, text and binary codecs.
- Sibling `🎚️config/🦀️component.rs`: public `Block2dConfigMutation::{Snapshot, SetLocale}`, application, codecs and inverse tests.
- `✏️s/🔌️plugins/📕️norm/👥️presence/🦀️component.rs`: public `NormPresenceMutation::Noop`, mutation and both codecs. A sentinel must be removed from the concrete roster, not preserved as a leaf merely to meet the count.
- `✏️s/🔌️plugins/📕️norm/🎚️config/🦀️component.rs`: `NormConfigMutation::{Snapshot, SetSelectedCheckIndex}`, apply and codec tests.
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/👥️presence/🦀️component.rs`: public `SpacePresenceMutation::Snapshot`, apply/inverse and both codecs.
- Sibling `🎚️config/🦀️component.rs`: `SpaceConfigMutation` exposes Snapshot, SetActiveNode, SetFocusedNode, SetClipboard, SetCollapsed, SetPreviewOff, SetCamera, SetWorkflowEngagementInput, SetCompiledDagEngagementInput, SetPendingImport, SetSpaceId, SetClient, SetActivePanelTab and SetLocale, with explicit application and codec tests.

Snapshot/setter names above are current names, not approved target semantic names. Whole-state replacement requires an explicit domain justification or removal as a fallback.

## Generic and Synthetic Boundaries

- `HashMutation` in `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs` is feature-gated production hash overwrite used by artifact-store/version graph. It needs a truthful infrastructure-owned leaf or a redesign below the public mutation boundary.
- `ProbeMutation` in `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs` is public, codec-registered and applied to ProbeSnapshot. Synthetic naming does not make it exempt.
- `SetArtifactMutation<D>` in `✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs` is used by public `commit_document<D>` in the sibling `🖥️app-surface/🦀️component.rs`. One type spanning multiple document schemas cannot supply one truthful document-owner identity. Specialize actual document operations or remove the generic operation from the public command/codec boundary.

Test implementations remain compile-consumer fixtures to update, not grounds for an optional descriptor or default bypass. The audit found these in SPR command/testkit, replication/causal, db/artifact, plugin builder/reactor/component, and store/sync/store. The final inventory must distinguish test consumers from production operations without hiding either.

## Remaining Work

Promote outside-facet implementation facts into the taxonomy inventory, enumerate every concrete aggregate variant, trace commands/codecs/schemas, assign direct owners and semantic target leaves, and validate with compiler-backed fixtures. This report does not satisfy the exhaustive census or zero-violation gates.
