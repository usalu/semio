# 🧭️ r1 — How artifacts and mutations are structured today

Read-only exploration for the BIM plugin (`model` artifact). Root of all relative paths: `C:\git\semio\`.
Verbatim file dumps are in the appendices at the end; they were generated from the files at read time, so they cannot drift from the code by transcription.

## 0. Scope and verification status

- Method: reading code and committed fixtures only. No `cargo`, no `verify mutation-outcome-law` gate run, no `bun test`. Nothing below is "verified by run".
- Design read: `📋️design.md` (laws L1–L5, violation codes V1–V5, fixed framework API), `📋️executor-brief.md`, `📓️coordination.md`, `📓️exec-fw-spine.md`, `📓️exec-fw-gate.md`.
- Exec reports exist only for `fw-spine` and `fw-gate`. No `exec-*.md` exists for drawing, shooting or any leaf, so no leaf is confirmed converted to the target shape. The exemplar in section 4 is judged from code and test bodies, not from a run.

## 1. Artifact directory tree

### 1.1 Addressing rule

`✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>/🏅️standards/🔖️<standard>/🪆️subsets/<subset>/…`

- Plugin crate lives at `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>/📦️packages/🦀️rust/Cargo.toml`. Crate names: drawing `semio-s-artifact-draw-drawing`, shooting `semio-s-artifact-shooting-shooting`.
- `🔖️1` is the standard. `🪆️subsets/✳️any` is the wildcard subset that holds the whole artifact. Drawing additionally splits into `🧱️structure`, `🎨️style`, `🏷️metadata`, `🔀️transform`; each split subset owns its own `🧬️schema/🧬️mutations`, `🧫️fixtures`, `🧪️tests`, `🔮️oracles`.
- Modules are mounted by hand, not by discovery. The artifact root `🦀️.rs` contains a `#[path = "."] pub mod standards { pub mod v1 { pub mod subsets { pub mod any { pub mod schema { … } } } } }` tree. Each leaf is mounted as `pub mod <kind> { #[path = "…/🦀️.rs"] mod component; #[path = "…/🔺️diff/🦀️.rs"] pub mod diff; #[path = "…/↩️inverse/🦀️.rs"] pub mod inverse; pub use component::*; #[cfg(test)] #[path = "…/🧪️tests/…"] mod tests_…; }`. Shooting: `🗿️artifacts/🎥️shooting/🦀️.rs` (mount block around line 768 onward, `#[cfg(test)]` only on the `tests` module). Drawing: `🗿️artifacts/🖍️drawing/🦀️.rs` (`pub mod standards` at line 567).

### 1.2 What each folder is

| Folder (as named in the repo) | Role | Observed example |
|---|---|---|
| `🧬️schema` | JSON Schema is the source. Hand-written projections beside it: `🦀️.rs`, `🟦️.ts`, `🛰️.proto`, `🔗️.graphql`. `🔣️.json` at this level is the artifact schema (shooting: `$id` `https://json.schemas.assets.semio-tech.com/s/shooting/shooting/artifact.json`, title `ShootingArtifact`). | `…/✳️any/🧬️schema/🔣️.json` |
| `🧬️schema/📸️snapshot` | Snapshot type (`ShootingSnapshot`, `DrawingSnapshot`) with the same five-projection set; its own `🔣️.json` (shooting `$id` `…/snapshot.json`). | `…/✳️any/🧬️schema/📸️snapshot/🦀️.rs` |
| `🧬️schema/🔺️diff` | Artifact-level diff type (`ShootingDiff`, `DrawingDiff`) plus `impl MutationDiff` / `impl DiffAlgebra`; its own `🔣️.json` (shooting `$id` `…/diff.json`). | `…/✳️any/🧬️schema/🔺️diff/🦀️.rs` |
| `🧬️schema/🧬️mutations` | Dispatch enum `🦀️.rs` (`#[derive(dsl::Mutations)]`) and one directory per kind. Its `🔣️.json` is an aggregate `oneOf` of leaf `$id`s (shooting). | `…/✳️any/🧬️schema/🧬️mutations/🦀️.rs` |
| `🧬️mutations/<kind>/` (leaf root) | Leaf descriptor `🔣️.json` (owner, semanticKind, aggregateVariant, payloadSchema, binaryTag, invertibility, diffParticipation, outcomeClasses, composition, requiredLanguageSurfaces) plus the leaf's `🦀️.rs` or `🦠️mutation/🦀️.rs`. | `…/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🔣️.json` |
| `🦠️mutation` | Payload struct + `impl protocol::MutationKind` (`diff`, `inverse`, `label`, `target`). Drawing uses this facet. Shooting puts the payload at `<kind>/🦀️.rs` with no facet. Both are accepted by the derive (see 3.4). | drawing `🗑️delete-layer/🦠️mutation/🦀️.rs`; shooting `🏷️rename-shot/🦀️.rs` |
| `🔺️diff` | Builder `pub fn diff(payload, base) -> protocol::MutationOutcome<…Diff>`. Target: declarative, sparse, reads `base` only. | shooting `🏷️rename-shot/🔺️diff/🦀️.rs` |
| `↩️inverse` | Builder `pub fn inverse(payload, base) -> Result<Vec<…Mutation>, ValueError>`. Target: concrete mutations built from payload plus `base`. | shooting `🚮️delete-shot/↩️inverse/🦀️.rs` |
| `🧪️tests` | Leaf tests. One folder per scenario (`🔤️relabels`, `🚫️removes`, `➕️appends`). Tests `include_str!` the committed fixtures. `🥒️.feature` at subset level holds language-agnostic cases. | shooting `🏷️rename-shot/🧪️tests/🔤️relabels/🦀️.rs` |
| `🧫️fixtures` | Committed vectors: per case `📸️snapshot/⬅️before/🔣️.json`, `📸️snapshot/➡️after/🔣️.json`, `🦠️mutation/🔣️.json`, `🔺️diff/🔣️.json`, `🎯️outcome/🔣️.json`. Drawing also has SVG pairs `⬅️before.svg` / `➡️after.svg` under `🧱️structure/🧫️fixtures/<case>/` and `🎛️input-choices/🔣️.json` (editor vocabulary). | `…/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/` |
| `➡️after` | Expected snapshot after the mutation, inside a fixture case. | `…/📸️snapshot/➡️after/🔣️.json` |
| `✏️editor` | Editor app. Commands live in `✏️editor/🎮️commands/<group>/🦀️.rs` and return `Emit<ArtifactMutation, ConfigMutation>`. Also `🎚️config`, `🎭️modes`, `👥️presence`, `📌️panels`, `🕹️interaction`. | shooting `✏️editor/🎮️commands/📦️asset/🦀️.rs` |
| `👁️viewer` | Viewer surface. Declares `type Mutation = …ShootingMutation`, and no actions. | shooting `👁️viewer/🦀️.rs` |
| `🏭️generator` | Third-party oracle generator (drawing: `json-rust` engine). Writes reviewed fixture pairs (`generate`) and refreshes sha256 in `🔮️oracles/🔣️.json` (`manifests`). It is NOT a type generator. | `…/✳️any/🏭️generator/📜️script.ts` |
| `🚪️io` | Codecs: `📝️text` (with a `.grammar.semio`), `💾️binary`, `📤️export`, `📥️import`, `🪶️sqlite`, `🧪️tests`. | shooting `🚪️io/💾️binary/🧬️mutations/🦀️.rs` |
| `📚️examples` | Example documents. The subset root exports them through `examples()`. | drawing `…/✳️any/📚️examples/🎬️demo` |
| `🔬️probes` | Probe script(s) and a reader used by oracles. | drawing `…/✳️any/🔬️probes/📜️script.ts` |
| `🔮️oracles` | `🔣️.json` manifest (see 6.3): `oracles`, `noOracleDecisions`, `mutationCatalogs` (kinds + vectors), `mutationManifests` (`semio.repository-test.mutation-manifest/v2`, `productionDispatch`, `oracleRequirements`), `testEvidence` (sha256, bytes, generator). | `…/🧱️structure/🔮️oracles/🔣️.json` |
| `🖼️assets` | Binary demo assets (`🎬️demo`). | drawing `…/✳️any/🖼️assets/🎬️demo` |

Not inspected in depth: drawing artifact-root `🤖️generated/` (`📇️registry`, `🔠️types`, `🖌️drawing-layers`) and `🛂️manifest/`.

### 1.3 Two complete trees

- Drawing structure subset tree and shooting `✳️any` tree: see Appendix A (generated `find` output, directories and files).
- One mutation kind leaf, complete, with every file quoted: drawing `delete-layer` in `🧱️structure` (Appendix B). Shooting `rename-shot` in `✳️any`, also complete (Appendix C), is the exemplar for section 4.

## 2. Schema-first

- **Where the JSON Schemas live.** Each artifact has `🧬️schema/🔣️.json` (artifact, snapshot, diff). Each leaf has `🧬️mutations/<kind>/🧬️schema/🔣️.json` (payload schema). Each leaf also has `🧬️mutations/<kind>/🔣️.json` (leaf descriptor, not a schema; it is parsed by the derive). The aggregate `🧬️mutations/🔣️.json` (shooting) is a `oneOf` of leaf `$id`s. Fields carry vendor keywords such as `x-semio-state: "artifact"` and `x-semio-ui`. The payload root may carry `x-semio-inverse-rows` (read by the derive; see 3.4).
- **Are Rust types generated from schemas?** No generator found for artifact types. Rust types are hand-written and annotated with derives, and the derives read the JSON at compile time. Evidence:
  - Framework `🧰️framework/🔨️modules/🧬️schema/🧬️entity-kinds` has a generated file (`🤖️generated.rs`, header `// @generated by framework/schema/script.ts — do not edit.`). That file covers the entity catalog only.
  - `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts` routes `generate`, `preview-generated`, `check` (entity catalog) and `test` (contracts `entity-ownership`, `subset-contract`, `mutation-leaf-registration` (independent Ajv oracle), `neutrality`).
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json` is generated by `dsl-derive-rs` `generate` (script `…/✨️derive/📦️packages/🦀️rust/📜️script.ts`, Ajv-validated) from the repo taxonomy `📋️project.json`. It tells the derive which roots are mutation collections and which owners are domain operations.
  - The hand-written `🟦️.ts`, `🛰️.proto`, `🔗️.graphql` projections are not generated (headers are hand-written; no generator for them found).
- **Snapshot, diff and payload declarations** (quoted from the code):
  - Snapshot: `#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)] pub struct ShootingSnapshot` (`…/📸️snapshot/🦀️.rs:5-10`).
  - Diff: `#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)] #[value(rename_all = "camelCase", default)] #[artifact_schema(id = "s.shooting.shooting")] pub struct ShootingDiff { #[state(artifact)] pub schema: Option<String>, … }` (`…/🔺️diff/🦀️.rs:9-30`). Every field is an `Option`, so "absent" means "untouched".
  - Payload: `#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)] #[mutation_leaf(contract = ::protocol)] #[cfg_attr(test, serde(rename_all = "camelCase"))] #[value(rename_all = "camelCase")] pub struct RenameShot { pub id: String, pub new_label: String }` (shooting `🏷️rename-shot/🦀️.rs`). Drawing's payload uses `DslRecord, dsl::MutationLeaf` and `#[dsl(keyword = "create-layer")]`.
  - Dispatch enum: `#[derive(…, dsl::Mutations)] #[mutations(snapshot = ShootingSnapshot, diff = ShootingDiff, schema = "shooting.shooting")] pub enum ShootingMutation { RenameShot(super::rename_shot::RenameShot), … }`.
- **Drift observed (not runtime-checked).** Shooting `🔺️diff/🔣️.json` has a top-level `artifact` property (`x-semio-state: "artifact"`, title `ShootingArtifact`). The Rust `ShootingDiff` has no `artifact` field (fields: `schema, assets, saved_cameras, scene, shots, active_shot_id, active_asset_id, emblem`). Drawing's committed diff fixture carries `"artifact": null`, and the drawing diff schema has no `artifact` property. Treat this as an open question for the BIM schema.

## 3. Framework mutation API

Crate and module map:
- `protocol` = the replication crate (`🧰️framework/🔨️modules/📡️replication/`). Plugins alias it: `extern crate semio_framework_os_kernel as protocol;`. The kernel re-exports the replication API at its root (`apply_diff`, `MutationDiff`, `ApplyCapability`, `Mutation`).
- Core contract file: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` (1867 lines).
- OS command layer (`MutationKind`, `SemanticMutation`, `CompositeMutationKind`, `Planner`, `fold_plan_*`, `NamedTripleDiff`): `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`.
- Kernel facade: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs` (`pub mod os_spr`, line 143; `protocol_laws` mount at line 188–189, `#[cfg(any(test, feature = "protocol-laws"))]`).

### 3.1 Traits (signatures as in code)

- `pub trait MutationDiff<P>: Clone + Default + PartialEq + ToValue + FromValue + DiffAlgebra<P>` (line 117)
  - `fn apply(&self, base: &P, capability: ApplyCapability) -> MutationApplyResult<P>;`
  - `fn absorb(&mut self, other: Self);` (doc: sequential coalesce, base-free, total, never a success path for a rejection)
  - `fn retire_cold(self) where Self: Sized {}` and `fn retire_projection(projection: P)`.
- `pub trait DiffAlgebra<P>: Sized` (line 171)
  - `fn inverse(&self, base: &P) -> Self;` (the negative diff)
  - `fn between(base: &P, other: &P) -> Self;` (sync/import only; the doc says leaves must not use it)
  - `fn is_empty(&self) -> bool;`
  - Stale doc: the LAWS paragraph (around line 165–170) writes `.await` in its formulas. The code is sync. Ignore the `.await`.
- `pub trait Mutation<P>: Clone + ToValue + FromValue` (line 190)
  - `type Diff: MutationDiff<P>;` `const DESCRIPTORS: &'static [MutationLeafDescriptor];`
  - `fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;`
  - `fn inverse(&self, base: &P) -> Result<Vec<Self>, semio_framework_value::ValueError>;`
  - Defaults for: `retire_cold`, `mutation_id`, `dependencies`, `conflict_target`, `base_version`, `author_id`, `timestamp`, `undo_policy` (`UndoPolicy::ExactBaseOnly`), `state_class`, `may_emit_foreign_steps`, `inverse_rows`, `foreign_steps`, `INPUT_SCHEMAS`, `input_schema`, `payload_value`, `with_payload_value`, `from_payload_value`.
- `pub trait MutationLeaf` (line 987): `const DESCRIPTOR: MutationLeafDescriptor; const PROVENANCE: MutationSourceProvenance; const PAYLOAD_SCHEMA: &'static str; const PAYLOAD_SCHEMA_DOCUMENTS: &'static [&'static str]; fn input_schema(&self) -> Option<&'static str>`.
- `pub trait MutationKind<P, Op>: MutationLeaf + Clone + ToValue + FromValue where Op: Mutation<P>` (`📡️spr/🎮️command/🦀️.rs:219`)
  - `const SEMANTICS: SemanticDescriptor;` `fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>;`
  - `fn inverse(&self, base: &P) -> Result<Vec<Op>, ValueError>;` (doc: missing or already-absent target ⇒ `Vec::new()`)
  - `fn label(&self) -> LocalizedLabel;` (every locale, no default) `fn timestamp`, `fn target(&self) -> Vec<String>`, `fn may_emit_foreign_steps`, `fn foreign_steps`.
- `pub trait SemanticMutation<P>: Mutation<P>` (line 261): `kinds()`, `semantics()`, `label()`, `target()`. Implemented only by the derive.
- `pub trait CompositeMutationKind<P, Op: Mutation<P>>` (line 799): `const SEMANTICS`, `fn plan(&self, base: &P, planner: &mut Planner<P, Op>) -> Result<(), PlanError>`, `label`, `timestamp`, `target`.

### 3.2 Outcome, errors, apply capability

