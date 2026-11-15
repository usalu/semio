# Map: how builds run on one machine today

Read-only exploration of `/Users/Shared/semio`. Paths are relative to the repo root unless absolute. `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `PROC` = `🧰️framework/🔨️modules/🏃️process`. Facts are cited with file:line; items marked GUESS were not verified.

## Headline findings

1. The default build backend is Nx (`LIB/🏗️build/🔌️backends/📋️registry/🔣️.json`, row `nx`, `default: true`, `NX_DAEMON=false`). Nx's own `cache` capability is native and its `single-flight` capability is `missing`, so two concurrent cold builds can both run a task.
2. Cargo runs with `rustc-wrapper = "repo"` and `build.rustc` pointed at a refusal script (`.cargo/config.toml` lines 1-10). Real compiles only proceed when `repo` issues them, so a bare `cargo` cannot compile.
3. The one host-wide admission decision is `evaluateHostCapacityV1` (`PROC/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts:128`), called from `withHostCapacityV1` (`PROC/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts:188`). A job that may not start is queued as a waiter file and polls; it is not failed, except when the queue is full (`maximumWaiters`).
4. Measured inputs are free memory, swap, memory-pressure level, load, open files, attached disk images, and cgroup/PSI on Linux (`PROC/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts`).
5. The resident service (`LIB/🏗️build/🏡️resident/`) serves plan requests over local endpoints and has a warm delegate pool (`🏊️delegate/🟨️.mjs:37,90`). `createDelegatedExecutionPortV1` is the cleanest existing seam for handing work to another process or machine.
6. Local caches: Nx at `.cache/b/r/m/nx/artifacts` (3 GB cap), cargo target and build dirs under `.cache/b/r/m/u/l/d/`, and a content-addressed unit cache keyed by `sha256(toolchain, normalized rustc args, source sha, extern sha, env)` (`LIB/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts:107`). Cargo target dirs are not portable; the unit-cache key is designed to be lane-neutral (`laneNeutralV1`, line 69).
7. Toolchain is pinned to `nightly-2026-07-20` with wasm32-unknown-unknown and wasm32-wasip2 (`rust-toolchain.toml`). The toolchain digest is an input to the unit key, but how machines obtain identical builds was not confirmed.
8. A remote-execution worker protocol exists (`LIB/🏗️build/🌐️remote-execution/`, schema `🏃️execution/🧬️schema/🌐️remote-execution.json`), with register, poll, heartbeat and task result. Whether a backend routes real builds through it was not verified. The remote cache is off unless an endpoint is configured (`LIB/🏗️build/🌐️remote-cache/🟦️.ts`, `resolveRemoteCacheContractV1`).

## 1. Command surface and the layers for one build

Entry and routing:
- `repo` entry: `LIB/🏗️build/⌨️command/📥️entry/🟦️.ts:12` `runCapturedRepositoryKernelV1` binds the captured command proof, then `withRepositoryRuntimeEntriesV1`.
- Front door (cheap, no build code loaded): `LIB/🏗️build/🚪️door/🟦️.ts`. `planBuildDoorV1` (line 85) picks one of full, light, help, refused or dashboard from the catalogue alone. Light commands are `leases`, `doctor`, `capacity`, `fleet`, `scheduler` (lines 41-47). `runBuildDoorV1` (line 130) returns `"full"` only when the implementation must start.
- Invocation: `LIB/🏗️build/🧭️invocation/🟦️.ts`. `resolveBuildSelectionV1` (line 40) rewrites public targets such as `workspace:build`, `workspace:test` and `@semio-tech/framework-os-dev:*` into Nx `run` arguments. `resolveBuildModeV1` (line 145) resolves prototype or release mode (`--configuration`, `-c`, `:suffix`, `SEMIO_BUILD_MODE`) and refuses conflicts. `parseBuildInvocationV1` (line 173) picks the tool: `SEMIO_BUILD_TOOL`, else the registry default (`nx`). `nxBuildArgumentsV1` (line 192) maps portable flags to Nx flags. `--parallel` is set by `semioBuildParallelFlag`.

Owner declarations:
- Each project declares targets in `📋️project.json`. Example Rust plugin: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📋️project.json`, target `build`: `executor: "repo:script"`, `cache: true`, `command: "repo ./📜️script.ts build"`, `cwd` = the package dir.
- The TypeScript sibling `…/📦️packages/🟦️typescript/📋️project.json` uses the same executor, and its script calls `runArtifactTypeScriptPackageMain` (`PROC/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts`).
- The Nx plugin `LIB/🟨️.mjs` maps each target to an executor: `@semio-tech/repo-lib:command` or `:owner-command` (`projectBuildTargetForNxV1`, around line 85; the executor choice is at the `executor:` line). Cache is disabled unless the outputs lie in the `rust` or `products` cache partitions (`nxOutputsCacheableV1`, line ~78; `LIB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`).

