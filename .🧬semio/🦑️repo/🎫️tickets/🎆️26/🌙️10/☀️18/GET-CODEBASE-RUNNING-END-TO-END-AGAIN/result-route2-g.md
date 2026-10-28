# result-route2-g (wave 1026g, executor route2-g): build-library suites green through their official targets

Tools `C/route2-g-tools/` (stage.mjs, stage-run.mjs, apply-staged.mjs, rt.sh, free-names.mjs, birth-probe.sh, progress.tsv), output `C/🗑️generated/route2-g/`.

## Status (newest first)
- 11:17 (10/26) MEASUREMENT WINDOW 10:30-11:30 respected: 9 edits landed BEFORE 10:30 (see LANDED), everything since is STAGED in `C/route2-g-tools/staged/` (build-relative) and `staged-lib/` (library-relative) and verified by `stage-run.mjs` (a staged test file is rewritten to absolute specifiers + original import.meta, with staged JSON/TS overlays, then run through route-g's census envelope). `apply-staged.mjs` applies everything after 11:30 in one batch and refuses a file that changed since staging (base digests in `staged-base.json`). Official runs: none started (rule), the target runs follow the apply.
- Census method change: the census art root was outside the checkout (like the official sandbox); the suites with `scratch: checkout` run inside `.cache/test-sandbox` (build-graph-rust staged).


## Status 11:35 (10/26): FREEZE EXTENDED (coordinator 11:28) — nothing applied, no official run
- All library/process edits made by this lane after 10:30 stay STAGED (`staged/` build-relative, `staged-lib/` library-relative, base digests in `staged-base.json`). `bun C/route2-g-tools/apply-staged.mjs --dry` lists them; `bun ... apply-staged.mjs` applies them in one batch and refuses a file that changed since staging. Verify after the apply with `sh C/route2-g-tools/verify-staged.sh` (stage-run of every staged test) and `sh C/route2-g-tools/rt.sh <file>` (real tree). Then resume the official chunks: `env -u SEMIO_SCRIPT_STALL_MS python3 $C/chain-tools/detach.py <log> sh $C/route-g-tools/many.sh <tag> bazel $C/route-g-tools/targets.txt 8` (one chunk at a time, refuses while >= 4 runs wait).
- Disposition (table-route2-g.md): 9 LANDED before the window (roster, cmake root file, build texts, bazel coverage/contract/arguments/environment/purpose-birth, value-module retirement-support law), 26 files STAGED and green under `stage-run.mjs` on the unchanged tree, 5 PARTIAL, rest OPEN with a cause below. Census counts: route-g 270 green / 83 red; route2-g c4 on the unchanged tree shows the current state per file.
- OPEN by cause: (1) Node runtime has no native (FFI) provider: `SEMIO_RUNTIME_NATIVE_PROVIDER` designates a module exporting `nativeSystemV1` and no such module exists in the repo, so every Nx/Node child that reaches `withArtifactPosixRelativeDirectoryV1` throws `RuntimeCapabilityMissingV1`; this blocks observation x3, graph-laws nx-engine/admission/portable-cli/portable-model/optional-cache, execution (FOR runtime owner: ship a provider or a pure-Node fallback; birth probes `birth.tsv`/`birth2.tsv` show `none` for all of them, only the two Nx-SDK files pass under `setup-sdk` and are routed). (2) Nested persistent image family (ledger, scoped x3, nx-recovery): fixtures must live in a scratch outside every open image (`testScratchRootV1`); ledger staged partial, the DMG-heavy ones need a quiet machine. (3) Source-closure family (compiler/node/stream/selection, bazel implementation): the closure refuses any path under `.cache`/`node_modules`, the sandbox lives outside the checkout and the checkout scratch lives under `.cache`: the tests need an in-checkout, non-`.cache` fixture root (design decision for the closure owner). (4) Large source-text rewrites not started: `📥️inputs/📋️data` (8), `🧪️tests/🕸️graph/🎯️source-demand` (11), `🧪️tests/🎟️phase` (10, same bindings as the staged storage phase file), `🧪️tests/🕸️graph/🌱️native-scope` (extracted `emojiProjectJsonNodes` gained ~30 free names), `🎟️custody` (deletion moved to the groups port). (5) Product defects staged, waiting for the apply: help before selection (preflight), stale-loaded refusals of pinned invocations (2 sites), owner project.json bound, door script missing in `test-build-caller-birth` closure resources. (6) Real defect found, not mine: the course `test` target reads `🖱️ui/🧱️elements/🥞️LayeredOverview/📋️types/🟦️.ts` without declaring or inferring it (course-inputs law, after the staged oracle fix). (7) `🚪️door/🧪️tests/🚦️startup` byte budget 96000 < 100084 (door grew), `🏡️resident/service` 3 watcher/journal laws, `🧭️selection/👁️watch` staged unverified.

## Progress per file (progress.tsv)
- `🧪️tests/🎯️source-roster/🟦️.ts` — LANDED 10:21: fixture dispatch owners follow task-routes + suite tables (json), schema pattern ts|json, runtimeLiterals reads tables
- `🛠️cmake/🧪️tests/🟦️.ts` — LANDED 10:21: real defect: root CMakeLists.txt restored from HEAD^ (digest equals schema pin)
- `📖️texts/🧪️tests/🟦️.ts` — LANDED 10:23: law scans caching engine-digest owner
- `🔧️tools/🏗️bazel/🧪️tests/📥️coverage/🟦️.ts` — LANDED 10:26: bazelStarlarkRulesV1 + repo_inputs_witness
- `🔧️tools/🏗️bazel/🧪️tests/📋️contract/🟦️.ts` — LANDED 10:28: corpus digests re-baselined (5 definitions), StorageLayout pool
- `🔧️tools/🏗️bazel/🧪️tests/📤️arguments/🟦️.ts` — LANDED 10:28: runTask now in 🎬️executor, new free names
- `🔧️tools/🏗️bazel/🧪️tests/📥️environment/🟦️.ts` — LANDED 10:29: import.meta.dirname
- `🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧪️tests/🌱️birth/🟦️.ts` — LANDED 10:29: import.meta.dirname
- `🔧️tools/🏗️bazel/🧪️tests/🟦️.ts` — STAGED: restore leases per output root (held-snapshot), build label expr, free names
- `🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts` — STAGED: guard excludes plan-child (BUILD_GRAPH_PLAN_OUT_V1)
- `🔧️tools/🏗️bazel/🦀️cargo/🧪️tests/🟦️.ts` — STAGED: corpus commands launcher repo (policy nativeCommand regex)
- `🧾️store-receipt/🧪️tests/🟦️.ts` — STAGED: law follows relative static+dynamic imports one level; quoted script path; stageAdmittedArtifactTreesV1 publisher
- `🚀️bootstrap/🧪️tests/🟦️.ts` — STAGED: import edge may be a public facade that explicitly re-exports the declared module
- `🕸️graph/🦀️rust/🔒️lifetime/🧪️tests/🟦️.ts` — STAGED: withSelectedRustBuildGraphV1 caller moved to ⌨️command/⚙️runtime
- `🕸️graph/🦀️rust/📥️source/🧪️tests/🟦️.ts` — STAGED: test artifact dir instead of ticket dir; observation fixture declarationExports +12 (staged)
- `👁️observation/🧪️tests/🧫️fixtures/🔣️.json` — STAGED: declarationExports +12 new API names (shared with observation root test)
- `🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts` — STAGED: stale-epoch law runs under routing.withoutImplementationPinV1 (invocations are pinned by design)
- `🧪️tests/🟦️.ts` — STAGED: root build is a noop aggregate (law asserts it); value-module retirement-support law uses Ajv2020 (LANDED, not frozen)
- `🔧️tools/💾️storage/🧪️tests/🟦️.ts` — STAGED: graph artifact model: real encodeGraphRecordsV1/policy/schema, readdir/ledger ports; call order reads the runtime module (main flow, backend execute)
- `📥️inputs/💾️storage/🧪️tests/🎛️phase/🟦️.ts` — STAGED: writer model: new free names (nativeStatfs, writer resource, schema, consumer guard), leases partition open call; artifact model with real record encoder
- `📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts` — STAGED: consumer guard binding + cache-gc launcher import stub
- `📥️inputs/💾️storage/🧪️tests/🔒️lifetime/🟦️.ts` — STAGED: cache-gc launcher import stub in extracted writer
- `📥️inputs/💾️storage/🧪️tests/🛑️primary/🟦️.ts` — STAGED: maintenance writer moved to 🔧️custody; helper searches both modules
- `📥️inputs/💾️storage/🧪️tests/🪪️identity/🟦️.ts` — STAGED: store variables: original + read-only current.store
- `📥️inputs/💾️storage/🧪️tests/🔏️closure/🟦️.ts` — STAGED: (+🧫️cases.json worker graph path) revision rows port names (ignoredSections, relative)
- `📥️inputs/💾️storage/🧪️tests/🎟️custody/🟦️.ts` — DEFERRED: optional headroom now delegates deletion to retireBuildGraphOptionalGroupsHeadroomV1 (groups tests cover it); test model needs rewrite to the groups port
- `📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts` — PARTIAL-STAGED: image dirs from testScratchRootV1 (nested-image law); still red: line 44 expects 17 got 1 + flaky capture under load
- `⌨️command/🩺️preflight/🟦️.ts` — STAGED-PRODUCT: help is answered before the granted-selection check (real defect: help must not need a selection); verify with ⚙️operations/🧪️tests after apply
- `lib:⚡️caching/…/📡️session/🟦️.ts + lib:🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts` — STAGED-PRODUCT: stale-loaded refusals let a pinned invocation finish (generation-protocol law; also kills the ⌨️command churn flake)
- `🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🔣️.json` — STAGED: law: raw reads banned (readFileSync); the single sanctioned read inside the memoized per-file computation required (independent python oracle agrees)
- `🏡️resident/🧪️tests/🏛️service/🟦️.ts` — PARTIAL-STAGED: service environment hermetic (the bound SEMIO_BUILD_* keys of a build run made every client refuse); 29 red -> 3 red (dirty/journal queries; watcher backend)
- `📥️caller/🧪️tests/📥️birth/📜️script.ts` — STAGED: (reclassified from GATE: refusals are keys now) child selection refusal asserts its key; verify 📥️birth/🟦️.ts after apply
- `🚀️bootstrap/⌨️cli/🧬️schema/🔣️.json` — STAGED-PRODUCT: (reclassified from GATE) owner project.json bound 64 KiB -> 256 KiB: ⚡️caching/📋️project.json is 99 KiB; verify the two bootstrap/⌨️cli tests after apply
- `📊️reporting/🧪️tests/🟦️.ts` — STAGED: (reclassified from GATE) callback failure closes the shared Original: isolated declared-rejection test (registerIsolatedBuildTestV1)
- `🎚️mode/🦀️profile/🧪️tests/📥️consumption/🟦️.ts + 🔣️.json` — STAGED: re-baselined Cargo profile corpus (framework 51 wasm, root 52 wasm = 81 rows), selection owner is the Once function
- `🗂️suites/🔣️.json` — STAGED: suite build-graph-rust scratch=checkout (its bounded Source roster needs an in-checkout fixture directory; the sandbox default lives outside)
- `🧪️tests/🕸️graph/🎯️source-demand/📋️scope/🧫️cases.json + lib:📦️packages/🟦️typescript/📋️project.json` — STAGED-PRODUCT: test-build-caller-birth closure resources declare the door script the birth test executes (cache key was blind to it); verify scope test after apply
- `🗂️suites/🔣️.json` — STAGED: + fileLaunchers build-inputs-selected: 📥️inputs/🎯️selected/🧪️tests/🟦️.ts under genuine birth setup-sdk (probe: passes)
- `📥️inputs/🧊️stable-file/🧪️tests/🔣️.json` — STAGED: size refusal text split from the changed-file refusal
- `📥️inputs/🧊️stable-file/🧪️tests/🟦️.ts + 🔣️.json + 🧬️schema/🔣️.json` — STAGED: (supersedes the earlier line) stable-file now re-captures a file edited mid-read (churn read layer): append/touch/truncate cases assert the re-capture, budget reset to the start admission, and its checkpoint counts

## Findings that change the route-g classification
- GATE is mostly a misnomer: of the 16 gated files only `📥️inputs/🎯️selected/🧪️tests/🟦️.ts` passes under a genuine birth (setup-sdk, routed by `fileLaunchers` in the staged table); `📥️caller/🧪️tests/📥️birth` (refusals are keys now), `⚙️operations` (help before selection: product fix staged), `🚀️bootstrap/⌨️cli` x2 (owner project.json bound 64 KiB < ⚡️caching project 99 KiB: staged), `📊️reporting` (callback failure closes the shared Original: isolated declared-rejection form), `🧪️tests/🏗️native` (real issued control + artifacts outside the checkout) were stale/product defects. Under birth setup-sdk the observation/selection files reach a Node child that needs `SEMIO_RUNTIME_NATIVE_PROVIDER` (ffi): open, see below.
- Real defects fixed at the source: root `CMakeLists.txt` restored (digest equals the schema pin), pinned-invocation stale refusals in `📡️session` and `🚪️command` (staged), owner project.json bound (staged), help before selection (staged), `test-build-caller-birth` closure resources miss the door script (staged).

FOR nest-g: the two `Stale loaded` sites named by the generation-protocol law are fixed in `staged-lib` (guard `!implementationPinnedV1()`); apply batch lands them.
FOR noop-g / chain-g: `🏡️resident/🧪️tests/🏛️service` was red because the service binds the ambient `SEMIO_BUILD_TOOL/MODE/WORKERS/GRAPH_POLICY` of a build run and refuses clients that send none: the test service now starts with a hermetic environment (29 red to 3 red, the 3 are the dirty/journal watcher queries). `🚪️door/🧪️tests/🚦️startup` stays red on its byte budget (door grows).
FOR cmake owner: root `CMakeLists.txt` restored from 7b2365ac08c^ (digest 4d986d08... equals the pin); do not delete it again without moving the pin.
