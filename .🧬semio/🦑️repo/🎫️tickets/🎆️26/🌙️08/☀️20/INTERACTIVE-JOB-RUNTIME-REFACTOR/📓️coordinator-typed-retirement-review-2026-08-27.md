# Typed UI Ownership Review

## Verified Scope

The coordinator read the complete retained typed decoder and its four mounted language-neutral tests. The path is `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts`. It adds typed semantic/default normalization after the owned wire decoder, explicit captured payload ownership, bounded nested UiValue frames and a read-only surface byte view. It is not yet mounted in live React or wgpu patch publication.

The original UiPayloadRetirement propagated the inner surface-byte cursor's Complete while its outer root still existed. The coordinator independently identified this source hazard; the executor had already demonstrated it with three failing tests. The unchanged terminal-empty assertions now pass after keeping the inner result Pending until the outer root is empty. Actual RED and GREEN outputs were read: four targeted tests pass, 517 skipped, 521 total. The coordinator's full quick run R11 subsequently timed out without a summary; the broader suite is not currently green from that run.

## Ownership Properties Reviewed

- Payload construction/capture is private and exact-root, with checked reference growth. A borrowed JS value alone does not own its bytes.
- Surface normalization captures UiSurfaceBytes before decoder close. A private ByteView exposes only length/byteAt; final retirement invalidates the borrowed view.
- Completed container ownership is reverse postorder, releasing parent references while child roots still remain retained. Nested arbitrary UiValue traversal and cancellation use explicit linked frames; fixed-schema generators have bounded delegation depth.
- Arrays/maps have individually immutable members and fixed shape. Native fixed domains are preserved: ordinary containers 256, children 128, bindings/grid tracks 32, scalar text 512 UTF-8 bytes; the separate surface document uses 256-byte pages up to 32 KiB.
- Zero/insufficient grants leave teardown blocked. Close only succeeds with the aggregate terminal-empty witness; candidate cancellation cannot close an older published snapshot's exact byte owner.

These are source and fixture-level properties, not physical JavaScript GC, WIT lifting allocation or an eight-millisecond worst-case certificate. The generic exported Contract types still include mutable TypeScript array shapes even where runtime values are frozen; the live retained read API should expose read-only field owners when mounted.

## Next Integration Boundary

The executor's proposed node representation uses seven direct field owners plus fixed scalar data. Replacing one field captures the unchanged roots and transfers only the new owner, avoiding an ancestor/overlay chain that grows with every update. Persistent-index final-value retirement must close those direct fields, and temporary reads must capture before removing the last entry. The complete tree/hash/ordered-notification/publication/ACK chain and exact app-close aggregate remain required.

Native serde parity now passes one targeted test, 90 skipped, 0.034 seconds; the coordinator read the actual output and full native test body. It checks all eighteen sparse/normalized component pairs and style/accessibility defaults (`🧪️member-ui-component-default-parity-r2-native-2026-08-27.txt`). Independent full React R12 passes 521/521 via the canonical long tier. Full typecheck R2 still has nine actual diagnostics (seven tutorial joins, two repository-discovery errors) and no typed-wire diagnostic. Nothing is suppressed or declared end-to-end complete.
