# CAS8: host store at the hard reserve (2026-11-24 18:09-)

B = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. Landing: `T/landings.md` (CAS8). Lock `lock.cas-admission`.

## 0. Coordinator items (18:11)
- Collector pid 37192: sampled 5 s: main thread idle in `kevent64`, thread pool in `lstat` (281) / `stat` (96) / `openat` (84) / `unlink` (24): it was walking the store, not stuck. It freed little because of the costs in section 1.
- Pin `remote-cache-live-76726.json`: 0 digests. 76726 is alive: the local remote-cache server of `repo service host run` (listens on localhost:6090). It holds nothing, so dropping it would free nothing; no stale-pin rule needed.
- Freed now: `gc --unreferenced` no longer stops at a busy lock (section 1, lockless unreferenced pass); a manual run 18:15:16-18:28:01 removed 5,160 objects, 2.75 GiB (94,400 kept, 765 s incl. 120 s lock wait). Free 74.3 GiB (18:15) -> 80.2 GiB (18:28) -> 83.2 GiB (18:41), the rest by the sidecar's collections and the build's own churn.
- "17.34 GiB referenced by nobody" was not one class: the old status computed it as total minus (pins + holders + every lease row, live or ended). The census (section 4) splits it; at 18:35: expired 3.33, young (< 180 s, in flight) 5.52, unreferenced 8.68, foreign-linked 0 GiB.


## 1. Why the sweep did not keep up (measured)
Sources: sidecar log `.cache/f/sidecars/build-cache.log` (stall profiler; no sweep journal existed), `sample` of 37192 and of the manual gc, `du` of the store, the lock heartbeat.
| cost | measurement |
|---|---|
| keeper reconcile at the start of EVERY non-dry collection: one `linkSync` per held digest (14-20k), synchronous | 33 event-loop stalls 11:33-17:46Z, median 26 s, max 50 s, 881 s in all; one per full collection (about every 11 min). The cache server answers nothing during it. |
| store walk (lstat of every object) | `du -sk v1/o` 34 s at load 68 for 106,740 objects; the JS walk is slower. A collection walks twice (scan + evict pass), the tick walked a third time (`guard.remeasure` = `casUsageV1`) on every 30 s tick. |
| re-check before every removal: `stampCasTransientsV1` = readdir + lstat of all 33 lease segments, plus pin-dir stat and object lstat | sample of the manual gc in its evict pass: 160 lstat per 18 unlink (about 9:1). 5,160 removals + 99,560 scanned took 645 s (765 s incl. 120 s lock wait) = 2.75 GiB in 10.75 min, ~15 GiB/h at best. |
| the sweep could not run | `guard.sweep` asked the lock with `waitMs` 0, so every tick in which an automatic/admission collection (9.5-12 min each) held the lock returned `locked`; and it ran only after `remeasure` + `admit(0)` + `settle()` of that collection. A step was capped at 4 GiB. |
| "12.7 / 17.3 GiB by nobody" | not one class: the old status counted total minus (pins + holders + every lease row). The census of 18:35 splits it: expired 3.33 GiB, young (< 180 s) 5.52 GiB, unreferenced 8.68 GiB, linked 0 GiB. So about 12 GiB were collectable, the rest in flight. |
Net: the sweep ran rarely, each pass took ~10 min of walking, the server stalled ~30 s per collection, and inflow (~15 GiB/h release + inputs) matched the best-case throughput.

