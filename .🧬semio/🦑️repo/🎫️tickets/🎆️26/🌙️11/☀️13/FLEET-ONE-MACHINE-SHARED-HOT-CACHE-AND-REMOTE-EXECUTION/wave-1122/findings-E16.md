# E16 findings (disk compaction)

## 02:00 CET plan and lock request
- Need area locks `native-image` (E13 holds it, 00:51Z) for `♻️reclaim` (`nativeImageReclaimDueV1`) and `cache-gc` / `cas` (free). E13: please release `native-image` and `caching-storage` as soon as you land; I will touch only `nativeImageReclaimDueV1` + its tests in `native-image`, nothing in `caching-storage` unless the bloat-rule policy needs a number.
- Facts measured: CAS guard in tree already derives its critical share from the partition size (1 - cushion 1.5 GiB / total = 99.8 % on this volume). At 33 GB free it admits a 20-byte write in 4 ms in a fresh process. The cache store holds 37.3 GB against a 32 GiB cap (cap-exceeded, not partition-critical); that is a likelier cause of PUT waits than 97 %.
- Why compaction does not run today: `repo cache gc` already calls `reclaimImageIfQuiescentV1` per partition, but the bloat rule `nativeImageReclaimDueV1` demands allocation >= 300 % of used even on a tight host. Measured: d 17G/7.2G (236 %), g 14G/7.3G (190 %) are never due; only r 20G/2.8G is.

## 02:03 why the automatic CAS collection never ran
1. `guard.admit` passed the writer's AbortSignal into `collectCasV1`. The cache's write budget is 10 s; a scan of 40 GB at load 30 takes minutes, so every PUT aborted the collection (partial, ~nothing freed) and then itself got 507. FIXED in `cas/guard` (collection is detached; the writer only stops waiting). Test added.
2. `casEnforceTickV1` (the periodic enforcement tick) has NO caller anywhere; the only trigger was a write. TODO in `remote-cache/🖥️server` maintenance interval (held by E10): call `casEnforceTickV1({root: hostStore.root})` next to `opened.refresh()`. Asking E10 to release `remote-cache`.
