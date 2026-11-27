# H3 result: fleet unit cache skip reasons

## Evidence (live, 00:03 11/27, `repo fleet status --json`; local m08 uploader log 11-26)
Per origin (skipped / hit): m01 77k/83k, m02 82k/74k, m06 74k/99k, m07 85k/117k, m08 38k/258k. Dominant reasons by origin:
- upload-failed: m02 16.6k, m07 21.3k, m08 15.5k, m01 14.7k.
- upload-entry-gone: m07 33.8k, m01 18.0k, m06 10.7k.
- spool-full: m01 27.9k, m07 15.7k, m09 10.3k. upload-expired-down: m07 10.2k, m01 7.9k.
- key-incomplete: m08 8.9k, m01 7.3k (first-build manifest learning, see below).
Local uploader log (m08, 11-26 05-09 h, 381k events): 341k `upload-deferred: down`, 36k `upload-capacity` (server 507), only 437 `uploaded`. So the uploader spent hours not uploading: the breaker is a 15 s window, every job was re-read, rewritten and logged every ~15 s (churn on a spool of up to 46k jobs, 290 MB event log), while compiles kept enqueuing until the bound was reached, then every further unit was `spool-full`. Jobs older than the age bound (6 h) were dropped as `upload-expired-down`. Jobs beyond the first 4096 (pinMaximumJobs) had no CAS pin, so host GC removed their objects before the outage ended: `upload-entry-gone`. Server now: 73.8k entries, no 507 in the current session; restarts 36 since 11-21 (the outages).
`upload-failed` was also a catch-all: an upload outcome `false` (incremental session entry or a run naming a local directory, never shareable) ended as upload-failed, hiding it.
`spool-full` was also reported for any enqueue exception, and the build-script run path (`shareRunV1`) used the non-waiting share, so it had no back-pressure at all.
Server divergences 1038 (unit 969): same key, different bytes from two origins, mostly `.d` and `.rmeta` outputs (path/embedded-hash text), many with empty `differing` (stdout/stderr or metadata). Not misses: nondeterminism across origins. Not fixed here (findings below).

## Changes
- Uploader (`📤️uploader`): while the breaker says `down` and nothing runs, the drainer waits as a whole (poll `waitingPollMilliseconds`) and touches no job file and writes no event; with `idleMilliseconds` 0 it reports every waiting job as deferred. Denied credentials keep the per-job handover (deferredBy).
- `upload-unshareable` is counted separately from `upload-failed`.
- `unsharedReasonV1`: `read-only` | `spool-full` (length >= bound) | `spool-error` (enqueue failed for another reason).
- Build-script runs use `shareFleetUnitAwaitingV1` (back-pressure, as compiled units).
- Policy (`🧬️schema`): spool.minimumEntries 4096 -> 16384, uploader.maximumJobAgeMilliseconds 6 h -> 24 h, pinMaximumJobs 4096 -> 16384.
- Test: outage test now asserts jobs are untouched during a long outage (no rewrite, no attempts, no notBefore).

## Verification
uploader tests 12 pass, 1 fail (`names the endpoint but not the access`, facts access: not touched by this lane, H2 area). buildscript fleet-run test 1 fail at `uploadUnitV1` returning "retry" with the fake client (function not edited; unverified whether pre-existing). Dangling imports 0.

## Before / after
Before: numbers above. After: NOT measured. Workers restart on replica; read `repo fleet status --json` counters (cacheSkipReasons) a few hours later: expected spool-full, upload-entry-gone, upload-expired-down to stop growing; upload-failed split into upload-unshareable. No two-machine slice build was run (coordinator load 60+, no live experiments).

## Unverified / for others
- Real cause of the outages (36 server restarts, hours of `down`) lies with the cache server host; breaker marks `down` after 2 faults incl. 20 s blob timeouts under load (could be too sensitive: untuned).
- key-incomplete: first build of a crate on a machine learns the manifest (`completeInputsV1`), the unit is not shared; a fleet manifest pre-seed would remove most of it.
- Divergence ownership: needs `repo remote-cache evidence show <id>` per `.d`/`.rmeta` class.
