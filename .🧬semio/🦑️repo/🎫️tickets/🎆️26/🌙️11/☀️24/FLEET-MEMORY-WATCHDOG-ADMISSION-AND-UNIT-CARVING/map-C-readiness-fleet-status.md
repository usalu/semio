# Map C: Readiness Stage Supervisor and `repo fleet status` (read-only audit)

Scope: read-only audit, no builds, no tests, no git commands, no source edits. Ticket folder `FLEET-MEMORY-WATCHDOG-ADMISSION-AND-UNIT-CARVING` is the only write.

## Corrections to the brief (verified in code)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/` is NOT the stage supervisor. It holds only the declared dev-server readiness (`metadata.semio.dashboard.ready`: port or `portEnv`, path, `printed` marker, HTTP 2xx/3xx probe). `🟦️.ts` lines 1-155, imports only the JSON-schema validator. It imports nothing for supervision. The supervisor lives in `…/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/`. Below, `R` = `…/🏗️build/🌐️remote-execution` and `S` = `R/🛰️sidecar/🚦️ready`, `B` = `…/🏗️build`.
- `ready-stage-claim.json` is NOT in the group store. It is written to the local host-capacity directory `hostCapacityDirectoryV1()` (`🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts:17-21`): macOS `~/Library/Caches/semio/host-capacity/`, Linux `$XDG_CACHE_HOME/semio/host-capacity` or `~/.cache/semio/host-capacity`, Windows `%LOCALAPPDATA%\semio\host-capacity`, overridable by the protocol directory environment variable. It is written only when `SEMIO_FLEET_STORE_ROOT` is set (`S/🟦️.ts:906`).
- `refused.json` and `ready-log-<stage>-containment.txt` are NOT written anywhere in the build modules (grep over `B`, excluding tests, finds neither). Refusals are in `ready.json` field `refusal`. Containment is in `ready.json` field `containment` (schema `R/…/🚦️ready/🧬️schema/🔣️.json` `Containment`) and in the containment kill log `kills.log` (`S/🌲️process-tree/🧹️containment/🧬️schema/🔣️.json` `killRecordName`).
- Real mirror files written by the supervisor: `ready-log-<stage>.txt` (every ~10 s, `S/🟦️.ts:1244-1262`), `ready-log-<stage>-stalled.txt` (`S/🟦️.ts:537`), `ready-log-<stage>-failed.txt` (`S/🟦️.ts:572`), `ready-timings.json` (`S/🟦️.ts:1270-1279`), `slice-memory.json` (`S/🟦️.ts:1253-1266`), `ready.json` (`S/🟦️.ts:336-362`). `service-failure-*.txt` is written by the resident service, not the supervisor (`B/🏡️resident/🏛️service/🧯️failure/🟨️.mjs:210-220`).
- The silence watchdog (`readySilenceWatchV1`, `S/💔️silence/🟦️.ts:23`) that answers `restart` on a silent sidecar is referenced only by its tests (`S/💔️silence/🧪️tests/🟦️.ts:4,23`). No production caller found. A silent ready sidecar is therefore detected by readers (`readReadyWorkersV1`) but not restarted by anything in this code.

## Subject A: stages, admission, sampling, stall and pressure behaviour

### A1. Stages and their commands

Source: schema `S/🧬️schema/🔣️.json`, `definitions.Stages.const` (read through `S/🟦️.ts:36`, `READY_STAGES_V1`). Run order is the array order.

| # | stage | kind | command (args) | timeout (ms) | declared inputs (digest) |
|---|---|---|---|---|---|
| 1 | `native` | bootstrap | `bash <bootstrap> setup` (`arguments: ["setup"]`, see wrapper `S/🏃️wrapper/🟦️.ts`; env `SKIP_REPO_BOOTSTRAP=1`) | 7 200 000 | `🔩️native/🥾️bootstrap/🧬️schema/🔣️.json`, `global.json`; bootstrap input hash |
| 2 | `repo-mcp` | repo | `repo run repo-mcp:install` | 3 600 000 | none |
| 3 | `dependencies` | repo | `repo dependencies sync --all` | 3 600 000 | `bun.lock`, `package.json` plus dependency generations (`dependencyInputs: true`) |
| 4 | `tools` | repo | `repo tools setup all` | 3 600 000 | `…/🧰️tool-setup/🔣️.json`, `package.json` |
| 5 | `workspace` | repo | `repo run-many --projects=@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp --targets=build` | 3 600 000 | caching bootstrap files (`⚡️caching/🚀️bootstrap/…`) |

