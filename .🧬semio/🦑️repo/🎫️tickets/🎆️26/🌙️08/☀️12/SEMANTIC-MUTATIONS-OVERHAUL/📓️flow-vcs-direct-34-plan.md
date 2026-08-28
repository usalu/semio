# Flow VCS Direct Mutation Plan

## Scope

This is a source-read-only inventory for the current `🌊️flow/🌿️vcs` `FlowMutation`. It has four public wrappers but ten actual codec operations: four widget collection operations, four synapse collection operations, layout assignment, and whole-fixture replacement. A four-row descriptor roster would erase eight semantic operations and is rejected.

The direct aggregate target is `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/🦀️.rs`. It will be a transparent ten-variant `FlowMutation` deriving `dsl::Mutations` and `dsl::DslOps`; every variant wraps a leaf payload. Direct owners are its ten children, each with `🦀️.rs`, `🔣️.json`, `🧬️schema/🔣️.json`, and leaf-owned tests. There are no aliases for `Widgets`, `Synapses`, `SetLayout`, or `SetFixture`.

## Canonical Roster and Descriptor Contract

Every row has these exact common descriptor values: `schemaVersion: 1`, `payloadSchema: "🧬️schema/🔣️.json"`, `outcomeClasses: ["applied"]`, `composition: "atomic"`, and `requiredLanguageSurfaces: ["rust", "json-schema", "text", "binary"]`. Its `owner` is exactly `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/<direct-owner>`. `textOpcode` equals `semanticKind`; `binaryTag` is unique in this aggregate. Each payload schema is Draft 2020-12, closed (`additionalProperties: false`), requires every listed property, and admits Unicode scalar strings only.

The exact `displayName` values, in row order, are `Add Widget`, `Remove Widget`, `Move Widget`, `Change Widget`, `Add Synapse`, `Remove Synapse`, `Move Synapse`, `Change Synapse`, `Change Layout`, and `Replace Flow Fixture`. The ten emoji prefixes are distinct and each has a retained existing taxonomy witness checked by the ticket oracle.

| Direct owner | Aggregate variant / semantic kind / grammar | Payload contract | Emoji | Tag | Invertibility / diff participation |
| --- | --- | --- | --- | --- | --- |
| `➕️add-widget` | `AddWidget` / `add-widget` / `add-widget` | `index: u32`, `widget: WidgetPayload` | `➕️` | 0 | explicit-mutation / detect |
| `🗑️remove-widget` | `RemoveWidget` / `remove-widget` / `remove-widget` | `id: String` | `🗑️` | 1 | explicit-mutation / detect |
| `↔️move-widget` | `MoveWidget` / `move-widget` / `move-widget` | `id: String`, `toIndex: u32` | `↔️` | 2 | explicit-mutation / apply-only |
| `🩹change-widget` | `ChangeWidget` / `change-widget` / `change-widget` | `id: String`, `widget: WidgetPayload` | `🩹` | 3 | explicit-mutation / detect |
| `🔗️add-synapse` | `AddSynapse` / `add-synapse` / `add-synapse` | `index: u32`, `synapse: SynapsePayload` | `🔗️` | 4 | explicit-mutation / detect |
| `✂️remove-synapse` | `RemoveSynapse` / `remove-synapse` / `remove-synapse` | `id: String` | `✂️` | 5 | explicit-mutation / detect |
| `🔀️move-synapse` | `MoveSynapse` / `move-synapse` / `move-synapse` | `id: String`, `toIndex: u32` | `🔀️` | 6 | explicit-mutation / apply-only |
| `🔄change-synapse` | `ChangeSynapse` / `change-synapse` / `change-synapse` | `id: String`, `synapse: SynapsePayload` | `🔄` | 7 | explicit-mutation / detect |
| `📐️change-layout` | `ChangeLayout` / `change-layout` / `change-layout` | `entries: [LayoutEntryPayload]` | `📐️` | 8 | explicit-mutation / detect |
| `♻️replace-flow-fixture` | `ReplaceFlowFixture` / `replace-flow-fixture` / `replace-flow-fixture` | `fixture: FlowFixturePayload` | `♻️` | 9 | explicit-mutation / apply-only |

The approved verbs are exactly `add`, `remove`, `move`, `change`, and `replace`; their past forms already exist in the shared vocabulary. `Change*` intentionally replaces the generic collection term `Patch`: the leaf still maps to `CollectionMutation::Patch` at its application boundary, but the public semantic identity is not a generic transport primitive.

`WidgetPayload` is a closed nine-way discriminated union for the current `Widget` variants: `neuron`, `inputSlider`, `inputNote`, `inputImage`, `variable`, `outputPreview`, `outputAction`, `outputExport`, and `cluster`. It preserves the current required/defaulted field shape, including `Neuron.params`, `Cluster.tree`, and `Cluster.flow`; it must not widen those foreign neural values to an unconstrained object. `SynapsePayload` requires `id`, `from`, `to`, `fromPort`, and `toPort`. `LayoutEntryPayload` requires `id` and explicit nullable `layout`; `layout` is either `{x:number,y:number}` or `null`.

The current Rust collection API uses `usize` for `index` and `to_index`; it has no portable cross-language maximum. The direct payload proposal uses `u32`, with a checked `usize::try_from` only at the leaf application boundary before constructing `CollectionMutation`. This is the required schema/codec boundary change, not an assertion about the old wire contract.

