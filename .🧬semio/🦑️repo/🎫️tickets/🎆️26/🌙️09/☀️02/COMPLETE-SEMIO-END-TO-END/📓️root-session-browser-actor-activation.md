# Session-Bound Closed Browser Actor Activation

## Outcome

The production document socket path now activates one private closed actor only after the exact current WebSocket accepts its matching Hub Session. The original document-open owner remains authoritative through the final activation continuation. This is a qualified mocked-child production-path slice, not a real GIS render claim.

## Ownership and Protocol

The exchanged lease privately retains a cloned binding and original open intent plus the original current-owner assertion. Neither descriptor, grant receipt, intent nor child is exported to the UI. Activation preflights descriptor capacity before child/body work, reserves the fixed dedicated child, POSTs the unchanged intent to the fixed scoped browser-actor route, reads bounded bytes under cancellation and deadline, verifies SHA-256, consumes/transfers the actor body, invokes describe, and compares canonical guest/staged descriptors using the existing normalizer.

Every asynchronous continuation rechecks current socket, Session actor, pending actor, private reservation, lease, grant, expiry, original open attempt/client, exact scope and installed selection. Status callbacks are fenced on both sides. A failed owner closes its child/socket and drops only its own lease; stale socket frames and close callbacks cannot retire a replacement owner. Duplicate Session is a protocol failure, not a second actor admission. The final status remains renderer-unavailable.

The bounded body reader acquires the Response reader before checking stale authority or HTTP failure, ensuring even an unread stale header response is cancelled and unlocked. Received body and descriptor buffers are wiped after use, including nested invalid result shapes.

## Executed Evidence

| Receipt | Result |
| --- | --- |
| 1124 | TDD RED: valid production Session never activated before implementation |
| 18217 | TDD RED: scope replacement at actor headers left the unread response uncancelled |
| 80018 | GREEN: twelve neutral Session cases after reader ownership repair |
| 67983 | TDD RED: client replaced in the post-describe microtask survived final publication |
| 75405 | GREEN: thirteen cases after restoring the captured open-owner check in every reservation guard |
| 22163 | GREEN: sixteen Session cases, seven selected Vitest tests, 273 skipped; AJV3 |
| 4090 | GREEN: six broader browser document-open regression tests, 274 skipped |
| 76305 | GREEN: execution-target lease source/oracle; 48 fields, twelve byte vectors, eleven lifecycle cases, 71 hostile mutations; independent Node/WebCrypto hash agreement |
| 56123 / 11776 | GREEN: scoped Nx format write/check |

The sixteen language-neutral rows cover pre-Session zero work, success, close during body, replacement during load, descriptor mismatch, same document in two spaces, wrong Session, wrong body hash, stale headers, client replacement during load, noncanonical descriptor, invalid result shape, post-describe client replacement, duplicate Session during load, post-describe socket close, and progress-observer client replacement.

Each row enters real connectHubOnce and the actual socket onmessage handler, not a direct synthetic activation call. A mocked Worker uses real MessagePorts and transferred buffers for load/describe. Cleanup requires one worker termination and child capacity zero. No real component or real Hub was substituted into this claim.

## Files

- OS backbone worker: private lease and reservation activation, source-socket frame/close ownership, bounded response cleanup, and production-path tests.
- Directory schema: browser-actor progress stage.
- Directory fixtures: Session corpus and JSON schema.
- Hub package script: existing reservation check validates the new corpus and reports explicit mocked-child scope. The existing registered Nx/launch command covers the tests.

## Next Required Work

A genuine cold GIS render still requires a private authenticated checkpoint-pair hydrate protocol, lifetime-bound actor turns and revisioned UI patches through UiDocumentStore. The fixed 256KiB child message cap requires bounded chunk ingress for larger pairs; it must not be bypassed by raising the global cap or granting general document-read authority. Actual GIS child and native Hub candidate runs remain nonterminal build checks. Durable GIS approval/undo, peer observation and whole-product acceptance remain mandatory.

