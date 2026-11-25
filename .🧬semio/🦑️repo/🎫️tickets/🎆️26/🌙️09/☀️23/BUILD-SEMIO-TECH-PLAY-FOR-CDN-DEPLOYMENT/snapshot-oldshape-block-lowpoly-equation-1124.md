# Snapshot Old-Shape Ports: Block, Lowpoly, Equation (2026-11-24)

Tree edited: `/Users/ueli/.sinc/play` (`P` = `/Users/ueli/.sinc/play/✏️s/🔌️plugins`). Scope: `🧱️block` (2d, 3d, 5d), `💠️lowpoly`, `➗️mathematical` (equation).
**Nothing was compiled.** No cargo or rustc was run. Every port was written by comparing the impl against the trait definition in the SNAPSHOT framework and against already-ported models. The release build is the verifier. No framework, derive, macro or `AGENTS.md` file was touched, no file was renamed, and no dependency or Cargo.lock was changed.

## Ported impls

### 1. `RetireOwned for Block3dSnapshot` (manual, old `fn retirement` shape)
- Stale: `fn retirement(self) -> Box<dyn RetirementCursor>` built from `sequence(vec![deferred(..)])`, plus an overridden `retirement_birth_bytes`. It also called `SharedControlledRetirement::lease`, which does not exist in the snapshot (`lease` exists only on `ArcLeaseRetirement`).
- Model: trinity jack `JackSnapshot` / wires `WiresSnapshot` / flow `FlowSnapshot`. A `#[derive(semio_framework_value::RetireOwned)] #[value(send_only)] #[doc(hidden)]` sources struct takes every field plus `Option<ArcLease<SemioKitSnapshot>>` from `catalog.take_local_owner`. `type RetirementCursor = <Block3dSnapshotSources as RetireOwned>::RetirementCursor` and `retirement_seed` destructures and seeds the sources. The default `retirement_birth_bytes` is used.
- File: `P/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

### 2. `ArtifactStoreOneItemPreparationFactory` and `ArtifactStoreOneItemPreparation` (block 2d, 3d ×2, 5d, lowpoly ×2)
- Stale factory: `begin_copy_byte_demand` was missing. Added it as in space-home config, generation2d and raster: `begin_demand` check, then `size_of::<Request<P,M,M>>() + size_of::<Preparation>()`.
- Stale preparation, which the 11-24 sweep **missed**. The old preparation shape had:
  - `advance(grant) -> Result<_, ValueError>`
  - `close_step(grant) -> Result<RetainedCloneStep, _>`
  - no `last_turn_work`
  - no `has_pending_receiving_recovery`, `pending_receiving_demand`, `pending_receiving_recovery` or `finish_receiving_recovery`
- Model: generation2d `📬️store-preparation` (same `OneItemOwners` design):
  - The old body becomes an inherent `advance_step(grant)` that sets `last_work`.
  - The trait `advance(_body, grant, receiving)` brackets it with `receiving.prepare_work(1)` / `commit_work(last_work)`.
  - The four receiving-recovery methods return none/empty.
  - `close_step(effect) -> OriginalRetirementPoll` delegates to `OneItemOwners::close_step(effect)`.
  - A `last_work: u64` field was added.
- The apply step called `mutation.inverse(..)` and `mutation.diff(..)` through `protocol::Mutation`, which no longer has those methods. It now goes through `protocol::ColdMutation::{inverse, diff}`. The delta is retired with `MutationDiff::retire_cold`, and on an apply failure the inverse rows are retired with `Mutation::retire_cold` (generation2d pattern). The same fix was applied to lowpoly's `prepare_lowpoly_artifact` and `prepare_lowpoly_config`. Those also named `ColdMutation` without importing it.
- Files:
  - `P/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  - `P/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📬️store-preparation/🦀️.rs`
  - `P/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  - `P/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📬️store-preparation/🦀️.rs`

### 3. Equation `EquationStorePreparationFactory` / `EquationStorePreparation` (generic, hand-rolled)
- Stale:
  - `begin_copy_byte_demand` was missing.
  - The whole preparation used the old shape: old `advance` and old grant-based `close_step`.
  - A hand-rolled `ManuallyDrop` owner ladder with its own `Drop` assert.
- It was functionally identical to the framework's cold factory: generic `P, M`, document lane, `for_leaf(.., ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)` preflight, then inverse, apply and prepare.
- Model: delegate to the framework's ready-made `store::ColdMutationApplyFactory` (as used by `cold_mutation_apply_preparation_factory` in jack, rewriting, remodeling and the stdio editors). Its unbounded preflight is byte-identical to the old one. The file is kept, because the publication-authority fixture lists it as mounted. It now holds only `pub type EquationStorePreparationFactory = store::ColdMutationApplyFactory<EquationSnapshot, EquationMutation>;`. The callers drop the generic arguments.
- Files:
  - `P/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📬️store-preparation/🦀️.rs`
  - `P/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎛️play-app-editor-and-constructor/🦀️.rs`
  - `P/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (test only, trivial)

