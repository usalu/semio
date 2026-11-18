# R107: cost of one take lookup, measured and LANDED 2026-11-17 04:00-04:05 UTC (lock under 5 min, GREEN)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R107/` (`look.ts` harness, `patches.mjs` anchored edits, `land.mjs`, `restore.mjs`, `window.sh`, `runall.sh`, `compare.py`, three test sources). The harness reads the real stored records and closure store of this machine (wave `c92b6f8cf8b1f0e1`) in a child with `performance.now()`/cpuUsage spans per step and applies the patches in memory through a Bun plugin, so the live tree was not touched for measuring.

## Where the time goes (per owner, CPU, same owners, one thread)
- read of the record: about 1 ms. refs (`resolveClosureRefsV1`): about 100-250 ms. settle (`settleClosureRowsV1`, i.e. entries load + verify): 500-1000 ms, the rest.
- The 13 s of the workers are NOT re-hashing of shared files (row verdicts and layer verdicts are already memoized per round: 59 000 hits vs 5 700 checks over 20 owners). They are: (1) every owner re-loads and re-verifies the closure entries it found STALE: a stale verdict was never remembered (field: 13 084 stale verdicts over 582 looks = 22 per look, each a read+brotli+parse of the entry file and a layer walk); (2) `unionMembers` built `new Set(wholeList)` for every layer of every loaded entry (about 13 % of samples); (3) per layer `idsSync` (readFileSync + sha256 + copy) and `existsSync` of the layer file: about 30 % of samples now, per distinct layer, not shareable across owners; (4) forced `gc` (duty limited) 4 %.
- No whole-document JSON round trip in the take path (checked: `store.load` parses each entry once; `localBundleV1` is only in the fleet intake).

## Before / after (harness, owners that have a stored record, ce store of this machine)
| run | owners | CPU per owner | owners/s | stale loads |
|---|---|---|---|---|
| 20 owners, before | 20 | 783 ms | 1.31 | 366 |
| 20 owners, stale memo | 20 | 722 ms | 1.21 | 260 |
| 20 owners, stale memo + members set | 20 | 507 ms | 1.91 | 260 |
| 55 owners (all in the wave with stored entries), before | 55 | 517 ms | 1.53 | 852 |
| 55 owners, both | 55 | 301 ms | 2.33 | 628 |
Per-step (55 owners): settle CPU 23.5 s -> 11.2 s (427 -> 204 ms per owner); refs about 90-95 ms per owner, unchanged. 200 owners could not be run: only 55 owners of the wave have all closure entries stored here.
Target under 200 ms NOT reached: 300 ms per owner remain (settle 204 + refs 95). Most of these owners are misses (47 of 55 have stale entries on this machine); only one took cleanly, so the clean-owner figure is the R87 one (75-90 ms) and is unmeasured again.

## Staged changes (all tests written first and seen red, then green through the in-memory patches)
1. `🧭️script-inputs/🟨️.mjs` `takenClosureEntryV1`: a stale verdict is remembered per round with the stamp of the stored entry (`staleTakesV1`); a later look of the same round that meets the same stamp is a miss without load. Freshness unchanged (another stamp or round judges anew). Test `🧪️tests/🪦️stale-take` (3 takes: 1 stale verdict, new round: judged again). Red: 3 verdicts; green: 1.
2. `💾️entries/🟨️.mjs` `unionMembers`: one set per member list (WeakMap, rebuilt only when the length differs). Test `💾️entries/🧪️tests/🧰️members-set`: red 1527 sets built, green at most 40 (one per layer).
3. `📦️project-nodes/🟨️.mjs` `warmOwners.describe`: `knownMegabytes:Math.round(enforcement.known?.(name)??0)`. Unit confirmed: `known` returns `row.megabytes * footprintFactor` (megabytes), and `knownOf` in the pool reads `payload.knownMegabytes` as megabytes; the extra `/1048576` made every owner 0. Test `📦️project-nodes/🧪️tests/🧮️known-unit` (source assertions, as the neighbouring tests do).

## Landing state
First attempt 01:30:01-01:31:52 UTC: all patched files applied, battery run, RED only because my own known-unit test sliced too little source (fixed in the ticket copy); everything restored by `restore.mjs` (files back to the pre-landing hashes, test dirs removed). Second window attempt at 01:34 found the lock held by R106. Then the coordinator froze landings. Nothing of R107 is in the tree. To land: `🧰️scripts/R107/window.sh 0,1,2` inside a window (it takes the lock, applies, runs the 24-file battery, compares with `before.txt` of the scratchpad, restores on red, releases). Baseline reds that also fail before: governed-dispatch 2, too-heavy 1, parent-budget 1, take-release 2.

## Not done / unverified
- Off-event-loop lookups (worker threads/children) not built; the remaining settle cost (ids files, layer existence stats) is per distinct layer and needs either a persistent per-entry verdict or children.
- Numbers come from this machine's store (mostly stale entries, load average 33), not from a worker; the field 13 s per look was not reproduced (0.3-0.8 s here), so the field gain is inferred from the stale-verdict count, not measured.
- The in-round memo assumes the take looks of a pass share one round (layer verdict hits suggest they do); not checked live.
