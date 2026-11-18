# Why `mit-bestand-demonstrator:build` crawls: 4774 owners, one farm child, a 0-row wait

Read-only diagnosis. Evidence: host log `host-f7f1b7406764.log` (lines 44453-46830, failure at 46436), code under `🏗️build/🕸️graph/`. Nothing was run or edited.

## 1. Why one target infers 4774 projects
- The slice read is not a target slice. `🧭️discovery/🟦️.ts` `discoverBuildGraphPartV1` passes `ownerRoots: part.roots` only when `!part.cold` (`...(part.cold?{}:{ownerRoots:part.roots})`). A cold part infers the whole graph and engages the farm at the first miss (`partInferenceEngagementV1`).
- Cold is the generation state, not the target. The log shows `graph: no usable generation (invalidated), building the first one` six times (lines 43815, 45239, 47596, 49016, 50588, 52203): `🕰️generations/🟨️.mjs:212` maps any pointer with no manifest to `cold("invalidated")`. `engine.invalidate` (reasons `closure-missing`, `reset`, ...) publishes exactly such a pointer, so every repair round or reset throws the whole graph away, whichever target asked.
- Only `🤝️prefetch/🟦️.ts` (`prefetchBuildGraphProjectsV1`) is scoped (`ownerRoots: roots`); it serves the warm refresh, never a cold read. No slice-closure computation exists in the cold path: the target's dependency closure is not known before the graph is, which is the circularity.
- Result: `4773 of 4774 owner records present`, yet the read still does `owner looks: 4774 owners in 797 farm jobs of up to 6 owners for 2 children` (lines 45874-45895). The cost is 4774 verifications plus the few stale owners, not 4774 inferences. The progress line "Inferring projects 12 of 4774" counts verifications.

## 2. Why wfc-3d-rs shows 0 rows for minutes
- It is not inferring and not memory-bound. Line 46113 onward: `wfc-3d-rs has run 122 s ... parked: waiting for a farm child to resolve the closure references (the parent resolves nothing) [children busy 1, idle 0, queued jobs 1]`, repeating to 604 s. The parent resolves nothing by design (`project-nodes/🟨️.mjs` farm-only look/resolve), so every started owner queues behind the single child.
- The farm had one child: `[capacity] admitted exec with 1 of 2 job(s): the host cannot carry more right now` (45875), 1700 MB each. That child first spent ~100 s on `shared closures in a child` (45879-45895), then serves 797 look jobs plus the closure-reference resolutions of the owners the parent admitted. Owners admitted by the parent therefore sit with 0 rows while holding a memory reservation.
- Memory governor: the parent reserves per owner (`wfc-bitmap-rs needs 1008 MB`, `waiting for memory headroom 712 of 1113 MB reserved`, 60 s rule then "runs because it fits the 1756 MB one project is granted"). The reservation is a learned/estimated peak for `language:rust`; the real child peak printed is `child peak 0 MB`. The governor waits on estimates while the actual bottleneck is the child queue, and its wait text ("1719 of 2099 MB") is what the no-progress guard reports, misleading the diagnosis.
- The no-progress guard (5 min, 12 of 4774 done) fires because owners that are present need a child verdict and none is free; it is a symptom of one child for a 4774-owner read.

## 3. Proposed root fix (not applied)
1. Slice-scoped cold read: when the pointer is invalidated but the owner records are present (4773 of 4774 here), do not go cold. Keep the previous owner set as stubs and run the warm path with `ownerRoots = dependency closure of the target + the stale set`; build the closure from the stored edges of the last manifest (or the owner records), the way `prefetch` already scopes. Cold with an empty store stays full.
2. Make `invalidate` not erase the manifest: write a `dirty:all` marker that keeps the last manifest readable for stubs/edges (reason carried in the pointer) instead of `manifest:null`.
3. Owner admission by farm capacity, not per-owner parent reservation: admit an owner into `running` only when a child is idle (`children busy 1, queued jobs 1` must block admission); queue depth, not estimated MB, bounds the owners in flight. Owners that only need verification/closure refs need no inference grant.
4. Cost estimate: price `verify/look` (child peak ~0) separately from `infer` for a rust owner; only a stale owner gets the 1008 MB class estimate.
5. Capacity: do not start a cold farm with 1 child; wait (bounded) for the second job or run verification in the parent with a small cap, since the parent has 4773 records to read and 2 GB headroom.
6. Progress: report `verified/stale/queued-for-child` separately so "12 of 4774" is not read as inference.

## 4. Not verified
Which call invalidated the generation at 45239 (the log lines around it were not mapped to the `closure-missing` repair versus `reset`); the 1-child admission reflects host memory at that moment. Fix 1 depends on stored edges being enough to compute the closure; check `edgeConsumersOfPartV1` and the manifest `owners` fragments before coding.
