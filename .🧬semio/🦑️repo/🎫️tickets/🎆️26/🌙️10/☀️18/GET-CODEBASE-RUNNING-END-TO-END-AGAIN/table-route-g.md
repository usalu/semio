# table-route-g: build library tests, closed route table and census

Route table `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🔣️.json` (51 suites, 353 test files, each routed exactly once; law + Python oracle `🗂️suites/🧪️tests`). Census: official-default capability mode, one file per process, graph-free, 240 s wall, 3 GB RSS ceiling, tools `C/route-g-tools/census-run.ts`. `before` = reached by no official route before this wave (audit-build-infra-tests-1026g). Verdicts: 0 = green, 1 = red, WALL = timeout, 75 = declared restart after a mid-run edit, `-` = created after the census.

**270 green / 83 not green / 0 not run, of 353 files.**

## Suites

| suite | budget | files | green | not run |
|---|---:|---:|---:|---:|
| `build-command-line` | 300 s | 24 | 16 | 0 |
| `build-parsers` | 300 s | 2 | 1 | 0 |
| `build-tool-setup` | 300 s | 8 | 7 | 0 |
| `build-doctor-preflight` | 300 s | 10 | 9 | 0 |
| `build-targets-selection` | 300 s | 14 | 11 | 0 |
| `build-resident` | 300 s | 10 | 9 | 0 |
| `build-workers` | 300 s | 2 | 2 | 0 |
| `build-leases-command` | 300 s | 2 | 2 | 0 |
| `build-host-capacity` | 300 s | 6 | 6 | 0 |
| `build-memory` | 300 s | 1 | 1 | 0 |
| `build-caller-birth` | 300 s | 4 | 3 | 0 |
| `build-observation` | 300 s | 13 | 9 | 0 |
| `build-incremental` | 300 s | 1 | 1 | 0 |
| `build-incremental-bench` | 300 s | 1 | 1 | 0 |
| `build-incremental-concurrent` | 300 s | 1 | 1 | 0 |
| `build-remote-cache` | 300 s | 4 | 4 | 0 |
| `build-remote-execution` | 300 s | 8 | 7 | 0 |
| `build-execution` | 300 s | 4 | 3 | 0 |
| `build-cache-gc` | 300 s | 6 | 6 | 0 |
| `build-reporting` | 300 s | 2 | 1 | 0 |
| `build-storybook` | 15 s | 2 | 2 | 0 |
| `build-inputs-storage` | 300 s | 19 | 7 | 0 |
| `build-inputs-selected` | 300 s | 10 | 6 | 0 |
| `build-inputs-revision` | 300 s | 16 | 12 | 0 |
| `build-source-closure` | 300 s | 14 | 8 | 0 |
| `build-bazel-sessions` | 300 s | 2 | 2 | 0 |
| `build-bazel-execution` | 300 s | 13 | 12 | 0 |
| `build-bazel-evidence` | 300 s | 9 | 9 | 0 |
| `build-bazel-snapshots` | 300 s | 5 | 5 | 0 |
| `build-bazel-host` | 300 s | 8 | 6 | 0 |
| `build-bazel-contracts` | 300 s | 24 | 17 | 0 |
| `build-nx-tool` | 300 s | 4 | 3 | 0 |
| `build-generation-claims` | 300 s | 2 | 2 | 0 |
| `build-generation-pin` | 300 s | 3 | 3 | 0 |
| `build-generation-coverage` | 300 s | 2 | 2 | 0 |
| `build-generation-facts` | 300 s | 2 | 2 | 0 |
| `build-graph-test-owners` | 300 s | 1 | 1 | 0 |
| `build-generation-load` | 300 s | 4 | 4 | 0 |
| `build-generations` | 300 s | 13 | 13 | 0 |
| `build-graph-protocol` | 300 s | 13 | 12 | 0 |
| `build-graph-owners` | 300 s | 3 | 3 | 0 |
| `build-graph-rust` | 300 s | 5 | 2 | 0 |
| `build-inference` | 300 s | 17 | 15 | 0 |
| `build-inference-projects` | 300 s | 4 | 4 | 0 |
| `build-inference-policy` | 300 s | 5 | 5 | 0 |
| `build-inference-script-inputs` | 300 s | 3 | 3 | 0 |
| `build-workspace-graph-laws` | 300 s | 16 | 5 | 0 |
| `build-workspace-phase-laws` | 300 s | 5 | 1 | 0 |
| `build-workspace-laws` | 15 s | 1 | 0 | 0 |
| `build-sealed` | 300 s | 4 | 3 | 0 |
| `build-route-table` | 300 s | 1 | 1 | 0 |

## Red files by cause

| class | files |
|---|---:|
| STALE | 52 |
| GATE | 16 |
| DEFECT | 7 |
| INFLIGHT | 6 |
| LOAD | 2 |

