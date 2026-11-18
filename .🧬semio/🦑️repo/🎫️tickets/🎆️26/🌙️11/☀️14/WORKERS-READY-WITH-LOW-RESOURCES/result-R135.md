# R135: the memory and time gates of worker readiness agree on small machines (2026-11-17)

Scripts: `T/🧰️scripts/R135/` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `stage-tests.mjs`, `stage-speed.mjs`, `mkovl.mjs`, `overlay.ts`, `run-staged.sh` (any test against the staged files), `battery.sh`, `compare.py`, `land.mjs` (`R135_STEP` 1 or 2), `restore.mjs`, `window.sh`, `farm-heavy.ts`/`.sh` and `deps-grant.ts`/`.sh` (the two real runs). The law test is `📚️library/🏗️build/🌡️host-capacity/🧪️tests/🖥️host-matrix`.

## 1. The host matrix and the cells that failed (red before, 5 of 7 laws)

The test builds one table: hosts 8 GB / 4 cores, 16 GB / 8 cores, 32 GB / 10 cores, free memory 50 and 75 percent, classes read parent, farm child, pool child, `dependencies`, build job, solo owner and service guard, each computed by the real decision function (`evaluateHostCapacityV1`, `sizeHostRequestV1`, `planFarmWorkersV1`, `planStageLimitBytesV1`, `graphServiceLimitMegabytesV1`, `ownerPoolSizingV1`, `soloGrantV1`, `ownerGrantV1`), with the host facts injected. Measured footprints that bound it (`MEASURED` in the test): farm child 560 MB working, 930 MB cold Rust peak (R129); `dependencies` 2921 MB peak, 1273 MB median and `show` 2153 MB median (the 16 samples of `estimates.json` of the host gate); a real parent 3500 MB.

Red cells and why (old tree, matrix before below):
- 8 GB at 50 percent free: read parent, `dependencies` and build job all waited with reason `memory`: the minimum 3072 MB plus the floor 1536 MB is 4608 MB, more than the 4096 MB that half of the host has; `dependencies` asks 6144 MB for a measured 2921 MB peak.
- farm width 0 on 8 GB: parent 1300 + child cap 2500 MB does not fit the hard share 2785 MB, so a read on 8 GB never had a farm child (16 GB had 2). The constants (typical 1720, cap 2500) were measured before the retained-memory trim; R129 measured 450 to 560 MB working, 930 MB peak.
- solo grant 937 managed MB on 8 GB (2344 MB footprint) while the machine minus the booked parent and the floor leaves 3584 MB: a gap of 1240 MB in which an owner fits the host but was refused by the solo limit. The per-owner share was 118 MB.
- service guard: plan limit 2457 MB on 8 GB is below the lean parent plus one child, 3200 MB: the smallest read is stopped by its own guard.
- `dependencies` books 6144 MB, 2.1 times its measured peak.
- alone cap of a farm child 4200 MB on every host: on 8 GB the parent and the child exceed the 60 percent plan ceiling (4915 MB) of the slice guard by 585 MB.

### Matrix before (old tree)

