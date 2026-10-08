# Mutation System Audit: Framework, Schemas, Generation3d and CAD

Date: 2026-10-07. Scope: read-only audit of the framework mutation system as used by plugin artifacts, with generation3d as the main example and cad as the secondary layout example.

Method: `git grep` and `find` over tracked files, plus targeted reads. No cargo builds, no tests executed, no edits. Every claim below is from code read in this session. Items marked "not verified" were not confirmed.

## 0. Corrections to the brief

- generation3d has 24 entries under `🧬️mutations/`, but only 22 are mutation leaves. The other two are `🧪️tests/` and `🧫️fixtures/`.
- Of the 22 leaves, only 20 are variants of `Generation3dMutation`. `👆️select-generation` and `📝️change-generation-preview` are descriptor-only folders: they have `🔣️.json` and `🧬️schema/🔣️.json` and nothing else. They are not in the enum, the TypeScript union, the root JSON `oneOf`, or the fixtures.
- The "generated code" in this system is mostly compile-time expansion of `#[derive(dsl::Mutations)]` and `#[derive(dsl::MutationLeaf)]`, not generated files. The only generated mutation artifact committed in this tree is `🔣️mutation-authority.json` (see §2.3). The schema catalog is also generated, but its writer was not located.

## Path abbreviations

Repo root: `/Users/ueli/Documents/semio`. All paths below are relative to it.

| Abbrev | Path |
|---|---|
| `GEN` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `GMUT` | `GEN/🧬️schema/🧬️mutations` |
| `GCMD` | `GEN/✏️editor/🎮️commands` |
| `CAD` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `CMUT` | `CAD/🧬️schema/🧬️mutations` |
| `SPR` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr` |
| `DSL` | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl` |
| `REPL` | `🧰️framework/🔨️modules/📡️replication` |
| `STORE` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` |
| `PLUGIN` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` |
| `VCS` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs` |
| `FLOW` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow` |
| `LIB` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` |

---

## 1. Core types and the mutation lifecycle

### 1.1 Where the core traits live

| Concept | Location (line) |
|---|---|
| `Mutation<P>` trait (per-op: diff, inverse, descriptor, conflict target, undo policy, state class) | `REPL/🎮️mutation/🦀️.rs:174` |
| `MutationDiff<P>` trait (`apply`, `absorb`, `retire_cold`, `retire_projection`) | `REPL/🎮️mutation/🦀️.rs:103` |
| `MutationLeaf` trait (static leaf metadata) | `REPL/🎮️mutation/🦀️.rs:971` |
| `DiffAlgebra<P>` (inverse / between / is_empty) | `REPL/🎮️mutation/🦀️.rs`, between lines 103 and 174 |
| Re-export surface of the mutation vocabulary | `SPR/🦀️.rs:46-48` (`pub use crate::os_spr::command::{... DiffRegions, Mutation, MutationKind ...}`) |
| `MutationKind<P, Op>` (per-leaf payload behaviour: `SEMANTICS`, `diff`, `inverse`, `label`, `timestamp`, `target`, `may_emit_foreign_steps`, `foreign_steps`) | `SPR/🎮️command/🦀️.rs:219` (diff :225, inverse :229, label :234, target :241, foreign_steps :251) |
| `SemanticMutation<P>` (kinds, semantics, label, target for the aggregate) | `SPR/🎮️command/🦀️.rs:261` |
| `DiffRegions` (`touches() -> TouchedPaths`) | `SPR/🎮️command/🦀️.rs:83` |
| `TouchedPaths` | `SPR/🎮️command/🦀️.rs:41` |
| `APPROVED_VERBS`, `is_approved_verb` (verb whitelist for `<verb>-<noun>` names) | `SPR/🎮️command/🦀️.rs:111`, `:179` |
| `MutationDescriptor` / `MutationDescriptorRegistry` (runtime registry, fingerprinted) | `SPR/🎮️command/🦀️.rs:437`, `:525` |
| `CompositeMutationKind` + `Planner` (multi-step plans, `MAX_PLAN_DEPTH = 8`) | `SPR/🎮️command/🦀️.rs:799`, `:688`, `:632` |
| Mutation law checks used by the test harness | `SPR/🎮️command/🦀️.rs:930` (round-trip), `:962` (input schema), `:979` (labels), `:1035` (inverse rows) |
| `apply_mutation` (pure step: `op.diff(snapshot)` then `diff.apply`) | `VCS:1911` |
| `Emit<Mutation, ConfigMutation, DraftMutation>` (what `ArtifactApp::handle` returns) | `PLUGIN:13430` (fields `artifact_mutations`, `config_mutations`, `transaction`, `transaction_phase`, `effects`, `extension_invocations`, `interaction_writes`, `ui_scope`) |
| `Emit::commit_transaction` | `PLUGIN:14027` |
| `ArtifactCommand<Mutation>` (`Apply`, `Undo`, `Redo`, `UndoWithPolicy`, `ApplyInLane`, ...) | `STORE:3380` |
| `UndoPolicy` (`ExactBaseOnly`, `TransformAgainstConcurrent`, `SemanticUndo`, `CompensatingAction`) | `REPL/🧾️wire/🦀️.rs:175` |

Key signatures, copied from the code:

```rust
// REPL/🎮️mutation/🦀️.rs:103
pub trait MutationDiff<P>: Clone + Default + crate::value::ToValue + crate::value::FromValue {
    fn apply(&self, base: &P) -> MutationApplyResult<P>;
    fn absorb(&mut self, other: Self);            // sequential coalesce only, never concurrent merge
    fn retire_cold(self) where Self: Sized {}      // explicit disposal, fail-closed roots override
    fn retire_projection(projection: P) { drop(projection); }
}

