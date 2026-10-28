# table-route2-g: the 83 red build-library test files after route2-g

Columns: route-g class, file, verdict of the last graph-free census on the unchanged tree (c4, outside-checkout artifact root, 11:2x; c3 for files the route landed), route2-g disposition (LANDED = already in the tree, STAGED = in `C/route2-g-tools/staged*/` and verified by `stage-run.mjs`, STAGED-UNVERIFIED = needs the real tree, PARTIAL, OPEN), note.

| class | file | tree verdict | disposition | note |
|---|---|---|---|---|
| DEFECT | `🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🟦️.ts` | rc=0 pass=10 fail=0 | OPEN |  |
| DEFECT | `📖️texts/🧪️tests/🟦️.ts` | rc=0 pass=3 fail=0 | LANDED 10:23 | law scans caching engine-digest owner |
| DEFECT | `🚪️door/🧪️tests/🚦️startup/🟦️.ts` | rc=1 pass=4 fail=1 | OPEN |  |
| DEFECT | `🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts` | rc=1 pass=46 fail=1 | OPEN |  |
| DEFECT | `🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🟦️.ts` | rc=1 pass=17 fail=2 | OPEN |  |
| DEFECT | `📥️source/🛗️zero-touch/🧪️tests/🟦️.ts` | rc=1 pass=34 fail=2 | OPEN |  |
| DEFECT | `🛠️cmake/🧪️tests/🟦️.ts` | rc=0 pass=7 fail=0 | LANDED 10:21 | real defect: root CMakeLists.txt restored from HEAD^ (digest equals schema pin) |
| GATE | `📥️caller/🧪️tests/📥️birth/🟦️.ts` | rc=1 pass=9 fail=6 | OPEN |  |
| GATE | `⚙️operations/🧪️tests/🟦️.ts` | rc=1 pass=14 fail=1 | OPEN |  |
| GATE | `🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts` | rc=1 pass=3 fail=7 | OPEN |  |
| GATE | `🚀️bootstrap/⌨️cli/🧪️tests/🧮️arguments/🟦️.ts` | rc=1 pass=1 fail=1 | OPEN |  |
| GATE | `🏃️execution/🧪️tests/🟦️.ts` | rc=WALL  | OPEN |  |
| GATE | `📥️inputs/🧮️digest/🧪️tests/🟦️.ts` | rc=1 pass=4 fail=1 | STAGED | (reclassified from GATE) two tests each birthed their own Original in one process: both use the shared build-test Original |
| GATE | `📥️inputs/🎯️selected/📦️snapshot/🧪️tests/🟦️.ts` | rc=1 pass=1 fail=1 | OPEN |  |
| GATE | `📥️inputs/🎯️selected/🦀️rust/🧪️tests/📌️publication/🟦️.ts` | rc=1 pass=0 fail=1 | OPEN |  |
| GATE | `📥️inputs/🎯️selected/🧪️tests/🟦️.ts` | rc=1 pass=0 fail=1 | OPEN |  |
| GATE | `👁️observation/🧪️tests/⚡️incremental/🟦️.ts` | rc=1 pass=2 fail=1 | OPEN |  |
| GATE | `👁️observation/🧪️tests/📥️reuse/🟦️.ts` | rc=1 pass=10 fail=2 | OPEN |  |
| GATE | `👁️observation/🧪️tests/📦️projects/🟦️.ts` | rc=1 pass=0 fail=2 | OPEN |  |
| GATE | `👁️observation/🧪️tests/🟦️.ts` | rc=1 pass=41 fail=11 | OPEN |  |
| GATE | `📊️reporting/🧪️tests/🟦️.ts` | rc=1 pass=4 fail=2 | STAGED | (reclassified from GATE) callback failure closes the shared Original: isolated declared-rejection test (registerIsolatedBuildTestV1) |
| GATE | `🧭️selection/📥️closure/🧪️tests/🟦️.ts` | rc=1 pass=0 fail=3 | OPEN |  |
| GATE | `🧪️tests/🏗️native/🟦️.ts` | rc=1 pass=4 fail=1 | OPEN |  |
| INFLIGHT | `📥️inputs/🧮️digest/🧪️tests/🔒️batch-snapshot/🟦️.ts` | rc=75  | OPEN |  |
| INFLIGHT | `🏡️resident/🧪️tests/🏛️service/🟦️.ts` | rc=1 pass=5 skip=2 fail=29 | PARTIAL-STAGED | service environment hermetic (the bound SEMIO_BUILD_* keys of a build run made every client refuse); 29 red -> 3 red (dirty/journal queries; watcher backend) |
| INFLIGHT | `🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts` | rc=1 pass=2 fail=3 | STAGED | child env clears LANG/LC_*: bilingual output expected when no language is chosen (a LANG=en shell made 3 laws red) |
| INFLIGHT | `🎚️mode/🦀️profile/🧪️tests/📥️consumption/🟦️.ts` | rc=1 pass=21 fail=3 | STAGED | re-baselined Cargo profile corpus (framework 51 wasm, root 52 wasm = 81 rows), selection owner is the Once function |
| INFLIGHT | `🧭️selection/👁️watch/🧪️tests/🟦️.ts` | rc=1 pass=2 fail=2 | OPEN |  |
| INFLIGHT | `🧪️tests/🕸️graph/🌱️native-scope/🟦️.ts` | rc=1 pass=2 fail=2 | OPEN |  |
| LOAD | `⚙️operations/🧪️tests/🧭️help-offline/🟦️.ts` | rc=0 pass=1 fail=0 | OPEN |  |
| LOAD | `🌐️remote-execution/📡️wire/🧪️tests/🟦️.ts` | rc=1 pass=6 fail=2 | OPEN |  |
| STALE | `🔧️tools/🏗️bazel/🧪️tests/📋️contract/🟦️.ts` | rc=0 pass=2 fail=0 | LANDED 10:28 | corpus digests re-baselined (5 definitions), StorageLayout pool |
| STALE | `🔧️tools/🏗️bazel/🧪️tests/📤️arguments/🟦️.ts` | rc=0 pass=3 fail=0 | LANDED 10:28 | runTask now in 🎬️executor, new free names |
| STALE | `🔧️tools/🏗️bazel/🧪️tests/📥️coverage/🟦️.ts` | rc=0 pass=6 fail=0 | LANDED 10:26 | bazelStarlarkRulesV1 + repo_inputs_witness |
| STALE | `🔧️tools/🏗️bazel/🧪️tests/📥️environment/🟦️.ts` | rc=0 pass=1 fail=0 | LANDED 10:29 | import.meta.dirname |
| STALE | `🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧪️tests/🌱️birth/🟦️.ts` | rc=0 pass=6 fail=0 | LANDED 10:29 | import.meta.dirname |
| STALE | `🔧️tools/🏗️bazel/🧪️tests/🔏️implementation/🟦️.ts` | rc=1 pass=0 fail=5 | OPEN |  |
| STALE | `🔧️tools/🏗️bazel/🧪️tests/🟦️.ts` | rc=1 pass=12 skip=10 fail=3 | STAGED | restore leases per output root (held-snapshot), build label expr, free names |
| STALE | `🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts` | rc=1 pass=7 skip=2 fail=1 | STAGED | guard excludes plan-child (BUILD_GRAPH_PLAN_OUT_V1) |
| STALE | `🔧️tools/🏗️bazel/🦀️cargo/🧪️tests/🟦️.ts` | rc=1 pass=5 skip=1 fail=2 | STAGED | corpus commands launcher repo (policy nativeCommand regex) |
| STALE | `⌨️command/🧪️tests/🟦️.ts` | rc=1 pass=2 fail=4 | OPEN |  |
| STALE | `🚀️bootstrap/🧪️tests/🟦️.ts` | rc=1 pass=3 fail=2 | STAGED | import edge may be a public facade that explicitly re-exports the declared module |
| STALE | `🧾️store-receipt/🧪️tests/🟦️.ts` | rc=1 pass=15 fail=1 | STAGED | law follows relative static+dynamic imports one level; quoted script path; stageAdmittedArtifactTreesV1 publisher |
| STALE | `🕸️graph/🦀️rust/📥️source/🧪️tests/🟦️.ts` | rc=1 pass=2 fail=2 | STAGED | test artifact dir instead of ticket dir; observation fixture declarationExports +12 (staged) |
| STALE | `🕸️graph/🦀️rust/🔒️lifetime/🧪️tests/🟦️.ts` | rc=1 pass=2 fail=1 | STAGED | withSelectedRustBuildGraphV1 caller moved to ⌨️command/⚙️runtime |
| STALE | `🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts` | rc=1 pass=16 fail=4 | STAGED | stale-epoch law runs under routing.withoutImplementationPinV1 (invocations are pinned by design) |
| STALE | `🕸️graph/📥️inference/🧪️tests/🎓️course-inputs/🟦️.ts` | rc=RSS  | PARTIAL-STAGED | oracle bundler loads the public .json facades (TS modules) as TS; the law then reports a REAL cache gap: the course test target reads 🖱️ui/🧱️elements/🥞️LayeredOverview/📋️types/🟦️.ts without declaring or inferring it (FOR course owner: declare the ui element so |
| STALE | `📥️inputs/🧊️stable-file/🧪️tests/🟦️.ts` | rc=1 pass=2 fail=1 | STAGED | (supersedes the earlier line) stable-file now re-captures a file edited mid-read (churn read layer): append/touch/truncate cases assert the re-capture, budget reset to the start admission, and its checkpoint counts |
| STALE | `📥️inputs/🧪️tests/📏️windows/🟦️.ts` | rc=1 pass=1 fail=3 | PARTIAL-STAGED | reader law inspects observeOnce behind the retry wrapper; the FD-operations child test still red (error/undefined cases: same/completed false under the retry design) |
| STALE | `📥️inputs/📋️data/🧪️tests/🟦️.ts` | rc=1 pass=3 fail=8 | OPEN |  |
| STALE | `📥️inputs/💾️storage/🧪️tests/♻️scoped/🔄️lifetime/🍎️nx-recovery-after-maintenance/🟦️.ts` | rc=1 pass=0 fail=3 | OPEN |  |
| STALE | `📥️inputs/💾️storage/🧪️tests/♻️scoped/🔄️lifetime/🟦️.ts` | rc=1 pass=0 fail=1 | OPEN |  |
| STALE | `📥️inputs/💾️storage/🧪️tests/♻️scoped/🟦️.ts` | rc=1 pass=0 fail=2 pass=1 fail=1 | OPEN |  |
| STALE | `📥️inputs/💾️storage/🧪️tests/🎛️phase/🟦️.ts` | rc=1 pass=0 fail=8 | STAGED | writer model: new free names (nativeStatfs, writer resource, schema, consumer guard), leases partition open call; artifact model with real record encoder |
| STALE | `📥️inputs/💾️storage/🧪️tests/🎟️custody/🟦️.ts` | rc=1 pass=0 fail=1 | DEFERRED | optional headroom now delegates deletion to retireBuildGraphOptionalGroupsHeadroomV1 (groups tests cover it); test model needs rewrite to the groups port |
| STALE | `📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts` | rc=1 pass=4 fail=4 | PARTIAL-STAGED | image dirs from testScratchRootV1 (nested-image law); still red: line 44 expects 17 got 1 + flaky capture under load |
| STALE | `📥️inputs/💾️storage/🧪️tests/🔏️closure/🟦️.ts` | rc=1 pass=0 fail=1 | STAGED | (+🧫️cases.json worker graph path) revision rows port names (ignoredSections, relative) |
| STALE | `📥️inputs/💾️storage/🧪️tests/🔒️lifetime/🟦️.ts` | rc=1 pass=0 fail=1 | STAGED | cache-gc launcher import stub in extracted writer |
| STALE | `📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts` | rc=1 pass=0 fail=2 | STAGED | consumer guard binding + cache-gc launcher import stub |
| STALE | `📥️inputs/💾️storage/🧪️tests/🛑️primary/🟦️.ts` | rc=1 pass=3 fail=1 | STAGED | maintenance writer moved to 🔧️custody; helper searches both modules |
| STALE | `📥️inputs/💾️storage/🧪️tests/🟦️.ts` | rc=1 pass=13 fail=8 | OPEN |  |
| STALE | `📥️inputs/💾️storage/🧪️tests/🪪️identity/🟦️.ts` | rc=1 pass=2 fail=1 | STAGED | store variables: original + read-only current.store |
| STALE | `🔧️tools/💾️storage/🧪️tests/🟦️.ts` | rc=1 pass=1 fail=3 | STAGED | graph artifact model: real encodeGraphRecordsV1/policy/schema, readdir/ledger ports; call order reads the runtime module (main flow, backend execute) |
| STALE | `🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts` | rc=1 pass=3 fail=1 | OPEN |  |
| STALE | `📥️source/🔏️closure/🏗️compiler/🧪️tests/🟦️.ts` | rc=1 pass=0 fail=2 | OPEN |  |
| STALE | `📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts` | rc=1 pass=0 fail=1 | OPEN |  |
| STALE | `📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts` | rc=1 pass=12 fail=4 | OPEN |  |
| STALE | `📥️source/🔏️closure/🧪️tests/🔏️selection/🟦️.ts` | rc=1 pass=1 fail=2 | OPEN |  |
| STALE | `📥️source/🔏️closure/🧪️tests/🚫️cache-root-refusal/🟦️.ts` | rc=0 pass=1 fail=0 | OPEN |  |
| STALE | `🧪️tests/🕸️graph/🌐️source-discovery/🟦️.ts` | rc=1 pass=4 fail=2 | OPEN |  |
| STALE | `🧪️tests/🕸️graph/🎯️source-demand/📋️scope/🟦️.ts` | rc=1 pass=3 fail=1 | OPEN |  |
| STALE | `🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts` | rc=1 pass=19 fail=11 | OPEN |  |
| STALE | `🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts` | rc=1 pass=5 fail=7 | OPEN |  |
| STALE | `🧪️tests/🕸️graph/📐️portable-model/🟦️.ts` | rc=1 pass=5 fail=2 | PARTIAL-STAGED | Nx projection now carries cache:false (expectation updated); the test also needs the Nx namespace birth (open, see FFI finding) |
| STALE | `🧪️tests/🕸️graph/🔄️candidate/🟦️.ts` | rc=1 pass=0 fail=3 | OPEN |  |
| STALE | `🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts` | rc=1 pass=14 fail=5 | OPEN |  |
| STALE | `🧪️tests/🕸️graph/🛂️admission/🟦️.ts` | rc=1 pass=7 fail=13 | OPEN |  |
| STALE | `🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts` | rc=1 pass=5 skip=1 fail=8 | OPEN |  |
| STALE | `🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts` | rc=1 pass=10 fail=5 | OPEN |  |
| STALE | `🧪️tests/🟦️.ts` | rc=1 pass=4 fail=1 | STAGED | root build is a noop aggregate (law asserts it); value-module retirement-support law uses Ajv2020 (LANDED, not frozen) |
| STALE | `🧪️tests/🎟️phase/🟦️.ts` | rc=1 pass=3 fail=8 | OPEN |  |
| STALE | `🧪️tests/🎟️phase/🦀️framework/🟦️.ts` | rc=1 pass=0 fail=2 | OPEN |  |
| STALE | `🧪️tests/🎯️source-roster/🟦️.ts` | rc=0 pass=3 fail=0 | LANDED 10:21 | fixture dispatch owners follow task-routes + suite tables (json), schema pattern ts|json, runtimeLiterals reads tables |
