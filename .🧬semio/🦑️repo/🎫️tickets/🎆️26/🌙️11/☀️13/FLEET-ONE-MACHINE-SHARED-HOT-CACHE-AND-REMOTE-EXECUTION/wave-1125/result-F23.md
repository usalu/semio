# F23 result: stale closure-graph fact and exit 75 restarts

## Finding (what is proven)
- The fatal text comes from `🕸️graph/📥️load/🟦️.ts` (`[graph-retry] unrefreshed-facts`): after one repair round the stale rows of the final facts-hold had the same path AND the same recorded fact as the round before, so the attempt ends as BUILD_AUTHORITY_CHANGED (exit 75). The slice pool counted three identical causes and refused the plan.
- Defect 1 (memo never invalidated): the repair loop evicted only the taken memo (`reuse.boundary` by moved paths); `ownerFactsMemoV1` was never evicted by the stale rows (only by watch events and hold failures). An owner answered from that memo, whose memo-check is a within-round hold (shares verdicts, `observationsHoldWithinRoundV1`) while the seal holds outside any round, returns the same row again. Fixed: both repair branches forget the owner facts of `moved`.
- Defect 2 (fatal on first repeat): now one targeted refresh (evict the rows, follow, re-infer) before the fatal; and the pool starts the load once more before `slice-worker-failing` when the cause is the stale-facts text.
- Probe `🧊️global-row-refresh` (real modules): a `global closure-graph` row holds on its fresh answer, fails after the closure roster changes, a fresh answer holds again, with the script cache bound and after `resetClosureEntriesV1`. So the row judge and answer agree in one process; an edit that leaves the roster unchanged keeps the row (the answer is the roster view), by design.

## Not proven
- Why the same recorded fact survived three fresh workers on m02 (a persisted layer, the `oc` checkpoint or admission memo, is the remaining suspect; the checkpoint has no forget-by-path). Not reproduced; no live slice run was made (load 34-43, seal window). Edits outside the closure never create rows, so they cannot restart; not separately measured.
- Fleet effect unmeasured: supervisors restart on the replicated code; check m02 `ready.json` for `refresh-facts` lines.
