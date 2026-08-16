# Contract Freeze — Plugin Dependencies, Artifact Contributions, Composite Mutations

Frozen 2026-08-16 by the coordinator. Every lane codes against exactly these names, shapes and rules. A lane that needs a change to this contract stops and reports; it does not improvise.

## 0. Crate / alias map

| Path | Crate | In-crate alias used by callers |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` (`📦️glue.rs` `#[path]` tree) | `semio-framework-os-kernel` | `protocol` (= `os_spr::command`), `store`, `dsl`, `pack`, `spr`, `semio` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` | ↑ same crate | `protocol::…` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` | ↑ same crate | `protocol::AppCommand`/`AppFrame` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` | ↑ same crate | `store::…` |
| `🧰️framework/📦️packages/🦀️rust` (`🔨️modules/🛂️manifest`, `🚪️io`) | `semio-framework` | `semio_framework::…`, `io::…` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust` | `semio-framework-plugin` | guest SDK |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust` | `semio-framework-plugin-host` | wasmtime host |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust` | `semio-framework-os-kernel-dsl-derive` | `dsl_derive` |

**Dependency edge law:** `semio-framework` depends on `semio-framework-os-kernel`, never the reverse. Therefore **nothing added to `protocol`/`store` may name `semio_framework::*` or `io::*` types** (`ArtifactRef`, `PluginId`, `InvocationId`, `ArtifactKindId`, …). Cross-boundary identities travel as plain `String`/newtypes declared inside `protocol`. This is the same rule `MutationMeta.group_id` already documents.

## 1. M1 — composite mutation spine (`📡️spr/🎮️command/🦀️component.rs`, new `//#region 🔖️Composite`)

```rust
/// 🎯️ A mutation step aimed at an artifact OTHER than the one being mutated.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignTarget {
    pub artifact_id: String,                 // io::ArtifactRef.artifact_id, as a plain string
    pub artifact_kind: String,               // canonical `s.<plugin>.<artifact>` where migrated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,             // io::ArtifactDialect coordinate string, when pinned
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignStep {
    pub target: ForeignTarget,
    pub mutation_id: crate::os_spr::ids::SchemaId,  // owner kind `"<doc-schema>#<kind>"` OR contributed id (§3)
    pub payload: Vec<u8>,                    // OpBinary bytes of the target Op, or contributed payload bytes
    pub label: String,
}

pub enum PlanStep<Op> { Local(Op), Foreign(ForeignStep) }

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum MutationOrigin {
    Owner,
    Contributed { plugin_id: String, mutation_id: crate::os_spr::ids::SchemaId, payload_hash: crate::os_spr::ids::PayloadHash },
    Transaction { initiator: ForeignTarget },
}
impl Default for MutationOrigin { fn default() -> Self { Self::Owner } }

pub const MAX_PLAN_DEPTH: u8 = 8;

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum PlanError {
    #[error("plan depth {0} exceeds MAX_PLAN_DEPTH")] DepthExceeded(u8),
    #[error("plan cycle on {0}")] Cycle(String),
    #[error("step rejected: {0}")] StepRejected(String),
    #[error("{0}")] Invalid(String),
}

pub struct Planner<P, Op: Mutation<P>> { /* base: P, steps: Vec<PlanStep<Op>>, depth: u8, seen: Vec<(String,[u8;32])> */ }
impl<P: Clone, Op: Mutation<P>> Planner<P, Op> {
    pub fn new(base: &P) -> Self;
    pub fn base(&self) -> &P;
    pub fn call(&mut self, op: Op) -> Result<(), PlanError>;          // validate → push Local → advance base by op.diff(base).apply(base)
    pub fn call_foreign(&mut self, step: ForeignStep) -> Result<(), PlanError>;
    pub fn steps(&self) -> &[PlanStep<Op>];
    pub fn into_steps(self) -> Vec<PlanStep<Op>>;
}

pub trait CompositeMutationKind<P, Op: Mutation<P>>: Clone + serde::Serialize + serde::de::DeserializeOwned {
    const SEMANTICS: SemanticDescriptor;
    fn plan(&self, base: &P, planner: &mut Planner<P, Op>) -> Result<(), PlanError>;
    fn label(&self) -> String;
    fn target(&self) -> Vec<String> { Vec::new() }
    fn validate(&self, _base: &P) -> Result<(), String> { Ok(()) }
}

// Free helpers — NOT a blanket impl (a blanket `impl<T: CompositeMutationKind> MutationKind for T`
// is rejected by coherence against the ~200 concrete `impl MutationKind` in the tree).
pub fn plan_of<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> Result<Vec<PlanStep<Op>>, PlanError>;
pub fn fold_plan_diff<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> <Op as Mutation<P>>::Diff;
pub fn fold_plan_inverse<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> Vec<Op>;
pub fn plan_foreign_steps<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> Vec<ForeignStep>;
```

