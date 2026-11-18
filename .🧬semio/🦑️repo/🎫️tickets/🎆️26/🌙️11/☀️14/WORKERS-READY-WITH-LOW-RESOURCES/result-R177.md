# R177: silence between "owners done" and the plan answer of a selected read (living report)

Written 2026-11-18 from 03:05 UTC. Scripts: `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R177/` (`land.sh`, `land.tsv`, `preload.ts`, `battery.sh`).

## 1. Finding: what the read does after the owner pass, and what each wait says

Path of a selected read (all in `LIB/🏗️build/🕸️graph/📥️load/🟦️.ts` unless named): `loadBuildGraphUnpinnedV1` (`for(;;)`, R167) -> `withBuildGraphStateV1` (custody door, R139) -> `selectedThroughFleetV1` (R171) -> `loadBuildGraphAttemptV1`: `settledInventory` -> receipt check -> repeat { `discoverOnce` (owner pass) -> `settledInventory` -> `buildGraphSourceFactsHoldV1` over every observed row } -> `[graph-seal]` -> serialize, `portableInputs`, write, rename.

| Phase | Progress line before | Bound | Typed failure |
|---|---|---|---|
| owner pass (`taken N of M`) | yes, only when `done` changes | stall watch 5 min x speed factor (`graph.no-progress`), heavy wait: 30 min without progress / 5 min with no live producer / 6 h (`awaitWantedV1`) | yes, EN/DE |
| inventory re-settle, facts hold over all observed rows, repair rounds, seal, publication | NONE: the line stays `owners done 590 of 590` (the phase of the last event) | `repairRounds` 4, then `unrefreshed-facts` (exit 75) | exit 75 with the stale paths, EN/DE |
| `for(;;)` of R167 | prints `[graph-repair] registry adopted` on stderr at every turn, cannot spin silently; it turns only when the owner registry changed | none needed (each turn is a registry change) | none |
| R171 exchange: look, claim wait, install, publish | stderr `waiting` line only; status line unchanged | wait 25 min (`waitMaximumMilliseconds`), claim ttl 30 s, heartbeat 8 s | falls back to reading here, never fails |
| R164 fleet waits (`waitHeavy`) | `[owners]` lines only | see above | `refuseHeavy` |
| R170 lease waits, R158 retirement | own lines/typed refusals (unchanged here) | 30 min x speed factor | typed |

Evidence of the silence (g1-m10, `ready-log-workspace.txt`, read restarted 02:53): first owner round ends at 1227 s, the next line is `taken 8 of 590` at 1341 s (114 s without a line: inventory + facts hold of ~590 owners under load 50), that round ends 1361 s, the read ends with exit 75 at ~1469 s (108 s more without a line). The final cause is NOT a hang: `component-targets/🟨️.mjs` was edited at 03:01:15 UTC by a peer (`git status`: modified, one line); the second round could not refresh a fact about code the process itself had loaded, so the R154 rule `unrefreshed-facts` ended the read with exit 75 and `code-stale` handling restarted it. That path is by design and works; what was wrong is that 220 of the last 260 s were silent and nothing said what the read was doing or for how long.

## 2. Cause of g1-m01 (590 owners, `present 583, fetched 3, here 3, stale 1, running 0`)
The log of that run is gone (restarted at 03:03), so this is derived from the code, two defects found, both fixed:
1. `createTakeLedgerV1.mark` let a late look of the take pass overwrite a decided owner: the look of the take pass runs beside the inference of the same owner, an owner inferred here (`inferred`) could be marked `stale` afterwards and stay `stale` for ever (`done` can never reach `total`, line `taken 589 of 590`). Now present, fetched and inferred are final.
2. An owner that is not done, not running, not in `never` and not parked had no rescuer once the loops had passed its index. New `sweepOrphans` in `inferAll` (project-nodes): before the wait for the fleet, every owner that is neither done (ledger), nor taken, running, in flight, too heavy, set aside nor skipped is requeued and read at once, with a bilingual line and the step `orphan-sweep`.
A stale owner that IS heavy for this machine (`never`) goes to `waitHeavy`; that wait now carries the named step `heavy-wait` (who, bound 30 min) and keeps its typed refusal.

