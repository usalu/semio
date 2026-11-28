# H105 result: coordinator CAS over cap (507 outage) and upload-entry-gone bursts

Measured 2026-11-27 23:10-23:20 CET on the coordinator (`semio-g1-server`), `repo fleet status --json --detail`, `repo remote-cache status`, `v1/census.json`, `.cache/f/sidecars/build-cache.log`.

## Finding 1: the cache server was refusing every write (outage), not just "over cap"
- Server log: `refused 851..1068 write(s) with 507 in the last 60 s` every minute (38,404 since the 21:3x start, 1,890 in the first 2 min after the 22:11Z restart). Text: "store holds 50.9 GiB of its 43.5 GiB cap in force (role cache-host, 66.8 GiB free, hard reserve 74.11 GiB)".
- `repo remote-cache status`: server holds 12 AC entries / 5.4 KB, referenced blobs 1.6 MB, blob bound 2.35 GB; CAS puts 2.91 M of which rejected 1.33 M. The fleet cache is empty. This is why H73 saw "the fleet did not hold the units": nothing could be stored.
- CAS now: used 54.70 GB, effective cap 46.98 GB (116 %), free 71.9 GB < reserve 79.6 GB (distance -7.7 GB). The collector runs (last 41 s ago) and frees 0.

## Root cause 1 (why over cap and why the collector frees nothing)
`census.json`: held 41,781 objects / 52.71 GB (96 %), pinned 585 / 1.99 GB, young 71, unreferenced 0, expired 0. All 52.7 GB are held by ONE holder: the coordinator's own checkout unit cache `.cache/b/r/m/u` (14,916 entries). Nothing but held objects remains to collect.
The cap in force is `store + free - reserve` (reserve violated by 7.7 GB because of other consumers on the disk). By design (`casPartitionPolicyForV1`) held entries are judged only against the proportional cap `heldCapBytes`, which is far above the store, so `capShortfallBytes` stays 0 and `evictHolders` never runs. The shortfall therefore never reaches the least recently used holder entries: a persistent reserve violation cannot be repaired, the guard refuses every put (507) forever, and the blob bound of the server collapses to its 5 % floor.
H17/H40 named this ("nothing evicts the holders") and left it open.

## Root cause 2 (upload-entry-gone)
Burst, not a steady storm: fleet 1,014 / 1,048 / 1,499 in 15 min / 60 min / 24 h; m10 558 / 560 / 566, m15 456 / 456 / 456 (all in one 15 min window; every other origin 0-30). Mechanisms found in code (the worker logs themselves are not reachable from the coordinator, so the exact event on m10/m15 is NOT proven):
1. Spill (m10 is `image-short`, 830 of 16,183 MB free): units the host store refuses live in `w/spill` with their entry. The folder bound (`spillBoundBytesV1`, volume driven) evicts the least recently used entries older than `protectMilliseconds` (15 min) whenever a new unit spills. A job waiting longer than 15 min (a server refusing with 507 -> backoff up to 5 min, 24 h age) loses its entry and objects to the next spill. Nothing tells the spill folder that an upload still waits. -> `upload-entry-gone`.
2. Starved refill: the drainer lists the spool (and pins the objects of new jobs) only in the branch `queue.length===0`. While a backlog keeps the pool saturated (`running.size>=maximumUploadJobs` -> `continue`), new jobs are neither listed nor pinned for the whole backlog (hours during a 507 outage); host store objects of such jobs are only protected by the 3 min minimum age.
3. Pins of a dead drainer vanish at once (pin belongs to the pid); unchanged, covered by 2 and the restart by the sidecar.
H3's 16,384 pin limit is not the cause: the largest spool seen is 119 jobs now (`offload.spool depth 119, bound 32,367`), pinned 585 objects on the coordinator.

## Fix (staged, one changeset; lands via `repo land --outage`)
1. `🏗️build/🗄️cas/🌡️partition/🟦️.ts`: on a role with `server:true` (cache-host), when the store is above the cap in force, held entries are judged against that cap, never below `holderProtectedShare` (40 %) of the proportional cap. The existing capacity stage (`evictHolders`: least recently used entries first, 40 % protected floor per holder, never an entry that holds a pinned/leased/young/linked object) then frees the reserve shortfall in `incrementBytes` steps on the next enforce tick or `repo cache cas gc`. Other roles and a store inside its cap keep the proportional cap.
2. Uploader: the drainer lists the spool and pins every `pinRefreshMilliseconds` even while the pool is saturated (so a new unit is pinned within 30 s whatever the backlog); the saturated wait is bounded by `waitingPollMilliseconds`.
3. Spill: `touchSpilledEntryV1` (store) marks the spilled entry and objects of every waiting job as read; the drainer does it every 5 min (a third of the protection), so a spilled unit that waits for the fleet cache is kept; the spill bound can then only refuse a NEW spill (the unit is not shared, counted) and never destroys a waiting one.
4. Tests: cas tests (held cap of the cache host above/inside cap, other role, updated 228 GiB test), spill test (touched entry stays, untouched older one goes).

