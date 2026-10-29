# Graph Parent Retention and Budget Audit

Parent supplied exact termination attribution: host earlyoom PID1587 sent SIGTERM to setup graph Bun356472 at21:32:08, VmRSS4178MiB, hostavailable308MiB, swapfree0/8947MiB. This supersedes the earlier unattributed SIGTERM statement. It was not a repo timeout and no audit signal was sent.

## Material Budget Mismatch Candidate

Graph memory module graphMemoryBudgetBytesV1 derives half of carried plan.memoryMegabytes plus plan.fixedMegabytes, clamped1024–16384MB. Actual inspected container descendants carried16384+461MB in SEMIO_HOST_CAPACITY_PLAN while parent reports the live setup reservation3831MB. That carried plan yields8422MB managed growth budget. createMemoryGovernorV1 additionally subtracts measureMemoryBaselineV1 taken after discovery/setup; createMemoryPortsV1 samples heapUsed+external, not RSS or full process-tree resident usage. Consequently a4178MiB RSS process can remain below admission threshold. This is concrete mismatch between the variables the governor reads and actual reservation, not proof of individual retained-object size. Existing owner readLiveHostCapacityPlanV1/followHostGrantPlanV1 is the architecture to inspect for current-ledger budget binding; no bypass or environment override was applied.

## Parent Lifetime Versus Worker Lifetime

project-nodes/🟨️.mjs169 builds results for all discovered configs;221 projects and222 projectsByRoot retain the authored graph;226 jobs retains references to each result/json;237 replaces each retained result with the assembled project. Native cache/nativeClosures at158 and scripts cache159 survive the entire inference. Observations rows/derived facts pass to onOwnerObservations243, whose caller controls retention. This necessary final-graph storage and shared derivation caches grow independently of worker process recycling. No heap profile was taken, so none is claimed the dominant object class.

The project farm prepares workers only after enough misses, then returns false on no usable pool, worker failure/loss, or declined publication; observeProjectInferenceV1 can fall back to inferProjectBodyV1 in the parent. The464adopted versus514progress figures are consistent with some parent/cache work but do not quantify per-object memory. Worker recycling by projectcount/RSS bounds each worker lifetime; it does not recycle parent parsers, memoized closure observations or accumulated results. Parent repeatedly reads adopted records into its own graph.

## Existing Controls and Limits

Memory schema owns minimum1024MB, maximum16384MB, grantShare0.5, hostShare0.25, resume0.85, childheap2048MB, retainedimage32MiB, interner67,108,864characters, maximumCachedClosureEntries4,194,304, maximumParsedManifests256. Governor rate-limits collections and new starts; after collection leaves irreducible data overbudget it admits one project when nothing runs. This preserves progress but is not a hard resident cap. SEMIO_GRAPH_MEMORY_MB is an existing explicit budget hook; it is not evidence of current owner accounting or a recommended bypass.

Next clean correction should first bind managed budget to effective current live reservation and account for baseline/parent+worker shares honestly, then prove bounded parent accumulation/cache behavior with neutral data and independent size/serialization checks. Preserve graph results and observation custody. Do not infer a confirmed memory leak, delete claims, weaken physical pressure guards, or restart the peer solely from this static review. No source edits/heavyprofiling/compilation performed.

## Bounded Serialized Extent Measurement

The private container cache remained accessible through surviving container sleep process142395, using read-only /proc/142395/root/workspaces/semio/.cache/b/g/m. Selection8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089 contains520 valid compressed np JSON records. Inspection read at most1MiB per compressed record and inflated each sequentially with an8MiB output cap; it did not retain a whole expanded graph. Format is two JSON lines, metadata then exact project.

Container520records: compressed2,705,214bytes; inflated40,754,093; projectlines40,360,440 (~38.49MiB); metadata393,133; largestproject1,701,601bytes. All520 project records parsed successfully. These reflect current cache, including parent inference/cache records, and are not an exact set of464 workeradoptions from the terminal summary. Host comparison3760validrecords: projectlines250,598,159bytes (~239MiB), metadata1,586,337; largestproject1,700,521. One other host record was excluded as nonconforming. Generated graph-record-extents.json retains numbers only.

