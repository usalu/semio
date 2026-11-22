# Result E74: unit-cache skips (`unrecorded`)

## Finding
`unrecorded` is not a skip class. It was `skipped counter - sum(reason files)`: the residue of skips counted by the wrapper before reason files existed (before E4's 02:59 landing; mostly outage skips with the breaker open, which E4 already stopped). Proof: on the coordinator the `skipped` file last grew 11:54 together with reason files; `unrecorded` stayed 5677 for the whole session; on m08 reasons grew 926->958 in 8 min while unrecorded stayed 1651. Local events since 03:00: 121 misses, all published, zero silent skips. All current code paths record a reason (checked every countSkipV1 site).

## Changes (caching/cargo/unit-cache)
- counters: `skipped` has no file of its own; it is the sum of the reason files. A skip without a reason cannot exist; `accountedSkipsV1`/`unrecorded` removed; a stale legacy `skipped` file is ignored.
- script.ts: two previously silent non-sharing paths now count a precise reason: `unreadable-source`/`unreadable-extern` (unit compiled unkeyed) and `publish-error`.
- uploader: offers of local hits may fill only half the spool; `maximumSpool` 512 -> 2048 (jobs are pointers, bytes stay in the local store), because `spool-full` is the real live loss class.
- Tests: counters sum vs independent python reading of the files (also beside a legacy skipped file), offer-cap test, ledger test via countSkipV1. counters 3/0, portable 27/0, uploader 4/0, fleet-status 16/0, unit-cache fleet 18/0.

## Live proof
m06 picked up the code within minutes: `unrecorded` 20828 -> 0 and skipped now equals the reason sum. Other machines follow when their wrapper restarts.

## Important, NOT solved
m06 now shows `spool-full` 50412 (it was 12 before). Real class: units compiled but not queued because the spool never drains on m06 (uploads blocked/denied?) or the counter reads a reset directory. Hermetic units are lost for the fleet there. Needs an owner: inspect m06 spool and uploader log. m10 has spool-full 9445, m12 836.
Other live classes (precise, not hermetic): key-incomplete (dep-info reads unkeyed files), path-dependent, bypass-no-artifact-messages, diverged (E66).

## NOT verified
- Fresh-local-unit-store `repo cargo check` run: not done (load ~38; repo cargo may still be refused by the builtin-read coverage fault). "Shared hits rise" is unmeasured.
- Backfill of units that were skipped before: only the existing offer-on-local-hit covers them.
- `skipNameV1` still maps an empty reason to `other` (no live occurrence).

# Round 2: spool-full

## Cause (code + measured)
The upload drain ran jobs in barrier batches (6 names, 3 workers, wait for the whole batch). Local uploader events: median upload 129 ms but max 57 s, queue wait max 190 s. One slow upload held the batch (up to the 120 s job timeout) while a compiling machine kept enqueuing; the 512-entry spool filled and every further compiled unit was dropped as `spool-full`. The drain also re-parsed every spool file each 200 ms loop. Not disk-full, not a refusing cache. The spool is sized by count, but entries are ~1 KB pointers (bytes stay in the local store), so the count was simply far too small.

## Fix (caching/cargo/unit-cache/fleet, landed 16:32, see landings.md)
- Sliding pool of `maximumUploadJobs` = 8: a slow upload no longer stalls the others; due-times cached by file mtime.
- `maximumSpool` 4096 (offers of local hits at most half), job age 1 h so an outage backlog is retried instead of expiring after 10 min.
- Only real progress resets the idle timer. My first version reset it on deferrals; the fleet e2e test caught it (a wrong-credential uploader never exited and held the lock).
- Tests: uploader 5/0 incl. new stalled-upload test (seven other uploads finish while one hangs 2.5 s), fleet e2e 18/0 (298 s), portable 27/0, counters 3/0.

## Live
- Coordinator machine: `unrecorded` gone, skipped 13, `spool-full` none.
- m06 `spool-full` stayed at 50412 and m10 at 10292 from 16:01/16:32 to 16:47 (no growth), but both machines were mostly idle, and m06/m10 may not run the round 2 wrapper yet (it takes the code when its wrapper restarts). So "stops growing" is observed, not proven under load; "uploads resume" is NOT proven live. Recheck m06/m10 `spool-full` and `stored` after their next big build.
- `repo cargo --store=shared --configuration=prototype -- check -p semio-framework-pack-json-source --lib` finished OK in 6 min 17 s at 16:47 (before the seal window). It used the shared store, not a fresh local unit store, so "unrecorded drops / shared hits rise" is not measured by it; the unrecorded drop is shown by the counters above.

## Not done
- Fresh-unit-store run and shared-hit-rate measurement (seal window 16:50-17:30 forbids `repo cargo` here).
- Retry bound: a job still gets 4 attempts with exponential backoff; a cache outage longer than 1 h still expires jobs (counted `upload-expired`, units stay local and are re-offered on the next local hit).
