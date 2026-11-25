# W18: units compiled but not shared (spool-full, upload-failed, upload-entry-gone, key-incomplete) (2026-11-25)

UC = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/`, CAS = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/`. Locks `lock.cas-admission`, `lock.unit-upload` taken and removed. Two landings in `T/landings.md` (W18 09:04, W18 09:05-09:40). Nothing restarted, no GC against a live store, no scratch build. Scripts: `W18/sample.ts` (one sample of every machine's counters from the scheduler's load table), `W18/rates.py` (growth between samples), `W18/debug-inflight.ts` (collector repro); samples in `W18/🗑️generated/samples.jsonl`.

## 1. Cumulative or current?
Cumulative. Each reason is one file under `<worktree>/.cache/f/unit-counters/skip/<reason>` that grows by one byte per unit (`UC🛰️fleet/📊️counters/🟦️.ts` `countSkipV1`). The files count from `unit-counters/since` (coordinator: 2026-11-13). The load report reads `build-counters.json`, and that file is rewritten only at the end of a build or a drain, so `repo fleet status` lags until a machine finishes one. (`repo fleet status` itself fails right now with "/cas/leases/expired must be at least 0", a census fault outside this lane. `sample.ts` reads the same table directly.)

Growth per reason, from three snapshots (H1 11-23 15:34, coordination 11-24 08:12, W17/this lane 11-25 08:55):

| reason | 11-23 15:34 -> 11-25 08:55 (41 h) | after U1 (11-24 08:12 -> 11-25 08:55) | verdict |
|---|---|---|---|
| spool-full | +1 fleet-wide (m08) | m02 53,450 -> 53,450 | history only (old 4,096-job spool plus outages; U1 fixed it) |
| upload-failed | m07 +8,476, m01 +1,607, m08 +1,401, m02 +1,121 | m07 21,287 -> 21,294 (+7) | mostly history (old 4-try cap during 507/503). The live remainder is oversize units, see 2b |
| upload-entry-gone | 0 -> m01 10,804, m08 7,412, m06 3,320, m02 2,276, m10 2,101 (25,986) | all of it: U1 introduced this reason at 11-24 08:5x | **live, the biggest loss after U1** |
| key-incomplete | m08 +4,496, m01 +3,200, m02 +2,159 | m08 8,686 -> 8,894 (+208) | mostly before K7; the coordinator's last one was 11-24 09:02 (file mtime) |

The live window measured 08:55-09:35 today: the fleet was idle except g1-m06, which finished a build. g1-m06 ran code f723f45de487, which is not this lane's. It compiled **14,523 units** and lost: upload-failed **74** (0.51 %), path-dependent 21, bypass 19, publish-refused 7, diverged 6, upload-entry-gone **4**, spool-full **0**, key-incomplete **0**.

## 2. Root causes
a. **upload-entry-gone: the host store's capacity stage removed units it took for dead.** Code: `CAS🧹️collector/🟦️.ts` `evictHolders`, before the fix.
- The collector reads the held entries before its scan and again before eviction.
- An entry published while the scan was walking the store names objects the scan never put into `heldBlobs`.
- `planCasHolderEvictionV1` counts an entry whose digest is missing from that map as "dead". When the need cannot be met, the walk takes every dead entry, so it removed the entry files of units compiled minutes earlier. Their objects stayed on disk.
- Evidence on the coordinator, 11-24 between 10:57 and 11:04 (one collection): 63 `upload-entry-gone` events for units that had waited 1.1 to 12.8 minutes. That matches one collection's held walk plus scan time at that load. One traced unit (`semio_framework_value_derive_expansion`) was published at 10:51:12, deferred while the cache was down, and gone at 10:57:12.
- Reproduced: `debug-inflight.ts` showed `evictedEntries:4, fresh:false`, and the new holders test fails on the old code.
- Second path: a unit that waits for hours (the 507 outage) becomes least recently used, so the capacity stage can legitimately take it before upload.
- Third path: a spilled unit (objects in the spill folder because the host store refused writes) had its entry in the holder index. Collectors judged it dead, and `pruneEntriesV1` deleted it.