- `pub struct MutationOutcome<D> { diff: D, messages: Vec<MutationMessage> }` (line 1254). Fields are private.
  - Constructors: `empty()`, `fatal(code, msg, target)`, `error(code, msg, target)`, `refuse(OutcomeCode, msg, target)`, `new(diff)`.
  - Accessors and builders: `diff()`, `messages()`, `into_parts()`, `info`, `warning`, `absorb_messages`, `stamp_op_index`, `worst_level`, `is_applicable(MergePolicy)`, `map`.
  - `MutationOutcome::apply_to(&mut P)` is gone. The only `apply_to` left is `MapDelta::apply_to` in `🗂️map/🦀️.rs:93`, which is a different type.
- `pub struct MutationApplyError { pub code: String, pub message: String, pub target: Vec<String> }` (line 20). `pub type MutationApplyResult<P> = Result<P, MutationApplyError>;` (line 92).
- `pub struct ApplyCapability { _sealed: () }` (line 97), `#[derive(Clone, Copy, Debug)]`. The private field means only this module can construct it.
- `pub fn apply_diff<P, D: MutationDiff<P>>(diff: &D, base: &P) -> MutationApplyResult<P>` (line 104) is the one mint point: `diff.apply(base, ApplyCapability { _sealed: () })`.

### 3.3 Central appliers and folds

- `protocol::apply_diff` (above). Every store, replay and fold path calls it.
- `store::apply_operation(state, operation, op_index)` (`🏪️store/🦀️.rs:25219`): `operation.diff(state).stamp_op_index(…)`, then `apply_diff`, then `retire_cold`.
- `store::apply_outcome(base, outcome)` (`🏪️store/🦀️.rs:25232`): applies a built outcome. On refusal it returns the unchanged base, an empty diff and one `Fatal` message.
- `os_vcs::apply_mutation(snapshot, operation)` (`🌿️vcs/🦀️.rs:1807`): `operation.diff(snapshot)`, then `crate::os_spr::apply_diff`.
- `fold_plan_diff(kind, base)` (`📡️spr/🎮️command/🦀️.rs:833`) applies each local child step through `apply_diff` against the state as it stood before that step, and folds the child diffs with `absorb`. Foreign steps never contribute. A planning error or an Error-level message yields an empty diff, with the messages kept.
- `fold_plan_inverse(kind, base)` (line 871) does not apply anything. It collects each local child's `inverse` against that child's stored pre-state (`Planner` keeps the pre-states) and concatenates the results in forward step order. The store reverses the vector once.
- Shared diff kit (`📡️spr/🎮️command/🦀️.rs`):
  - `pub struct NamedTripleDiff<K, V, Patch> { pub removed: Vec<K>, pub modified: Vec<ItemPatch<K, Patch>>, pub added: Vec<V> }` (line 289). This is the keyed added/removed/modified shape the design asks for.
  - `pub fn named_apply(items, diff)` (line 310) validates before writing and is atomic.
  - `IndexedTripleDiff<V, Patch>` and `indexed_apply` (lines 362, 383).
  - The doc comment above `NamedTripleDiff` (about lines 283–288) says `absorb`, `inverse` and `between` stay handcrafted per artifact. The kit has no `absorb` or `inverse`.
  - `CollectionDiff`, `Identified`, `ItemPatch`, `Patchable` come from `os_vcs`.

### 3.4 Derives

- Proc-macro crate: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs` (glue, `#[path = "../../🦀️.rs"] mod component;`). Implementation: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` (1412 lines).
- Entry points in that crate: `#[proc_macro_derive(MutationLeaf, attributes(mutation_leaf))]`, `#[proc_macro_derive(Mutations, attributes(mutations))]`, `#[proc_macro_derive(CompositeMutation, attributes(composite))]`, `#[proc_macro_derive(DslArtifact, attributes(artifact))]`, plus the function-like `diff_text!` and `diff_binary!`. `DslRecord` (used by payloads and snapshots, e.g. `semio_framework_dsl_record_derive::DslRecord`) comes from a different crate, `semio_framework_dsl_record_derive`, which I did not read.
- `#[mutations(snapshot = …, diff = …, schema = "…" [, retire_cold = path])]` is required on the enum. A missing `snapshot`, `diff` or `schema` is a compile error.
- `#[derive(Mutations)]` emits, for the enum `Agg`:
  - `impl Mutation<S> for Agg { type Diff = D; … }`, forwarding `diff`, `inverse`, `timestamp`, `target`, `foreign_steps`, `input_schema` to each variant's `MutationKind`.
  - `impl SemanticMutation<S> for Agg`, `impl From<payload> for Agg` per variant, a `register_*` function (`SchemaId(format!("{}#{}", schema, kind))`), and `const` assertions that the kebab kind equals the variant, the verb is in `APPROVED_VERBS`, and the leaf `DESCRIPTOR.semantic_kind` equals the kind.
  - A `#[cfg(test)]` payload law test (`mutation_payload_law_test`). It checks: every leaf publishes a payload schema; every committed fixture decodes as the aggregate; every fixture case inverse fits the declared inverse rows; demo cases are labelled in every locale.
- `#[derive(MutationLeaf)]` reads the leaf's `🔣️.json` at compile time. It refuses a `semanticKind` that is not kebab-case with at least one hyphen, and it checks `payloadSchema` against the leaf directory. It embeds `PAYLOAD_SCHEMA` and `PAYLOAD_SCHEMA_DOCUMENTS` and derives `inverse_rows` from `x-semio-inverse-rows` in the payload schema (default: one row).
- Source authority: the derive accepts a leaf source either as `<leaf>/🦠️mutation/🦀️.rs` or as `<leaf>/🦀️.rs`. The descriptor `🔣️.json` must sit next to the owner directory. A flat leaf lives directly under the mutation collection (`🧬️mutations`). A domain-operation leaf lives under `🧬️mutations/<domain>/<verb>/` and must be registered in the projection. A `🧬️mutations` root can also be an aggregate of component roots (drawing `✳️any` aggregates the four split subsets).

### 3.5 Law helper

Location: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`, line 641. Kernel path: `protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law`. Gated by `#[cfg(any(test, feature = "protocol-laws"))]`. Plugins enable the feature as a dev-dependency (shooting `📦️packages/🦀️rust/Cargo.toml:60`).

```rust
pub async fn assert_mutation_inverse_sum_law<P, Op>(mutation: &Op, base: &P)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: crate::os_spr::Mutation<P>,
```

Differences from the design:
- The design writes a synchronous helper. The code is `pub async fn`, and its body contains no `.await`. Tests call it as `…assert_mutation_inverse_sum_law(&mutation(), &before()).await;`.
- Assertions, in order: (a) the forward outcome has no Error or Fatal message; (b) the inverse is non-empty when `after != base`; (c) sequential replay of `mutation.inverse(base)` (reversed) restores `base`; (d) `apply_diff(Σ, after) == base`, where Σ is the absorbed sum of the inverse step diffs; (e) `canon(Σ) == canon(forward.inverse(base))`, where `canon(x) = absorb(default, x)`.
- Note the step (e) comparison uses `DiffAlgebra::inverse` on the forward diff, so it requires that inverse to be concrete.

### 3.6 Gate rules that a new leaf must pass

`verify mutation-outcome-law` (root `📜️script.ts`, rules R8–R16 from `📓️exec-fw-gate.md`). They are textual, so the checks below are as strict as the gate:
- R8: no `&mut` in the signature or body of a leaf `fn diff` or `fn inverse`.
- R9: no `.apply(`, `::apply(`, `apply_diff(` or `ApplyCapability` in leaf `🔺️diff`, `↩️inverse`, `🦠️mutation` files, or in `impl … MutationKind<`/`Mutation<` blocks. `impl … MutationDiff<`/`DiffAlgebra<` blocks are exempt.
- R10: no `between(` call in leaf files outside diff-type impls.
- R11: no `diff(`, `.diff(` or `*_inverse(…diff…)` in an inverse body.
- R12: no `let mut x = <param>….clone()` in a diff body.
- R13: no `impl MutationDiff<X> for X`, and no `type Diff = Self/<P>` in leaf `Mutation` impls.
- R14: no `SetSnapshot`, `PatchSnapshot`, `ReplaceDocument`, `ReplaceSnapshot`, `Restore*`, `X::Snapshot` in an inverse body.
- R15: a leaf directory that owns `🔺️diff` needs at least one `🧪️tests/**/*.rs` in the leaf itself that mentions `assert_mutation_inverse_sum_law`. The shared `🧬️mutations/🧪️tests` does not count.
- R16: no `.apply_to(` and no `fn apply_to` inside `impl MutationOutcome<…>`.

The gate has known false positives (exec-fw-gate, "Known precision limits"). R9 flags any `.apply(` in scope, so a domain method named `apply` must be renamed. R16 flags any `.apply_to(`.

## 4. Exemplar

Candidate: shooting `✳️any` (`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any`). It is the only artifact I found with a sparse keyed diff, a concrete inverse per leaf, and an L3 test in the leaf itself. Counts: 32 leaf directories under `🧬️mutations`; 32 files in the plugin call `assert_mutation_inverse_sum_law`.

Shape of the artifact-level diff: `ShootingDiff` (Appendix D) has one `Option<…Delta>` per collection (`assets`, `savedCameras`, `shots`), plus scalar `Option`s and `scene`/`emblem`. Each collection delta is `ShootingListDelta<T, P> { edits: Vec<ShootingEdit<T>>, patched: Vec<ShootingPatchEntry<P>> }`. `ShootingEdit` is `Add{index,item} | Remove{id} | Move{id,index}`. `patched` is keyed by id and sorted. The shape is per-id and sparse.

Leaves (all in the leaf directories under `🧬️mutations`):

- `🏷️rename-shot`: diff is `ShootingDiff::shot_patches([(id, ShootingShotPatch { label: Some(new), ..Default::default() })])`. Inverse is `RenameShot { id, new_label: base_label }`, an absolute setter. Test `inverse_diffs_sum_to_the_negative_diff` calls the law helper.
- `📸️create-shot`: diff is `ShootingDiff::shot_edit(ShootingEdit::Add{…})`, fatal `mutation.duplicate-id` on a duplicate. Inverse is `DeleteShot { id }` (empty when the id already exists).
- `🚮️delete-shot`: diff is `ShootingDiff::shot_edit(ShootingEdit::Remove{id})`, error `mutation.target-missing` if absent. Inverse is `CreateShot { shot: base.shots[index].clone(), index: Some(index) }`, with the captured position, so the full record returns.
- `🪪️rename-saved-camera`: same pattern as rename-shot, on `savedCameras`.

Checked against the design laws:

| Law | Status for shooting exemplar | Evidence |
|---|---|---|
| L1 declarative diff | Met for the four leaves. Diff reads `base` and builds sparse rows. | leaf `🔺️diff/🦀️.rs` files, Appendix C |
| L2 concrete inverse | Met. Each inverse reads `base` and builds a concrete leaf. | leaf `↩️inverse/🦀️.rs` files |
| L3 sum law test | Present in the leaf's own `🧪️tests`. Not run here. | `inverse_diffs_sum_to_the_negative_diff` in rename-shot, create-shot, delete-shot tests |
| L4 central apply only | Met for the four leaves. Gate-pattern scan (R8–R16) found nothing in them. | grep of `.apply(`, `between(`, `&mut`, `.clone()` in diff, `apply_to` |
| L5 impossible by design | Met. `ApplyCapability` has a private field. `ShootingDiff::apply` takes the capability and ignores it. | `🧰️…/🎮️mutation/🦀️.rs:97` |

Gaps in the exemplar, relative to the design:
- `absorb` is sound but not canonical. `Add∘Remove` cancels (good). `Remove∘Add` of the same id stays as two edits, not a single replace. Patches merge per id. Patches on rows that a later edit removes are dropped. Edits run before patches in `write_into`, which the coalescing preserves.
- `DiffAlgebra::inverse` for `ShootingDiff` calls `negative_list`, a shared helper on the list delta. That is a diff-level inverse, and the design allows it. Leaves do not use it.
- `between` exists on the diff and the list delta (sync and import). Leaves do not call it.
- No exec report exists for shooting. Its status is "code and test bodies read", not "confirmed converted".
- The shooting diff schema drift (section 2) is open.

Drawing is a reference violation and should not be copied:
- Drawing tests call `apply_drawing_mutation(&mut snapshot, &mutation())`: 104 call sites in 31 files (for example Appendix B, the delete-layer test). That violates L4.
- `DrawingLayersDelta.reordered: Option<Vec<String>>` (`diff_reorder_layers`, `…/✳️any/🧬️schema/🔺️diff/🦀️.rs:863`) is a whole-list replace. The design's ruling forbids that (V1-GENERIC-DIFF).
- Drawing has 25 files that call the law helper, and its `DrawingDiff` implements `between` and `inverse` at the whole-snapshot level (`🔺️diff/🦀️.rs:770–781`). The gate counts 4140 breaches overall, with draw at 12.

NamedTripleDiff in brep (`🗄️stdio/🗿️artifacts/🧿️semio/…/🧊️brep`): 14 leaves use `NamedTripleDiff`, but none calls the law helper. The brep diff is a whole-snapshot type with `between` and `inverse`. It matches the added/removed/modified shape but is not a leaf-level exemplar.

## 5. Registration, commands, store (event sourcing)

### 5.1 How a leaf is registered (to add `model` kinds)

1. Leaf directory `🧬️mutations/<kind>/` with `🔣️.json` (descriptor). Required fields: `schemaVersion`, `owner` (relative path to the leaf), `semanticKind` (kebab, at least one hyphen), `displayName`, `emoji`, `aggregateVariant` (Pascal), `payloadSchema` (`🧬️schema/🔣️.json`), `textOpcode` (or `null`), `binaryTag`, `invertibility` (`self` | `explicit-mutation` | `plan` | `non-invertible`), `diffParticipation` (`detect` | `apply-only` | `plan` | `none`), `outcomeClasses` (`applied` | `no-op` | `empty` | `disjoint` | `rejected`), `composition` (`atomic` | `composite`), `requiredLanguageSurfaces` (`rust`, `typescript`, `graphql`, `protobuf`, `json-schema`, `text`, `binary`). Vocabulary is in `🧰️…/🎮️mutation/🦀️.rs:298–420`.
2. Payload struct in `🦠️mutation/🦀️.rs` (or `<kind>/🦀️.rs`). Derives `dsl::MutationLeaf` with `#[mutation_leaf(contract = ::protocol)]`, `#[dsl(keyword = "<kind>")]`, and `impl protocol::MutationKind<Snapshot, Aggregate>`.
3. `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs` with the signatures in section 2 and 3.
4. Payload JSON Schema `🧬️schema/🔣️.json` with `$id` `https://json.schemas.assets.semio-tech.com/s/<plugin>/<artifact>/mutation/<kind>/schema.json`, `mutation` as a `const`, and `x-semio-ui` for each property.
5. Enum variant in `🧬️mutations/🦀️.rs` (`#[derive(dsl::Mutations)]`) plus the `KINDS` slice (kebab list in declaration order). Shooting also has the aggregate `🧬️mutations/🔣️.json` `oneOf`.
6. Mount the leaf in the artifact root `🦀️.rs` mount tree (`pub mod <kind> { … }`), and in the subset mount if the leaf is in a split subset.
7. Taxonomy projection `🔣️mutation-authority.json`: flat leaves need no row. A domain-operation leaf needs a row under its root.
8. Oracle manifest `🔮️oracles/🔣️.json`: `mutationCatalogs` (kind list and vectors) and `mutationManifests` (`productionDispatch`, `oracleRequirements`).

### 5.2 How a command invokes a mutation

- Command handler (plugin editor): `handle(payload, doc: &ArtifactView<…>, cfg: &ConfigView<…>, ctx) -> Result<Emit<ArtifactMut, ConfigMut>, Fault>`. Example: shooting `✏️editor/🎮️commands/📦️asset/🦀️.rs` returns `Emit { artifact_mutations: vec![ShootingMutation::SetActiveAsset(…)], config_mutations: vec![…], ..Default::default() }` or `Emit::mutations(vec)`.
- The command never calls `diff` or `apply`. It emits payloads.

