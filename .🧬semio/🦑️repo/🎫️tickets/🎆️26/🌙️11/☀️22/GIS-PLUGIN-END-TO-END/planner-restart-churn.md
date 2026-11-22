# Planner restart churn and slice workers

Sources: resident log `host-0003b7003034.log` (pid 19037, started 11:59, 46.9k lines, the only host log written today after 14:38) and the client logs `🗑️generated/{os-hub-dev,gates-4,s-shell-dev,test-sweep-4}.log`, read by grep and sed ranges. `host-9fccfa4354c2.log` is a separate 14:21 service whose log stops at one earlier request.

## 1. Why os-hub:dev "restarted"

It did not restart on an implementation authority change: no "implementation authority changed" or "Stale source facts" line exists in the resident log after 04:38, and no `[graph-retry]` line at all.

What happened (client counter 2235 owner checks at 3021 s, then 20 observations / 0 checks at 3046 s):

- The owner pass of the 1533 admitted owners finished (`[graph-closure] pass 5`), then the settle of that pass said "Owner round 0 repeats: 15 path(s) were read in two states inside one owner" (`[graph-settle]`, `evicted: 44`). The paths are planner files: `build/🔬️test-file/🏃️run`, `native-build/🎟️admission/🧬️schema`, `build/🧹️cache-gc`, `🧠️memory/🦶️footprint`, `headroom/🧾️closures`.
- Settle repeats re-walk the closure and re-observe every evicted owner (`evictOwnerFactsReferencingV1` forgets every owner memo whose rows name the path, 35 to 48 owners per round). Each repeat is announced to the client as "a new pass over the owners starts (round N): 1 path(s) changed under the read" (35 lines for round 1, 149 for round 2, about 600 s of heartbeats) and the counter restarts at 0.
- Two `[graph-repair] registry followed` rounds (new Cargo owner directories under `✏️s/.../📦️packages/🦀️rust` and `raw-media`) added further closure walks.

So the cause is stale source facts of planner files under constant edit by peers: those files are in the command-source closure of dozens of owners, an owner inference takes minutes (a TypeScript program of 17.5k files, 88 s each, and 100k+ observations), and any edit inside that window is a contradiction inside one unit, which repeats the whole round.

## 2. Why gates progress slowly

`gates-4.log`: 5 completed owner checks and 108969 distinct source observations at 2075 s. Only 5 owners are selected, but each is a heavy owner (observations jump to 82k to 108k per owner; one owner check takes 100 s or more). The pass restarts "a new pass over the owners starts" every 80 s from 311 s to 1679 s (nine times) with the counter back at 1 to 5: the same settle repeat as above, each repeat re-reading the tree inventory (13k to 108k observations) before the next owner can finish. The run also waited 37 s for the service and about 8 s for capacity first (load 16 to 28, `swapping out too fast`). It is churn-bound, not owner-count-bound.

## 3. Slice worker allocation

- Pool size: `maximum: max(2, min(4, floor(totalMB / 8192)))` in `⌨️command/🌐️host/🟦️.ts`. This Mac has 16 GB and 10 cores, so the pool has 2 workers.
- A load additionally needs a memory reservation (`reserveGraphPlanCapacityV1`, plan booking up to 6351 MB, admitted with as little as 2048 MB). With 16 GB and a 2048 MB floor, 3 to 5 GB available and load 16 to 28, a third worker would only deepen swapping (the log already shows `the host is swapping out too fast`). Scaling the count with cores would be wrong here; the existing formula scales with memory and stays correct. Not changed.
- Sharing: `shareKeyV1` joins requests with the same root, arguments, tool and graph policy only. The four requests have four different selections, so they cannot share one slice. A superset/subset join (s-shell's closure is inside os-hub's) would need the load result to be provably a superset of the other selection's graph (tasks, configuration, inputs differ per selection); not safe to add blind. What already is shared across loads is the owner fact store, so the waiting s-shell load becomes cheap after os-hub's owners are memoized; the wait (3618 s) is the two workers being held by loads stuck in settle repeats.
- Priority only decides queue order; both slots were occupied by loads that make no net progress, which is why the fourth request starves.

## 4. Change made

Implementation stamps in this planner are explicit protocol versions, not file hashes, so an edit of a planner file does not move a stamp; what invalidates records is the tree facts of the planner files that owners read. A request cannot ignore those facts soundly (they are inputs of the tasks), and a first-read-wins pin of tree facts would have to substitute facts at every read path (`observedFs`, analyses, memos), which would record digests for bytes the reader did not see. So the change attacks the multiplier instead: how often a request repeats the closure walk during a burst.

- New `build/🕸️graph/📥️load/🔕️settle-quiet/🟦️.ts`: `plannerPathsV1` (paths inside the build library tree the module runs from) and `awaitPlannerQuietV1`, which reuses the slice pool's quiet gate (`awaitImplementationQuietV1`, 60 s quiet, 600 s maximum from the host policy) and returns at once when no contested path is a planner file.
- `📥️load/🟦️.ts`: the settle `onRound` handler now waits for that quiet after evicting, before a repeated owner round. A burst of N planner edits repeats the walk once instead of up to N times; edits of tenant files are not delayed. The wait is bounded, abortable and applies to nothing else.
- Test: `📥️load/🔕️settle-quiet/🧪️tests/🟦️.ts` (5 pass): tree membership (sibling prefix and `..` rejected), tenant-only paths never wait, quiet planner file passes at once, a burst is waited out once naming the newest file, an endless burst stops at the maximum, abort ends the wait. `🌀️settle-churn` still 4 pass.

## 5. Not changed, still open

- Evicting only the owners of the contradicting unit instead of every owner that names the path would cut re-observation further, but the unit identity is not exposed by `settleObservedComputationV1` today.
- Pool size and superset sharing stay as analysed above; re-evaluate on a host with more memory.
- The real root cause remains that command-source closures of ordinary package scripts include planner files, so every planner edit stales about 40 owners; the longer-term fix is to hash planner code as one implementation input of the tool, not per owner.
- The change is not verified end to end: the four live requests were not rerun.

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🔕️settle-quiet/🟦️.ts` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🔕️settle-quiet/🧪️tests/🟦️.ts` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🟦️.ts` (import and `onRound`)
- this report
