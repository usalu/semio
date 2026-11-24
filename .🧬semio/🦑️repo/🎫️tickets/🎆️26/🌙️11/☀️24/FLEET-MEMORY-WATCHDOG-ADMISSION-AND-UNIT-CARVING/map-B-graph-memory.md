# Map B: Graph Memory Behaviour (Resident Service, Heavy Child, Governor, Slice Read)

Date: 2026-11-24. Read-only audit. Nothing was built, tested, edited or committed. The only write is this file.

Conventions used below:

- `B` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. Paths without a root are relative to `B`.
- `M` = `B/🧠️memory`, `R` = `B/🏡️resident`, `G` = `B/🕸️graph`, `O` = `B/👁️observation`.
- Line numbers are from the files as read today. JSON constants are cited by key name and schema file.
- "Derived" means I computed a number from the declared constants for a 16 GB host. Nothing here was measured at runtime.

## 0. Summary

1. A graph read runs in up to six process kinds: core resident service, graph host service, slice worker (2 on a 16 GB host), heavy-question child (up to 2 at once), farm workers (per project), and a detached warm deriver. Each has its own cap, and none of the caps is summed against the others.
2. The in-process governor (`M/🟨️.mjs`) measures `heapUsed + external` (`M/🟨️.mjs:111`), not RSS. Its admission headroom comes from the cgroup or host free memory, minus 10 %. Overshoot never kills a process. It stops the owner whose growth passed its limit (`M/🟨️.mjs:170-180`).
3. The 2560 MiB number appears twice with different owners. The graph host's `maximumResidentBytes` is enforced only while no graph read is in flight (`R/🏛️service/🌐️host/🟨️.mjs:408`). The heavy-question child's `memoryLimitMegabytes` is enforced by the child itself and ends in exit 137, with no retry at a larger size.
4. `slice-memory.json` is written by the slice worker every 30 s (`M/📋️slice-report/🟨️.mjs:229-243`). Its `guard` is `heapUsed + external`. `ready.json` `peakMegabytes` is the maximum over 5 s samples of the summed RSS of the whole stage process tree (`B/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts:1076`). The two numbers therefore measure different things, and the plan guard uses a third (footprint).
5. The 130 MB / 89 MB "estimated" collection sizes are inflated. The estimator JSON-serialises values, and `Uint32Array` and `Buffer` serialise far larger than their bytes (`M/📋️slice-report/🟨️.mjs:46-49`). The real byte budgets of those two maps are 16 MiB and 32 MiB.
6. Real unbounded or byte-unbounded growth exists in the observation module (`O/🟨️.mjs`), in the owner facts memo (bounded by entry count only), in the host plan memo (entry count only), and in the heavy-child spawn path (no shared accounting).

---

## 1. Processes during a graph read

### 1.1 Process inventory

