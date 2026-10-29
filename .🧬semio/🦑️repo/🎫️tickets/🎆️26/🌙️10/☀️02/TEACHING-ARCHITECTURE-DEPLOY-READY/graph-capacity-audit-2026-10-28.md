# Graph Wait Capacity Lifecycle Audit — 2026-10-28

Read-only source audit. Root reports only its own32120/82275/4072 were retired with130, while five peer grants hold roughly2.3–3GB and corrected backend test22049 queues. Those live-state facts were supplied by root; this audit did not sample, signal, change, lend or release any process/grant.

## Admission Precedes Graph Wait

Library 🏗️build/📜️script.ts:120–122 admits withBuildCapacityV1 before running the admitted command implementation; the owned nested path at96–99 likewise holds capacity around captured execution. The graph loader 🕸️graph/🟦️.ts:439–444 upgrades rebuild capacity before withGenerationCustodyV1 obtains its claim, then marks the rebuild/plan phases. Therefore a requester may already possess machine reservation while waiting for a conflicting graph claim. This ordering is source evidence; it does not establish why each of the five particular peer grants remains held.

Slice custody 🕸️graph/🕰️generations/🔐️custody/🟨️.mjs:106–126 records a waiting claim and waits on blockers with cancellation; the graph generation loader 🔌️load/🟨️.mjs:323–325 tracks whether builder custody is held and emits its queue notice while waiting.

## Existing Supported Yield Mechanism

There is already a schema-defined automatic graph queue yield, not a need to terminate a peer or alter a ledger. 🕸️graph/🕰️generations/🛌️yield/🔣️.json declares yield after15seconds, at most3yields, ten-minute wake limit,3second polling and exit75. The loader queue notice at 🔌️load:324 calls graphQueueYieldsV1 only while not holding builder custody and only when a pointer exists. The function yield/🟨️.mjs:14–22 additionally requires an offered ticket and an unexhausted round count, writes the graph/request ticket and exits the waiting implementation process. This drops its loaded implementation heap by ending that process.

The outermost front door 🚪️door/🔄️restart/🟦️.ts:14–17 offers the ticket; nested doors deliberately remove it. The same wrapper delegates to waitGraphQueueYieldV1, whose yield module:30–41 waits alone for generation advance or the bounded wake time, then carries the first requestedAt and incremented round into replacement. Existing stale reexecution retains executable arguments and original environment/envelope while propagating nested75. The graph queue branch uses that existing supervisor; it does not synthesize new Original authority or refresh an elapsed-time deadline. The previously audited dependency reexecution tests and graph-yield suite cover related custody/restart behavior, but no new tests were run here.

A command whose3yields are exhausted intentionally queues in place thereafter. A nested command has no own yield ticket. A builder that already holds custody is not an eligible queue waiter. These are concrete policy limits that can explain absence of automatic yield in some situations; this audit did not inspect peer envelopes/round counts, so none is asserted as the actual explanation for the current five grants.

## Lending Versus Heap Release

The actual machine capacity gate is 🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts. Its lender at254–265 measures the watchdog wait episode and requires the reservation owner to be childless before marking lentSinceMs. It restores the grant when waiting ends or children return. Therefore merely observing that a child waits for a graph cannot force lending; the childless lightweight front door is the intended owner state. Lending updates scheduling reservation and does not itself reclaim arbitrary heap; graph queue process exit is what removes the expensive loader heap.

The gate retains original control checks and normal cleanup. Its park callback is for stale-restart custody handover (library 🌡️host-capacity/🟦️.ts:13–16), not a public manual API to release another requester grant. The existing front-door yield/lending path is the supported mechanism found. No supported command was identified that externally forces an already running peer to yield only its graph wait while retaining all its immutable invocation state.

## Distinct Memory Cleanup

Graph host release at 🕸️graph/🟦️.ts:435 releases inference/input memos, resets closure/setup caches when not carried, clears the interner and invokes collection. The generation loader calls release after its builder loop at 🔌️load:326. This is in-process post-stage cleanup, not proof that a long-blocked waiter executes cleanup before the claim becomes available. Do not infer memory release from a waiting notice alone.

## Bounded Conclusion

The repository already models the desired wait-alone behavior with guarded automatic graph queue exit/restart and childless capacity lending. Its eligibility and maximum-round limits must be checked against an actual owned requester before attributing current retained grants to a bug. Source review alone does not justify peer termination, deadline resets, fake invocation state, forced ledger edits or bypassing current-source/claim checks. No such action was taken or proposed.

