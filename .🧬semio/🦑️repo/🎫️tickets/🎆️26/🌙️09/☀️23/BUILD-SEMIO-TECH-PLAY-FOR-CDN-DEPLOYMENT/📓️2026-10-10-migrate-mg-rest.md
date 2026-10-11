# 2026-10-10 mg-rest: round 2 status (scope plus writer), blocked on foundation and four decisions

Executor `mg-rest`. Scope: `✏️s/🔌️plugins/{➗️mathematical,🎥️shooting,🏙️bim,🏛️architect,📕️norm,📜️imperative,✒️writer}/**`.

No migrated crate compiles yet. The one cargo run, `cargo check -p semio-s-artifact-shooting-shooting` through the slot gate, stops in `semio-framework-tool-run`, which is outside my scope. It reports 10 errors: `RetireOwned` conflicts in `♻️retirement/🦀️.rs:3-8` and a `RetainedCloneSource` mismatch in `👥️entities/📸️version/🦀️.rs:38`. `os.status` was still RED at 10:21. Everything below is read from the framework sources and is unverified.

## Done (uncompiled)

1. **Mutation derive sweep.** 1136 `#[derive(...)]` sites across the 7 plugins now carry `semio_framework_value::RetireOwned` and `semio_framework_value::CanonicalJsonTree`, with `#[canonical_json(owner = semio_framework_pack_json)]` on the next line.
   - The edit targets every derive naming `dsl::MutationLeaf` or `dsl::Mutations`.
   - The sweep ran from a temporary Python script in the session scratchpad, so no script is left in the repo.
   - Every crate that gets the derive already depends on `semio-framework-pack-json`.
   - Expected follow-ups once it compiles:
     - Field types without a canonical-tree impl. Main says value-core is adding `u16`, `u32`, `i8`, `i16`, `f32`, `DslValue`.
     - Nested snapshot types embedded in mutations, which need the same derives.
2. **norm** (`📇️registry/🧬️contract/🖥️app-surface/🦀️.rs`): removed the stale `close_step` override and its import. The trait default (grant-based, terminal-empty) applies.
3. **shooting** gumball unit test: the drain loop quotes `retirement_demands`, builds a `RetainedCloneGrant`, calls `close_step(&mut store, grant)` and asserts the receipt fits.
4. **mathematical** equation editor and unit test: `EquationRetainedCommandWork` close protocol rewritten.
   - It now has `close_step(RetainedCloneGrant)` returning `Pending { progress }` with one unit per call.
   - It also has the four `next_close_*_demand` methods and `terminal_frame_release_bytes`.
   - The test uses a new `close_grant()` helper and the new `InteractiveJobCloseStep` shapes.

## New findings, beyond the close protocol (each needs a decision or foundation work)

A. **Owner-catalog hooks changed signature.**
   - On `ArtifactEditor` and `ArtifactApp`, `build_{document,config,draft}_store_owners` now take a `RetainedCloneGrant` and return `Option<Result<(owners, progress), DocumentStoreOwnersAdmissionError>>`.
   - The trait defaults are framework-bounded. `bounded_document_store_owners` and `bounded_config_store_owners` also take a grant now.
   - In `🪟️window/🎚️config` and the Draft/Config "none" modules, `no_config_store_owners()` and `no_draft_store_owners()` return `Result<DocumentStoreOwners, ValueError>`. That does not match the editor hook's return type.
   - Old-shape overrides are in mathematical, shooting, architect, imperative, norm (`app-surface:85-150`) and the writer viewer and editor.
   - Proposal: delete these overrides and use the trait defaults. I have not done it, because the no-op config and draft cases depend on the defaults for `NoConfig` and `NoDraft`, which I could not verify.
   - Please confirm, or tell me whether another executor is doing this as a repo-wide codemod.
B. **`WindowConfigOwner` has new required items:** `type Edit: RetainedCloneEdit`, `const MAXIMUM_PREPARATION_DEPTH`, `fn build_retained_edit()`. `State` must be `RetainedClone`, and `Mutation` needs `ArtifactCanonicalJsonTree` plus `RetireOwned`.
   - Owners in scope: mathematical graph window, architect (register, graph, report, adjacency), norm results, bim window-config macro, writer main window.
   - The only template is the framework test `🪟️window/🎚️config/🧪️tests/📥️retained-pack-load/🧬️preparation/🦀️.rs`.
   - That template uses `WindowEditCustody`, which is `pub(crate)` in the plugin crate. Plugins cannot use it until it is exported, or a generic whole-record `Replace`/`Snapshot` edit is provided by the plugin crate.
   - Question for main: will value-core or rn-os provide that generic edit (the bim macro and the mathematical camera are whole-record replaces)?
