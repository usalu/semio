# Graph plan ceiling: why a plan showed 13 GB, and how it is now held under its ceiling

## Symptom
`repo capacity status` / the service log showed `graph-plan (running, 1 job(s), 13213..13788 MB)` on the 16 GB Mac. The free-memory accounting went negative (`-6349 MB spare above the 2048 MB floor`), so every other command (`exec`, `command`, the shooting activation) waited behind it. The plan ceiling is 6553 MB (`hostCapBasisPoints` 4000 of 16384), the absolute ceiling 9830 MB (`slicePlanGuard.ceilingBasisPoints` 6000).

## Which guard should have stopped it, and why it did not
Three candidates were checked against the service logs (`$TMPDIR/semio-repo/501/host-*.log`), the footprint traces (`$TMPDIR/semio-footprint/<pid>.log`) and the code.

1. Pressure gating (`planGuardVerdictV1`): NOT the cause. The pressure rule only decides inside the band between the limit (6553) and the absolute ceiling (9830) after one settle; the absolute ceiling refuses always. The log shows it firing (`graph-plan-too-heavy ... "megabytes":9895,"limit":6553,"reason":"ceiling"`). A real tree never stood above 9830 for long.
2. Accounting gap (the actual cause of the 13 GB figure): the number was a double count, not memory.
   `reserveGraphPlanCapacityV1` gave the held grant an observer that ignored its pid argument and always returned the footprint tree of the attached slice worker. The grant's heartbeat (`observeAll` in `holdHostGrantV1`) sums `observe(owner pid)` plus `observe(each attached pid)`. The owner is the service, the attached pid is its child, so the worker tree was read twice.
   Evidence: the series of the `behind graph-plan (...)` figures in `host-ac6bfe8f337c.log` ends at 13489 and 12859 MB while the pool never refused (it measures the worker once: about 6.7 GB, a few percent over the 6553 limit, inside the settle band); on the live host `repo capacity status` showed 2730 MB observed while the footprint of the service tree was 1621 MB and of the worker tree 1365 MB (sum 2986 MB, the two readings are seconds apart). Test below reproduces it exactly: a worker tree of 5700 MB was booked as 11400 MB.
   Consequence: the gate believed the plan held twice its real size, saw the host below the floor and parked every other command, although about half of that memory did not exist.
3. Managed bytes against resident memory (a real second gap, found while checking the worker): the slice worker had NO footprint gate. A gate is installed from `SEMIO_HOST_CAPACITY_GRANT` (`useEnvironmentGateV1`), and the pool starts the worker without that variable (the service, not the worker, holds the booking). `ps eww` of the live worker 54378 has no `SEMIO_HOST_CAPACITY_*`, its footprint trace has 0 `gate installed` lines. So in the process that does the inference `releaseOwnerStateV1` saw no share, never shrank or spilled (`closureEntriesV1.shrink`), the governor `hold` never held, and the only bound was the slice pool sampling the tree after the fact (once per second plus the time of a `footprint` call, against growth of over 1 GB/s: the log has `plan stage inference settled from 7338 to 10179 MB`, i.e. 2.8 GB grown while the settle waited).

Answer to the hypothesis in the task: the change that refuses only under `critical` pressure is not what let the plan reach 13 GB. The 13 GB was the double count; the real, but unspilled, growth was the missing worker gate.

