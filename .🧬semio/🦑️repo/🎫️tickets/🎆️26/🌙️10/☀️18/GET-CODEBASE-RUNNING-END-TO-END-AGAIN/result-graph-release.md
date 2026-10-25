# result-graph-release (wave 1023a, executor graph-release)

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `G` = `L/🏗️build/🕸️graph`, `C` = this ticket. Tools `C/graph-release-tools/` (`sample.sh` = ps tree + graph leases every 10 s), logs `C/🗑️generated/graph-release/`.

## Status (newest first)
- 02:05 Fix written LIVE, unit/law tests green; live proof pending (detached run pid 80593 waits in the machine-capacity queue, sampler pid 80679).

## 1. Findings
- The `bun --eval` command child is the process that loads the graph, pins it, plans AND waits for Bazel. Everything it loaded stayed referenced by live frames for the whole build: `baseline` + `prepared.graph` (runtime), `capturedGraph` (a `structuredClone` of the whole graph kept by `withSelectedRustBuildGraphV1`), `encoded.records` (all gz records, `withRepositoryBuildGraphArtifactV1`), `graph` parameters of `runBazelTasksV1` / `runBazelTasksInStorageV1`, the module caches (`expandedClosuresV1` up to 4 M entries, `graphStringInternerV1` 64 M characters).
- Lease: all graph leases are scoped (`withBuilder` / `withBuildGraphStateV1` release in `finally`), so `repository-graph-facts` is released when `loadGenerationGraphV1` returns, before pinning and before Bazel. Live sampling (below) is the proof; the idle holders seen at 01:50 (pid 77841 `show projects`) were graph builders, not a Bazel phase. What stays held during a build is the shared snapshot pin `build-graph:<digest>.json` (by design: nested `repo` calls of Bazel actions read the pinned graph).
- JSC detail that shaped the fix: a collection requested inside the releasing call does not free anything (the caller's stack/registers still reference the content) and `process.memoryUsage().heapUsed` lags until the sweep; collecting after one event-loop turn frees it (65 MB -> 7.5 MB in the law).

## 2. Fix (LIVE, nothing committed)
- `G/🟦️.ts`: `releaseBuildGraphV1(...graphs)` (async): replaces `projects/dependencies/namedInputs/externalNodes/unavailable/admission` of each graph by throwing accessors (every later read throws `Released build graph read: ...`), clears `expandedClosuresV1` and `graphStringInternerV1` (new `clear()` in `L/🏗️build/🧠️memory/🟨️.{mjs,d.mts}`), turns the event loop once, then `collect()`. `requireLiveBuildGraphV1(graph)`.
- `L/🏗️build/🔌️backends/🟦️.ts`: `BackendExecutionContextV1.release: () => Promise<void>`; runtime passes `release:()=>releaseGraph(graph,baseline)` (`L/🏗️build/⌨️command/⚙️runtime/🟦️.ts`).
- Bazel: `runBazelTasksV1(..., onStarted?, planned?)` -> `runBazelTasksInStorageV1(..., planned)`; `await planned?.()` right after the plan, emitted actions and options are computed and before the first Bazel process (`bazel info`); the entry point refuses a released graph; the backend passes `context.release` for a run; `repair` re-reads the pinned snapshot (`withRepositoryBuildGraphPublicationV1`) instead of reusing the released graph.
- Nx backend: `await context.release()` before its child starts (graph only needed for the receipt tasks). Semio engine and watch paths unchanged (they plan from the graph while running).
- `G/🦀️rust/🟦️.ts`: the selection copy `capturedGraph` is released before the backend body runs (both the unchanged and the Rust-prepared branch).
- `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts`: `missing.length=0;encoded.records.clear()` after the snapshot is published.
- No message changed (EN/DE progress lines untouched); the new error is a developer error like the surrounding `Original ...` errors.

## 3. Tests
- NEW `G/🧪️tests/🕊️graph-release/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json,🐍️oracle.py,🧪️child.mjs}`: 18 pass (`bun test ./<file>`): fixture closed under its schema; order laws (plan -> release -> first Bazel process / build / continuous task; builder custody ends before the graph is returned; capture copy and encoded records released before the backend body; Nx releases before its child); presence/absence laws (repair never reuses the released graph; nothing reads `graph` after the planned hook); Python oracle re-checks all laws with plain string search; process-isolated heap law (60 projects: 64.8 MB -> 7.6 MB, reads throw, a surviving project object stays intact, second release is a no-op).
- Unchanged and green: memory 7, sealed-reuse 14, backends 8. `bun build --no-bundle` of every edited .ts/.mjs: ok. (`⌨️command/🧪️tests` needs a parent Script invocation and fails outside `repo`, not related.)
