# Lane `cancel`: orphaned jobs and coalesced plan requests

## Root cause of the 40-minute orphan

- The warm pool / delegate / executor / resident service / composition are RETIRED (`🧪️tests/🚫️retired`); the live service is the per-host graph service (`🏛️service/🌐️host`). The warm `bun --eval` code I first suspected is dead code and was left exactly as before (my edits there were reverted).
- Live path: `host plan` -> `slicePool.run` -> a `worker_threads` Worker (`👷️worker.ts`) -> `loadBuildInvocationGraphV1`, which starts child processes (project farm, tool captures; `bun --eval`, 3 to 4 GB).
- On cancel the pool only calls `worker.terminate()`. A terminated thread runs no cleanup, so the processes it started were never signalled and lived on, still working, until they finished (about 40 min), blocking later loads. Verified with a probe: after `terminate()` the thread's child kept running (the child was killed by me afterwards, it was one I started).
- Host side was already right: a closed connection aborts its request (`socket.on("close")`), SIGKILL of a client therefore cancels the job.

## Changes (all under `🏡️resident/`)

- `🏗️composition/🌐️host/👷️slice-pool/🧹️reap/🟨️.mjs` (+ `.d.mts`), new: every load gets a token `SEMIO_SLICE_JOB=<servicePid>.<index>.<random>` in the worker's own `env` (children inherit it). `reapSliceProcessesV1` finds processes carrying a token (Linux `/proc/*/environ`, macOS `ps -Eww`; Windows hides environments, so nothing is found there), sends SIGINT, kills with SIGKILL after 2 s. `killSliceProcessesV1` kills at once. `deadServiceTokenV1` recognises tokens of services that are gone.
- `👷️slice-pool/🟨️.mjs`: the worker gets the token env; when a load ends (done, error, cancel, memory guard) the pool reaps the token's processes first and terminates the thread afterwards (so the thread can still collect its children's exit status); at creation it sweeps processes of dead services; `process.once("exit")` kills the tokens of running loads (reap on service exit); new `close()`. `graceMilliseconds` is injectable.
- `🏛️service/🌐️host/🟨️.mjs`: plan coalescing. A job (`jobs` map) is keyed by workspace (one index per workspace), revision, parent, tool, graph policy, arguments, sorted targets and dirty set. Identical requests attach to the running job instead of queueing; late joiners are replayed the last progress per phase; progress, queue position and result are fanned out to every attached client (each with its own request id and language). Each client has its own AbortController (cancel frame, deadline, closed connection); the job's controller aborts only when its last client is gone, and an abandoned job leaves the map immediately so the next request starts fresh. Counters `jobs`, `coalesced`, `abandoned`.
- Client invocation file: no change needed; it already cancels on SIGINT, SIGTERM and SIGHUP, and `requestPlanV1` sends `cancel` and destroys the socket on abort.

## Tests (written first, `🧪️tests/🛑️cancel/🟦️.ts`, data in `🧪️tests/🧫️fixtures/🛑️cancel.json`)

Only processes the tests spawned are started or signalled; every service has its own socket directory.

- A SIGTERM'd client subprocess (wired like the invocation: abort on SIGTERM) leaves no job: planner signal aborted and governor `inFlight` 0 within 3 s.
- A SIGKILLed client: same, within 3 s.
- Two clients, same plan: one planner call, `coalesced` 1, both get progress and the same reply. Different targets: two jobs.
- Cancelling one of two clients: other still gets the reply, job signal not aborted. Both gone: job aborted within 3 s, `inFlight` 0, the next identical request starts a new job.
- Slice workers: cancelled load, finished load, and a restarted service each leave none of the processes the worker thread started (within 3 s; zombies of a terminated thread count as dead).
- Oracle: the coalescing outcome is checked against the fixture numbers; plan content is the existing oracle-checked plan.

## Verified (`repo tools exec --light bun -- test ./<path>`, `SEMIO_TEST_ARTIFACT_DIR` set)

All resident suites pass: cancel 9, retired 3, host 26, service 37, delegate 7, executor 9, client 8, protocol 22, provision 16, graph-hooks 42+3+1, journal 5+4+3+1+8+3+2+1+1, state 14, watch 14+5, plan 2, command 3, exit-log 2, environment 2, source 1, endpoint directory 1. `📊️bench` and `🌊️burst` are scripts, not suites (bench needs `SEMIO_WORLD_WITNESS_MODULE`).

## Remaining

