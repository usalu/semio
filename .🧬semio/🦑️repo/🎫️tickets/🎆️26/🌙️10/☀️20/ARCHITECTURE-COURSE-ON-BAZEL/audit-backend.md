# Audit: `bun repo --tool=bazel run <project>:<target>` in the live tree

Snapshot taken 2026-10-20, about 15:15 local time. The live tree was changing during the audit: `🎯️affected/🟦️.ts` and `📜️script.ts` (adapter) were written at 15:14:09 by a live apply of the bzl-lazy set. Re-check the table before relying on it.

Paths: `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`. `BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. `A` = `B/📜️script.ts` (819 lines). `R` = `/Users/ueli/Documents/semio`. Line numbers refer to the live files at the snapshot.

## 1. Code path for `run` with `--tool=bazel`

1. Tool selection. `parseBuildInvocationV1` (`BUILD/🧭️invocation/🟦️.ts:126-140`) takes `--tool=` or `--tool <x>`, falling back to `SEMIO_BUILD_TOOL`, then `policy.defaultTool`. Any value other than `nx` or `bazel` is refused. `--workers` is parsed in the same function.
2. Dispatch. `BUILD/📜️script.ts`: `RepoScript.run` calls `runOwnedBuildInvocationV1` (~line 96). That reads the caller capabilities, handles `--help`, `preflight` and the dashboard, and wraps `runOwnedBuildInLeasesV1` in cache-store leases. The leases are `leases`, `diagnostics` and `dependencies`. `runOwnedBuildInLeasesV1` sets `SEMIO_BUILD_TOOL` and the mode, then handles `tools`, `reset`, `cache`, `graph` and `show` before the task path.
3. Task path (for bazel, after the `selected.tool==="nx"` block, ~lines 205-230):
   - `loadBuildInvocationGraphV1` loads the graph, then `withSelectedRustBuildGraphV1` and `withRepositoryBuildGraphArtifactV1` pin it (env `SEMIO_BUILD_GRAPH_PATH` and `SEMIO_BUILD_GRAPH_DIGEST`).
   - `selection.selectBuildTasksV1` (`BUILD/🧭️selection/🟦️.ts`) turns the CLI arguments into `request` (tasks, args, configuration, bail, skipCache, workers, excludeDependencies).
   - `planning` (`--dry-run` or `--graph`) prints `bazelDryRunPlanV1(planBazelTasksV1(...))`. Otherwise `untouched` (no affected tasks) returns early.
   - `--watch` runs `adapter.runBazelTasksV1` inside `watchBuildInvocationV1`.
   - Normal runs call `withStoreReceiptsV1` around `adapter.runBazelTasksV1(root, graph, request, invocation, phase)`. Repair re-runs the task with `skipCache` and `excludeDependencies`.
4. Adapter entry. `runBazelTasksV1` (`A:713`) takes `withBazelStorageV1` (tools, dependencies, products stores), then `runBazelTasksInStorageV1` (`A:716`).

### Task graph
- `planBazelTasksV1` (`A:106-132`). Task identity is `project:target[:configuration]` (`taskIdentity`, `A:79`). Each task gets `label = t_ + sha256(id)[0:16]`, so every Bazel label is ASCII.
- Dependencies come from `dependsOn`: `^` means transitive over the project graph, `dependencies: true` and `projects` are supported, and `params: forward` forwards args. Cycles are refused ("Build task dependency cycle").
- `requireBazelDependencyV1` (`A:88`) refuses a dangling dependency. `bazelDanglingDependenciesV1` (`A:95`) lists all of them at once.
- `excludeDependencies` keeps only explicitly named tasks.
- `bazelActionsV1` (`A:255-277`) turns tasks into actions. Kinds: `task`, `noop` (ordering only, collapsed to its producers), `launch`, `hermetic` and `test`. A selected cacheable task with no consumers and disposition `test` becomes kind `test`.

### BUILD and MODULE files, and where they live
- Bazel binary: `prepareBazel` (`⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/📜️script.ts`, pinned version 8.4.2) into `tools/bazel/bin`. Install base `tools/bazel/install/<ver>/<platform-arch>`.
- Module catalog (once per identity hash): `prepareBazelModulesV1` (`A:69-78`). It writes `MODULE.bazel` (`module(name="semio_tasks")`) and a `setup` rule into `dependencies/bazel/modules/<id16>/workspace`. It runs `bazel --batch --output_base=<dir>/base build //:setup --lockfile_mode=update --repository_cache=...`. Offline with no prepared catalog is refused.
- Per-selection workspace: `publishBazelGraphV1` (`A:575`) and `generateBazelWorkspaceV1` (`A:587-618`). The directory is `products/bazel/workspaces/<ns16>`, where `ns16 = sha256(selected, args, configuration)[0:16]`.
  - Writes `MODULE.bazel`, `tasks.bzl` (`STARLARK_RULES` at `A:512-566` plus `BAZEL_CARGO_RULE_V1` plus hermetic Starlark), `<label>.json` manifests, `<label>.input` fingerprints, `BUILD.bazel`, and `MODULE.bazel.lock`.
  - `linkBazelSourcesV1` (`A:447-452`) symlinks the checkout as `s`. It removes any `.bazelignore`.
  - `renderBazelBuildV1` (`A:388`) renders file groups (`d_`, `k_`, `i_` names) and the `repo_task`, `repo_test`, `repo_launch` and `repo_noop` rules.