C. **Writer host-owned module** (`🔨️modules/🏠️host/🧰️owned/🦀️.rs`, 1257 lines, 103 old-protocol hits) is not migrated.
   - **Blocker.** The plugin's `ArtifactStoreInitializationAuthority::step` still returns `StepOutcome` while its `close_step` and `retirement_demands` are grant-based. The trait is only half migrated, so the writer initializer cannot be written against a stable signature.
   - **Proposal.**
     - Replace `WriterSnapshotRetirement`, `WriterSnapshotRootRetirement` and `WriterMutationRetirement` with the generic `semio_framework_value::retirement::OwnedValueRetirementFactory` and `SharedValueRetirementFactory`. The plugin composition test fixture already does this.
     - Reduce the three envelope decode authorities and the custom initializer to the framework-bounded ones: `bounded_document_store_initialization_job` and the default owner bundle. Writer's custom code exists for unbounded `text`, so this needs a check that the bounded path pages `String`.
   - Please confirm that direction, or tell me the exact trait shape to implement once rn-os finishes the initializer.
D. **Equation store preparation** was already grant-shaped at HEAD, but it lacks `begin_batch_digest` and still uses the two-generic request. Per the guide, add `begin_batch_digest` via `store::admit_artifact_batch_digest(edit, grant)`. Switch `begin` to the three-generic request and the `(box, progress)` return with request handback. I have not done this, because a trait mismatch is certain and I cannot get compiler feedback yet.
E. **Test helpers.**
   - `bounded_document_store_owners::<P, M>()` calls in the shooting and equation tests (`install_document_store_owners_exact(...)`) become `store::funded_bounded_artifact_store_owners::<P, M>().expect(...)`. `install_document_store_owners_exact` now returns a `Result`.
   - The shooting disposer drain uses `ArtifactOwnedDisposer::close_step` with a grant. That is already migrated.

## Out-of-scope errors seen

- `semio-framework-tool-run`, `♻️retirement/🦀️.rs:3-8`: conflicting `RetireOwned` impls for `ToolRunIdentity`, `ToolRunState`, `ToolRunStepKind`, `ToolRunStepArg`, `ToolRunCounter`, `ToolRunTraceOp`.
- `semio-framework-tool-run`, `👥️entities/📸️version/🦀️.rs:38`: `RetainedCloneSource<_>` generic mismatch.
- The plugin framework file `🔌️plugin/🦀️.rs` still has `ArtifactStoreInitializationAuthority::step -> StepOutcome` (see C).
- The framework composition test `🧪️tests/🧩️composition/🦀️.rs` contains spliced old and new code around line 320. It looks mid-edit by another owner.

## Next, once unblocked

1. Re-run the slot-gated native and wasm32-wasip2 checks per crate.
2. Fix derive fallout (item 1).
3. Apply A, D and E.
4. Do B and C after the answers above.
5. Run the unit tests.


## Round 3 (after decisions A to E in corrections #9), still nothing compiled

`os.status` stayed RED (10:30) and `semio-framework-tool-run` still blocks every plugin crate, so this round is static only.

