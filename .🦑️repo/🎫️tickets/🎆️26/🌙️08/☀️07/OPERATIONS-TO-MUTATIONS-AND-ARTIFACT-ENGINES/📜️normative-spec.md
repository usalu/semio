# Normative Spec: Operations → Mutations

Every later wave agent MUST follow this document. No legacy aliases.

## Glossary

| Term | Meaning |
|------|---------|
| **Mutation** | Declarative document change that yields a Diff and an Inverse (list of mutations that revert it). |
| **Op** | Handcrafted compact grammar/protocol that serializes mutations (`🔧️op`, `*.op`, `OpText`/`OpBinary`). Kept. |
| **Diff** | Projection delta produced by applying a mutation. Artifact-level aggregate + per-mutation specific. |
| **Inverse** | `Vec<Mutation>` arguments that revert a mutation when applied. |
| **ArtifactEngine** | UI-independent state machine; every transition is a mutation. Mandatory per artifact. |
| **Operator** | Neural/DAG eval operator (formerly `neural_engine::Operation`). Different concept. |

## Folder layout

```
<plugin>/🗿️artifacts/<artifact>/
  🧬️mutations/                         # REQUIRED facet
    🦀️component.rs                     # <Artifact>Mutation dispatch enum
    🟦️component.ts
    <emoji><kebab-name>/               # one dir per mutation; emoji unique within artifact
      🦠️mutation/
        🦀️component.rs                 # struct + builder
        �派克component.ts
      🔺️diff/
        🦀️component.rs                 # Diff this mutation yields for its args
        🟦️component.ts
      ↩️inverse/
        🦀️component.rs                 # inverse(&self, base) -> Vec<ArtifactMutation>
        🟦️component.ts
  🔧️op/                                # KEPT: grammar combining mutations
  🔺️diff/                              # KEPT: aggregate artifact Diff
  🗣️dsl/ 🎒️pack/ 📡️spr/ 📚️examples/
  ⚙️engine/                             # REQUIRED: ArtifactEngine impl
```

## Kind emojis (unique per kind)

| Kind | Dir name |
|------|----------|
| Mutations facet | `🧬️mutations` |
| Per-mutation struct+builder | `🦠️mutation` |
| Per-mutation inverse | `↩️inverse` |
| Diff (artifact + per-mutation) | `🔺️diff` |
| Op grammar | `🔧️op` |
| Engine | `⚙️engine` |

Specific mutation dirs each pick a unique emoji within their artifact (policy-enforced).

## Leaf files

Always `🦀️component.rs` / `🟦️component.ts` (taxonomy leaf names). Struct + builder live in `🦠️mutation/🦀️component.ts`.

TS: one facade at `🧬️mutations/🟦️component.ts` plus per-kind leaves under each mutation (structural completeness). Stubs allowed until WASM wiring.

## Traits

```rust
pub trait MutationDiff<P>: Clone + Default + Serialize + DeserializeOwned {
    fn apply(&self, base: &P) -> P;
    fn absorb(&mut self, other: Self);
}

pub trait Mutation<P>: Clone + Serialize + DeserializeOwned {
    type Diff: MutationDiff<P>;
    fn diff(&self, base: &P) -> Self::Diff;
    fn inverse(&self, base: &P) -> Vec<Self>;
    // retain optional metadata methods renamed from Operation:
    fn mutation_id(&self) -> Option<MutationId> { None }
    fn dependencies(&self) -> Vec<MutationId> { Vec::new() }
    fn base_version(&self) -> Option<DocumentVersion> { None }
    fn author_id(&self) -> Option<ActorId> { None }
    fn timestamp(&self) -> Option<HybridLogicalTimestamp> { None }
    fn undo_policy(&self) -> UndoPolicy { UndoPolicy::ExactBaseOnly }
    fn merge_strategy(&self) -> MergeStrategyKind { MergeStrategyKind::LwwRegister }
    fn conflict_rule(&self) -> ConflictRule { ConflictRule::Merge(self.merge_strategy()) }
    fn state_class(&self) -> StateClass { StateClass::Persistent }
    fn reconcile(&self, projection: P) -> (P, Vec<ReconcileReport>) { (projection, Vec::new()) }
    fn validate(&self, _projection: &P) -> Result<(), String> { Ok(()) }
}

pub trait ArtifactEngine: Send + Sync {
    type Projection;
    type Mutation: Mutation<Self::Projection>;
    type Diff: MutationDiff<Self::Projection>;
    fn projection(&self) -> &Self::Projection;
    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, EngineFault>;
    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation>;
}
```