// REPL/🎮️mutation/🦀️.rs:174
pub trait Mutation<P>: Clone + crate::value::ToValue + crate::value::FromValue {
    type Diff: MutationDiff<P>;
    const DESCRIPTORS: &'static [MutationLeafDescriptor];
    fn descriptor(&self) -> &'static MutationLeafDescriptor;
    fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;
    fn inverse(&self, base: &P) -> Result<Vec<Self>, semio_framework_value::ValueError>;
    fn conflict_target(&self) -> Vec<String> { Vec::new() }   // forwarded from MutationKind::target by the derive
    fn undo_policy(&self) -> crate::UndoPolicy { crate::UndoPolicy::ExactBaseOnly }
    fn state_class(&self) -> semio_framework_schema_state::StateClass { /* Artifact */ }
    fn input_schema(&self) -> Option<&'static str> { None }
    // plus mutation_id, dependencies, base_version, author_id, timestamp, inverse_rows, foreign_steps
}

// SPR/🎮️command/🦀️.rs:83
pub trait DiffRegions { fn touches(&self) -> TouchedPaths; }
```

### 1.2 Lifecycle: schema, generated code, apply, diff, history, replication

1. Schema (declaration). The taxonomy (`LIB/🔣️taxonomy.json`) declares a mutation collection. Each leaf declares a descriptor `🔣️.json` (`semanticKind`, `binaryTag`, `invertibility`, `outcomeClasses`, `requiredLanguageSurfaces`, ...) and a payload JSON Schema `🧬️schema/🔣️.json`. The artifact root declares the enum and union in `🦀️.rs`, `🟦️.ts`, `🔣️.json` (`oneOf`), `🛰️.proto`, `🕸️.graphql`.
2. Generated Rust at compile time. `#[derive(dsl::MutationLeaf)]` on each payload struct reads the leaf's `🔣️.json` and the committed authority file, validates it, and emits the `MutationLeaf` constants, including `PAYLOAD_SCHEMA = include_str!(...)` (`DSL/✨️derive/🦀️.rs:629`) and inverse-row counts (`:649`). `#[derive(dsl::Mutations)]` on the aggregate enum emits `impl protocol::Mutation` for the enum: `DESCRIPTORS`, `descriptor()`, `diff` and `inverse` delegated to each leaf's `MutationKind`, `conflict_target` from `MutationKind::target`, `INPUT_SCHEMAS`, and a generated law test (`mutation_payload_law_test`, `DSL/✨️derive/🦀️.rs:1214`). Entry points: `DSL/✨️derive/📦️packages/🦀️rust/🦀️.rs:8` (`MutationLeaf`), `:43` (`Mutations`), `:59` (`CompositeMutation`); implementations in `DSL/✨️derive/🦀️.rs` (`expand_derive_mutations` :924, `expand_mutations` :944, `expand_mutation_leaf` :572).
3. Decide (diff). A leaf's `MutationKind::diff` (hand-written, delegating to `🔺️diff/🦀️.rs`) builds a sparse aggregate diff and returns `MutationOutcome`. Rejections are `MutationOutcome::fatal("mutation.duplicate-id", ...)` style messages, not errors.
4. Apply. `MutationDiff::apply(base)` (`REPL:103`). `VCS:1911` `apply_mutation` = `op.diff(snapshot).into_parts()` then `diff.apply(snapshot)`, then `retire_cold`.
5. Inverse, computed from the pre-state. `MutationKind::inverse(base)` returns `Vec<Aggregate>` from `↩️inverse/🦀️.rs`. For example, `create-widget` inverts to `delete-widget` by the created id.
6. Emit and record. A command `handle` returns `Emit { artifact_mutations, config_mutations, ... }`. The plugin publishes the artifact mutations in chunks of 32 (`PLUGIN:31683`) as `ArtifactCommand::Apply { mutations, transaction }`. The store's `dispatch` (`STORE:20377`) calls `apply_command` (`STORE:22998`), which runs `replay_mutations` to get forwards and inverse, mints an edit id, and builds `Edit { id, actor, forwards, inverse, mutation_meta, verb, line, sequence_number, ... }`.
7. History, undo, redo. `undo_with_policy` (`STORE:22857`):
   - `ExactBaseOnly` and `TransformAgainstConcurrent` take the latest local document-lane edit and apply its stored inverse (`undo_lane_position`).
   - `SemanticUndo` and `CompensatingAction` dispatch a supplied compensating command.
   - Redo walks `redo_edit_ids`, restricted to edits this actor authored.
   The history is persisted as an SPR log (`SPR/📜️history`, `SPR/💎️materialize`).
8. Replication. `flush_outbound` (`STORE:24031`) drains `pending_report.outbound` in batches (`document_backbone_batches`) and sends `BackboneMessage::Mutations { envelopes: encode_envelopes(batch) }`. Each envelope carries `conflict_target()`, so the hub can tell disjoint writes from overlapping ones. Concurrent merge is an authority `MergePolicy` concern (`REPL/⚔️conflict`), not `MutationDiff::absorb`.

### 1.3 Composite and config layers

