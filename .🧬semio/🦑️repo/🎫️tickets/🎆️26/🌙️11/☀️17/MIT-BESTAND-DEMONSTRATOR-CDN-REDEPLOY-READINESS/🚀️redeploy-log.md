# 🚀️ Redeploy Log

## Blockers

### 1. `undefined is not an object (evaluating 's.busy')` in the resident service

- Symptom: `repo run @semio-tech/mit-bestand-demonstrator:build` failed about 25 s in during planning (`request-failed f63a0eb9` in the host service log).
- Cause: a concurrent edit added `describe()` to the project farm hub (`🏊️project-farm/🟨️.mjs:254`), reading `pool.farm.snapshot`, but the farm (`🏊️project-farm/🏊️pool/🟨️.mjs`) had no `snapshot`.
- Fix: the farm exposes `get snapshot()` (children per state, queued jobs, `alone`, `restarts`), typed as `GraphFarmSnapshotV1` in `🏊️pool/🟨️.d.mts`.

### 2. `unserved build.graph.no-progress` (graph inference starved)

- Symptom: the retried `repo run …:build` read the project graph slice for about 13 minutes, finished 12 of 4774 projects, then failed (`reference 0c9325a1`, waiting on the memory governor for 1719 of 2099 MB, running `@semio-tech/wfc-3d-rs` and `@semio-tech/wfc-bitmap-rs` with 0 rows).
- Handling: as instructed, the build fell back to native tools. Task scripts run directly as `repo capacity run --memory=<MB> -- repo ./<script>.ts <command>`, which issues the script process envelope that a bare `repo ./<script>.ts` refuses without (`Original parent Script process invocation required`).

## Native release prepare lanes (aggregator, aussuchen, bearbeiten, energy, fem3d, generation3d, generator, koordinator, verfolgen)

- Driver: `native-lanes.sh` + `native-launch.ts` (this folder, logs in `🗑️generated/`); stages `prereqs`, `components`, `catalog`, `session <v>`, `prepare <v>`.
- `repo ./x.ts` and `repo capacity run -- repo ./x.ts` refuse (no invocation envelope); `native-launch.ts` mints envelope + native-owner capabilities + `SEMIO_SCRIPT_PROVIDERS` and spawns the command.
- Lane `prepare-<v>-react-release` is a read-only verdict (`♻️activation/🧰️preparation/📜️script.ts prepare <v> react release`) over: plugin-registry `session-<v>`, framework-plugin-web `support`, `semio-os-infinite:fonts`, engine wasm targets and the `component` targets of the closure. Roots: demonstrator, energy, fem, procedural.
- Catalog `generate` yields an empty `🔌️plugins.json` while committed descriptors are channel 24 (host pin 25): the `component` builds must refresh them, then `generate` runs again.
- Done: wasm-opt, plugin-web support, canvas font-assets, infinite fonts, stdio-format-claims, catalog generate. `describe-build` (cargo) spent ~35 min mostly in cargo-provenance reading 1.6 GB of build records.

## Native non-lane prerequisites (release, no graph)

- Driver: `native-wasm.sh <surface|editor|node-graph|puzzle|flow|canvas|gen:<id>=<projectRoot>>` (helpers `native-wire.ts`, `native-receipt.mjs`, `native-gen-all.sh`). It issues the genuine build-caller envelope (deadline-free policy, otherwise the nested wasm lane refuses a re-projected deadline), the provider designation, the task identity and the store receipt, then runs the script under `repo capacity run`, and settles `.🧾️store/<target>.json`.
- All six receipts exist: surface, editor, node-graph, puzzle, flow `wasm.json`; canvas `stage-runtime-assets.json`. Packages live under `.cache/b/p/m/browser/release/wasm-packages/`.
- Puzzle needed the generated registries (`graph-generate` of puzzle-2d/3d/5d, trinity-jack/rewriting, reasoning-wires, writer, draw-drawing, flow-plugin); run through `gen:` mode.
- Source fixes: `maintenance_retirement_demands` now takes `maximum_work_units`; the 3d and 5d wasm bridges (`pollEnvelopeLoad`) quote it with the turn's `fuel_remaining()`. NodeGraph component `Cargo.toml` gets `[package.metadata.wasm-pack.profile.custom] wasm-opt = false` (wasm-pack maps `--profile release` to `custom`; binaryen rejected bulk memory).

