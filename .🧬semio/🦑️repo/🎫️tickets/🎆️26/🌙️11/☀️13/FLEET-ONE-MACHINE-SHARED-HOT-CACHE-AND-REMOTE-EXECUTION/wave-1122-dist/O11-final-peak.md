# O11: what a wasm component final really needs, and sizing it by measurement

L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/`, NB = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/`. Data in `O11-data/` (`fleet-load-1845.json`, `plan-smoke.json`, the two probe scripts). Patches and whole copies against the play snapshot: `O11-patched/`.

## 1. Measured peaks of a final (existing data only)

Source: the driver's unit telemetry `/Users/ueli/.sinc/play/.cache/b/r/m/components/build-telemetry/units.jsonl` (every locally compiled unit: `peakRssBytes`, wall, cpu; 11,375 rows, 24 of them component finals = non-host `cdylib`). The unit cache holds no per-crate `.peak` for any final (H2's writer is newer than every local final compile; main has 0 `.peak` files, the snapshot 13, none a final), `h/history.json` only knows whole component stages (`component#cold` 8.9 to 13.2 GB, a whole cargo batch, not a final), `fl/cost` is wall time only. The play progress lines agree: the single-final windows of `site-build-100131.log` (t 138-177 min, one unit open) peak at `rustc RSS` 5.9 GiB, `site-build-181007.log` (t 148-189) at 4.9 GiB (sampled every 30 s, so lower than the true peak).

| Final (crate) | builds | min MB | median MB | max MB | wall | output |
|---|---|---|---|---|---|---|
| semio_framework_editor_component | 5 | 325 | 382 | 385 | 36-91 s | 4 MB |
| semio_framework_surface_component | 5 | 507 | 548 | 548 | 51-117 s | 5 MB |
| semio_os_node_graph_component | 5 | 1156 | 1229 | 1232 | 111-268 s | 8 MB |
| semio_os_flow_component | 5 | 1704 | 1857 | 1857 | 113-245 s | 6 MB |
| semio_hub_puzzle_component | 3 | 3279 | 5022 | 6148 | 1078-1260 s | 88 MB |
| semio_hub_cad_component (wasip2) | 1 | 6550 | 6550 | 6550 | 2277 s | 214 MB |
| all 24 rows | 24 | 325 | 1229 | 6550 | | |
| one sample per crate (6) | | 385 | 1545 | 6550 | | |

Only 6 of the finals were ever measured, and only 2 of them are `semio_hub_*` (the two largest plugins). The other 45 `semio_hub_*_component` finals of the play plan have no measurement anywhere (dag, draw, norm_*, stdio_*, ... never finished locally with telemetry on). The peak grows with the size of the linked output (4 to 214 MB output = 0.4 to 6.5 GB), which is why one constant cannot be right for the family: 6144 MB under-sizes cad (6550) and is 3x too large for a flow-sized final.

Per worker spare (scheduler `/v1/fleet/load` at 18:45, signed read, `O11-data/fleet-load-1845.json`; `repo fleet status --json` does not export memory per worker). `spare` = `freeMegabytes` = memory a unit may take above the machine's floor:

| worker | RAM MB | spare MB | pressure | running | admits a final of 7938 MB | 
|---|---|---|---|---|---|
| g1-m12 | 16384 | 4471 | normal | 7 | no |
| g1-m03 | 16384 | 3477 | normal | 5 | no |
| g1-m06 | 16384 | 3099 | normal | 6 | no |
| g1-m08 | 16384 | 2492 | normal | 6 | no |
| g1-m13 | 16384 | 1267 | warn, refusing | 3 | no |
| g1-m10 | 16384 | 1255 | warn | 5 | no |
| g1-m07 | 16384 | 872 | normal | 8 | no |
| g1-m01 | 16384 | 725 | warn | 3 | no |
| g1-m02 | 16384 | 440 | warn | 6 | no |
| g1-m11 | 16384 | 0 | critical, refusing | 6 | no |
| g1-m05 (Windows) | 65447 | 28478 | normal | 4 | other platform: cannot take a wasm32 final of this toolchain key |
| g1-m04 (Linux) | 15848 | 1859 | normal, refusing (disk) | 0 | other platform |
| g1-server (coordinator, the requester) | 32768 | 8524 | normal | 3 | never its own requester |

