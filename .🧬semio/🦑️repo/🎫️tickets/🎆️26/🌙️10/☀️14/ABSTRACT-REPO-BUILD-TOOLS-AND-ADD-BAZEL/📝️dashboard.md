# Dashboard Build Tool Boundary

## Changes

- The neutral build runner in `📚️library/🏗️build/📜️script.ts` now detects per-mode installed dashboard launches and hands over directly with the original cancellation/deadline and selected backend.
- Dashboard registry effects use `buildFlags`/`BuildFlags`/`build_flags` and neutral task options. Target, group, playground, declared tool and projected editor commands invoke `bun repo`.
- Dashboard inventory consumes `.🧬semio/🦑️repo/⚡️cache/build/graph.json` with `version: 1`, direct `projects` records and dependency arrays. The snapshot version is now 7. Backend graph errors are represented as neutral strings.
- Graph reuse compares the neutral graph publication and manifest content hashes. Nx file map reads were removed. The runner receives neutral terminal, graph reuse, dynamic output and invocation environment names.
- Launch generator, launch schema, shared fixtures, authority schema and independent TypeScript oracles use the neutral interfaces.
- Editor seed and generated output include neutral commands, options and serialized capability argument arrays. Public backend environment entries were removed. `normalizeDashboardLaunchBuildV1` is called by projection and by the package launch write owner; recursive normalization covers fresh backend command text, argv, nested capability JSON and backend environment fields, while preserving Nx/Bazel selector choices. RFC6902 and Ajv independently validate the normalization vector.
- Projection refresh preserves a peer configuration appended after an existing owner block. Added an explicit regression assertion. Fixed preexisting launch/authority fixture paths and one generated configuration with missing debug type/request.

## Validation

Executed with Bun 1.3.14:

- Launch and control-plane suites: 15 tests, 767 assertions passed. Independent libraries: Ajv, fast-json-patch, Node readline/Buffer, strip-ansi and Nx task graph scheduling. Runtime `[DEBUG]` output confirmed neutral commands for 496 current ticket tools and 992 English/German launch entries.
- Registry declaration and neutral publication schema subset: 2 tests, 20 assertions passed. Ajv accepted neutral shared vectors and rejected the obsolete backend graph/effect shape.
- Installed native launch argument selection and neutral runner bridge: 2 tests, 25 assertions passed, with Nx independently confirming task scheduling. A controlled immutable installation received daemon status, release mode and Bazel backend before graph discovery; `[DEBUG]` output confirmed the handoff.
- Root authority subset: 4 tests, 22 assertions passed, confirming the actual root package script and declared script command.
- Both editor documents parsed as JSON after neutralization. `git diff --check` reported no whitespace errors.

## Native Validation

The actual dashboard Rust library compiled during qualification. A retained compiled binary then passed **41 registry/inventory tests**, with **169 other tests skipped**, through the neutral Bun → Bazel → Nextest execution path. The command exited 0 and Bazel reported successful action completion.

The successful filter was:

```text
(test(/^registry::tests::/) or test(/^inventory::tests::/)) and not (test(the_job_publishes) or test(global_axes_select_tools))
```

The invocation used `SEMIO_BUILD_REUSE_GRAPH=true bun repo --tool=bazel exec --projects=@semio-tech/repo-dashboard-rs -- cargo nextest run --binaries-metadata <retained metadata> --manifest-path <dashboard Cargo.toml> --config-file .config/nextest.toml --profile fundamental -E <filter> --no-fail-fast --status-level=pass --final-status-level=pass`. It did not regenerate the project graph or rebuild Rust dependencies.

Selected successful native proofs:

- `registry::tests::a_build_reuses_the_neutral_graph_only_while_its_manifest_inputs_are_unchanged`
- `registry::tests::a_target_resolves_axes_then_parameters_then_extra_arguments_after_the_double_dash`
- `registry::tests::a_group_runs_several_targets_over_project_patterns_unchanged`
- `inventory::tests::discovery_reads_manifests_the_graph_the_catalog_and_the_open_tickets_and_nothing_else`
- `inventory::tests::a_snapshot_serves_an_unchanged_workspace_and_gives_way_to_any_changed_source`
- `inventory::tests::a_corrupt_snapshot_or_one_of_another_workspace_is_ignored`
- `inventory::tests::a_partially_published_graph_is_a_problem_per_recorded_plugin_error`