| Process | Entry and spawner | Memory limit and sizing | Measured by |
|---|---|---|---|
| Core resident service | `R/🏛️service/🟨️.mjs`, composed at `R/🏗️composition/🟨️.mjs:45` | `POLICY_V1.maximumResidentBytes` = 536870912 (512 MiB), `R/🧬️schema/🔣️.json` Policy | `rss`, timer at `R/🏛️service/🟨️.mjs:502`, every `residentCheckMilliseconds` (5000) |
| Graph host service | `R/🏛️service/🌐️host/🟨️.mjs`, composed at `R/🏗️composition/🌐️host/🟨️.mjs:65`, launched at `…/🏗️composition/🌐️host/🟨️.mjs:107-140` | Env `BUN_JSC_forceRAMSize` = `serviceHeapMegabytes` (1024 MiB) and `MIMALLOC_PURGE_DELAY=0` (`…/🌐️host/🟨️.mjs:107`). cgroup limit `graphServiceLimitMegabytesV1(host)` = max(2048, floor(0.6 × host)) (`M/🟨️.mjs:73-76`), so 9830 MB on 16 GB. Soft ratio 9000 bp (`…/🌐️host/🟨️.mjs:111`). The cgroup path applies on Linux only (`linux` at line 111). | `HostPolicy.maximumResidentBytes` = 2684354560 (2560 MiB), `R/🧬️schema/🔣️.json` HostPolicy. Checked by `sweep()` every `residentCheckMilliseconds` (5000), `R/🏛️service/🌐️host/🟨️.mjs:459` |
| Slice worker (the "project-graph slice read") | `R/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts`, spawned by `createProcessWorkerV1` inside `…/👷️slice-pool/🟨️.mjs:67`. Pool built at `B/⌨️command/🌐️host/🟦️.ts:23` | `maximum = max(2, min(4, floor(hostMB / 8192)))` → 2 on 16 GB. Per-run env at `…/👷️slice-pool/🟨️.mjs:67`: `graphChildEnvironmentV1()` (`BUN_JSC_forceRAMSize` 512 MiB, `M/🐏️child-environment/🟨️.mjs:8`), `SEMIO_SLICE_RECYCLE_HEAP_MB` = 2048 (`M/♻️recycle` schema, `heapMegabytes`), `SEMIO_GRAPH_CUT_CONCURRENCY` (`M/✂️cut/🟨️.mjs:26`), `SEMIO_GRAPH_MEMORY_MB` (only when the reservation is smaller than booked), `SEMIO_GRAPH_PLAN_CEILING_MB` | Plan guard on the footprint of the whole tree (`observeTreeMegabytesV1`, default measure `footprint`), sampled each `sampleMilliseconds` = 1000 (`…/👷️slice-pool/🟨️.mjs:96`). Heartbeat 2000 ms. |
| Heavy-question child | Spawned by `G/📥️inference/🧭️script-inputs/🪆️heavy-questions/🟨️.mjs:59` (`process.execPath` running `📜️script.ts`). The caller is the process that derives the row: `G/📥️inference/📦️project-nodes/🟨️.mjs:502` (`isHeavyQuestionV1(row) ? askHeavyQuestionV1(...)`) | `memoryLimitMegabytes` = 2560 (`…/🪆️heavy-questions/🔣️.json:4`). Env at line 59 is `{...process.env, SEMIO_HEAVY_QUESTION_CHILD: "2560"}`. No `BUN_JSC_*` is set here, so the child inherits whatever its parent had (the slice worker's 512 MiB, if the parent is a slice worker). Kinds that trigger it: `program`, `derive` (`…/🔣️.json` `kinds`). | `rss` polled every 200 ms by the child itself (`…/📜️script.ts:11`). Silence kill after `silenceMilliseconds` 60000 × scale (`…/🟨️.mjs:74`). Timeout 900000 × scale (`…/🟨️.mjs:67`). Scale = min(8, speed × max(1, load/cores)) (`…/🟨️.mjs:41-44`). |
| Farm worker (per-project inference) | `G/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs`, worker `…/🏊️project-farm/👷️worker/🟨️.mjs`. Count from `planFarmWorkersV1` (`M/🪜️plan-stage/🟨️.mjs:93`). Pool bounds at `…/🏊️pool/🟨️.mjs:85`: `work = floor(projects/6)`, `cpu = cores − 1`, `memory = maximumWorkers` | `workerMegabytes` = 1536 (`…/🏊️pool/🧬️schema/🔣️.json:17`). BUN heap env = floor(1536 × 0.75) = 1152 MiB (`…/🏊️pool/🟨️.mjs:124-128`, `ramFactor` 0.75). Footprint cap default `farmChildFootprintMegabytes.capMegabytes` = 1400, alone cap 4200 (`M/🪜️plan-stage/🧬️schema/🔣️.json`). Recycle when the footprint is above 1.5 × baseline (`…/🏊️pool/🟨️.mjs:117`), or after 96 projects (`…:119`). | Footprint of each worker every 1000 ms (`footprintMilliseconds`), `…/🏊️pool/🟨️.mjs:304`. Over cap → `capped` / `cappedAlone` (`…:262`). |
| Warm deriver (detached) | `B/🧭️invocation/🕸️graph/🧠️memo/🧮️derive/♨️warm/⌨️main/🟦️.ts` | `maximumResidentBytes` = 2147483648 (2 GiB) in the memo schema, `warm` block | Self-exit when rss is above the limit (`…/⌨️main/🟦️.ts:23` and `:32`). It is detached, so it outlives the read. |
| Owner pool child (fleet) | `G/📥️inference/📦️project-nodes/🌐️owner-pool/🧒️child/🟨️.mjs` | Not audited in depth | Not audited |

Notes:

- Two different "resident services" exist. The graph host (`R/🏛️service/🌐️host/`) holds the graph state and plans. The core service (`R/🏛️service/🟨️.mjs`) holds watcher state and gets `ports.graph`. The live evidence (rss 3.5–5.4 GB, `slice-memory.json`) comes from the slice worker, not the host.
- The ready-stage process (`B/🌐️remote-execution/🛰️sidecar/🚦️ready/`) is the build stage that contains the read. Its `peakMegabytes` sums the RSS of every descendant (Section 4).

### 1.2 Spawn sequence for one read

1. The CLI asks the graph host (`R/🏛️service/🌐️host/`). The host runs plans through `slicePool` (`B/⌨️command/🌐️host/🟦️.ts:23`).
2. A slice worker (`👷️worker.ts`) loads the slice. It writes `slice-memory.json` (Section 4).
3. Inside the worker, the owners are inferred. Each owner is admitted by the in-process governor (Section 2). Farm workers take projects out of the process (`…/🏊️project-farm/🟨️.mjs:119`, `reserveFarmV1`).
4. A project whose rows are heavy (program or derive) calls `askHeavyQuestionV1`. That spawns a heavy child in one of two serial lanes, `questions` or `prime` (`…/🟨️.mjs:48-49`, `:88`, `:91`). The two lanes can run at the same time.

---

## 2. The memory governor

Source: `M/🟨️.mjs`. Constants: `M/🧬️schema/🔣️.json`, Policy.

### 2.1 What it measures

- Sample: `graphMemoryPressureBytesV1(counters) = heapUsed + external` (`M/🟨️.mjs:111`). `createMemoryPortsV1().sample` (`:114`) calls it. `collect` runs `runtimeCollectGarbageV1(true)` (`:115`).
- Headroom for the parent (`createGraphParentMemoryGovernorV1`, `:297-300`):
  - `host()` = `serviceMemoryHeadroomBytesV1` (`:120-125`):
    - If `SEMIO_GRAPH_MEMORY_LIMIT_MB` is set: limit − usage − 10 % of the limit. Usage is the footprint if the gate exists, else RSS.
    - Else with a cgroup: `memory.max − memory.current − 10 % of limit` (`cgroupMemoryHeadroomV1`).
    - Else (macOS): total − (total − available) − 10 % of total, i.e. available − 10 % of total.
  - With a `budget`: `min(host(), budget − (sample() − baseline))` (`:299`).
- Attributable growth (`attributable`, `:299`): `ports.sample` only when there is no cgroup headroom.
- The `hold` port (`:299`) blocks admission while `footprintGateV1().headroomBytes() < 0`.

### 2.2 Budget formula

- Parent managed budget, `graphParentManagedBytesV1` (`M/🟨️.mjs:303-308`):
  - Base `parentManagedMegabytes` = 1536.
  - If host ≥ `serveHostMegabytes` (24000): `max(1536, floor(host × 0.25))`.
  - Else: `min(1536, floor(host × parentHostShare 0.0625))`. On 16 GB this is **1024 MB**.
  - If a plan allocation is qualified, `min(…, allocation.parentMegabytes)`.
  - Result: `max(minimumBudgetMegabytes 1024, …)`.
- Per owner (`costOf`, `:167`): `max(peakOf(kind), recordedOf(row))`.
  - `peakOf(kind)` = max of the learned samples of that kind, floored at `floorOwnerMegabytes` 64; `initialOwnerMegabytes` 256 when unlearned (`:167`).
  - Learned samples: last `peakSamples` = 8 completed owners (`:272-277`). `ceilingOf(kind)` (`:162`) is `peakCeilingMegabytes` 4608 for the root kind and `grantMegabytes` for every other kind. A sample at or above its ceiling is replaced by the estimate (`:163`).
- Per-owner grant: `grantMegabytes` = 2048 (`:165`). `ownerLimitBytes` overrides it when set.
- Admission (`graphOwnerAdmissionV1`, `:145-150`): first owner always runs. An owner of an unlearned kind runs alone (`:147`). Otherwise `headroom − Σ max(0, estimate − taken) of running owners ≥ estimate`.
- Parent collector (`createParentCollectorV1`, `:314-329`):
  - After every `collectEveryProjects` = 2 finished projects (`:321`).
  - If managed bytes ≥ `collectAboveMegabytes` 768 and past the quiet interval, run a full GC (`:318`). The next trigger is `max(768, live + collectGrowthMegabytes 384)`.
  - Quiet interval ≥ `collectMinimumIntervalMilliseconds` 3000, and at least `collectDutyPercent` 10 % of the GC duration.

Wiring (`G/📥️inference/📦️project-nodes/🟨️.mjs:756`): the governor gets `budgetBytes: parentBytes` **only when there is no farm** (`farm===undefined ? {budgetBytes:parentBytes} : {}`). With a farm, the managed-byte budget does not enter admission. The collector still uses the budget (`:755`).

### 2.3 What waits on it

- Owners inside the process only. `run()` (`:250-252`) queues the owner and awaits admission. Farm children are not governed by this queue; they are reserved by `reserveFarmV1` (plan stage).
- `away(work)` (`:265`) releases the owner's slot while a farm child works, so the queue bounds what the process holds, not what the children hold.
- Wait text: `"waiting"` (`M/🧬️schema` Messages), "graph: waiting for memory headroom ({used} of {budget} MB reserved, …)", printed by `say("waiting")` at `:239`. `used` = Σ max(estimate, taken). `budget` = free + Σ taken. This matches the observed `waiting: 1719 of 2099 MB`.
- Wait bound: after `waitBoundMilliseconds` 60000 with at most one owner running, the smallest pending owner is admitted if its estimate fits `grantBytes` and free memory (`:217-223`). If the estimate is above both grant and free, the wait is refused with exit 75, `BUILD_GRAPH_MEMORY_REFUSED` (`:224`). After `fleetFallbackMilliseconds` 180000 the fleet is asked (`:211-216`).

### 2.4 Interval

- Sampling timer: `setInterval(sample, pollMilliseconds)` = 50 ms, started at `:253`.
- Pump loop sleep: 50 ms (`pollMilliseconds`, `:242`). Admission spacing: 100 ms (`admitMinimumMilliseconds`, `:201-202`).
- Progress line: every 5000 ms (`progressMilliseconds`, `:236`).

### 2.5 Overshoot: what happens

- `enforce()` (`:170-180`), run on every sample. For each owner whose growth exceeds its limit:
  1. Run one full collection, at most once per `stopCollectMinimumMilliseconds` 1000 (`:174`).
  2. If still over, `owner.scope.exceed(...)`. The owner's inference stops at its next yield point. The owner's taken bytes are not learned (`:273`), so one heavy owner does not make its kind heavy.
  3. `onStop` and `costs.record` receive the numbers.
- Nothing in the governor kills a process. Recycling of slice workers is a different mechanism (Section 3.3).
- Growth is attributed by `measure()` when a single owner runs alone, and by `attributable()` otherwise (`:169`, `:184-189`).

---

## 3. The resident cap (2560 MiB) and its enforcement

### 3.1 Graph host service: `maximumResidentBytes` = 2560 MiB

- Declared at `R/🧬️schema/🔣️.json`, HostPolicy: `"maximumResidentBytes": 2684354560` (2560 MiB). Also `maximumIndexBytes` 2684354560.
- `sweep()` at `R/🏛️service/🌐️host/🟨️.mjs:402-414`, every `residentCheckMilliseconds` 5000 (`:459`):

```
const rss=…,governed=(ports.workers?.active?.()??0)>0||governor.inFlight>0||[...workspaces.values()].some(w=>w.active>0)||[...connections].some(c=>c.requests.size>0);
if(!governed&&rss>policy.maximumResidentBytes){ close workspaces; pool.clear(); memo.clear(); Bun.gc(true);
  if(rss-after > policy.maximumResidentBytes*1.25) drainAndExit("memory"); }
```

- So while any read is in flight (`governed`), the 2560 MiB cap is **not enforced at all**. The host can grow without bound during a long read. When no read is in flight and RSS is above 2560 MiB, it clears caches and runs `Bun.gc`, and drains only if RSS is still above 3200 MiB afterwards.

### 3.2 Core service: 512 MiB, ungated

- `R/🏛️service/🟨️.mjs:500-503`: `if(rss > policy.maximumResidentBytes) { state.dropCaches(); runtimeCollectGarbageV1(true); if (rss > max*1.25) drainAndExit("memory") }`. `policy` = `POLICY_V1` (512 MiB), not the host policy. This check has no `governed` gate. It guards watcher state, not the graph.

### 3.3 Heavy child: `memoryLimitMegabytes` = 2560 MiB, exit 137

- Child side: `G/…/🪆️heavy-questions/📜️script.ts:8` reads `limit` from the env. `:11`:

```
setInterval(()=>{if(process.memoryUsage().rss>limit){process.stderr.write("resident memory cap of "+…+" MB exceeded\n");process.exit(137);}},HEAVY_QUESTION_POLICY_V1.pollMilliseconds).unref();
```

- Parent side, `…/🪆️heavy-questions/🟨️.mjs`:
  - Spawn (`:59`) with env `SEMIO_HEAVY_QUESTION_CHILD=2560`.
  - Output file (`:55`) is created but never written when the child exits 137. `child.once("close")` (`:77-83`):
    - `silent` → `graph.heavy-question-silent` (`:80`).
    - `code !== 0` → `failureOfV1(code, tail, output)` (`:81`).
  - `failureOfV1` (`:29-37`): the output file is missing, so the catch runs. `named` lines match `/^(error|[A-Za-z]*Error)\b/` and do not match "resident memory cap…". The fallback is the last 3 stderr lines. Result: `refuseBuildV1("failed","graph.heavy-question-failed",{exit:"137",detail:…})` (`:36`). The message key is in `B/📖️texts/🔣️.json:141`.
- After exit 137, the refusal is returned to the caller `project-nodes/🟨️.mjs:502`, then to the owner-failure path `G/📥️load/🛟️owner-isolation/🟦️.ts:80-86` (`ownerFailureDecisionV1`):
  - `foreign` if the error is another layer's.
  - `stable` at once if the rows did not move and were not written in the racy window. This is the normal case for a memory kill, so no retry happens.
  - `retry` only if the rows moved or are young, up to `retryRounds`.
- There is **no escalation**: the retry, if any, uses the same child with the same 2560 MiB cap. No learned peak is recorded for heavy questions. The heavy-child cap is not part of the governor's grant.
- Concurrency: two lanes (`questions`, `prime`) can run two children at once, each up to 2560 MiB RSS, for up to 5120 MiB outside any grant. This is derived from `…/🟨️.mjs:48-49, :88, :91`.

The string the user quoted, "heavy-question-failed exit 137 resident memory cap of 2560 MB exceeded", is the `detail` of the refusal above.

---

## 4. `slice-memory.json`, `peakMegabytes` in `ready.json`, and the plan guard

### 4.1 `slice-memory.json`

- Written by `startSliceReportV1` (`M/📋️slice-report/🟨️.mjs:229-243`), called from the slice worker at `…/👷️worker.ts:39` with `root`, `footprint:{self,tree}`, and `heap:heapStats`.
- Location: `machinePathV1(root, P.directory)` with `directory` = `.cache/f` and `name` = `slice-memory.json` (`M/📋️slice-report/🧬️schema/🔣️.json` Policy). `writeSliceReportV1` (`:220-225`). When readiness owns a slot, the owner path is `.cache/f/ready/slice/<slot>/slice-memory.json` (`M/📋️slice-report/🏷️owner/🟦️.ts:11-19`, `:40`).
- Timer: `intervalMilliseconds` 30000 (`:242`). `publish("end")` on stop.
- Collection: `collect()` at `:235-241`. Runs `Bun.gc(true)` **once per process**, the first time `heapUsed ≥ collectMinimumHeapMegabytes` 2500 (`:236-238`). The field `afterCollect` records the result.
- Contents (`buildSliceReportV1`, `:206-217`):
  - `memory`: `rss`, `heapUsed`, `heapTotal`, `external`, `arrayBuffers` (process.memoryUsage).
  - `guard`: `{measure:"heapUsed+external", megabytes, heapUsed, external}` (`:210`). This is the number the plan guard "compares with its limit" per the schema.
  - `footprintMegabytes` (self) and `treeFootprintMegabytes` (tree, only with children) from `measureSliceProcessesV1` (`:197-203`).
  - `processes`: the `ps` table of this process and its children (`parseProcessTreeV1`, `:179-191`).
  - `collections`: the top 20 by estimated bytes (`collectSliceCollectionsV1`, `:53-60`). See Section 5.3 for the estimator.
  - `sources`: each registered part's own report.
- Progress text "Reading the project graph slice (…): reading the owners (round 1) 11 of 3600" comes from `startSliceProgressV1` (`:127-150`) and the step table in the schema. The `owners-round` bound is 3600000 ms (`M/📋️slice-report/🧬️schema/🔣️.json`, Steps).

### 4.2 `peakMegabytes` in `ready.json`

- Sampler: `B/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts:951` sets `sampleEvery = round(samplerMilliseconds / pollMilliseconds)` = round(5000 / 250) = 20 ticks.
- At `:1076`: `ps -A -o pid=,ppid=,rss=,time=`, then `readyTreeMegabytesV1(rows, inflight.pid)` (`:230-236`).
- `readyTreeMegabytesV1` sums the `ps` RSS column over the stage pid and every descendant (depth ≤ 64), in MB. This is **resident set, summed over processes**. Shared pages are counted once per process.
- `peakMegabytes` = max over samples (`:1077`). Anything that lives between two samples (5 s apart) is missed. It can only undercount spikes, and can overcount shared pages.
- Consequence for the live numbers: a stage tree that holds the slice worker (3.5–5.4 GB RSS), two farm workers, and heavy children would sum to the observed 5.5–10.7 GB without any single process being above its cap. This explanation is inferred from the code, not measured.
- The learned slice peak used by the scheduler is a different measure: `outcome.peakMegabytes` from the pool's footprint sampling of the tree (`…/👷️slice-pool/🟨️.mjs:69`, `:89`), and `recordHostSlicePeakV1` (`B/🗓️scheduler/📈️learn/🟦️.ts:41`).

### 4.3 The plan guard (what actually stops a slice)

- Measure: footprint of the tree, `observeTreeMegabytesV1` with default `"footprint"` (`🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts:295`, outside `B`), sampled every 1000 ms (`…/👷️slice-pool/🟨️.mjs:96`).
- Limit and ceiling (`…/👷️slice-pool/🟨️.mjs:72`, `:81`, and the verdict at `…/🧱️limit/🟨️.mjs:54-57`):
  - `planLimitMegabytes`: plan-stage `hostShare` 0.3, minimum 2048, maximum 16384, `treeShare` 0.4. On 16 GB, about 4915 MB (derived).
  - `ceilingMegabytes` = max(host × ceilingBasisPoints 6000 / 10000, planLimit) = 9830 MB on 16 GB (derived).
  - `refuse` if the tree is above the ceiling. `settle` if above the limit and not yet settled (`settleWaitMilliseconds` 5000).
- The check on memory budget (`…/👷️slice-pool/🟨️.mjs:96`):

```
if(grown>budgetMegabytes||current+marginMegabytes>limit){ … code:"memory-budget" … }
```

  - `budgetMegabytes` is **not passed** by the host (`B/⌨️command/🌐️host/🟦️.ts:23`), so it stays `Infinity`.
  - `limit` comes from `headroom()`. The host passes `cgroupMemoryHeadroomV1() ?? {limitBytes: Infinity, …}`. On macOS there is no cgroup, so `limit` is `Infinity` and this check never fires. On Linux it checks `memory.current` against `memory.max`, which includes page cache.
- Retry and cut ladder (`…/👷️slice-pool/🟨️.mjs:147-151`): `plan-cut` lowers `level` (cut ladder, `M/✂️cut/🟨️.mjs`, 6 levels, halving concurrency and wave). `slice-recycle` restarts the worker at a pass boundary (Section 5.4). `owner-too-heavy` hands the owner to the fleet. `memory-budget` and `memory-pressure` cut the level.

---

## 5. In-memory tables: definitions, bounds and growth

For each collection named in the report: definition, bound, and how the bound is enforced. Byte figures are the declared budgets. "Reported" means what `slice-memory.json` prints.

### 5.1 Named collections

| Collection (as printed) | Definition | Bound | Enforcement | Notes |
|---|---|---|---|---|
| `observation:sourceAnalysisImages` | `O/🟨️.mjs:143` `new Map()`; keyed by provider+path+sha | 16 MiB (`sourceAnalysisBytes` 16777216, `O/🧬️schema/🔣️.json:537`) | Byte-accounted: `size = Buffer.byteLength(image) + 4·nested + 192` (`:521-522`); evict oldest while over (`:523`) | Reported 130 MB is an estimate. See 5.3. |
| `graph:ownerFactsMemo` | `G/📥️load/🟦️.ts:58` `new Map()` of `{facts, rows}` per owner scope | **Entry count only**: `ownerFactsMemoEntries` 16384 (`G/🧬️schema/🔣️.json:16`), enforced at `:64-67` | Entries evicted oldest-first | Rows per entry are unbounded in bytes. See Section 7, gap 3. |
| `graph:ownerFactsMemoRows` | Distinct rows over all memo entries (`G/📥️load/🟦️.ts:62`) | Follows the entries | — | Reported as an estimate of row bytes. |
| `observation:settledReads` | `O/🟨️.mjs:1010`, `new Map()` of `{size,mtimeMs,ctimeMs,ino,bytes:Buffer,rowId}` | 32 MiB (`settledReadBytes` 33554432, `O/🧬️schema/🔣️.json:539`); per file ≤ 4 MiB (`settledReadMaximumFileBytes`) | Whole-map clear when over (`:1043`) | Reported 89 MB is an estimate. `Buffer` serialises to `{"type":"Buffer","data":[…]}`, so the JSON estimate is several times the bytes. |
| `observation:fileRecordImages` | `O/🟨️.mjs:1013` | 16 MiB (`fileRecordImageBytes`) | Budget constant `FILE_RECORD_IMAGE_BYTES` at `:1013` | Cleared by `releaseInferenceMemoryV1` (`:809`). |
| `observation:rowTable` | `O/🟨️.mjs:308` `createRowTableV1({maximumRows: 1048576, maximumCharacters: 16777216})` (`O/🧬️schema/🔣️.json:568-569`) | As stated | Whole-table `clear()` on overflow (`O/🔢️row-table/🟨️.mjs:122`) | Reports real bytes (typed-array byte lengths, `…/🔢️row-table/🟨️.mjs:171`), so this figure is not an estimate. Clear-all on overflow can thrash. |
| `observation:rowPool` | `O/🟨️.mjs:316` | — | — | Reported as entries × 160 bytes (`:787`), a fixed guess. |
| `memory:graphStringInterner` | `M/🟨️.mjs:335-371` | `maximumInternedCharacters` 8388608 (8 MiB), `minimumLength` 24 (`M/🧬️schema/🔣️.json`) | Whole-table clear when characters exceed the bound (`:341`) | The bound counts characters, not Map entries. Entries are at most about 350k. Reported as 2 × chars + 48 × entries. |
| `observation:retainedProjectImages` / `projectImages` | `O/🟨️.mjs:150-158` | `GRAPH_MEMORY_POLICY_V1.retainedImageBytes` 33554432 (32 MiB), counted in string length | Oldest removed from both sets (`:157`) | Bounded. |
| `observation:projectComputations` | `O/🟨️.mjs:136`, set at `:242` | None | Never deleted | Grows with projects per process lifetime. |
| `observation:inferenceStamps` | `O/🟨️.mjs:50`, set at `:52` | None | Not cleared by `releaseInferenceMemoryV1` | Grows with registered stamps. |
| `observation:loadObservations` | `O/🟨️.mjs:135` | None | `clear()` only on a new load (`:230`) | Holds every load observation of the load. |
| `observation:fieldDigests` | `O/🟨️.mjs:924`, cleared at `:932` only | None | Partial | Not in `releaseInferenceMemoryV1`. |
| `observation:recordStores`, `recordStoreHandles` | `O/🟨️.mjs:131-132` | None | Deleted per path (`:1249`, `:1255`) | Depends on path churn. |
| `observation:originalInferencePublications`, `usedInference` | `O/🟨️.mjs:133-134` | None | Cleared in `releaseInferenceMemoryV1` (`:810`) | Bounded only by release calls. |
| `observation:sourceAnalyses` | `O/🟨️.mjs:141` | Keyed by provider id | — | Bounded by number of providers. |
| `observation:verifiedBlocks` | `O/🟨️.mjs:1494` | `verifiedBlockRows` 1048576 (`O/🧬️schema`) | Counted rows | Cleared at `:808`. |
| `resident:digest-owners` | `R/🗃️state/🟨️.mjs:52` | Bounded cache (`createBoundedCacheV1`) | Bounded | — |
| Plan memo (host) | `R/🏛️service/🌐️host/🟨️.mjs:310` `memo.set(memoKey, plan)` | `planMemoEntries` 256 | Entry count only, `while(memo.size>…) delete` | Each plan can be large. See gap 6. |
| Heavy-question `preflights` | `…/🪆️heavy-questions/🟨️.mjs:95` | One entry per workspace and policy | — | Negligible. |

### 5.2 Heavy child outputs and farm outputs

- The heavy child's answer is one JSON file (`answer.json`), read once and removed (`…:82`, `rmSync` at `:58`). It does not grow in the parent except through the returned `rows`.

### 5.3 Why the reported numbers are inflated

- `describeCollectionV1` (`M/📋️slice-report/🟨️.mjs:19-24`) takes a sample and `estimateCollectionBytesV1` (`:46-49`) sums `JSON.stringify(item).length` for non-strings and `2 × length` for strings, then scales by entry count.
- For `sourceAnalysisImages`, the item is `[key, {image, ids: Uint32Array, bytes}]`. `JSON.stringify` on a `Uint32Array` yields an object with one entry per index (`"0":…,"1":…`), which is roughly 8–12 characters per id.
- For `settledReads`, the item carries a `Buffer`. Its JSON form is `{"type":"Buffer","data":[…]}`, with one decimal number per byte. This is several times the bytes.
- `sharedBytesV1` (`:159-169`) was written to correct this: it charges each object once and each typed array by its `byteLength`. It is used only in `B/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:8`, for the closure entries (`:1430`). The slice collections (`collectSliceCollectionsV1`, `:55`) still use the JSON estimate.
- So for the numbers the user has, 130 MB and 89 MB are estimator outputs, not measured bytes. The budgets are 16 MiB and 32 MiB.

### 5.4 Recycle

- `♻️recycle` (`M/♻️recycle/🟨️.mjs`): `sliceRecycleDecisionV1` triggers when the live heap (`liveHeapBytesV1` = `Bun.gc(true)` then `heapUsed`, `G/📥️load/♻️pass-reuse/🟦️.ts:20`) is above `heapMegabytes` 2048 (`SEMIO_SLICE_RECYCLE_HEAP_MB`) at a pass boundary (`…/♻️pass-reuse/🟦️.ts:43-44`). `maximumRecycles` = 3.
- Recycling measures `heapUsed` only (not external, not RSS). Heap and external together were 2.4 GB in the live example, so `heapUsed`-based recycle can fire later than the managed bytes suggest.

---

## 6. Why RSS (3.5–5.4 GB) is about 2× heapUsed + external (2.4 GB)

Code notes that bear on this:

1. `M/🟨️.mjs:110`: the governor charges "the managed bytes (heap in use plus external buffers), never the resident pages: with a small collector target the pages hold freed memory the allocator has not returned yet, which no admission can release."
2. `M/🦶️footprint/🧬️schema/🔣️.json` (description): the resident set "falls to a few MB while the pages are swapped out"; the managed heap "misses the allocator's own pages". The gate reads physical footprint (macOS `phys_footprint`, Linux `smaps_rollup`), not RSS.
3. `R/🧬️schema/🔣️.json` HostPolicy, `memoryPurge` description: `MIMALLOC_PURGE_DELAY` 0 returns freed memory at once. "the service worker's resident size after a settle fell from about 1.5 GB to 1.1 GB". The slice pool sets the same variable (`…/👷️slice-pool/🟨️.mjs:67`), so mimalloc-owned pages should be returned quickly. JSC-owned heap blocks are not covered by mimalloc purge.
4. `M/🟨️.mjs:101-103`: `BUN_JSC_forceRAMSize` is the heap size JSC uses to decide when to collect. The comment measured: at 2 GiB, 5.5 GB resident and 2.0 GB footprint; at 8 GiB, 7.5 GB resident and 2.8 GB footprint, with identical rows. The target is a collection heuristic, not a hard limit. With 512 MiB as the target, `heapUsed` can still be 1.4 GB if the collector does not run.
5. `Bun.gc(true)` runs at only three points: the governor's collector (`M/🟨️.mjs:115`, `:318`), the slice report's one-time collection at ≥ 2500 MB (`M/📋️slice-report/🟨️.mjs:238`), and the host sweep when idle (`R/🏛️service/🌐️host/🟨️.mjs:410`). Between them, the heap is not shrunk.
6. Farm workers use `BUN_JSC_numberOfGCMarkers=1` and `BUN_JSC_useConcurrentGC=0` (`…/🏊️pool/🧬️schema/🔣️.json:71-72`). Those settings change collection timing and may raise peak RSS. The pool does not measure it.
7. `external` (1.0 GB in the example) includes Buffers and ArrayBuffers. The `settledReads` Buffers are owned copies (`O/🟨️.mjs:1044`). Freed Buffers go back to the allocator and may stay resident.
8. `ps` RSS also counts shared library and binary pages in every process that maps them. This inflates the tree sum but not a single process's RSS.

Likely contributors, ranked by what the code suggests (none measured):

- JSC heap pages that stay committed after collection (no shrink until a full GC).
- Freed Buffers and typed arrays in the allocator.
- The Bun runtime, JIT code and mapped files. These are not in `heapUsed + external` at all.

A heap snapshot and `heapTotal` against `rss` at the same moment would separate these.

---

## 7. Gaps: unbounded growth and capacity-gate blind spots

1. **The 2560 MiB host cap is not enforced during reads.** `R/🏛️service/🌐️host/🟨️.mjs:408` skips the check while `governed`. A long read can hold any RSS. The slice worker is not under this cap at all, only under the plan guard (footprint vs 4915 MB limit, 9830 MB ceiling, derived).
2. **MacOS has no slice memory-budget check.** `…/👷️slice-pool/🟨️.mjs:96` compares against `headroom().limitBytes`, which is `Infinity` without a cgroup. `budgetMegabytes` is never passed. The only macOS guard is the footprint plan guard and the pressure watch. Linux `memory.current` includes page cache, so it can trip early.
3. **Governor budget is off when a farm is open.** `…/📦️project-nodes/🟨️.mjs:756` passes `budgetBytes` only without a farm. The parent's managed-byte budget (1024 MB on 16 GB) then does not limit admission.
4. **The governor kills nothing.** Stopping an owner (`enforce`, `:170-180`) does not reduce process RSS. Only a full collection does, and only when it is run.
5. **Heavy children are outside the grant.** Two lanes, up to 2560 MiB RSS each, inherit `BUN_JSC_forceRAMSize` from the spawner. No learned peak, no escalation, and no reservation with the capacity gate (`B/🌡️host-capacity/`). Exit 137 retries the same size.
6. **Plan memo is entry-bounded only.** `R/🏛️service/🌐️host/🟨️.mjs:310` holds up to 256 plans of unbounded size. `planMaximumBytes` 4 GiB applies to disk retention (`:135`), not memory.
7. **`ownerFactsMemoV1` is entry-bounded only.** 16384 entries × their row arrays (`G/📥️load/🟦️.ts:58-67`) has no byte bound.
8. **Observation maps not released.** `projectComputations`, `inferenceStamps`, `loadObservations`, and `fieldDigests` are not in `releaseInferenceMemoryV1` (`O/🟨️.mjs:807-811`). A host that runs reads for hours accumulates them.
9. **The estimator misreports the top two collections.** Section 5.3. Diagnostics point at the wrong collections as culprits.
10. **Three memory measures for one question.** Governor: heapUsed + external. Slice report `guard`: heapUsed + external. Plan guard and learned peak: footprint. Ready `peakMegabytes`: summed RSS, 5 s samples. Recycle: heapUsed after GC. Decisions about one slice use different numbers. The ready number cannot be compared with the plan limit.
11. **Two farm worker sizes disagree.** `workerMegabytes` 1536 (`…/🏊️pool/🧬️schema/🔣️.json:17`, used for reservation and the BUN heap), footprint cap 1400 and alone cap 4200 (plan-stage schema), and `farmWorkerPeakMegabytes` 1200 (plan-stage schema). A worker can be reserved for 1536 and capped at 1400.
12. **Warm deriver is detached.** It has its own 2 GiB self-exit (`⌨️main/🟦️.ts:23`, `:32`). It is not reported as part of a read's process tree, so the stage sum and the plan guard may not see it. Not verified in the capacity gate.
13. **Diagnostic tracking itself costs memory.** `trackMinimumEntries` 64: each Map of 64 or more entries gets a `WeakRef` and a `FinalizationRegistry` entry (`O/🟨️.mjs:136-140`). The schema comment records 4 s of 20 s spent there for a cold owner.
14. **Row table clear-all.** The row table clears entirely on overflow (`O/🔢️row-table/🟨️.mjs:122`). Under load this can repeatedly drop reusable rows.

---

## 8. What this audit did not verify

- No runtime measurement. All numbers marked "derived" come from declared constants and host size 16384 MB.
- Bun/JSC semantics of `BUN_JSC_forceRAMSize`, `Bun.gc` and `MIMALLOC_PURGE_DELAY` were taken from code comments, not from Bun sources.
- The owner pool child (`G/📥️inference/📦️project-nodes/🌐️owner-pool/`) and the warm deriver's gating in the capacity gate were not traced.
- Whether `slice-memory.json` lands in `.cache/f/` or in the owned slot depends on `writeSliceReportTextV1` (`M/📋️slice-report/🏷️owner/🟦️.ts:40`); I read the signature but not the branch that chooses the file.
- `fieldDigests` clear paths beyond line 932 were not fully traced.
- Repo-wide `grep` for "Speichersteuerung" returned nothing. The user-facing text is "graph: waiting for memory headroom" (`M/🧬️schema/🔣️.json` Messages). "memory governor" appears only in comments and descriptions.
