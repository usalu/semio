# Result R9: the plan-stage memory guard decides on what endangers the host

Landed 2026-11-15 05:26 under the build lock (`landings.md`, originals `T/🗑️generated/R9-landing-052617`, `land.py --rollback`). Script: `T/🧰️scripts/R9/land.py`.

## Cause
The text "the plan stage result (during result) grew to 6558 MB, more than the 6553 MB" is not the in-process guard of `🪜️plan-stage` but the slice pool (`🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`): every second it measures the footprint of the worker tree (`observeHostTreeMegabytesV1`) and refused at once above `planLimitMegabytes` (40 % of memory), whatever the host had free. The in-process guard (managed bytes, collect-confirmed) is unchanged.

## What was built (schema first: `HostPolicy.slicePlanGuard` in `🏡️resident/🧬️schema/🔣️.json`: reserve 2048 MB, ceiling 6000 basis points, settle wait 5000 ms)
- `planGuardVerdictV1` (`🧱️limit/🟨️.mjs`): pure table. Above ceiling: refuse (always, no settle). Within limit: continue. Above limit, not yet settled: settle. Settled and still above: refuse if pressure is warn/critical (`pressure`), available unknown (`unknown`) or available < reserve (`memory`); else continue.
- Pool: on the first tick above the limit it sends `{type:"settle"}` to the worker (new `send` of the process worker), waits for `{type:"settled"}` (or 5 s), measures again, writes ONE line per language `graph: plan stage <name> settled from X to Y MB` / `Graph: Plan-Stufe <name> beruhigt von X auf Y MB`, then judges with the verdict. A worker without `send` is judged at once. Settle is asked once per load.
- `👷️worker.ts`: on `settle` runs `settleGraphMemoryV1({force:true})` (collect, `Bun.shrink`, collect) and answers `settled`; the IPC channel is unref'd so it never keeps a worker alive.
- Available memory: `headroom().availableBytes` = `hostAvailableBytesV1` (R4 corrected reading, vm_stat free + reclaimable / MemAvailable). Pressure: new `hostPressureLevelV1` (`🌡️pressure/🟨️.mjs`): macOS `kern.memorystatus_vm_pressure_level` (1 normal, 2 warn, 4 critical), Linux PSI `full avg10` against the existing `memoryPressure.fullAvg10Percent` (warn at it, critical at twice), else undefined (then only available memory decides).
- Footprint instead of resident: already the case, `observeTreeMegabytesV1` defaults to `footprint` (macOS `phys_footprint`, Linux smaps_rollup Pss+SwapPss); nothing to change.
- Refusal: error carries `reason`, `availableMegabytes`, `reserveMegabytes`; service log line `graph-plan-too-heavy` and the refusal fields carry them; the texts `host-graph-plan-too-heavy` (en, de) name tree, limit, available memory and reserve (`planRefusalOfV1` fills `?` where the in-process guard knows no host numbers).

## Proof
Tests (new `🏡️resident/🧪️tests/🛡️plan-guard/🟦️.ts`, 10 pass): verdict table (6558/6553/6000 continue; 7000/1500 refuse `memory`; 2047 refuse, 2048 continue; warn and critical refuse; unknown available refuses; 9831 refuses with 60000 available and without settle; 9830 continues); schema values; pool with fake workers: settle once then continue (log lines both languages, `overBudget` 0); settle 6600 to 5900 logs `graph: plan stage edges settled from 6600 to 5900 MB`; 7000/1500 refused with tree, limit 6553, available 1500, reserve 2048 and rendered en+de containing all four numbers; pressure warn refuses at 6000 available; ceiling refuses with nothing sent; unanswered settle judged after the wait; pressure reader with fake darwin/linux sources. Changed `🪜️plan-refusal` test (+1 test, fields). Real IPC round trip checked by hand on the real `createProcessWorkerV1` with a child using the same handler: `send` true, `settled` received (rss 190 MB).
Landing run (live, baseline before and after): all 33 resident test files, texts, error, plan-stage, settle, plan-stages: 0 red, same counts as before. `🧭️invocation/🧪️tests/🛰️plan` has 1 red before and after (timing in the title, same as R7). All touched modules load; `worker.ts` parses. Not run: any `repo` graph/build command (forbidden on this machine).

## Not verified
- No real 16 GB worker run: the settle gain on a live slice worker (how many MB `Bun.shrink` returns during the `result` stage) is unmeasured; a worker busy in synchronous work answers only at its next event-loop turn, then the 5 s wait ends and the verdict uses the unsettled size.
- Third-party cross-check: none exists for a pure threshold table.
- The pressure source on Windows is absent (available memory alone decides there).
- Services already running keep the old code until they restart (the policy file is part of the service revision files, so a new service starts on its own).

## For the other packages
A refusal now needs a short host: workers with free memory finish where they were stopped (g1-m08/m07/m03 pattern). The absolute ceiling is 60 % of memory (9830 MB on 16 GB), the old limit (40 %) is now only the point where the settle and the host check start. The service log line `graph-plan-too-heavy` has new keys `available`, `reserve`, `reason`.