| file | suite | verdict | counts | wall | peak RSS | class | owner | cause / first failing test |
|---|---|---|---|---|---|---|---|---|
| `🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🟦️.ts` | build-bazel-execution | 1 | pass=8 fail=2 | 10s | 354MB | DEFECT | exec-g/keys-g, churn-g, lease-g | keyed executor-command closure misses 🧾️observation/📁️current (opened at runtime); keyed 📥️inputs/🔏️revision/🟨️.mjs imports 🔁️churn/🌊️races and 🧷️format-stamps/🟨️.mjs imports 🔒️leases/📈️standing outside a declared scheduling port — first red: scope executor-command: everything the runtime trace of each of its classes opened is inside |
| `📖️texts/🧪️tests/🟦️.ts` | build-command-line | 1 | pass=2 fail=1 | 0s | 0MB | DEFECT | nxperf-g | ⚡️caching/…/🧮️engine-digest throws refuseBuildV1 keys nx-engine.* that the build text law sees as unused: move to the caching catalog or widen the law — first red: every message is used by the code it belongs to and every key the code names exists [107.44ms |
| `🚪️door/🧪️tests/🚦️startup/🟦️.ts` | build-command-line | 1 | pass=4 fail=1 | 1s | 0MB | DEFECT | noop-g/chain-g | light `help` command loads 16 modules, budget 13 (startup regression) — first red: a light command loads no heavy module group and stays within its module and byte budget; a ne |
| `🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts` | build-graph-protocol | 1 | pass=43 fail=3 | 1s | 0MB | DEFECT | nest-g/churn-g (headroom closures session) | ⚡️caching/…/🧾️closures/📡️session/🟦️.ts:10 has a `Stale loaded` self-check without implementationPinnedV1() — first red: present: the part replaces the virtual groups wholesale [0.52ms] |
| `🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🟦️.ts` | build-inference | 1 | pass=17 fail=2 | 10s | 82MB | DEFECT | exec-g/cutoff-g | law `no closure member is read for its bytes` contradicts routerKeptV1 reading each command script (🧭️script-inputs/🟨️.mjs:281): decide law or code — first red: absent: no closure member is read for its bytes [0.23ms] |
| `📥️source/🛗️zero-touch/🧪️tests/🟦️.ts` | build-source-closure | 1 | pass=34 fail=2 | 41s | 355MB | DEFECT | knot/exec-g | 13 real imports leave a zero-touch closure (build-bootstrap/repo-cli/repo-door seam-static to 🏗️build/🕸️graph/📥️inference/🟨️.mjs and 🏗️builder/📥️inputs/🔌️nx, ajv in root-setup, workspace specifiers in 🏃️process/🟦️.ts) — first red: the real checkout has no import that leaves a zero-touch closure [7100.14ms] |
| `🛠️cmake/🧪️tests/🟦️.ts` | build-tool-setup | 1 | pass=6 fail=1 | 3s | 100MB | DEFECT | cmake owner | root CMakeLists.txt was deleted from the worktree (not tracked since commit …702) while 🧬️schema pins its digest; the owner-script assertion was repaired (moved to 🏃️execution) — first red: neutral CMake intent and authored Source agree with Ajv and SQLite [798.21ms] |
| `📥️caller/🧪️tests/📥️birth/🟦️.ts` | build-caller-birth | 1 | pass=9 fail=6 | 4s | 227MB | GATE | caller owner | child Nx receipt needs the declared selection — first red: closed Nx child data preserves private Original and actual Node receipt with authored ceiling |
| `⚙️operations/🧪️tests/🟦️.ts` | build-command-line | 1 | pass=14 fail=1 | 25s | 240MB | GATE | ops owner | nested real CLI `repo preflight --help` is refused `Undeclared original Repo build selection`: help must not need a granted selection (product: ⌨️command/🩺️preflight reads the selection before printing help) — first red: the real CLI prints the own help of a command, also for preflight, without running it [21406. |
| `🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts` | build-command-line | 1 | pass=3 fail=7 | 11s | 212MB | GATE | bootstrap owner | Bounded physical Source revision required: needs the captured Source birth — first red: actual Root entry dispatches native recovery before generic image admission [40.82ms] |
| `🚀️bootstrap/⌨️cli/🧪️tests/🧮️arguments/🟦️.ts` | build-command-line | 1 | pass=1 fail=1 | 0s | 0MB | GATE | bootstrap owner | Bounded physical Source revision required: needs the captured Source birth — first red: every argument vector selects what the corpus and the Python reading say, with the authored o |
| `🏃️execution/🧪️tests/🟦️.ts` | build-execution | WALL |  | 240s | 311MB | GATE | execution owner | Original prepared execution inventory required (240 s wall) — first red: every actual generated command family admits its authored execution service through the indep |
| `📥️inputs/🧮️digest/🧪️tests/🟦️.ts` | build-inputs-revision | 1 | pass=4 fail=1 | 15s | 88MB | GATE | inputs owners | Original process birth already received — first red: neutral exact Source batch reads a file replaced mid-read again and yields the replacement's |
| `📥️inputs/🎯️selected/📦️snapshot/🧪️tests/🟦️.ts` | build-inputs-selected | 1 | pass=1 fail=1 | 0s | 0MB | GATE | inputs owners | Original immutable native inventory SDK / snapshot fixture; publication harness lacks buildNativeSnapshotProtocolV1 — first red: closed physical selected Source snapshot retains genuine funding and Native reader until sett |
| `📥️inputs/🎯️selected/🦀️rust/🧪️tests/📌️publication/🟦️.ts` | build-inputs-selected | 1 | pass=0 fail=1 | 1s | 0MB | GATE | inputs owners | Original immutable native inventory SDK / snapshot fixture; publication harness lacks buildNativeSnapshotProtocolV1 — first red: selected Rust Native snapshots consume canonical pinned record publications [168.41ms] |
| `📥️inputs/🎯️selected/🧪️tests/🟦️.ts` | build-inputs-selected | 1 | pass=0 fail=1 | 0s | 0MB | GATE | inputs owners | Original immutable native inventory SDK / snapshot fixture; publication harness lacks buildNativeSnapshotProtocolV1 — first red: native selected maps match physical context and refuse incomplete compiler Source references |
| `👁️observation/🧪️tests/⚡️incremental/🟦️.ts` | build-observation | 1 | pass=2 fail=1 | 1s | 0MB | GATE | observation owners | Original artifact namespace and immutable SDK required; reuse/root laws drifted (extracted free names, content-change) — first red: parallel incremental inference settles concurrent edits onto serial full inference and withho |
| `👁️observation/🧪️tests/📥️reuse/🟦️.ts` | build-observation | 1 | pass=10 fail=2 | 10s | 277MB | GATE | observation owners | Original artifact namespace and immutable SDK required; reuse/root laws drifted (extracted free names, content-change) — first red: neutral inference actual graph callers retain compact reference admission [190.15ms] |
| `👁️observation/🧪️tests/📦️projects/🟦️.ts` | build-observation | 1 | pass=0 fail=2 | 0s | 0MB | GATE | observation owners | Original artifact namespace and immutable SDK required; reuse/root laws drifted (extracted free names, content-change) — first red: real Node project admission reuses unchanged owners and rebuilds reverse command imports [40. |
| `👁️observation/🧪️tests/🟦️.ts` | build-observation | 1 | pass=41 fail=11 | 103s | 310MB | GATE | observation owners | Original artifact namespace and immutable SDK required; reuse/root laws drifted (extracted free names, content-change) — first red: non-blocking validation agrees with reuse: content-change [2240.48ms] |
| `📊️reporting/🧪️tests/🟦️.ts` | build-reporting | 1 | pass=4 fail=2 | 7s | 111MB | GATE | reporting owner | Original report reader failure — first red: build report callback failures release original publication custody [2412.43ms] |
| `🧭️selection/📥️closure/🧪️tests/🟦️.ts` | build-targets-selection | 1 | pass=0 fail=3 | 4s | 148MB | GATE | selection owner | Original Nx SDK required — first red: selected prerequisite closure follows indirect owners and mode against actual Nx [3422.76ms] |
| `🧪️tests/🏗️native/🟦️.ts` | build-workspace-phase-laws | 1 | pass=4 fail=1 | 1s | 0MB | GATE | native owner | Live Original resource control issuance required — first red: Cargo synchronization reuses content-bound admission and observes manifest and lock changes [ |
| `📥️inputs/🧮️digest/🧪️tests/🔒️batch-snapshot/🟦️.ts` | build-inputs-revision | 75 |  | 18s | 59MB | INFLIGHT | keys-g | exit 75 (declared restart after an implementation edit mid-run): rerun |
| `🏡️resident/🧪️tests/🏛️service/🟦️.ts` | build-resident | 1 | pass=5 skip=2 fail=29 | 15s | 81MB | INFLIGHT | noop-g | 29 laws: the service answer lacks `served` (protocol under edit) — first red: memo, planning and invalidation > a second run is answered from the memo without executing an |
| `🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts` | build-sealed | 1 | pass=2 fail=3 | 4s | 108MB | INFLIGHT | chain-g/noop-g | new sealed-door module under development: 3 of 5 trampoline laws red when the census ran — first red: the sealed door runs the stored snapshot: a live edit that does not parse is reported and not |
| `🎚️mode/🦀️profile/🧪️tests/📥️consumption/🟦️.ts` | build-targets-selection | 1 | pass=21 fail=3 | 3s | 159MB | INFLIGHT | profile owner | the workspace Cargo.toml profile constraint inventory moves under peers (80 -> 81 rows, wasm 50 -> 51, digest changed twice during the census); the corpus is re-baselined by its owner once the workspace is quiet — first red: neutral consumed Cargo profile policy preserves every actual workspace constraint inventory [ |
| `🧭️selection/👁️watch/🧪️tests/🟦️.ts` | build-targets-selection | 1 | pass=2 fail=2 | 4s | 97MB | INFLIGHT | churn-g | watch child fails without message; rerun after churn-g — first red: actual watch child retains Original bounds, coalesces events and drains cancellation [2238.29 |
| `🧪️tests/🕸️graph/🌱️native-scope/🟦️.ts` | build-workspace-graph-laws | 1 | pass=2 fail=2 | 1s | 0MB | INFLIGHT | infer-g | harness repaired for parserTypescriptV1; now `createErrorCollectorFromEnvV1` is a new free name of the extracted function — first red: actual selected project assembly preserves closed owners and prepares only selected native an |
| `⚙️operations/🧪️tests/🧭️help-offline/🟦️.ts` | build-command-line | 1 | pass=0 fail=1 | 120s | 483MB | LOAD | ops owner | spawns the real CLI per operation x language (>120 s test budget) — first red: every operation of the catalogue prints its own help in every catalogue language without runn |
| `🌐️remote-execution/📡️wire/🧪️tests/🟦️.ts` | build-remote-execution | 1 | pass=6 fail=2 | 122s | 287MB | LOAD | remote-exec owner | gRPC curl/h2 test of 122 s: per-test 120 s timeout under load 25-35; TLS ALPN case gets ERR_HTTP2_STREAM_CANCEL — first red: gRPC: the server decompresses gzip frames, refuses oversized messages and unauthorized calls, |
| `🔧️tools/🏗️bazel/🧪️tests/📋️contract/🟦️.ts` | build-bazel-contracts | 1 | pass=0 fail=3 | 0s | 0MB | STALE | bazel owners | contract corpus / generated BUILD / purpose-birth harness drift (import.meta in extracted source, python parse of BUILD, runtime dependency under the issued cache root) — first red: Bazel component contracts have one canonical owner with unchanged normalized constraints [50. |
| `🔧️tools/🏗️bazel/🧪️tests/📤️arguments/🟦️.ts` | build-bazel-contracts | 1 | pass=0 fail=4 | 1s | 0MB | STALE | bazel owners | contract corpus / generated BUILD / purpose-birth harness drift (import.meta in extracted source, python parse of BUILD, runtime dependency under the issued cache root) — first red: Bazel per-command forwarding executes the same literal argv as Nx and SQLite [111.02ms] |
| `🔧️tools/🏗️bazel/🧪️tests/📥️coverage/🟦️.ts` | build-bazel-contracts | 1 | pass=5 fail=1 | 1s | 0MB | STALE | bazel owners | contract corpus / generated BUILD / purpose-birth harness drift (import.meta in extracted source, python parse of BUILD, runtime dependency under the issued cache root) — first red: generated BUILD declares one repo_test per Bazel test and an independent Python parse agrees |
| `🔧️tools/🏗️bazel/🧪️tests/📥️environment/🟦️.ts` | build-bazel-contracts | 1 | pass=0 fail=2 | 0s | 0MB | STALE | bazel owners | contract corpus / generated BUILD / purpose-birth harness drift (import.meta in extracted source, python parse of BUILD, runtime dependency under the issued cache root) — first red: Bazel permanent dispatcher publishes its canonical workspace before purpose birth [132.86ms] |
| `🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧪️tests/🌱️birth/🟦️.ts` | build-bazel-contracts | 1 | pass=5 fail=1 | 0s | 0MB | STALE | bazel owners | contract corpus / generated BUILD / purpose-birth harness drift (import.meta in extracted source, python parse of BUILD, runtime dependency under the issued cache root) — first red: Bazel permanent phase projects its genuine finite child before independent purpose birth [93. |
| `🔧️tools/🏗️bazel/🧪️tests/🔏️implementation/🟦️.ts` | build-bazel-contracts | 1 | pass=0 fail=5 | 8s | 433MB | STALE | bazel owners | contract corpus / generated BUILD / purpose-birth harness drift (import.meta in extracted source, python parse of BUILD, runtime dependency under the issued cache root) — first red: Bazel implementation identity changes for an imported schema without changing the entry [8100 |
| `🔧️tools/🏗️bazel/🧪️tests/🟦️.ts` | build-bazel-contracts | 1 | pass=12 skip=10 fail=3 | 1s | 0MB | STALE | bazel owners | contract corpus / generated BUILD / purpose-birth harness drift (import.meta in extracted source, python parse of BUILD, runtime dependency under the issued cache root) — first red: Bazel actual snapshot contention retains original finite deadline and genuine progress cancel |
| `🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts` | build-bazel-host | 1 | pass=7 skip=2 fail=1 | 1s | 0MB | STALE | bzsrv-g | tools setup bazel partition expectation — first red: tools setup bazel opens only the tools and dependencies partitions and never installs JavaScr |
| `🔧️tools/🏗️bazel/🦀️cargo/🧪️tests/🟦️.ts` | build-bazel-host | 1 | pass=5 skip=1 fail=2 | 1s | 0MB | STALE | rust-g | quoted cargo command classification — first red: native Cargo commands classify by their authored command and execution kind [0.47ms] |
| `⌨️command/🧪️tests/🟦️.ts` | build-command-line | 1 | pass=2 fail=4 | 5s | 206MB | STALE | command owner | test expects the child envelope policy to equal the original policy; the child is narrowed to the control budget 30000 — first red: actual command ingress executes authenticated captured Source with unchanged cwd and Original |
| `🚀️bootstrap/🧪️tests/🟦️.ts` | build-command-line | 1 | pass=3 fail=2 | 5s | 137MB | STALE | bootstrap owner | publisher import shape of flow-core-package changed — first red: bootstrap publishers remain executable captured Source with closed DATA [4868.38ms] |
| `🧾️store-receipt/🧪️tests/🟦️.ts` | build-doctor-preflight | 1 | pass=15 fail=1 | 2s | 73MB | STALE | course owner | course package build target no longer publishes through the recording publisher — first red: every declared receipt output belongs to its own target and to a handler that publishes throu |
| `🕸️graph/🦀️rust/📥️source/🧪️tests/🟦️.ts` | build-graph-rust | 1 | pass=2 fail=2 | 1s | 0MB | STALE | rust-g | selected Rust graph laws: Actual ticket required, source text of withSelectedRustBuildGraphV1 moved, budget epoch expectations — first red: neutral selected Rust bounded Source ordinary files bind private observations and refuse fail |
| `🕸️graph/🦀️rust/🔒️lifetime/🧪️tests/🟦️.ts` | build-graph-rust | 1 | pass=2 fail=1 | 2s | 141MB | STALE | rust-g | selected Rust graph laws: Actual ticket required, source text of withSelectedRustBuildGraphV1 moved, budget epoch expectations — first red: neutral selected Rust descriptors remain inside their actual compiler store [19.18ms] |
| `🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts` | build-graph-rust | 1 | pass=16 fail=4 | 3s | 329MB | STALE | rust-g | selected Rust graph laws: Actual ticket required, source text of withSelectedRustBuildGraphV1 moved, budget epoch expectations — first red: default indexed selected Rust graph retains loaded source policy and graph budget epochs [62. |
| `🕸️graph/📥️inference/🧪️tests/🎓️course-inputs/🟦️.ts` | build-inference | 1 | pass=7 fail=2 | 45s | 2807MB | STALE | course owner | esbuild bundle of the course inputs fails with 12 errors — first red: test: every file the target reads is declared or inferred [8303.38ms] |
| `📥️inputs/🧊️stable-file/🧪️tests/🟦️.ts` | build-inputs-revision | 1 | pass=2 fail=1 | 2s | 61MB | STALE | inputs owners | windows / stable-file / data producers drifted behind the Source reader changes — first red: every case captures, refuses and spends the budget as authored, with the authored number of c |
| `📥️inputs/🧪️tests/📏️windows/🟦️.ts` | build-inputs-revision | 1 | pass=1 fail=3 | 25s | 141MB | STALE | inputs owners | windows / stable-file / data producers drifted behind the Source reader changes — first red: neutral actual Source reader retains EOF namespace and per-chunk FD verification [279.10ms] |
| `📥️inputs/📋️data/🧪️tests/🟦️.ts` | build-inputs-selected | 1 | pass=4 fail=7 | 2s | 203MB | STALE | inputs owners | windows / stable-file / data producers drifted behind the Source reader changes — first red: neutral actual graph inventory producers admit closed DATA before writer effects [41.22ms] |
| `📥️inputs/💾️storage/🧪️tests/♻️scoped/🔄️lifetime/🍎️nx-recovery-after-maintenance/🟦️.ts` | build-inputs-storage | 1 | pass=0 fail=3 | 1s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: actual Nx recovery reopens after consumed Native maintenance: false [804.04ms] |
| `📥️inputs/💾️storage/🧪️tests/♻️scoped/🔄️lifetime/🟦️.ts` | build-inputs-storage | 1 | pass=0 fail=1 | 0s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: Nx closes every consumed optional receiver before fresh physical admission [99.30ms] |
| `📥️inputs/💾️storage/🧪️tests/♻️scoped/🟦️.ts` | build-inputs-storage | 1 | pass=0 fail=2 pass=1 fail=1 | 12s | 161MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: scoped optional maintenance refuses its own accepted reader [10123.28ms] |
| `📥️inputs/💾️storage/🧪️tests/🎛️phase/🟦️.ts` | build-inputs-storage | 1 | pass=0 fail=8 | 0s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: graph writer carries only genuine explicit phases through each store admission [45.02ms] |
| `📥️inputs/💾️storage/🧪️tests/🎟️custody/🟦️.ts` | build-inputs-storage | 1 | pass=0 fail=1 | 0s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: optional graph headroom inventories once inside acquired custody and never before deferred cu |
| `📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts` | build-inputs-storage | 1 | pass=4 fail=4 | 1s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: incremental optional ledger commits intent before mutation and repairs real API crashes [826. |
| `📥️inputs/💾️storage/🧪️tests/🔏️closure/🟦️.ts` | build-inputs-storage | 1 | pass=0 fail=1 | 0s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: physical group implementation participates in every current and pinned authority [103.45ms] |
| `📥️inputs/💾️storage/🧪️tests/🔒️lifetime/🟦️.ts` | build-inputs-storage | 1 | pass=0 fail=1 | 0s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: actual graph writer releases facts and image before maintenance or blocking admission [51.84m |
| `📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts` | build-inputs-storage | 1 | pass=0 fail=2 | 0s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: graph writer admission reconciles exactly once before pressure-free work [46.24ms] |
| `📥️inputs/💾️storage/🧪️tests/🛑️primary/🟦️.ts` | build-inputs-storage | 1 | pass=3 fail=1 | 1s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: graph namespace writer defers only a live exact timeout without masking its primary [21.27ms] |
| `📥️inputs/💾️storage/🧪️tests/🟦️.ts` | build-inputs-storage | 1 | pass=12 fail=9 | 31s | 219MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: actual shared graph writer completes inside its retained graph artifact reader [20003.53ms] |
| `📥️inputs/💾️storage/🧪️tests/🪪️identity/🟦️.ts` | build-inputs-storage | 1 | pass=2 fail=1 | 0s | 0MB | STALE | storage owners | nested persistent image / extracted ports (cache-d class `nested-image law`) and phase roster drift — first red: neutral graph writer identity preserves the exact original store and hides quota effect ports |
| `🔧️tools/💾️storage/🧪️tests/🟦️.ts` | build-nx-tool | 1 | pass=1 fail=3 | 0s | 0MB | STALE | storage owner | call-order of graph publication preparation — first red: tool storage binds graph publication preparation to the artifact owner [19.72ms] |
| `🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts` | build-parsers | 1 | pass=3 fail=1 | 57s | 1106MB | STALE | ext-f | paths[0] undefined: parser package moved to devDependencies — first red: actual pinned Nx resolves the declared workspace instead of inherited private launch roots [8 |
| `📥️source/🔏️closure/🏗️compiler/🧪️tests/🟦️.ts` | build-source-closure | 1 | pass=0 fail=2 | 7s | 134MB | STALE | closure owners | admitted-program / Source revision harness drift (Immutable implementation snapshot changed, separated capture) — first red: actual command Source capture follows compiler imports and excludes dormant test DATA [6732.5 |
| `📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts` | build-source-closure | 1 | pass=0 fail=1 | 0s | 0MB | STALE | closure owners | admitted-program / Source revision harness drift (Immutable implementation snapshot changed, separated capture) — first red: captured Node modules execute immutable Source with live image DATA [74.06ms] |
| `📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts` | build-source-closure | 1 | pass=12 fail=4 | 3s | 400MB | STALE | closure owners | admitted-program / Source revision harness drift (Immutable implementation snapshot changed, separated capture) — first red: actual auxiliary immutable loader binds eval bytes argv and physical Source [157.11ms] |
| `📥️source/🔏️closure/🧪️tests/🔏️selection/🟦️.ts` | build-source-closure | 1 | pass=1 fail=2 | 2s | 162MB | STALE | closure owners | admitted-program / Source revision harness drift (Immutable implementation snapshot changed, separated capture) — first red: actual admitted compiler binds child Source JSON and helper images under its Original [1537.0 |
| `📥️source/🔏️closure/🧪️tests/🚫️cache-root-refusal/🟦️.ts` | build-source-closure | 1 | pass=0 fail=2 | 2s | 67MB | STALE | closure owners | admitted-program / Source revision harness drift (Immutable implementation snapshot changed, separated capture) — first red: a nested child under the issued cache artifact root is refused as an external implementation |
| `🧪️tests/🕸️graph/🌐️source-discovery/🟦️.ts` | build-workspace-graph-laws | 1 | pass=4 fail=2 | 1s | 0MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: primary native owner discovery retains its original phase cancellation before any fact write |
| `🧪️tests/🕸️graph/🎯️source-demand/📋️scope/🟦️.ts` | build-workspace-graph-laws | 1 | pass=3 fail=1 | 0s | 0MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: birth text references stay data while its literal child remains executable [16.01ms] |
| `🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts` | build-workspace-graph-laws | 1 | pass=19 fail=11 | 1s | 0MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: runtime module facts remain bound to the admitted parser and compiler revision [3.14ms] |
| `🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts` | build-workspace-graph-laws | 1 | pass=5 fail=7 | 30s | 338MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: nested consumers reuse the exact pinned authority and reject a changed publication [667.57ms] |
| `🧪️tests/🕸️graph/📐️portable-model/🟦️.ts` | build-workspace-graph-laws | 1 | pass=5 fail=2 | 2s | 313MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: portable configuration and Nx projection preserve command semantics [4.14ms] |
| `🧪️tests/🕸️graph/🔄️candidate/🟦️.ts` | build-workspace-graph-laws | 1 | pass=0 fail=3 | 0s | 0MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: Graph candidates release each registry scope before bounded same-Original readmission [113.48 |
| `🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts` | build-workspace-graph-laws | 1 | pass=14 fail=5 | 8s | 364MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: Nx projects exactly the common graph into its independent task graph implementation [1048.95m |
| `🧪️tests/🕸️graph/🛂️admission/🟦️.ts` | build-workspace-graph-laws | 1 | pass=7 fail=13 | 3s | 199MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: selected graph admission agrees with independent Nx task prerequisites [48.76ms] |
| `🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts` | build-workspace-graph-laws | 1 | pass=6 skip=1 fail=7 | 27s | 268MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: ordinary immutable graph readers do not inventory optional accelerator caches [847.91ms] |
| `🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts` | build-workspace-graph-laws | 1 | pass=10 fail=5 | 1s | 0MB | STALE | graph laws owners | extracted-function ports and Original gates (nx namespace / issuance) after the graph refactors — first red: portable native caller preserves exact original resources and literal task arguments [79.15ms |
| `🧪️tests/🟦️.ts` | build-workspace-laws | 1 | pass=3 fail=2 | 3s | 485MB | STALE | law owner | ajv 2020 meta schema not registered in the retirement SQLite oracle — first red: recursive retirement support matches independent SQLite reachability [32.09ms] |
| `🧪️tests/🎟️phase/🟦️.ts` | build-workspace-phase-laws | 1 | pass=3 fail=8 | 1s | 0MB | STALE | phase owner | phase propagation laws drifted behind graph API changes — first red: Build graph APIs carry the exact active issued phase before inference and reads [18.51ms] |
| `🧪️tests/🎟️phase/🦀️framework/🟦️.ts` | build-workspace-phase-laws | 1 | pass=0 fail=2 | 0s | 0MB | STALE | phase owner | phase propagation laws drifted behind graph API changes — first red: Framework Cargo helper carries the caller actual phase through completion [75.56ms] |
| `🧪️tests/🎯️source-roster/🟦️.ts` | build-workspace-phase-laws | 1 | pass=1 fail=2 | 0s | 0MB | STALE | dashboard owner | roster entry 🎛️dashboard/🧪️tests/🧊️execution moved — first red: all26 Source target declarations and audited script literals retain runtime entries separatel |

## Green files (all suites)

| file | suite | counts | wall | peak RSS | before |
|---|---|---|---|---|---|
| `🔧️tools/🏗️bazel/🧪️tests/📥️actions/🟦️.ts` | build-bazel-contracts | pass=11 skip=1 fail=0 | 2s | 170MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️closure/🟦️.ts` | build-bazel-contracts | pass=8 skip=1 fail=0 | 1s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️control/🟦️.ts` | build-bazel-contracts | pass=1 fail=0 | 6s | 140MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️delegation/🟦️.ts` | build-bazel-contracts | pass=1 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️fingerprint/🟦️.ts` | build-bazel-contracts | pass=3 fail=0 | 1s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️framing/🟦️.ts` | build-bazel-contracts | pass=1 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️inventory/🟦️.ts` | build-bazel-contracts | pass=2 fail=0 | 1s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️keys/🟦️.ts` | build-bazel-contracts | pass=3 fail=0 | 4s | 91MB | unreached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️mode/🟦️.ts` | build-bazel-contracts | pass=34 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️outputs/🟦️.ts` | build-bazel-contracts | pass=2 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️paths/🟦️.ts` | build-bazel-contracts | pass=10 skip=1 fail=0 | 1s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️platform/🟦️.ts` | build-bazel-contracts | pass=3 skip=1 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧪️tests/🟦️.ts` | build-bazel-contracts | pass=2 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️sources/🟦️.ts` | build-bazel-contracts | pass=9 skip=1 fail=0 | 2s | 144MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📥️workspace/🟦️.ts` | build-bazel-contracts | pass=4 skip=1 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/📦️snapshot/🟦️.ts` | build-bazel-contracts | pass=4 fail=0 | 1s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/🟦️.ts` | build-bazel-contracts | pass=12 fail=0 | 3s | 117MB | reached |
| `🔧️tools/🏗️bazel/👁️evidence/♨️warm/🧪️tests/🟦️.ts` | build-bazel-evidence | pass=3 skip=1 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/👁️evidence/🧪️tests/🟦️.ts` | build-bazel-evidence | pass=2 skip=1 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/👁️evidence/🧪️tests/🧮️closure/🟦️.ts` | build-bazel-evidence | pass=3 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🚦️test-classes/🧪️tests/🟦️.ts` | build-bazel-evidence | pass=6 skip=1 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🚦️test-classes/🧾️witness/🧪️tests/🟦️.ts` | build-bazel-evidence | pass=3 skip=1 fail=0 | 1s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🚦️test-classes/🧾️witness/🧪️tests/🧮️digest/🟦️.ts` | build-bazel-evidence | pass=4 fail=0 | 1s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧾️test-results/🧪️tests/🟦️.ts` | build-bazel-evidence | pass=6 skip=1 fail=0 | 1s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🪵️failure-tail/🧪️tests/🔁️churn/🟦️.ts` | build-bazel-evidence | pass=2 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🪵️failure-tail/🧪️tests/🟦️.ts` | build-bazel-evidence | pass=5 fail=0 | 1s | 0MB | unreached |
| `🔧️tools/🏗️bazel/⚙️operations/🧪️tests/🟦️.ts` | build-bazel-execution | pass=10 skip=2 fail=0 | 4s | 173MB | unreached |
| `🔧️tools/🏗️bazel/🎬️executor/🧪️tests/🟦️.ts` | build-bazel-execution | pass=5 fail=0 | 7s | 204MB | unreached |
| `🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🧱️workspace/🟦️.ts` | build-bazel-execution | pass=3 fail=0 | 2s | 72MB | unreached |
| `🔧️tools/🏗️bazel/🎯️affected/🧪️tests/🟦️.ts` | build-bazel-execution | pass=5 skip=1 fail=0 | 2s | 134MB | reached |
| `🔧️tools/🏗️bazel/🏃️execution/🧪️tests/👷️worker/🔒️real-bazel/🟦️.ts` | build-bazel-execution | pass=0 skip=1 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🏃️execution/🧪️tests/👷️worker/🟦️.ts` | build-bazel-execution | pass=10 fail=0 | 6s | 116MB | reached |
| `🔧️tools/🏗️bazel/🏃️execution/🧪️tests/📥️strategies/🟦️.ts` | build-bazel-execution | pass=2 skip=1 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️classes/🟦️.ts` | build-bazel-execution | pass=0 skip=1 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts` | build-bazel-execution | pass=0 skip=1 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🟦️.ts` | build-bazel-execution | pass=16 fail=0 | 5s | 124MB | reached |
| `🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️shim/🟦️.ts` | build-bazel-execution | pass=2 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🧭️commands/🧪️tests/🟦️.ts` | build-bazel-execution | pass=3 skip=1 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🟦️.ts` | build-bazel-host | pass=4 skip=1 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🧮️resources/🟦️.ts` | build-bazel-host | pass=5 fail=0 | 1s | 0MB | reached |
| `🔧️tools/🏗️bazel/🐹️go/🧪️tests/🟦️.ts` | build-bazel-host | pass=4 fail=0 | 1s | 0MB | unreached |
| `🔧️tools/🏗️bazel/👷️workers/🧪️tests/🟦️.ts` | build-bazel-host | pass=3 skip=1 fail=0 | 1s | 0MB | reached |
| `🔧️tools/🏗️bazel/💾️storage/🗑️reset/🧪️tests/🟦️.ts` | build-bazel-host | pass=1 fail=0 | 0s | 0MB | reached |
| `🔧️tools/🏗️bazel/🧰️toolchains/🧪️tests/🟦️.ts` | build-bazel-host | pass=4 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🔬️concurrency/🟦️.ts` | build-bazel-sessions | pass=0 skip=1 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts` | build-bazel-sessions | pass=40 fail=0 | 17s | 349MB | reached |
| `🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🔒️real-bazel/🟦️.ts` | build-bazel-snapshots | pass=0 skip=1 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🟦️.ts` | build-bazel-snapshots | pass=5 fail=0 | 1s | 0MB | unreached |
| `🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🔒️planned/🟦️.ts` | build-bazel-snapshots | pass=1 skip=1 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🟦️.ts` | build-bazel-snapshots | pass=21 fail=0 | 5s | 223MB | unreached |
| `🔧️tools/🏗️bazel/🔁️probe/🧪️tests/🟦️.ts` | build-bazel-snapshots | pass=6 fail=0 | 0s | 0MB | unreached |
| `🧹️cache-gc/👯️dedupe/🧪️tests/🟦️.ts` | build-cache-gc | pass=8 fail=0 | 1s | 0MB | reached |
| `🧹️cache-gc/🧪️tests/💰️budget/🟦️.ts` | build-cache-gc | pass=5 fail=0 | 0s | 0MB | reached |
| `🧹️cache-gc/🧪️tests/📁️directory/🟦️.ts` | build-cache-gc | pass=2 fail=0 | 0s | 0MB | unreached |
| `🧹️cache-gc/🧪️tests/🔬️verify/🟦️.ts` | build-cache-gc | pass=4 fail=0 | 0s | 0MB | unreached |
| `🧹️cache-gc/🧪️tests/🚀️launch/🟦️.ts` | build-cache-gc | pass=9 fail=0 | 0s | 0MB | unreached |
| `🧹️cache-gc/🧪️tests/🟦️.ts` | build-cache-gc | pass=19 fail=0 | 1s | 0MB | unreached |
| `📥️caller/🛰️service/🧪️tests/🟦️.ts` | build-caller-birth | pass=117 fail=0 | 2s | 186MB | unreached |
| `📥️caller/🧪️tests/🌐️globals/🟦️.ts` | build-caller-birth | pass=1 fail=0 | 20s | 382MB | unreached |
| `📥️caller/🧪️tests/🚪️door/🟦️.ts` | build-caller-birth | pass=3 fail=0 | 0s | 0MB | unreached |
| `⌨️command/🧪️tests/🌱️plan-child/🟦️.ts` | build-command-line | pass=1 fail=0 | 0s | 0MB | reached |
| `⏱️profile/🧪️tests/🟦️.ts` | build-command-line | pass=8 fail=0 | 0s | 0MB | unreached |
| `⚙️operations/🔤️options/🧪️tests/🟦️.ts` | build-command-line | pass=15 fail=0 | 0s | 0MB | unreached |
| `⚙️operations/🧪️tests/🛗️requires/🟦️.ts` | build-command-line | pass=3 fail=0 | 1s | 0MB | unreached |
| `📤️arguments/🧪️tests/⚖️contracts/🟦️.ts` | build-command-line | pass=4 fail=0 | 0s | 0MB | reached |
| `📤️arguments/🧪️tests/🟦️.ts` | build-command-line | pass=18 fail=0 | 1s | 0MB | reached |
| `🚀️bootstrap/🧪️tests/🎯️closure/🟦️.ts` | build-command-line | pass=5 fail=0 | 1s | 0MB | unreached |
| `🚪️door/🧪️tests/📤️output/🟦️.ts` | build-command-line | pass=4 fail=0 | 1s | 0MB | unreached |
| `🚪️door/🧪️tests/🟦️.ts` | build-command-line | pass=7 fail=0 | 1s | 0MB | unreached |
| `🟢️readiness/🧪️tests/🟦️.ts` | build-command-line | pass=19 fail=0 | 3s | 73MB | unreached |
| `🧭️invocation/🧪️tests/📌️pinned-reader/🟦️.ts` | build-command-line | pass=3 fail=0 | 1s | 0MB | unreached |
| `🧭️invocation/🧪️tests/📝️plan-file/🟦️.ts` | build-command-line | pass=2 fail=0 | 1s | 0MB | reached |
| `🧭️invocation/🧪️tests/🔗️process-chain/🟦️.ts` | build-command-line | pass=9 fail=0 | 60s | 110MB | reached |
| `🧭️invocation/🧪️tests/🟦️.ts` | build-command-line | pass=8 fail=0 | 1s | 0MB | reached |
| `🧭️invocation/🧪️tests/🧊️isolated-plan/🟦️.ts` | build-command-line | pass=4 fail=0 | 2s | 931MB | unreached |
| `🧭️invocation/🧪️tests/🧊️taxonomy/🟦️.ts` | build-command-line | pass=2 fail=0 | 0s | 0MB | unreached |
| `📐️conformance/🧪️tests/🟦️.ts` | build-doctor-preflight | pass=14 fail=0 | 31s | 116MB | unreached |
| `🧾️ci-report/🧪️tests/🟦️.ts` | build-doctor-preflight | pass=18 fail=0 | 2s | 94MB | unreached |
| `🩺️doctor/🧪️tests/⌨️command/🟦️.ts` | build-doctor-preflight | pass=9 fail=0 | 0s | 0MB | unreached |
| `🩺️doctor/🧪️tests/🔎️probes/🟦️.ts` | build-doctor-preflight | pass=14 fail=0 | 1s | 0MB | unreached |
| `🩺️doctor/🧪️tests/🟦️.ts` | build-doctor-preflight | pass=17 fail=0 | 0s | 0MB | unreached |
| `🩺️preflight/🧪️tests/🟦️.ts` | build-doctor-preflight | pass=3 fail=0 | 0s | 0MB | reached |
| `🩺️preflight/🧪️tests/🧮️matrix/🟦️.ts` | build-doctor-preflight | pass=4 fail=0 | 1s | 0MB | reached |
| `🪞️parity/🧪️tests/🟦️.ts` | build-doctor-preflight | pass=13 fail=0 | 2s | 104MB | unreached |
| `🪪️agent-sessions/⌨️command/🧪️tests/🟦️.ts` | build-doctor-preflight | pass=9 fail=0 | 2s | 77MB | unreached |
| `🏃️execution/🔒️hermetic/🧪️tests/🔗️shims/🟦️.ts` | build-execution | pass=2 fail=0 | 0s | 0MB | unreached |
| `🏃️execution/🔒️hermetic/🧪️tests/🟦️.ts` | build-execution | pass=7 fail=0 | 0s | 0MB | unreached |
| `🏃️execution/🧪️tests/🎯️coverage/🟦️.ts` | build-execution | pass=5 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/🕰️generations/🧩️claims/🧪️tests/🏃️processes/🟦️.ts` | build-generation-claims | pass=7 fail=0 | 9s | 106MB | reached |
| `🕸️graph/🕰️generations/🧩️claims/🧪️tests/🟦️.ts` | build-generation-claims | pass=113 fail=0 | 25s | 58MB | reached |
| `🕸️graph/🕰️generations/🔭️coverage/🧪️tests/🟦️.ts` | build-generation-coverage | pass=61 fail=0 | 17s | 71MB | unreached |
| `🕸️graph/🕰️generations/🔭️coverage/🧪️tests/🤝️queue/🟦️.ts` | build-generation-coverage | pass=8 fail=0 | 3s | 58MB | unreached |
| `🕸️graph/🕰️generations/🧪️tests/🧮️fact-index/🟦️.ts` | build-generation-facts | pass=4 fail=0 | 3s | 150MB | unreached |
| `🕸️graph/🕰️generations/🧮️fact-codec/🧪️tests/🟦️.ts` | build-generation-facts | pass=38 fail=0 | 22s | 808MB | unreached |
| `🕸️graph/🕰️generations/🌊️json-stream/🧪️tests/🟦️.ts` | build-generation-load | pass=4 fail=0 | 4s | 189MB | unreached |
| `🕸️graph/🕰️generations/🌍️environment/🧪️tests/🟦️.ts` | build-generation-load | pass=20 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🕰️generations/🔌️load/🧪️tests/🚧️withheld-dependency/🟦️.ts` | build-generation-load | pass=20 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts` | build-generation-load | pass=52 fail=0 | 15s | 92MB | unreached |
| `🕸️graph/🕰️generations/🧷️pin/🧪️tests/🟦️.ts` | build-generation-pin | pass=101 fail=0 | 12s | 68MB | unreached |
| `🕸️graph/🕰️generations/🧷️pin/🧪️tests/🤝️nested/🟦️.ts` | build-generation-pin | pass=7 fail=0 | 2s | 67MB | unreached |
| `🕸️graph/🕰️generations/🧷️pin/🧪️tests/🤝️plan/🟦️.ts` | build-generation-pin | pass=6 fail=0 | 2s | 61MB | unreached |
| `🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🏪️store/🟦️.ts` | build-generations | pass=3 fail=0 | 11s | 74MB | reached |
| `🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🟦️.ts` | build-generations | pass=135 fail=0 | 21s | 77MB | reached |
| `🕸️graph/🕰️generations/🌿️frontier-walk/🧪️tests/🟦️.ts` | build-generations | pass=4 fail=0 | 5s | 43MB | reached |
| `🕸️graph/🕰️generations/🛌️yield/🧪️tests/🟦️.ts` | build-generations | pass=7 fail=0 | 1s | 0MB | reached |
| `🕸️graph/🕰️generations/🧪️tests/♻️refine/🟦️.ts` | build-generations | pass=1 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🕰️generations/🧪️tests/🌊️stream/🟦️.ts` | build-generations | pass=5 fail=0 | 15s | 767MB | unreached |
| `🕸️graph/🕰️generations/🧪️tests/🌱️unborn-root/🟦️.ts` | build-generations | pass=2 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🕰️generations/🧪️tests/📋️summary/🟦️.ts` | build-generations | pass=2 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🕰️generations/🧪️tests/📦️document-size/🟦️.ts` | build-generations | pass=11 fail=0 | 0s | 0MB | reached |
| `🕸️graph/🕰️generations/🧪️tests/🟦️.ts` | build-generations | pass=26 fail=0 | 50s | 629MB | unreached |
| `🕸️graph/🕰️generations/🧪️tests/🧱️stable-directories/🟦️.ts` | build-generations | pass=5 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🕰️generations/🧪️tests/🧶️shared-strings/🟦️.ts` | build-generations | pass=1 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🕰️generations/🧪️tests/🧹️retire/🟦️.ts` | build-generations | pass=1 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/🧭️file-owner/🧪️tests/🟦️.ts` | build-graph-owners | pass=6 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🧾️collect-errors/🧪️tests/🟦️.ts` | build-graph-owners | pass=4 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🧾️registry-consistency/🧪️tests/🟦️.ts` | build-graph-owners | pass=28 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/⏱️phases/🧪️tests/🟦️.ts` | build-graph-protocol | pass=17 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🔁retry/🧪️tests/🧮️projection/🟦️.ts` | build-graph-protocol | pass=4 fail=0 | 1s | 0MB | reached |
| `🕸️graph/🗂️generation-files/🧪️tests/🟦️.ts` | build-graph-protocol | pass=3 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🗂️generation-files/🧪️tests/🧮️model/🟦️.ts` | build-graph-protocol | pass=4 fail=0 | 1s | 0MB | reached |
| `🕸️graph/🚪️admission/🧪️tests/🟦️.ts` | build-graph-protocol | pass=5 fail=0 | 1s | 0MB | reached |
| `🕸️graph/🧪️tests/♻️tagged-retention/🟦️.ts` | build-graph-protocol | pass=74 fail=0 | 12s | 81MB | reached |
| `🕸️graph/🧪️tests/📤️portable/🟦️.ts` | build-graph-protocol | pass=3 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/🧪️tests/🔓️sealed-reuse/🟦️.ts` | build-graph-protocol | pass=14 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🧪️tests/🔬️analysis-providers/🟦️.ts` | build-graph-protocol | pass=1 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/🧪️tests/🕊️graph-release/🟦️.ts` | build-graph-protocol | pass=18 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/🧪️tests/🖼️canvas-runtime-assets/🟦️.ts` | build-graph-protocol | pass=1 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🧪️tests/🪪️generation-identity/🟦️.ts` | build-graph-protocol | pass=3 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/🦀️rust/📥️intent/🧪️tests/🟦️.ts` | build-graph-rust | pass=10 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/🦀️rust/📥️intent/🧪️tests/🪢️trim-paths/🟦️.ts` | build-graph-rust | pass=2 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/🎯️test-owners/🧪️tests/🟦️.ts` | build-graph-test-owners | pass=66 fail=0 | 0s | 0MB | reached |
| `🌡️host-capacity/📖️texts/🧪️tests/🟦️.ts` | build-host-capacity | pass=3 fail=0 | 0s | 0MB | unreached |
| `🌡️host-capacity/🧪️tests/🏊️farm/🟦️.ts` | build-host-capacity | pass=3 fail=0 | 1s | 0MB | unreached |
| `🌡️host-capacity/🧪️tests/📈️memory-fit/🟦️.ts` | build-host-capacity | pass=4 fail=0 | 1s | 0MB | reached |
| `🌡️host-capacity/🧪️tests/🔁️restart/🟦️.ts` | build-host-capacity | pass=2 fail=0 | 0s | 0MB | unreached |
| `🌡️host-capacity/🧪️tests/🟦️.ts` | build-host-capacity | pass=15 fail=0 | 1s | 0MB | unreached |
| `🌡️host-capacity/🧪️tests/🧮️arguments/🟦️.ts` | build-host-capacity | pass=6 fail=0 | 1s | 0MB | reached |
| `📏️incremental/🧪️tests/🟦️.ts` | build-incremental | pass=46 skip=3 fail=0 | 7s | 112MB | unreached |
| `📏️incremental/🏎️bench/🧪️tests/🟦️.ts` | build-incremental-bench | pass=38 fail=0 | 13s | 152MB | unreached |
| `📏️incremental/🤝️concurrent/🧪️tests/🟦️.ts` | build-incremental-concurrent | pass=25 skip=1 fail=0 | 53s | 125MB | unreached |
| `🕸️graph/📥️inference/♻️revision/🧪️tests/🟦️.ts` | build-inference | pass=4 fail=0 | 2s | 89MB | reached |
| `🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🟦️.ts` | build-inference | pass=2 fail=0 | 3s | 116MB | reached |
| `🕸️graph/📥️inference/🧪️tests/💾️cache/🟦️.ts` | build-inference | pass=11 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/📥️inference/🧪️tests/📐️declared-inputs/🟦️.ts` | build-inference | pass=5 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/📥️inference/🧪️tests/🔌️providers/🟦️.ts` | build-inference | pass=4 fail=0 | 19s | 1012MB | reached |
| `🕸️graph/📥️inference/🧪️tests/🔖️output-stamps/🟦️.ts` | build-inference | pass=4 fail=0 | 0s | 0MB | reached |
| `🕸️graph/📥️inference/🧪️tests/🔗️late-closures/🟦️.ts` | build-inference | pass=4 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/📥️inference/🧪️tests/🔗️workspace-closure/🟦️.ts` | build-inference | pass=7 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/📥️inference/🧪️tests/🖨️print-documents/🟦️.ts` | build-inference | pass=4 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/📥️inference/🧪️tests/🗜️interned-closures/🟦️.ts` | build-inference | pass=3 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/📥️inference/🧪️tests/🙈️presentation-resources/🟦️.ts` | build-inference | pass=3 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/📥️inference/🧪️tests/🟦️.ts` | build-inference | pass=5 fail=0 | 1s | 0MB | reached |
| `🕸️graph/📥️inference/🧪️tests/🦀️compiled-closure/🟦️.ts` | build-inference | pass=5 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/📥️inference/🧪️tests/🧩️component-cache/🟦️.ts` | build-inference | pass=1 fail=0 | 2s | 77MB | unreached |
| `🕸️graph/📥️inference/🧪️tests/🧮️covered-inputs/🟦️.ts` | build-inference | pass=3 fail=0 | 1s | 0MB | unreached |
| `🕸️graph/📥️inference/🎯️target-policy/🧪️tests/🟦️.ts` | build-inference-policy | pass=10 fail=0 | 2s | 128MB | reached |
| `🕸️graph/📥️inference/🏭️generator-outputs/🧪️tests/🟦️.ts` | build-inference-policy | pass=6 fail=0 | 3s | 108MB | reached |
| `🕸️graph/📥️inference/🕸️dependency-edges/🧪️tests/🟦️.ts` | build-inference-policy | pass=4 fail=0 | 1s | 0MB | reached |
| `🕸️graph/📥️inference/🦀️native-manifests/🧪️tests/📜️toml/🟦️.ts` | build-inference-policy | pass=5 fail=0 | 0s | 0MB | reached |
| `🕸️graph/📥️inference/🦀️native-manifests/🧪️tests/🦀️cargo-go/🟦️.ts` | build-inference-policy | pass=5 fail=0 | 0s | 0MB | reached |
| `🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🟦️.ts` | build-inference-projects | pass=20 fail=0 | 13s | 362MB | unreached |
| `🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🚦️governed-dispatch/🟦️.ts` | build-inference-projects | pass=2 fail=0 | 0s | 0MB | unreached |
| `🕸️graph/📥️inference/🔎️owner-discovery/🧪️tests/🔁️observation/🟦️.ts` | build-inference-projects | pass=1 fail=0 | 0s | 0MB | reached |
| `🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts` | build-inference-projects | pass=17 fail=0 | 3s | 122MB | reached |
| `🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests/🟦️.ts` | build-inference-script-inputs | pass=25 fail=0 | 8s | 582MB | unreached |
| `🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🟦️.ts` | build-inference-script-inputs | pass=4 fail=0 | 1s | 0MB | reached |
| `🕸️graph/📥️inference/🪢️revision-graph/🧪️tests/⚖️roster/🟦️.ts` | build-inference-script-inputs | pass=7 fail=0 | 1s | 0MB | reached |
| `📥️inputs/🌳️owner-view/🧪️tests/🟦️.ts` | build-inputs-revision | pass=4 fail=0 | 0s | 0MB | reached |
| `📥️inputs/🌳️owner-view/🧪️tests/🧬️inference/🟦️.ts` | build-inputs-revision | pass=2 fail=0 | 0s | 0MB | reached |
| `📥️inputs/🔏️revision/🧪️tests/📌️received-pin/🟦️.ts` | build-inputs-revision | pass=3 fail=0 | 0s | 0MB | unreached |
| `📥️inputs/🔏️revision/🧪️tests/🖼️captured-image/🟦️.ts` | build-inputs-revision | pass=4 fail=0 | 2s | 44MB | unreached |
| `📥️inputs/🔏️revision/🧪️tests/🟦️.ts` | build-inputs-revision | pass=8 fail=0 | 20s | 121MB | unreached |
| `📥️inputs/🔏️revision/🧪️tests/🧊️own-stamp/🟦️.ts` | build-inputs-revision | pass=2 fail=0 | 4s | 35MB | unreached |
| `📥️inputs/🔏️revision/🧪️tests/🧓️stale-reader/🟦️.ts` | build-inputs-revision | pass=8 fail=0 | 3s | 31MB | unreached |
| `📥️inputs/🦀️resolution/🧪️tests/🟦️.ts` | build-inputs-revision | pass=15 fail=0 | 1s | 0MB | unreached |
| `📥️inputs/🧪️tests/✂️closure-chunks/🟦️.ts` | build-inputs-revision | pass=3 fail=0 | 1s | 0MB | unreached |
| `📥️inputs/🧪️tests/🟦️.ts` | build-inputs-revision | pass=4 fail=0 | 3s | 85MB | reached |
| `📥️inputs/🪛️toolchain-activation/🧪️tests/🟦️.ts` | build-inputs-revision | pass=2 fail=0 | 0s | 0MB | reached |
| `📥️inputs/🪶️lean-inventory/🧪️tests/🟦️.ts` | build-inputs-revision | pass=14 fail=0 | 0s | 0MB | unreached |
| `📥️inputs/🎯️precision/🧪️tests/🔣️glob-dialects/🟦️.ts` | build-inputs-selected | pass=6 fail=0 | 1s | 0MB | reached |
| `📥️inputs/🎯️precision/🧪️tests/🟦️.ts` | build-inputs-selected | pass=14 skip=1 fail=0 | 5s | 216MB | unreached |
| `📥️inputs/🎯️precision/🧪️tests/🦀️exporters/🟦️.ts` | build-inputs-selected | pass=4 fail=0 | 3s | 319MB | unreached |
| `📥️inputs/🎯️selected/🦀️rust/🧪️tests/🟦️.ts` | build-inputs-selected | pass=4 fail=0 | 0s | 0MB | reached |
| `📥️inputs/📂️membership/🧪️tests/🛡️ignore/🟦️.ts` | build-inputs-selected | pass=3 fail=0 | 2s | 59MB | reached |
| `📥️inputs/📂️membership/🧪️tests/🟦️.ts` | build-inputs-selected | pass=2 fail=0 | 11s | 248MB | unreached |
| `📥️inputs/💾️storage/🧪️tests/♻️retirement/🟦️.ts` | build-inputs-storage | pass=2 fail=0 | 1s | 0MB | reached |
| `📥️inputs/💾️storage/🧪️tests/♻️scoped/⚠️scoped-retirement-and-bridge/🟦️.ts` | build-inputs-storage | pass=3 fail=0 | 1s | 0MB | reached |
| `📥️inputs/💾️storage/🧪️tests/💧️headroom/🟦️.ts` | build-inputs-storage | pass=4 fail=0 | 0s | 0MB | reached |
| `📥️inputs/💾️storage/🧪️tests/📏️quota/🔎️observation/🟦️.ts` | build-inputs-storage | pass=1 fail=0 | 0s | 0MB | reached |
| `📥️inputs/💾️storage/🧪️tests/📒️ledger/🔎️observation/🟦️.ts` | build-inputs-storage | pass=2 fail=0 | 0s | 0MB | reached |
| `📥️inputs/💾️storage/🧪️tests/🤝️shared-writer/🟦️.ts` | build-inputs-storage | pass=4 fail=0 | 0s | 0MB | reached |
| `📥️inputs/💾️storage/🧪️tests/🪟️retirement-window/🟦️.ts` | build-inputs-storage | pass=4 fail=0 | 0s | 0MB | unreached |
| `🔒️leases/⌨️command/🧪️tests/🟦️.ts` | build-leases-command | pass=4 fail=0 | 0s | 0MB | unreached |
| `🔒️leases/⌨️command/🧪️tests/🧮️status/🟦️.ts` | build-leases-command | pass=3 fail=0 | 4s | 73MB | reached |
| `🧠️memory/🧪️tests/🟦️.ts` | build-memory | pass=7 fail=0 | 2s | 71MB | unreached |
| `🔧️tools/🐙️semio/🧪️tests/🟦️.ts` | build-nx-tool | pass=35 fail=0 | 8s | 114MB | unreached |
| `🔧️tools/🧶️nx/📨️envelope/🧪️tests/🟦️.ts` | build-nx-tool | pass=2 fail=0 | 0s | 0MB | unreached |
| `🔧️tools/🧶️nx/🪵️failure-tail/🧪️tests/🟦️.ts` | build-nx-tool | pass=7 fail=0 | 0s | 0MB | unreached |
| `👁️observation/🧪️tests/⏱️analysis-stamp/🟦️.ts` | build-observation | pass=3 fail=0 | 1s | 0MB | unreached |
| `👁️observation/🧪️tests/📒️registry/🟦️.ts` | build-observation | pass=4 fail=0 | 0s | 0MB | unreached |
| `👁️observation/🧪️tests/🕊️release/🟦️.ts` | build-observation | pass=2 fail=0 | 0s | 0MB | unreached |
| `👁️observation/🧪️tests/🧊️verified-blocks/🟦️.ts` | build-observation | pass=2 fail=0 | 4s | 65MB | unreached |
| `👁️observation/🧪️tests/🧠️retention/🟦️.ts` | build-observation | pass=5 fail=0 | 7s | 831MB | unreached |
| `👁️observation/🧪️tests/🧭️resolution/🟦️.ts` | build-observation | pass=5 fail=0 | 1s | 0MB | unreached |
| `👁️observation/🧪️tests/🧱️block-cost/🟦️.ts` | build-observation | pass=2 fail=0 | 0s | 0MB | unreached |
| `👁️observation/🧪️tests/🧹️superseded-blocks/🟦️.ts` | build-observation | pass=7 fail=0 | 0s | 0MB | unreached |
| `👁️observation/🧪️tests/🧾️record-header/🟦️.ts` | build-observation | pass=2 fail=0 | 5s | 74MB | unreached |
| `🚀️bootstrap/📦️parsers/🧪️tests/📥️resolution/🟦️.ts` | build-parsers | pass=2 fail=0 | 4s | 187MB | reached |
| `🌐️remote-cache/🔬️action-result/🧪️tests/🟦️.ts` | build-remote-cache | pass=4 fail=0 | 1s | 0MB | unreached |
| `🌐️remote-cache/🖥️server/🧪️tests/🟦️.ts` | build-remote-cache | pass=6 fail=0 | 5s | 141MB | unreached |
| `🌐️remote-cache/🧪️tests/🟦️.ts` | build-remote-cache | pass=19 skip=2 fail=0 | 6s | 139MB | unreached |
| `🌐️remote-cache/🧹️collector/🧪️tests/🟦️.ts` | build-remote-cache | pass=7 fail=0 | 2s | 56MB | unreached |
| `🌐️remote-execution/⌨️command/🧪️tests/🟦️.ts` | build-remote-execution | pass=7 fail=0 | 1s | 0MB | unreached |
| `🌐️remote-execution/🔒️sandbox/🧪️tests/🟦️.ts` | build-remote-execution | pass=3 fail=0 | 0s | 0MB | unreached |
| `🌐️remote-execution/🗓️scheduler/🧪️tests/🟦️.ts` | build-remote-execution | pass=6 fail=0 | 0s | 0MB | unreached |
| `🌐️remote-execution/🧪️tests/📜️scenarios/🟦️.ts` | build-remote-execution | pass=1 fail=0 | 2s | 48MB | reached |
| `🌐️remote-execution/🧪️tests/🔧️bazel/🟦️.ts` | build-remote-execution | pass=0 skip=1 fail=0 | 0s | 0MB | unreached |
| `🌐️remote-execution/🧪️tests/🔧️bazel/🧭️flags/🟦️.ts` | build-remote-execution | pass=2 fail=0 | 0s | 0MB | unreached |
| `🌐️remote-execution/🧪️tests/🟦️.ts` | build-remote-execution | pass=16 fail=0 | 72s | 353MB | unreached |
| `📊️reporting/🧪️tests/🧮️names/🟦️.ts` | build-reporting | pass=2 fail=0 | 1s | 0MB | reached |
| `🏡️resident/🧪️tests/⌨️command/🟦️.ts` | build-resident | pass=3 fail=0 | 1s | 0MB | unreached |
| `🏡️resident/🧪️tests/🏊️delegate/🟦️.ts` | build-resident | pass=7 fail=0 | 2s | 53MB | unreached |
| `🏡️resident/🧪️tests/🏗️provision/🟦️.ts` | build-resident | pass=6 skip=1 fail=0 | 1s | 0MB | unreached |
| `🏡️resident/🧪️tests/🏭️executor/🟦️.ts` | build-resident | pass=7 fail=0 | 3s | 45MB | unreached |
| `🏡️resident/🧪️tests/📨️protocol/🟦️.ts` | build-resident | pass=23 fail=0 | 1s | 0MB | unreached |
| `🏡️resident/🧪️tests/🔭️watch/🟦️.ts` | build-resident | pass=5 skip=8 fail=0 | 6s | 34MB | unreached |
| `🏡️resident/🧪️tests/🕰️graph-hooks/🟦️.ts` | build-resident | pass=12 fail=0 | 0s | 0MB | unreached |
| `🏡️resident/🧪️tests/🗃️state/🟦️.ts` | build-resident | pass=13 fail=0 | 5s | 63MB | unreached |
| `🏡️resident/🧪️tests/🚪️client/🟦️.ts` | build-resident | pass=8 fail=0 | 0s | 0MB | unreached |
| `🗂️suites/🧪️tests/🟦️.ts` | build-route-table | pass=7 fail=0 | 14s | 88MB | reached |
| `🏛️sealed/🧪️tests/👥️processes/🟦️.ts` | build-sealed | pass=3 fail=0 | 1s | 0MB | reached |
| `🏛️sealed/🧪️tests/🔀️interleavings/🟦️.ts` | build-sealed | pass=3 fail=0 | 17s | 91MB | reached |
| `🏛️sealed/🧪️tests/🗂️store/🟦️.ts` | build-sealed | pass=10 fail=0 | 4s | 184MB | reached |
| `📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts` | build-source-closure | pass=15 fail=0 | 8s | 469MB | unreached |
| `📥️source/🔏️closure/🚪️command/🧪️tests/🏷️loader-identity/🟦️.ts` | build-source-closure | pass=3 fail=0 | 0s | 0MB | reached |
| `📥️source/🔏️closure/🚪️command/🧪️tests/🟦️.ts` | build-source-closure | pass=1 fail=0 | 0s | 0MB | reached |
| `📥️source/🔏️closure/🧪️tests/🔭️trace/🟦️.ts` | build-source-closure | pass=3 fail=0 | 1s | 0MB | unreached |
| `📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts` | build-source-closure | pass=3 fail=0 | 3s | 124MB | reached |
| `📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts` | build-source-closure | pass=2 fail=0 | 1s | 0MB | reached |
| `📥️source/🔏️closure/🧪️tests/🧱️static/🟦️.ts` | build-source-closure | pass=1 fail=0 | 0s | 0MB | unreached |
| `📥️source/🔏️closure/🧾️diagnostics/🧪️tests/🟦️.ts` | build-source-closure | pass=2 fail=0 | 0s | 0MB | reached |
| `📖️storybook/💾️storage/🧪️tests/🟦️.ts` | build-storybook | pass=3 fail=0 | 3s | 108MB | unreached |
| `📖️storybook/🧪️tests/🟦️.ts` | build-storybook | pass=1 fail=0 | 0s | 0MB | unreached |
| `🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🧪️tests/🟦️.ts` | build-targets-selection | pass=9 fail=0 | 0s | 0MB | unreached |
| `🎚️mode/🦀️profile/🧪️tests/🟦️.ts` | build-targets-selection | pass=11 fail=0 | 1s | 0MB | unreached |
| `🎯️targets/🧪️tests/🟦️.ts` | build-targets-selection | pass=1 fail=0 | 0s | 0MB | reached |
| `🧩️units/🧪️tests/⌨️command/🟦️.ts` | build-targets-selection | pass=10 fail=0 | 9s | 149MB | unreached |
| `🧩️units/🧪️tests/🌐️languages/🟦️.ts` | build-targets-selection | pass=52 fail=0 | 53s | 395MB | unreached |
| `🧩️units/🧪️tests/🟦️.ts` | build-targets-selection | pass=129 fail=0 | 32s | 286MB | unreached |
| `🧭️selection/👁️watch/🧪️tests/🎟️phase/🟦️.ts` | build-targets-selection | pass=19 fail=0 | 4s | 117MB | reached |
| `🧭️selection/🧪️tests/🟦️.ts` | build-targets-selection | pass=6 fail=0 | 1s | 0MB | reached |
| `🧷️format-stamps/🧪️tests/🔁️implementation-edit/🟦️.ts` | build-targets-selection | pass=10 fail=0 | 3s | 79MB | unreached |
| `🧷️format-stamps/🧪️tests/🔁️stale-codes/🟦️.ts` | build-targets-selection | pass=1 fail=0 | 0s | 0MB | unreached |
| `🧷️format-stamps/🧪️tests/🟦️.ts` | build-targets-selection | pass=14 fail=0 | 3s | 131MB | reached |
| `🔌️backends/🧪️tests/🟦️.ts` | build-tool-setup | pass=8 fail=0 | 1s | 0MB | unreached |
| `🟦️typescript/📚️libraries/🧪️tests/🟦️.ts` | build-tool-setup | pass=3 fail=0 | 1s | 0MB | unreached |
| `🟦️typescript/🧩️units/🧪️tests/🟦️.ts` | build-tool-setup | pass=21 fail=0 | 3s | 216MB | unreached |
| `🧰️tool-registry/🧪️tests/🔎️exec-command/🟦️.ts` | build-tool-setup | pass=4 fail=0 | 0s | 0MB | unreached |
| `🧰️tool-setup/⌨️command/🧪️tests/🟦️.ts` | build-tool-setup | pass=5 fail=0 | 1s | 0MB | unreached |
| `🧰️tool-setup/🧪️tests/📮️dispatch/🟦️.ts` | build-tool-setup | pass=5 fail=0 | 0s | 0MB | unreached |
| `🧰️tool-setup/🧪️tests/🟦️.ts` | build-tool-setup | pass=22 fail=0 | 0s | 0MB | unreached |
| `👷️workers/🧪️tests/🟦️.ts` | build-workers | pass=11 fail=0 | 1s | 0MB | unreached |
| `🧵️workers/🧪️tests/🟦️.ts` | build-workers | pass=7 fail=0 | 1s | 0MB | unreached |
| `🧪️tests/🕸️graph/🔁️settle-repeat/🟦️.ts` | build-workspace-graph-laws | pass=1 fail=0 | 1s | 0MB | unreached |
| `🧪️tests/🕸️graph/🔄️settlement/🟦️.ts` | build-workspace-graph-laws | pass=1 fail=0 | 0s | 0MB | reached |
| `🧪️tests/🕸️graph/🔭️observed-facts/🟦️.ts` | build-workspace-graph-laws | pass=7 fail=0 | 1s | 0MB | reached |
| `🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts` | build-workspace-graph-laws | pass=3 fail=0 | 1s | 0MB | reached |
| `🧪️tests/🕸️graph/🧲️owner-claims/🟦️.ts` | build-workspace-graph-laws | pass=2 fail=0 | 0s | 0MB | unreached |
| `🧪️tests/🎟️phase/🏭️producer/🟦️.ts` | build-workspace-phase-laws | pass=1 fail=0 | 0s | 0MB | reached |