### 5.3 How the store folds mutations (CQRS and event sourcing)

- Write side: `ArtifactCommand<Mutation>` (`🏪️store/🦀️.rs:3565`) with variants `Apply { mutations, transaction }`, `Undo`, `Redo`, `UndoWithPolicy`, `ApplyInLane`, and lane-scoped undo. `ArtifactStore::dispatch(&mut self, command)` is at `🏪️store/🦀️.rs:20645`.
- Each applied operation is the event. Its diff is computed against the current projection (`diff(state)`), applied through `apply_diff`, and the projection advances (`projection.advance(next)`). Undo replays the stored inverses; the store reverses the stored inverse vector once. `inverse_rows` is the declared upper bound of inverse rows per operation.
- Read side: the projection (`DrawingSnapshot`, `ShootingSnapshot`) that the viewer and editor read.
- Text and binary codecs (`binaryTag`, `textOpcode`) serialize commands and operations. They are part of the leaf descriptor.

## 6. Fixtures and tests

### 6.1 Shape of a committed case

Per kind, per case, under the subset's `🧫️fixtures/🧬️mutations/<kind>/<case>/`:

- `📸️snapshot/⬅️before/🔣️.json`: the before-snapshot (full JSON document).
- `📸️snapshot/➡️after/🔣️.json`: the after-snapshot.
- `🦠️mutation/🔣️.json`: the payload (`{"mutation": "renameShot", "id": …, "newLabel": …}`).
- `🔺️diff/🔣️.json`: the expected sparse diff (see the shooting `rename-shot` diff in Appendix C: `shots.patched` with `patch` slots and `null` for untouched slots).
- `🎯️outcome/🔣️.json`: `{ "status": "applied" }` for an accepted case. A refusal carries `{ "status": "rejected", "code": "mutation.target-missing", "path": ["shape-missing"] }` (drawing `📋️duplicate-layer/🚫️rejects`).

Drawing structure keeps the same quintet. Its SVG fixtures sit beside it: `🧱️structure/🧫️fixtures/<case>/⬅️before.svg` and `➡️after.svg`. Shooting keeps the quintet under `✳️any/🧫️fixtures/🧬️mutations/<kind>/<case>/`.

### 6.2 How tests load them

- Rust leaf test (shooting `🏷️rename-shot/🧪️tests/🔤️relabels/🦀️.rs`): `const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/📸️snapshot/⬅️before/🔣️.json");`, decoded with `semio_framework_pack_json::from_json_str(…, JsonMemberPolicy::Reject)`. Tests use `#[semio_framework_async_macros::async_test] async fn …`. The standard set per leaf: `relabels_*` (after-snapshot), `inverse_*` (restore), `committed_json_is_canonical` (decode→encode fixed point), `declared_outcome_*` (status and diagnostic codes), `produces_committed_diff` (diff equals `🔺️diff/🔣️.json`), `committed_diff_applies_to_after`, `inverse_diffs_sum_to_the_negative_diff` (L3).
- Drawing leaf tests use the same includes, but call `apply_drawing_mutation(&mut …)` (see section 4).

### 6.3 Language-agnostic cases and third-party oracles

- Feature file: `🧱️structure/🧪️tests/🧱️mutate-drawing-1-structure/🥒️.feature` (Gherkin with tags such as `@capability-drawing-1-structure-mutate`, `@comparison-ordered-json-v1`, `@id-mutate`, `@mode-conformance`, `Scenario Outline` tables over kinds). Its Rust adapter is `…/🦀️.rs`, which imports `semio_repo_test_host::{Adapter, Context, Outcome, law}`.
- Oracle catalog: `🔮️oracles/🔣️.json` with `mutationCatalogs[]`, each `{ id, capability, standardDirectoryName, subsetDirectoryName, kinds[], vectors[{ mutationId, sourceMutationDirectoryName, mutationDirectoryName, scenarios[{ id, directoryName }] }] }`.
- `mutationManifests[]` (schema `semio.repository-test.mutation-manifest/v2`): per mutation `payloadSchema`, `outcomes`, `productionDispatch { operation, bridgeVersion, variant }`, `oracleRequirements[{ capability, qualifyingKind, oracle }]`, `carriers`, `invariants`.
- `testEvidence[]`: `{ id, class: "third-party-generated" | …, target, mutation, outcome, files[{ role, path, mediaType, sha256, bytes }], generator { oracle, packageVersion, engineFamily, engineVersion, command, platform } }`.
- Third-party oracle implementation: drawing `…/✳️any/🏭️generator/🧩️json` (json-rust 0.12). Built through `cargo build --release --offline`. Refresh digests with `bun 📜️script.ts manifests`. Gis has a Python oracle (`🐍️.py`) in `🗿️mutate-gismap-1`.
- The exec-fw-gate report notes that `bun test` with cwd at repo root segfaults in this environment. Run from an isolated cwd.

## 7. Risks and open questions for the BIM plugin

1. Layout choice: `🦠️mutation/` triad (drawing, gate R9/R11 names) versus flat `<kind>/🦀️.rs` (shooting, accepted by the derive). Pick the triad: the gate's leaf-file detection and the design use the triad names.
2. Diff shape: use `ShootingListDelta`-style keyed deltas (or `NamedTripleDiff` plus handwritten `absorb`/`inverse`). Do not use whole-list replaces (drawing `reordered`).
3. Law helper is async and feature-gated; the BIM plugin's dev-dependencies must enable `protocol-laws`.
4. Schema drift to check before generating BIM JSON Schemas: a top-level property that Rust does not have (shooting `artifact`).
5. Stale docs: `DiffAlgebra` LAWS paragraph shows `.await`. Code is sync.
6. No exec report exists for any leaf. Treat every "converted" claim as unverified until `cargo test -p <crate>` and the gate pass for that leaf.

## Appendices

Appendix A: directory trees (generated).
Appendix B: drawing `delete-layer` leaf, complete, with every file quoted.
Appendix C: shooting `rename-shot` leaf, complete, with every file quoted.
Appendix D: shooting artifact-level diff and algebra, verbatim line ranges.
Appendix E: framework contract, verbatim line ranges (replication and os command layer).

---

## Appendix A — Directory trees (generated from the repo)

### A.1 Drawing `🧱️structure` subset (all directories and files)

````text
🧱️structure/
🔮️oracles
  🔣️.json
🧪️tests
  🧱️mutate-drawing-1-structure
    🥒️.feature
    🦀️.rs
🧫️fixtures
  ➕️create-layer-adds-a-node
    ➡️after.svg
    ⬅️before.svg
  📋️duplicate-layer-inserts-a-copy
    ➡️after.svg
    ⬅️before.svg
  🔀️reorder-layer-swaps-two-nodes
    ➡️after.svg
    ⬅️before.svg
  🗑️delete-layer-removes-a-node
    ➡️after.svg
    ⬅️before.svg
  🧬️mutations
    ➕️create-layer
      ➕️appends
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    📋️duplicate-layer
      🚫️rejects
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🚫️.absent
        🦠️mutation
          🔣️.json
    🔃reorder-layer
      ⬆️moves
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🗑️delete-layer
      🚫️removes
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
🧬️schema
  🔣️.json
  🧬️mutations
    ➕️create-layer
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦠️mutation
        🦀️.rs
      🧪️tests
        ➕️appends
          🦀️.rs
      🧫️fixtures
        🎛️input-choices
          🔣️.json
      🧬️schema
        🔣️.json
    📋️duplicate-layer
      ↩️inverse
        🟦️.ts
        🦀️.rs
      🔣️.json
      🔺️diff
        🟦️.ts
        🦀️.rs
      🦠️mutation
        🟦️.ts
        🦀️.rs
      🧪️tests
        🚫️rejects
          🦀️.rs
      🧬️schema
        🔣️.json
    🔃reorder-layer
      ↩️inverse
        🟦️.ts
        🦀️.rs
      🔣️.json
      🔺️diff
        🟦️.ts
        🦀️.rs
      🦠️mutation
        🟦️.ts
        🦀️.rs
      🧪️tests
        ⬆️moves
          🦀️.rs
      🧬️schema
        🔣️.json
    🗑️delete-layer
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦠️mutation
        🦀️.rs
      🧪️tests
        🚫️removes
          🦀️.rs
      🧬️schema
        🔣️.json
````

### A.2 Shooting `✳️any` subset (all directories and files)

````text
✳️any/
✏️editor
  🌉️wasm
    🦀️.rs
    🧪️tests
      🔬️unit
        🦀️.rs
  🎚️config
    🔮️oracles
      🔣️.json
    🚪️io
      💾️binary
        🦀️.rs
        🧬️mutations
          🦀️.rs
      📝️text
        🦀️.rs
        🧬️mutations
          🦀️.rs
      🦀️.rs
    🦀️.rs
    🧪️tests
      🔬️contract-vectors
        🦀️.rs
      🔬️unit
        🦀️.rs
    🧫️fixtures
      ☑️set-shot
        ✅️set
          🎯️outcome
            🔣️.json
          📸️snapshot
            ➡️after
              🔣️.json
            ⬅️before
              🔣️.json
          🔺️diff
            🔣️.json
          🦠️mutation
            🔣️.json
      🎥️set-camera
        ✅️set
          🎯️outcome
            🔣️.json
          📸️snapshot
            ➡️after
              🔣️.json
            ⬅️before
              🔣️.json
          🔺️diff
            🔣️.json
          🦠️mutation
            🔣️.json
      🎯️set-center
        ✅️set
          🎯️outcome
            🔣️.json
          📸️snapshot
            ➡️after
              🔣️.json
            ⬅️before
              🔣️.json
          🔺️diff
            🔣️.json
          🦠️mutation
            🔣️.json
      📸️replace
        ✅️replace
          🎯️outcome
            🔣️.json
          📸️snapshot
            ➡️after
              🔣️.json
            ⬅️before
              🔣️.json
          🔺️diff
            🔣️.json
          🦠️mutation
            🔣️.json
      🔁️mutation-contracts.json
      🔢️set-fit
        ✅️set
          🎯️outcome
            🔣️.json
          📸️snapshot
            ➡️after
              🔣️.json
            ⬅️before
              🔣️.json
          🔺️diff
            🔣️.json
          🦠️mutation
            🔣️.json
      🔧️set-defaults
        ✅️set
          🎯️outcome
            🔣️.json
          📸️snapshot
            ➡️after
              🔣️.json
            ⬅️before
              🔣️.json
          🔺️diff
            🔣️.json
          🦠️mutation
            🔣️.json
    🧬️schema
      🔗️.graphql
      🔣️.json
      🔺️diff
        🔣️.json
        🦀️.rs
      🛰️.proto
      🟦️.ts
      🦀️.rs
      🧬️mutations
        ☑️set-shot-selection
          🔣️.json
          🦀️.rs
          🧬️schema
            🔣️.json
        🎥️set-camera
          🔣️.json
          🦀️.rs
          🧬️schema
            🔣️.json
        🎯️set-center-model
          🔣️.json
          🦀️.rs
          🧬️schema
            🔣️.json
        📸️replace-config
          🔣️.json
          🦀️.rs
          🧬️schema
            🔣️.json
        🔢️set-fit-revision
          🔣️.json
          🦀️.rs
          🧬️schema
            🔣️.json
        🔣️.json
        🔧️set-defaults
          🔣️.json
          🦀️.rs
          🧬️schema
            🔣️.json
        🦀️.rs
  🎭️modes
    ✏️edit
      🎚️config
        📌️.empty.md
      🎮️commands
        📌️.empty.md
      👥️presence
        📌️.empty.md
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
      🪟️windows
        🎥️scene
          ☑️options
            ☀️sun-enabled
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
            ✨️roughness
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
            🌑️shadow
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
            🌫️ambient
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
            🎯️center-model
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
            💡️sun-intensity
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
            📐️sun-elevation
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
            🧭️sun-azimuth
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
          🎚️config
            📌️.empty.md
          🎬️actions
            📌️.empty.md
          👥️presence
            📌️.empty.md
          🟦️.ts
          🦀️.rs
          🧪️tests
            🔬️unit
              🦀️.rs
          🪛️utilities
            📌️.empty.md
          🫧️transient
            📌️.empty.md
        🖼️icon
          ☑️options
            📷️shot
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
            🔷️shape
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
            🗂️format
              🟦️.ts
              🦀️.rs
              🧪️tests
                🔬️unit
                  🦀️.rs
          🎚️config
            📌️.empty.md
          🎬️actions
            📌️.empty.md
          👥️presence
            📌️.empty.md
          🟦️.ts
          🦀️.rs
          🧪️tests
            🔬️unit
              🦀️.rs
          🪛️utilities
            📌️.empty.md
          🫧️transient
            📌️.empty.md
      🫧️transient
        📌️.empty.md
  🎮️commands
    ☀️scene
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
    🎥️camera
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
    📄️document
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
    📦️asset
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
    📷️shot
      🦀️.rs
      🧪️tests
        🔬️field-value-contract
          🦀️.rs
        🔬️unit
          🦀️.rs
      🧫️fixtures
        🔢️field-values.json
    🖨️export
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
    🗂️selection
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
    🧭️gumball
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
  👥️presence
    🔮️oracles
      🔣️.json
    🚪️io
      💾️binary
        🦀️.rs
        🧬️mutations
          🦀️.rs
      📝️text
        🦀️.rs
        🧬️mutations
          🦀️.rs
      🦀️.rs
    🦀️.rs
    🧪️tests
      🔬️contract-vectors
        🦀️.rs
    🧫️fixtures
      📸️replace
        ✅️replace
          🎯️outcome
            🔣️.json
          📸️snapshot
            ➡️after
              🔣️.json
            ⬅️before
              🔣️.json
          🔺️diff
            🔣️.json
          🦠️mutation
            🔣️.json
      🔁️mutation-contracts.json
    🧬️schema
      🔗️.graphql
      🔣️.json
      🔺️diff
        🔣️.json
        🦀️.rs
      🛰️.proto
      🟦️.ts
      🦀️.rs
      🧬️mutations
        📸️replace-presence
          🔣️.json
          🦀️.rs
          🧬️schema
            🔣️.json
        🔣️.json
        🦀️.rs
  📌️panels
    🔍️inspection
      🦀️.rs
      🧪️tests
        🔬️semantic-contract
          🦀️.rs
        🔬️unit
          🦀️.rs
      🧫️fixtures
        🔣️panels.json
    🗿️artifact
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
    🛍️catalogue
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
  📚️examples
    🎬️demo-session
      🖼️assets
        🎮️.cmd.semio
      🟦️.ts
      🦀️.rs
      🧪️tests
        🧩️example
          🟦️.ts
          🦀️.rs
  🗣️terminology
    🦀️.rs
    🧪️tests
      🔬️unit
        🦀️.rs
  🟦️.ts
  🦀️.rs
  🧪️tests
    🔬️unit
      🦀️.rs
    🔬️window-action-contract
      🦀️.rs
  🧫️fixtures
    🔣️window-actions.json
    🧫️retained-command-limits
      🔣️.json
  🫧️transient
    📌️.empty.md
👁️viewer
  🎚️config
    📌️.empty.md
    🧬️schema
      🔗️.graphql
      🔣️.json
      🛰️.proto
      🟦️.ts
      🦀️.rs
  🎭️modes
    👁️view
      🎚️config
        📌️.empty.md
      🎮️commands
        📌️.empty.md
      👥️presence
        📌️.empty.md
      🦀️.rs
      🪟️windows
        🎥️scene
          🟦️.ts
          🦀️.rs
          🧪️tests
            🔬️unit
              🦀️.rs
      🫧️transient
        📌️.empty.md
  🎮️commands
    📌️.empty.md
  👥️presence
    📌️.empty.md
    🧬️schema
      🔗️.graphql
      🔣️.json
      🛰️.proto
      🟦️.ts
      🦀️.rs
  🟦️.ts
  🦀️.rs
  🧪️tests
    🔬️unit
      🦀️.rs
  🫧️transient
    📌️.empty.md
