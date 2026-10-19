# Result scc: the last 3 repo cycles (SCCs) closed

Executor `scc`. Designs of `result-lw10b.md` section 4 implemented as ports and leaf extraction; no lazy-import cycle breaker is left in the three SCCs. No `package.json`/`bun.lock` edit, nothing under the frozen paths, no cargo, no git-modifying command, no compat barrel.

## 1. Totals (dependency-cruiser, config of `result-lay0.md`, repo product, command of `lw10b-tools/cruise.sh`)

| rule | before | after |
|---|---:|---:|
| no-circular | **6** (3 SCCs) | **0** |
| no-cross-package-relative | 1,012 | 1,015 (+3 are a peer's new test `🧪️test/🧪️tests/🔮️oracle-source-ownership`; none from this work) |

Set difference of the violation lists (rule, from, to): the 6 `no-circular` records are the only ones gone, the only new ones are the 3 peer edges above.

## 2. What changed

### 2.1 SCC build selection / caller / child / bazel (CLI closure)
- `🏗️build/🧭️selection/👁️watch/🟦️.ts`: new port type `BuildWatchChildPortV1 {readOwnedBuildInvocationV1, runOwnedBuildChildInvocationV1}`.
- `🧭️selection/🟦️.ts`: `watchBuildInvocationV1(…, options:{initial?;child:BuildWatchChildPortV1})`; the two `import("../📥️caller/…")` calls are gone (selection imports neither caller nor child any more).
- `📜️script.ts` (composition root): both `watch` call sites (`watch` command and the Bazel `resolved.watch` branch) load caller and child and pass `{child:{readOwnedBuildInvocationV1,runOwnedBuildChildInvocationV1}}`.
- `📥️caller/🟦️.ts`: `withBuildOrchestrationTaskEnvironmentV1(root,invocation,phaseControl,execution,args,environment,context,planner,body)` takes the new `BuildOrchestrationPlannerV1` port (type derived from `selectBuildTasksV1`); the lazy `import()` of the Bazel script is gone. `🔧️tools/🏗️bazel/📜️script.ts` passes its own `planBazelTasksV1`.
- Tests re-pointed: `watch/🧪️tests/🎟️phase` (no more import-text replacement for the child ports; ports go through `options.child`), `watch/🧪️tests/📜️script.ts` (2 calls), `🧪️tests/🕸️graph` (watch fixture call), `🕸️graph/🦀️rust/📥️intent/🧪️tests` (5 calls pass `planBazelTasksV1`).

### 2.2 SCC nx graph storage <-> publication transaction
- New leaf `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/📒️journal/🟦️.ts`: the journal read side moved verbatim out of the transaction (`checks` scope, `check`, schema contract, `policy`, `names`, types, `digest`/`identity`/`stamp`, `regular`, `descriptor`, `bytes`, `journalValue`, `journalRead`, `paths`, `allocation`) plus `graphPublicationFileAllocationV1` and `withGraphPublicationChecksV1`.
- `📦️graph/🟦️.ts` (storage `bytes()`): static import of `graphPublicationFileAllocationV1` from the leaf (dynamic import of the transaction removed).
- `🕸️graph/📤️publication/📦️transaction/🟦️.ts`: imports the leaf under the old local names (`import {… as bytes, …}`), so the function bodies are byte-identical and the transaction rollback test, which binds those free names into `applyGraphPublicationTransactionV1`, still holds. `📤️publication/🟦️.ts` (facade) takes `graphPublicationFileAllocationV1` from the leaf.

### 2.3 SCC nx plugin root <-> executor
- New leaf `⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🟨️.mjs` (+`.d.mts`): implementation identity moved out of the root (`nxBuildDependencySourceIdentityV1`, `nxEngineNamedInputV1`); the identity now also hashes the root, itself and the graph-pin leaf (projection list = root, implementation, graph-pin, pin.json, engine.json).
- New leaf `🔌️nx-plugin/📌️graph-pin/🟨️.mjs` (+`.d.mts`): `graphPublication` (exported as `withNxBuildGraphPublicationV1`) and `readPrivateNxBuildGraphV1(workspaceRoot, environment, implementation)`; the identity check is an injected port, the leaf imports neither root nor executor.
- Root `📚️library/🟨️.mjs`: `nxBuildDependencyImplementationV1 = executor identity + leaf identity`; `graphPublication` is a thin binding of the leaf with that check; `readPrivateNxBuildGraphV1` removed from root and `🟨️.d.mts`.
- Executor `📤️arguments/🟨️.mjs`: static imports of the two leaves; reads the graph with `()=>{nxTaskArgumentsSourceIdentityV1();nxBuildDependencySourceIdentityV1();}` (same two checks as before, no lazy root import).
- Fixtures/tests re-pointed: `🧪️tests/🧫️fixtures/🕸️graph/🔣️.json` (projection list), `…/📥️required/🔣️.json` (cold-import row now loads the graph-pin leaf) and `…/📥️required/🟦️.ts` (reads `graphPublication` from the leaf).

## 3. Verification (exact commands, outcomes)
- `bun repo --help`: usage printed, stage `complete` (after the final edit).
- `bun test ./…/🕸️dependencies/🧭️direction/🔗️relative-closure/🧪️tests/🟦️.ts`: 2 pass, 20 expects (the `🔗️relative-closure` law; the new leaves import only relative specifiers).
- tsc (`scc-tools/tsconfig.json`, 19 files: all touched `.ts`, the three tests, both new `.d.mts`, the root `.d.mts`): 0 errors. It found two real errors on the first run (a second `watchBuildInvocationV1` call in the Bazel branch of `📜️script.ts`, a binding name in the phase test), both fixed.
- dependency-cruiser, repo product, before and after: table in section 1.
- `SEMIO_TEST_ARTIFACT_DIR=… bun test ./…/🏗️build/🧪️tests/🕸️graph/🟦️.ts -t "private Nx implementation authority"`: 1 pass, 37 expects. This is the identity law: oracle recomputation in `node` over the new projection list, and every projection/private-task file mutation throws "Source changed"; it covers the new leaves.
- `bun scc-tools/tx-smoke.ts <dir>` (the shared-transaction scenario with a stubbed lease directory over the transpiled transaction + leaf): publish, snapshot read equals input, republish, recover and `graphPublicationFileAllocationV1` OK.
- Runtime import under node, bun and `createRequire` of root, executor and graph-pin leaf: loads; root exports no longer contain `readPrivateNxBuildGraphV1`; `nxBuildDependencyImplementationV1()` returns an identity.
- NOT verified by execution: the harness-bound suites (`Original parent Script process invocation required` / `Build test receiver admission closed` under plain `bun test`, the same blocked state as lw10b): watch phase law, watch script test, the `watchBuildInvocationV1` graph fixture, the 5 orchestration intent calls (also gated by `SEMIO_TEST_ORCHESTRATION_BACKEND`), the nx graph and cold-import suites. They are verified by tsc and module loading only.

## 4. Notes for the next executor
- The engine named-input closure (`nxBuildEngineInputsV1`) starts at the root and follows static and dynamic imports, so the two new `.mjs` leaves enter the Nx cache inputs automatically; no list had to change.
- `watchBuildInvocationV1` now loads caller and child eagerly in the composition root when `watch` runs (previously only when the callback was an owned repo command); both are cheap and caller was already loaded.
- Frozen paths were not needed by any of the three designs.

## Tools kept (`scc-tools/`)
`cruise.sh`, `quick.sh`, `cycles.ts`, `scc.ts` (copies of lw10b's), `tsconfig.json`, `tx-smoke.ts`, `ctx.py`, `files.py`. Generated output (`🗑️generated/scc`: cruise JSON, tsc and test logs) deleted.