## 3. Fix: every phase after the owner pass is a declared step (law)
`sliceReadStepV1(id, run, detail)` in `🧠️memory/📋️slice-report` (schema `Steps`: id, EN, DE, bound; `idleMilliseconds` 120000). While a step runs the heartbeat of the slice worker (every 5 s) emits `{phase: "<EN> / <DE>", done: seconds run, total: bound seconds}` so the status line reads e.g. `Reading the project graph slice (1300 s): checking that the observed source facts still hold (round 1) / Prüfe, ob die beobachteten Quellfakten noch gelten (Runde 1) 41 of 600.`; past the bound the text adds `(beyond the expected 600 s / über den erwarteten 600 s)` at every tick. Inside the owner round the `taken N of M` lines keep running while the count moves, the step line replaces them while it stands still. Steps wired: `inventory`, `receipt-check`, `owners-round`, `facts-hold` (two sites), `seal` (three sites), `exchange-look`, `exchange-wait` (names the holder `who`, bound = the wait limit), `exchange-install`, `exchange-publish`, `heavy-wait`, `orphan-sweep`. A static law reads the three sources and fails when a used step is undeclared or one of the eleven is no longer used.

## 4. R171 claim (task 3)
The claim was already a lease (ttl 30 s, renew 8 s, takeover at once on `lapsed`), but the renewal was a bare timer: a claimant whose read is stuck or waiting renewed it for ever and every waiter waited up to 25 min. Now the renewal runs only while `sliceReadActiveV1()` (a step began or ended, or the owners ledger/finished/taken/CPU of the process moved within 120 s). An idle claimant stops renewing, the lease lapses after 30 s and the first waiter takes over (`taken-over`). NOT done: "a machine that already has all owners present never waits for anyone": the owner census is only known after admission, which is after the claim; it needs the exchange to ask the census of the owner records for the scope's owners before it waits. Proposed next step, not built.

## 5. Proof
Tests (new, all red on the old files, green staged): `slice-report/🧪️tests/🪜️read-steps` (6), `selected-exchange/🧪️tests/🫀️idle-claim` (3: idle holder taken over before it ends, a progressing holder keeps its claim, every await is a step), `project-nodes/📥️take/🧪️tests/🔒️decided` (4). Changed tests: `stale-facts`, `seal-under-churn`, `repair-rounds` (one binding added). Staged battery of 21 files equals the baseline (candidate 1/2 is the known baseline red; `no-progress` has a real-CPU-child test that is flaky under load 50 in base and staged alike). Import smoke of staged `load`, `project-nodes`, `attempt` modules passes. Staged runs used a bun preload with `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`.

## 6. NOT verified
No real slice-lab run (R161 held the lab, load average 50); no real two-machine claim run; the sweep is proven by a source law, not by a read that produces an orphan (no reproduction of g1-m01 exists since its log was overwritten).

## 7. Landed and watched (03:30:02 to 03:30:44 UTC, hold 42 s, entry in landings.md)
Nine test files 0 fail under the lock; no bun transpiler cache entry names the scratch path. Running reads (g1-m01, m03, m08, m10) keep the old code until their next restart; g1-m07 started with the new code at 03:3x and is the only live proof so far.

g1-m07, first read with the steps (seconds since read start, from `ready-log-workspace.txt`): inventory 96-206 (the first inventory check takes 110 s under load 50), owners round 1: 208-348 (`taken` lines while the count moves, the step line while it stands still), inventory re-check 350-447 (97 s, before: silent), observed-facts check 449-473 (24 s, before: silent), sealing and publishing 475-477, then `Planning build tool storage` at ~480 s. From the end of the owner pass to the sealed graph: 130 s, every second named in English and German with run seconds and bound. First `"ready": true`: not yet (g1-m07 stage `workspace` still `code-stale` at 03:41, planning storage started); next failure: none seen on m07 so far.

Live data on g1-m01 before the restart (old code, 1545 s, `slice-memory.json`): `ledger {total 591, fetched 42, here 548, stale 1, running 0, wanted 1, done 590}`, `never [norm-js, s-dev-composition]`, `cursor 591`, `farm null`, nothing active, pool `idle, nothing-fits: the next child needs 1024 MB but only 407 MB are free above the 2048 MB floor`. So that shape is the heavy wait (`waitHeavy`): one owner is stale in the fleet, this machine may not run it (`never`), and no machine of the fleet can start a child for it (every pool of the 16 GB machines says `nothing-fits`). It is a capacity wait, bounded by 30 min without progress (then `refuseHeavy`), now named as the step `heavy-wait`. The ledger race and the orphan sweep are still real defects found on the way, but they are not what held g1-m01. Open question for the coordinator: a stale heavy owner needs one machine with a free 1 GB child; the pools reserve 2457 MB for own work on every 16 GB Mini, so nobody can take it.

g1-m10 (old code): `owners done 589 of 591` (2 are `skipOwner`, not missing) for 1450 s, then the first R154 exit 75 caused by the 03:01 peer edit of `component-targets/🟨️.mjs`, as in section 1.

Next to watch: the first read on m01/m03/m08/m10 with the new code, the line `heavy-wait` on m01, and the plan answer of m07.
