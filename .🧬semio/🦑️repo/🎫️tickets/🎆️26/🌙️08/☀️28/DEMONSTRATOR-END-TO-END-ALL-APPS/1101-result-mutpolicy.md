STATUS: PARTIAL. GREEN: `semio-framework-replication-mutation-retained` native `--lib` (rc 0, 0 errors, `🗑️generated/1101/mutpolicy-retained.{txt,rc}`). Everything else UNVERIFIED and BLOCKED by foreign crates that are not yet ported to the work-accounting API; none of the blocking errors names a crate of this lane.

## Done this wave
- Retained set-field leaf (the `RetainedMutation` work): compiles green with the peer API after `ValueError::invariant` landed.
- Ported `plugin/🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs` (10 `close_original_retained_step` closures now return `RetainedWorkOutcome{result,work:1}`, import added): the 11 instance-render errors are gone in the next run (`mutpolicy-fem2d.txt`, instance-render no longer listed).
- Ported `plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs` (`admit_original_construction` closure outcome; `admit_loan(&mut loan,cx)`), UNCOMPILED since (written after the last fem2d run).
- Replay law test (`store/📬️paged-one-item/🧪️tests/❄️cold-replay/🦀️.rs`): `advance_returned_snapshot_read(..).result.unwrap()` fixed.
- Replay catalogs (process3d, gen2d, gen3d, raster): already present, nothing to add. process3d (`editor-impl`), gen3d (`artifact-editor-impl`), gen2d (`play-app-editor`) override `build_document_store_owners` with `cold_replay_document_store_owners`; raster's `raster_document_store_owners_admission` installs `RasterColdIssuer` via `admit_source_constructor_with_shared_semantic_preparation` (replay authority). The 1029 audit text predates this. They compile only when the app editors do (UNVERIFIED).

## Blocked runs (logs `C/🗑️generated/1101/`)
| run | rc | blocker (owner) |
|---|---|---|
| fem2d transient | 1 | `plugin-tool-run-turns` `⏯️tool-run/🧵️instance/🎟️job-admission-and-finalize-turn/🦀️.rs` (5 errors: `drive_step` now takes `&mut OriginalJobReturnReceiver` and returns `OriginalJobReturn`; `descriptor.acknowledge(grant)` is an outcome; close closure). Needs a real `OriginalJobReturnOrigin` place in the tool-run job entry plus admit/close wiring: design port for the job/plugjob lane, file untouched since 10/31 18:50. Also 2d path-flatten warning only. |
| wfc2d transient | 1 | `semio-s-plugin-wfc-engine-job` 34 errors (`InteractiveJob::step` arity, `try_admit_owned`, `close_with_self_funded_context`), `semio-framework-test-original-snapshot-owners` 9, tool-run 1 (wfc lane + job lane) |
| wfc3d transient | 1 | same wfc-engine-job 34 errors, `plugin-instance-initialization` 65 errors |
| wfc-bitmap transient | 1 | wfc-engine-job 34, `semio-framework-plugin` 14 (latest run, after my command-routes/tool-machine edits; not yet triaged) |
| replay law (`--test mutation_apply cold_replay`) | 1 | the library compiles; the shared `mutation_apply` test target has 64 old-API errors in sibling test files (`🧪️tests/🦀️.rs`, `🧳️semantics/**`, `🔗️source-custody`, `🌳️immutable-root`: `admit_owned`, `?` on `RetainedWorkOutcome`, `retire_owned` signature). My one error there is fixed. Fix = port those fixtures (store test lane); the law cannot be isolated because it uses the target's `PlainDocument`/`DocumentMutation` fixtures. Last green run was 1030 (2 passed). |

## Not run
- `1030-tools/policy/apps.sh` (token API, bounded cold factory, process3d test, gen2d/gen3d rejection laws): every target depends on tool-run and instance-commands; would stop at the same errors. Root crates of the token API/cold factory were green in 1030.
- Mutation derive laws (`mutation_descriptor_roster_failures`, tail-hash test), Home fold stepper law, epw roster law, hub batch: depend on the same unported plugin crates (epw additionally on stale test code).

## Resume
1. job/plugjob lane ports tool-run `finalize-turn` and wfc lane ports wfc-engine-job; store test lane ports the `mutation_apply` fixtures.
2. Then: `1101-tools/mutpolicy/run.sh` (fem2d, wfc2d/3d/bitmap), `1030-tools/policy/apps.sh`, `1030-tools/mutderive/finish-all.sh`, and the law command in `1030-tools/policy/law.sh`.