Additions to existing traits (both **defaulted**, so no implementor breaks):
- `Mutation<P>`: `fn foreign_steps(&self, _base: &P) -> Vec<ForeignStep> { Vec::new() }`
- `MutationKind<P, Op>`: same defaulted method.

`MutationMeta` gains `#[serde(default, skip_serializing_if = "MutationOrigin::is_owner")] pub origin: MutationOrigin`. All 21 struct-literal sites (`📡️spr/{🎮️command,🔀️crdt,🔗️causal,🧪️testkit}`, `🏪️store`) get `origin: MutationOrigin::Owner,` — a one-line mechanical fixup lane 0-A is explicitly allowed to make outside its lease.

`MutationDescriptor` gains `#[serde(default)] contributor: Option<String>` and `#[serde(default)] artifact_kind: Option<String>`, set via `.with_contributor(..)` / `.with_artifact_kind(..)` builder methods (never new `::new` parameters). **Neither participates in `fingerprint`** — the golden pin `operation_descriptor_fingerprint_is_golden_pinned` must stay green unchanged.

### Derive
`dsl_derive` gains `#[derive(CompositeMutation)]` with `#[composite(snapshot = Ty, op = OpEnum)]`, emitting the delegating `impl MutationKind<Snapshot, Op>` (`diff`→`fold_plan_diff`, `inverse`→`fold_plan_inverse`, `foreign_steps`→`plan_foreign_steps`, `SEMANTICS`/`label`/`target`/`validate`→delegate) plus the same `const _: () = assert!(…)` kind/verb checks `#[derive(Mutations)]` already emits. `#[derive(Mutations)]` gains a `foreign_steps` match arm delegating per variant.

### Laws (unit tests owned by 0-A)
1. `fold_plan_diff(k, b).apply(&b)` equals sequential application of the plan's local steps.
2. `fold_plan_inverse(k, b)` applied after the composite restores `b`.
3. Composite-of-composite nests and folds identically to the flattened plan.
4. Depth > `MAX_PLAN_DEPTH` and repeated `(kind, payload_hash)` are typed `PlanError`, never panics.
5. Foreign steps never appear in `fold_plan_diff` output.

## 2. M2 — channel & transaction protocol (`📡️spr/🧵️channel` + TS mirror)

Wire law: `tag: u8` = **variant declaration order**, fields in declaration order, no per-field tags. New variants are **appended**; `CHANNEL_VERSION` 8 → **9**.

Next free tags: `AppCommand` = **22**, `AppFrame` = **19**.

```rust
// AppCommand, appended in this order (tags 22,23,24,25,26):
TransactionPrepare { seq: u64, txn_id: String, mutation_id: String, payload: Vec<u8>, prepared_ops: Vec<Vec<u8>>, label: String, origin: Vec<u8> },
TransactionCommit  { seq: u64, txn_id: String },
TransactionRollback{ seq: u64, txn_id: String },
TransactionUndo    { seq: u64, group_id: String },
TransactionRedo    { seq: u64, group_id: String },

// AppFrame, appended in this order (tags 19,20,21,22):
TransactionProposal { in_reply_to: u64, proposal_id: String, local_ops: Vec<Vec<u8>>, description: String, coalesce_key: String, foreign: Vec<Vec<u8>> },
TransactionPrepared { txn_id: String, foreign: Vec<Vec<u8>>, rejection: Vec<u8> },
TransactionCommitted{ txn_id: String, edit_id: String },
TransactionRolledBack { txn_id: String },
```

Encoding notes frozen: a `TransactionPrepare` carries EITHER `mutation_id`+`payload` (owner-mutation form; `prepared_ops` empty) OR `prepared_ops`+`label`+`origin` (pre-planned form; `mutation_id` empty) — flat fields, not a nested enum, so the hand-rolled codec stays one level deep. Each `foreign` element is one `ForeignStep` encoded with `store::pack_rt::encode_wire_value` of its serde form. `rejection` empty = no rejection. `origin` is the wire-encoded `MutationOrigin`. Empty `String`/`Vec` are the absent markers (no `Option` combinators added).

TS mirror (`🧰️framework/🛍️products/💻️os/🟦️component.ts`, `AppChannelCodec` region): same tags, same field order, `AppCommandValue`/`AppFrameValue` variants named identically in camelCase (`transactionPrepare`, …). Golden vectors live in `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/` as hex `.json` and are asserted from both Rust and vitest.

## 3. M3 — identity grammars (frozen)

- Plugin id: existing `[package.metadata.component] package = "semio:<plugin-id>"` value.
- Owner mutation id (unchanged): `"<document-schema>#<kebab-kind>"`.
- **Contributed mutation id: `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"`.** The `:` segment makes collision with an owner kind impossible and is parseable both ways.
- Contributed inference: existing `(artifact_kind, inference_schema)` key; `inference_schema` of a contributed inference MUST start with `s.<contributor-plugin-id>.`.
- Version requirement grammar (in-repo parser, no external crate): `=X.Y.Z`, `^X.Y.Z`, `~X.Y.Z`, `>=X.Y.Z`, `*`.