Spare on the 10 compatible Macs: 0 to 4471 MB, mean 1.8 GB, max 4.5 GB, because every worker also runs its own `workspace` stage (2.3 to 4.6 GB RSS) and 3 to 8 units at load 7 to 35 on 10 cores. So the answer to "no 16 GB worker has 6144 spare" is: right, but not because of the constant alone.

## 2. Findings that matter more than the constant

1. The constant was too small for the big finals, not too large. cad needs 6550 MB measured (x1.25 = 8189 placed), puzzle 3279 to 6148.
2. Second cause, not fixed here (scheduler, restart rule): `committedMegabytes(worker)` in `L/🗓️scheduler/🟦️.ts:240` sums the declared peaks of every task still running on the worker for its whole life and `placementHasHeadroomV1` / `placementSlotsV1` (`L/🗓️scheduler/⚖️placement/🟦️.ts:60,101`) subtract it from the reported spare. The spare already shows what those running units use, so each running unit is charged twice. Live proof: `GET /v1/fleet/idle?peak=P` (the probe) gives slots 68 (no peak), 2 at 1024, then 0 at 2048 ... 8192 although m12 reports 4471, m03 3477, m06 3099, m08 2492 spare (each carries 4 to 5 tasks x >= 2048 declared). Without the double count 2048 would fit 5 times (m12 twice, m03, m06, m08). Recommended fix (schema-first, scheduler side): count a task as committed only until the worker's next load report after its start plus a ramp window from the policy (`receivedAtMs >= startedAt + ramp`), keep headroom as is. Not landed: it needs a scheduler restart (barred) and changes safety margins of every worker.
3. Remote finals teach nothing. A final that only ever ran on a worker has no measured peak: the worker's action result carries timestamps only (`L/👷️worker/🟦️.ts:332` `execution_metadata`), its `spawn` has no rusage. Open: the worker should sample its child's RSS (it already runs the host sampler) and return `peak-rss-mb`; the requester then calls `writeUnitPeakV1`. Not landed (worker restart). Until then the learning sources are: local compiles (unit-cache wrapper provenance and the broker telemetry) and kills.

## 3. The fix (landed, schema-first, key-neutral)

- `L/🚀️offload/🧬️schema/🔣️.json`: `policy.peak.family` = {crateSuffix `_component`, quantilePercent 90, minimumCrates 3} (definition + value; the cold bound `wasm.peakMegabytes` 6144 is now documented as used only while fewer than 3 family crates were measured, and as the sign of a linked wasm unit).
- `L/🚀️offload/📐️policy/🟦️.ts`: `offloadFamilyPeakMegabytesV1(samples)` (R-7 quantile, one sample per crate, `undefined` below the minimum), `offloadFamilyCrateV1`, `offloadUnitPeakMegabytesV1(learned, types, target, family?)`: learned x1.25, else for a linked wasm unit family x1.25 (bounds 64..16384), else the unknown/cold default.
- `L/🚀️offload/🚦️admission/🟦️.ts`: `readFamilyPeakV1(root)` reads `c/*_component.peak` (names only, one value file per crate). `L/🚀️offload/🟦️.ts:121`: a wasm final without a learned peak is placed with the family estimate; a worker kill (exit 137) still writes `2 x placed` to that crate's `.peak` (force), and since that file is a family sample the next unseen final is sized higher.
- `NB/🎟️admission/🟦️.ts`: `componentFinalCratePeaksV1(history)` (largest recorded peak per crate with a non-host cdylib, MB). `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts` (`componentJobs`): folds those telemetry peaks into the `.peak` ledger for crates without a record (never overwrites, so a kill is not eroded), then sizes the job plan with learned-else-family peaks per crate and the median of them. Data flow: any local final (wrapper provenance or broker telemetry) -> ledger -> the placement of the next final of that crate (wrapper) and of the plan (driver); a kill -> ledger -> family. Remote-only successes remain unmeasured (finding 3).
- Result on the measured set: family p90 = 6350 MB (6 crates), an unseen final is placed with 7938 MB, cad with 8189, puzzle 7685, flow 2323... The 47 `semio_hub_*_component` crates of the play plan: 45 at 7938, puzzle 7685, cad 8189, median 7938 (`O11-data/plan-smoke.json`). A kill of an unseen final at 7938 teaches 15874 and lifts the family to 10280 (unseen 12850), checked in the oracle and on disk.
- Safety: workers still admit by measured spare; the demand is higher than before for the heavy finals, so a final no longer lands on a worker that cannot hold it. The kill path falls back to local and raises the crate and the family.