`OpText` / `OpBinary` stay. `DiffCodec` stays. Existing byte-cache `Engine` trait stays (different concept).

Each concrete mutation is a struct implementing `Mutation<P>` (or helper that the dispatch enum delegates to). `<Artifact>Mutation` is a thin dispatch enum.

## Old → new identifier table

### Kernel / protocol

| Old | New |
|-----|-----|
| `Operation<P>` | `Mutation<P>` |
| `OperationDiff<P>` | `MutationDiff<P>` |
| `Operation::backwards` | `Mutation::inverse` |
| `Operation::operation_id` | `Mutation::mutation_id` |
| `OperationId` | `MutationId` |
| `OperationMeta` | `MutationMeta` |
| `OperationMeta.operation_id` | `MutationMeta.mutation_id` |
| `Edit.backwards` | `Edit.inverse` |
| `Edit.operation_meta` | `Edit.mutation_meta` |
| `OperationDescriptor` | `MutationDescriptor` |
| `register_operation_descriptor` | `register_mutation_descriptor` |
| `operation_descriptor` | `mutation_descriptor` |
| `OperationUpcaster` | `MutationUpcaster` |
| `OperationEvent` | `MutationEvent` |
| `OperationEvent.operation_id` | `MutationEvent.mutation_id` |
| `CommandOutcome.effects: Vec<OperationEvent>` | `Vec<MutationEvent>` |
| `OperationEnvelope` | `MutationEnvelope` |
| `OperationEnvelope.operation_id` | `mutation_id` |
| `InverseOperation` | `InverseMutation` |
| `OpDag` | `MutationDag` |
| `OperationTransform` | `MutationTransform` |
| `CollectionOperation` | `CollectionMutation` |
| `apply_collection_operation` | `apply_collection_mutation` |
| `invert_collection_operation` | `inverse_collection_mutation` |
| `collection_diff_from_operation` | `collection_diff_from_mutation` |
| `apply_operation` | `apply_mutation` |
| `DocumentCommand<Operation>` | `DocumentCommand<Mutation>` |
| `DocumentCommand::Apply { operations }` | `Apply { mutations }` |
| `AmendLast { operations }` | `AmendLast { mutations }` |
| `replay_operations` | `replay_mutations` |
| `DocumentVcs<P, Operation>` | `DocumentVcs<P, Mutation>` |
| `DocumentStore<P, Operation>` | `DocumentStore<P, Mutation>` |
| `DocumentApp::Operation` | `DocumentApp::Mutation` |
| `ConfigOperation` | `ConfigMutation` |
| `DraftOperation` | `DraftMutation` |
| `NoConfigOperation` | `NoConfigMutation` |
| `NoDraftOperation` | `NoDraftMutation` |
| `Emit.document_operations` | `Emit.document_mutations` |
| `Emit.config_operations` | `Emit.config_mutations` |
| `Emit.draft_operations` | `Emit.draft_mutations` |
| `Emit::operations` helper | `Emit::mutations` |
| `neural_engine::Operation` | `neural_engine::Operator` |

### Per-artifact

| Old | New |
|-----|-----|
| `<X>Operation` enum | `<X>Mutation` dispatch enum |
| `apply_<x>_operation` | `apply_<x>_mutation` |
| `invert_<x>_operation` | `inverse_<x>_mutation` |
| `*_operations` builders | `*_mutations` |
| `host_operations` | `host_mutations` |
| `ops_from_host_mutation` | `mutations_from_host` |

