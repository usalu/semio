# wfc2d (slot graph) status

## Results
- TS: `bun test` over the 5 test files under `◻️2d` (excluding bridge): 26 pass, 0 fail (log `🗑️generated/wfc2d-bun-test.log`).
- Rust: NOT yet compiled or tested. `repo cargo --store=isolated` builds 2-6 all died in source admission: crate `semio-plugin-window-config` exceeds the repo-wide 262144-byte per-crate boundary (another agent is splitting it). Native cargo is refused by the compiler ingress (not bypassed). Build 1 (before the sqlite-bitmap test fix by the coordinator) failed on E0433 `semio_os_kernel::pack`.

## Source ports done (unverified by compiler)
- `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`: `decode/encode_sqlite_snapshot_native` take the `output` slot, return `SnapshotReceivingOutcome`.
- `🚪️io/📝️text/📸️snapshot/🛬️native/🦀️.rs`: decode now goes through `decode_sqlite_snapshot_record_native` with the output slot; the post-decode `admit_sqlite_values` is gone.
- `🚪️io/💾️binary/📸️snapshot/🦀️.rs`: removed `decode_into` and its retirement region; `🧪️tests/🔬️store-fixture`: removed the decode-in-place test and its helpers.
- `✏️editor/🫧️transient/🦀️.rs`: `SnapshotProjection` is now derived.
- `🧬️schema/🧬️mutations/🦀️.rs`: added `dsl::original_pack_value_operation_receiving!(Wfc2dMutation);`.
- `🔨️modules/🏠️host/💡️inferences/🦀️.rs`: payload builder/output closes use `close_original_effect_step` with `close_step(effect)`.
- sqlite snapshot tests: added `decode_native`/`encode_native` helpers and replaced the 13 direct call sites. Remaining in that file, not ported: `provider.export/import` and `with_snapshot_file_decoding` call sites (shape unknown until the compiler reports).

## Wiring review (source only)
- Catalog: `s.wfc.wfc2d@1/*#editor` is in the generated `playgrounds.json` and `catalog.json` (variant `wfc2d`, crate `🌎️hub/🧩️compositions/🀄️wfc/🧩️component/📦️packages/🦀️rust`, ports 6043/6143); plugin `on-artifact-kind:s.wfc.wfc2d` in `plugins.json`.
- `⏯️tool-run-matrix.json` has `s.wfc.wfc2d@1/*#editor` fill (non-mutating).
- Dispatch (`✏️editor/🧭️dispatch-and-artifact-editor`): slot create/delete/move/resize/pin/unpin/connect/disconnect, tile create/delete/weight/media (clear only, by design), rule create/delete, change-seed, camera/active-tile (config), solve (transient), commit-fill. Fill tool in `🎭️modes/✏️edit/🛠️tools/🔣️fill`.
- Not verified: wasm component build, runtime in the playground.
- transient diff: added retained_empty_option_patch!(Wfc2dTransientDiff { assignments, contradiction, source_revision }) between retained_row_patch! and retained_patch_diff!.
- config enum + 3 leaves: DslReceiving derive; set-solve imports Wfc2dTransientMutation via super::.
- Wfc2dConfig record: added DslReceiving derive (mirrors Wfc3dConfig); enum + 3 leaves already match 3d.
- unit tests (config, transient): MutationKind::{diff,inverse} -> ColdMutationKind.
- host inferences: E0283 annotated `RetainedWorkOutcome::<_, ValueError>` in the commit-bytes close.
- editor pre-check: transient got `by_value_projection` and `original_pack_value_operation_receiving!(Wfc2dTransientMutation)`; command work is already unboxed (`Wfc2dCommandWork::new` passed directly), fill/host `InteractiveJob::step` already has the 3-parameter form, no `ui_assembly_error` use in 2d.
- drag unit test ported (connect_slots type, Arc view_state, HistoryInputDrafts, fixture result turn, close_step_cold). Build 8 had only these 7 errors in wfc-2d-v1 lib test.
- builds 9/10 blocked by repo-internal ENOENT (generations/whole-gate import); waiting for coordinator all-clear.
- build 11: only wfc-2d-tests lib-test errors (fill test): made WFC_2D_RETAINED_{RAW,OUTPUT}_BYTES pub + re-exported, finish_success pub, InteractiveJob in scope, tests mirror re-exports command/consts.
- build 13: fixed oracle dev-dep, pub decode_inference_value/Wfc2dInferenceJob::new/FillRun/finish_success, store-fixture close_owned_step_cold, fill payload from_cold, sqlite provider.import/export output slots.
- headless site: named constants HEADLESS_FUEL_PER_STEP=4096 / HEADLESS_STEP_BUDGET_US=4000 (as grid2d/bitmap). Unconfirmed by a run.
- pin-slot lib test errors fixed (diff tests: explicit trait paths and slice bases; config test: sparse diff Options).
- build 20 result: 22 suites, 261 pass, 26 fail. Fixed after: preview tests retire shared payload (retire_payload helper), ReplaceConfig inverse is one ReplaceConfig (as 3d), create_then_patch test asserts applied state (as grid2d), restored post-decode admit_sqlite_values(DecodeNative) in text native decode and kept the decode-cancel test.
- Left as shared framework: headless teardown (12x), terminal close PluginLifecycleFailure "original native producer omitted observed native work" (2x history_edits*), drag "dispatch did not settle" (2x) and "standalone store retires to terminal-empty shell" (1x).

