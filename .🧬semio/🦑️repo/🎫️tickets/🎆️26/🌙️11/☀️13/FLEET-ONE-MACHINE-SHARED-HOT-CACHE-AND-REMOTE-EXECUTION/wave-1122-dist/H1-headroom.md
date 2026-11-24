# H1 Headroom audit: why 11 of 13 workers refuse

Read-only audit, no code edits, no sub-agents. Raw verdicts: `H1-fleet-status.json` (this folder, one `repo fleet status --json` run, `atMs` 15:26:54). Per-worker memory: `/Users/Shared/semio-store/v1/origins/<origin>/{ready,pool,slice-memory}.json` (snapshot 15:31 to 15:32). Snapshot and fleet status are about five minutes apart, so the worker-level numbers are indicative, not one instant.

## Short answer

- Verdicts: 11 of 13 entries refuse (m01 m02 m03 m05 m06 m07 m08 m10 m11 m12 m13). m04 (Linux, idle) and g1-server (busy, coordinator) have headroom true. The observation said 12 of 13; the count is 11.
- The refusals are not a CPU problem. m01 (3.0 of 12 cores), m07 (3.1 of 10) and m10 (5.1 of 10) are far below the load ceiling of 2 x cores (`policy.json` `hostCapacity.load.perCoreCeiling` 2).
- Eight of the 11 happen while a Ready stage claim is live. Seven of them are caused by the claim's memory hold (the stage reserves `entitlement - observed` out of available memory for every admission). The eighth (m13) is the pressure gate that the running stage trips.
- The remaining three (m02, m06, m12) are the stage's own real memory and swap use, or a failed stage with no claim.
- Fleet-wide, the claims hold about 24 GB that the machines are not using: 11.9 GB across six 16 GB machines (pool-reported) and roughly 12 GB on m05 (64 GB, estimate).

## 1. Per-machine table

Total memory: 16384 MB on all 16 GB Macs; m05 is 65447 MB. Floor = clamp(12.5 % of total, 1536, 4096) = 2048 MB on 16 GB (4096 on m05). A remote unit reserves `kinds.build.megabytes` = 2048 MB. "Pool hold" is the pool's own text ("N MB are held for the machine's own work"). "Claim E" is my derivation of `readyStageEntitlementV1` from the `ok` workspace runs in `ready-timings.json`; `ready-stage-claim.json` itself is not in the snapshot, so E is unverified.