The commands are built by `readyStageCommandV1` (`S/🟦️.ts:98`, the repo binary found below `repoDirectories`). Stage digests: `readyDigestsV1` (`S/🟦️.ts:89-98`), a sha256 of stage name, platform, and each input's content hash (`readyInputHashV1`, `S/🟦️.ts:101-127`, normalises the native pin file, tool-setup file, `package.json`). Whole digest = sha256 of stage digests joined by newline. The `ready.json` `digest` field is this whole digest.

Stage decision, `readyDecideV1` (`S/🟦️.ts:242-256`), in order: server machines never run it; a current stage list with the marker only marks; a person running setup (`human`) is observed; a lease held by another runner is observed (`lease-held`); a recorded failure of the same digest waits for its back-off; `!memoryOk` waits (`memory-low`, poll 60 s, `memoryPollMilliseconds`); otherwise the first stale stage runs.

### A2. Admission: the Ready stage claim (`ready-stage-claim.json`)

Publisher: `createReadyStageClaimPublisherV1` in `R/🌡️load/🪑️stage-claim/🟦️.ts:74-89`. Schema `R/🌡️load/🪑️stage-claim/🧬️schema/🔣️.json`.

Policy (schema `Policy.const`): `fileName ready-stage-claim.json`, `shareBasisPoints 3000` (30 % of the machine when no run completed yet), `maximumShareBasisPoints 5000` (50 % ceiling), `minimumMegabytes 1024`, `percentile 90`, `marginPercent 25`, `samples 16`, `freshMilliseconds 120000`, `writeMilliseconds 5000`, `changeMegabytes 64`, `reservePercent 125`, `floorMegabytes 256`.

- Entitlement (ceiling, learned once per stage run): `readyStageEntitlementV1` (`stage-claim/🟦️.ts:20-24`). Takes the last 16 `outcome === "ok"` runs of this stage from `ready-timings.json` (`peakMegabytes`), takes the 90th percentile, multiplies by 1.25 (`ceil`), never below 1024 MB, never above 50 % of total memory. With no completed run: 30 % of total memory.
- Reserve (what the stage holds back for its own growth): `readyStageReserveV1` (`stage-claim/🟦️.ts:31`) = `waiting ? 0 : min(entitlement, max(256, ceil(observed × 1.25)))`. So the "measured ×1.25" is `observedMegabytes × 125 %` (the measured tree size), capped by the entitlement.
- Hold (what other admissions must leave free): `readyStageHoldV1` (`stage-claim/🟦️.ts:35`) = `max(0, reserve − observed)` while the claim is fresh (written within 120 s).
- Applied to host view by `withReadyStageV1` (`stage-claim/🟦️.ts:43-47`): `host.availableMegabytes -= hold`, plus a grant `{id:"ready-stage", reservedMegabytes: observed, jobs:1, cpu:1}` (`readyStageGrantV1`, line 39).
- Readers: `readReadyStageClaimV1` (`stage-claim/🟦️.ts:50-56`), used by the remote-execution load view (`R/🌡️load/🟦️.ts:129,133`).

Write path: `observe(megabytes, waiting)` (`stage-claim/🟦️.ts:79-87`), atomic tmp + rename (`writeReadyStageClaimV1`, lines 59-64). Called from the supervisor on each process-tree sample: POSIX from the `ps` sampler (`S/🟦️.ts:1077`), Windows from the tree sampler (`S/🟦️.ts:1037`). Cleared on stage end (`clear`, `S/🟦️.ts:997`).

The claim is a file in the local host-capacity directory and NOT in `/Users/Shared/semio-store/v1/origins/<origin>/`. Other machines never see it; the coordinator reads only its own machine's host view.

### A3. How `peakMegabytes` is sampled

Two samplers run in the stage loop (`S/🟦️.ts`, function `runReadyStageV1`, from about line 835 to 1095):

