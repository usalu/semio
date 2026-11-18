# R43: cold owner wave throughput (LANDED twice, NOT yet observed on a read)

Scripts: T/🧰️scripts/R43 (edits.py, edits_fix.py, land.py re-pointed to /Users/ueli/Documents/semio, overlay.py, new/ tests). Both landings finished before the 14:50Z tree move; verified intact at the new path (all edits and 3 new test files present, no LAND-OK, no lock).

## Findings (evidence, no new read was possible after landing)
1. Attribution: parent main thread of read 48503 was 98% CPU (not waiting) in the heavy phase; sync stretches up to 40 s (loop lag) while `workspace` (root cargo) ran 190+ s in-parent; at most 3 owners running (admission). `git` children at 40-130% CPU belong to the replica daemon (pid 48370 `boot.mjs run`), not to the read: no per-owner git spawn exists in the graph read.
2. Reservation: owner-peaks.json held `project` 8x2048 MB, `language:cargo` 7x2048+666, i.e. samples clamped at the ceiling (shared-garbage growth); `language:rust` 1382 MB stuck because shared samples may only raise. Hence 2048 MB for a font crate, 3 owners at once.
3. Start phase: manifest holds 8129 entries but 0 vouches (probe: scratch/R43/probe.mjs). 8123 of 8129 closures read config/bare-resolve parts, and the stale summary of a continuously edited tree always has configs=true/modules=true, so `vouchOf` refuses every closure (hard parts). R41 seals per chunk but nothing can be sealed. Round 1 derives (7 min), round 2 derives nothing yet still takes 4 min (30 ms/closure, loop lag 1-3 s, no starved stacks): cause unproven, now timed.
## Landed
- Governor: samples at the ceiling count as the initial guess (256 MB); a shared sample can raise a kind only up to max(solo peak, 256 MB); 4 windows (32) of low shares decay a stuck high peak; floor 64 MB (`floorOwnerMegabytes`, schema+fixture). Tests `🧠️memory/🧪️tests/🪙️estimate` (red before, green after), one old test fixture 3000->2000 MB.
- Owners recorded >=120 s run after the light ones (`orderOwnerJobsV1`, test 🐌️slow-last).
- Trace: `owner <name> wall=... queue/authority/farm/inline/observe/refs/observations/release/other`, rollup `owners n=50 ... running.avg` and `settle closures=... vouched= slow= summary= ask/parts/invalidate/rederive/support/seal/flush/save` (tests ⏲️clocks).
- Neighbour suites identical before/after; only pre-existing reds (process tree reader, too-heavy yield points, derived-nodes 7).
## Not verified
No read ran with the new code (R42 makes the server attach to a running read; then the tree moved). Owners/min, concurrency, start duration and peak vs grant are UNMEASURED. First thing to read in the next trace: `owners n=` and `settle closures=` lines. Start-phase fix NOT done (needs those numbers; candidate: stamp stale config files instead of treating configs/modules as hard).
