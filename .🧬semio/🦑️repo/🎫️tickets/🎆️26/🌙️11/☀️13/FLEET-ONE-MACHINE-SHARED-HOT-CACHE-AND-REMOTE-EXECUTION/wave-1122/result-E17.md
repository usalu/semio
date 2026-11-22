# E17 result: run waits (single-target `repo run`)

## What changed (all landed under area locks, recorded in `T/landings.md`)
1. **Optional ledger convoy fixed** (`🏗️build/📥️inputs/💾️storage/🟨️.mjs`, 381a80bb57ae -> see landings.md). `openOriginalGraphLedgerV1` no longer loops on `SQLITE_BUSY` while holding the queued exclusive `repository-graph-optional-ledger` lease: each attempt takes the lease (with the remaining budget as `timeoutMs`), tries the open once, releases the lease on BUSY, waits 50 ms and requeues, for at most `ledgerBusyLimitMsV1` = 5 s. On expiry (or when the lease queue itself times out) it returns a declining ledger (`unavailable: true`; `observe()` reports the quota full, `publish` declines with reason `ledger-contention`, `retire` throws BUSY, `close` no-op), so the writer runs and skips optional bookkeeping (optional cache leaves are recomputable). The writer skips LRU eviction and the retirement-due retry for such a ledger (otherwise it would loop). Dataset retirement (`retire`) uses the same bounded release-and-requeue and skips (directory is reclaimed later) when the ledger stays busy.
   Tests: new `📥️inputs/💾️storage/🧪️tests/⏳️ledger-busy` (fixture + schema, 5 cases: healthy, busy-then-ready, busy-forever, queue held by a peer, other error; asserts lease never held while waiting, bounded wall time, declining ledger contract) 1/0. Also fixed the stale `quotaSchema` binding of `🧪️tests/🔒️lifetime` (it failed before my edit: `definitions.DefaultPolicy` missing), now 1/0. retirement 2/0, shared-writer 4/0, identity 3/0, admission 1/0, retirement-window 4/0. Not run: `♻️scoped`, `🪣️refusal` (need a script invocation env). `🛑️primary` 3/1: case `funded-retired` of the preparation test (maintain path), not touched by this edit.
2. **Interactive priority end to end** (areas command, invocation, resident). `requestInvocationPlanV1` now sends `frame.priority` = `planPriorityV1(environment)` (foreground run 1, `SEMIO_GRAPH_BACKGROUND=1` 8; the plan client default stays 5); the host service passes `frame.priority` to `planner.plan`, the planner port to `loadSlice`, the command host to `slicePool.run`. The pool serves the waiting queue by urgency (0 most urgent; `slicePriorityV1` maps missing/invalid to 5), arrival order within one urgency, and a client that joins a queued shared load with a more urgent priority lifts it (`mine.priority = min`). Frame schema already had `priority` 0..9 "0 is most urgent".
   Tests: new `🏡️resident/🧪️tests/🚦️slice-priority` (4 fixture cases + schema) 1/0; invocation `🛰️plan` 17/0 (new `planPriorityV1` test); resident host 39/0, queue-progress 5/0, same-root-bound 4/0, slice-share 6/0, lease-wait 3/0, cut-load 9/0, roster-scope 3/0.
   Limits: the host service job (`governor.enter(agent/priority)`) is still keyed per frame and is not re-prioritised when a more urgent client joins an existing job; only the slice pool lifts.