## Public Cargo Priority Interface Check

Inspected the neutral operations catalog, global flags and Cargo launch schema without invoking/restarting any build or changing a queue. No declared CLI/MCP Cargo priority override was found. operations/🔣️catalog.json cargo synopsis/flags and launcher schema Configuration options expose only store, configuration and parallelism. The catalog foreground flag is explicitly scoped to service start (attached versus detached service), not scheduling classification of cargo. capacity run exposes operation/memory/cpu/jobs, not priority; manually lowering the admitted reservation would not be an equivalent Cargo route.

Session open exposes weight for session budget allocation and client identity, not an authorized mechanism to relabel an agent as a human foreground caller or reprioritize a queued Cargo request. Existing automatic classification at library host-capacity/🟦️.ts:115–116 derives from real current agent session/human context and learned policy. The lease priority schema declares SEMIO_LEASE_PRIORITY classes interactive/agent/background, but that is an environment route outside this requested no-private-environment CLI/MCP check, not a Cargo flag to apply to an existing queue item. No recommendation to set it or impersonate another client was made.

Thus no supported priority/foreground option applicable to the already queued native Cargo build was identified within the permitted interface scope. This audit does not propose FIFO/fairness changes, reservation reduction, queue metadata edits or policy bypass. Backend owner subsequently reported17816 terminal at actual compiler diagnostics; that changed live state does not change the interface finding.

## Fresh44967 Graph Wait Sample — 21:48 UTC

Backend confirmed current owned44967 log and graph-claim-slot resource; no owner handle was polled by this audit. Registered Node read only immediate generation metadata, /proc command/stat/io and the bounded owned log tail. Retained input graph-wait-sample-2026-10-28.mjs; raw two-sample output 🗑️generated/graph-wait-current.json. No broad cache crawl, old211237 attribution, socket read, environment dump, mutation or job replacement.

Current observed lineage is shell446176 → repo door446177 → captured implementation446214 → captured implementation446483. Both captured processes carry run @teaching/proctor:build --configuration=release --parallel=1 under captured identity543a13b32b89a2c1a366c06f092275403f604fe998238ec050ec3619d08b069f. This establishes course build lineage, not an exact mapping of a shared graph claim to a producer PID; claim records inspected contain no PID. Do not relabel446483 the claim producer solely from CPU/memory.

Two snapshots at2026-10-28T21:48:00.419Z and21:48:15.431Z (15.012s apart) retain generation1/current manifest6b780921… and verification count4030 unchanged. Claimf2c1ff305b75b8cc9874afe6 stays running, base1,128owners, implementation6b797e3b…, unchanged hash. Earlier bounded inspection observed claim7044d8bc… running; by sample1 it is done (endedMs1793224068296) and f2c1ff… started101ms later. That is actual batch turnover, not proof current generation has committed. Prior claimc21ca42b… is also done.

During15s,446483 advanced46CPU ticks (utime+40/stime+6), read characters+2622423 and written bytes+176128; its RSS rose2850pages.446214 advanced5CPU ticks and44617710. Linux process stat state was S at both instants. Units are actual kernel ticks, not assumed seconds. This shows some work/I/O, but does not demonstrate useful completion of the two stale projects or identify a hotspot.

Actual44967 log continues build:graph-plan/graph-claim-slot shared wait and says all1compute slots busy,2stale projects still open, age161s by sample2. Therefore current conclusion is custody/admission wait for shared graph work, with recent batch completion and low sampled course-lineage activity; no evidence of Rust application compilation, qualified receipt or runtime readiness yet. It is neither justified to declare a deadlock nor to claim the graph finished. Preserve the same backend owner; any later progress decision should use new claim terminal/current-generation evidence, not repeat the old captured-producer theory.

## Bounded Producer Identity Follow-Up

Read only current generations/c running claim fields, immediate generations/s admission record, admission directory and existing course /proc ancestry; no recursive cache/lock scan. Current running compute claim remains f2c1ff305b75b8cc9874afe6, base1, implementation6b797e3b…, startedMs1793224068397,128owners. Its complete non-owner/stamp fields are base,implementation,startedMs,state,token,version: NO pid, process-start identity, boot identity, descriptor or lock pathname. Admission has no entries; the single generations/s record81c5cc12… is held/structural/ticket1/queuedMs1793223724709 with owners and token, also no PID/boot binding.

