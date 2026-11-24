# Design: Host Warden (Memory Watchdog + Admission as One Mechanism) and Unit Carving

Date 2026-11-24. Sources: `map-A` … `map-E` in this folder, live readings in `measurement.md`.

## 1. What the measurements say

- Every 16 GB Mac worker shows a Ready `workspace` stage tree peak of 5.5–10.7 GB (`ready.json` `peakMegabytes`, summed RSS). The stage contains the project-graph slice read: slice worker 3.5–5.4 GB RSS, plus farm workers (1536 MB each), plus up to two heavy-question children (2560 MB each), plus rustc units of the same stage.
- The slice plan guard on a 16 GB host has a limit of about 4.9 GB and a **ceiling of 9.8 GB (60 % of the host)**; on macOS the memory-budget check of the slice pool compares against `Infinity` (no cgroup). The graph host's 2560 MiB cap is not enforced while a read is in flight.
- Beside the stage a worker runs: the replica daemon, the pool puller (2 children × 800 MB), up to one lent remote unit (a component final can need 6.5 GB), the cache sidecar.
- Three admission systems exist (host capacity gate, native-build broker, remote-execution worker gate) plus the Ready stage claim and the in-process graph governor. **All of them only refuse or delay starts.** Nothing acts on a process that is already running: no suspend, no cancel-and-requeue, no priority order. An overshoot is discovered at exit 137, i.e. after the operating system acted.
- Per-worker swap, available memory and compressor state never leave the worker: the load report copies `pressure` and a derived `freeMegabytes` only, `ready.json` carries the stage peak only. The fleet load endpoint itself timed out in 7 of 8 one-minute samples tonight (coordinator load 53), so it cannot be the only path.

## 2. One mechanism: the host warden

One repo-owned process per machine, the **host warden**, closes the loop that admission leaves open. Admission and warden read the same host sample and the same authored policy, and form one declared ladder on the memory the host can still give out:

| step | condition (share of physical memory available) | who | effect |
|---|---|---|---|
| hold | below 15 % (`hostCapacity.pressure.availableWarnBasisPoints`, exists) or OS level warn | every admission gate (exists) | no new start |
| suspend | below `guard` (10 %), or swap worked while below `hold` | warden | `SIGSTOP` the cheapest-to-redo running subjects until the suspended megabytes cover the deficit; at least one subject keeps running |
| cancel | below `danger` (5 %), OS level critical, or a subject suspended longer than `suspendMaximumSeconds` | warden | `SIGKILL` the cheapest-to-redo subjects (suspended first) until the freed megabytes cover the deficit; the owner requeues |
| resume | above `resume` (15 %) for `calmTicks` | warden | `SIGCONT`, most expensive first, one per tick |

The warden acts well before macOS does: the system starts its own measures (jetsam, the "out of application memory" dialog with paused programs) when the compressor and swap are exhausted, which is beyond `kern.memorystatus_vm_pressure_level` 4; the warden cancels at 5 % available or at OS level 4 at the latest and suspends at 10 %.

### 2.1 Subjects: found, not registered

A subject is a process the repo started that holds memory worth acting on. Subjects are **classified from the process table**, never registered, so no launch path can forget to take part (that is what makes the guarantee structural):

