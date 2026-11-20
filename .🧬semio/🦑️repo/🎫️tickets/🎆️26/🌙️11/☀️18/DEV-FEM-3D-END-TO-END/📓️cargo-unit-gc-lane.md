# Cargo unit collector and the native debug lane

- Layout `u/l/d/b/debug/build/<crate>/<hash>` is discovered and measured correctly (probe: 2289 units, 52 GB, only 279 held on Windows). With a large need the planner retires 48.6 GB of it. Discovery, depth, profile and hold logic are not the cause.
- Root cause: the unit collector had no budget. It removed units only on (a) host free-space need (used above the 87.5 % high-water mark of the volume) or (b) "superseded" = not among the newest 2 per crate and idle for 24 h. A lane that grows within a day, far below the volume high-water mark, was never retired until the disk was already full; `repo cache gc` (dry run) then reports "Within the budget; 0 B to evict" and plans 0 units (need = 0).
- Fix: `planUnitsV1`/`collectUnitsV1` take `budgetBytes`; the need becomes `max(needBytes, totalUnitBytes - budgetBytes)`, so eligible units (not held, past the age holds, non-newest first, then newest) are retired oldest first whenever units exceed their budget, and the apply loop only stops early when both the free-space target and the budget excess are met. `collectGarbageSingleFlightV1` passes `capacityBytes * gc.unitShare` (new policy `unitShare: 0.5`).
- Not wired: `repo cache gc` command (`cargoIncrementalReclaim`) and `reclaimUnitStorageV1` still pass no budget (they lack the partition capacity); the 6 h unit stamp still gates the scheduled scan.
- Files: `unit-cache/🧹️gc/🧱️units/🟦️.ts`, `unit-cache/🧹️gc/🟦️.ts`, `unit-cache/🧬️schema/🔣️.json`, `unit-cache/🧹️gc/🧱️units/🧬️schema/🔣️.json`, `unit-cache/🧹️gc/🧱️units/🧪️tests/🟦️.ts` (new fixture-tree test for the exact lane layout).
- Tests: `bun test ./🧪️tests/🟦️.ts` in `🧹️gc/🧱️units`: 11 pass. In `🧹️gc`: 15 pass, 1 fail (pre-existing, Windows: the volume test needs Python `os.statvfs`).
- Probe script: `probe-units.ts` in this ticket folder.

## Wiring (follow-up)

- `unitBudgetBytesV1(capacityBytes)` (gc/🟦️.ts) = `capacityBytes * gc.unitShare`, the single source for the scheduled collector, `reclaimUnitStorageV1` (new option `capacityBytes`) and the `repo cache gc` command (`cargoIncrementalReclaim`, capacity = `repositoryCacheBudgetV1(root).partitions.rust.capacityBytes`).
- Stamp gate: `units.stamp` (6 h) exists only in `collectGarbageSingleFlightV1` (scheduled); the command calls `collectUnitsV1` directly and always scans.
- New test: explicit reclaim with and without capacity (units/🧪️tests).
- `repo cache gc --dry-run` (timed out at 400 s in later phases, units phase finished): would remove 16 of 3803 units (3.0 GiB), 1792 held, 1995 kept; reason shown "image below its free-space floor" (host need was positive at that moment, so not attributable to the budget alone; total unit bytes now are under 0.5 x rust capacity, so the budget itself planned nothing extra).
- Tests: units 12 pass; with gc, budget and incremental files 32 pass, 3 fail, none from this change: discovery of incremental dirs (Windows separators, `b\p\...` vs `b/p/...`), `statvfs` volume test (no Python statvfs on Windows), background collector (flaky under load, passes alone).
- Incremental discovery test: the test was wrong (production returns `join`ed native paths); expected paths are now built with `join(store,...)`; `🌱️incremental/🧪️tests/🟦️.ts`: 7 pass, 1 skip, 0 fail.