## Gates and measurement
- Changeset A (outage, staged `T/🧰️scripts/H105/stage3-cas-outage`: `🗄️cas/🌡️partition` + cas tests): typecheck passed, debug-lines/dangling passed; cas tests 76/76 green when run directly in the overlay and 75/75 green live, but `repo land --outage` REFUSES (3 runs): in the gate overlay two tests (`workspace source closures...`, `operator collection enforces the role capacity...`) see the overlay as workspace (205 MB of sources counted as protected) and fail. Not related to the change (worker role, no server); the land policy `gates.workspaceBound` has no marker for them. NOT LANDED. Needs a coordinator exception apply (as in landings.md "Manual follow-up (coordinator exception)") or a marker; nothing else blocks it.
- Changeset B (staged `T/🧰️scripts/H105/stage4-uploader-spill`: uploader refill/pin, `touchSpilledEntryV1`, spill test): typecheck passed; new spill test passes in the overlay; 4 uploader tests and the existing spill test fail in the overlay for overlay-environment reasons (counters/fleet facts read the real machine, spill bound 0 on this disk) and pass live (13/13): unproven that my change is innocent there, so B is NOT LANDED and needs the same decision. Product landing, not outage: via H101.
- Nothing from this lane is live. The cache server keeps old code.

## Before / after (23:10 vs 23:56 CET)
| Measure | Before | After |
|---|---|---|
| Volume free | 66.6-71.9 GiB (reserve 74.1) | 79 GiB (df, others' cleanup; `repo cache gc --idle` and `--reclaim=12GiB` freed about 0.3 GiB: machine bounds only, held objects untouched, reclaim exit 1) |
| 507 rate (server log) | 850-1,090 per min, 38k+ per 40 min | no 507 line after 22:54Z (last: 954 in 62 s) |
| CAS used / cap | 54.7 GB / 46.98 GB (116 %) | usage hint 58.2 GB, cap follows free: store still above cap until the code lands; 507 stopped because free crossed the reserve |
| upload-entry-gone 15 min | 1,014 (m10 558, m15 456) | not re-measured (needs workers on new code) |
The relief is incidental: the held 52.7 GB stay, so the next dip below the reserve repeats the outage until changeset A is applied. Graph image `g` (27 GiB physical vs 6 GiB declared): not touched, no compaction command found in `repo cache --help`, lane H106 owns it.

## Unverified
Exact event that removed ~500 spooled entries on m10 and m15 (worker logs unreachable); spill pruning is the best-fitting mechanism. Held eviction on the cache host not exercised on live data (tests only: cap judgement and planner).

## Update 23:12 CET (coordinator exception)
- Changeset A APPLIED manually under `landing.lock.cas` (atomic temp+fsync+rename, lock released), landings.md entry written. Live: typecheck passed, cas tests 76/76 (includes the two overlay-mis-counted ones).
- Effect of A: NOT yet visible. The cache role (pid 23603, code f3ed17f40a95) is flagged stale since 23:01 (disk 2a0155...) but had not handed over by 23:12; `repo cache cas gc` is refused while the server's collector holds the lock. At 23:11Z the server still refused ~600 writes/min (4,630 since start); CAS usage hint 58.2 -> 66.6 GB (growing), free 74 GiB (reserve 74.1). Re-read `v1/census.json`/`usage.json` after the handover line "cache handover: successor" appears: expect held bytes to fall in 2 GiB steps toward the cap.
- Changeset B (staged `T/🧰️scripts/H105/stage4-uploader-spill`: uploader, store touch, spill test, plus `gates.workspaceBound` markers for the 2 cas tests and the 4 uploader tests that read red only in the overlay). Innocence proof (overlay, same gate runner): WITHOUT B (stage0, only a blank line in the two test files) the overlay shows spill 1 fail (pre-existing, same on live) and uploader 9 pass / 4 fail with the same four tests as WITH B (9 pass / 4 fail; spill + my new test passes); live uploader 13/13. Hand B to H101 (landing train): `repo land check` on it defers the 6 marked tests to live.

## Update 23:18 CET: hand-over and effect of A
- No supported control triggers a cache-role hand-over (`repo service host` is the graph service). The sidecar restarts itself after its quiet period / 9 min maximum deferral: "cache code changed, restarting (deferred 780 s ...)" at 23:14:25Z, successor (pid 1186) listening 23:14:46Z, code `2a0155...` current. Waiting cost 13 min after the apply (stale since ~23:01); proposal for H101: when the host is below its reserve (`reserve distance < 0`) and the cache role is stale, skip the deferral and hand over at once.
- 507 rate: 850-1,660/min until 23:14:46Z (last line 314 writes in 51 s, 7,562 since the earlier start); none since 23:14:48Z (3+ min, checked 23:18).
- Held bytes: NOT falling: 55.6 -> 55.8 GB held, usage 66.1 -> 63.7 GB (transients), free 74 GiB. The outage ended because free crossed back over the 74.1 GiB reserve (cap in force 61.5 GiB vs store 61.55), so the store is no longer above the cap and the new held-entry stage had nothing to do; it triggers on the next dip below the reserve. Its eviction has only been exercised by the tests, not on live data.
