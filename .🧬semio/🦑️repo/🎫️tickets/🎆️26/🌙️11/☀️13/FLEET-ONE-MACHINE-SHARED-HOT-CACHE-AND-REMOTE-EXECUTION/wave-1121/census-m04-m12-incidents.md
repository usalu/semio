# 🧭 Fleet census: g1-m04 (Linux) and g1-m12 (macOS) incidents

Read-only audit. No source, git, process or ticket state was changed. Store evidence lives in `/Users/Shared/semio-store/v1/origins/`. Sidecar code lives under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/` (abbreviated `READY/`). Process primitives live under `🧰️framework/🔨️modules/🏃️process/` (abbreviated `PROC/`).

## Summary

| | g1-m04 (Linux) | g1-m12 (macOS) |
|---|---|---|
| Symptom | `tools` stage running, sidecar silent since 2026-11-20T23:53:10Z | `workspace` stage SIGTERMed at 2026-11-21T20:54:40Z after 14 min adopted |
| SIGTERM sender | none (no SIGTERM in m04 tools log) | sidecar stall rule: `READY/🟦️.ts:1021` → `kill("stall")` |
| Root cause | Stage deadline and stall kill both live inside the sidecar; when the sidecar stops, nothing enforces the stage deadline or restarts it | Stall rule counts only new log lines and child CPU; it ignores in-process waits and the `phase` of the stage |

## A. g1-m04: `native-artifact-input-read` and the silent tools stage

### What the name is

- `native-artifact-input-read:N` is a progress publication, not a lock a worker holds. It is emitted by the staged-input verifier at `PROC/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🟦️.ts:140` (per chunk of a hashed read) and `:141` (every 256 quiet non-hashed files). The counter `nativeArtifactInputStepV1` is at `:158`.
- The exclusive lease that the verifier actually depends on is `native-storage:<backing identity>`, taken in `withProtectedArtifactInputJournalV1` at `:128` via `acquireFreshResourceLeaseV1`.
- The progress names are in the sidecar schema: `READY/🌲️process-tree/🧬️schema/🔣️.json:51` (`native-artifact-input-(stage|read|verification):\d+`). Counters with a changing number are new log lines, so they reset the stall clock (`READY/🌲️process-tree/🟦️.ts:121-136`).

### Why a tools stage can wait without bound

- `PROC/🔒️leases/🟦️.ts:363`: `acquireFreshResourceLeaseV1` passes `timeoutMs:0`.
- `PROC/🔒️leases/🟦️.ts:92-93` and `:317`: a `timeoutMs` of 0 never throws a timeout. Only the abort signal ends the wait.
- `PROC/🔒️leases/🟦️.ts:354`: admission patience (`admissionPatienceMs`) bounds only the queue admission, not the lease acquisition after it.
- Result: if another process holds `native-storage:<identity>` exclusively, the tools stage waits with no deadline of its own. The only ends are the abort signal from the sidecar kill or the stage deadline.

The m04 log, however, shows the counter moving (`native-artifact-input-read:4733` through `:4760`, then `native-artifact-input-funding-measured`). In that window the stage was doing real reads, not a silent wait. The older `ready-log-tools-stalled.txt` (2026-11-16) shows the bare line with no counter, which is the true stuck pattern: no new line, so the stall rule fires after 10 min.

### Deadlines that bound the tools stage (while the sidecar is alive)

- Soft cap: `READY/🧬️schema/🔣️.json:179` (`tools`, `timeoutMilliseconds` 3600000).
- Hard cap: `READY/🌲️process-tree/🧬️schema/🔣️.json:14` (`hardCapMultiplier` 4). Workspace is 8 (`:15`).
- Tools grace: `READY/⏳️tools-grace/🧬️schema/🔣️.json:148` (`baseMultiplier` 4, `extraIntervals` 1). `READY/⏳️tools-grace/🟦️.ts:86-102` allows an extra interval only while a registry counter advances, up to 5× soft cap.
- So a tools stage is bounded at 4 h, or 5 h under grace. At 6671 s (1.85 h) the m04 stage had not reached either.

### Linux-specific path

- Linux native images are loop-backed files. `PROC/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts:83-98` runs `losetup`, `mount -o discard`, `findmnt`, `umount`, `fstrim` and `losetup -d` through `options.run`. I did not find a per-command deadline there. A hung `mount` or `fstrim` would hold the whole path. This is a hypothesis; it is not shown by the m04 logs.
- Locking on Linux is `flock` via `dlopen` of libc, `PROC/🔒️leases/📁️physical/🟦️.ts:30-37` and `:96`. It is advisory and tied to the file descriptor, so a holder that is killed releases it. The `tryLock` loop is bounded (`protocol.attempts`).

### Why the worker went silent for about one day

- m04 `ready.json` (`/Users/Shared/semio-store/v1/origins/g1-m04/ready.json`): `updatedMs` 1795218790432 = 2026-11-20T23:53:10Z. `startedMs` 1795218232844 = 2026-11-20T23:43:52Z, the adoption time of pid 2818793, with `attempts` 9. The log file has the same mtime.
- The sidecar writes status every `statusHeartbeatMilliseconds` 30000 (`READY/🧬️schema/🔣️.json:25`) and checkpoints the in-flight file every `watchdogWriteMilliseconds` 10000 (`READY/🟦️.ts:865`). A 1-day silence means the sidecar loop stopped, or its output stopped reaching the store.
- Other machines were fresh at the same time (g1-m05, m11 at 2026-11-22 00:14 local). Stale machines in the store: g1-m06 (2026-11-20 21:03 local), g1-m09 (2026-11-14). So a replica-transport fault on m04 is possible too. I cannot read the m04 host, so this is not decided.
- Who restarts a silent sidecar: nothing found. The supervisor's watchdog (`READY`'s sibling `PROC`-independent `🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts:273-284`) watches only the daemon's `beat.json`, with a 30 s silence limit, not the ready sidecar. The tools stage runs in a detached wrapper (`READY/🏃️wrapper/🟦️.ts`, 25 lines) that outlives the sidecar by design. A new sidecar then adopts it (`reason: adopted`, "not restarted"). The hard cap and the stall rule both live inside the sidecar (`READY/🟦️.ts:1021-1029`), so a dead or hung sidecar leaves the stage running with no enforcement.
- The 24 h gap is therefore consistent with: sidecar hung or dead, no external restart, no out-of-process deadline.

### Fixes (A)

1. Finite deadline on the lease. In `🌐️platform/🟦️.ts:128` and the other `acquireFreshResourceLeaseV1` callers on this path, pass `timeoutMs` = remaining tools deadline instead of 0. Keep `admissionPatienceMs` for the queue.
2. Out-of-process deadline. At launch, write an absolute `deadlineEpochMs` (hard cap, plus grace when eligible) into the in-flight file. The detached wrapper `READY/🏃️wrapper/🟦️.ts` then SIGTERMs and SIGKILLs its child at that time, without the sidecar.
3. External liveness. A `ready.json` whose `updatedMs` is older than 3 × `statusHeartbeatMilliseconds` (about 90 s) should be reported by `fleet_status` and trigger a sidecar restart by the supervisor. Add a watcher to the supervisor, not only to the daemon beat.
4. Operational check, not a code change: on g1-m04, `ps -p 2818793` and the replica status of that host. I could not run this from here.

## B. g1-m12: `SIGTERM received` at 2026-11-21T20:54:40Z

### Who sent the SIGTERM

- `ready-log-workspace-stalled.txt` line header `workspace-stalled` is written by `READY/🟦️.ts:486` (`ports.mirror?.(\`${row.name}-stalled\`, ...)`) in the `stalledOut` branch. That branch runs only after the sidecar's own stall kill.
- The kill is `READY/🟦️.ts:1021`: `if(!killed && Date.now()>=armedAt && readyStalledV1(Date.now()-stallAt, treePolicy)) kill("stall")`. `kill()` at `:906-913` sets `killReason`, and `settle()` then SIGTERMs the stage tree via `readyKillTreeV1`.
- The stage's own message at `PROC/🧭️routing/📥️invocation/🏃️process/🟦️.ts:266` (`process.once("SIGTERM", terminate)`) and `:103-105` (`interrupted` notice) matches the log text.
- Ruled out:
  - Replica restart: `🛟️supervisor/🟦️.ts:289` also sends SIGTERM, but the log has no restart line and the `-stalled` mirror is written only by `:486`.
  - Lease yield: `READY/🟦️.ts:927-929` logs `yielded-reader`, which is absent.
  - Containment: `ready-log-workspace-containment.txt` is an after-stop report (31 entries, 19 `not-ours`). It reaps by registry only on `holder` (`:1016`) or on stage end, and nothing in the log shows a reap.
  - Owned execution: `PROC/🎛️owned-execution/🟦️.ts:165-167` prints "from owned execution", which is not in the log.

