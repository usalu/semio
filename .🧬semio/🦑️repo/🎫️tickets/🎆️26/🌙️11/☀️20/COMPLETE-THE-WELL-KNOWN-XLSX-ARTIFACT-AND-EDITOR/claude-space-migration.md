# Space component wasm migration (semio-hub-space-component, wasm32-wasip2)

Status: `cargo check -p semio-hub-space-component --lib --target wasm32-wasip2` is CLEAN (exit 0, separate target dir). Test targets of the Space/Home crates could not be type-checked with cargo (see "Test targets").

Loop: `claude-space-migrate.sh` (separate target dir, `PKG=<crate>` narrows to one package; log `🗑️generated/claude-space-migrate/check.log`). A concurrent agent briefly broke `semio-framework-io-control` with a `[DEBUG]` edit (E0308 in `🚪️io/🦀️.rs:71`); it cleared by itself, retry if seen.

## Errors fixed (file under `✏️s/🔌️plugins/🪐️space/🗿️artifacts/…`)

1. `SSpaceMutation: OpBinaryReceiving` not satisfied (space editor, viewer).
   - `🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`: added `DslReceiving` derive on the enum.
   - same dir `🌱create-artifact`, `🏷️rename-artifact`, `🕒touch-artifact`, `🗑️delete-artifact` `🦀️.rs`: added `DslReceiving` derive to each leaf.
   - `🪐️space/…/🚪️io/💾️binary/🧬️mutations/🦀️.rs`: `dsl::original_tagged_operation_receiving!(SSpaceMutation, COMPONENT_PROTOCOL_SEMIO);` (keeps the existing tagged wire, same pattern as Trinity rewriting/jack, Puzzle).
2. `build_tool_job` has 3 parameters, trait has 4 (hub space app).
   - `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🏠️space-app/🦀️.rs`: added `_construction: &mut semio_framework_job::OriginalConstructionAdmission` (the retained producer does not use it, same as Trinity rewriting).
3. Home transient retirement trait signatures (`close_step`, `retirement_birth_bytes`, `retire`, `retire_owned`, missing `retire_demands`/`retire_work_demand`/`recover_retire`/`*_owned_*`).
   - `🏠️home/…/✏️editor/🫧️transient/🦀️.rs` Retirement region rewritten: `HomeTransient` now implements `RetireOwned` through a hand-written leaf-style cursor `HomeTransientRetirement` (keeps the byte-sliced release of 4096 bytes per turn measured by `retained_bytes()`, uses `native_retirement_reply!`, `record_retirement_work`); `HomeTransientRetirementFactory` delegates to `SharedValueRetirementFactory` / `OwnedValueRetirementFactory` like GisMap/Raster. The old `HomeTransientRootRetirement` is gone.
   - The unit test `🫧️transient/🧪️tests/🔬️unit/🦀️.rs:239` still calls the old `SnapshotRetirementFactory::retire(&Factory, root, birth)` and needs porting to the effect API (not checked, test target not built).
4. `SHomeMutation: OpBinaryReceiving` not satisfied.
   - `🏠️home/…/🧬️schema/🧬️mutations/🦀️.rs` and `🔢️change-catalog-generation/🦀️.rs`: `DslReceiving` derives.
   - `🏠️home/…/🚪️io/💾️binary/🧬️mutations/🦀️.rs`: `dsl::original_tagged_operation_receiving!(SHomeMutation, COMPONENT_PROTOCOL_SEMIO);`
5. E0609 `BackboneExport<ArtifactPackFiles>` has no field `pack`/`spr`.
   - `🏠️home/…/✏️editor/🎮️commands/🗃️apply-local-catalog-document/🦀️.rs`: `files.pack` -> `files.files.pack`, `files.spr` -> `files.files.spr`.
