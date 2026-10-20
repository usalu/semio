## 📘️ Build tools guide

This guide is the working view of the tool-neutral build system: which tool to pick, what each operation does, how results are cached and shared, how work is spread over workers and executors, and how to classify a new target. The reference sections after it (Neutral model to Remote cache) hold the contracts, the measurements and the open items. Every command below is an operation of the catalogue `📚️library/🏗️build/⚙️operations/🔣️catalog.json`; `repo <command> --help --lang=en|de` prints the same usage in English or German. Where `repo` is not on the `PATH`, write `bun repo`, the root package script. A German version of this guide follows in the German chapter (Build-Werkzeuge im Alltag).

### 🔀️ Choose the tool

- **Select.** `--tool=nx|bazel` wins, then the environment variable `SEMIO_BUILD_TOOL`, then the policy default `nx`. The choice is exported as `SEMIO_BUILD_TOOL` to every nested task, so a task that calls `repo` again stays on the tool you chose. The dashboard axis `build-tool` sets the same variable. Anything else is refused ("Unknown build tool").
- **Provision.** `repo tools setup all` installs the pinned Nx and Bazel for this host; `repo tools setup nx` or `repo tools setup bazel` installs one. `workspace:setup` runs the same step.
- **One graph.** Both tools run the same neutral graph with the same task ids (`<project>:<target>[:<configuration>]`). A selection that resolves to different tasks under the two tools is a defect, not a feature.

| | Nx (default) | Bazel (`--tool=bazel`) |
|---|---|---|
| What runs | one `nx run` per selection | one generated workspace per selection; a task is a Bazel action, a selected cached `test` task is a Bazel test, a service is a `bazel run` target |
| Local cache | task results in `nx/artifacts` of the rust partition | action cache of the output base plus the disk cache `bazel/actions` of the products partition |
| Remote cache | Nx self-hosted cache API | Bazel HTTP cache protocol, token through a credential helper |
| Remote execution | no | opt-in for targets that declare `metadata.semio.hermetic` |
| Concurrency | `NX_PARALLEL` | persistent workers and `--jobs` |
| Reuse across selections | by task hash in the task cache | by action key in the disk cache: a second selection reuses the first one's results |

Use Nx for the default flow. Choose Bazel when you want results shared across selections and machines, workers, or remote execution. Both are driven by the same flags; where a tool cannot honour a setting, `repo cache remote status` lists it as `unsupported` (see Remote cache for the Nx fields).

### ⚙️ The neutral operations

| Operation | What it does | Example |
|---|---|---|
| `run` | one task and the tasks it depends on | `repo run @semio-tech/semio-tech-play:serve --tool=bazel` |
| `run-many` | the same targets in many projects (`--projects`, `--exclude`, `tag:`, `name:`, `directory:`, `!` patterns) | `repo run-many --targets=test --projects=tag:rust --tool=bazel` |
| `affected` | targets in the projects changed against `HEAD` (tracked changes plus untracked files) and their dependents | `repo affected --targets=build --base=main --head=HEAD` |
| `exec` | a command once per project, project root as working directory, `SEMIO_PROJECT_NAME` and `SEMIO_PROJECT_ROOT` exported | `repo exec --projects=@semio-tech/repo-client -- bun test` |
| `--dry-run`, `--graph` | print the task graph as a `DryRunPlan` and run nothing; `--graph=<file>` writes it | `repo run-many --targets=build,test --dry-run` |
| `show`, `graph`, `watch` | the neutral graph, and a command rerun on source changes | `repo show projects --json` |
| `cache status`, `cache clear` | count or empty the selected tool's local task cache | `repo cache status --tool=bazel` |
| `cache remote status`, `cache serve` | the remote cache contract and a local cache server (see Caching model) | `repo cache remote status --tool=bazel` |
| `reset` | invalidate the graph and reset the tool's local state | `repo reset --tool=bazel` |

