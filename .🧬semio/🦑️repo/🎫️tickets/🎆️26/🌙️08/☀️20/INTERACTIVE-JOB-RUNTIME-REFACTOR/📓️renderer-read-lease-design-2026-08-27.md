# Renderer Read Lease Boundary

The existing interpreter already renders flat node IDs. Its current `useSyncExternalStore` returns borrowed records, which is insufficient once old SurfaceDoc byte owners retire explicitly. A speculative or concurrent React render may still hold an old record when a newer index is accepted.

The proposed per-consumer lease has two issued snapshot slots: one current/committed snapshot and one newer issued snapshot. This bounds retained roots without shrinking the valid document or node domain. Further publication/notification admission waits until an exact commit acknowledgement and bounded retirement release a slot. Repeated snapshot reads return the same immutable object without incrementing any owner count.

Acknowledgements use the exact issued snapshot object, not a supplied generation number. A foreign consumer's token is rejected before mutation. A stale token cannot retire a newer snapshot. The acknowledged snapshot stays owned; only earlier issued snapshots enter retirement. Capacity is reusable only after their retirement cursor is terminal. Closing the consumer drains current, pending/speculative and in-progress retirement owners under the ordinary one-item/4096-byte grant.

The React adapter must create this lease in the subscription/effect lifetime, not in a speculative render initializer. Before subscription it can return an empty read/version, then request a render after the exact lease is installed. Layout-commit acknowledges the exact token read by that render. Subscription teardown must retain the close job in the host owner, rather than merely dropping the lease. StrictMode remount, aborted renders, stale layout effects, independent consumers and multiple surfaces require actual DOM tests before live adoption.

The future per-instance aggregate owns all leases, pending publication/notification frontiers and surface roots. It cannot finish because a global registry happens to be empty, and must not confuse a per-index retirement result with the instance close witness. This design is staged, not yet a working React lifecycle or aggregate API.

## Executed Foundation Checkpoint

The exact two-slot owner is implemented in `retained/📖️read-lease/🟦️component.ts`. Canonical `@semio-tech/framework-renderer-react:test-long --args='--run -t ReadLease'` R1 failed collection at the intentionally missing module. R2 executes and passes 1 test, 532 skipped, 533 total across five files, 19.78 seconds total/398 ms test time. It covers stable repeated reads, foreign and stale exact-token ACKs, deferred slot reuse, two independent consumers, final surface-byte lifetime and zero-grant behavior. Complete output is `🧪️renderer-read-lease-r2-2026-08-27.txt`. The React subscription adapter and per-instance aggregate remain unmounted; this is not concurrent DOM proof yet.

## DOM And Hostile Token Checkpoint

The real `useOwnedUiNode` hook is now test-mounted with testing-library/React, including StrictMode replacement, two independent consumers, exact layout-commit tokens and an aborted Suspense render that acquires no subscription. DOM R3 passed 3 tests, 532 skipped, 535 total. Strict typecheck R1 reports exactly the seven outstanding tutorial producer joins and no owned-read errors.

The added JavaScript hostile `acknowledge(null)` case exposed an actual bug: null compared equal to the empty second slot and authorized retirement. `🧪️renderer-read-lease-null-ack-red-r1-2026-08-27.txt` records 1 failed/2 passed. Both equality checks now require non-null issued slots. The unchanged case passes in `🧪️renderer-read-lease-null-ack-green-r1-2026-08-27.txt`: 3 passed/532 skipped/535 total, 17.46 seconds total and 824 ms tests. A separate source/id-replacement DOM law is being added next.

The live UiNodeView still uses its old store hook. This adapter does not yet provide a mounted publication, bounded notification, or per-instance close witness. The next surface-owner phase must stage captures invisibly and expose them with one exact publication epoch, not make per-consumer `offer` calls visible before atomic root publication.