📚️examples
  🌲️hexagonal-cut-concrete-forest-left
    🖼️assets
      🌲️hexagonal-cut
        🗣️.dsl.semio
    🟦️.ts
    🦀️.rs
    🧪️tests
      🧩️example
        🟦️.ts
        🦀️.rs
  🎬️demo
    🟦️.ts
    🦀️.rs
    🧪️tests
      🧩️example
        🟦️.ts
        🦀️.rs
🔮️oracles
  🔣️.json
🖼️assets
  🎬️demo
    🗣️.dsl.semio
🚪️io
  💾️binary
    💡️inferences
      🌶️.spicy
      📡️.protocol.semio
      🔠️.abnf
      🟦️.ts
      🥋️.ksy
      🦀️.rs
    📸️snapshot
      🌶️.spicy
      📡️.protocol.semio
      🔠️.abnf
      🟦️.ts
      🥋️.ksy
      🦀️.rs
    🔺️diff
      🌶️.spicy
      📡️.protocol.semio
      🔠️.abnf
      🟦️.ts
      🥋️.ksy
      🦀️.rs
    🦀️.rs
    🧬️mutations
      🌶️.spicy
      📡️.protocol.semio
      🔠️.abnf
      🟦️.ts
      🥋️.ksy
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
  📝️text
    💡️inferences
      🅰️.g4
      📖️.grammar.semio
      🔗️.graphql
      🔣️.json
      🔤️.ebnf
      🛰️.proto
      🟦️.ts
      🦀️.rs
    📸️snapshot
      🅰️.g4
      📖️.grammar.semio
      🔗️.graphql
      🔣️.json
      🔤️.ebnf
      🛰️.proto
      🟦️.ts
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
    🔺️diff
      🅰️.g4
      📖️.grammar.semio
      🔗️.graphql
      🔣️.json
      🔤️.ebnf
      🛰️.proto
      🟦️.ts
      🦀️.rs
    🦀️.rs
    🧬️mutations
      🅰️.g4
      📖️.grammar.semio
      📖️mutations.grammar.semio
      🔗️.graphql
      🔣️.json
      🔤️.ebnf
      🛰️.proto
      🟦️.ts
      🦀️.rs
  📤️export
    🧵️serializers
      🗿️artifacts
        🔣️json
          🔖️rfc8259
            ✳️any
              🟦️.ts
              🦀️.rs
        🔤️txt
          🔖️utf-8
            ✳️any
              🟦️.ts
              🦀️.rs
  📥️import
    🧩️deserializers
      🗿️artifacts
        🔣️json
          🔖️rfc8259
            ✳️any
              🟦️.ts
              🦀️.rs
        🔤️txt
          🔖️utf-8
            ✳️any
              🟦️.ts
              🦀️.rs
  🟦️.ts
  🦀️.rs
  🪶️sqlite
    📸️snapshot
      🗄️.d.ts
      🗄️.sql
      🟦️.ts
      🦀️.rs
      🧪️tests
        🟦️.ts
        🦀️.rs
      🧫️fixtures
        🔣️.json
        🔬️independent
          🔣️.json
    🦀️.rs
🧪️tests
  🎚️mutate-shooting-shooting-1-any-editor-config
    🥒️.feature
    🦀️.rs
  🎥️mutate-shooting-1
    🐍️.py
    🥒️.feature
    🦀️.rs
  👥️mutate-shooting-shooting-1-any-editor-presence
    🥒️.feature
    🦀️.rs
🧫️fixtures
  🧬️mutations
    ↔️drag-assets
      🚚️offsets
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    ↕️scale-assets
      📏️doubles
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    ☀️change-scene-sun
      ☀️switches
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    ✂️change-shot-shape
      ⭕️rounds
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    ✏️rename-asset
      🏷️renames
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    ➕️create-asset
      ➕️appends
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🌅️change-scene-sun
      🌅️raises
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🌐️change-asset-url
      🌐️points
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🌑️change-scene-shadow
      🌑️switches
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🎞️replace-saved
      📍️repositions
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🎥️create-saved-camera
      🎥️appends
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🎯️set-active-shot
      🎯️activates
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🏷️rename-shot
      🔤️relabels
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    💡️change-scene-sun
      💡️dims
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    📌️set-active-asset
      📌️activates
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    📏️change-shot-width
      ↔️widens
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    📐️change-shot-height
      ↕️heightens
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    📷️replace-shot-camera
      📷️rewrites
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    📸️create-shot
      📸️appends
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🔀️reorder-assets
      🔀️moves
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🔁️reorder-saved-cameras
      🔁️moves
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🔃️reorder-shots
      ⬆️moves
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🔄️rotate-assets
      🔄️spins
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🔅️change-scene-ambient
      🔅️dims
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🖼️change-shot-format
      🎨️switches
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🗑️delete-asset
      🗑️removes
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🚮️delete-shot
      🚫️removes
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🧭️change-scene-sun
      🧭️turns
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🧹️delete-saved-camera
      🚫️removes
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🪨️change-scene
      ✨️polishes
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
    🪪️rename-saved-camera
      🔤️relabels
        🎯️outcome
          🔣️.json
        📸️snapshot
          ➡️after
            🔣️.json
          ⬅️before
            🔣️.json
        🔺️diff
          🔣️.json
        🦠️mutation
          🔣️.json
🧬️schema
  💡️inferences
    🔗️.graphql
    🔣️.json
    🛰️.proto
    🟦️.ts
    🦀️.rs
    🧪️tests
      🔬️unit
        🦀️.rs
    🧭topology
      🟦️.ts
      🦀️.rs
      🧪️tests
        🔬️unit
          🦀️.rs
  📸️snapshot
    🔗️.graphql
    🔣️.json
    🛰️.proto
    🟦️.ts
    🦀️.rs
  🔗️.graphql
  🔣️.json
  🔺️diff
    🔗️.graphql
    🔣️.json
    🛰️.proto
    🟦️.ts
    🦀️.rs
    🧪️tests
      🔬️unit
        🦀️.rs
  🛰️.proto
  🟦️.ts
  🦀️.rs
  🧪️tests
    🔬️unit
      🦀️.rs
  🧬️mutations
    ↔️drag-assets
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🚚️offsets
          🦀️.rs
      🧬️schema
        🔣️.json
    ↕️scale-assets
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        📏️doubles
          🦀️.rs
      🧬️schema
        🔣️.json
    ☀️change-scene-sun
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        ☀️switches
          🦀️.rs
      🧬️schema
        🔣️.json
    ✂️change-shot-shape
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        ⭕️rounds
          🦀️.rs
      🧬️schema
        🔣️.json
    ✏️rename-asset
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🏷️renames
          🦀️.rs
      🧬️schema
        🔣️.json
    ➕️create-asset
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        ➕️appends
          🦀️.rs
      🧬️schema
        🔣️.json
    🌅️change-scene-sun
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🌅️raises
          🦀️.rs
      🧬️schema
        🔣️.json
    🌐️change-asset-url
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🌐️points
          🦀️.rs
      🧬️schema
        🔣️.json
    🌑️change-scene-shadow
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🌑️switches
          🦀️.rs
      🧬️schema
        🔣️.json
    🎞️replace-saved
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        📍️repositions
          🦀️.rs
      🧬️schema
        🔣️.json
    🎥️create-saved-camera
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🎥️appends
          🦀️.rs
      🧬️schema
        🔣️.json
    🎯️set-active-shot
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🎯️activates
          🦀️.rs
      🧬️schema
        🔣️.json
    🏷️rename-shot
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🔤️relabels
          🦀️.rs
      🧬️schema
        🔣️.json
    💡️change-scene-sun
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        💡️dims
          🦀️.rs
      🧬️schema
        🔣️.json
    📌️set-active-asset
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        📌️activates
          🦀️.rs
      🧬️schema
        🔣️.json
    📏️change-shot-width
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        ↔️widens
          🦀️.rs
      🧬️schema
        🔣️.json
    📐️change-shot-height
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        ↕️heightens
          🦀️.rs
      🧬️schema
        🔣️.json
    📷️replace-shot-camera
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        📷️rewrites
          🦀️.rs
      🧬️schema
        🔣️.json
    📸️create-shot
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        📸️appends
          🦀️.rs
      🧬️schema
        🔣️.json
    🔀️reorder-assets
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🔀️moves
          🦀️.rs
      🧬️schema
        🔣️.json
    🔁️reorder-saved-cameras
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🔁️moves
          🦀️.rs
      🧬️schema
        🔣️.json
    🔃️reorder-shots
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        ⬆️moves
          🦀️.rs
      🧬️schema
        🔣️.json
    🔄️rotate-assets
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🔄️spins
          🦀️.rs
      🧬️schema
        🔣️.json
    🔅️change-scene-ambient
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🔅️dims
          🦀️.rs
      🧬️schema
        🔣️.json
    🔗️.graphql
    🔣️.json
    🖼️change-shot-format
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🎨️switches
          🦀️.rs
      🧬️schema
        🔣️.json
    🗑️delete-asset
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🗑️removes
          🦀️.rs
      🧬️schema
        🔣️.json
    🚮️delete-shot
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🚫️removes
          🦀️.rs
      🧬️schema
        🔣️.json
    🛰️.proto
    🟦️.ts
    🦀️.rs
    🧪️tests
      🔬️unit
        🦀️.rs
    🧭️change-scene-sun
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🧭️turns
          🦀️.rs
      🧬️schema
        🔣️.json
    🧹️delete-saved-camera
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🚫️removes
          🦀️.rs
      🧬️schema
        🔣️.json
    🪨️change-scene
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        ✨️polishes
          🦀️.rs
      🧬️schema
        🔣️.json
    🪪️rename-saved-camera
      ↩️inverse
        🦀️.rs
      🔣️.json
      🔺️diff
        🦀️.rs
      🦀️.rs
      🧪️tests
        🔤️relabels
          🦀️.rs
      🧬️schema
        🔣️.json
````

---

## Appendix B — Drawing `delete-layer` leaf, complete (subset `🧱️structure`)

Every file of the leaf, and every committed fixture of the case `🚫️removes`, is quoted below.

### B.1 Leaf files

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/↩️inverse/🦀️.rs`

````rust
//! ↩️ Inverse for `DeleteLayer` — reconstructs a `create-layer` at the exact captured (parent,
//! index) BASE location, carrying the full removed subtree (children included for a group).
//! Missing target ⇒ `Vec::new()`.
use crate::mutations::DrawingMutation;
use crate::schema::{find_drawing_layer, find_drawing_layer_location};
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteLayer, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    let (Some(layer), Some(location)) = (find_drawing_layer(base, &payload.layer_id), find_drawing_layer_location(base, &payload.layer_id)) else {
        return Ok(Vec::new());
    };
    Ok(vec![crate::mutations::create_layer(location.parent_id, Some(location.index), layer.clone())])
}
//#endregion 🔖️Inverse

````

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🔣️.json`

````json
{
  "schemaVersion": 1,
  "owner": "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer",
  "semanticKind": "delete-layer",
  "displayName": "Delete Layer",
  "emoji": "🗑️",
  "aggregateVariant": "DeleteLayer",
  "payloadSchema": "🧬️schema/🔣️.json",
  "textOpcode": "delete-layer",
  "binaryTag": 12,
  "invertibility": "explicit-mutation",
  "diffParticipation": "apply-only",
  "outcomeClasses": [
    "applied",
    "rejected"
  ],
  "composition": "atomic",
  "requiredLanguageSurfaces": [
    "rust",
    "text",
    "binary",
    "json-schema"
  ]
}

````

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🔺️diff/🦀️.rs`

````rust
//! 🔺️ Sparse diff builder for `DeleteLayer`.
use crate::diff::{diff_remove_layer, DrawingDiff};
use crate::schema::find_drawing_layer;
use crate::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteLayer, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    if find_drawing_layer(base, &payload.layer_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.to_string_owner()]);
    }
    protocol::MutationOutcome::new(diff_remove_layer(&payload.layer_id))
}
//#endregion 🔖️Diff

````

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🦠️mutation/🦀️.rs`

````rust
//! 🗑️ Drawing mutation — `DeleteLayer`: removes an id-keyed layer (captures its full subtree +
//! location for undo).
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 🗑️ `delete-layer` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-layer")]
pub struct DeleteLayer {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_layer(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>) -> DrawingMutation {
    DrawingMutation::DeleteLayer(DeleteLayer { layer_id })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for DeleteLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "layer", kind: "delete-layer", record: "DeletedLayer" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete layer \"{}\"", self.layer_id), &format!("Ebene \"{}\" löschen", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation

````

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/🚫️removes/🦀️.rs`

````rust
//! 🧪️ `delete-layer` fixture — `🚫️removes`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::mutations::{apply_drawing_mutation, inverse_drawing_mutation, DrawingMutation};
use crate::schema::find_drawing_layer;
use crate::DrawingSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/🎯️outcome/🔣️.json");

fn before() -> DrawingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> DrawingSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> DrawingMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("delete-layer applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-layer/removes-group-a-with-its-child: applied state differs from committed after-snapshot");
}

/// 🗑️ Deleting a GROUP takes its whole subtree with it — the nested `text-a` must become
/// unaddressable too, not be reparented to the root.
#[semio_framework_async_macros::async_test]
async fn deleting_a_group_takes_its_subtree() {
    let base = before();
    assert!(find_drawing_layer(&base, "text-a").is_some(), "removes-group-a-with-its-child's before-snapshot must nest text-a inside group-a");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("delete-layer applies");
    assert!(find_drawing_layer(&snapshot, "group-a").is_none(), "the addressed group must be gone");
    assert!(find_drawing_layer(&snapshot, "text-a").is_none(), "the group's child goes with it — a delete never reparents a subtree to the root");
    assert_eq!(snapshot.layers.len(), 1, "only the untouched sibling remains at the root");
    assert!(find_drawing_layer(&snapshot, "shape-a").is_some(), "the sibling layer survives the delete");
}

/// ↩️ The inverse is a `create-layer` carrying the FULL removed subtree back to its exact captured
/// (parent, index) address.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_whole_subtree_at_its_old_address() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_drawing_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "delete-layer undoes with exactly one create-layer, subtree included");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_drawing_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-layer/removes-group-a-with-its-child: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-layer/removes-group-a-with-its-child: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-layer/removes-group-a-with-its-child: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied, with a `removed`-only delta naming
/// the group alone — the child is implied by the tree, never listed separately.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-layer/removes-group-a-with-its-child declares an applied outcome");
    let produced = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "delete-layer/removes-group-a-with-its-child: group-a exists, so target-missing must not fire, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("delete-layer's diff pins a layers delta");
    assert_eq!(delta.removed, vec!["group-a".to_string()], "the delta names only the addressed group");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-layer is a pure removal");
}

/// 🔺️ The produced diff is EXACTLY the committed one: a `removed` list naming ONLY the group. The
/// nested `text-a` is implied by the tree and is deliberately absent from the diff — a delete that
/// enumerated its descendants would be describing the outcome instead of the change.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-layer/removes-group-a-with-its-child: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("delete-layer pins a layers delta");
    assert_eq!(delta.removed, vec!["group-a".to_string()], "only the addressed group is named");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "a delete is neither a move nor a patch");
    assert!(!DIFF.contains("text-a"), "the nested child must not be enumerated in the committed diff");
    assert!(!DIFF.contains("shape-a"), "the untouched sibling must not appear in the committed diff");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawingDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-layer/removes-group-a-with-its-child: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::DrawingDiff as protocol::MutationDiff<DrawingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-layer/removes-group-a-with-its-child: committed diff did not carry before to after");
}

/// ⚖️ The concrete inverse's diffs sum to exactly the negative of the forward diff, restoring the committed before-document.
#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    let mutation: DrawingMutation = serde_json::from_str(MUTATION).unwrap();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before()).await;
}

````

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🧬️schema/🔣️.json`

````json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/s/draw/drawing/1/structure/mutation/delete-layer/schema.json",
  "title": "DeleteLayer",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "mutation",
    "layerId"
  ],
  "properties": {
    "mutation": {
      "const": "deleteLayer"
    },
    "layerId": {
      "type": "string",
      "x-semio-ui": {
        "widget": "reference",
        "role": "target",
        "label": {
          "en": "Layer",
          "de": "Ebene"
        },
        "description": {
          "en": "The layer this mutation addresses.",
          "de": "Die Ebene, die diese Mutation adressiert."
        },
        "ref": {
          "kind": "layer",
          "domain": "strokes",
          "granularity": "stroke"
        },
        "group": "target",
        "order": 10
      }
    }
  }
}