### 4. `RetireOwned` derives over `Arc` fields (lowpoly session), not in the sweep
- Stale: `LowpolyTransientState { mesh_workspace: Arc<BTreeMap<..>> , .. }` and `LowpolyTransient { state: Arc<LowpolyTransientState> }` derived `RetireOwned`. The snapshot framework has no `RetireOwned for Arc<T>`; only `ArcLease` / `SharedOriginalStorage` have one. This matches the earlier "`Arc<…>: RetireOwned`" note for lowpoly view.
- Model: wires `WiresSnapshot`. A hand-written `RetireOwned` seeds `ArcLease::new(arc)`. The state uses a tuple `(ArcLease<BTreeMap<..>>, BTreeMap<String, LowpolyPaintGesture>)`, and the transient uses `ArcLease<LowpolyTransientState>`. `LowpolyTransientState` went from `pub(crate)` to `pub` + `#[doc(hidden)]`, because the public impl's associated cursor type names it. Its fields stay private.
- File: `P/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs`

### 5. `InferredField::Value` retirement bound (equation inferences), not in the sweep
- Stale: the snapshot's `InferredField<P>::Value: RetireOwned + ..`, but `EquationRoot` did not implement it. This matches the earlier "`EquationRoot: RetireOwned`" note.
- Model: equation snapshot structs (`semio_framework_value::RetireOwned` derive beside `ToValue`). `f64` is a retirement leaf.
- File: `P/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌱roots/🦀️.rs`

## Checked and found current (library code, these three plugins)
| Trait / site | Count | Note |
|---|---|---|
| `ArtifactStoreBatchItemAuthority`, `ErasedSnapshotRetirement`, `SnapshotRetirementFactory`, `ArtifactOwnedValueRetirementFactory` impls | 0 | none exist in scope |
| `artifact_retire_struct!` (framework macro) | 26 | block document fields 16, block 2d/3d/5d diff patches 3/2/4, block 3d window-view patch 1 |
| `ArtifactPresenceSnapshot` | 4 | `decode_presence_native` matches |
| `ArtifactSqliteSnapshotHost` | 5 | output-slot signatures (ported earlier) match |
| `ArtifactStoreSnapshot` | 15 | all methods optional, signatures match |
| `WindowTransientOwner` (block 3d) | 1 | `owners_birth_demands`/`build_owners`, `ArtifactEphemeralTransferPreparationFactory::new` with dyn factories |
| `WindowConfigOwner` (equation) / `ConfigApplyMutation` | 1 / 1 | match |
| `ToolJobFactory` / `ArtifactOwnedToolJobFactory` | 5 / 6 | admission via `retained_command_factory_admission!` (both macro arms exist) |
| `ArtifactCommandWork` | 3 | signatures match |
| `ArtifactEditor`/viewer `build_*` retirement/disposer/preparation hooks | all | names and return types exist in the snapshot trait; helpers `bounded_transient_root_retirement_factory`, `no_*_retirement_factory` exist |
| `RetireOwned` derives over `Arc`/`Rc`/`Box<dyn>` fields | 0 left | after port 4 |