- Poll tick: `P.pollMilliseconds = 250` (`ready/🧬️schema` Policy). The loop `setInterval` runs every 250 ms (`S/🟦️.ts:1091` ends the interval with `P.pollMilliseconds`).
- Memory sampler (POSIX): every `sampleEvery = round(samplerMilliseconds / pollMilliseconds) = 20` ticks, i.e. every 5 s (`S/🟦️.ts:951`, `P.samplerMilliseconds = 5000`). Command `ps -A -o pid=,ppid=,rss=,time=` (`S/🟦️.ts:1076`). `readyTreeMegabytesV1` (`S/🟦️.ts:230-237`) sums `rss` (KB) over the stage's root pid and every descendant (ppid chain, depth ≤ 64), converts to MB. Pids sampled: the stage wrapper/root `inflight.pid` and its descendants.
- Memory sampler (Windows): `readyWindowsTreeSampleV1` on the tree table (`S/🟦️.ts:1032-1042`), with `P.watchdogCpuMaximum` as bound.
- Process-tree table sampler (for tree membership, not memory): `sampleTree` (`S/🟦️.ts:1017-1045`) with `readySampleDelayV1` (`S/🌲️process-tree/🧹️containment/🟦️.ts:29`): 1 000 ms for the first 60 s (`fastWindowMilliseconds 60000`), then 5 000 ms. Writes `<stage>.tree.json` (`readyWriteTreeV1`, `S/🌲️process-tree/🟦️.ts:426`). Known members are root plus descendants plus heirs (`readyHeirsV1`).
- Peak: `peak = max(peak, observed)` (`S/🟦️.ts:1077`); the peak survives supervisor restarts through the watchdog record (`ReadyWatchdogV1.peakMegabytes`, `S/🟦️.ts:926`).
- Caveat: this is RSS, not the physical footprint. The footprint module (`B/🧠️memory/🦶️footprint/🟨️.mjs`) and the host-capacity footprint measure (`🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts:272`) exist, but the readiness supervisor does not use them. On macOS, RSS is misleading for compressed/swapped pages (it under-counts what the OS already had to push out).

### A4. Stall, restart, kill, timeout and pressure (exact conditions)

Kill mechanism: `kill(reason, evidence)` (`S/🟦️.ts:962-970`) sets `killed=true`, records the stop in the running record, then `readyKillTreeV1` sends `SIGTERM` to the stage tree (`S/🟦️.ts:985`), and `SIGKILL` to the tree after `P.graceMilliseconds = 10 000` ms (`S/🟦️.ts:986`). Kill reasons (`grep kill("…")`): `overdue` (1072), `restart` (1073), `stall` (1079), `timeout` (1086-1087), `cancel` (1089, cancel file `.cache/f/ready/cancel`).

Stall: `readyStalledV1` (`S/🌲️process-tree/🟦️.ts:149`) = no progress for `stallStopMilliseconds = 600 000` ms (10 min), exempt while a live read record says reading. Progress means a change of the progress key (hash of last 200 log lines, `readyProgressKeyV1`), graph counter or tool counters, or a moved CPU/liveness sample. Checked every poll (`S/🟦️.ts:1079`).

Stall restart policy: `readyStallRestartV1` (`S/🌲️process-tree/🟦️.ts:86-90`): consecutive stalls n; n ≤ `stallRestartMaximum = 5` gives `restart` after back-off `min(600 000, 30 000 × 2^(n−1))` ms; n = 6 gives `fail`, which becomes refusal `stage-stall-exhausted`, a retained failure (`refreshKeepingFailureCodes`). Mirror: `ready-log-<stage>-stalled.txt` (`S/🟦️.ts:537`). State written with refusal code `stage-stalled` / `stage-holder-silent` / `stage-code-changed` / `stage-code-stale` / `stage-step-overdue` (`S/🟦️.ts:530-543`).

Other kill conditions:
- Step overdue: `readyStepOverdueSecondsV1` (`S/🌲️process-tree/🟦️.ts:186`), kill `overdue` (`S/🟦️.ts:1071-1072`).
- Code change: every `codeCheckMilliseconds = 60 000` (`readyCodeStopV1` plus `readySemanticsChangedV1`, `S/🌲️process-tree/🟦️.ts:316`) kills with `restart` (`S/🟦️.ts:1073`). A stage running for more than `staleStageMilliseconds = 1 800 000` is `code-stale` (retry at once). A named holder silent for `holderSilentMilliseconds = 300 000` or waited ≥ `holderWaitRestartMilliseconds = 600 000` is `holder-silent`/`code-changed`.
- Deadline: `timeoutMilliseconds` of the stage (soft cap). `hardCapMultiplier = 4` (workspace 8), `progressCeilingMultiplier = 24`. Past the soft cap, a stage that still moves inside `stallMilliseconds` is extended (`readyDeadlineV1`); otherwise `timeout` with `soft-cap-idle` or `hard-cap` (`S/🟦️.ts:1081-1087`). Tools-grace denial: `toolsGraceDenial` (tools stage only).