````

### B.2 Committed fixtures

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/🎯️outcome/🔣️.json`

````json
{
  "status": "applied"
}

````

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/➡️after/🔣️.json`

````json
{
  "schema": "drawing.document",
  "id": "drawing-fixture",
  "title": "Fixture Base",
  "layers": [
    {
      "kind": "shape",
      "id": "shape-a",
      "name": "Alpha",
      "visible": true,
      "locked": false,
      "opacity": 1.0,
      "blendMode": "normal",
      "transform": {
        "x": 0.0,
        "y": 0.0,
        "scaleX": 1.0,
        "scaleY": 1.0,
        "rotation": 0.0,
        "shear": 0.0
      },
      "attributes": {
        "fillRule": "evenodd"
      },
      "shapeKind": "rect",
      "rect": {
        "x": 0.0,
        "y": 0.0,
        "width": 120.0,
        "height": 60.0
      }
    }
  ],
  "artboard": {
    "width": 640.0,
    "height": 480.0
  }
}

````

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json`

````json
{
  "schema": "drawing.document",
  "id": "drawing-fixture",
  "title": "Fixture Base",
  "layers": [
    {
      "kind": "shape",
      "id": "shape-a",
      "name": "Alpha",
      "visible": true,
      "locked": false,
      "opacity": 1.0,
      "blendMode": "normal",
      "transform": {
        "x": 0.0,
        "y": 0.0,
        "scaleX": 1.0,
        "scaleY": 1.0,
        "rotation": 0.0,
        "shear": 0.0
      },
      "attributes": {
        "fillRule": "evenodd"
      },
      "shapeKind": "rect",
      "rect": {
        "x": 0.0,
        "y": 0.0,
        "width": 120.0,
        "height": 60.0
      }
    },
    {
      "kind": "group",
      "id": "group-a",
      "name": "Caption Group",
      "visible": true,
      "locked": false,
      "opacity": 1.0,
      "blendMode": "normal",
      "transform": {
        "x": 0.0,
        "y": 0.0,
        "scaleX": 1.0,
        "scaleY": 1.0,
        "rotation": 0.0,
        "shear": 0.0
      },
      "attributes": {
        "fillRule": "evenodd"
      },
      "children": [
        {
          "kind": "text",
          "id": "text-a",
          "name": "Caption",
          "visible": true,
          "locked": false,
          "opacity": 1.0,
          "blendMode": "normal",
          "transform": {
            "x": 0.0,
            "y": 0.0,
            "scaleX": 1.0,
            "scaleY": 1.0,
            "rotation": 0.0,
            "shear": 0.0
          },
          "attributes": {
            "fillRule": "evenodd"
          },
          "x": 8.0,
          "y": 16.0,
          "content": "Alpha",
          "size": 12.0
        }
      ]
    }
  ],
  "artboard": {
    "width": 640.0,
    "height": 480.0
  }
}

````

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/🔺️diff/🔣️.json`

````json
{
  "artifact": null,
  "schema": null,
  "id": null,
  "title": null,
  "layers": {
    "added": [],
    "removed": [
      "group-a"
    ],
    "patched": [],
    "reordered": null
  },
  "assets": null,
  "artboard": null
}

````

#### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/🦠️mutation/🔣️.json`

````json
{
  "mutation": "deleteLayer",
  "layerId": "group-a"
}

````

---

## Appendix C — Shooting `rename-shot` leaf, complete (subset `✳️any`, exemplar of section 4)

### C.1 Leaf files

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/↩️inverse/🦀️.rs`

````rust
//! ↩ Inverse constructor for `RenameShot` — reconstructed from BASE state.

use super::RenameShot;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &RenameShot, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::RenameShot(RenameShot { id: payload.id.clone(), new_label: shot.label.clone() })],
        None => Vec::new(),
    }

    })())
}

````

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/🔣️.json`

````json
{
  "schemaVersion": 1,
  "owner": "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot",
  "semanticKind": "rename-shot",
  "displayName": "Rename Shot",
  "emoji": "🏷️",
  "aggregateVariant": "RenameShot",
  "payloadSchema": "🧬️schema/🔣️.json",
  "textOpcode": null,
  "binaryTag": 10,
  "invertibility": "explicit-mutation",
  "diffParticipation": "detect",
  "outcomeClasses": [
    "applied",
    "no-op",
    "rejected"
  ],
  "composition": "atomic",
  "requiredLanguageSurfaces": [
    "rust",
    "json-schema",
    "text",
    "binary"
  ]
}

````

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/🔺️diff/🦀️.rs`

````rust
//! 🔺 Diff constructor for `RenameShot`. Error `target-missing` when absent, Warning `no-op` when
//! already at that label.

use super::RenameShot;
use crate::diff::ShootingDiff;
use crate::ShootingShotPatch;
use crate::ShootingSnapshot;

pub fn diff(payload: &RenameShot, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.shots.iter().find(|shot| shot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Shot \"{}\" already has label \"{}\".", payload.id, payload.new_label));
    }
    protocol::MutationOutcome::new(ShootingDiff::shot_patches([(payload.id.clone(), ShootingShotPatch { label: Some(payload.new_label.clone()), ..Default::default() })]))
}

````

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/🦀️.rs`

````rust
//! 🏷️ Shooting mutation payload — `RenameShot`. Changes a shot's identity `label` field.

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RenameShot {
    pub id: String,
    pub new_label: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for RenameShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "shot", kind: "rename-shot", record: "RenamedShot" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename shot to \"{}\"", self.new_label), &format!("Aufnahme in \"{}\" umbenennen", self.new_label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

````

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/🧪️tests/🔤️relabels/🦀️.rs`

````rust
//! 🧪️ `rename-shot` fixture — `🔤️relabels`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::mutations::ShootingMutation;
use crate::{ShootingDiff, ShootingSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/🎯️outcome/🔣️.json");

fn before() -> ShootingSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> ShootingSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> ShootingMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn apply(base: &ShootingSnapshot, step: &ShootingMutation) -> ShootingSnapshot {
    protocol::apply_diff(&step.diff(base).into_parts().0, base).expect("rename-shot diff applies")
}

/// ▶️ `rename-shot` patches the shot's `label` — the human caption — and leaves the `id` that every
/// cursor and camera binding is keyed on alone.
#[semio_framework_async_macros::async_test]
async fn relabels_without_rekeying() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "rename-shot/relabels-shot-close-to-detail: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.shots[1].label, "Detail", "rename-shot/relabels-shot-close-to-detail: the new label must land on \"shot-close\"");
    assert_eq!(snapshot.shots[1].id, "shot-close", "rename-shot/relabels-shot-close-to-detail: a relabel never re-keys the shot");
    assert_eq!((snapshot.shots[1].width, snapshot.shots[1].height), (before().shots[1].width, before().shots[1].height), "rename-shot/relabels-shot-close-to-detail: the pixel dimensions are outside this patch");
    assert_eq!(snapshot.shots[0], before().shots[0], "rename-shot/relabels-shot-close-to-detail: the other shot is untouched");
}

/// ↩️ The inverse is a `rename-shot` back to the BASE label.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_label() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base).expect("valid retained mutation inverse fixture");
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "rename-shot/relabels-shot-close-to-detail: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-shot/relabels-shot-close-to-detail: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-shot/relabels-shot-close-to-detail: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the equality guard: relabelling to the label the
/// shot already carries is `mutation.no-op` at Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_relabelling_to_the_same_label_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-shot/relabels-shot-close-to-detail: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "rename-shot/relabels-shot-close-to-detail: a real relabel must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(semio_framework_diagnostic::Severity::Warning), "rename-shot/relabels-shot-close-to-detail: relabelling to the current label is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "rename-shot/relabels-shot-close-to-detail: the equality guard's frozen code");
    let unchanged = protocol::apply_diff(&again.into_parts().0, &expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "rename-shot/relabels-shot-close-to-detail: a no-op relabel applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the `ShootingShotPatch` has `label` filled and every sibling slot null, `background` and
/// `cameraId` included, so a relabel cannot disturb either.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-shot/relabels-shot-close-to-detail: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["shots"]["patched"][0]["patch"]["label"], "Detail", "rename-shot/relabels-shot-close-to-detail: `label` is the one filled patch slot");
    assert!(committed["shots"]["patched"][0]["patch"]["width"].is_null() && committed["shots"]["patched"][0]["patch"]["height"].is_null(), "rename-shot/relabels-shot-close-to-detail: the pixel-dimension slots stay null");
    assert!(committed["shots"]["patched"][0]["patch"]["cameraId"].is_null(), "rename-shot/relabels-shot-close-to-detail: the `cameraId` slot stays null, so a relabel cannot rebind a shot");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed rename-shot patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-shot/relabels-shot-close-to-detail: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — a one-slot patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-shot/relabels-shot-close-to-detail: committed diff did not carry before to after");
}

/// ⚖️ The inverse rows' diffs sum (`MutationDiff::absorb`) to the negative of this mutation's diff, and replaying them restores the before-snapshot.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

````

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-shot/🧬️schema/🔣️.json`

````json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/s/shooting/shooting/mutation/rename-shot/schema.json",
  "title": "RenameShot",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "mutation",
    "id",
    "newLabel"
  ],
  "properties": {
    "mutation": {
      "const": "renameShot"
    },
    "id": {
      "type": "string",
      "x-semio-ui": {
        "widget": "reference",
        "role": "target",
        "label": {
          "en": "Shot",
          "de": "Aufnahme"
        },
        "description": {
          "en": "The shot this mutation addresses.",
          "de": "Die Aufnahme, die diese Mutation adressiert."
        },
        "ref": {
          "kind": "shot"
        },
        "group": "target",
        "order": 10
      }
    },
    "newLabel": {
      "type": "string",
      "x-semio-ui": {
        "widget": "text",
        "role": "value",
        "label": {
          "en": "Name",
          "de": "Name"
        },
        "group": "identity",
        "order": 20
      }
    }
  }
}

````

### C.2 Committed fixtures

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/🎯️outcome/🔣️.json`

````json
{
  "status": "applied"
}

````

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/📸️snapshot/➡️after/🔣️.json`

````json
{
  "schema": "shooting.shooting",
  "assets": [
    {
      "id": "asset-hero",
      "name": "Hero",
      "url": "/mesh/hero.glb",
      "format": "glb",
      "origin": [
        1.0,
        2.0,
        3.0
      ],
      "orientation": [
        0.0,
        0.0,
        0.0,
        1.0
      ],
      "scale": [
        2.0,
        2.0,
        2.0
      ]
    },
    {
      "id": "asset-prop",
      "name": "Prop",
      "url": "/mesh/prop.glb",
      "format": "glb",
      "origin": [
        0.0,
        0.0,
        0.0
      ]
    }
  ],
  "savedCameras": [
    {
      "id": "cam-wide",
      "label": "Wide",
      "camera": {
        "position": [
          10.0,
          -10.0,
          6.0
        ],
        "target": [
          0.0,
          0.0,
          1.0
        ],
        "zoom": 1.0,
        "fov": 50.0
      }
    },
    {
      "id": "cam-close",
      "label": "Close",
      "camera": {
        "position": [
          2.0,
          -2.0,
          1.5
        ],
        "target": [
          0.0,
          0.0,
          1.0
        ],
        "zoom": 2.0,
        "fov": 35.0
      }
    }
  ],
  "scene": {
    "background": "#101014",
    "sun": {
      "enabled": true,
      "azimuth": 45.0,
      "elevation": 35.0,
      "intensity": 2.4,
      "color": "#ffffff"
    },
    "ambient": {
      "intensity": 1.15,
      "color": "#ffffff"
    },
    "shadow": {
      "enabled": true,
      "opacity": 0.35,
      "softness": 1.0
    },
    "material": {
      "color": "#9aa0ab",
      "metalness": 0.0,
      "roughness": 1.0,
      "emissive": "#000000",
      "emissiveIntensity": 0.0,
      "stroke": ""
    }
  },
  "shots": [
    {
      "id": "shot-wide",
      "label": "Wide",
      "width": 512,
      "height": 512,
      "format": "png",
      "shape": "rectangle",
      "background": "#ffffff",
      "cameraId": "cam-wide"
    },
    {
      "id": "shot-close",
      "label": "Detail",
      "width": 256,
      "height": 256,
      "format": "svg",
      "shape": "ellipse"
    }
  ],
  "activeShotId": "shot-wide",
  "activeAssetId": "asset-hero"
}

````

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/📸️snapshot/⬅️before/🔣️.json`

````json
{
  "schema": "shooting.shooting",
  "assets": [
    {
      "id": "asset-hero",
      "name": "Hero",
      "url": "/mesh/hero.glb",
      "format": "glb",
      "origin": [
        1.0,
        2.0,
        3.0
      ],
      "orientation": [
        0.0,
        0.0,
        0.0,
        1.0
      ],
      "scale": [
        2.0,
        2.0,
        2.0
      ]
    },
    {
      "id": "asset-prop",
      "name": "Prop",
      "url": "/mesh/prop.glb",
      "format": "glb",
      "origin": [
        0.0,
        0.0,
        0.0
      ]
    }
  ],
  "savedCameras": [
    {
      "id": "cam-wide",
      "label": "Wide",
      "camera": {
        "position": [
          10.0,
          -10.0,
          6.0
        ],
        "target": [
          0.0,
          0.0,
          1.0
        ],
        "zoom": 1.0,
        "fov": 50.0
      }
    },
    {
      "id": "cam-close",
      "label": "Close",
      "camera": {
        "position": [
          2.0,
          -2.0,
          1.5
        ],
        "target": [
          0.0,
          0.0,
          1.0
        ],
        "zoom": 2.0,
        "fov": 35.0
      }
    }
  ],
  "scene": {
    "background": "#101014",
    "sun": {
      "enabled": true,
      "azimuth": 45.0,
      "elevation": 35.0,
      "intensity": 2.4,
      "color": "#ffffff"
    },
    "ambient": {
      "intensity": 1.15,
      "color": "#ffffff"
    },
    "shadow": {
      "enabled": true,
      "opacity": 0.35,
      "softness": 1.0
    },
    "material": {
      "color": "#9aa0ab",
      "metalness": 0.0,
      "roughness": 1.0,
      "emissive": "#000000",
      "emissiveIntensity": 0.0,
      "stroke": ""
    }
  },
  "shots": [
    {
      "id": "shot-wide",
      "label": "Wide",
      "width": 512,
      "height": 512,
      "format": "png",
      "shape": "rectangle",
      "background": "#ffffff",
      "cameraId": "cam-wide"
    },
    {
      "id": "shot-close",
      "label": "Close",
      "width": 256,
      "height": 256,
      "format": "svg",
      "shape": "ellipse"
    }
  ],
  "activeShotId": "shot-wide",
  "activeAssetId": "asset-hero"
}

````

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/🔺️diff/🔣️.json`

````json
{
  "schema": null,
  "assets": null,
  "savedCameras": null,
  "scene": null,
  "shots": {
    "edits": [],
    "patched": [
      {
        "id": "shot-close",
        "patch": {
          "label": "Detail",
          "width": null,
          "height": null,
          "format": null,
          "shape": null,
          "background": null,
          "cameraId": null
        }
      }
    ]
  },
  "activeShotId": null,
  "activeAssetId": null,
  "emblem": null
}