- Composite mutations (`#[derive(CompositeMutation)]`, `DSL/✨️derive/📦️packages/🦀️rust/🦀️.rs:59`) are not used by generation3d or cad. They plan multi-step ops through `Planner`.
- Editor config is a separate `dsl::Mutations` enum: `GEN/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:37-39` with `#[mutations(snapshot = Generation3dConfig, diff = Generation3dConfig, schema = "generation3dcfg")]`. Its leaves are `set-snapshot`, `set-sun`, `set-show-mode`, `set-preview-camera`, `set-lod-mode`, `set-camera`, `set-selected`. Config mutations ride `Emit.config_mutations`, not the document history.

---

## 2. Schema-first layout of a mutation folder

### 2.1 Per-leaf files (generation3d, all 20 enum leaves verified)

For each of the 20 enum leaves in `GMUT/<leaf>/`:

| File | Hand-written or generated | Purpose |
|---|---|---|
| `🔣️.json` | Hand-written descriptor (scaffold may create it) | `semanticKind`, `aggregateVariant`, `binaryTag`, `textOpcode` (null for all), `invertibility` (explicit-mutation for all), `diffParticipation` (detect), `outcomeClasses`, `composition` (atomic), `requiredLanguageSurfaces` (rust, json-schema, text, binary). Parsed by `DSL/✨️derive/🦀️.rs:371`. |
| `🦀️.rs` | Hand-written | Payload struct with `#[derive(dsl::MutationLeaf)] #[mutation_leaf(contract = ::protocol)]`, `SEMANTICS` const (verb, entity, kind, record), and `impl protocol::MutationKind<Snapshot, Aggregate>` (`diff`, `inverse`, `label`, `target`). Example: `GMUT/🌱️create-widget/🦀️.rs`. |
| `🧬️schema/🔣️.json` | Hand-written JSON Schema, `$id` under `https://json.schemas.assets.semio-tech.com/s/procedural/generation3d/mutation/<kind>/schema.json` | Payload contract. Included at compile time (`DSL/✨️derive/🦀️.rs:629`). Carries `x-semio-ui` widget hints. |
| `🔺️diff/🦀️.rs` + `🔺️diff/🟦️.ts` | Hand-written (Rust is the source; TS mirror) | Builds the sparse `Generation3dDiff` for the payload. |
| `↩️inverse/🦀️.rs` + `↩️inverse/🟦️.ts` | Hand-written | Inverse mutation list from the base. |
| `🦠️mutation/🟦️.ts` | Hand-written TS mirror of the payload type | Used by the root `🟦️.ts` union. |
| `🧪️tests/<case>/🦀️.rs` | Hand-written | Loads the fixture quintet via `include_str!` and checks apply, diff, inverse. Present for 14 leaves (see §6.2). |

The shared aggregate files are at `GMUT` root:

| File | Role |
|---|---|
| `🦀️.rs` | Aggregate enum `Generation3dMutation` (`:224`), its `#[derive(dsl::Mutations)]` and `#[mutations(snapshot = Generation3dSnapshot, diff = Generation3dDiff, schema = "generation.3d")]` (`:223`), kind list `KINDS` (`:252`), host-helper functions, and `pub mod` glue blocks (the `//#region 🔖️NewLeaves` block at the top). The file header refers to a `glue.rs` that does not exist in this tree. |
| `🟦️.ts` | Discriminated union `Generation3dMutation` over the 20 leaves, with `import type` per leaf. Hand-written mirror. |
| `🔣️.json` | JSON Schema `oneOf` with 20 `$ref`s to each leaf's `schema.json`. Hand-written. |
| `🛰️.proto` | Protobuf: artifact-lane fields only (`FlowHostSnapshot host_snapshot`, `GenerationPlayState generation`). Hand-written. |
| `🕸️.graphql` | GraphQL mirror of the proto. Hand-written. |
| `🧪️tests/` | `🔬️unit`, `🧪️gesture-leaves` (covers the six leaves without a per-leaf test), `📄️document-restoration` (Bun test plus tsc). |
| `🧫️fixtures/🧬️mutations/<leaf>/<case>/` | Fixture quintet per leaf (see §2.2). |

The `🟦️.ts` root mirror is validated by the package checks listed in §5.5, not by a generator.

### 2.2 Fixtures and tests

Fixture quintet, one case per leaf (20 cases total), at `GEN/🧫️fixtures/🧬️mutations/<leaf>/📝️inserts/` (the case name varies):

```
🦠️mutation/🔣️.json        the typed payload
🔺️diff/🔣️.json            the expected diff
🎯️outcome/🔣️.json         the expected outcome
📸️snapshot/⬅️before/🔣️.json
📸️snapshot/➡️after/🔣️.json
```

The leaf's `🧪️tests/<case>/🦀️.rs` includes these files with `include_str!` (e.g. `GMUT/🌱️create-widget/🧪️tests/📝️inserts/🦀️.rs` loads `GEN/🧫️fixtures/...`). Its header says the JSON quintet is the source of truth and other encodings (`.op.semio`, `.spr.semio`, `.dsl.semio`, `.pack.semio`, `.patch.semio`) are derived by a `fixtures generate` command. That command was not located in this audit.

### 2.3 What is generated, by which tool

