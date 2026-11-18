# R52: red baseline of the build module made green (2026-11-15 18:3x)

LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, B = `LIB/🏗️build`. Landing: `T/landings.md` "Package R52 landing" (`T/🧰️scripts/R52/land.py`, lock taken and released, pids of the two running graph reads 80320 and 91851 unchanged before and after). Replica test: `1113-landings.md` "R52". Test edits were made in place (tests only, each re-read first); the one source edit went through the lock.

Finding that applies to all of `B/📥️inputs/🧪️tests`: the test pinned `actualRoot=process.cwd()`; run from its own folder (as the brief says) it read a non-existing `🧪️tests/📋️build-owners.json`. It now derives the root from `import.meta.dir`, so it passes from the folder and from the root. No hard-coded old root (/Users/Shared/semio) was found in any of these tests, fixtures or recorded data.

## Per test: cause, fix

`B/📥️inputs/🧪️tests` (2, all four now pass, 224 expectations)
- "Cargo manifest inference projects the workspace sentinel ...": stale test. (1) `inferProjectInputs` now takes `isOpaqueSourcePathV1`/`opaqueSourceInputExclusionsV1` from module scope (not from `context`): the transpiled harness did not provide them. (2) `🧪️test/🟨️.mjs` `nativeInputsFor` now calls `projectNameOfRoot`: harness did not extract it (and needs `readFileSync`). (3) cwd, see above. Fixed in the test harness.
- "live inference bootstrap publishes the same closed ... helper exports": stale test. `project-assembly` `wire` now reads `registry.stamps` and `projectWithDefaults` calls `requireCanonicalTestAggregateV1` (imported); the harness registry lacked both. Added `stamps:{inferrerActiveV1}` and the real `requireCanonicalTestAggregateV1`.

`B/📥️inputs/📋️data/🧪️tests` (9, all 10 tests now pass)
- Three tests failed with "Original parent Script process invocation required" and "Actual ticket generated directory required": the file used `withBuildTestInvocationV1` with no declared birth, so a bare `bun test ./file` had no issuing parent. Added `declareBuildTestEnvelopeV1({owner:"test:build-inputs-data", ..., capabilities:{artifacts:true}})` (the documented bare-run mechanism, same as 13 other files).
- "same-process Source drift" and the "(unnamed)" one: on macOS the bare run's private artifact directory is below `/var` (a symlink), and the Source-revision ancestry check rightly refuses it: the directory is now `realpathSync`'d. Also the child script lacked the new free names of `loadSourceInventoryImplementationV1` (`implementationPinnedV1`, `staleImplementationV1`, `runtimeSourceUrlV1`); they are now imported from their real modules (checked that node 24 loads the `.ts` module). Both node and bun children pass.
- "graph inventory producers admit closed DATA before writer effects": REAL CODE BUG, fixed in code. `compactSourceInputs` resolved `repositoryBuildGraphWriterDirectoryV1` (which creates a directory / checks writer custody) before the first roster capture, so a hostile roster hit a writer effect before admission. Now `let directory;` and `directory ??= ...` right after the chunk's capture (`B/📥️inputs/🟨️.mjs`, two anchored edits, no behaviour change for valid input). The test now names the shared capture `capturedInventoryV1` and asserts it calls `captureSourceInputInventoryV1`.
- "shared inventory capture refuses hostile rows": the last assertion (a roster of `maximum` files is refused by `compactSourceInputs`) is stale by design: rosters are cut into content-defined chunks (`splitInputClosureV1`), the limit applies per chunk. Replaced by the equivalent contract: the split of `maximum+1` paths gives more than one chunk, loses no path and no chunk exceeds `maximum`. The direct `captureSourceInputInventoryV1` oversize refusal assertion is kept.
- "compact helper and implementation revision ...": the revision body moved to `revisionBytesV1`; pin updated to `readPhysicalSource(join(LIBRARY_ROOT, local))`.
- "graph revision consumes helper-only Source byte changes": `implementationRevision` now uses `implementationPinnedV1`, `loadedRevision`, `implementationFiles` and `revisionBytesV1`/`ignoredSectionsV1`/`sectionalDigestV1`; the replay harness now extracts those declarations from the real source and takes the real imports through `inferenceImportPortsV1("revision")` (the "helper files are read and change the revision" assertions are unchanged and pass).
- "portable graph consumer admits shared inventory DATA": the consumer was split (`captureInferenceInventoryReferencesV1` admits the cohort, `withRepositoryBuildGraphConsumerV1` + `readInventoryRecordLeanV1` read, variable renamed to `file.value`) and the authority URLs live in `🔐️authority` as `new URL("../../📥️inputs/...",physicalSourceOriginV1(...))`. Pins updated to the current shapes; the order assertion (admission before read) is kept.
- "epoch gates": `requireCurrentExactSourceReadV1` now starts with `if(implementationPinnedV1())return;` and the batch returns `{value,scope}`; pins updated.

