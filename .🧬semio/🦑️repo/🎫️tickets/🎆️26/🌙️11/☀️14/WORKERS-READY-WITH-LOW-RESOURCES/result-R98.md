# R98: the fleet waited on itself; the reader's inference is released early and pool claims are 60 s leases

Status: LANDED 2026-11-16 22:00 to 22:03 UTC (lock `landing.lock`, owner claude-R98, released). Scripts: `T/🧰️scripts/R98/` (`land.py`, `restore.py`, `window.sh`, `runall.sh`, the two new test files). T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`.

## Why `wanted` owners were not inferred (`here 0, running 0`)
Evidence: `slice-memory.json` of g1-m01, m02, m08 (`sources.owners`), code of `project-nodes/🟨️.mjs`.
1. The inference workers (`loop`) start with `await resolved`; `resolved` was released only after `runWarmBesideTakeV1(warmOwners, resolveOwners)` returned, which awaits BOTH the take pass and the warm pass.
2. The take pass is bounded at 900 s (`passBoundMilliseconds`); a look costs 3 to 8 s on a loaded host (m02: 744 looks in 1758 s, average 8.5 s, `lookup.settle` 3.6 million ms in total on m01, 21 217 stale closure entries). After the bound the rest is "wanted" (m02: wanted 4745, taken 29).
3. The warm pass takes all 4773 stored owners (every owner has a stale local record), 1 to 2 lanes (memory plan of a 16 GB host), about 10 s per owner (m02: 166 done after 1758 s; owners of 4.4 GB fail inside a child). That is hours, and nothing could start before it ended. So `missed` stayed 0, `computed` 0, `farm: null` on every worker.
4. Nobody produced for the fleet either: the owner pool is published by `ownerPool.miss(...)`, which a reader calls after its pass or from `publishRunning` once inferences are missing. With workers gated, the machine published late, its own pullers `yield` to the reader's grants (m07 `pool.json`: reason `yielding`, 541 held, running 0), and every other machine was in the same state: each waited for the fleet, the fleet was the same waiting machines.
5. Claims of the pool lived `claimTtlMilliseconds` = 300 s after the last renewal, so a dead claimer held its owners for 5 minutes (heartbeat is 8 s).
The 40-wanted line of the server (48 owners, warm pass off because under 64 owners) is the snapshot printed right after the pass, before the released workers had started; the same code path.

Not reproduced live: a real small-slice read on g1-server only attached to the graph build another command was running ("attached to the graph build of another command", generation 246), at load average 30, so no cold slice could be observed. The cause above is read from the stored counters of three workers and the code, and proven by the laws below.

## Change
- `📥️take/🟨️.mjs` `runTakeBesideInferV1`: starts the take pass and the warm pass, opens the inference (`release`) when the take pass ends or `releaseMilliseconds` (30 s, schema `Policy`) after the start, stops the warm pass at that moment (its farm children are the ones the inference needs), tells the pass to stop when the inference is over, awaits both at the end, rethrows a failing pass, a failing inference wins.
- `project-nodes/🟨️.mjs`: uses it instead of `runWarmBesideTakeV1` (warm and pass get abort signals `warmStop`, `passStop`). Unlooked owners are inferred by the workers through the normal fleet gate (cache hit, claim, compute, publish per owner), looks in flight are awaited as before; the farm engages by its existing rule (a quarter of the seen owners missing) and sizes children by cores and memory.
- `🌐️owner-pool/🧬️schema/🔣️.json`: `claimTtlMilliseconds` 300000 to 60000 (heartbeat 8 s unchanged, so 7 renewals per lease).
- Dead code left: `runWarmBesideTakeV1` is no longer used by the reader (still exported and tested in `🔱️warm`).

## Proof
| law | test | before | after |
|---|---|---|---|
| inference starts when the take pass ends though the warm pass never ends; warm stopped then | `📥️take/🧪️tests/🚀️release` | red (function missing) | 4 pass |
| a slow pass releases after the declared time | same | red | pass |
| two machines infer 40 owners exactly once, both start within 1 s, makespan under 0.8 x serial (virtual clock, real claim table, real `pullOwnerPoolV1`) | `📥️take/🧪️tests/🤝️two-machines` | pass (the pull protocol was sound) | pass |
| a dying claimer blocks for at most the lease, the others infer its owners once | same | red (ttl 300000) | pass |
| lease law on the real claim table: renewed it outlives the lease time, left alone it lapses | same | red | pass |
Regression: take 7, owner-pool 29, project-nodes take 11, inferred 3, away 1, slow-last 5, stopped-wave 5, ready-context 2, both standing preflights (2 and 10): green. `parent-budget` (1 red), `governed-dispatch` (2 red), `too-heavy` (1 red) were red before the landing with identical counts (baseline run 21:53 to 21:57, peers' uncommitted edits in the same module).

## Not verified / open
- No live fleet effect measured yet: `taken` and `here` counters on m01 to m10 after the machines pick up the new code are the proof still to take.
- The reader's worker still WAITS inside `decideGraphEntryV1` (up to 900 s) when a peer holds a live claim on the owner it picked; the waiting is on a working peer and ends at the lease, but a pull-style skip of held owners in the reader loop would keep the worker busy (not done).
- Parent workers are in-process (`min(8, cores)`, one event loop); cores are used only through the farm children. Per-owner footprint for lane sizing is R97's.
- Warm pass is now only effective for the first 30 s of a read; stale closures are settled by the looks and the inference themselves.
- Linux and Windows not run.
