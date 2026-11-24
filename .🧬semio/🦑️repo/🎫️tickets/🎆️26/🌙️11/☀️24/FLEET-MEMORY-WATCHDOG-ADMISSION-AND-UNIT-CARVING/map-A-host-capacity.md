# Map A: Host Memory Capacity and Admission Gate (read-only audit)

Ticket: FLEET-MEMORY-WATCHDOG-ADMISSION-AND-UNIT-CARVING
Scope: host memory sampling, admission decision, reservations, runtime watch, alarms, tests, gaps.
Method: read-only. Only `git grep`, `git ls-files`, `git show` (read-only), `grep`, `awk`, `sed`, `python3 -I` for JSON reads, and file reads. Nothing was built, tested, run, or edited. No timings were measured; every millisecond figure below is a configured limit, not an observed cost.

Path aliases used below (repo-relative):

- **HC** = `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/` (core engine, `🎫️gate`, `📏️decision`, `📡️sampling`, `🧬️schema`, ...)
- **HCP** = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/` (product layer: build-invocation wrapper, `🧹️disk-recovery`, `🧰️`-independent gate users, `⌨️command`)
- **POL** = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json` (key `hostCapacity`, plus `workers`)
- **TOOLS** = `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json` (tool registry with `capacity.mode`)

Important naming note: `grep` in this shell is an ugrep alias that silently misses matches on emoji paths (see memory "Shell grep Alias Misses Matches"). All searches below use `git grep` or explicit `awk`/`python3`.

---

## 0. Headline numbers (from `POL` and `HC/🧬️schema/🔣️.json`)

| Item | Value | Source |
|---|---|---|
| Floor | `clamp(12.5% of total, 1536 MB, 4096 MB)`, never above total | POL `memory.floorBasisPoints/Minimum/Maximum`; `HC/📏️decision/🟦️.ts:108-111` |
| Budget share | 100% (`budgetBasisPoints 10000`) of `total - floor - unmanaged` | POL `memory`; `decision:216,221` |
| Heartbeat (grant observation) | 15 000 ms; first beat at 3 000 ms | POL `wait.heartbeatMilliseconds`; `HC/🎫️gate/🟦️.ts:421` |
| Waiter poll | 1 000 ms (x 0.75..1.25 jitter) | POL `wait.pollMilliseconds`; `gate:358` |
| Shared sample cache (`sample.json`) | max age 1 000 ms | POL `wait.sampleMaximumAgeMilliseconds`; `HC/🎫️gate/📖️status/🟦️.ts` (`currentSample`) |
| In-process "available" cache | 5 000 ms | `Limits.availableAgeMilliseconds` (HC schema); `HC/📡️sampling/🟦️.ts:183` |
| Sample considered stale (aged to partial) | 30 000 ms | `Protocol.staleSampleMilliseconds`; `decision:404-409` |
| Grant liveness window (heartbeat mtime) | 21 600 000 ms (6 h) | POL `wait.staleMilliseconds`; `decision:59-63` |
| Lock stale | 10 000 ms | POL `wait.lockStaleMilliseconds` |
| Deadline batch / interactive | 300 000 / 120 000 ms | POL `wait.deadlineBatchMilliseconds` / `deadlineInteractiveMilliseconds` |
| Starved refusal | 1 800 000 ms | POL `wait.starvedMilliseconds` |
| Batch aging to interactive | 300 000 ms | POL `wait.agingMilliseconds` |
| Override after nothing of ours runs | 120 000 ms | POL `load.overrideAfterMilliseconds` |
| Ramp (new grant counts in full) | 60 000 ms, max 2 ramping grants | POL `memory.rampMilliseconds`, `maximumRamping` |
| Settled grant | after 900 000 ms, keeps 150% of observed | POL `memory.settledAfterMilliseconds`, `settledKeepBasisPoints` |
| Child start cost | 512 MB per job while booting | POL `memory.childStartMegabytes` |
| Per-core load ceiling | 2 x cores (own grants + load) | POL `load.perCoreCeiling` |
| Command timeout (sampler commands) | 10 000 ms; `vm_stat` 2 000 ms; optional probes (`hdiutil`) 3 000 ms | HC schema `Limits`; `sampling:129-132,169-176` |
| Command output cap | 4 MiB | `Limits.commandOutputBytes` |

---

## 1. Sampling: how host memory is read

### 1.1 Entry points

- `readHostSampleV1(policy, probe, previous, now, env)` at `HC/📡️sampling/🟦️.ts:204-209`: counters + cores + total + load, then `reduceHostSampleV1`.
- `readHostCountersV1(probe)` at `sampling:151-166`: the only per-platform switch.
- `currentSample(...)` in `HC/🎫️gate/📖️status/🟦️.ts` (the function is named `currentSample`): reads `sample.json` in the ledger directory first, returns it if younger than 1 000 ms and the declaration key matches, otherwise calls `readHostSampleV1` and writes `sample.json` (tmp + rename via `writeJson`). If sampling throws and a previous sample exists it returns `agedHostSampleV1(known)`; otherwise the error propagates to the waiter.
- `createHostAvailableBytesV1` / `hostAvailableBytesV1` at `sampling:173-195`: a separate, synchronous, in-process reader used by telemetry and planners (see 1.5).

### 1.2 Per platform

**macOS (darwin)** — `sampling:152-156`, `parseDarwinCountersV1` at `sampling:28-32`:

