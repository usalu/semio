# F39: split closures `🟦️.ts`, fix `pack-unproven`

Folder `C = …/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/` ; closures = `C🧾️closures/`. Lock `caching-storage`, scripts `T/🧰️scripts/F39/{split,harness}.py` (anchored, atomic temp+rename). Landing record in `T/landings.md`.

## 1. Split (landed 19:45)
- `🟦️.ts` 65243 B -> 56853 B (8.7 KB below the 64 KiB pin read).
- New `🏷️names/🟦️.ts` (role, spill, temporaryBase, orphanTemporary, processAlive, liveSpillDirectory, retiredSpill; trailing `export{}` so function-text extraction keeps working) and `🪟️policy/🟦️.ts` (plan, classify, retirement window, capture budget + types). Parent imports them, re-exports the four public functions explicitly, and both members are pinned in `closureImplementationURLsV1`.
- Harnesses reading the parent text now read the members (concatenated where both are needed): in-flight-temporary, row-log-spill, owner-peaks-spill, fleet-marker, formats, census (two places; also the missing `directoryChangeV1` port, F26 round 4 drift), cardinality, members tests, resolve.
- Bug I introduced and fixed within minutes: the parent still calls plan/classify/window internally; first landing re-exported only (`ReferenceError`, groups 2/11). Fixed with an import; groups then 12/13.
- Tests: in-flight 6/0, cardinality 1/0, census 3/0, row-log-spill 7/0, owner-peaks 6/0, fleet-marker 3/0, formats 4/0, window 4/0, members 1 file ok, resolve ok, batch ok, groups 12/1 (the 1 = "Native receiver physical envelope changed" after 110 s, native-build churn by peers, not the split). dangling-imports 0.

## 2. `pack-unproven` (real product bug under load, plus test shape)
Evidence by [DEBUG] logging in the test (file based, removed): the scenario called `retireBuildGraphClosureProofV1` directly, which returns 0/0 without throwing, but the scenario requires the headroom funding refusal ("Authenticated graph cache cannot fund required physical headroom", thrown only by `recoverBuildGraphImageHeadroomHeldV1`). Same shape as `optional-new-reference`, which already goes through recovery.
- Test: `pack-unproven` now takes the recovery branch and is refusal-ready; body/control budgets use `stretchedBudgetMillisecondsV1`; the optional-call counter is `>= 1`.
- Product bug found by the same logging: `retireBuildGraphOptionalGroupsHeadroomV1` returns `{retiredBytes,retiredFiles,partial:true}` when its retirement window expires (always, under load), but `📏️headroom/🧬️schema` `OptionalRetirement` is closed (`additionalProperties:false`) so recovery threw "Closed optional graph retirement result required" (also failed `optional-release`, `optional-new-reference` in a loaded run). Fixed schema-first: `partial: {const:true}` in the schema; `recoverBuildGraphImageHeadroomHeldV1` loops the optional callback while it reports `partial` and headroom is still short (each call completes at least one step, so it terminates).

## Not verified / remaining
See the end of this file for the final full-file run.