Fixes (B`🗄️cas/`):
- `🧹️collector/🟦️.ts:170` an unreferenced-only pass (sweep and `gc --unreferenced`) runs without the lock when another collector holds it (same rules as the full-disk lockless path: every removal re-checked; it skips compaction, keeper reconcile and never touches held/pinned/leased/linked/young objects). A full collection still waits (`locked` test kept).
- Pre-removal recheck batched: per shard or per 256 candidates one lease-stamp/pin-stamp read, then lstat + lease + pin check + unlink each (~35 -> ~2 syscalls per removal). `beforeEvict` runs before the batch check, so "a lease taken while the collector runs protects the object" still holds (test green).
- Keeper reconcile at most once per `holderRenewMs` (15 min) per root and process (`:40`), and `🫂️holders/🟦️.ts` links 256 at a time with an event-loop turn between (no 26-50 s stalls). Writers already link keepers on every entry write/hit.
- `🚦️guard/🟦️.ts:263` while free < hard reserve the tick sweeps FIRST, step unbounded, lockless; the per-tick store walk runs only when no usage hint is younger than two sweep periods.
- Expected throughput: one sweep = one walk + removals at ~2 syscalls each; at the measured walk cost (~5-10 min at load 70) and unbounded steps below the reserve that is all expired+unreferenced data per pass, i.e. far above 15 GiB/h whenever there is that much; above the reserve 4 GiB per pass every minute-tick that finds the lock busy or not.

## 2. Transient inputs: who holds them, for how long
- Holders: the requester lease `input:<action>` (30 min fixed by the server, `transientLeaseMs`, old clients never end it) and since O25 the scheduler's `input:op-<action>` for the operation's life (renewed every 600 s, released at finish). The requester lease was redundant once the scheduler holds the operation.
- Fix (`📌️pins/🟦️.ts:161` `casLeaseUntilV1`, schema `Policy.requesterLeaseMs` 600000): applied when the ledger is folded, so it binds every line already written by old clients too: an `op-` lease counts as written; a requester lease (named or anonymous) counts at most `requesterLeaseMs` after it was taken and never beyond the operation lease of its own action once that exists (operation released -> requester also down to the 60 s grace). Server code needs no change; the play clone's 30-min leases are cut to <= 10 min and to op end + 60 s.
- Live: census 18:35: leased 2,524 objects 3.02 GiB (was 34,448 objects 31.31 GiB "leased or expired" at 17:51), expired 3,836 objects 3.33 GiB.
- Dedup: the store is content-addressed (path = sha256), so an input set uploaded by 8 concurrent finals is one object per digest; a second PUT of a present digest only re-leases (server writes the lease before answering HEAD/PUT). Input blobs that are also unit-cache artefacts are the same object (same digest) with a keeper hard link (`v1/k`), not a second copy: `k` = 47,734 links, `du` counts them separately but they share inodes with `v1/o`.

## 3. Reserve semantics (rule as stated by the coordinator 18:2x)
Rule (`🚦️guard/🟦️.ts` `createCasGuardV1` doc): a write that would leave less than the hard reserve free, transient inputs included, is admitted only after the shared unreferenced sweep (expired inputs + data nobody references; never held/pinned/leased/linked) freed enough to keep the reserve; else it is refused (507). Below the reserve the store never grows and never evicts a held entry to make room. The admission waits on ONE shared lockless sweep (`state.sweeping`, budget `admissionBudgetMs`) instead of a full collection (which needed 10 min and the lock).
Test (cas): "below the hard reserve a transient input is admitted only after the sweep freed enough unreferenced data ...; with only pinned data it is refused and nothing held is touched".

## 4. Status in seconds
- Census (`🧹️collector/🟦️.ts` `CasCensusV1`, `v1/census.json`): every complete scan counts objects/bytes per class (pinned, held, leased, linked, expired, young, unreferenced) and per holder, minus what that pass removed; written atomically. `repo cache cas status` and the 507 detail read it (no walk, no lease-ledger read, no holder scan); `repo fleet status` (`🚢️fleet-status/📥️readers`) reads leased/expired objects+bytes from it (dead `maximumLeaseStats` removed from its schema).
- Measured: the status function 4 ms (import 120 ms). The `repo cache cas status` CLI still took 87 s at load 70, all of it in the repo CLI's own `source.capture`/`source.served` phases before the command runs (not CAS; open item).
- The census is as old as the last complete scan (status prints its age).

## Tests (`repo test-file`, one at a time)
cas 66/0 (4 new: requester lease bound, lockless sweep + census, reserve admission via sweep, status from census EN+DE; 2 updated to the 10-min requester bound), holders 12/0 (after reconcile chunking/gating), pins 1/0, server admission 3/0 (updated bound), server main 10/0, retention 18/0, fleet-status 28/0 (updated to census).
Not mine, not passing: `🌐️remote-execution/🗓️scheduler/🧪️tests` hung twice at 0 % CPU (560 s, 285 s; killed); I touched no scheduler file (its `🟦️.ts` was edited 17:38 by another lane; its tests use in-memory content, not the CAS lease fold).