| host | free | class | admitted | jobs | reserved MB | limit MB | note |
|---|---|---|---|---|---|---|---|
| 8 GB / 4 cores | 50 % | read parent | NO | 0 | 0 | 0 | memory |
| 8 GB / 4 cores | 50 % | farm child | yes | 1 | 1536 | 1536 | reduced |
| 8 GB / 4 cores | 50 % | dependencies | NO | 0 | 0 | 0 | memory |
| 8 GB / 4 cores | 50 % | build job | NO | 0 | 0 | 0 | memory |
| 8 GB / 4 cores | 50 % | pool child | yes | 2 | 1860 | 3072 | nothing-fits |
| 8 GB / 4 cores | 50 % | solo owner | yes | 1 | 930 | 2048 | 819 managed MB |
| 8 GB / 4 cores | 50 % | service guard | NO | 1 | 4772 | 2457 | service 4915, plan 2457 |
| 8 GB / 4 cores | 75 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 8 GB / 4 cores | 75 % | farm child | yes | 2 | 3072 | 3072 | reduced |
| 8 GB / 4 cores | 75 % | dependencies | yes | 1 | 4608 | 4608 | shrunk |
| 8 GB / 4 cores | 75 % | build job | yes | 1 | 3584 | 3584 | reduced |
| 8 GB / 4 cores | 75 % | pool child | yes | 4 | 3720 | 3072 | fits |
| 8 GB / 4 cores | 75 % | solo owner | yes | 1 | 930 | 2344 | 937 managed MB |
| 8 GB / 4 cores | 75 % | service guard | NO | 1 | 4772 | 2457 | service 4915, plan 2457 |
| 16 GB / 8 cores | 50 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 16 GB / 8 cores | 50 % | farm child | yes | 4 | 6144 | 6144 | reduced |
| 16 GB / 8 cores | 50 % | dependencies | yes | 1 | 6144 | 6144 | fits |
| 16 GB / 8 cores | 50 % | build job | yes | 2 | 5632 | 5632 | reduced |
| 16 GB / 8 cores | 50 % | pool child | yes | 6 | 5580 | 4915 | nothing-fits |
| 16 GB / 8 cores | 50 % | solo owner | yes | 1 | 930 | 6144 | 2457 managed MB |
| 16 GB / 8 cores | 50 % | service guard | yes | 1 | 4772 | 4915 | service 9830, plan 4915 |
| 16 GB / 8 cores | 75 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 16 GB / 8 cores | 75 % | farm child | yes | 6 | 9216 | 9216 | reduced |
| 16 GB / 8 cores | 75 % | dependencies | yes | 1 | 6144 | 6144 | fits |
| 16 GB / 8 cores | 75 % | build job | yes | 4 | 9728 | 9728 | reduced |
| 16 GB / 8 cores | 75 % | pool child | yes | 8 | 7440 | 4915 | fits |
| 16 GB / 8 cores | 75 % | solo owner | yes | 1 | 930 | 10240 | 4096 managed MB |
| 16 GB / 8 cores | 75 % | service guard | yes | 1 | 4772 | 4915 | service 9830, plan 4915 |
| 32 GB / 10 cores | 50 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 32 GB / 10 cores | 50 % | farm child | yes | 8 | 12288 | 12288 | reduced |
| 32 GB / 10 cores | 50 % | dependencies | yes | 1 | 6144 | 6144 | fits |
| 32 GB / 10 cores | 50 % | build job | yes | 5 | 11776 | 11776 | reduced |
| 32 GB / 10 cores | 50 % | pool child | yes | 10 | 9300 | 9830 | fits |
| 32 GB / 10 cores | 50 % | solo owner | yes | 1 | 930 | 14336 | 5734 managed MB |
| 32 GB / 10 cores | 50 % | service guard | yes | 1 | 4772 | 19660 | service 19660, plan Infinity |
| 32 GB / 10 cores | 75 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 32 GB / 10 cores | 75 % | farm child | yes | 9 | 13824 | 13824 | fits |
| 32 GB / 10 cores | 75 % | dependencies | yes | 1 | 6144 | 6144 | fits |
| 32 GB / 10 cores | 75 % | build job | yes | 9 | 19968 | 19968 | fits |
| 32 GB / 10 cores | 75 % | pool child | yes | 10 | 9300 | 9830 | fits |
| 32 GB / 10 cores | 75 % | solo owner | yes | 1 | 930 | 22528 | 9011 managed MB |
| 32 GB / 10 cores | 75 % | service guard | yes | 1 | 4772 | 19660 | service 19660, plan Infinity |