- Windows cannot attribute processes to a load (no environment scan); a process-group or job-object approach is needed there.
- A SIGKILLed service cannot reap at once; the next service start sweeps its tokens (tested), but until then the processes run.
- The host composition does not yet call `slicePool.close()` (the pool is created in `⌨️command/🌐️host/🟦️.ts`, outside this lane); the exit hook covers it.
- Not run against the live shared service (not allowed); coalescing key uses the workspace id, not the index digest.

## Follow-up (coordinator review)

- Cross-platform reaping replaced the token-only scan (`👷️slice-pool/🧹️reap/🟨️.mjs`, rewritten): the pool tracks per load (a) pids the worker reports with `{type:"child",pid}` (`reportChildV1` helper for worker code), (b) rows carrying the load's token (POSIX; children inherit it), (c) on Windows the new children of the service while no other load runs. Victims plus all descendants (process table: `/proc`, `ps`, PowerShell CIM) get SIGINT, SIGKILL after 2 s; each is signalled as a process group where it leads one, else directly (POSIX); `taskkill /T` then `/F` on Windows (`signalProcessTreeV1`, injectable `list`/`send`). The token is kept because it is the only way to see children spawned by library code that cannot be hooked (patching `child_process.spawn` does not reach named ESM imports in bun, verified) and for the dead-service startup sweep. Windows paths are modelled in a test but not run on Windows.
- Coalescing key is now the index digest (`identify`, or the loaded index key) plus revision, parent, tool, graph policy, arguments, sorted targets and dirty set. Caveat: two workspaces with identical inputs share the first workspace's job, so without a pinned `revision` the reply's capture/pin comes from the first workspace's files.
- `⌨️command/🌐️host/🟦️.ts`: minimal edit, `await slicePool.close()` in the `finally` of the foreground run.
- Tests added: reported child plus grandchild killed without token (own process group), two workspaces share a job, different revision does not, reaper over a modelled table. Cancel suite 13 pass; all resident suites pass (host 26, service 37, retired 3, protocol 22, provision 16, graph-hooks 42, state 14, watch 14+5, journal 5+4+3+1+8+3+2+1+1, others unchanged).

## Follow-up 2: no reply data across workspaces

