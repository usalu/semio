# Owned Flat Surface Publication

The new `ui/contract/retained/🖼️surface` owner joins real typed operations, persistent node edits, retained graph validation, canonical JSON/FNV hash, exact read-epoch staging, atomic root publication, bounded listener notification, and a one-shot publication acknowledgement. The owner-created private patch cursor binds these results to the exact captured source object. Callers cannot supply a candidate root, validation token or hash to publication.

## Ownership And Scheduling

Subscriptions expose only a frozen snapshot token, not lease close authority. Their initial node lookup advances through the retained numeric index under one item/4096 bytes; the candidate node survives reader retirement until the new lease captures it. A changed source revision retries lookup. Registration, unlinking, and queue manipulation are fixed-size linked-owner operations. No full Map copy or subtree rebuild is introduced.

Preparation stages changed consumer reads invisibly. One private epoch and root/view swap exposes all prepared reads together. Subscription additions during staging extend its retained cursor; additions after publication initialize from the new root and do not enter the older notification cutoff. Each advance invokes at most one consumer callback. Callback execution itself remains a host callback timing boundary, not an empirical eight-millisecond certification.

Unsubscription transfers the exact reader/node/lease state into retained maintenance. Surface close blocks on still-active subscriptions, preserving React-held reads until actual unmount/unsubscription. It then drains only its owned detached roots. A same-identity independent surface is unaffected.

Late cancellation after publication continues committed notifications. Close then blocks on the pending one-shot acknowledgement, available through the patch or `takePendingAcknowledgement`; it does not silently discard the receipt. The eventual transport must retain and retry that acknowledgement until sending it succeeds. This is not yet an installed actor transport join.

Thrown callbacks produce a retained failed-consumer entry. `takeNotificationFailure` identifies the exact opaque subscription; `retryNotification` schedules one explicit retry. Its issued snapshot remains owned throughout. An unresolved failed consumer can still backpressure its next update, but it is neither silently dropped nor irrevocably stuck without retry/unsubscription.

## Executed Tests

- R1 intentionally failed collection at the missing surface module, not a behavioral implementation failure.
- R2 passed 1 test/537 skipped/538 total in 9.51 seconds; strict R1 had only the existing seven tutorial producer joins.
- R3 passed three and failed one: the test incorrectly expected a post-publication reentrant subscription's retained initial lookup to finish before patch readiness. The assertion now separately drives that owned initialization queue; no synchronous lookup was added.
- R4 passed four and failed the every-prefix byte-retirement law. The cancelled patch had transferred staged consumer cleanup to surface maintenance and reported its own terminal state too early for that law. It now retains the exact staged-cell frontier, drains each cancelled captured read, and only then releases the frontier. It does not wait for a global registry to become empty.
- R5 passes all 5 tests, 537 skipped, 542 total, 7.95 seconds total/1.91 seconds tests. The actual React hook renders an incrementally initialized byte owner, replaces it, acknowledges the exact new snapshot, and releases old bytes only after retained maintenance. Every cancellation prefix spans operation, validation, hashing, staging, publication/notifications and receipt completion with real SurfaceDoc byte owners. Full output: `🧪️renderer-owned-surface-r5-2026-08-27.txt`.
- Strict R2 found seven tutorial joins plus one new test-only self-referential parameter type. The parameter was renamed; the next strict run is pending.

The strict language-neutral schema/fixture, existing Immer reference state, Node JSON/Buffer byte oracle, and real React/testing-library are used. Additional source snapshots are recorded separately rather than retroactively changing these counts.

## Remaining Live Boundaries

This concrete owner is test-mounted, not yet adopted by PluginRuntime, UiNodeView, Shell refresh caches, or wgpu. The per-instance aggregate and exact actor/generation routing still need to own it. Live WIT field ingress and actor ACK delivery still need retained ownership. Interpreter's current SurfaceDoc conversion performs whole-array scene decoding and cannot consume the byteAt-only read view; it needs an owned incremental scene projection before the final live cutover. No compatibility byte-array adapter will be added. The seven tutorial joins remain Dag-owned.
