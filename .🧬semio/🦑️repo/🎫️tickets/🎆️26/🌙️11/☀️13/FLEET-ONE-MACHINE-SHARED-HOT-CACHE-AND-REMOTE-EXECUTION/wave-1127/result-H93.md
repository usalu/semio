# H93 result: host-wide cache budget (design + scratch prototype)

Design: `W/design-H93-host-cache-budget.md`. Scripts (staged, nothing landed, freeze respected): `T/🧰️scripts/H93/` = `allocator.ts` (pure planner), `oracle.py` (independent Python model with Fractions, corpus generator), `corpus.json` (500 seeded cases, language-agnostic fixture, seed 93), `allocator.test.ts`, `simulate.ts`.

## Proof
- `nice -n 10 bun test allocator.test.ts`: 4 pass / 0 fail, 14453 expects: (1) TS output equals the Python oracle on all 500 cases (budget, evictions, per-purpose targets, shard sizes, actions); (2) invariants: Σ target ≤ budget unless `unmet`, bucket 0 never evicted, used-after ≥ protected, shard sum = target, every shard ≤ image cap, every bucket with a higher age/cost score than the last eviction is evicted whole; (3) ample host evicts nothing; (4) zero free shrinks the budget below physical use. Corpus mix: 331/500 evict, 147 unmet, actions retire/birth/quota/keep/shrink/grow all exercised.
- Simulation: `bun simulate.ts` over the 14 `S/origins/*/partitions.json` (host 228 GiB assumed, free 12/25/40 GiB, two last-use profiles, rebuild costs from the design table).

| host free | origins unmet (budget-short) | Σ declared GiB | Σ budget GiB | Σ target GiB | evicted GiB (stale profile) | births |
|---|---|---|---|---|---|---|
| 12 GiB (< 18.2 reserve) | 6 of 14 | 1996 | 309 | 344 | 131 | 22 |
| 25 GiB | 1 of 14 (m05 Windows, empty) | 1996 | 486 | 476 | 9.5 | 23 |
| 40 GiB | 0 | 1996 | 696 | 578 | 0 | 24 |

m01 at 25 GiB free (content lower bound 40.6 GiB, declared 179.6 GiB): budget 55.8, target 54.6; `r` 96 GiB retire + 2 shards 16+15 GiB, `d` 36 -> 3, `p` 24 -> 12, `g` 12 -> 4, `t` 8 -> 3, `o` 2 -> 0.3 (shrink), `l`/`n` keep; no eviction.

## Unverified / limits
- Host totals/free are NOT in `partitions.json` (host.disk.* sit in binary metric rings, not decoded): 228 GiB total assumed for all workers, free as scenarios. Partition scans are partial lower bounds (d shows 1.8 GiB, real need 29 GiB), last-use histograms are synthetic (two profiles), rebuild costs and the 1.05x/1.3x physical-per-used factors are assumptions; read the table for shape, not sizing.
- No third-party cross-check (no scipy/pulp installed): oracle is an independent Python model, not a library. Greedy optimality is argued, not machine-proven.
- Prototype splits shards evenly; real shards are subtrees (cargo home, components, lanes), needs first-fit-decreasing input. Executors (apply, Windows/Linux) and every live behaviour are design only.
- Eviction-per-cycle limit, EWMA of free, cooldown are in the schema but not in the planner.
