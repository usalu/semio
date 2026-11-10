# Lane kernelui

Crates: semio-ui-runtime-presentation, semio-framework-kernel-ui, semio-os-inference, semio-framework-mesh-engine.

## Step 1 lib ports (native W-kernelui-3 rc0, wasm32-wasip2 W-kernelui-4 rc0)
- presentation `📦️resources`: `place.close_step(body, effect)`, body from `cx.retained_grant().maximum_copy_bytes`.
- mesh-engine: `close_factory_ticket(.., grant.retirement.maximum_copy_bytes, effect)` (io/text, still behind its grant-shaped public `step`), `record_close.step(grant.maximum_copy_bytes, effect)`, `close.step(body, effect)` in canonical decode.
- kernel-ui: `OriginalPatches::close_step(body, effect)` is inherent, trait impl uses `native_retirement_reply!()`. Cargo.toml dep `semio-framework-value-native-owned` (LOCK NEEDED logged).
- inference: same inherent `close_step(body, effect)` + `native_retirement_reply!()` on `InferenceValueRetirement`; four `ControlledRetirement::step(body, effect)` sites; `Sealed::close_step(body, effect)`.
- inference source staging: `Staging` now carries the published-source destination (`Option<RetainedCloneSource>`); `take_output(&mut destination, effect)` runs as an ordinary effect turn in `admit_step` and in the closing ladder (no `admit_original_construction` detour); `take_sealed()` moves it into `self.source` once the publication completes.
- REMOVED (no callers anywhere, relied on deleted `RetainedCloneSource::project_owned`): `OriginalInferenceSource::project_snapshot_step`, `Sealed::project`. Re-add on top of `RetainedOwnedProjectionAdmission` (admit + take with a reply slot, and its close) when a consumer exists.


## Step 2 tests (ported, NOT yet run green)
- Root cause of the first test failures: `semio-framework-job` feature `testkit` links `semio-framework-value-testing-allocator` (a global allocator), so test targets declaring their own `#[global_allocator]` fail with a conflict. Fixed by using `observe_retirement_allocations` of the testing allocator.
- presentation `native_resource_custody`: own allocator removed, local `observe_heap_allocations_on_this_thread` wrapper; dev-dep `semio-framework-value-testing-allocator`.
- kernel-ui `🛬️receive/🧪️tests`: reservation law now drives `reserve_step(.., control, maximum, grant)` through `admission::Authorities::nested` and `close.step(body, effect)` through a per-thread `EffectBank`; dev-dep `semio-framework-value-testing-admission`.
- mesh-engine: `🧪️allocation` now re-exports the testing allocator observer and hosts the effect bank (`warm`, `fx`); unit close law uses `owner.step(copy, effect)`; canonical oracle drives `RetainedOwnedProjectionAdmission<dyn Tree>` (admit + take with reply slot) and `source.close_step(copy, effect)`. dev-deps allocator + admission. `io/text/tests/ownership` still calls the grant-shaped public `PolygonSourcePreparation::step`, untouched.
- inference: no test run (package is hub-only, hub cannot test it: kernel-ui "not a member of the workspace" and `semio-os-inference` is not in the framework workspace).
- Cargo.toml edits, LOCK NEEDED (logged): kernel-ui (native-owned dep, testing-admission dev-dep), mesh-engine (2 dev-deps), presentation (1 dev-dep). Lock already resolved them (W-kernelui-9 compiled past resolution).
- Rerun: `"$C/1105b-tools/runtime/tst.sh" W-kernelui-<n> 🧰️framework/Cargo.toml native -p semio-ui-runtime-presentation -p semio-framework-kernel-ui -p semio-framework-mesh-engine`.
- Last runs W-kernelui-12/13: red in peer crate `semio-framework-job-native-turn` (`🌱️birth/🦀️.rs:129`, if-without-else expected `()` found `bool`), before reaching my targets. W-kernelui-7/9 (before that peer edit) compiled presentation tests; replication-mutation-messages was briefly red (RetirementHeaderRelease) in W-kernelui-10.

## Debts
- `OriginalInferenceSource::project_snapshot_step` removed (no callers); re-add via `RetainedOwnedProjectionAdmission` when needed.
- Inference/mesh-engine/kernel-ui/presentation tests unverified.

