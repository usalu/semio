# 2026-10-10 mg-misc2: migration state (source-complete except flow brep mesh, NOT compiled)

Executor `mg-misc2`. Scope: block, dag, note, remodel, playbook, forms, reasoning, demonstrator, vcs, flow.
No cargo ran: `os.status` is PARTIAL (semio-framework-plugin RED, 455 errors, 11:50), so no scoped crate can be checked.
Syntax gate that ran: every changed `.rs` (331 files in scope) parses with `rustfmt --edition 2021` from stdin (0 failures).

## Done (source level)

- One-item preparations rebuilt on `store::OneItemOwners` (corrections #14): block 2d, 5d, 3d (artifact + config), dag config, playbook, demonstrator playground, forms, vcs. Each has `begin_batch_digest` (`store::admit_artifact_batch_digest`), `begin_demand`, `begin(request, grant)`, progress-carrying `Progress`/`Prepared` steps, `refused` parking of the three owners from `prepare_one_item`, grant guards before reporting copied bytes.
- Derives (`semio_framework_value::RetireOwned` + `CanonicalJsonTree`, `#[canonical_json(owner = semio_framework_pack_json)]`) added by closure script on every `dsl::Mutations`/`dsl::MutationLeaf` type, its snapshot, and nested row types lacking them: block 122, dag 9, note 47, remodel 94, forms 24, playbook 11, vcs 11, reasoning 5, demonstrator 3 derive lines. Types with hand-written `RetireOwned` keep it (block rows).
- Decision 9A: all old-shape `build_{document,config,draft}_store_owners()` overrides deleted (defaults apply); block `document_store_owners()` helpers and reasoning's helper deleted; flow keeps its custom catalog migrated to `store_owners(grant)` + `store_owners_source_demands()` (`DocumentStoreOwners::admit_source_constructor`), TS source-contract strings updated.
- Command works: vcs `VcsEditCommandWork`, note `NoteCommandWork`, flow `FlowDirectStoreWork`, `FlowChildGroupWork`, graph work: grant-based `close_step`, four `next_close_*_demand`, `terminal_frame_release_bytes`, `checkpoint_byte`. Remnant owners retire through `ControlledRetirement` (vcs/note remnants derive `RetireOwned`).
- Flow `retained::Retirement` rewritten over `ControlledRetirement<Vec<Vec<String>>>`.
- Note presence retirement uses `SharedValueRetirementFactory<NotePresence>` (`artifact_retire_struct!`), bespoke type deleted.
- Playbook procedural `ModuleGeometryOwner`: `ArtifactInstanceOperationOwner` new signatures (`retirement_demands`, `maintenance_step(grant)`, `close_step(grant)`), stale import cache staged and admitted into `ValueRetirement` with a grant.
- Remodel reconstruction run and revalidate jobs rewritten to `step -> Result<Option<JobOutcomeBorrow>>` with an `Outbox` over `RetainedJobPublication`, `borrow_outcome`, grant `close_step` and demand methods.
- Tests migrated: block 2d/5d/3d fixtures and maintenance drains, block 3d transient publication, dag snapshot/sqlite, vcs (seeded envelope, edit-work close), reasoning (seed envelope), note (window transient, settle/close), playbook procedural (geometry shell, store close), remodel (session test driver, preparation-law), flow (transient trace, connect-media-ports, child-preparation, interactive-job, unit, viewer, retained), all `close_owned_step(1, ..)` loops -> `close_owned_unscheduled()`, `install_document_store_owners_exact(..)` -> `store::funded_bounded_artifact_store_owners` + Result.

## Open (not done)

1. Flow brep mesh job (`🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs`, 1043 lines): `MeshOperatorJob: neural_engine::OperatorJob` and `Operator::step_plan` are still the old shape; neural_engine now requires `step(budget, grant) -> (Step, Progress)`, `normal_step_progress`, `next_step_*_demand`, `next_close_*_demand`, `close_step(grant)`, and `step_plan(input, grant)`. Its tests (`🥽️mesh/🧪️tests/🔬️unit`, `🧪️tests/🔬️evaluate-budget`, `🔬️extension-guest-standalone`) are unmigrated.
2. Remodel jobs leave `ReconstructionEngine`, scene and writer as plain drops inside one close turn each (no RetireOwned for the engine).
3. `begin_batch_digest` bounds require every mutation type's closure to implement `ArtifactCanonicalJsonTree`; foreign types in my closures are unresolved until others add them: `ArtifactChild`/`SemioKitSnapshot` (Block3dSnapshot), `FormQuestion`/`FormStep`/`FormExpr`/`FormVectorField`/`FormQuestionOption` (forms: framework forms crate), `PlaybookFlowChild`, `ProgramContributionEntry`, `Host` (playbook config), `FormsStructureChild`/`FormsResultsChild` (forms snapshot), `SemioTextSnapshot`/`ArtifactChild` (note), `RemodelingAssetChild`/`RemodelingMeshChild` and `MeshData` (remodel). Externally tagged named-field variants (derive refuses them) not scanned yet.
4. Reasoning `WiresMutation` is an uninhabited enum: no `CanonicalJsonTree` impl written (only needed if a factory takes it as M).
5. Flow mutation leaves: corrections say os-domains derives them; not touched by me.
6. Expect compile follow-ups: exact `ControlledRetirement`/`OneItemOwners` semantics, `StepContext` retained accounting in the remodel Outbox, `PluginApp` demand helpers used in tests, derive limits.

## Out-of-scope compile errors

None collected (nothing could be compiled).

## Update 12:xx (after coordinator GO)

- Flow brep mesh job migrated (`🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs`): `MeshOperatorJob` implements neural_engine's grant-based `OperatorJob` (`step(budget, grant) -> (step, receipt)`, `normal_step_progress`, `next_step_*`, `next_close_*`, `close_step(grant)`, `terminal_is_empty`); preparation/import/cancel paths take a `RetainedCloneGrant`, original input dictionaries retire through `ValueRetirement::push_dictionary(grant)` instead of drops; `MeshOperation::step_plan(input, grant)` + `next_plan_*` demands; sync `evaluate` drives job then closes it. Receipts only cover the retirement turns (kernel/source step work is not metered).
- `BrepBooleanOperatorJob` (brep main) migrated the same way; its three boolean operators' `step_plan` take the owned input.
- Every other `impl Operator for` in `🌊️flow/🧩️extensions/**` (123 impls in brep, math, text, list, draw, logic, primitive, bim, dictionary) got the five required plan methods (`step_plan` -> `OperatorPlanAdmission::immediate`, zero demands, depth 1) because the trait no longer defaults them.
- Remodel run job: `ReconstructionEngine`, `FrameIngestion`, `ProductPreparation`, `EngineObservation`, `RunPhase`, `Owed` and their 113-type closure now derive `RetireOwned` (script, including types with no prior derive); engine, scene, observations and phase retire through `ControlledRetirement<RunRemnant>`. `ToolRunTickWriter` (tool-run crate) still drops plainly; it needs `RetireOwned` from its owner.
- `Form*` aliases (forms `🦀️.rs:24`) are `PlaybookBlock`, `PlaybookBlockOption`, `PlaybookExpr`, `PlaybookStep`, `PlaybookVectorField` in the framework crate `semio_framework_artifact_playbook_playbook` (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs` lines 34-140). Not my scope; they need `RetireOwned` + `CanonicalJsonTree` there (`PlaybookExpr` is internally tagged, so the derive accepts it).
- #13 scan: no externally tagged named-field enums among the 9 plugins' canonical closures.
- Still unmigrated tests: `🥽️mesh/🧪️tests/🔬️unit/🦀️.rs` (853 lines of `job.step(n)`/`close_step(n, bytes)`/`dispatch_job`/`retire_cancelled_evaluations_close_step` one-liners), `🧪️tests/🔬️evaluate-budget`, `🔬️extension-guest-standalone`, `ValueRetirementStep` users. They need the neural registry test harness API (`dispatch_job`, `TestGeometryRegistry`) first.

## PARKED (usage limit) - first compile round, exact state

No edit in flight; every changed `.rs` in scope parsed (`rustfmt --edition 2021` from stdin) at the last full check, and the later remodel/brep/mesh edits were each parse-checked.

Compile rounds started with `cc.sh` (scratchpad; slot-gated `cargo check --manifest-path <artifact>/Cargo.toml -p <pkg> --lib --keep-going --message-format short`, private target dir `.🧬semio/🦑️repo/⚡️cache/play-fleet/mg-misc2/`, logs `<pkg>.log` there). Each plugin artifact is its own workspace (`✏️s/🔌️plugins/<p>/<artifact>/Cargo.toml`, not the `✏️s` workspace).

Native `--lib` results (11:xx after plugin lib GREEN): every crate in scope is blocked by an UPSTREAM red dependency, so no error in my own code was reached yet:
- semio-s-artifact-demonstrator-playground: `semio-s-artifact-stdio-contract` (8 errors: `AppOperationContext` missing field `retained` at stdio `✏️editing/🦀️.rs:1399`, etc.).
- semio-s-artifact-vcs-vcs: `semio-s-artifact-stdio-binary` (9 errors).
- semio-s-plugin-flow-extension-math: `semio-framework-artifact-playbook-playbook` (1 error: `OrderedMap<DslValue>: ArtifactCanonicalJsonTree` at framework playbook `🦀️.rs:360`) and `semio-framework-artifact-flow-flow` (4 errors, `OrderedMap` trees, flow snapshot `🧬️schema/📸️snapshot/🦀️.rs:80,140,250,273`).
- semio-s-artifact-block-2d: `semio-s-plugin-block` (8 errors, MY crate, not yet inspected: `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust`) and stdio-binary (9).
- semio-s-artifact-dag-dag: `semio-framework-graph-layout-run` (6), flow-flow (4), stdio-binary (9), `semio-s-artifact-stdio-semio` (196).
- semio-s-artifact-note-note: stdio-binary (9), `semio-framework-artifact-workflow-workflow` (5).
- semio-s-artifact-remodel-remodeling: workflow-workflow (5), `semio-framework-plugin` (1 error, 1274 warnings) when built with the artifact's features.
- semio-s-artifact-reasoning-wires: graph-layout-run (6), semio-framework-plugin (1).
- semio-s-artifact-forms-forms: semio-framework-plugin (1).
No wasm32 / `--tests` / unit-test run yet.

Next steps after resume:
1. Re-read `os.status` and the newest corrections; re-run `cc.sh` for the crates above (order: `semio-s-plugin-block` first, it is mine: `cc.sh 🧱️block "" semio-s-plugin-block --lib`; then block-2d/3d/5d, catalog, dag, note, remodel, playbook (+procedural), forms, reasoning, demonstrator, vcs, flow artifact, the nine flow extensions).
2. Fix every error whose path is under `✏️s/🔌️plugins/{🧱️block,🕸️dag,🗒️note,📸️remodel,📖️playbook,📋️forms,💡️reasoning,🎪️demonstrator,🌿️vcs,🌊️flow}`; send cross-scope errors (stdio-contract/binary/semio, framework playbook/flow `OrderedMap` trees, graph-layout-run, workflow, plugin features) to main.
3. Then `--target wasm32-wasip2 --lib` with each crate's component features, then `--tests` (skip testkit-dependent until corrections says GREEN), then unit tests (`cargo test ... --lib --no-fail-fast -- --test-threads=4`).
4. Still to do in source: brep/mesh unit tests and `evaluate-budget`/`extension-guest-standalone` tests; config-lane preparations that used `bounded_config_store_one_item_preparation_factory` must switch to `config_apply_preparation_factory` (#36) and non-mutating editors to `None` (#38); `ArtifactCommandWork::work_demands` must be implemented by vcs/note/flow works (#29); `ArtifactPresenceSnapshot` one-liners for non-empty presences (#34); `ToolRunTickWriter` needs `RetireOwned` from tool-run; Form* alias derives belong to the framework playbook artifact.

## PARKED 2 (coordinator PARK at 5h usage 88%; resume after ~19:10)

All edited files parse (rustfmt gate). No background runs of mine remain.

Native `--lib` results (exact, `cc.sh`, exit 0 = green):
- semio-s-plugin-block, semio-s-artifact-block-2d/-3d/-5d: GREEN
- semio-s-artifact-note-note: GREEN (window config on `WindowConfigApplyEdit`, `SetCamera(NoteCamera)` newtype, `NoteViewCommand`/`NoteCommandUnit`/`NoteIdOwner`/schema `NotePresence` RetireOwned, `ActorId(.into())`, window transient via `try_duplicate`)
- semio-s-artifact-dag-dag: GREEN (`DagViewCommand: RetireOwned`)
- semio-s-artifact-reasoning-wires: GREEN (wires dep `semio-framework-canvas` added, `CanvasExtension` impl path, `*Diff` imports, transient Diff impls retargeted to the live `WiresCanvasTransient`, window config on apply edit)
- semio-s-artifact-forms-forms: last run 2 errors (E0117 `Keyed` orphan on foreign playbook rows). Fix applied just before park, NOT yet compiled: `FormsQuestionKeys`/`FormsStepKeys` `KeyOf` markers + `list: Vec<..>, key: String = by <Marker>, values_only` in `🧬️schema/🔺️diff/🦀️.rs`. Also done: `ChunkAddressableJson` on `SharedUtf8`, patch types RetireOwned, `ChangeBlockField` hand `ArtifactCanonicalJsonTree` (flatten wire), try window config apply edit, `FormsViewCommand`, sqlite admission `&mut *native`.
- Edited, NOT yet compiled: flow main window config (RetainedClone + Edit), remodel model/report/frames window configs (derives, exchange, Edit).
- Not yet compiled at all this round: remodel, playbook (+procedural), demonstrator, vcs, flow artifact, nine flow extensions, block catalog (manifest: `🧱️block/🗂️catalog/📦️packages/🦀️rust/Cargo.toml`).

Framework edit made (cross-scope, tell main): `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` `field_set_mutations!` now derives `RetainedClone` + `CanonicalJsonTree` (tag=kind, content=value, per-variant rename) on the generated set enum and emits `ConfigApplyMutation::exchange`; this serves every `config_record!` user (forms try, flow main, generation2d).

Cross-scope notes for main:
- `semio_framework_plugin` crate root lacks re-exports of `WindowConfigApply{Cursor,Edit,Mutation}` and `io_dispatch`/`ErasedComposeSource`/`IoDirection`/`IoKey`/`IoPayload`; my files use `semio_framework_plugin::app::…`.
- Plugin crate was transiently broken by a peer (`MountedOwnerPhaseV1`/`MountedOwnerTurnV1` at ~16450); recovered.

Next steps: `cc.sh 📋️forms 🗿️artifacts/📋️forms semio-s-artifact-forms-forms --lib`, then remodel, playbook, demonstrator, vcs, flow, extensions, block catalog (native), then wasm32 with component features, then tests.

## RESUME 2 results (native `--lib`, exact `cc.sh` exit 0 = green)

GREEN: semio-s-plugin-block, semio-s-artifact-block-2d/-3d/-5d, semio-s-plugin-block-catalog, semio-s-artifact-dag-dag, semio-s-artifact-note-note, semio-s-artifact-reasoning-wires, semio-s-artifact-forms-forms, semio-s-artifact-vcs-vcs, semio-s-artifact-remodel-remodeling, semio-s-artifact-playbook-playbook, semio-s-artifact-demonstrator-playground, semio-s-artifact-flow-flow, and all nine flow extensions (brep, bim, list, dictionary, text, primitive, draw, logic, math).

Fixes in this round (beyond earlier sections):
- forms: `KeyOf` markers for foreign playbook rows in list deltas; `ChangeBlockField` hand `ArtifactCanonicalJsonTree` (flattened wire); `ChunkAddressableJson` on `SharedUtf8`; try window config on apply edit.
- vcs: `RetireOwned` on config/presence and their mutations, config lane factory `None` (no config mutations, #38), `protocol::apply_diff`.
- remodel: window configs (model/report/frames) on apply edit; durable artifact, rows, patches, `KdNode`, buffers derive; `Arc<Vec<u8>>` rope leaves; `OpaqueOwner` (one-drop retirement) for the foreign `PngScanlineDecoder`/`JpgStepDecoder` (their crates own no retirement cursor: heap not individually accounted); jpg snapshot `.image` fields; sqlite `control().checkpoint`; stale `native_codec` re-exports removed.
- playbook: `PlaybookSnapshot` RetireOwned, config `*Diff` import, `app::` composer imports.
- flow artifact: cold-grant helpers (`cold_grant`, `host_with_session`), session tick API grants, `value_fault`, deps (`semio-framework-async` moved to dependencies), `FlowPresence` RetireOwned.
- brep extension: kernel `modeling_slice`/`tessellation_slice` local helpers (same funding as the kernel's sync drivers), `ExtensionResourceOwner::invoke` on `StepContext`, `plan_job`/`plan_capacity` moved out of the trait impl, `semio-framework-job` dependency.
- procedural (in progress): broken `use crate::::*;` repaired, codec imports via `crate::component`, `module_value_fault`, `Command: RetireOwned`; its last compile was blocked by a peer-broken `semio-framework-dsl-record` (E0119 `RetireOwned` on `WireNode`/`WireEdgeLabel`/`WireValue`, `🗣️dsl/🧬️schema/🦀️.rs:377,385,397`).

Pending: procedural native re-run; wasm32-wasip2 with component features for every crate above; `--tests` and unit tests; brep/mesh tests, `evaluate-budget`, `extension-guest-standalone` test migrations.

## FINAL (this turn): release path GREEN

Native `--lib` exit 0 and `--target wasm32-wasip2 --lib` exit 0 (block artifacts with `--features component-app-assembly`, native and wasm) for: semio-s-plugin-block, semio-s-artifact-block-2d/-3d/-5d, semio-s-plugin-block-catalog, semio-s-artifact-dag-dag, -note-note, -reasoning-wires, -forms-forms, -vcs-vcs, -remodel-remodeling, -playbook-playbook, semio-s-plugin-playbook-procedural, semio-s-artifact-demonstrator-playground, semio-s-artifact-flow-flow, and flow extensions brep/bim/list/dictionary/text/primitive/draw/logic/math.

Component-feature fixes: block `Block{2d,3d,5d}Config(+Mutation)` and Presence RetireOwned, `ArtifactPresenceSnapshot` one-liners, `artifact_retire_struct!` lines for artifact-local patch types and `BlockOptional{Number,Orientation,Scale}` (next to hub-a's lines), `BlockCamera3d` canonical tree, `Block3dConfigMutation` internally tagged (`kind`, kebab-case) with canonical tree, `Block3dWindowView(+Patch)` RetireOwned, `protocol::apply_diff`, 3d world transient codec paths.

Not done: `--tests`, unit tests, brep/mesh test migration, `evaluate-budget`, `extension-guest-standalone`. Cross-scope: store `field_set_mutations!` extended (config_record users); `PngScanlineDecoder`/`JpgStepDecoder` have no RetireOwned (wrapped locally in `OpaqueOwner`, heap not individually accounted); plugin crate root lacks `WindowConfigApply*`/`io_dispatch`/`ComposerEntry` re-exports (files use `semio_framework_plugin::app::`).