Trace of one Rust (cargo) build:
1. `repo run <project>:build` goes through the door (full), then `resolveBuildInvocationV1` (mode and selection), then `parseBuildInvocationV1` (tool = nx).
2. Nx schedules the task. Its executor runs `repo ./📜️script.ts build` in the package directory. That script calls `runArtifactRustPackageMain` (`PROC/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts`).
3. Before running cargo, `withBuildCapacityV1` (`PROC/🏗️build/🌡️host-capacity/🟦️.ts:134`) sizes the request, checks the disk (`admitBuildDiskV1`), then calls `withHostCapacityV1`. The grant gives a job count, which is written into argv via `grantedBuildArgumentsV1` (line 82) and into the environment via `SEMIO_BUILD_PARALLEL`.
4. The cargo launcher `PROC/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` (about lines 170-233):
   - creates a jobserver with exactly the admitted tokens (`createJobserverV1`, line ~218, tokens file under `TMPDIR`);
   - builds the rustc broker, sized to a share of physical memory;
   - runs `ensureRepositoryCargoLockV1`;
   - executes `cargo` with `RUSTC_WRAPPER` = the scheduler binary (`semio-build-scheduler`, built by `PROC/🏃️process/🦀️cargo/🧰️native-helpers/🟦️.ts:39-40`), `CARGO_BUILD_JOBS` = tokens, and, when a lane is set, the unit-cache environment (`unitCacheEnvironmentV1`).
5. `LIB/🏗️build/…` workspace command `LIB/🗂️workspaces/🦀️cargo/🏃️command/🟦️.ts:44` is the other entry. It takes `acquireCargoBuildLeaseV1` on the build dir and target dir, then `execute("cargo", …)` with `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` from provenance.
6. Per rustc call, the unit cache (`LIB/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts`, `🏁️entry.ts`) is consulted; a hit replays outputs, a miss runs the compiler under the jobserver.

Trace of one wasm plugin build:
- Same executor path, but the package is a `[package.metadata.component]` plugin. `componentWorkspacesV1` (`LIB/⚡️caching/🦀️cargo/🟦️.ts:26`) discovers workspaces from manifests; `componentPassPlanV1` (line 57) makes one pass per workspace per profile with `target: "wasm32-wasip2"`; `nativeBuildPipelinePlanV1` (line 69) attaches jobs and per-job environment.
- Target flags come from `.cargo/config.toml` `[target.wasm32-wasip2]` (`-C link-arg=--max-memory=536870912`, `-zstack-size=8388608`) and `[target.wasm32-unknown-unknown]` (`getrandom_backend="wasm_js"`, `-zstack-size=16777216`).
- The exact argv assembly (`--target`, `--profile`) was not located; the grep for the flag literal returned nothing. GUESS: it lives in the native-build pipeline under `PROC/📦️artifacts/🏗️native-build/`, which was not read.

Trace of one TypeScript/vite build:
- Same `repo:script` executor, script `repo ./📜️script.ts build` calls `runArtifactTypeScriptPackageMain` (`PROC/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts`).
- Vite: `LIB/⚡️caching/🌐️vite/` (`🏗️build/🟦️.ts`, `🎟️capacity/🟦️.ts`, `💾️storage/🟦️.ts`, `🌐️service/🟦️.ts`). `runViteBunxDev` (`🌐️service/🟦️.ts`) starts dev via `bunx`. The vite build storage and capacity internals were not read.
- Playground outputs go under `.cache/b/p/m/vite/` and `.cache/b/p/m/browser/` (`LIB/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs:59,119,245`).

