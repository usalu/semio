STATUS: PARTIAL. cad native check (--tests) and cad-editor tests GREEN (cad-n2, cad-t7); external unit targets (cad_editor_unit 135 errors, cad_panels_artifact 4) need production visibility + API-drift port (see cad-t9 notes); catalogue-add-box-undo-redo law UNVERIFIED.

## cad-n1 (`C/🗑️generated/1102/cad-n1.txt`, rc 1)
No compile error in any cad crate. Died on the 60-minute owned-command deadline ("cargo:check stopped: timeout 3593031ms") while still compiling stdio/framework dependencies (last: stdio-contract-editing-patch). Retrying with a smaller batch through the slot semaphore.

## Test coverage found (not yet run)
- `✏️editor/🧪️tests/🔬️unit/⏪️history-and-convergence/🦀️.rs`: `add_object_yields_a_new_rendered_instance` (box, shape window, instance in lane), `undo_redo_round_trips_added_node_*`, `object_gestures_undo_back_to_the_demo_document`.
- `⚙️operations-and-file-effects`: `add_object_through_wrapper_grows_the_composed_pane` (one insert-element on the pane child, parent unmoved).
- Gap: no single test chaining catalogue add (box) -> other panes unchanged -> document row -> undo -> redo. To be added under `🧪️tests/` after the check.

## cad-n2 (`C/🗑️generated/1102/cad-n2.txt`, rc 0, native check --tests)
GREEN for the native compile of cad-editor with test targets and its cad deps (interaction, transform, kit, modes, commands, panels). Added test `catalogue_add_box_grows_only_the_shape_pane_with_one_row_and_undoes_and_redoes` (included in the compile? it was added before the compile reached editor tests: UNVERIFIED until cad-t3). cad-t3 (native test, cad-editor) started via `1102-tools/cad/tst.sh`.

## cad-t3 (`C/🗑️generated/1102/cad-t3.txt`, rc 1, native `cargo test -p cad-editor`)
No cad crate compiled yet: failed in `semio-framework-job-context` (8 x E0433). Root cause: `🧰️framework/🔨️modules/🧵️job/⏱️context/…/../../../📬️outcome/⏸️continuation/🎟️origin/🦀️.rs` (lines 81, 102) qualifies `semio_framework_value::{RetainedWorkOutcome,ValueError}` but `semio-framework-value` is only a dev-dependency of job-context; both names are already imported at line 4 from `semio_framework_value_retained_clone`. Fix: use the bare names on those two lines. File was edited 14:37 by a peer (runtime lane); retrying after 10 minutes of quiet.

## cad-t4 (`cad-t4.txt`, rc 1)
job-context now compiles (peer fixed 14:45). New first failure: `semio-framework-plugin-retained-command`, `🔌️plugin/🧵️retained-command/🔄️full-operation/📥️emission/🦀️.rs:46` missing `;` after `...absent"))?` (fused `if`). Fixed in the window (one-char edit, file untouched since 14:37). cad-t5 reruns the editor tests.

## cad-t5 (`cad-t5.txt`, rc 1)
Retained-command fix works; the build reached cad. First cad failure: `semio-s-artifact-cad-cad-inferences` E0432 `semio_s_artifact_cad_cad_model::geometry::CadObjectName` unresolved. This is the cadi18n B8 object-name change landing half-way (cad files reserved to B8); no cad edits by this lane. Waiting for `win-B8-done`, then cad-t6 (editor tests incl. the new catalogue/undo/redo test).

## cad-t6 (rc 1)
After win-B8-done: failed in framework `value/🛬️decode/🧳️owned/🧩️record/🔎️schema/🦀️.rs` (E0382 moved `error` in with_retained_progress, lines 33/39/42), a peer edit that was being fixed at 17:44 (no match remaining). Rerun as cad-t7.