- **Dry runs and graphs.** `--dry-run` and `--graph` print the same `DryRunPlan` for every tool and start neither Nx nor Bazel: `roots`, `tasks` and `dependencies` (under Bazel also `selected`, and dependencies on services in `continuousDependencies`). The plan says what would run, not what is cached; use `cache status` for that.
- **Affected.** The rule is the same for both tools: the deepest project that owns a changed file, projects whose target inputs name a changed `{workspaceRoot}/…` file, every project when `nx.json` changes, then all dependents. Bazel answers it with a `rdeps` query over a generated package, cross-checked against Nx's own locators on four of five change sets; a change to `bun.lock` alone is the known difference (Nx treats it as "everything", the neutral rule and Bazel see only path ownership). `--projects` is refused for `affected`; narrow with `run-many`.
- **Cancel and progress.** Every operation can be cancelled (Ctrl+C) and reports progress lines while it walks, queues or downloads; a cancelled run stops at the next checkpoint.
- **Bazel specifics.** A selection builds exactly the transitive closure of the selected tasks, nothing else is analysed. `run-many` is one Bazel invocation with all labels, so shared dependencies run once. When a selected task is a test the invocation is `bazel test` (with `--build_tests_only` only when every selected label is a test); `--skip-cache` adds `--nocache_test_results`.

### 💾️ Caching model

A result is reused when the key of the task is unchanged. Each tool derives its key from the same declared `inputs` (files and named groups, environment values, `runtime` command outputs, the closure of the files the command script imports, platform, Bun version, build mode) and from the identity of the build tool itself. Consequences: a file the task reads but does not declare does not invalidate it (a stale hit, not a failure); an edit to the build tool's own implementation re-keys its tasks once (for Bazel the captured implementation is an input of every action), so expect one cold run after pulling such a change; a task that declares the whole repository as input re-keys on any edit and is an owner problem, not a cache problem.

| Layer | Where | Cleared by |
|---|---|---|
| Nx task cache | `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts` (rust partition) | `repo cache clear` (`nx reset --onlyCache` inside the bounded Nx namespace) |
| Bazel action cache | output base `bazel/outputs/server/<hash>` of the products partition, one per generated workspace directory; the server idles out after 120 s | `repo cache clear` stops the servers and removes the output bases |
| Bazel disk cache | `.cache/b/p/m/bazel/actions`, collected down to 40 percent of the products budget, entries older than 30 days dropped | `repo cache clear`; survives `bazel clean` and the loss of a server |
| Remote cache | an HTTP endpoint you configure, or `repo cache serve` (store in `.cache/remote-cache`) | the server's own bound and eviction |

All build state below `.cache/b/<partition>` is derived and safe to delete when nothing runs (partitions `d` dependencies, `t` tools, `g` graph, `o` observations, `p` products, `l` leases, `n` Nx pool; see Cache partitions). `--skip-cache` ignores the local task cache of the run (for Bazel it also stops accepting remote hits), `--skip-remote-cache` ignores only the remote cache, and the dashboard axis `cache` offers `use`, `skip-local` and `skip-all`.

- **Early cutoff.** Under Bazel a task whose output snapshot is byte-identical after its input changed does not re-run its dependents (measured: a whitespace-only edit of a generator input re-ran the generator only).
- **Store receipts.** Products that live in a leased store, not in the project directory, declare a receipt as their only output; both tools verify it after every run and re-run that single task uncached when the receipt is damaged. See Store receipts.

**Remote cache.** It is off until `SEMIO_REMOTE_CACHE_ENDPOINT` names an `http://` or `https://` origin; the contract and its variables are in the Remote cache section.

```bash
repo cache serve --port 6090
```

```bash
SEMIO_REMOTE_CACHE_ENDPOINT=http://127.0.0.1:6090 SEMIO_REMOTE_CACHE_ACCESS=read-write repo run @teaching/architecture-course:test --tool=bazel
repo cache remote status --tool=bazel
```