Host memory pressure: the supervisor does NOT kill for memory pressure and does NOT read `pressure`, `swapUsedMegabytes`, swap-out rate or compression (grep of `S/🟦️.ts`: no `pressure` use). Its only memory input is `fleetHostReadingV1(...).availableMegabytes` (`S/🟦️.ts:1239`, source `B/🧠️memory/🟨️.mjs:79-83`, from `hostAvailableBytesV1`). The rules:
- Start gate: `memoryOk = free === null || free ≥ floor`, floor `readyMemoryFloorMegabytesV1` (`S/🟦️.ts:1121-1124`) = `min(1536, max(512, hostMB × 8 %))`. Not ok: `wait` with `memory-low`, poll 60 s; after `memoryWaitMaximumMilliseconds = 600 000` the stage starts anyway (`S/🟦️.ts:424`).
- Parallelism per stage: `readyJobsV1` (`S/🟦️.ts:224-227`) = `min(cores − 1, floor(free / 1536))`, at least 1.
- No back-off or SIGTERM on pressure. The host-capacity pressure level (`warn`/`critical`, thresholds in `B/🔣️policy.json` `hostCapacity.pressure`: PSI warn 10 %, critical 30 %, available warn 15 %, critical 6 %, compressed warn 35 %, critical 55 %) is consumed only by the remote-execution admission gate (`R/🌡️load/🟦️.ts:98-103`) and the capacity gate, not by the readiness loop.
- OS-level kills (macOS jetsam, OOM) are visible only as "vanished" stages: `vanishedLineV1` (`S/🟦️.ts:784`) says "a SIGKILL from a person or a program, the system under memory pressure, or a restart of the machine".

Back-off for a failed stage: `readyBackoffV1` (`S/🟦️.ts:130-133`): initial 30 s, doubling, maximum 1 200 000 ms (20 min), jitter 10 % per machine. `retryExit 75` restarts at once (`S/🟦️.ts:569`).

### A5. How a record is written to the group store (schema-first)

- Owner: `createReadyStatusV1` (`S/🟦️.ts:336-362`). Store path: `<SEMIO_FLEET_STORE_ROOT>/v1/origins/<origin>/ready.json` (`P.statusName = "ready.json"`).
- Write (`S/🟦️.ts:343-349`): builds `out = {...doc, updatedMs, beatMilliseconds}`; validates against `READY_STATUS_SCHEMA_V1` (`S/🧬️schema/🔣️.json` `definitions.Status`, `additionalProperties: false`, required list lines in the schema). An invalid record is silently NOT written (`if(validate…)return`). Then `writeFileSync(<file>.<pid>.tmp)` and `renameSync` (atomic), with fallback to a direct write and temp removal (`S/🟦️.ts:346-347`). Same helper for side files: `readyWriteStoreFileV1` (`S/🟦️.ts:698-702`).
- Throttle: `update()` writes at once on a state/stage/ready/refreshing change, else at most every `statusIntervalMilliseconds = 5000`; `heartbeat()` every `statusHeartbeatMilliseconds = 30 000` while running/backing off/disk-collecting (`S/🟦️.ts:357`). `beatMilliseconds` = `beatByState` (running and backoff 30 s, waiting and ready 120 s, observing 60 s, server 120 s).
- Current fields (`Status`, lines in `S/🧬️schema/🔣️.json`): `version, origin, host, platform, pid, state, digest, ready, stage, stageIndex, stageCount, attempts, exit, refusal{code,en,de}, reason{code,en,de}, containment?, peakMegabytes, tail[], nextAttemptMs, readyMs, refreshing?, startedMs, updatedMs, beatMilliseconds`.
- Adding a memory or watchdog section, schema-first: (1) add the object to `Status` (and a `$defs` entry, e.g. `Memory`, with `availableMegabytes`, `swapUsedMegabytes`, `pressure`, `swapOutMegabytesPerSecond`, `interventions`) in `S/🧬️schema/🔣️.json`; (2) add the TS type `ReadyStatusV1` (`S/🟦️.ts:55`) and fill it in `update()` callers around `S/🟦️.ts:427`; (3) the reader `readReadyWorkersV1` (see B4) and `ReadyWorkerRowV1` (`B/🛰️sidecar/🚦️ready/🩺️workers/🟦️.ts`). Because `additionalProperties: false`, a new section fails validation until the schema changes, which is the silent-drop path above.
- Other supervisor files in the group store: `ready-timings.json` (`{version, origin, runs[≤40], updatedMs}`, schema `definitions.Timings`), `slice-memory.json` (≤ 64 KB, owner-checked), `ready-log-<stage>[-stalled|-failed].txt` (text, ≤ 64 KB, from `ports.mirror`).

## Subject B: `repo fleet status` and the load report

### B1. Where each row comes from

