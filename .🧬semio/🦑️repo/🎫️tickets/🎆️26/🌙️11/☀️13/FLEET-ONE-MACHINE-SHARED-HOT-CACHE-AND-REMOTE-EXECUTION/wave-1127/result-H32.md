# Wave 1127 H32: closures `optional-new-reference` (possible cache-retention data loss)

## Verdict
No live deletion of referenced data. A leaf is only unlinked when no live record and no unsettled retirement intent references it; the failure was an ordering defect in the optional-group retirement loop, and it made the maintenance pass evict the optional group that a writer had just republished (together with its reference), after which the orphan leaf was legitimately unreferenced.

## Diagnosis (measured, instrumented runs of the one scenario)
- Test sequence: invalidate optional group `inference/mr` (intent retained), a writer publishes a new pack in a fresh `inference/mr` referencing the orphan leaf, then `recoverBuildGraphImageHeadroomHeldV1` loops `retireBuildGraphOptionalGroupsHeadroomV1`.
- Within one call the fresh group is protected (`protectedSources`, filled while the older intent is still settling) so it is not invalidated. That set was a local of the call. The call returns `partial` when the retirement window is due; the next call started with an empty set, found no intent, and invalidated the fresh group (`mr` gone in the log, a new intent with the group's references), the intent then settled, its retained references vanished and `retireBuildGraphClosureHeadroomV1` removed the orphan: operation succeeded instead of refusing.
- So the outcome depended on timing (window due or not) and the protection was inconsistent between passes. The reference-recheck under the lease (`recheck(proof)` directory stamps before every unlink) already holds: a record published between capture and unlink changes the directory stamp and refuses (scenario `new-reference` green).

## Fix (landed 03:00-03:04, graph lock, typecheck 0 errors)
`closures/🟦️.ts`: `protectedGroupsV1` WeakMap keyed by the maintenance custody; `retireBuildGraphOptionalGroupsHeadroomV1` reuses and fills the same set across all partial passes of one exclusive maintenance. Patch script `T/🧰️scripts/H32/apply-protected-persist.py`, previous file `T/🧰️scripts/H32/closures.before.ts`.
Result: the scenario now ends with the expected `Authenticated graph cache cannot fund required physical headroom` and the orphan retained.

## Law test (Python oracle)
`closures/🧪️tests/🛡️never-deletes-referenced/🟦️.ts` + `🐍️.py`: 60 seeded random traces (publish record, evict record = references move to an intent, settle intent, retire pass with the product `planBuildGraphClosureRetirementV1`); the Python model replays the trace, recomputes the greedy selection independently and flags any selected referenced leaf, any selection mismatch and any dangling record or intent. 1 pass, 1452 expects, 5 s.

## Verification
- Scenario run (env `SEMIO_ISOLATED_BUILD_TEST=...optional-new-reference`, 114-180 s under load): refusal observed, orphan retained.
- Whole closures file (T1): 27 pass, 0 fail, 1074 s (was 26/1).
- Debug edits in the test were reverted (test file identical to its pre-lane content); claim in `claims-tests.md`.

## Unverified / notes
- The first of my runs (before the fix) showed a non-partial pass where the orphan also vanished with the pack intact; that path was not reproduced after the fix (timing). If the full-file run below still reds, that path is the remaining suspect (fresh unindexed pack handling in `freshClosures`).