Parallelism, summary: nx `--parallel` (from `semioBuildParallelFlag`) is granted by the host-capacity gate. Cargo jobs are the jobserver tokens. The rustc broker admits memory per compile. The scheduler (`LIB/🏗️build/🗓️scheduler/🟦️.ts`) is a separate fair queue for `repo scheduler`.

## 2. Cargo specifics

`.cargo/config.toml` (complete facts):
- `[build] rustc = <refusal script>`, `rustc-wrapper = "repo"`, `target-dir = ".cache/b/r/m/u/l/d/t"`, `build-dir = ".cache/b/r/m/u/l/d/b"`, `rustflags = ["-Z","threads=0"]`.
- `[unstable]`: `embed-metadata = "DoNotEmbed"`, `fine-grain-locking = true`, `build-dir-new-layout = true`, `checksum-freshness = true`.
- `[target.wasm32-unknown-unknown]` and `[target.wasm32-wasip2]` as above.
- `[target.'cfg(all(target_os = "linux", target_env = "gnu"))']`: `linker-features=+lld`, `link-self-contained=+linker`.
- `[env] RUST_MIN_STACK = "67108864"`.
- Not found in config: `--remap-path-prefix` (only appears as a token in the unit-cache key normalizer, `🔏️key/🟦️.ts:19`), `CARGO_INCREMENTAL` (no hit; GUESS: default on for dev profiles), `-Z` flags beyond those listed. Profiles are in the root `Cargo.toml` (not read) and selected by `buildModePolicy(mode).cargoProfile`.
- Jobserver: `CARGO_MAKEFLAGS`, with POSIX stand-ins that pass descriptors to build scripts (`🗃️unit-cache/🎰️jobserver/🟦️.ts`, `🚪️launch/🟦️.ts`, `🏗️buildscript/🟦️.ts`).

Artifact locations (`.cache/b/...`):
- Cargo target `.cache/b/r/m/u/l/d/t`, build dir `.cache/b/r/m/u/l/d/b`, lanes `m/u/l/<lane>` (comment in `.cargo/config.toml`).
- Lease directory `.cache/b/l/m/data` (used by many modules, e.g. `LIB/🏗️build/🔒️leases/⌨️command/🟦️.ts:8`).
- Nx cache `.cache/b/r/m/nx/artifacts` (root `nx.json` `cacheDirectory`, `maxCacheSize: 3GB`).
- Other: `.cache/b/c` (storage budget, `LIB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts:25`), `.cache/b/d/m/bun-cache`, `.cache/b/d/m/j`, `.cache/b/t/m/downloads` (`LIB/🏗️build/🗄️cas/🧬️schema/🔣️.json:154-178`), `.cache/b/g/m/task-keys` (`LIB/🏗️build/🧭️invocation/🏃️run/🟦️.ts:15`).

Workspaces: many, not one. Root `Cargo.toml`, `✏️s/Cargo.lock`, and per-plugin `Cargo.lock` files (e.g. `✏️s/🔌️plugins/✒️writer/.../Cargo.lock`). The unit of scheduling is one cargo invocation per workspace pass (componentPassPlanV1), with rustc calls inside it going through the unit cache. Individual rustc invocations are the cacheable unit.

## 3. Pressure and limits

Measurements (`PROC/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts`):
- macOS: `vm_stat` (free, file-backed, purgeable pages, compressor pages), `sysctl -n vm.swapusage`, `sysctl -n kern.memorystatus_vm_pressure_level`, `sysctl kern.num_files kern.maxfiles`, `hdiutil info` (attached images), `loadavg()[0]` (line 174). Swap-out rate from `Swapouts`/`Swapins` deltas.
- Linux: `/proc/meminfo`, `vmstat` pswpout/pswpin, PSI pressure, cgroup memory max/current, `file-nr`.
- Windows: `Get-CimInstance` counters.
- Cores: `readHostCoresV1`, including cgroup CPU quota (line 98).

