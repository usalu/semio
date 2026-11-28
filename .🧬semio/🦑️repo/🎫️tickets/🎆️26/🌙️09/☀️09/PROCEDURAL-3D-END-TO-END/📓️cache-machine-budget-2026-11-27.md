# Machine cache budget derived from the live volume (2026-11-27)

## Root cause

`repo cache gc` planned against `defaultTotalBytes` (194.31 GiB), a constant that knows nothing about the rest of the volume. Only two host terms existed: a 60 % share cap of the volume (136 GiB) and a pressure term that fired only below the hard free-space floor (min(15 GiB, 6 %) = 13.7 GiB). With 95.89 GiB held (104.77 GiB on disk) both were inactive at about 13 GiB free, so gc printed "Within the budget; 0 B to evict" while the volume was effectively full (git 13 GiB, user data and other apps are non-cache use the budget never saw). The hard floor also sits at the point where builds already fail (ENOSPC inside images), so it is too late to be the trigger.

## Change

- `ledger/🎚️machine/🔣️policy.json` and schema: new `hostReserveBytes` (24 GiB) and `hostReservePercent` (10 %), optional, never below the floor (semantic check in `readMachinePolicyV1`).
- `ledger/🎚️machine/🟦️.ts`: `machineHostReserveV1`; `planMachineBudgetV1` now derives the limit from the live volume: `hostBudget = max(floors, sum(max(demand, floor)) + (free - reserve) scaled by the headroom ratio)`, `effectiveTotal = min(total, 60 % cap, hostBudget)`. Total minus non-cache use minus reserve is exactly `caches + free - reserve`, so everything else on the disk counts against the caches. Free space is read through the existing statfs port (`nativeFreeSpaceSyncV1`, win/mac/linux). The plan carries `reserveBytes` and `hostBudgetBytes`; the report labels a default total that the volume cannot give as "derived from the volume". Pressure is relative to the reserve, so gc evicts (loose areas via tightened quotas) well before the hard floor. The headroom scaling makes evicting the planned bytes actually free what the host lacks (pre-scaling, only (1 - headroom) of the shortfall was freed).
- `ledger/🔣️policy.json`: new area `rust-tmp` (root rust, `tmp`, 1 GiB, collected after 1 day unused and 10 min quiet). `.cache/b/r/m/tmp` held 146 leaked `semio-jobs-*` dirs and response files that no area claimed; they are now measured and collected. cargo-store remainder shrinks by 1 GiB accordingly.
- Texts: `machine.source.derived`, `machine.pressure` names the reserve (en/de).

## Tests

- `ledger/🎚️machine/🧪️tests/🟦️.ts` (14 tests, pass): Python oracle (`🐍️.py`) extended with the reserve, scaled slack and host budget, 400 seeded machines identical; reserve law; fixtures `🧫️fixtures/🔣️.json` (Mac 228 GiB at 13 GiB free: pressure, eviction, converges within 5 %; 27 GiB free: derived label, no eviction; explicit budget never relabelled).
- `ledger/🧪️tests/🟦️.ts` (rust remainder 32 GiB), cache-gc budget/units/quota/directory/main, fleet-status, disk-recovery tests: pass. Typecheck of machine module and tests: pass (the first importer pass reported only test typing errors, fixed).

## Dry run (`repo cache gc --dry-run`)

- Before: `Machine budget: 194.31 GiB (default); 95.89 GiB in use, 104.77 GiB on disk ... Within the budget; 0 B to evict` (user-reported; volume 13 GiB free).
- After (volume 23 GiB free, reserve 22.8 GiB): `Machine budget: 73.34 GiB (derived from the volume: caches plus free space minus the reserve); 61.40 GiB in use, 88.04 GiB on disk`, per-store quotas shown, `Within the budget` only because free is at the reserve now; at 13 GiB free the fixture case plans about 10 GiB of eviction. The run also lists 125 `rust-tmp` entries to remove (0 B each, leaked pool dirs).

## The 96 GiB rust image

`.cache/b/r/fixed.dmg` is 103079256064 bytes (96 GiB) with 20.8-23 GiB of content; macOS unmaps only containers up to 32 GiB, so freed blocks never return to the host and the image is a permanent 96 GiB footprint. The plan reports "an image of 27.75 GiB would fit". Correct fix: relayout rust into shards at or below 32 GiB each (the existing shard machinery, `.cache/b/r/s`, `shards.jsonl`), created through the governed reset/regrow path, then retire the monolith; no relayout process was running and no `cache-relayout-rust` artifact exists in the tree (ticket note `native-publication-refusal-2026-11-26.md` calls it in-flight and needing the home image openable). Not triggered here. Until then the budget counts the image footprint (`overBytes`) but cannot shrink it.