| Artifact | Generated by | Evidence |
|---|---|---|
| Compile-time impls on the enum and leaves (`impl Mutation`, `MutationLeaf` consts, `DESCRIPTORS`, `INPUT_SCHEMAS`, law tests) | `#[derive(dsl::Mutations)]`, `#[derive(dsl::MutationLeaf)]` in `DSL/✨️derive/🦀️.rs` | `:924`, `:944`, `:572`, `:1214` |
| `DSL/✨️derive/🔣️mutation-authority.json` | `bun nx run @semio-tech/dsl-derive-rs:generate`, script `DSL/✨️derive/📦️packages/🦀️rust/📜️script.ts` (`GenerateScript`, `mutationAuthorityProjection`) | The file is a projection of the taxonomy's mutation collections. `check` fails when stale. |
| New-mutation skeleton | `bun ./📜️script.ts new mutation <owner-mutation-root> <emoji-verb-noun> [--typescript --json-schema --text --binary --graphql --protobuf --composite --dry-run]`. Implementation: `LIB/🏗️authoring/🧬️mutation-tree/🟦️.ts:224` (`newScaffoldMutationTree`), command class `LIB/🏗️authoring/🎮️command/🟦️.ts` (`CleanMechanismNewScript`). | Usage string at `🎮️command/🟦️.ts`. The router that exposes it was not located by name in this audit. |
| `LIB/🔣️schema-catalog.json` | Listed in the taxonomy as `schemaExportResolution.catalogPath`. Writer not located. | `LIB/🔣️taxonomy.json:28458` |
| Everything else (`🦀️.rs`, `🔣️.json`, `🧬️schema/🔣️.json`, `🟦️.ts`, `🛰️.proto`, `🕸️.graphql`, fixtures) | Hand-written. None of these files carries a `@generated` or `DO NOT EDIT` marker (checked for `GMUT` root and leaves). | `git grep` for markers returned none. |

Note: `🧰️framework/🔨️modules/🧬️schema/🤖️generated/` contains only `🏷️entity-kinds/`. It is not a mutation generator.

Note: the committed `DSL/✨️derive/🔣️mutation-authority.json` contains no generation3d or procedural entries, and HEAD matches the working tree. This is consistent with the taxonomy's `mutationAggregateSources`, and it does not break flat leaves: `mutation_source_authority` (`DSL/✨️derive/🦀️.rs:82`) only requires registration for domain-operation roots and component aggregates.

---

## 3. The generation3d mutations

