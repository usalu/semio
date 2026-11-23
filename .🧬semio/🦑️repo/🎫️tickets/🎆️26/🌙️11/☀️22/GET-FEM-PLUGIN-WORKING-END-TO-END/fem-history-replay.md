# FEM history replay (2D and 3D): status report

## Verdict
Partially done. The replay-preparation owner, the semantic and retirement issuers, and the envelope decode owner bundle are now installed for FEM 2D and 3D, and the original failure ("original report requires its installed semantic and retirement issuers") is gone. The two acceptance tests still fail on a later stage, so they are NOT yet green.

## Design (how process3d does it, and the FEM equivalent)
process3d installs, in its `ArtifactEditor` impl:
1. `document_store_owners_source_demands` and `build_document_store_owners`: `cold_replay_document_store_owners` gives the shared cold issuer (replay preparation, initial snapshot and mutation retirement factories); then `operation_wire = semantic().with_batch_digest().with_original_pack_value_receiving().with_original_target(whole_document_target)`.
2. `build_envelope_decode_owner_bundle`: `process3d_envelope_decode_owner_bundle()`, with hand-written snapshot and mutation field authorities.

FEM equivalent, written once and shared by 2D and 3D:
- `✏️s/🔌️plugins/🏗️fem/🧩️receiving/✉️pack-envelope/🦀️.rs` (new): generic `PackField<T>` (bounded hex capture of one envelope field, `decode_pack` for snapshots, `decode_op` for mutations, exact bounded retirement), `FreshConflictRefusal`, and the macro `fem_pack_envelope_decode_owner_bundle!(fn, module, Snapshot, Mutation)` that defines the catalog and `fn() -> ArtifactEnvelopeDecodeOwnerBundle`. It is path-included into each editor crate (same pattern as the framework's own path includes), so the 230 duplicated lines of layout, raster and process3d are not copied a fourth and fifth time.
- 2D editor crate `🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs` and 3D editor crate `.../✏️editor/🦀️.rs`: include the module and instantiate `fem2d_envelope_decode_owner_bundle` / `fem3d_envelope_decode_owner_bundle`.
- 2D `✏️editor/🎛️play-app-editor/🦀️.rs` and 3D `✏️editor/🖥️play-app/🦀️.rs`: override `document_store_owners_source_demands`, `build_document_store_owners` (cold replay owners plus the process3d operation wire) and `build_envelope_decode_owner_bundle`.
- 2D `🧬️schema/🔺️diff/🦀️.rs`: `Fem2dDiff::retire_projection` and the `apply` snapshot replace now retire through `semio_s_fem_receiving::cold::retire` (3D already had both). Without this the first replayed history fold panicked with "sealed source requires original granted handback" (a dropped `SealedShared<PersistentList<FemNode>>`).

## Test progress (2D, `editor_app_2`, filter `history_edit`)
- r2: compile fixed (macro path argument must be an ident); stage moved to "sealed source requires original granted handback".
- r5 (after the 2D diff retirement fix): `history_edit_inputs_resolve` ok; `history_edits_end_to_end` and `conflict_history_edits_end_to_end` still fail:
  - `history_edits_end_to_end`: the pump never settles, history stage `Replaying (3/5)`, replay owner present, work 163840, pending history work true.
  - `conflict_history_edits_end_to_end`: archive load fails with `document archive parent Pack and SPR hydration was rejected at DecodeForward: Replay: truncated at offset 14` (same symptom the earlier attempt saw, so it is independent of the decode bundle).
- r6: tried `with_original_receiving()` (the gisterrain wire): does not compile for `Fem2dMutation` (needs `NativeReceive<DslValue>`), reverted to the process3d wire.
- r7 (debug eprintln of decode errors in `PackField`, `[DEBUG]` prefixed): queued behind other agents' cargo jobs on the shared `C:\t\2d-a` lock for over 30 minutes at the time of writing; no result yet.

## Open / next
1. The `[DEBUG]` eprintln probes were removed again; r7 was killed (my PID) while still queued on the shared 2D lock. Re-add a probe in `PackField` decode closures to learn which decode yields "truncated at offset 14".
2. Find the source of `truncated at offset 14` (candidates: `Fem2dMutation` binary receiving cursor via `with_original_pack_value_receiving` vs the persisted op encoding; FEM mutations have no hand-written `OpBinaryReceiving`, process3d does).
3. Stuck `Replaying 3/5`: likely FEM replay step cost vs the interactive per-turn grant (the same class as the open `playback_presses...` WorkLimit finding), not yet confirmed.
4. 3D tests not yet run (3D r1 failed to compile only because it builds the 2D editor crate, which then had the experimental wire; fixed since). Need `editor_app_2` and `editor_app` for 3D.

## Files touched
- NEW `✏️s/🔌️plugins/🏗️fem/🧩️receiving/✉️pack-envelope/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎛️play-app-editor/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🖥️play-app/🦀️.rs`
- Ticket helper: `fhr-test.sh` (runs a 2D/3D editor test target into `🗑️generated/fhr-<dim>-<tag>.log`).

## Round 2 (in progress notes)
Root causes found so far:
1. `Fem2dMutation`/`Fem3dMutation` used `dsl::variants_binary::encode_op` (ordinal tag + record body) as `OpBinary`, but their original receiving is `dsl::original_pack_value_operation_receiving!` which reads a complete Pack document: the archive load failed with `Replay: truncated at offset N`. Fix: `OpBinary` now encodes `pack_rt::encode_pack_value(to_value)` / decodes with `decode_pack_value` + `renormalize_whole_number_floats` (the form `Generation3dMutation` uses).
2. With decoding fixed, replay completes; the tests then stalled while closing: `terminal ticket has no explicit native header lifecycle` (framework `factory_ticket_demands`): `ArtifactStoreStringVectorRetirement` and `ArtifactStoreSupersessionRetirement` (store replay `lifted/read`) never declared `native_header_release`. Both now declare it (terminal Drop is trivial).

## Round 2 (final status)
Env: 2D `C:\t\2dh`/`2dhb`, 3D `C:\t\3d`/`3db`, native cargo (repo binary not used for builds; same fallback as round 1). Helper `fhr-test.sh <2d|3d> <tag> <test-target> <filter>` (logs `fhr-r2-<dim>-<tag>.log`). All my `[DEBUG]` probes are removed; the `[DEBUG]` lines still visible in logs (`close#`, `step_commit`, `replacement wire`, `primary history acceptance failure`) belong to other agents.

### Root causes found and fixed
1. Archive load `Replay: truncated at offset N` (2D `conflict_history_edits_end_to_end`, also 3D): `Fem2dMutation`/`Fem3dMutation` implemented `OpBinary` with `dsl::variants_binary::encode_op` (ordinal tag + record body), while their original receiving is `dsl::original_pack_value_operation_receiving!`, which reads a complete Pack document (`RetainedRecordReceiving::admit_document`). Fix: `OpBinary` now encodes `pack_rt::encode_pack_value(&to_value(self))` and decodes with `decode_pack_value` + `renormalize_whole_number_floats` + `FromValue` (the form `Generation3dMutation` uses). Files: 2D and 3D `io/binary/mutations/rs`.
2. After that fix the acceptance replays ran through, and the test then stalled while closing the fixture (`unretired closing session`, history commit finalizing, `step_commit` failure `terminal ticket has no explicit native header lifecycle`). Cause (framework): `factory_ticket_demands` requires `ErasedSnapshotRetirement::native_header_release` for every terminal ticket; `semio_os_store_replay::lifted_snapshot_read::ArtifactStoreStringVectorRetirement` and `ArtifactStoreSupersessionRetirement` (both used by `EditReplay` close, `raw_retirement`) never declared it (found with a `type_name` probe in the trait default: 138k hits, one type). Both now declare `native_header_release` (terminal Drop is trivial: `native(self,0,0)`). File: `🧰️framework/.../🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs`. A scan of all `ErasedSnapshotRetirement` impls shows many more types without the method (list in `/c/t/tmp/nhrscan-*.out`, store ledger/read/composition families); only the ones reached by the replay close were fixed.

### Results (after both fixes)
- 2D `editor_app_2` `history_edit`: `history_edit_inputs_resolve` ok; `history_edits_end_to_end` and `conflict_history_edits_end_to_end` still FAIL (run `fhr-r2-2d-k`).
- 3D `editor_app_2` `history_edit` (run `fhr-r2-3d-c`): same: 1 passed, 2 failed. 3D `editor_app` target and the 2D lib tests (`two_actors_reconnect...`, `two_instances_converge...`, interaction `a_gumball_move_edited_in_history_replays_its_downstream`) were NOT run: every compile cycle took 40-70 min on the contended host (C: shared by ~8 cargo jobs, others edit framework crates, one of them broke the 2D test crate for ~30 min).
- New failure shape (identical in 2D and 3D, first leaf `change-load-case-self-weight`): `the instance never settled within its original turn bound. History stage Some((Replaying, None, None)), replay owner false, pending history work false, pending typed operations false, work 654`. That is a different, earlier stage than before the two fixes (previously the pump reported `Replaying 3/5` with a live replay owner; the close stall hid behind it). The `[DEBUG] replay finish/step faulted {error:?}` probe I added to `step_replay` (time-travel store-state) did NOT fire, so the replay is not faulting through `step_derived_report_replay`; the session is in stage Replaying with no replay owner while `step_commit` of an earlier overwrite is stuck at `commit_stage=1` (publisher `advance`, 12 samples n=3..11 then no more calls before the pump bound). Most likely the commit publisher stage 1 (`OriginalReplayPublication::advance`) or the instance turn driver (`🧵️instance/🏃️turn-driving-and-replay-steps` line ~23, `StepCommit` only when `finalizing`) is not re-driven by the pump after the first calls.

### Next steps (not done)
1. Instrument `OriginalReplayPublication::advance` (`🏪️store/🕰️history/🏁️replay/🦀️.rs`) and the `StepCommit` driver to see why stage 1 stops being stepped (grant/fuel quote vs pump grant), then compare with process3d/energy, whose acceptance passes this stage.
2. Finish the `native_header_release` audit for the remaining store types if they surface after step 1.
3. Re-run 2D/3D `editor_app_2` and `editor_app` (`history_edit`), then 2D lib tests `two_actors_reconnect_after_local_edits_without_freezing_or_duplicate_replay`, `two_instances_converge_on_disjoint_edits` and the gumball interaction test (they were never reached; not confirmed to be related).

### Files touched in Round 2
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` (OpBinary pack-value form)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` (same)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs` (`native_header_release` on `ArtifactStoreSupersessionRetirement` and `ArtifactStoreStringVectorRetirement`)
- ticket helper `fhr-test.sh` (new env, `--config` flags, per-round log names)

## Round 3
Env: private dirs (since removed), native cargo with `CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false`, `-j 3`, 2D and 3D builds serialized after the coordinator's RAM notice. Helpers kept here: `fhr3-test.sh`, `fhr3-retry.sh` (reruns after foreign compile breaks), `fhr3-patch.py` / `fhr3-scan.py` (header-release patcher and scanner), `fhr3-sym.py` (PDB symbolizer for truncated Windows backtraces). No `[DEBUG]` of mine is left.

### Root causes found and fixed
1. Pump "never settled", stage Replaying (2D and 3D): the commit publisher did not stall, it failed. `step_commit` stage 1 (`OriginalReplayPublication::advance`, metadata phase) raised `original read birth inputs or actual Source changed` and `step_commit` swallows failures into `commit_failure` (stage then never leaves Replaying). Cause: `store/history/replay` `effected()` ran the owner action in `cx.with_original_effect`, whose Work carries NO Source turn, while an original read birth (`OriginalReadBirth::start_into`, `advance_root` of the edit-log source) requires `effect.work_source_turn()`. Fix: `effected` now uses `cx.with_original_source_effect` (same pattern as the other original-source callers). File: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/🦀️.rs`. With it stages 1 to 5 of the publisher complete (verified in 3D: no more `commit failure` and no "acknowledgement" refusal).
2. Round 1 FEM overrides had been lost from the shared tree (last-writer-wins: a locale edit of `play-app` overwrote them; the 2D `play-app-editor` as well), which brought back `original report requires its installed semantic and retirement issuers`. Re-added in `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/✏️editor/🖥️play-app/🦀️.rs` and `…/◻️2d/…/✏️editor/🎛️play-app-editor/🦀️.rs` (`document_store_owners_source_demands`, `build_document_store_owners` with the process3d operation wire, `build_envelope_decode_owner_bundle`). Re-check they survive.
3. `native_header_release` on every store retirement ticket: one pattern (`self.terminal_is_empty().then(||unsafe{RetirementHeaderRelease::native(self,0,0)})`) inserted into all 60 `ErasedSnapshotRetirement` impls under `🏪️store` that lacked it (scan: 0 left; test/testing dirs excluded). Compiled in the 3D editor build (store crates included). The replication (14) and plugin (13 non-test) lists from round 2 (`/c/t/tmp/nhrscan-*.out`) are NOT patched (outside the store scope you named).
4. Windows trap: the rustc command line of big FEM graphs now exceeds 32 767 chars with the new build-dir layout (`os error 206`); `CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false CARGO_BUILD_BUILD_DIR=<target dir>` (old layout, one `-L`) fixes it.

### Results (3D `editor_app_2`, filter `history_edits_end_to_end`)
- After fixes 1+2 (run q): `conflict_history_edits_end_to_end` PASSED in 3D (first time), `history_edits_end_to_end` FAILED with a debug panic `sealed source requires original granted handback` while the OwnedStores close rung retires the document store (`close_owned_stores_step`): a last `Arc<Fem3dSnapshot>` alias is dropped plainly (FEM `SealedShared<PersistentList<FemNode>>` lists require retirement; candidates: tail/prefix/history-cache aliases whose other holder was already retired; `cold::retire` itself is not the cause, probe showed no early exits).
- Later runs (r, s, and 2D `b`) fail EARLIER, at app terminal close, with `ticket close changed its selected body` (2D and 3D) and in 2D `history_edit_inputs_resolve` `captured shared original changed its Work or destination`. These appeared after another agent rewrote `🌱️value/♻️retirement/🎟️tickets/🔒️close` and `🔗️arc-custody/🔒️sealed` at 07:50 (run q finished before); I could not tell them apart from my header-release patch without a 30 minute A/B build, so the 2D tests, the 2D unit/interaction history tests and the full 3D `editor_app` target were NOT confirmed green.
- Release budget observation: an earlier 3D conflict run reached the close rung with only 4725 release bytes left of 536 870 912 (one ticket quoted 27 136); not reproduced in run q (passed). Watch it if close starts starving again.

### Open
1. Plain drop of the last `Arc<Fem3dSnapshot>` in the close rung (history_edits only). Next probe: line-tables for `semio-os-store-engine*` crates were already enabled (`--config profile.dev.package.<crate>.debug="line-tables-only"`), then `fhr3-sym.py` resolves frames.
2. A/B the `native_header_release` patch against the `ticket close changed its selected body` failure once the other agent's close/sealed edits settle (revert one file group at a time).
3. Run the 2D `editor_app_2`, the 2D lib history tests and 3D `editor_app` after 1 and 2.

### Files touched in Round 3
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/🦀️.rs` (`effected` uses the Source-turn effect)
- 60 retirement impls under `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/` (header release; list reproducible with `fhr3-scan.py`)
- FEM 3D `play-app` and 2D `play-app-editor` (owner overrides restored)
- ticket helpers listed above
