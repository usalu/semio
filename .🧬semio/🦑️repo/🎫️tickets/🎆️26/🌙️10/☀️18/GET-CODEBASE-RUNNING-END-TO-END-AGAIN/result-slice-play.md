# result-slice-play (executor slice-play, wave 1022a)

Goal: `@semio-tech/semio-tech-play:activate-prototype` exits 0, served app boots on :6033, play E2E steps pass.

## Log (newest last)

- 01:38-02:05 Reproduction attempts of `SEMIO_BUILD_GRAPH_POLICY=current bun repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype --dry-run`
  (logs `🗑️generated/slice-play/dry1..7.log`). The post-graph runaway was NOT reached yet: every attempt died earlier on live-edit
  loader faults that peers fixed in parallel within minutes (so these are not mine, but they hide the runaway):
  1. `Authorizer source closure capacity exceeded: files=1025/1024` (authorizers schema maxItems) -> peer raised to 2048 at 01:39.
  2. `Implementation module outside the captured closure: ✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts` (bootstrap `capturedRevisionImportersV1`) -> peer added the importer at 01:47.
  3. `Uncaptured implementation require refused: @iarna/toml(/package.json)` in `🦀️native-manifests/🟨️.mjs` -> peer rewrote the toml reader at 01:50-01:52.
  4. `Uncaptured implementation schema read refused` for `Cargo.toml` of crates inside `🧬️schema/` dirs (closure guard matched every path with a `🧬️schema` segment) -> peer restricted the guard to `.json` at 01:54.
  5. `Exact live original writer proof required` (store format check) -> a peer had `[DEBUG] writerproof` logging in at 01:44; gone later.
- Dependency build lease: process 52904 (child of a `show projects --json` of an e2e tool) held `javascript-dependency-build` EXCLUSIVE > 13 min walking `.cache/b/d/m/j/<gen>/node_modules` (`bun install --frozen-lockfile` + generation receipt verification), blocking every other repo command (my dry7 waited 12+ min on it). Not mine; noted for graph-perf.

## Static finding (unverified, to confirm by the real run)

- `build:graph complete` is advanced at `🏗️build/🕸️graph/🟦️.ts:396` inside the generation fragment builder. What follows is sealing/publishing the generation and the slice load (`loadBuildGraphSliceV1` -> `loadGenerationGraphV1`).
  conc-graph measured compact nodes of ~715 KB each (2.86 GB for 4000 projects); the play task-level slice is 2,839 projects (~2 GB compact JSON). Candidate: serialisation/canonicalisation of those nodes (canonicalJsonV1 of `metadata.semio.moduleGraph` + target input lists per project) is the unbounded memory step.

## 02:55 Finding A: why every repo command now costs 6-40 min (dependency-walk convoy), root cause read from the code

Evidence: `leases status` shows ONE `javascript-dependency-build` EXCLUSIVE holder at a time (pids 52904: 40 min, 60336: 17+ min) while 6-8 other repo commands (my dry7 41 min, slice-demo's dry-run 41 min, graph-perf's show) wait with their 3.5-5 GB capacity grants held idle. Each holder runs the COMPLETE dependency walk (graph-perf phase file `graph-perf-1022/cold.phases.txt`: store-stage 48 s, workspace-edges 90-105 s per scope x6 scopes, installed-paths scan, "Reading source text: 314,048 operations").

Why no command reuses the previous walk: `captureGenerationReceiptV1` (`L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts:136-137`) DEFERS the receipt with `implementation-unsettled` when ANY file of the dependency-subsystem implementation closure (`captureJavascriptDependencyGenerationImplementationV1`, entries = protocol.implementation.entries; ~2,000 files incl. leases and storage) has mtime/ctime newer than `process start - settleMilliseconds`. With 15-20 agents editing those files live, no receipt is ever published, so the next command (new implementation digest anyway) redoes the whole walk; the log line is `Store dependency generation receipt deferred ... implementation-unsettled path=<edited file>`. The pre-existing receipt reads back as `stale class=unreadable`.