| Machine | RAM | Load / cores | Pressure | Verdict and reason | Stage now | Claim E (derived) | Pool hold | Local work |
|---|---|---|---|---|---|---|---|---|
| g1-m01 | 16 GB | 3.04 / 12 | warn | refuse, pressure-warn | workspace, graph slice "taken 163 of 605" for 1793 s | 5497 (ok ws 4397) | 1056 (0 free above floor) | slice RSS 0.57 GB, offIn 159 |
| g1-m02 | 16 GB | 10.74 / 10 | normal | refuse, memory | workspace, graph inventory for 10647 s (about 3 h) | 1024 (ok ws 138, min) | 0 (waves done) | slice RSS 4.6 GB, offOut 2196, running 5 |
| g1-m03 | 16 GB | 8.15 / 10 | warn | refuse, pressure-warn | workspace, "taken 0 of 605" for 717 s | 4915 (30 % default) | 2457 | slice RSS 2.6 GB, offIn 1823 |
| g1-m05 | 64 GB, Windows | 13.48 / 16 | normal | refuse, memory | dependencies, "waiting for another build" for 265 s, peak 7.4 GB | 19634 (30 %, cap 50 %) | about 12 GB (est: 19.6 minus 7.4) | running 5 |
| g1-m06 | 16 GB | 23.0 / 10 | normal, swap-out 73 MB/s | refuse, memory (load also above the 20 ceiling) | workspace, graph inventory for 3120 s | 4255 (ok ws 156, 3404) | pool: pressure, no children | slice RSS 2.8 GB |
| g1-m07 | 16 GB | 3.05 / 10 | normal | refuse, memory | workspace, graph inventory for 2717 s | 4915 | 2457 | slice RSS 2.5 GB |
| g1-m08 | 16 GB | 10.83 / 10 | normal | refuse, memory | workspace, graph inventory for 4808 s (80 min) | 5494 (ok ws 4395) | 2457 | slice RSS 2.4 GB, offOut 2968 |
| g1-m10 | 16 GB | 5.11 / 10 | normal | refuse, memory | workspace, "reading the owners round 1" for 725 s | 4915 | 1035 | slice RSS 2.3 GB |
| g1-m11 | 16 GB | 14.42 / 10 | normal | refuse, memory | workspace, still running after 3740 s (door.script) | 7342 (ok ws 4514, 5873) | 2457 | slice RSS 3.3 GB |
| g1-m12 | 16 GB | 15.28 / 10 | normal, swap-out 11 MB/s | refuse, memory | fleet: dependencies (attempt 47); snapshot: workspace in deterministic backoff, exit 1 | none in snapshot | pool: pressure | no claim |
| g1-m13 | 16 GB | 17.64 / 10 | warn, swap-out 52 MB/s | refuse, pressure-warn | workspace, graph slice for 810 s | 4915 | pool: pressure (held not stated) | slice RSS 2.7 GB |
| g1-m04 | 16 GB, Linux | 3.31 / 12 | normal | headroom true (idle) | dependencies "waiting" on disk (6 GB needed), not running, so no claim | none | 0 | the only machine taking units |
| g1-server | 32 GB | 73.3 / 10 | normal | headroom true (busy) | coordinator | none | n/a | 12 running |