### Site build (native, first attempt)

- `native-wasm.sh site` runs `./🔨️modules/📦️site/📜️script.ts build` with the same build-caller envelope. The 396 map tiles (raster and vector z0–10) were prefetched.
- The Vite config then refused with `Unknown runtime component procedural`, because the generated plugin catalog is empty until the component descriptors are regenerated at channel 25 (lane chain: components → catalog → sessions). The site build reruns after the lane chain.

### 3. Component builds compiled one unit at a time

- Cause: `🧩️component-pass/🟦️.ts` planned compile units from the authored 2048 MB per unit instead of the measured unit history (`.cache/b/r/m/components/build-telemetry/units.jsonl`, 75th-percentile peak 416 MB), so 4.8 GB free memory planned `-j1`. See `🐢️serial-units.md`.
- Fix: the history is read once and reused for the planner and the memory broker; the next run plans about six units.

### 4. Release components rejected (actor ABI, size)

- `semio-plugin-describe` rejects the demonstrator and fem components: `component has no core module implementing the owned Semio actor ABI`.
- The release components are oversized: the demonstrator is 519 MB, and energy exceeds the 512 MiB cap.
- Owned by a separate fix pass; procedural is still compiling.
- Component stage (native-lanes.sh components): demonstrator, energy and fem failed at the describe step. Energy: raw release wasm 612 MB (core module code section 525 MB), over the 512 MiB describe cap. Fem: "no core module implementing the owned Semio actor ABI". The demonstrator wasm is 519 MB. Owned by the fixer agent; failed steps get no `done/` marker, so rerun with `./native-lanes.sh components` once released. Procedural was still compiling at last check.

### 4a. Actor ABI rejection and component size (root causes)

- ABI: the components are fine. The fem core module exports all 36 `semio_owned_*_v1` functions with the guest macro's signatures (`📣️macros/⚛️wp4-reactor/🦀️.rs`). The validator was wrong: `OwnedSemioExport::core_type()` (`🧠️interpreter/🔌️owned-semio-boundary/🦀️.rs`, ~line 213) typed Checkpoint, Describe, Restore, PackSchemaHash, Genesis, PrintMirror, ApplyOps, ReplayEnvelopes and the Sqlite* exports as 19 `i64`. Fixed: Checkpoint/Describe `()->i64`; the codec and snapshot exports `(i32,i32)->i64`; Poll, AdmitActorContext, CloseActorContext, CloseActorOutput 19 `i64`. The descriptor emitter was rebuilt (`describe-build`) with the fix. Stale fixtures (`🧪️tests/🔬️unit/🦀️.rs`, `runtime/🧪️tests/🧫️fixtures/🌱️channel/🦀️.rs`) still encode the old signatures.
- Size: the applied profile is correct (`opt-level=s`, thin LTO, `codegen-units=16`, `strip=symbols`, `debug=0`). The fem core module has 285,810 functions, 150 MB code, 76 MB data (panic locations); only 5 MB is duplicated. The bloat is source-level: the `energy-model-initialization` rlib is 1.26 GB from 3.7 KB of source (`…mutations` 1.0 GB, `stdio-semio` 1.2 GB), most likely derive/macro expansion. The shipped release cap is 256 MiB (`🖨️describe/🏷️component-receipt/🧪️tests/🎚️profile/🔣️.json`); fem (228 MB) fits, energy (612 MB) and demonstrator (519 MB) do not. No profile change fixes this.
- Verification pending: fem component rebuild (`step component-🏗️fem`) was queued behind other builds' cpu-picker (log `🗑️generated/fem-verify-driver.log`).

### 5. Retirement crate tests after the `ControlledCore` refactor