The risk named above is closed by construction. Before this change a pinned `revision` in a request never selected the content (the capture hook runs inside the slice load against the workspace's own root), so no cheap capture digest exists in the host; a reply's `pin`, `manifest`, `revision`, `stale` flags and slice are all workspace-bound.
- Job key = index digest + content scope + revision, parent, tool, graph policy, arguments, sorted targets, dirty set. The content scope is `content:<digest>` only when the planner port offers both optional hooks `contentDigest({root,workspace,targets,arguments,tool,graphPolicy,dirty,signal})` (digest of the captured inputs, computed before attaching) and `bind({root,workspace,agent,reply,signal})` (the client's own `pin`, `manifest`, `revision`, ... merged over the shared reply and re-validated); otherwise it is `workspace:<id>` plus the root, so workspaces never share. The production planner port implements neither hook yet, so in production only requests of one workspace coalesce; nothing can leak. Implementing the hooks over the capture hook (a digest of the captured inputs without loading the slice) is the work that turns on cross-workspace sharing.
- The plan memo key now includes the workspace id too (it held another workspace's `stale` flags before).
- Tests (`🧪️tests/🛑️cancel`, fixture rows `otherWorkspace`, `boundContent`, `differentEdits`): without the hooks two workspaces are two jobs; with them, identical inputs share one job and each client gets its own root-bound pin back (equal plan tasks); a workspace with an uncommitted edit gets its own job.
- All resident suites pass again (cancel 15, host 26, service 37, retired 3, protocol 22, provision 16, graph-hooks 42, state 14, watch 14+5, journal and others unchanged).

## Follow-up 3: sustained memory throttling becomes a refusal, not a crawl

- Cause of the livelock: a load held 3.85 to 4.18 GB of a 4.29 GB cgroup; the kernel throttled it at `memory.high` (240k events) so it advanced too slowly to ever hit the byte budget guard or `memory.max`, and nothing else looked at the stall.
- New `👷️slice-pool/🌡️pressure/🟨️.mjs`: one watch per load, sampled with the pool's guard interval. Measure: memory PSI `full avg10` of the service cgroup where the host has PSI; else the `high` event rate of `memory.events`; else (no cgroup: macOS, Windows, devcontainers without cgroup v2) the host's free memory. PSI is authoritative when present. It trips once the measure stays over its limit for `sustainedSeconds`; any dip resets the clock, so bursts never trip. The pool then fails the load with `memory-pressure`, which reaps the load's children and terminates the thread as for any cancel.
- Schema (`🧬️schema/🔣️.json`): `HostPolicy.memoryPressure` = `fullAvg10Percent` 20, `throttleEventsPerSecond` 50, `freeFloorMegabytes` 256, `sustainedSeconds` 30 (typed in the policy `.d.mts`); EN and DE messages `host-memory-pressure-psi|events|free`. The host maps the error to a typed `memory` refusal with retry `later` (same path as the byte budget).
- Note: `sliceSafetyMarginMegabytes` sits in the `Policy` const, but `⌨️command/🌐️host/🟦️.ts` reads it from `HOST_POLICY_V1` (the `HostPolicy` const), so it is `undefined` there today; not changed here (outside the lane).
- Tests (fixture `pressure` rows, modelled `memory.events` / `memory.pressure` / free-memory sequences with a fake clock): PSI sustained trips at 30 s, PSI bursts do not, PSI wins over a hot events counter, events rate trips (31 s: the first sample has no rate) and a short event burst does not, free memory trips and a short dip does not; the cgroup reader parses a modelled cgroup tree and falls back; the schema declares the thresholds and both languages; end to end a load under modelled pressure is cancelled, its child process is gone within 3 s and the client gets a `memory` refusal (retry later) in EN and DE.
- All resident suites pass: cancel 25, host 26, service 37, retired 3, protocol 22, provision 16, graph-hooks 42, state 14, watch 14+5, journal and others unchanged. Nothing was run against the live service.

## Follow-up 4: allocator purge setting (request from rowstore, now `[done]` in `📌️requests.md`)

- Schema: `HostPolicy.memoryPurge` = `{environmentVariable: "MIMALLOC_PURGE_DELAY", delayMilliseconds: 0, description: {en, de}}`; typed in the policy `.d.mts`.
- mimalloc reads the variable when the process starts, so a worker thread's env alone cannot change the allocator of its process: it is set where a process starts. `spawnHostServiceV1` (`🏗️composition/🌐️host/🟨️.mjs`) puts it into the env of the service process (covering its worker threads), and the slice pool puts it into every worker's `env` next to the load token, so the children a worker spawns (derivation farm, tool captures) inherit it; the token and reaping are unchanged. The existing service env (`BUN_JSC_forceRAMSize`, client-bound variables removed) is kept.
- Test: the schema declares the setting with distinct EN and DE descriptions, and a slice worker's env carries the value, the load token and the inherited `PATH`. The service spawn itself starts a real detached process and is not covered by a test (the one line is read from the same policy).
- Resident suites all pass (cancel 26; host now 27 because another lane added a test). Nothing was run against the live service.

## Follow-up 5: the reaper missed the live job (read-only inspection, nothing signalled)

Inspection of pid 3288836 (`/proc` status, stat, cgroup, environ, cmdline, task list; exit.log; host log):
- It is not a child of the slice worker: it is the host service itself (`bun --eval ... service host run`, own process group and session, parent the trampoline 3288776, grandparent the client 3267039), 3.8 GB RSS in the service cgroup `semio-service-graph-d33e866b5344`, 16 threads. Its environ has `MIMALLOC_PURGE_DELAY=0` and no `SEMIO_SLICE_JOB` (the token only exists in worker envs).
- Its thread list shows one `Worker` thread (tid 3300600, state D, about 290 s of CPU) next to the main thread (state D, throttled at `memory.high`): the job is the slice worker THREAD inside the service process. `exit.log` shows this process wrote its own `exit` record with reason `memory`, code 75 about 348 s before my look, and never exited; the record lists the load as `running:false`, i.e. the pool had already declared it cancelled.
- Cause: the pool "cancelled" a load by `worker.terminate()`. A thread that is busy in synchronous work (the first Rust owner's inference) never reaches a point where it can be terminated, and a process cannot exit while it runs. So no token, pid report or process group was ever involved (there is no child process): the earlier reaper could only kill children, and its kill order made it worse (reap first, then terminate the thread). The same explains pid 3237097.

Fix:
- `👷️slice-pool/🧵️process/🟨️.mjs` (new): a slice load is now a process (`fork` of the bun runtime, own process group, IPC channel; request in `SEMIO_SLICE_REQUEST`), exposing the old `Worker` shape. `terminate()` is driven from the service's main thread: SIGINT to the group, SIGKILL after the grace period (2 s) whether or not the victim answered, `taskkill /T` on Windows. The load script (`👷️worker.ts`) and fixtures import `workerData` and `parentPort` from this module instead of `node:worker_threads`.
- The pool terminates the process first, then reaps stragglers by token and reported pid as before. Memory guard, pressure watch, deadline, client cancel and service exit all go through the same `finish`, so every cancel path kills the process. The service's exit hook and `close()` still sweep.
- Test (fixture row `busy`): a load that installs a no-op SIGINT handler and busy-loops synchronously, never yielding, is gone together with its child and grandchild within 3 s of cancel (the load process pid is asserted too). Existing slice-pool, memory guard, pressure and host tests pass on the process worker.
- Cost: the graph result now crosses the process boundary as JSON over IPC instead of a structured clone; a very large slice doubles in memory briefly in the child (its limit is the child's), not in the service.
- Resident suites: all pass (cancel 27, host 27, service 37, protocol 23, provision 16, retired 3 on a second run: it failed once while another lane was writing files, passes alone), nothing run against live processes.

## Follow-up 6: per-load cgroup, result handoff, `retired` flake

1. Per-load hard limit (`👷️slice-pool/🧱️limit/🟨️.mjs`, wired in `⌨️command/🌐️host/🟦️.ts` with one line `limiter: sliceLimiterV1()`):
   - Capability check once: `sliceLimiterV1()` = the existing `detectCgroupV2V1` + `createCgroupLimiterV1` (job-limits lane); undefined on macOS, Windows, containers without delegation. Then there is one code path: the pool asks the limiter if one exists, otherwise the existing guard, pressure watch and process kill apply.
   - Each load gets a leaf cgroup with `memory.max` = service limit - service usage - margin (floor 64 MB; no limit if the service has none), `memory.high` at 90% of that, `memory.swap.max` 0, `memory.oom.group` 1; the load process is attached after its fork (it boots for 100+ ms first); on end the pool terminates the process, reaps, then `release()` writes `cgroup.kill` and removes the group. A kernel OOM kill (`oom_kill` in `memory.events`) becomes `memory-budget`, the typed `memory` refusal with retry later; only that load dies.
   - Deviation from the request: the leaf is a SIBLING of the service's cgroup (under the delegated `semio-jobs`), not nested under it: cgroup v2 does not let a cgroup that holds processes enable controllers for children, so a child of `semio-service-graph-*` could not carry `memory.max`. The effect is the same and the service's own `memory.high` can no longer be reached through a load.
   - Test with a modelled cgroup fs (fixture `limit`): policy numbers (4096 MB limit, 3000 MB used, 100 MB margin gives 996 MB max, 896 MB high), floor, no limit without a service limit, files written, pid attached, `cgroup.kill` and removal on end, and an overrun (`oom_kill 1`) rejects with `memory-budget`, kills the process and writes `cgroup.kill`. Not run against a real cgroup.
2. IPC cost (measured with generated graph-shaped results, bun 1.4.2, this host):

   | result | JSON over IPC (round trip) | peak RSS child / service | file handoff (round trip, parse in service) |
   |---|---|---|---|
   | 50 MB | 0.9 s | 256 / 191 MB | 0.9 s (parse 0.4 s) |
   | 200 MB | 4.0 s | 749 / 772 MB | 2.9 s (parse 1.3 s) |
   | 600 MB | 42.8 s | 2124 / 2262 MB | 9.5 s (parse 5.0 s) |

   The live slice size is not recorded anywhere I could read (no spilled slice, exit.log has heap sizes only; the earlier note "1,208 MB of JSON for 4,748 projects" is the whole graph), so the sizes bracket it. IPC exceeds 1 s above about 50 MB and the peak is doubled in both processes, so the result now travels as a file: the load process stringifies into a private spool directory (`SEMIO_SLICE_SPOOL`) and sends only the path; the service reads and parses it and removes the directory when the process is gone and every read has ended (the `exit` event waits for pending reads: that race failed a host test until fixed). The parse (about 0.7 s per 100 MB) is the remaining cost and is inherent while the service needs the object; avoiding it would need the planner to consume the slice from the CAS by digest. Test: a 60,000-project result arrives intact and the spool is removed.
3. The `retired` flake: 26 runs (20 alone, 6 under parallel load) all passed, so the failure of the one sweep did not reproduce. The test scans the whole library tree while other lanes and suites write and delete files there; a file that vanishes between `readdirSync` and `readFileSync` throws `ENOENT` and fails the test. The scan now treats a vanished file or directory as not live source. This is the most likely cause, not a proven one. Also made the memory-guard test in `🌐️host` deterministic on a process worker (its modelled growth starts when the worker's first memory sample has arrived, not at fork).
- Resident suites all pass: cancel 31, host 27, service 37, protocol 23, provision 16, retired 3, graph-hooks 42, state 14, watch 14+5, others unchanged. Nothing live was touched; the measuring script was removed.
