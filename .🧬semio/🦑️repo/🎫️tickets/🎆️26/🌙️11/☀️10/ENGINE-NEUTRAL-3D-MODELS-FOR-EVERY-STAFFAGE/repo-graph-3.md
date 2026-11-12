# repo graph reading 3: static pre-flight, root fixes, throughput and resume, full graph, `repo run`

Result in one line: `repo show projects` now completes (exit 0, 8 215 s, 5 079 projects, generation 0 published). `repo run @semio-tech/framework-3d-js:test` does NOT complete: this macOS machine has no Node.js, and every `repo run` needs it (Nx). The staffage, React, site and `export-models` targets were therefore not run. Details and the exact approval needed are in section 6.

Machine: macOS arm64, 12 cores, 16 GB, other agents active during the diagnostics. Bun 1.4.2 (the repo's pinned runtime, `~/Library/Caches/semio/runtimes/bun/9098.../bun`). Tool output only under `🗑️generated/repo-graph-3/` (13 MB; logs of every attempt are there).

## 1. The check that raised the blocker

`Workspace import is not resolved by current package metadata: <specifier> from <file>` is thrown by `workspacePackageModuleV1` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`. "Current package metadata" is: TypeScript NodeNext (then Bundler) resolution of the specifier through the `exports` map of the workspace package whose `package.json` name equals the specifier's package, reached via the package's `node_modules` link; an `exports` target that does not exist counts as resolved only when a declared producer promises it (`📋️project.json` `metadata.semio.artifactPackage` for `dist/`; `wasmPackageSourceV1.compileOutputs` for wasm `pkg/` and bindings; declared ignored `outputs` roots, but only for relative imports of absolute paths). It is not a generated index.

## 2. Static pre-flight (no graph build)

`repo-graph-3-static-check.mjs` in this ticket folder (run with the repo's bun from the workspace root: `bun <ticket>/repo-graph-3-static-check.mjs out.json`; 20 s without produced-output awareness, about 4 min with it). It imports the tool's own `workspacePackageModuleV1` and `readBuildProducedCompileOutputV1` with the real generator contracts and owner registry (so declared wasm and generated outputs resolve exactly as in the graph) and runs them over every import specifier of 23 393 source files (tracked or untracked, not ignored) against 325 workspace packages. It also checks all 1 560 `📋️project.json` files outside ticket folders: JSON parse, name, duplicate names, project JSON schema (Ajv, the schema's cross-file `$ref` rebased), `metadata.semio.cache` against the closed `💾️cache.json` schema, `options.cwd`, `$schema`, `sourceRoot`, `artifactPackage` producer admission, and the tool's own owner registry consistency law (`checkBuildOwnerRegistry`). Helper `repo-graph-3-exports-check.py` lists `exports` targets missing on disk (187 are build outputs of producers, 3 are declared wasm bindings, none else).

Complete findings before any fix (`static.json`) and after the fixes (`static-3.json`, final):

| class | before | after | note |
|---|---|---|---|
| workspace imports unresolved | 9 | 4 | the 4 wasm ones resolve once produced outputs count; `shell/generated` fixed; 4 left, see below |
| invalid cache decisions | 0 | 0 | yesterday's `writes-build-output` is gone |
| project parse / schema / duplicate names / missing referenced files / producer admission | 0 | 0 | the 26 parse, 18 duplicate and 42 missing-file hits of a first run were all inside `.🧬semio` ticket copies, which the graph ignores; excluded |
| owner registry consistency | 26 | 28 | `@semio-tech/value-retained-clone-protocol-rs` and `semio-framework-value-native-leaf` rows whose root is gone, and manifests without a row; all in-flight work of others (`🌱️value`, `🧵️job`, plugin norm/energy/os plugin); count grows while others edit; not touched, the graph adopts rows lazily (`adoptPendingBuildOwnersV1`) and the full run did not stop on them |

The 4 remaining unresolved imports are test or story sources of other people's work in progress and are not in a command walk, so the graph does not stop on them: `@semio-tech/writer-writer` from the writer sqlite test (package exports `dist/` but has no source and no `artifactPackage`), `@semio-tech/trinity-jack-lsp` (`wasmPackageSourceV1` without `compileOutputs`), `@semio-tech/framework-os-dev/parity/execution` from the stdio acceptance test (no such `exports` entry), `@semio-tech/presentation-react/🎨️.css` from a story (TypeScript cannot resolve a `.css` export as a module). Left to their owners.

Two known gaps of the static check, stated plainly: it cannot tell which files are reached by a command walk (it reports the superset), and it does not run the compile-closure path of owner admission (`repo run` selection), which still throws a plain error on an unresolved import of a selected owner.

## 3. Fixes (root cause)

### `@semio-tech/framework/shell/generated`
Root cause: the `exports` entry of `@semio-tech/framework` pointed to `./🐚️shell/🤖️generated/🟦️.ts` inside the package folder, but the generator (`@semio-tech/framework-os-shell-rs:typegen`, declared output root `🧰️framework/🔨️modules/🐚️shell/🤖️generated`, consumed by `🔨️modules/🐚️shell/🟦️.ts` as `./🤖️generated/🟦️.js`) renders beside the reducer, and `🤖️generated` is gitignored build output. The export could never resolve. Fix in package metadata, no source of others edited: new authored wrapper `🐚️shell/🪞️generated/🟦️.ts` (`export * from` the declared ignored output, the same pattern as the existing `./shell` wrapper; an `exports` map cannot reach out of the package root) and the entry `./shell/generated` now points to it. The graph treats the wrapper's relative import of the declared output root as a produced output.

### Hard errors become tool behaviour, not blockers (repo product)
- `workspacePackageModuleV1` marks the error `unresolved: true`; `walkImportsV1` (command and runtime-reference imports) lets an unresolved workspace import fall through to the existing unresolved-import path, so one broken file no longer aborts the whole graph with a plain `Error`. Test `🧭️script-inputs/🧪️tests/unresolved-workspace-import.test.ts` (2 tests; the walk test failed with `MODULE_NOT_FOUND` before the change, 27/27 of the folder pass).

## 4. Throughput, memory and resume (measured)

Root causes found by timing the real hit path (`[DEBUG]` timers in the lookup and settle path, removed afterwards; the sealed graph child cannot be given `--cpu-prof`, a profile of an isolated `os-dev` inference gave 28 s CPU, 34 % `readFileSync`):

1. A resumed project record is not free: every hit settles about 11 closure entries per project (each reading, decompressing and validating a stored entry that references on average 735 row layers). Measured 0.2 to 7 s per hit.
2. The cause of the worst part: `createMemoryGovernorV1().pressure()` was true whenever the headroom was below the largest LEARNED owner peak. The learned peaks were poisoned again (`language:cargo` 5.86 GB, rust 2.15 GB, project 1.17 GB: the first owner of every run pays the one-time loading of the shared closures, and the maximum is persisted forever) while the headroom was 4.4 to 4.9 GB, so after EVERY project `releaseOwnerStateV1` flushed the walk nodes, source memos and half of the closure entries, and the next project reloaded and revalidated them. Fix: `pressure()` is now true only when the headroom crosses the declared host floor (`hostAvailableFloorMegabytes`, 1 536 MB); admission still uses the learned peaks. Test: the pressure test in `🧠️memory/🧪️tests/🟦️.ts` (19/19 pass) states the new law, including "a poisoned peak far above the headroom does not report pressure".
   - Measured on the resume (hit) path, same 100 s window, other agents running: before 56 projects (34/min) and 73 in 217 s; after 98, 107 and 101 projects in 90 to 100 s (about 70/min). During the full run the hit region ran at 25 to 90/min.
3. Resume ("three runs each started from scratch"): the per-project records DO persist (`np/<sha>.json` plus `.records` packs; 1 321 projects were on disk from the earlier runs and were reused, only about 20 of the first 73 were recomputed in each cancelled probe, and each cancelled probe left its new records). What looked like starting again is the progress counter: it starts at 0 and counts hits, and each hit cost seconds (items 1 and 2), so 1 300 resumed projects took 20 to 40 minutes. A real danger for the next runs was found and fixed: the durable closure store (`.cache/b/g/m/inference/selections/*/ce`) already held 15 020 entries and 63 158 layers (4.6 GB) after 1 321 projects (about 11.4 entries and 47.8 layers per project), against maxima of 32 768 entries and 65 536 layers; the first save of a process removes the oldest records beyond the maximum, which would delete layers that project records still name and turn resumed projects into misses. The maxima in `🕸️graph/📥️inference/🧬️schema/🔣️.json` are now 131 072 entries and 262 144 layers. Test `closure-store-limits.test.ts`: the maxima cover 4 754 projects at the measured entries and layers per project, and a later process within its maxima keeps every earlier record while a smaller maximum removes the oldest.
4. Not changed, found: the learned owner peak is a persisted monotonic maximum; a decay or growth cap was tried in the governor and reverted because the language-agnostic scenario corpus, the wrong-first-guess correction and the other-process-noise tests pin the max rule. Before the full run the stale `np/owner-peaks.json` was deleted (cache data). The worker farm cannot help on this machine: with `coldInference` forced (it is read and never set anywhere) the farm asked the host for 11 workers and waited with `not enough free memory (need 3584 MB; -2562 MB spare above the 2048 MB floor)` because the parent alone holds 8 to 12 GB; the debug switch was reverted. Cold per-project cost is dominated by a few giant projects (see section 5).

## 5. Every attempt

| attempt | what | outcome |
|---|---|---|
| earlier agents | runs 1 to 3 (see `repo-graph.md`, `repo-graph-2.md`) | died at 451 and 1 360 projects (`writes-build-output` refusal; `@semio-tech/framework/shell/generated`) |
| `sync-1.log` | `repo dependencies sync --all` once, because `@semio-tech/framework`'s `package.json` (a workspace manifest) was edited at 01:24; root `package.json` and `bun.lock` unchanged since 21:48 yesterday | exit 0, 103 s, 4.06 GB max RSS, kept 30 generations |
| `profile-1`, `profile-2` | bounded diagnostics, `--cpu-prof` through `BUN_OPTIONS` (the sealed child writes no profile) | cancelled by SIGINT after 217 s (73 projects, 9.3 GB RSS) and about 150 s (82 projects) |
| `probe-1` to `probe-6` | bounded 90 to 150 s runs with `[DEBUG]` timers and the fixes of section 4 one by one (probe-6: forced farm) | cancelled by SIGINT as planned; all `[DEBUG]` code removed (`grep` over the build folder finds only the pre-existing test file `🏡️resident/🧪️tests/🌐️host/🖥️host-main.mjs`) |
| `full-1` | ONE full `repo show projects`, 02:23:50 to 04:40:45, started when `dwg-wasm.md` and `silhouette-15.md` existed and no repo/cargo/vitest process ran (`python-manim.md` was written at about 03:10, during the run; only a single-threaded python test loop of that agent was still in flight at the start) | exit 0. `graph: generation 0 published, 5079 projects refreshed in 8207 s`, 688 245 facts, 8 215 s real, 3 205 s user, 2 084 s sys, max RSS 11.93 GB, 5 073 output lines. 6 projects withheld because a source of theirs does not parse (other people's work: `@semio-tech/cad-js`, `@semio-tech/s-cad-composition-rs`, `@semio-tech/s-spatial-kernel-semio-session-rs` and 3 generated `test-s-plugins-*` projects); `@semio-tech/framework-os-shell*` are not withheld |

Shape of the full run (sampled every minute in `full-1.watch`): the 1 278 resumed projects took 55 minutes (hits at 25 to 90/min, interrupted by a few giant projects), the 3 400 recomputed ones the rest. The giant projects dominate: `os-hub` 770 s, `@semio-tech/stdio-plugin` 525 s, root `workspace` (268 821 rows) 740 s and `@semio-tech/framework-os-dev` over 3 430 s (about one hour at 100 % of the main thread while everything else waited; 28 s in isolation, so the cost is in the in-process late closure resolution, not in the project itself). That is the next target, not done: a JS-level profile of the sealed child (the build tool does not forward `--cpu-prof`).

## 6. `repo run` results (verbatim)

All three attempts at `repo run @semio-tech/framework-3d-js:test` (`run-3d-js-test*.log`):

1. Plain environment, 04:41:34 to 04:42:37, exit 1:
   `Internal error: Executable not found in $PATH: "node"` then `repository:command failed with exit code 1`. Cause: this machine has no `node` anywhere on `PATH` (`which node` finds nothing). `repo run` executes through Nx and its authorizer preflight spawns `node --input-type=module --eval ...` (`⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📥️loading/🟦️.ts`); the repo's bootstrap script installs Node.js 24.15.0 from a checksum-verified download (README line 738; pin in `🔩️native/🥾️bootstrap/🧬️schema/🔣️.json`: `https://nodejs.org/dist/v24.15.0/node-v24.15.0-darwin-arm64.tar.xz`, sha256 `af5cfaeafe603aaf7599f287fd9d100bb41f16794f49788fa59dd3f25546930f`). It was not run on this machine.
2. `PATH` shim `node` -> the pinned bun, 04:45:30 to 04:46:03, exit 1: `SyntaxError: JSON Parse error: Unexpected identifier "undefined"` in the authorizer probe. Bun cannot stand in for Node there.
3. `PATH` shim `node` -> `/Applications/ChatGPT.app/Contents/Resources/cua_node/bin/node` (v24.22.1, only as a diagnostic; the same version exists in the Cursor helper, not used because IDEs are off limits), 04:48:15 to 04:49:27, exit 1: the Nx run reached `nx run @semio-tech/framework-3d-js:test:prototype` (`Run duration: 3.9s`), then vitest failed to start (`Startup Error ... Cannot find module @rollup/rollup-darwin-arm64 ... dlopen ... code signature ... not valid`: that app's Node enforces library validation and refuses the repo's rollup native module), `error: node exited with status 1`, `OwnedCommandFailure: process:owner-command failed (1)`, ` NX   Running target test for project @semio-tech/framework-3d-js failed`, final `Internal error: Owned Nx invocation failed (1): nx run @semio-tech/framework-3d-js:test --parallel=8`, exit 1. Pass/fail/skip counts: none, no test ran.

Not run (same blocker): `repo run @teaching/staffage:test`, `repo run @teaching/staffage-react:test`, `repo run @teaching/staffage-site:test` (project and target names found with `repo show project`; each has a target `test`), `repo run @teaching/staffage:export-models`. The direct vitest results in `final-integration.md` therefore stay the only evidence; the question "does a target fail where direct vitest passes" cannot be answered until a pinned Node.js is on the machine.

Approval needed from the user (nothing was downloaded): download Node.js 24.15.0 darwin-arm64 (`node-v24.15.0-darwin-arm64.tar.xz`, about 45 MB, sha256 above) through the repo's own bootstrap, or install any Node.js 24 that does not enforce library validation. After that `repo run` can be tried directly; the graph is warm (generation 0 published, hits fast).

## 7. Files touched

Repo product (`🧰️framework/🛍️products/🦑️repo/`):
- `🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs` (unresolved marker and fall-through; restored without debug code)
- `🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/unresolved-workspace-import.test.ts` (new)
- `🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/closure-store-limits.test.ts` (new)
- `🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧬️schema/🔣️.json` (`closureStoreMaximumEntries` 131 072, `closureStoreMaximumLayers` 262 144)
- `🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs` and `🧠️memory/🧪️tests/🟦️.ts` (`pressure()` and its test)
- unchanged after temporary instrumentation: `🏗️build/👁️observation/🟨️.mjs`, `📥️inference/📦️project-nodes/🟨️.mjs` (restored from backup, byte-identical to before)

Outside the repo product:
- `🧰️framework/📦️packages/🟦️typescript/package.json` (one `exports` line: `./shell/generated`)
- `🧰️framework/📦️packages/🟦️typescript/🐚️shell/🪞️generated/🟦️.ts` (new)

Ticket helpers: `repo-graph-3-static-check.mjs`, `repo-graph-3-exports-check.py`, `repo-graph-3-run.sh`, `repo-graph-3-runtarget.sh`, `repo-graph-3-watch.sh`, `repo-graph-3-store-stats.mjs`, `repo-graph-3-profile-summary.py`. Cache data removed: `np/owner-peaks.json` (twice, poisoned learned peaks).

## 8. Open

- Giant projects (`framework-os-dev` over 1 h, `os-hub`, `workspace`) dominate a cold graph; needs a JS profile of the sealed child and a look at late closure resolution (`closureAnswerV1` re-expands provider closures per query for entries with shared references: `stable` is false for them, so their answers are never memoized).
- Learned owner peaks still ratchet on the first owner of each run (one-time shared load charged to it).
- A failing project still makes the graph wait for the running ones (`await Promise.allSettled(workers)`).
- Owner registry drift (28 findings) from others' in-flight work.
- Node.js is missing (section 6); the repo's bootstrap would install the pinned one after approval.
