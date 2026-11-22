# 🕰️ Result E2 Round 1: Generation Adoption (transcribed by the coordinator; the lane's file write was refused)

## Live state (01:05 CET)
- Coordinator whole read running (`repo show projects`, pid 56836, since 23:39 UTC); G1 admitted 4096 MB instead of 14,336 MB.
- Last sealed whole generation: 259 (2026-11-20). Generation 260 is a quarantine pointer (`manifest: null`, `closure-missing`), so every later read starts cold.
- Every whole read of the last 20 h was cancelled, not refused: the graph sidecar restarts after code edits and defers an in-flight read at most 1 h; with many landings no read finishes. **Main blocker.**
- Only fleet head: g1-m10 partial (5100 owners, 4952 stale, 2026-11-18). g1-server publishes none. Owner pool throughput 0.4 owners/min. Implementation digest moves with each landing (invalidates records and quarantines the parent).
- Swap refusal no longer blocks (hard stop needs swap > 25% RAM and active swap-out; 0 MB/s out). The 60,129,542,144-byte floor exists only in the ticket lab script `constrained.ts`; production floor is 15 GiB bounded to 6% of the volume.

## Change
`🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts`: new `generationKeepsLocalV1`, `generationImplementationKeyV1`; `keepsLocalV1` and `completeAheadHeadV1` use them. A worker now takes a complete head of its own code and platform published after it last sealed (once per publication), even though its own pointer is newer (it rewrites the pointer every sealed wave). Implementation digests compared as sha256(platform + digest). `wholeReadGateV1` already returned `fetch` with `held > 0`. Ready comes from the slice read; the adoption lane only runs on a Ready worker, so the 13 missing adoption receipts are a symptom. Landing in `T/landings.md` "Wave 1122 E2" under `landing.lock.graph`.

## Tests
New `🧪️tests/📥️newer-complete` (case table vs independent SQL model, platform key, two real-cache-server scenarios). Pass/fail: newer-complete 4/0, remote-exchange 13/0, head-rank 5/0, scope 4/0, wave-delta 7/0, restore 1/0, whole-gate 13/0, adoption 3/0.

## Not verified
Live adoption (no complete head exists); whether a 16 GB worker's slice read takes the head before inferring; digest agreement across all ten macOS workers (only m10 vs coordinator at gen 259); no typecheck.

## For others
- Sidecar restart deferral (`sidecar/🧬️schema/🔣️.json` `serveDeferralMilliseconds`) vs in-flight whole reads: E8 area.
- E1: a worker below 24,000 MB must beat another head by 50 owners to publish (`publishGenerationV1` shadow margin), so a first slice-scope head may not be published.

## Round 2 (transcribed by the coordinator; the lane's write was refused again)
Landed under `landing.lock.graph` (record "Wave 1122 E2 round 2" in `T/landings.md`):
1. Resume base: `🩹️closure-repair/🟨️.mjs` `compatible()` and `openRepairBase` no longer require equal implementation digests; a quarantined parent of another implementation is a valid repair base (repaired candidate is verified by content). New red→green case in `🩹️closure-repair/🧪️tests/🟦️.ts`. Resume after cancelled read unchanged (build journal + wave publishing + owner records already exist).
2. Digest scope: already narrowed by `♻️revision` declared ignored sections (`hostCapacity`, `workers`, `remoteCache`, `remoteExecution`, caching `storage`); only 7 data files + 18 stamp rows reach the digest. New test `🔌️load/🧪️tests/🧭️digest-scope` (ignored edits keep digest, `graphMembers`/`namedInputs` move it, equals independent sha256). Not tested: "unrelated library file → zero owners re-inferred" at load level.
3. Shadow margin: `publishGenerationV1` skips the 50-owner margin when a complete head meets a partial one; two complete heads still keep it. New case in `📥️newer-complete`.
Tests pass/fail: closure-repair 11/0, newer-complete 5/0, digest-scope 3/0, remote-exchange 13/0, generations 37/0, load 114/0, waves 8/0, handover 2/0, whole-gate 13/0, adoption 3/0, lost-leaves 5/0, closure-leaves 9/1 (closure-memo `fragments` undefined; not checked against pre-change tree).
4. Live: coordinator read since 23:39 UTC survived 25+ min, but cold: 29 of 4818 owners, "0 present"; no progress over a one-minute sample; farm at ~1.4 GB "waiting for memory headroom" with 12 GB available (follows the 4096 MB `show` grant). Nothing sealed (`current.json` = gen 260 quarantine pointer).
Findings: fleet lists 1880 owner records but the reader adopted 1, missed 2584, faulted 1255 (1240 "unavailable") → record key or fetch mismatch (E1). This machine's own owner records were deleted, probably by the LRU reclaim ("freeing least recently used cache objects") that runs before the command starts. E8: defer sidecar restart while the read's `.cache/b/g/m/generations/s/<token>.progress.json` advanced in the last 10 min.