6. `HomeCreateStudioWork::close_step` (E0308/E0061: `ControlledRetirement::step` now takes `(body, effect)` and returns a poll; the granted-step closure now returns `RetainedWorkOutcome`).
   - `🏠️home/…/✏️editor/🦀️.rs`: rewritten on `semio_framework_job::close_original_effect_step` (first turn moves the completion into a `ControlledRetirement` under `effect.performed(1)`, later turns forward `owner.step(body, effect)`, returns `Complete` once nothing is left), as in Trinity jack.

## Final decision for the Home transient (items 7 to 9)

Coordinator decisions: option 1 (derive value traits + `SealedShared` rows) and option 3 (read model outside the store) were analysed and rejected as framework redesigns (see the analysis below); the chosen route keeps the transient lane and gives it a cold preparation.

7. `bounded_transient_preparation_factory::<HomeTransient, HomeTransientMutation>()` (needed `SnapshotProjection`, `RetainedMutation`, `StoreMutationFor`).
   - Framework extension point `cold_transient_preparation_factory<P, M>()` added next to it in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🧬️native/🦀️.rs`, re-exported from `…/🫧️transient/🧵️publication/🦀️.rs` and `…/🔌️plugin/🦀️.rs` (both lists). Same custody machinery as the bounded factory, but with `store::ColdMutationApplyFactory<P, M>` as the semantics: the mutation's cold diff is applied to a clone of the captured root. Bounds: `P: Clone + RetireOwned`, `M: Mutation<P> + ColdMutation<P> + ArtifactCanonicalJsonTree + RetireOwned + StoreMutationFor<P>`. `HomeTransientDiff` already implements the cold diff algebra and `ApplyDirectoryPage` the cold mutation kind, so the page parse/admit/fold (`admit_page`, `fold_page`, per-row `Arc` sharing) is unchanged.
   - Home editor and viewer (`✏️editor/🦀️.rs`, `👁️viewer/🦀️.rs`) install it instead of the bounded factory.
8. `HomeTransientMutation: StoreMutationFor` (needs `OpBinaryReceiving`).
   - `DslReceiving` derive on the enum and on `ApplyDirectoryPage`.
   - Transient codec switched to the tagged frame: new `✏️editor/🫧️transient/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio` (`apply-directory-page` tag 0), codec file uses `encode_tagged_op` / `decode_tagged_op` and `dsl::original_tagged_operation_receiving!(HomeTransientMutation, COMPONENT_PROTOCOL_SEMIO)`.
9. `semio-hub-space` (composition) E0433 `semio_framework`, `semio_framework_job`, `semio_framework_value` not found: `dyn_enum_close!` now expands to those crates in non-test code. `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/Cargo.toml`: moved `semio-framework`, `semio-framework-value` from dev-dependencies to dependencies and added `semio-framework-job`.

Tests ported (`✏️editor/🫧️transient/🧪️tests/🔬️unit/🦀️.rs`, run through the kinds-conformance mirror):
- `publish` helper now uses the cold factory's preflight and the cold diff (the old call sequence used the removed `advance(grant)` API and a base kind the native factory refuses).
- `a_displaced_root_retires_in_page_grants` drives `ControlledRetirement<HomeTransient>` through `EffectBank`/`with_original_invocation` in page-sized turns and asserts the paid release covers `retained_bytes()` over several turns.
- New `the_cold_factory_folds_a_page_and_closes_after_publication_or_cancellation`: begin over a `TransientRead` base, advance with a wide grant to `Prepared`, compare the published root with the cold diff, then cancel, `begin_close` and close through `native.with_effect(..., close_step_controlled)` to terminal-empty (both after publication and when cancelled before the first step).
- Dev-dependencies added to the kinds-conformance `Cargo.toml`: `semio-framework-trace`, `semio-framework-value-testing-admission`, `semio-framework-value-native-owned`, `semio-framework-value-snapshot-read`.

## Test targets (not verified)

`cargo check --tests` cannot resolve the Space/Home crates' dev-dependencies: they are path dependencies outside every workspace `members` list (`🏅️standards` crates), so Cargo drops their dev-dependencies (`semio_framework_async_macros`, `serde_json`, `semio_framework_test_fixtures` and the new ones all show as unlinked, also in untouched crates such as `semio-s-space-core` and `semio-s-artifact-space-space-schema`). `repo show projects` lists no Home project either, and `repo`'s own gate requires its launcher. Consequence: the two ported tests and the new factory test are written against the current API but have NOT been compiled; the lib targets (what the wasm component needs) are clean. Run them through the repo test gate once the standards crates are registered as projects, then fix any signature drift in the new factory test first (it uses `store::SnapshotReadRegistryHandle`, `semio_framework_value_snapshot_read::SnapshotRead`, `NativeDecodeControl::with_effect`).
Other unrelated environment gap seen: `semio-s-artifact-trinity-jack-model` needs the generated `🤖️generated/🏢️nakagin/🦀️.rs`, which does not exist in this checkout (blocks native checks of anything depending on it).

## Analysis kept for the record: why not option 1 or option 3

Option 1 (derive value traits on the framework directory types, `SealedShared` rows): `SealedShared<T>` has no plain constructor (it is born only through effect-funded `admit_from`/`prepare_birth`), `PersistentList` is append/splice oriented, per-page fold mutations are not part of the `transient_root!` whole-root convention, and every nested directory type would need the full derive set. Option 3 (directory outside the store): the plugin runtime has no app-instance read-model facility; render receives the directory as `&TransientView<HomeTransient>` and `ArtifactInstanceOperationOwner` is job-lifecycle state, so it would need a new framework mechanism or a thread-local shim.

## Follow-up

A framework read-model lane (app-instance, query-side state folded from events, render-visible with a host invalidation signal and a job-captured read handle) would let Home leave the store entirely: the directory projection is a CQRS read model, not mutation-sourced user state, and does not belong in the undo-able transient lane. Home would then drop `HomeTransient`/`HomeTransientMutation`, the transient codec, this cold preparation and the hand-written `HomeTransientRetirement` cursor in one step. Until that lane exists, the transient lane with the cold preparation is the supported home for the projection.

## Tests

Root cause of "nothing builds the Space/Home tests": the artifact workspaces already exist (committed `🏠️home/Cargo.toml` and `🪐️space/Cargo.toml` under `✏️s/🔌️plugins/🪐️space/🗿️artifacts/`, modelled on the xlsx one, members = every `🏅️standards` crate + `📦️packages/🦀️rust`, bridge/oracles excluded). The earlier check ran `cargo check --tests` from the `🌎️hub` workspace, where these path crates are not members and cargo drops their dev-dependencies. Run from the artifact workspace (`claude-space-tests.sh`, `WS=home|space`, `SEL="-p <crate>"`, separate target dir) the dev-dependencies resolve; no manifest or project registration was needed, the only manifest edits are three dev-dependencies (`semio-os-store-testing-prelude` in kinds-conformance, `semio-framework-value-testing-admission` in examples/demo). The Trinity `🤖️generated/🏢️nakagin/🦀️.rs` gap never blocked these workspaces (not touched).

Why the tests had never compiled: they had drifted from the framework API. Fixes (all test-side unless noted):
- `ColdMutation` imports for `diff`/`inverse`; `apply_diff_cold` to materialise a diff (config, transient, viewer, space diff leaves: `SSpaceArtifactsDelta` is a list delta now).
- `command_from_action(action, &mut Option<DslValue>, &mut Option<Fault>)`; `OriginalTurnFailure::primary_fault()`; `Observer`-typed `EntityIdentityAuthority`; `Arc<ViewModel>` view state; `BackboneExport.files.*`; `semio_framework_os` -> `semio_os`.
- Original fixture caller: `OriginalWorkFixture::work_turn` + `finish()`, `ArtifactCommandSnapshotRead::unavailable`, `with_original_invocation` around `ControlledRetirement`/`close_step`, `bind_instance_id` + `settle_registered_typed_operation` + `close_registered_fixture_app`, `FixtureUiArena::admit()` in every rendering test, `observe(...)` instead of dropping a built tree.
- SQLite snapshot tests (home, space): `original_owners::{encode_native,decode_native,io_run}` and the `exported`/`imported` out-parameter pattern; the bun script read the fixture from the wrong argv slot.
- Fixtures/schema: Home `✅️apply` diff wrapped as `{"directory": ...}`, `🟰️`/`🚫️` diffs are `{}`; Home `change-catalog-generation` descriptor declares `"editable": false`; the demo catalog document (`🧰️framework/.../🪐️space/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️.semio`, handcrafted, no generator) uses the localized port label record (`label=native=en=.. de=.. reuse=en=.. de=..`) instead of a bare string; structural-correspondence tests point at the `🚪️io/📝️text|💾️binary` surfaces; history-edit law path fixed.
- Framework additions: `FixtureStoreReceiving::receive_with_work` (store testing prelude, additive); `cold_transient_preparation_factory` is installed by Home editor/viewer (see above).
- The cold-factory test is reduced to begin + cancel + close to terminal-empty: the generic ephemeral native preparation (`🔌️plugin/🫧️transient/🧵️publication/🧬️native`) never drives the mutation admission's projection (`projectionStart` -> "original projection must be driven or returned through its resident frame"), so no transient preparation can be advanced past admission; the fold itself is covered by the cold diff (`publish`).

Numbers (last runs, `cargo test --workspace --no-fail-fast`):
- Home workspace: 135 passed, 3 failed. Green suites (passed counts per test binary: 6, 1, 8, 2, 38, 16, 19; logs in `🗑️generated/claude-space-tests/home-test.log`): includes root sqlite 6/6 and kinds-conformance 38/38 (all Home transient tests incl. retirement, vectors, cold factory). Viewer 45 passed / 3 failed: `a_dispatched_page_publishes_one_transient_item_and_no_history_row`, `a_viewer_page_publishes_only_its_transient_item` (typed op `interactive-job.publication-stalled`: receiving quote has 0 items; the fixture turn grant leaves no headroom for the transient lane publication, `HOME_RETAINED_WORK_ITEMS = 1`, to be analysed in the plugin publication ladder), `the_viewer_folds_every_sealed_page_exactly_as_the_editor_does` (hand-built `ArtifactOwnedToolJobContext` lacks the Store issuers; needs the registered-app route, blocked by the stall).
- Space workspace: 102 passed, 20 failed, all in `semio-s-artifact-space-space-editor`: the 18 command/dialog tests read `requested_effects`/`mutations` from the dispatch result, which is now only a job ticket (needs `settle_registered_typed_operation` like Home's route test), and the two `history_edits_end_to_end` laws (archive load refusal `value.refusal`). Everything else (diff 14/14, schema 49/49, mutations 5/5, root 4/4) is green.

## Tests 2

Runs used `claude-space-tests2.sh` (separate target dir, `-j 4`, now deleted). Cargo was stopped on the coordinator's order after the overnight kernel panic; the last edits are therefore source-only.

### Space editor crate: 50 passed, 0 failed (last run, editor11)
- 18 command/dialog tests ported to `settle_registered_typed_operation`: `context::dispatch`/`dispatch_as` dispatch then settle and return the receipt (`effects`, `lanes`); `requested_effects` became `effects`; "mutates nothing" became `context::writes_the_document` (no Artifact/Draft/Config lane, since every settled operation reports a revision). `expect_err` became `.err().expect` (the receipt has no `Debug`). `SpaceIndexApp` pays its original close in `Drop` (fixes "Store slot requires payload handoff").
- Real defect found: `command_from_action("createArtifact")` read only `kindChoice`, while the registered route passes the record field `kind_choice`, so the command degraded to the "open dialog" path. Added the snake_case fallback like the sibling actions.
- Test drift: registry check reads the app-wide `definition.actions` too; the dialog count excludes the framework-injected history-edit finalize dialog; `parse_document_pack(..).into_snapshot()` retires the parsed envelope.
- history_edits: root cause was the Space editor installing no replay issuer. `bounded_artifact_store_owners` has no `replay_preparation` factory, so the first refusal was "initialization requires its original installed native semantic producer", then "no authored conflict path issuer", then "no original receiving capability". Fix in the editor: `document_store_owners_source_demands` plus `build_document_store_owners` using `cold_replay_document_store_owners` with `with_original_ordinal_binary_receiving().with_original_target(whole_document_target)` (the same installation as the sourcing artifact; Space persists tagged operations). `with_original_pack_value_receiving` was wrong (comparison ceiling 114 B against a 405 B document). Both history laws pass.

### Home viewer: framework fix, not yet verified
Root cause of the stall, from the code and the earlier failure: the ephemeral native preparation (`🔌️plugin/🫧️transient/🧵️publication/🧬️native`) advanced its `RetainedCloneSourceAdmission<M, FactoryOwnedSourceAuthority<M>>` only with `advance`/`take_output`. After the identity phase the admission sits at `projectionStart`, where `advance` refuses ("original projection must be driven or returned through its resident frame"). Fix: the preparation now carries the staged projection frame and action and drives `projection_stage`/`project_staged` (stages 1 to 5), hands a failed projection's refusal back, and closes the in-flight frame in `close`/`close_demand` (same protocol as `SnapshotReadSourceStaging`). Compiled; the `ownership` crate built with it.
Not verified: the three viewer tests still failed in the last runs. The stall `[0, 516650, 1999946]` (receiving quote items 0) was seen under load; an isolated run after the fix instead failed with `interactive-job.worker-continuation` "original effect performed more work than its ceiling" before any projection stage printed, so a work-ceiling mismatch remains to be located (suspect: the `with_allocated_effect(demand, 1<<20)` ceiling against the turn's receiving work quote). Debug prints were removed.
- The new Home test `the_cold_factory_advances_a_page_through_admission_to_the_published_root_and_closes` (kinds-conformance) blocked at turn 1 under `with_native_receiving`; changed to `with_native_receiving_work` (uncompiled).
- The viewer-folds test still hand-builds an `ArtifactOwnedToolJobContext` without Store issuers; it must be rewritten over registered editor and viewer apps (dispatch, settle, compare the receipt events); not done.

### Numbers
Space workspace: editor 50/50 (was 30/20); other Space suites were green before. Home workspace: 135 passed and 3 failed before; last full run 38 plus the other suites green, 1 new test failing and the 3 viewer/editor page tests unresolved.

### Changed files
Space editor `🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`, `🎮️commands/🧪️tests/🔬️unit/🦀️.rs`; framework `🫧️transient/🧵️publication/🧬️native/🦀️.rs`; Home `🫧️transient/🧪️tests/🔬️unit/🦀️.rs`; ticket `claude-space-tests2.sh`.

### Tests 2 addendum (source-only, uncompiled)
The staged projection stage now runs under its exact quoted Work (`projection_stage_quote(..).1`) instead of the constant `1<<20`: a recorded original action must be re-driven under the same ceiling it was quoted with, which is the likeliest cause of the `worker-continuation` "performed more work than its ceiling" refusal. To be confirmed by the queued home3 run; if it still fails, the next suspect is the receiving-scope work quote (`fuel.div_ceil(2)`) in `typed-operation-unit` against this effect's ceiling.
The viewer-folds test is rewritten (source-only, uncompiled) over a registered viewer and a registered editor: `feed_registered` dispatches and settles each sealed page, the typed receipts of both surfaces must be equal, and every refused page must name the fixture's fault code on both. The hand-built `page_context`, `page_step`, `viewer_feed` and `editor_feed` helpers were removed. Work-ceiling reading: the transient lane calls `advance_publish_one` inside `with_native_receiving_quoted(.., receiving_work)` with no work cap (the document lanes cap through `store_unit_run`), so the preparation's effect ceiling must not exceed the scope body work; this is why the projection stage now runs under its exact quote. Unconfirmed until home3 runs.
