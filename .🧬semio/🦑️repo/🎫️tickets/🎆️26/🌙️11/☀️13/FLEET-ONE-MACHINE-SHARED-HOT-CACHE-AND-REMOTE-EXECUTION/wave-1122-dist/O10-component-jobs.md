# O10: fleet-sized concurrent component jobs

## Root cause (measured)
Play log: `27 pass(es), 73 component build(s)`; the live cargo is `-p dag -p draw` (a batch). All passes are profile `release`, so they form ONE lane and the driver ran them in sequence (`for (const pass of lane.passes) await runPass(pass)`, `component-pass/🟦️.ts` ~1173); each cargo also held a local jobserver token for its whole life (`runAdmittedPass`, `jobserver.acquire`). O7's -j sizing only acts inside one cargo.

## Change
1. `component-pass/🧵️job-slots/🟦️.ts` (NEW) + `🧬️schema/🔣️.json` (NEW, own schema): `runPassesInJobSlotsV1` runs a lane's passes with up to `jobs` offloadable (`wasm32-` target) passes at once; the first holds the local slot, the others a light one (no jobserver token: only the cargo process and input shipping live here, units go to workers via the wrapper; a final the fleet declines waits for a local unit slot, never exceeds the local grant). Non-offloadable pass, or jobs 1 = old strictly sequential behaviour. First failure stops launching, calls the driver `stop` (kills sibling cargos) and rethrows after all settled. `planComponentJobsV1` = local + fleet slots, bounded by schema cap 24, coordinator memory (256 MB residue per remote job after reserve and local compiles), none under critical / half under warn pressure; fleet down or 0 slots = local grant (1). EN+DE plan line.
2. `component-pass/🟦️.ts`: option `componentJobs` (called only when O7's fleetJobs granted remote > 0), `jobSlots = min(plan.jobs, tokens + fleetRemote)`, lane loop uses the runner (`fail: stop` only when jobSlots > 1), `held` slot parameter (light = no jobserver acquire).
3. `driver-stages/🟦️.ts`: product callback `componentJobs`: peak = median over component crates of `offloadUnitPeakMegabytesV1(readUnitPeakV1(...), cdylib, wasm32-wasip2)` (learned else 6144), slots = `fleetIdleSlotsV1(env,{peakMegabytes})`, plan, log line.
4. Peak-aware slots (scheduler side, optional for the clone): `placementSlotsV1` (placement): slots of a sized action = min(free action slots, floor((reported free MB - committed)/peak)), >= 1; `idleFor(requester, peak)`, `/v1/fleet/idle?...&peak=MB` (server), `fleetIdleSlotsV1` ports `peakMegabytes` (admission, already in SY1 set A). An old scheduler ignores `peak`: slots are then action slots only (over-optimistic, placement still gates; jobs wait).

## Shared work between concurrent cargos (verified by reading)
All passes share one `CARGO_TARGET_DIR`/`build-dir` (repo-wide nightly `-Zfine-grain-locking`: concurrent cargo blocks per unit, not per dir). The unit cache claim is `acquireUnitClaimV1(<UC root>/locks, key[0..40])` (`unit-cache/📜️script.ts:189`), keyed by unit key only (out-dir stripped in the key, `🔏️key:132`), under the shared cache root, with re-`serve()` after the wait (`:193`): a unit needed by two cargos is compiled once, the second takes the cache hit. Nothing here depends on target dir. No private target dirs are created.

## Realistic parallelism now
`repo fleet status` could not be read in time (two runs under the old taskpolicy rule timed out; not re-read). Arithmetic: 11 workers x 16 GB; spare memory ~10 GB each -> floor(10/6) = 1 wasm final per worker (2 only on a >= 12.3 GB spare), so realistic = 11 (up to 22 when idle), plus 1 local = about 12 concurrent jobs, instead of 1. Learned peaks (measured 1.2 to 4.7 GiB, x1.25 margin = 1.5 to 5.9 GiB) would give 1 to 2 per worker: worth up to ~2x, applied through the median. Learned peaks only come from local compiles; finals that only ran remote keep 6144 (H2 open item: workers should return peak RSS).

## Risks
- Several cargos on one build dir: `feedback-fine-grain-locking-wasm-deadlock` (prebuild_lock_exclusive cycle) was seen with 12 concurrent wasm-dev cargos. Cap `componentJobsMaximum` (24) is schema-declared; the driver stall watchdog (`stallMs` 15 min) still applies. Lower the cap if a cycle shows.
- The wrapper's unit slots/jobserver are shared pools, so total rustc concurrency is still bounded by `tokens + fleetRemote` (pool of the fleet jobserver).

## Tests (`repo test-file`, one at a time, no taskpolicy)
- `job-slots/🧪️tests/🧵️plan` 4/0: fixtures valid for Ajv, 11 cases, 5 invalid, 6-axis grid vs Python oracle, N+1 jobs / fleet down 1 / memory bound, EN+DE text.
- `job-slots/🧪️tests` 5/0: schedule vectors (fleet down 1, 3 jobs, native pass alone), stub blocking jobs show 4 concurrent with <= 1 local, failure and cancel semantics.
- placement 10/0 (new: sized slots, 16 GB worker takes 1, 26 GB takes 2).
- driver test file (`component-pass/🧪️tests`, new test "component jobs": 3 wasm workspaces overlap with jobs 3, sequential with jobs 1 or no fleet): see landings.md for the result.

## Clone sync (snapshot = main ~00:40 + SY1)
Key-neutral: nothing under `🔏️key`, `🪪️extern`, `buildscript`, unit-cache schema; no cache key input changes (only scheduling of cargo processes).
Required (3 files + 2 new):
- NEW `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🟦️.ts`
- NEW `.../🧵️job-slots/🧬️schema/🔣️.json`
- `.../🧩️component-pass/🟦️.ts`: apply `O10-patched/component-pass.patch` (on top of SY1-patched/component-pass.ts) or copy `O10-patched/component-pass.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`: `O10-patched/driver-stages.patch` (against the snapshot file) or copy `O10-patched/driver-stages.ts`. It imports `readUnitPeakV1` / `fleetIdleSlotsV1` (admission) and `offloadUnitPeakMegabytesV1` (policy): both in SY1 set A.
Not needed in the clone (scheduler/server side, restart rules): placement, scheduler, server edits. Tests are not part of the sync.

## Driver test result (final)
`component-pass/🧪️tests/🟦️.ts`: 46 pass / 24 fail. The failures are environmental: every stub driver run builds the native-build scheduler through `repo cargo` and dies with "Internal error: Undeclared original Repo build selection" (the `🎟️admission/📦️packages/🦀️rust` build selection is undeclared in this tree state), so the new "component jobs" stub test did NOT get to run and is UNVERIFIED. The job-slot runner itself is covered by `job-slots/🧪️tests` (5/0, stub blocking jobs, 4 concurrent, <= 1 local slot). Not judged against a baseline; the old stub driver tests fail the same way.