Applied:
- **A, owner hooks.** Deleted 23 old-shape `build_{document,config,draft}_store_owners` overrides in mathematical, shooting, bim, architect, norm, imperative and the writer viewer and editor. The trait defaults apply now. bim was touched only where the old override no longer compiled (the viewer config hook).
- **E, test helpers.** `bounded_document_store_owners()` became `store::funded_bounded_artifact_store_owners::<P, M>()` with the `Result` from `install_document_store_owners_exact` handled. `close_owned_step(1, bytes)` loops became self-funded `RetainedCloneGrant` loops in the shooting wasm and binary tests and the mathematical and writer standalone stores.
- **D, equation preparation.** `begin_batch_digest` is added via `store::admit_artifact_batch_digest`, with the `ArtifactCanonicalJsonTree` bound. `begin` was already three-generic with request handback.
- **Writer host-owned module.** 1257 lines became about 600.
  - **Removed:** the bespoke retirement structs, the bespoke initializer authority and its phase machine, and `writer_document_store_owners` as a hook. `writer_document_store_owners()` remains only as a funded helper for tests and `new_writer_store`.
  - **Replaced:** retirement now uses the generic `OwnedValueRetirementFactory`.
  - **Kept and migrated** to the grant protocol (`close_demands`, `next_close_*_demand`, `artifact_retirement_box_close_step` / `artifact_retirement_admit_owned`): the three envelope decode authorities (snapshot, mutation, SPR conflict) and the field catalog with `FactoryPayloadRetirement`. This is Writer's real semantic envelope decode; it stays.
  - **Initializer.** The editor no longer overrides `build_document_store_initialization_job`. It uses the framework bounded initializer. If rn-os decides Writer's custom initializer is semantic, this needs to come back. My reading is that it was a copy of the generic replay with `text` paging, and main should veto this if not.
- **Writer editor command job.** The 700-line bespoke `WriterCommandToolJob` (hand-written `InteractiveJob`) is replaced by `WriterRetainedCommandWork` as `ArtifactCommandWork`, running on the framework `ArtifactRetainedCommandJob`, the same shape the equation editor uses.
  - **Kept:** all Writer logic. The admission caps and the emit are free functions, `writer_command_admitted` and `writer_command_emit`, with identical behaviour.
  - **Changed:** `build_tool_job` and the factory use `ArtifactRetainedCommandPayload`.
  - **Behaviour caveat:** the bespoke job's rejected-completion and parent-allocation return handling is now the framework job's.
- **Writer artifact preparation.** The bespoke one-item preparation (about 150 lines, old one-argument shape) is replaced by `bounded_config_store_one_item_preparation_factory::<WriterSnapshot, WriterMutation>("writer-artifact-retained", ...)`, as shooting does. Divergence: the old admission restricted the lane to EditText and SpliceText with a 4 KiB cap; the shared factory admits any Writer mutation within the store ceiling.
- **Writer tests.**
  - **Rewritten for the generic path:** snapshot and mutation retirement, the edit-history decoder, and the hub-tail reload (the old law initialized through the bespoke initializer).
  - **Rewritten for the new free functions:** the three text-admission tests.
  - **Removed with their code:** the tests of the bespoke preparation and the bespoke job's return and completion handling.
  - **Cleaned up:** deleted `📦️parent-return.json` and the `artifactPreparation` key in the writer-migration fixture. Added `close_retirement` and `retire_writer_envelope` test helpers in the host-owned module.

Still open (static, nothing compiled):
1. **B, window configs.** mathematical graph, architect (register, graph, report, adjacency), norm results, bim macro and writer main need `Edit`, `MAXIMUM_PREPARATION_DEPTH` and `build_retained_edit()`, waiting on the public generic whole-record replace edit from rn-os. The writer main window config is the same.
2. **Writer window transient.** `🫧️transient/📢️publication/🦀️.rs` implements `ArtifactEphemeralPreparationTask` with the old `SnapshotRetirementStep` close; it needs the new grant-based task trait. The partial and transient tests in that folder have the same dependency.
3. **mathematical `EquationRetainedCommandWork` etc.** are migrated but unverified.
4. **Verification** is outstanding for every crate in scope.


## Round 4 (corrections #18), still nothing compiled

**Writer artifact preparation, decision (supersedes the Round 3 note).** The old lane's cohort (exactly `EditText` and `SpliceText`, at most `MAX_WRITER_COMMAND_TEXT_BYTES` = 4096 per edit, base at most 32768 bytes) is a deliberate lane contract. Every Writer tool that publishes on the `Artifact` lane emits only those two mutations. The writer-migration fixture pins `artifactPreparation` with `maxBaseBytes`, `maxEditTextBytes`, `workItemsPerAdvance` and `sealedByStore`.
- **Kept.** `WriterArtifactPreparationFactory` wraps the shared bounded factory. It refuses other lanes and other mutations in `preflight`, refuses an over-cap edit, and refuses an over-sized base in `begin`. It delegates every other trait method, including the batch digest, wire source, schema parts and stamped clock and mutation identity. The shared factory still supplies the actual retained-clone preparation.
- **Tests and fixture.** I restored the `artifactPreparation` fixture key and its assertions. A new test checks that cohort, cap and lane are enforced and that the footprint declares forward plus inverse rows.

