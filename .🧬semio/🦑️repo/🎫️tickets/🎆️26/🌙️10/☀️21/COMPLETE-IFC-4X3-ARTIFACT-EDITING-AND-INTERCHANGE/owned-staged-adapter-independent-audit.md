# Owned Staged Controls Independent Audit

Read-only source review of intents/owned.ts, session.canInteract and actual retained surface staging/read-lease behavior. Root reports structural law probes70/293 plus narrow strict; these are not genuine mint/DOM/native proof.

## P2: notification can retire authority before staged callback

Current dispatch consumes the resident binding synchronously, clears #borrow in finally, then calls #notify followed unconditionally by this.staged(result). A presentation subscriber can synchronously close the adapter/session, or reset/revise it, during notification. The old result is nevertheless delivered to the host callback after that retirement/new generation. Reproduce with a subscribed callback that calls controls.close(), dispatch a valid text event, and assert staged callback is not invoked after close. Also exercise subscriber causing a newer edit/reset. Recheck exact current session token and closed state after notification, or define callback order with equivalent current-owner protection. Sent to root promptly; correction pending at report creation.

## Positive source boundaries

#record requires exact live subscription, exact current source.view object, matching view revision and snapshot version, and exact subscription.snapshot identity before reading issued record. This is stronger than active lease membership and rejects retained old roots. Actual retained surface publication stages subscribed node roots under the publication commit; no inference from unchanged JavaScript node identity is needed.

The adapter/source identity and checked u64 sequence live with the whole owner, not individual control objects. #borrow is single-entry and cleared in finally, and reentrant resident reads are refused. Reserved dispatch passes args:null and consumes source binding immediately; no resident args are forwarded to an async sink. Text/longText only accepts change semantics (blur refused); Select choices and Buttons remain resident-bound. Session.canInteract uses existing control admission including owner/action/token/readiness generation rules; preserved text may interact under old proof while page requires latest token. Structural operations clear proof.

Status is checked as closed own-data small Extension shape, bounded16 diagnostics/path64 and canonicalu64 offset/total, with pagination consistency. Full binding/domain checks still occur in shared reducer; the adapter is not a second domain validator. Production must supply the actual admitted owned surface, not arbitrary objects matching these structural interfaces. Unit structural probes intentionally do not authenticate mint/transport.

Read close unsubscribes owned reads, clears status/borrow and closes session. As with other callback owners, subscriber exception/reentrancy and synchronous notification during subscribe require explicit production semantics; no actual mounted runtime claim was made. Genuine OwnedRootSource fixture and replacement/ack/retire DOM coverage remain the next gate.

## Source closure after root correction

Reread current owned.ts: after notification dispatch now requires adapter not closed plus exact current request.token and request.args identities matching the produced result. Listener-triggered close and newer edit/reset therefore suppress the stale callback. Notification listener exceptions are isolated. unsubscribe deletes its retained read only after successful source unsubscribe; close retains failed reads for retry even after the closed flag. The previously reported P2 is closed at source level. Root reports74tests/310assertions and narrow strict pass; this reviewer did not rerun and genuine owned DOM/native gates remain unchanged. Root plans an additional current-view/read recheck after notification; that is not claimed reviewed here.

## Owned hook and authentic fixture source checkpoint

Reviewed control-port/owned.tsx, genericized UiDocumentStore/owned useOwnedRead and new owned-staged-controls fixture. No new confirmed P1/P2. The hook subscribes outside speculative render, keeps exact source/id/subscription, reads exact issued snapshot with useSyncExternalStore and acknowledges that snapshot only in layout commit. Status proof is installed by a later layout effect using the same subscription/snapshot/view. Control rendering borrows the actual typed ordinary component through the port; event authority remains callback-time reread rather than rendered object lifetime. Stable owner presentation subscription updates text drafts without copying the resident record.

Fixture is now an actual first-party owned wire/ACK source route, not loadSnapshot: nativeInstanceFixture activation/lifetime, real OwnedUiInstance, surface lookup, encodePackValue upserts, retained patch offer/input receipts, actual publication acknowledgement, bounded maintenance, StrictMode and shared ordinary views. It deliberately keeps an old snapshot active across replacement and verifies obsolete port dispatch cannot change args; it also asserts text preservation, structural disable and page transition without args edits. Cleanup unmounts subscriptions before controls/owner retirement and checks retirement witness. The worker-backed fixture is a test authority, not execution of native code.

Execution remains gated by renderer Vitest policy. The authored fixture therefore supplies an executable target, not a passing DOM result. Real production form mount/source owner and host supplied surface identity remain separate integration gates. Generic hook uses the source unsubscribe contract; failed cleanup retry is owned by OwnedStagedControls only if its actual lifetime cleanup invokes close.
