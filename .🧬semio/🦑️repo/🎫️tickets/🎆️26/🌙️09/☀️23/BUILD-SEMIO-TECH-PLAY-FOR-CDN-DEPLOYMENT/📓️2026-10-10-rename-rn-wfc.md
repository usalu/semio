# 2026-10-10 rn-wfc: protocol migration of wfc, energy, fem, animate (source-complete, uncompiled)

Executor `rn-wfc`. Scope: `✏️s/🔌️plugins/{🀄️wfc,🔋️energy,🏗️fem,🎞️animate}/**`. Every edit is source-level only. `os.status` is still `PARTIAL os` (`semio-framework-plugin` red, 455 errors), so no cargo result exists for any crate in scope. All 783 `.rs` files touched in the last 25 hours were run through `rustfmt --emit=stdout` as a pure syntax gate: zero parse errors (three real brace errors in wfc job and five wfc inference files were found and fixed that way).

## What changed per plugin

### Cross-cutting (all four plugins)
- `RetireOwned` (1149 types) and `CanonicalJsonTree` + `#[canonical_json(owner = semio_framework_pack_json)]` (558 types, the closure of every `dsl::Mutations` / `MutationLeaf` root) added by script `derive_sprinkle.py`.
- 39 boilerplate `build_{document,config,draft}_store_owners` overrides deleted in 15 files; 10 test sites moved to `funded_bounded_artifact_store_owners` (animate host `new_presentation_store` included).
- One-item preparations rebuilt on `store::OneItemOwners` with `begin_batch_digest`, three-generic requests and quoted-demand close: fem2d editor, fem3d editor, wfc bitmap editor, energy editor, animate editor (config preparation).
- Window config owners: every owner got `type Edit = WindowConfigApplyEdit<S, M>`, `MAXIMUM_PREPARATION_DEPTH = 64`, `build_retained_edit()` and a `WindowConfigApplyMutation` impl (`exchange` swaps the carried field(s) with the post record and returns the displaced values as the inverse). Done for energy model 3d (editor + viewer), fem 2d/3d model and results (4), wfc grid2d (grid + preview), grid3d (grid + preview), bitmap input and output.
  - Externally tagged enums with named-field variants were converted to newtype variants over payload structs with identical field names (wire stays byte-identical, per corrections #13): fem `Update { patch }` -> `Update(Fem*WindowConfigUpdate)` (4 files, tests updated), wfc grid2d `Grid2dWindowConfigSet*` (6 payloads) and grid3d `Grid3dWindowConfigSet*` (3 payloads); all call sites rewritten. Bitmap input/output are internally tagged, which the derive supports for named fields, so they keep their shape.
  - Added `RetainedClone` derives on the configs, mutations, payload structs and the shared fem app-surface types (`ResultMode`, `FemResultsAnimation`, `FemLoopMode`, `FemWaveform`, `FemResultSourceChange`, gumball config).

### wfc
- Engine `💼️job`: `📤️publication` rewritten on `RetainedJobPublication` + two `RetainedPayloadBuilder`s (`Kind`, `Publication::{poll, borrow_outcome, retirement_demands, close_step, terminal_is_empty}`); new `🏃️headless` (`HEADLESS_GRANT`, `run_headless`, `close_job`, `payload_bytes`); `WfcJob` and `WfcRestore` are `InteractiveJob`s with `run()` decisions (`WfcRun`, `WfcRestoreRun`) so the lent borrow is an unconditional tail; grant-based close ladders with quoted demands; `🔍️search` `drive_batch_job` on the new context API.
- Snapshots (5 artifacts): hand-written retirement structs removed; `decode_into` uses `admit_owned_retirement`.
- Inference jobs (5 artifacts) and fill jobs (bitmap, grid2d, grid3d, wfc2d, wfc3d): publication + lent child outcome pattern, `solve_with_clock` replaced by `run_headless`.
- Tests rewritten to the new API: engine job unit tests, publication vectors (step counts are no longer asserted exactly, the neutral fixture's page/byte/progress vectors still are), grid-job-drive, inference preview tests (3), five fill tests, drag tests (2), bitmap brush tool, grid2d window test, five snapshot tests.

### energy
- `🧪️sim`: `EnergyRun` decision enum, `EnergyJobAuthority::run`, `InteractiveJob for EnergyJob` (`step` lends empty marker payloads, a pending outcome is retried when admission yields, `borrow_outcome`, quoted `close_demands`, grant-based ladder), wire packet / queue / restore / rejected-owner closes on grants (`energy_guarded`, `energy_turn`, `energy_spend`), `Engine::run` batch adapter with self-funded close, tests rewritten (`Seen` helper).
- `🧵️simulation-session`: `EnergySimulationRunJob` stages each tick as a `RetainedJobPublication` over the encoded bytes, settled outcomes are lent as empty markers, nested numerical job steps are charged to the outer wallet; tests rewritten.

### fem
- `🧵️session` 2D and 3D: reactor `BoundedJob` impls moved to the current trait (`step(budget, original, snapshot, cx) -> Result<JobStep, ValueError>`, `close_step(cx)`, `retirement_demands`, `BoundedJobFactory { admit, demands }`); child job outcomes are classified into owned enums (`ChildEnd`) before the borrow ends; close ladders return `PluginLifecycleStep` with a grant, plus `close_demands`; editors (2d, 3d, 3d viewer) implement `mounted_job_{maintenance_phase,maintenance_demands,close_demands,maintenance_step,close_step}`. The 3D retained `child_outcome` slot and its test were replaced by a classification test.
- Tests: preparation laws (2d) on `OneItemOwners::detached`, gumball disposer loops, session tests.
- Removed duplicate hand-written `RetireOwned` impls on the fem2d results transient types (now derived).

### animate
- Host `🧰️owned`: the 1650-line `PresentationEnvelopeMaterializeJob` / handle / registry / completion machinery was deleted: it had no consumer outside its own tests and the editor uses only `presentation_envelope_decode_owner_bundle()`. The decode catalog and its three authorities (pack snapshot, rejected mutation, rejected conflict) are migrated to the current `ArtifactEnvelope*Authority` traits; bespoke retirement factories replaced by `OwnedValueRetirementFactory`. Tests reduced to the store laws that remain meaningful.
- `PresentationImportJob` and the editor were migrated in the earlier wave.

## Upstream dependencies and blockers (exact)
1. `semio-framework-plugin` must compile (455 errors, rn-os). Everything above is uncompiled until then.
2. **Request to main:** `admit_original_job` and `original_job_admission_demands` in `reactor/💼️jobs` are `pub(crate)`. The fem sessions reimplement the same quote and admission locally (`mounted_job_demands`, `mounted_job_admit` / `job_demands`, `job_admit`). Make them public or give plugins a supported helper; then delete the local copies.
3. **Request to main (already messaged):** `semio_framework_value::RetainedClone` derive on `Viewport2d` and `Viewport3dOrbit` (ui viewport schema). `WindowConfigOwner::State` requires `RetainedClone` and the four fem window configs embed them.
4. The fem engine jobs (`FemJobGraph`, `MeshJob`, `AssemblyJob`, `PcgJob`, `LdltJob`, `SubspaceIterationJob`) live in `✏️s/🔨️modules/🏗️fem/⚙️engine` (out of my scope) and still implement the old `InteractiveJob` (`StepOutcome`). The fem sessions assume they implement the new trait (`step -> Result<Option<JobOutcomeBorrow>>`) and keep calling their inherent `close_step(maximum_bytes) -> (bool, items, bytes)` ladders. If the engine owner changes those ladder signatures, the session closes need the same one-line adaptation.
5. fem 3D session still calls `semio_framework_ui_scene::world3d_snapshot_*` with the old shape (`world3d_snapshot_close_step(snapshot)`, `world3d_snapshot_recovery_close_step(maximum_bytes)`); not verified against the ui crate.
6. wfc/energy window transient owners (`WindowTransientOwner`, `window_transient_transfer!`, `transient_root!`) were not touched; verify at compile time.
7. Likely compile-time friction to expect: `JobPayloadPageSource::allocated_capacity_bytes` visibility, `HistoryView` / `Media` `RetireOwned` bounds, `store::artifact_bounded_history_entry_decoder` still existing for the animate catalog, `RetainedClone` derive on types that embed `Option<crate::Viewport*>`, the `Update` payload DslRecord keyword attribute for fem window mutations, and an `InteractiveJobCloseStep` `PartialEq` use in tests.

## Verification state
Crates checked: none (blocked). When `os.status` starts with `GREEN os`: slot-gated `cargo check --manifest-path <artifact>/Cargo.toml --lib --tests` native and `--target wasm32-wasip2 --lib` (component features) for `semio-s-plugin-wfc-engine`, the five wfc artifacts, the energy model artifact, the fem 2d/3d artifacts and the animate presentation artifact, then `cargo test --no-fail-fast`.

## Scratch
Generators and logs: `.🧬semio/🦑️repo/⚡️cache/play-fleet/rn-wfc/` (`derive_sprinkle.py`, `energy_edit1-5.py`, `energy_session*.py`, `fem_session2d.py`, `fem_session3d.py`, `fem_window_cfg.py`, `wfc_window_cfg.py`, `animate_host.py`, test rewriters).

## Fem engine scope extension (`✏️s/🔨️modules/🏗️fem/**`)

Migrated to the `InteractiveJob` / five-currency grant protocol:
- `🕸️mesh`: `MeshJob` (verified earlier: native tests, wasm32 check).
- `🔢️sparse`: `LdltJob`, `PcgJob`, `SubspaceIterationJob`, `PcgJobConstruction`, the three restore cursors, `pcg()` / `subspace_iteration()` batch adapters. Shared at the top of the file: `NumericalRun`, `NumericalOutcomeDesk` (lends one finished payload at a time, retries a refused admission, pays the lent payload back from the next call's own wallet), `numerical_close_gate` / `numerical_rung_demand`, `NUMERICAL_BATCH_GRANT`, `numerical_batch_step`, `numerical_batch_close`.
- `🧮️analyses`: `FemJobGraph`, `AssemblyJob` (desk pattern; inherent tuple ladders renamed `close_retained_step`; `AssemblyJob::retire_outcome` pays back the one lent completion payload before `finish()`).
- Close contract of the numerical jobs: a payload or writer frontier must be covered by its own exact quote (`close_frontier`); an owner rung is limited by its own `maximum_release_bytes` (the old ladders self-gate); `close_demand` is the upper-bound quote (16 KiB owner page; the largest actual owner when the job is admission-faulted).
- Sessions: 2D and 3D `close_step` take the full `RetainedCloneGrant` and pass child grants to `FemJobGraph`, `MeshJob`, `PcgJob`, `PcgJobConstruction`, `LdltJob`, `SubspaceIterationJob`; `AssemblyJob` closes through `close_retained_step` (mounted models never lend payloads). Session close quote is now `copy_bytes 4096, release SESSION_CLOSE_RELEASE_BYTES, depth 3` (was depth 1, copy 0) because the mesh pop quote carries element copy bytes and a writer-with-payload frontier is depth 2 plus the session owner. The fem 2D/3D admission copies are deleted: `mounted_job_demands` / `job_demands` call `reactor::jobs::original_job_admission_demands`, `mounted_job_admit` / `job_admit` call `admit_original_job`.
- Tests: shared `🧪️tests/⚙️engine/🦀️.rs` now holds `Seen`, `seen_of`, `close_job`, `close_payload`, `close_writer`, `payload_from_pages`, page-byte accounting helpers; sparse and analyses tests rewritten for fresh per-context receipts, borrowed outcomes and grant closes.

Behavior changes worth knowing:
- A delivered payload (preview / checkpoint / complete) is paid back by the job's NEXT step before any work resumes, so a loop that calls `step` once per iteration advances work one step later than before (the subspace stage-walk test now steps until the stage changes).
- A sub-page grant can still close a small ledger (progress without releasing a page); tests assert "no page released and backing unchanged" instead of "no progress".
- Payload close now also closes the operation ledger as a separate frontier; page-release conservation tests count page bytes only (`close_payload_page_bytes`).

Verification (slot-gated):
- Fem engine (scratch crate `fem-engine-check`): native `check --lib --tests` 0 errors; native `test --lib` 197 passed (one env-dependent mesh corpus test skipped, passes with its env); wasm32-wasip2 `check --lib` GREEN (`fem-12.log`).
- WFC engine (`semio-s-plugin-wfc-engine`): native lib GREEN, wasm32-wasip2 lib GREEN, `test --lib` 301 passed / 0 failed. Fixes this round: `+ Send` on two helper impls, `os_kernel::json` -> `pack_json` (+ `JsonMemberPolicy::Reject`) in tests, batch driver `drive_batch_job` and `WfcJob::from_checkpoint` now keep one `JobPayloadAuthority` per operation (new `admit_payload_authority` / `close_payload_authority`, test-only `test_step_context`), headless acknowledgement uses `HEADLESS_GRANT`, stale-generation steps close an in-flight publication before publishing the fault (previously the fault replaced and dropped it), delivered-payload retirement yields to cancellation and identity checks.

Framework cost found (tell main/pl): every retained publication (WFC preview, checkpoint, commit with two streams) spends `JOB_PAYLOAD_OPERATION_PAGES` (256) one-item initialization turns per payload builder before any byte moves (`RetainedPayloadBuilder::advance_initialization`). With one `poll` per `step`, a cadence-16 preview costs about 270 steps. Fine when the worker loops steps inside a lane turn, a latency regression otherwise. WFC tests use bounds of `256 * streams + 64`.

## Artifact crates state at PARK (native `check --lib`, default features)

GREEN native lib: wfc bitmap, grid2d, grid3d, 2d, 3d; fem 2d, fem 3d; animate presentation. Logs `plug-*.log`, `fem-f2d.log`, `fem-f3d.log` in the play-fleet scratch dir.

Fixes this round (all outside the engines):
- Derives (rule 46): `RetireOwned` on `ArtifactDocumentPayload` (plugin crate root file) and on every type generated by `list_delta!`, `plain_list_delta!`, `row_patch!` (`__value_derive::RetireOwned`); `wfc_patch!` in the five wfc diff files; `impl RetireOwned for std::path::PathBuf` (value crate `♻️retirement/🦀️.rs`, next to the `String` impl).
- Plugin-root composition names moved to `semio_framework_plugin::io::{AnalyzeSource, ComposeError, ComposeSource, Composition, Analysis, ComposerEntry}`; rewritten in the wfc and fem io files.
- wfc: host files import `InteractiveJob`; `Run::` to the per-artifact run enums; grid3d diff `rejection` helper; grid3d dead `retire` helper removed; grid2d binary snapshot gets `BASE64_ALPHABET` and drops a stray `controlled_native` re-export; wfc 2d host leftovers (`rejected_output_page`) removed; wfc 2d `Wfc2dTileMedia` / `Wfc2dPathSegment` variants now wrap records (`Wfc2dBitmapMedia`, `Wfc2dVectorMedia`, `Wfc2dImageMedia`, `Wfc2dSegmentTo/Quad/Cubic`) so the canonical-tree derive accepts them; wire form stays externally tagged, so fixtures are unchanged. All construction/match sites rewritten.
- fem: `Fem2dArtifact` duplicate `RetireOwned` removed; `FemDof` and `FemAxis` derive `RetireOwned` + `CanonicalJsonTree`; fem 3D `elements` field gets `#[dsl(statements, block)]`; fem 3D element delta hand-expanded like 2D (boxed element patch); `create-element` `new_id` typo.
- animate: duplicate `RetireOwned` on `PresentationConfig`; `PresentationPresence` gets `ArtifactPresenceSnapshot`; `AnimateViewCommand` derives `RetireOwned`; three crude `apply_diff` leftovers in the animation engine restored to `self.apply(..)`; `cause.message` `.into()` in three serializers; pack snapshot retirement uses `retire_owned(factory, value, grant)`; `retained: request.retained` in the operation context.
- `retained: request.retained` added to every `AppOperationContext` literal in the four plugins (several are behind `component-app-assembly`, so they were invisible to the default-feature check).

Energy (`semio-s-artifact-energy-model`) native lib: 13 errors left, all rooted in the snapshot: `EnergyModelSnapshot: RetireOwned` unsatisfied (6) plus `ModelPatch: RetireOwned` (2) and the `OneItemOwners<EnergyModelSnapshot, EnergyModelMutation>` close bounds (5). Added this round: `artifact_retire_leaf!(EntityId, ScheduleId)`, canonical-tree impls for both ids, `RetireOwned for FixedTable<K, V>` (in `🔋️model/🦀️.rs`). Next step: find the field of `EnergyModelSnapshot` / `ModelPatch` that lacks `RetireOwned` (`rg` the first note of the error in `plug-energy.log`).

Not yet run: wasm32-wasip2 `check --lib` and `--features component-app-assembly` (native and wasm32) for fem 2d/3d, wfc x5, energy, animate; `AppOperationContext` edits are only compile-checked once the component feature is built. Tests for the fem artifacts, wfc artifacts, energy and animate.

## Release path result (native + wasm32-wasip2 + `component-app-assembly`)

All `cargo check --lib` GREEN (logs `m2-*.log`, `mx-*.log`, summaries `matrix.txt`, `matrix2.txt` in the play-fleet scratch dir):
- wfc bitmap, grid2d, grid3d, 2d, 3d: native default, native `--features component-app-assembly`, wasm32-wasip2 default, wasm32-wasip2 with the feature.
- fem 2d, fem 3d: same four.
- animate presentation and energy model: native default and wasm32-wasip2 default (neither crate has a component feature).
- Engines: fem engine (native, wasm32, 197 tests) and wfc engine (native, wasm32, 301 tests) as above.

Energy fix this round: `RetireOwned` on `EnergyModelSnapshot`, the `patch!` structs, `OptionChange`, `Slot`, and hand impls for `Splice`, `Slots`, `ListEdit`, `Rows` in the diff patch module.
Other fixes: `*ViewCommand` enums derive `RetireOwned` (wfc 2d/grid3d/bitmap, fem 2d/3d); `transient_root!` mutation enum derives `RetireOwned` (plugin crate macro); `Wfc2dConfigDiff`/`Wfc3dConfigDiff` imports; fem `ActorId(.. .into())`; fem 3D `pcg_build` close; grid2d window transient duplicate impl removed (derive kept); `retained: request.retained` in every `AppOperationContext`.

Not run: artifact tests (fem 2d/3d, wfc x5, energy, animate); `--all-features`; component builds beyond `check`.
