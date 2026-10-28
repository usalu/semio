# Synchronous Owned Control Adapter Audit

Read-only source review of the retained read lease and StagedFormSession; no production modification or execution claim.

## Smallest authentic boundary

Keep the real OwnedUiReadSource, exact subscription, exact issued snapshot and real host surface identity in the control port owner. At event delivery, reread subscription.snapshot and require identity equality with the snapshot whose ordinary control was rendered, then borrow snapshot.record inside the same synchronous call. Require the expected node id/key, Input/Select/Button kind, current action trigger, disabled/read-only policy and resident binding. Supply surface/revision from the actual host publication, never from event input. Consume the reserved binding immediately through StagedFormSession.handle; only its staged result and host-owned request may outlive this call. Do not forward resident binding/action args to a Promise sink.

The ordinary record is already RetainedUiNodeRecord and its ordinary component already has the native Component shape. No object spread, full UiDocumentStore projection, copied component graph or alternate renderer is necessary. Borrowed presentation readers may expose those existing scalar fields only for the active render read; callbacks retain identities, not the record as their authority.

## Important integration hazards

* An active snapshot is not proof of latest publication. OwnedUiNodeReadLease.canReadSnapshot accepts either still-active issued root; the old root deliberately remains active until commit acknowledgement/retirement. Likewise snapshot.record checks active lifetime, not currentness. Using either alone to authorize a callback would accept an obsolete enabled binding after replacement. Compare the exact current subscription snapshot, plus publication identity, immediately before consumption.
* The owned surface facade does not mint action sequence numbers. Keep a checked nonwrapping u64 sequence in the actual surface/session lifetime; changing per-control React mounts must not reset it. StagedFormSession stores last accepted sequence per source object, so creating a new source adapter per event would discard replay continuity. Keep one stable source adapter for that real owner/publication stream and invalidate it on close.
* StagedFormSession assumes its source methods are trusted synchronous resident readers. Its read callback must derive action/path/fixed choices from the admitted record, not intent.args. For reserved handling, construct the ephemeral intent with no borrowed args payload; handle ignores args. Text/choice raw input is separately admitted as a string; page/buttons require absent input.
* Retaining the JavaScript reference after typed retirement may physically preserve fields, but that does not extend accounted owned custody. A generic asynchronous fallback requires its own authentic owned input admission/ack before the read lease can retire. This narrow staged adapter should refuse unsupported nonreserved dispatch rather than treat a surviving object alias as authority.

These are concrete integration requirements, not findings that the currently unmounted port already violates them. No new confirmed P1/P2 in StagedFormSession itself was established by this focused source review.

## Existing fixture and runner

Use UiDocumentStore/tests/typedwire-helpers nativeInstanceFixture and the existing owned-intake-and-surface OwnedRootSource test route: actual typed wire offer/finish/publication acknowledgement, OwnedUiInstance and source subscription, React render/commit acknowledgement, replacement, retirement. Do not use builtNodeToSnapshot/loadSnapshot. Add old enabled to new disabled replacement before old callback, key reuse, closed lease, duplicate sequence, structural page replacement and text-preserving pending refresh scenarios. Assert reserved consumption is synchronous and no borrowed args reach an async sink.

Renderer React target tests/config uses jsdom; SEMIO_TEST_LEVEL=exhaustive includes UiDocumentStore, long/exhaustive includes Interpreter. OSroot config does not discover these in-source suites. Exact runner map is in renderer-test-discovery-audit.md. repo is absent, so no new list/test command was represented as executed.

## Source anchors

- framework/ UI contract retained/read-lease/TS: OwnedUiNodeReadSnapshot record getter; OwnedUiNodeReadLease snapshot/canReadSnapshot/acknowledge and two-root retirement.
- Renderer ShellHelpers/window-panes/staged-args/intents/TS: StagedControlSource, control, handle, sequences and canExecute.
- Existing detailed source paths and fixture imports: owned-intent-custody-api-map.md and owned-control-fixture-map.md.