Command: `B/🚢️fleet-status/⌨️command/🟦️.ts` → `collectFleetFactsV1` (`B/🚢️fleet-status/📥️readers/🟦️.ts`) → aggregation `B/🚢️fleet-status/🟦️.ts` (`summarizeFleetV1`, line 183+) → text `formatFleetReportV1` (line ~260+). Machine table (`fleet.machine`) rendered at `B/🚢️fleet-status/🟦️.ts:302`, columns: origin, platform, pressure, load, cores, running, queued, worker, headroom (`fleet.noheadroom`), offloadedIn, offloadedOut, refused, cacheHit, cacheMiss, cacheSkipped, age.

Sources, one per section:
- `host` (local machine only, line 66 of readers): `readHostCapacityStatusV1` on the local host-capacity directory. Has `swapUsedMegabytes`, `pressure`, `availableMegabytes`, `load`. This is NOT per worker.
- `fleet` (all machines): `readFleetLoadV1` (readers line 136-140) → `readRemoteExecutionFleetFactsV1` (`R/🚢️status/🟦️.ts:16-25`), which calls `GET <scheduler>/v1/fleet/load` with the fleet credential, timeout 5 s.
- `workers` (readiness record per origin): `readFleetWorkersV1` (readers ~line 150) → `readReadyWorkersV1` (`R/🛰️sidecar/🚦️ready/🩺️workers/🟦️.ts`) over `<store>/v1/origins/*/ready.json` plus heartbeat files `pool.json`, `attest.json` (`policy.heartbeatNames`), judged by `readyWorkerLivenessV1` (`R/🛰️sidecar/🚦️ready/💔️silence/🟦️.ts`), offline after 600 s (`READY_OFFLINE_SECONDS_V1`).

Per machine row (`LoadRow`, schema `B/🏃️execution/🧬️schema/🌐️remote-execution.json` definitions `LoadRow`/`FleetLoad`): `origin, platform{key,os,arch,toolchain,target}, capacity{cores,memoryMegabytes}, running, queued, pressure, freeMegabytes, load, headroom, ageSeconds, worker{state,concurrency,running,sandbox,confinement,network,why?}, offloadedIn, offloadedOut, refused, counters`.

### B2. Publication path and interval

- Each worker builds a `LoadReport` (`R/🌡️load/🟦️.ts:137-142`, `buildRemoteExecutionLoadReportV1`) from `readRemoteExecutionHostViewV1` (`R/🌡️load/🟦️.ts:124-134`), i.e. the host capacity gate's `host` sample plus grants and waiters plus the ready stage claim (`withReadyStageV1`). The report copies only `host.pressure`, `host.totalMegabytes`, `host.cores`, `host.load`, and a computed `freeMegabytes` and `headroom`. It does NOT copy `availableMegabytes`, `swapUsedMegabytes`, `swapOutMegabytesPerSecond`, or the compressed bytes, although `host` carries the first three (HostSampleV1 type: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts:23` = `availableMegabytes`, `pressure`, `swapUsedMegabytes`, `swapOutMegabytesPerSecond`, `load`, `openFiles`, `attachedImages`, `unmanagedMegabytes?`). Compressed bytes exist only in the parser counters (`HostCountersV1.compressedBytes`, `📡️sampling/🟦️.ts:17`) and are not carried into `HostSampleV1`.
- Interval: `runRemoteExecutionReporterV1` (`R/🌡️load/🟦️.ts:148-165`) POSTs every `loadIntervalSeconds`, default `5` (`B/🔣️policy.json:578`, `worker.loadIntervalSeconds`; CLI `--load-interval`). One round at a time, 5 s send timeout. Scheduler expiry `loadExpireSeconds = 15` (`B/🔣️policy.json:561`): a machine not reporting for 15 s leaves the table.
- Endpoint and handler: `POST /v1/fleet/load` in `R/🖥️server/🟦️.ts:246-252` (guard role `worker`, validates `LoadReport` closed shape, rejects another origin with 403), then `scheduler.acceptLoad(body)` (`R/🗓️scheduler/🟦️.ts:408`). Read: `GET /v1/fleet/load` (`R/🖥️server/🟦️.ts:245`) → `scheduler.fleetLoad()` (`R/🗓️scheduler/🟦️.ts:411-428`), which maps every report into a `LoadRow` (the `machines` mapping, lines 413-420) and checks `FleetLoad`.
- Schemas: `B/🏃️execution/🧬️schema/🌐️remote-execution.json` (`LoadReport`, `LoadRow`, `FleetLoad`, `WorkerState`, `WorkerWhy`, `Pressure`). Client-side closed-shape checks: `requireRemoteExecutionShapeV1("LoadReport")` at `R/🌡️load/🟦️.ts:140`.
- Fleet-status row type: `FleetLoadFactV1` (`B/🚢️fleet-status/🟦️.ts:43`), schema `B/🚢️fleet-status/🧬️schema/🔣️.json` `FleetMachine` (line ~3264: `required: origin, platform, pressure, load, cores, running, queued, headroom, ageSeconds, worker, offloadedIn, offloadedOut, refused, cacheHit, cacheMiss`).
- Worker side (`ready.json`): written by the ready supervisor (A5), read by `readReadyWorkersV1`. No `ready.json` field carries the LoadReport numbers.

