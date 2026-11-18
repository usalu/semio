# R137: a worker becomes ready without reading the whole workspace (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R137/` (`stage-ready.py`, `stage-exchange.py`; staged copies and tests in scratch `R137/stage`). Analysis tools (read-only, static): scratch `R137/static.py`, `closure2.py`.

## 1. Part A: analysis

### 1.1 What `workspace:setup` is

- Project `workspace` = the root `📋️project.json` (name from `package.json`). Target `setup`: executor `repo:script`, command `repo ./📜️script.ts setup`, `cache: false`, inputs `rootFiles` (the four root files plus `sharedGlobals`, no tree glob). `package.json` script `setup` = `repo run workspace:setup`. The ready stage `workspace` (`READY_STAGES_V1`, 4 h limit, `inputs: []`) ran exactly that.
- `setup` is a `tasks` selection (`buildInvocationGraphSelectionV1`): the admission closes the task prerequisites (`closeBuildOwnerAdmissionV1`, `🕸️graph/🚪️admission`). The target's own `dependsOn` has 14 entries, expanded:

| dependency | what it does | projects it touches | needs |
| --- | --- | --- | --- |
| `deps-build-tools` | `repo tools setup all` | none (root files) | duplicate of ready stage `tools` |
| `deps-js-all` | `dependencies sync --all` | none | duplicate of ready stage `dependencies` |
| `deps-python` | `sync python` + `ui-styling-py:deps`, `energy-oracle-py:deps` (side-effecting, host store) | 2 owners | on demand per Python project |
| `deps-cargo`, `deps-go`, `deps-tools`, `deps-trunk`, `deps-wasm-opt` (via `deps-wasm`) | toolchain sync | none | on demand per language |
| `deps-dotnet` | `sync dotnet` + `repo-test-dotnet:deps` | 1 owner | on demand |
| `deps-cpp` | `sync cpp` + `repo-lib:cpp-setup` | 1 owner | on demand |
| `deps-browsers` | `sync browsers` (after `deps-js-all`) | none | on demand (tests that need it say `workspace:deps-browsers`) |
| `setup-git` | local git configuration | none | once per checkout |
| `prepare` (noop) | `framework-schema:generate`, `ui-styling-tokens:generate`, `assets:build`, `framework-graph:generate`, `plugin-registry:generate` (the last after `repo:generator-inputs`, which reads `^default` and `reads` of the build library and writes the plugin registry catalog) | 5 owners plus the wider inputs below | writes into the tree (`mutates-workspace`): the replica copies the result to every machine |
| `repo-mcp:install` | `repo-mcp:build` then publish the native `repo` command | 1 owner plus its dependency inputs (`^default`) | duplicate of ready stage `repo-mcp` |
| `@semio-tech/framework-os-mcp-rs:build` | cargo build, `cache: true`, inputs `production` and `^production` | the Cargo dependency closure of the crate | cacheable: fetch from the fleet |