b. **upload-failed is now mostly "too large".** The client capped an object and an entry at 256 MiB (`UC🛰️fleet/🧬️schema` `limits`), while the server admits 1.25 GiB (CAS7). `semio_hub_cad` is 272 MiB and `semio_hub_cad_component` 256.7 MiB (measured in the coordinator's entries). They were refused locally on every compile, counted as `upload-failed`, and so the most expensive crates were compiled again on every machine. The coordinator's last 6 `upload-failed` are exactly these two crates. In addition, any exception while reading or packing was dropped as `upload-failed` instead of retried.

c. **spool-full**: no current cause. The U1 spool bound is 1 % of free disk / 8 KiB (coordinator 37,405 jobs). A full spool makes the compile wait for up to 30 s, and jobs that must wait are rewritten in place. The only remaining drop is "spool at bound AND cache down", and no machine has hit it since 11-23.

d. **key-incomplete**: after K7, only a missing dep-info or more than 512 files / 64 names leaves a unit unshared. The rate is 0 of 14,523 on m06's build, and nothing on the coordinator since 11-24 09:02. Nothing to fix.

e. **507 until 08:50**: a refused upload (`capacity`) is retried with backoff up to 5 min and a 6 h job age (U1). The units were not lost to the 507 itself. They were lost because the local store removed the waiting entries (2a).

## 3. Changes
1. `CAS🧹️collector/🟦️.ts` `heldOnDisk`: a held entry's digest that the scan missed is checked on disk with one `lstat`. If the object is present, the entry is alive and not freeable in this pass, so only a really lost object makes an entry dead.
2. `UC🛰️fleet/📤️uploader/🟦️.ts`:
   - The drainer pins the host-store objects of the oldest 4,096 waiting jobs (holder `unit-spool-<root hash>`, 1 h lease, refreshed at most every 30 s, released when the drainer exits). No collector, sweep or capacity eviction can take a unit before it reached the fleet.
   - New outcome `oversize`, counted as `upload-oversize`.
   - A throwing read or pack is `retry`, not a drop.
   - A process-wide in-flight byte budget (`limits.maximumUploadBytesInFlight` 1 GiB; a larger unit runs alone), so raising the limits cannot hold several GiB in memory.
3. `UC🛰️fleet/🧬️schema/🔣️.json`: `maximumEntryBytes` and `maximumBlobBytes` raised to 1,342,177,280 (the server's largest entry), plus the budget and pin settings.
4. `UC💾️store/🟦️.ts`: a spilled unit's entry is written beside its spilled objects (`readSpilledEntryV1`), outside the holder index. The uploader reads it there and drops it with the spill.

## 4. Tests (`repo test-file`, one at a time)
- CAS holders 13/0. New test "an entry published while the collector scans is alive": 12/1 before the fix, 13/0 after.
- CAS main 66/0, full-disk 17/0.
- Uploader 11/0, with 2 new tests:
  - Bounds equal the build policy's `remoteCache.server` entry maximum (independent source). An over-limit unit ends once as `upload-oversize` and the unit within the limit arrives.
  - A waiting unit pinned during the drain survives a capacity eviction at cap 1 byte, while an unspooled control unit is evicted. The pin leaves with the upload.
- Spill 1/0 (updated: the entry is outside the index and survives a prune), registry 2/0, pipeline 7/0, fallback 4/0, complete 10/0, stale 1/0, rooted 4/0, session 2/0, unit-cache main 28/0.
- Fleet main: 18/0 in the last 3 runs in a row. The 3 runs before failed the known "a refusal is re-checked ..." case ("uploads did not settle", the K7/T3 flake), once together with the manifest-directory case. The cause was not identified. The helper's error now names the waiting jobs and the last uploader events.

## 5. Rates falling
Not shown yet. I promised falling rates with numbers and can't give them yet: no machine has built on the new code since it landed (fleet idle; m06's build ran on f723f45de487).
- The CAS fix takes effect at the cache sidecar's next self-restart and at each worker agent's restart.
- The uploader and limits take effect at each worker's next build (the wrapper re-seals and spawns a new drainer).
- Measured before the change, on m06's build: entry-gone 4 and upload-failed 74 per 14,523 compiles. The 41 h before that: 25,986 entry-gone.
- To check: run `W18/sample.ts` before and after a worker build, then `python3 W18/rates.py`. Expected after: `upload-entry-gone` about 0, `upload-failed` about 0, `upload-oversize` only above 1.25 GiB (none known), spool-full and key-incomplete staying at 0.

## 6. Open
- The pin covers the oldest 4,096 jobs per root. The pin file is about 1.1 MB at the cap, and the collector re-reads pins per shard.
- Build-script runs are not pinned (their entries are small and are read through `newestBuildRunV1`).
- `repo fleet status` schema fault (negative `cas.leases.expired`): another lane's census edit in the collector is in flight.
- Flaky fleet-main case above.