| host | floor | readFixed | readMinimum | dependenciesFixed | dependenciesMinimum | farmPerChild | soloManaged | soloFootprint | shareManaged | planLimit | service | farmWidth | poolRecycle | poolHard | sliceBooking | sliceMinimum | planCeiling | leanParent | leanChild |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 8 GB / 4 cores | 1536 | 3072 | 3072 | 6144 | 3072 | 1536 | 937 | 2344 | 118 | 2457 | 4915 | 0 | 1200 | 3072 | 2048 | 2048 | 4915 | 1500 | 1700 |
| 16 GB / 8 cores | 2048 | 3072 | 3072 | 6144 | 3072 | 1536 | 4214 | 10536 | 1756 | 4915 | 9830 | 2 | 1200 | 4915 | 2048 | 2048 | 9830 | 1500 | 1700 |
| 32 GB / 10 cores | 4096 | 3072 | 3072 | 6144 | 3072 | 1536 | 10768 | 26920 | Infinity | Infinity | 19660 | Infinity | 1966 | 9830 | 2048 | 2048 | 0 | 1500 | 1700 |

### Matrix after (landed, live tree, same host facts)

| host | free | class | admitted | jobs | reserved MB | limit MB | note |
|---|---|---|---|---|---|---|---|
| 8 GB / 4 cores | 50 % | read parent | yes | 1 | 2560 | 2560 | shrunk |
| 8 GB / 4 cores | 50 % | farm child | yes | 1 | 1536 | 1536 | reduced |
| 8 GB / 4 cores | 50 % | dependencies | yes | 1 | 2560 | 2560 | shrunk |
| 8 GB / 4 cores | 50 % | build job | yes | 1 | 2560 | 2560 | memory, bypass after 30 s with 2560 MB (asked 3584) |
| 8 GB / 4 cores | 50 % | pool child | yes | 2 | 1860 | 3072 | nothing-fits |
| 8 GB / 4 cores | 50 % | solo owner | yes | 1 | 930 | 2560 | 1024 managed MB |
| 8 GB / 4 cores | 50 % | service guard | yes | 1 | 4004 | 3200 | service 4915, plan 3200 |
| 8 GB / 4 cores | 75 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 8 GB / 4 cores | 75 % | farm child | yes | 2 | 3072 | 3072 | reduced |
| 8 GB / 4 cores | 75 % | dependencies | yes | 1 | 3584 | 3584 | fits |
| 8 GB / 4 cores | 75 % | build job | yes | 1 | 3584 | 3584 | reduced |
| 8 GB / 4 cores | 75 % | pool child | yes | 4 | 3720 | 3072 | fits |
| 8 GB / 4 cores | 75 % | solo owner | yes | 1 | 930 | 3584 | 1433 managed MB |
| 8 GB / 4 cores | 75 % | service guard | yes | 1 | 4004 | 3200 | service 4915, plan 3200 |
| 16 GB / 8 cores | 50 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 16 GB / 8 cores | 50 % | farm child | yes | 4 | 6144 | 6144 | reduced |
| 16 GB / 8 cores | 50 % | dependencies | yes | 1 | 3584 | 3584 | fits |
| 16 GB / 8 cores | 50 % | build job | yes | 2 | 5632 | 5632 | reduced |
| 16 GB / 8 cores | 50 % | pool child | yes | 6 | 5580 | 4915 | nothing-fits |
| 16 GB / 8 cores | 50 % | solo owner | yes | 1 | 930 | 6144 | 2457 managed MB |
| 16 GB / 8 cores | 50 % | service guard | yes | 1 | 4004 | 4915 | service 9830, plan 4915 |
| 16 GB / 8 cores | 75 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 16 GB / 8 cores | 75 % | farm child | yes | 6 | 9216 | 9216 | reduced |
| 16 GB / 8 cores | 75 % | dependencies | yes | 1 | 3584 | 3584 | fits |
| 16 GB / 8 cores | 75 % | build job | yes | 4 | 9728 | 9728 | reduced |
| 16 GB / 8 cores | 75 % | pool child | yes | 8 | 7440 | 4915 | fits |
| 16 GB / 8 cores | 75 % | solo owner | yes | 1 | 930 | 10240 | 4096 managed MB |
| 16 GB / 8 cores | 75 % | service guard | yes | 1 | 4004 | 4915 | service 9830, plan 4915 |
| 32 GB / 10 cores | 50 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 32 GB / 10 cores | 50 % | farm child | yes | 8 | 12288 | 12288 | reduced |
| 32 GB / 10 cores | 50 % | dependencies | yes | 1 | 3584 | 3584 | fits |
| 32 GB / 10 cores | 50 % | build job | yes | 5 | 11776 | 11776 | reduced |
| 32 GB / 10 cores | 50 % | pool child | yes | 10 | 9300 | 9830 | fits |
| 32 GB / 10 cores | 50 % | solo owner | yes | 1 | 930 | 14336 | 5734 managed MB |
| 32 GB / 10 cores | 50 % | service guard | yes | 1 | 4004 | 19660 | service 19660, plan Infinity |
| 32 GB / 10 cores | 75 % | read parent | yes | 1 | 3072 | 3072 | fits |
| 32 GB / 10 cores | 75 % | farm child | yes | 9 | 13824 | 13824 | fits |
| 32 GB / 10 cores | 75 % | dependencies | yes | 1 | 3584 | 3584 | fits |
| 32 GB / 10 cores | 75 % | build job | yes | 9 | 19968 | 19968 | fits |
| 32 GB / 10 cores | 75 % | pool child | yes | 10 | 9300 | 9830 | fits |
| 32 GB / 10 cores | 75 % | solo owner | yes | 1 | 930 | 22528 | 9011 managed MB |
| 32 GB / 10 cores | 75 % | service guard | yes | 1 | 4004 | 19660 | service 19660, plan Infinity |

