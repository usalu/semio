# mg-procspace — retained-clone protocol migration of procedural + space

Scope: `✏️s/🔌️plugins/🌀️procedural/**`, `✏️s/🔌️plugins/🪐️space/**` (plus the permitted `store::OneItemOwners` file and 3 lines in the store root).

## Status

SOURCE-LEVEL ONLY. Nothing below has been compiled or run: `os.status` was never `GREEN os` while this report was written (plugin crate under reconstruction, see corrections "rn-os 11:50 FINDING"). A syntax-only pass (`rustfmt --check` with `skip_children`) over all 224 changed `.rs` files plus the three new files is clean. Cargo check / wasm check / unit-test counts are therefore NOT reported yet; this file is updated when the gate opens.

## What was migrated

- Shared owners: `store::OneItemOwners<P, M>` (new, `🏪️store/📬️one-item-owners`) replaces every per-plugin one-item owner copy; usage appended to the corrections file.
- Config mutation enums with named-field externally tagged variants (home, space-index, generation2d) restructured into newtype variants over payload records with byte-identical serde wire, each with a serde_json identity test; trees derived; `begin_batch_digest` implemented normally.
- Generation2d (editor, host, preview-eval run job, config, apply, presence): instance owner, import job (new `step`/`borrow_outcome`/`RetainedJobPublication`), store and config preparations on `OneItemOwners`, replay displacement as `Generation2dReplayDisplaced: RetireOwned`, initializer authority (BuildOwners phase, closer, factory close), copy cursor `close_demands`/`abandon`, envelope snapshot/mutation/SPR-conflict authorities with the four demand methods, bespoke retirement factories replaced by `OwnedValueRetirementFactory`/`SharedValueRetirementFactory`.
- Generation3d: the same set for editor, viewer and host (3.5k-line host file), plus: command works (`work_demands`, `terminal_frame_release_bytes`, grant-based close ladders including emit/presence `ControlledRetirement`, host `FlowHostRetirement`, eval session), instance owners (`retirement_demands`/`maintenance_step`/`close_step`), mutation-session flow frontier on the new `FlowRetirement::{push,step}` API, document store fixture on `funded_bounded_artifact_store_owners` + `close_owned_unscheduled`.
- Config lanes of generation3d editor and viewer now mount `store::snapshot_clone_preparation::config_apply_preparation_factory` (corrections #36): leaves/enums/configs derive `RetainedClone`, `ConfigApplyMutation::exchange` implemented. generation2d, home and space-index keep their `OneItemOwners`-based config preparations.
- Presence: `impl store::ArtifactPresenceSnapshot for {Generation2dPresence, Generation3dPresence, Generation3dViewPresence, HomePresence} {}` (corrections #34).
- Home / space: transient root retirement in 4096-byte slices, command works (`checkpoint_byte`, `work_demands`, close ladders), disposer/presence/owners overrides removed where trait defaults suffice.
- Procedural core sqlite native value: ordered-map retirement and insert driven with `RetainedCloneGrant`.
- Tests: per-crate `🧪️tests/🔬️retirement-driver` (exact-grant close/step helpers) in generation2d and generation3d; retained-authority-laws, editor unit tests, work-capacity, fold-contract, artifact-surface, text-snapshot, store-fixture, home viewer and preview-eval tests ported to the grant API (10 `StepContext::new` call sites now carry the retained grant and receipt).

## Intentionally unchanged (still the pack library's count-based protocol)

`store::mounted_pack_rt::{RetainedPackCloseStep, RetainedTypedPackCloseStep}` session closes inside the envelope authorities and `Generation*MutationSession` (the pack crate API did not change); `semio_framework_ui_contract::BuiltTreeRetirement::close_step(1, 4096)` in four space/home window unit tests (ui crate API did not change).

## Cross-scope dependencies (also in the corrections file)

1. `CameraJson` (flow crate snapshot.rs:165) needs `RetainedClone` for the generation2d/3d configs.
2. `BoundedArtifactCommandWork` (plugin crate retained-command) implements neither `work_demands` (default refuses) nor `terminal_frame_release_bytes` (default `None` makes the job close refuse); space mounts it.
3. `Option::None` clone bug in `OptionCursor` (#36) affects `Option<String>` config fields.
4. fd-init shared replay initializer: the generation2d/3d store initializers are ported in place (#26) and may be swapped later.

## Unverified items to watch at the first compile

- wire literals of the three payload-record wire tests;
- `RetireOwned` inference on `Snapshot`/`Command` associated types;
- `retire_shared_for_test` lease semantics with a zero grant (`Progress(default)` assumed);
- `FactoryPayloadRetirement` derive on generic `DiffApply…` was removed with the generic; none left;
- `Generation3dViewConfig`/`Generation3dConfig` `RetainedClone` derive over `[f64; 3]` fields.

## Check / test results (compile rounds started after `GREEN os 11:30`; PARKED at usage limit)

Runner: `/private/tmp/claude-501/-Users-ueli-Documents-semio/0f187895-1699-4e68-8c53-7e6030677f5a/scratchpad/mgps/chk.sh <manifest-dir> <crate> <logname> <cargo args>` (slot-gated, logs in `.🧬semio/🦑️repo/⚡️cache/play-fleet/mg-procspace/<logname>.log`, last line `exit=N`). No source edits are in flight; all touched files parse.

| crate | command | result |
|---|---|---|
| `semio-s-space-core` (`✏️s/🔌️plugins/🪐️space/🫀️core`) | `check --lib` native | `Finished dev … in 1m 15s`, exit=0, 0 errors (log `core-native`) |
| `semio-s-artifact-space-home` | `check --lib` native | blocked, exit=101: dependency `semio-s-artifact-stdio-contract` had 8 errors at 12:xx (media-export `PluginCloseStep`, part21 `RetireOwned` conflicts, `AppOperationContext.retained`, `ValueError: From<&str>`); the `PluginCloseStep` one has since been fixed by its owner; NOT re-run |
| `semio-s-artifact-space-space` | not run (same stdio-contract dependency) |
| `semio-s-artifact-procedural-generation2d` | `check --lib` native | blocked, exit=101: after the stdio fix the only error is `playbook/🦀️.rs:360:95 E0277: OrderedMap<DslValue>: ArtifactCanonicalJsonTree is not satisfied` (two re-runs, 1 error each, log `g2d-native`); no error in procedural/space sources reached yet |
| `semio-s-artifact-procedural-generation3d` | not run (same playbook blocker) |

Earlier flow-crate errors (4x E0277 in `flow/🧬️schema/📸️snapshot/🦀️.rs`) disappeared between the two runs; the remaining blocker is the pack-json `OrderedMap` canonical-tree impl (value-core / os-domains, corrections "os-domains 12:0x").

## After resume (playbook/flow GREEN)

generation2d native `check --lib` still exit=101, all 5 errors outside my scope, in stdio crates that generation2d (and home/space) depend on: `semio-s-artifact-stdio-xml` (`XmlDoctype: ArtifactCanonicalJsonTree` unsatisfied, snapshot.rs:49/74, set-doctype:6) and `semio-s-artifact-stdio-pdf` (`canonical/🦀️.rs:8,16` E0106 missing lifetime specifier); earlier in the same session `stdio-txt` (`TxtSnapshot` not found, `receive_pack_rows`/`receive_text` missing) blocked and cleared. No procedural/space source error reached yet. Polled 9 times over ~25 min; stdio owners are mid-edit.

## Latest round (after stdio-zip green)

generation2d and generation3d native `check --lib` (logs `g2d-native`, `g3d-native`) both exit=101 on the same 2 errors, outside scope, first error: `semio-s-artifact-stdio-png` `…/🏅️standards/🔖️1.2/…/🧬️schema/⚙️operations/🦀️.rs:239:101` E0061 (function takes 3 arguments but 2 supplied; second at :239:25 takes 7 but 6 supplied). generation2d depends on stdio-png directly (cargo tree). zip/xml/pdf/txt blockers cleared earlier. home/space not re-run (same stdio dependency family). Still no procedural/space source error reached. Per corrections #45: parked, no polling.

## Release-path round (after stdio-png green)

- `semio-s-space-core` native `check --lib`: exit=0, 0 errors.
- `semio-s-artifact-space-space` native `check --lib`: exit=0 (`Finished dev … 56.60s`) after two in-scope fixes: `io/🦀️.rs` `semio_framework_plugin::ComposerEntry` -> `semio_framework_plugin::io::ComposerEntry` (also applied in generation2d, generation3d, home), and `diff/🦀️.rs` stale `id: first.id.clone()` removed from `SSpaceArtifactPatch::absorb`.
- `semio-s-artifact-procedural-generation2d` / `generation3d` native: exit=101, first error outside scope: `stdio-semio` `…/🖊️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔄️dxf/🔖️r12/✳️any/🦀️.rs:23:63` E0432 unresolved imports from `semio_s_artifact_stdio_dxf`.
- `semio-s-artifact-space-home` native: exit=101, first errors outside scope: `stdio-xlsx` (15 errors; `OpcPackage`/`OpcDiff: RetireOwned` unsatisfied, E0061 arg counts in io snapshot files, E0053 `decode_sqlite_snapshot_native`).
- wasm32-wasip2 `--features component-app-assembly`: `space-core` and `space-space` exit=101, only outside scope: framework `os/🪐️space/🗿️artifacts/🪐️space` `SpaceUserPatch`/`SpaceCollectionPatch`/`SpaceExtensionPatch: RetireOwned` unsatisfied (`🦀️.rs:270-291`) and `🗂️collection` `CollectionFolderPatch`/`CollectionEntryPatch: RetireOwned` unsatisfied (`🦀️.rs:379-388`).

## Rule #46 derives + wasm re-run

Added `semio_framework_value::RetireOwned` derives (no manual impls existed): `SpaceUserPatch`, `SpaceCollectionPatch`, `SpaceExtensionPatch`, `SpaceOptionalAvatar` (`os/🪐️space/🗿️artifacts/🪐️space/🦀️.rs`), `CollectionFolderPatch`, `CollectionEntryPatch`, `CollectionOptionalLink` (`…/🗂️collection/🦀️.rs`). All RetireOwned errors of the space domain crates are gone. space-core and space-space wasm32-wasip2 `--features component-app-assembly` now stop at `semio-framework-os` (`os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:2897,2901,2934,2976,3052,3078,3082,3102,3181`): 9x E0433 `semio_framework_async` unresolved (missing crate dependency in the os host manifest, pl-c).

## Round after stdio-semio green / os host async fix

Green (exit=0): `semio-s-space-core` native + wasm32-wasip2 `component-app-assembly`; `semio-s-artifact-space-space` native + wasm32-wasip2 `component-app-assembly`; `semio-s-artifact-procedural-generation2d` native `check --lib`.

In-scope fixes made this round: `RetireOwned` derives on `SSpaceArtifactPatch`, `SpaceIndexPresencePatch`, `Generation2d{Widget,Synapse,Generation}Patch`, `Generation2dValueRow/ValuesDelta` and the gen3d twins; stale `id`/`artifact_id` fields removed from two `absorb` impls; `AppOperationContext.retained` supplied (`request.retained`) in 6 production sites + test literals; `semio_framework_plugin::io::{…}` paths for the composer/analysis types; `FactoryPayloadRetirement` on both envelope catalogs; displaced `Layout(WidgetLayout)`/`Json(DslValue)`; `retire_displaced(Option<DslValue>)`; `is_empty_delta` uses `patched()`; sqlite native decode ported to `NativeSnapshotDecodeOwner` + `allocation_stage_native` tuple closures; `#[cfg(test)]` split of central_apply test-only imports; stale `index: None` in the text mutation decoder; `ComposerEntry` etc. Added `impl RetireOwned for GenerationPlayRoot` in the playbook crate (cross-scope, flagged in corrections).

### Remaining blockers (all in procedural; not yet fixed)

1. `generation3d` native `check --lib`: 41 errors in the geometry inference engine and its io, caused by the new brep/mesh kernel and history APIs, not by the retained-clone protocol of my files:
   - `💡️inferences/📐️geometry/🗃️registry/{🔁️brep-transform,🏳️brep-surface,🛠️brep-feature,✂️brep-intersect,🐚️brep-topology,🧱️mesh-support,🔀️mesh-convert}` + `💎️value`: `ShapeValue` import, `Pipeline` instead of `Box<dyn WidgetJob>` (≈20 sites), `Arc<ShapeValue>`, `Arc<HalfedgeMesh> ==`, `MeshKernelError::Retained`, tessellation step now `(MeshTessellationStep, RetainedCloneProgress)` taking a `RetainedCloneGrant`, `BrepError: From<KernelError>`.
   - `🚪️io/📐️geometry/{💾️brep-interchange,📼️mesh-interchange}` same, and `🚪️io/🦀️.rs:244,294,390,391` `HistoryFoldIndex` vs `BTreeMap`.
   - `host/📐️geometry-service/🦀️.rs`: `new_contextual` needs the third (demands) fn, `ArtifactInferenceExecution.retirement_progress`, `Quality::to_value` import. The flow plugin's brep extension (`🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry`) already holds the migrated retained geometry inference (`GeometryInferenceContext` normal cursor + `geometry_inference_demands`); generation3d's engine should either be ported the same way or replaced by it.
2. `generation2d` wasm32 `component-app-assembly` (44 errors; generation3d expected similar): 
   - window configs `Generation2d{GeneratePreview,EditPreview,Main}WindowConfig` need `RetainedClone` + the new `WindowConfigOwner` items (`Edit`, `MAXIMUM_PREPARATION_DEPTH`, `build_retained_edit`; use `WindowConfigApplyEdit`, corrections #36);
   - command payloads `FlowEvalTick`/`FlowEvalResolve: RetireOwned`; `Generation2dPreviewTextChange` missing in the transient mutation;
   - `FlowHost`/`FlowEvalSession` grant API in `🧬️schema/🦀️.rs:116-197`, `set-eval-outputs`, `set-contributions`, `🧵️preview-eval/🦀️.rs:116-226` and `⏯️tool-run/🦀️.rs:333,345,416,549,667`: `FlowHost::new` returns `Result<(FlowHost, Progress), …>`, `arm_window_tick(window, grant)`, `set_eval_json(Arc<String>, grant)`, `tick`/`resolve_preview_eval` only as `tick_cold`/`resolve_preview_eval_cold` or the grant-driven steps.
3. `semio-s-artifact-space-home`: `stdio-xlsx` (B), not re-run.

## Out-of-scope errors

- `🌊️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/../../🦀️.rs:360:95` E0277 `OrderedMap<DslValue>: ArtifactCanonicalJsonTree` not satisfied (pack-json impl pending).
- stdio-contract (8 errors earlier, owners stdio-a/b): see table.

## Next steps after resume

1. Re-run `chk.sh … g2d-native check --lib` for generation2d; fix errors that land in procedural/space sources; repeat for generation3d, home, space artifact (stdio-contract must be green first for home/space).
2. `--target wasm32-wasip2 --lib --features component-app-assembly` for the same crates (space-core feature `component-app-assembly` too).
3. `check --tests` per crate (skip testkit-dependent tests until corrections says testkit GREEN).
4. `test --lib --no-fail-fast -- --test-threads=4` per crate, including the three wire-identity tests (home config, space-index config, generation2d config), then fill this table and message main.

## Round: release path final (generation3d handed to g3d-geo)

| crate | native --lib | wasm32-wasip2 component-app-assembly |
|---|---|---|
| space-core | green | green |
| space artifact | green | green |
| generation2d | green | green |
| home | green (exit=0, 0 errors) | green (`Finished dev profile in 1m 25s`, exit=0) |
| generation3d | handed to g3d-geo (not mine) | handed to g3d-geo |

Home wasm fixes: `ControlledRetirement::close_step` -> `step(grant)` in the create-studio effect close; `RetireOwned` derive on `HomePresenceMutation`.

Notes: the grant-driven preview-eval hop uses `tick_cold` / `resolve_preview_eval_cold` (no non-cold driver in the framework yet). Cross-scope flagged: `CameraJson` RetainedClone (os-domains), `BoundedArtifactCommandWork` work_demands / terminal_frame_release_bytes (pl-d), manual `GenerationPlayRoot: RetireOwned` impl (os-domains).

Not yet run (budget rule #45): `--tests` and unit tests incl. the three wire-identity tests. generation2d `--tests` currently shows 75 test-side errors from API drift (missing `apply_generation2d_mutation`, `Generation2dDiffRead`/`SnapshotRead` mismatches, sqlite snapshot test arities, a retained-authority-laws E0502 at line 230).