### B3. Findings: declaration and text

- Finding production: `B/🚢️fleet-status/🟦️.ts:199-240` (`say(id, severity, subject, values)` pushes `{id, severity, subject, values, text: fleetTextV1(language, "finding."+id, values)}`). Ordered by severity then id, truncated to `policy.findings.maximum = 30` (`B/🚢️fleet-status/🧬️schema/🔣️.json` `Policy.const.findings`).
- Existing findings that concern workers or machines: `memory-pressure` (host, line 208, alert if critical else warn), `sample-stale` (30 s), `under-reserved` (observed > 150 % of reserved, `underReservedBasisPoints 15000`), `worker-offline`, `worker-silent` (line 233: `liveness === "silent"`), `worker-absent` (info), `unit-cache-denied` (alert), `unit-cache-read-only` (warn). `image-full` (line 218, storage images ≥ 90 %, `imageFullBasisPoints 9000`).
- EN/DE texts: `Messages.const` in `B/🚢️fleet-status/🧬️schema/🔣️.json` keyed `finding.<id>` with `en` and `de`. Examples verbatim: `finding.image-full` = "Image {id} is {percent}% full" / "Image {id} ist zu {percent} % voll"; `finding.worker-silent` = "worker {origin} lives but wrote no readiness record for {age} although it is {recorded} in stage {stage}" / "Worker {origin} lebt, schrieb aber {age} lang keinen Bereitschaftseintrag, obwohl er {recorded} in Stufe {stage} ist"; `finding.memory-pressure` = "Host memory pressure is {pressure}" / "Der Speicherdruck des Rechners ist {pressure}". A new finding needs: an id in `say(…)`, a `Messages` entry in both languages, a threshold in `Policy.findings` (schema), and a Severity mapping (`SEVERITY`).

### B4. Exact changes to publish per-worker memory, swap, pressure, compressor rate and watchdog interventions

Per worker, in the load report (the fleet status path):
1. Source numbers: `R/🌡️load/🟦️.ts:137` `buildRemoteExecutionLoadReportV1` takes `input.view.host` (already has `availableMegabytes`, `swapUsedMegabytes`, `swapOutMegabytesPerSecond`, `pressure`). Add the copy to a new `memory` object.
2. Compressed bytes: add `compressedMegabytes` to `HostSampleV1` (`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts:23`), fill it in `reduceHostSampleV1` (`📡️sampling/🟦️.ts:109-117`, from `counters.compressedBytes`), and in its schema (`🧬️schema/🔣️.json` of host-capacity, `Host`).
3. Watchdog interventions: the stage loop's counts (stall kills, overdue kills, timeouts, `memory-low` waits, restarts) are in `ready.json` `refusal`/`exit` only, and as the last refusal. Add `interventions{stalls,overdue,timeouts,memoryWaits,restarts}` counters to the ready record (`Status`, `S/🟦️.ts:55`) or to `RemoteExecutionMachineCountersV1` (`R/🌡️load/🟦️.ts:20`, read from `.cache/f/build-counters.json`).
4. Schemas to change (closed shapes, all `additionalProperties: false`): `B/🏃️execution/🧬️schema/🌐️remote-execution.json` `LoadReport`, `LoadRow`, `FleetLoad`, `MachineCounters`; `R/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json` `Status`; `B/🚢️fleet-status/🧬️schema/🔣️.json` `FleetMachine`, `WorkerRow`, `Facts`.
5. Scheduler table: `R/🗓️scheduler/🟦️.ts:413-420` (the `machines` mapping copies named fields only; add the new ones), and `R/🗓️scheduler/🟦️.ts:408` `acceptLoad` / load store `this.load.accept`.
6. Fleet-status types and readers: `B/🚢️fleet-status/🟦️.ts:43` (`FleetLoadFactV1`), `:24` (`FleetFactsV1.host`), `:50-53` (workers types), `B/🚢️fleet-status/📥️readers/🟦️.ts` `readFleetLoadV1` (~line 136) and `readFleetWorkersV1` (~line 150), `R/🚢️status/🟦️.ts:16-25` (fetch of `/v1/fleet/load`).
7. Rendering: `B/🚢️fleet-status/🟦️.ts:302` (`fleet.machine` line; add the new numbers), `Messages` `fleet.machine` in fleet-status schema (EN/DE), `FleetMachine` table text.
8. Findings: `B/🚢️fleet-status/🟦️.ts:199-240` new `say("worker-swap", …)` for swap growth or `pressure` on a worker, `"worker-intervened"` for watchdog interventions; texts in `Messages` (EN/DE) and thresholds in `Policy.findings`.
9. Tests that pin the closed shapes must be updated together: `R/🧪️tests/🔎️why/🟦️.ts:24`, `R/🧪️tests/🌐️fleet/🟦️.ts:37`, `R/🛰️sidecar/🧪️tests/🟦️.ts:50-57` (signed load POST), `R/🗓️scheduler/…/🧪️tests/🟦️.ts`.

