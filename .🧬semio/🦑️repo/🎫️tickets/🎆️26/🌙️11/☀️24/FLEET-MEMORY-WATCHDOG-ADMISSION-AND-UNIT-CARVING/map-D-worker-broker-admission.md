# Map D: Memory Admission For Compile Units, Local And Offloaded

Read-only audit. No build, no test run, no git write, no source edit. Only this file was written.

## Conventions

All paths are relative to the repo root `/Users/ueli/Documents/semio`. Line numbers are from the files as they stood at the time of reading. Shorthands used below:

| Shorthand | Expands to |
|---|---|
| `NB` | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build` |
| `ADM` | `NB/🎟️admission` (the local broker, `🦀️.rs` and `🟦️.ts`) |
| `CP` | `NB/🧩️component-pass` (the native build driver, `🟦️.ts`, and `🧵️job-slots/🟦️.ts`) |
| `BLD` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build` |
| `RE` | `BLD/🌐️remote-execution` (worker, scheduler, load, offload) |
| `LAUNCH` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` (second broker user) |

Note: the `🦀️.rs` file is the only Rust source in this area. Everything else is TypeScript with JSON schemas and Python oracles.

---

## 0. Short Answer Overview

- The local path is a real admission broker. Every rustc unit asks a TypeScript broker for memory before it starts (`ADM/🦀️.rs`, `ADM/🟦️.ts`). Prediction is learned per unit key from a JSONL ledger, with a cold-start model for units without history.
- The offloaded path is admitted twice: the scheduler places a unit on a worker whose reported free memory covers the predicted peak (`RE/🗓️scheduler/⚖️placement/🟦️.ts`), then the worker admits it again against the host capacity gate (`RE/🌡️load/🟦️.ts`).
- Per-task memory on a worker is sampled once per second through `ps` process-group RSS (`RE/👷️worker/📏️task-memory/🟦️.ts`).
- Nothing in the codebase suspends, deprioritises, or cancels a running compile unit because of host memory. The only memory-driven action is refusing to START new units. Killed units (exit 137) are only observed after the OS killed them.
- A worker OOM-kill does not requeue on another worker. The offload client falls back to a local compile after teaching the placement a larger peak (`RE/🚀️offload/🟦️.ts:155-156`).

---

## 1. Local Rustc Unit: Chain, Prediction, Ledger, Measurement

### 1.1 The chain: cargo to rustc wrapper to broker

1. The driver first builds the scheduler binary itself with `RUSTC_WRAPPER` blanked: `ADM/🟦️.ts:464` sets `RUSTC_WRAPPER: "", CARGO_BUILD_RUSTC_WRAPPER: ""` and runs `cargo build --locked --release --bins` on `ADM/📦️packages/🦀️rust/Cargo.toml` (`schedulerCargoSelectionV1`, `ADM/🟦️.ts:448-450`).
2. The native build driver sets the wrapper for every job: `CP/🟦️.ts` in `jobEnvironmentOf` (around line 523) contains `RUSTC_WRAPPER: options.scheduler!`. It then calls `compilerWrapperEnvironmentV1(options.scheduler!, ...)`, which, when a unit cache is named, makes Cargo run the cache wrapper as `RUSTC_WRAPPER` with `inner: [scheduler, RUSTC]` (`ADM/🟦️.ts:425-429`).
3. So the chain is: cargo calls `RUSTC_WRAPPER` (the unit-cache wrapper, which serves a hit from cache or falls through), then `semio-build-scheduler` (`ADM/🦀️.rs`, `fn main`, line 564), then the real rustc.
4. The broker endpoint and pass id reach the wrapper as env vars. The driver sets them in `CP/🟦️.ts:668`: `late:{[ADMISSION_POLICY_V1.brokerEnvironment]: broker.endpoint, [ADMISSION_POLICY_V1.passEnvironment]: job.id}`. Names come from the schema: `SEMIO_BUILD_BROKER` and `SEMIO_BUILD_PASS`.
5. The second broker user is the cargo launcher, `LAUNCH:240`. It starts its own broker over the same ledger, with `physicalBytes` taken from the launcher's budget, and registers a pass with `broker.registerPass({id: policy.pass, profile, target})` (`LAUNCH:244`).
6. Wrapper behaviour in `ADM/🦀️.rs` `main` (564-584):
   - `expand_admission_arguments` (41) reads one level of `@argfile`.
   - `parse_unit` (54) gets crate name, crate types, target, out dir, externs, source, and `CARGO_PKG_NAME`, `CARGO_PKG_VERSION`, `CARGO_PRIMARY_PACKAGE`, `OUT_DIR`.
   - Probes (`-vV`, `--print`, stdin `-`) return `None` and run unchanged (line 65; test `refuses_probes`, line 681).
   - Line 575: `match (unit, env SEMIO_BUILD_BROKER, env SEMIO_BUILD_PASS)`. Without all three, `passthrough` runs rustc unchanged (line 580). Nothing is admitted or measured in that case.
7. `scheduled()` (`ADM/🦀️.rs:515-561`) does the admission:
   - connects to the broker (516) and sends an `admit` request with the unit identity and the wrapper pid (517; `admit_request`, 124-132).
   - `await_admission` (509-512) loops on `broker.verdict()`. The docstring (505-508) says the wrapper KEEPS its jobserver token while it waits, so Cargo cannot hand it to another unit that can never reacquire it.
   - spawns rustc with stdin/stdout inherited and stderr piped (520), so that it can detect the `emit=metadata` artifact message for rmeta timing (530).

### 1.2 How memory is predicted (`ADM/🟦️.ts`)

- Unit identity key (line 87-89): `${package}@${crate}|${target}|${profile}|${crateTypes sorted}`. The ledger is keyed by this, not by crate name alone.
- History read per key: `predict()` (337-342) uses `options.history.get(key)`.
- The predictor is `predictPeakBytesV1` (132-137):
  - with history: `max(ceil(headroom × quantileR7(last window peaks, quantile)), floorBytes)`.
  - no history and not a component final: `max(ceil(coldBaseBytes + coldBytesPerKloc × kloc), floorBytes)`. `kloc` comes from `sourceKlocV1` (259-275), which counts newline bytes in every `.rs` file under the crate root directory, skipping `target`, `node_modules`, `dist`, `.git`.
  - no history and a component final: `componentFinalBytes` (6 GiB) until `componentColdMinimumRows` (3) component-final rows exist, then `max(ceil(headroom × median(component peaks of this profile)), floorBytes)`.
- Policy values (from `ADM/🧬️schema/🔣️.json`, `definitions.Policy.const`):
  - `headroom` 1.15, `quantile` 0.95, `window` 8.
  - `floorBytes` 268435456 (256 MiB).
  - `coldBaseBytes` 644245094, `coldBytesPerKloc` 41943040 (40 MiB per 1000 lines).
  - `componentFinalBytes` 6442450944 (6 GiB), `componentColdQuantile` 0.5, `componentColdMinimumRows` 3.
  - `maximumReserveBytes` 6442450944, `reserveBasisPoints` 2500 (25 %).
  - `swapOutBytesPerSecond` 67108864 (64 MiB/s), `resumeFraction` 0.9, `nonRustcWindowMs` 10000.
- Expected wall time (used for backfilling order): median of recent walls (`expectedWallMsV1`, 140-142).
- Waiter order (145-147): longest recorded `remainingMs` first, then largest predicted peak, then arrival.
- Admission law `admitWaitersV1` (153-176): running units reserve their predicted peak. The first waiter that does not fit gets a reservation with a shadow time (EASY backfilling). Later waiters start only if they end before the shadow time or fit in the spare memory beside it. With nothing running, the first waiter always starts, so a wrong prediction cannot deadlock the build.
- Budget (broker, line 313-314): `budget = physicalBytes − reserve − max(0, …nonRustc over the last 10 s)`, where `reserve = admissionReserveBytesV1(physicalBytes)` (34-37: `min(6 GiB, 25 % of the value)`).
- Running entries count `max(predictedBytes, chargedBytes)` (317). `chargedBytes` is the live pressure-accounted figure (see 1.4).

### 1.3 The ledger: file, schema, write-back

- Location: `unitHistoryPathV1(cacheRoot)` (`ADM/🟦️.ts:192-194`) = `<cache>/build-telemetry/units.jsonl`. Namespace `semio.build-telemetry/v1`, directory `build-telemetry`, file `units.jsonl` (schema policy). The store is never cleared by fresh builds (comment at line 191).
- Schema of a line, `UnitObservationV1` (TS type at line 62, validated against `ADM/🧬️schema/🔣️.json` `definitions.ObservationV1` via `admitDefinition`):
  `{ v: 1, key, peakRssBytes, outputBytes, wallMs, cpuMs, remainingMs, atMs }`.
- Read: `readUnitHistoryV1` (205-209) keeps only newline-terminated lines and compacts with `compactUnitHistoryV1` (197-202), which keeps the newest 8 rows per key.
- Append: `appendUnitHistoryV1` (231-254). Under an exclusive lease it cuts an unterminated tail left by a killed writer, appends whole lines, and calls `sync`.
- Compact-and-rewrite: `recordUnitHistoryV1` (212-228) writes a temporary file and renames it. It is run at the start of each build (`CP/🟦️.ts` around line 540, `historyWrites = recordUnitHistoryV1(historyPath, [], historyOptions)`).
- Write-back after a unit ends: the broker's `onCompleted` callback is `recordCompletedUnit` in `CP/🟦️.ts` (about lines 541-546). It computes:
  - `unitOutputBytesV1` (`ADM/🟦️.ts:442-445`): sum of the sizes of files in the output directory whose names start with the artifact stem (`<stem>.` or `lib<stem>.`).
  - `unitObservationV1` (432-434): returns a row only for `exit === 0` and not a cache hit, with `peakRssBytes`, `wallMs = endMs − startMs`, `cpuMs = userMs + sysMs`, `remainingMs` from the critical path.
  - the row is appended under the lease. A cache-served unit is recorded as `cached` and is not an observation (`cache: {key, recorded}`, 357).
- Critical-path (`remainingMs`) is computed in `CP` from the unit graph and fed to the observation. This is the ordering key for waiters.

### 1.4 How the peak is measured during and after the run

- While running: the wrapper samples its rustc child once per second (`ADM/🦀️.rs:539-552`). The sampler loops over 10 × 100 ms sleeps and sends `{"op":"rss","bytes":…,"chargedBytes":…}` each second (line 544).
  - macOS (`ProcessProbe::memory`, 256-264): `proc_pid_rusage` flavor 2. `bytes = resident_size`, `chargedBytes = phys_footprint`.
  - Linux (265-270): `/proc/<pid>/statm` resident pages times page size. `chargedBytes` equals resident here, so no pressure charge beyond RSS.
  - Windows (271-275): `K32GetProcessMemoryInfo` working set.
- Broker side: `rss` handler (`ADM/🟦️.ts:388-392`) stores `entry.rssBytes` and `entry.chargedBytes`, then `schedule()` re-decides. A measured overshoot of the budget, or swap-out above 64 MiB/s, sets `frozen` (`admissionFrozenV1` 185-189). A frozen broker starts no new units while anything runs (161). Running units are NOT stopped.
- After the run: `reap()` (`ADM/🦀️.rs:280-305`) calls `wait4` and reads `ru_maxrss`. Linux value is scaled by 1024. The exit status is `128 + signal` when killed by a signal (287-288).
- The `done` message (559) carries `peakRssBytes`, user and system ms, start and end ms, rmeta offset. The broker stores it as the unit's peak (`ADM/🟦️.ts:399`), and `recordCompletedUnit` writes it back to the ledger.

### 1.5 Thread bounding and the cdylib final (also in the wrapper)

- `bounded_threads` (`ADM/🦀️.rs:372`) rewrites `-Z threads=0` (or larger) to the admitted parallelism (`SEMIO_COMPILER_PARALLELISM`, read at line 471). Comment at 370 documents the measured peak: 1 thread 2.9 GB, 4 threads 5.4 GB, 10 threads 5.2 GB on a 2.7 MB crate.
- `frontend_tokens` (456-458) gives a `cdylib` final one thread regardless of grant. Comment at 454 cites a 1438-unit demonstrator final measured at 4 to 5.9 GB resident that was killed by memory pressure.
- `serial_backend` (462-468) adds `-Z no-parallel-backend` only when the grant is exactly 1 and the unit is a cdylib final.

### 1.6 Issues in the local path (verify before relying)

- Possible double reserve. `CP/🟦️.ts:551` passes `physicalBytes: options.physicalBytes ?? admissionMemoryShareV1(admissionMemoryBytesV1(), tokens, availableParallelism())`. `admissionMemoryShareV1` (`ADM/🟦️.ts:40-43`) already includes a reserve. The broker then subtracts a second reserve from that value (`ADM/🟦️.ts:313`). The same call also passes `reserveBytes: admissionReserveBytesV1(...)`, which is NOT a field of the `startAdmissionBrokerV1` options type (`ADM/🟦️.ts:309`) and is silently ignored. Verify the intended budget.
- Two learning stores for peaks: `units.jsonl` (per unit key, p95 of last 8, admission) and the offload store `<cacheRoot>/offload/c/<crate>.peak` (per crate, grows at once and shrinks halfway, placement; see 2.5). They are not reconciled.
- `ProcessProbe::memory` on Linux reports only RSS. Pressure charge equals RSS there, so swap-heavy units are under-charged.

---

## 2. Offloaded Unit On A Worker

### 2.1 Offload decision (client side, before any worker sees the unit)

- `RE/🚀️offload/🚦️admission/🟦️.ts` `offloadGateV1` (last function): order of signals, each asked only if the cheaper ones did not decide:
  1. expensive unit (`finals` rule) → only a worker with room (`workerFits`, waits up to `finals.waitMilliseconds` = 30000 ms);
  2. free local compile slot → compile here;
  3. round-trip test (`worth`: recorded local cost vs `roundTripMilliseconds` 8000 ms);
  4. door refusal for the whole build (`SEMIO_OFFLOAD_PRESSURE=door`);
  5. slot queue ≥ `queueThresholdMilliseconds` (1500 ms);
  6. host capacity `wait`;
  7. the build's own admission broker `wait` (`broker-wait`), then `headroom`.
- `localUnitAdmissionV1` (`RE/🚀️offload/🚦️admission/🟦️.ts`, near the top) asks the host capacity decision for one `rustc` job of `unitMegabytes` (768 MB).

### 2.2 How the worker admits an offloaded unit

Two gates, in order:

1. Placement (scheduler, `RE/🗓️scheduler/⚖️placement/🟦️.ts:57-61`), `placementHasHeadroomV1`:
   `entry.report.pressure !== "critical" && entry.report.headroom && (peakMegabytes <= 0 || entry.report.freeMegabytes − (worker.committedMegabytes ?? 0) >= peakMegabytes)`.
   So the scheduler compares the unit's predicted peak against the worker's reported free memory minus what it already committed.
2. Worker admission at task arrival (`RE/👷️worker/🟦️.ts:459`): `options.admit(running.size, {stage:"task", megabytes: answer.task.peakMegabytes ?? 0, held})`. The default admit is `evaluateRemoteExecutionAdmissionV1` in `RE/🌡️load/🟦️.ts`:
   - `running >= lanes` → refuse `slots`. `lanes = min(concurrency, stageConcurrency)` while the Ready stage is fresh and not waiting (see 2.6).
   - `remoteExecutionPressureGateV1`: `critical` refuses everything; `warn` refuses units above `warnMaximumMegabytes` (1024 MB) or while swap-out is at or above the active share; otherwise `relaxed`.
   - host capacity: one `remote-execution` waiter of `fixedMegabytes = megabytes` in the lent class, against the grants of local builds, the Ready stage, and `held` (peak of each running remote unit). Admitted only if `decision.verdict === "admit"`.
3. Refusal path: the worker sends `outcome: "refused"` with `status.code 8` and the refusal reason (`RE/👷️worker/🟦️.ts:460-463`). The scheduler's `report()` (`RE/🗓️scheduler/🟦️.ts:315-323`) sets a cooldown, records `refused`, and:
   - if another machine has idle headroom (`placementIdleElsewhereV1`): requeue with the worker added to `tried`;
   - else: `DECLINED` (offloadRejected), and the client compiles locally.

### 2.3 What the worker publishes

- Load report every `intervalSeconds`: `runRemoteExecutionReporterV1` (`RE/🌡️load/🟦️.ts`, end of file) posts `buildRemoteExecutionLoadReportV1` to `/v1/fleet/load`. `freeMegabytes` is `spareMegabytes`, the memory a unit may take above the floor, computed with the smallest unit (`smallestMegabytes` 256). `headroom` is the verdict for that smallest unit.
- Heartbeat usage (`RE/👷️worker/🟦️.ts:419-422`): every heartbeat sends `usage: meter.usage()` for running tasks. The scheduler keeps it in `worker.usage` (`RE/🗓️scheduler/🟦️.ts:~244`).

### 2.4 How the sidecar samples a task's memory while it runs

File: `RE/👷️worker/📏️task-memory/🟦️.ts` (42 lines).

- Table read (12-15): `ps -A -o pgid= -o rss=`, 5 s timeout, 64 MB buffer. Not available on Windows (returns `undefined`, so nothing is sampled).
- Grouping (3-10): sums RSS kilobytes of every line whose process-group id is one of the attached task pids, rounds each group up to MB.
- Interval (`RE/👷️worker/🟦️.ts:390`): `createRemoteExecutionMeterV1(remoteExecutionProcessTableV1, 1000)`. One `ps` read per second for all attached tasks, and only while at least one task is attached (`attach` starts the timer; `finish` stops it when empty).
- Per task it keeps `current` and `peak`. `peak` is returned by `finish()` and sent as auxiliary metadata of the result (`RE/👷️worker/🟦️.ts` around 327 and 341).
- Not verified: the meter assumes the action's pid is also its process group id. I did not confirm that `executeRemoteExecutionActionV1` spawns the action detached.

### 2.5 What happens when the prediction is exceeded or the host is under pressure

- Prediction exceeded: nothing stops the unit. The meter only measures. The worker keeps running it and reports the real peak.
- Host pressure: no running unit is affected. The gate refuses new units (`pressure-warn`, `pressure-critical`) and the load report marks `refusing`.
- Killed by the OS (SIGKILL, exit 137 = `peak.killedExit`, `RE/🚀️offload/📐️policy/🟦️.ts:113-115`):
  - The worker reports `failed` with exit 137. The scheduler retries only on status codes 10 and 14 or the offload policy refusal (`RE/🗓️scheduler/🟦️.ts:328-329`). 137 is not retried, so the operation finishes as failed.
  - The offload client classifies it as `infrastructure` (`RE/🚀️offload/🟦️.ts:153-156`), writes a learned peak of `killedFactor` (2) × placed, with `force` (`writeUnitPeakV1(root, crate, taught, true)`), and then calls `local("infrastructure", …)`. The name suggests the unit is compiled locally. I did not read `local()` to confirm.
  - Net effect: on a worker OOM the unit is NOT retried on another worker. It goes back to the local compile.
  - Locally, the OS OOM-kill (SIGKILL) of a rustc unit is classified by `killedUnitsV1` as transient (`CP/🩺️failure-diagnosis/🟦️.ts:23-33`), and the driver then throws `UnitKilledErrorV1` (`CP/🟦️.ts:1269`) so the whole pass restarts (exit 75 by the churn protocol).

### 2.6 "A worker whose Ready stage runs lends at most one remote slot"

- Policy (`RE/🌡️load/🧬️schema/🔣️.json`, `definitions.Policy.const`): `stageConcurrency: 1`. Its description: "While the readiness stage of the machine runs (a fresh claim that does not wait) the machine runs at most `stageConcurrency` remote units at once … measured on a 16 GB worker with 3 to 6 remote units beside its stage: 9 s instead of 5 ms".
- Code (`RE/🌡️load/🟦️.ts`, `evaluateRemoteExecutionAdmissionV1`):
  `const lanes = input.view.stage !== undefined && !input.view.stage.waiting ? Math.min(input.concurrency, REMOTE_EXECUTION_ADMISSION_POLICY_V1.stageConcurrency) : input.concurrency;`
  then `if (input.running >= lanes) return {admit:false, reason:"slots"}`.
- The stage claim comes from `RE/🌡️load/🪑️stage-claim/🟦️.ts` (`readReadyStageClaimV1`), read in `readRemoteExecutionHostViewV1`. "Waiting" means the stage is blocked on a lookup. While it waits, `lanes` is not capped.

---

## 3. Component Finals (wasm cdylib link and codegen, 325 to 6550 MB)

### 3.1 Definition

- A component final is a non-host `cdylib` unit: `componentFinalKeyV1` (`ADM/🟦️.ts:110-113`) checks target ≠ host, profile match, and `cdylib` in crate types.
- Offloadable targets: `wasm32-` prefix (`CP/🧵️job-slots/🧬️schema/🔣️.json`, `offloadableTargetPrefixes`). Offload wasm policy (`RE/🚀️offload/🧬️schema/🔣️.json` `wasm`): targets `wasm32-unknown-unknown`, `wasm32-wasip2`, crate type `cdylib`.

### 3.2 Where the step is defined

- Per-unit: a rustc `cdylib` compile through the wrapper (`ADM/🦀️.rs`, section 1.5).
- Pass level: `CP/🧵️job-slots/🟦️.ts` `runPassesInJobSlotsV1` (line 35) runs passes in lanes; `CP/🟦️.ts` `runPass` launches them (roughly lines 1190-1260).

### 3.3 Memory charge: one charge per final, and the single-charge rules

- Local reservation (`CP/🧵️job-slots/🟦️.ts`, `componentPassPeakMegabytesV1`, 163-173): a pass holds the largest `units` finals' predicted peaks at once, where `units` is the local token grant. Each prediction is `predictPeakBytesV1` on that crate's recorded peaks, or the unmeasured quantile of the profile's recorded finals (`unmeasuredQuantile` 0.9, `componentFinalBytes` while fewer than 3 are recorded).
- Local admission (`admitLocalComponentJobV1`, line 136): `free = available + min(owned, reserved) − floor − reserved`. Admit when `peak ≤ free`. Reasons: `local-first`, `local-fits`, `local-cap`, `local-unknown`, `local-pressure`, `local-memory`.
  - `owned` = memory the compilers of this build already hold, added back because reservations are charged at their whole peak.
  - Floor (`CP/🟦️.ts:1196`): `ceil(admissionReserveBytesV1(admissionMemoryBytesV1()) / MiB)`, i.e. 25 % of memory, capped at 6 GiB.
- Single charge on the worker: `placementChargeMegabytesV1` (`RE/🗓️scheduler/⚖️placement/🟦️.ts:64-69`) charges a running remote action `declared − measured`, with a ramp when nothing was measured yet. This is the figure the oracle `RE/🧪️tests/🧮️single-charge/🐍️.py` checks.
- Single charge at rustc level: each rustc invocation is admitted once, with one predicted peak (1.2). A `cdylib` final gets one front-end thread (1.5).
- Cargo tokens: the scheduler build is charged as exact parallelism 1 (`CP/🟦️.ts:517-518` via `withCompilerCapacityConsumptionV1`). The host compile units are charged with `componentCompileUnitsV1` (`CP/🟦️.ts:446-449`, `minimum: 1`).

### 3.4 Predicted peaks of finals

- Local: `componentFinalBytes` = 6144 MiB while fewer than 3 finals are recorded. After that, `1.15 × median`, floor 256 MiB.
- Offload placement (`RE/🚀️offload/🧬️schema/🔣️.json`): `wasm.peakMegabytes` = 6144 (used when no learned peak exists and no family estimate). Family estimate: the p90 of the `_component`-suffixed crates' learned peaks, used once at least 3 are measured. Margin 25 %, min 64 MB, max 16384 MB, unknown default 2048 MB.
- The 6550 MB figure appears in `ADM/🧪️tests/🟦️.ts:47` as `semio_hub_cad_component` in a fixture. I could not find a literal 325 MB anywhere in the code I searched. The lower bound of the range in the question is not a constant in these files.

### 3.5 How many run at once

- Local (`CP/🧵️job-slots/🧬️schema/🔣️.json`): `localJobsMaximum` 3, `componentJobsMaximum` 24, `reserveMegabytes` 2048, `componentJobResidueMegabytes` 256, `pressureWarnDivisor` 2, `localJobsPollMilliseconds` 5000, `localJobsProgressMilliseconds` 60000.
  - `planLocalComponentJobsV1` (`CP/🧵️job-slots/🟦️.ts:149`) sizes the lane from memory alone (longest prefix of peaks that fits above the floor).
  - `planComponentJobsV1` (line 99) combines the local grant and the fleet's idle worker slots (cap 24, memory residue, pressure).
  - `runPassesInJobSlotsV1` (line 35): the first pass always holds the local token. Light jobs hold no token (their units go to workers). Extra local jobs hold a memory reservation instead of a token.
- Fleet (`RE/🚀️offload/🧬️schema/🔣️.json`): `fanOutJobs` 12 per build.
- Per worker: `concurrency` slots, with the Ready-stage cap of 1 (2.6).

---

## 4. Does Anything Suspend, Deprioritise, Or Cancel And Requeue For Host Memory?

Answer: NO, for host memory, today.

Evidence:

- `SIGSTOP` / `SIGCONT` / `SIGTSTP` appear only in test code (`🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🛟️custodian/🧪️tests/🟦️.ts:44-46`, which stops a parent to simulate a hang) and in unrelated packages (TUI pty, plugin transport). No production code in the admission, native-build, scheduler, worker, or offload paths sends them.
- Priority hooks exist but are static per service: `RE/🛰️sidecar/🪜️priority/🟦️.ts` sets nice, ionice, and `chrt --idle` for sidecar services (lines 33-36), not per unit and not in response to memory.
- The only per-unit kill for memory is a job-limit kill (`BLD/🧱️job-limits/🟦️.ts:69-83`, `runLimitedJobV1`, `process.kill(-pid, "SIGKILL")` on breach). Its only runtime caller I found is the scheduler's `run -- <command>` (`BLD/🗓️scheduler/⌨️command/🟦️.ts:37-38`). No rustc unit path uses it.

Existing kill and cancel paths (none triggered by memory):

| Where | What ends a unit | Trigger |
|---|---|---|
| `BLD/🧱️job-limits/🟦️.ts:72`, `:83` | `SIGKILL` process group | memory breach of a scheduled job |
| `RE/👷️worker/🟦️.ts` `finishTask` | `controller.abort()` | task timeout + cancel grace |
| `RE/👷️worker/🟦️.ts:422` (heartbeat) | abort of cancelled ids | scheduler `cancel` list |
| `RE/🛰️sidecar/🚦️ready/🟦️.ts:968`, `:1089` | `kill("timeout"|"stall"|"cancel")` | Ready stage timing, stall, cancel file |
| `RE/🗓️scheduler/🟦️.ts:363-374` | operator cancel | operator request |
| `ADM/🟦️.ts:420` | `close()` destroys sockets | broker shutdown |

Requeue paths in the scheduler (`RE/🗓️scheduler/🟦️.ts`):

- `requeue()` (249-258): counts failures, gives up after `maximumAttempts`, otherwise re-queues at the front.
- lease expiry (301): "Worker no longer runs the task", requeued with the worker in `tried`.
- worker lost or left (`drop`, 221): running tasks requeued.
- refusal (315-322): requeue to another machine, else declined.
- result codes 10 and 14, or the offload policy refusal (328-329): requeue with the worker in `tried`.
- worker-reported `cancelled` (325): requeue, unless the operator cancelled.

Natural hook points for a memory-driven suspend or cancel (none exist today):

1. Local, pid known at spawn: `ADM/🦀️.rs:520` (`Command::spawn`, child pid from `child.id()`), `ProcessProbe::of` (243). The `rss` sampler loop (539-552) already runs once per second with the pid and the live charge. A SIGSTOP/SIGCONT could be sent from there on a broker verdict. The broker's `schedule()` (`ADM/🟦️.ts:318-329`) and the `rss` handler (388-392) already see `frozen` and the charged total, so a "pause this entry" verdict would fit the existing protocol line format (`VerdictV1`).
2. Local, cancellation already exists for the cargo tree: `terminateOwnedChildTree` (`ADM/🟦️.ts:465`) and the lease-deadline cancel (`ADM/🟦️.ts:466-467`). A memory-driven cancel of one rustc would need a new per-child kill, then a requeue through the pass retry (`UnitKilledErrorV1`, `CP/🟦️.ts:1269`).
3. Remote, pid known: `RE/👷️worker/📏️task-memory/🟦️.ts` `attach(task, pid)` already receives it. The worker's `controller.abort()` (task limit) is the cancel point; `task-memory` `usage()` is the signal to use.
4. Remote, requeue exists: a worker-side "refused" for a running unit would take the scheduler's existing `requeue()` (249) with `tried` set, so the unit moves to another machine with no new protocol.
5. Admission-only: `admitWaitersV1` and `admissionFrozenV1` (`ADM/🟦️.ts:185-189`) already freeze starts. A suspend verdict would need the running set, not the waiter set.

---

## 5. Language Boundary And Cross-Language Fixtures

### 5.1 Decisions in Rust

- `ADM/🦀️.rs` only: argv and argfile parsing, the jobserver-token keep, thread and back-end flags, RSS and peak sampling per platform, the `wait4` rusage read, rmeta detection, and the wire messages (hand-encoded JSON lines, `admit_request` at 124-132, `done` at 559, `rss` at 544).
- It does not read `Policy` from the schema. Its constants (for example `VALUED`, `SOCKET` names, `PIPE_BUSY_ATTEMPTS`) are hard-coded.

### 5.2 Decisions in TypeScript

- Broker and decision laws: `ADM/🟦️.ts` (`admitWaitersV1`, `admissionFrozenV1`, `predictPeakBytesV1`, `admissionMemoryShareV1`, ledger read and write).
- Scheduler receipt (binary identity and staleness): `ADM/🔖️scheduler/🟦️.ts`.
- Component-pass planning and local slots: `CP/🧵️job-slots/🟦️.ts`.
- Driver: `CP/🟦️.ts`, `LAUNCH`.
- Offload policy, gate and client: `RE/🚀️offload/*`.
- Remote scheduler, placement, load, worker, sidecar: `RE/🗓️scheduler/*`, `RE/🌡️load/*`, `RE/👷️worker/*`, `RE/🛰️sidecar/*`.

### 5.3 Shared schemas

- `ADM/🧬️schema/🔣️.json`: `Policy`, `AdmitRequestV1`, `VerdictV1`, `RssReportV1`, `DoneReportV1`, `ObservationV1`, `Receipt`, `Protocol`. Read by TypeScript (`admitDefinition`). The Rust side does not read the file; it writes the same shapes by hand.
- `RE/🌡️load/🧬️schema/🔣️.json`: `Policy` (`smallestMegabytes`, `warnMaximumMegabytes`, `stageConcurrency`), `LoadReport`.
- `RE/🚀️offload/🧬️schema/🔣️.json`: offload `Policy`.
- `CP/🧵️job-slots/🧬️schema/🔣️.json`: slot policy and `Messages` (English and German, the required bilingual catalogue).

### 5.4 Cross-language tests with an independent oracle (examples)

- `RE/🧪️tests/🏋️unit-admission/🐍️.py` with its fixture `🔣️.json` and TS driver `🟦️.ts`. The TS test calls `Bun.spawnSync(["python3", "🐍️.py", "--check"])` (line 112).
- `RE/🧪️tests/🧮️single-charge/🐍️.py`: independent Python oracle of the single memory charge (declared minus measured, RSS from a `ps` table). The TS test is `RE/🧪️tests/🧮️single-charge/🟦️.ts`.
- `RE/🗓️scheduler/⚖️placement/🧪️tests/🐍️oracle/🐍️.py`: placement oracle.
- `RE/🚀️offload/🧪️tests/🏁️finals/🐍️.py`: finals oracle, with `🔣️.json`.
- `CP/🧵️job-slots/🧪️tests/🧵️local/🐍️oracle/🐍️.py` and `🧵️plan/🐍️oracle/🐍️.py`: local-job and plan oracles with fixtures under `🧫️fixtures/`.
- `ADM/🧪️tests/🧵️frontend-threads/🐍️.py` with `🔣️.json`: drives the Rust binary and checks thread bounds against an independent Python oracle.
- `ADM/🧪️tests/🧫️fixtures/🔣️.json`: fixture for component-final peaks (`componentFinalCratePeaksV1` expects `semio_hub_cad_component` = 6550 at `ADM/🧪️tests/🟦️.ts:47`).

---

## 6. Test Files And How They Run

### 6.1 Rust

- Inline unit tests in `ADM/🦀️.rs` (lines 586-770, `#[cfg(test)]`): `retries_only_busy_pipes`, `skips_leftover_response_files`, `reads_library_unit`, `reads_response_file_unit`, `bounds_threads_across_response_boundaries`, `bounds_long_response_path_without_mutating_original`, `refuses_probes`, `waits_through_refusals`, `bounds_front_end_threads`, `grants_one_front_end_thread_to_a_cdylib_final`, `serializes_the_back_end_of_a_one_job_cdylib_final`, `bounds_front_end_threads_in_argument_files`, `escapes_json`.
- Crate: `ADM/📦️packages/🦀️rust/Cargo.toml` (with `Cargo.lock`), binary `semio-build-scheduler`. Built by `buildSchedulerV1` with `cargo build --locked --release --bins`. Run as `cargo test` in that crate. I did not run it.

### 6.2 TypeScript (run under Bun, by the repo's test entry)

Admission:
- `ADM/🧪️tests/🟦️.ts` (laws, ledger, component-final peaks, fixtures).
- `ADM/🧪️tests/♻️replay/🟦️.ts` (replays a recorded run).
- `ADM/🧪️tests/🔭️probe/🟦️.ts` (the `probe` op, headroom answer).
- `ADM/🧪️tests/🔓️tokens/🟦️.ts` (token behaviour).
- `ADM/🧪️tests/🧵️frontend-threads/🟦️.ts` (spawns the real binary with `SEMIO_BUILD_BROKER` unset, and a Python oracle).
- `ADM/🔖️scheduler/🧪️tests/🟦️.ts` (receipt).
- `NB/🧪️tests/🦀️cargo/🟦️.ts` (starts a broker for a cargo run, line 113).

Native build:
- `CP/🧪️tests/🟦️.ts` (line 855 reads the ledger back).
- `CP/🧵️job-slots/🧪️tests/🟦️.ts` (plan, local, finals sub-tests).
- `CP/🩺️failure-diagnosis/🧪️tests/🟦️.ts` (killed-unit classification).

Remote execution:
- `RE/👷️worker/🧪️tests/🟦️.ts`, `RE/👷️worker/🧵️codegen/🧪️tests/`.
- `RE/🧪️tests/🏋️unit-admission/`, `🧮️single-charge/`, `🪑️stage-claim/`, `🌐️fleet/`, `🧰️cluster/`.
- `RE/🚀️offload/🧪️tests/`: `🏁️finals/`, `🚪️gate/`, `⏱️timeouts/`, `🩺️failure/`, `🎯️determinism/`, `🔗️native-link/`, `🕸️wasm-cdylib/`.
- `RE/🗓️scheduler/🧪️tests/` and `⚖️placement/🧪️tests/`.
- `RE/📡️wire/🧪️tests/` (wire and a Python oracle).

Gaps:
- `RE/👷️worker/📏️task-memory/` has no test directory of its own. The meter is exercised only through `RE/🧪️tests/🧮️single-charge/🟦️.ts` (which imports `createRemoteExecutionMeterV1`).
- No test found for the host-memory suspend or cancel because the feature does not exist.
- No test found for the worker-side OOM to local-fallback chain as a single end-to-end path. `RE/🚀️offload/🧪️tests/🩺️failure/` may cover part of it; not read.

How they run: TypeScript tests use `Bun.spawn`/`spawnSync` and the Python oracles run with `python3`. The repo test entry is `repo` (per the project rules). I did not run any of them.

---

## 7. Findings To Act On

1. Double reserve and an ignored argument: `CP/🟦️.ts:551` passes `reserveBytes`, not in the broker options type (`ADM/🟦️.ts:309`), and the share already includes a reserve (`ADM/🟦️.ts:40-43`). Confirm the intended budget.
2. Two peak ledgers with different rules: `units.jsonl` (p95 of 8, per unit key) and `offload/c/<crate>.peak` (grow at once, shrink halfway). Placement and admission can disagree about the same crate.
3. Nothing acts on an overshoot in flight. A worker OOM is discovered only at exit 137 and falls back to a local compile (no retry on another worker, `RE/🚀️offload/🟦️.ts:155-156`).
4. Local OOM restarts the whole pass (`CP/🟦️.ts:1269`, exit 75) rather than requeueing the unit.
5. The worker sampler is RSS only on Linux and Windows has no sampling (`task-memory/🟦️.ts:12-15`). macOS reports `phys_footprint` on the local side but the worker uses `ps` RSS on all platforms.
6. The `325 MB` lower bound of the question was not found as a literal in these files.

## Not Verified

- The `local()` helper in the offload client (assumed to mean compile locally).
- Whether `executeRemoteExecutionActionV1` spawns the action in its own process group (the meter assumes it).
- The runtime behaviour of any test. Nothing was built or run.