### The criteria (all must hold)

- `stallStopMilliseconds` = 600000 (`READY/🌲️process-tree/🧬️schema/🔣️.json:17`).
- `stallAt` moves only on: a new normalised log line (`READY/🌲️process-tree/🟦️.ts:113-136`, times and `ms`/`s` counters are stripped by `normalisePatterns`, `:23`), tree CPU ≥ 500 ms per sample (`:437-441`, `livenessCpuMilliseconds` `:21`), holder evidence (`READY/🟦️.ts:921`), or step watch. `READY/🟦️.ts:852` initialises it from the restored watchdog file.
- The log in the 20:54 attempt shows: `Waiting for the repo graph service (471 ms)` ... `(8567 ms)`, then `Loading the project graph of 440cffa5563d.` and `Planning the selected targets and what they need.`, then nothing. The `(N ms)` ticks normalise to the same line, so they are not progress.
- No holder exemption: `holderWaitPatterns` (`READY/🌲️process-tree/🧬️schema/🔣️.json:24`) match "waiting for the holder/lease", not "Waiting for the repo graph service".
- Result: about 10 min with no new line and no tree CPU kills the stage. Adoption at 20:40:13Z plus the 10 min window gives 20:54:40Z.

### Can it kill a slow but progressing graph read?

- Yes, when the progress is not a new log line and not CPU in the stage tree. The sidecar never reads in-process declarations. `PROC/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts:25-26` and `:88` make an in-process wait "healthy" for up to `waitCeilingMilliseconds` 14400000 (`PROC/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧬️schema/🔣️.json`). The sidecar's rule at `:1021` does not use that information.
- The `phase` variable (`READY/🟦️.ts:858`, reading/executing from `readingPatterns` and `executingPatterns`) is computed and used only for step overdue (`:1013-1015`). It is not used in the stall rule at `:1021`. That is the concrete gap: a `reading` phase gets the same 10-min rule as a build.
- Retries are bounded: `readyStallRestartV1` (`READY/🌲️process-tree/🟦️.ts:86-90`) restarts up to `stallRestartMaximum` 5 times with back-off, then fails as `stage-stall-exhausted`. A graph read that is silent for 10 min each attempt can never finish on a 16 GB machine.
- Current m12 attempt (`g1-m12/ready.json`, pid 58294, updated 2026-11-21T23:14:18Z) is progressing: `Reading the project graph slice (8338 s): taken 461 of 600.` That is a step line, so it passes the rule. It is allowed up to 8 h (`READY/🌲️process-tree/🧬️schema/🔣️.json:15`).

### Fixes (B)

1. In `READY/🟦️.ts:1021`, skip or extend the stall rule while `phase==="reading"` (or while the stage declares an in-process wait). Use a separate reading ceiling equal to the workspace hard cap, not 10 min.
2. Let the stage publish a heartbeat line that the tracker treats as progress only when the graph server's own counter moves. Add the graph-service wait line to `holderWaitPatterns`, or to a new `waitPatterns` list that resets `stallAt` without resetting the step clock.
3. Count the CPU of the repo graph server (`🕸️graph/🟦️.ts`, "the graph server of a machine") into the stage's liveness, or read its status file. I did not verify whether that server runs inside the stage tree; this must be checked before the fix.
4. Make `stallRestartMaximum` progress-aware, so repeated restarts of a slow graph read do not exhaust the budget.

## Open questions

- Whether the m04 sidecar process is alive, and whether replica sync from m04 is stuck. Needs a read on g1-m04.
- Whether the graph server runs in the stage's process tree (B3). Needs a process listing during a graph read.
- Whether the Linux `losetup`/`mount` calls have deadlines (A, Linux path). Needs the `options.run` implementation.