## 5. Reclaimable on this host without touching a running build (listed, nothing deleted)
| item | size | note |
|---|---|---|
| host store: expired inputs + unreferenced (census 18:35) | 3.33 + 8.68 GiB | the sweep takes these by itself now (unbounded below the reserve) |
| host store: young (< 180 s) | 5.52 GiB | in flight, goes by itself if nobody leases it |
| host store: main checkout unit cache (held) | 28.78 GiB, 5,250 entries | live holder; R5 found 4.22 GiB of it held only by stale entries (local `gc.staleness` dry-run governs them) |
| host store: play unit cache (held) | 42.06 GiB, 8,404 entries | play build: untouchable |
| stale holder files `v1/h` / keeper dirs `v1/k` | 0 | both holders' roots exist; `k` has exactly those 2 holders |
| lease ledger `v1/x` | 241 MB, 33 segments | compaction drops ended lines from segments idle 1 h |
| access ledger `v1/l` | 120 MB | |
| `.cache/f/lost` (main checkout) | 884 MB, 1,228 entries | timestamped `...-g1-mNN` records; owner unknown to this lane |
| sealed wrapper dirs `.cache/b/r/m/u/sealed` | main 204 MB (35 dirs); play 46 MB (10, play's) | |
| image slack (on-disk `fixed.dmg` minus used inside) | main `g` 2.8 GiB, main `r` 2.2 GiB, main `p` 0.9 GiB; play `r` 6.9 GiB (play's) | needs unmount + compact; not while mounted |
| simulator runtime restore images (`/System/Library/AssetsV2/...iOSSimulatorRuntime`, `xrOSSimulatorRuntime`) | 7.9 + 6.8 GiB | system-managed; `xcrun simctl runtime` if unused |
| `/var/folders/.../sidecar-closure-*` | 6 dirs, 0 B | |
Also: five Nov-22 old-code sidecar agents (29501, 30712, 32321, 50317, 99865) are still alive; they free no disk but are old collectors (CAS3).

## 6. In force, before/after, play log
- Sidecar pid 18639 (handover 18:34:57) runs: lockless unreferenced sweep, batched recheck, requester lease bound, reserve-first sweep, census, 507 detail from census. Its first new-code collection: scan 18:35:05 -> finished 18:44:36 (still with one 45 s reconcile stall). The reconcile chunking/gating (`🫂️holders`, `🧹️collector` edits 18:39) waits for its next self-restart (300 s quiet, 1800 s latest: by ~19:09).
- Before (17:51, coordinator): store 109.8 GiB / 110,124 objects, free 74 GiB (0.11 GiB below the 74.11 GiB reserve), 31.31 GiB "transient", status > 5 min.
- After: census 18:35 80,557 objects / 91.4 GiB, hint 18:44 91.4 GiB; leased inputs 3.02 GiB; free 81.4-90 GiB (moves with play's image), 7-16 GiB above the reserve; status function 4 ms (repo CLI boot 87 s at load 70).
- Play log `x/offload.jsonl` (read-only): 993 x `Content store answered 507` in all, last at 17:00:45; 0 after that (log's last line 18:09:53; play restarted `site-build` 18:30:38).

## Open
1. Verify after the sidecar's next restart: no `reconcileCasKeepersV1` stalls in `build-cache.log`, census refreshed every pass, free stays above the reserve.
2. `repo cache cas status` CLI spends ~85 s in the repo door's `source.capture`/`source.served` before the 4 ms command (load 70): repo CLI boot, not CAS.
3. O25 open item 2 (restored operations re-lease nothing) matters more now: a restored queued operation keeps only its requester lease, now <= 10 min; a loss costs one O25 repair round.
4. RE scheduler test hang (above) belongs to the lane that edited `🗓️scheduler/🟦️.ts` at 17:38.