- Output base: `bazelServerBaseV1` (`B/💾️storage/🟦️.ts`) is `products/bazel/outputs/server/<sha256(workspace)[0:16]>`. It is passed as `--output_base=` and binds one server to one workspace directory.
- Storage layout (`bazelCacheLayoutV1`, `storage/🟦️.ts:~28`): `tools/bazel/{bin,install}`, `dependencies/bazel/modules`, `products/bazel/{workspaces,outputs,actions,outputs/server}`, `products/tmp`. Paths over 256 characters are refused.
- ASCII guard (schema `HostPaths`, `B/🧬️schema/🔣️.json:1055-1076`): on darwin `install`, `outputs` and `server` must match `^[\x20-\x7e]+$`. On linux only `install` is guarded. The root `R` is ASCII, so the current paths pass.
- Path budget (`PlatformRules`, schema line ~823): posix limit 4095, execroot `/execroot/_main/s/`, 256 tracked units. `bazelPlatformProfileV1` refuses a profile that does not fit.
- `.bazelrc` (repo root, present, modified today 14:56): `--enable_bzlmod`, `--enable_workspace=false`, `--experimental_convenience_symlinks=ignore`, disk-cache GC settings. The adapter also passes `--bazelrc=R/.bazelrc`.

### Action keys and inputs
- `bazelTaskInputsV1` (`A:197-216`) expands the declared inputs. Named inputs, `^` transitive inputs, `{projectRoot}` and closure markers are resolved.
- `bazelSourceCoverV1` (`A:354`) turns declared inputs into Bazel file groups. Bazel sees only declared source groups.
- `taskFingerprint` (`A:482-491`) hashes: platform, arch, bun version, build mode, implementation sha256, the Cargo toolchain digest for cargo tasks, sorted source-file identities, runtime-command stdout, and declared env values. An uncached task also mixes in `randomUUID()`. The result is written to `<label>.input`.
- Cacheability is decided by `bazelCacheDecisionV1` (`A:226-233`): `continuous` and `skipped` are uncached, `noop` is cached, a forbidden plugin cache class is uncached, `cache:false` is `declared`, `undeclared` and `unbounded` (no `outputs` array) are uncached. Cached is `cache:true` with an `outputs` array.
- Starlark `_requirements` (`STARLARK_RULES`, `A:512-520`) sets `no-remote-exec` on every rule. `cache` false needs a reason and sets `no-cache`. `host_state` sets `no-sandbox`. The env `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` is set.

### Outputs: snapshot and restore
- `captureBazelSnapshotV1` (`A:626-644`): each declared output pattern is expanded (globs via `Bun.Glob`). Entries are recorded as directory, file (copied by hash), or symlink. `snapshot.json` is written.
- `bazelMissingOutputRefusedV1` (`A:620`): a missing output is an error only when `cache:true` and `optionalOutputs` is not set.
- `restoreBazelSnapshotV1` (`A:646-665`): compares the tree with the snapshot. If it differs, it removes the roots and recreates the entries, fixing directory modes. It runs under an exclusive lease `bazel-snapshot-restore`. After the build, every `task` and `hermetic` action in `plan.selected` is restored.

