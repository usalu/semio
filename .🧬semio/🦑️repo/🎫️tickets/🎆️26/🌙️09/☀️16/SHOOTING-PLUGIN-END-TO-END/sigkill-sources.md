# 💀 SIGKILL sources on this macOS host (read-only audit)

Repo root: `/Users/ueli/Documents/semio`. Paths below are relative to the repo root; `🧰️fw` = `🧰️framework/🛍️products/🦑️repo/🔨️modules` unless stated otherwise. Times: log timestamps are UTC (`Z`); local time is CET (UTC+1), which is what the user saw ("19:44", "20:4x").

## Result in one paragraph

Several in-repo code paths send SIGKILL directly and without a prior SIGTERM. The two strongest matches to the observed symptoms are both in the sidecar "ready" stage machinery: (1) the **registry orphan reaper** (`readyReapOrphansV1`) kills any process that holds a lease registered under this checkout and started near a stopped stage try, and it has already killed two graph-service processes (`repo service host run`, reported as "Stopped orphan 33589 (bun)" and "34627 (bun)" in `.cache/f/ready/workspace.log`); (2) the **stage stall kill** stops a stage after 10 minutes without new log output and SIGKILLs its whole tree at settle, which matches the stall timestamps 19:44:48, 20:37:04, 20:47:18 and 20:57:36 CET in `.cache/f/sidecars/build-agent.log` and kills silent cargo compiles. Both can hit foreign processes (other agents' cargo builds and graph services). No code path in the repo was found that kills by command line alone; the victim selection in the reapers is by lease-registry identity (pid + start time) or by process-tree ancestry.

No kill record exists for the **launcher** label `repository:command` in the logs I could read; the message "was killed by SIGKILL" is produced by `🧰️framework/🔨️modules/🏃️process/📪️outcome` (owned-execution outcome, `🏃️process/🎛️owned-execution/📪️outcome/🟦️.ts:26`) for any owned child that ends by signal, so the wrapper only tells you the signal, not the killer.

## Ranked suspects

### 1. Registry orphan reaper in the ready stage (most likely for the graph service)

- Code: `🧰️fw/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧹️containment/🟦️.ts`
  - Verdicts: lines 44-70 (`readyOrphanVerdictsV1`), `judge` at 56-67. Plan: 74-77 (`readyOrphanPlanV1`). Reaper: 181-205 (`readyReapOrphansV1`), which calls `readyStopV1` per orphan (inside lines 181-205).
  - `readyStopV1` (lines 125-146) kills with SIGKILL directly: default `kill` at line 129, `Promise.all(... kill(pid))` at line 141, and an unconditional group kill `process.kill(-input.group,"SIGKILL")` at line 142. There is no SIGTERM on this path.
- Callers: `🚦️ready/🟦️.ts:340` (`reapRegistry`), `:343` (registry sweep at every stage start that has no in-flight record), `:373` (holder reap: when the stage log matches "waiting for it (", "holds the dependency store since" or "waiting for (the holder|lease)" for at least `holderReapMilliseconds` = 15 s).
- Trigger: a registry entry in the lease registry (`~/Library/Caches/semio/lease-registry/<dir>/<pid>-<startMs>.json`, policy `🌲️process-tree/🧹️containment/🧬️schema/🔣️.json`) where all of these hold:
  - the process is alive with the registered start time (identity);
  - the entry's directory is under `<checkout>/.cache/`, or the command matches `^[0-9a-f]{64}(\s|$)` (every `repo` client command is registered in that form, e.g. `<hex> service host run`, `<hex> cargo --store=shared ...`, 24 cargo entries found);
  - it is not in the stage tree, has no ancestor that is in the stage or is the "person" wrapper;
  - it started between the start of a stopped try (outcome `stalled`, `stage-timeout` or `interrupted`, from `ready/ready-timings.json`) minus 2 s and the end of that try plus 60 s (`windowGraceMilliseconds`);
  - it is not within 2 h (`humanWindowMilliseconds`) after a live human wrapper started.
- Victim selection: any lease holder of this checkout that fits the window. There is no check on the owner beyond the command pattern and the directory. Other agents' cargo builds and graph services are eligible.
- Could it hit a foreign graph service or cargo build: **yes, and it did**.
  - `ready/workspace.log` line 1: "Stopped orphan 33589 (bun), started at 2026-11-16T19:37:09.973Z inside a stopped try of this worker; it was registered by a command of this checkout and no live process of the current stage is its ancestor." The text is `reaped-registry` (`🚦️ready/🌲️process-tree/🧬️schema/🔣️.json`, line 31).
  - Line 2: the same for pid 34627, started 19:38:16.303Z (20:38 CET).
  - The lease registry has the service entry for pid 33589 (`startedAtMs` 20:37:09 CET, `updatedAtMs` 20:47:12 CET, command `<hex> service host run`).
  - Pid 33589 started 5 s after the stall stop at 19:37:04Z (20:37 CET), which is inside the 60 s grace window. It was then reaped at the next stall stop (19:47:18Z, 20:47 CET).
  - The reaper uses SIGKILL with no SIGTERM first, so the service's own `record()` (`🧰️fw/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🟨️.mjs:66-72`, which writes an exit row on SIGTERM/SIGINT) never runs. This is consistent with "killed by SIGKILL, no exit row".
- Why it hits at this time: each stage stall creates a window. Processes started during or just after the stalled try and not shielded by a human wrapper become orphans. Whenever no human `human.json` is fresh, `human` is null and nothing is shielded.
- Confidence: high that this path SIGKILLs graph services and cargo holders of this checkout. Medium on the exact victim of each event (33589 is confirmed; 34627 has a registry row only under the directory filter I could not isolate).

### 2. Stage stall and timeout kill, then SIGKILL at settle (most likely for the cargo builds)

- Code: `🧰️fw/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts`
  - Kill path: `stopTree` and `kill` at lines 348-349 (SIGTERM, then SIGKILL after `graceMilliseconds` = 10 s; also `process.kill(-inflight.pid, ...)` group kill at 348).
  - Stall: line 374 (`readyStalledV1(now - stallAt)`, `stallAt` moves only when the last 60 log lines contain a new normalised line, line 373).
  - Timeout: line 377 (`readyDeadlineV1`: `timeout` when elapsed > 4 × soft cap).
  - Settle SIGKILL: line 352 (`readyStopV1` with the same SIGKILL semantics as item 1, including the group kill).
- Policy: `🚦️ready/🌲️process-tree/🧬️schema/🔣️.json`: `stallStopMilliseconds` 600000 (10 min), `hardCapMultiplier` 4. Stage soft caps in `🚦️ready/🧬️schema/🔣️.json` (`Stages`): native 7200000, repo-mcp / dependencies / tools 3600000, workspace 14400000 (`repo run workspace:setup`, see `package.json` line 148).
- Trigger: the stage prints no new normalised line for 10 minutes. A silent `cargo`/`rustc` codegen step does this.
- Victims: the stage root (pid + start time, `readyIdentityV1`), its descendants by parent pid, and survivors recorded in `tree.json` (identity within 15 s). The group kill at settle is not identity-checked (line 142 of the containment file), so a detached child that is still in the stage's process group is taken with it.
- Timeline from `sidecars/build-agent.log` (lines 9567, 9649, 9654, 9684; same events in `ready/ready-timings.json` as `workspace stalled` rows):

  | local CET | UTC | event |
  |---|---|---|
  | 19:44:48 | 18:44:48Z | "stage workspace showed nothing new for 10 min, its process tree was stopped" |
  | 20:37:04 | 19:37:04Z | same |
  | 20:47:18 | 19:47:18Z | same |
  | 20:57:36 | 19:57:36Z | same |

- Could it hit a foreign process: yes for anything in the stage tree or group, including a detached graph service started by a stage client (it becomes a recorded survivor and is SIGKILLed on a later pass). It does not reach a user's own `repo cargo` run unless that run is in the stage tree or is reaped by item 1.
- Confidence: high that it kills the workspace stage's tree at those times; medium that the killed cargo was the stage's own (not confirmed by tracing the stage's children).

### 3. Sidecar restart on code change, then abort of in-flight work

- Code: `🧰️fw/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts:32-63` (`createSidecarRestartV1`): `input.abort()` runs at once for every role except `agent` (line 58); `agent` drains and aborts after `graceMilliseconds` = 300000 (line 59-60; value in `🛰️sidecar/🧬️schema/🔣️.json` line 12).
- Abort of owned work ends in `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:121,124` → `🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts:65-67` → SIGKILL to the tree (`:25-60`, no SIGTERM).
- Sidecar supervisor kills (`🧰️fw/🤝️shared-worktree/🔁️replica/🧷️sidecars/🟦️.ts`): `end()` sends SIGTERM and SIGKILL after `graceSeconds` (lines 179-188); `claim()` sends SIGTERM and SIGKILL to the pid of an old record with a different code hash (lines 168-178), checking only that the pid is alive and the record is fresh, not the start time.
- Trigger: any edit of a file in a sidecar's import closure, quiet for 90 s, at the latest after 600 s (`createSidecarRestartDebounceV1`). The shared worktree receives edits continuously, so restarts are frequent.
- Evidence (`sidecars/build-agent.log`): "code changed, restarting" for agent 18:14:41Z, 18:16:42Z, 18:34:44Z (drain 300 s); graph 18:04:14Z, 19:02:46Z, 19:31:48Z; pool 18:08:42Z, 19:01:58Z, 19:31:37Z. The agent role is offering rustc jobs at the same time ("joining ... offering rustc for 3 target(s)", 18:14:43Z, 18:34:46Z), so an in-flight rustc build of the agent is one of the things aborted.
- Victims: all work of that sidecar on this machine, including other agents' builds.
- Consistency with the symptom: `end()` sends SIGTERM first, so a graph service stopped this way writes a `stop:` exit row. This path fits cargo builds better than the graph service.
- Confidence: medium. The repo-side kill is clear; the link to the agent's cargo jobs is inferred from the rustc offer lines.

### 4. Slice-pool reaper (token and pid based)

- Code: `🧰️fw/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧹️reap/🟨️.mjs`
  - Token regex line 12 (`SEMIO_SLICE_JOB=<pid>.<index>.<random>` in the environment). Process list 15-33 (`ps -Eww` on macOS, line 28). Victims 49-58. Reap with SIGINT, then SIGKILL after 2 s (62-69). Kill now (SIGKILL) at 71.
- Callers: `👷️slice-pool/🟨️.mjs:20-24`, sweep of stale tokens at boot (`reaper.reap({match:stale})`, line 24) and at exit (`sweepAtExit`, 22, 120).
- Trigger: end or cancellation of a load, sustained memory pressure (`👷️slice-pool/🌡️pressure/🟨️.mjs`), boot and exit sweeps.
- Victims: processes whose environment carries the load's token; processes whose token names a dead pool pid (`deadServiceRowV1`, line 76, `kill(pid,0)` only); and reported child pids, filtered only by presence in the table (line 52, no start-time identity, so a reused pid is a victim).
- Protection: only processes whose command line matches `service host run` and their descendants (lines 12, 49-57). A graph service started with a different command line, or a cargo build started from a dead pool's worker, is not protected.
- Foreign risk: yes (token inheritance across agents), but no log evidence found in `.cache/f` for today.
- Confidence: medium-low.

### 5. Dashboard daemon session kill

- Code: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🧠️supervisor/🦀️.rs`: `SIGKILL_EXIT` at line 40; stop request forces kill (570-580); kill after `KILL_AFTER` (or half for batch) and give-up after `GIVE_UP_AFTER` (650-665). The pty signal path with `killpg` / `kill(member.pid, SIGKILL)` is in `🎛️dashboard/…/🚇️pty/🦀️.rs:348-377`, under `🖱️ui/⌨️tui/🚇️pty/🦀️.rs`.
- Trigger: a human or agent stop of a dashboard session (`repo dashboard kill` verb, `NATIVE_VERBS`), or timers.
- Victims: sessions the dashboard itself started (its ptys), so a cargo run started in a dashboard session.
- Confidence: low for this symptom (no dashboard records today), but note that human devs use `repo dashboard`.

### 6. Replica boot supervisor watchdog

- Code: `🧰️fw/🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts`
  - Child self-kill when its parent is gone: line 127 (`process.kill(process.pid,"SIGKILL")` when `!alive(config.parent)`; `parent` is `process.ppid` at start, line 146).
  - Parent watchdog: lines 244-263. SIGKILL when the beat is older than 30 s ("fell silent", line 259), when the loop was blocked for `killMs` (120 s, line 261), or when not ready after `readyMs` (900 s, line 263). `recordBlock` happens only in phase `run` (lines 248-249); in other phases the kill is silent.
  - SIGTERM then grace kill at 268-269. Mirror in `🥾️entry/🟨️.mjs:136-155`; timeouts at `🥾️entry/🟦️.ts:47,87` (SIGKILL after 180 s by default).
- Victims: only the replica's own boot child.
- Evidence: `.cache/f/health.json` has `loop-blocked-restart` count 19, last at 1794710801756 (2026-11-15 03:46 CET). Nothing for today.
- Confidence: low for the graph service and for today.

### 7. Lease yield and reclaim (`terminate` mode)

- Code: `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts:103-131`: SIGTERM to the holder, then SIGKILL to the holder and its children after 15 s (lines 122-126). Gated by `SEMIO_LEASE_RECLAIM`, default `request` (schema `📈️standing/🧬️schema/🔣️.json`, `reclaim.mode`); `terminate` only on request.
- Evidence: `SEMIO_LEASE_RECLAIM` is not set in this shell or in any repo config I searched; no `reclaim.jsonl` exists under the lease registry or `.cache`.
- Confidence: ruled out for today unless the variable is set on a service's environment.

### 8. Other in-repo kill paths (lower)

- Owned execution, timeouts, deadlines, output limits: `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:104-220` (timeout line 179, progress deadline 180, output limit 146-148, SIGINT/SIGTERM handlers 164-167); capture `🏃️process/📥️capture/🟦️.ts:68-85`; tool run `🏃️process/🧰️tools/🏃️run/🟦️.ts:143`; native build cancel `🏃️process/📦️artifacts/🏗️native-build/🟦️.ts:924-930,998`; admission `🎟️admission/🟦️.ts:377`. All use the immediate SIGKILL tree termination (`🪓️termination/🟦️.ts`). The cargo progress deadline is the idle window of 1 h (`ticket cargo-progress-deadline.md`), which does not match 8-40 min. Only the child's own tree is hit, except group members with a reused pgid.
- Bun test orphans and memory ceiling: `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/🟦️.ts:82-96` (SIGKILL of descendants and of members of the watched pid's process group, via `terminateOwnedProcessTree`); ceiling `🧪️testing/🟨️bun/🧯️ceiling/🟦️.ts:41-42`; parent guard `…/👷️parent/📜️script.ts:16`. Only bun test trees. The word "bun" in "Stopped orphan 33589 (bun)" is the process name, not this reaper.
- Go CLI executor: `💻️client/⌨️cli/🧵️executor/🐹️.go:77-101` (`kill -KILL -<group>` and `kill -KILL <pid>`), called from `💻️client/⌨️cli/🧫️test-runners/🐹️.go:294-306` when the command context is cancelled. Cancellation comes from signals (`signal.NotifyContext`, `🔌️mcp/🧩️component`, `⌨️cli/🧩️component/🐹️.go:168`) or a parent deadline.
- Graph-read runner: `🧰️fw/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🕸️graph/🏃️runner/🟦️.ts:13` (SIGINT, then SIGKILL after 10 s) and `🧰️fw/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🕸️graph/🔭️read/🟦️.ts:82` (SIGKILL after 15 s). Victims are the read's own `repo show projects` children.
- Watch and consent children: `🏗️build/🏡️resident/🔭️watch/🌐️hub/🟨️.mjs:35,63`, `🔐️consent/🟨️.mjs:83`. Own children only.
- Host capacity: no SIGKILL. The orphan watch sends SIGTERM to itself when its stdout pipe breaks (`🏗️build/🌡️host-capacity/🎫️gate/👻️orphan/🟦️.ts:13`, gate call at `🎫️gate/🟦️.ts:287`). The memory pressure and free-memory checks refuse plans and exit the service with code 75 (`🏡️resident/🏛️service/🌐️host/🟨️.mjs:341-345`, `🏛️service/🟨️.mjs:492-493`), which is graceful, not SIGKILL.
- Custodian (`🏗️build/🎟️capacity/🛟️custodian/🟦️.ts:31`): on IPC disconnect it cancels its children and so SIGKILLs their trees. No production caller of `withPaidProcessCustodianV1` was found, so it is not on today's path.
- launchd: `🏡️resident/🏗️composition/🌐️host/🍎️launchd/🟨️.mjs` (label prefix `tech.semio.repo.service.`, `AbandonProcessGroup`). Provisioning runs `launchctl bootout` for fleet labels only (`🤝️shared-worktree/🗺️topology/🥾️provision/🐚️.sh:367-399`). Launchd sends SIGKILL after its exit timeout; that is OS behaviour, not repo code, and it does not target the service label.
- Service unload and reload: `…/⌨️command/🌐️host/🟨️.mjs:41` and `🔄️reload/🟨️.mjs:45` send SIGTERM only.
- Sandbox: `pkill` and `killall` are refused (`🧪️testing/🏖️sandbox/🔣️.json:143-144,526`). No sandbox cleanup kill found. Ruled out.

### 9. Not in the repo (cannot be ruled out, no evidence)

- A kernel or user SIGKILL (jetsam, OOM, Activity Monitor, `kill -9`). `~/Library/Logs/DiagnosticReports` has no jetsam report for 2026-11-16. `log show` for SIGKILL/jetsam returned nothing (may be permission-limited). The exit-log classifier in `🏡️resident/🧾️exit-log/🟨️.mjs:30-43` says "killed without an exit record ... no oom kill in its cgroup: an external kill or a crash" only when a cgroup exists; on macOS there is no cgroup, so it reports "its cgroup is gone". No `exit.log` exists under `.cache/` on this machine.

## Timeline check against `.cache/f`

- `ready/workspace.log`: lines 1-2 ("Stopped orphan 33589 ..., 34627 ...") at 20:37 and 20:38 CET (item 1).
- `sidecars/build-agent.log`: stage stalls at 19:44:48, 20:37:04, 20:47:18, 20:57:36 CET (item 2). Code-change restarts at 20:01:58 (pool), 20:02:46 (graph), 20:31:37 (pool), 20:31:48 (graph) CET (item 3).
- `ready/running.json`: workspace stage pid 43995, started 20:47:22 CET (the last stage start recorded there).
- `boot/beat.json` and `health.json`: replica pid 51497 in phase `run`, lag 118 ms at 20:56 CET; no kill records today.
- `graph-state/denied` (20:51 CET) and `graph-state/down` (20:52 CET): markers only, no kill.
- Registry (`~/Library/Caches/semio/lease-registry`): graph service entries with `updatedAtMs` 19:44:36 CET (pid 53817, started 19:15:33) and 20:25:54 CET (pid 85145, started 19:44:52), and 33589 (see item 1). Pid 53817 stopped heartbeating 12 s before the 19:44:48 stall stop, and I could not attribute that to a path in the logs.

## Not verified

- Whether the workspace stage's own `repo` children include a cargo build (item 2) or a graph service (item 1 is verified only by the registry and log text).
- Whether the agent sidecar's in-flight work includes the user's `repo cargo test --no-run` (item 3).
- Whether `repo service host run` is launched by a stage client on this machine (its ppid chain).
- The exact launcher code that prints `repository:command was killed by SIGKILL` (I found only the generic outcome text).
- The `human.json` state at each reaper run.

## Suggested checks (read-only, next step)

- `ps -axo pid,ppid,pgid,lstart,command | grep -E 'service host run|cargo'` at the next stall, to see who is parent of a graph service and cargo.
- `cat ~/Library/Caches/semio/lease-registry/*/33589-*.json` and the matching `ready-timings.json` rows, to confirm the window arithmetic.
- `grep -c "SEMIO_LEASE_RECLAIM" ~/Library/LaunchAgents/*` and the sidecar environment, to confirm reclaim is off.
