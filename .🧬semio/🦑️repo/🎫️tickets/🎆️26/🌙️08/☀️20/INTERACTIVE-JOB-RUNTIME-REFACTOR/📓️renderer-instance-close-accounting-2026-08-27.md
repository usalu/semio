# Exact Instance Close Accounting

## Reproduced Defects

The neutral `instance-close` fixture exercises actual private children at their terminal frontier. The first canonical run failed all three tests: surface child completion reported 1152 bytes instead of its actual 4096; patch wire completion reported 128 instead of 4096; lookup `blocked` became `pending`. Result: three failed, 634 skipped, 637 discovered, 8.09 seconds. Full output is `🧪️renderer-owned-instance-close-accounting-red-r1-2026-08-27.txt`.

## Repair

Instance and patch close now forward the exact child kind/counts, translating only successful child completion into aggregate pending. Actual terminal inspection gates a later step for surface unlink (1152), input-retirement token creation (128), wire-wrapper release (128) or resident-scope release (64). Lookup refusal and error remain unchanged, and invalid/over-grant child counts remain visible as rejected rather than being replaced by wrapper costs.

Input receipt peeking is now read-only. Normal patch advance mints an available exact input-retirement token in its own 128-byte step before returning input readiness. Cancellation does the same after the wire owner becomes terminal. This avoids moving the token work into an unaccounted peek callback. The intake state machine already waits for patch readiness before releasing the token; broader regression is queued.

R2 passed surface and lookup tests but did not reach the old wire spy's `complete` interception: the corrected wrapper recognizes the actual terminal child on the preceding pending return. The fixture was narrowed to that real terminal frontier and injects a valid complete/4096 result there; it does not fabricate a nonterminal completion. R3 then passed all three tests, 634 skipped, 637 discovered, 9.27 seconds (126 ms execution), including both patch and instance wire modes. Full outputs are `🧪️renderer-owned-instance-close-accounting-r2-2026-08-27.txt` and `🧪️renderer-owned-instance-close-accounting-r3-2026-08-27.txt`.

Canonical command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedInstanceCloseAccounting'`.

The broader `OwnedInstance` cohort passed all 13 tests (624 skipped, 637 discovered, 12.43 seconds), and `OwnedIntake` passed all five (632 skipped, 637 discovered, 10.97 seconds). These include the new read-only peek and separate input-token step. Full outputs: `🧪️renderer-owned-instance-close-regression-r4-2026-08-27.txt` and `🧪️renderer-owned-intake-close-regression-r1-2026-08-27.txt`.

## Lower Surface Repair

The lower `OwnedUiSurface` layer had analogous joins: maintenance child retirement results forced to pending, reader completion plus immediate 512-byte lease creation, and close retirement results forced to pending. The neutral `surface-child` fixture exercises five exact private child boundaries with blocked/rejected cases and a terminal-reader-4096 law. Its first canonical run failed both tests: blocked became pending, and the reader's 4096 bytes became 4608. Full output is `🧪️renderer-owned-surface-child-red-r1-2026-08-27.txt` (two failed, 637 skipped, 639 discovered, 8.56 seconds).

Surface maintenance now retains its exact queue head while a child runs, forwards refusal/fault and raw over-grant counts, and separates queue rotation (64 bytes), terminal-owner release (64 bytes) and completed-reader lease creation (512 bytes). Reader output remains owned even when a returned child count is invalid. Throws leave the queue/root resumable. Unsubscribe or newly enqueued work invalidates an earlier completed-queue phase. Surface close similarly forwards children and performs field release only in a later terminal step. The generator-based patch paths have distinct semantics and were not mass-rewritten.

R2 passed both tests (637 skipped, 639 discovered, 6.74 seconds); the broader `OwnedSurface` R3 passed ten tests (629 skipped, 639 discovered, 10.07 seconds). A third test adds actual private-reader throw and over-grant retention through cancellation. R4 passed all three tests (637 skipped, 640 discovered, 21.91 seconds, 158 ms test execution). Logs are `🧪️renderer-owned-surface-child-r2-2026-08-27.txt`, `🧪️renderer-owned-surface-child-regression-r3-2026-08-27.txt` and `🧪️renderer-owned-surface-child-r4-2026-08-27.txt`.

Strict R1 reported seven existing tutorial errors plus four fixture return-type widening errors. An explicit schema-owned `RetainedUiWireStep` annotation repaired the fixture; strict R2 reports exactly the seven tutorial errors and no owned surface diagnostics. Both complete outputs are retained as `🧪️renderer-owned-surface-child-strict-r{1,2}-2026-08-27.txt`.

Canonical targeted command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedSurfaceChild'`. The source, fixtures and collected tests are held for the coordinator's combined full-renderer and strict snapshot. No live renderer or full-stack latency certificate follows from these selected tests; lower index/generator forwarding and live paged admission remain separate inspected obligations.