Note on the ready-side numbers: `ready.json` does not carry `availableMegabytes` or swap today. Only `peakMegabytes` (the stage's own tree), `containment`, `refusal`. Per-worker swap must therefore come from the host sample (LoadReport), not from ready.json.

### B5. Existing scheduler-side counters that are already published

- `counters` in each LoadReport: `unit{localHit,remoteHit,miss,stored,skipped?}`, `skipReasons`, `keySamples`, `offload{out,refused,fellBack}`, read from `.cache/f/build-counters.json` (`R/🌡️load/🟦️.ts:71-78`).
- Scheduler-side per-machine counters: `offloadedIn`, `offloadedOut`, `refused` (`R/🗓️scheduler/🟦️.ts:411-420`).

## Subject C-5: How workers pick up new code

Three mechanisms, all in `R/🛰️sidecar/🟦️.ts` unless noted.

1. Launcher, the supervised process (`runRemoteExecutionSidecarLauncherV1`, `🟦️.ts:106-152`): the replica daemon starts the launcher; the launcher runs the role in a child (`Bun.spawn([execPath, entry, role], …)`, `SEMIO_SIDECAR_CHILD=1`) and loops. Child exit code `75` = "new code", restarted after 500 ms at once with delay reset (`🟦️.ts:147`). Any other exit: back-off from 1 s doubling to 15 s (`REMOTE_EXECUTION_SIDECAR_RESTART_V1`, line 103), reset after a 60 s stable run (line 148). A stop signal to the launcher exits with the child's status (line 146).
2. Child self-restart, the closure watch (`createSidecarRestartV1`, `🟦️.ts:34-78`; `createSidecarClosureWatchV1`, `🧮️closure/🟦️.ts:25-56`): entries are the role's own file plus the role's module (`remoteExecutionSidecarEntriesV1`, `🟦️.ts:91-93`), walking static imports (`sidecarImportsV1`, `🧮️closure/🟦️.ts:6`). Digest = sha256 of "relative path sha256" lines. The watch runs every `SEMIO_SIDECAR_WATCH_MILLISECONDS` (default 30 s, `🟦️.ts:163`). A changed digest must stay quiet for `quietMilliseconds` (default 300 000 ms; ready 90 000; pool 90 000) or the restart is forced after `maximumDeferralMilliseconds` (1 800 000; ready 600 000). On restart the child exits with 75 (`🟦️.ts:171`), and the launcher starts a successor. The successor is handed over by IPC `successor` message (`hear`, `🟦️.ts:118-130`): the old child keeps serving until the new one listens (`handoverMilliseconds 120 000`, `handoverAttempts 3`).
3. Work-aware deferral (`🧰️shared/🟦️.ts:9-10` `SidecarWorkV1{busy, restartPending, wake}`): roles `graph`, `ready`, `cache` may report `busy`. A restart waits while `busy` up to `serveDeferralMilliseconds`: graph 28 800 000 (8 h); cache 0; ready 0 (the next child adopts the stage in flight: `ready.json`/`running.json`). Agent drains up to 7 200 000 ms (`drainMaximumMilliseconds`, `drainPollMilliseconds 5000`). Policy: `S/…/🛰️sidecar/🧬️schema/🔣️.json` `Restart` (common: watch 30 000, quiet 300 000, maximumDeferral 1 800 000, grace 300 000, serveDeferral 28 800 000, progressStall 600 000, minimumInterval 1 800 000).
4. The graph service child reloads only on restart (as the brief says): `R/🛰️sidecar/🕸️graph/🟦️.ts:80` passes `restartPending` to `runWantedServerV1`; the graph child serves until idle or 8 h. The per-role entry is `ROLE_MODULES` (not found by grep under that name; the role-to-module mapping is in `R/🛰️sidecar/🟦️.ts` `remoteExecutionSidecarEntriesV1`).
5. Readiness supervisor's own loop restart: `runReadyV1` (`S/🟦️.ts:378`): a throw (store not writable, port fails) writes refusal `loop-failed`, state `backoff`, and restarts the loop after back-off; `loopStableMilliseconds 60 000` resets the back-off.
6. Stage code change inside a run (not a process restart): every 60 s `readySemanticsChangedV1` (`S/🌲️process-tree/🟦️.ts:316`) compares the semantics sources (`🕸️graph/…/🧬️schema/🔣️.json` revision, `👁️observation/…/🧬️schema/🔣️.json` stamp) and kills the stage tree with `restart`, which then restarts the stage at once with the new code (`S/🟦️.ts:538`).
7. Silence restart: defined (`readySilenceWatchV1`) but unused in production (see Corrections).

## Gaps and risks found

- `readySilenceWatchV1` is test-only; a silent ready sidecar is not restarted.
- `createReadyStatusV1` drops a schema-invalid record silently (`S/🟦️.ts:345`), so a schema mismatch makes the coordinator show the worker as stale, not as an error.
- Per-worker swap or pressure is not published by the readiness supervisor at all. The load report omits three host fields it already has (B4.1).
- Peak memory is RSS (`ps rss`), which under-counts a process with compressed or swapped pages on macOS. The footprint measure exists and is not used here.
- `ready-stage-claim` is local-only; the coordinator's `memory` section therefore never reflects a worker's stage entitlement unless the load report carries it.
- `repo-mcp` and `workspace` stages have no declared inputs (`inputs: []`), so their digest does not change with code. Stage refresh relies on `codeChanged`, not the digest.

## File index (absolute paths)

Base `B` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.
- `B/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts` (supervisor, 1292 lines)
- `B/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json` (Policy, Stages, Status, Texts, Reasons, Timings, Containment)
- `B/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts` (stall, restart, deadline, step, code-change policy)
- `B/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧹️containment/🟦️.ts` (kills, sample delay, reap)
- `B/🌐️remote-execution/🌡️load/🪑️stage-claim/🟦️.ts` (claim, entitlement, reserve, hold)
- `B/🌐️remote-execution/🌡️load/🟦️.ts` (LoadReport builder, reporter, pressure gate)
- `B/🌐️remote-execution/🛰️sidecar/🚦️ready/🩺️workers/🟦️.ts` (readers of ready.json)
- `B/🌐️remote-execution/🛰️sidecar/🚦️ready/💔️silence/🟦️.ts` (liveness, unused watchdog)
- `B/🌐️remote-execution/🖥️server/🟦️.ts:245-252` (GET and POST `/v1/fleet/load`)
- `B/🌐️remote-execution/🗓️scheduler/🟦️.ts:408-428` (acceptLoad, fleetLoad)
- `B/🌐️remote-execution/🚢️status/🟦️.ts` (fleet facts reader)
- `B/🌐️remote-execution/🛰️sidecar/🟦️.ts:34-171` (restart, launcher, handover)
- `B/🌐️remote-execution/🛰️sidecar/🧮️closure/🟦️.ts` (closure digest)
- `B/🌐️remote-execution/🛰️sidecar/🧰️shared/🟦️.ts` (work, restartPending)
- `B/🚢️fleet-status/🟦️.ts` (types, findings, summary, render at 302)
- `B/🚢️fleet-status/📥️readers/🟦️.ts` (host, fleet, workers readers)
- `B/🚢️fleet-status/🧬️schema/🔣️.json` (Policy.findings, Messages, FleetMachine, WorkerRow)
- `B/🏃️execution/🧬️schema/🌐️remote-execution.json` (LoadReport, LoadRow, FleetLoad, WorkerWhy, Pressure)
- `B/🔣️policy.json` (loadIntervalSeconds 5 at line 578, loadExpireSeconds 15 at line 561, hostCapacity.pressure and swap thresholds)
- `B/🧠️memory/🟨️.mjs:79-83` (fleetHostReadingV1)
- `B/🏡️resident/🏛️service/🧯️failure/🟨️.mjs:210-220` (service-failure-*.txt writer)
- `…/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts` (host sample parsers, reduceHostSampleV1 at 109-117, swap-out rate)
- `…/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts:23` (HostSampleV1 type)
- `…/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts:17-21` (hostCapacityDirectoryV1)

Not touched: no source edits, no git, no build or test run. The only write is this report.