## Unresolved
None of the store/retirement items. Not checked: non-retirement families the earlier notes flagged as unconfirmed, outside this scope:
- `LowpolyMutation: OpBinaryReceiving`
- equation inferences' orphan diff impls
- DSL derives

## Open risks
- Nothing compiled.
- `LowpolyTransientState` visibility widened to `pub` so the public `RetireOwned` impl can name its cursor.
- `Block3dSnapshotSources` relies on the `RetireOwned` derive's `send_only` option, as jack and flow use it.
- `protocol::Mutation::retire_cold` / `protocol::MutationDiff::retire_cold` are passed as paths and rely on inference of the trait's `P` from the single impl.

## Block 3d io root

Fixes `semio-s-artifact-block-3d-io`, and the identical defect in `-2d-io` and `-5d-io` (same log, lines 295321 and 295586). Nothing was compiled.

- **Cause:** the 10/15 crate split (live commit `793f63533f2`) deleted the io root's `derived_composition`, `derived_construction` and `derived_analysis` modules (`BlockNdComposerComposition`, `BlockNdBuilderConstruction`, `BlockNdParts`, `BlockNdAnalyzerAnalysis`) and moved them nowhere. It kept the `derive_artifact_facets!` call that names them, and it kept `use crate::{BlockNdMutation, BlockNdSnapshot, BLOCKND_DIALECT, BLOCK_ND_SCHEMA}`. Those names now live in sibling crates, so the io crate root no longer has them. The live repo has the same root, byte for byte. Today's "Block io orphan move" touched only `💾️binary/🦀️.rs` and `📝️text/🦀️.rs`, never the root, and it did not cause this. 2d and 5d are not a valid model: they fail with the same four errors.
- **Owners** (the io crate already depends on all of them):
  - `BlockNdSnapshot`, `BLOCKND_DIALECT` and `BLOCK_ND_SCHEMA` come from `semio-s-artifact-block-Nd-model`.
  - `BlockNdMutation` comes from `-mutations`.
  - `BlockNdDiff` comes from `-diff`.
- **Fix,** in each `…/🧱️block/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`:
  - `io()` imports the four names from the model and mutations crates.
  - The three facet modules are restored before `derive_artifact_facets!` in the process3d shape. They use the cold APIs: `protocol::ColdMutation::diff` and `protocol::apply_diff_cold`. They are typed through `semio_plugin::{ArtifactBuilder, ArtifactAnalysis, ArtifactComposition}` and `semio_plugin::io::{Analysis, AnalyzeSource, ComposeSource, ComposeError, Composition, Confidence}`.
  - The dialect is the model crate's `BLOCKND_DIALECT` instead of a duplicate literal.
  - Composition stays native-only (`reads() == [BLOCKND_DIALECT]`), as before the split.
- **Rest of each io crate re-read:**
  - `💾️binary`/`📝️text` re-export the model, mutations and diff leaves. The `💡️inferences` leaves are const-only.
  - Tests are `cfg(test)`.
  - No other unresolved `crate::` names were found.
- **Cargo.toml / Cargo.lock:** unchanged. Every needed dependency was already declared.
- **Open risks:**
  - Nothing was compiled.
  - The restored builder assumes the `#[mutations(reference_diff = BlockNdDiff)]` derive implements `ColdMutation<BlockNdSnapshot, ReferenceDiff = BlockNdDiff>`.
  - The 2d model types `BLOCK2D_DIALECT` as `semio_framework_artifact_reference::Dialect`, which is taken to be the same type as `semio_plugin::app::Dialect` (it is re-exported).