Current machine boot identity read directly is c773a46d-79d8-4506-be8e-b3855e4455c7. Existing course process identities include446177 starttick12301458,446214 starttick12301686,446483 starttick12303581 and parent chain already recorded. These identify those exact live processes within this boot, but no inspected authoritative claim/lock record associates f2c1ff… with any of those identities. Neither PID proximity, ancestry, process activity nor the matching course command is sufficient to supply that missing edge.

A bounded Source-path follow-up for the earlier observed claims implementation path returned ENOENT after concurrent refactoring; it was not treated as a peer failure or justification to crawl stores. Exact missing evidence is an authoritative active compute-slot lease/kernel-lock holder record binding claim/token/resource to boot identity plus PID and process-start identity, or a directly reported producer identity from its owner. Backend owns44967 waiter only and did not supply that producer binding. This audit therefore stops without assigning an inference producer, declaring stale/dead custody, probing sockets, or changing any process/claim.


## Root Capacity Recheck — 21:49 UTC

At21:48:20UTC two2048MB command requests were waiting, PIDs448585 and450210; frontend confirms neither belonged to its focused light test. A subsequent bounded /proc ancestry lookup found both PIDs absent (ENOENT); no ancestry or ownership inference was made from stale PIDs. At21:49:19UTC capacity reported no waiters and2857MB headroom. The proctor qualified build PID446177 retained its3584MB grant, observed1356MB. Other build grants remained active. No unowned process was interrupted and no duplicate semantic job was added.

## Front-Door-Only Wait Explains Grant Absence

Parent reports44967 authoritatively live at710s, front-door-only peer wait300s, while earlier3584MB grant446177 is absent from21:55:29 capacity snapshot (headroom10410MB). These are compatible states; grant absence must not be promoted to process/invocation death. This review reads only the relevant current restart/yield functions and policy, without a new process probe, build or gate.

Canonical door/restart runBuildWithRestartV1 offers a graph-yield ticket only for outermost invocations and passes waitGraphQueueYieldV1 to runWithStaleReexecV1. Canonical graph/yield policy explicitly says the waiting child ends so the front door remains alone WITHOUT a loader/child and its capacity reservation CAN BE LENT. Current policy is yieldAfter15000ms,max3yields,wake600000ms,poll3000ms,progress30000ms,exit75. Therefore a300s frontdoor wait is within the declared600s wake bound, not an observation timeout or terminal outcome.

Actual functions: yieldGraphQueueV1 atomically writes the generation/pointer/original requestedAt/round ticket and exits75. waitGraphQueueYieldV1 consumes it, polls only the generation pointer until it advances or600s passes, then returns original requestedAt and round+1. reexec runWithStaleReexecV1 awaits that yielded wait BEFORE offering replacement custody and launching the same original argv/environment with the returned request-time fields. This yielded path does not count as an early failed restart; final restart custodies settle through finally. Subsequent execution retains ordinary capacity/artifact/Source checks; no grant is fabricated and no bypass should be introduced to force resumption.

This establishes intentional loader retirement/frontdoor wait/lendable capacity/restart semantics, not the exact internal accounting event that removed446177 from that particular snapshot. The live backend owner handle remains authoritative for44967 liveness. At this phase the missing old grant alone is neither a failed build nor evidence of Rust progress. Frontend may use independently admitted current capacity for its single normal semantic gate as parent directs; do not reserve historical3584MB twice or restart44967 merely because its old grant is absent. Keep later product readiness and current capacity readmission results distinct.

## Fresh Producer Identity Has Already Advanced To Nx

Linux root supplied historical concrete binding: structural claimd543ead75f1bed0b7e2db372,199owners/ticket1; producer462345 FD31 tokenlease resource DB SHA8b814103…dfcbe, parent460615 captured course:typecheck. By this independent bounded recheck,462345 /proc/stat is ENOENT and that exact claim token is absent from both generations/c and/s. Its FD31 therefore cannot be independently rebound now. Do not relabel a new PID as that producer or invalidate the earlier observation.

Current boot c773a46d-79d8-4506-be8e-b3855e4455c7 is unchanged. Captured course:typecheck parent460615 (starttick12380020,parent460571) remains live and has new direct child468805 (starttick12430754). Its actual argv is the immutable Nx SDK nx.js run @teaching/architecture-course:typecheck --parallel=1. Metadata-only FD links show ordinary current SDK/cache lease DBs; no raw DB contents were read. No claim is made that any of these new descriptors holds the prior tokenlease.