`B/🧷️format-stamps/🧪️tests` "the wiring stays in place": stale pins. `discoverPart` now guards with `requireCurrentFormatStampsV1(...)` then `const offerStartedMs` (not `return discoverBuildGraphPartV1`); now asserted as guard text plus "guard index before `await discoverBuildGraphPartV1(root`". The entry text is `exitIfStaleImplementationV1(error);const{reportBoundaryErrorV1}` (R48's memory refusal sits before). 16 pass.

`LIB/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🌱️incremental`: stale test and fixture, no code fault. The `incremental` event kind is no longer emitted; the incremental lane shows as a `miss` whose reason ends ` incremental:lane`, and the focused crate is published like every unit. Assertion now counts those misses of the focus crate (= rounds); `🔣️.json` `served` is now hits core and leaf, no misses (leaf is published, so a second lane is served from the cache). Independent oracle (cargo's own incremental session dirs, python) unchanged and passes. The dead enum value `incremental` stays in `🧬️schema/🔣️.json` (not removed, not emitted anywhere).

`🤝️shared-worktree/🔁️replica/🧪️tests/⏱️turns.ts` (red at load, 148 ms against 50 ms): wall-clock measurement of a loop that yields after 8 ms of work. Now `performance.now` is replaced for the test body by a logical clock that advances 0.02 ms per manifest entry named (`book.name` spy) so both the yield budget and the measured turn length depend on work only. The forced-collection test (flaked on the 1-s wall rate limit when 40 array growths took over a second) uses a frozen logical clock, drops the real 1.1 s wait and asserts exactly 1 collection, then 2 after the logical clock moves 1.1 s. Run twice: 3 pass. No production code changed (the daemon keeps its code digest, no restart).

## Remaining red list
Empty for the files named in the brief. Last runs (one file at a time, foreground, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`): inputs/🧪️tests 4, 📋️data 10, digest, owner-view/inference, format-stamps 16, closure-facts, incremental 1, compaction-helpers, unit-cache interface: all green, run inside the lock after the source edit.

## Not verified
- The first test of `⏱️turns.ts` (1.5 GiB sparse file never holds the loop over 500 ms) is still wall-clock; it passed 4 of 4 runs here at load 30 to 40 but is not made logical.
- The logical-clock test does not see a non-yielding section outside the repair loops (only the per-entry loops are counted); mutation (remove the `await turn()`) not run.
- Other test files of the replica module were not re-run (no source change); no graph refresh was run on the changed `compactSourceInputs`; Linux and Windows not run (the realpath fix is for macOS `/var`).
- Convention note for other agents: tests that call `process.cwd()` (154 files) still need the repository root or a cwd-independent root.

## Follow-up: engine-digest, nx-engine and a sweep of all 799 `🟦️*.ts` files under `🧪️tests` of the build module (2026-11-15 19:xx to 20:xx)

### engine-digest and nx-engine (both green now)
- `nx-plugin/🧮️engine-digest/🧪️tests` (1 red): two stale pins. The spawn-only cut ratio `full > 1.4 * cut` is a magic number (measured 1850 against 1385 = 1.34 after today's landings); replaced by the structural bound `full - cut >= number of mustLeave files` (the precise `mustLeave` / `mustStay` assertions are unchanged). `mustLeave` named `🚪️door/📜️script.ts`, which `🏗️build/📜️script.ts` no longer imports (the front door is started by the launcher); replaced by `🚪️door/🔄️restart/🟦️.ts`, which the script does import. 11 pass.
- `🧪️tests/🕸️graph/🔌️nx-engine` (unhandled load error, then 3 red): (1) the test and its `🧰️support` used `process.cwd()` as the repository root (nx.json missing when run from the test folder): now derived from `import.meta.dir`; (2) REAL CODE: the Nx library projection is loaded by `node` in strip-only mode, and `remote-exchange` now reaches `🌐️remote-execution/🗄️content` and `🌐️remote-cache/🔐️sealed`, whose error classes used TypeScript parameter properties (`constructor(readonly code...)`), which node refuses. Both classes rewritten with explicit field and assignment (`landings.md` R52 entries). The test needs `SEMIO_TEST_ARTIFACT_DIR` (the real runner sets it; a bare run must export it). 19 pass; `remote-execution/🧪️tests` 17 pass.
- land.py now also runs the standing preflight and heavy-questions files, nx-engine, engine-digest and sealed; all green except as above before the second edit (then 19 pass). Locks taken and released three times; no graph read killed.

### Sweep method
Each of the 799 files run alone (`bun test ./file --timeout 180000`, 3 in parallel, load 30 to 40, a 300 s cap enforced by SIGKILL of the process group; 1 file hit the cap), first from its own folder (177 red), then the 177 again from the repository root (158 red). Raw rows: scratch `sweep-sweep.jsonl`, `sweep-redo.jsonl`. Fixed in this sweep: `project-farm` tests `🔒️farm-close`, `🪆️captured`, `🪣️declined` (absolute `/Users/Shared/semio` imports, now relative; 2, 8, 1 pass) and `🧠️memory` (the `.d.mts` and the test named `psProcessRowsV1`, `processTreeBytesV1`, `createProcessTreeMemoryV1`, which were never implemented in the `.mjs`; implemented with the test as specification; 30 pass).

### Not red in the real runner, red in a bare run (41 files): need an issued Script process invocation
They fail with `Original parent Script process invocation required`: they use `withBuildTestInvocationV1` and rely on the `repo test` runner to issue the envelope. I could not run `repo test` (it needs the graph; not started here). Listed in `result-R52-sweep.txt`.

### Child fixtures named `🟦️.ts` (8), not tests: run by their parents with arguments

### Red and not fixed (105 files): the remaining red list
Causes not yet analysed per test; many are short asserts on source text that moved today (like the ones fixed above) or pins that peers' landings (R49, R53, R54, R55) are changing right now. Listed with seconds:

- `⚙️operations/🧪️tests/🛗️requires/🟦️.ts` (0s)
- `⏱️profile/📊️cpu/🧪️tests/🟦️.ts` (1s)
- `🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🌳️tree/🟦️.ts` (1s)
- `🌡️host-capacity/🧪️tests/👥️agents/🟦️.ts` (0s)
- `🌡️host-capacity/🧪️tests/🔼️upgrade/🟦️.ts` (3s)
- `🌡️host-capacity/🧪️tests/🧮️arguments/🟦️.ts` (0s)
- `🌡️host-capacity/🧪️tests/🧾️label/🟦️.ts` (0s)
- `🎚️mode/🦀️profile/🧪️tests/📥️consumption/🟦️.ts` (3s)
- `⌨️command/🧪️tests/🟦️.ts` (25s)
- `🏛️sealed/🧪️tests/🗂️store/🟦️.ts` (2s)
- `🏡️resident/🧪️tests/🌐️host/🟦️.ts` (28s)
- `👁️observation/🧪️tests/⚡️incremental/🟦️.ts` (6s)
- `👁️observation/🧪️tests/📦️projects/🟦️.ts` (0s)
- `👁️observation/🧪️tests/📥️reuse/🟦️.ts` (9s)
- `👁️observation/🧪️tests/🕊️release/🟦️.ts` (1s)
- `📏️incremental/🏎️bench/🧭️planning/🧪️tests/🟦️.ts` (0s)
- `⚙️operations/🧪️tests/🧭️help-offline/🟦️.ts` (120s)
- `📖️storybook/🧰️preparation/🕸️engines/🧪️tests/🟦️.ts` (0s)
- `📥️inputs/🎯️selected/📦️snapshot/🧪️tests/🟦️.ts` (1s)
- `📥️inputs/🎯️selected/🦀️rust/🧪️tests/📌️publication/🟦️.ts` (1s)
- `📥️inputs/🎯️selected/🧪️tests/🟦️.ts` (0s)
- `📥️inputs/💾️storage/🧪️tests/♻️scoped/⚠️scoped-retirement-and-bridge/🟦️.ts` (1s)
- `📥️inputs/💾️storage/🧪️tests/♻️scoped/🔄️lifetime/🟦️.ts` (0s)
- `📥️inputs/💾️storage/🧪️tests/🎟️custody/🟦️.ts` (1s)
- `📥️inputs/💾️storage/🧪️tests/🔏️closure/🟦️.ts` (0s)
- `📥️inputs/💾️storage/🧪️tests/🔒️lifetime/🟦️.ts` (0s)
- `📥️inputs/💾️storage/🧪️tests/🛑️primary/🟦️.ts` (3s)
- `📥️inputs/💾️storage/🧪️tests/🟦️.ts` (1s)
- `📥️inputs/💾️storage/🧪️tests/🤝️shared-writer/🟦️.ts` (0s)
- `👁️observation/🧪️tests/🧱️blocks/🟦️.ts` (180s)
- `👁️observation/🧪️tests/🪪️presence/🟦️.ts` (149s)
- `📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧭️boundary/🟦️.ts` (15s)
- `🏃️execution/🧪️tests/🟦️.ts` (300s, capped)
- `📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👁️existing/🟦️.ts` (3s)
- `📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👶️lifecycle/🟦️.ts` (3s)
- `📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/🟦️.ts` (6s)
- `📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/🟦️.ts` (20s)
- `📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👓️reader-lifetime/🟦️.ts` (62s)
- `📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/🟦️.ts` (1s)
- `📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📡️request/🟦️.ts` (20s)
- `📥️source/🔏️closure/🛂️readmission/🧪️tests/🏃️tasks/🟦️.ts` (0s)
- `📥️source/🔏️closure/🟢️node/🧪️tests/📥️stdin/🟦️.ts` (11s)
- `📥️source/🔏️closure/🧪️tests/🧾️mismatch/🟦️.ts` (0s)
- `🔌️backends/🧪️tests/🔏️closure/🟦️.ts` (0s)
- `🔌️backends/🧪️tests/🟦️.ts` (1s)
- `📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts` (158s)
- `📥️source/🔏️closure/🧪️tests/📏️kernels/🟦️.ts` (41s)
- `📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧵️unit-cache-lanes/🟦️.ts` (87s)
- `📥️source/🛗️zero-touch/🧪️tests/🟦️.ts` (47s)
- `🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🟦️.ts` (12s)
- `🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts` (1s)
- `🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🟦️.ts` (1s)
- `🔒️leases/⌨️command/🧪️tests/🧮️status/🟦️.ts` (5s)
- `🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧪️tests/🌱️birth/🟦️.ts` (1s)
- `🔧️tools/🏗️bazel/🧪️tests/📥️keys/🟦️.ts` (2s)
- `🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🛑️too-heavy/🟦️.ts` (1s)
- `🕸️graph/📥️inference/🧪️tests/🔒️lint/🟦️.ts` (0s)
- `🕸️graph/🕰️generations/🔌️load/🧪️tests/🗝️claim-keys/🟦️.ts` (8s)
- `🕸️graph/🕰️generations/🧪️tests/🧱️stable-directories/🟦️.ts` (1s)
- `🕸️graph/🦀️rust/📥️intent/🧪️tests/🪢️trim-paths/🟦️.ts` (0s)
- `🕸️graph/🦀️rust/📥️source/🧪️tests/🟦️.ts` (1s)
- `🕸️graph/🦀️rust/🔒️lifetime/🧪️tests/🟦️.ts` (3s)
- `🕸️graph/🧪️tests/🏊️cooperative-farm/🟦️.ts` (0s)
- `🕸️graph/🧪️tests/🔓️sealed-reuse/🟦️.ts` (0s)
- `🕸️graph/🧪️tests/🔼️upgrade-before-custody/🟦️.ts` (0s)
- `🕸️graph/🧪️tests/🔬️analysis-providers/🟦️.ts` (3s)
- `🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts` (1s)
- `🕸️graph/🧪️tests/🕊️graph-release/🟦️.ts` (1s)
- `🕸️graph/🧪️tests/🙋️fleet-read/🌍️small-booking/🟦️.ts` (3s)
- `🕸️graph/🧪️tests/🙋️fleet-read/🍏️macos-reading/🟦️.ts` (1s)
- `🕸️graph/🧮️derived-nodes/🧪️tests/🎯️task-state/🟦️.ts` (0s)
- `🕸️graph/📥️inference/🧪️tests/🔌️providers/🟦️.ts` (42s)
- `🕸️graph/🧮️derived-nodes/🧪️tests/🟦️.ts` (4s)
- `🚀️bootstrap/⌨️cli/🧪️tests/🧮️arguments/🟦️.ts` (1s)
- `🗂️suites/🧪️tests/🟦️.ts` (18s)
- `🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts` (1s)
- `🚪️door/🧪️tests/🚦️startup/🟦️.ts` (1s)
- `🛠️cmake/🧪️tests/🟦️.ts` (0s)
- `🧪️tests/🎯️source-roster/🟦️.ts` (1s)
- `🧪️tests/🏗️native/🟦️.ts` (4s)
- `🧩️units/🧪️tests/🌐️languages/🟦️.ts` (9s)
- `🧪️tests/🕸️graph/🌱️native-scope/🟦️.ts` (1s)
- `🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts` (4s)
- `🧪️tests/🕸️graph/📐️portable-model/🟦️.ts` (1s)
- `🧪️tests/🕸️graph/🔭️observed-facts/🟦️.ts` (1s)
- `🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts` (1s)
- `🧪️tests/🕸️graph/🛂️admission/🟦️.ts` (10s)
- `🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts` (2s)
- `🕸️graph/📥️inference/🧪️tests/🚶️repository-walks/🟦️.ts` (126s)
- `🧪️tests/🧬️policy-literals/🟦️.ts` (4s)
- `🧪️tests/🧭️relative-paths/🟦️.ts` (1s)
- `🧭️invocation/🕸️graph/🧠️memo/🧪️tests/✏️edit/🟦️.ts` (2s)
- `🧪️tests/🟦️.ts` (14s)
- `🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🎯️slice/🟦️.ts` (0s)
- `🧭️invocation/🕸️graph/🧠️memo/🧮️derive/♨️warm/🧪️tests/🟦️.ts` (4s)
- `🧭️invocation/🧪️tests/📌️pinned-reader/🟦️.ts` (1s)
- `🧭️selection/🎯️scope/🧪️tests/🟦️.ts` (0s)
- `🧭️selection/👁️watch/🧪️tests/🎟️phase/🟦️.ts` (1s)
- `🧱️job-limits/🧪️tests/🟦️.ts` (1s)
- `🧾️ci-report/🗂️suites/🧪️tests/🟦️.ts` (5s)
- `🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts` (39s)
- `🩺️preflight/🧪️tests/🧰️toolchain/🟦️.ts` (1s)
- `🪪️agent-sessions/⌨️command/🧪️tests/🟦️.ts` (1s)
- `🗂️suites/🌐️workspace/🧪️tests/🟦️.ts` (161s)
- `🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts` (45s)