The container exact project serialization is under1percent of the killed parent4178MiBRSS. Serialization is not JavaScript resident size, so no strict heap upperbound follows. Still, serialized finalproject rows by themselves do not directly explain that RSS; expanded references, repeated observations/derived closures, native/script memoized state and runtime heap/allocator overhead remain material candidates. Worker recycling leaves all parent-lifetime structures identified above intact. No heapdump, profiler, signal, graph request, cache mutation or source edit was used.

## In-progress Memory Correction Review

Current correction reads same-id ledger reservation instead of theoretical execution plan; budget includes baseline0, samples max(RSS,heapUsed+external), refreshes reservation before each admission and caps explicit override by the bound. These address the prior measured accounting gaps without changing physical admission guards.

Remaining observed in-progress mismatch: project-farm/pool ownGrantPoolV1 still reads theoretical memoryMegabytes/jobs, yielding2048MB for original16384/8 plan. Parent budget1915MB from live3831MB plus that owned worker2048MB totals3963MB before outer captured-process resident pages. Parent and owned worker shares must derive coherently from one live reservation rather than individually claiming it. Sent bootstrap owner and root. This may already be under active repair; it is not a claim about final patch.

New readLiveHostCapacityReservationV1 currently validates grant schema and same plan ID, not PID/boot freshness. It is accounting DATA, not a fresh ownership authorization proof; maintain that distinction or use existing liveness decision where actual live-row proof is needed. No Source edits or test execution by review.

Liveness follow-up recommendation to bootstrap: reuse isCurrentHostGrantV1 with same-id ledger row, alive(grant.pid), hostBootAtV1 anchor and filemodified time under the existingpolicy (the exact inherited admission check), preserving quietrow0 and conservative absent DATA for stale/malformed/missing rows. PID existence alone does not bind same boot or heartbeat. Do not grant new runtime authority from an accounting helper.

## Shared Allocation and Liveness Review

Current Source now derives parent/fixed/worker from one qualified reservation. For3831MB: parent1915, fixed461, remainingworker1455; sum3831. parent is clamped by total before minimum effects, fixed is clamped by remainingtotal, worker gets nonnegative remainder. ownGrantPool floors workerbudget across finite worker count and declines a pool below the schema minimum; it does not manufacture1MB when no capacity remains. Small reservations therefore allocate no owned worker rather than exceeding remainder. Reader now reuses existing current-grant same-ID, PID, boot anchor and heartbeat predicate under canonicalpolicy; no ownership authority is widened.

Remaining conditional lifetime issue sent to bootstrap/root: ownpool worker limit is captured once, while graph parent refreshes budget at each admission. Host gate enter(next)243–244 can publish a resizedreservation while descendants live; farm prepare76–78 dispatches into existing pool without checking its old allocation. If a shared grant shrinks before farm closes, the old worker envelope can exceed new remainder. Need either an explicit owner guarantee no phase-resize while farmalive, or revalidation/drained retirement before future dispatch/recycle. No actual later shrink failure was observed; this is a Source invariant gap, not a measured regression. Reported fourtests56assertions belong to implementation owner; no new audit test execution.

## Frozen Memory Patch Final Verdict

Independent final Source review found no material remaining defect in the authored allocation and shrink-drain correction. Startup shared reservation sum and tiny-remainder worker suppression hold; reader checks same ID/live PID/sameboot/currentheartbeat. Parent refreshes actual budget before admission, includes resident baseline and max(RSS,managedbytes).

The earlier conditional pool-lifetime issue is closed: refreshOwned serializes refresh, marks old owned pool unready, and retireOwnedGraphPoolV1 awaits the snapshot of accepted work through adoption before closing and releasing. Those inner work promises do not await refresh, so no self-deadlock was found. Future prepare calls await the shared refresh; replacement allocation is recomputed after old pool closes, before new dispatch. close marks closing then awaits any refresh, preventing replacement during final cleanup. Already accepted work drains instead of being silently lost.

Bootstrap reports final combined37tests1777assertions terminal0, including actualpositiveworker fixture7168=3584parent+1536fixed+2048worker, stale/dead/boot/quiet reservation cases and shrink/RSS tests. This audit reviewed Source and did not rerun those tests. Fresh full graph and container/runtime gates remain required; numeric allocation correctness does not itself prove resident memory can never overshoot during one expensive operation. Root received final freeze verdict.