### TypeScript

| Old | New |
|-----|-----|
| `KernelOperation` | `KernelMutation` |
| `OperationEnvelope` / `WireOperationEnvelope` | `MutationEnvelope` / `WireMutationEnvelope` |
| `operation_id` | `mutation_id` |
| `InverseOperation` | `InverseMutation` |
| `applyOperations` | `applyMutations` |
| `encode/decodeOperationEnvelopesPack` | `encode/decodeMutationEnvelopesPack` |
| `operationEnvelopeToWire` / `FromWire` | `mutationEnvelopeToWire` / `FromWire` |
| `remoteOperations` / `pendingOperations` | `remoteMutations` / `pendingMutations` |
| `inverseOperations` | `inverseMutations` |
| `relayOperationsToHub` | `relayMutationsToHub` |
| `Puzzle2dLiveMirrorOperations` | `Puzzle2dLiveMirrorMutations` |
| `ActionDefinition.kind: "operation"` | `"mutation"` |
| `backbone kind: "operations"` | `"mutations"` |

### Grammars / protocols / examples

| Old | New |
|-----|-----|
| `start operation` | `start mutation` |
| production `operation =` | `mutation =` |
| `schema <x>.operation` | `schema <x>.mutation` |
| (filenames `*.op.semio`, folder `🔧️op`, `grammar <x>.op`) | **unchanged** |

### Taxonomy / policy

| Old | New / Add |
|-----|-----------|
| `artifactComponentDirs` includes `🔧️op` | also add `🧬️mutations`, `⚙️engine` |
| — | `mutationChildDirs: ["🦠️mutation","🔺️diff","↩️inverse"]` |
| `POLICY_PROTOCOL_MIGRATION_NAMES` Operation* | Mutation* |
| `type Operation =` scanners | `type Mutation =` |
| `OperationDiff` scanners | `MutationDiff` |
| `POLICY_TS_FACADE_ALLOWLIST` | structural rule (no per-file allowlist) |

## Kept (op brand)

`🔧️op`, `🔧️ops`, `*.op.semio`, `grammar <x>.op`, `OpText`, `OpBinary`, `print_op`, `parse_op`, `encode_op`, `decode_op`, `LanguageRole::Ops`, `dsl::DslOps`, `DslVariants`.

## Untouched (different concepts)

GraphQL `OperationDefinition`, compose kit worker `operation` discriminators, CAD scripting/kernel `operation` fields, 2d boolean `operation`, ink/NodeGraph UI event `operation`, “no-operation” comments, math `inverse`/`Inverter`.

## Engine ownership

1. App `handle` **builds** mutations only → `Emit.document_mutations`.
2. Store `dispatch(Apply { mutations })` drives `ArtifactEngine::apply` / `inverse`.
3. Engine owns projection; compute helpers (`LowpolyDocument`, hosts) are engine internals.
4. Every artifact MUST have `⚙️engine` implementing `ArtifactEngine`.

## Wave gates

- W1: `cargo check -p semio-framework-os-kernel` (with `DEVELOPER_DIR=/Library/Developer/CommandLineTools` if needed)
- W2: taxonomy + policy parse; scanners report missing mutations for unmigrated artifacts
- W3: lowpoly green (`cargo test -p semio-s-plugin-lowpoly --lib`)
- W4: each plugin crate `cargo check -p <crate>`
- W5: renderer vitest green
- W6: `verify-gate` + `policy` + zero legacy `Operation` for document-mutation concept

## File ownership

- Wave 1/2/6 only: kernel crates, `taxonomy.json`, discovery, registry, root `📜️script.ts`
- Wave 3: `semio-s-plugin-lowpoly` only
- Wave 4: one plugin crate per agent
- Wave 5: framework-core / framework-os / renderer TS only
