# O8 deadlines, liveness, scheduler worker loss, cancel CLI

Status: code landed (no restarts: site-build was active). Tests: see "Tests" (filled below).

## Root cause of the 23:05-23:20 wedge (from code + scheduler log `.cache/f/sidecars/build-scheduler.log`)
The log (UTC) shows no restart of the scheduler after 22:03:47Z but constant worker churn (worker-left/joined every 1-2 min, "cancelled N, running M" growing 2 -> 9/10 over 22:10-22:20). Defects found in `🗓️scheduler/🟦️.ts`:
1. `heartbeat()` dropped every task the worker did not list from `worker.running` immediately, even before the lease: a heartbeat sent before the poll answer reached the worker made the scheduler forget a live task. Such an operation stays EXECUTING but is in no worker's `running` set, so `drop()` (worker lost/left) never requeued it, and only the action timeout (1200 s + lease, then a second attempt) ended it. This is the "lost inside the scheduler" path.
2. `execute()` joined a new client to an operation whose cancellation was already sent to the worker (linger cancel after the previous wrapper died: the same action digest recurs across play attempts). `cancel()` leaves the operation EXECUTING until the worker acknowledges; a worker that never acknowledges (wedged kill, restarted) left the new client waiting. The 9 `cancelledExecuting` are exactly these cancel requests (counted at request time, not at acknowledgement).
3. `poll()` answered at once whenever `worker.cancel` was non-empty and stale cancel entries were never pruned: the worker spun (no `waiter`, so never dispatched to again).
4. Client side: only the overall 1200 s + 30 s wait existed; no liveness, no per-unit deadline.

## Changes
- Schema/policy: `🚀️offload/🧬️schema/🔣️.json` `policy.deadline` (factor 4, floor 60 s, cap 900 s, unknown 240 s, linked cap 3600 s / unknown 2400 s, silence 20 s, reattach 10 s); `🔣️policy.json` + `🏃️execution/🧬️schema/🌐️remote-execution.json` server `keepAliveSeconds 5`, `cancelGraceSeconds 20`, `stuckSeconds 600`; new closed definition `Cancellation`.
- `offloadDeadlineMillisecondsV1` (policy): factor x recorded cost between floor and cap; unknown -> unknown value.
- Client `🔌️client/🟦️.ts` `executeOffloadV1`: deadline counted from first EXECUTING (never restarted by a requeue); silence watchdog; on silence closes the stream and re-attaches by name (`WaitExecution`) on a new connection, repeatedly while each attach answers; silent attach -> decline `silent after attach`; NOT_FOUND -> decline `operation lost` (immediate local); deadline expiry closes the stream (server cancels remotely after linger) and declines `deadline <ms>`. Ride-through carries the remaining deadline. `onStage` fires on change only.
- `🚀️offload/🟦️.ts`: rustc passes `deadlineMilliseconds` from `readUnitCostV1(root,crate)` (linked = wasm peak > 0); `offloadCommandV1` from new optional `predictedMilliseconds` else `readUnitCostV1(root,label)` (linked when the tool declares a peak).
- Scheduler: keep-alive repeat of the operation every `keepAliveSeconds`; attach revokes a pending cancellation; `cancelGraceSeconds` frees an unacknowledged cancellation; heartbeat no longer forgets tasks before the lease and prunes stale cancel entries; poll announces each cancel once; `requeue` releases the old task; sweep requeues an EXECUTING operation whose worker no longer holds it; `abort(target)` for operators.
- Server: `POST /v1/operations/<id|digest-prefix|all-stuck>/cancel` (roles submit/admin, write/worker token).
- Worker: whole-task watchdog (timeout + grace) so a hung materialization is aborted and reported.
- CLI: `repo remote-execution cancel <operation|all-stuck> [--join <url>]` in the catalogue (EN+DE summary, example, flag text), `📖️texts` (EN+DE), `Cancellation` document.

## Rollout
Nothing was restarted. The running server (old code) sends no keep-alives: new clients then re-attach every 20 s (cheap, each attach answers at once) and still hold; the deadline works against old servers. The scheduler fixes and the cancel endpoint apply at the next scheduler restart, the worker watchdog at the next worker restart, new wrappers (new sealed generation) get the client.

## Tests
- Scheduler file: 21 pass, 1 fail on the first run. The failure was the new operator test (test bug: default action timeout 60 s requeued the operation during the simulated 105 s; fixed with `defaultActionSeconds:600` in that test). 6 new scheduler tests: keep-alive, rejoin revokes cancel, cancel grace and no poll spin, heartbeat race, worker lost fails back, operator abort; the 5 others of the 6 passed.
- NOT RUN: fixed scheduler test, `🚀️offload/🧪️tests/⏱️timeouts` (5 new: deadline vectors + Python oracle, silent stream re-attach, silent attach, lost operation, deadline, silent worker via cluster), `⌨️command/🧪️tests`. From 23:55 `repo test-file` refuses every file with "Explicit finite owned Bun file selection required": host load 75 stretches the suite budget above the 1 800 000 ms cap of the selection schema (environmental, also for untouched files).
- Open: rerun the three files when load drops; regenerate help fixtures (`⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json`, synopsis now `serve|status|workers|cancel`); `repo remote-execution cancel` not exercised against a live scheduler.