**Writer transient, decision.** I did not replace `bounded_transient_preparation_factory`. The plugin's own bounded factory is itself half migrated (its `advance` still returns `String`). The partial-construction law is also deliberate: a 12 KiB UTF-8 base is rebuilt with one-byte copy grants. I kept Writer's own task and migrated it to the new `ArtifactEphemeralPreparationTask` plus `ErasedSnapshotRetirement` contract. Each turn pays reservation, copy page and Arc root allocation from its own grant, with exact ownership receipts. The test uses five-currency grants and RetainedCloneStep close.

**Window configs: NOT migrated. They need a decision, because `WindowConfigReplaceEdit` does not fit them.** All of the following need either a change in framework or a wire-surface change that I should not make unilaterally:
1. The replace edit needs `S: Copy` and a mutation that carries the whole record (`replacement(&self) -> &S`, `restoring(Box<S>) -> Self`). Only 5 of the 8 owner families in my scope have a Copy state:
   - math graph (`EquationCamera`)
   - writer main (camera, editor settings)
   - norm results (`Option<u32>`)
   - architect graph (`Viewport2d`)
   - architect adjacency (`Option<AdjacencyKind>`, assuming Copy)
2. These three have String state and cannot use it at all: architect register (`active_register: String`), architect report (`EntityId(String)`) and bim plan (`storey: String`).
3. Even for the Copy states, every mutation today is a field set: math `SetCamera`, architect `SetViewport` / `SetAdjacencyKindFilter`, writer main `SetCamera` / `SetEditorSettings`, norm `ChangeSelectedCheckIndex`. None holds a whole record, so none can implement `replacement()`. Making them whole-record replaces changes their wire vocabulary, their ts/proto/graphql schemas and their fixtures. The only exception I see is the single-field configs, where the variant payload can become the config itself and stay byte-identical, as #13 prescribes for named variants. I did not do this speculatively.
4. The bim `Replace { config }` named variant is also refused by the `CanonicalJsonTree` derive (#13). That macro is the bim agent's active work.
5. Writer main has three further mutation kinds.

**Proposal for rn-os.**
- **Field-mutation edit (Copy states, no wire change).** Add a sibling edit, `WindowConfigApplyEdit<S, M>`, for `M: WindowConfigApplyMutation<S>` with `apply(&self, post: &mut S)`, `inverse(&self, base: &S) -> Self` and an optional `admissible()`. It would have the same four-turn cursor as the replace edit and cover math graph, writer main, norm results and the architect graph and adjacency windows with their current mutation shapes. The named-variant enums still need the #13 newtype restructure for `CanonicalJsonTree`; that I can do per type once the edit exists.
- **String-bearing states.** The architect register and report windows and the bim plan window need a bounded String clone edit. That needs another primitive or a decision to carry these states as `PagedUtf8`.

Please answer: (a) will rn-os add `WindowConfigApplyEdit`, and (b) how should String-bearing window states be edited? I will migrate all eight owner families as soon as the edit exists.

Still unverified: everything. Verification is gated on `os.status` and the plugin crate compiling.


## Round 5 (corrections #35 field-set window edit), still nothing compiled

**Window configs migrated, all eight families, source-level only.** Installed `type Edit = WindowConfigApplyEdit<State, Mutation>`, `MAXIMUM_PREPARATION_DEPTH = 64` and `build_retained_edit()`. State and mutation types gain `RetainedClone` (states also `RetireOwned`), and each mutation implements `ConfigApplyMutation::exchange`.
- **mathematical graph.** `EquationCamera`, `EquationGraphWindowConfig`, `SetCamera` and the mutation enum gain `RetainedClone`. `exchange` swaps `camera` and refuses non-finite values through `admissible`.
- **architect register, report, adjacency, graph.** The named variants (`SetActiveRegister`, `SelectReport`, `SetAdjacencyKindFilter`, `SetViewport`) became newtype variants over payload structs with identical camelCase field names. This is #13, so the wire stays byte-identical, but the byte-identity test vs serde_json is still owed. `EntityId` and `AdjacencyKind` gained `RetireOwned`, `RetainedClone` and `CanonicalJsonTree`.
- **norm results.** The shared state gained `RetireOwned` and `RetainedClone`. The mutation enum and `ChangeSelectedCheckIndex` gained `RetainedClone`. `exchange` swaps `selected_check_index`. The owner macro carries the edit, so all 15 norm artifacts get it.
- **writer main.** `WriterCamera` and `WriterEditorSettings` gained `RetireOwned`, `RetainedClone` and `CanonicalJsonTree`. The config, `SetCamera`, `SetEditorSettings` and the enum gained `RetainedClone`. `exchange` swaps the camera and the editor settings.
- **bim macro.** `Replace { config }` became `Replace(Replace)`, a payload struct with the same wire field `config`. The macro derives `RetainedClone` and `CanonicalJsonTree` for it and implements `exchange` (`mem::replace(post, config)`). The kit `window_config!` macro and both viewer configs gained the derives. The seven bim `Replace { config }` test sites were updated. Only the macro and its callers were touched; nothing else in the bim agent's area.

**Not done: Writer initializer restoration.**
- A faithful port is about 600 lines: the Jack close ladder (displaced workspace, runtime, clone, envelope with its catalog, bare catalog, actor, fault with its publication), plus Writer's field-paged initial-snapshot clone. That clone traversal must now be sealed through `RetainedCloneSource::admit_borrowed` (#24).
- It cannot be verified until the plugin crate compiles, and #26 says fd-init's generic replay initializer will replace it. Writer's initializer was a plain replay: validate, seed, fold, apply, commit, with paged text.
- Writer currently runs on the framework default initializer, which refuses. I recommend that fd-init's generic initializer be treated as the Writer path; if main wants the throwaway port anyway, it should go to an executor with working compiler feedback.

**Finding: nine other sites will refuse at runtime.** `bounded_config_store_one_item_preparation_factory` always refuses in `advance` (INTENDED upstream). It is used at: imperative ×2, bim ×2, shooting ×2, architect ×2, norm ×1 and the Writer wrapper (which delegates to it). stdio-B has a working generic `diff_apply_preparation_factory` built on `store::OneItemOwners`, but it lives in a stdio crate. #14 forbids per-plugin copies. Request: move `DiffApplyPreparationFactory` into the plugin or store crate as a public generic. I will then swap every site, and the Writer wrapper's `inner`, in one pass.


## Round 6: compile rounds started, PARKED at usage limit (no in-flight edit; all files parse as last written)

Helper: `.🧬semio/🦑️repo/⚡️cache/play-fleet/mg-rest/check.sh <name> <manifest> <package> [cargo args]` runs the slot-gated `cargo check` and writes `<name>.log` beside it. The last line of each log is `exit N`.

Fixes made this round (all source-level):
- `AppOperationContext` gained a `retained` field. Every builder in scope now sets `retained: request.retained` (math, writer, shooting, bim ×2, architect, norm app-surface, imperative). The math and architect test one-liners use `Default::default()`.
- norm `norm_artifact_store_preparation` gained the `store::ArtifactCanonicalJsonTree` bound. It still binds the refusing bounded factory until the #37 store generic exists.

Results:
- `semio-s-artifact-norm-contract --lib` native: `Finished dev profile` (exit 0).
- `semio-s-artifact-norm-contract --lib --target wasm32-wasip2`: `Finished dev profile` (exit 0).
- `semio-s-artifact-norm-en1990 --lib` native: `exit 101`, 1 error in `semio-framework-plugin` (out of scope): `🔌️plugin/🦀️.rs:18728:307 E0308 expected Option<Arc<[u8]>>, found Option<Arc<Vec<u8>>>`. Another owner is editing that file.
- shooting, architect, writer, mathematical, bim and imperative-procedure `--lib`: blocked by red stdio crates (stdio-contract 8 errors incl. `PluginCloseStep` and `AppOperationContext.retained` in stdio-B scope; stdio-binary 9; stdio-semio 196). They are not my scope.
- imperative extensions (control, effect, logic, math, text) and the remaining norm artifacts have not been run.

Next steps after resume:
1. Re-run `check.sh` for norm-en1990 and the other 14 norm artifacts. All except din18599 have no stdio dependency. Then wasm32 with the component features, `--tests` (not the `artifact-app-testing` ones), and `cargo test --lib --no-fail-fast` (manifest-path for non-root-member crates).
2. Run the imperative extension crates; they have no stdio dependency.
3. When stdio is green, run shooting, architect, writer, mathematical, bim and imperative-procedure.
4. Pending in scope, unchanged:
   - document-lane preparation factories (#37) at the nine call sites plus the Writer wrapper;
   - Writer initializer, which is fd-init's generic replay;
   - a serde_json byte-identity test for the architect newtype variants;
   - possible `CanonicalJsonTree` or `RetainedClone` fallout on nested snapshot types.


## Round 7: compile rounds (parked on budget rule #45; background jobs killed, no half edits)

Helpers in `.🧬semio/🦑️repo/⚡️cache/play-fleet/mg-rest/`:
- `check.sh` and `check2.sh` run the slot-gated `cargo check`. `check2.sh` uses a second target dir.
- `test.sh` runs `cargo test --lib`.
- `fixloop.sh <artifact>` loops check then `fix_traits`. The `fix_traits.py` script (in the session scratchpad) adds the missing `RetireOwned`, `CanonicalJsonTree` and `RetainedClone` derives named in E0277 errors.
- Logs are `*.log`, and the last line is `exit N`.

Results (native `--lib` unless noted):
- **Imperative extensions.** Logic, effect, math, text and control: `Finished dev` and exit 0. Their unit tests pass: logic 1, effect 3, math 1, text 4, control 1, all `0 failed`. They needed the neural `Operator` plan methods (`step_plan` plus four demand methods, copied from the flow text extension).
- **Imperative extensions, wasm32-wasip2.** All five fail with 11 errors each. `ExtensionBundle::handler` now takes `fn(&[u8], &mut StepContext) -> Result<ExtensionInvokeStep, Fault>`. The imperative SDK `evaluate_invoke` (`✏️s/🔨️modules/📜️imperative/🧩️extension_sdk`, not my scope) needs the incremental evaluation-resources port that flow has (`ExtensionEvaluationResources`, `evaluate_invoke_json`). The other wasm errors come from the plugin `__semio_plugin_actor_exports!` macro (`IoRunControl` 5-argument call). Cross-scope: main.
- **norm contract.** Native and wasm32 `Finished`, exit 0.
- **norm en1990 and en1991.** Native `exit 0` after derive fixes, the `Insert*{index: Some(index)}` inverse fix, and the `io::` import fix.
- **norm en1992.** Last state: 9 errors, none a derive gap.
  - `.info(..)` takes 2 arguments (`LocalizedLabel` and `InteractiveJobClassification: AsRef<str>`). These come from an editor API change at `✏️editor/🦀️.rs:244` and `:288`.
  - `diff/🦀️.rs:463`: `En1992Snapshot` not in scope.
- **norm en1993 and the remaining artifacts.** The loop was killed during en1993 iteration 1, so en1993 onwards were not run to green. Rerun `fixloop.sh` for each; it is idempotent.
- **shooting.** Blocked by `semio-s-artifact-stdio-png` (2 errors, `🧬️schema/⚙️operations/🦀️.rs:239`, an argument-count mismatch in stdio-B's png).
- **architect, writer, mathematical, bim, imperative-procedure.** Check runs started; last observed exit 101 with the cause not yet read (`c2-*.log`), and the runs were killed. Re-run.

Source changes this round beyond Round 5:
- `semio_framework_plugin::WindowConfigApply*` became `semio_framework_plugin::app::WindowConfigApply*`, because the root does not export them (8 files).
- Plugin names defined in `os_kernel::io` (`Analysis`, `AnalyzeSource`, `ComposeError`, `ComposeSource`, `Composition`, `ComposerEntry`, and others) now use the `semio_framework_plugin::io::` path (about 20 files).
- Derive fallout on norm snapshot, action and enum types (dozens of types) was auto-fixed from compiler messages.
- `Insert*` inverse constructors use `index: Some(index)`.
- Imperative extension `Operator` impls gained the plan methods (4 crates).

Still open and not started:
- The nine refusing preparation sites plus the Writer wrapper `inner`.
  - Document lanes use `store::mutation_apply_preparation_factory::<S, M>()`. It needs `S: RetainedClone`, so the snapshots and their nested types need `RetainedClone` derives.
  - Config lanes use `store::snapshot_clone_preparation::config_apply_preparation_factory::<S, M>()`. That needs `RetainedClone` plus `ConfigApplyMutation::exchange` for the architect config (1 variant), imperative config (3) and shooting config (6).
  - bim's config is `NoConfig`, so its config lane becomes `None`.
- Window config byte-identity test (architect newtype variants).
- wasm32 with component features and `--tests` for everything.


## Round 8: release path (lib native, wasm32), parked again (jobs killed, nothing mid-edit)

Done in source:
- **Imperative evaluation resource.**
  - `ImperativeEvaluationResources` is now an `ExtensionResourceOwner` in `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️.rs` (the SDK crate gained `semio-framework-plugin` and `semio-framework-job` dependencies).
  - All five extension bundles use `.resource_owner(...).owned_handler(IMPERATIVE_MODULE_EVALUATE_CAPABILITY)`.
  - Each invoke is one bounded turn, builds a `ColdOwner<Registry>`, and returns an exact receipt.
- **Neural `Operator` plan methods.** `step_plan` plus four demand methods were added to:
  - the five extensions;
  - `semio-s-imperative` registry `ContributedExtensionStub` (`✏️s/🔨️modules/📜️imperative/📇️registry/🦀️.rs:84`), requested by main;
  - the test operator in `⚙️engine/🧪️tests`.
  - `semio-s-imperative` is not yet re-checked. The plugin lib had a transient error at the time of my last check (`🔌️plugin/🦀️.rs:465` `E0753`, later `MountedOwnerPhaseV1` not found), caused by someone else's in-flight edit.
- **Math equation preparation.** `ArtifactStoreOneItemPreparationStep::Prepared` now takes `(checkpoint, ownership)`.
- **Norm.**
  - Removed the stale orphan `.action_interactive_job(InteractiveJobClassification::Migrated)` and `.action_describe(LocalizedLabel::native("Replaces the whole ...` lines (12 lines across editor files).
  - Added `use crate::EnXXXXSnapshot;` to the en1992, en1994, iso16757 and vdi3805 diff files.
  - `Insert*` mutations: the label and target use `index.map_or_else(.., "end")` only where `index` is `Option<usize>`. Five files with a plain `usize` index were restored.
  - Derive fallout is auto-fixed from compiler messages.

Results:
- **Imperative extensions (logic, effect, math, text, control).** Native `--lib` exit 0 and 10 unit tests pass. The wasm32-wasip2 handler errors are gone. The remaining 10 errors per extension all sit at the `extension_exports!` call and come from the plugin macro `__semio_plugin_actor_exports!` (`🔌️plugin/🦀️.rs:~46661`): stale `reactor::jobs` signatures (`IoRunControl`, `SqliteSnapshotControl`, `JobStep`). That is plugin-owner scope.
- **Norm native `--lib` green:** contract (also wasm32), en1990, en1991, en1992, en1996, en1998, din4108. Last loop states:
  - en1993, en1994, en1995, en1997, en1999 were red (derive fallout plus the label/index issue); I have since fixed the causes but not re-run them.
  - din16798: 2 errors, `mismatched types` in `remove-zone/↩️inverse` and `remove-vent-system/↩️inverse` (index types).
  - din18599, vdi3805, iso16757: not completed.
- **Mathematical equation:** 8 errors remain. They are `EquationGraphDsl` and `EquationPoint` `RetireOwned`, which the auto-fixer now covers, and the `pack::record_spec_producer`, `reconstruct_record_controlled` and `record_controlled_semantic` calls in the binary snapshot, which are missing from `semio-framework-pack`.
- **Architect:** blocked by `semio-s-artifact-stdio-xlsx` (8 errors: `OpcPackage`, `OpcDiff`, `XlsxXmlPart(Diff)` lack `RetireOwned` or `CanonicalJsonTree`; stdio-b).
- **Shooting:** 141 errors at last read. Derive fallout is auto-fixable; the remainder is:
  - `spr` and `pack` crate aliases not found;
  - `set_shot_selection`, `set_camera`, `set_defaults` and similar module imports unresolved in the config mutations path;
  - `IoPayload` not at `semio_framework_plugin` root (use `io::IoPayload`).
  - Re-run `fixloop2.sh` for these.
- **Writer:** the auto-fixer cleared 21 derive gaps. Two errors were left at iteration 2 and are unread. The latest re-checks hit the plugin lib's transient error.
- **Bim:** `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🦀️.rs:7335: unexpected closing delimiter` (the file is unchanged vs HEAD, so the monolith file has an existing brace imbalance; the bim agent's file).
- **Imperative-procedure:** blocked by `semio-s-imperative` (now fixed in source, needs a re-check).

Cross-scope items for main:
1. `pack::record_spec_producer`, `pack::reconstruct_record_controlled` and `pack::record_controlled_semantic` are missing from `semio-framework-pack`. Forms worked around it with a local `📦️pack` module per plugin, but #14 says not to copy. Several binary-snapshot plugins are affected.
2. The `__semio_plugin_actor_exports!` wasm macro has stale job signatures (blocks wasm32 of every extension).
3. The plugin lib needs to be consistently green again (someone's in-flight edit).
4. stdio-xlsx derives block architect.
5. The bim monolith root file has a brace imbalance.

Next steps: re-run the helpers (`check.sh`, `check2.sh`, `fixloop.sh`, `fixloop2.sh` in the play-fleet mg-rest cache) for norm en1993..iso16757 and din16798, then shooting, writer, mathematical, imperative core and procedure. After that do the preparation-factory swaps, wasm32 for norm artifacts, and tests.


## Round 9: parked (background runs stopped, no half edit)

Done this round:
- **Bim root.** Fixed `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🦀️.rs`: removed the stray fragment `od inverse;` plus one extra `}` (old lines 6395-6396, inside the `set_element_workset` / `create_clash_set` leaf region). A brace scan now shows zero mismatches and depth 0 at the end. The cargo result for bim has not been read yet.
- **`semio-s-imperative`** (imperative core) native `--lib`: exit 0.
- **`semio-s-artifact-imperative-procedure`** native `--lib`: exit 0, after two fixes:
  - `ColdRetire` is not implemented for `BTreeMap`, so the seed is retired by iterating over its values (in `⚙️engine` and in the artifact root).
  - The empty `ProcedureMutation` enum cannot use the `CanonicalJsonTree` derive (it generates `match self {}` on a reference), so it has a manual impl with `match *self {}`.

State of the rest, unchanged since Round 8 except as noted:
- **Norm:**
  - Native `--lib` green: contract (also wasm32), en1990, en1991, en1992, en1996, en1998, din4108.
  - Red at the last loop, causes since fixed but not re-run: en1993, en1994, en1995, en1997, en1999.
  - din16798: 2 index type mismatches in `remove-zone` and `remove-vent-system` `↩️inverse`.
  - din18599, vdi3805, iso16757: not run to green.
- **Writer:** the loop (`fixloop1.sh wr ...`) was killed mid-run. No result was read this round.
- **Bim:** the same for `fixloop2.sh bim ...`. Its first iteration showed 4 errors, with the cause not read.
- **Mathematical, shooting, architect:**
  - Mathematical: 8 errors at last read, including the missing `pack::record_*`.
  - Shooting: 141 errors at last read.
  - Architect: blocked by stdio-xlsx.
- **Corrections #50.** `EquationCommand` gets `RetireOwned` from `app_commands!`. Nothing needs deleting on my side: no manual `RetireOwned` for `EquationCommand` exists in my scope (searched).
- **Preparation-factory swaps:** not started.

Next steps after resume, in order:
1. Re-run `fixloop.sh <artifact>` for the norm artifacts en1993, en1994, en1995, en1997, en1999, din16798 (fix the two inverse type mismatches first), din18599, vdi3805 and iso16757.
2. Run `fixloop2.sh bim` and `fixloop1.sh wr` (writer). Read the 4 bim errors and the 2 writer errors left at the last full read.
3. Mathematical and shooting: re-run after the derive auto-fixer, fix the shooting module/import issues, and await the pack helpers from core-tests.
4. Preparation swaps: document lanes to `store::mutation_apply_preparation_factory`, config lanes to `config_apply_preparation_factory` (derive `RetainedClone` plus `exchange` for the architect, imperative and shooting configs), bim config lane to `None`.
5. wasm32 `--lib` for the norm artifacts, then `--tests` and unit tests.
