# Snapshot Old-Shape Ports: Writer, VCS, Forms, GIS, Flow (2026-11-24)

Tree edited: `/Users/ueli/.sinc/play` (snapshot). **Nothing was compiled.** No cargo or rustc ran. Every change was checked by reading the snapshot's framework trait definitions. The release build is the only verifier.

Scope: `✏️s/🔌️plugins/{✒️writer,🌿️vcs,📋️forms,🌍️gis,🌊️flow}`. Flow's `📐️brep` extension was excluded. No framework, derive or macro files were touched. No files were renamed. No dependencies were added, so **no Cargo.lock edits** were made.

## Ports

| # | Plugin | Trait / impl | Model followed | File |
|---|---|---|---|---|
| 1 | writer | `ArtifactStoreOneItemPreparationFactory` for `WriterArtifactPreparationFactory`: added `begin_copy_byte_demand`, delegating to `inner`. | Framework `operation-wire` wrapper (`self.factory.begin_copy_byte_demand`) | `/Users/ueli/.sinc/play/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️command-admission-emission-and-retained-work/🦀️.rs` |
| 2 | writer | `ErasedSnapshotRetirement` for `WriterPreparation`: `close_step(_body, effect) -> OriginalRetirementPoll`, settled through `RetirementPollExit::settle`. Removed the unused `RetainedCloneGrant` import. | Framework `TransferTask` / `ArtifactStoreWireRetirement` | `/Users/ueli/.sinc/play/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/📢️publication/🦀️.rs` |
| 3 | writer | `ArtifactEphemeralPreparationTask::advance` (same impl, **missed by the sweep**): gained the new `receiving: &mut NativeDecodeControl<'_>` parameter, unused. | Framework `TransferTask::advance` | same file as #2 |
| 4 | writer | `ArtifactEnvelopeSnapshotFieldAuthority` / `ArtifactEnvelopeMutationFieldAuthority` / `ArtifactEnvelopeSprConflictAuthority` (**missed by the sweep**). `close_step(effect) -> ArtifactEnvelopeClosePoll` is now a ladder over `ArtifactEnvelopeCloseExit`, settled by `artifact_envelope_close_settle`. Children go through `artifact_retirement_box_close_step(.., maximum_copy_bytes, effect)` + `artifact_envelope_close_resume_with` and `artifact_retirement_admit_owned(.., effect).result` + `with_native`. The conflict authority's depth demand is now `usize::from(!terminal)`, as in gis. | gis `🔡️owned-field-decode` + `📇️owned-field-catalog` (same helpers used by raster and process3d) | `/Users/ueli/.sinc/play/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🔨️modules/🏠️host/🧰️owned/🦀️.rs` |
| 5 | vcs | `ArtifactStoreOneItemPreparationFactory` for `VcsOneItemPreparationFactory`: added `begin_copy_byte_demand` (request header + preparation size quote); added the `M: ColdMutation<P>` bound. | space-home config / framework `snapshot-clone` | `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📬️store-preparation-and-proofs/🦀️.rs` |
| 6 | vcs | `ArtifactStoreOneItemPreparation` for `VcsOneItemPreparation` (**missed by the sweep**). Old `advance(grant)` and `close_step(grant)` are replaced by `advance(body, grant, receiving) -> Result<_, OriginalReceivingPrimary>` with `prepare_work`/`commit_work`, plus `last_turn_work` (new `last_work` field), the four `*_receiving_*` methods and `close_step(effect)` via `OneItemOwners::close_step`. Semantic work moved into an inherent `advance_step`. `diff`/`inverse` now come from `ColdMutation` (the `Mutation` trait no longer has them). | gis `📬️one-item-preparation` (`Gis2dOneItemPreparation`) | same file as #5 |
| 7 | forms | `ArtifactStoreOneItemPreparationFactory` for `FormsStorePreparationFactory`: added `begin_copy_byte_demand`; added the `M: ColdMutation<P>` bound. | space-home config | `/Users/ueli/.sinc/play/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📬️store-preparation/🦀️.rs` |
| 8 | forms | `ArtifactStoreOneItemPreparation` for `FormsStorePreparation` (**missed by the sweep**): same port as #6. `prepare_forms_store_mutation` gained `P: Clone + M: ColdMutation<P>`, which `apply_diff_cold` and `ColdMutation::inverse` already needed. | gis `Gis2dOneItemPreparation` | same file as #7 |
| 9 | forms | `RetireOwned` for `TryValueContent` and `FormsTryValues`. The old `retirement()` built a `sequence` of `deferred` values plus `SharedControlledRetirement::lease`, which no longer exists. Now: `type RetirementCursor = retirement_fields_type!(..)`, and `retirement_seed` destructures and uses `retirement_fields_seed!` with `ArcLease::new(arc)` for the shared `Arc` fields. The default `retirement_birth_bytes` applies, with no `unsafe`. `TryValueContent` became `pub struct` (fields stay private), because it now appears in `FormsTryValues`' public associated cursor type (E0446). | puzzle 2d/3d/5d play snapshots (`retirement_fields_type!` + `ArcLease`) | `/Users/ueli/.sinc/play/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs` |