Decision point (quote, `PROC/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts`):
- Line 134: `observed = host.availableMegabytes-floor-unrealized`, `budget = floor(share*budgetBasisPoints/10000) - committed`, `binding = budget<observed ? "budget" : "available"`.
- Line 137: `loadRoom = perCoreCeiling*cores - max(host.load, ledgerCpu)`.
- Line 142 (hard stops, refuse): `pressure === "critical"` → `pressure-critical`; swap used above ceiling and actively swapping → `swap`; swap-out rate above `outMegabytesPerSecond` → `swap-activity`.
- Line 120 `softHostReasonV1` (waitable): pressure `warn` → `pressure-warn`; open files above `ceilingBasisPoints` → `files`; attached images at or above `attaches.maximum` → `attaches`.
- Line 108 `hostFloorMegabytesV1`: floor = share of total memory clamped to [floorMinimumMegabytes, floorMaximumMegabytes].
- Line 143 `drainedRoom` and `blocked`/`heads` logic: a waiter can become a strict barrier after `bypassMilliseconds` or `maximumBypasses`, so it cannot starve.
- Line 166: a session with no live grant is admitted with minimum jobs even above the load ceiling (so every agent progresses).

Gate loop (`PROC/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts:188-225`): writes a waiter file, samples the host, takes a lock, calls `evaluateHostCapacityV1`, and if the verdict is admit writes a grant file. Otherwise it sleeps and retries. Waiting is not timed out except by cancellation, the gate being off (`SEMIO_HOST_CAPACITY=off`, line 192 bypass), or a full queue (`maximumWaiters`, line 206, schema maximum 4096).

Authored constants (`PROC/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json`): protocol values lines 66-112 (`growSpareJobs: 1`, `growSettleAttempts: 2`, `maximumJobs: 1024`, `staleSampleMilliseconds: 30000`). The memory and load policy values (`floorBasisPoints`, `budgetBasisPoints`, `perCoreCeiling`, etc.) are in the policy constant of `PROC/🏗️build/🌡️host-capacity/🎛️control/🟦️.ts` (`BUILD_HOST_CAPACITY_POLICY_V1`), which was not read; values not stated here.

Job-level limits (`LIB/🏗️build/🧱️job-limits/🟦️.ts`):
- Per job: `runLimitedJobV1` (line 77) spawns the job, attaches a limiter (cgroup-v2 on Linux, job object on Windows, watchdog fallback), kills the process tree on breach, and returns `memory-exceeded`. Swap is always 0 (line 12).
- Constants (`LIB/🏗️build/🧱️job-limits/🧬️schema/🔣️.json`, `Rules.const`): `softRatioBasisPoints 9000`, `headroomBasisPoints 12500`, `breachGrowthBasisPoints 12500`, `minimumLimitMegabytes 64`, `watchdogPollMilliseconds 100`, `cgroupPollMilliseconds 20`, `graceMilliseconds 2000`.
- Limit per learned class: `max(1.25 x learned peak, floor, 64 MB)` (`jobEstimateV1`, line 35).

Fair scheduler (`LIB/🏗️build/🗓️scheduler/🟦️.ts`): `decideTickV1` (line 143) is the pure admission step for the queue. Start condition (line 156): `running === 0 || reserved + job.reserveMegabytes <= budgetMegabytes`, and, if `availableMegabytes` is known, `reserve + availableHeadroomMegabytes <= availableMegabytes`. Also `maxConcurrent`, `maxPerAgent`, `sharedCompilerLanes`, aging, and virtual-time fairness (`pick`, line 136). Admission is preemption-free.

Single decision point to answer "may this job start now": `evaluateHostCapacityV1` (decision module, line 128), reached via `withHostCapacityV1` (gate, line 188) and `withBuildCapacityV1` (host-capacity, line 134).

## 4. Resident service

