STATUS: PARTIAL. All procedural crates that gen-art1 (`C/🗑️generated/1101/gen-art1.txt`, wasm32-wasip2) reached compile clean (about 90: all 2d/3d model, mutations, io-*, geometry*, preview-eval, envelope, host, editor-*, viewer-*, examples). `semio-hub-procedural-component` itself and the crates that depend on stdio pdf-schema-4 are UNVERIFIED: the only red crate in gen-art1/gen-hub13 is foreign `semio-s-artifact-stdio-pdf-schema-4` (6 x E0432 `windows` import: `🧬️schema/📦️packages/🦀️rust/🪞️mirror` mounts a `#[cfg(any(test, feature="testkit"))] pub mod windows` that `✏️editor/🎭️modes/✏️edit/🦀️.rs:5` imports unconditionally). Native generation3d tests: NOT RUN, blocked (gen-test3 `C/🗑️generated/1101/gen-test3.txt`: `semio-framework-os-store-replay` 5 trait-bound errors in `⏩️replay-owned-state-operations:66,71`, replay lane).

## Fixes this stretch (all compile-guided from hub09/hub10/hub9..hub11 logs)
- NativeDefault: Generation3dPreviewCamera / Generation3dViewCamera (3d editor/viewer config) -> `NativeDefaultValue` + `NativeDefault` cursor impls.
- `begin_projection` of the 3 transient crates -> `RetainedWorkOutcome<(Projection, Progress)>`.
- 2d mutations apply: `owner.close_step(grant).result.expect(..)`.
- 3d geometry: cold oracle `compute` now builds its own `StepContext` per turn (`SelfFundedCloseCaller` no longer exists; `COLD_FUEL`), engine `admit_loan(&mut self.owners.source, cx)` staged form, engine `admit_failure` through an `OriginalDomainReservation<InferenceWorkFailure>` field (`failure_place`) that `close_original_step` closes and `terminal_is_empty` checks.
- 2d preview-eval tool-run and 2d editor reserved-import: `InteractiveJob::step(cx, claim) -> InteractiveJobStep::from_callback(..)`, `step_value` returns `OriginalTurnFailure`, publication `close_step` unwrapped with `consume_fuel(work)`, `close_original_granted_step` closure returns `RetainedWorkOutcome` with real work.
- 2d host / 3d envelope (snapshot-authority, mutation-decode-authority): `artifact_retirement_box_close_step(..).result`.
- 3d host: copied the 2d neural copy fns (`copy_neural_value/dictionary/tree/flow_ui`), `Removal::into_owned` for layout delete, `flow.step(grant).result`; 3d geometry-service `has_failure` closure; 3d io-binary `Generation3dMountedStringOwner` and 3d envelope `Generation3dMutationSynapseOwner` made `pub(super)`.

## Open
Waiting for foreign stdio crates (pdf-schema-4, png-tests, deflate-zlib) to compile; then rerun `C/1101-tools/gen/hub.sh N`, then the generation3d native tests.

## Verification notes
- `C/1101-tools/gen/art.sh N` checks the procedural artifact crates directly (wasm); `hub.sh N` the hub component; `test3d.sh N <suffix>...` runs native lib tests in the generation3d workspace (hub manifest cannot: analysis-check etc. are not hub members). Running 13 crates at once trips the Cargo Source gate (`semio-s-artifact-procedural-3d-model`, "files" refusal, remainingFiles 0, cap maxSourceFiles 32 / 262144 B per plan); the crate itself reaches 14 files / 134 KB, so the gate is shared across the whole plan; run few crates per invocation.
- Not done: brep-boolean fixture law, zero-fuel law, `every_funded_export_turn_advances_even_when_the_grant_is_already_exhausted` (phased-job unit test) — all native, all blocked by os-store-replay.