Folder names as on disk, `semanticKind` from `🔣️.json`, and the one-line effect on the document (from each leaf's module doc). All 20 enum leaves have the full file set (rs, schema, TS mirror, diff rs+ts, inverse rs+ts). Per-leaf test status is in §6.2.

| # | Folder (under `GMUT/`) | semanticKind | Enum variant | binaryTag | Effect |
|---|---|---|---|---|---|
| 1 | `🌱️create-widget` | create-widget | CreateWidget | 0 | Inserts a widget at an index; duplicate id rejected with `mutation.duplicate-id`. |
| 2 | `🩹update-widget` | update-widget | UpdateWidget | 1 | Replaces an existing widget's whole body atomically. |
| 3 | `❌delete-widget` | delete-widget | DeleteWidget | 2 | Removes a widget by id. The feature narrative says it does not cascade. |
| 4 | `🔗️connect-synapse` | connect-synapse | ConnectSynapse | 3 | Adds a synapse (edge) between two ports at an index. |
| 5 | `🔄️update` | update-synapse | UpdateSynapse | 4 | Atomically replaces endpoints and ports of an existing synapse. |
| 6 | `✂️disconnect-synapse` | disconnect-synapse | DisconnectSynapse | 5 | Removes a synapse by id. |
| 7 | `📍️move` | move-widget | MoveWidget | 6 | Sets the absolute canvas position of a widget in the layout map. |
| 8 | `🧹️delete-widget` | delete-widget-position | DeleteWidgetPosition | 7 | Removes a widget's layout override. The directory name is misleading. |
| 9 | `📷️update-camera` | update-camera | UpdateCamera | 8 | Replaces the document camera `{x, y, zoom}` as one facet. |
| 10 | `🔤️change-schema` | change-schema | ChangeSchema | 9 | Sets the fixture's schema version string. |
| 11 | `➕create-generation` | create-generation | CreateGeneration | 10 | Adds an id-keyed generation; delegates to the playbook `GenerationMutation::Add`. |
| 12 | `🗑️delete` | delete-generation | DeleteGeneration | 11 | Removes a generation by id. |
| 13 | `🏷️rename` | rename-generation | RenameGeneration | 12 | Changes a generation's `name`. |
| 14 | `🔧️change` | change-generation-value | ChangeGenerationValue | 13 | Sets one answer in a generation's form-values map. |
| 15 | `🎚️change-slider-value` | change-slider-value | ChangeSliderValue | 14 | Sets an input slider's absolute value. Continuous controls commit this leaf. |
| 16 | `✋️drag-transforms` | drag-transforms | DragTransforms | 15 | One gumball drag as intent: an offset added to each addressed translate operator. |
| 17 | `🔃️rotate-transforms` | rotate-transforms | RotateTransforms | 16 | One gumball rotation as a world-axis rotation composed into each rotate operator. |
| 18 | `📏️scale-transforms` | scale-transforms | ScaleTransforms | 17 | One gumball scale: per-axis factors multiplied into each scale operator. |
| 19 | `🚚️move-nodes` | move-nodes | MoveNodes | 18 | One node-graph drag: a canvas offset applied from each addressed widget's base position. |
| 20 | `🎛️change-widget-input` | change-widget-input | ChangeWidgetInput | 19 | Sets an unconnected operator input, or a text source, to an absolute typed literal. |
| 21 | `👆️select-generation` | select-generation | none (descriptor only) | 20 | No Rust leaf and no fixture. Selection in practice is the config mutation `SetSelectedGeneration`. |
| 22 | `📝️change-generation-preview` | change-generation-preview | none (descriptor only) | 21 | No Rust leaf and no fixture. Not wired into the enum, union, or `oneOf`. |

Binary tags 0 through 21 are each used exactly once across the 22 folders. The uniqueness check was not located, so the next free value (22) must be chosen by hand.

Naming hazard: several directory names are pre-migration names that no longer match `semanticKind`: `❌delete-widget` is kind `delete-widget`, `🧹️delete-widget` is kind `delete-widget-position`, `🔄️update` is `update-synapse`, `📍️move` is `move-widget`, `🗑️delete` is `delete-generation`, `🔧️change` is `change-generation-value`, `🏷️rename` is `rename-generation`. Each doc header says so, but the pair `❌delete-widget` and `🧹️delete-widget` is easy to misread.

---

## 4. How editor commands become mutations

There are 40 command folders under `GCMD/`. Each has a `🦀️.rs` with `handle(payload, doc: &ArtifactView<Generation3dSnapshot>, cfg: &ConfigView<Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault>`. Commands are registered in one place: `GEN/✏️editor/🦀️.rs:118`, which is a `semio_framework_plugin::app_commands!` block listing `"camelName" as "kebab-keyword" => module::Payload` rows. Row order is the binary ordinal, so appending is safe and reordering is a wire break.

`doc` is an immutable `ArtifactView`, so no command writes the document in place. The routes below are the ways a command produces a change.

### 4.1 Classification of all 40 commands

Category A: declared leaves emitted directly (15 commands).

| Command | How the mutation is produced |
|---|---|
| `↔️translate-selection`, `📏️scale-selection`, `🔄️rotate-selection` | `gumball_once` in `GCMD/🧭️transforms` yields one declared transform leaf. |
| `🧭️transforms` | Gumball tool yields `Drag`/`Rotate`/`ScaleTransforms` leaves (`GCMD/🧭️transforms/🦀️.rs`, `leaf` :127, `of_leaf` :136). Operator splices use `commit_host_snapshot` (see B). |
| `🎚️set-widget-input` | `input_leaf` (`:93`) returns `ChangeWidgetInput` or `UpdateWidget`. Mesh fields are category C. |
| `🩹️patch-flow-widgets` | `patch_leaves` returns `ChangeSliderValue` per addressed slider. |
| `➕️add-generation`, `🏷️rename-generation`, `🗑️remove-generation`, `🎚️update-generation-values` | `generation_command_result` (`GCMD/🧬️generation/🦀️.rs:21`) runs the playbook `generation_operations` on a clone of state and maps each op with `generation_mutation_to_generation3d`. |
| `🎯️select-generation` | Same helper, no artifact ops, config mutation `SetSelectedGeneration`. |
| `🧬️generation` | Same helper plus `SetSelectedGeneration`. |
| `🎨️set-active-example` | `generation3d_host_snapshot_operations(current, target)` (diff) plus config `SetSnapshot`. See B. |
| `📥️import-document` | `apply_complete_payload` (diff ops plus `SetSnapshot`). The texture path uses category C. |
| `🔪️knife-mesh-selection` | `cut_rows` builds rows via `mesh_operation_rows`, which was not inspected in depth. Also sets interaction writes. |

Category B: declared leaves computed by whole-snapshot diff through a scratch host (6 commands).

| Command | Route |
|---|---|
| `➖️remove-widget` | `with_host` builds a scratch `FlowHost`, calls `host.remove_widget` (`FLOW/🖥️host/🦀️.rs:906`), then `commit_host_snapshot` (`GEN/🧬️schema/🦀️.rs:260`). |
| `❌️delete-selection` | `delete_selected` (`:17`) on a scratch host, then `commit_host_snapshot` (`:26`). |
| `✏️node-graph-edit` | `apply_rows` (`:38`) on a scratch host, then `commit_host_snapshot` (`:62`). Also emits slider and node-drag leaves directly (`:78`). |
| `🧩️add-widget` | `host.add_widget` (`FLOW/🖥️host/🦀️.rs:891`), then `commit_host_snapshot` (`:92`). |
| `🗺️reorganize` | `host.reorganize` (`FLOW/🖥️host/🦀️.rs:1238`), then `Emit::mutations(commit_host_snapshot(...))`. |
| `🥽️edit-mesh-selection` | Three `commit_host_snapshot` sites plus `commit_transaction`. Mesh edits also use category C. |

Category C: declared leaf whose payload is an ad-hoc JSON or text patch (no structured intent).

| Command | Route |
|---|---|
| `🎚️set-widget-input` (mesh paths) | `mesh_source_leaf` (`:136`) reads the whole mesh JSON, patches it in `edit_mesh_source` (`:144`), `edit_mesh_asset` (`:191`), `edit_mesh_attribute` (`:245`), `edit_mesh_array` (`:274`), and emits `ChangeWidgetInput { value: WidgetInputValue::Text(<whole document>) }`. Also `import_mesh_texture` (`:173`). |
| `📥️import-document` (texture) | Builds a `ChangeWidgetInput` text value from a patched mesh source (`GCMD/📥️import-document/🦀️.rs:88`). |

Category D: out-of-band writes that bypass the mutation system (1 command).

| Command | Route |
|---|---|
| `🧩️set-contributions` | Calls `semio_framework_os_flow::sync_host_flow_extension_contributions_page`, which writes a process-wide flow extension registry, then invalidates the session. Returns `Emit::default()`. No document or config mutation, so it is not in history. |

Category E: config-only mutations (10 commands).

`🌄️set-sun-elevation`, `🌞️toggle-sun`, `🔆️set-sun-intensity`, `🧭️set-sun-azimuth` (`SetSun`); `👁️set-show-mode`, `🔁️cycle-show-mode` (`SetShowMode`); `📷️set-camera` (`SetPreviewCamera`); `🔭️node-graph-viewport` (`SetCamera`); `🔬️set-lod-mode`, `🔁️cycle-lod-mode` (`SetLodMode`).

Category F: ephemeral, session, or interaction only (8 commands).

`⏱️flow-eval-tick`, `✅️flow-eval-resolve`, `🔓️flow-eval-release`, `🔺️flow-tessellate-resolve`, `🧯️flow-tessellate-cancel-resolve` (extension invocations and UI scope), `🧭️navigate-graph` (interaction writes), `📂️import-document-request` (file-open effect), `📤️export-document` (download effect).

Counts: A 15, B 6, C folded into A (`set-widget-input`, `import-document`, `edit-mesh-selection`), D 1, E 10, F 8. Total 40.

### 4.2 Commands that change the document without a declared intent

Strict bypass (a Rust write to the document without going through `Emit`): none found. `doc` is immutable.

Effective bypasses, in order of severity:

1. Category C, ad-hoc JSON patching. `GCMD/🎚️set-widget-input/🦀️.rs:136-142` and `:144-170` produce a `ChangeWidgetInput` whose value is the entire serialized mesh. History granularity and conflict granularity (target = widget id) are both coarse. A TypeScript mirror (`GCMD/🎚️set-widget-input/🟦️.ts`, `editMeshSource`) does the same patch in the UI.
2. Category B, undeclared host vocabulary plus snapshot diff. Commands call `FlowHost` methods (`add_widget`, `remove_widget`, `move_widget`, `connect`, `connect_ports`, `disconnect`, `reorganize`, `FLOW/🖥️host/🦀️.rs:891-1238`) that are not `Generation3dMutation` leaves. The result is then turned into ops by `generation3d_host_snapshot_operations` (`GMUT/🦀️.rs:462`). That function emits `DeleteWidget`, `CreateWidget`, `UpdateWidget`, `DisconnectSynapse`, `ConnectSynapse`, `UpdateSynapse`, `DeleteWidgetPosition` purely from before and after. So the declared ops are real, but the intent is lost, and cascades appear as separate rows. Whole-document replacement (`set-active-example`, `import-document`) is a diff of the entire snapshot.
3. Category D, process-wide registry write. `set-contributions` mutates shared process state, outside undo and replication.
4. Ephemeral evaluation outputs. `FlowHost::apply_eval_outputs_json` (`FLOW/🖥️host/🦀️.rs:519`) is called from `FLOW/🕸️wasm/🦀️.rs:1546`. It writes evaluation results into the host, not the document. Listed for completeness.

---

## 5. Recipe: add a new mutation to an artifact

Example: adding `change-generation-preview` as a real leaf in generation3d. Adapt the paths for another artifact.

### 5.1 Name and scaffold

1. Choose `<emoji>-<verb>-<noun>` (kebab, verb first, at least two parts). `newMutationSemanticParts` (`LIB/🏗️authoring/🧬️mutation-tree/🟦️.ts:40`) enforces the form, and the verb must be in `APPROVED_VERBS` (`SPR/🎮️command/🦀️.rs:111`).
2. Scaffold the skeleton. Usage string from `LIB/🏗️authoring/🎮️command/🟦️.ts`:
   ```
   bun ./📜️script.ts new mutation GMUT "📝️change-generation-preview" --typescript --json-schema [--text --binary --graphql --protobuf] [--dry-run]
   ```
   Run it from the owning script directory and check the router. The scaffold creates `🔣️.json`, `🦀️.rs`, `🧬️schema/🔣️.json`, `🔺️diff`, `↩️inverse`, `🦠️mutation/🟦️.ts` (with the flags), and updates the aggregate root (`newMutationUpdateAggregate`, `🧬️mutation-tree/🟦️.ts:181`). Read the aggregate edit before relying on it. Use `--dry-run` first.

### 5.2 Hand-written edits (generation3d)

3. Descriptor `🔣️.json`: set `semanticKind`, `aggregateVariant`, `displayName`, `emoji`, a unique `binaryTag` (next free: 22), `invertibility`, `outcomeClasses`, and `requiredLanguageSurfaces`. Keep `payloadSchema` as `🧬️schema/🔣️.json`.
4. Payload `🦀️.rs`: struct with `#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]` and `#[mutation_leaf(contract = ::protocol)]`. Add `SEMANTICS` (verb, entity, kind, record; the derive asserts `kind` matches the descriptor). Implement `protocol::MutationKind<Generation3dSnapshot, Generation3dMutation>`: `diff`, `inverse`, `label` (native EN/DE), and `target` (the address used for conflict detection).
5. `🔺️diff/🦀️.rs` (and `🟦️.ts` mirror): return `MutationOutcome` built from `diff_snapshot_from_helpers`. Reject invalid input with `MutationOutcome::fatal(code, message, target)`. Mirror in `🔺️diff/🟦️.ts`.
6. `↩️inverse/🦀️.rs` (and `🟦️.ts`): return the inverse `Generation3dMutation` list from the base.
7. `🦠️mutation/🟦️.ts`: TS payload type. `🧬️schema/🔣️.json`: JSON Schema with `$id` `https://json.schemas.assets.semio-tech.com/s/procedural/generation3d/mutation/<kind>/schema.json`, `mutation` as `const`, and `x-semio-ui` hints.
8. Aggregate `GMUT/🦀️.rs`: add the variant to `Generation3dMutation` (`:224`), its kind to `KINDS` (`:252`), and the `pub mod` glue block (the `//#region 🔖️NewLeaves` pattern).
9. Aggregate `GMUT/🟦️.ts`: add the import and the union member with `mutation: "<camelCase>"`.
10. Aggregate `GMUT/🔣️.json`: add the `$ref` to the new `schema.json` in `oneOf`.
11. If the wire needs it, extend `GMUT/🛰️.proto` and `GMUT/🕸️.graphql` (artifact-lane fields only, not per-leaf).
12. Text and binary codecs: add entries to `GEN/🚪️io/📝️text/🧬️mutations/🦀️.rs` and `GEN/🚪️io/💾️binary/🧬️mutations/🦀️.rs` (opcodes and tags, not inspected in depth).
13. Editor command (only if user-facing): add `GCMD/<emoji-name>/🦀️.rs` with a `handle` (signature in §4), `#[derive(... DslRecord)] #[dsl(keyword = "...")]`. Import the module at the top of `GEN/✏️editor/🦀️.rs` and append a row to the `app_commands!` block at `:118`.
14. Config, if the command changes view state: add a variant to `GEN/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:39`.

### 5.3 Authority and generation

15. If the leaf lives in a new domain-operation root or aggregate, update `LIB/🔣️taxonomy.json` (`mutationDomainOwners`, `mutationAggregateSources`) and run `bun nx run @semio-tech/dsl-derive-rs:generate` (writes `DSL/✨️derive/🔣️mutation-authority.json`). Flat leaves in an existing aggregate do not need this (see §2.3).
16. Do not hand-edit `DSL/✨️derive/🔣️mutation-authority.json`.

### 5.4 Tests and fixtures

17. Fixture quintet at `GEN/🧫️fixtures/🧬️mutations/<leaf>/<case>/` (§2.2): `🦠️mutation`, `🔺️diff`, `🎯️outcome`, `📸️snapshot/⬅️before`, `📸️snapshot/➡️after`. Write these from the intended semantics, not from the current implementation.
18. Per-leaf Rust test at `GMUT/<leaf>/🧪️tests/<case>/🦀️.rs` that `include_str!`s the quintet and asserts apply, diff, inverse, and outcome.
19. Language-agnostic case (required by the brief's TDD rule): extend the cross-language differential `GEN/🧪️tests/🧊️mutate-procedural-3d-1/🥒️.feature` and `🐍️.py`. The `.py` is an independent Python second implementation written from the snapshot schema and the quintets. Its narrative says "fourteen" kinds, which is stale (see §7). Add the new kind to its verb list, to the `KINDS` catalog check named in the `KINDS` doc comment (`GMUT/🦀️.rs:249-251`), and to the exhaustive case.
20. Gesture or editor leaves without a per-leaf test go into `GMUT/🧪️tests/🧪️gesture-leaves/🦀️.rs`.

### 5.5 Verification targets

These are the `project.json` targets of `GEN/📦️packages/🦀️rust/📋️project.json`. None was run in this audit.

- `bun nx run @semio-tech/procedural-generation3d-rs:test`
- `bun nx run @semio-tech/procedural-generation3d-rs:check`
- `bun nx run @semio-tech/procedural-generation3d-rs:verify-generation3d-document-io`
- `bun nx run @semio-tech/procedural-generation3d-rs:canonical-architecture`
- `bun nx run @semio-tech/procedural-generation3d-rs:semantic-wire-check`
- `bun nx run @semio-tech/procedural-generation3d-rs:verify-document-restoration-oracle`
- `bun nx run @semio-tech/procedural-generation3d-rs:verify-generation3d-widget-inputs`
- `bun nx run @semio-tech/dsl-derive-rs:check` (authority freshness), and the schema package `test mutation-leaf-registration`.

Per the brief, do not report a test as passing until it has been run.

---

## 6. Test conventions

### 6.1 Language-agnostic

- Gherkin `🥒️.feature` plus Python `🐍️.py` under the subset's `🧪️tests/` (`GEN/🧪️tests/`). Examples: `🧊️mutate-procedural-3d-1`, `🚪️io-procedural-3d-1`, `📐️example-geometry-3d-1`, `🎚️mutate-procedural-generation3d-1-any-editor-config`, `👥️mutate-procedural-generation3d-1-any-viewer-presence`.
- The mutation case is a cross-language differential: Rust and Python apply every typed mutation to the same committed quintets and must agree. Tags: `@oracle-procedural-3d-python-independent`, `@comparison-ordered-json-v1`. Its own header says no third-party library was used because none models this carrier.
- The fixture JSON itself is the language-agnostic oracle, consumed by Rust (`include_str!`), TypeScript (`🟦️.ts` tests), and Python.

### 6.2 Per-leaf coverage (generation3d)

- 20 fixture cases, one per enum leaf (`GEN/🧫️fixtures/🧬️mutations/`). The two descriptor-only folders have none.
- 14 leaves have a Rust test in `GMUT/<leaf>/🧪️tests/`.
- 6 leaves have no per-leaf test: `✋️drag-transforms`, `🎚️change-slider-value`, `🎛️change-widget-input`, `📏️scale-transforms`, `🔃️rotate-transforms`, `🚚️move-nodes`. The root test `GMUT/🧪️tests/🧪️gesture-leaves/🦀️.rs` exercises all six (its `🎚️change-slider-value`, `🎛️change-widget-input`, `📏️scale-transforms` and `🔃️rotate-transforms` paths are present in the file).
- Root `GMUT/🧪️tests/📄️document-restoration/` runs `bun test` and `tsc --strict` on a TS test.

### 6.3 Mutation laws and derive checks

- Generated law test per aggregate (`DSL/✨️derive/🦀️.rs:1214`).
- Shared law helpers in SPR (`SPR/🎮️command/🦀️.rs:930`, `:962`, `:979`, `:1035`, `:1091`): payload round trip, input schema, label, inverse rows.
- Compile-time checks emitted by the derive: `const _` assertions on `DESCRIPTORS` (`DSL/✨️derive/🦀️.rs:1104`). The kind-equals-kebab(variant) check and the approved-verb assertion are documented at `SPR/🎮️command/🦀️.rs:160` and `:177`.

---

## 7. Findings and risks

1. Count mismatch with the brief. The "24 mutations" is 22 leaf folders plus `🧪️tests` and `🧫️fixtures`. Only 20 are enum variants.
2. Two descriptor-only leaves: `👆️select-generation` (binaryTag 20) and `📝️change-generation-preview` (binaryTag 21). They have descriptors, `outcomeClasses`, and `requiredLanguageSurfaces`, but no `🦀️.rs`, no fixture, and no entry in the enum, the TS union, or the JSON `oneOf`. Either implement them or delete them. Their binary tags stay reserved.
3. Stale narrative. `GEN/🧪️tests/🧊️mutate-procedural-3d-1/🥒️.feature` and its `🐍️.py` both say "fourteen typed mutations"; the enum has 20.
4. Stale comment. The `GMUT/🦀️.rs` header refers to `glue.rs`, which does not exist in this tree.
5. Misleading folder names: `❌delete-widget` vs `🧹️delete-widget`, and `🔄️update`, `📍️move`, `🗑️delete`, `🔧️change`, `🏷️rename`, `🩹update-widget`, as listed in §3.
6. Coarse mesh edits. Mesh edits write a whole serialized mesh into `ChangeWidgetInput` (`GCMD/🎚️set-widget-input/🦀️.rs:136`), so history and conflict detection are per-widget, not per-field.
7. Intent loss in diff-derived commands. `remove-widget`, `delete-selection`, `add-widget`, `reorganize`, `node-graph-edit`, `edit-mesh-selection`, `set-active-example` and `import-document` emit ops by whole-snapshot diff. Any cascade appears as separate history rows. The feature narrative says the `delete-widget` leaf does not cascade, but the editor `remove-widget` path does cascade by diff: `DisconnectSynapse` is emitted for any synapse absent after (`GMUT/🦀️.rs:487-494`). Whether `FlowHost::remove_widget` removes synapses itself was not verified.
8. Process-wide write outside the mutation system: `set-contributions` (`GCMD/🧩️set-contributions/🦀️.rs`).
9. Text surface: all 20 leaves list `text` in `requiredLanguageSurfaces`, but `textOpcode` is null for all. The text codec exists at artifact level (`GEN/🚪️io/📝️text/🧬️mutations/🦀️.rs`), not per leaf. Not verified whether that satisfies the requirement.
10. Schema catalog writer not located. `LIB/🔣️schema-catalog.json` is listed as generated, but I did not find which command writes it. The `schema` generator in `🧬️schema/🏷️entity-kinds/` writes only the entity-kinds catalog.

---

## 8. CAD example (layout)

`CMUT` has 21 leaf folders with `binaryTag` 0 through 20, and a root `🔣️.json` `oneOf` with 21 `$ref`s. Root `🦀️.rs:60` carries `#[mutations(snapshot = CadSnapshot, diff = CadDiff, schema = "cad.cad")]`.

Per-leaf files (checked on `➕create-node`): `🔣️.json`, `🦀️.rs`, `🧬️schema/🔣️.json`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, `🧪️tests/<case>/🦀️.rs`. Unlike generation3d, there is no per-leaf `🦠️mutation/🟦️.ts` or diff/inverse `🟦️.ts`. TypeScript appears only at the root (`CMUT/🟦️.ts`, 7967 bytes). Root also has `🛰️.proto`, `🔗️.graphql`, `🔣️.json`, `🦀️.rs`, and `🧪️tests`, `🧫️fixtures/🪪️owned-target`.

Descriptor example, `CMUT/➕create-node/🔣️.json`: `semanticKind` `create-node`, `binaryTag` 10. Folder `🏛️create-structure-classic` has kind `create-structure-classic-model`, so the folder name is not the kind.

---

## 9. Not verified

- No tests, builds, or `nx` targets were run. Test statements are from reading the code only.
- `mesh_operation_rows` (used by knife and edit-mesh) and the `FlowHost` implementation of `remove_widget` were not read in full.
- Writers for `LIB/🔣️schema-catalog.json` and for fixture encodings (`fixtures generate`) were not located.
- The `new mutation` router (the repo script that exposes `CleanMechanismNewScript`) was not located by name.
- The binary-tag uniqueness check was not located.
