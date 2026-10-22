# Result cargo-fix (executor cargo-fix, wave 1020d, phase 1 done: static set APPLIED, compile GO pending)

UPDATE (partial GO): the 77-file static set was applied live (`apply.py --write`, 0 drift) and `plan/gen.ts` regenerated (1,283-step plan refreshed, 3,423 packages). The compile runner is NOT started; compile GO follows chain15 plus the post-show batch (conc-cargo lanes). **Group `fem` (steps 076-078) depends on the separate executor `fem-finish`**: decision is to FINISH the funded-paged-owners redesign from the API spec in section 4 (options A/B there are decided: A). The /private/tmp/cargo-fix overlays were deleted.

READY: `python3 C/cargo-fix-tools/apply.py` (dry run on the live tree at 20:50: 77 operations, 0 drifted; `--write` applies all-or-nothing), then regenerate the plan (`bun C/cargo-fix-tools/plan/gen.ts`) and start the window (section 3). Nothing was written outside `C/` and `/private/tmp/cargo-fix/` (overlays `r`, `r0`; delete both when done). No cargo, no `bun repo`, no sub-agent; the static checker ran as ONE `nice -n 10` bun process at a time on the overlay (not on the live tree).

`C` = this ticket folder. Tools: `C/cargo-fix-tools/` = `plan/` (generator + runner), `rs/` (copy of `rs-sweep3-tools` with a `CF_ROOT` overlay switch), `stage.py`, `apply.py`, `staged/` (77 files + `manifest.json`), `idx.ts` (symbol index), `fill.ts`, `env.sh`. Scratch output: `C/🗑️generated/cargo-fix/` (delete at ticket close).

## 1. Static state at 20:00 (live tree, checker `rs-sweep3` over all 3,422 packages)