| host | floor | readFixed | readMinimum | dependenciesFixed | dependenciesMinimum | farmPerChild | soloManaged | soloFootprint | shareManaged | planLimit | service | farmWidth | farmAloneCap | poolRecycle | poolHard | sliceBooking | sliceMinimum | planCeiling | leanParent | leanChild |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 8 GB / 4 cores | 1536 | 3072 | 2304 | 3584 | 2304 | 1536 | 1433 | 3584 | 409 | 3200 | 4915 | 1 | 3615 | 1200 | 3072 | 2048 | 2048 | 4915 | 1500 | 1700 |
| 16 GB / 8 cores | 2048 | 3072 | 2304 | 3584 | 2304 | 1536 | 4505 | 11264 | 2048 | 4915 | 9830 | 4 | 4200 | 1200 | 4915 | 2048 | 2048 | 9830 | 1500 | 1700 |
| 32 GB / 10 cores | 4096 | 3072 | 2304 | 3584 | 2304 | 1536 | 11059 | 27648 | Infinity | Infinity | 19660 | Infinity | 4200 | 1966 | 9830 | 2048 | 2048 | Infinity | 1500 | 1700 |

The build job of 8 GB at 50 percent free still waits for its reservation (3584 MB, 2560 MB spare) and runs after the gate's own 30 s bypass with the 2560 MB that are there; the matrix models that rule (`unsatisfiableHostRequestV1`). I did not shrink the cargo reservation: seven replays of incidents (`🌤️followup`, `🌱️grow`) are pinned to it.

## 2. The new derivations