## Checked and Already Current
- **gis**: gismap `ArtifactOwnedValueRetirementFactory` ×2 and `SnapshotRetirementFactory` ×1 (`♻️retirement-and-clone`). The sweep listed these as old-shape, but they already delegate to `OwnedValueRetirementFactory` / `SharedValueRetirementFactory` with the new signatures. Also current:
  - `Gis2dOneItemPreparationFactory` + `Gis2dOneItemPreparation`
  - envelope snapshot/mutation/conflict authorities and the catalog
  - gisterrain `NativeMapValidation`
- **flow** (library): all current.
  - `RetireOwned` ×5: `FlowHostEffectPayload`, `FlowComposedScene`, `FlowWorkingScene`, `FlowSnapshot`, `FlowMutation`
  - `SnapshotRetirementFactory` ×3: presence, snapshot, mutation
  - `ArtifactOwnedValueRetirementFactory` ×2
- **writer**: also current:
  - `ArtifactOriginalTypingFoldFactory` / `ArtifactOriginalTypingFoldProducer`
  - `ArtifactCommandWork::step`
  - `ArtifactEnvelopeOwnedFieldCatalog`
- **Totals checked in these 5 plugins** (library code, brep excluded): 27 retirement/store-family impls.
  - 12 ported (the 9 rows above; rows 4, 6 and 8 cover several impls each).
  - 15 found current.
- **Not swept exhaustively**: other framework traits (ToolJobFactory, WindowConfigOwner, ArtifactPresenceSnapshot, MutationKind, and others). Only their retained/retirement signatures were spot-checked.

## Test-Only, Deliberately Left Alone
These are under `#[cfg(test)]`, outside the release library:
- flow `…/🪆️composed-source/🧵️assembly/🧪️scene.rs`: `OriginalModelLease`, old `fn retirement`. Included by `#[cfg(test)] #[path="🧪️scene.rs"] mod scene_laws;`.
- writer `…/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️unit/🦀️.rs`: `UnusedWriterEditRetirementFactory`.
- writer `close_retirement` in `🧰️owned/🦀️.rs`: calls the old `ErasedSnapshotRetirement::close_step(grant)`.
- vcs `✏️editor/🧪️tests/🔬️unit`: calls the old preparation shape.

## Unresolved / Risks
- No missing framework API was found. Every port uses items that exist in the snapshot framework.
- The ported plugins have never been in a build log. These ports remove the known old-shape impls, but more drift elsewhere in these crates is likely (see the earlier sweep's "stale throughout" note).
- Possible lints that would only matter under deny-warnings:
  - #4: the original `-admission` diagnostic code was kept for the snapshot `value` branch.
  - `FormsStorePreparation.prefix` stays unused, as before.
- #9 relies on `ArcLease<T>: RetireOwned` needing `T: RetireOwned + Sync`. `Vec<SharedUtf8>` and `BTreeMap<String, TryValueContent>` satisfy this.
