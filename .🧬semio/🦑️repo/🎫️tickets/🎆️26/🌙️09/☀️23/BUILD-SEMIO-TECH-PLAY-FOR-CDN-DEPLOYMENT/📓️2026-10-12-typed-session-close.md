# 2026-10-12 typed-command session close: the refusing owner, a required retirement declaration, a fixture that names refusals

Executor: typed-close (Sonnet). All cargo ran in the foreground of its own slot through `🚦️cargo-slot.sh` with `SLOTS=2` and the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/typed-close/target` (deleted at the end; scripts, logs and result tables stay in `play-fleet/typed-close/`). Test binaries outside cargo ran with `RUST_MIN_STACK=67108864` (plugin lib) and `268435456` (hub editors, one process per test). No git-modifying command, no ticket operation, no sub-agent, no `🗑️generated`. A slot was the scarce resource (both held by `proc-describe-nx*` for ~1 h), so the cargo work was batched into three slot holds (`strict.sh`, `verify.sh`, `verify2.sh`).

## Result

| proof | before | after |
| --- | --- | --- |
| hub `editor_catalog`, 177 tests, one process each | 88 ok / 88 FAILED / 1 TIMEOUT (`cat-before-all`, current tree before any edit of mine) | **135 ok / 41 FAILED / 1 TIMEOUT**, 0 regressions, 47 newly green (`cat-after-all`) |
| the 19 `semio_*_editor` + `txt` + `md` controls | 0 of 22 | **22 of 22**; `gltf_any` still fails (below) |
| plugin `--lib`, one process per test (1230 tests, 6 parallel, 180 s) vs `pl-tests/r15/results.tsv` (951 ok) | - | 948 ok; 4 not green that were green in r15, **none reachable from this change** (below); +1 new law (green) |
| stdio-semio `preparation_law` | red 5/5 at 03:55 (`⚖️law/🦀️.rs:87`, close accounting `(544, 0)`) | **still red 5/5, identical `(544, 0)`**: the store law, not this change (below) |
| native `--lib` build of plugin tests, hub `editor_catalog`, stdio-semio `preparation_law` tests | - | Finished, 0 errors |
| `cargo check --target wasm32-wasip2 --lib` plugin + os-kernel | - | Finished, 0 errors |
| `cargo check --manifest-path ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml --all-features --lib`, native and wasm32-wasip2 | - | Finished, 0 errors |
| hub workspace (35 compositions) with the new requirement forced (`--cfg strict_retirement`, private dirs) | - | 0 errors |

The hub editors moved from 88 to 135 green in the same window in which pub-perf edited the store law, `mutation-apply` and the snapshot read registry (their files carry mtimes 03:12 to 04:14 and are in my "after" binary), so the +47 is the whole tree, not this change alone. What is mine is below.

## 1. Which owner refused

`WorkerJobSession::close_step` (plugin `🦀️.rs`, `MountedTypedCommandFullOperation`, two sites) only forwards `WorkerJobCloseStep::Refused{kind}` of the job it owns. The job is `ArtifactRetainedCommandJob` (`🧵️retained-command/🦀️.rs`). At `HEAD` its `close_step` ended with:

`if self.command.is_some()||self.snapshot.is_some()||self.config.is_some()||self.history.is_some()||self.interaction_state.is_some()||self.interaction_hover.is_some()||self.context.is_some()||self.operation.is_some(){return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::UnsupportedOwner,..}}`

(and `close_demands` answered the same inputs with `unadmitted_input()`, `UnsupportedOwner` "requires its original app-owned retirement authority"). The command, the snapshot/config/history/interaction leases and the operation are borrowed (`as_ref`) for the whole run, so a job that reached close with any of them still in place was refused, whatever the editor; only the context had a retirement lane (`context_retirement`). That is the `typed operation session retirement refused: UnsupportedOwner` of the 19 semio cases and of `gltf`/`txt`/`md`. (The version of the file between `HEAD` and 02:56 is not recoverable from the shared tree, so this is read from `HEAD`, not from the binary semio-pub measured.)

State of the tree now: the job closes those inputs through typed retirement lanes (`input_retirement`: `artifact_retirement_admit_owned` for command/operation/context, `artifact_retirement_admit_shared_lease` for the four shared roots; retained-command file mtime 02:56, i.e. between semio-pub's last hub measurement at 02:42 and its report at 04:00). I did not write it and did not touch the job. Evidence that the lanes remove the refusal: with the current tree every hub editor passes `detail action publishes` and reaches close (the old binaries `bin-after1` 03:41 and `bin-after2` 03:56 of pub-perf, run for `txt_editor`, no longer show `UnsupportedOwner` either). The new lanes make the old latent trap explicit: `artifact_retirement_owned_birth_demands` / `artifact_retirement_shared_lease_birth_demands` refuse with `UnsupportedOwner` ("typed retirement has no controlled owner authority") for any input type whose `RetireOwned::controlled_retirement_supported()` is false. So the remaining way for a typed command to fail to close is exactly the class fixed in 2.

## 2. The class: a hand-written `RetireOwned` that forgets the declaration

`RetireOwned::controlled_retirement_supported()` defaulted to `false`. A hand-written impl that did not override it compiled, passed every unit law that never asks, and was then refused by every factory that demands controlled retirement (`mutation_apply_preparation_factory::begin_demand`, `config_apply_preparation_factory`, the job input lanes of 1). semio-pub found 20 such impls in the semio crate root; the scan below finds 61 more.

Fix at the root, in the trait (`🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs`): **`controlled_retirement_supported` has no default.** An owner states `true` (derive, `artifact_retire_struct!`, `artifact_retire_leaf!` already do) or states `false` on purpose; forgetting it is a compile error naming the type. Rollout without a break window: a `#[cfg(strict_retirement)]` twin of the method, a full dry build of the hub workspace (35 compositions, all plugin crates) and of the root workspace with `RUSTFLAGS=--cfg strict_retirement` in private dirs (hub 0 errors; root: its pre-existing unrelated failures only, no `controlled_retirement_supported` error), then the flip.