## Step 3 test run on green native-turn (W-kernelui-15, native, all test targets compile)
Pass counts: kernel-ui lib 24 pass / 1 fail; mesh-engine lib 43 / 7 fail; presentation lib 20 / 4 fail; presentation `native_resource_custody` 0 / 5 fail.
Triage of failures:
- kernel-ui reservation law: denied-turn heap is (64,0) not (0,0); `Authorities::nested` allocates 64 bytes inside the observed region (harness, mine). Needs the denied probe observed around the poll only.
- mesh canonical oracle: "original projection must be driven or returned through its resident frame" during `admission.advance` of `admission_owned` (my fx driver stops driving projection frames; needs the staged owned path `advance_owned`).
- mesh unit close law: owner not terminal after the bounded loop (steps now spend one unit per effect; loop budget/body assumption mine). mesh decode tests: `turn < 199999` (no settle) in canonical decode tests, not touched by me beyond `.step(body, effect)`.
- mesh `neutral_polygon_io...` and `mesh_data_from_value_round_trip`: "JSON candidate release was denied after admission" inside pack-json via the still grant-shaped `AuthoringWallet` public `PolygonSourcePreparation::step` (the wallet ruling forbids that wallet; the mesh text preparation needs an effect-shaped `step`). `empty_scene...gltf`: "missing field nodes" fixture/serde, unrelated to the port.
- presentation lib: all 4 panic with "queued UI value retirement arena is poisoned" (ui-contract-values collection-arena, a peer's upstream panic poisons the shared arena) and "original fixture root closure exhausted its initiating authority" (job testing caller). `native_resource_custody`: `Closing` / None unwraps in grant-shaped mesh3d receiving (ui_scene math resource receiving, not ported).
Not fixed here: needs owner decisions (effect-shaped `PolygonSourcePreparation::step`, ui_scene resource receiving port, ui-contract-values arena poison).

## Step 4 follow-up (W-kernelui-21, SLOT_HANG_SECS=100)
Pass counts: kernel-ui lib 24 / 1 fail; mesh-engine lib 44 / 6 fail; presentation lib 24 / 0 fail (was 20/4); `native_resource_custody` 0 / 5 fail.
Fixed:
- ui-contract-values `close_ui_value_page_with_grant`: the "arena is poisoned" message was a mislabel. `borrow()` also returns Err(UnsupportedOwner) when no arena registration exists; the presentation test process has none, so idle drain now returns complete. This was the whole cascade of 4 present tests.
- mesh-engine `PolygonSourcePreparation::step(text, grant, effect)` is now the public, effect-shaped step (the wallet wrapper is gone). `parse_polygon_mesh_source` keeps a cold `AuthoringWallet` (no turn exists there) and now has a hard turn bound (spin guard). Callers still on the old signature, to port by their lanes: flow mesh operator-core (use `cx.with_original_effect`) and generation3d mesh-source closure.
- pack-json parser `normal_step_demands`: the page-release quote omitted `next_release_copy_byte_demand`, so `release_empty_page` was denied on a copy-0 grant ("JSON candidate release was denied").
- mesh unit close law passes: grants must cover `next_demand(body).copy_bytes` (the quote now includes the cursor header), and the close is driven through `step_into` under a real producer authority (`drive` in `🧪️allocation`). Inference Owned staging now drives `project_owned` at `projectionStart`.
- kernel-ui reservation law: denied probes now observe only `reserve_step`; the control work ledger is primed.
Still failing (why):
- kernel-ui: closing `ControlledRetirement` needs `step_into` under `drive` ("retirement field requires its original producer invocation"); the fx bank has no producer invocation.
- mesh canonical oracle: `source.close_into` fails "sealed retirement requires its original producer invocation" even under `drive` (the `RetainedCloneSource` close needs the original-close loan route, not the plain one).
- mesh decode tests (2): state-machine laws keep the old grant model (`copy = body`); needs the same quote-covering grants.
- mesh polygon tests (2): after the port the preparation still never yields its source within the turn bound (stage progression with the effect grant); spin is now fail-fast. `empty_scene...gltf`: "missing field nodes", unrelated.
- presentation `native_resource_custody` (5): fixture `OriginalJobFixtureCaller` root closure exhausts 64 turns, and mesh receiving makes no progress on TURN; ui_scene `Mesh3dResourceReceiving` has no reply-shaped surface to port to (pure grant state machine); not started.
- inference StoreRead staging cannot advance: `projectionStart` for a SnapshotRead authority needs the `OriginalProjectionFrame` driver (`StagedSourceAdmission::projection_step`) which requires the job turn's original action; Owned works. Debt.

## Step 5 (last items, context limit reached)
- flow mesh operator-core: the one job-turn caller (`step_preparation`) now calls `cx.with_original_effect(source_grant.retirement, |_, effect| source.step(text, source_grant, effect))`. UNVERIFIED: not compiled (its package manifest was not located in the time left).
- generation3d mesh-source closure: NOT ported; it is a fuel closure with no `StepContext`, so no turn effect exists there.
- `parse_polygon_mesh_source` still holds a cold `AuthoringWallet`: NOT removed. It is a synchronous convenience used by ~10 non-turn callers (flow `decode_mesh` in mesh-operator-io, glTF deserializer, set-widget-input commands, 3d mesh job tests, plugin unit tests, mesh-engine tests). Removing the wallet means replacing every caller by a turn-driven `PolygonSourcePreparation` loop; needs an owner-wide decision, not a local port.
- Polygon tests via the shared `driven` harness, kernel-ui close and mesh canonical oracle onto the original-close route: NOT done (kernel-ui needs `step_into` under `admission::driven`; the canonical oracle needs `RetainedCloneSource::close_original_into` with an `OriginalRetirementCloseLoan`).
- Final counts (W-kernelui-21, unchanged): kernel-ui 24/25, mesh-engine 44/50, presentation lib 24/24, `native_resource_custody` 0/5.

STATUS: BLOCKED tests (libs FIXED-verified W-kernelui-3 native, W-kernelui-4 wasm; native recheck W-kernelui-16); open items listed above