## Snapshot Replacement Decision

`SetFixture` is an actual whole-`FlowFixture` replacement used by `FlowHost` history, not a cosmetic aggregate wrapper. The current fixture has exactly `schema`, `camera`, `widgets`, `synapses`, and `layout`; it has no attached artifact reference or foreign-step payload. Therefore this plan makes it the atomic `ReplaceFlowFixture` domain-import leaf: its diff is `FlowDiff { fixture: Some(...) }`, its inverse is one `ReplaceFlowFixture { fixture: snapshot.clone() }`, and it emits no foreign steps.

This decision is conditional on that current self-contained shape. If an attachment/reference is introduced before cutover, replacement becomes a composite import with explicit foreign steps and the corresponding descriptor `composition` must change to `composite`; renaming `SetFixture` alone is not an acceptable substitute.

## Codec and Consumer Cutover

Current text/binary grammar is the `FlowMutationDsl` ten-variant twin: `widgets-add`, `widgets-remove`, `widgets-move`, `widgets-patch`, `synapses-add`, `synapses-remove`, `synapses-move`, `synapses-patch`, `layout`, and `fixture`. The cutover deliberately changes the generic/twinned identities to the ten canonical grammar values in the roster. `DslOps` on the aggregate delegates tuple variants to leaf `DslRecord`; leaf modules own the `#[dsl]` field grammar and any `WidgetPayload`/`SynapsePayload` adapters. The central `FlowMutationDsl`, `flow_mutation_to_dsl`, and `flow_mutation_from_dsl` are removed.

Exact production consumers to rewrite in the later released implementation are:

- `🌊️flow/🌿️vcs/🦀️component.rs`: mount/reexport the aggregate; remove the inline enum and manual `Mutation<FlowFixture>` implementation; route `FlowDiff`, inverse, and `flow_fixture_operations` through wrapped leaves; replace all codec/store tests with the ten direct variants.
- `🌊️flow/🖥️host/🦀️component.rs`: two real history dispatches at the pending-change and gesture-commit paths become `FlowMutation::ReplaceFlowFixture(ReplaceFlowFixture { fixture })`; their comments and history tests follow the same identity.
- The independent `✏️s/🔌️plugins/🌊️flow/.../FlowMutation` is a distinct plugin artifact type and is not a consumer of this OS Flow VCS type. No change there is in scope.

`FlowConfigMutation::SetContributions` is not in this source, is not in the roster, and is explicitly excluded. The Flow snapshot-retirement implementation remains infrastructure and is not redesigned by this adoption.

## Current Red Gates

The existing `CollectionDiff` is not order-preserving. `collection_diff_from_mutation` discards an `Add.index`; it turns `Move` into `removed + added`; and `apply_flow_collection_diff` finishes by extending additions. Therefore `[a,b,c] + Add(index=1,x)` currently applies as `[a,b,c,x]`, not `[a,x,b,c]`, while moving `c` to index zero applies as `[a,b,c]`, not `[c,a,b]`. Direct leaves must use a real ordered diff/application representation that carries insertion and relocation coordinates, and must prove forward and inverse order recovery. They must not wrap `CollectionDiff` for these operations.

The existing detector is also central and operation-specific: `flow_fixture_operations` branches over widgets, synapses, and layout while `MutationKind` exposes only `diff` and `inverse`; there is no leaf registration/detection seam. Consequently the `detect` descriptor rows are planned identities, not current behavior. Full acceptance requires a narrow repository-owned leaf-detection contract that each detecting leaf implements and that `#[derive(Mutations)]` mechanically collects into aggregate output. It must not add a Flow-specific central switch. If that shared contract is not released, the affected descriptor values must remain `apply-only` and the direct cutover is incomplete.

The ticket oracle runs this as a retained red baseline: it validates the complete ten-row proposed descriptor roster against the authoritative descriptor schema, then confirms that the canonical aggregate and all ten leaf descriptors are absent in the current source and demonstrates the two order failures above. This is not a compiler or runtime pass.

```text
[DEBUG] Flow VCS inventory wrappers=4 codecOperations=10 historySetFixtureCallers=2
[DEBUG] Authoritative descriptor proposal accepted leaves=10 canonicalSourceRed aggregate=false descriptors=0
[DEBUG] Ordered collection red insert=a,b,c,x move=a,b,c
```

## Test Matrix

Each leaf receives schema-first positive/negative fixtures, descriptor/provenance validation, text and binary round trips, and apply/inverse recovery from a nontrivial fixture. The aggregate receives only structural correspondence checks: ten variants, kinds/descriptors, exact unique tags/opcodes, and unknown envelope rejection. The direct test cases cover duplicate/missing ids, invalid checked indices, nullable layout removal, widget and synapse variant discrimination, change inversion, ordered move inversion, and full-fixture replacement/undo history. A ticket-local Bun oracle validates all ten descriptor JSON documents against the authoritative descriptor schema and validates representative valid/invalid payloads through Ajv; it is separate from the root-owned Rust compiler/runtime gate.

## Write Boundary

Allowed only after root release: the Flow VCS aggregate and its ten new direct owner trees, the Flow VCS component, the two FlowHost history call sites/tests, and a dedicated ticket test/report directory. No FlowConfig, shared protocol/derive changes, Store/Workflow/Run source, plugin Flow sources, compose paths, or Cargo artifacts are in this packet.
