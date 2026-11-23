# O7: fleet-sized parallelism and light slots

## Change
1. **-j sizing** `planFleetJobsV1` (capacity `🧮️units`): jobs = local grant + idle fleet slots, bounded by schema `fleetJobsMaximum` 48, by memory (`lightUnitMegabytes` 96 per remote job beside the local compiles, after reserve), none under critical and half under warn pressure; fleet unknown/0 idle -> local only. Chosen once at build start. Idle slots: `fleetIdleSlotsV1` (offload admission) reads the scheduler `/v1/fleet/idle`; scheduler `placementIdleCountV1` now also returns `slots` (sum of free action slots of candidates); an old scheduler without `slots` falls back to `candidates`.
2. **Local vs light**: wrapper env `SEMIO_UNIT_CACHE_SLOTS` stays the local grant, new `SEMIO_UNIT_CACHE_LIGHT_SLOTS` = remote count. `script.ts` takes a light slot (`acquireLightSlotV1`, files `s/light-N.lock`, no ticket queue) only around the offload round trip; a locally compiled unit takes a normal slot as before. Light slots never count in `slotQueueMillisecondsV1`; waits instead of failing. Broker memory share, `SEMIO_COMPILER_PARALLELISM` and local slots follow the local grant only (coordinator memory protected). Light count also bounded by free memory in the plan.
3. **Wiring**: launcher (`repo cargo`): jobserver + `CARGO_BUILD_JOBS` = jobs; driver (play component build): second jobserver of jobs tokens used only for cargo passes (shared pool, stages and jobs stay local-grant sized); product callback in driver-stages. Progress: `N running locally · M on the fleet`; plan line EN+DE.
4. Running builds keep their wrapper/N; nothing restarted.

## Tests (repo test-file)
- capacity fleet-jobs: 4 pass / 0 fail (11 fixtures + 4 invalid, Ajv schema oracle, 5376-state grid vs Python model, bounds, EN/DE text).
- unit-cache light-slot: 3/0 (light not charged to local, wait without failing, cancel).
- placement: 9/0 (new idle `slots` case).
- offload tests (fleetHeadroomV1 refactor): NOT finished at report time (the file was still running after >30 min under host load); result unverified. Output: scratchpad o7-t.txt.
- Syntax of launcher, component-pass, driver-stages, script, admission verified with `bun build --no-bundle`.

## Open
- Build-script RUNs are executed by cargo itself, not via the wrapper: with larger -j more of them can run locally at once (cap 48).
- Local rustc compile can overlap with stage jobs (separate jobserver for passes): bounded by slots and the memory broker.
- Not run live (play build in flight); the `slots` field reaches the group server after its restart.