- `vm_stat` (awaited through the probe, timeout = `commandTimeoutMilliseconds` 10 s from `runHostCommandV1`, `sampling:212-214`).
- `sysctl -n vm.swapusage` (swap used).
- `sysctl -n kern.memorystatus_vm_pressure_level` (the kernel's own level: `4` critical, `2` warn, `1` normal; `sampling:31`).
- optional: `sysctl kern.num_files kern.maxfiles` and `/usr/bin/hdiutil info` (attached images), both wrapped in `boundedSignalV1` (3 s, `sampling:129-132`) and `.catch(() => null)`.
- Available = `Pages free + max(inactive + speculative, file-backed + purgeable)` x page size (`sampling:30-31`). Compressor pages (`Pages occupied by compressor`) are reported separately as `compressedBytes`.
- Swap traffic = `Swapouts + Swapins` pages (cumulative; the rate is derived against the previous record in `reduceHostSampleV1`, `sampling:116`).

**Linux** — `sampling:44-48`, reads in `readHostCountersV1` at `sampling:157`:

- `/proc/meminfo` (required; missing = `countersUnavailable` error, `sampling:157`), `/proc/vmstat` (required), `/proc/pressure/memory` (PSI, optional), `/proc/sys/fs/file-nr` (optional), `/proc/self/cgroup` plus the cgroup v1/v2 memory limit/usage files (`cgroupMemoryFilesV1`, `effectiveCgroupMemoryV1`, `sampling:55-74`).
- Available = `MemAvailable`, capped by `cgroup limit - cgroup usage` when a cgroup limit exists.
- Swap used = `SwapTotal - SwapFree`; swap traffic = `pswpout + pswpin` pages.
- CPU count: `readHostCoresV1` reads `cpu.max` (v2) or `cpu.cfs_quota_us`/`cpu.cfs_period_us` (v1) (`sampling:100-106`).
- No attached-image count on Linux (`attachedImages: null`, `sampling:47`).

**Windows (win32)** — `sampling:159-163`, `windowsCountersV1` at `sampling:135-137`:

- `GlobalMemoryStatusEx` through `bun:ffi` `dlopen("kernel32.dll")` (`HC/🪟️native-memory/🟦️.ts:44-56`, loaded lazily, `createRequire(import.meta.url)("bun:ffi")`). This is in-process; no command is spawned for memory.
- Available = `min(availablePhysical, availablePageFile)` (commit). Swap used, swap traffic, compressor, open-file table: all `null` (unknown never raises pressure).
- If the native call fails, the counters become `partial: true` with `os.freemem()` (`sampling:164-165`), and `countersPartial` is announced once in EN and DE.

**Any other platform**: partial counters with `os.freemem()`.

### 1.3 What "available", "pressure", "swap" mean and where they are computed

All in `reduceHostSampleV1`, `HC/📡️sampling/🟦️.ts:109-118`.

- `total` = `min(host total, cgroup limit)` when a constraint exists; `physical` = real physical memory (`sampling:110-111`).
- `available` (MB) = `min(total, availableBytes)`.
- **Pressure** is the worst of three levels (`sampling:112-115`):
  1. native level: macOS `kern.memorystatus_vm_pressure_level` (`counters.pressureLevel`); on Linux there is none, so PSI `some avg10` is used (`fromStall`: >= `psiCriticalPercent` 30 critical, >= `psiWarnPercent` 10 warn);
  2. available share of `pressureTotal`: `< availableCriticalBasisPoints` 600 (6%) critical, `< availableWarnBasisPoints` 1 500 (15%) warn;
  3. compressor occupancy of physical memory: `>= compressedCriticalBasisPoints` 5 500 (55%) critical, `>= compressedWarnBasisPoints` 3 500 (35%) warn (macOS only).
- **Swap used**: `swapUsedMegabytes` from `vm.swapusage` (macOS) or `SwapTotal - SwapFree` (Linux); `null` on Windows.
- **Swap activity**: `swapOutMegabytesPerSecond` = delta of cumulative swap traffic / seconds between two records (`sampling:116`). This is traffic, not occupancy (the comment at `sampling:16` says so explicitly).
- **Swap stop** `hostSwapStopV1`, `decision:206-211` (all thresholds are shares of total memory, so the same judgment on 16 GB and 64 GB hosts):
  - `"swap"` when swap used > 25% of total (`swap.ceilingBasisPoints` 2500) AND traffic >= 0.6 bp/s of total (`activeBasisPointsPerSecond` 0.6) AND available < 25% of total (`shortBasisPoints` 2500);
  - `"swap-activity"` when traffic > 40 bp/s of total (`outBasisPointsPerSecond` 40 = 0.4 % of total per second).
- `hostUnderPressureV1` (`decision:132-134`): pressure != normal OR swap stop. Used to exclude a run from learning (`pressured`).
- `softHostReasonV1` (`decision:136-141`): pressure warn, open files > 80% of the maximum (`files.ceilingBasisPoints` 8000), attached images >= 32 (`attaches.maximum`).
- Hard stop (`decision:227`): `host.pressure === "critical" ? "pressure-critical" : hostSwapStopV1(...)`.

### 1.4 Cost and caching (by construction, not measured)

- Machine-wide: one `sample.json` shared by all processes; reuse window 1 000 ms. Waiters poll about once per second with jitter, so roughly one sampling per poll interval machine-wide when the cache is stale.
- macOS per fresh sample: `vm_stat`, 3 x `sysctl`, and optionally `hdiutil info` (3 s bound). Each is a spawned process via `execFile` (`sampling:212-214`).
- Linux per fresh sample: about 6 small `/proc` and cgroup file reads (`sampling:157`) plus the cgroup chain (`cgroupMemoryFilesV1` reads). No process spawn.
- Windows per fresh sample: in-process FFI calls (no spawn) for memory. The children table (used by the lender, see 3.2) spawns `powershell.exe` with a 10 s timeout (`HC/🎫️gate/🟦️.ts:196`).
- Sync reader `hostAvailableBytesV1` (used by scheduler/telemetry/bench planning) spawns `vm_stat` with `spawnSync` and a 2 s timeout on macOS (`sampling:175`); it blocks the event loop for up to 2 s, cached 5 s (`sampling:183`).
- Measured costs: not measured in this audit.

### 1.5 Consumers of the samplers outside admission

- `hostAvailableBytesV1`: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📈️telemetry/🟦️.ts:73`, `HCP/📏️incremental/🏎️bench/🧭️planning/🏃️runner/🟦️.ts:14`, `HCP/🗓️scheduler/💾️store/🟦️.ts:30`, `HCP/🧾️ci-report/🗂️suites/📜️script.ts:55`.
- `readHostSampleV1` directly: offload admission `HCP/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts:22`, ci-report `📜️script.ts:51`.
- `observeTreeMegabytesV1`: test-worker learning `🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/📐️learning/🟦️.ts:1,24`, bench acceptance runner `HCP/📏️incremental/🏎️bench/🛂️acceptance/🏃️runner/🟦️.ts:5,143`, fleet-status readers `HCP/🚢️fleet-status/📥️readers/🟦️.ts:72-78`, ci-report `📜️script.ts:49-51`.
- Read-only status readers: `readHostCapacityStatusV1` callers at `HCP/🌐️remote-execution/🌡️load/🟦️.ts:132`, `HCP/🔒️leases/⌨️command/🟦️.ts:16`, `HCP/🪪️agent-sessions/⌨️command/🟦️.ts:46`, `HCP/🩺️doctor/🔎️probes/🎫️capacity/🟦️.ts:25`, `HCP/🚢️fleet-status/📥️readers/🟦️.ts:56`, the host-capacity `status` subcommand (`HCP/⌨️command/🟦️.ts:27`), the Bazel tool (`HCP/🔧️tools/🏗️bazel/📜️script.ts:933-935`).
  Note: these are not write-free. `currentSample` writes `sample.json`, and `hostBootAtV1` writes `boot.json` when missing (`status:` boot anchor code). Fleet status therefore writes to the ledger directory.

---

## 2. The admission decision, reservations and callers

### 2.1 Entry chain

Two entry points reach the gate:

1. **Build invocations**: `withBuildCapacityV1(args, env, body, ports)` at `HCP/🌡️host-capacity/🟦️.ts:159`. Called by the repo entry `HCP`-adjacent `📜️script.ts:121-124`, the invocation wrapper `HCP/⌨️command/🏃️invocation/🟦️.ts:9`, and bootstrap `HCP/🚀️bootstrap/🟦️.ts:103-104`.
   - It computes the ask with `buildCapacityAskV1` (`HCP/🌡️host-capacity/🟦️.ts:56-66`): returns `undefined` when the command is not in `policy.operations`, when `--help`/`-h` is present, or when `sealedCurrentReadV1` (a `show` with graph policy `current`) says it is a sealed read (`:70`). Unknown operation or sealed read = no admission.
   - Then `sizeHostRequestV1` (`HC/📏️decision/🟦️.ts:319-327`) builds the request, `admitBuildDiskV1` runs for the cache partition (`:173`), and finally `withHostCapacityV1` (`HC/🎫️gate/🟦️.ts:273`) is called inside `holdHostGrantV1`.
2. **Tool runs**: `runToolV1` in `HC`-adjacent `🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts:160-192`. Line 165:
   `const admit = options.admission === false || entry.capacity.mode === "none" || entry.capacity.mode === "inherit" && options.heavy !== true ? undefined : options.admission ?? defaultAdmission;`
   `createToolAdmissionV1` (`run:59-72`) builds the port over `withHostCapacityV1` with `sizeHostRequestV1` and the policy `BUILD_HOST_CAPACITY_POLICY_V1`. Installed as the default by the tool-registry command (`HCP/🧰️tool-registry/⌨️command/🟦️.ts:49`), so every tool run in a repo command gets it.

Entry point of the whole gate: `withHostCapacityV1` (`HC/🎫️gate/🟦️.ts:273-362`), then `holdHostGrantV1` (`:370-481`) for the body.

### 2.2 The decision function

`decideHostCapacityV1` (`HC/📏️decision/🟦️.ts:215-286`) is pure. `evaluateHostCapacityV1` (`:191-202`) first applies lending (`lendHostHoldersV1`, `:150-177`), then calls `decideHostCapacityV1`.

Inputs: `policy`, `host` (sample + `unmanagedMegabytes`), `ledger` (live grants), `waiters`, `nowMs`.

Formula, exactly as coded:

- `floor = hostFloorMegabytesV1(policy, host)` (`:216`).
- `cap = max(0, total - floor)`; `unmanaged = host.unmanagedMegabytes ?? 0`; `share = max(0, cap - unmanaged)` (`:216`).
- For each active grant (`lentSinceMs` undefined):
  - `ramping(g)` = not quiet AND `now - (resumedAt ?? admittedAt) < (g.rampMilliseconds ?? 60000)` (`:217`).
  - `keptHostMegabytesV1(policy, g, now)` (`decision:181-188`): for a farm child or `graph-farm` label: `min(reserved, max(observed, jobs x 512))`; for a settled grant (>= 900 000 ms and observed > 0): `min(reserved, max(3072, ceil(observed x 1.5)))`; for an idle ramp grant (>= 30 000 ms, observed < 25% of reserved): `min(reserved, max(3072, ceil(observed x 1.5)))`; else `reserved`.
  - `unrealizedOf(g)` = `max(0, kept - observed)` while ramping, else 0 (`:219`). This is the **reserved-but-not-resident** part.
  - `committedOf(g)` = `max(kept, observed)` while ramping, else `observed ?? reserved` (`:219`).
- `unrealized` = sum of unrealized; `committed` = sum of committed (`:220`).
- `observed = available - floor - unrealized` (`:221`) — the "physical" room.
- `budget = floor(share x 100%) - committed` (`:221`) — the "budget" room.
- `binding = budget < observed ? "budget" : "available"` (`:221`).
- `headroom = min(observed, budget)` (`:223`).
- `loadRoom = ceiling - max(load, ledgerCpu)` where `ceiling = 2 x cores` (`:223-224`), or Infinity when neither load nor our CPU is known.
- `soft = softHostReasonV1(...)`; `hard = pressure critical ? "pressure-critical" : hostSwapStopV1(...)` (`:226-227`).
- Waiters ordered by `orderHostWaitersV1` (`decision:126-129`): lent last, grow/elastic after others, interactive before batch, then the session with least held memory per unit of weight (fair share), then arrival, then id. Batch waiters older than `agingMilliseconds` rank as interactive (`:229`).

Per waiter (`:232-284`):

- `need(jobs) = min(fixed + jobs x perJob, cap)`.
- `fits(jobs)`: `need(jobs) - held <= freeObserved AND max(need(jobs), held) <= freeBudget` (`:236-237`), where `held` is what an upgrade replaces.
- Hard stop (`:243-247`): refuse with `hard`, except `shrunk` when the deadline has passed and the request has a `minimumMegabytes` that fits the room; refusals become `starved` when waited >= 1 800 000 ms.
- `grow` requests (`:249-257`): only when nothing else waits and no barrier; admitted with the largest job count whose memory and cores fit; reason `fits`/`reduced`; they never shrink.
- Normal request: jobs = largest candidate in `[preferredJobs .. minimumJobs]` where `fits(candidate)` and cores fit (`:268`). Fallback to `minimumJobs` for fresh sessions and light graph requests under the ceiling (`:269`).
- If memory does not fit: shrinkable requests (with `minimumMegabytes`) are admitted with the room that exists (`shrunk`, `:266`).
- Otherwise refuse with reason `memory`, `load`, `pressure-warn`, `files`, `attaches`, `ramp`, `quiet-limit`, `queue-order`, `never`, `starved`, `override` (`:274-276`).
- Barrier: a memory-refused batch request that would drain could become a barrier head (`:279-281`) after `bypassMilliseconds` (30 000) or `maximumBypasses` (6) overtakes; this prevents starvation.
- Override (`:276`): when nothing of ours runs and the wait exceeds 120 000 ms, the request is admitted at its minimum jobs.

Admission of an unsatisfiable request: `unsatisfiableHostRequestV1` (`decision:297-299`) plus `hostRequestNeverFitsV1` (`:290-293`) — requests that can never fit even when everything of ours drains are admitted with reduced reservation (`unsatisfiable`) or refused as `never` (named error `memoryNeverFits`).

Answer to "admitted-ahead": the progress line (`hostWaitingLineV1`, `gate:264-268`) reports `taken` = sum of reservations of requests decided before this one in the same evaluation, and `spare = headroom - taken`. "Reserved but not yet resident" is `held` = `unrealizedMegabytes`.

### 2.3 Reservation sizing (request)

`sizeHostRequestV1` (`decision:319-327`): `kinds = [operation.kind or input kinds] + "build"` (always adds the `build` kind; `:324`), `declared = max(kinds' megabytes, envelope megabytes matched by target)`, `perJob = min(declared, max(512, reach - fixed))` with `reach = half of cap` (`:322,325`). Fixed MB per operation from POL `operations`: run/run-many/affected/exec/watch/cargo 1536; show/graph 3072; dependencies 3584; command 2048; tool-format/tool-query 512. Kinds: test 1536, check 4096, generate 1024, build 2048, script 768, service 2048. Envelope: `^test-e2e` 6144 MB and 4 cores per job.

Learned estimates (`HC/📐️estimates`, `HC/🪜️phases`) rescale the request through `scaleHostRequestV1` and `hostClassCeilingV1`; learning is in `withBuildCapacityV1` (`HCP/🌡️host-capacity/🟦️.ts:193-199`) and skipped for pressured runs (`learnableHostRunV1`).

### 2.4 Reservation lifecycle

Ledger directory (`HC/🎫️gate/📖️status/🟦️.ts`, `hostCapacityDirectoryV1`): machine-wide, one per user, shared by all checkouts.
- macOS `~/Library/Caches/semio/host-capacity`; Linux `$XDG_CACHE_HOME/semio/host-capacity` or `~/.cache/...`; Windows `%LOCALAPPDATA%\semio\host-capacity`; override `SEMIO_HOST_CAPACITY_DIRECTORY`.
- Files: `grants/<id>.json` (grant), `grants/<id>.attached` (attached pids), `grants/<id>.wait.*` (wait markers), `grants/<id>.upgrade`, `grants/<id>/` rebuild folder, `queue/<arrival>-<id>.json` (waiter), `lock/` (directory with `owner`), `sample.json`, `boot.json`, `estimates.json`, `<id>.phases`.
- Writes use temp file + rename (`writeJson`, `writeFileSync` then `renameSync`).

Lifecycle:
1. **Queue**: waiter file written at `gate:293-294` (`writeJson(waiterPath, record)`).
2. **Decide under lock**: `takeLock` (`gate:312`) reads ledger and waiters (`readGrants`, `readWaiters`), evaluates (`:316`), and if admitted writes the grant (`writeJson(grantPath(directory, request.id), grant)`, `:326`), removes the waiter file (`:326`).
3. **Heartbeat**: `holdHostGrantV1` `heartbeat()` every 15 s (`gate:400,421`) rewrites `observedMegabytes` (`publish`, `:379`). Phase transitions re-publish `reservedMegabytes` (`enter`, `:401-410`).
4. **Lend**: `lender` every 1 s (`gate:424-438`) sets `lentSinceMs` while the process tree only waits (`scriptProcessWaitEpisodeV1`, child list, wait markers). A lent grant counts neither memory nor cores.
5. **Release**: `finally` (`gate:476-481`) calls `cleanup` (`:377`): unlinks grant, attached, phases, upgrade file, rebuild folder and wait markers. Also registered as `process.once("exit", cleanup)` (`:378`).
6. **Park** (restart handover): if the body fails and the caller says so, `parkHostGrantV1` keeps the grant for the replacement process for at most 120 s (`gate:474,479`, `HC/🎫️gate/🤝️handover/🟦️.ts`).
7. **Reclaim by others**: `readGrants` (`gate`/`status`) reclaims a grant whose pid is dead, whose boot differs, or whose mtime is older than 6 h (`isCurrentHostGrantV1`, `decision:59-63`). It also breaks a pid reuse (`hostGrantPidReusedV1`) when the heartbeat is older than 2 beats and the process started after admission. `reclaimStale` removes only if the mtime is unchanged (`status`). Waiters are reclaimed when their pid is dead or the file is older than 30 polls (30 s).

### 2.5 Callers (every path into the gate)

Direct callers of `withHostCapacityV1` (non-test):
1. `HC`-adjacent `HCP/🌡️host-capacity/🟦️.ts:150` — `reserveGraphPlanCapacityV1` (operation `graph-plan`, label `graph-plan`, priority batch, not in POL `operations`).
2. `HCP/🌡️host-capacity/🟦️.ts:159` — `withBuildCapacityV1`, which serves the build invocations (see 2.1).
3. `🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts:68` — `createToolAdmissionV1` (every tool run with admission, see 2.1).
4. `HCP/⌨️command/🟦️.ts:41-51` — the host-capacity `run` subcommand (request `command`, priority batch, session attached).

Callers of `reserveGraphPlanCapacityV1` (held graph grants):
- `HCP/🌐️remote-execution/🛰️sidecar/🕸️graph/📥️adoption/🟦️.ts:118-127` (`withHeadCapacity`), exact-size grant around a graph adoption.
- `HCP/⌨️command/🌐️host/🟦️.ts:23` (resident graph host, reserve + attach).

Build-invocation paths into `withBuildCapacityV1`:
- `📜️script.ts:121-124` (repo command entry, `door.capacity` phase).
- `HCP/⌨️command/🏃️invocation/🟦️.ts:9`.
- `HCP/🚀️bootstrap/🟦️.ts:103-104` (`dependencies ensure`, deferred; see section 6, gap G7).

Tool runs into `runToolV1` (async, admitted unless exempt, see 2.6):
- `repo exec` (`HCP/🧰️tool-registry/⌨️command/🟦️.ts:49-52`; the file sits under `🧰️tool-registry/` in the product build folder) with `heavy: !head.includes("--light")` and `admission: false` when the enclosing grant is owned (`owned` argument).
- Playwright wrapper always `heavy: true` (`🧰️framework/🔨️modules/🏃️process/🧰️tools/🎭️playwright/🟦️.ts:7`).
- Other tool wrappers (git, gh, github, scan, cmake, go, uv, cargo launcher, bazel, nx, docker, dotnet, wasm tools) call `runToolV1` with their registry mode; no `heavy` flag at call site unless the wrapper sets it.

Synchronous runs: `runToolSyncV1` (`🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts:201-211`) uses `spawnSync` and **never** admits ("joins the enclosing grant only", comment at `:197`).

Admission-free tool modes (`TOOLS` `capacity.mode`):
- `none` (9): rustup, hdiutil, diskutil, git, gh, repo-watch, tar, ffmpeg, wasm-bindgen.
- `inherit` (9): repo, bun, node, python, tsc, vite, vitest, playwright, cc. Admitted only when `heavy: true` (`run:165`). `playwright` sets heavy; `bun`/`node`/`python`/`tsc`/`vite`/`vitest`/`cc` do not, unless the caller passes `heavy`.
- `admit` (16): bazel, nx, cargo, go, uv, micromamba, dotnet, cmake, ninja, tectonic, wasm-opt, jco, wasm-pack, docker, cargo-nextest, cargo-llvm-cov, plus any `rules` override (go, uv: rules change only operation/kind, not the mode).

Other things that use the gate's state but do not admit: `HCP/🔧️tools/🏗️bazel/📜️script.ts:920` (`attachHostCapacityProcessV1`, attaches a persistent Bazel server to the grant); `HCP/🚢️fleet-status`, `HCP/🩺️doctor` (readers).

### 2.6 Which process launches do NOT go through admission

1. Operations not in POL `operations` (`buildCapacityAskV1`, `:57-58`): `test`, `test-file`, `lint`, `check`, `build`, `doctor`, `fleet`, `tools` (except `tools exec|setup`, mapped to `command`), `generate`, `preflight` (mapped to `command`, admitted), and all others. `repo test-file` is not in the list, so its own `repo`-level run is not admitted (see G5).
2. `--help`/`-h`, and sealed reads: `show` with graph policy `current` (`:61`).
3. `dependencies ensure` (deferred, `buildCapacityDeferredV1` `:87`), unless the caller passes `admitDeferred`. The bootstrap call (`HCP/🚀️bootstrap/🟦️.ts:104`) does not pass it.
4. `withBuildCapacityV1` returns the body unadmitted when `nativeDashboardLaunchV1` or the native image recovery path applies (`:161,164-168`).
5. Policy switch off: `SEMIO_HOST_CAPACITY=off` or `policy.enabled === false` (`gate:277-280`). The body runs with a plan sized against 1 TiB (`totalMegabytes: 1048576`).
6. Tool runs with `capacity.mode` `none` (9 tools, see 2.5), `inherit` without `heavy` (7 tools), any `runToolSyncV1` call, and `admission: false`.
7. `createToolAdmissionV1` returns the body unadmitted when `sizeHostRequestV1` returns undefined, i.e. the tool's operation is not in POL `operations` (`run:61`).
8. Fleet offload: `withBuildCapacityV1` runs the body without a local grant when the fleet admission accepts the unit (`:185`, `runWithFleet` `:178-184`); local cargo slots pinned to 1 (`SEMIO_UNIT_CACHE_SLOTS=1`).
9. Light commands (`--light`): tool runs with `heavy !== true` (`run:165`, `⌨️command` `heavy:!head.includes("--light")`).
10. Process starts allowed by the raw-spawn law's allowlist (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🚷️raw-process-spawn/📒️allowlist/🔣️.json`, 35 exceptions). Relevant classes: `os-primitive` (ps, taskkill, lsof, kill, `defaults`, `icacls`, `powershell.exe`, `whoami`, `clip`, `reg.exe`, `mkfifo`, `plutil`), `own-executable`, `shell-host` (bash, powershell.exe in `setup`), `layer-below-registry` (git in `⚙️runtime/⚖️law`), `generic-runner`, `pending-route` (rust `system-verbs`).
11. Processes not started by the repo: editors, agent CLIs (claude, codex, droid, kiro-cli, copilot, windsurf, cursor, antigravity), watchers, browsers, Docker VM, Spotlight. They are never admitted; they are only counted in `unmanagedMegabytes` (see 3.4 and G9).
12. Test workers (`🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🟦️.ts`): they size themselves from the live plan of the enclosing grant (`readLiveHostCapacityPlanV1`, `:41-44`), not from their own admission. Whether a `repo test-file` run is admitted at all depends on the parent (see G5).

---

## 3. Runtime watch: sampling running processes, reactions

### 3.1 What is sampled, at what interval

In `holdHostGrantV1` (`HC/🎫️gate/🟦️.ts:370-481`):

- `observeAll()` (`:388-398`): `observeTreeMegabytesV1(pid, ..., {attached, excluded})` (default path). Attached pids (`grants/<id>.attached`) are added; graph-plan grants' attached trees are excluded so they are not booked twice (`:391-394`).
- `heartbeat()` (`:400`): reads cached `sample.json` (only to set `pressured` when a pressured sample is <= 2 beats old, `:400`), then `observeAll()`, then `publish({...state, observedMegabytes})` and records a peak mark. Interval: 15 000 ms (`:421`), first beat at 3 000 ms (`:421`).
- `lender` (`:424-438`): every 1 000 ms (`:438`). Reads phase marks (`readPhases`), wait episodes and child lists (`liveChildPidsV1` every heartbeat, `:433`). Sets or clears `lentSinceMs`.
- `grower` (`:445-449`): every heartbeat (15 s, `:449`), for non-quiet growable grants; calls `growHostGrantV1` (`HC/🔼️upgrade/🟦️.ts:73`).
- `watcher` (`:439-444`): quiet grants only, every 100 ms (`protocol.upgradePollMilliseconds`), calls `upgradeHostGrantV1` when `outgrown` (observed > reserved) or a request file exists.
- `rebuildWatcher` (`:450-468`): every 100 ms, for graph rebuild requests (`rebuildRequests`, `upgradeHostGrantV1`).
- `orphaned` (`:422`): every 15 s, checks whether the output pipe is broken (`currentHostOrphanWatchV1`, `HC/🎫️gate/👻️orphan/🟦️.ts:12-17`).

`observeTreeMegabytesV1` (`HC/📡️sampling/🟦️.ts:295-321`) per platform:

- macOS: `ps -A -o pid=,ppid=,rss=` (`parseProcessTableV1`, `:234`), tree by ppid (`treePidsV1`, `:242-251`), then one `footprint -f bytes --noCategories -p <pid> ...` call for all members (`:316-318`), parsing `phys_footprint` (`parseFootprintV1`, `:276-285`). Fallback to resident (RSS) if footprint fails, announced once (`footprintFallback`).
- Linux: `readdir("/proc")` and `/proc/<pid>/status` for **every** live process on the host (`readLinuxProcessTableV1`, `:267-270`, called on every observation), then `/proc/<pid>/smaps_rollup` for each tree member, summing `Pss + SwapPss` (`:304-311`, `OBSERVERS` in HC schema). Fallback to `VmRSS`.
- Windows: `systemWindowsMemoryPortV1().processes()` walks the toolhelp snapshot and calls `K32GetProcessMemoryInfo` per process (`HC/🪟️native-memory/🟦️.ts:96-114`), private usage as footprint, working set as fallback (`sampling:297-302`). `treeUnobserved` when the snapshot cannot be read (`:299`).

Measure: footprint (private + compressed/swapped), not RSS, on all platforms (`OBSERVERS` in HC schema: "The resident set stays at the high-water mark ... so it is the fallback").

### 3.2 Children and waits

- `processChildrenTableV1` (`gate:188-204`): Linux reads `/proc/*/stat`; Windows spawns `powershell.exe` (10 s timeout, `:196`); macOS and others spawn `ps -axo pid=,ppid=` (`:200`).
- `hostTreeWaitsV1` (`gate:206-214`) decides whether a process tree only waits (wait markers), and the lender then lends the reservation (`lentSinceMs`) so queued work counts nothing.

### 3.3 Reactions to exceeding a reservation or host pressure

- A running grant that exceeds its reservation: nothing kills, suspends, or shrinks it. The heartbeat writes `observedMegabytes` (`:400`). Only a quiet grant that observes more than reserved sets `outgrown` (`:400`) and asks for an upgrade (`:439-444`). A non-quiet grant can grow to more **jobs** via `growHostGrantV1` when memory frees (`:445-449`); it is never shrunk. Fleet status warns at 150% observed over reserved (`under-reserved`, see 4.2).
- Host under pressure or swap stop: the decision refuses **new** admissions (`hard`, `decision:243-247`). Running grants are untouched. The heartbeat only sets `pressured` so the run is not learned from (`:400`).
- Process starts outside the tree: not sampled unless attached (`attachHostCapacityProcessV1`, `gate:55-60`, `sampling`), see G11.
- Cancel and requeue: not present. `cancelledV1` only classifies the failure of a body (`gate:364-365`) for the learning record (`status: "cancelled"`, `gate:478`).
- Priority ordering: yes, but for queued requests only (`orderHostWaitersV1`, `decision:126-129`): interactive before batch, lent last, fair share by session, aging. No preemption of running work.
- SIGSTOP / SIGCONT: none in the capacity code (HC, HCP). Verified by `git grep` over `🧰️framework` (excluding tests): the only hits are a law pattern in `🧪️testing/🗂️suites/🖥️platforms/🔣️.json:52` and an unrelated Rust TUI pty (`🖱️ui/⌨️tui/🚇️pty/🦀️.rs:359`).
- SIGKILL/SIGTERM in the capacity area: `HC/🎫️gate/👻️orphan/🟦️.ts:17` sends `process.kill(process.pid, "SIGTERM")` to itself when its stdout pipe is broken (for example `repo show | head`), which ends the run. `HCP/⌨️command/🟦️.ts:53` forwards SIGINT/SIGTERM to the child. `HCP/⌨️command/⚙️runtime/🟦️.ts:58` stops a runtime child with SIGTERM. Everything else is liveness probing (`process.kill(pid, 0)`).
- Outside the capacity area, the termination module `🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts:18-107` kills owned trees (SIGTERM, then SIGKILL; `taskkill /T /F` on Windows) on cancel, timeout or ceiling (per its schema description). Stale lease reclaim sends SIGTERM/SIGKILL (`🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts:64-80`). Neither is memory-driven.

### 3.4 Unmanaged memory

`unmanagedMegabytes` = `total - available - sum(observed or reserved of live, non-lent grants)` (`decision:394-397`), then subtracted from the budget share (`decision:216`). Thus other tenants (agents, editors, Docker VM, Spotlight) reduce the budget only implicitly. `censusUnmanagedHostV1` (HC `🧺️unmanaged/🟦️.ts:18-30`) can classify the process table into classes (agent, editor, watcher, tool, other), but it has **no production caller** (only its definition and tests), so the class breakdown is not wired to the gate or to `fleet status`.

---

## 4. Alarms and findings (EN/DE, where declared, how they reach `repo fleet status`)

### 4.1 Gate messages (stderr, prefixed `[capacity]`)

- Declared in `HC/🧬️schema/🔣️.json` under `Messages` (line 1473; `en` and `de`, 75 keys each, same key set checked).
- Rendered by `formatHostCapacityV1` (`HC/📏️decision/🟦️.ts:341`), written through the `write` port of the caller.
- Examples of keys (EN text abbreviated):
  - `waiting`: progress line with `need`, `spare = available - floor - held - taken`, `held` (unrealized), `taken` (admitted ahead), `ahead`, `behind` (blockers).
  - `admitted`, `reduced`, `reducedGrows`, `shrunk`, `shrunkStopped`, `override` ("nothing of ours is running and memory holds"), `unsatisfiable`, `lent`.
  - `status`, `grant`, `grantLent`, `grantQuiet`, `quiet`, `quietFollowUp`, `upgrading`, `upgraded`, `upgradedPartly`, `upgradedReduced`, `upgradeWithdrawn`, `grown`, `handover`, `adopted`, `pastDeadline`, `phased`, `estimated`.
  - Once-per-process notices (`noticeOnceV1`, `sampling:144-148`, both languages, never throws): `treeUnobserved`, `footprintFallback`, `countersPartial`, `availableUnread`.
  - Reasons (`reasons.*`): `memory`, `swap`, `swap-activity`, `pressure-critical`, `pressure-warn`, `ramp`, `quiet-limit`, `never`, `starved`, `files`, `attaches`, `load`, `queue-order`, `lent`.
  - Errors (`errors.*`, thrown): `memoryNeverFits`, `memoryStarved`, `memoryWaitedOut`, `countersUnavailable`, `pressureOrder`, `floorOrder`, `quietUpgradeRequired`, `queueFull`, `recordTooLarge`, `platformUnsupported`.
- The `[capacity]` lines are the only memory alarms the running user sees during a build; "unknown is not pressure" is the rule for missing counters.

### 4.2 Fleet findings (`repo fleet status`)

- Computation: `HCP/🚢️fleet-status/🟦️.ts:199-218`, function building `findings` (`say(id, severity, subject, values)`), sorted, capped at `policy.findings.maximum`.
- Memory findings:
  - `memory-pressure` (line 208): severity `alert` when pressure is critical, `warn` when warn. Text key `finding.memory-pressure` (schema `HCP/🚢️fleet-status/🧬️schema/🔣️.json:489`, EN "Host memory pressure is {pressure}", DE "Der Speicherdruck des Rechners ist {pressure}").
  - `sample-stale` (line 209): warn when the host sample is older than `sampleStaleSeconds` 30 (schema line 493).
  - `under-reserved` (line 210): warn when a grant's observed > `underReservedBasisPoints` 15 000 (150%) of reserved (schema line 497).
  - `capacity-wait` (line 211): warn when a capacity waiter waited >= `capacityWaitSeconds` 600 (schema line 501).
  - Findings policy values: fleet schema lines 39-44 (`underReservedBasisPoints 15000`, `capacityWaitSeconds 600`, `sampleStaleSeconds 30`).
- Memory summary row: `memory` (schema line 241; `memoryNone` line 245) "Memory: {admitted} MB admitted, {observed} MB real over {grants} run(s); host ...".
- Per-run line: `run` (schema) includes `memory {memory}` and `waiting`; `waiting.capacity` ("waits {waited} for memory").
- Findings header `findingsHeader` (schema line 433).
- Readers: `HCP/🚢️fleet-status/📥️readers/🟦️.ts:51-78` read the capacity status (`readHostCapacityStatusV1`) and Bazel slots (`observeTreeMegabytesV1`).
- Doctor probe `HCP/🩺️doctor/🔎️probes/🎫️capacity/🟦️.ts:20-37` reports host and capacity sections (no findings, only data).

Severity `alert` exists only for `memory-pressure` critical among memory findings; swap stop and reservations below the floor produce no finding of their own.

---

## 5. Test layout for the gate

### 5.1 Trees

- Core: `HC/🧪️tests/` (about 45 folders), e.g. `🚪️gate`, `🔼️upgrade/🌱️grow`, `🤝️handover`, `🤲️lend/🚪️gate`, `📡️sampling/🐧️linux`, `🔬️footprint`, `🪟️windows`, `📐️estimates`, `🧯️progress`, `🕰️stale-sample`, `🧾️blockers`, `🪜️phases/🚪️gate`, `🧭️liveness`, `🔓️release`, `♻️pid-reuse`, `⚓️boot-anchor`, `💿️leaked-images`, `🪂️partial-start`.
- Product: `HCP/🧪️tests/` (about 30 folders), e.g. `📉️shrink`, `🧮️graph-plan`, `🛟️unsatisfiable`, `🔁️restart` (with `📜️child`), `🛰️planned-grant`, `🧮️arguments`, `🦀️cargo-scaling`, `📈️memory-fit`, `🌱️grow`, `🏊️farm`, `🖥️host-matrix`, `🧾️label`, `🪶️lazy-graph`, `🛠️setup`, `🔓️sealed-read`.
- Disk recovery tests: `HCP/🧹️disk-recovery/🧪️tests/` and `image-slack`, `native-shards`, `volume-facts`, `replica-store`.

### 5.2 Shape of one test folder

Observed structure (no test was run):

- `🟦️.ts`: `bun:test` (`import {expect,test} from "bun:test"`), imports the module under test (e.g. `HC/🧪️tests/🤝️handover/🟦️.ts:1-12`), builds a policy from the decision fixture (`decisionFixture.policy` with overrides), uses temp directories (`mkdtempSync`) and removes them in `afterAll`.
- `🥒️.feature`: Gherkin-style prose spec (e.g. `HC/🧪️tests/🤝️handover/🥒️.feature`: "Feature: Capacity Grant Handover To A Restarted Command" with Scenario and Given/When/Then). No parser for these files was found in the host-capacity tests; the TS test refers to them only by name. Framework has a `FeatureScenario` type in `🧰️framework/🔨️modules/🧪️test/🔌️adapter/🟦️.ts:24` (not traced further; whether it consumes these `.feature` files is **not verified**).
- `🧫️fixtures/🔣️.json`: language-agnostic case tables (JSON), e.g. `HC/🧪️tests/🪜️phases/🧫️fixtures/🔣️.json`, `HC/🧪️tests/🔬️footprint/🧫️fixtures/🔣️.json` (+ `🪢️trees.json`), `HCP/🧪️tests/🤫️quiet/🧫️fixtures/🔣️.json`.
- `🐍️oracle/🐍️.py`: independent oracle in Python standard library only, read as `python3 -I <fixtures.json>` and printing JSON. Example: `HC/🧪️tests/🤝️handover/🐍️.py` (decides `token`, `gone`, `expired`, `identity`, `operation`, `adopt` from the same cases as `🤝️handover/🔣️.json`); `HC/🧪️tests/🔬️footprint/🐍️oracle/🐍️.py` (parses `footprint` text and `smaps_rollup`).
- The TS test runs the oracle through `testPythonV1` (`🧰️framework/🛍️products/🦑️repo/...` packages: `HC`-import `📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts`), compares the outputs, and sometimes also a third-party library: `ajv` for JSON schema validation (`HC/🧪️tests/🤝️handover/🟦️.ts:2`), `bun:sqlite` (`footprint/🟦️.ts:2`).
- Count: about 30 test files reference `testPythonV1` or an oracle (list from `git grep -l "testPythonV1|oracle"`).
- Contract docs: many tests start with a `🔬️ Contract:` docstring (e.g. `footprint/🟦️.ts:1`) stating what the test pins, including the oracle used.

Language-agnostic fixture consumed by more than one language: yes. Examples:
- `HC/🪟️native-memory/🧪️tests/🧫️fixtures/🔣️.json` consumed by its TS test and `🐍️oracle/🐍️.py`.
- `HC/🧪️tests/🤝️handover/🔣️.json` consumed by TS and `🐍️.py`.
- Product: `HCP/🧪️tests/🤫️quiet/🧫️fixtures/🔣️.json` consumed by TS and `🐍️oracle`.
- Schema-first: `HC/🧬️schema/🔣️.json` is imported by the TS code (`import schema from "../🧬️schema/🔣️.json" with {type:"json"}`, `sampling:5`) and supplies `Limits`, `Observers`, `Protocol`, `Messages`.

### 5.3 Budgets and running

- Test time budgets: `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🧪️tests/⏳️literal-deadlines/🔣️.json` lists the host-capacity test files with literal numbers (e.g. line 68 `🚪️gate/🟦️.ts: 18`, line 67 `🔼️upgrade/🚪️gate/🟦️.ts: 7`).
- Running one file: `repo test-file <file>` (`HCP/🔬️test-file/⌨️command/🟦️.ts:11`: "routes every file to the suite that owns it"; `--dry-run` prints the plan). Suites: `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/`; route table `HCP/🔬️test-file/🔣️.json`. Operations in the build catalog: `⚙️operations/🔣️catalog.json:95` includes `test-file`.
- Bun-specific: `runToolSyncV1` calls `requireExplicitBunTestOperandsV1` before spawning (`run:203`), so `bun test` must receive explicit file operands.
- Not verified in this audit: the exact suite names that a `repo test-file` run maps each host-capacity folder to. The route table was not opened.

---

## 6. Gaps: where memory is consumed without admission or without a running watch

Each item is a code fact; the impact is an inference.

**Without admission**

- G1. Synchronous tool runs (`runToolSyncV1`, `run:201-211`) never wait for capacity. Called by many wrappers in `🧰️tools` (for example `git rev-parse`, `gh` queries).
- G2. Registry modes `none` (rustup, hdiutil, diskutil, git, gh, repo-watch, tar, ffmpeg, wasm-bindgen; `TOOLS` lines 917, 2309, 2369, 2521, 2596, 2678, 2749, 2812, 2886 in the `"mode": "none"` list). `hdiutil` creates and mounts disk images (memory-relevant for image-backed builds); `ffmpeg` and `tar` can be large; `repo-watch` is a resident watcher.
- G3. Mode `inherit` (bun, node, python, tsc, vite, vitest, cc; repo) admits only with `heavy: true` (`run:165`). Any direct `runToolV1` call from a wrapper that does not pass `heavy` is unadmitted. A test or build run through `bun`/`node` directly is therefore not gated. Whether `repo test-file` sets `heavy` was not verified (the test-file run module does not contain `heavy`; `git grep` returned no hit).
- G4. Operations not in POL `operations` are never admitted: `test`, `test-file`, `lint`, `check`, `build`, `doctor`, `fleet`, `generate`, `tools` (non-exec), plus `--help` and sealed reads (`HCP/🌡️host-capacity/🟦️.ts:57-61`). The POL list contains only run, run-many, affected, exec, watch, cargo, show, graph, dependencies, command, tool-format, tool-query.
- G5. `repo test-file` is not an operation in POL, so its own process is not admitted. Its children (vitest, bun test, cargo test) are admitted only if their wrappers pass `heavy` or use a mode `admit` tool (cargo: `admit`). Consequence: a test suite of many workers may run without any memory reservation of its own; the test-worker planner then sizes from the host share (`testWorkerFactsV1` falls back to `policy.hostShareBasisPoints` when no plan is inherited, `🧪️testing/👷️workers/🟦️.ts:42`). This needs confirmation by a run, which this audit did not do.
- G6. `dependencies ensure` is deferred (`buildCapacityDeferredV1`, `:87`); the bootstrap call (`HCP/🚀️bootstrap/🟦️.ts:104`) passes no `admitDeferred`. Installs (bun, uv, cargo fetch, npm) may run without memory admission. The "real synchronisation" admission was not traced.
- G7. Fleet offload bypass (`HCP/🌡️host-capacity/🟦️.ts:178-185`): offloaded units run without a local grant. Local memory is not reserved for those units; local cargo is pinned to one slot.
- G8. Policy off switch and `SEMIO_HOST_CAPACITY=off` (`gate:277-280`) remove all admission and all watch decisions (the plan is sized against 1 TiB). The switch is environment-driven and can be inherited by child processes.
- G9. Unmanaged processes: agents (claude, codex, droid, kiro-cli, copilot, windsurf, cursor, antigravity), editors (code, idea, zed, nvim, ...), watchers (watchman, fswatch, inotifywait), Docker/containers, browsers, and any tool started by hand. They are counted only as `unmanagedMegabytes` (`decision:394-397`). The census that could split them into classes (`censusUnmanagedHostV1`) has no production caller.
- G10. Docker containers (`docker` mode `admit`, operation `command`, kind `service`, `TOOLS`): the container memory lives in the Docker VM / dockerd, not in the client process tree that `observeTreeMegabytesV1` follows. It is reserved at admission (2048 MB service kind, plus build row) but not observed afterwards (inference from the tree-based sampler).
- G11. Daemons and reparented processes (double fork, nohup, setsid, persistent servers): `treePidsV1` follows ppid only (`sampling:242-251`). Processes that leave the tree are invisible unless attached by an explicit `attachHostCapacityProcessV1` call (`gate:55-60`; used for Bazel `🔧️tools/🏗️bazel/📜️script.ts:920` and the graph-plan worker). Other persistent servers (native build servers, fleet worker, `graph` service) have no such call in the non-test code that was checked.
- G12. Test workers and runner children: their memory is part of the parent's tree only when the parent is admitted and the children keep the parent's ppid. Pool workers that re-parent are not counted (same as G11).
- G13. Graph rebuild, adoption and resident host reservations use `graph-plan` (not in POL operations) and fixed sizes; their real use is observed only via attached workers (`graphPlanObserverV1`, `HCP/🌡️host-capacity/🟦️.ts:134-136`), so the service process itself counts nothing (by design, to avoid double counting).
- G14. Sync sampler `hostAvailableBytesV1` blocks the event loop up to 2 s (spawnSync `vm_stat`, `sampling:175`); used by scheduler store and bench planning. It is a read, not admission, but it can stall a caller.

**Without a running watch (or with a watch that only logs)**

- G15. No enforcement for running grants: observed memory above reservation is written (`publish`, `gate:400`) and only quiet grants trigger an upgrade; non-quiet grants are never stopped, suspended or shrunk. Critical pressure and swap stop block only new admissions (`decision:227,243-247`).
- G16. Sampling interval 15 s per grant heartbeat (`gate:421`). Spikes between beats are invisible; the peak is the maximum of the recorded marks (`marks`, `gate:400,478`). Memory of a short, large process can be lost.
- G17. A hung but alive holder keeps its reservation for up to 6 h (`isCurrentHostGrantV1` `decision:59-63`, `wait.staleMilliseconds` 21 600 000) as long as its pid lives and the boot is the same. The pid-reuse check (`hostGrantPidReusedV1`) helps only when the pid was reused. A SIGSTOP-ed or swap-frozen process is not reclaimed.
- G18. Lent grants (`lentSinceMs`) count neither memory nor cores (`decision:222`, `lendHostHoldersV1`). A process that only waits in the lender's view but still allocates is not counted while lent; the wait marker logic (`hostTreeWaitsV1`) decides this from child lists taken every 15 s (`gate:433`).
- G19. Sampler failure path: when sampling fails and no previous sample exists, the waiter gets an error and the admission loop ends (`currentSample`, `gate` status module); when a previous sample exists, the stale copy is aged after 30 s to "warn, partial" (`decision:404-409`). Partial counters on Windows and on any unknown platform mean `os.freemem()`, which the code itself says undercounts available memory on macOS (comment at `sampling:171`).
- G20. The `pressured` flag only excludes a run from learning when the cached sample is <= 2 beats old (`gate:400`). It does not change the reservation of the running grant.
- G21. The settled and idle rules release reserved memory to others (`keptHostMegabytesV1`, `decision:181-188`): a grant that settles at 150% of observed can grow beyond that without a new admission until its next upgrade, so the sum of observed memory can exceed the budget share transiently.
- G22. Windows: no swap, no compressor, no pressure level; pressure is inferred from available physical and commit only (`windowsCountersV1`, `sampling:135-137`). Process tree footprint uses private commit (`K32GetProcessMemoryInfo`), so swapped-out private pages are not visible there. The native reader requires `bun:ffi` (Bun runtime on Windows; `native-memory:44-56`).
- G23. Linux: pressure uses PSI only if `/proc/pressure/memory` exists; otherwise only the available share. Cgroup limits are read from the process's own chain; the resident graph service uses `wholeHostProbeV1` to see the whole machine instead (`sampling:228-231`), so its view differs from its cgroup.
- G24. Findings do not cover swap stop, reservations below the floor, or unmanaged growth; only `memory-pressure`, `under-reserved` (150%), `capacity-wait` (600 s), and `sample-stale` (30 s) exist (section 4.2).

---

## 7. Not verified / limits of this audit

- No command was executed: no sampling values, no timings, no test results. Every "ms" above is a configured bound.
- `repo test-file` mapping to suites and whether it sets `heavy` for its children: not verified (G3, G5).
- Whether `dependencies ensure` gets a real admission later: not traced (G6).
- Whether the Gherkin `.feature` files are consumed by the framework's `FeatureScenario` adapter: not traced.
- The `repo fleet status` command entry (`HCP/🚢️fleet-status/⌨️command/`) was not read line by line; the findings and texts were read in `🟦️.ts` and the schema.
- Remote-execution worker processes (`HCP/🌐️remote-execution/👷️worker/🟦️.ts`) and the offload admission path were only partly read.
- `hostCapacityDirectoryV1` line numbers in the status module were taken from grep output; function names are exact.