- `roots`: authored patterns that mark a process as repo-started (the repo CLI entry, the sidecar entries, `semio-build-scheduler`, the workspace's Bun runtime under `.cache/b/t`).
- `kinds`: authored, ordered patterns over the command line of a process that has a root among its ancestors. Each kind declares `rank` (cost to redo, ascending = cheapest first), `actions` (`suspend`, `cancel`) and `scope` (`process`, `group` or `tree`: what a signal is sent to).

| rank | kind | pattern | actions | owner's requeue path |
|---|---|---|---|---|
| 10 | `remote-unit` | `rustc` below the remote-execution work directory | suspend, cancel | worker reports the task refused; the scheduler places it on another machine |
| 20 | `local-unit` | `rustc` below a repo root | suspend, cancel | pass restart of the native build (`UnitKilledErrorV1`, exists); finished units come from the cache |
| 30 | `farm-worker` | project-farm worker | suspend, cancel | the pool starts a new worker for the project (recycle path, exists) |
| 40 | `heavy-question` | heavy-question child | suspend, cancel | owner failure decision answers `retry` for a warden cancel |
| 50 | `test` | `bun test` below a repo root | suspend, cancel | the run fails with the warden's alarm as reason |
| 60 | `slice-worker` | slice-pool worker | suspend, cancel | `slice-recycle` (exists) |
| 90 | `stage` | Ready stage root | cancel (tree, `SIGTERM`) | supervisor back-off and restart (exists) |

Never a subject, whatever it holds: the replica daemon, sidecar launchers, the cache server, the remote-execution scheduler, the warden, and every process without a root ancestor (editors, agents). A machine-local `protect` list (workspace paths) excludes whole workspaces: the pinned play clone on the coordinator is protected.

Order inside one step: suspended before running (cancel only), then `rank` ascending, then elapsed time ascending (least work lost), then megabytes descending, then pid.

### 2.2 The decision is a pure function

`decideHostWardenV1(policy, host, subjects, previous, nowMs) → {level, actions[], hold}`; no clock, no I/O. Schema-first (`🛡️warden/🧬️schema/🔣️.json`: `Policy`, `Host`, `Subject`, `Previous`, `Action`, `Decision`, `State`, `Alarm`, `Messages` EN/DE). A language-agnostic fixture file (`🧫️fixtures/🔣️.json`, cases with expected decisions) is consumed by the TypeScript implementation and by an independent Python oracle (`🐍️.py`) written from the prose of this document, the pattern of `remote-execution/🧪️tests/🏋️unit-admission`.

### 2.3 The loop

- Sample: the existing `readHostCountersV1` / `reduceHostSampleV1` (extended by `compressedMegabytes`) and one `ps -A -o pid=,ppid=,pgid=,rss=,etime=,args=` per tick. No `footprint` call in the loop (it is the expensive one).
- Interval: `calmMilliseconds` 2000, `tightMilliseconds` 500 while the level is not calm.
- Single instance per machine: lock file with pid and start time in the host-capacity directory (`warden/lock.json`). `ensureHostWardenV1()` starts it detached when the lock is free; it is called by `withHostCapacityV1`, the native-build broker start, the remote-execution worker and the Ready supervisor tick. The warden ends itself after `idleMilliseconds` without any subject and when its own source digest changed (the next caller starts the new code): no daemon needs a restart by hand.
- Safety of `SIGSTOP`: every suspended pid (with start time) is written to `warden/state.json` before the signal; a starting warden resumes everything a dead predecessor left suspended; a subject is never suspended longer than `suspendMaximumSeconds` (then cancelled or resumed by the ladder).
- Mode: `enforce` (default) or `observe` (decides and alarms, sends no signal), read from `warden/mode` in the host-capacity directory. The coordinator runs `observe` while the play release builds.
- Cancel record: `warden/cancelled/<pid>.<start>.json` is written before the kill, so the owner can tell a warden cancel from a crash and requeue instead of failing (`readHostWardenCancelV1(pid)`).

### 2.4 Alarms and visibility

- Every action appends one line to `warden/alarms.jsonl` (bounded): time, level, action, pid, kind, owner (root label and grant label when the process sits below a grant), megabytes, the host numbers, EN and DE text. The same line goes to the standard error of the warden log.
- `warden/state.json` (written each tick, atomically): host numbers (`availableMegabytes`, `swapUsedMegabytes`, `swapOutMegabytesPerSecond`, `compressedMegabytes`, `pressure`), level, subjects by kind, counters (`suspended`, `resumed`, `cancelled`), the last alarms, the overhead of the loop (`tickMilliseconds` p50/p95, CPU share).
- Per worker: the Ready supervisor mirrors `state.json` to `<store>/v1/origins/<origin>/memory.json` (its existing store mirror); `repo fleet status` reads it in the workers reader and shows available, swap, compressed, level and interventions per worker, with findings `worker-memory-tight`, `worker-warden-intervened`, `worker-warden-absent` (EN/DE). This path does not need the scheduler.

## 3. Admission corrections that belong to the same mechanism

1. Slice plan ceiling: `ceilingBasisPoints` 6000 lets one read tree hold 9.8 GB of 16 GB. Reading the limit code afterwards showed that the read's own guard already settles and refuses against the host's reserve once the tree passes its limit (`planGuardVerdictV1`), so the ceiling was left as tuned; the overcommit comes from the combination with the other holders, which the warden covers. Not changed in this ticket.
2. macOS slice memory-budget check compares against `Infinity`: it gets the host's available memory as the limit.
3. Heavy-question children (2 lanes × 2560 MB) and farm workers hold memory outside any grant; they are subjects of the warden now (acted on), and their caps count in the read's reservation.
4. Ready `peakMegabytes` is summed RSS; the stage claim learns from it. It moves to the footprint measure the gate uses.

## 4. Carving: a declared ceiling per unit

- Schema: `unitCeilingMegabytes` per host class in the host-capacity policy (16 GB class: 4096 MB for a compile unit, the same for a read tree), and a law `unit-ceiling` that reads the learned peaks (`build-telemetry/units.jsonl`, `offload/c/*.peak`, slice peaks) and fails for every unit above the ceiling, naming the unit, its peak and the ceiling.
- Units above the ceiling today (from `map-E`): component finals `cad` 6.5 GB, `puzzle` 3.3–6.1 GB; the graph read tree up to 10.7 GB.
- Reduction at the root, per unit, in this order: (1) backend parallelism and codegen units of the final (the wrapper already serialises a one-job final), (2) per-unit profile settings that bound memory without changing semantics (`codegen-units`, `lto`, `debuginfo` of the wasm final), (3) splitting the component (the final links every artifact of a plugin: split by artifact family), (4) for the read: streaming owner facts instead of the in-memory memo (entry-bounded only today), and the slice ceiling of §3.

## 5. Proof

- Fixture cases + Python oracle for the decision; a mounted test that starts real child processes (a memory hog under a simulated host) and checks suspend, resume, cancel and the alarm.
- Overhead: tick duration and CPU share of the warden measured on the coordinator and reported by every worker in `memory.json`.
- Worker run: `memory.json` series of the workers before and after, interventions, zero OS pauses (no vanished stage, no jetsam report).

## 6. Rollout as done

`Policy.defaultMode` went out as `observe` first (every worker decided and published without signalling for 12 minutes: classification and overhead checked on real workers), then `enforce`. The coordinator keeps a machine-local `mode` = `observe` and protects the pinned play clone. Items 2 and 4 of §3 are not done (see `results.md` §5).