Consequence for the fleet: strictly serial, 6-40 min per command. Fix direction (not applied, graph-perf owns the dependency profile): hash the implementation closure from the loader's admitted immutable images (`import.meta.semioCapturedPhysicalSources`: path+sha256 of exactly what ran) instead of live bytes + mtime heuristics, so a young mtime cannot defer the receipt; and/or shrink `protocol.implementation.entries` to the walk logic. Also: a command that merely WAITS for the dependency build should release its machine-admission grant (idle grants of 3.5-5 GB starve the other jobs: 5 QUEUED vs 10 GRANTED, 388 MB spare).

## 02:55 Finding B: the memory root of the old post-graph runaway is already fixed by graph-perf (02:33)
The cargo-lock owner script closure (about 1,900 files) was literally inlined into `inputs` of every native target (12 per crate): node ~715 KB x 3,737 = 2.7 GB compact; graph-perf interned it (38 sampled projects 39.0 MB -> 2.2 MB, projected whole tree 3.8 GB -> 0.24 GB). The play task slice (2,839 projects) held every parsed fragment at once in `assemble` (2 GB JSON -> 6+ GB heap) and then `portableInputs` + `readBuildGraphV1`: that was the RSS 9.8 -> 11.7 GB silence after `build:graph complete`. Needs a real re-run to confirm (blocked on A and on the cold generation: `.cache/b/g` was recreated empty at 01:52, so the first command that gets the builder lease pays a ~1 h cold inference).

