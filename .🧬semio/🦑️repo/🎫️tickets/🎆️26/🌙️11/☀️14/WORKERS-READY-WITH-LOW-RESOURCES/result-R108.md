# R108: pool children outlived their pool

## Why children survived
- The runner (`owner-pool/🏃️runner/🟨️.mjs`) spawned children `detached` and designed the handover on purpose: a pool that ends drains its children, which finish the owner in flight and then exit, bounded only by `drainExitMilliseconds` (10 minutes) plus the owner itself. The only parent signal was the end of the child's stdin, read by the child's own event loop, which an owner (CPU bound JavaScript) holds for minutes. A superseded or declined wave only got a polite `drain` line, nothing else.
- The old `orphans()` counted live registry pids as part of the pool's quota (`have + passed < target`), so 16 orphans filled the target and the live pool answered `children 0`, reason `yielding`. Their own load also kept `idleCores` at 0.
- The registry held only pid and a wall-clock `startedMs`, so no identity could be verified and nobody ended them.

## What changed (all under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/`)
- `🌐️owner-pool/🪦️reaper/🟦️.ts` (new): process start time per platform (`ps -o lstart=` on macOS/Linux, PowerShell `StartTime` on Windows, runner injectable), lease read/write (atomic rename), pure `ownerPoolChildVerdictV1`, `reapOwnerPoolOrphansV1`, `ownerPoolSurvivorsV1`.
- `🌐️owner-pool/🐕️watchdog/🟨️.mjs` (new) started by the child in a worker thread: ends the process (SIGKILL, `process.kill` also ends on Windows) within `orphanGraceMilliseconds` when the parent is dead or (POSIX) no longer the parent, or the lease is missing, names another pool, is older than `leaseStaleMilliseconds`, or does not list the child's wave.
- Runner: heartbeat lease `.cache/f/owner-pool-children/lease.json` (pid, start time, beat, open non-declined wave ids) every 2 s; child args carry `waveId`, `parentPid`, `lease`; `child-<n>.json` and the registry row now carry `pid`, `startTime`, `parentPid`, `parentStart`; the pool reaps orphans before the first capacity call and every look; orphans are no longer part of the quota, survivors only count as load in the `children` the gate gets (not as "own work").
- Schema: policy keys `leaseFileName`, `leaseHeartbeatMilliseconds`, `leaseStaleMilliseconds`, `childWatchMilliseconds`, `orphanGraceMilliseconds`, `reapWaitMilliseconds`, definition `Lease`.
- Farm (`🏊️project-farm`): confirmed by a new test that a worker whose read process is SIGKILLed while its loop is busy did NOT end (red: alive after 15 s); added `🐕️watchdog/🟨️.mjs`, started by `👷️worker/🟨️.mjs`, now green. The idle-loop case was already covered by `🧪️tests/🪦️orphans`.
- Tests: `owner-pool/🧪️tests/🪦️reaper/🟦️.ts` (5: verdict table, start time, real orphan killed under a busy loop, stale/other/superseded lease, real reap with stranger/guarded/own pids), `owner-pool/🧪️tests/🟦️.ts` (the old "handover never kills a working child" test replaced by two: supervisor end takes children with it within seconds; a starting supervisor ends orphans before the first measurement and its gate counts 0), `project-farm/🧪️tests/🪦️busy-orphan/🟦️.ts`; fixtures `🛌️blocking-driver.mjs`, `🪦️busy-service.mjs`. Red first for the reaper children tests and the farm test.

## Measured
- reaper 5 pass, owner-pool 30 pass, farm busy-orphan 1 pass (red before), farm orphans 2 pass, provider-closures preflight 2 pass, heavy-questions 10 pass.

## Not verified
- Windows and Linux: only macOS ran; the Windows start-time command and `kill` semantics are tested through the injected runner only.
- Pre-existing orphans (records without `startTime`) are not reaped (the coordinator stopped them by hand).
- Live effect on g1-server and the 16 GB workers (needs the next revision there).
- Process note: edits were made in the live tree during the freeze (tests need the live root), before the freeze was lifted at 02:00; the verification pass under the lock is `🧰️scripts/R108/window.sh`.