In PowerShell the same is `$env:SEMIO_REMOTE_CACHE_ENDPOINT = "http://127.0.0.1:6090"`, `$env:SEMIO_REMOTE_CACHE_ACCESS = "read-write"`, then `bun repo run @teaching/architecture-course:test --tool=bazel`.

- **Access.** The default is `read-only`; results are pushed only with `SEMIO_REMOTE_CACHE_ACCESS=read-write` and `SEMIO_REMOTE_CACHE_UPLOAD` not `false`. The token is read from the variable `SEMIO_REMOTE_CACHE_TOKEN` (or the variable named by `SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT`) and never appears in a flag or on disk beyond one build: Bazel receives it through a credential helper, Nx through its own variable. Nx cannot withhold uploads, so for Nx read-only means a read token on the server.
- **`cache remote status`.** Prints the resolved contract (`configured: false` while no endpoint is set), the rendering the selected tool gets with its `unsupported` fields, whether the token variable is set (never its value) and a read-only probe of the endpoint: `reachable`, `authorized` and the HTTP status. It stores nothing.
- **`cache serve`.** Serves Bazel's HTTP cache protocol (`/ac/<sha256>`, `/cas/<sha256>`, blobs verified against their digest) and the Nx self-hosted cache API (`/v1/cache/<hash>`) from this machine until you stop it. Options: `--port` (default 6090, `0` picks a free one), `--host` (default `127.0.0.1`), `--directory`, `--max-bytes` (default 4 GiB, at least 1 MiB; the least recently used entries go first). Tokens come from `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN` and `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN`; without a token the server refuses to listen beyond loopback. The dashboard tool `build-cache-serve` starts it.
- **Download modes.** `toplevel` (default) and `all`. `minimal` is not part of the contract and the schema refuses it: the Bazel adapter restores the snapshot of every selected task from `bazel-bin`, and with `minimal` the snapshot stays in the remote store, so the restore fails with `ENOENT` and nothing reaches the checkout (measured). Nx always fetches everything.
- **What is shared.** Bazel keys of today's tasks contain the absolute Bun path and the checkout root of the captured implementation, so a hit across machines needs the same user and checkout path. Hermetic actions (next section) have host-independent keys (measured: the same inputs at a different checkout path hit the disk cache).

### 🌍️ Remote execution

Remote execution is optional and opt-in per target. The build system defines the contract and the Bazel rendering; it ships no executor.