The 61 hand-written impls that lacked the declaration (found by `scan.py`: brace-matched scan of every `impl ... RetireOwned for`, 372 impls over framework, `✏️s`, hub, teaching, 366 after the six derive conversions; 0 missing afterwards):
- 6 uninhabited enums became `#[derive(RetireOwned)]` and lost their hand impl: `Block2dPresenceMutation`, `Block3dPresenceMutation`, `Block5dPresenceMutation`, `Block2dConfigMutation`, `Block5dConfigMutation`, `WiresMutation` (the derive handles empty enums).
- 55 state `fn controlled_retirement_supported() -> bool { false }` (behaviour-preserving: that is what the default was). This is the explicit **uncontrolled-owner debt list**, greppable from now on:
  - framework: `MapDelta<V>`, `TextError`, `MeshPackEncodingJob`, `SpaceMutation`, `CollectionMutation`, `std::vec::IntoIter<T>` (a cursor with no birth quote, deliberate); tests: two `Unsupported` owners, one queue `Unsupported`, `HashProjection`.
  - block: `BlockCamera3d`; block 3d `KindIdentity`, `Attribute`, `Author`, `Compatibility`, `Representation`, `Camera`, `Meta`, `VortexKindExtra`, `VortexKind`, `Vortex`, `KitType`, `Transform`, `KitPiece`, `KitConnection`, `KitDesign`, `KitSnapshot`, `Block3dSnapshot`, `Block3dMutation`, `Block3dBrushPreview`, `Block3dWorldWindowTransient`, `Block3dWorldWindowTransientMutation`; block 5d `Block5dPart3d`, `Block5dGripTemplate`, `Block5dMutation`; block 2d `Block2dMutation`.
  - cad `CadReferenceIndex`; reasoning `WiresSnapshot`, `WiresCanvasTransientMutation`; sequence `StepParams` (documented fail-closed); writer `WriterEditorSelection`, `WriterMainWindowTransient`; generation3d `Generation3dViewTransientMutation`, `Generation3dPreviewWindowTransientMutation`; bim `BimPresence`, `BimPresenceMutation`; note `NoteCompositeWindowTransientMutation`; forms `SharedText`, `SharedChunks`, `TryValueContent`, `FormsTryValues`; layout `LayoutDropPreviewState`; puzzle `Puzzle{2,3,5}dWindowTransientMutation`.
  Each of the `*Mutation` / transient entries is a typed-command payload that the generic factories and the job input lanes will still refuse; they need `#[derive(RetireOwned)]` (and on the payload leaves) exactly like semio-pub did for the 18 semio subsets. The migration is mechanical and per crate; I did not do it here because it cannot be verified without compiling and running each of those apps, and a wrong `true` is worse than an explicit `false`.