## 4. Registration gates (frozen)

1. A contribution targeting artifact kind `K` is accepted only if `K`'s owning plugin is a **direct** entry in the contributor's declared `dependencies` (and, for extensions, `extends` is `dependencies[0]`).
2. Runtime `dependencies` ⇔ Cargo `semio-s-plugin-<id>` dependency, both directions, enforced by policy gate.
3. Router conflict: two sources claiming the same `(artifact_kind, mutation_id)` or `(artifact_kind, inference_schema)` is an error unless byte-identical metadata (same idempotence rule `ArtifactInferenceRouter::register_plugin` already uses).
4. Contributed inference metadata: `owner == contributor plugin id`, `artifact_kind == target`.
5. Dependency graph: missing dependency, version mismatch, or cycle ⇒ plugin load rejected with a typed error; unload/hot-reload refused while dependents are loaded.

## 5. Transaction protocol (frozen, both hosts)

1. Guest dispatch produces foreign steps ⇒ guest emits `TransactionProposal` and applies **nothing**.
2. Host mints `txn_id`, registers member #0 = initiator instance with its `local_ops`.
3. Foreign step resolution: `InstanceDirectory(target) → (plugin, instance)`; `ArtifactMutationRouter(artifact_kind, mutation_id) → Owner | Contributed{plugin}`. Owner ⇒ `TransactionPrepare` (owner-mutation form). Contributed ⇒ host calls the contributor's `contributor.artifact-mutation-plan` with the target's current snapshot pack, then sends `TransactionPrepare` (pre-planned form) to the target instance.
4. Recursion over returned `foreign`, depth ≤ `MAX_TXN_DEPTH` (= `MAX_PLAN_DEPTH`), cycle key `(artifact_id, mutation_id, payload_hash)`. One instance appears at most once per transaction; a second visit appends ops to that member in discovery order.
5. Phase 1 succeeds only if every member answered `TransactionPrepared` with empty `rejection`; otherwise `TransactionRollback` to all prepared members + typed fault to the initiator.
6. Phase 2 commits in **reverse discovery order** (deepest member first, initiator last). Each member applies its ops as ONE `Edit` with `group_id = txn_id` and per-op `MutationMeta.origin`. A commit failure ⇒ `TransactionUndo{group_id}` to already-committed members, then rollback the rest.
7. Group undo/redo fan out `TransactionUndo`/`TransactionRedo{group_id}` to every member of that group.
8. A guest rejects `TransactionCommit` whose recorded `base_generation` no longer matches its current generation.
9. **One pending transaction per instance.** A `TransactionPrepare` arriving while another transaction is pending on that instance is rejected (`TransactionPrepared` with a typed `rejection`); the host must not retry it inside the same transaction. This is what makes the flat `pending: Option<PendingTransaction>` guest field correct rather than a latent overwrite bug.
10. **A pending transaction freezes its instance's mutating surface.** While `pending` is set, any command that would emit artifact mutations (including a second gesture from the UI) is rejected with a typed fault rather than applied or queued; read-only commands (`RefreshUi`, `ReadDocument`, `ContextMenu`, ephemeral lanes) continue to work. Without this rule the initiator's already-computed `local_ops` could be planned against a base that no longer exists by commit time, and the §5.8 generation check would turn a routine second click into a transaction abort.

### Rejection taxonomy (typed, frozen)

`transaction.dependency-missing`, `transaction.version-mismatch`, `transaction.unknown-target`, `transaction.unknown-mutation`, `transaction.contribution-not-permitted`, `transaction.depth-exceeded`, `transaction.cycle`, `transaction.instance-busy`, `transaction.generation-mismatch`, `transaction.member-rejected`, `transaction.commit-failed`. Hosts and guests use these exact codes so a UI or a test can distinguish "the plugin said no" from "the transaction machinery said no".

## 6. WIT (frozen shape)

`interface contributor` exports, moved out of `plugin`: `list-artifact-mutations`, `artifact-mutation-plan`, `list-artifact-inferences`, `artifact-infer`. `world plugin-world { import host; export plugin; export contributor; }`; `world extension-world { import host; export extension; export contributor; }`. **No new `host` imports** — the guest never calls out mid-mutation. `artifact-inference-request` gains `dependencies: list<tuple<string, list<u8>>>`.

⚠️ `📜️world.wit` is under concurrent edit by ticket `26/08/16/FULL-STDIO-…` (it just replaced `artifact-infer`'s byte envelope with typed records). Re-read before every edit; never restore an older shape; coordinator serializes WIT writes.
