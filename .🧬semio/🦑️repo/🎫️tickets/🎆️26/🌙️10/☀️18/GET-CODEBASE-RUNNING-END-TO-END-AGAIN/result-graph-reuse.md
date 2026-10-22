# result-graph-reuse (wave 1022a, executor graph-reuse)

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `G` = `L/🏗️build/🕸️graph`, `C` = this ticket. Tools `C/graph-reuse-tools/`, logs `C/🗑️generated/graph-reuse/`.

## Status (newest first)
- 22:08 WIND-DOWN (coordinator, quota). Root cause found, fix LIVE, unit/law-proven. Live two-reader proof NOT done: my warm run (`show targets --json`, pids 57501/57507/57508) waited 11+ min in the machine-admission queue (need 5120 MB, 3 ahead) and was killed by me (child first). No file touched for a proof; nothing of mine is running.

## 1. Why every record was invalidated (from the audit logs + the sealed generation files)
Run1 `refreshing 3840 projects (generation 23)` and run2 (started after run1 published gen 23!) `refreshing 3869 projects (generation 24)` named 62,340 / 70,824 "more stale inputs". The generation fact index of gen 23 (`.cache/b/g/m/generations/x/8a23...`) holds **70,825 `analysis` rows**: the stale count of run2 is exactly the number of analysis rows, i.e. EVERY recorded source analysis was judged stale, in a generation that had just been published by an identical-implementation process.
- (a) legitimate one-time causes in run1 only: declared data `⚡️caching/🔣️policy.json` changed, `target-policy` stamp /2 -> /3 (buildlib-reds bump), dependency receipt `nx@23.2.0.patch` (installer input). Each costs one re-inference, and they were absorbed by gen 23.
- (b) REAL DEFECT: `sourceAnalysisFactHoldsV1` (`L/🏗️build/👁️observation/🟨️.mjs`) fails closed when `sourceAnalyses.get(fact.provider)` is unregistered. The six structural parsers (`command-imports`, `command-references`, `module-imports`, `script-requirements`, `native-script-admission`, `rust-references`) are registered only by `prepareSourceAnalysesV1()`, which the graph host called only on the DISCOVERY path (`G/🟦️.ts` `discoverBuildGraphV1`). The generation reader verifies facts first (`holdsByStat` -> `observationsHold`) in a process that never registered them, so every `analysis` row (59-71k of 437-448k) was reported stale, `plan` returned `delta` for ~all owners, the reader asked for the builder custody (exclusive `repository-graph-facts` + `build-graph-state`) and re-inferred all 3,750 projects. This is why reuse and reader leases both failed regardless of the edit.
- Measured (`C/graph-reuse-tools/hold-probe.mjs`, 142 sampled analysis rows of gen 23, fresh process): holding before `prepareSourceAnalysesV1()` = 0 of 142, after = 142 of 142. With the parsers registered `kinds-probe.mjs` (every 7th non-environment row of gen 23, 54k rows) finds 21 stale rows = 0.04 percent (peer edits since 21:12 and 12 `resolve` facts that name the old dependency-generation store path `.cache/b/d/m/j/<id>/node_modules/...`; the latter is the legitimate one-time cost of a dependency generation change, class (a)).

## 2. Fix (LIVE)
- `G/🟦️.ts` `generationHostV1`: `analysesPrepared()` (memoized: imports the inference module, awaits `libraryBootstrap`, calls `prepareSourceAnalysesV1()`), awaited in `holdsByStat` right before `observationV1.observationsHold(plain, ...)`. Environment rows (`field`/`digest`) do not need it.
- No stamp/format change (no output changed), so no extra cold build.

## 3. Tests
- NEW `G/🧪️tests/🔬️analysis-providers/{🟦️.ts,🧪️child.mjs,🐍️oracle.py}`: process A records a real `analysis` row (`captureObservedSourceInputsV1` + `module` parser) of a temp file; process B (fresh) verifies it: `before:false` (the trap), `after:true` once the parsers are registered; Python hashlib oracle recomputes the recorded source digest. 1 pass.
- `G/🧪️tests/🔓️sealed-reuse/🔣️.json` (+ Python oracle unchanged): new function `generationHostV1` and order law "recorded facts are verified only after the structural source parsers are registered" (`await analysesPrepared();` before `observationV1.observationsHold(plain,64`). 14 pass, oracle `{"failures": []}`.

## 4. Live proof for later (not run)
1. `bun repo leases status | grep -E 'repository-graph-facts|build-graph-state'` must show no exclusive holder.
2. `nice -n 10 bun repo show targets --json > a.json` once (may refresh; with the fix it is a delta of the peer-edited files only, log shows `graph: refreshing N projects` with N small and `Inferring projects: 0 of N`).
3. Back up and append a newline to a repo-library file (e.g. `L/🏗️build/🕸️graph/📥️inference/🏭️generator-outputs/🟨️.mjs`, sha256 `33ce24a402abb44b1ecec83e850e4a450cbca3058939983b17cfa36f63b029a0`, 6044 bytes), start two concurrent `nice -n 10 bun repo show targets --json`, expect `graph: generation N verified, M facts checked` in both, no `refreshing`, no exclusive `repository-graph-facts`/`build-graph-state` holder in `leases status`; restore (remove last byte) and compare the sha.
Expected residue: any peer edit of a file inside a project's import closure (e.g. `process/routing`, `schema/validator`) still stales exactly the projects that import it (by design: source analysis is an input), taking the exclusive builder custody for that delta; a dependency-generation change stales the `resolve` facts that name `.cache/b/d/m/j/<id>/...`.

## 5. Open
- Live proof above. The earlier audit claim "reader leases FAIL" is explained by the same defect (readers needed the builder because everything was stale), not by a lease-order bug.
- Possible follow-up: make `sourceAnalysisFactHoldsV1` report an unregistered provider as an error instead of silently `false`.