All 14 are `side-effecting` or uncached except the last, so they run on every worker. Five of them (`repo-mcp:install`, `deps-build-tools`, `deps-js-all`, plus `prepare`'s generated files that the replica copies) are done twice or are replicated by other means.

### 1.2 Why the closure is large, and what is NOT proven

Edges that widen an admission (all in `closeBuildOwnerAdmissionV1`):
1. `^production` / `^default` on a target: `dependencyInputs` of each dependency, transitively. `framework-os-mcp-rs:build` (`production`, `^production`), `repo-mcp:install` (`default`, `^default`) and `repo:generator-inputs` (`default`, `^default`, `reads`) carry it: the full dependency closure of a Rust crate, of the Go command and of the build library (TypeScript imports).
2. Every file a named input matches is admitted through `sourceOwner(file)`; `reads` of `plugin-registry` (`🔌️plugin/**`, `📚️library/**` = 23 owners) and of the Rust crates (`**` of module subtrees) pull whole module trees.
3. Catalog providers: the one provider (`✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts`) has consumer roots over 3412 owners (3285 plugin owners, 106 hub compositions), but admission admits only the provider's own owner when a consumer is admitted, so this edge alone does not add the consumers.
4. `dependsOn` entries with `projects: "*"` are the only edges that name every owner (`test`, `lint`, `build` of the root project); `setup` has none.

What is proven and what is not: (a) the claim "the closure of `workspace:setup` is 4775 owners" comes from worker logs written BEFORE the 15:31 landing of R133, when every scope other than `all` took the generation of the whole workspace (R133 wall 9: a 7-owner `run-many` also admitted 4775). So the logs prove nothing about the closure of `setup`; (b) a static count (manifests, named inputs, Cargo path dependencies and TypeScript imports resolved to owners, no reading through the build) is in progress and is added below if it finishes; (c) no declared edge of the 14 names every owner. Whatever the real number is, it is the union of the dependency closures of the crate, the Go command and the build library, i.e. hundreds to thousands of owners, and every worker pays for it again because none of `setup`'s inputs is shared.

### 1.3 Which tasks need the whole graph

None genuinely needs the whole workspace graph. They split as: (1) no project at all (the `deps-*` toolchain syncs, `setup-git`): they need `package.json`/`bun.lock`/tool tables, which the ready stages `dependencies` and `tools` already cover; (2) one to five owners (`*:deps`, `cpp-setup`, the four generators); (3) a dependency closure (`framework-os-mcp-rs:build`, `repo-mcp:install`, `repo:generator-inputs`): large, but cacheable or replicated.

### 1.4 What a machine must be for development and for the fleet (question 2)

Able to run one target of any one project on demand: the native bootstrap and `repo` binary (stage `native`, `repo-mcp`), the JS dependencies (`dependencies`), the toolchains (`tools`), a graph read that costs only the closure of the requested project (R133 landing 2), and caches that are reachable (remote cache endpoint, fleet facts). The ready stages already establish all of it except the last two, which `workspace:setup` replaced by "read and run everything". What can be done once per fleet and fetched: the generated sources (`prepare`; they are tree files and the replica copies them), the cacheable cargo build (remote cache), and the sealed graph generation (below). What is per machine: toolchains, dependency stores, the `repo` binary.

### 1.5 Generation exchange (question 3)

A mechanism exists and works (`🕸️graph/🕰️generations/🌐️remote-exchange`): `publishGenerationV1` uploads the sealed generation in neutral form (root token, content-addressed documents, a snapshot of the shared files) to the remote cache; the action key is `sha256([domain, registry identity, platform, origin])`, each origin owns one head; `restoreGenerationV1` takes the best head, verifies every document against its name, renumbers the manifest above the local generation and writes the pointer last; the load then verifies the restored generation against the tree by content and recomputes only owners whose facts differ. Ranking (`generationHeadScoreV1`): owners the producer knew stale (or the reader's code differs: all) plus changed paths between the head's snapshot and the reader's tree (`ownersPerChangedPath` 0.25), else the age of the state; up to 3 candidate heads are compared by snapshot distance (`cursors.distance`): that is the restore-nearest-then-delta path. The tree of each machine therefore does not have to equal the head's tree. Existing proof (`remote-exchange/🧪️tests`, two origins, real cache server, 20-owner ring world): "restore without computing, the edited project only, ... never a cold build": 0 owners reinferred on the same tree, exactly the edited project after one edit.

Gaps found:
1. The key has no scope. The registry identity deliberately excludes the owners ("membership change the generation absorbs incrementally"), so the generation of a slice (fewer owners) has the same key as the generation of the workspace: a reader of the whole workspace adopted a 12-owner head in a test (red, see 2.2), and publish/rank compare only stale counts, not membership. Every slice read would also overwrite the head of its origin.
2. The exchange is wired only for the `all` selection (`loadBuildGraphUnpinnedV1` kind `all` uses `generationHostV1`); a selected scope (R133 landing 2) goes through `loadBuildGraphAttemptV1` with its own `selected/` snapshot: it neither restores nor publishes a generation. Slice assembly, seal and publish are R136's.
3. Production proof: R127 lists `restoreRemoteGenerationV1`/upload as not exercised on a worker; the only runs are the tests.

## 2. Part B: what was built

### 2.1 Readiness probes (a)

Schema `Probes` in the ready schema (`🚦️ready/🧬️schema`): target `build`, projects `@semio-tech/norm-din4108` (typescript), `@semio-tech/norm-din4108-rs` (rust), `repo-mcp` (go); export `READY_PROBES_V1`. Stage `workspace` is `run-many --projects=<probes> --targets=build` (limit 1 h, was 4 h `run workspace:setup`). The other stages are unchanged (`repo-mcp`, `dependencies`, `tools` already cover what `setup` duplicated); generators are replicated by the replica, per-language `deps-*` run on demand when a target names them, the cargo build is cacheable. `workspace:setup` stays for humans. Test `🧪️tests/🎯️probes` (4: stage runs the declared probes and not `workspace:setup`; 3 to 8 literal probes of at least 3 kinds; every probe is a registry owner declaring `build`; stage names unchanged): red on the live tree (export missing), 4 pass staged.

Lab run (2 machines, 16 GB/8 cores, the 3 probes, `--dry-run`, load 42): WALL, both machines in `start`: after 280 s of "Reading the project graph slice" the resident service fails with `Maximum call stack size exceeded` (reference acfd801c, service log removed with the lab directory), 0 owners taken, peak footprint 2738 MB. Same phase as R133 wall 11 (source admission, one owner at a time); the stack overflow is new and open (owner of the admission closure; reproduce with the 3 probes). So the probe closure is NOT yet shown to be small or readable.

### 2.2 Scoped generation exchange (b)

Key gains `scope` (`generationExchangeKeyV1(identity, platform, origin, scope = "all")`; "all" yields the former key, so no head is orphaned), note field `scope` (schema), `generationScopeMatchesV1` makes restore answer `foreign` and `ahead` answer false for a head of another scope, `remoteGenerationExchangeV1({scope})`, `generationHostV1(..., compact, scope)` passes it. Test `remote-exchange/🧪️tests/🔭️scope` (4): key law; slice machine B restores A's generation with 0 owners inferred (same tree) and exactly the edited owner (1) after one edit, not cold; a whole-workspace reader (40-owner tree) and a reader of another slice never adopt the 12-owner head; the slice reader still finds its head after the whole-workspace head was written. Live tree: 3 of 4 fail (the whole reader restores the slice head). Staged: 4 pass; existing exchange tests 13 pass and head-rank 5 pass against the staged module. This is the mock-world harness on a real cache server, NOT the slice lab: the lab has no slice generation until R136.

### 2.3 Landing and lab after it

Landed 2026-11-17 16:35:48 to 16:36:40 UTC (lock under 1 min; the windows were lifted at 16:40 by rule, the lock was taken by script after R138 released it), re-merged against R134's caps by anchored edits of the live files; entry appended to `landings.md`. Under the lock: probes (4), scope (4), stage-plan and both standing preflights green; after: ready main test 38 pass 0 fail. Files and hashes: `landings.md` entry "R137 landing".

Second lab run after landing (same 3 probes, 2 machines, load 48): WALL in `start` after 449 s, 0 owners taken, peak footprint 3475 MB: `Unknown selected named source input: scriptProvider:artifact-package:native` (reference f8b20b65, admission `buildSourceContractGroupsV1`: a named input of a probe's closure is not defined). The earlier stack overflow did not recur. So the probe stage is landed but a lab machine does not yet become ready through it: an owner in the probes' closure carries an undefined named input. Fix owner: whoever defines `scriptProvider:*` named inputs (the probe `@semio-tech/norm-din4108-rs` or a dependency); then rerun. The same wall would stop `workspace:setup` on any machine, so it is not caused by the probe choice.