## Fix (schema first, declared values only)
- Accounting: `graphPlanObserverV1` in `host-capacity` (used by `reserveGraphPlanCapacityV1`) reads the tree of the attached worker only and counts 0 for every other pid, including the service that holds the grant. It also honours the `observe` port of the caller (was overridden), which makes it testable.
- Worker gate: the pool names the declared plan limit to each worker it starts (`planLimitMegabytes`, derived from `sliceHostElasticity`, nothing hardcoded) in the new declared policy value `planCeilingEnvironment` (`SEMIO_GRAPH_PLAN_CEILING_MB`) of the memory policy. `worker.ts` calls `useSliceCeilingGateV1()`: a footprint gate whose grant is that ceiling and whose meter reads the whole tree of the worker (worker plus farm children, the same measure the pool and the capacity gate use, `readTreeFootprintBytesV1`). With the declared gate thresholds (`softPercent` 60, `hardPercent` 85) the existing paths now act before the ceiling: `releaseOwnerStateV1` releases from 60 % of the ceiling and shrinks and spills closure entries to the store from 85 %, the governor holds further owners while the tree stands above 85 %, and the settle watcher collects and shrinks. The pool refusal (`plan-memory`, absolute ceiling 9830 MB) stays as the last resort. Where the limit is off (`SEMIO_GRAPH_PLAN_LIMIT_MB=off`, serving hosts) no ceiling is named and the worker stays ungated as before.
- `serviceRevisionFiles` lists the two new code paths (`host-capacity/🟦️.ts`, `memory/footprint/🟨️.mjs`) so a service started before this change restarts on the new code.

## Changed files (all under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`)
- `🌡️host-capacity/🟦️.ts` (`graphPlanObserverV1`, `reserveGraphPlanCapacityV1`)
- `🧠️memory/🦶️footprint/🟨️.mjs`, `.d.mts` (`readTreeFootprintBytesV1`, `planCeilingBytesV1`, `useSliceCeilingGateV1`)
- `🧠️memory/🧬️schema/🔣️.json`, `🧠️memory/🧪️tests/🔣️.json`, `🧠️memory/🟨️.d.mts` (`planCeilingEnvironment`)
- `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` (names the ceiling to the worker), `👷️worker.ts` (installs the gate)
- `🏡️resident/🧬️schema/🔣️.json` (`serviceRevisionFiles`)
- tests: `🌡️host-capacity/🧪️tests/🧮️graph-plan/🟦️.ts` (two new tests, SQLite recursive-CTE process-table oracle; the existing booking test now also keeps its attach file in its own directory instead of the real capacity directory), `🧠️memory/🦶️footprint/🧪️tests/🟦️.ts` (new test, SQLite oracle for run/settle/wait), `🏡️resident/🧪️tests/🛡️plan-guard/🟦️.ts` (new test: the worker environment carries the declared limit, none where it is off)

## Test results (`repo tools exec bun -- test`)
- `graph-plan` (host-capacity): 6 pass, 0 fail. With the old observer semantics restored the two new tests fail (`Received: 11400`, expected 5700), then the fix was restored.
- memory + footprint + plan-stage: 87 pass, 1 fail on the first run (`releasing between the TypeScript programs ...`, a child-process timing test that hit the 5 s default timeout on this saturated host); alone with `--timeout 120000` it passes (6.2 s). New test `a slice worker told its plan ceiling ...`: pass.
- `plan-guard`: 12 pass, 0 fail (11 existing + 1 new).
- resident revision, plan-limit, plan-refusal, cut-load, cut-order, cut-stops, leaf-ceiling, history, reservation, booking, queue-progress, slice-worker-scope plus host-capacity shrink and partial: 59 pass, 0 fail.

## Not done / honest limits
- The runaway was not a live process any more: `repo capacity status` showed `graph-plan` at 2.7 GB observed with a healthy worker (pid 54378, child of the graph service pid 25029, front door pid 6726 still waiting for it), so nothing was stopped or restarted. The service (started 20:06) still runs the old code; it picks up the fix on its next restart (revision files changed) or `repo service` restart by its owner.
- Not verified live: a cold 4774-owner inference with the new gate. The 13 GB numbers I could explain are from earlier runs; for them the double count is shown by the test and by the live 2730 vs 1621 + 1365 MB reading.
- The band between the limit (6553) and the absolute ceiling (9830) is unchanged in the pool: a settled tree there continues unless the host is short or critical. With the worker gate the tree is spilled well before it (hard 85 % = 5570 MB), so that band is the backstop, not the working range.
- `repo tools exec bun -- test` of `resident/tests/host` (service lifecycle) was not run here.
- A stray `.attached` file from some existing host-capacity test (`VCFk05BiNtaUk1OWb_LNxw.attached`) stays in the real capacity directory; I did not create it knowingly and did not delete it.
