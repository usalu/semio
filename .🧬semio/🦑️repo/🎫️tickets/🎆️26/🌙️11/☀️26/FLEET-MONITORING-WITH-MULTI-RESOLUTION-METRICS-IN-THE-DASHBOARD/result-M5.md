# Result M5: persisted counters, local hit counters, MA2 defects

Paths are relative to the worktree root. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `C` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching`. Schema: `B/📈️counter-ledger/🧬️schema/🔣️.json` (Ledger, Policy). Code: `B/📈️counter-ledger/🟦️.ts` (`openCounterLedgerV1`, `countEventV1`, `readEventCountsV1`).

## Counter files M3 reads (all monotonic, never reset)

### A. Ledger files (JSON, atomic replace at most every 5 s, only when a count changed)
Shape: `{version:1, name, sinceMs, updatedMs, restarts, groups:{<group>:{<count>:int}}}`. `groups` is already cumulative over every run (ledger plus the run in progress is flushed). `restarts` counts process starts, `sinceMs` is the first start. A rate = difference of two samples; a drop never happens.

1. Remote cache server: `<cache data dir>/v1/counters.json`, default dir `.🧬semio/🦑️repo/⚡️cache/remote-cache` (policy `remoteCache.server.directory`, `--directory` overrides), name `remote-cache`.
   - groups `ac`, `cas`, `nx`: `hits, misses, puts, conflicts, rejected, stale`.
   - group `peers` (only when peers are configured): `hits, misses, errors, skipped, absent, bytes`.
   - Same numbers cumulative in `GET /status`: `counters` (cumulative), `session` (this run), `ledger:{sinceMs,restarts}`, `peers` cumulative. Divergences were already persisted by the provenance sheet (`divergences.count`, `byClass`, `since`), unchanged.
2. Remote execution scheduler: `<sidecars dir>/build-scheduler.counters.json` (sidecars dir = `.cache/f/sidecars` or the dir of `SEMIO_FLEET_SIDECAR_RECORD`, beside `build-scheduler.operations.json`), name `remote-execution`.
   - group `scheduler`: `completed, cached, failed, cancelled, retried, deduplicated, refused, offloadRejected, cancelledQueued, cancelledExecuting, workersReused, sweepStalls, restored, resumed, submitted, assigned`.
   - groups `offload.<origin>` per machine: `offloadedIn, offloadedOut, refused`. The machine rows of `fleetLoad` and the worker rows of `status()` show these cumulatively.

### B. Per-event counter files (one byte appended per event, file size = count, file `since` = ms of first count)
3. Unit cache (already existed): `.cache/f/unit-counters/{localHit,remoteHit,miss,stored}` plus `skip/<reason>`; mirrored by `.cache/f/build-counters.json` field `unit` (`localHit, remoteHit, miss, stored, skipped`), `wallMs` since first count. `localHit` = unit cache restored-local.
4. Nx (new): `.cache/f/nx-counters/{nxLocalHit,nxRemoteHit,nxExecuted}`, written by `C/🚀️bootstrap/📊️nx-cache` from the output of every Nx process the bootstrap starts (task lines with `[local cache]`, `[existing outputs match the cache...]`, `[remote cache]`, else executed).
5. CAS local (new): `<CAS root>/v1/counters/{localHit,localMiss}` (root = `SEMIO_CAS_ROOT`, else `~/.cache/semio/cas`, `%LOCALAPPDATA%\semio\cas` on Windows), one byte per `get`/`read`/`link` that found / missed its object (`B/🗄️cas/🗃️store`, `casCounterDirectoryV1`).
Reader: `readEventCountsV1(dir, names)`.

## MA2 defects
- `pool.json.<pid>.tmp` orphans: the pool status writer (`B/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/📊️status/🟦️.ts`) now removes its own temp when write or rename fails (Windows rename over an open file was the 852-file source on g1-m05). New member `🧹️origin-files` retires, once per hour per pool process, every `<name>.<pid>.tmp` (also `fleet-auth` `-<hex>` form, ready, memory, ready-log, slice-memory) in the own origin dir whose pid is dead or older than 1 h.
- `pool-published-*.txt`: same member removes files older than `publishedRetentionMilliseconds` 7200000 (schema `🧹️origin-files/🧬️schema/🔣️.json`; a test guards it never drops below the reader age of the take schema).
- Not changed: the ready/attest writers themselves still lack the catch-cleanup; their orphans are now swept by the pool.

## Tests (repo test-file)
counter-ledger 8/0; cache server counters 1/0 (server 10/0, restart 3/0, peers 15/0 still pass); origin-files 5/0 (pool-status-read 3/0, early-wave 7/0); cas hits 1/0 (cas 70/0, full-disk 17/0, pins 2/0, holders 13/0); nx-cache 2/0. All cross-checked with Python oracles and Ajv. dangling-imports exit 0 for every touched folder.

## Not verified / notes
- Successor server overlap (two servers briefly on one dir) can lose the old server's counts of those seconds (last writer wins).
- Pre-existing, not mine: whole-tree type check fails with many errors (e.g. `remote-cache/🖥️server/🟦️.ts` `releaseLease` on `hostStore` Pick); none in files I touched.
- No live restart of the real servers was done.