- Public surface: `LIB/🏗️build/🏡️resident/🟨️.mjs` exports the governor (`createGovernorV1`, `BusyErrorV1`), workspace state, the frame protocol, capability tokens (`mintCapabilityV1`, `verifyCapabilityV1`), endpoints (`endpointsV1`, `hostEndpointsV1`), the graph port, watch supervisor, client (`requestPlanV1`, `requestHostStatusV1`, `holdHostV1`, `connectHostV1`), and `runHostForegroundV1` / `spawnHostServiceV1`.
- Host service: `🏛️service/🌐️host/` (with `🧊️pool` and `🚦️governor`). Client: `🚪️client/🛰️plan/`. Commands talk to it over local endpoints with capability-checked frames (`📨️protocol/`). Endpoints are directory- and host-scoped (`📍️endpoint/🏠️host/`).
- Delegate and executors: `🏊️delegate/🟨️.mjs` line 37 `createWarmPoolV1` (a pool of warm processes spawned from a preload, sized and idle-timed), line 90 `createDelegatedExecutionPortV1` (routes an execution id to an executor, with an argv builder and a cold fallback). `🏭️executor/🟨️.mjs` is the executor side.
- Slice pool: `🏗️composition/🌐️host/👷️slice-pool/` (workers for graph inference, with `🧱️limit` memory limits, `🌡️pressure`, `🧹️reap`). Memory for a plan is reserved in the host gate via `reserveGraphPlanCapacityV1` (`PROC/.../🌡️host-capacity/🎫️gate/🟦️.ts`, reserved in `🟦️.ts:118`).
- Journal: `📜️journal/` (event records of runs); `🕰️graph-hooks/` (watch-driven graph invalidation).
- Is it a natural hand-off point? Yes for graph and slice work: `createDelegatedExecutionPortV1` already has the shape of "submit an id, get an answer from a worker", and the remote-execution protocol (`🌐️remote-execution/🟦️.ts`: registration, poll, heartbeat, task result) has the same shape. For cargo, the handoff point is the launcher's `execute(...)` call, not the resident service.

## 5. Local caching layers

| Layer | Stores | Where on disk | Key | Machine-independent? |
|---|---|---|---|---|
| Nx task cache | task outputs (rust/products partitions only) | `.cache/b/r/m/nx/artifacts`, 3 GB cap (`nx.json`) | Nx hash over namedInputs (`sharedGlobals`: node, bun versions, `SEMIO_BUILD_MODE`; `default`, `production`, plugin engine digests) | Partly. Input hashes use workspace-relative globs, but outputs restored from cache can embed absolute paths (GUESS). |
| Cargo fingerprints and dep-info | cargo freshness | target dir `.cache/b/r/m/u/l/d/t` | cargo fingerprint | No. Dep-info records absolute paths (GUESS, standard cargo behavior). Not portable across worktree paths. |
| Unit cache (`⚡️caching/🦀️cargo/🗃️unit-cache/`) | per-rustc outputs (replayed) | `m/u` content-addressed root, per-lane `m/u/l/<lane>` (`.cargo/config.toml` comment) | `unitKeyV1` = sha256 of `[namespace, toolchain, normalized args, source sha256, extern sha256s, env pairs]` (`🔏️key/🟦️.ts:107`). `laneNeutralV1` strips lane roots (line 69). | Designed yes: lane- and path-neutral keys. Outputs may still contain paths (GUESS); needs verification. |
| Storage ledger / enforcement (`⚡️caching/💾️storage/`) | usage ledger, budgets, compiler ingress | `.cache/b/c` and partitions in `🔒️enforcement/📦️budget/🔣️policy.json` | per partition | Not a content cache; bookkeeping. |
| CAS (`🏗️build/🗄️cas/`) | downloads and bun cache | `.cache/b/t/m/downloads`, `.cache/b/d/m/bun-cache` | not read | GUESS: content-addressed. |
| Sealed (`🏗️build/🏛️sealed/`) | promoted, partitioned results (promotion, retention) | not read; README only (`🏛️sealed/README.md`) | not read | Unknown. |
| Incremental (`📏️incremental/`) | benchmark and acceptance of incremental builds | not a store; bench tooling | n/a | n/a |
| Vite (`⚡️caching/🌐️vite/`) | dev/playground outputs | `.cache/b/p/m/vite/` | not read | Unknown. |
| Repo artifacts (`⚡️caching/📦️artifacts/`) | native build artifacts, wasm, executables | `.cache/b/l/m/data`, package `.🧾️store/build.json` outputs | not read in detail | Unknown. |

## 6. Toolchain identity

