# Result law3: framework source-text laws, rows 55-79, plus the process-family and cache-retirement items

Executor `law3` (Sonnet 5.5), foreground only, no sub-agents, no cargo (only a PATH shim for one law, see row 76), no modifying git command, no `package.json` / `bun.lock` edit. The coordinator STRICT FREEZE (no manifest edits, nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`) was honoured: no file in those areas was touched, and no manifest change is needed (see "Manifest needs").

Abbreviations: `M/` = `🧰️framework/🔨️modules/`, `C` = this ticket folder. Decision classes as in `result-law1.md`: A moved/split -> re-pointed, B renamed -> needle updated, C design change with evidence -> new invariant asserted, D accidentally lost -> production restored, E harness/environment, G open (owner decision or law ahead of production), H needs cargo or native toolchain.

## Result

| | before | after |
|---|---|---|
| rows 55-79 (25 suites) green as a file | 0 | 21 (rows 55-58, 60-66, 68-72, 74, 75, 77-79) |
| row 67 | red (no result) | suite removed by `uicarve` on purpose, no action |
| rows 55-79 still red | 25 | 3: row 59 (1 test), row 73 (3 tests, H), row 76 (1 test, H) |
| `cache/🧹️retirement` | 53/4 | 56/1 |
| `storage/platform/contract` (own suite) | 9/3 | 11/1 |
| `witness` (per test, native grant emulated) | 38/8 | 42/2 (full file stays red: first test poisons the Original) |
| `witness/🪟️windows` | 11/16 | 26/0 |
| `receipt/📦️retention` | 14/1 | 15/0 |
| `build/🎛️environment` | 1/1 (regressed after law1 by two new crates) | 2/0 |

## Harness findings (all reproducible with `C/law3-tools/`)

* **Native grant.** `readNativeImagePublicationOptionsV1` and the whole `witness` family require `capabilities.native = {child:{maximumElapsedMilliseconds}, transport:{maximumBytes, maximumLines}}` (schema `NativeGrant`), which the real launcher issues. `envwire.ts` mints it from `LAW3_CAPS`. With `LAW3_CAPS='{"native":{"child":{"maximumElapsedMilliseconds":600000},"transport":{"maximumBytes":1048576,"maximumLines":32768}}}'` the previous "H" items `publisher options`, `entry refusal`, and all of `witness/🪟️windows` pass. This is what law1 recorded as "capabilities.native not reproducible": it is reproducible.
* **Ticket-local artifact dir.** Several laws (`schema/🧪️tests/🧱️neutrality`, the witness family) accept only `SEMIO_TEST_ARTIFACT_DIR` below `.🧬semio/🦑️repo/🎫️tickets/🎆️YY/🌙️MM/☀️DD/<slug>/🗑️generated` and an absolute path (`LAW3_ART=ticket` / `LAW3_ART_DIR`). The scratchpad dir used by tst2/law1 yields false reds (`Caller-owned ticket output required`, `Canonical publication directories required`).
* **`SEMIO_BUILD_MODE` is a forbidden env name** of the standalone asset helper (`C`-external `☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/asset-publication/📜️script.ts`, `FreshEnvironment`); runners that export it (tst2, law1) break that law (`LAW3_NO_MODE=1`).
* **One failing child poisons its Original.** Files that run children inside one Original (`cache/🧹️retirement`, `witness`) show `Original process admission closed` for every fixture after the first real failure; the first printed error is therefore never the cause. Use `iso.py` (one test per process) and, for fixture arrays, log all `allSettled` reasons.
* **Multi-receive files can be made green as a file**: `withBuildTestInvocationV1` (shared Original) with the expected refusal caught *inside* the callback (row 68). This is the migration law1 had reverted; it works when the law catches the rejection itself.
* Bun evaluates `new URL(..., import.meta.url)` file reads at import time: a module that does this cannot be bundled into a single file (row 58 root cause).

## Decision table, rows 55-79

| # | suite (under `M/`) | before | after | class | evidence and change |
|---|---|---|---|---|---|
| 55 | `📡️replication/⚠️error` | 3/1 | 4/0 | B | glue now re-exports one item per `pub use semio_framework_replication_error::<Item>;` (carve of 690); law asserts the five canonical items instead of the grouped `{` form |
| 56 | `📡️replication/🎮️mutation/🧾️transaction/🎟️admission` | 1/1 | 2/0 | A | ToolRun plugin split: ingress proofs live in `🔌️plugin/⏯️tool-run` tree plus the `🧵️instance` crate; law reads both through `readModuleTreeSource` (all 6 positive and 3 negative needles hold) |
| 57 | `🔲️pixels/🧪️testing/🧭️ownership` | 10/1 | 11/0 | C | `runScriptMain(router, options)` now carries the issued invocation and `router.run` requires it (`checkScriptInvocation`); the mock forwards `options.invocation` |
| 58 | `🕸️graph/🧪️tests/🧩️suite` | 22/1 | 23/0 | D | `process/🧪️testing/🦀️cargo/🟦️.ts` read its policy schema with `readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url))` at import; the graph preview closure pulls it in, so a bundled run failed with ENOENT. It is now a JSON import like its `owned-execution` siblings (bundle probe `C/law3-tools/graphbundle.ts` finds no other `import.meta.url` read in that closure) |
| 59 | `🖱️ui/🌐️locale` | 55/3 | 57/1 | C + G | (1) `semio-framework-ui` is a 2-file crate (uicarve): law walks all 105 ui manifests (none depends on `semio-framework-os-kernel`) and requires the locale package in the 7 declared carved owners (`uiTree`, `localeOwners` in the fixture). (2) wgpu root removed (uicarve section 3): law asserts it is gone and no `pub use dsl::` below the wgpu tree. (3) OPEN G: generated `Locale`/`Terminology` still derive `Default` with `#[default]` on the first entry (`🎚️axes/📽️projection`); AGENTS.md demands no default language and the law says so. Not changed: `Locale::default()` is used in `🦑️repo/…/🎛️dashboard/⚙️preferences:191` and census of struct fields typed `Locale` is not decidable without cargo |
| 60 | `🖱️ui/…/⚙️engine/🎟️presented-source` | 0/1 | 1/0 | A | `is_some(){return false}` lives in `⚙️engine/🪟️surfaces`, which the engine root no longer mounts; law reads root tree plus surfaces |
| 61 | `🖱️ui/…/🧊️wgpu/🖍️draw` | 12/2 | 14/0 | A + B | old `wgpu/🦀️.rs` aggregate gone (law asserts absence); shader owner uses `…_prepared_shaders::UI_SHADER` directly, and the shader consts moved into three submodules (reads via `readModuleTreeSource`, robust against the peer carve in progress) |
| 62 | `🖱️ui/🧠️runtime/…/📨️message` | 0/1 | 1/0 | A | schema import pointed at the sibling `transport/🧬️schema`; the owner's own `📨️message/🧬️schema` is the right one |
| 63 | `🖱️ui/🧪️tests/♻️physical-job-close` | 7/2 | 9/0 | A + B | end marker `pub struct FrameActionEnvelope` / `pub struct FrameDeferredExecutionEnvelope` (visibility added by the carve); rustfmt'd fields (`interaction: Option<…>`) compared whitespace-insensitively; `winit-app` and `frame-job` are read as module trees (a peer is carving them right now) |
| 64 | `🖱️ui/🧪️tests/📐️dock-axis-geometry` | 0/1 | 2/0 | A | `applyAxisResizeDelta` moved to `🧱️elements/🎨️Canvas/🔗️join-corners` |
| 65 | `🖱️ui/🧪️tests/📦️prepared-close` | 3/1 | 4/0 | C | `PreparedRenderJob::step` returns `OriginalTurnFailure` (cstep API), law pins `Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure>` |
| 66 | `🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit` | 4/1 | 5/0 | A + B | `LayoutWorkerAdmission` and the close function are in `⚙️engine/🪟️surfaces` (fixture key `engineSurfaces` added); admission fields are `pub` now |
| 67 | `🖱️ui/🧪️tests/🧊️feature-ownership` | -/- | removed | C | deleted by `uicarve` ("audited feature gating that no longer exists", `result-uicarve.md` section 3); no re-creation |
| 68 | `🖱️ui/🧪️tests/🧭️native-command` | 0/1 | 17/0 | C + E | script runs Cargo through `runAdmittedCargoTestsV1` / `runAdmittedCargoCommandV1` (provider port, inv work): law mocks the provider module and `readCargoTestPolicyV1`, spreading the real exports (the old whole-module mocks hid `captureOwnedCommandV1`); one shared Original via `withBuildTestInvocationV1` with refusal caught inside makes the multi-receive file green as a file |
| 69 | `🖱️ui/🧬️contract/…/🔮️corpus-oracles` | 3/1 | 4/0 | A | `runtime-tree-retirement` helper read `♻️reconcile/🦀️.rs` and `⏭️cursor/🦀️.rs` as flat files; both are module trees now |
| 70 | `🖱️ui/🧬️contract/…/🧱️owners` | 4/3 | 7/0 | D + C + B | D: `🌳️typed-descendants/🦀️.rs` had `include_str!("../../♻️retirement/🌳️typed/🧩️components.json")` doubled by the carve (resolves to `…/🌳️typed/♻️retirement/…`); fixed to `../../🧩️components.json`. C: the `🧪️conformance` companion mounts test roots from sibling owner folders by design (`companion: true` in fixture and schema; escape check applies to the contract root for it, uniqueness is unchanged). B: generated TS golden header `bun nx run` -> `bun repo run` in `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` and `🪪️manifest/🟦️.ts` (all generators already emit `bun repo run`) |
| 71 | `🗣️dsl/🧬️schema/✨️derive/…/🧱️ownership` | 7/1 | 8/0 | C | bundle of `📦️packages/🦀️rust/📜️script.ts` legitimately includes framework package code under `🧰️framework/📦️packages/`; law accepts `🔨️modules/` and `📦️packages/` roots (product roots still throw in the esbuild plugin) |
| 72 | `🚪️io/⏱️control/🛫️snapshot` | 6/1 | 7/0 | B | `std::mem::size_of` is imported: needle `copied_bytes:size_of::<Option<O>>()` |
| 73 | `🚪️io/🪶️sqlite-snapshot/…/🤝️interoperability` | 0/3 | 0/3 | H | needs the cargo-built `semio-io-sqlite-snapshot-oracle` binary (`CARGO_TARGET_DIR` / `SEMIO_SQLITE_SNAPSHOT_ORACLE`); not built here |
| 74 | `🧊️3d/🥽️mesh/🛠️modeling/✂️clipping` | 8/7 | 15/0 | A + C + D | modeling and document are module trees now (reads via `readModuleTreeSource`); `MeshModelingOps` trait/impl live in `🛠️modeling/🧰️modeling-operations`; weld-key needle is the new `let point=…;let key=mesh_weld_key(point,…)?;` (B). D: the brep mesh extension (`✏️s/…/📐️brep/🥽️mesh/🧮️operator-core`) calls 50 `MeshModelingOps` methods but imported no trait since the document/modeling split; added `use semio_framework_3d::mesh::MeshModelingOps;` (the law's caller list already names this caller). Packaging tests of rows 74/75 shared two failures, see row 75 |
| 75 | `🧊️3d/🥽️mesh/🧪️tests/📦️targets/🧪️packaging` | 0/2 | 2/0 | C | `semio-framework-test-fixtures` is a dev-dependency of `mesh-document` and `mesh-tests` (fixture reader replaces `include_str!`); corpus `🧪️.json` updated; framework and the other 29 locks already carry the edge |
| 76 | `🧬️schema/✅️validator/…/🧩️neutral-owner` | 0/1 | 0/1 | C + H | three layout defects fixed: ownership law's fixture path lacked `🧪️tests/` (A); source-owner fallback copied the first three path segments, which copied all of `🧬️schema` including `⚛️component` (now the source's directory); the copied workspace must live at `🧰️framework/Cargo.toml` (the validator's `workspace = "../../../../.."`) with the framework lock, as law1 did for `value/neutral-owner`. With a `cargo` shim on PATH that exits 0 the setup and every pre-cargo assertion pass; the real `cargo test --offline` is unverified (H) |
| 77 | `🧬️schema/🧩️composition/📥️admission` | 3/1 | 4/0 | C | `📋️callers.json`: 7 plugin roots (`wires`, `forms`, `rewriting`, `jack`, `dag`, `note`, `curation`) plus `equation` and `builder-contract-part-1` are facades or moved text now; those 8 rows dropped (viewer/editor rows of the same artifacts remain), 2 raster rows re-pointed to the viewer/editor crate manifests that declare the dependency, 2 rows added (`rewriting`, `jack` snapshot, manifest `🧬️schema/📦️packages`); floor (`minItems: 80`) kept: 81 rows |
| 78 | `🧬️schema/🧪️tests/🧱️neutrality` | 12/2 | 14/0 | E + C | E: ticket artifact dir. C: the replication glue crate has `semio-framework-schema-registry` only as dev-dependency (production wire no longer uses it, `🧪️tests/🔬️facade` does); law asserts registry in `[dependencies]` or `[dev-dependencies]` and `semio-framework-schema` in neither |
| 79 | `🧬️schema/🧪️tests/🩹️fragment-validation-oracle` | 0/1 | 1/0 | A | fixture path had its file name joined twice |

## Decision table, process family and cache retirement

| suite (under `M/🏃️process/`) | before | after | class | evidence and change |
|---|---|---|---|---|
| `📦️artifacts/💾️cache/🧹️retirement` (row 41) | 53/4 | 56/1 | B + C + born-red | `git log -S` finds `requireCompilerWorkspaceProducerCustodyV1` only in the law's own commit 7880cc3: it never existed in production. The compiler store moved to *borrow* custody: `withRepositoryCompilerArtifactCacheV1` -> `withCompilerWorkspaceBorrowV1` + `requireCompilerWorkspaceBorrowCustodyV1`, writers are owned by `withNativeArtifactCacheProducerV1`. Law restated: borrow custody uses `requireRetainedOperationFrameV1` and never `requireCompilerWriterCustodyV1`; writer custody still needs the live writer (`Original live compiler writer required`); compiler root uses `withCompilerWorkspaceLifetimeV1`/`…BorrowV1` and not `…WriteV1`; cargo state consumes the borrowed volume. Producer volume: renamed needles (`if(fresh)return withNativePlatformFundedChildStorageV1`, `cwd:paths.cwd`, `producer.accept(()=>ledger.run(`, `…scratch retained`); the removed inline `producerIdentity`/`maintenance`/`rmdirSync` protocol is replaced by `binding()` (`child.directory`, `child.parentIdentity`), `image.withExclusiveMaintenance({version:1,wait:true}` with `Actual producer kernel EX required`, and `retireNativePlatformFundedChildStorageV1(opened.image,held.reservation`, plus `not.toContain("rmdirSync(")`. Born-red typo: `JSON.parse(line.slice(21))` for a 20 character prefix. OPEN (E/G): `durable bounded journal` still red: children of fixtures 0 (`preflight-normal`: `Canonical bounded underlying mount directory required`), 2 (`creator-claim fresh`: `Exact live original writer proof required` only when the child env is derived from the parent Original), 7 (cold namespace child) and 8 (`Mapping child ended before entered`) fail; fixtures 1,3,4,5,6 are poison cascade |
| `💾️storage/🌐️platform/📋️contract` (row 22) | 9/3 | 11/1 | C + E | extraction found 2 `withExclusiveMaintenance` arrows, the persistent one lives in `🧹️cleanup/🎛️command` (third path added). The three arrows share no free-variable set any more (`withNativeImageBorrowerOperationV1`, receiver state machines, no `shared.release()`), so the stub model cannot run all of them: the image arrow keeps the dynamic model (stub `withNativeImageBodyOperationV1`, interrupts precheck/body/postcheck/failure/undefined, 2 rechecks, value identity); the fixed and persistent arrows are asserted structurally (writer proof capture, borrower operation, physical recheck after the body and before `acquired:true`, `acquired:false` only for the exact timeout, `finally{held.live=false;}`, terminal close). `historical retirement … queued exclusive acquisition`: receipt fixtures needed `mountDirectory` (schema `ImageReceipt`, added). OPEN (G): retirement resolves `true` in 10 ms while a blocker holds the same lease resource in the same Original (no queueing observed); needs a lease owner decision |
| `…/📤️publication/📚️witness` (rows 24, 27) | 38/8 | 42/2 per test | E | `entry refusal` and `publisher options` are the `native` grant (above), not defects. OPEN: `descriptor benchmark` (55 s, `Original resource loan deadline exhausted`: host load, plus it poisons the file), `prepared parent isolated failures settle` depends on a helper under the closed ticket `☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/asset-publication` whose `🗑️generated` output dir no longer exists (`SEMIO_TEST_ARTIFACT_DIR` must equal it): G, the helper must become a permanent fixture or the law must drop it |
| `…/📚️witness/🪟️windows` (row 25) | 11/16 | 26/0 | E + C | native grant; six `LogicalObservation*` / `LogicalEntryBound*` definition hashes and `witnessSha256` in `🔣️.json` were pinned before the logical-observation protocol grew (the tests that exercise those definitions pass): re-pinned to the independently recomputed values (the law still recomputes them in Python) |
| `…/📤️publication` `🧪️tests/📦️owner`, `🧪️tests/🟦️.ts`, `🔎️receipt-conflict` (rows 26, 27) | 3/13, 16/61 | files removed | - | staged deletions (`D `) by a concurrent peer; successors `🧱️fixed/🧪️tests` 4/0 and `🧱️fixed/💾️metadata/🧪️tests` 5/0 are green. The `publisher lifecycle success row` and `receipt conflict` coverage exists in no remaining file: the peer must confirm that loss is intended |
| `🧾️receipt/📦️retention` (row 39) | 14/1 | 15/0 | D | `CargoGraphOperationsV1.start` threw `Original process admission closed` synchronously from a `Promise`-returning method once the Original was poisoned, so `.catch(()=>{})` could not absorb the late start and the primary (`graph-primary`) was replaced; `start` now returns a rejected promise |
| `📥️inputs/🧪️tests/📥️consumers` (row 34) | 0/2 | 0/2 | G | law injects a model whose `expect` assertions run inside issued operations; failures leave the Original callback and the file reports the poison error. Needs a model that records mismatches and reports after settlement |
| `🧪️testing/🦀️cargo/🏃️execution` (row 46) | 15/2 | 15/2 | G | the law builds a literal `ScriptInvocation`; `checkScriptInvocation` refuses forged objects, so 11 of 13 rows pass **vacuously** (refused for the wrong reason) and `original` / `cancel-during-execution` fail. Needs per-row issued invocations (`createScriptProcessEnvelope` + `withScriptProcessEnvelope`) and tamper rows re-expressed against frozen invocations |
| `🏗️build/🎛️environment` | 1/1 | 2/0 | D | 12 generated `🦀️ semio-framework-ui-contract-values-laws` and `semio-framework-quiz-tests-runs` launch entries (build, check, test, two modes) lacked `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` / `NODE_DISABLE_COMPILE_CACHE=1` in `.vscode/launch.json`; inserted. Same producer defect as law1: the generator of per-crate entries must emit `env` |

Not touched (still red, unchanged owners): `component-pass` (needs nightly cargo), `leases/overlap`, `custodian`, `scope`, `recipe`, `cleanup/{command,operation}` multi-receive suites that pass per test, `cache` descriptor wire (Python oracle disagrees with corpus row 32), `sources` (5 red as a file, 0 per test), `inventory` (30 s deadline), `cold`, `exact/image`, `lease/callers` (`immer` unresolved).

## Files changed

Production and generated:
- `M/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` (policy schema as JSON import, unused `readFileSync` import removed)
- `M/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/📥️planning/🟦️.ts` (`start` returns a rejected promise)
- `M/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧪️tests/🌳️typed-descendants/🦀️.rs` (include path)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧮️operator-core/🦀️.rs` (`MeshModelingOps` import)
- `.vscode/launch.json` (12 entries, env)
- `M/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`, `M/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` (generator header only)

Laws, fixtures, corpora (all under `M/`):
- `📡️replication/⚠️error/🧪️tests/🟦️.ts`, `📡️replication/🎮️mutation/🧾️transaction/🎟️admission/🧪️tests/🟦️.ts`
- `🔲️pixels/🧪️testing/🧭️ownership/🧪️tests/🟦️.ts`, `🚪️io/⏱️control/🛫️snapshot/🧪️tests/🟦️.ts`
- `🖱️ui/🌐️locale/🧪️tests/🟦️.ts` + `🧫️fixtures/🔣️.json`; `🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🎟️presented-source/🧪️tests/🟦️.ts`; `…/🖍️draw/🧪️tests/🟦️.ts`; `🖱️ui/🧠️runtime/🎭️present/📦️resources/📤️transport/📨️message/🧪️tests/🟦️.ts`
- `🖱️ui/🧪️tests/♻️physical-job-close/🟦️.ts`, `…/📐️dock-axis-geometry/🟦️.ts`, `…/📦️prepared-close/🟦️.ts`, `…/🔬️targets-wgpu-mounted-layout-unit/🟦️.ts` + `🧫️fixtures/🫴️receiving.json`, `…/🧭️native-command/🟦️.ts`
- `🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🔬️runtime-tree-retirement/🟦️.ts`
- `🖱️ui/🧬️contract/🧪️tests/🧱️owners/🟦️.ts` + `🔣️.json` + `🧬️schema/🔣️.json`
- `🗣️dsl/🧬️schema/✨️derive/🧪️tests/🧱️ownership/🟦️.ts`
- `🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🟦️.ts`, `🧊️3d/🥽️mesh/🧪️tests/📦️targets/🧪️packaging/🧪️.json`
- `🧬️schema/✅️validator/🧪️tests/🧩️neutral-owner/🟦️.ts`, `🧬️schema/✅️validator/📏️ownership/🧪️tests/🧫️fixtures/🔣️.json`, `🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json`, `🧬️schema/🧪️tests/🧱️neutrality/🟦️.ts`, `🧬️schema/🧪️tests/🩹️fragment-validation-oracle/🟦️.ts`
- `🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts`, `🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts`, `…/📤️publication/📚️witness/🪟️windows/🧪️tests/🔣️.json`

Temporary instrumentation (`[DEBUG]` prints in `🔒️leases/🟦️.ts` and in four law files) was added and removed; `🔒️leases/🟦️.ts` is byte-identical to its pre-instrumentation content (a transient syntax slip while restoring was fixed and checked with `bun build`). `[DEBUG]` lines that were already present (`cache/🧹️retirement` 4, `ui/🧬️contract/…/🧱️owners` 1) belong to earlier work and were left.

## Manifest needs (frozen, not edited)

None. The `lru-cache` / `immer` / hoist needs from law1 are unchanged.

## How verified

Per suite, full file: `LAW3_ART=ticket LAW3_MAX_MS=3600000 LAW3_CAPS=<native grant> python3 C/law3-tools/run1.py <file>`; per test: `iso.py <file>`; ad hoc: `runt.sh <file> <name-regex>` (all from the repo root, foreground). Final sweep of rows 55-79 with `runall.py list-rows.txt` after the last edit gave the "after" column except row 61 and 63 (re-run individually after concurrent peer carves, 14/0 and 9/0). Probes: `graphbundle.ts` (bundle `import.meta.url` census), `callers.ts` (composition callers audit), `owners.ts` / `neutral.ts` (source-owner closure replay), `methods.ts` (maintenance arrows), `rserr.ts` (tree-sitter error locator). No cargo build was run; `cargo` was replaced by a PATH shim only to prove the pre-cargo setup of row 76. Output under `C/🗑️generated/law3-out` and `…/ta` deleted (one leftover mounted fixture image, `hdiutil`, was ejected first).

## Still open (owners)

1. Row 59: remove `Default` from the generated `Locale`/`Terminology` (decision on `ui_locale::Locale::default()` in the repo dashboard; census of structs with a `Locale` field needs cargo).
2. Rows 73, 76: need cargo (oracle binary; offline validator test in the copied workspace).
3. `cache/🧹️retirement` `durable bounded journal` (children fixtures 0, 2, 7, 8), `platform/contract` queued retirement, `witness` benchmark and `prepared parent isolated failures` (closed-ticket helper), rows 34 and 46 (model redesign; row 46 is vacuous today).
4. Peer confirmation that the deleted `publication/🧪️tests/📦️owner` and `🔎️receipt-conflict` laws are intended to go.
5. The generator of per-crate launch entries must emit `env` (twice regressed).
