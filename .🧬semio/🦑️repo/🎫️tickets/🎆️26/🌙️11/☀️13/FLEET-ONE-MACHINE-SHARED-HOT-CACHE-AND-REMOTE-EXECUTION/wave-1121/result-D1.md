# Result D1: Disk and Image Thresholds as Fractions of the Host (2026-11-22)

File list and before/after sha256 are in `../landings.md` (section "Wave 1121 D1").

## 1. Budgets and minimums

- `elastic/<rust|products|graph>`: the absolute `minimumBytes` is now only an upper bound. `elasticMinimumV1` gives `min(minimumBytes, aligned host share)` with the share `100 - hostFreeReservePercent` of free bytes (rust reserve 66, products 88, graph 92), at least 64 MiB. Before, the 32 GiB rust minimum overrode the 50% reserve.
- Fixed partitions have `hostSharePercent` (dependencies 20, tools 10, diagnostics 2, nx 3, leases 1). `pinnedCapacityV1` cuts the capacity of a not yet born image to that share. Born images keep their receipt size.
- Invariant: all shares add up to at most 90 (34 + 12 + 8 + 20 + 10 + 2 + 3 + 1). Test over the declared policy with an independent Python model on 16 GB / 60 GB free (56 GiB), 32 GB / 200 GB free, Windows 256 GB disk / 5 GB free and 1 GB free: the sum of born image capacities fits the free bytes (checked from 4 GiB free).
- `tools` logical limit 64 GiB -> capacity minus 1 MiB (8588886016). Test: every `maximumLogicalBytes` is at most capacity minus 1 MiB. The old tests documented tools as an intended exception (hardlinked toolchains counted once per name); that exception is removed as ordered. Risk: one publication read of tools may now refuse above about 8 GiB logical. The tools image holds 3.3 GiB on this machine.
- Machine budget: `hostFreeFloorBytes` 15 GiB (it exists, in `ledger/🎚️machine/🔣️policy.json`, the census grep missed it) is now bounded by `hostFreeFloorPercent` 6 of the volume (`machineHostFloorV1`, Python oracle synced). A 60 GB volume keeps 3.5 GiB free, a 400 GB one keeps 15 GiB.

## 2. Stage disk needs and reclaim

- `readyDiskNeedV1(stage, runs, totalBytes)`: declared and measured (2x growth) need are capped at `diskGrowth.ceilingPercent` 25 of the volume (never below the 1 GiB floor). Python oracle synced (arguments 5 and 6).
- Refusal texts `disk-short`, `disk-collecting`, `disk-consumers` (en and de) now say "free at least {missing} MB more" (m05: 1207 MB of 6144 needed, 4937 free).
- Why reclaim freed 0 B (cause chain, found by reading, not reproduced on Windows):
  1. The recovery runs only writers with `relief host`. The writers for dependency generations, tool generations, closure layers and cargo incremental have `relief image` (they free space inside an image). On a directory-backed host (Windows, Linux) there is no image, so they give host bytes at once but were never run. `diskReliefV1` now maps them to host relief when the platform is not image-backed.
  2. The ledger collector `repo cache gc` is need-blind: a unit is removed only when unused for days or when its area is over budget. Nothing is eligible when the host is short but the areas are within their budgets. New `repo cache gc --reclaim=<bytes>` and `planCachePressureV1`: after age and budget, stale entries (not held, not quiet) of declared areas are removed oldest first until the shortfall is covered. The recovery passes the shortfall.
  3. On macOS the order is: ledger entries first (priority 10 to 62), then `image-slack` (90) returns the blocks. That only helps if step 2 removed something.
  Not covered: areas managed by their owner (unit cache, incremental) are delegated and ignore `--reclaim`.
- Python models for the plan (`planCachePressureV1`) and the born-capacity sum are inside the tests. The `planCacheCollectionV1` SQLite oracle still passes.

## 3. The 60,129,542,144-byte "laboratory floor"

Only a ticket script constant, no build-system constant: `WORKERS-READY-WITH-LOW-RESOURCES/constrained.ts` computes `max(reserve, minimumHostReserveGiB 12) + freeMarginGiB 1 + (imageGiB 41 + backingOverheadGiB 2) = 56 GiB`, policy in `constrained.json` `x-policy`. Not changed (ticket owned; the 12 GiB reserve and the 41 GiB minimum image are the absolute parts).

## 4. Nx workspace ENOSPC

- Nx partition: 3% share on a new image (about 1.7 GiB at 56 GiB free), artifact limit clamped to capacity. Live: nx 797 MB of 1.5 GiB used.
- A stage whose tail says "No space left on device", "os error 28", ENOSPC or the Windows disk-full text (`noSpacePatterns`, also the existing `storeFullPattern`) triggers one `sweepDiskV1` (every collector, image-relief writers included, whatever the host volume says) and one immediate retry without a recorded failure. A second shortage fails the stage as before (`store-full`). A clean run re-arms it. Tests: ready `💽️disk-recovery`.

## Verification

- bun test of all touched members and dependants: see landings.md. Not caused here: three budget tests need the repo test runner ("Original parent Script process invocation required"), two ready `resume` back-off tests, two fleet-status tests.
- Live read only: `repo cache status --json` and `repo doctor --only=storage,caches --json` ran clean (one existing warn about 24 dead graph publications, not fixed).
- Concurrent edit seen: the `cache-gc/⌨️command/🟦️.ts` edits were lost once (another writer) and re-applied; all markers verified at the end.

## Open follow-ups

- Typecheck was not run (only bun test). The `repo` typecheck target for the touched members should be run by the owner.
- Owner-managed areas ignore `--reclaim`; the Windows chain (EPERM rename, bun.exe sharing) is not addressed here.
- Gate G1 owns memory admission; nothing touched there.