- **Declare.** A target is a hermetic action only when its manifest says so: `"metadata": { "semio": { "hermetic": { "version": 1, "tools": ["bun"] } } }` (schema `🏗️build/🏃️execution/🧬️schema/🔒️hermetic.json`). It must also be cached with a declared `outputs` array, run plain `command` strings, have only file inputs (no `runtime` or `env` input, no `{options.…}` or `{env.…}` token), not be continuous and not orchestrate other builds. A target that does not qualify keeps host state; the reason is closed (`undeclared`, `unsupported-execution`, `host-inputs`, `uncached`, `continuous`, `orchestration`, `tool`). No target in the repository declares it yet, so nothing runs remotely today.
- **What a hermetic action is.** It runs under a pinned Bun that is a declared tool input (downloaded under a lease with progress, verified by size and SHA-256 before extraction), in a sandbox or on a worker, with a strict environment (`LC_ALL=C`, `TZ=UTC`, `SOURCE_DATE_EPOCH=0`, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`), no host path in its key, normalised outputs, and it fails on an undeclared write. Hermetic actions run the pinned Bun, not the one on your `PATH`.
- **Switch.** Unset keeps every action local; a malformed value fails closed.

| Variable | Meaning | Default |
|---|---|---|
| `SEMIO_REMOTE_EXECUTION_ENDPOINT` | `grpc://`, `grpcs://` or `unix:` URI of the executor | none (local) |
| `SEMIO_REMOTE_EXECUTION_PLATFORM` | worker platform: `darwin`, `linux` or `windows` plus `-arm64` or `-x64` | `linux-x64` |
| `SEMIO_REMOTE_EXECUTION_PROPERTIES` | extra worker properties, `name=value,…` (up to 16), added to the platform's `OSFamily` and `Arch` | none |
| `SEMIO_REMOTE_EXECUTION_JOBS` | concurrent actions while the executor is on | 128 (at most 512) |

- **Routing.** Only hermetic actions go to the executor, with `--noremote_local_fallback`: an unreachable executor fails the build, it never runs the action locally. Host-state actions stay local (`no-remote-exec`).
- **Executor requirements.** A service for Bazel's Remote Execution API v2 with content store, action cache and scheduler; workers whose properties match the platform (Buildbarn matches exactly; BuildBuddy uses `container-image` and `pool`); Linux workers need only the uploaded Bun, no checkout. Execution flags never carry cache settings: the cache endpoint, instance, upload, download mode and token stay with the remote cache contract, whose endpoint is an `http(s)` origin. Pointing Bazel at one gRPC service for cache and execution is not defined by the contract yet.
- **Platforms.** Network isolation of a hermetic action is enforced on Linux and on remote workers; macOS' sandbox cannot block it (measured with Bazel 8.4.2). On Windows there is no sandbox, so the runner refuses a hermetic action there (the source link is a junction); run Windows hermetic targets remotely.
- **Optional local stack.** To try the contract without a service, any local executor that speaks REAPI v2 on loopback works, for example a Buildbarn stack (storage, scheduler, worker, runner) in Docker. It is not a `repo` command and not needed for anything else in this guide; the build system treats it like any other executor, so set the four variables above to its address.

### 👷️ Worker pools

Persistent workers keep a process alive between actions, so an action no longer pays for starting and loading the build tool again.

- **Switch.** `--workers on|off`, then `SEMIO_BUILD_WORKERS`, then the worker policy in `📚️library/🏗️build/🔣️policy.json` (`enabled`, on by default). `--parallel <n>` or `SEMIO_BUILD_PARALLEL` sets the number of concurrent tasks (default: the number of cores, at most 8).
- **Kinds.** The kind is derived from the target name and continuity, never from the tool; the first matching rule wins.

| Kind | Target names | Instances at most |
|---|---|---|
| `service` | `continuous: true` | none (never a worker) |
| `test` | `test`, `smoke`, `mutation`, `bench`, `e2e` (also `<name>-…`) | 8 |
| `check` | `check`, `lint`, `verify`, `typecheck`, `format-check`, `contract`, `schema`, `stdio` (also `<name>-…`), and names ending in `-check` | 8 |
| `generate` | names with `generate` or `generator` | 4 |
| `build` | `build`, `prepare`, `activate`, `wasm`, `native`, `package`, `bundle`, `cpp`, `publish`, `canonical`, `preview`, `setup`, `deps` (also `<name>-…`) | 2 |
| `script` | everything else | 4 |

- **Budget.** All pools together may use 30 percent of the host's memory at 512 MB per worker. Each kind gets one instance first (test, check, generate, build, script, as far as memory holds), the remaining instances go to the kinds in the same order up to their bound and the job count; a kind that finds none runs without workers.
- **Bazel.** Workers speak Bazel's JSON worker protocol, run the action body inside the worker process (leaf commands stay separate process trees), support cancellation and shrink the pool under memory pressure. Worker and non-worker actions have the same action key, so caches are shared across both modes. Actions that never use workers: services, hermetic actions and tests run as `bazel test`. Cargo tasks run one at a time.
- **Nx.** Nx forks a process per task, so only `NX_PARALLEL` is mapped (to the job count); `NX_BATCH_MODE` is set only for an executor that registers a batch implementation, which none does. Every other worker setting (pools, instance bounds, memory, sandboxing) is unsupported under Nx.
- **Measured.** CPU per action fell from 2.50 s to 1.46 s in steady state; wall clock was not consistently better on a saturated host, because 85 percent of a warm in-worker request is opening the graph cache store.
- **Turn them off** with `--workers off` to compare against one process per action, to debug a single task, or when memory is scarce.

### 🚦️ Task dispositions

Every task has a tool family (`bun-script`, `tsc`, `vitest`, `bun-test`, `vite`, `playwright`, `go`, `python`, `dotnet`, `latex`, `wasm`, `cargo`, `docker`, `aggregate`) and one disposition. Nx runs all of them as `nx run`; Bazel maps them as follows (`📚️library/🏗️build/🏃️execution/🧬️schema/🎯️coverage.json`).

| Disposition | Meaning | Under Bazel |
|---|---|---|
| `action` | cached, replayed from the action cache | cached build action, output snapshot restored into the checkout |
| `test` | cached and named `test` or `test-…`, `outputs: []`, nothing else selected consumes it, at most 10,000 declared files | `bazel test`: the passed result is cached, `(cached) PASSED` on the next run; not used where the platform has no runfiles (Windows) |
| `interactive` | `continuous`, runs until stopped | `bazel run` target; it starts on every run and is never produced |
| `uncached` | always executes; its recorded reason says why | action tagged `no-cache` |
| `refused` | `cache` or `outputs` left undeclared (reasons `undeclared`, `unbounded`) | fail-closed until the owner declares what is missing |

Static census of 2026-10-20 (8,028 tasks of the reconstructed graph, with the classification sets applied): 3,771 cached (actions and tests), 1,320 interactive, 2,937 uncached with a recorded class, 0 refused and 0 uncached for the reason `declared` (373 before).

### 🏷️ Classify a new target

Answer these in order; the first yes decides.

1. **Does it run until stopped** (server, watcher, shell)? `"continuous": true`, class `continuous`, reason `serves-until-stopped`. It becomes `interactive`.
2. **Does it read only repository files, write only declared outputs (or none), use no network, host store, lease, clock or running process?** Then cache it: `"cache": true`, `inputs` for everything it reads outside its project, `outputs` (`[]` for a pure check), and give it a cached name family or its own decision (below).
3. **Otherwise** `"cache": false` plus the decision that names the reason.

| Class | Reasons | Use for |
|---|---|---|
| `deterministic` | `pure-function-of-inputs` (declare outputs), `check-without-outputs` | cached checks, tests, generators with declared outputs |
| `continuous` | `serves-until-stopped` | servers, watchers, shells |
| `live` | `drives-running-process`, `exercises-host-resources` | e2e against a running process, tests of real locks, storage, process trees |
| `side-effecting` | `mutates-workspace`, `mutates-host-store`, `publishes-remote`, `publishes-live-state` | formatters, scaffolders, installers, releases |
| `diagnostic` | `measures-environment`, `reads-caller-input`, `admission-refusal` | benchmarks, doctors, tools driven by their arguments |
| `aggregate` | `orders-prerequisites`, `reenters-build-tool` | pure ordering; a verb that runs other targets through `repo` should declare them in `dependsOn` instead |
| `managed-output`, `driver-owned` | `outputs-in-managed-store`, `outputs-are-store-links`, `freshness-owned-by-driver` | store-published builds and driver-owned playground steps |

The policy resolves a target name in this order (`🎯️target-policy`): continuous names, `cache: false`, the policy's uncached names, mutating and live names, then your decision in `metadata.semio.cache`, then the policy's `targetClasses` (name families such as `test`, `check`, `lint`, `typecheck`, `verify`, `build`, `generate`), then a `verify` command. **A target that none of them classifies never replays** (`unclassified`, `no-cache-classification`), and `"cache": true` alone does not change that, so give an unusual name its decision:

```json
"digest-inputs": {
  "executor": "repo:script",
  "cache": true,
  "inputs": ["default", "^default"],
  "outputs": [],
  "options": { "command": "bun ./📜️script.ts digest-inputs" },
  "metadata": { "semio": { "cache": { "version": 1, "class": "deterministic", "reason": "check-without-outputs" } } }
}
```

```json
"measure-startup": {
  "executor": "repo:script",
  "cache": false,
  "outputs": [],
  "options": { "command": "bun ./📜️script.ts measure-startup" },
  "metadata": { "semio": { "cache": { "version": 1, "class": "diagnostic", "reason": "measures-environment" } } }
}
```

- **Outputs.** A cached target must produce every declared output that is not a glob: otherwise it fails with "Declared output of a replayed Bazel task was not produced". Set `metadata.semio.optionalOutputs: true` only for an output that is legitimately absent. A `pure-function-of-inputs` target without declared outputs is demoted to `side-effecting`.
- **Test under Bazel.** Name it `test` or `test-<what>`, keep it cached with `outputs: []` and under 10,000 declared files (every declared file is linked into the test's runfiles before every run, even on a hit).
- **Remote execution.** Add the `hermetic` declaration (see Remote execution) only when the target truly reads its declared inputs alone.
- **Law.** The cache-correctness law (`📚️library/⚖️laws/🔒️cache-correctness`) reads every manifest through the production policy and fails for an unclassified target, a cached target that the policy overrides, an uncached target without a recorded reason, empty inputs, a producer without outputs, an unhashed environment override, a named input nobody declares, outputs that escape or overlap, and a mutating or network command in a cached target.

### 🔁️ Incremental builds and how to verify them

An incremental build re-runs exactly the tasks whose declared inputs changed and replays the rest. Prove it with the operations you already have; the CI job `smoke` is written to do the same for both tools on `@teaching/architecture-course:test` (the workflow has not run yet).

```bash
repo cache clear --tool=bazel
repo cache status --tool=bazel
repo run @teaching/architecture-course:test --tool=bazel
repo cache status --tool=bazel
repo run @teaching/architecture-course:test --tool=bazel
repo cache status --tool=bazel
```

1. Note the counts after `cache clear`; after the first run `entries` and `bytes` grew and, for Bazel, `activity.executed` counts the actions that ran.
2. The second run spawns no work: Bazel's summary shows no `local`, `worker` or sandbox spawn (or `disk cache hit`), `entries` do not grow and `activity.executed` stays, while `activity.cacheHits` counts the hits.
3. Edit a file the task declares: only that task and its dependents run (a dependent whose input snapshot is unchanged is replayed). Edit a file it does not declare and nothing runs: the input list is too short.
4. `repo cache clear` followed by the run proves a cold miss; `--skip-cache` forces execution once without clearing; `--dry-run` shows the plan the run would execute.
5. Expect extra work after: editing the build tool's own files (every action is re-keyed once), switching `prototype` and `release` (the mode is part of every key), and editing a declared input while the build runs (Bazel prints `Skipping uploading outputs because of concurrent modifications` and the next run executes again).

### 🖥️ Cross-platform notes

| Host | Notes |
|---|---|
| macOS | Cache partitions are case-sensitive APFS images, no privilege. Keep the checkout path ASCII (Bazel crashes at startup on a non-ASCII install base and its sandbox mis-encodes a non-ASCII output base; the layout is refused up front). The adapter sets a UTF-8 locale. |
| Linux | Directory-backed partitions, no root and no loop device; needs a case-sensitive file system with atomic directory exchange (ext4, xfs, btrfs). A non-ASCII `install` path is refused; a non-ASCII output base works. In an unprivileged container Bazel falls back to `processwrapper-sandbox` (same results, weaker isolation); `linux-sandbox` needs a privileged container. The adapter never names a sandbox strategy. |
| Devcontainer | As Linux. `.cache` is a named volume private to the checkout (a bind mount from a case-insensitive host is refused by `repo preflight`); `postCreateCommand` runs `repo run workspace:setup`. |
| Windows | Directory partitions swapped by a journaled rename on NTFS with per-directory case sensitivity. Needs `LongPathsEnabled` and a short ASCII checkout root (the Bazel path budget is `len(output base) + len(\execroot\_main\s\) + 256` against 259, or 32766 with long paths). The source link is a junction (no developer mode, no elevation), runfiles are off (`--noenable_runfiles`, startup `--nowindows_enable_symlinks`), so tests run as cached actions instead of `bazel test`, and hermetic actions are remote-only. Set variables with `$env:NAME = "value"` in PowerShell and call `bun repo`. Not run on Windows. |

No cache path may exceed 256 characters on any host. Bazel and Bun are pinned per OS and CPU (darwin, linux, windows on arm64 and x64) with checksums, so `repo tools setup` needs no manual download.

### 🩹️ Troubleshooting

- **Every command refuses after a reboot or crash** ("Exact canonical persistent Native publication required …"). Receipts record the boot session. On macOS, where an image receipt pins the device number, the first open of each partition after a reboot rebinds its receipt to the new device number under the exclusive writer lease, but only when the image is not attached, nothing is mounted below it and the directory identity (inode, size, allocation, mount directory) is unchanged; run any command that opens the stores, such as `repo cache status`, and let it finish. A device mismatch within one boot is never rebound. If the refusal names facts that changed, recreate the partition (Recovery, item 2).
- **A command waits.** Stores are claimed through a first-in-first-out lease queue. A waiter at position 2 or later prints a `lease-queue` line every 15 seconds that names the holder's process id; a line `remains active on …` is progress of a running walk, not a wait. Do not kill a holder that is making progress; read the head ticket first.
- **A receipt fails.** Store receipt: one task re-runs uncached (`[store-receipt] re-executing`). Dependency generation receipt: an edit of any file in its closure costs the next command one complete walk (progress `dependencies:generation-verify:<n>`). Image receipt: see the first item.
- **No cache hit.** Check in this order: `--skip-cache` or the dashboard axis `cache` set to a skip value; the target's decision in the manifest (`unclassified` never replays); an undeclared or too wide input; the mode (`prototype` vs `release`); another process editing declared inputs during the build; for a remote hit a different user or checkout path.
- **Remote cache.** `repo cache remote status` shows `configured: false` when no endpoint is set, `reachable: false` when the endpoint does not answer and `authorized: false` for a wrong token (Nx aborts a run on 401, ignores 403 on upload silently).
- **Workers.** Rerun with `--workers off` to separate a worker problem from a task problem. A build that fails with "Captured implementation Source changed" saw a build-tool file change under it; rerun it.
- **Installed packages.** `Cannot find module` for a workspace package means the dependencies are older than the sources: `repo dependencies lock --all`, then `repo dependencies sync --all`. On a fresh clone the Nx setup may need the installed workspace links: if it stops with `Cannot find module`, run the sync first.
- **Nx reports `.nx/installation` missing.** Run `repo tools setup nx`.

### ✅️ Verified and not yet verified

Verified on 2026-10-20 (macOS arm64, Bazel 8.4.2, Nx 23.2.0; Linux arm64 container for the platform rows): real-Bazel cold, warm, after-`clean` and after-server-loss runs with the production task owner, early cutoff, tests as cached `bazel test`, `affected` against Bazel `rdeps` and Nx's locators, real Bazel and a pristine Nx against `repo cache serve` (remote hits after clearing the local cache, wrong and read-only tokens), persistent workers against real Bazel, the hermetic law, the Linux container setup, and the fixture suites of every component. Not yet verified: a production `repo --tool=bazel run` after the integration, Nx with the repository's patched install against a remote cache, a real remote executor and remote execution of any repository target, Windows, the devcontainer lifecycle, and every end-to-end chain under both tools (see Verification status).