- `show`, `graph`: start 3072 MB (unchanged), minimum 3072 -> 2304 MB = lean parent budget 1536 MB + one farm child booked at its measured 768 MB floor (`askWorkerMegabytesV1`); minimum plus floor must stay within half of a host.
- `dependencies`: 6144 -> 3584 MB (1.23 times the measured peak 2921 MB), minimum 3072 -> 2304 MB (1.8 times the median 1273 MB).
- farm child: typical 1720 -> 900 MB (working 560 MB with fragmentation), cap 2500 -> 1400 MB (cold peak 930 MB plus half), worker peak 2200 -> 1200 MB. Width = 1 + floor((0.4 x host x 0.85 - 1300 - cap) / typical): 8 GB 1 child (was 0), 16 GB 4 (was 2), 12 GB 2 (was 1).
- `farmAloneCapV1(host)` = min(4200, 60 percent of the host - 1300 MB parent): 8 GB 3615 MB, 16 GB and up 4200 MB. A heavy owner on a small host gets the whole plan the guard allows, and the child is stopped by its cap before the guard stops the tree.
- plan limit (`planStageLimitBytesV1`) = max(2048, lean parent 1500 + child 1700 = 3200, 30 percent of the host) within 60 percent of the host: 8 GB 3200 MB (was 2457).
- solo grant (`soloGrantV1`): host - system reserve - parent footprint. Parent = min(3072, 37.5 percent of the host) (3072 MB is the read parent's booking), reserve = clamp(18.75 percent of the host, 1024, 2048): 8 GB 3584 MB footprint = 1433 managed MB (was 937), 16 GB 4505 MB managed, 32 GB 11059. The available-memory side uses min(2048, reserve) as its floor: with half of an 8 GB host free the solo owner may take 2560 MB.
- per-owner share (`ownerGrantV1`): half the host - the same parent: 8 GB 409 MB (was 118), 16 GB 2048 MB (was 1756).

Heavy owners: an owner whose recorded cost is above the share runs alone under the solo grant (existing `planSoloV1`: a learned cost never refuses an owner); it is set aside and waited for from the fleet by name (EN and DE lines of R103) only when it really grows past its cap. What is left unchanged on purpose: the seeds and recorded costs (13976 MB ... in managed megabytes) are inline-era overcounts: the real run below shows `@semio-tech/stdio-ifc-rs`, recorded at 6216 MB, holding 414 MB of footprint in a farm child.

## 3. Timers and loops (step 2)

- New member `🐢️speed` (schema, `machineSpeedFactorV1`, `localMachineSpeedFactorV1`, `processCpuMillisecondsV1`): factor = max(1, 8 / cores) x max(1, 16384 / MB) ^ 0.5, at most 4, rounded up to two decimals; 1 at 8 cores and 16 GB, 2.83 at 4 cores and 8 GB. A machine declared smaller (`SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES`, `..._CORES`) counts as declared. Law test with an independent Python model (exact fractions) over 8 machines, anchors, and the `ps`/`/proc` processor-time reader (a busy loop raises it).
- Farm: init timeout x factor; job timeout is a window without processor time: a child that spent at least 50 ms of processor time since the last look (every 5 s) re-arms it, so a progressing job is never killed by the cap; where the platform cannot say (Windows) it stays a wall-clock window x factor. Test `⏳️progress-timer` (3, red 2 on the old pool): 2600 ms of busy work outlives a 700 ms cap, 5000 ms of waiting is lost at it.
- Walk farm request timeout x factor. Take pass no-progress window (300 s) x factor (the progress sign and the CPU sign of R132 are untouched).
- Readiness: stage soft caps, the stall window, the stall stop and the wait for a holder x factor. The silent-stall restart is no longer "at once, for ever": pause 30, 60, 120, 240, 480 s (doubling, at most 600 s), at most 5 restarts in a row, the sixth stop is a failure `stage-stall-exhausted` (EN and DE text names the stage and the count) with the normal failure back-off (30 s to 20 min); a stage that ends in another way, or a stop for a changed code, resets the count. Tests in `🌲️process-tree` (pause doubling against an independent loop; scaled windows) and `🚦️ready` (two loop tests with the `world` harness; the old "at once" test now expects the first pause).
- Not changed: farm restarts total 8, slice restart total 120 and settle 900 s, stage hard caps (4 x soft, now x factor through the soft cap), fleet exchange sizes, closure store caps (rank 5 to 13 of the inventory outside memory and these timers).

## 4. Two small real runs on this Mac with injected host facts (declared 8 GB / 4 cores)

Cell farm child / heavy Rust owner (`farm-heavy.sh`, run twice: first against the staged files, then against the landed live code; the real plan functions, then the real pool and a real worker process on `@semio-tech/stdio-ifc-rs`, whose recorded cost is 6216 managed MB; load average 33 to 44 from peers). Live run after both landings:
`HOST {"total":8192,"cores":4,"free":4096,"speedFactor":2.83,"farmAloneCapMegabytes":3615,"capacity":{"verdict":"admit","jobs":1,"reserved":1536,"reason":"reduced"},"farmWidth":1,"soloManagedMegabytes":1024,"soloFootprintMegabytes":2560,"capMegabytes":{"typicalMegabytes":900,"capMegabytes":1400,"aloneCapMegabytes":4200}}`
`JOB {"owner":"@semio-tech/stdio-ifc-rs","ok":true,"seconds":388.8,"footprintMb":578,"peakFootprintMb":578,"peakRssMb":2507,"started":1,"capped":0,"lost2":0,"events":[{"kind":"priority","nice":5,"wanted":0}]}`
i.e. admitted alone with 1 job (reason `reduced`, 1536 MB booked), speed factor 2.83, farm width 1, alone cap 3615 MB, solo footprint 2560 MB at half free, and it completed: ok, 388.8 s, child footprint 578 MB (staged run: 389.5 s, 414 MB), resident set 2.5 to 3.0 GB (freed pages the host has not taken back), not capped, not lost. Before the change the farm width on this host was 0 (the read ran in the parent) and the alone cap 4200 MB.

Cell `dependencies` (`deps-grant.sh`, the real gate with lock directory and grant ledger on disk, sample injected, `repo dependencies sync --all`): old policy waited 30.1 s (`need 6144 MB; 2560 MB spare ... short by 3584 MB`) and was then admitted with 2560 MB by the gate's bypass; new policy admitted at once (76 ms) with 2560 of 3584 MB ("only 2560 MB are free above the 1536 MB floor right now, and 2304 MB are enough to start").

## 5. Landings (entries `R135 landing step 1` and `step 2` in `landings.md`)

- Step 1, arithmetic: 2026-11-17 15:30:00 to 15:31:26 UTC (first attempt 15:02 restored on red: the matrix expected the 8 GB build job at once).
- Step 2, timers and loops: 2026-11-17 17:00:00 to 17:01:38 UTC (the 16:00 and 16:30 windows were lost to R134 and R138 by seconds; a run at 16:34 was restored on red: my speed test measured the processor time of a fixed 1.2 s loop at load 44).
- Batteries before each window against the unchanged tree: none worse. Existing tests that pinned the old arithmetic were moved on purpose (cost, host-capacity main and shrink, plan-stage main and tree-plan, farm-width model and fixture, idle-grant and linux-host oracles, cap, readiness "stalled at once").
- Both standing preflights green under both locks.

## 6. Not verified, not done

- The service guard's restart rules (slice worker restart total 120, settle wait 900 s, safety margin 256 MB, plan ceiling 60 percent) are not changed: they react to code changed under the worker, not to memory, and I had no run that tripped them. Only the plan limit under the 3200 MB lean parent plus child was raised.
- Fleet side (rank 4): a small host still cannot serve (threshold 24000 MB); a heavy owner that really outgrows its alone cap is still handed to a larger machine and waited for by name, ending in `graph.owner-too-heavy` or `graph.wanted-expired` only if no machine delivers within the (now speed-scaled) no-progress window. Not run end to end.
- No real 8 GB machine: all small-host cells use declared facts on this 32 GB Mac (`SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES`, `..._CORES`). The farm run cannot show swapping behaviour of a really small host; the speed factor is a model (cores ratio times the square root of the memory ratio), not a measurement of this repository on a 4-core machine.
- The job timer is progress-based only where the processor time of a child can be read (Linux, macOS). On Windows it is a wall-clock window times the factor. The walk farm request timer and the farm init timer are scaled but not progress-based.
- The build job of an 8 GB host at half free still waits 30 s for the gate's bypass (reservation 3584 MB against 2560 MB spare); I kept the cargo reservation because replays of incidents are pinned to it. `check` jobs (4096 MB per job) are not in the matrix.
- Recorded costs and seeds of owners (managed megabytes from the inline-inference era, for example 6216 for the owner of the real run, which holds 414 to 578 MB of footprint) are untouched; they only classify an owner as heavy, which now means "runs alone under the solo grant", not "refused".
- The python models read the live schemas, so under the staged overlay the farm-width model disagreed until the files landed; under the lock it passed.