## cad-t7 (`cad-t7.txt`, rc 1) and PRODUCTION FAULT
STATUS line summary: native compile of all cad crates and cad-editor test targets GREEN after B8; run ended on the 60-minute "Original child phase deadline exhausted" AFTER the test binaries (not a test failure). Passed: `example::primary_asset_is_nonempty`, `original_host_configuration_preserves_cad_raw_config_lane`.
RETRACTED: the earlier 'unmounted unit tree' claim was wrong. The 13-module tree IS mounted as the integration test target `cad_editor_unit` of crate `semio-s-artifact-cad-cad` (`📦️packages/🦀️rust/Cargo.toml:19`), alongside `cad_transform_tool`, `cad_panels_inspection`, `cad_panels_artifact`, `cad_panels_catalogue`. My cad-t3..t7 used `-p semio-s-artifact-cad-cad-editor` and so never ran them. The bogus patch was deleted. Next: cad-t8 = `-p semio-s-artifact-cad-cad` (all five targets).

## Unmounted test survey (systemic gap), script `C/1102-tools/cad/unmounted.py <dir>` (path attrs, Cargo [[test]] paths, include! resolved) and `unmounted2.sh`
- cad: nothing real unmounted. `member-authority`, `construct-query-unit`, `derive-transformation-unit` are `include!`d; the editor `🔬️unit` tree is the `cad_editor_unit` target of `semio-s-artifact-cad-cad`. `📐️mutate-cad-1` is a conformance case dir (oracle-driven), not a Rust target.
- Case dirs named `mutate-*`, `solves-*`, `io-*`, `simulate-*`, `export-*` (procedural, puzzle, gis, fem, energy, sourcing, process) were skipped as fixture/conformance cases, not audited one by one. fem `round-trips-the-committed-document` is referenced by comment only: not audited.
- GENUINE gaps (referenced nowhere by path/include/Cargo): puzzle3d editor `🧪️tests/🤖️agent-lane/🦀️.rs` and `🧪️tests/🧵️retained-wiring/🦀️.rs`. Mount patch: `C/1102-tools/cad/patches/mount-puzzle3d-unmounted-tests.md` (test-file edit, puzzle lane).
- process3d editor `🧪️tests/🪧️pane-smoke-root/🦀️.rs`: no Cargo [[test]]; it only re-mounts `../🔬️unit/🪧️pane-smoke`, which `🔬️unit/🦀️.rs:89` already mounts, so it is a redundant dead root (delete it or add a [[test]]; not needed).
- Not checked: gis, fem3d, energy, procedural, sourcing beyond the script's output (script found no non-case dirs there except procedural `flow-eval-resolve/🧪️tests/🔬️unit` and `io-procedural-3d-1`, both unaudited).

## cad-t8 (`cad-t8.txt`, rc 1) -- real test-side staleness (first run of the `semio-s-artifact-cad-cad` targets)
Test edits made (all under `🧪️tests/`): `📐️cad/🧪️tests/🚀️activation/🦀️.rs` imports `CAD_DIALECT`/`artifact_kind` from `cad_model::snapshot`; `✏️editor/🧪️tests/🧰️support/🦀️.rs` imports `CadDispatchCtx, CadInteractionSnapshot, CadPlayRuntime, CadPlayView, cad_runtime_from` from `cad_kit` and `CAD_DIALECT`, `command_from_action` goes through `<CadPlayApp as ArtifactEditor>::command_from_action` (the free fn is `pub(super)`), `config_after` applies diffs with `protocol::apply_diff_cold`; transform tool unit test: `prepare_child_one(grant)`, tuple variants `Ready(_)/Pending(_)/Refused(fault,_)`, `AppOperationContext.retained`. cad-t9 reruns. cad_editor_unit and cad_panels_artifact had not been reached yet.

