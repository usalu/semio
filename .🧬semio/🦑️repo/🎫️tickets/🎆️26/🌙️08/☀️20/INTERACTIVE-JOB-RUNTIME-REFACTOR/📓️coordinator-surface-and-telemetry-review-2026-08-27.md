# Surface Ownership and Telemetry Review

## Surface Foundation

The coordinator read the complete initial297-line OwnedUiSurface implementation and its mounted test. Actual executor result is1PASS/537skipped/538discovered,9.51seconds, after test-first missing-module RED. Strict React still reports exactly the seven tutorial joins. These are scoped results, not live UiNodeView integration.

The owner keeps a flat node index, opaque per-consumer subscription tokens, retained initial lookup, operation/validation/hash cursors, one staged-read epoch publication, bounded notification traversal and an exact actor/instance/surface/revision/hash receipt. Active readers explicitly block surface closure until unsubscribed; detached readers retire through the owner queue. The test verifies a text-node publication, independent surface isolation, hash parity with JSON/Buffer/Immer, invalid-root rejection, no premature ACK and explicit reader closure.

Two source-review obligations were assigned before adoption. First, post-publication beginClose currently clears pending notifications and receipt ownership after the root is already visible. Late cancellation must not silently discard committed delivery obligations. Second, notification exceptions currently become a counter while the notification cursor advances; recoverable failure must have an explicit retained retry/fault-consumer path, not leave an unseen second read indefinitely blocking future publication. Tests are required for every cancellation prefix and actual Surface byte owners, not only the current text fixture.

The live synchronous whole-scene decode remains separately assigned in `📓️coordinator-owned-read-publication-r2-2026-08-27.md`.

## Telemetry and Exact Watchdog Authority

The coordinator read all four new held-mutex test bodies and actual native failed result footers. Event-ring, timer/site, watchdog/site and watchdog/violation-ring cases each actually fail0PASS/1FAIL because callback completion waits for a held real mutex to be released. The initial nextest group stops at its first failure; the remaining three were executed separately. The watchdog cases preserve their exact test operation/generation evidence before the callback-return assertion fails. These deterministic tests demonstrate blocking; the100ms test rendezvous is not an8000us production limit change or a general runtime latency measurement.

Source review also confirms `watchdog_step_overrun_us` locks and allocates a global snapshot and selects only numeric operation/generation. Global bounded-ring eviction and unrelated sessions therefore cannot be a reliable exact fault authority. The compiler executor owns the repair: callback/session-owned verdict first, optional fixed nonblocking telemetry separately, and migration of actual plugin/renderer quarantine consumers. Merely dropping a contended violation is not accepted. The transport executor retains disjoint plugin close diagnostics; the demonstrator peer owns only the extension/plugin WIT macro export region.

Native full-plugin/Wasm rebuilds remain queued behind the disk/publication coordination. Small incremental trace/job verification and non-Rust work continue; no cache or evidence deletion was performed.
