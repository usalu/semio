# Resident Slice Root Tail Custody

The resident slice pool stores root serialization promises in the named bounded LRU `host:slice-roots`, with 64 entries and 64 bytes. Each new root calls `tails.set`; the generic cache evicts its least recently used entry when the bound is exceeded regardless of whether its promise has settled. A pending root A can therefore be displaced after 64 other distinct root insertions, even when those other turns have already settled. A later A lookup sees no predecessor and can enter concurrently. The retained promise is coordination, so ordinary value-cache eviction is not a safe lifetime rule.

The only production direct caller of `shrinkBoundedCachesV1` found under the repository modules is the project-farm worker service `trimRetainedV1`, called after a child job. The slice pool and farm operate in separate processes, and the bounded-cache registry is local to its module instance and process. This source evidence does not establish that this shrink reaches the resident host tail cache. LRU capacity eviction is an independent source-level path.

The 20:00 UTC m07 receipt described a follower waiting behind one same-checkout leader; the leader inventory rose from 69,539 to 69,540 observations between two samples. It did not expose the leader PID or resident tail cache entries and did not show 64 other roots or same-root concurrent workers. The eviction defect cannot be attributed as the cause of that physical wait from available evidence. No cache shrink, service action or worker process change was performed.

## Workspace serialization regression

The resident pool records each checkout's active and queued tail in a map and removes it only after that exact chain settles. This limits entries to live or queued checkout roots. Cleanup from an earlier request cannot remove a later request's tail. A canceled waiter keeps the predecessor's custody in place until the predecessor finishes.

Both cases failed against the bounded cache. An explicit cache trim and 64 sequential unrelated checkout loads each let the second same-checkout load start before the first released the fact store. An AJV-validated JSON fixture specifies the request order, with SQLite independently selecting the eligible requests. The production queue, slot transfer, reservation and restart suites passed 22 tests with 175 assertions after the change.

The current m07 workspace run reported a leader holding the checkout slot. The fixture proves the eviction bug and its fix. Fleet telemetry has not established an eviction or overlap in that specific m07 attempt.

## Global slot during replica-restart backoff

A slice that hit the restart exit code retained its global worker slot after its process and reservation had already been released. A one-slot regression observed one occupied slot at the first backoff heartbeat. The pool now gives up that slot before backoff and rejoins the normal worker queue afterward. The same checkout's ownership stays with the original request. The regression uses a schema-validated JSON schedule and SQLite's independent selection of the eligible unrelated checkout, then checks exact subsequent order. It also checks that the pool ends with no occupied or queued slots. The timer removes its abort listener when each interval ends.

The combined restart, queue, slot transfer and reservation run passed 23 tests with 180 assertions in 3.79 seconds. Pool fixtures inject a fake reaper because their fake children do not have real process trees. An earlier run with the real reaper timed out in the 64-root fixture under the five-second default. The private regression does not measure elapsed savings on a live worker.