## Measured anatomy (item 3 and 4)
Run: `repo run repo:test-cache-reset --exclude-dependencies --skip-cache` at 03:01 on the coordinator, load 40 to 50, after landings 1 and 2, timestamps by piping through `T/🧰️scripts/E17/phases.pl` (collapses the progress stream into phase starts). Total 908 s, failed at the end only because a peer's half landing: `Export named 'hostUnderPressureV1' not found in module .../🌡️host-capacity/🎫️gate/🟦️.ts` (not my area, reproduce-free; the Nx run itself took 20 s).
| t (s) | phase |
|---|---|
| 0 - 49 | `door.capacity`: waiting for 512 MB (host memory gate, 8.5 GB available, 4.5 GB reserved not resident) |
| 49 - 207 | graph service start (158 s; "The code on this machine changed while the graph service started; also waiting for the service of the current code (2 implementations seen)": every peer landing restarts it) |
| 207 - 327 | slice read waits for host capacity (memory: 0 to 295 MB spare above the 4096 MB floor, 6 grants, 2 queued); no slice-pool queue wait this time (pool max 4) |
| 327 - 491 | slice read itself 165 s (inventory 16k to 45k observations, 3,600 owners round 1) |
| 491 - 522 | `repo run` reservation grown 512 -> 2574 MB after 31 s (memory) |
| 523 - 576 | `build:source-admission` / `implementation-source` / `dependencies:generation-verify` (about 38 s of shared lease activity, twice) |
| 576 - 602 | `nx:storage-planning`, lease sweep, structural source membership (1024 dirs), owner custody |
| 602 - 620 | `nx:source-index` reading 11.6k entries |
| 620 - 833 | **Nx tool verification**: `nx:sdk-derivative` (exclusive), `sdk-derivative-recheck`, "Verifying build tools: N entries", a staged `nx-sdk-derived-namespace` (`.stage-...`) that is then published under a new digest: the derived tool namespace was REBUILT, about 210 s |
| 833 - 878 | implementation source read again, native existing-journal replay, Nx daemon wrote its project graph |
| 878 - 906 | Nx run of the task (20.1 s) |
Findings for item 3:
- The "fleet cache is not used for the nx-graph: fault" line did not appear in this run; the fleet step is not what the 420 s was. The graph-state client (`🪪️graph-state/🛰️exchange/📡️client`) already has `lookupMilliseconds` 1500, `claimMilliseconds` 1500, a state-dir breaker (`down` 15 s, `denied` 60 s, `noclaims` 600 s) and `beginNxFleetV1` never throws. Its message at 330 s in the earlier run is only the point in time at which that step was reached, after the phases above.
- The 420 s "nx admission" is a chain of CPU/IO-bound verification phases that rerun after every code change: source admission, dependency generation verify, source index, and above all the Nx tool derivative (`nx-sdk-derived-namespace`) being rebuilt (about 210 s here). With 30+ peer landings per hour the derived namespace digest changes and every machine rebuilds it. This is the lever (keep the derived-tool digest independent of unrelated sources, or share the derived namespace through the fleet cache), but it is in the Nx tool area (`⚡️caching/💾️storage/🔒️enforcement/🔌️nx`, E13 round 5 holds `native-image`); I did not change it. The other candidate wait in that area is `poolWithRoom`: up to `capacityWaitMilliseconds` 900,000 ms (15 min) waiting for a holder when the disk has no room; not seen in this run (no `nx:graph-capacity` activity), it would hurt at 97% disk.
- No code landed for item 3 (no wait found that is wrong in code and fixable safely in this time). Instrumentation was done outside the code by timestamping the progress stream (script kept).

## NOT verified
- Warm answer time of a cached target in seconds: not reachable on this machine at load 40 to 50 with the graph service restarting on each peer landing (service start 158 s, memory gate 31 to 120 s). The ledger and priority fixes are proven by unit tests and by the run not queueing behind slice workers or the optional ledger (no `repository-graph-optional-ledger` wait appeared), but a before/after on equal load is not possible.
- Priority was not observed live with two competing runs (only fixture tests on the real pool code).
- Item 5 (closure-first discovery) not started: E1 holds `graph`.

## Other lanes must know
- A peer's half landing broke the test task at 03:15 (`hostUnderPressureV1` named import missing from `🌡️host-capacity/🎫️gate/🟦️.ts`).
- My run left one resident graph service child (pid 45564, started by "Starting the repo graph service for this host", parent was my run) alive; it is the host's service, not killed by me.
- `🧪️tests/🛑️primary` case `funded-retired` fails (maintain path), independent of this lane.