- Rust: `rust-toolchain.toml` pins `channel = "nightly-2026-07-20"`, profile minimal, components rust-src, llvm-tools-preview, clippy, targets wasm32-unknown-unknown and wasm32-wasip2. The rustup install of that channel is the mechanism (GUESS: rustup is installed by the bootstrap; not traced). The toolchain digest is an input to the unit key (`store.toolchain.digest` in the launcher, `🧰️native-helpers`).
- Cargo env: `CARGO_HOME` and `RUSTUP_HOME` default to `~/.cargo` and `~/.rustup` (`LIB/⚡️caching/🦀️cargo/🧾️provenance/🟦️.ts:82`).
- Node and Bun: versions are read at run time by Nx `sharedGlobals` (`repo tools exec --light bun -- --version`, `node -p process.platform+arch`) and added to the Nx hash (`nx.json` namedInputs). Bun lockfile `bun.lock`, `bunfig.toml`.
- Nx itself: installed under `🧰️framework/.../🔧️tools/🧶️nx/`; Bazel is also a registered backend (`.bazelversion`, `MODULE.bazel`), not default.
- `~/Library/Caches/semio/runtimes`: not confirmed. A grep under `🧰️toolchains` for `Library/Caches` and `runtimes` returned no hits. Native bootstrap: `LIB/⚡️caching/🚀️bootstrap/` (not read).
- Identity across machines: NOT guaranteed by anything read here. The pin is the channel name plus the digest in the unit key. Whether two machines produce byte-identical toolchains was not verified. `repo tools` / `🧰️tool-setup/` (not read) is the likely mechanism.

## 7. Seams for the fleet goal

(a) Shared cache lookup and upload:
- Least distorting: a second store behind the unit cache store `LIB/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts` (lookup by `unitKeyV1`, replay outputs, upload on miss), keeping the local store as first tier. The launcher already routes every rustc through `RUSTC_WRAPPER` (the scheduler), so the lookup sits behind one wrapper boundary.
- For Nx tasks: the backend row already declares `remote-cache: native`; `LIB/🏗️build/🌐️remote-cache/🟦️.ts` `resolveRemoteCacheContractV1` (endpoint, instance, access, upload, compression, download) is the contract to point at a shared service. Off unless an endpoint is set.
- Caution: Nx `single-flight` is `missing`; a shared cache does not stop two cold runs on one machine.

(b) Remote execution hand-off:
- Compile unit: the cargo launcher `execute(...)` call in `PROC/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` (around line 227), or the unit cache's miss path. This is the smallest unit with a complete content key. Risks: jobserver tokens, the rustc broker and the lease are host-local; a remote compile needs the same toolchain digest and sysroot.
- Graph and slice work: `createDelegatedExecutionPortV1` (`LIB/🏗️build/🏡️resident/🏊️delegate/🟨️.mjs:90`) as the port; the worker protocol in `LIB/🏗️build/🌐️remote-execution/` (register, poll, heartbeat, result) is the existing wire shape.
- Offload trigger: `evaluateHostCapacityV1` already returns `pressure-critical`, `swap` and `load` refusals. A "refused for memory here" verdict is the natural trigger to try a remote worker before waiting.

## Unverified or not read (honest gaps)

- Root `Cargo.toml` profiles; `CARGO_INCREMENTAL` and `--remap-path-prefix` usage (not located).
- Exact wasm argv assembly (`--target`, `--profile`) in the native-build pipeline (`PROC/📦️artifacts/🏗️native-build/`).
- `🔒️leases` internals (only the lease directory path and `checkResourceLeaseControl` were seen).
- `BUILD_HOST_CAPACITY_POLICY_V1` numeric values (`🎛️control/🟦️.ts`).
- Sealed, incremental, vite storage, and `⚡️caching/📦️artifacts` internals.
- Whether `🌐️remote-execution` is wired into any real build path.
- Toolchain bootstrap (`🧰️tool-setup`, `⚡️caching/🚀️bootstrap`) and the `~/Library/Caches/semio/runtimes` location.
- Process note: a broad recursive grep over the library exceeded 120 s and was stopped (task brlyvyotv). No builds were run, and no git or file-changing commands were issued.
