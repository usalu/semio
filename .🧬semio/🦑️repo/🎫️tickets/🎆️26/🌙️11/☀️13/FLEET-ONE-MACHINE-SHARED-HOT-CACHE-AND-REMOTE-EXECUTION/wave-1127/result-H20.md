# H20 result: remote-execution scheduler and sidecar lifecycle (E3-E6)

Live `repo remote-execution status` before (01:00): 15 workers registered, one per origin (m01-m08, m10-m15, g1-server, m05), no duplicate. `no-endpoint` not reproducible: the status call resolved the endpoint from fleet facts (generation 13). The structural cause was real though: the worker name is `<host>-<pid>`, the scheduler keyed identity on name+origin, so every process restart registered a second entry and the old id lived until the lease ran out (`lastSeen` only), and was a placement candidate meanwhile.

## E5 identity-based registration (fixed)
- Schema `🏃️execution/🧬️schema/🌐️remote-execution.json` WorkerRegistration: optional `startedMs` (process start). Type in `🌐️remote-execution/🟦️.ts`; the worker (`👷️worker`) sends `Date.now()-process.uptime()*1000`.
- `🗓️scheduler/🟦️.ts` `register`: a registration of the same origin, other name and younger `startedMs` drops every older (or `startedMs`-less) entry of that origin (event `worker-replaced`, printed by `serve`); an older process never replaces a younger one, a registration without `startedMs` replaces nothing (no flapping).
- Test: scheduler test "a restarted worker of one origin replaces the registration of its older process ..." (maximumAttempts 1 proves no attempt is consumed).

## E4 restart drain (fixed the loss path; drain itself already existed)
- Drain exists: `createSidecarRestartV1` stops the agent taking work (`drain`), waits for `served.busy` up to `drainMaximumMilliseconds` (agent 7 200 000 ms), then aborts; the worker aborts its tasks, reports `cancelled`, deregisters.
- Defect: that path counted every abandoned or deregistered task as a failure, so three code-landing restarts in a row ended an offloaded action with "gave up after 3 attempts" (falls back local, but the fleet work is lost). Now `worker-left`, `worker-replaced` and "Worker abandoned the task" requeue without consuming an attempt; only `worker-lost` (crash, silence) counts (a poison action still ends). Test "a worker that leaves or abandons a task while it stops does not cost the task an attempt, a worker that is lost does".
- Windows restart-on-failure: already by `windowsSuperviseScriptV1` (F17). Found a probable defect (see E6).

## E3 queue wait derived from measurement
- `queueSaturatedMilliseconds` (120 s) stays only as the value while the scheduler has < `queueSaturatedMinimumSamples` (20) samples. With samples: wait = `queueSaturatedFactor` (10) x measured p90 queue wait, between `queueSaturatedFloorMilliseconds` (60 s) and `queueSaturatedCapMilliseconds` (300 s). Live numbers (42 samples): p50 1 ms, p90 1.7 s, max 84 s -> 60 s now, rising automatically to 300 s when the fleet saturates; the old fixed 120 s cap could not follow the 84 s tail.
- Code: `📐️policy` `offloadQueueWaitMillisecondsV1`; `🚦️admission` `fleetWorkersV1` returns `queueWait` (from `/status` counters, memoised with the workers); `🚀️offload` uses it for `waitMs` (pin mode keeps `pinQueueMilliseconds`). Schema keys added to the offload schema (required list, properties, const).
- Test: new `🚀️offload/🧪️tests/⏳️queue-wait/` (JSON vectors + independent Python oracle + policy-equals-vectors check).

## E6 Windows launch path
- Defect found by reading about_Parsing: the supervise script was one line, `& exe --% args;$c=$LASTEXITCODE;...`. The stop-parsing symbol hands the rest of the line to the program, so the `;` tail was probably passed to the service as arguments and the restart loop never ran. Now each statement is on its own line (`do{`, call, `$c=...`, `if...`, `Start-Sleep`, `}while`), which no reading of `--%` can swallow.
- Tests (topology): existing hosted-script test updated; new test asserts one statement per line, exactly one `--%`, the loop skeleton, and `conhost.exe <Arguments>` below 32 000 characters for the worker, cache and relay tasks. Launch of sidecars uses `windowsHide:true` (CREATE_NO_WINDOW), covered by `🪟️hidden-spawn`.

## Not verified
- Nothing ran on Windows (no PowerShell on this Mac): the newline form is by documentation, the `;` swallowing is a reading, not an observation. Check on m05 after the replica reinstalls the tasks: `schtasks /Query /TN semio-fleet-replica /XML`, kill the service process, expect restart within 5 s.
- The live scheduler (started 11-26 23:27) runs the old code until its sidecar restarts; duplicate-registration check after restart is open: `repo remote-execution status` must still list 15 workers and a restarted machine must not show two entries.
- `repo typecheck` of all touched files: 1 error in `🚀️offload/🚛️beyond-host/🧪️tests/🟦️.ts:80` (`expect(tries,...).toBe(row.tries)` number|undefined), a file I did not touch (peer H*/beyond-host lane).
- Not run: offload cluster tests (`🧪️tests/⏱️timeouts`, `🌐️cluster`), too heavy under load.

## Test status at 01:32
- Passed: scheduler 31/0, queue-wait 3/0. `dangling-imports` exit 0 (remote-execution, topology).
- NOT completed: `repo test-file` of topology tests, `⌨️command` tests and `🧪️tests/🟦️.ts` (one combined call, >20 min under load, still running without output when I stopped waiting). The topology edit (script text + 2 tests) is therefore only typechecked (clean), not run. Re-run: `repo test-file 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🧪️tests/🟦️.ts`.

## Hand-off 01:41
Coordinator: H25 owns running the topology, command and remote-execution suites to completion. My combined `repo test-file` run (pids 91939/91940/91942/91945) was stopped at 01:41 without a result; the topology script change and its two tests are untested beyond `repo typecheck` (clean) and `dangling-imports` (0). No further landings from H20.