````

#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-shot/🔤️relabels/🦠️mutation/🔣️.json`

````json
{
  "mutation": "renameShot",
  "id": "shot-close",
  "newLabel": "Detail"
}

````

---

## Appendix D — Shooting artifact-level diff type (whole file, verbatim)


#### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`

````rust
//! 🧬️ Shooting diff schema — sparse typed delta over the artifact: ordered structural edits plus keyed field patches per list,
//! a field patch for the scene, and assignable scalars. The central applier is the only caller of [`MutationDiff::apply`].

use crate::{ShootingAsset, ShootingAssetPatch, ShootingAssigned, ShootingEmblemChild, ShootingSavedCamera, ShootingSavedCameraPatch, ShootingScenePatch, ShootingShot, ShootingShotPatch, ShootingSnapshot};
use protocol::{ApplyCapability, DiffAlgebra, Identified, MutationApplyError, MutationApplyResult, MutationDiff, Patchable};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the shooting artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub assets: Option<ShootingAssetsDelta>,
    #[state(artifact)]
    pub saved_cameras: Option<ShootingSavedCamerasDelta>,
    #[state(artifact)]
    pub scene: Option<ShootingScenePatch>,
    #[state(artifact)]
    pub shots: Option<ShootingShotsDelta>,
    #[state(artifact)]
    pub active_shot_id: Option<String>,
    #[state(artifact)]
    pub active_asset_id: Option<String>,
    /// 🕸️ Composed `s.stdio.semio.image` child slot: the outer `Option` says the slot was assigned, the inner whether it is now present.
    #[state(artifact)]
    pub emblem: Option<ShootingAssigned<Option<ShootingEmblemChild>>>,
}
//#endregion 🔖️Diff

//#region 🔖️Builders
/// 🏗️ Sparse single-collection diff constructors: a leaf names its edit or its keyed patches and nothing else.
impl ShootingDiff {
    /// ➕️ A diff carrying one structural edit of `assets`.
    pub fn asset_edit(edit: ShootingEdit<ShootingAsset>) -> Self {
        Self { assets: Some(ShootingAssetsDelta { edits: vec![edit], patched: Vec::new() }), ..Default::default() }
    }

    /// 🩹 A diff carrying keyed `assets` patches, kept in id order.
    pub fn asset_patches(entries: impl IntoIterator<Item = (String, ShootingAssetPatch)>) -> Self {
        Self { assets: Some(ShootingAssetsDelta { edits: Vec::new(), patched: sorted_entries(entries) }), ..Default::default() }
    }

    /// ➕️ A diff carrying one structural edit of `shots`.
    pub fn shot_edit(edit: ShootingEdit<ShootingShot>) -> Self {
        Self { shots: Some(ShootingShotsDelta { edits: vec![edit], patched: Vec::new() }), ..Default::default() }
    }

    /// 🩹 A diff carrying keyed `shots` patches, kept in id order.
    pub fn shot_patches(entries: impl IntoIterator<Item = (String, ShootingShotPatch)>) -> Self {
        Self { shots: Some(ShootingShotsDelta { edits: Vec::new(), patched: sorted_entries(entries) }), ..Default::default() }
    }

    /// ➕️ A diff carrying one structural edit of `savedCameras`.
    pub fn camera_edit(edit: ShootingEdit<ShootingSavedCamera>) -> Self {
        Self { saved_cameras: Some(ShootingSavedCamerasDelta { edits: vec![edit], patched: Vec::new() }), ..Default::default() }
    }

    /// 🩹 A diff carrying keyed `savedCameras` patches, kept in id order.
    pub fn camera_patches(entries: impl IntoIterator<Item = (String, ShootingSavedCameraPatch)>) -> Self {
        Self { saved_cameras: Some(ShootingSavedCamerasDelta { edits: Vec::new(), patched: sorted_entries(entries) }), ..Default::default() }
    }
}

fn sorted_entries<P>(entries: impl IntoIterator<Item = (String, P)>) -> Vec<ShootingPatchEntry<P>> {
    let mut rows: Vec<ShootingPatchEntry<P>> = entries.into_iter().map(|(id, patch)| ShootingPatchEntry { id, patch }).collect();
    rows.sort_by(|a, b| a.id.cmp(&b.id));
    rows
}
//#endregion 🔖️Builders

//#region 🔖️ListDelta
/// 🧩 One ordered structural edit of an identified list; `index` is the destination position in the list as it stands when the edit runs.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "edit", rename_all = "camelCase")]
pub enum ShootingEdit<T> {
    #[value(rename = "add", rename_all = "camelCase")]
    Add { index: usize, item: T },
    #[value(rename = "remove", rename_all = "camelCase")]
    Remove { id: String },
    #[value(rename = "move", rename_all = "camelCase")]
    Move { id: String, index: usize },
}

/// 🩹 One keyed field patch.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ShootingPatchEntry<P> {
    pub id: String,
    pub patch: P,
}

/// 🧩 Identified-list delta: structural `edits` run in order, then each keyed `patched` entry (kept in id order, at most one per id) patches a surviving row.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingListDelta<T, P> {
    pub edits: Vec<ShootingEdit<T>>,
    pub patched: Vec<ShootingPatchEntry<P>>,
}

impl<T, P> Default for ShootingListDelta<T, P> {
    fn default() -> Self {
        Self { edits: Vec::new(), patched: Vec::new() }
    }
}

/// 🧩 Identified-collection delta for `assets`.
pub type ShootingAssetsDelta = ShootingListDelta<ShootingAsset, ShootingAssetPatch>;
/// 🧩 Identified-collection delta for `shots`.
pub type ShootingShotsDelta = ShootingListDelta<ShootingShot, ShootingShotPatch>;
/// 🧩 Identified-collection delta for `savedCameras`.
pub type ShootingSavedCamerasDelta = ShootingListDelta<ShootingSavedCamera, ShootingSavedCameraPatch>;

/// ➕️ Field-wise composition of two patches of the same row: the later value of a field wins.
pub trait ShootingPatchAlgebra: Sized {
    /// ➕️ Composes `later` over `self`.
    fn merge(&mut self, later: Self);
}

impl ShootingPatchAlgebra for ShootingAssetPatch {
    fn merge(&mut self, later: Self) {
        macro_rules! take {
            ($field:ident) => {
                if later.$field.is_some() {
                    self.$field = later.$field;
                }
            };
        }
        take!(name);
        take!(url);
        take!(format);
        take!(origin);
        take!(orientation);
        take!(scale);
    }
}

impl ShootingPatchAlgebra for ShootingShotPatch {
    fn merge(&mut self, later: Self) {
        macro_rules! take {
            ($field:ident) => {
                if later.$field.is_some() {
                    self.$field = later.$field;
                }
            };
        }
        take!(label);
        take!(width);
        take!(height);
        take!(format);
        take!(shape);
        take!(background);
        take!(camera_id);
    }
}

impl ShootingPatchAlgebra for ShootingSavedCameraPatch {
    fn merge(&mut self, later: Self) {
        macro_rules! take {
            ($field:ident) => {
                if later.$field.is_some() {
                    self.$field = later.$field;
                }
            };
        }
        take!(label);
        take!(camera);
    }
}

fn position_of<T: Identified<String>>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}

impl<T, P> ShootingListDelta<T, P>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + ShootingPatchAlgebra,
{
    /// 🧬️ Runs the edits then the patches over `items`; private so that only [`MutationDiff::apply`] (the central applier's entry) reaches it.
    fn write_into(&self, items: &[T]) -> MutationApplyResult<Vec<T>> {
        let mut next = items.to_vec();
        for (row, edit) in self.edits.iter().enumerate() {
            let at = |field: &str| ["edits".to_string(), row.to_string(), field.to_string()];
            match edit {
                ShootingEdit::Add { index, item } => {
                    if position_of(&next, item.id()).is_some() {
                        return Err(MutationApplyError::new("mutation.apply.duplicate-target", "added item identity already exists").at(at("item")));
                    }
                    if *index > next.len() {
                        return Err(MutationApplyError::new("mutation.apply.invalid-index", format!("insertion index {index} exceeds length {}", next.len())).at(at("index")));
                    }
                    next.insert(*index, item.clone());
                }
                ShootingEdit::Remove { id } => {
                    let position = position_of(&next, id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(at("id")))?;
                    next.remove(position);
                }
                ShootingEdit::Move { id, index } => {
                    let position = position_of(&next, id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "moved item does not exist").at(at("id")))?;
                    let item = next.remove(position);
                    next.insert((*index).min(next.len()), item);
                }
            }
        }
        let mut seen = std::collections::HashSet::new();
        for entry in &self.patched {
            if !seen.insert(entry.id.as_str()) {
                return Err(MutationApplyError::new("mutation.apply.duplicate-target", "item is patched more than once").at(["patched", entry.id.as_str()]));
            }
            let position = position_of(&next, &entry.id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "patched item does not exist").at(["patched", entry.id.as_str()]))?;
            next[position].apply_patch(&entry.patch);
        }
        Ok(next)
    }

    /// ➕️ Sequentially composes `later` after `self`: adjacent edits of one row coalesce (add∘remove cancels, move∘move keeps the last, move∘remove keeps the remove) and keyed patches merge per id.
    pub fn absorb(&mut self, later: Self) {
        let removed: Vec<String> = later.edits.iter().filter_map(|edit| if let ShootingEdit::Remove { id } = edit { Some(id.clone()) } else { None }).collect();
        self.patched.retain(|entry| !removed.contains(&entry.id));
        for edit in later.edits {
            match (self.edits.last(), &edit) {
                (Some(ShootingEdit::Add { item, .. }), ShootingEdit::Remove { id }) if item.id() == id => {
                    self.edits.pop();
                    self.patched.retain(|entry| &entry.id != id);
                }
                (Some(ShootingEdit::Move { id: prior, .. }), ShootingEdit::Move { id, .. } | ShootingEdit::Remove { id }) if prior == id => {
                    self.edits.pop();
                    self.edits.push(edit);
                }
                _ => self.edits.push(edit),
            }
        }
        for entry in later.patched {
            match self.patched.iter_mut().find(|prior| prior.id == entry.id) {
                Some(prior) => prior.patch.merge(entry.patch),
                None => self.patched.push(entry),
            }
        }
        self.patched.sort_by(|a, b| a.id.cmp(&b.id));
    }
}

impl<T, P> ShootingListDelta<T, P>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + PartialEq,
{
    /// 🔁️ The negative delta against `base`: reversed structural edits that put every row back where it was, then patches restoring the pre-patch field values.
    pub fn negative(&self, base: &[T]) -> Self {
        let mut current = base.to_vec();
        let mut added: Vec<String> = Vec::new();
        let mut undo = Vec::new();
        for edit in &self.edits {
            match edit {
                ShootingEdit::Add { index, item } => {
                    undo.push(ShootingEdit::Remove { id: item.id().clone() });
                    added.push(item.id().clone());
                    current.insert((*index).min(current.len()), item.clone());
                }
                ShootingEdit::Remove { id } => {
                    if let Some(position) = position_of(&current, id) {
                        undo.push(ShootingEdit::Add { index: position, item: current.remove(position) });
                    }
                }
                ShootingEdit::Move { id, index } => {
                    if let Some(position) = position_of(&current, id) {
                        undo.push(ShootingEdit::Move { id: id.clone(), index: position });
                        let item = current.remove(position);
                        current.insert((*index).min(current.len()), item);
                    }
                }
            }
        }
        undo.reverse();
        let mut patched: Vec<ShootingPatchEntry<P>> = self
            .patched
            .iter()
            .filter(|entry| !added.contains(&entry.id))
            .filter_map(|entry| {
                let before = current.iter().find(|item| item.id() == &entry.id)?;
                let mut after = before.clone();
                after.apply_patch(&entry.patch);
                after.diff_patch(before).map(|patch| ShootingPatchEntry { id: entry.id.clone(), patch })
            })
            .collect();
        patched.sort_by(|a, b| a.id.cmp(&b.id));
        Self { edits: undo, patched }
    }

    /// 🧭️ The delta from `base` to `other` for sync and import: removals, then the inserts and moves that reach `other`'s order, then keyed patches.
    pub fn between(base: &[T], other: &[T]) -> Self {
        let mut edits = Vec::new();
        for item in base {
            if position_of(other, item.id()).is_none() {
                edits.push(ShootingEdit::Remove { id: item.id().clone() });
            }
        }
        let mut order: Vec<String> = base.iter().filter(|item| position_of(other, item.id()).is_some()).map(|item| item.id().clone()).collect();
        for (index, target) in other.iter().enumerate() {
            if order.get(index) == Some(target.id()) {
                continue;
            }
            match order.iter().position(|id| id == target.id()) {
                Some(position) => {
                    let id = order.remove(position);
                    order.insert(index, id.clone());
                    edits.push(ShootingEdit::Move { id, index });
                }
                None => {
                    order.insert(index, target.id().clone());
                    edits.push(ShootingEdit::Add { index, item: target.clone() });
                }
            }
        }
        let mut patched: Vec<ShootingPatchEntry<P>> = base
            .iter()
            .filter_map(|item| {
                let counterpart = other.iter().find(|candidate| candidate.id() == item.id())?;
                item.diff_patch(counterpart).map(|patch| ShootingPatchEntry { id: item.id().clone(), patch })
            })
            .collect();
        patched.sort_by(|a, b| a.id.cmp(&b.id));
        Self { edits, patched }
    }

    /// 🕳️ Whether the delta names no edit and no patch.
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty() && self.patched.is_empty()
    }
}

/// ➕️ Composes an optional list delta.
fn absorb_list<T, P>(target: &mut Option<ShootingListDelta<T, P>>, later: Option<ShootingListDelta<T, P>>)
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + ShootingPatchAlgebra,
{
    match (target.as_mut(), later) {
        (Some(prior), Some(later)) => prior.absorb(later),
        (None, Some(later)) => *target = Some(later),
        _ => {}
    }
}

/// 🔁️ Negates an optional list delta against its base list.
fn negative_list<T, P>(delta: &Option<ShootingListDelta<T, P>>, base: &[T]) -> Option<ShootingListDelta<T, P>>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + PartialEq,
{
    delta.as_ref().map(|delta| delta.negative(base))
}

/// 🧭️ The optional list delta between two lists, absent when they are equal.
fn between_list<T, P>(base: &[T], other: &[T]) -> Option<ShootingListDelta<T, P>>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + PartialEq,
{
    let delta = ShootingListDelta::between(base, other);
    (!delta.is_empty()).then_some(delta)
}
//#endregion 🔖️ListDelta

//#region 🔖️SceneAlgebra
macro_rules! scene_patch_algebra {
    ($($field:ident => $($path:ident).+),+ $(,)?) => {
        impl ShootingScenePatch {
            fn write_into(&self, scene: &mut crate::ShootingSceneLighting) {
                $(if let Some(value) = &self.$field {
                    scene.$($path).+ = value.clone();
                })+
            }

            /// 🔁️ The patch that restores `base` for exactly the fields this patch names.
            pub fn restoring(&self, base: &crate::ShootingSceneLighting) -> Self {
                Self { $($field: self.$field.as_ref().map(|_| base.$($path).+.clone()),)+ }
            }

            /// 🧭️ The patch from `from` to `to`, naming only differing fields.
            pub fn between(from: &crate::ShootingSceneLighting, to: &crate::ShootingSceneLighting) -> Self {
                Self { $($field: (from.$($path).+ != to.$($path).+).then(|| to.$($path).+.clone()),)+ }
            }

            /// ➕️ Composes `later` over this patch: the later value of a field wins.
            pub fn merge(&mut self, later: Self) {
                $(if later.$field.is_some() {
                    self.$field = later.$field;
                })+
            }

            /// 🕳️ Whether the patch names no field.
            pub fn is_empty(&self) -> bool {
                $(self.$field.is_none())&&+
            }
        }
    };
}