## 03:30 Live edits by slice-play (cold-inference loader faults)
- `L/🏗️build/👁️observation/🟨️.mjs`: `observedRequireResolve` and the verify `resolve` fact use a local `createRequire(base).resolve` that calls `node:module.__physicalResolveV1(base, request)` when the captured loader provides it (fallback: real createRequire). Reason: the analysed workspace files (e.g. `⚡️caching/🦀️cargo/📜️script.ts`, `📦️packages/🟦️typescript/🧭️routing/🟦️.ts`) are DATA for the closure walk, but the captured `createRequire` and the global Bun.plugin `onResolve` refused them ("Captured require declaring module required" / "Uncaptured implementation import refused ... imported by <data file>").
- `L/🏗️build/📥️source/🔏️closure/🟦️.ts`: captured module object gains `__physicalResolveV1:(base,specifier)=>physicalResolve(awaitModule.createRequire(base),specifier)` (the peer's `resolving.depth` bypass of the plugin). No law added yet (WRITTEN, exercised only by the live cold run below).
- dry11 (pid 72152, log `🗑️generated/slice-play/dry11.log`, `SEMIO_BUILD_GRAPH_POLICY=current ... run @semio-tech/semio-tech-play:activate-prototype --dry-run`) reached cold inference: `Inferring projects: 422 of 3741` at 03:3x. Leave running (deliberate detached run) until it ends.

## 04:35 dry11 died at 1479/3741 (40 min of cold inference lost) + resilience edit
- Cause: a peer renamed `authorizers/🧪️tests/🧭️computed-imports` -> `🧭️closure-law` (03:28) and left `import "./🧭️computed-imports/🟦️.ts"` in `authorizers/🧪️tests/🟦️.ts:6`; an unresolved import of an analysed command-closure file threw out of the whole graph build. I repointed the import to `🧭️closure-law` (the peer still edits that dir).
- Root fix (live, `L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`): `unresolvedSourceImportV1` wraps the "required import resolves to nothing" throw into `BUILD_SOURCE_UNPARSEABLE` (source = importing file, sha256 of its bytes), the existing withheld-project path (`unavailableProjectV1`, `UNSETTLED_SOURCE_MS` rethrow keeps freshly edited files in settlement). One broken file of a peer now withholds only its own project (with reason) instead of aborting a 1 h cold build. WRITTEN, exercised by the next cold run; no law yet.
- Machine state at 04:35: slice-demo's run (pid 4325, shard 7 exclusive holder, 7 GB RSS) is the current cold builder; 8 more commands (mine = dry12 pid 4821, position 8) queue on `repository-graph-facts:...:shard:7` behind it. Per-project inference records persist, so queued runs should find most projects warm.

## 06:55 Cold v3 generation 29009 failed after 3741/3741 (not mine; I had killed my queued dry12 at 04:55 on the coordinator's order)
- `L/🧪️test/🟨️.mjs`: the two non-literal `import(libraryModule.href)` / `import(new URL(...).href)` of `🕸️graph/📥️inference/🟨️.mjs` (lines ~233, ~328) replaced by `loadInferenceModuleV1()` = literal `import("../📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs")` in a captured closure, URL import otherwise (edited 06:51:12, concurrently with loader-own's fix of the same fault: re-read before touching). Not run.
- Waiting for loader-own's relaunched detached `show targets` to publish the v3 generation; my next step is one `--dry-run` of `@semio-tech/semio-tech-play:activate-prototype` behind it.

## 08:45 Final state of slice-play (goal NOT reached: play activate never got past the cold graph)
Outcome: `activate-prototype --dry-run` did not complete; no real activation, serve or E2E was possible. Nothing of mine is running (dry12 pid 4821 killed 04:55 on the coordinator's order; no detached run handed over).

Changed files (all live, none covered by a new law, none run green yet):
1. `L/🏗️build/👁️observation/🟨️.mjs`: `createRequire` local wrapper -> `node:module.__physicalResolveV1` (resolve-only for analysed workspace files).
2. `L/🏗️build/📥️source/🔏️closure/🟦️.ts`: captured module exports `__physicalResolveV1` (uses the peer's `physicalResolve` plugin bypass).
3. `L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`: `unresolvedSourceImportV1` (unresolved import in a command closure withholds only its project as BUILD_SOURCE_UNPARSEABLE).
4. `L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts:6`: import repointed `🧭️computed-imports` -> `🧭️closure-law`.
5. `L/🧪️test/🟨️.mjs` (L = `🧰️framework/🛍️products/🦑️repo/🔨️modules`): `loadInferenceModuleV1` literal import.

Commands: `SEMIO_BUILD_GRAPH_POLICY=current bun repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype --dry-run` x12 (logs `🗑️generated/slice-play/dry1..12.log`). dry11 (03:3x-04:12) is the only run that reached cold inference (1479/3741, 7-10 GB RSS oscillating, ~28-45 projects/min) and died on the peer's broken test import (fixed, item 3/4).

Findings for the next owner:
- The old post-graph runaway is explained by node size (graph-perf's interning) and `assemble` holding all fragments; NOT re-verified because no run got past inference (v3 generation 29009 failed at 3741/3741 on `🧪️test/🟨️.mjs:181`, fixed by loader-own and item 5).
- Every command serialises on `repository-graph-facts:...:shard:7` (exclusive) and one holder (course typecheck, bazel runs) keeps it 30-60 min; plus machine admission (needs 3.5-5 GB, load 20-30) and the dependency walk convoy (Finding A, graph-perf). With 20 agents the wall clock per repo command is 1-3 h; the cold v3 generation (loader-own pid 35824, log `🗑️generated/loader-own/v3-show.log`) was still queued at position 4/5 at 08:42.
- Next steps once the v3 generation is published: (1) `SEMIO_BUILD_GRAPH_POLICY=current bun repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype --dry-run` and read the slice size/RSS (expect 2,839 projects, tens of MB); (2) real `activate-prototype` (35 lanes, wasm mutex notes in memory `project-play-lane-merge-and-wasm-build-mutex.md`); (3) `bun repo run @semio-tech/semio-tech-play:serve --exclude-dependencies` from the root, then `C/e2e-orch-tools/run.sh play --phase e2e --go`.