Not done, on purpose: making the derive compute `controlled_retirement_supported` as the AND of its field types. A recursive or mutually recursive type (`DocBlock::Quote{blocks: Vec<DocBlock>}`, A holds Vec<B> holds Vec<A>) would recurse at runtime; it needs a coinductive guard first.

## 3. The fixture names refusals instead of spinning

`settle_registered_typed_operation` (`🔌️plugin/🦀️.rs`, `testing` laws):
- The first `Fault` result page it takes now returns `Err(fault)` at once (it used to be remembered and returned only after the loop drained, which a stuck close never reaches).
- After every publication unit it asks the app for a retained structural refusal and returns it at once as `interactive-job.owner-refused`. New `PluginApp::typed_operation_refusal()` (default `None`, `VcsArtifactApp` implements it): per live operation, `retained_structural_refusal()` = the operation retains a close `Fault` (`retained_close_fault`: the normal retirement failed and kept its cause), or its publication holds a preparation refusal / its fault close holds a refusal of kind `UnsupportedOwner` (a refused `begin` or a close that can never succeed on retry). Recoverable kinds (`OwnershipLimit`, ...) are not structural.
- The deadline message now carries `PluginApp::typed_operation_diagnostic()` (verb, stage, session, terminal, retained refusal per live operation) and states the real 40 s.
Existing behaviour kept: `Blocked`/`Ok(())` ladders still end at the 4096-unit `interactive-job.publication-stalled` fault, which the fixture now also returns at once (it is how the 28 editors of section 5 present).

Law (language-agnostic): corpus `🧵️retained-command/🪟️mounted/🚨️structural-refusal/🧫️fixtures/🔣️.json` (+ draft-07 schema). Rust law `test_retained_structural_refusal::<ToyRunApp>` (`🧪️tests/🔬️tool-run`, test `a_mounted_operation_reports_the_structural_refusal_it_retains`) reads it and checks four verdicts on a blank mounted operation (clean, recoverable refusal, `UnsupportedOwner` refusal, retained close fault). TS twin `🧪️tests/🟦️.ts` re-derives the verdicts without any Rust code and validates the corpus with Ajv (third-party): `bun test ./🧪️tests/🟦️.ts` 2 pass; the Rust law passes (also in the one-process-per-test run).

## 4. Plugin lib per-test comparison (r15 951 ok vs now 948 ok of 1230)

Nothing that this change can reach regressed. The four tests that were green in r15 and are not now:
- `time_travel_tests::a_mutation_row_offers_withdraw_and_restore_takes_an_accepted_draft_back`: flaky, passes twice in isolation (`left (false, 4)` only under 6-way load).
- `time_travel_tests::the_history_view_patches_only_the_rows_a_change_touched` ("a history edit rebuilds the whole view"): deterministic. It dispatches straight into the store (`apply_toy_edit` = `store.dispatch`), no settle, no retained command.
- `runtime_close_budget_tests::live_maintenance_caller_funds_exact_retained_physical_extent_without_more_work` (`(50425956, 25222528)` vs `(0, 0)` heap events) and `retained_window_input_tests::window_config_paged_registry_actual_transient_partition_custody` (`51015837` vs `25807991` allocated/released): deterministic heap-accounting laws over `runtime_lifecycle_step` / the window transient registry close; their own files predate r15 (window transient 10-10, test 10-11 12:32).
The change set is runtime-neutral outside the fixture: a trait requirement (compile time), `false` literals equal to the former default, derives on uninhabited enums (unreachable), fixture-only settle code and two defaulted trait methods only settle reads, plus one new test. After r15 these files changed under peers: `value` root, `value/♻️retirement/🎟️turn`, `job` root and `job/📬️outcome/📦️builder`, store `🦀️.rs`, `paged-one-item`, `snapshot-clone`, `canonical-edit`, `read/registry` (list from `find -newermt '2026-10-11 20:50'`). I could not build a pre-change binary of this shared tree to prove the attribution, so these four go to the owners as "red since r15 in the shared tree".

## 5. What the editors still stop at (not the typed-session close any more)