## 4. Plan against the live fleet (plan only, no build), 18:45

`O11-data/plan-smoke.ts` (folded real telemetry into a scratch ledger, 47 hub finals, local grant 4, memory 13 GB normal pressure):

| demand per final | fleet slots | job plan |
|---|---|---|
| 1024 MB | 3 to 4 | 7 to 8 jobs (4 local + 3 to 4 remote) |
| 2048 / 4096 / 6144 MB (old constant) | 0 | 4 jobs (local only) |
| 7938 MB (corrected median) | 0 | 4 jobs (local only) |
| 10000 MB | 0 | 4 jobs (local only) |

So with the corrected demand 4 finals run concurrently now (all local), the same as with 6144. No demand value from 2 GB up gets fleet slots today, because of finding 2 and the stages on the workers (spare <= 4.5 GB). The corrected demand pays off when the workers' own stages finish: an idle 16 GB Mac has about 11 to 12 GB spare = 1 hub final (7.9 GB) or several small ones (the five framework/os finals take 0.5 to 2.3 GB each: 4 to 5 at once), 10 compatible Macs = about 10 hub finals + 4 local.

## 5. Tests (`repo test-file`, one at a time)

- `L/🧪️tests/🏋️unit-admission` 9/0 (+2): vectors from the Python oracle `🐍️.py` (10 sample sets, 18 kill cases, the observed release; own R-7 cross-checked against `statistics.quantiles(method="inclusive")` inside the oracle); TS equals the oracle; disk test: fewer than 3 family crates leave the cold bound, 6 measured crates read back 6349 (oracle), a kill of a fresh crate raises the family and the next unseen final's placement, the killed crate itself is placed with `2 x placed x 1.25` clamped.
- offload 41/0, offload wasm-cdylib 8/0, offload failure 2/0, placement 10/0, stage-claim 8/0.
- `NB/🎟️admission/🧪️tests` 14 pass / 2 fail: the new `componentFinalCratePeaksV1` test passes; the 2 failures are the broker capacity probes (`Actual finite memory capacity required`, `local is not defined`), untouched code, environmental.
- driver-stages: no runnable test file here (stub driver tests die on the undeclared Repo build selection, see O10); `bun build --target=bun` of its closure 0 errors; the callback's logic is exercised by `plan-smoke.ts`.
- Fixture float note: the real fold uses ceil in MB (cad 6551, family 6350, unseen 7938); the oracle uses the integer MB of the table (6550, 6349, 7937).

## 6. What the play session must change (snapshot `/Users/ueli/.sinc/play`, key-neutral)

Snapshot driver-stages is exactly the SY2 file (44df7226fd59), so the patches apply: `cd /Users/ueli/.sinc/play && patch <snapshot file> < O11-patched/<name>.patch` (each proven on a scratch copy, result byte-identical to main). Six files, order = schema, policy, admission, client, native admission, driver-stages (copy to a temp name, then `mv`):
1. `L/🚀️offload/🧬️schema/🔣️.json` (`offload-schema.json`)
2. `L/🚀️offload/📐️policy/🟦️.ts` (`offload-policy.ts`)
3. `L/🚀️offload/🚦️admission/🟦️.ts` (`offload-admission.ts`)
4. `L/🚀️offload/🟦️.ts` (`offload-client.ts`)
5. `NB/🎟️admission/🟦️.ts` (`native-admission.ts`)
6. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts` (`driver-stages.ts`)
Driver restart: yes (driver-stages and the job plan are loaded once per driver process; the wrapper picks up the offload files in a new sealed copy on the next rustc call, copy between units). No scheduler, worker or cache restart.
One-value alternative with no code: `policy.wasm.peakMegabytes` in `L/🚀️offload/🧬️schema/🔣️.json` (snapshot line 954, `"peakMegabytes": 6144`) is the only constant the unknown final reads. Setting it to 7938 gives the measured demand for every final but no per-crate or learned value and no gain in parallelism today; lowering it (to get slots) is unsafe (cad measured 6550). The plan line will then print `peak 7938` and still `0 on the fleet` until the workers' spare exists (finding 2).

## Open items
- Scheduler double count of committed memory (finding 2): the largest lever for fleet parallelism, needs a scheduler restart.
- Worker-measured peak returned in the action result (finding 3).
- 45 hub finals remain unmeasured; the first local compile of each teaches its own peak (ledger) and moves the family.
