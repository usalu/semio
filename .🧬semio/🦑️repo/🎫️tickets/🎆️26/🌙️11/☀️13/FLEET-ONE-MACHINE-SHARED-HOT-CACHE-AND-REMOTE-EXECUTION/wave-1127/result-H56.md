# Result H56: fast developer commands on the coordinator

## Measured baseline (07:32, load 45-55, `repo inputs repo-graph-maintenance:test`, own admission memo 2 h old)
Total 17:28. Graph slice: worker start 8 s, image/ledger/lease 14-20 s, memo seed + memo-check ~21 s (journal: 26,467 of 41,758 rows skipped, then `changed 729`: the memo is dropped as a whole at the first stale row), **owner scan 20 -> 341 s (about 300 s)**, owners read/seal 350-520 s, then Nx source capture/served/verify to 1048 s. Journal-vouched row kinds are only path facts (lstat/readdir/exists/file/...); 15,291 of 41,758 rows (resolve 8.1k, analysis 3.5k, global 2k, field 1.6k) are probed every time (second memo-check 26 s even when clean).
After the 08:00 landing (08:05, same command, peers had edited the closure meanwhile): slice again ~460 s (owner scan 72 -> 306 s). NOT a clean measurement; the target (clean narrow < 30 s, suite target with deps < 3 min) is NOT reached and NOT proven.

## Root causes
1. (1)/(2) are one cause: any edit of a row of a closure makes the memo of that selection stale; the 3600-owner closure of `repo:test-<suite>` has no memo of its own at all, so every owner is discovered cold (0.33 owners/s under load = the 43 min of H50). Facts of an owner depend only on owner + targets + inputs, not on the selection, so other selections' memos are valid seeds (each owner revalidates its rows).
2. (3) The slice worker is one process per read by design (`🏡️resident/🏗️composition/🌐️host/👷️slice-pool`): boot + module load 8 s, image open 6 s of ~1000 s. A warm worker would save ~10-15 s only; NOT done (needs a loop in `👷️worker.ts`, small payoff next to the scan).
3. (4) A superseded service never ended while any connection stayed open; a younger generation waited without bound for an older busy one; liveness was pid only.

## Landed 08:00 (before the 08:15 quiet window; landings.md "Wave 1127 H56")
- Handover: descriptor `startMs`, liveness by pid+start time (`processHolderAliveV1`), idle connections end a superseded service after 300 s without traffic, predecessor wait bounded to 60 s (policy keys `supersededIdleConnectionMilliseconds`, `predecessorWaitMilliseconds`). Tests: supersession-drain 5/0 (table + SQL oracle), generation-handover 5/0.
- Sibling seed: `🕸️graph/📥️load/🌱️sibling-seed` + hook in `load/🟦️.ts`; memo publish keeps only owners the closure used. Test 2/0 (CPython model). Admission-memo 14/0; typecheck 17 files 0 errors; dangling-imports 0. Scripts `🧰️scripts/H56/`.
- Unverified live: the `[graph-sibling-seed]` log line / owner-scan time after seeding was not observed (service log of the new generation not located in time); effect on a cold 3600-owner closure NOT measured.

## Next (staged ideas, not landed)
- Vouch `analysis`/`field`/`typescriptText` rows by a clean journal path (needs parser-currency argument) and `resolve` by the clean set of directories it consulted: removes the 15k probes per memo-check.
- Per-owner delta instead of dropping the memo at the first stale row; skip the whole-memo check when the journal reports changed rows inside it.
- Measure again once the seal quiet window is lifted: clean narrow run twice in a row, then `repo run repo:test-build-source-tree` to first test.

# Round 2 (08:20): per-owner memo granularity - finding, nothing landed (seal quiet window)

## The coordinator's premise does not hold for the narrow command
- Service log `[graph-closure] {"owners":1,"kinds":{"selection":1}}` and `[graph-sibling-seed] {"memos":3,"owners":88}`: `repo inputs repo-graph-maintenance:test` admits exactly ONE owner (the "of 600" in the progress line is a wave size, not an owner count). The 300 s owner scan is the rediscovery of that one owner: ~50k observation rows (memo `ea76...`: lstat 16k, resolve 8.1k, readdir 6.2k, exists 4.1k, analysis 3.5k, global 2k, field 1.6k, file 0.1k). A per-owner memo (existing: `ownerFactsMemoV1` + per-owner `hold`, journal-vouched, plus the checkpoint) cannot help: one stale row of the one owner drops all of its facts. The 3600-owner closure of `repo:test-<suite>` is where per-owner granularity matters, and there the new sibling seed (08:00) is the per-owner reuse (each seeded owner revalidates its rows through `hold`); live effect not yet measured.
- The unit that must become incremental is INSIDE an owner's discovery: the per-file analyses (`module-imports` 1757 files, `command-references` 1757 files) and module resolutions (`resolve` 8.1k rows). A discovery that re-runs all of them for one changed file costs ~230 s under load 50.

## Why "journal-vouch the 15k analysis/resolve rows" is not a sound drop-in (measured, `🧰️scripts/H56/measure-vouch.ts`, `measure-stale.ts`)
- `analysis` hold = contract + provider registered with the same revision + nested rows + file stamp/hash (`sourceAnalysisFailureV1`, `👁️observation/🟨️.mjs`). A clean journal path only replaces the last step (a stat in the common case); provider currency and nested rows are not a function of the file. Vouching by path alone would accept a row whose parser changed. A sound version needs an exported `analysisProviderCurrentV1(fact)` (registered + revision equal) combined with the journal verdict; it would save a stat per row, i.e. little.
- `global` rows (owner registry digest, 1967) and `field` rows (package identity, 1626) depend on registry/environment state, not on one path.
- `resolve` rows record (from, specifier) only, not the directories consulted; a new file earlier in the resolution order changes the answer without touching the recorded path, so a clean recorded path proves nothing. They would need the consulted directory set recorded in the row (schema change in the observation, not a journal tweak).
- Harness caveat: a standalone process has no registered analyses, so it reports every analysis/global/field row stale (7,109 of 41,777); my scratch comparison of "vouched vs probed stale set" is therefore NOT a soundness proof and I did not stage a vouching change. The existing journal vouching stays path-facts only (`sameStale true` for it, 26,503 of 41,777 rows skipped in 269 ms vs 4.3 s probing).

## What would make the single-owner case fast (staged as a design, not coded)
1. Per-file analysis memo inside discovery: key `(provider id+revision, file sha256)` -> `{resultSha256, nestedObservations}` persisted next to the admission memo; discovery reuses it for every file whose journal path is clean AND whose provider is current, so one edited file re-analyses one file. Needs profiling first: which discovery operation dominates the 230 s (`activeOwner.operation` in the slice report; use `SEMIO_REPO_PROFILE=1` on a slice read after the quiet window).
2. Record the consulted directories in `resolve` rows (observation schema), then vouch them through the same journal clean set.
3. Fixture to write with (1): N analysed files, one edited -> exactly one analysis recomputed (count via the provider), result equal to a cold discovery and to a CPython model of the import graph.
No code landed in round 2; the seal quiet window is respected.

Post-landing run (08:05, same command) total: total 2692 s.
