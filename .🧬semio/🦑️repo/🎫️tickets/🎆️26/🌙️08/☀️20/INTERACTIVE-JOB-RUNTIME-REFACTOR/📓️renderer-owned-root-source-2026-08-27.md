# Owned Root Source

`OwnedUiInstanceSurface.subscribeView` now forwards the exact underlying Surface subscription and queues its one-cell maintenance in the same instance. `useOwnedUiView` observes that facade's stable immutable root/revision/hash object through React's external-store API. It captures no node tree, reconstructs no snapshot, and releases its exact subscription on source replacement or unmount.

The schema-first `root-source` fixture specifies two safe-53-bit IDs, English/German labels, Unicode text, cancellation, independent consumers, root replacement and close waiting for unmount. The actual DOM law creates real `ShardClient` lifecycle/native patch authorities, decodes upserts, publishes through the aggregate, submits private publication tokens and checks actual rendered text against the fixture's Immer/JSON oracle. One consumer is mounted under StrictMode. Two consumers independently follow the same facade; unmounting one preserves the other. An unpublished cancelled replacement keeps the previous view identity and visible text. The next valid root replacement uses remove/upsert/set-root, so no unreachable old root remains. Revoked-instance close cannot yield its final witness until the last mounted consumer unmounts.

## Canonical Evidence

Command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedRootSource'`.

| Run | Actual result |
| --- | --- |
| R1 | One failure, 616 skipped, 617 discovered, 10.85s. Missing `useOwnedUiView` reached in actual React render. |
| R2 | One failure, 616 skipped, 5.19s. Hook implemented; second replacement fixture left an unreachable old root and authoritative validation rejected it. Production validation was preserved. |
| R3 | One failure, 616 skipped, 4.88s. Correct replacement reached all DOM assertions; test incorrectly required the first close step to block, although it legitimately retires the completed wire cell first. |
| R4 | One passing, 616 skipped, 617 discovered, 5.12s. Close test advances bounded cleanup to exact `surface-readers` blockage and verifies no early witness. |

Full stdout/stderr remains in the four corresponding `🧪️renderer-owned-root-source-rN-2026-08-27.txt` files. This is actual native-authority-to-DOM test behavior, not a mounted production window cutover. The older coordinator full616/strict7 checkpoint predates this added law; no newer full-suite or strict result is claimed here.

## Next Boundary

The retained intake API is specified in `📓️renderer-live-owned-source-contract-2026-08-27.md`. It will perform the test's currently explicit per-patch orchestration as retained one-step work, without editing Demonstrator's scheduler/create/destroy regions. Prepared nested JSON and all fifteen host consumers remain required before switching live bodies. No copied content tree or empty host retirement participant was added. No cleanup, Rust execution, modifying git command or ticket closure occurred.