# Round 3 (E2, 01:35 CET)

## 1. Farm starvation: fixed in code, live effect pending a sidecar restart
- Cause: the owner governor of the farm parent charged every in-flight owner (estimate 371 to 441 MB, they run in child processes) against `budgetBytes` = the parent's managed bytes (about 1.4 to 1.8 GB of the 4096 MB `show` grant), so 5 to 6 owners at most ran with 12 GB free. Log: "waiting for memory headroom (1465 of 1508 MB reserved, 5 projects running)". Children are already paid by the exec grants of the host gate (8190 MB reserved, "admitted exec with 3 of 7 jobs"), so the budget counted them twice.
- Change: `B/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs` passes `budgetBytes` only without a farm (`...farm===undefined?{budgetBytes:parentBytes}:{}`); with a farm the headroom is the real host headroom (`serviceMemoryHeadroomBytesV1`, free memory less the safety margin), which grows and shrinks with the machine. The parent collector still holds the parent to its own managed budget. Host admission (exec grants, floor, swap stops from G1/E7) still bounds the children.
- Test: `B/🧠️memory/🧪️tests/🧹️parent-budget/🟦️.ts` (new case): 12 lanes, 12 GB free, owner estimate 371 MB: with the parent budget at most 4 run at once, without it all 8 lanes run; plus a source guard on the project-nodes line. Memory tests 30/0, parent-budget 13/0.
- Not verified live: the running read (pid 56836, started 23:39 UTC) still has the old limit; it needs the sidecar to restart (E8's progress-based deferral keeps it alive while it advances, 245 of 4818 owners at 00:27 UTC, about 2 per 5 minutes in the last window).
- Not done: thresholds with E7 (no `findings-E7.md` existed when I looked; E7 held the graph lock for the owner-pool swap shares).

## 2. LRU reclaim deleted the own owner records: fixed
- Cause (measured): the active whole-read selection group `inference/selections/8d8f21...` held 4688 `.records` and 4758 `.fleet` markers but only 54 record headers `np/<24 hex>.json`; the graph-inference area (5.7 GB against a 2 GiB budget) retires the least recently written units of the writer's own group (`retireBuildGraphOptionalEntriesLruV1`) and whole groups (`...GroupsLruV1`); only units written in the last 10 minutes (`groupProtectedIdleMilliseconds`) were protected, so a read that takes hours lost the headers it had written.
- Change: `B/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts`: `pinnedOptionalUnitsV1(root)` pins (a) the record headers of the owners of the newest sealed generation, or of its parent while the pointer is a quarantine (`closure-missing`), (b) every unit written since the oldest live builder (custody file `generations/s/<token>.json`, state held, pid alive) began. Both retirement functions skip pinned units (accelerator packs of a pinned unit may still be shed); the group pass retires only the unpinned leaves of a group.
- Test: `🧪️tests/♻️lru/🟦️.ts` (new case, owner key computed by an independent Python sha256): the parent's header survives the own-group and the group pass with target 0, a dead holder pins nothing, a live builder pins what it wrote. lru 7/0, scan 11/0, members 11/0.
- Not changed: the 2 GiB `graph-inference` budget itself, and the host-level "Partition / is 94.7% used" selected/optional passes (they go through the same retirement functions).

## 3. closure-leaves memo failure: stale test, not a bug
The encoder writes `{version,sha256,entries}` since 2026-11-21 17:42 (`🧲️closure-shards/🟦️.ts`); the test still read `fragments`. The schema `ClosureMemo` validator in the same test already passed. Test moved to the current format: closure-leaves 10/0.

## 4. Whole read to seal: not sealed
- At 00:35 UTC the read is at about 247 of 4818 owners after 56 minutes, `0 present`, old farm limit (see 1). No generation sealed, so no wall time and no peak MB for a completed generation. Peak so far: parent about 2.1 GB RSS (pid 56895), `show` grant 4096 MB.
- Next blockers I can see: (a) the running process needs a restart to take the farm fix; (b) fleet side adoption: 1880 owners listed stored, 6 adopted, 5387 missed, 2516 faulted (E1); (c) the fleet cache was down at 00:48 CET (E1 finding 2); (d) host load 37 to 41 from other lanes.

# Round 4 (E2, 01:52 CET)

## 3. Test fixtures (activeBasisPointsPerSecond)
After E7's 01:13 policy fix: supersede 16/0, resolve 9/0 pass. `📏️headroom/🧪️tests` still fails 13 cases with "Original parent Script process invocation required": that is the repo test runner requirement (not a fixture or policy key), so nothing to fix in fixtures; run through `repo test`.

## 1. Per-owner cost: root cause found, farm restored, profile only partial
- Root cause of 10 to 25 owners/min on the coordinator: the read logged "graph: inferring 4818 projects in this process (memory)". The farm width was computed from the rebuild reservation (`grantedHostMegabytesV1`): 4096 MB gives 1 child, the shrunk minimum 2304 MB gives 0, so the whole read ran on ONE thread in the parent (main thread about half busy in `sample`, the rest waiting). My round 3 governor fix only applies with a farm, so it did nothing there.
- Change (memory area, landed 01:55): `farmHostMegabytesV1` in `B/🧠️memory/🪜️plan-stage/🟨️.mjs`: the granted memory plus the free memory above `availableFloorMegabytes` (4096, schema) turned into the physical memory whose `treeShare` it is, never above the host; `planFarmWorkersV1` takes `availableMegabytes`. Caller (graph area, landed 01:50): `project-nodes/🟨️.mjs` passes the host sampling reading (`hostAvailableMegabytesV1`). Children are still admitted one by one by the host gate (exec grants), so over-asking is safe. 2304 MB granted plus 12 GB free: 0 children before, at least 5 now.
- Tests: `📐️farm-width` (4 new model cases, Python model extended, new test: shrunk grant plus free memory widens, never beyond the host, no reading changes nothing), plan-stage 12/0, farm-width 2/0, tree-plan 7/0, parent-budget guard 13/0.
- Profile: from `sample` of the inference process (pid 56895, 8 s): main thread about 45% in JS, native frames unsymbolicated (Bun), so no phase attribution; no standalone single-owner harness exists (the farm child needs a session file). I did not measure per-phase cost. The owner cost table (`.cache/f/graph-costs/owner-costs.3.json`, 2766 owners) shows 1827 `language:cargo` and 653 `language:rust` owners with a median of 65 to 157 s of WALL time under contention (not CPU), so cargo/rust owners are the bulk; E1's finding 1 (presence batches 15 s, `allocationJournal`) is the other measured cost. The order-of-magnitude target is NOT met or measured: only the farm lever (about x(children)) is delivered.

## 2. Fresh whole read: not started by me
- E1 round 3 landed 01:46; my caller edit landed 01:50. The graph sidecar did not restart: its log says "attached to the graph read (pid 56836) that has run for 4249 s; it is not started again" (E8's progress-based deferral: restart only after `progressStallMilliseconds` without progress). I did not kill anything.
- The old read keeps going: 607 of 4818 at 00:49 UTC, 627 at 00:51, about 30 owners/min since load dropped, so it may seal in about 2.3 h on its own (generation of the OLD implementation digest; a parent base for the next read, valid under the relaxed repair rule). New code (farm, pins, E1 keys) takes effect when that read ends or stalls.
- Not verified: wall time and peak MB of a sealed generation (none exists yet); live effect of the farm widening.

# Round 5 (E2, 03:40 CET = 02:40 UTC)

## Stopping the old read
`read-watch` is only a touch file (no cancel path). I sent SIGTERM to exactly pid 56836 from `read-record.json` (verified command and parent 56835 first). It exited in under 10 s ("Cancelled (SIGTERM); leases released", child 56895 TERM then KILL by its own execution, `read-record.json` removed, checkpoint file intact). The sidecar logged "the graph read ended with status 1 after 4474 s" and did not start a new read by itself for about 54 minutes: `runWantedServerV1` refuses a refresh while `work.restartPending` (code edits by 14 lanes keep a sidecar restart pending up to 1800 s each) and holds 180 s after a failed read.

## Blocker found and fixed: every read died at start
The first fresh read (01:21 UTC, pid 53445) failed after 206 s: "Undeclared compiler-captured physical Source read refused ... `📥️load/💾️owner-checkpoint/🟦️.ts` (sourceCount 2271)". `🔐️authority/🟦️.ts` `graphSelfUrlsV1` still listed that module and its schema although nothing imports it any more (load/🟦️.ts changed 02:13 CET), so the compile capture lacked it. I dropped the two entries (landings.md "Wave 1122 E2 round 5", lock graph). Module loads, 14 self urls, no test covers the list. Told E1 in findings-E2.md.

## The fresh read (pid 14453, child 16892, started 01:52 UTC with farm widening, pins, E1 keys)
- Phases (from `generations/s/<token>.progress.json`): start and lease queue about 10 min (waited behind the resident service host's graph/ledger leases), inference `5101 of 5101` done at 02:04:09 UTC (no `[owners]` lines were printed, so I cannot split single-thread vs farm or count children; the owners were not computed one by one at the old 10 to 30 per minute: the pinned and re-keyed records were present), fact index 841,640 facts in about 15 min (about 23k per minute at first, 120k per minute later), verification of input closures 0 to 7141, flush wave at 02:23, then 1398 of 7141 at 02:26.
- Since 02:26 UTC (11+ min) verification does not advance. Diagnosis of the graph server: "the read holds 4371 MB, 1.1 times its grant of 4096 MB: it pages against the swap", pressure warn, 5836 MB free then 77 MB; child 16892 has 79 MB resident but a 2.9 GB physical footprint (swapped out). Host: swap 20.8 of 22.5 GB, load 57, `fseventsd` 6.7 GB at 100% CPU, parallel `rustc` builds of other lanes. It is host overload and swapping, not a deadlock in the code: children are idle waiting, the parent is idle in kevent, lease churn continues.
- Peak MB: 4371 MB physical footprint of the process tree (grant 4096 MB, largest pid 2918 MB).
- NOT sealed: `current.json` still the generation 260 quarantine pointer. The read is left running under the sidecar (I killed nothing else); if the sidecar's stall rule restarts it, the next read finds the records and only repeats the fact index (about 15 min) and verification.
- Wall time so far: 48 min (01:52 to 02:40 UTC), of which 12 min inference, 15 min facts, 10 min waiting for leases, 11+ min verification stalled on swap.

## Not verified / next
- Single-thread vs farm owners per minute (no data from this read); farm children count (not logged).
- Needs the host to breathe: fseventsd (E13/E7), parallel builds of other lanes, swap at 93%. A seal needs about 6000 more closure verifications (about 4 per second when memory is available, about 25 min).
