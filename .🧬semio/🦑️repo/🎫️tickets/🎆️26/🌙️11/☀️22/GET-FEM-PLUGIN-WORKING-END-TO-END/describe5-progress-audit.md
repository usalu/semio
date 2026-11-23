# Describe5 progress audit: the FEM graph read (read-only)

Scope: `repo run … describe fem` log `🗑️generated/pub-describe5.log` (4 441 lines at 00:48, read-only, nothing started, stopped or edited). Times are the `(N s)` of the reading clock, which is monotonic over the whole log (3 759 reading lines, 0 backward steps). Helper outputs written into `🗑️generated/`: `describe5-phases.awk` and `describe5-phase-runs.tsv` (deleted with the generated folder when the ticket closes).

## 1. What the counters mean (decoded from the emitter, not guessed)

The line `Reading the project graph slice (T s): <phase> X of Y.` is produced by `🧠️memory/📋️slice-report/🟨️.mjs` and the step table in `🧠️memory/📋️slice-report/🧬️schema/🔣️.json` (lines 15-17):

| Phase label | X | Y | Source |
|---|---|---|---|
| `checking the file inventory of the tree` (inventory step) | seconds since the inventory last made progress (observations, owner checks, memo rows, admitted owners or active phase changed) | stall bound 600 s | `🟨️.mjs` lines 100-110 |
| `reading the owners (round N)` | seconds the owners round has run | 3 600 s bound | `describeSliceReadStepV1`, lines 84-99 |
| `checking that the observed source facts still hold (round N)` | seconds the step has run | 600 s bound | same |
| `taken` / `taken (pass N)` | owners decided in the ledger (taken from store, fleet or inferred) | owners total of this pass (434) | `startSliceProgressV1`, lines 123-148 |
| `owners done (pass N)` | ledger owners done when the pass ends | owners total (434) | lines 138-141 |

So `X of 600` on inventory lines is NOT a count of work to do. It is a stall counter that resets on progress. `X of 3600` on owner lines is elapsed seconds, not owners. Only the `taken` and `owners done` lines count owners. Values above the bound (e.g. `1 205 of 600`) mean the step is past its expected bound, not that it is finished.

## 2. Ordered phase sequence (start-end in reading seconds)

| # | Phase | Start-end (s) | Counter at end |
|---|---|---|---|
| 0 | Host grant (6 986 MB), no named step yet | 0-755 | none named, 750 s bare |
| 1 | Inventory #1 (cold memo, owner checks climb to 870) | 755-14 022 (13 267 s, interleaved with 37 `graph service holds` lines) | `166 of 600` (stall 166 s), distinct observations 236 017, memo rows 573 440 |
| 2 | Owners round 1, taken pass 1 | 14 027-16 363 (2 336 s) | `owners done 318 of 434` at 16 309 (116 owners not done in this pass) |
| 3 | Inventory #2 (after round 1) | 16 395-16 799 (404 s) | `137 of 600` |
| 4 | Facts check round 1 | 16 804-17 001 (197 s) | `83 of 600` |
| 5 | Owners round 2, taken pass 2 | 17 020-18 628 (1 608 s) | `owners done (pass 2) 434 of 434` at 18 503 |
| 6 | Inventory #3 (after round 2) | 18 638-19 035 (397 s) | `5 of 600` |
| 7 | Owners round 3, taken pass 3 | 19 037-19 868 (831 s) | `owners done (pass 3) 434 of 434` at 19 154, then ~700 s more inside the round |
| 8 | Inventory #4 (after round 3) | 19 880-still running | stalled 19 880 to about 21 490 (1 610 s, distinct observations 0, memo rows flat at 102 336, stall counter reached 860 of 600); resumed at 21 490 (distinct 1 then 2 789 by 21 537) |

Counts of loop phases across the log: 2 456 inventory lines, 745 owners-round lines, 123 `taken (pass N)`, 6 `owners done`, 37 `graph service holds`. Reset check: the reading clock never goes backwards. The `graph service holds this request (N s)` line carries a separate service clock that restarts (2 575 then 129 then 1 618 ...). That is a service-side re-hold, not a restart of the reader. There are no `recycled`, `restarted`, `handover`, `lease`, `queued` or `[graph-retry]`/`[graph-repair]` JSON lines in this log.

Repeat or convergence: this is a bounded multi-round process, not an identical restart. Round 1 covered 318 of 434 owners, round 2 and round 3 each re-took 434 owners in their pass (pass 2 cost about 1 470 s, pass 3 about 110 s because the memo answered). The inventory step is the expensive part: inventory #1 ran 3.7 h alone. The pass counter restarts at 0 each round, so `taken 0 of 434` lines are a new pass, not a loop.

## 3. What drives the number of rounds (code)

Search note: the phase text lives in `🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️build\…`, which is under `🧰️framework`, so the search scope was respected. The text `graph service holds this request` does not exist in any `.rs`/`.ts` under `🧰️framework` (checked); its source was not found.

- `🕸️graph/📥️load/🟦️.ts`, `settledInventory` (lines 226-246): retries the inventory step on membership change or moved paths. It gives up after 2 identical repeat signatures, or when a membership error has no paths and round >= `repairRounds`. Wait between retries `min(round,8) * 250 ms`.
- Same file, outer `for(;;)` (lines 279-303): each pass = owners round (`discoverOnce`) -> `settledInventory(true)` -> facts hold (`VERIFY`). It breaks when facts hold AND `membershipMoved` is false AND `admissionMoved` is false. Otherwise it does a repair: `rounds++` (line 278, `followed`) and loops, as long as `rounds < GRAPH_LIMITS_V1.repairRounds`.
- Failure path (lines 282-288) also increments `rounds` and throws once `rounds >= repairRounds`.
- Limit: `🕸️graph/🧬️schema/🔣️.json` line 15, `repairRounds: 4`. So at most 4 repairs, i.e. up to 5 owner passes (labels round 1-5), then it seals with the stale rows it has (`sealVerdictV1`, line 304).
- Trigger: `membershipMoved = current.digest !== basis.digest` after the post-owners inventory. The worktree is live and replicated from 50+ machines (AGENTS.md), so the file set changing during a read is the likely cause of the repairs (this is an inference; the log does not print the `[graph-repair]` JSON).

So the number of rounds = (number of times the tree membership or admission changed during the read), capped at 4 repairs.

## 4. Estimate and stuck check

- Not stuck now. The stall 19 880-21 490 was 2.7x the 600 s bound and the reader did recover; the log is still moving (21 752 s at 00:48).
- Remaining steps: finish inventory #4 (unknown, currently progressing), facts check (about 200 s), then either seal (bound 600 s) or repair again.
- Worst case, both remaining repairs fire: ~1 600 s owners + ~400 s inventory + ~200 s facts per round, twice, plus seal, about 1.5 h from now.
- Typical: round 3 converged quickly on memo answers, so a seal within 20-40 min is plausible if the tree stops changing.
- Watch for: inventory stall counter above ~1 200 with distinct observations flat again (that is the 2nd stall); owners-round lines whose X passes 3 600 (bound exceeded).

Answers in one line each:
1. Bounded multi-round, 3 owner rounds so far (repeats ledgers, but round 3 is cheaper); the 37 holds lines are service-side, not restarts.
2. Rounds = tree-membership changes during the read, capped by `repairRounds = 4` (`🕸️graph/📥️load/🟦️.ts` 279-303, `🕸️graph/🧬️schema/🔣️.json` 15).
3. Remaining: ~20-40 min typical, up to ~1.5 h worst case; not stuck, but inventory #4 stalled for 27 min before resuming.
