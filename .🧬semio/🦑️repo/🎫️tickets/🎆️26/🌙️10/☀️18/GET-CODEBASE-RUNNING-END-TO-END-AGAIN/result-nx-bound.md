# Result nx-bound: Nx adapter boundary, cache location truth, neutral parity (WP-N, WP-C, G4 G6 G8 G9)

Abbreviations: `L/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `B/` = `L/🏗️build/`, `BS/` = `L/⚡️caching/🚀️bootstrap/`, `NX/` = `BS/🛠️tools/` (the Nx implementation module), `C/` = this ticket.

No builds, no `bun repo` runs, no cargo, no git-modifying command, no sub-agent. The Nx cache verification was NOT run (see "Not run").

## 1. G4: the "dead path" premise is wrong, nothing to fix in `nx.json`

The audit and the brief say `nx.json:91` is dead and that Nx artifacts should live in `.cache/b/n`. Source and tests prove otherwise:

- `nx.json` `cacheDirectory` = `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts` is the authored contract of the **rust** partition. `readAuthoredNxCacheDirectoryV1` (`L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts`) refuses any other value, and `repositoryCacheBudgetV1` throws when the rust partition directory disagrees with `strictStorage.directory` (`rust/fixed`).
- The runtime `NX_CACHE_DIRECTORY` is `join(<rust image root>, "nx/artifacts")`: `withRepositoryNxStorageV1` opens partition `"rust"` and calls `boundedNxNamespaceV1(image.root, graph.root, …)`; `graph.root` (partition `n`, `.cache/b/n`) only receives the graph snapshots and the funded native namespaces (`nx/graphs/**`). `result-nxpart.md` moved only that graph pool, not the artifact cache.
- The audit's statement "NX_CACHE_DIRECTORY → `.cache/b/n/m/nx/artifacts`" is not true. The rust partition is mounted (`mount` shows `…/⚡️cache/rust/fixed/m`), its `m/` has no `nx/` folder yet, because Nx never reached task execution; the namespace folder is created by `withRepositoryNxStorageV1` (`mkdirSync` of `NX_CACHE_DIRECTORY`) on the first run.
- Moving artifacts into `n` would be harmful: `n` is 1.5 GiB with a 512 MiB artifact charge, the rust partition is 16 GiB with 1 GiB per artifact and also holds the Rust unit outputs that Nx caches (`withOutputCustodyV1` classifies `rust` vs `products` paths).

Evidence run: `SEMIO_TEST_ARTIFACT_DIR=<dir> bun test ./L/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🟦️.ts -t "authored Nx default"` -> 1 pass (nx.json equals `<rust directory>/m/nx/artifacts`). New: `NX/⚙️operations/🧪️tests` asserts `nxCacheDirectoryV1(root) === join(root, nx.json.cacheDirectory)`.
Result: `nx.json` unchanged. "Used" can only be proven by a run (script below, not run).

## 2. G6: Nx adapter boundary

New Nx SDK port `NX/🔌️sdk/` (`🟦️.ts`, `🔣️policy.json`, `🧬️schema/🔣️.json`): `openNxSdkV1(base)` returns a frozen set of typed doors (`packageDirectory`, `cli`, `workspaceRoot`, `readCachedProjectGraph`, `readSourceMapsCache`, `authorizers`, `daemonClient`, `daemonEnvironment`, `native`, `socketTransport`, `socketFraming`, `taskPlanning`). Every module name is checked against the closed catalogue in `🔣️policy.json` (13 modules); lookups use a variable name so the frozen authorizer compiler does not see new literal `require`s. `nativeSelectedTaskPlannerV1` moved into the port (it is Nx task-graph code).

Re-pointed sites (behaviour unchanged, same call order):
| Site | Before | After |
|---|---|---|
| `L/⚡️caching/📇️inventory/🟦️.ts` | 3 direct Nx requires | `sdk.workspaceRoot()`, `readCachedProjectGraph()`, `readSourceMapsCache()` |
| `L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🔌️sdk/🟦️.ts` | 5 requires (workspace-context x2, client, native, daemon-environment) | `sdk.authorizers()`, `daemonClient()`, `native()`, `daemonEnvironment()` |
| `…/🎯️selection/📥️scope/🔌️sdk/🟦️.ts` | create-task-graph + task-hasher | file deleted, function moved to `NX/🔌️sdk/🟦️.ts`; test import and `NX/🔏️authorizers/📦️runtime/🟦️.ts` export re-pointed |
| `…/📥️journal/👷️worker/📜️script.ts` | `require.resolve("nx/package.json")`, native, consume-messages, socket-utils | `sdk.packageDirectory()`, `native()`, `socketTransport()`, `socketFraming()` |
| `L/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts` (`cache-verify`) | wrote `nx.json`/`project.json`, spawned `nx/bin/nx.js` | neutral probe spec `🔁️verification/🔣️probe.json` (+ `🧬️schema/🔣️.json`) and the tool port `CacheProbeToolV1`; Nx side `NX/🔁️probe/🟦️.ts` (`nxCacheProbeV1`); non-Nx tools get "No cache probe implements build tool X" |

Boundary law (language-agnostic, with a third-party cross-check): `NX/🔌️sdk/🧪️tests/🟦️.ts` scans all 1,216 production files under `🦑️repo/🔨️modules` with an own tokenizing scanner and the TypeScript compiler AST as oracle; both must agree on 13 fixture cases (`🧫️fixtures/🔣️.json`) and on every real file; offenders outside `BS/` must be `[]`. A separate Python scan over `🧰️framework`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`, `♻️mit-bestand`, `.🧬semio/🦑️repo` found production Nx loads only in `BS/📜️script.ts`, `BS/🔌️nx-plugin/**` and `NX/📜️script.ts` (adapter). Left in place on purpose: `L/⚡️caching/🕸️graph/🔏️seal/🟦️.ts:68` only hashes the files `node_modules/nx/package.json` etc. (no module load).

Frozen authorizer closure still builds: `C/nx-cache-tools/compile-authorizers.ts` (real `captureNxAuthorizerSourcesV1` + `compileNxAuthorizerSnapshotV1`, no cache partition touched) -> 248 sources, 792,598 bytes, includes the port and its policy. The source digest changed, so the next `bun repo tools setup nx` recompiles the authorizers (expected).

## 3. G8/G9: neutral parity and cache operation

New neutral module `B/⚙️operations/` (`🧬️schema/🔣️.json`, `🔣️catalog.json`, `🟦️.ts`, `🧪️tests`): operation catalogue with per-tool state (`native|adapter|neutral|missing`), dry-run/progress/cancel flags, result schemas `TaskGraph`, `DryRunPlan`, `CacheStatus`, `CacheClear`; tool ports `BuildCacheToolV1` and `CacheProbeToolV1`; `runBuildCacheOperationV1`; `buildExecPlanV1`.

| Operation | Existed in neutral layer | Nx side now |
|---|---|---|
| `affected` | yes (git rule inside `selectBuildTasksV1`) | rule extracted to `buildChangedFilesCommandsV1` (`B/🧭️selection/🟦️.ts`, shared by Bazel and the test harness). New `nxOperationArgumentsV1`/`nxAffectedArgumentsV1` (`NX/⚙️operations/🟦️.ts`): no selector -> `--base=HEAD` (was `defaultBase` `⛳wip`), `--uncommitted --untracked` -> `--base=HEAD`, `--all` -> `run-many`, `--projects`/`-p` refused (Nx `affected` has no such option; before it leaked to the executor). Used for both the runtime launch and the declared bootstrap launches (`B/📜️script.ts`, `B/📥️caller/📤️child/🟦️.ts`). |
| `exec` | yes | `exec --dry-run` under Nx used to map to `--graph=stdout` while `nx exec` still ran the command; now prints the neutral `ExecPlan` (`project:repo-exec` tasks) and runs nothing |
| `--dry-run` | yes | Nx keeps `--graph=stdout`; contract is the `DryRunPlan` schema (Nx `createTaskGraph` output validates) |
| `cache status|clear` | no | new: `bun repo [--tool=nx] cache status|clear` (help line, `ensureBuildEnvironmentV1` skipped like `reset`, dispatch in `B/📜️script.ts`). Nx: status walks `<rust partition>/m/nx/artifacts` (links not followed, `checkResourceLeaseControl` per directory, progress every 64 directories); clear runs `nx reset --onlyCache` inside the bounded namespace; result = status before/after (`CacheClear`). |

### Spec for the Bazel side (bzl-act)
1. `cache`: implement `BuildCacheToolV1` (`B/⚙️operations/🟦️.ts`): `status(context)` -> `CacheStatus` (disk cache / output base: directory, file count, bytes, `available` false when absent), `clear(context)` removes Bazel's action cache entries only (no `--expunge`, toolchains stay), call `context.progress("measuring"|"clearing")` regularly and `checkResourceLeaseControl(context.phaseControl)` per directory. Replace the `throw` for `bazel` in `buildCacheToolV1` with a dynamic import of the adapter and set `cache.implementations.bazel` to `adapter` in `B/⚙️operations/🔣️catalog.json` (the test `every neutral operation has a tool state…` fails if the catalogue still says `missing` while the call works, and the reverse).
2. `--dry-run`/`--graph` for `run`, `run-many`, `affected`: print `{"tasks":{"roots":[ids],"tasks":{id:{"id","target":{"project","target","configuration"?}}},"dependencies":{id:[ids]}}}` (`DryRunPlan`). The current `BazelTaskPlanV1` (`{version,tasks:Record,selected}`) may stay as an extra key, but `tasks` must be the TaskGraph.
3. `exec`: dry-run must print `buildExecPlanV1(projects)` (same ids as the Nx path); execution runs once per selected project with cwd = project root and `NX_PROJECT_NAME` / `NX_PROJECT_ROOT_PATH` exported (Nx exports these natively; catalogue note).
4. `affected`: already shares the git rule. Nx refuses `--projects`; decide whether Bazel also refuses (recommended, one rule in `validateBuildTaskArgumentsV1`) or the catalogue documents narrowing as a tool difference.
5. `cache-verify`: provide `CacheProbeToolV1` for Bazel (render a Bazel workspace from `L/⚡️caching/🔁️verification/🔣️probe.json`, invocation of one task); until then `SEMIO_BUILD_TOOL=bazel cache-verify` fails with a clear message.

## 4. Verification (exact)

All from `/Users/ueli/Documents/semio`, `bun test ./<path>`:
- `NX/🔌️sdk/🧪️tests/🟦️.ts` 4 pass (schema, door parity with Node resolution, scanner agrees with TS AST on all cases, zero offenders).
- `NX/🔁️probe/🧪️tests/🟦️.ts` 3 pass (rendered workspace equals the previous inline fixture; accepted by Nx's own `nx-schema.json` and `project-schema.json` via Ajv; invocation strips ambient task env).
- `NX/⚙️operations/🧪️tests/🟦️.ts` 5 pass (translation corpus; every flag declared by installed Nx `affected`/`reset` yargs grammar; installed Nx `parseFiles` yields the same changed files as the neutral git rule in a temp repo for 7 scenarios; cache tree status/cancel; nx.json default equals the cache directory).
- `B/⚙️operations/🧪️tests/🟦️.ts` 4 pass (catalogue vs schema vs CLI help text; honest `missing`; results incl. Nx `createTaskGraph` output against `DryRunPlan`; cache argument validation and clear arithmetic).
- `B/🧭️selection/🧪️tests/🟦️.ts` 6 pass (harness list extended with `buildChangedFilesCommandsV1`), `B/🧭️invocation/🧪️tests/🟦️.ts` 2 pass. Combined run of the 6 touched suites: 25 pass, 0 fail.
- `bun build --target=bun 📜️script.ts` -> bundled 2,472 modules; `bun build` of every edited file resolves; `bun repo --help` prints the usage line including `cache status|clear`.
- `bun C/nx-cache-tools/compile-authorizers.ts` -> see section 2.

Failing but not caused by this work: `B/📥️caller/🧪️tests/📥️birth` 2 of 4 need the real process envelope ("Original parent Script process invocation required"), same class as before.

## 5. Not run

- `C/nx-cache-tools/verify-nx-cache.ts [<project:target>]` (default `@teaching/architecture-course:test`, `cache: true`): forced-miss run with `--skip-cache` that still stores, `cache status` must show growth, an identical run must print the Nx cache-hit marker, final status. NOT run: `ps` shows `bun repo dependencies lock --all` running. The script refuses to start while such a process exists or `node_modules` does not resolve; logs go to `C/🗑️generated/nx-cache-tools/` (delete after reading).
- Real `bun repo cache status|clear`, `affected --dry-run`, `exec --dry-run` under Nx, `cache-verify`: not run (same reason; they need the dependency/tooling bootstrap).
- Taxonomy conformance for the new directories (`🔌️sdk`, `🔁️probe`, `⚙️operations` x2, `🔁️verification/🧬️schema`): names follow the existing emoji+slug convention, sibling emoji are unique, no banned stems; the taxonomy law itself was not run.

## 6. Files

Created: `NX/🔌️sdk/{🟦️.ts,🔣️policy.json,🧬️schema/🔣️.json,🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json}`, `NX/🔁️probe/{🟦️.ts,🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json}`, `NX/⚙️operations/{🟦️.ts,🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json}`, `B/⚙️operations/{🟦️.ts,🔣️catalog.json,🧬️schema/🔣️.json,🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json}`, `L/⚡️caching/🔁️verification/{🔣️probe.json,🧬️schema/🔣️.json}`, `C/nx-cache-tools/{verify-nx-cache.ts,compile-authorizers.ts}`.
Changed: `L/⚡️caching/📇️inventory/🟦️.ts`, `L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🔌️sdk/🟦️.ts`, `L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts` (import), `L/⚡️caching/🕸️graph/🔏️seal/📥️membership/📥️journal/👷️worker/📜️script.ts`, `L/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts`, `NX/🔏️authorizers/📦️runtime/🟦️.ts` (export path), `B/📜️script.ts` (help line, `cache` branch, Nx exec dry-run guard, `nxOperationArgumentsV1`), `B/📥️caller/📤️child/🟦️.ts` (declared launches use `nxOperationArgumentsV1`), `B/🧭️selection/🟦️.ts` (`buildChangedFilesCommandsV1`), `B/🧭️selection/🧪️tests/🟦️.ts` (harness name list).
Deleted: `L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🔌️sdk/🟦️.ts` (moved into `NX/🔌️sdk/🟦️.ts`).
Untouched on purpose: `nx.json`, `🔧️tools/🏗️bazel/**`, dependencies bootstrap, `.vscode/`, `AGENTS.md`, `package.json`, `bun.lock`.

## 7. Open items

- Run `verify-nx-cache.ts` once the dependency lock/sync is green (and `bun repo tools setup nx` has rebuilt the authorizers).
- The new test files are not yet registered in a runner target (`L/⚡️caching/📜️script.ts` inference tests / project.json); they run with `bun test ./<path>`.
- `NX/⚙️operations` `exec` env contract uses the Nx names; a tool-neutral pair (`SEMIO_BUILD_PROJECT*`) needs a decision.
- Nx `affected` with `--base` and `--head` uses Nx's merge-base semantics while the neutral rule uses a two-dot diff; identical for a head that descends from the base (tested), different when the head is older than the base.
- `L/⚡️caching/🕸️graph/🔏️seal/🟦️.ts:68` still names `node_modules/nx/package.json` for hashing (not a module load).