### Continuous tasks (dev servers) and tests
- Continuous tasks (`definition.continuous`) are never cached. Each is started with `bazel run //:<label> -- ... birth-launch`, under the cancellation control. Readiness is `readyWhen` markers or the `readReadinessDeclarationV1` watcher. On success it prints `[bazel] task-ready <id> <addr>`.
- A continuous task that exits before readiness, or while a selected consumer depends on it, is an error ("Continuous task exited ...").
- `watch` uses the `onStarted` callback to start `watchBuildInvocationV1`.
- Tests: `repo_test` rule (`test=true`). When any test is selected, the build is `bazel test --test_output=all|errors --test_summary=short --enable_runfiles`, otherwise `bazel build`. Non-cacheable tests take the build path.

### Progress and cancellation
- Bootstrap progress: `bazelProgressLineV1` and `[bazel] <phase>` lines. Task progress: `invocation.control.publish` with `build:bazel:*` stages, `advanceScriptInvocation`, and `[bazel] task-started`.
- Cancellation: `phaseControl.signal` aborts `controller`. `bazelCancellationControl` (`A:697-705`) opens a loopback TCP server on 127.0.0.1. Actions join with a token. On stop it sends `cancelled` to birth actions, waits up to `policy.terminationReceiptMilliseconds` for `closed` acks, then aborts `commands` and destroys sockets.
- Bazel is also launched with `--max_idle_secs=${policy.serverIdleSeconds}`.
- BEP: `--build_event_json_file=bazelBuildEventsFileV1(...)`, then `recordBazelBuildMetricsV1` in a `finally`. I did not verify that the BEP file is deleted afterwards.

### How Bazel is launched (flags)
Startup, per `A:723`:
- `--max_idle_secs`, `--install_base`, `--host_jvm_args=-Xmx2048m`, `--nosystem_rc`, `--nohome_rc`, `--bazelrc=R/.bazelrc`, `--output_base=` (from `cache.startup`), profile startup (empty on darwin and linux).

Options, per `A:724-726`:
- `--lockfile_mode=error` (set after `MODULE.bazel.lock` is written), `--incompatible_strict_action_env`.
- Disk cache: `--disk_cache=products/bazel/actions`, `--experimental_disk_cache_gc_max_size=<budget share>`.
- `--noremote_accept_cached` when skipCache. Remote-cache flags only when a remote contract is configured.
- Platform command: `--nobuild_runfile_links` (darwin and linux); win32 adds `--noenable_runfiles`.
- `--incompatible_autoload_externally=`, `--repository_cache=`, `--repository_disable_download`, `--keep_going` unless `bail`, `--subcommands` if verbose.
- Hermetic execution adds `--jobs`.
- Environment on darwin: `LANG=en_US.UTF-8`, `LC_ALL=en_US.UTF-8`.

Sandbox: no `--spawn_strategy` and no `processwrapper` in the live Bazel directory, so Bazel's platform default applies (darwin-sandbox on macOS, linux-sandbox on linux). `host_state` tasks, continuous tasks and birth actions run with no sandbox (schema lines 1154-1162).

### Where `.bazelignore` is written
Nowhere. The adapter removes it: `linkBazelSourcesV1` (`A:448`) runs `rm(join(workspace,".bazelignore"))`. The doc comment there explains why. The fixture test `B/🧪️tests/📥️workspace/🟦️.ts:44-52` asserts that no `.bazelignore` exists and that a stale one is removed. Queries use `//:all` or explicit labels.

## 2. Staged sets: LIVE / PARTIAL / STAGED-ONLY

Method: each result file's claimed identifiers and files were grepped in the live library and process trees. Where a staged full-file copy existed, it was diffed against the live file. The live tree was changing, so this is a snapshot.