scene_patch_algebra! {
    background => background,
    sun_enabled => sun.enabled,
    sun_azimuth => sun.azimuth,
    sun_elevation => sun.elevation,
    sun_intensity => sun.intensity,
    sun_color => sun.color,
    ambient_intensity => ambient.intensity,
    ambient_color => ambient.color,
    shadow_enabled => shadow.enabled,
    shadow_opacity => shadow.opacity,
    shadow_softness => shadow.softness,
    material_color => material.color,
    material_metalness => material.metalness,
    material_roughness => material.roughness,
    material_emissive => material.emissive,
    material_emissive_intensity => material.emissive_intensity,
    material_stroke => material.stroke,
}
//#endregion 🔖️SceneAlgebra

//#region 🔖️Apply
impl MutationDiff<ShootingSnapshot> for ShootingDiff {
    fn apply(&self, base: &ShootingSnapshot, _capability: ApplyCapability) -> MutationApplyResult<ShootingSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.assets {
            next.assets = delta.write_into(&base.assets).map_err(|error| error.under(["assets"]))?;
        }
        if let Some(delta) = &self.saved_cameras {
            next.saved_cameras = delta.write_into(&base.saved_cameras).map_err(|error| error.under(["savedCameras"]))?;
        }
        if let Some(patch) = &self.scene {
            patch.write_into(&mut next.scene);
        }
        if let Some(delta) = &self.shots {
            next.shots = delta.write_into(&base.shots).map_err(|error| error.under(["shots"]))?;
        }
        if let Some(id) = &self.active_shot_id {
            next.active_shot_id = id.clone();
        }
        if let Some(id) = &self.active_asset_id {
            next.active_asset_id = id.clone();
        }
        if let Some(emblem) = &self.emblem {
            next.emblem = emblem.value.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(active_shot_id);
        take!(active_asset_id);
        take!(emblem);
        absorb_list(&mut self.assets, other.assets);
        absorb_list(&mut self.saved_cameras, other.saved_cameras);
        absorb_list(&mut self.shots, other.shots);
        match (self.scene.as_mut(), other.scene) {
            (Some(prior), Some(later)) => prior.merge(later),
            (None, Some(later)) => self.scene = Some(later),
            _ => {}
        }
    }
}

impl DiffAlgebra<ShootingSnapshot> for ShootingDiff {
    fn inverse(&self, base: &ShootingSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            assets: negative_list(&self.assets, &base.assets),
            saved_cameras: negative_list(&self.saved_cameras, &base.saved_cameras),
            scene: self.scene.as_ref().map(|patch| patch.restoring(&base.scene)),
            shots: negative_list(&self.shots, &base.shots),
            active_shot_id: self.active_shot_id.as_ref().map(|_| base.active_shot_id.clone()),
            active_asset_id: self.active_asset_id.as_ref().map(|_| base.active_asset_id.clone()),
            emblem: self.emblem.as_ref().map(|_| ShootingAssigned::new(base.emblem.clone())),
        }
    }

    fn between(base: &ShootingSnapshot, other: &ShootingSnapshot) -> Self {
        let scene = ShootingScenePatch::between(&base.scene, &other.scene);
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            assets: between_list(&base.assets, &other.assets),
            saved_cameras: between_list(&base.saved_cameras, &other.saved_cameras),
            scene: (!scene.is_empty()).then_some(scene),
            shots: between_list(&base.shots, &other.shots),
            active_shot_id: (base.active_shot_id != other.active_shot_id).then(|| other.active_shot_id.clone()),
            active_asset_id: (base.active_asset_id != other.active_asset_id).then(|| other.active_asset_id.clone()),
            emblem: (base.emblem != other.emblem).then(|| ShootingAssigned::new(other.emblem.clone())),
        }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

````

---

## Appendix E — Framework contract (verbatim line ranges, line numbers from the source)

### E.1 Applier, capability, MutationDiff, DiffAlgebra, Mutation (start)

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs lines 92-205`

````rust
pub type MutationApplyResult<P> = Result<P, MutationApplyError>;

/// 🔑️ Proof that the caller is the central applier. The private field makes this module the only place that can construct it,
/// and [`apply_diff`] is the only function here that does.
#[derive(Clone, Copy, Debug)]
pub struct ApplyCapability {
    _sealed: (),
}

/// 🎯️ THE central applier: the only mint point of [`ApplyCapability`] and the one callable entry that turns a diff into a
/// snapshot. Store lanes, replay, merge, backbone, plugin transaction folds, tool folds, the db and composite planners all
/// route through it; mutation leaves never call [`MutationDiff::apply`].
pub fn apply_diff<P, D: MutationDiff<P>>(diff: &D, base: &P) -> MutationApplyResult<P> {
    diff.apply(base, ApplyCapability { _sealed: () })
}

/// 📦️ Centralized snapshot mutation — one fallible `apply` per technology. A
/// malformed or base-incompatible persisted diff must return [`MutationApplyError`]; it must never
/// clamp an index, ignore a missing target, or return the unchanged base as implicit success.
///
/// Bound on [`crate::value::ToValue`]/[`crate::value::FromValue`], not `serde::Serialize`/
/// `serde::de::DeserializeOwned` — every plugin technology implementing this trait used to be
/// forced onto `serde` by this supertrait alone; see
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️serde-replacement-surface.md`.
pub trait MutationDiff<P>: Clone + Default + PartialEq + crate::value::ToValue + crate::value::FromValue + DiffAlgebra<P> {
    /// 🔑️ Turns the diff into the next snapshot. Callable only with an [`ApplyCapability`], which only [`apply_diff`] mints,
    /// so leaves cannot apply diffs; a composite diff forwards the capability it received to its sub-diffs.
    fn apply(&self, base: &P, capability: ApplyCapability) -> MutationApplyResult<P>;
    /// ➕️ Composes `self` (base→mid) with `other` (mid→after) into base→after, in place.
    /// Normative absorb contract (`.claude/plans/the-current-schemas-are-scalable-journal.md`
    /// `## Absorb`): **structural** (operates on the diff's own key/index/field shape, never on
    /// applied snapshot values), **total** (defined for every pair of diffs over the same
    /// artifact, including out-of-range/no-op cases — never panics), **base-free** (no snapshot
    /// parameter; the two diffs alone determine the result), and **sequential-coalesce only**
    /// (this composes two diffs known to have been applied in sequence by the same actor;
    /// concurrent-edit merging is an authority's `MergePolicy`/`📡️spr/⚔️conflict` job, never this
    /// method's — the CRDT-era concurrent-diff merge helper this docstring used to point at is
    /// deleted, see `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`).
    /// LAW: whenever sequential application succeeds,
    /// `apply_diff(&absorb(d1, d2), base) == apply_diff(&d1, base).and_then(|mid| apply_diff(&d2, &mid))`, associative
    /// over further absorbs of the same artifact's diff vocabulary. A rejection remains a
    /// rejection; absorb must not manufacture an implicit success path.
    fn absorb(&mut self, other: Self);

    /// 🧊️ Explicit cold disposal of a diff nobody will apply again. The default IS a plain drop,
    /// which is correct for every diff built out of plain values; a technology whose delta owns a
    /// fail-closed root (an `OrderedMap`, a neural `Dictionary`) MUST override it, because such a
    /// root aborts the process on a bare drop (`🌱️value/🗂️ordered/🦀️.rs`'s `Drop`). Every generic
    /// replay/fold seam that builds a delta and throws it away — `os_vcs::apply_mutation` and the
    /// store's history folds — routes through this instead of dropping
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn retire_cold(self)
    where
        Self: Sized,
    {
    }

    /// 🧊️ Explicit cold disposal of a SCRATCH projection the replay arithmetic built and displaced.
    /// A history fold walks `base → mid₁ → mid₂ → … → head`, and every intermediate is an owned `P`
    /// the next step's assignment throws away; when `P` owns a fail-closed root that bare drop aborts
    /// the process, so `undo`/`redo` and every `.pack`/`.spr` reload aborted the app for any artifact
    /// whose projection carries an `OrderedMap`. It hangs off the DIFF, not off the projection, because
    /// `P` itself carries no bound at those seams while `Self::Diff` always does
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn retire_projection(projection: P) {
        drop(projection);
    }
}

/// 🧮️ Diff-level algebra for a technology's [`MutationDiff`] type: inverse, state-delta
/// construction, and emptiness. Deliberately a SEPARATE trait from `MutationDiff` (not new
/// methods added to it) — `MutationDiff` already has 51+ repo-wide implementors, so a breaking
/// method addition there would break all of them at once. Follows this crate's own `DiffCodec`
/// precedent below: land the trait standalone in a spine wave, adopt it per-type in later waves
/// via a seeded shrink-only policy allowlist (`POLICY_DIFF_ALGEBRA`), never as a hard bound on
/// `MutationDiff` itself until every implementor is covered.
/// LAWS (for valid diffs): `d.inverse(base).await.apply(&d.apply(base).await?).await == Ok(*base)`;
/// `Self::between(a, b).await.apply(a).await == Ok(*b)`; `Self::between(a, a).await.is_empty().await`.
pub trait DiffAlgebra<P>: Sized {
    /// 🔁️ Diff-level undo: the diff that, applied after `self`, restores `base`.
    fn inverse(&self, base: &P) -> Self;
    /// 🧭️ State delta: the diff that, applied to `base`, yields `other`.
    fn between(base: &P, other: &P) -> Self;
    /// 🕳️ Whether this diff changes nothing relative to whatever base it was built against.
    fn is_empty(&self) -> bool;
}

/// 🔁️ Stored operation: emits a [`MutationOutcome`] (diff plus messages) and computes inverse
/// from pre-state. Moved from `os_store::Mutation` verbatim except: `mutation_id`/
/// `dependencies`/`author_id` now return the `protocol_core` id newtypes (were bare `String`) and
/// `base_version` now returns `Option<crate::ids::ArtifactVersion>` (was a bare `u64`
/// defaulting to `0`, which conflated "no base" with "based on version 0" — `None` fixes that);
/// `state_class` is a new defaulted method so every existing `impl` recompiles unchanged.
/// `validate` and its CRDT-era merge/reconcile hooks are GONE (ticket
/// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C4): every rejection or
/// merge-policy concern a technology used to express through those hooks now travels as a
/// [`MutationMessage`] on `diff`'s own [`MutationOutcome`].
pub trait Mutation<P>: Clone + crate::value::ToValue + crate::value::FromValue {
    type Diff: MutationDiff<P>;
    /// 🧷️ Every direct mutation leaf's static metadata, in aggregate-variant order.
    const DESCRIPTORS: &'static [MutationLeafDescriptor];

    /// 🧷️ This value's own direct mutation leaf descriptor.
    fn descriptor(&self) -> &'static MutationLeafDescriptor;
    fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;
    fn inverse(&self, base: &P) -> Result<Vec<Self>, semio_framework_value::ValueError> ;

