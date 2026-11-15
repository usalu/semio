# Package R4: the 16 GB Mac worker has a grant again (2026-11-15, 04:30 CET)

LANDED 04:30 (`T/landings.md`, "Package R4 landing"; originals in `T/🗑️generated/R4-landing-043019`; rollback `T/🧰️scripts/R4/land.py --rollback <dir>`). T = `…/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, PROC = `🧰️framework/🔨️modules/🏃️process/🏗️build`. No graph command, nothing killed. `LAND-OK` written and removed again.

## Cause (two layers, both file:line in the code before the landing)

1. **macOS "available" was far too small.** `PROC/🌡️host-capacity/📡️sampling/🟦️.ts:29-30` (`parseDarwinCountersV1`, N7) returned `min(free + inactive + speculative, free + File-backed + purgeable)`. The second term is the binding one: free pages plus only the file-backed cache plus purgeable. Anonymous inactive pages, the bulk of "cooling" memory on an idle Mac, were never counted. Live example: this Mac (32 GB) read 4 557 MB (the kernel says 75 % free); a synthetic idle 16 GB Mac (inactive 9.5 GB, file cache 5.2 GB) read 5 515 MB.
2. **The grant was tied to that reading.** `LIB/🪪️graph-state/🙋️wanted/🟦️.ts:116-119` (`fleetAdmittedMegabytesV1`) = `max(0, min(half of host, available - 2048))`; `LIB/🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts:56-59` (`ownerGrantV1`) subtracts the 3 800 MB floor, so any reading below 5 848 MB gives a grant of 0 MB per project and (`inferableProjectsV1`, wanted `🟦️.ts:16-18`) 0 projects per wave. With a grant of 0 every project that has a measured cost (226 on the Mac) is "too heavy". The cost table itself is fine: `classifyOwnersV1` (`💰️cost/🟦️.ts:63-65`) already compares the measured managed cost with the managed grant (physical = managed x 2, both printed in physical MB); the "12 too heavy" before P14 and "226" now differ only by the grant (3 512 vs 0). Not a unit or parent-attribution error.

## Fix

- `parseDarwinCountersV1`: available = free + max(inactive + speculative, file-backed + purgeable), i.e. what the kernel can hand out without swapping (the compressor is not counted). One reader (N7 interface `createHostAvailableBytesV1` / `hostAvailableBytesV1` and the capacity gate share it).
- New `fleetGrantMegabytesV1` (wanted `🟦️.ts`): the grant is judged by the machine's share (half of its memory, below a serving machine's headroom) as long as a wave of at least `minimumWaveProjects` still fits; a busy moment lowers `admitted` (projects per wave) first. Only a machine that cannot run the smallest wave has a grant of its admitted memory (0 MB). `fleetReadV1` takes `grantMegabytes` (default: admitted), `fleetReadStaleGenerationV1` (prefetch) passes it.

## Numbers (`T/🧰️scripts/R4/lab/evaluate.out.txt`, real functions, `lab/evaluate.ts`)

| reading (16 GB machine) | available before / after | grant per project before -> after | per wave before -> after |
|---|---|---|---|
| this Mac live (32 GB, counted as 16 GB) | 4 557 / 14 156 MB | 0 -> 3 512 MB | 0 -> 48 (192) |
| synthetic idle 16 GB Mac | 5 515 / 10 000 | 0 -> 3 512 | 12 -> 48 (192) |
| build running | 4 921 / 6 750 | 0 -> 3 512 | 0 -> 24 (96) |
| busy | 4 062 / 5 593 | 0 -> 3 512 | 0 -> 12 (48) |
| starved | 685 / 1 000 | 0 -> 0 | 0 -> 0 |
| idle 16 GB Linux (15 848 MB, MemAvailable 11 000) | unchanged | 3 298 -> 3 298 | 48 -> 48 |

## Tests (all green live after landing, BUN_RUNTIME_TRANSPILER_CACHE_PATH=0)

- NEW `LIB/🪪️graph-state/🙋️wanted/🧪️tests/🍏️idle-grant` (12): table of 4 macOS `vm_stat` and 4 Linux `MemAvailable` readings against an independent Python computation of the policy; law idle = 3000..3600 MB and 48 per wave (192) on both platforms; busy lowers the wave before the grant, grant 0 only without a wave; the old reading (free + cache) of the idle Mac was under 5.8 GB; "too heavy" = measured above the grant only (edge case equal is light, kind/unknown never heavy).
- NEW `LIB/🕸️graph/🧪️tests/🙋️fleet-read/🍏️macos-reading` (3): through `fleetReadStaleGenerationV1` with a `vm_stat` reading; idle 16 GB Mac prints `it can give about 3512 MB per project ... it can infer 192 of the others (48 per wave)`; busy `96 (24 per wave)` with the same grant; starved `about 0 MB ... 0 (0 per wave)`. Both new files fail on the pre-landing code.
- Updated expectations of the new reading (independent Python parser in the test changed): `🔋️available` 12, `📡️sampling` 11 (fixture), `🏡️resident/🌡️pressure` 7. Neighbours green: wanted 6, read 10, server 11, two-platforms 5, cost 11 + measured-only 3, fleet-read 5 + small-booking 3, capacity 5 + units 14, resident cancel 37, memory available 1.

## What the worker's first plan line must read (idle 16 GB Mac, 10 GB available or more)

`graph: <N> stale projects, 0 taken from the fleet, <K> too heavy for this machine (<names with about X MB>; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for <N-?>`, with K about 12 to 14 (only measured owners above 3 512 MB physical: framework-os-dev, repo, workspace, framework-process, framework-plugin, renderer-wgpu, ...), not 226. With 6.7 GB available the same line reads `96 ... (24 per wave)`, with 5.6 GB `48 (12 per wave)`; `about 0 MB` only below about 5.1 GB available.

## Not verified

- No real 16 GB Mac ran it; the Mac figures are the real code on this machine's `vm_stat` and synthetic 16 GB readings. Inactive anonymous pages need compressing when taken, so a Mac under real memory pressure may be read a little generously; the pressure level, compressor and swap signals of the capacity gate and the memory governor (stops a project above its grant) are unchanged.
- `tsc`, taxonomy and name-registry gates for the two new test folders (`🍏️idle-grant`, `🍏️macos-reading`); Windows (unchanged). The real `repo` build command of the landing rules was not run (no graph command allowed here).
- The worker must restart its service to run the new code (its `build-agent.log` / host log shows the new identity).

Scripts: `T/🧰️scripts/R4/` (`land.py`, `edits.json`, `baseline.json`, `payload.py`, `new/`, `lab/`).