### Qualification Fixes and Limits

Qualification found and repaired three defects within this change's edited sources: the shared catalog fixture path was stale, `SEMIO_BUILD_DYNAMIC_OUTPUT` was incorrectly reserved despite being emitted by the playground contract, and one native assertion expected choice id `ship` instead of its authored environment value `release`.

After the catalog and reservation repairs, the named dashboard test target compiled and ran 36 selected tests: 35 passed and the stale assertion failed. The assertion was corrected in source. A new compile attempt then failed in an unrelated, concurrently edited UI scene dependency: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/📦️resource/📥️receiving/🦀️.rs:34` returned `Option<Mesh3dResourceOwner>` from an `if` branch requiring `()`, Rust E0308. That dependency was not edited here.

Consequently, the 41-pass qualification deliberately reuses the prior binary and excludes the source-only corrected assertion. This is native evidence for the selected production behavior, **not a claim that every final source revision compiled or that the full 210-test dashboard suite passed**. The earlier broad suite had also stopped at a missing configured catalog prerequisite after 15 passes. Four current edited Rust modules/unit modules parsed with Rustfmt after the final source edit; an independent JSON-fixture/source check confirmed that choice `ship` emits `release` and the corrected assertion expects `release`.

### Source and Binary Provenance

- Nextest retained metadata (removed after recording): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/🗑️generated/semio-nextest-GiPOYG/binaries-metadata.json`
- Metadata SHA-256: `33e598cd9ddf9713610daa71bee23612e0df2fb55b1976801ff83dbc7fc73bd7`
- Binary: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture/prototype/build/semio-framework-repo-dashboard/eefc9aec4c064563/out/semio_framework_repo_dashboard-eefc9aec4c064563`
- Binary SHA-256: `b32226e5f2ad5e77fbb46f3d0d75f5e8bf187fe6f590e37501da2d062a6fc977`
- Binary modification time: `2026-10-14T11:31:18.457891+00:00`
- Nextest run id: `4bd54518-2c8e-4582-a187-6b73d8fab11b`; summary: `41 tests run: 41 passed, 169 skipped`.

Current source digests (the registry unit source includes the assertion correction absent from this binary):

| Source | SHA-256 |
|---|---|
| `🎮️registry/🦀️.rs` | `57cd6603c0b5cb0097eb3601a9f6035bd1397ae1fe9564bd7cbdb4e43aca17ad` |
| `📚️inventory/🦀️.rs` | `bbe3c38dea0405a9b62a9e514eafcaed12a29d37a711ae3e1335fa47a9f80eea` |
| `🎮️registry/🧪️tests/🔬️unit/🦀️.rs` | `62c4fb37fa1170eb13ae5400b1a84ce0b108f042d5dbc4aeac21e6997c983b85` |
| `📚️inventory/🧪️tests/🔬️unit/🦀️.rs` | `aa7bc163de88af8856b9781ea6402d0877183756b9253c388e8389941c9976ad` |
| `🧪️tests/🧫️fixtures/🎮️registry/🏗️workspace.json` | `f237a4ff39c7776e181b47da4ce5fdc7fbea4b90371df86c4826a79012040e10` |
| `🧬️schema/🎮️registry/🔣️.json` | `1fb0b5aaa77764deb7c7390cfed9ac9d0b50873fbc0562b44f85cc26592fb96e` |

## Notes

Other concurrent workers continue appending backend-specific launch rows. The active dashboard launch projection and package write owner now normalize these fields at publication. The current root script contains no general launch-seed generator to call; searches of active source found only the dashboard owner. Both documents were normalized using that same exported production helper. Generated installed-runner fixture paths assert the 256-character limit and the fixture removes its generated tree afterward.

## Owned Files

`📥dashboard-files.json` records 38 exact owned source/editor/report paths from the prior edit inventory. It does not attribute other chats’ shared changes from Git history. Qualification logs and this qualification’s fixture/Nextest output trees were removed after their results and provenance were recorded.

## Authored CLI Installed-Fast-Path Regression

Added a language-neutral `installedRunner.authoring` vector before extending the existing installed-runner test. The new case launches a real Bun subprocess that imports the actual `runBuildCliV1` from its repository source, passes the fixture root and leading `repo` arguments, supplies only authored `SEMIO_SCRIPT_POLICY` and `SEMIO_SCRIPT_CAPABILITIES`, and explicitly deletes `SEMIO_SCRIPT_PROCESS_INVOCATION`. It reuses the controlled immutable installed Bun executable and generates only the short receiving fixture script; repository source is imported directly rather than copied.

The controlled daemon calls the actual framework `receiveScriptProcessInvocation` and records the received process envelope and deadline before/after a 40 ms delay. Independent Ajv validation accepts the authored capabilities with omitted optional `tool`. Assertions confirm:

- Authored owner `dashboard:authored-fast-path` and maximum elapsed time `30000` remain intact.
- Workspace root, exact selected argv and launches remain intact; inferred tool is `bazel` and mode is `release`.
- Custom native transport (`65537` bytes, `321` lines), child deadline (`7000` ms), offline grant (`true`) and native artifact directory survive unchanged, despite ambient offline selection being `false`.
- The actual receiving control uses the transferred deadline and consumes its remaining budget across the 40 ms delay; it does not receive an unbounded or reset budget. The final run logged `[DEBUG] authored installed dashboard owner=dashboard:authored-fast-path remaining=29924`.
- The neutral graph publication is absent after the successful installed fast path.

Executed the existing execution suite with `--test-name-pattern installed` and `SEMIO_TEST_ARTIFACT_DIR` set to this ticket's generated root: **1 test passed, 16 assertions, 5 other tests filtered out**. No backend graph discovery or native Rust compilation was requested. The fixture asserts its executable path is at most 256 UTF-16 code units and removes its generated directory afterward. The qualification log was removed after recording this result.

Only the existing execution test and its shared JSON fixture were edited for this regression. Both paths were already present in `📥dashboard-files.json`; the verified inventory remains 38 paths.

- Current `🧪️tests/🧊️execution/🟦️.ts` SHA-256: `dabcda0ba7c75a26745c384ebdaec3d25210a9a96c14f2e19d77c9b92566d3fa`.
- Current `🧪️tests/🧫️fixtures/🧊️execution/🔣️.json` SHA-256: `42e9a9eac51c7f56955b2e61c5946e7953201020f05c1d02f290c1a33cd35f95`.

## Final Strict Launch Type Qualification

Strict TypeScript checking was run for exactly the production launch helper and launch test after the quoted/unquoted private-bootstrap normalization change. The first check reached TypeScript 5.9.3 and reported one local diagnostic: `🧪️tests/🚀️launch/🟦️.ts:41`, TS18048, `result.inputs` possibly undefined. No unrelated types blocked the check. At the parent agent's explicit request, the test access was changed to `result.inputs![0]`, consistent with the production projection always returning an inputs array and the existing schema/runtime assertions.

The exact two-file check then exited **0**, using the existing compiler through neutral Bazel execution and the reused graph:

```text
SEMIO_BUILD_REUSE_GRAPH=true bun repo --tool=bazel exec --projects=@semio-tech/repo-dashboard-rs -- bun <workspace>/node_modules/typescript/bin/tsc --noEmit --strict --skipLibCheck --allowImportingTsExtensions --resolveJsonModule --esModuleInterop --isolatedModules --target ESNext --module ESNext --moduleResolution bundler --lib DOM,DOM.AsyncIterable,DOM.Iterable,ESNext --types bun <launch-helper> <launch-test>
```

The two explicit inputs were `🎛️dashboard/🌳️command-tree/🚀️launch/🟦️.ts` and `🎛️dashboard/🧪️tests/🚀️launch/🟦️.ts`. No emit or graph regeneration was requested. The four launch tests were also run through neutral Bazel execution: **4 passed, 524 assertions, 0 failures**, including independent Ajv and RFC6902 validation of command/capability normalization. Both Bazel actions and both neutral runner invocations completed successfully. Core was notified that these sources/tests were stable.

Preserved generated logs for the parent agent's final cleanup:

- `🗑️generated/dashboard-launch-strict-types.log`: first strict runner failure.
- `🗑️generated/dashboard-launch-strict-diagnostics.log`: exact compiler diagnostic.
- `🗑️generated/dashboard-launch-strict-green.log`: successful exact two-file strict check.
- `🗑️generated/dashboard-launch-final-tests.log`: four successful launch tests.

The optional-input access was the only source edit made during this final qualification; no launch document or Rust source was edited. The existing owned-file inventory already includes the test.

- Final `🌳️command-tree/🚀️launch/🟦️.ts` SHA-256: `eed52ac6683ca4abebcfd3b6af0373004cbc44795b97c19f0571500d77a118f3`.
- Final `🧪️tests/🚀️launch/🟦️.ts` SHA-256: `a0d82e1f91cf4d51105112cafbe06030fac35fe4a5bc1d3c52340d2ea07eee88`.

## Consumed Observation Authority Regression Fix

At the parent/core owners' request, this agent took ownership of the observation helper, its existing law tests/portable fixture, observation schema, and the single kind enum in the graph authority schema.

Directory observations now fingerprint the exact returned listing from one read. Typed entries and plain names have separate portable fact kinds. Every observed TypeScript resolution-host port records its actual returned result: file/directory existence, text, real path and directory list. The replay implementation handles TypeScript UTF-8, UTF-16LE and UTF-16BE BOM decoding; the existing third-party TypeScript host independently validates those outputs. No runtime library dependency was added.

Scope, global capture and cached-project inheritance retain every distinct actual fact. Repeated identical bytes with restamped metadata remain semantically consistent; incompatible consumed facts for the same kind/path refuse inference before publication. Nested inference and valid cache hits now inherit facts into their parent scope. Failed reads retain their error facts. Nonblocking validation accepts an optional original cancellation signal and checks it before validation and between rows.

TDD evidence: the initial new-regression selection executed 13 tests against the old helper and reported 13 failures. Some initial capture cases also exposed test module-load pollution; test setup now explicitly seals module-load observations before exercising isolated captures. After the implementation and setup fix:

- Full standalone observation law file: **34 passed, 0 failed, 109 assertions** in 73.14 seconds. This includes all 21 original laws and 13 new consumed-result, BOM, conflict, cached inheritance, failure and cancellation cases.
- Core graph capture selection, `--test-name-pattern 'observed authority|failed reads'`: **2 passed, 0 failed, 15 assertions**, 17 filtered out.
- Existing independent Ajv persisted-record validation and fast-glob directory oracle pass; new BOM vectors agree with the actual installed TypeScript `sys.readFile` implementation.

Commands were direct explicit-path `bun test ./...` executions with `SEMIO_TEST_ARTIFACT_DIR` set to the ticket generated root. No workspace graph regeneration, native compilation or Node SDK project admission suite was requested. The first discovery-style Bun test filter was corrected to an explicit `./` path before collecting TDD evidence; its search-only result was not counted as a test execution.

The graph/core owner received the stable helper handoff and exact kinds after both green runs. Preserved generated logs: `dashboard-observation-red.log`, `dashboard-observation-green.log`, and `dashboard-observation-capture.log`, all under `🗑️generated`, for final ticket cleanup.

## Compact Input Expansion Shape and Allocation Fix

Corrected the private `portableInputs` helper in the shared graph owner, coordinating narrow ownership with core. Core retained loader/publication/performance work and added the optional invocation argument at its existing call sites.

The helper now scans ordinary values synchronously, collects unique singleton `{env:compactToken}` declarations through a WeakSet, and returns the original value immediately when there are no compact tokens. Actual closure reads run through a shared unique-token map with at most 16 workers. Content hashes remain verified against the declared token. Failure stops scheduling and drains started reads before rejection. A synchronous WeakMap transform copies only containers whose children changed; arrays flatten only direct recognized compact-token entries, preserving ordinary nested metadata/execution arrays. Large expanded lists append without spread-argument limits. Original invocation checks occur periodically during traversal/transformation and before/after each read.

Added language-neutral graph fixture vectors with source/lock contracts, RFC6902 input/expected patches, ordinary nested arrays, a literal non-token metadata object, repeated tokens and a 21-closure concurrency vector. Tests enter the existing public `pinBuildGraphV1` / `loadBuildGraphV1(...,{reuse:true})` path. They do not export a new testing API or perform full discovery.

After correcting the test oracle import to the already installed `fast-json-patch`, both new tests failed against the previous implementation with array-shape differences. After the fix, the final focused run passed **5 tests / 26 assertions**: existing independent graph JSON Schema and esbuild checks, existing pinned authority reuse/refusal, and both new expansion regressions. Runtime logs confirm exact ordinary JSON digest preservation, one read per repeated token, **16 maximum concurrent reads**, and refusal after compact content corruption. Ajv independently admits the source/lock fixture contracts against their existing owned schemas; Bun CryptoHasher supplies the independent content-digest oracle.

Strict TypeScript checking used the installed compiler with the repo owner's `allowJs:true`, `strict:true` and matching module/target options. The production graph/helper file alone passed with exit **0**. The combined graph/helper and existing graph-test check has no remaining local diagnostics, but its final run failed on unrelated concurrent `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts:117` errors (`plan`/`options` undefined and await in a non-async context). Core authorized correcting the existing fixture index cast in its earlier capture test; that local diagnostic was fixed. This is not a claim that the entire final test dependency tree typechecks.

No full graph discovery, dependency installation, native compilation, or Git operation was performed. Generated outputs remain under `🗑️generated`: `dashboard-expansion-red.log`, `dashboard-expansion-green.log`, `dashboard-expansion-strict.log`, `dashboard-expansion-owner-strict.log`, `dashboard-expansion-strict-final.log`, and `dashboard-expansion-helper-strict.log`, for final cleanup.

## Current Public Surface Execution Gate

Reran exactly the four dashboard launch tests and ten library build-surface tests against the current shared sources after the latest peer editor/project controls: **14 passed, 0 failed, 27055 assertions** across the two explicit test files, in 719 ms.

The actual current declarations admitted **496 tools and 992 launch configurations**, with BuildRouting enabled, using their owner schemas. The four launch tests include neutral command/capability normalization, nested payloads, backend-choice preservation, peer controls, locales, raw arguments and idempotence. The ten surface tests include editor argument boundaries, neutral project/native declarations, infrastructure controls in both languages, root/CI contexts and Bazel filename authority.

Executed direct explicit-path `bun test` with `SEMIO_TEST_ARTIFACT_DIR` set to the ticket generated root. The passing native-host contract subtest used its fixture oracle; this gate requested no native compilation or graph discovery.

No normalization or source/data edit was needed. The existing seed/current projection and peer controls were left as tested. Only this report was updated; the exact owned source inventory remains 48 files. The actual combined output is preserved at `🗑️generated/dashboard-current-surface-gate.log` for final cleanup.

## Registered Semantic Source Analysis

Implemented the agreed observation-owner API while core's diagnostic was stopped. This agent edited only the existing observation helper, its owned schema, existing portable fixture/law tests and the analysis kind in the graph authority schema. Core owns parser registration/inference integration, graph loading and compiler/import integration tests.

The API is `registerSourceAnalysisV1({id,revision,analyze(path,bytes)})` returning a frozen `{id,revision}` handle, plus `observedSourceAnalysisV1` and `observedSourceAnalysisAsyncV1`. Both invoke a synchronous JSON analyzer; only input I/O differs. Same revision cannot silently replace a different callback. An explicitly newer registration makes old handles/facts invalid. Receipts supply identity and data only, never executable code.

Each analysis fact contains provider/revision, source byte fingerprint hints, semantic result SHA-256, and complete nested observations. Validation first admits the owned recursive fact schema and matching registered producer, then reads current bytes and verifies SHA even when stamp tuples match. Identical bytes skip parsing; changed bytes replay the current registered analyzer and compare its normalized JSON result hash and nested dependencies. Semantic consistency excludes original source byte hints, permits equivalent body changes, and distinguishes different parser providers/modes on the same file. Existing raw consumers retain their separate raw file facts.

Nested probes are inherited on cached hits and on parser failure. A throwing analyzer retains the actual raw source fingerprint for failed-attempt retry. Replayed validation probes are isolated from live capture scopes. Parser results remain JSON images with an LRU budget of **16 MiB including serialized nested facts plus per-entry bookkeeping**; source buffers/ASTs are not retained in that memo, and oversized results are not cached. Returned values are independent JSON clones. Existing content-addressed inference records still provide their scoped memo layer.

`sourceObservationStatsV1()` exposes counts only for module-load facts, project images/observation rows, original publications, current capture rows, record stores and analysis memo entries/bytes. Capture Map keys are cleared in finally after success/failure snapshot arrays are materialized; immutable loaded provenance and returned snapshots remain intact. The module-load lifecycle was not changed.

TDD included failing vectors for missing API, direct replay memoization, malformed semantic source hints, throwing-parser source provenance and same-stamp changed bytes. The final focused semantic selection passed **8 tests / 71 assertions**. The full current observation law file passed **43 tests / 183 assertions, 0 failures**, in 95.62 seconds. This includes all 35 existing laws present when this task began plus eight semantic laws. Independent TypeScript preProcessFile, Ajv owner/authority schema checks, RFC6902 malformed-fact patches and system SHA hashing support the fixture results. Core separately reported actual Rust/compiler/import integration; those counts are not included in this agent's test totals.

Targeted strict TypeScript checking of the existing observation test and imported API passed with exit **0**, using repo-owner allowJs=true, checkJs=false and strict=true. The test's implicit-any callback parameters were annotated; the reported extra EOF blank line was removed and a single final newline verified.

No full graph discovery or native compilation was launched by this agent. Source is stable and core/root received the final handoff. Existing owned inventory remains **48 files**; all semantic-task source/schema/fixture paths were already listed. Final evidence logs under `🗑️generated` include `dashboard-semantic-final-focused.log`, `dashboard-semantic-final-full.log`, `dashboard-semantic-strict-final.log` and the corresponding staged TDD red/intermediate logs, for final ticket cleanup.





## Direct Neutral Inference Consumers And Provider Memo Qualification

Updated **38 consumer files** to import common graph/source APIs from `📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs`. This includes production caching, test inference, source watch and their portable fixture wrappers. Actual `projectBuildGraphForNxV1`/`projectBuildTargetForNxV1` imports and the artifact test's explicit Nx plugin registration remain at the backend entry. The native dependency traversal oracle still reads its pinned historical source path; its current implementation comes from the neutral owner. Catalog/execution imports already changed by core were not edited or claimed here. The complete owned inventory now contains **86 paths** in `📥dashboard-files.json`.

The parent project-observation memo now keys each row with the existing provider-aware `observationIdentityV1`. A language-neutral fixture specifies two projects and two expected physical validations. The new actual inference regression failed first with four reads, then passed with two after the one-line correction. Independent TypeScript import preprocessing verifies both provider results. This keeps command/module analyses of the same physical file independently memoized under one parent.

Executed evidence:

- New parent-provider regression: **1 pass / 2 assertions** after documented red **4 reads versus expected 2**.
- Full observation law file: **44 pass / 185 assertions**.
- Exact strict check of the observation law/API plus precise-command and native-closure tests: **exit 0**.
- Test inference discovery/bootstrap/source-mutation fixture suite: **6 pass / 82 assertions**, including actual resident Node and Bun paths.
- Native dependency traversal fixture versus pinned original traversal: **1 pass / 24 assertions**; this parses Cargo declarations and does not compile Rust. Its AST sandbox now includes the renamed `buildPath` helper.
- Actual precise-command fixture versus esbuild: passed. Updated the test to independently compare source strings and the now-explicit `externalDependencies: []` declaration, and to expect package/lock authority in fallback inputs.
- TypeScript parser accepted all **38 changed consumer files**; all **9 static neutral import declarations** resolved. Final exact-entry scan found only the two intended backend projection consumers.

The first live source-watch qualification failed at the source-coordinate guard because the isolated test workspace placed the actual `LIBRARY_ROOT` outside its authority. The fixture now invokes the actual owner using its real workspace and an in-workspace fixture crate, compares full Bun/Node plans, then projects only fixture-owned coordinates for the bounded live watcher. No tracked sources are copied into the ticket. This exposed a second regression: watch visited compact hash inputs and skipped `SEMIO_SOURCE_CLOSURE_*` tokens, losing exact Cargo declarations and original source exclusions. Root explicitly authorized the narrow production correction coordinated with core: `projectInputs` now selects a source representation for the watcher; `inferProjectInputs` retains deduplicated uncompressed groups for that representation while graph hash consumers retain their existing compact representation. Original workspace containment checks remain unchanged.

Final live qualification **passed**: full actual Bun/Node plan equality, Cargo metadata source oracle, all **6 required owner/dependency paths**, **7 independent minimatch vectors**, explicit fixture `dist`/`target` source exclusions, live modification/delete/restore delivery, identical/unrelated write suppression, and cancellation acknowledgement. The updated language-neutral watch fixture names the two required exclusion globs. The exact strict check including all **4 focused test files and their imported API** passed again with exit **0**. Generated evidence is `dashboard-direct-import-watch-green.log`, `dashboard-direct-import-watch-final.log` and `dashboard-direct-import-strict.log`; first-failure diagnostic logs are retained separately. The final source handoff was sent to core before further graph qualification.

No full graph discovery or native compilation was launched. All outputs remain under `🗑️generated` for root's final cleanup: `dashboard-provider-parent-{red,green,full,strict}.log`, `dashboard-direct-import-{discovery,watch,native-closure,strict,parse}.log` and the generated changed-file list. Source changes are stable; core/root were notified before final graph qualification.

The final owned inventory after the authorized watch projection correction contains **89 paths**, including the two narrowly shared production modules and the watch JSON fixture. All 38 mechanically changed consumer absolute paths were checked against the 256 UTF-16 path limit. No source edits remain pending from this agent.

## Bounded Native Script Admission

Replaced native-host ownership's discarded script text read with an explicit required `NativeHostSourceViewV1.admitScript(path)` port. Its closed schema result is `{withinLimit:boolean}`; `nativeHostScriptAdmissionV1(decodedText)` computes exactly the existing decoded/re-encoded UTF-8 **≤1 MiB** predicate. Only that predicate is semantically consumed, so within-bound body or length changes do not invalidate ownership. Core binds the actual inference port to its registered semantic provider; that inference integration remains core-owned. The native filesystem source view preserves fatal UTF-8 decoding, and the generic catalog textual view is explicitly adapted using genuine source text. No fabricated text is returned.

Every original no-follow ancestor/file-kind probe remains before admission. Cargo and project declaration reads/parsing remain exact, and router reads were untouched. The declared executable input list still records the script path through the new port, so execution hashing retains the actual script input. The existing native-host test target now awaits the async law; its Bun surface caller already returns that promise.

Schema and language-neutral fixture preceded the implementation; the initial run failed on the missing admission export. The final existing native-host owner surface test passed **1 test / 9 filtered**, with runtime debug evidence for: four independent `Buffer.byteLength` ASCII/multibyte boundary vectors; Ajv acceptance/refusal of result shapes; unchanged facts after different within-bound bodies while `readText(script)` is forbidden; actual filesystem rename/absence and invalid UTF-8; source kind/ancestor refusal and injected reader errors; semantic receipt acceptance after body rewrite and rejection after oversize/removal. Existing JSON/TOML/Nx output oracles and declared-input closure laws also ran. No exact-size hint is persisted as semantic meaning.

The exact strict TypeScript check of the host API and its law passed **exit 0** after annotating the test's mode/kind literal unions. No full graph, native compilation or Git mutation was performed. All **9** scoped files satisfy the 256 UTF-16 absolute path bound. Complete owned inventory now lists **98 paths**. Evidence remains under `🗑️generated/dashboard-script-admission-{red,green,runtime,owner,strict}.log` and the generated scoped file list for final cleanup. Source is stable and core/root received the ownership handoff.