- Result: all targets of `semio-framework-value-retirement` compile and pass (lib 7, controlled-retirement 9, original-inline-retirement 1, original-retirement-work 6, original-selected-retirement 4, scalar-collection-grants 3).
- Refactor-caused: `reserve_cursor_slot` lost its closure parameter; it is public again as `reserve_cursor_slot(effect, &mut dyn FnMut)` forwarding to the non-generic `ControlledCore::reserve_cursor_slot_with` (internal callers use `PagedList::reserve_one`). No per-owner-type instantiation was added. Callers updated in `🎮️controlled/🧪️tests/🦀️.rs` and `📋️queue/🧪️tests/🎮️controlled/🦀️.rs`.
- Behaviour fix in `🎮️controlled/🦀️.rs`: `borrowed_child_step` now attributes the performed Work to a failing child's reply instead of 0, because the native reply path strips the work from `RetirementStep::Failure`.
- Pre-existing drift fixed in tests: the nonexistent `semio_framework_value` import, `step` now takes a body, `RetirementCursor::close_step` became `close_into` through `native_retirement_reply!`, `admit_shared_retirement` became `admit_shared_retirement_into`, poll turns need an original invocation (`effect::drove`), `semio-framework-value-clone-source` added as dev-dependency, and the collection law follows the page-wise copy of commit 734 and the quote retention of commit 737 (`📋️collection/🧪️tests/cases.json`, `schema.json`, `law.test.ts`, `native.rs`).
- Dependents: `check --lib` passes for retirement-queue, factory, clone-source, snapshot-projection and the procedural retained-source plugin crate. The retirement-queue test targets still fail on their own older drift (not touched).
- Note: `--store=shared` failed with "Previous lane custody is unresolved" (a pooled release lane handed to a prototype run); `--store=isolated` was used.

### 5. Disk full, images unmounted (11:36)

- The data volume reached 98 % (5.1 GiB free). Every native build died with a bus error, EPERM and an unreadable registry, and the cache images were unmounted.
- With the developer's approval, `.cache/b/g-full-2026-11-14/fixed.dmg` (a 12 GiB backup graph image) was deleted. `repo cache gc` timed out after an hour (`Original resource loan deadline exhausted`).

### 6. Two drivers deadlock on the lease image

- Two component drivers started while `l` was unmounted stalled for 20+ minutes in stage `loading` (`remains active on native-fixed-image …/b/l (exclusive)`).
- Fixed in source: an opener waiting for the exclusive writer retries as a shared reader once a peer has mounted the image. See `🔒️image-livelock.md`.

### 7. Component results after the fixes