Two actual counters at22:06:01.157Z and22:06:16.174Z:460615 advances1CPUtick,468805 advances9CPUticks; parent/start identities remain stable, both S at sample instants, Nx child469693 stays present. Retained input graph-producer-identity-2026-10-28.mjs and raw 🗑️generated/graph-producer-identity.json capture these15.017s observations.

Actual course-typecheck-resumed log now says Nx is running the course target AND6prerequisites, with assets:build:prototype and styling-tokens:generate:prototype successful. Frontend owner independently confirms197/197 inference completed and same62580 remains live. Thus the supplied producer identity is historical because graph inference advanced naturally to target/prerequisite execution. Current evidence supersedes a producer-still-waiting interpretation; it does not imply semantic compiler success, qualified receipt or browser readiness. No build/process mutation, signals, DB read, additional gate or peer intervention.

## Fresh MCP grant owner00:46–00:47UTC

Bounded current /proc reads bind PID732754/startTicks13304733 to current boot c773a46d-79d8-4506-be8e-b3855e4455c7. Its actual command is the repo frontdoor running @semio-tech/framework-os-mcp-rs:build --output=stream. Parent732705/start13304678 runs root dev mcp stdio os, parent711006 is claude, then shell/terminal ancestry. This identifies the current task from actual process argv, not an old owner report.

Sole direct child732784/start13304782 is the captured Bun loader, implementation59c5661d4f7453b4341014b27399f7cede281d12c6e2c0dc8d272a18956d5a56, same build with --parallel=7. At00:46:59.803 it was runnable with62,540CPUticks/no child. At00:47:15.225 the same start identity was runnable with65,119CPUticks/no child: +2,579ticks over15.422seconds. Frontdoor itself slept in epoll,701→708ticks. This demonstrates ongoing actual captured-loader computation; it is not an owner whose only evidence is a reservation or a frontdoor custody wait. No actual compiler child was observed in this bounded tree.

The stated8,269MB reservation/799MB held/-280MB headroom comes from root capacity evidence; it was not independently resampled here. No machine-wide process scan, raw descriptor contents or lease DB was read. stdout/stderr are sockets, not discoverable retained log paths; selected ticket/build context environment keys were absent. Thus exact internal compute phase and ticket authority cannot be authenticated from this sample, and no idle/leak/deadlock or lendable-capacity claim follows. Root received live task/CPU evidence for Linux coordination; no signal, mutation, new build or namespace poll occurred.

## Current nested capacity attribution02:06UTC

Canonical capacity_status snapshot02:06:07.312UTC: grants815045typecheck5383MB/702observed and815084dev5301MB/2769observed. Evaluation floor1981MB/unrealized7213/headroom-735; oldest qualified serve-release815158 waits for memory, dependencies816038 and all later waiters explicitly wait queue-order. Values are instantaneous and differ from earlier root snapshot; no reservation estimate was edited.

Bounded /proc PID/PPID/start metadata on current boot c773a46d-79d8-4506-be8e-b3855e4455c7 binds exec816659/start13839209 to816506/start13837466→815084/start13826520 previewdev. It is graph-farm request fixed1536+2048perjob (minimum3584), session a2m6c56sjvr exactly matches its devgrant; runnable CPU40717ticks at sample. Thus this current exec is a preview descendant, not independently established as semantic descendant. Typecheck815045/start13826325 remains distinct grant.

Dependencies6144MB waiter816038/start13833275 has separate shell816034→systemd ancestry; selected safe operand identifies staffage/tests/blackbody-exchange Python test. Command2048MB waiter816822/start13840654 is separate shell816821→Codex ancestry, selected operand identifies caching aggregate suite test. Full argv/environment/FD contents were not exposed. These are logical operand attribution, not proof of external ticket/user authority.

The exec retains matching session identity and canonical evaluator explicitly reports memory/fair-queue reasons. This does not demonstrate missing Original inheritance or authorize bypassing a child reservation; parent grant identity and separate graph-farm worker requests are not interchangeable. Root reports graph progress93/93semantic and70/128preview, so no deadlock/abandonment inference is justified. No signal, build, claim/estimate edit, environment read or process mutation occurred; all handles unchanged.
