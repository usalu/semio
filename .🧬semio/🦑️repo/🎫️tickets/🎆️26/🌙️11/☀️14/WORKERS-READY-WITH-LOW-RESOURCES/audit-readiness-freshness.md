# Fleet Readiness Freshness

`ready.json` is written by each worker's Ready sidecar. Its `updatedMs` is the publication timestamp. The writer publishes immediately when state, stage or readiness changes. Its five-second status interval coalesces update calls. Normal idle turns re-evaluate readiness at least every 120 seconds while the loop advances.

The existing process-tree schema defines the 600,000 ms foreground freshness window. The graph's HEAD adoption probe requires a valid origin-bound Ready receipt, an age in the inclusive range from zero to that window, and an affirmative Ready state with no active stage or refusal. The fleet display uses the same receipt schema and time window. It labels stale and future-dated receipts as last reported, keeps their stage and diagnostics available, and counts only current affirmative receipts. Missing or invalid receipts are shown separately. The table includes the worker's refusal and stage reason in the requested language, plus one bounded line of recent output. The attestation verdict and JSON output contract retain their existing behavior.

## Writer Liveness Finding

Source control flow in `📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts` establishes a publication gap:

- The status writer publishes from `update`. It does not schedule recurring writes on its own.
- A running stage publishes its initial state. Further stage status updates arrive through `onStage` or `onLine`. The watchdog invokes `onLine` only when the latest formatted output is nonempty and changes. The same watchdog separately samples a live stage's CPU and RSS, records real movement and updates its local custody checkpoint. A stage can make progress without writing a new line.
- The outer loop publishes `loop-failed` once before sleeping for exponential backoff. The schema allows 1,200,000 ms before jitter, beyond the 600,000 ms freshness window. The ordinary per-stage decision loop bounds its sleep, so this gap belongs to the outer exception retry path.

These source paths explain how a receipt can expire while a worker is still running. The age of an individual remote receipt does not identify the cause of its silence.

## Verification

The regression fixture failed against the earlier store reader. The operator tests passed with the fix and exercise English and German CLI output. The independent Python Draft 7 validator reproduced all nine JSON receipt cases, including the inclusive stale boundary, a future timestamp, an origin mismatch, malformed and absent receipts, and an affirmative receipt that still carries a stage.

## Ready Receipt Heartbeats

The Ready status schema now specifies a 30-second heartbeat interval alongside the five-second update throttle. The stage watch can publish a heartbeat when its watchdog detects new movement and persists a checkpoint for the same physical flight. The observer forwards no historical movement to newly attached subscribers. The readiness loop disables the callback when its stage returns, throws or is cancelled. The publisher has no heartbeat timer and accepts heartbeats only while the receipt says a stage is running or a retry is in backoff. Ordinary Ready receipts continue to depend on fresh successful evaluations by the sidecar.

On an exception retry, the loop wakes at bounded intervals and can renew the backoff receipt until its original absolute retry deadline. The status retains the refusal and planned retry time. The sleep removes its abort listener on wake. The per-stage timing, lease renewal, process custody and stage-result paths retain their existing decisions.

The clock-driven regressions reproduced all three cases against the preceding behavior: receipt expiration during silent local movement, only one publication during a 16-minute retry backoff, and an old movement notification refreshing a replacement stage after failure. The fixed tests cover the full 16-minute schedule, cancellation, expiration when no further movement is reported, and the guard against refreshing a ready receipt from a heartbeat. The Python dateutil model reproduced the heartbeat publication sequence. The real POSIX process test also confirmed that the watchdog emits the activity callback for a CPU-active silent stage and still detects a quiet idle stage as stalled.

Verification completed locally on macOS. The four operator tests, five targeted Ready publisher tests, the POSIX watchdog process test, both Python oracles and the scoped whitespace check passed. The Windows rename fallback was exercised through its fault-injection test. The deployed worker processes were outside these local tests.