- Demonstrator: every library compiled; the final crate was SIGKILLed (exit 137) at 5.1 GB peak RSS, over the 4.9 GiB rustc budget, while procedural compiled beside it. Retried alone after the queue.
- Procedural: compiled and staged a 161 MB component, then `jco transpile` failed with `Cannot find package '@semio-tech/process'`, because the workspace dependency links are missing (root `node_modules` empty, the demonstrator's `node_modules` dangling). `repo dependencies sync --all` was run.

### 6. Retirement-queue crate tests (`semio-framework-value-retirement-queue`)

- Result: lib tests 6 of 7 pass (`--store=isolated`). `original_turn_tests::retirement_turn_original_queue_preserves_full_grants_all_physical_currencies_and_paused_custody` still fails with `original queue stalled`.
- Test drift fixed: `step` takes a body (grant `maximum_copy_bytes`); `OriginalRetirementFailure` imported from `semio_framework_value_retirement`; `FailedChild::close_step` takes `body`; poll-shaped queue and owner steps need an original invocation, so `bank.rs` gained `drive_poll` (runs under `clone_source_testing::drive`, debits the bank's work), plus the `semio-framework-value-clone-source-testing` dev-dependency; `bank.json` depth 128 to 1024 (257 owners need depth 258).
- Expectation change: `ControlledRetirement::step` with an under-depth grant returns no-op progress (as the pre-refactor source did via `retained_clone_grant_admits`), so `🎮️ownership` line 114 now expects default progress instead of `DepthLimit`; the queue itself still errors.
- Library fix (`📋️queue/🦀️.rs`): `RetirementQueue` had no `close_into`, so `ControlledRetirement::new(queue)` failed with `UnsupportedOwner`; added inherent `close_step` and `native_retirement_reply!()`.
- Stall diagnosis: from turn 0 with copy budget 1 the queue quotes copy 1 / depth 2 but `queue.step` returns zero progress and zero work (queue length 1, selected owner `FactoryOwnedRetirement<String>`). It has no `resident_reply`, so `close_owner` falls to its legacy `close_step`, whose depth/demand admission refuses; the queue quote (`len + owner.next_depth_demand()`) and the factory's own admission (`demands` adds 1 for the child) are inconsistent by one depth level. Not fixed yet; next step is to align `🏭️factory/📦️owned/🦀️.rs` `demands`/`close_step` with the queue quote.

## Energy component release build: mutation source authority failure

- Cause: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json` (edited 09:26, uncommitted) renamed two registry keys to names that no directory carries: `🥂️window-layer/🥽️change-glazing-visible-transmittance` and `💧️humidistat/🏝️change-dehumidifying-setpoint-schedule`. The directories (HEAD, unchanged) are `🥽️glazing-visible-transmittance` and `🏝️dehumidify-setpoint-schedule`. The derive (`🔖️mutation-source-authority/🦀️.rs:100`) matches the owner directory name exactly, so the leaves in crates `mut-window-layer-optics`, `mut-layer-optics-visible` and `mut-humidistat` fail.
- Fix (not yet applied, derive registry is a build input of the running fem/procedural/demonstrator component builds): revert those two keys to the directory names (`git diff HEAD` on the registry shows exactly these 2 lines).
- Audit: a script comparing every registry owner against its directories found no other mismatches in any root (other differences are facet subdirectories such as `📦️packages`, `🧪️tests`).

### Correction: the registry is generated, the taxonomy is the source

- `🔣️mutation-authority.json` is a generated projection of `mutationDomainOwners` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (`GenerateScript` in `✨️derive/🧪️tests/🔬️rust-package-commands/🟦️.ts`; `generate` writes it, `check` fails when stale). The 09:26 change was a regeneration, so a plain revert of the registry would be overwritten.
- The stale input is the committed taxonomy (unchanged since Nov 13): line 34140 `🥽️change-glazing-visible-transmittance` and line 34231 `🏝️change-dehumidifying-setpoint-schedule`. The directories carry the short names. The taxonomy and the registry are currently identical (verified), so the registry is otherwise fresh.
- Fix to apply once no `native-lanes.sh component` runs for two 60 s checks: rename those two taxonomy keys to `🥽️glazing-visible-transmittance` and `🏝️dehumidify-setpoint-schedule`, then run `repo run @semio-tech/dsl-derive-rs:generate`.

### 8. Describe tool invalidated mid-build

- Fem (linked, 1197 units) and procedural (staged) failed at describe with `Original mutable compiler input changed` (`🖨️describe/🛠️emitter/🟦️.ts`: the emitter rechecks the sha256 of every current source of the descriptor compiler). A concurrent fix to `♻️retirement/📋️queue/🦀️.rs`, a dependency of the describe tool, changed one of them.
- Handling: all component builds were stopped; the pending source fixes (taxonomy keys, factory depth accounting) land first, then the describe tool is rebuilt (`FORCE=1 native-lanes.sh prereqs`) and the four components run one after another.

#### 6a. Status update (source edits frozen)

- Queue crate source and test edits are final until the release components are built; no factory, queue or retirement source is touched while the rebuild runs.
- State: 6 of 7 queue lib tests pass. The open stall in `original_turn_tests` is unchanged. A diagnosis run was refused by the disk guard (10.7 GiB free, limit 97%); `repo cache cas gc` was deliberately not run.
- Hypothesis (unverified): at turn 0 with copy budget 1 the queue quotes copy 1 / depth 2 and passes its depth gate, but the selected `FactoryOwnedRetirement<String>` has no `resident_reply`, so `close_owner` reaches its legacy `close_step`. That path re-gates on `demands(body)` (child depth plus 1) and refuses with zero work, so the queue quote and the factory admission differ by one depth level.
- Next step, after the coordinator reports the components built: print the selected owner's `next_demand(0)`, `next_demand(body)`, computational work and the queue depth quote at turn 0 (a `[DEBUG]` line in `🎟️turn/🦀️.rs`, tests may read the private `queue.selected()`), then align `demands` and `close_step` in `🏭️factory/📦️owned/🦀️.rs`, pin it with a factory test, and run the factory, queue and retirement suites with `--store=isolated`.

### Applied

- Renamed the two keys in `🔣️taxonomy.json` to the directory names and regenerated `🔣️mutation-authority.json` through the ticket launcher (`mutation-authority: written`); the registry is back to its HEAD bytes and every entry has a directory.
- `repo cargo --store=isolated --configuration=release -- check --target wasm32-wasip2` on the energy component manifest exits 0 (no `MutationLeaf` errors).

### 9. Rebuild results (after describe tool rebuild)

- Demonstrator: final crate jetsam-killed again (exit 137, 4.0 GB resident, swap exhausted at 10.7 GB). Fix: one front-end thread for a `cdylib` final (`🏗️native-build/🎟️admission/🦀️.rs`, see `💥️demonstrator-oom.md`). Retried after the chain.
- Energy: compiled and linked (1025 units, 2837 s), then `jco transpile` trapped with `out of bounds memory access` on the 442 MB component (612 MB before the retirement fix). The next bloat cut is in progress (`🎈️component-bloat.md`).

### 10. Describe recheck against the live worktree

- Fem linked (890 units, 1116 s) and failed at describe with `Original mutable compiler input changed` for the third time; no fix of this ticket had touched a source in between, so other agents' live edits invalidated the descriptor compiler's recorded inputs.
- The emitter re-digests the descriptor compiler's sources in the live worktree at the end of each component build; in a shared worktree edited by many machines, any change during a 20–40 minute build kills the component. A design fix is in progress (`🧾️describe-recheck.md`). Component builds wait for it.

### 11. Sealed descriptor compiler recheck

- Fixed the live-worktree recheck: the descriptor compiler is now verified against its own sealed digests and refusals name path plus both digests. Details: `🧾️describe-recheck.md`.

### 12. Energy component after bloat cuts; jco limit

- Cuts 1–3 (non-generic producer driver chain, `native_close_settle`, `admit_frame`, derive `plan` hoisted into `plan_field_stage`) took energy from 442 MB to 379 MB; it compiles and links.
- `jco transpile` still traps (`out of bounds memory access`) at 379 MB, also with `--wasm-max-mem-pages=65536` at 1.78 GB peak, so it is not a memory cap. 137–228 MB components pass.
- Handling: a native transpile tool (the `js-component-bindgen` crate in a native CLI behind `transpilePluginComponentModulesAsync`) is being built, see `🦀️native-transpile.md`. Further size cuts are drafted, not applied, until the redeploy is done.

### 13. Native transpile in place

- `semio-native-transpile` (js-component-bindgen 2.16.0, the version behind the vendored jco) replaces jco in `transpilePluginComponentModulesAsync`; output is byte-identical to jco on the 137 MB component and the 379 MB energy component transpiles natively in 5.4 s at 1.28 GB. Stage it with `repo ./📜️stage.ts` in its folder before a component build. Details: `🦀️native-transpile.md`.

### 13. Disk at 100 %, fem at describe

- At 20:00 the data volume had 1.7 GiB free; energy's final crate was jetsam-killed again (4.6 GB). With the developer's approval, the unused 4 GB `🗑️generated` folder of ticket 26/11/14 PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE, `~/.cache/uv`, `~/.cache/codex-runtimes` and regenerable app caches were deleted. Root cause: raw fixed images never return freed space (`💽️image-reclaim.md`).
- The final cdylib now also runs its LLVM back end serially (`-Z no-parallel-backend`).
- Fem compiled and linked a 128 MB component (bloat cuts), passed the ABI validator and transpile, then describe refused it: every app-ownership probe failed with `native io: original interpreter deadline requires its caller clock`. A fix is in progress.

### 14. Describe refusals: caller clock, procedural claims, fem record spec

- Clock (fixed, verified): the owned runtime's codec path never bound the original caller clock. `assemble_codec_origin` and `origin_instance` in `🔌️plugin/🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs` built `OwnedGuestInstance` with `original_job_clock:None` (only `describe_observed` bound it), so `original_deadline_at` refused. Both now bind `(budget.retained.work.clock_binding, self.original_clock)`. Verified with the rebuilt tool (`repo cargo --store=isolated -- run -p semio-plugin-describe`): the clock error is gone on the fem wasm. The staged binary is NOT replaced yet (gate: component builds running); `repo cargo` cannot emit a binary to scratch.
- Fem (second defect, fixed in source, unverified): after the clock fix, `fem.2d`/`fem.3d` fail with `artifact codec schema has no structural record specification`. The codec table reads `ArtifactPackMetadata::native_record_spec_producer()`, i.e. `ArtifactPack::borrowed_record_spec_producer()`; the Fem2d/Fem3d snapshots only overrode `record_spec()`. Added `borrowed_record_spec_producer()` (`BorrowedRecordSpecProducer::of::<FemNdSnapshotValue>()`) to both `🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/…/💾️binary/📸️snapshot/🦀️.rs`. Needs a fem component rebuild. Many other artifacts (flow, process3d, cad, block, demonstrator playground, dag, raster, ...) also lack it; a bundle whose kinds all lack it will hit the same refusal.
- Procedural (separate defect, fixed in source, unverified): `composer_identity_claims` requires `composition-route` claims for foreign-output composers, but `generation2d`/`generation3d` `definition()` declared plain `dialect` claims for svg/pdf/png/json/dxf/txt. Replaced with framed `composition-route` claims (`🌀️procedural/🗿️artifacts/{🌀️generation2d,🧊️generation3d}/🦀️.rs`). Needs a procedural component rebuild.
- Disk fell to 2.8 GiB during verification; isolated builds stopped.

## 15. Pack schema hash gate keyed on the wrong source

Root cause: `artifact_codec_table::pack_schema_hash` (plugin declaration surface) gated on `ArtifactPackMetadata::native_record_spec_producer()` (borrowed producer), but the hash itself comes from `ArtifactPack::record_spec()`. The producer was consumed nowhere else, so every snapshot with `record_spec()` but no producer (fem, flow, process3d, cad, block, playground, dag, ...) faulted with "no structural record specification" at describe.

Fix: `ArtifactPackMetadata::native_pack_schema_hash()` now returns `Option<[u8;32]>` straight from `record_spec()` (single source, trait file `🧰️framework/🔨️modules/🚪️io/💾️binary/📸️snapshot/🦀️.rs`); `native_record_spec_producer` and the dead `ArtifactPack::borrowed_record_spec_producer` (plus all 9 overrides, the derive helper `__artifact_borrowed_record_spec_producer`, and the fem Fem2d/Fem3d overrides) removed. Store codec registration uses `.unwrap_or([0;32])`; native-registration fixture updated.

Check (wasm32-wasip2, release, isolated): fem, demonstrator, energy, procedural all pass with no errors. `cargo test -p semio-framework-pack` does not compile because of unrelated in-flight edits in semio-framework-value (`PagedListAllocationError.work`, `RetainedCloneSourceTake`, `admit_owned`), not this change.

### 16. Disk reclaimed; demonstrator compiles

- The rust image was reclaimed through `diskutil image attach` (host allocation 99.8 GB → 19.5 GB); the data volume went from 3.7 GiB to 80 GiB free. `repo cache gc` now reclaims idle bloated images (`💽️image-reclaim.md`).
- With one front-end thread and a serial back end, the demonstrator's final crate compiled (1441 units, 4557 s; component 359 MB) and passed transpile through the native tool.
- Describe refused it: `plugin assembly failed — no declared composer capability owns the runtime claims`, as for procedural, whose six generation2d/3d declarations were fixed. The remaining declarations of the bundled plugins are being fixed, and the refusal will name the unowned claims.

### 15. Composer claims: demonstrator and every chained declaration

- Rule kept, data fixed: `composer_identity_claims` (identity crate, contract and shooting) requires `composition-route` claims for foreign outputs; only declarations chained through `.composers(entries())` are checked, so plain `dialect` claims on foreign composer rows fail assembly there.
- Demonstrator: the only declaration it chains is `playground` (zip and json rows). Fixed. The same defect in flow, gis (gismap, gisterrain), architect, process3d, lowpoly, layout, cad, imperative procedure, raster and space home (49 rows) is fixed by `composition-route-fix.py`.
- Deferred until the energy/fem/procedural chain is quiet: energy model (3 rows) and the diagnostics change (names the definition and the unowned claims), both in `apply-deferred-describe-fixes.py` (the identity crate is compiled into every component). The staged describe binary must be rebuilt afterwards.
- Verified: the staged binary reproduces the demonstrator refusal. Rebuilt components needed: demonstrator, energy, and the other hubs above.

## Energy composition routes (2026-11-18)
- Root cause: idf export entry read `[MODEL_DIALECT, s.stdio.json]`; `composer_identity_claims` refuses foreign output routes that read non-owned dialects, so the route rows could never pass.
- Fix: export reads only the artifact-owned `s.energy.model@1/*`; removed the json-bridge source branch and `MODEL_JSON_BRIDGE_DIALECT` from `rebuild_native_snapshot` (no other entry read it, so export behaviour from the native model is unchanged). The three `composition-route` rows (idf, zip, json) are now exactly one route each, matching the runtime sets.
- Removed unused `BorrowedRecordSpecProducer` import in the energy carrier.
- Verified statically only (no builds).