## Build 25 (isolated, SEMIO_OFFLOAD=never): 22 suites, 264 pass, 23 fail (log wfc2d-isolated-test25.log)
Fixed since build 20 (261/26): sqlite native frontier (framework ReceivingSettlement fix), config payload law (ReplaceConfig inverse), create_then_patch_folds_into_the_row.
Remaining 23:
- headless teardown `wfc2d-inference-headless-teardown-retains-original-owner` (solve/example/fill tests, ~17): shared framework headless runner, owned by grid2d agent.
- `original acceptance app terminal close: PluginLifecycleFailure ... original native producer omitted observed native work` (history_edits_end_to_end, conflict_history_edits_end_to_end): shared.
- drag: `the wfc2d dispatch did not settle` (2), `the standalone store retires to its terminal-empty shell` (1): shared.
- `shared original alias must retire or transfer` still in `completed_live_preview_uses_authoritative_assignments_and_partial_preview_keeps_trace` and `mounted_live_fill_obeys_the_portable_revision_contract` despite the retire_payload helper (drop of a non-retired alias happens elsewhere, probably a cloned alias held by the render; needs a debug run).
Next runs use --store=shared with SEMIO_OFFLOAD=never.
- per coordinator: retire_run (provisional_entities + payload), drag settle via original_lifecycle_step, disposer via OriginalFixtureScope; added semio-plugin-ownership workspace dep and v1 dev-dep. Awaiting go for build 26 (shared store, SEMIO_OFFLOAD=never).

## Build 26 (shared store, SEMIO_OFFLOAD=never): 22 suites, 266 pass, 21 fail (wfc2d-shared-test26.log)
Compiled cleanly (decode fix compiled; no flow-to-fleet). Shared-alias failures are gone.
Remaining 21:
- headless teardown `wfc2d-inference-headless-teardown-retains-original-owner` (17: solve, example, fill, preview tests): the paid begin_close fix was not in this snapshot or does not cover wfc2d.
- history_edits_end_to_end, conflict_history_edits_end_to_end: now `original fold retirement requires its actual Source decoder` (was: native work handoff).
- drag (3): after the original_lifecycle_step / OriginalFixtureScope port, 2 still "the wfc2d dispatch did not settle" (drag test:148) and 1 still "standalone store retires to its terminal-empty shell" (:244).
- drag settle: maintenance step only when !has_runnable_typed_operations (as wfc3d). Store-retire test left pending wfc3d trace; full verification awaits coordinator go.