## W3 changes (window): editor unit tests mounted inside the crate
cad-t9 (rc 1): `cad_editor_unit` 135 errors, `cad_panels_artifact` 4; the unit tree used private editor items (`pub(super)`/`pub(crate)`: CAD_RETAINED_TOOL_IDS, cad_interaction_scope, object_tree_item, CadRetainedCommandJobFactory, CadMediaWork ...) and cannot work as an external integration target. Fix (replaces the earlier retracted idea, now correct): mount it as `#[cfg(test)] mod unit` in `✏️editor/🦀️.rs` (with `extern crate self as semio_s_artifact_cad_cad_editor;`), unit root `use super::*;`, remove `[[test]] cad_editor_unit` from `📦️packages/🦀️rust/Cargo.toml` (cad-cad) and add the dev-deps the tree needs to the editor `Cargo.toml` (stdio-semio-base, stdio-semio-brep, cad-mut-nodes, cad-typology, ui-scene, ui-declaration, sqlite-snapshot testing backing). Manifest edit: coordinator must run `dependencies lock`. Run W-cad-1: `-p semio-s-artifact-cad-cad-editor --lib`.

## W3: puzzle3d mount applied
Appended the two `#[path]` mods (agent-lane, retained-wiring) to puzzle3d editor `🧪️tests/🔬️unit/🦀️.rs` (target `editor-surface`). Compile: W-puzzle-1 (`--test editor-surface`), see log.

## W-cad-1 (rc 1): in-crate mount REVERTED
Source admission refused it: editor crate byte budget exceeded (`✏️editor/🧪️tests/🔬️unit/⏪️history-and-convergence/🦀️.rs` 23102 B > 9121 B remaining). No limit raise. Mount, extern-crate-self, editor dev-deps and unit-root glob are all restored; `cad_editor_unit` is again a `[[test]]` of `semio-s-artifact-cad-cad`. Tree state equals cad-t9 plus the test-side edits listed under cad-t8.
## Remaining port for `cad_editor_unit` / `cad_panels_artifact` (135 + 4 errors, cad-t9.txt)
Needs PRODUCTION visibility (tests are external targets): make `pub` and re-export from the editor root under a `testkit` feature (enabled by the cad-cad dev-dependency): `CAD_RETAINED_TOOL_IDS`, `CAD_CONFIG_STORE_MAXIMUM_BYTES`, `CAD_RETAINED_RAW_BYTES`, `CAD_RETAINED_COMMAND_SCHEMA`, `CAD_RETAINED_PUBLICATION_CONTRACTS`, `CadRetainedCommandJobFactory` (`🧵️retained-command-tools`), `CadMediaWork` (`📤️media-export-job`), `admit_cad_config_mutation`, `prepare_cad_config` (`🔧️config-store-preparation`), `cad_interaction_scope`, `CAD_WORLD_BODY_KEYS` (`🪟️editor-and-interaction-scope`); `object_tree_item` (panels `🗿️artifact`, pub(crate)); `align_mesh_to_host_snapshot_centroid` (inferences scene-compute, pub(crate)). Items already pub in `cad_kit` (`CAD_DISLOCATE_UTILITY_ID`, `CAD_INTERACTION_DOMAIN`, `make_object_for_typology`, `engagement_submit_entries`, `CAD_FALLBACK_MESH_KIND`) only need test imports. Plus real API drift in tests: `semio_framework_job::close_with_self_funded_context` (3 uses, removed), `ArtifactStore::close_owned_byte_demand` (2), `CadFixtureApp: PluginApp/PluginAppDelegate` unsatisfied (9), `*mut OriginalFixtureCaller` not Send (3), `AppOperationContext.retained` (2 more), `semio_framework_job_payload` not a cad-cad dev-dep, E0308/E0061 arity (about 10). Not done in W3.

## W-puzzle-1 (`W-puzzle-1.txt`, rc 1, `--test editor-surface`)
12 errors, none in the newly mounted `🤖️agent-lane` / `🧵️retained-wiring` (they were not reported; rustc stopped at earlier type errors, so their compile is UNVERIFIED). All errors are pre-existing drift in the already mounted `🔬️unit/🧮️extent-and-step-laws/🦀️.rs` (E0283 `work.extent(..., None)` type annotation, E0425 `original` at line 159): puzzle lane to fix; the mount patch stays applied.
Correction: the log carries only warnings (unnecessary qualification) for agent-lane, no errors, so the new mounts resolve; their tests still did not run because the target failed to compile on the extent-and-step-laws errors.
