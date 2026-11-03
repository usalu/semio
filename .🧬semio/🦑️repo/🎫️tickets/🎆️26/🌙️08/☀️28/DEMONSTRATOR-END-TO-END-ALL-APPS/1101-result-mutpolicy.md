STATUS: PARTIAL, stopped by the 19:50 activation freeze. GREEN (finished runs): retained set-field leaf native `--lib`; fem2d transient native `--lib`; replay law `cold_replay` (3 passed). Everything else UNVERIFIED or blocked below. No processes of mine are running; no edits after the freeze.

## Green (logs in `C/🗑️generated/1101/`)
- `mutpolicy-retained.{txt,rc}`: `semio-framework-replication-mutation-retained` `--lib` rc 0, 0 errors.
- `mutpolicy-r2-fem2d.{txt,rc}`: `semio-s-artifact-fem-2d-editor-transient` `--lib` rc 0, 0 errors (`transient_root!{retained:true}` closure, after my tool-machine/command-routes ports).
- `mutpolicy-r3-law.{txt,rc}`: `-p semio-framework-os-store-one-item-batch --test mutation_apply cold_replay`: rc 0, `cold_replay_meters_the_base_snapshot_clone_as_the_bounded_path_does`, `cold_replay_inverse_rows_restore_every_pre_state`, `cold_replay_of_n_saved_edits_reopens_to_the_same_snapshot` ok (3 passed). To get the whole `mutation_apply` target compiling I finished the half-ported fixtures: `retire_owned`/`retire`/`project_root` outcome signatures (`🧬️mutation-apply/🧪️tests/🦀️.rs`, `🔗️source-custody`, `🌳️immutable-root`) and `semantics/apply` (`SealedShared::admit(..).result`, new `admit_source` helper driving the staged `RetainedCloneSourceAdmission`, including paying the input back on refusal). Only the 3 cold_replay tests were run; the other 16 tests of the target are not run.

## Edits outside the lane (small, API mirror)
- `plugin/🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs`: 10 `close_original_retained_step` closures return `RetainedWorkOutcome`.
- `plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs`: `admit_original_construction` closure outcome, `admit_loan(&mut loan,cx)`.
- Both compile (fem2d closure includes instance-render and instance-commands).

## Not proven
- gen2d `vcs_artifact_app` tests (`mutpolicy-r2-gen2d.txt`, rc 1): `semio-s-artifact-procedural-2d-testkit` `🧪️tests/🧰️support/…/🔬️retirement-driver/🦀️.rs` still on the old API (13 errors: `close_with_self_funded_context` gone, `drive_step` needs `&mut OriginalJobReturnReceiver` and returns `OriginalJobReturn`, `retire(..)` and `close_step(..)` outcomes). Owner: gen lane (shared procedural testkit). The token rejection laws of gen2d/gen3d/process3d therefore remain unrun; gen3d testkit shares it.
- gen3d test run, process3d and raster lib checks (`apps.sh` entries; run2.sh r2-gen3d/p3d/raster): killed mid-queue by the freeze, no result. These lib checks are what prove the replay producers compile.
- Replay catalogs: process3d (`editor-impl`), gen3d (`artifact-editor-impl`), gen2d (`play-app-editor`) use `cold_replay_document_store_owners`; raster `raster_document_store_owners_admission` installs `RasterColdIssuer` as replay authority. Static review only, nothing to implement.
- Mutation derive laws (`mutation_descriptor_roster_failures`, tail-hash), Home fold stepper law, epw roster law, hub batch: not run.
- wfc transients: SKIPPED. wfc is not one of the 8 demonstrator panes (generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik) and `♻️mit-bestand` has no wfc reference, so it is off the demonstrator closure. Last wfc runs (before the plugin family compiled) failed in `wfc-engine-job` (34 errors).

## Resume after `act-green` / `freeze-lifted`
`1101-tools/mutpolicy/run2.sh` runs fem2d, law, gen2d, gen3d, p3d, raster, derive in order; for the replay-producer proof run only the p3d and raster lib checks, then `1030-tools/mutderive/finish-all.sh` laws once the gen testkit is ported.