| item | value |
|---|---|
| findings on the live tree | **1,086 in 149 crates** (11 findings classes: 379 unresolved-qualifier, 209 unresolved-path, 156 unresolved-import, 238 undeclared-dependency, 52 feature-dep-undeclared, 36 private-item, 14 duplicate-name) |
| after the staged set (overlay re-check) | **556 in 69 crates** (60 of them known checker false positives, see below) |
| overlay fidelity | the overlay reproduces the live count exactly (1,086) before any edit; ignored `🤖️generated/**` files were copied in |
| not run | any rustc; the rustfmt syntax gate (peers' load); wasm32 evaluation beyond the checker's own |

### Staged fixes (77 files: 44 `Cargo.toml`, 33 `.rs`; all anchored on the sha of the live content)
1. **Dependencies (61 lines in 42 manifests)** by `fixdeps` (cycle-checked, 0 refused), 2 feature declarations by `fixfeat`, plus hand-added `directory` (= `semio-framework-os-kernel`) dev-dependencies in `semio-hub-space` and `semio-hub-space-laws`. `Cargo.lock` files are NOT touched: the lock group of the plan regenerates them.
2. **Plugin facade `semio-framework-plugin`** (`🔌️plugin/🦀️.rs`): root re-exports `__semio_dispatch_PluginApp` (macro now owned by `plugin-reactor-app`; fixes 41 hub crates whose `dyn_enum_close!` imports it from the facade), `NeedsLabel, NeedsVersion, Ready` (typestate builder), `IconName, Locale, Terminology` and `LocalizedLabel`; the test-only private import of the same names was reduced to `Label, LabelText`.
3. **Framework facade `semio-framework`**: `pub use semio_framework_ui_locale::{Locale, Terminology}`. **os-kernel** root: the private `use semio_framework_ui_locale::{...}` became `pub use` (the `protocol::Locale::En` users). **job** root: `use semio_framework_job_payload::{...}` became `pub use` (job-conformance, job-unit-tests, job-preview, job-worker-authority name `JobPayloadStream`, `RetainedPayloadBuilder`, ... through `semio_framework_job::*`). **os-spr** root re-exports `MutationAdmission` and `MutationMessageLedgerRetirement` from `protocol` (replication facade; its lib name is `protocol`).
4. **Store cold migration leftovers**: `apply_diff` -> `apply_diff_cold` in `store-components/.../store-cursor-operations` and the remove-space-alternative test, unused `apply_diff` import dropped in `paged-one-item/mutation-apply`; stale `ArtifactOperationImage` export dropped from `operation-wire`.
5. **Hub components after the carves**: `hub-layout` and `hub-lowpoly` follow the `hub-cad` model (`semio_s_artifact_<x>_editor/_viewer/_model` crates instead of `crate::editor::<x>`); `hub-demonstrator` imports `manifest::create_generation3d_app` and the new root re-export `Generation3dPlayApp` of the generation3d artifact crate; the five trusted-catalog crates and the profile tests use `directory::os_pack::...`; route-models/hostile/session laws use `directory::os_directory::...`; `ArtifactDialect` comes from `semio_framework::ArtifactDialect`.
6. **Smaller**: time-travel panel imports `Terminology`/`IconName`; `flow-wasm` test imports deduplicated (E0252) and `FlowBridge/FlowDomain` taken from the crate itself; flow-host test uses `board::schema::dag_input::{DagChannelRef, DagChannelDirection}`.

### Known checker false positives (do not chase): 60
`semio-framework-filesystem-workspace` (54, three `cfg`-gated `mod native` plus `unsafe extern` grammar gap), `semio-build-scheduler` (2, `unsafe extern` fn visibility), `semio-s-artifact-layout-layout-model` (4, derive macro and trait of one name live in different namespaces). Also macro-generated items (bim gestures `Cursor*`, `BimCommand`, `SourcingModules`).

### Open after the staged set (496, for the compiler-driven pass; ordered by group value for the three products)
- **fem (blocked on design, section 4)**.
- **hub / engines**: `hub-trinity` (jack crate moved to `semio_s_dev_trinity_jack`, half-edited), `hub-wfc` and `hub-block` surface tests (`...::schema::inferences::*` and `kit_catalog_artifact_kind` paths), `hub-flow` surface test (`PluginCloseStep` no longer exists, old `close_step(items, bytes)` API), `hub-gis` inference (`MemberStoreOwner`, `gis_map_document_store_owners`: the store-owner API is mid-migration, call sites use `install_document_store_owners_exact(bounded_artifact_store_owners())`), `hub-stdio-mutation-bridge` (18, in a generated file that names the removed `semio_s_artifact_stdio_html::standards::v5` layout: regenerate at activation), `hub-hostile-input-laws` and `trusted-catalog-fixtures` (`Budget{fuel, deadline_ms, ...}` constructor is stale against kernel `Budget{retained, retained_policy, work_fuel, deadline_us, ...}` and `codec_print_mirror(&Budget)`), `hub-trusted-catalog-contract` (30: `opened_root::platform::*`), `semio-framework-ui-runtime-presentation` (11: `UiNativeMesh*` live in `ui-runtime/present/resources`, not at the `ui_runtime` root), `os-store-composition` (`ReferenceSpaceMember`), `os-store-engine-live` (`OriginalReplayEdits`), `plugin-plugin-program` (`plugin_runtime::cancel_extension_inference`), `plugin-tool-run` (`protocol::codec::RetainedWireByteSource`), `os-flow-wasm` (`flow_extension_registry_generation`), `os-store-sync-wasm` (`now_ms`), wgpu engine tests (`ui_wgpu`/`SurfaceKind` aliases).
- **test-only crates** (group `touched`, highest counts): plugin-tests-14 (80, `Component`/`UiValue` prelude), os-mcp-hub-remote pair tests (73, tests compile as the pair crate but name `semio_framework_os_mcp_hub_remote::remote`), os-directory-client-tests (private `client` re-exports), os-dsl-tests (`os_kernel::store` is private), `assert_registered_presence_round_trip` (framework law helper removed, 3 editors), `ToolJobDispatch` (layout export tests).

## 2. The ordered plan (`C/cargo-fix-tools/plan/`: `steps.tsv`, `plan-commands.txt`, `summary.json`, `touched.tsv`, `lock-members.tsv`)

`bun C/cargo-fix-tools/plan/gen.ts` (25 s; `git ls-files` + stat, no `find`) builds the package index (3,422 packages, 290 workspaces), the per-workspace path-dependency graph and a workspace-level topological order, then emits **1,283 steps of at most 8 crates**, every chunk in dependency order (leaf first; a dependency of a crate is in an earlier or the same chunk; chunks never mix workspaces or flag classes). Command shape: `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- <check|test> --manifest-path <ws> -p ... --lib --bins [--tests] --keep-going --message-format=short`; `--tests` only for touched crates, wasm steps `--lib --target`.

| group | steps | crates | est. min | content |
|---|---:|---:|---:|---|
| lock (001-015) | 15 | 118 ws | 236 | `metadata --locked`, `update --workspace` on refusal; includes the 106 stale plugin locks (then run `post_lock.py` of testfix-real) |
| value (016-031) | 16 | 55 | 93 | the 35 value-family crates and their closure, leaf first |
| retained (032-075) | 44 | 152 | 255 | `replication-mutation-retained-field` (new carve), `-retained`, `-mutation`, `-messages`, `replication` facade; direct dependents of retained-field and retained (derive macro crate, `os-spr`, stores); rg-core downstream (pack-json, tool-run, job, job-payload); pets-staging and store testing |
| fem (076-078) | 3 | 13 | 19 | `fem-sparse*`, `fem-analyses-model`, `fem-analyses-assembly`, `fem-analyses` (expected red, section 4) |
| proctor (079-086), course (087) | 9 | 17 | 41 | `🎓️teaching/Cargo.toml` proctor closure (check), then the `semio-framework-course*` crates |
| test (088-114) | 27 | 192 | 226 | `cargo test --no-fail-fast` of the touched crates of the groups above, dependency order |
| engines (115-268) | 154 | 378 | 782 | surface, editor, canvas, os-node-graph, plugin-describe, os-infinite, renderer-wgpu closures |
| hub (269-1071) | 803 | 2,102 | 4,163 | the 36 hub lane roots (play lanes and `semio-hub-demonstrator-component`) and the artifact crates below them, per owning workspace |
| touched (1072-1274) | 203 | 445 | 992 | every remaining touched crate (2,118 touched in total, 0 unscheduled) |
| wasm (1275-1283) | 9 | 42 | 59 | `wasm32-unknown-unknown` engines, `wasm32-wasip2` canary `semio-hub-lowpoly-component` then the 35 other lanes |

Milestones (estimates, unmeasured): through step 114 about 14.5 h (value, retained, fem, proctor, course and their tests); through engines about 27 h; hub is the long tail (about 69 h) and is only worth starting after the first lanes are green. Estimates are model numbers (3 min + 0.8 min per crate); recalibrate after step 016.

## 3. Running it (Phase 2)

`C/cargo-fix-tools/plan/run.sh` is the cargo-plan runner with these changes: plan and logs in `C/cargo-fix-tools/plan/` and `C/🗑️generated/cargo-fix/window/`, `STOP_ON_FAIL=1` (stop at the first non-ok step, delete its marker, `FROM=<n>` to resume), `GATE_MODE=deps|all|none` (default `deps`: wait only for a foreign `bun repo dependencies|show`, because conc-cargo makes foreign `bun repo cargo` runs harmless), `LANE=<id>` (exports `SEMIO_CARGO_LANE`). Aligned with conc-cargo lanes: two runners in parallel when load allows, disjoint by `ONLY`:
```
LANE=a STOP_ON_FAIL=1 ONLY='lock|value|retained|fem|proctor|course|test' nohup C/cargo-fix-tools/plan/run.sh &
LANE=b STOP_ON_FAIL=1 ONLY='engines|wasm' nohup C/cargo-fix-tools/plan/run.sh &     # hub only after lane a is green
```
(start detached from the main session: children of a subagent turn die with it; the runner itself keeps retrying infrastructure errors, stops on disk < 40 GiB, alarm 3,300 s per step). Per group: run the check steps until green, fix errors (the tools in `rs/` re-check an edit set statically before the next build), then its `test` steps. `DRY=1 FROM=1 UNTIL=30 C/cargo-fix-tools/plan/run.sh` prints the exact commands (verified).

Before the first step: apply the staged set (`apply.py --write`), re-run `gen.ts` (new manifests, current mtimes), make sure chain15 and the conc-cargo batch are applied, and that `bun repo dependencies lock|sync` is not running.

## 4. fem assembly: origin of `AssemblyOrder`, `SparseTriplet`, `CsrTripletBuild`, `CsrTripletBuildStage` (read-only git history; NOT fixable by renaming)

- **Never defined in any commit.** `git log -S` over `*.rs` finds the four names first in `bd9e4e02bd0` (auto-commit 2026-10-19 12:21, "696") and only as uses; no commit before or after contains `struct SparseTriplet`, `AssemblyOrder<T>`, `struct CsrTripletBuild` or `enum CsrTripletBuildStage`. (`HeapSortCursor` is fine: `semio_framework_value_edit::sort`.)
- Before 696 the same code used the private `AssemblyTriplet`, plain `Vec<usize>/Vec<bool>/Vec<Option<usize>>` plan orders, and `AssemblyCsrBuild` carried the whole inline sort/merge state with a non-funded `step_one()` (110 lines, recoverable with `git show bd9e4e02bd0^:<csr-build-and-run>`; copy kept at `/private/tmp/cargo-fix/old/csr-before-696.rs`).
- 696 migrated the **consumers** only: `assembly/{csr-build-and-run, job-state-and-construction, element-build}`, `analyses/🧪️tests/🧮️triplet-csr` and `🍳️assembly-page-and-mounted-admission-laws`, plus the TS fixtures. The **definitions** (half of a "funded original paged owners" redesign) were never written or were lost. `bfe5c8314f5` (fnsplit4b) then made types `pub` on top of it.
- **Missing surface, derived from the call sites and the two tests** (this is the spec for whoever owns the redesign):
  - `AssemblyOrder<T>` = paged owner with `default()`, `reserve_capacity_one(bytes)`, `reserve_capacity_one_funded(target, grant)`, `capacity()`, `len()`, `get/get_mut/last_mut/pop/push_reserved`, `Index<usize>`, demands (`next_reserve_*_demand`, `allocated_bytes`, `terminal_is_empty`): the existing `PagedList<T, ASSEMBLY_TRIPLET_INDEX_SPACE>` API.
  - **`DofMap` must become paged as well** (the assembly code calls `dof_map.order.reserve_capacity_one`, `dof_map.slots.push_reserved(None)`, `DofMap::slot_count(max_dofs)`, `order.last_mut()/pop()/get(i)`, `DofMap::push((node, dof))`, `get(node_id, dof)`); `analyses-model` still has `order: Vec<(String, Dof)>` and `slots: HashMap<String, [Option<usize>; 6]>`. Also `analyses/🦀️.rs` indexes `dof_map.order[i]` (lines 462 to 584) and `build_rcm_permutation` walks it.
  - `SparseTriplet { sequence: u64, row: u32, col: u32, value: f64 }` (Copy, Debug, PartialEq; former `AssemblyTriplet`), shared by assembly and the sparse crate.
  - `CsrTripletBuild` with pub fields `entries, stage, sort: HeapSortCursor (Copy), count_cursor, unique_count, count_key, merge_cursor, row_cursor, row_counts, indptr, indices, values, last_key, matrix: Option<Csr>, n` and `new(n, entries)`, `next_demands()`, `step_one_funded(grant) -> Result<(bool, RetainedCloneProgress), &'static [u8]>` (a denied axis items/copy/capacity/depth must return `(false, default)` and leave stage, `allocated_bytes()` and `sort` unchanged; `progress.retained_capacity_bytes == allocated_bytes` delta), `take_demands()`, `take_complete(grant)`, `close_step(max)`, `allocated_bytes()`; `CsrTripletBuildStage` = the old `AssemblyCsrBuildStage` with the heap sort replacing `Sort` outer/inner.
- **Options**: (A) finish the redesign (types above plus the paged `DofMap`; the old `step_one` and the `PencilConstruction` funded patterns in `fem-sparse` are the templates; roughly 600 lines and a model-crate change); (B) restore the pre-696 assembly crate (`bd9e4e02bd0^`) for the four assembly files and the two analyses test files and drop the funded API users in `analyses` (loses 696's funded paging and the bfe5c helper split). I staged **neither**: A needs a compiler loop on the paged `DofMap` contract, B discards work I cannot attribute. Plan group `fem` is 3 steps and will be red until one is chosen; `semio-s-fem-analyses-assembly` also has the testfix-real moved law test.

## 5. Coordinator inputs honoured
- testfix-real: the retained chain is group `retained` (leaf first: retained-field, retained, mutation, messages, replication facade), followed by the direct dependents (derive macro crate, `os-spr` and stores); the 106 stale plugin locks are the `lock` group; run `post_lock.py` of testfix-real after it.
- conc-cargo: lane-aware runner (section 3); no dependence on the foreign-cargo gate.
- load rule: one process at a time, `nice -n 10`; the last repo-wide scan (the checker over the overlay, `git ls-files`) finished before the 20:40 notice.

## 6. Cleanup
`/private/tmp/cargo-fix/` (about 500 MB: overlays `r`, `r0`, `old/`) and `C/🗑️generated/cargo-fix*/` are scratch; delete when the ticket closes.

## 7. Compile window (phase 2), log

Lane `cargofix` (lane ids are 1 to 12 lowercase letters or digits: `cargo-fix` is refused), `GATE_MODE=none`, `STOP_ON_FAIL=1`; helpers `cargo-fix-tools/go.sh FROM UNTIL` (start detached) and `wait.sh` (block until stop or end).

| step | outcome | secs | notes |
|---|---|---:|---|
| 001-lock | ok | 106 | first attempt failed in 47 s: invalid lane id; hub `metadata --locked` then failed on a real defect of my staged set (below) |
| 002-lock | ok | 546 | |
| 003-lock | STOP rc=124 | 3,000 | waited in the mem-gov capacity queue ("not enough free memory", spare negative, host swap about 10 to 20 GB, load 30 to 60); the runner alarm counts queue time and expired twice. No compile defect. |

Defect found by the lock group and fixed live: `cargo` refuses a package that depends on the same crate under two names ("depends on crate ... multiple times with different names"). My static set had (a) added a `directory` alias dev-dependency to `semio-hub-space` and `semio-hub-space-laws` next to the existing `semio-framework-os-kernel`, and (b) let `fixdeps` add `semio-framework-os-kernel` next to the existing `directory` alias in the hub `📇️directory/🏛️laws` and `🚀️boot/🏛️laws` test crates. Fixed by using the existing name (space tests now name `semio_framework_os_kernel::io`; the two boot/directory test crates use `directory as ...`). Lesson for the tools: `fixdeps` must check for an existing alias of the same package (all 44 staged manifests were scanned afterwards, no other double name).

State: PAUSED on coordinator order (swap 19.7 of 20.5 GB, load 60). No runner and no cargo of mine is running. Resume with `C/cargo-fix-tools/go.sh 3 15` (then 16 onward; raise the lock-step alarms in `plan/run.sh` first, `cargo_call 1500/2400` and the plan's 3,000 s, because queue waits count against them).

## 8. Compile window status (wave 1020d, 2026-10-21 11:10)
- Map run 016-087 (check, lane cargofix): lock, value, retained, proctor, course libs compile. Remaining lib reds: `semio-framework-os-store-operation-wire` / `-replay` (generic ColdMutation bounds, native control params, ported blind, unverified), fem `semio-s-fem-sparse-subspace` (`SubspaceCheckpoint::advance_shift_operator` is called but defined nowhere since commit bd9e4e0; needs the fem owner).
- Test steps 088-114: `--keep-going` removed from `cargo test` steps (gen.ts + steps.tsv); not executed, starved in the mem-gov capacity queue (need 3584 MB, host load 20-45, 4 foreign cargos running). Step 115 (engines) queued since 10:01, >65 min without start.
- Fixes this window: job-payload `From<JobPayloadAdmissionFault> for ValueError`; value facade exports `OwnedValueRetirementFactory`/`SharedValueRetirementFactory` and ~58 files repointed off `retirement::`; derive `Mutations` stale `ColdMutation<snapshot>` block removed + footprint-law call fixed; space-history mutation kinds/diff split into Mutation/ColdMutation traits; store prepared-retirement `outcome_messages`; wire-factory `begin_copy_byte_demand`; queue tests moved into the queue crate; many test dev-dep/import repairs (dsl-record receiving tests -> dsl-record integration test, course validation test moved, spr/space-schema/pack/ui-builder/job-payload dev-deps).
- iface-wrap: groups tokio+json+wasm applied (30 files) + lock-sync; react group has 19 TS drifts (needs rebuild by its owner). lockref + testfix-real post_lock.py not yet run (after locks settle).
- Pending peer request (not applied): legacy-purge asks to apply tool-iface-tools then legacy-purge-tools apply.py; needs coordinator go.

## 9. Window stop 2026-10-21 16:10
- Runner 115-268 reached step 170; steps 115-169 map done: libs red only in pixels, 2d-booleans, mesh-engine (Sync `RetirementCursor` -> `#[value(send_only)]` added), actor (`next_close_demands` -> `next_close_demand`), ui-prepared (testing helper `close_step` now Result; `ui_contract` dev-dep), surface-paint (`RasterLease` wrap). All fixes unverified by cargo.
- Stopped the runner (host load 38-54, guard waited 10-20 min per step); no orphan `bun repo`/cargo of lane cargofix left.
- Ownership: wire-fix owns store group (durable-group-wire and downstream); fem-finish owns fem sparse-subspace (`advance_shift_operator` missing).
- To resume: `ACCEPT_NEW_PLAN=1 STOP_ON_FAIL= C/cargo-fix-tools/go.sh 170 268`, then re-map 016-087 once, then test steps 088-114 (flags fixed), lockref + post_lock.py last.
