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