| Set | Status | Evidence |
|---|---|---|
| bzl-stage (source groups, transitive `^`, dangling refusal, uncached report, `.bazelignore` fix, Cargo dirs) | LIVE | `bazelSourceIndexV1` (302), `bazelSourceCoverV1` (354), `requireBazelDependencyV1` (88), `bazelDanglingDependenciesV1` (95), `bazelUncachedReportV1` (235), `bazelTaskCargoDirectoriesV1` (467), `linkBazelSourcesV1` exported (447) with no `.bazelignore`. Fixture tests `📥️sources`, `📥️workspace` exist. The result file's "keep `.bazelignore`" claim is superseded by the live code. |
| bzl-lazy (neutral command plan, `bazel test`, native `affected`) | PARTIAL, applying now | Live `B/🎯️affected/🟦️.ts` (created 15:14) matches the staged `affected.ts` except import paths. `bazelCommandPlanV1`, `bazelTestTargetV1`, `readBazelTestsV1`, `bazelAffectedProjectsV1` are absent. Staged `commands.ts` differs from live `⚙️operations/🟦️.ts` by 280 lines. |
| bzl-coverage (disposition classes, `repo_test`, interactive class) | PARTIAL | Live: test kind via `bazelTaskCoverageV1` in `bazelActionsV1` (`A:255-277`). `repo_test` rule in `STARLARK_RULES`. Disposition classifier with `interactive`/`refused` in `BUILD/🏃️execution/🟦️.ts`. Full staged set not confirmed. |
| bworkers (worker pools, `--workers on|off`) | PARTIAL | Live: `readWorkerSwitchV1` (invocation), `WorkerModuleCacheV1` and `workerContractFieldsV1` in `BUILD/👷️workers/`, `bazelWorkerSettingsV1` imported by `A`. `NX_PARALLEL` appears in live source only in the nx patch and tests. |
| rcache (neutral remote-cache contract, `cache serve`) | LIVE (core) | `🌐️remote-cache.json` in `BUILD/🏃️execution/🧬️schema/`. `resolveRemoteCacheContractV1`, `bazelRemoteCacheV1` flags, `nxRemoteCacheToolV1`, `BUILD/🌐️remote-cache/⌨️command`, `🖥️server` all live. BEP token deletion not verified. |
| rexec (hermeticity, toolchains, `bun` provisioning) | PARTIAL | Live: `bazelExecutionFlagsV1`, `bazelExecutionPlanV1`, `bazelHermeticDecisionV1`, `prepareBazelBunV1`, `bazelHermeticViolationsV1`, `verifyBazelBunArchiveV1`, `hermetic.json`. Staged-only: `runBazelTasksOnHostV1` (absent). Per result file, actions still use the host bun by absolute path. |
| bzl-xplat (Windows, Linux, devcontainer) | PARTIAL | Live: `bazelPlatformProfileV1`, `requireBazelHostPathsV1` (Linux non-ASCII install guard), win32 and linux platform rules, `nxToolingInstallManifestV1`, `nxExternalPinsV1` in caching bootstrap. Staged-only: processwrapper sandbox in unprivileged containers (no `processwrapper` in live). Windows was not run by the executor. |
| ingress-perf (cargo ingress, CLI startup) | PARTIAL, unverified | Live markers: `createRepositoryCustodyPoolV1`, `openRepositoryCargoSessionV1`, `verifyPersistentManagedChildrenV1`, `withRetainedRepositoryCompilerStoreV1`, `buildSchedulerV1`. The 27 s to 2 s result is not checked. |
| bzl-rust (per-crate cargo actions) | PARTIAL | Live: `BAZEL_CARGO_RULE_V1`, `bazelCargoClassV1`, `renderBazelCargoBuildV1`, `selectCargoArtifactsV1`, `BAZEL_CARGO_EXCLUSIVE_V1`. STAGED-ONLY: the wasm chain cargo to jco to wasm-opt (no `jco` or `wasm-opt` in the live Bazel directory). |
| uncached-owners (every `cache:false` has a reason) | PARTIAL | Live: law rule `uncached-without-reason` in `⚖️laws/🔒️cache-correctness/` rules. STAGED-ONLY: `withRepositoryTestArtifactsV1` (test files only). Manifest changes not checked. |
| cache-fix (fail-closed target classes, output collisions) | LIVE | `targetClasses` in `⚡️caching/🔣️policy.json` and its schema. `cachedOutputsAdmittedV1` in `BUILD/🕸️graph/📥️inference/🎯️target-policy`. `hostPlatformOutputsV1` in the cache-correctness law. `bazelMissingOutputRefusedV1` (`A:620`) live. |
| integrate (ordered apply of all sets) | IN PROGRESS | `integrate-tools/` is modified in the working tree. No `result-integrate.md` exists. |

