# 1029 Result: pubtoken

STATUS (CHECKPOINT): AMBER, edits complete and compile-atomic, NOTHING COMPILED YET. Batch b1 (`1029-tools/pubtoken/run.sh b1`, logs `🗑️generated/1029/pubtoken-b1-*`): p3d/g2d/g3d exited rc=1 with a repo/bun "repository:command failed" and zero `error[` lines (capacity/infra, not rustc); raster (`-ras`) still queued/running at checkpoint (pid in pubtoken-b1.pid). b1 started before the Part 2 edits, so it is not a valid verdict on them.

## Part 1: publication token (done)
- Framework: `DocumentStorePublicationToken` (mint/operation/generation/validate_live) in `plugin/🌱️initialization/♻️retirement/🦀️.rs`, re-exported via lifecycle + `semio_framework_plugin`. Minted in `ActiveArtifactStoreReplacement::new` (new field `publication_token`), passed by the maintenance ladder to `validate_document_store_publication(&token, live_generation)`. Removed: `admit_document_store_publication_authority` hook, `REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY` (ArtifactApp, ArtifactEditor, EditorApp). No fallible admit remains, so no `let _ =`.
- Hosts: deleted process3d crate `📬️publication-authority` (+ workspace dep, host Cargo.toml, both Cargo.lock, 📋️build-owners.json), gen2d modules publication-leases/credits/hostile and testkit `publication_authority`, gen3d lease table in host `🦀️.rs` and testkit door. Decoders use the domain constants `*_MAXIMUM_DOMAIN_ITEMS` instead of lease credits. Editors lost const/hook/validate overrides.
- Tests: hostile-lease laws removed; replaced by token ABA laws (`authoritative_publication_token_rejects_stale_generation_aba` in 3 wasm mounted-registry files; process3d retained-laws use token.validate_live). dev/composition gen3d `publication_authority` door left (pure serial-lock alias + seed fn).
- SHARED CHANGE: ArtifactApp/ArtifactEditor trait signature of `validate_document_store_publication`.

## Part 2: replay producer (written, unverified)
- `store/📬️paged-one-item/❄️cold-apply/🦀️.rs`: `ColdReplayProjection`, `ColdReplayEdit` and `impl ArtifactReplayPreparationFactory for ColdMutationApplyFactory` (NativeSemanticPreparation, in-place `apply_diff_cold_into`).
- `cold_replay_document_store_owners(_source_demands)` in the initializer file (shared cold issuer = publication + replay), exported from `semio_framework_plugin`; process3d/gen2d/gen3d editors override `document_store_owners_source_demands`/`build_document_store_owners` with it; raster host catalog (`🚀️store-initialization`) installs `RasterColdIssuer` via `admit_source_constructor_with_shared_semantic_preparation` (editor+viewer use it).
- Default `bounded_artifact_store_owners` unchanged (not universal: needs `ColdMutation`; domains opt in).
- NOT DONE: law test (N edits saved/reopened replays to same snapshot). Probe: raster `a_demo_edit_archive_loads_back_through_the_document_archive_door`; also add a framework law for ColdMutationApplyFactory replay next to `replay/🎮️operation/🧬️native/🧪️tests`.

## Next step
1. Wait for/inspect raster b1; re-run `bash 1029-tools/pubtoken/run.sh b2` (p3d, g2d, g3d, ras `--workspace --lib`) when the queue is free; then `1029-tools/dispatch/chk.sh <N> 3d` for puzzle wasm.
2. Likely compile risks: bounds `Snapshot: Clone`/`Mutation: ColdMutation` on the four domains, `RetainedCloneClose` API use in ColdReplayProjection, unused imports (Fault/FaultOrigin) in the three editors, host Cargo still lists `semio-framework-job-registry`.
3. Add the replay law test and run the raster probe.

## Update (10/30) batch b2
- process3d b2 refused: workspace `Cargo.toml` members still listed the deleted `📬️publication-authority` crate -> FIXED (member line removed). The process3d Cargo.lock/hub lock already edited; if a lock refusal remains after relock by main, it is this crate. Rerun `1029-tools/pubtoken/p3d.sh b3` (p3d then puzzle 3d wasm `chk.sh 9 3d`) in flight; logs `pubtoken-b3-p3d.*`, `dispatch-chk9.*`.
- gen2d b2 rc=1: ONLY foreign errors (io-text orphan-rule: Generation2dMutation/Snapshot/Diff impls in a crate other than the model; testkit follows). Editor/host of gen2d are blocked behind io-text, so my gen2d edits are unverified.
- gen3d b2 rc=1: ONLY foreign errors (brep-boolean/analysis-check `step_boolean_job_sync` signature (hubconv), testkit). Host/editor unverified.
- raster b2 rc=1: ONLY foreign errors (raster-model missing `asset_data_base64` crate, tiff regressions, `semio-framework-os` ArtifactHistoryLedger vs Vec mismatch in plugin-host). Raster host/editor unverified; framework crates incl. plugin-instance-docs, paged-one-item (cold replay) checked with no errors of mine.
- Replay law: the existing raster probe `a_demo_edit_archive_loads_back_through_the_document_archive_door` is the law (one edit saved + reopened); it cannot run until raster-model compiles. No additional test written.
- process3d b3 (`--workspace --lib`): rc=0, zero errors; host + editor libs compile with the token change and the cold replay catalog (process3d GREEN natively; test targets not checked). Puzzle 3d wasm `dispatch-chk9` queued after it.