The 41 failures of `cat-after-all`:
- 28 `interactive-job.publication-stalled` ("advanced 4096 publication units without moving its ladder"): dwg x2, epw, gif89a, gltf, ifc x4, pdf17 x8, pptx x3, step x6, xlsx x3. Every log carries the same pub-perf line `[DEBUG] mutation-apply blocked: copy_needed=.. copy_grant=32768 clone_bytes=<267 KB .. 5.4 MB> capacity_grant=262144`: the fixture preparation policy grants 256 KiB of capacity and the initial snapshot of these editors clones to more. The refusal is real and now visible at once, but it is a grant/perf question of the `mutation-apply` clone (pub-perf), not a retirement one.
- 11 `needs a meaningful edit fixture` (docx x3, las, svg x3, tiff x2, xml x2), unchanged.
- 2 `jpg_any` / `jpg_baseline`: `complete editable source reopens: snapshot-edit.lossy-conversion` (new, past the settle).
- 1 `sqlite_snapshot_semio_any_editor` 200 s alarm (known debug timeout).

`preparation_law` (stdio-semio, 5 tests) is red inside the store law (`⚖️law/🦀️.rs:87`, a close step allocates 544 bytes its receipt does not report) with the same numbers as at 03:55, before any edit of mine; the law and `mutation-apply` were edited by pub-perf (03:12 / 03:51) and the read registry at 04:13. Owner: pub-perf.

## Evidence files

`play-fleet/typed-close/`: `cat-before-all/`, `cat-after-all/` (per-test logs and `results.tsv`), `plugin-run/results.tsv`, `strict-hub.log`, `strict-root.log`, `v-*.log`/`v-*.status`, `scan.py`, `apply2a.py`, `each.sh`.

## Debug hygiene

No `[DEBUG]` line added. The `[DEBUG]` lines in the logs are pub-perf's. Private `target`, `target-strict`, `build-strict` and the three test binaries are deleted.

## Files touched

- value: `🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs` (required method, `IntoIter`), `♻️retirement/🧪️tests/🔬️unit/🦀️.rs`, `♻️retirement/📋️queue/🧪️tests/🎮️ownership/🦀️.rs`.
- replication: `🔨️modules/📡️replication/🧬️retirement/🦀️.rs`, `.../🔁️fold/📏️retirement/🧪️tests/🦀️.rs`. diagnostic: `🔨️modules/⚠️diagnostic/🧬️retirement/🦀️.rs`.
- os: `🌊️flow/🎒️mesh/🦀️.rs`, `🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`, `🪐️space/🗿️artifacts/🪐️space/♻️retirement/🦀️.rs`, `🪐️space/🗿️artifacts/🗂️collection/♻️retirement/🦀️.rs`.
- plugin: `🔌️plugin/🦀️.rs` (trait methods, impl, `retained_structural_refusal`, settle), `🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs` (blank builder + law), `🧪️tests/🔬️tool-run/🦀️.rs` (test), new `🧵️retained-command/🪟️mounted/🚨️structural-refusal/{🧫️fixtures/🔣️.json,🧬️schema/🔣️.json,🧪️tests/🟦️.ts}`.
- `✏️s/🔌️plugins`: cad `📎️references/🦀️.rs`; block shared `♻️retirement/🦀️.rs`, 3d `🧬️schema/♻️retirement/🦀️.rs`, `✏️editor/👥️presence/🦀️.rs`, `🌐️world/🫧️transient/🦀️.rs`, 5d and 2d `♻️retirement`, `🎚️config`, `👥️presence`; reasoning wires `♻️retirement/🦀️.rs`, `🧬️mutations/🦀️.rs`, `🕸️canvas/🫧️transient/🦀️.rs`; sequence `🦀️.rs`; writer `📢️publication/🦀️.rs`; generation3d `👁️viewer/🫧️transient/🧬️mutations/🦀️.rs` and `👁️preview/🫧️transient/🦀️.rs`; bim `👥️presence/🦀️.rs`; note `🪟️window/🦀️.rs`; forms `▶️try/🫧️transient/🦀️.rs`; layout `📐️blueprint/🫧️transient/🦀️.rs`; puzzle 2d/3d/5d `🪟️window/🦀️.rs`.
- This report and the `70.` entry in `📓️2026-10-10-migration-corrections.md`.