Coordination note: `policy-admit` was applied live at 14:47 per coordination.md.

## 3. Known defects that would hit a single-project run

| # | Defect | Status now | Evidence |
|---|---|---|---|
| 1 | Adapter wrote `.bazelignore`, which disables Bazel caching | Fixed live | No writer in `A`. `linkBazelSourcesV1` removes it (`A:448`). |
| 2 | Bare import `@semio-tech/ui/axes/plan` in the repo CLI closure breaks zero-touch | Not found in `⌨️cli` now | Grep of the CLI module returned no match. The zt-e2e-fix set is staged for the rest. |
| 3 | Emoji generator targets crash under the store receipt | Unverified | coordination.md line 241. |
| 4 | `waitForServiceReady` 60 s default too short for cold vite | Staged (zt-e2e-fix) | coordination.md. Not checked live. |
| 5 | `tools setup nx` broken by the `workspace:*` recipe | Staged (xplat) | Not checked live. |
| 6 | Dangling `dependsOn` refuses the whole plan | Live behaviour | `A:88`. Known dangling refs include `@semio-tech/trinity-jack-lsp:wasm`. Check course closure. |
| 7 | Targets with `cache:true` and no `outputs` are uncached (reason `unbounded`) | Live behaviour | `bazelCacheDecisionV1` (`A:226`). Stage report counted 809 such targets in 278 projects. Warm rebuilds miss for these. |
| 8 | Hermetic actions use the host bun by absolute path, so keys are not portable | Live | rexec result; `bazelExecutionPlanV1`. |
| 9 | Tasks that use `host_state` or run as continuous are never cached and run with no sandbox | Live | Schema lines 1154-1162. |
| 10 | Linux unprivileged containers may need processwrapper, which is not live | Staged only | No `processwrapper` in live `B`. |
| 11 | `setup bazel` must run before `run`; offline without a prepared module catalog is refused | Live | `prepareBazelModulesV1` (`A:69`). |
| 12 | Live adapter is mid-apply: script and `🎯️affected` changed at 15:14 while commands and neutral layers are still staged | Live hazard | Mixed state. Re-run the table after integrate finishes. |
| 13 | BEP file may keep the remote-cache token until deleted | Unverified | rcache result. |

## 4. Risks for running the architecture course under Bazel

1. Warm-rebuild target. Course targets must declare `outputs` arrays (`outputs: []` for pure checks). Anything else is uncached by design, so an edit will re-run it and a warm build will not be a cache hit.
2. Continuous targets (`dev`, `dev-site`) are uncached and started under the cancellation control. Readiness depends on `readyWhen` or a readiness declaration. A dev target that exits early fails the whole run.
3. Tests become `bazel test` only when they are cacheable test-family tasks. Playwright (`test-e2e`) is `cache:false` and runs live.
4. Dangling references in the closure refuse the plan before any action runs. Run `bazelDanglingDependenciesV1` (or the `show` and `graph` commands) first.
5. Darwin sandbox: default spawn strategy, `no-sandbox` for host-state tasks. Non-ASCII paths are refused for install, outputs and server, so keep the checkout path ASCII.
6. Linux devcontainer: default sandbox may not run without processwrapper (staged only). Verify with a real run.
7. Windows: loopback control socket, junction source links, `--nowindows_enable_symlinks`. Not run by anyone.
8. Concurrent applies: the live adapter and `🎯️affected` changed at 15:14. Results may differ between two runs in the same hour. Re-audit after integrate.
9. Snapshot restore takes an exclusive lease, so parallel course runs in one checkout serialise on it.
10. Remote cache, if configured, puts the token in the BEP file until the adapter removes it (unverified).

## Backend edits

None. This was a read-only audit. No source files were changed, and no build or Bazel run was started.

## Process notes

- The repo `grep` alias is ugrep, which fails on these emoji paths. `/usr/bin/grep` was used instead.
- Two background greps were stopped with `TaskStop` after they were launched over too large a scope.
