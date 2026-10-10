# r11-store-p3 execution report

Gate label `r11-store-p3`, logs `🗑️generated/r11-store-p3/` (`c0..cN.txt` human logs, `extract.py` root-file error extractor). File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, chunk = everything after the `impl PluginApp for VcsArtifactApp` ladder (`PluginProgram`/`Plugin`, window kits, `ArtifactEditor`, `ArtifactViewer`, codec apply/replay helpers, `NaturalFileImportJob`, `EditorApp`, `ViewerApp`, builders, `plugin_runtime`, extension module, exports).

## Decisions (approved by r11-store)
- `ArtifactEditor`/`ArtifactViewer`/`EditorApp`/`ViewerApp`: `build_{document,config,draft}_store_owners() -> Option<Result<store::DocumentStoreOwners<..>, ValueError>>`; defaults `Some(store::funded_bounded_artifact_store_owners::<P, M>())`; consumers install with `store::install_unscheduled_catalog`.
- Mounted-job hooks (grant currency): `mounted_job_maintenance_demands(instance, body)`, `mounted_job_maintenance_step(instance, grant) -> Result<PluginLifecycleStep, Fault>`, `mounted_job_close_demands(instance, body)`, `mounted_job_close_step(instance, grant)`; defaults `Complete(default)`.
- `ArtifactReservedJob` becomes a marker over `InteractiveJob` (p2 range); `NaturalFileDecodeCursor` takes `close_demands(body)` + `close_step(grant) -> PluginLifecycleStep` (p2 range); `Emit::close_child_demands(body)` + `Emit::close_child_one(grant) -> Result<Option<PluginLifecycleStep>, Fault>` (p2 range).

## Changes
(see end of file; filled when the gate is clean)
All paths in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (root file).

- `ArtifactEditor` / `ArtifactViewer` / `EditorApp` / `ViewerApp`: mounted-job hooks ported to `(instance_id, RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>` plus new `mounted_job_{close,maintenance}_demands(instance_id, body) -> Result<RetirementDemand, ValueError>` (defaults `Complete(default)` / `RetirementDemand::default()`); `build_{document,config,draft}_store_owners` now `Option<Result<DocumentStoreOwners, ValueError>>` defaulting to `store::funded_bounded_artifact_store_owners`. Same shapes applied (with r11-store approval) to `ArtifactApp` (~14772/14883) and the cfg(test) `BoundedViewerFixture` forwards (~9200/9252).
- `artifact_app_apply_ops` / `artifact_app_replay_envelopes`: funded catalog via `store::install_unscheduled_catalog` / `funded_bounded_artifact_store_owners`, identity via `with_authoring_identity!`, close via `ArtifactStore::close_owned_unscheduled` (the hand-rolled `SnapshotRetirementStep` loop is deleted).
- `NaturalFileImportJob`: whole close path rewritten in the grant currency. `InteractiveJob::close_step(grant)` + the four `next_close_*_demand` quotes derive from one `close_demand(body)` (box -> controlled decoder cursor -> pending bytes -> rejection emit -> emit -> media -> port -> completion); originals are moved into typed pending slots (`seal`, free, no grant) and admitted through `store::artifact_retirement_admit_owned` (hand back on refusal), boxed children close via `artifact_retirement_box_close_step`; `impl ArtifactReservedJob {}` is the marker. No `owned_retirement`, no `PluginCloseStep`.
- `plugin_runtime`: `RuntimeLiveCleanupJob` / `RuntimeCloseCleanupJob` `close_step(grant)` + four demand methods (release of the weak cell/state = `copied_items:1`, depth 1 while held); pump/session/rejected teardown calls use `runtime_lifecycle_grant()`; dead `RUNTIME_CLOSE_ITEMS_PER_STEP` deleted.
- Extension module: `extension_close_step(grant) -> PluginLifecycleStep`, `extension_retirement_demands(body) -> RetirementDemand` (replaces `extension_next_close_byte_demand`), `extension_retirement_turn(grant)`, `extension_dispose_cold` via `cold_grant`; export list updated.
- `Emit::close_child_demands` flipped to `&self` (p2's code, one-word change); removed an unused `use crate::dsl::ArtifactDsl`.

## Result
`cargo check -p semio-framework-plugin --lib` (human log `🗑️generated/r11-store-p3/c23.txt`): 0 errors (borrowck included). `--lib --tests` (`t1.txt`): 1503 errors, all in test files (`🧪️tests/**`, window/interaction test dirs), none in the root file; those belong to the test migrators.

## Open items
- `PluginCloseStep` (definition ~15834 + export in `plugin_app_close_prelude` tail) has no remaining user in the lib; the tests and external artifacts (writer, wfc, generation2d, presentation, sequence, layout, fem, BIM editor/viewer `mounted_job_*`, draw, ...) still use it/the old 3-arg `mounted_job_*` and old `ArtifactReservedJob::close_step`. Delete it once the plugin tests are migrated.
- r11-store ladder must call `A::mounted_job_close_demands/step` and `mounted_job_maintenance_demands/step` (not wired yet when I finished).
- Callers outside my chunk must follow the extension API change: `⚛️reactor/🔄️turn` (`extension_retirement_turn(grant)`) and `🧪️tests/🔬️extension-retirement` (`extension_next_close_byte_demand` gone).
- Nothing was run (no tests executed); only compile gates.
