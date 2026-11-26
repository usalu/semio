# 🔭️ Audit MA2: Existing Fleet Metric Sources

Read-only inventory of every source of fleet metrics, so executors reuse them instead of re-measuring. Paths are relative to the repo root `/Users/ueli/Documents/semio`. `FW` below means `🧰️framework/🛍️products/🦑️repo/🔨️modules`; `HC` means `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity`; `WARDEN` means `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden`. Writer line numbers were read from the source; where a value is a policy constant it is named by its schema key, not re-derived.

## 1. Per-origin files in `/Users/Shared/semio-store/v1/origins/<origin>/`

Sample of live store (`g1-m01`, 16 origins in total): `addresses.json`, `attest.json` (5 KB), `fleet-auth.json`, `head.json`, `home.json`, `memory.json` (4 KB), `pool.json`, about 880 `pool-published-*.txt` files, and orphan `pool.json.<pid>.tmp` files.

| File | Writer (file:line) | Cadence | Fields / schema | Cross-platform | Overhead notes |
|---|---|---|---|---|---|
| `ready.json` (status) | `FW/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts` ~386-402 (`createReadyStatusV1`, `write`, atomic tmp + rename) | on change, else `statusIntervalMilliseconds` 5000; heartbeat 30000 while running/backoff/waiting (`statusHeartbeatMilliseconds`) | schema `Status` in `.../ready/🧬️schema/🔣️.json`: `version, origin, host, platform (linux/macos/windows), pid, state (ready/running/waiting/backoff/observing/server), digest, ready, stage, stageIndex, stageCount, attempts, exit, refusal, reason, containment, peakMegabytes, tail[], nextAttemptMs, readyMs, refreshing, startedMs, updatedMs, beatMilliseconds` | all three | one small JSON per write; the schema validates before writing |
| `memory.json` (warden mirror) | `FW/.../ready/🟦️.ts:788-803` (`createReadyMemoryMirrorV1.tick`), wired in `runReady` ~1420 (poll `memoryMirrorPollMilliseconds` 1000) | writes when the key `level|suspended|resumed|cancelled` changes, else every `memoryMirrorMilliseconds` 5000 | warden state as written (`HostWardenStateV1`: `version, atMs, pid, mode (observe/enforce), code, host{totalMegabytes, availableMegabytes, swapUsedMegabytes, swapOutMegabytesPerSecond, compressedMegabytes, pressure, swapFreeMegabytes, swapVolumeFreeMegabytes}, level, hold, trigger, guardMegabytes, dangerMegabytes, resumeMegabytes, swapRoomMegabytes, headroomMegabytes, subjects{count, megabytes, byKind}, others[], ...`), or `{version, atMs, absent: true}` | all three (warden enforces on darwin/linux, observes on win32) | copy of the warden state file; 1 s poll is a Map lookup, no spawn beyond `ensure` |
| `ready-timings.json` | `FW/.../ready/🟦️.ts:1395-1408` (`timing(run)`) writes local `.cache/f/ready/ready-timings.json` and the store copy (`v1/origins/<origin>/ready-timings.json`) | once per finished stage run | schema `Timings`: `version, origin, updatedMs, runs[]` where a run has `stage, startedMs, endedMs, seconds, exit, outcome (ok/stage-failed/stage-timeout/stalled/spawn-failed/interrupted), peakMegabytes, growthMegabytes, growthInodes, attempts, digest, endEvidence, units{restoredLocal, restoredFleet, compiled, stored, skipped}` | all three | rewrites the whole file each run (max 40 runs) |
| `pool.json` (owner pool status) | `FW/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/📊️status/🟦️.ts:11-30` (`createOwnerPoolStatusV1`) | on state, wave or reason change, else at most every `statusIntervalMilliseconds`; `flush()` on demand | schema `Status` (owner-pool): `version, role (puller/publisher), origin, host, pid, state (idle/pulling/declined/too-small/off), wave, units, claimed, published, stored, held, failed, running, slots, lastError, rssMegabytes, siblings, reason, reasonText, startedMs, updatedMs, consumedByHost, waveSeconds, ownersPerMinute, totalMegabytes` | all three | **Defect:** temp name `pool.json.<pid>.tmp`, and the write is rename-based with a silent `catch`, so failed renames leave orphan `.tmp` files (at least 15 in `g1-m01`). No cleanup code found |
| `attest.json` | `FW/🤝️shared-worktree/🩺️attest/🟦️.ts:280-283` (`writeAttestationV1`, via `replaceOriginFileV1`), called from the attest loop ~770 | heartbeat + spacing gate (due when a heartbeat elapsed or the next snapshot differs and spacing elapsed); `view.txt`, `refused.json`, `retired.json` written only on change | `FleetAttestationV1`: `version, originId, groupId, platform, wallMs, hlc, sync, code, cursors, checkpoints, git{ref, commit}, view digest/counts, refusal{code, reason}, failures[]` | all three | git-state reading is the costly part (`guardsEvery`); the write itself is one file |
| `head.json` | `FW/🤝️shared-worktree/📒️journal/🟦️.ts:530` (replaces `head.json`, then `publishFlatHeadV1`), replica mirror `FW/🤝️shared-worktree/🔁️replica/🟦️.ts:3831`, closure-graph `FW/.../📥️inference/🧭️script-inputs/🌳️closure-graph/🛰️fleet/🟦️.ts:65` | after each journal append batch (fsync'd) | `FleetHeadV1`: `version, segment, bytes, events, hlc` | all three | fsync per batch; the head is the only cheap progress signal of the journal |
| `home.json` | `FW/🤝️shared-worktree/🔁️replica/🟦️.ts:1234` (`ensureHome`) | once per replica, then `homeWritten` | origin key and ownership record (origin claim) | all three | none |
| `addresses.json` | `FW/🤝️shared-worktree/🗺️topology/🧭️addresses/🟦️.ts:172` (path), refresh at ~209 (`createFleetAddressSetV1`) | TTL `FLEET_ADDRESS_TTL_MS_V1` 300000 (line 23) | `FleetAddressSetV1`: `origin, generation, hostLabel, advertise, addresses[{address, scope}], signature`; `ready.json` mtime is used as liveness (line 202) | all three | interface enumeration on refresh only |
| `fleet-auth.json` | identity `FW/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🧬️schema/🔣️.json` (`Registration`) | once per origin registration, rotated only on key change | `version, originId, group, sequence, createdAtMs, origin, keys, rotatedFrom, signature` | all three | none |
| `slice-memory.json` | graph sidecar mapping queue (read in `FW/📚️library/🏗️build/🚢️fleet-status/📥️readers/🟦️.ts:165-177`, `readWorkerMappingV1`) | written by the graph read (`sources.owners.presence.mappingQueue`), freshness `mappingFreshSeconds` | `sources.owners.taken`, `presence.mappingQueue` | all three | read is capped at `maximumMemoryRecordBytes` |
| `pool-published-<wave>-<key>.txt` | owner-pool take/publish `FW/.../📦️project-nodes/📥️take/🟨️.mjs:363-375` | per published wave | owner names stored by a pool (text) | all three | accumulates without pruning (about 880 in `g1-m01`) |

Other store files seen: `refused.json` (canary), `retired.json` (attest), `blobs` records (`FW/.../🔁️replica/🩹️blobs/🟦️.ts:29`), `beat.json` and `ready.json` in the supervisor boot dir (`FW/.../🔁️replica/🛟️supervisor/🟦️.ts:256-307`).

## 2. Host samplers

### Warden (`WARDEN/🟦️.ts`, `WARDEN/🔁️loop/🟦️.ts`, schema `WARDEN/🧬️schema/🔣️.json`)

- Tick loop `runHostWardenLoopV1` (loop 🔁️loop/🟦️.ts). Each tick: `ports.sample()` (host counters), `ports.table()` (process table), classify, decide, act. Cadence policy: `calmMilliseconds` 2000, `tightMilliseconds` 500, `ensureMilliseconds` 15000, `observeOnlyMilliseconds` 8000, `codeCheckMilliseconds` 60000, `idleMilliseconds` 600000 (stop when no subject).
- Swap read: `Swap.intervalMilliseconds` 2000 (cached between ticks; each read spawns commands).
- Process table: one `ps` with `quickArguments` every tick (pid, ppid, pgid, rss, state, etime, lstart). The command column is read only for new processes (`commandAllMinimum`, `commandSettledSeconds`), because it costs about three times the rest. `commandMilliseconds` 5000 timeout.
- Persisted artifacts: `state.json`, `lock.json`, `alarms.jsonl` (1 MiB cap, keeps the newer half at 209), warden log (`logMaximumBytes` 4 MiB), `cancelled/<pid>.<digest>.json` (max age 1 h), `mode` file (defaults to `enforce`).

### Host capacity sampling (`HC/📡️sampling/🟦️.ts`, `readHostSampleV1` ~line 200-210, `readHostCountersV1` ~line 60-95)

| OS | Memory / pressure | Swap | CPU / load | Cost per read |
|---|---|---|---|---|
| macOS (darwin) | `vm_stat` (free + inactive/speculative + file-backed/purgeable pages), `sysctl kern.memorystatus_vm_pressure_level` (pressure), `sysctl -n vm.swapusage` | `vm.swapusage` plus `df -k` on `/System/Volumes/VM`, `/System/Volumes/Data`, `/` (first answer) | `loadavg()[0]` | 3 mandatory and 2 optional spawns (`sysctl kern.num_files`, `hdiutil info` bounded and skipped in the warden's quick probe) |
| Linux | `/proc/meminfo` (MemAvailable, SwapTotal/SwapFree), `/proc/vmstat` (swap in/out), `/proc/pressure/memory` (PSI), `/proc/sys/fs/file-nr`, cgroup limits from `/proc/self/cgroup` (`memory.max`, `cpu.max`, v1 quota) | `/proc/meminfo` | `loadavg()[0]`; cores capped by cgroup CPU quota (`readHostCoresV1`) | file reads only, no spawn; warden reads `smaps_rollup` only for RSS above `chargeMinimumKilobytes` |
| Windows (win32) | `GlobalMemoryStatusEx` via `bun:ffi` in `FW/.../🪟️native-memory` (`dlopen kernel32.dll`), `K32GetProcessMemoryInfo` for process private usage; fallback `os.freemem()` with `partial: true` | null (observe only) | busy share from two `os.cpus()` readings (`busyLoadV1`, first call returns null) | in-process FFI, no spawn; no `ps`, no signals (`Platforms.observeOnly`) |

Disk: `statfsSync` (`node:fs`) on every platform, in `HC/🧹️disk-recovery/📊️volume-facts/🟦️.ts:11` (files, ffree, bytes), and `HC/💽️`-side admission in `FW/.../🏗️build/💽️disk` used by build admission. Cost: one syscall per path.

Available memory for build decisions: `createHostAvailableBytesV1` (`HC/📡️sampling/🟦️.ts`, cached for `LIMITS.availableAgeMilliseconds`). **Note:** on darwin it calls `spawnSync("vm_stat")`, which blocks the caller. Executors should read the cached value, not call it per frame.

Sample record (`HostSampleV1`, reduced by `reduceHostSampleV1`): `version, atMs, platform, cores, totalMegabytes, availableMegabytes, pressure (normal/warn/critical), swapUsedMegabytes, swapOutMegabytesPerSecond (rate of pswpout+pswpin), load, openFiles, openFilesMaximum, attachedImages, ...`.

## 3. Build counters, status commands and HTTP endpoints

### Unit cache (`[units]`)

- Writer: `FW/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts` (`countUnitV1` ~line 15, `countSkipV1` ~line 30, `unitLedgerV1`, `unitLedgerLinesV1` ~line 108-111).
- Storage: `machinePathV1(workspace, ".cache","f","unit-counters")` with one byte-per-event file each for `localHit`, `remoteHit`, `miss`, `stored`, a `since` timestamp, and `skip/<reason>` files. Lock-free, monotonic, never reset (only by deleting the directory).
- Run ledger: growth between start and end of a stage run (`unitLedgerV1`), printed as two `[units] …` lines (EN and DE) and stored in `ready-timings.json` as `units`.
- Exposed in: `ready-timings.json`, and fleet status per machine `cacheHit`, `cacheMiss`, `cacheSkipped` (`RemoteExecutionFleetMachineV1`, `FW/.../🚢️status/🟦️.ts`).

### Offload counters

- `FW/📚️library/🏗️build/🌐️remote-execution/🚀️offload/📊️counters/🟦️.ts`: `out`, `refused`, `fellBack` as files under `.cache/f/offload-counters/`, plus `reason/<reason>` files (`no-idle-machine`, `worker-policy`, `incomplete-inputs`, `infrastructure`, ...). Read by the load reporter and sent in `POST /v1/fleet/load`.
- Scheduler placement counters: `offloadedIn`, `offloadedOut`, `refused` (`FW/.../🗓️scheduler/⚖️placement/🟦️.ts:5,15`). `fleetLoad()` at `FW/.../🗓️scheduler/🟦️.ts:467`.

### Remote cache (AC/CAS/Nx)

- Counters: `RemoteCacheCountersV1 = {ac, cas, nx} x {hits, misses, puts, conflicts, rejected, stale}`, in memory in `FW/.../🌐️remote-cache/🖥️server/🟦️.ts:97`. Reset on process restart (`startedAt` is reported).
- Divergences: `FW/.../🌐️remote-cache/🧾️provenance/🟦️.ts` (evidence file, `appendFile`), reported as `divergences` in `/status`.
- Nx single-flight and scratch: `FW/.../🌐️remote-cache/🧶️nx/🚦️single-flight/🟦️.ts`, `.../🧰️scratch/🟦️.ts`.
- HTTP endpoints (cache server, `🖥️server/🟦️.ts:160-171`): `/healthz` (unauthenticated `{"status":"ok"}`), `/status` (auth; `version, protocols, entries, bytes, evicted{entries, bytes}, blobs{referencedBytes, boundBytes, evictedBytes}, reservedBytes, maximumBytes, maximumEntryBytes, readOnly, validation, counters, startedAt, fleet, divergences`), `/evidence` and `/evidence/`, `/mirror/*`. Protocol routes: `/ac/<sha256>`, `/cas/<sha256>` (Bazel), `/v1/cache/<hash>` (Nx), each optional `/<instance>` prefix.

### Remote execution

- Server (`FW/.../🌐️remote-execution/🖥️server/🟦️.ts:229-246`): `/healthz`, `/status`, `/v1/fleet/idle?origin=…&peak=…`, `GET|POST /v1/fleet/load` (POST `LoadReport` from each worker, origin-checked), `POST /v1/operations/<id>/cancel`.
- Fleet aggregate: `readRemoteExecutionFleetFactsV1` (`FW/.../🚢️status/🟦️.ts:17`): `scheduler, expireSeconds, machines[{origin, platform, pressure, load, cores, running, queued, headroom, ageSeconds, worker (absent/idle/busy/refusing), offloadedIn, offloadedOut, refused, cacheHit, cacheMiss, cacheSkipped}], queued, executing, offloadRejected, refused, retried, cache{hits, misses, puts}, claims{active, waiting, oldestSeconds}, keys, retention`. `cache` and `claims` are absent while the cache server does not answer.

### Graph reads (owners, waves, heap)

- Owner pool progress: `pool.json` (section 1) carries `claimed`, `published`, `stored`, `held`, `failed`, `wave`, `rssMegabytes`.
- Generation (owners, build kind, verification): `FW/.../🚢️fleet-status/🟦️.ts` reader `readFleetGenerationV1` (~line 116-127), reads `graph/generations/<pointer>` and manifests.
- Mapping queue: `slice-memory.json` (above).
- Heap census: **not located** in this pass.

### Readiness stages and attempts

- `ready.json` `stage`, `stageIndex`/`stageCount`, `attempts`, `exit`, `refusal`, `nextAttemptMs`; per-run `attempts` in `ready-timings.json`. Stage list in the readiness schema `Stages` (native, repo-mcp, dependencies, …).

### Nx run phase lines (`[phase]`)

- `FW/📦️artifacts/🏗️native-build/🧮️phase-ledger/🟦️.ts:70-74`: `[phase] task=… name=… seconds=… count=… [bytes=…]` per phase and `[phase] task=… total=… phases=… other=…`. Cargo timings: `readCargoTimingsV1` (line 118). The module imports `appendFileSync`, but I did not confirm the target file.

### CLI status commands

| Command | Implementation | Output / cost |
|---|---|---|
| `repo fleet status [--json] [--limit=n]` | `FW/📚️library/🏗️build/🚢️fleet-status/⌨️command/🟦️.ts`, readers `📥️readers/🟦️.ts` | reads all sources in parallel, no lease, no graph, writes nothing; `sources[]` carries per-source `milliseconds`; report `{summary, memory, runs, queues, bazel, storage, sealed, generation, scheduler, fleet, workers, cas, retention, offload, findings, sources}` |
| `repo remote-cache status` | `FW/.../🌐️remote-cache/⌨️command/🟦️.ts` | HTTP GET `/status` with fleet credential |
| `repo remote-execution status [--join]` | `FW/.../🌐️remote-execution/⌨️command/🟦️.ts`, `🚢️status/🟦️.ts` | GET `/status` with 5 s timeout |
| `repo capacity status [--json]` | `HC/⌨️command/🟦️.ts` | `{...snapshot, warden, sessions, estimates}` (`readHostEstimateStoreV1`) |
| `repo doctor [--json] [--fix] [--strict] [--facts] [--only=]` | `FW/.../🩺️doctor/⌨️command/🟦️.ts:8-46`, probes under `🩺️doctor/🔎️probes/` | on-demand probes; the probe list was not enumerated |

## 4. Time-series and history stores

| Store | Path | Writer | Retention | Notes |
|---|---|---|---|---|
| Ready timings | `v1/origins/<origin>/ready-timings.json` and local `.cache/f/ready/ready-timings.json` | `ready/🟦️.ts:1395-1408` | last 40 runs (`timingsKept`) | the only per-stage history; rewritten whole |
| Warden alarms | `.../warden/alarms.jsonl` | `WARDEN/🔁️loop/🟦️.ts:209` | 1 MiB, keeps the newer half | |
| Warden log | `.../warden/` log | warden | 4 MiB (`logMaximumBytes`) | |
| Cancel records | `.../warden/cancelled/` | warden | 1 h (`cancelRecordMaximumAgeMilliseconds`) | |
| Lease history | lease registry (`FW/🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts:168`, `readLeaseHistoryV1`) | leases | per waiter key; read by fleet status as `history` and `activeMilliseconds` | the only existing series inside fleet status |
| Unit counters | `.cache/f/unit-counters/` | `countUnitV1` | unbounded, monotonic, `since` | byte-per-event files |
| Offload counters | `.cache/f/offload-counters/` | offload | unbounded | |
| Pool waves | `pool-published-*.txt` in each origin | owner pool | none | about 880 per origin in `g1-m01` |
| Provenance evidence | cache server evidence file | `provenance/🟦️.ts` | not checked | |

No sqlite store and no ring buffer were found in the fleet path. The fleet dashboard therefore has no historical series yet beyond the lease history.

## Cross-platform summary

- Warden: enforces on darwin and linux, observes on win32 (`Platforms.observeOnly`). Warden writes `mode` and the state file on all platforms.
- Memory sampling: darwin (`vm_stat`, `sysctl`, `df`), linux (`/proc`, cgroup), win32 (`bun:ffi` kernel32, `os.cpus()` busy share, no swap).
- Status files: `ready.json` platform enum `linux|macos|windows`.

## Overhead notes

- Warden: one `ps` per tick (2 s calm, 0.5 s tight), swap commands every 2 s, command-line reads only for new processes.
- Memory mirror: 1 s poll, file write only on change or every 5 s.
- Ready status: write on change, every 5 s, heartbeat 30 s.
- Owner-pool status: on change, every 5 s.
- Attest: heartbeat plus spacing gate; git-state reading is the expensive part.
- `createHostAvailableBytesV1` on darwin uses blocking `spawnSync("vm_stat")`.
- Fleet status: reads per origin only `memory.json`, `ready.json`, `slice-memory.json`, plus trees and host names.

## Defects and gaps found

1. `pool.json.<pid>.tmp` orphans accumulate in each origin directory (no cleanup found). `ready.json` and `attest.json` use the same tmp-plus-rename pattern (`ready/🟦️.ts:394`; `replaceOriginFileV1` in `journal/🟦️.ts:264`); check them too.
2. `pool-published-*.txt` files are never pruned.
3. Remote cache counters are in memory and reset on restart; no persisted cache-hit series.
4. Not verified: the heap census, the `[phase]` target file, the full `capacity status` estimate schema, the `doctor` probe list, `attest` heartbeat constants, and the fleet status `history` retention.
