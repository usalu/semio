# GR12: owner records are valid on the ANSWER of their closure rows (2026-11-23, 18:20 to 18:45 Mac clock)

`I` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/`. Lock `lock.graph-closure` (released). Probe `GR12/rederive.mjs`.

## Verdict in five lines
1. Implemented GR10 option 3: a take that meets a STALE or ABSENT closure entry no longer ends the look; it re-asks the recipe in a walk child (`offloadClosureRecipeV1`, never this thread, never in a read parent with a farm), keeps the answer as the stored entry and the existing `provider.verify` compares each recorded answer with the re-derived one. Equal: the record holds and nothing is inferred. Different: `closure-value`, the owner is re-inferred as today.
2. Found a bug that hid half the picture: `takenClosureEntryV1` returned the memoised miss when `judged.get(id) === stamp`, and for an absent entry both sides are `undefined`, so the `closure-entry-absent` branch (GR9) never ran and every absent entry printed `closure-entry-stale (... no moved row found)`. The 24 live "closure" stale looks are therefore a mix of stale and absent entries that the logs could not tell apart. Fixed (`stamp !== undefined` guard).
3. No record content, key or task-hash change; no walk change. Errors, a missing farm, a timeout (`closureReaskMilliseconds` 180 s) or `SEMIO_CLOSURE_REASK=off` fall back to today's miss. Cancellation is rethrown (not stale).
4. Cost: re-deriving a closure answer takes 0.7 to 2.4 s per recipe once the module graph is warm (cold first recipe 37 s of a 1,903-file tool closure under load 40); re-inferring one owner took 150 to 520 s in the last read. One walk serves every owner of the group (in-flight map by entry id), the stored entry serves every later one, a failure is remembered per round.
5. Measurement on a real read is NOT possible yet: the coordinator read that is running started at 18:15 and loaded the old module; no restart (preamble). Before: `98 adopted (1 fetched, 97 present), 42 stale (analysis 1, closure 24), 45 missed, 18 inferred here although listed` = 98/140 = 70 %.

## What changed
- `I/🧭️script-inputs/🟨️.mjs`: `runClosureTakeV1(body, signal, { reask })` (`true` = the walk farm of this process with `after: 0`, or a port function for tests); `takenClosureEntryV1(workspaceRoot, id, recipe)` calls `reaskClosureEntryV1` before it throws the miss (stale counted first; moved-row search only when the re-ask failed); `keepDerivedClosureV1` is the former inline `deriveAndKeep` body (shared by the normal flight and the re-ask); `walkClosureRecipeV1(..., { farmOnly })` returns undefined instead of walking on this thread; `closureReasksV1` (bounded cache) dedupes by entry id, `reaskFailedV1` (per round) stops repeated asks; progress line per re-derivation in EN and DE (`[owners] closure question asked again: recipe X in N ms`); counters `reasked, reaskFailed, reaskEqual, reaskDifferent` in `closureEntryCountsV1()`.
- `I/📦️project-nodes/🟨️.mjs`: the take look passes `{ reask: true }`.
- `I/🧬️schema/🔣️.json`: `closureReaskMilliseconds` (EN/DE texts). `🟨️.d.mts` types.
- Memoisation key: the stored entry itself (its rows carry the current input digests); a later take finds it holding, so N owners of a group pay once, across rounds and processes.

## Tests (`repo test-file`, one at a time)
- NEW `I/🧭️script-inputs/🧪️tests/🔁️answer-validity` 10 pass: Python oracle `🐍️.py` (own import-graph walk) states for 3 scenarios the recorded and current answers; the TS side runs each as stale and as absent entry: equal answers hold with one walk and no owner inference, a dropped import (answer true to false) fails `closure-value`, an added file (false to true) fails; 8 concurrent owners cause 1 walk, 4 later owners 0; a failing or empty walk gives `closure-entry-stale` / `closure-entry-absent`, no option means no walk; an aborted look rejects with the abort reason.
- Still green: take-no-derive, stale-take, moved-row, import-refused, `📏️closure-ceilings` (4), `🖥️machines` (4), adopt-counters, late-closures, read-verdicts, verdict-wait.
- Failing before and after, not caused here: `closure-answer-refresh.test.ts` (regex on project-nodes source), `🫧️take-release` (`const look=job=>{` no longer exists), `🔒️lint` (10 module-level Maps of earlier waves; my first version added an 11th, replaced by a bounded cache).

## Limits and open items
- `digest` queries (owners that call `api.derive`) change whenever any file of the walk moves, so those owners stay stale: only options 1/2 of GR10 (after the play release) help them.
- The take only re-asks where a walk farm can be granted (the first call of a process asks it to start with `after: 0`); in a read parent that has a project farm the existing refusal applies and the miss stands, but there the farm child already derives in its verdict.
- Next read: count `closure question asked again` lines, `closure N` in the stale tally, and the `adoption:` line; expected: closure stale 24 to the owners whose answer really changed, adoption back toward 86 %. I did not verify the equal-answer share on real owner records (needs the `closureInputs` table of the record).
