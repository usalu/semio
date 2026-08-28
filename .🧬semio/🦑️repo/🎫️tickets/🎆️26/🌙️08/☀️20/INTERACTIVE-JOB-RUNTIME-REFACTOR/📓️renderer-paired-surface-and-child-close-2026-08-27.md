# Paired Surface Publication and Child Close

## Executed Checkpoint

Canonical `@semio-tech/framework-renderer-react:test-long --args='--run -t OwnedSurface'` R2 executed five passing tests and one failing phase-census assertion. The new retained `scenes` phase was absent from the expected set, so that run did not execute the every-prefix cancellation loop. R3 includes that phase and actually passed all six tests, 582 skipped / 588 discovered, 23.23 seconds total, 6.64 seconds aggregate tests. Full outputs are `🧪️renderer-surface-scene-r2-2026-08-27.txt` and `🧪️renderer-surface-scene-r3-2026-08-27.txt`. The runner did not print the debug prefix census; only the executed `steps > 100` assertion is claimed.

The private Surface pipeline now retains a persistent prepared-binding index alongside its wire-node index. It visits touched IDs, prepares changed scene roots, shares exact unchanged component identities with the same owning-host width, stages paired read roots, and atomically publishes both indexes with one epoch. Wire-node hash authority is unchanged. Both old indexes retire before the publication receipt becomes available. Cancellation owns preparation, lookup, edit, both candidate indexes, staged reads, and committed notification/receipt obligations.

The new valid TextEditor scene test checks invisibility before publication, paired visibility in notification, and final Surface close waiting for an independently issued scene reader. Existing every-prefix byte test still uses a three-byte malformed scene packet; it does not establish maximum valid-scene cancellation coverage.

## Read Authority Repair

Canonical ReadPublication R1 actually failed: a reflected TypeScript-private constructor minted a token whose owner/status was accepted by staging and consumed the spare issued-read slot. This was not forged publication: the publication owner's pending-token identity check already rejected it. Fixture `forgedVersions` now exercises both pending and unrelated versions. The repair adds private runtime mint authority and an exact pending-owner check before staging captures or mutates the lease. Green R2 actually passed one test, 594 skipped / 595 discovered, 12.73 seconds total. The reflected test type is `typeof first`, preserving constructor privacy without constructor-utility type errors.

## Mounted Child Close Design

An issued snapshot allows at most two simultaneous child readers. This is concurrent-reader admission, not a scene/field limit. Refused admission retains the snapshot and all existing readers. Beginning child close does not release its slot; only terminal retirement does. Surface and read-lease close consequently wait for exact children.

Implemented integration retains child retirement in its issuing subscription's fixed four-slot queue (two snapshots times two readers). Admission checks the private issuing-lease identity before moving the child. Maintenance services this queue before waiting on issued-snapshot retirement. A queue slot clears in the same step that the child becomes terminal, preventing a released reader count from admitting a replacement while its queue slot remains occupied. Managed readers are retained in four fixed active slots; active plus queued ownership cannot exceed the four admitted child roots. Unsubscribe preflights the complete fixed transfer before moving managed children to the queue, so a consumer cannot deadlock on its own unserviced child. Manual independent readers continue to hold close until their exact owner transfers or retires them. No cross-instance/global cleanup authority is introduced.

`openSceneRecord`/`openSceneText` return frozen bounded-read facades, not destructively owned prepared documents. `useOwnedUiScene` acquires these only from a committed layout effect. Its bounded two-reader effect scope rejects late calls after cleanup, queues readers in `finally`, and uses the exact subscription captured by that effect. The source also owns every managed reader, so unsubscribe queues them even if consumer cleanup fails. The source-replacement DOM law demonstrates the old source reaches terminal while the replacement remains independently readable, stale close cannot close the successor, and unmount services the final children through Surface close.

Actual tests: child queue R1 missing-method RED then R2 1/1 GREEN; managed-child R1 missing-method RED then R2 1/1 GREEN; effect R1 1 pass / 1 fail (missing hook), then R2 2/2 GREEN, 594 skipped / 596 discovered, 4.61 seconds total. Full outputs use the `renderer-scene-{child-queue,managed-child,effect}` prefixes.

## Surface Runtime Mint

The reflected module-local Subscription constructor accepted a crafted cell. The adversarial R1 executed unsubscribe and demonstrated real list corruption: close advanced `subscription-close` instead of blocking on the still-live real reader. A private runtime mint now rejects before any cell transfer. The exported SurfacePatch constructor likewise rejects before accessing an attacker-supplied source getter; exact live patch/source publication checks remain intact. The patch getter law was not reached in the original failing run and is only credited in GREEN.

Canonical OwnedSurface combined GREEN R2 actually passed 8 tests, 589 skipped / 597 discovered, 10.44 seconds total and 4.90 seconds aggregate tests. This includes all existing cancellation/notification/byte laws, paired scene publication, managed/unmanaged child close, real React replacement/unmount, and the two constructor checks. Log: `🧪️renderer-surface-mint-green-r2-2026-08-27.txt`. Targeted `git diff --check` passed for the Surface, read lease, React hook, and UiDocumentStore test files. Coordinator's independent full 595-pass checkpoint predates the final two tests; no full 597-pass claim is made here.

## Remaining Live Boundaries

Actual UiNodeView/Interpreter and wgpu entry points remain uncutover. The old Interpreter whole ScenePack conversion is still blocked; all fifteen supported scene hosts, including nested JSON/pack consumers, require real prepared views. The per-instance aggregate and transport ACK join are not yet mounted. No full-renderer or runtime latency claim follows from these targeted tests. Native numeric ScenePack regression is separately reported by the compiler owner as 96/96 passing; finite scene geometry/default/unknown-field parity remains a queued owned follow-on.