    /// 🧊️ Explicit cold disposal of one owned operation, for the same reason [`MutationDiff::retire_cold`]
    /// exists: a row that carries a domain value with a fail-closed root cannot be dropped.
    fn retire_cold(self)
    where
        Self: Sized,
    {
````

### E.2 MutationLeaf

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs lines 985-1000`

````rust

/// 🪪️ Ownership contract for a direct mutation leaf, and the editable payload of one of its operations.
pub trait MutationLeaf {
    const DESCRIPTOR: MutationLeafDescriptor;
    const PROVENANCE: MutationSourceProvenance;
    /// 🧬️ The leaf's normative payload JSON Schema text (`DESCRIPTOR.payload_schema`, embedded at compile time by
    /// `#[derive(dsl::MutationLeaf)]`) — the source of its input descriptors (`manifest::mutation_input_defs`).
    const PAYLOAD_SCHEMA: &'static str;
    /// 🔗️ Every schema document [`Self::PAYLOAD_SCHEMA`] references by `$id`, transitively, that the leaf's own plugin (or
    /// framework module) tree holds — embedded by `#[derive(dsl::MutationLeaf)]` and published at runtime, so every input resolves
    /// whatever facet or artifact of the plugin it lives in (design §16.3).
    const PAYLOAD_SCHEMA_DOCUMENTS: &'static [&'static str] = &[];
    /// 🧬️ [`Self::PAYLOAD_SCHEMA`] when this operation is user-editable, else `None`. A leaf whose type wraps its payload
    /// in one variant of an enum (`#[mutation_leaf(payload = Apply)]`) is editable only in that variant; its other
    /// variants (an internal inverse such as `Restore`) are not.
    fn input_schema(&self) -> Option<&'static str> {
````

### E.3 MutationOutcome (struct)

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs lines 1252-1262`

````rust
/// [`MutationOutcome::absorb_messages`].
#[derive(Clone, Debug, PartialEq)]
pub struct MutationOutcome<D> {
    diff: D,
    messages: Vec<MutationMessage>,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin above
/// documents. Mirrors `#[serde(rename_all = "camelCase", default, skip_serializing_if =
/// "Vec::is_empty")]` byte-for-byte.
impl<D: crate::value::ToValue> crate::value::ToValue for MutationOutcome<D> {
````

### E.4 MutationOutcome (constructors, accessors)

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs lines 1312-1330`

````rust
impl<D> MutationOutcome<D> {
    /// ✅️ A successful diff, no messages.
    pub fn new(diff: D) -> Self {
        Self { diff, messages: Vec::new() }
    }

    pub fn diff(&self) -> &D {
        &self.diff
    }

    pub fn messages(&self) -> &[MutationMessage] {
        &self.messages
    }

    /// ➡️ Consumes `self` into its raw `(diff, messages)` parts.
    pub fn into_parts(self) -> (D, Vec<MutationMessage>) {
        (self.diff, self.messages)
    }

````

### E.5 MutationKind and SemanticDescriptor (command layer)

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs lines 196-240`

````rust
/// naming system in Rust code, past tense lives here and nowhere else.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue)]
pub struct SemanticDescriptor {
    pub verb: &'static str,
    pub entity: &'static str,
    pub kind: &'static str,
    pub record: &'static str,
}

//#region 🪪️MutationLeafDescriptor
/// 🪞️ Reexports the lower mutation metadata contract through the public OS command façade.
pub use protocol::mutation::{
    validate_mutation_leaf_descriptor, validate_mutation_leaf_descriptor_roster, validate_mutation_leaf_descriptor_roster_uniqueness, validate_mutation_leaf_source, MutationComposition, MutationDiffParticipation, MutationDomainOperation,
    MutationInvertibility, MutationLanguageSurface, MutationLeaf, MutationLeafDescriptor, MutationLeafDescriptorRosterValidationError, MutationLeafDescriptorValidationError, MutationLeafSourceScope, MutationLeafSourceValidationError,
    MutationOutcomeClass, MutationOwnerLayout, MutationSourceProvenance, ValidatedMutationLeafSourceScope,
};
//#endregion 🪪️MutationLeafDescriptor

/// 🦠️ One direct mutation leaf with mandatory source-derived metadata and handcrafted behavior.
/// `Op` wraps the owner's concrete leaves; inverses may select a different leaf from that roster.
/// 🌱️ Bound on [`protocol::value::ToValue`]/[`protocol::value::FromValue`], not serde's — the same
/// move [`Inference`] above, [`CompositeMutationKind`] below and `protocol::Mutation` itself already
/// made. Every mutation-leaf payload derives `ToValue`/`FromValue` and no longer derives serde's.
pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
where
    Op: Mutation<P>,
{
    const SEMANTICS: SemanticDescriptor;

    fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>;
    /// Missing/already-absent target ⇒ `Vec::new()` (the semantic replacement for the old
    /// `NoMutation` sentinel variant — there is no "no-op mutation", only an inverse with nothing
    /// to undo).
    fn inverse(&self, base: &P) -> Result<Vec<Op>, semio_framework_value::ValueError> ;
    /// 🏷️ Human undo/history label in every shell locale, e.g. `Rename piece "a" to "b"` /
    /// `Piece "a" in "b" umbenennen`. [`crate::LocalizedLabel::native`] matches on `Locale`
    /// exhaustively with no catch-all arm, so a locale added to `🖱️ui/🎚️axes/🔣️.json` fails every
    /// implementor's build until it is translated — the history panel has no English fallback.
    fn label(&self) -> crate::LocalizedLabel;
    /// ⏱️ Returns the authored clock, or absence when this leaf does not carry one.
    fn timestamp(&self) -> Option<protocol::ids::HybridLogicalTimestamp> {
        None
    }
    /// 🎯️ Structured address of the target inside the artifact (outermost segment first);
    /// empty means whole-artifact scope.
````

### E.6 SemanticMutation

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs lines 259-270`

````rust
/// `Mutation` to `SemanticMutation`, making semantic vocabulary the only expressible one at
/// compile time — see `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`.
pub trait SemanticMutation<P>: Mutation<P> {
    /// This artifact's full kind table, one row per variant — registration/introspection source.
    fn kinds() -> &'static [SemanticDescriptor];
    fn semantics(&self) -> &'static SemanticDescriptor;
    fn label(&self) -> crate::LocalizedLabel;
    fn target(&self) -> Vec<String>;
}
//#endregion 🔖️Semantics

//#region 🔖️Collection
````

### E.7 NamedTripleDiff and named_apply (diff kit)

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs lines 283-312`

````rust
// 🎞️ Was declined in an earlier pass (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️directory-spr-serde-
// removal.md`, decline #3): `modified: Vec<ItemPatch<K, Patch>>` needed `ItemPatch<K, Patch>:
// ToValue + FromValue`, which `crate::os_vcs::ItemPatch` did not yet have. It does now — converted.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedTripleDiff<K, V, Patch> {
    pub removed: Vec<K>,
    pub modified: Vec<ItemPatch<K, Patch>>,
    pub added: Vec<V>,
}

impl<K, V, Patch> Default for NamedTripleDiff<K, V, Patch> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

impl<K, V, Patch> NamedTripleDiff<K, V, Patch> {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Validates and applies a [`NamedTripleDiff`] to an id-keyed `Vec` in place:
/// removals, then patches, then appends. Validation is completed before the first write, so a
/// missing/duplicate/contradictory persisted target rejects the whole diff atomically.
pub fn named_apply<K, V, Patch>(items: &mut Vec<V>, diff: &NamedTripleDiff<K, V, Patch>) -> Result<(), MutationApplyError>
where
    K: PartialEq,
````

### E.8 CompositeMutationKind

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs lines 795-812`

````rust
/// `serde::de::DeserializeOwned` — mirrors `MutationDiff`'s own supertrait migration (see
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/`), so a plugin/extension implementing this trait never needs `serde` just to
/// satisfy it.
pub trait CompositeMutationKind<P, Op: Mutation<P>>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue {
    const SEMANTICS: SemanticDescriptor;
    fn plan(&self, base: &P, planner: &mut Planner<P, Op>) -> Result<(), PlanError>;
    fn label(&self) -> crate::LocalizedLabel;
    /// ⏱️ Returns only the clock explicitly carried by this composite payload.
    fn timestamp(&self) -> Option<protocol::ids::HybridLogicalTimestamp> {
        None
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

/// 🏗️ Runs `kind.plan` against a fresh [`Planner`] seeded at `base`. NOT a blanket
````

### E.9 fold_plan_diff and fold_plan_inverse

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs lines 818-906`

````rust
    kind.plan(base, &mut planner)?;
    Ok(planner.into_steps())
}

/// 🧬️ Folds a composite's LOCAL steps into one [`MutationOutcome`] via
/// [`MutationDiff::absorb`], applying each step against the snapshot as it stood right before that
/// step (matching [`Planner::call`]'s own advance-as-you-go semantics) — so a successful
/// `apply_diff(fold_plan_diff(k, b).diff(), &b)` equals sequential application of the plan's local steps.
/// Foreign steps never contribute to the folded diff (LAW 5 of the contract freeze). **All-or-
/// nothing** (§C4): if planning itself fails (`PlanError`) or any step's messages reach `Error` or
/// worse, the returned diff is empty (`Default::default()`) — but every message collected along the
/// way is still kept, so a caller sees exactly why. A `PlanError` additionally contributes one
/// `Fatal` message: a [`PlanError::Refused`] composite precondition its own domain-coded refusal
/// (`mutation.target-missing`, `mutation.duplicate-id`, …), any other planning failure
/// `"mutation.invariant"`. Never panics, matching this fn's frozen non-`Result` signature.
pub fn fold_plan_diff<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff> {
    let mut planner = Planner::new(base);
    let plan_result = kind.plan(base, &mut planner);
    let (steps, mut messages) = planner.into_parts();
    if let Err(error) = &plan_result {
        messages.push(match error {
            PlanError::Refused(refusal) => refusal.clone(),
            other => MutationMessage::fatal("mutation.invariant", other.to_string()),
        });
    }
    let rejected = plan_result.is_err() || matches!(worst_level(&messages), Some(level) if level >= semio_framework_diagnostic::Severity::Error);
    if rejected {
        return MutationOutcome::new(<Op as Mutation<P>>::Diff::default()).absorb_messages(messages);
    }

    let mut current = base.clone();
    let mut folded = <Op as Mutation<P>>::Diff::default();
    for step in steps {
        if let PlanStep::Local(op) = step {
            let diff = op.diff(&current).into_parts().0;
            match apply_diff(&diff, &current) {
                Ok(next) => current = next,
                Err(error) => {
                    messages.push(MutationMessage::fatal("mutation.invariant", error.to_string()).at(error.target));
                    return MutationOutcome::new(<Op as Mutation<P>>::Diff::default()).absorb_messages(messages);
                }
            }
            folded.absorb(diff);
        }
    }
    MutationOutcome::new(folded).absorb_messages(messages)
}

/// ↩️ Stores each local step's inverse against its own pre-state in forward local-step order.
/// The returned flat vector uses the same storage order as [`Mutation::inverse`]; Store reverses
/// that entire vector once when applying it. Preserving both the group order and each group's
/// stored order is required for checked or otherwise noncommutative steps. A planning failure
/// folds to an empty vector.
pub fn fold_plan_inverse<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let mut planner = Planner::new(base);
    if let Err(error) = kind.plan(base, &mut planner) {
        let (steps, _) = planner.into_steps_with_pre_states();
        for step in steps {
            if let PlanStep::Local(op) = step { op.retire_cold(); }
        }
        return Err(error.into_value_error());
    }
    let (steps, pre_states) = planner.into_steps_with_pre_states();
    let mut pending = steps.into_iter().zip(pre_states);
    let mut inverses = Vec::new();
    while let Some((step, pre_state)) = pending.next() {
        if let PlanStep::Local(op) = step {
            let Some(pre_state) = pre_state else {
                op.retire_cold();
                for value in inverses { <Op as Mutation<P>>::retire_cold(value); }
                for (step, _) in pending { if let PlanStep::Local(value) = step { value.retire_cold(); } }
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "local inverse pre-state is missing"));
            };
            let result = op.inverse(&pre_state);
            op.retire_cold();
            match result {
                Ok(values) => inverses.extend(values),
                Err(error) => {
                    for value in inverses { <Op as Mutation<P>>::retire_cold(value); }
                    for (step, _) in pending { if let PlanStep::Local(value) = step { value.retire_cold(); } }
                    return Err(error);
                }
            }
        }
    }
    Ok(inverses)
}

/// 🌐️ The [`ForeignStep`]s of a composite's plan, in discovery order — what
````

### E.10 assert_mutation_inverse_sum_law (with its LAW (L3) doc)

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs lines 635-689`

````rust
/// ✅️ LAW (L3): the concrete inverse of `mutation` sums to the negative of its diff. With `d = mutation.diff(base)`, `after =
/// apply_diff(d, base)`, the inverse operations replayed in storage-reversed order build `d_k = inv_k.diff(s_{k-1})` and
/// `s_k = apply_diff(d_k, s_{k-1})`, then `Σ = d_1 ⊕ … ⊕ d_n` by [`crate::os_spr::MutationDiff::absorb`]. Asserts (1) the inverse is
/// non-empty whenever `after != base` (no `V2-EMPTY-INVERSE`), (2) the sequential replay restores `base`, (3)
/// `apply_diff(Σ, after) == base`, and (4) `canon(Σ) == canon(d.inverse(base))` with `canon(x) = absorb(default, x)` — the inverse
/// operations are concrete, not derived from the forward diff, yet their diffs sum to exactly the negative diff.
pub async fn assert_mutation_inverse_sum_law<P, Op>(mutation: &Op, base: &P)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: crate::os_spr::Mutation<P>,
{
    use crate::os_spr::{DiffAlgebra, MutationDiff};
    use semio_framework_value::ToValue;
    fn canon<P, D: MutationDiff<P>>(diff: D) -> D {
        let mut canonical = D::default();
        canonical.absorb(diff);
        canonical
    }
    let (forward, messages) = mutation.diff(base).into_parts();
    let rejected = messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    assert!(!rejected, "a mutation expected to invert cleanly must not have been rejected — forward outcome carries an Error/Fatal message: {messages:?}");
    let after = crate::os_spr::apply_diff(&forward, base).expect("valid forward diff must apply");
    let changed = after != *base;
    let negative = forward.inverse(base);
    forward.retire_cold();
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    let non_empty = !backward.is_empty();
    backward.reverse();
    let mut state = after.clone();
    let mut sum = <Op::Diff as Default>::default();
    for undo in &backward {
        let (step, _) = undo.diff(&state).into_parts();
        let next = crate::os_spr::apply_diff(&step, &state).expect("valid inverse diff must apply");
        sum.absorb(step);
        state = next;
    }
    for undo in backward {
        Op::retire_cold(undo);
    }
    let replayed = state == *base;
    let summed = crate::os_spr::apply_diff(&sum, &after);
    let summed_restores = summed.as_ref() == Ok(base);
    let canonical_sum = canon::<P, Op::Diff>(sum);
    let canonical_negative = canon::<P, Op::Diff>(negative);
    let matches_negative = !changed || canonical_sum == canonical_negative;
    let report = format!("sum={:?} negative={:?} restored={state:?} base={base:?}", canonical_sum.to_value(), canonical_negative.to_value());
    canonical_sum.retire_cold();
    canonical_negative.retire_cold();
    assert!(!changed || non_empty, "mutation.inverse(base) must not be empty for a mutation that changes state; base={base:?}");
    assert!(replayed, "applying mutation.inverse(base) (reversed) after mutation must restore base; {report}");
    assert!(summed_restores, "the summed inverse diffs must restore base from the applied state; {report}");
    assert!(matches_negative, "the summed inverse diffs must equal the negative of the forward diff, d.inverse(base); {report}");
}

/// ✅️ LAW: `D::between(a, b).apply(a) == b`, and `D::between(a, a).is_empty()` —
````

### E.11 store::apply_operation and store::apply_outcome

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs lines 25216-25244`

````rust
/// 🎯️ The one store step from an operation to its next projection: computes the op-stamped diff, applies it through
/// [`protocol::apply_diff`] and retires the diff cold. Every store lane, replay, merge and backbone fold routes through
/// here; the apply refusal stays structured for callers that must fail instead of record.
pub fn apply_operation<P, Mutation>(state: &P, operation: &Mutation, op_index: u32) -> (crate::os_spr::MutationApplyResult<P>, Vec<crate::os_spr::MutationMessage>)
where
    Mutation: self::Mutation<P>,
{
    let (diff, messages) = operation.diff(state).stamp_op_index(op_index).into_parts();
    let applied = crate::os_spr::apply_diff(&diff, state);
    <Mutation::Diff as MutationDiff<P>>::retire_cold(diff);
    (applied, messages)
}

/// 🛡️ Applies an already computed outcome through [`protocol::apply_diff`]: the next projection with the outcome intact, or —
/// on an apply refusal — the unchanged `base`, an empty diff and one `Fatal` message. The builder-side twin of the fatal
/// message production dispatch records.
pub fn apply_outcome<P, D>(base: &P, outcome: crate::os_spr::MutationOutcome<D>) -> (P, crate::os_spr::MutationOutcome<D>)
where
    P: Clone,
    D: MutationDiff<P>,
{
    let (diff, mut messages) = outcome.into_parts();
    match crate::os_spr::apply_diff(&diff, base) {
        Ok(next) => (next, crate::os_spr::MutationOutcome::new(diff).absorb_messages(messages)),
        Err(error) => {
            messages.push(crate::os_spr::MutationMessage::fatal(error.code, error.message).at(error.target));
            D::retire_cold(diff);
            (base.clone(), crate::os_spr::MutationOutcome::new(D::default()).absorb_messages(messages))
        }
````

### E.12 os_vcs::apply_mutation

`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs lines 1796-1816`

````rust
// 🎞️ `Mutation`/`MutationDiff`/`MutationMessage` live in `protocol_command`; this region just
// replays a snapshot through an operation's forward diff — the pure per-step transform every
// store-level replay uses.

/// ▶️ Computes `operation.diff(snapshot)`, applies the resulting diff, and returns the new
/// snapshot alongside every [`crate::os_spr::MutationMessage`] the outcome carried. Diff-apply
/// rejection is returned as its structured [`MutationApplyError`] before a snapshot is produced. A `Fatal`
/// message's diff is `D::default()` by construction (§C2 LAW 1), so applying it is always a no-op —
/// callers that must not silently apply a rejected op check `worst_level(&messages)` against their
/// `MergePolicy` themselves (this fn stays policy-agnostic, matching its old unconditional-apply
/// shape).
pub fn apply_mutation<P, Mutation>(snapshot: &P, operation: &Mutation) -> Result<(P, Vec<crate::os_spr::MutationMessage>), MutationApplyError>
where
    Mutation: self::Mutation<P>,
{
    let (diff, messages) = operation.diff(snapshot).into_parts();
    let applied = crate::os_spr::apply_diff(&diff, snapshot);
    MutationDiff::retire_cold(diff);
    Ok((applied?, messages))
}

````

### E.13 Kernel facade: os_spr module, protocol_laws mount and cfg gate

`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs lines 140-190`

````rust
}

#[path = "."]
pub mod os_spr {
    #[path = "../../🔨️modules/📡️spr/🦀️.rs"]
    mod component;
    pub use component::*;

    // 📡️ The replication contract itself (frames, envelopes, mutation traits, conflict vocabulary,
    // `.spr` format) lives in `🧰️framework/🔨️modules/📡️replication`; the kernel speaks it but no
    // longer owns it. This facade keeps every historical `protocol::`/`os_spr::` path working.
    pub use protocol::causal;
    pub use protocol::conflict;
    pub use protocol::crypto;
    pub use protocol::dictionary;
    pub use protocol::format;
    pub use protocol::ids;
    pub use protocol::scalar;
    pub use protocol::wire;

    #[path = "../../🔨️modules/📡️spr/🧵️channel/🦀️.rs"]
    pub mod channel;

    #[cfg(not(target_arch = "wasm32"))]
    #[path = "../../🔨️modules/📡️spr/⌨️cli/🦀️.rs"]
    pub mod cli;

    // 🎞️ The os authoring half of the command layer (inference, semantics, diff kit, descriptor
    // registry, composite planner). It re-exports `protocol::mutation`'s contract from its own file,
    // so `os_spr::command::Mutation` and friends still resolve here.
    #[path = "../../🔨️modules/📡️spr/🎮️command/🦀️.rs"]
    pub mod command;

    pub use self::crypto::*;
    pub use self::dictionary::*;
    pub use self::ids::*;
    pub use self::wire::*;

    #[path = "../../🔨️modules/📡️spr/📜️history/🦀️.rs"]
    pub mod history;

    #[path = "../../🔨️modules/📡️spr/🔌️io/🦀️.rs"]
    pub mod io;

    #[path = "../../🔨️modules/📡️spr/💎️materialize/🦀️.rs"]
    pub mod materialize;

    #[cfg(any(test, feature = "protocol-laws"))]
    #[path = "../../🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs"]
    pub mod protocol_laws;
}
````
