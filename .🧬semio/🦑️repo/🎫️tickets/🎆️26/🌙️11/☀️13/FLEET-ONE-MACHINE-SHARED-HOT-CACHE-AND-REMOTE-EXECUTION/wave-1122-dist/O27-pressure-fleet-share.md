# O27: raised memory pressure must not halve the fleet share

## Root cause
Two blunt divisors stacked on the memory term (which already uses free memory above the floor and a per-job footprint):
- `planComponentJobsV1` (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🟦️.ts:150`): `limit(floor(remote/pressureWarnDivisor),"jobs-pressure")`, policy `pressureWarnDivisor: 2`.
- `planFleetJobsV1` (`🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧮️units/🟦️.ts:87`, the fleet-sized cargo -j of O7/O13): same term, same key name in the units schema (44 -> 14 remote jobs in the coordinator's build).
Under "warn" all day both fired together, so the fleet was cut exactly when the local machine most needed work moved off it. (The runtime admission `admitFleetComponentJobV1` already stopped only on "critical".)

## Measured footprint (read-only ps, running play build, 2026-11-24 15:07)
Waiting unit-cache wrapper processes (script.ts / buildscript.ts under `.sinc/play/.cache/b/r/m/u/sealed/`): 19 samples, RSS min 40.7 MB, median 48 MB, max 51.4 MB. Policy `lightUnitMegabytes` 96 stays (about 1.9x the maximum); the number and source are now in the units policy (`lightUnitMeasurement`). Fleet-placed pass footprint `fleetPassFootprintMegabytes` 512 and `componentJobResidueMegabytes` 256 unchanged.

## Change
- Both warn branches removed: under warn the remote/fleet share is bounded by cap and MEMORY TERM only; critical still gives 0 (`jobs-pressure` / `fleet-pressure`, now meaning critical only).
- `pressureWarnDivisor` deleted from the job-slots schema (no compat value). In the units schema the same key also drove the LOCAL compile-unit halving (`pressure-warn`, protects this host's own memory, not the fleet share); it is kept but renamed `localPressureWarnDivisor` so nothing remote can read it.
- Texts EN+DE: "memory pressure is raised, so fewer remote jobs wait here" is gone; `jobs-pressure`/`fleet-pressure` now read "memory pressure is critical, so no remote job waits here" / "der Speicherdruck ist kritisch, daher wartet hier kein entfernter Job"; the "fewer remote jobs" statement lives only in `jobs-memory`/`fleet-memory`, i.e. only when the memory term bound.
- Docstrings updated; fixtures: warn with ample memory now keeps the full share, warn with scarce memory reports `*-memory`, the 92-idle-slot case under warn gives 44 remote jobs (was 14); Python oracles (no shared code) drop the warn branch.

## Tests (repo test-file, one at a time, main tree)
- job-slots plan: 4 pass / 0 fail (4792 expects, grid vs Python model)
- units fleet-jobs: 4/0; units (compiler-units): 14/0; job-slots tests/🟦️.ts: 5/0; job-slots fleet: 7/0.
- Patch set checked on a scratch copy of the clone's files: apply twice (second run 0 files), post-images parse (`bun build --no-bundle`), JSON valid, no `pressureWarnDivisor` left in the set.

## Patch set `wave-1122-dist/O27-patched/` and restart
7 files (job-slots ts + schema, units ts + schema, 3 units test/oracle files); README has the apply command and the interim one-liner (job-slots key is already 1 in the clone; units key still 2 and also halves local units unpatched). job-slots files are driver code: effective after a chain restart. units files: per next build/launcher start. The plan test files of job-slots do not exist in the clone and are not shipped. Note the clone's `componentJobsMaximum` is 6 (main 24); kept as is.

## Open
- Not run live (no restart). Edits were made to main only; clone untouched.
- `wave-1122-dist/O27-patched/edit-o27*.py` are the anchored edits used by build.sh.
