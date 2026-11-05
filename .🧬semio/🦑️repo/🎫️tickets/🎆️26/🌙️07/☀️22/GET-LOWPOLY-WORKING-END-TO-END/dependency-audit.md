# Dependency Gate Audit

Read-only audit on 2026-11-04. No installer, cancellation, private-store mutation or infrastructure edit performed.

## Observations

Doctor returned blocked: no published generation and 36 historical unfinished generations; oldest 474639 seconds. Both findings prescribe repo dependencies ensure. Historical generations do not prove current-owner failure.

Lease snapshot epoch-ms 1793827807019: PID 2044, parent 30392, Draw guest-instance-check, sole exclusive javascript-dependency-build owner for 785 seconds. Standing progressing, dependencies:store-ownership:frontier, done 816000, silent 14 seconds. Earlier independent snapshot had the same count and silent 3 seconds. Cycles and violations empty. Parent previously observed current-input then store-input-replay. This establishes activity but not overall completion or monotonic progress.

## Source Findings

Owned repo library caching/bootstrap/dependencies/storage TypeScript lines 642-668 stages or adopts generations, runs installer, verifies providers and installed paths, writes ready receipts, then publishes endpoints and pointers. Fence at line 645 intentionally repeats ownership, source input, endpoint and immutable input receipt rechecks. Replay can recur within one successful attempt; phase labels alone cannot identify retries. Replay line 427 hashes bounded bytes and yields/checks cancellation per chunk. Installed scanning lines 33-45 bounds depth, path units, entries, directory entries and work.

Real sync retries emit explicit DATA purpose current-store-dependency-sync-retry at line 668. Incremental schema limits total attempts to three, accepting named ownership/provider/source input change patterns, excluding custody, lease, released, cancel and abort. Unresolved source transitions also prohibit retry. The retry loop is finite. Owner terminal capture was not inspected, so this audit neither proves nor denies actual retry/error records there.

Concurrent/storage lines 35-44 probes build lease without queueing, coalesces on readiness and enforces declared maximum. Storage line 83 retains one exclusive builder through the operation.

## Safe Canonical Recovery

Retain the current live owner and inspect its actual terminal and explicit retry/error records. A second ensure now would wait/coalesce. After owner termination, rerun repo doctor --only=dependencies --json; if still unpublished, run repo dependencies ensure once and rerun the waiting Lowpoly gates. This is the doctor-prescribed canonical recovery. Do not delete private generations, reset pointers, clear storage, break leases or kill unrelated owners. No current evidence justifies manual recovery or infrastructure changes.

Lowpoly validation remains pending dependency publication and the actual Lowpoly terminal results.