Per-machine free memory is not exported by `repo fleet status` (`fleet.machines` has no memory field, only the coordinator's `memory` block). That is an observability gap (section 4).

## 2. Which term refuses each machine (code, file:line)

Admission for one remote unit is `admitRemoteExecutionActionV1` in `🌡️load/🟦️.ts:88-97` (`.../🏗️build/🌐️remote-execution/🌡️load/🟦️.ts`). The order of checks:

1. `running >= concurrency` gives `slots` (not reached: concurrency is MAX in the report path).
2. Line 95: if a Ready stage is running (`view.stage` set) and pressure is not normal, refuse `pressure-warn` or `pressure-critical`. This comes before any memory arithmetic.
3. The remote candidate (`lent` priority, 2048 MB, one job) goes to `evaluateHostCapacityV1` (`🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts`, `decideHostCapacityV1` at lines 215-285). Refusal reasons are `memory` (room above the floor is under 2048), then `load` (only if memory fits), `swap`, `swap-activity`, `pressure-critical`, `queue-order`.

The inputs that matter:

- Floor: `hostFloorMegabytesV1` (`decision/🟦️.ts:107-111`), 2048 MB on 16 GB, 4096 on m05. Policy numbers in `🏗️build/🔣️policy.json` `hostCapacity.memory` (`floorBasisPoints` 1250, `floorMinimumMegabytes` 1536, `floorMaximumMegabytes` 4096).
- Room above floor: `observed = host.availableMegabytes - floor - unrealized` (line 221). Budget term: `(total - floor - unmanaged) - committed` (line 221); committed includes the ready-stage grant at its observed size, every running remote unit at 2048, and local grants. The budget is rarely the binding term here.
- Stage claim hold: `withReadyStageV1` (`🪑️stage-claim/🟦️.ts`) subtracts `entitlement - observed` (`readyStageHoldV1`) from `host.availableMegabytes` before any admission. This is the single biggest term, and it is applied in the view that the pool, the agent and the load report all read (`readRemoteExecutionHostViewV1`, `🌡️load/🟦️.ts:105-115`).
- Entitlement: `readyStageEntitlementV1` (`🪑️stage-claim/🟦️.ts`). With no `ok` run of that stage: `shareBasisPoints` 3000 of total, capped at `maximumShareBasisPoints` 5000, floor 1024 (`🪑️stage-claim/🧬️schema/🔣️.json` Policy). With `ok` runs: ceil(p90 of the last 16 peaks x 1.25), same clamp. Peaks are RSS sums (E73 notes), so they are inflated.
- Pool sizing reads the same view, so the pool's "held for own work" is the same hold (`🛰️sidecar/🏊️pool/🟦️.ts:26`).
- Scheduler: `placementHasHeadroomV1` (`🗓️scheduler/⚖️placement/🟦️.ts:57-61`) needs `report.headroom` and, when the unit has a peak, `freeMegabytes - committed >= peak`. The reported `freeMegabytes` is the post-hold value (`buildRemoteExecutionLoadReportV1`).
- Pressure: `reduceHostSampleV1` (`🌡️host-capacity/📡️sampling/🟦️.ts:112-114`) gives warn when available is below 15 % of total (2458 MB on 16 GB), when PSI stall is 10 % or more, or when compressed memory is 35 % or more of physical (5734 MB on 16 GB). Snapshots do not say which of the three fired, so the pressure-warn sub-cause is unknown per machine.

Per machine:

- m01, m03, m13 (pressure-warn): line 95 fires first. The stage itself drives the pressure (its tree grows the compressed share and the available share) and the gate then blocks every offload. m01 and m03 also have a hold (1056 and 2457 MB). m13 has pressure and swap-out 52 MB/s, and no held figure in the pool.
- m07, m08, m11 (memory, pool hold 2457): `available - hold - floor < 2048`. Pool text says 0 MB free above floor. Without the hold, free would be 2457, which would admit one unit. The hold is the binding term.
- m10 (memory, hold 1035) and m01 (hold 1056): the hold is part of it, but without it free would still be about 1 GB, below 2048. These two need real memory as well.
- m05 (memory, Windows 64 GB): the claim is 19634 MB, the 30 % default, because no `ok` dependencies run exists. The stage is in a wait ("another process is building the dependencies") but still holds the full entitlement. Estimated hold about 12 GB. This is the largest single term in the fleet.
- m02 (memory, normal pressure): the claim is 1024 MB (learned from one 138 MB `ok` run), so the hold is zero. The refusal is the stage's real memory: a 4.6 GB slice RSS during a 10647 s graph inventory, plus 5 running units. This is a real memory term, not a claim term.
- m06 (memory, swap-out 73 MB/s): swap is real, the stage's graph inventory is at 2.8 GB RSS, and load 23 is above the 20 ceiling. The load term would refuse it too. Real memory and load, not the claim.
- m12 (memory, swap-out 11 MB/s): the fleet row says dependencies running, the snapshot says workspace in deterministic backoff (exit 1, "trying again does not help"), so no claim is written. The refusal is real memory on a machine swapping. A failing stage is still occupying the machine (fleet says attempt 47).

## 3. Cause split (11 refusals)

- Claim hold is binding (hold 1 GB or more): m01, m03, m05, m07, m08, m10, m11 = 7. Of these, the hold alone decides it on m03, m07, m08, m11 (2457 MB each, which would admit a unit without the hold). On m05 the hold is about 12 GB. m01 and m10 need memory beyond the hold as well.
- Pressure gate of a running stage (line 95): m13 = 1 (m01 and m03 are counted above).
- Real memory and swap of the stage or the machine, no hold: m02 (claim hold 0), m06, m12 = 3.
- CPU: no refusal is caused by load alone. m06 would also fail on load (23 against 20).
- Disk: m04 is refused for disk only in its stage (`disk-collecting`), but it is idle and takes units, so disk is not blocking the fleet.

So 8 of 11 are gated by the Ready stage claim (7 by its memory hold, 1 by the pressure gate). Fleet-wide, about 24 GB is reserved and not used: 11.9 GB on six 16 GB machines (pool-reported) and about 12 GB on m05.

## 4. Options, ranked

1. Make the claim reserve measured use, not the entitlement (biggest effect, small change). The hold should be `max(1024, observed x 1.25) - observed`, not `entitlement - observed`. On m03 (observed 2.6 GB) this cuts the hold from 2457 to about 660 MB, which admits units. The 50 % cap on m05 (19.6 GB for a dependencies stage that is waiting) would fall to about 8 GB. Files: `🌡️load/🪑️stage-claim/🧬️schema/🔣️.json` (Policy `shareBasisPoints`, `maximumShareBasisPoints`), `🪑️stage-claim/🟦️.ts` (`readyStageEntitlementV1`, `readyStageHoldV1`). Keep the entitlement only as the start value for a stage that has not run yet.
2. Release the hold while a stage waits. A stage in "waiting for another build" (m05) or in a graph read that has not grown (m04's disk wait) should write `entitlement = observed`. Files: `🛰️sidecar/🚦️ready/🟦️.ts` (publisher at lines 906-1037, `stageClaim?.observe`).
3. Admit by per-unit demand, not a fixed 2048 MB. Use the unit's predicted peak (the unit cache already has per-crate costs; `readUnitCostV1`) against `spare = room above floor`, and admit a small unit when it fits even with a stage running. Files: `🌡️load/🟦️.ts` `admitRemoteExecutionActionV1` (line 88 onward, `remoteExecutionActionMegabytesV1`), `decision/🟦️.ts` (`need`, `fits`).
4. Narrow the pressure gate. Line 95 refuses every unit while the stage runs at any pressure above normal, including compressed-memory warn that the stage causes itself. Gate on critical only, or on warn with spare below the unit's peak. Files: `🌡️load/🟦️.ts:95`, `📡️sampling/🟦️.ts:112-114`.
5. Replace the boolean scheduler gate with a predicted-peak placement. `placementHasHeadroomV1` checks `freeMegabytes - committed >= peak`, and the peak is 6144 MB for every wasm cdylib/link unit (`🚀️offload/📐️policy/🟦️.ts:71`, `policy.wasm.peakMegabytes`). On a 16 GB machine with a stage that is at most about 6 GB free, no wasm link unit can be placed while the stage runs. Use a measured per-unit peak instead of the constant. Files: `🗓️scheduler/⚖️placement/🟦️.ts:57-61`, `🚀️offload/📐️policy/🟦️.ts:71`.
6. Heavy self-stages on non-Ready workers. The graph slice read is 1 to 3 h on m02, m06, m08 and m11 and holds 2.3 to 4.6 GB RSS. Cap the slice's resident memory (or read the graph on the coordinator) so a worker can accept units during the read. Also stop a stage that fails deterministically (m12) from holding the machine in backoff while the fleet needs it.

## 5. Open items and caveats

- `ready-stage-claim.json` and per-machine `available` are not in the snapshot. The hold numbers are the pool's own text (authoritative for the machine at 15:31), and E is my derivation. Verify by reading `ready-stage-claim.json` in each worker's host-capacity directory.
- The pool's `grantMegabytes` is 4915 on every 16 GB worker, which is the 30 % default, not the learned entitlements derived above (5497 on m01, 1024 on m02). Either the pool uses the default or the claim uses another run set. Not resolved.
- Timing: fleet status `atMs` is 15:26:54, the snapshots are 15:31 to 15:32. m12's stage differs between the two. Treat the m12 row as unresolved.
- Fleet status reports m09 as absent and the coordinator as `headroom true`. Neither is counted in the 11.
- Scheduler auth blocks direct GETs (`/v1/fleet/load` returns 401), so the scheduler's own per-machine load table was not read.
- No file was edited. The only file written in the ticket is this report and the copied `H1-fleet-status.json`.
