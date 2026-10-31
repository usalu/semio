# Dense Payload Prefix Source

The six defining bodies now own a fixed MaybeUninit page array with an exact dense live count. One paid scalar-header turn creates an empty live prefix; no256 unused Option rows are written. Actual commit initializes one new row; borrowed reads/capacity iteration stay inside the live prefix; close and typed last-page transfer move/pop exactly that initialized row. All page/byte/process limits, backing/ledger identity and staged/incoming reservation phases remain.

The invariant-bearing fields are crate-private. External safe code cannot fabricate a live count and read uninitialized storage; existing public page/reader/identity/typed transfer methods remain canonical. No alias preserves initialized_bindings: all migrated native observations use actual page_count and readiness.

The work schema now pins actual header1, livePages0 and maximumPages256. Original257 initialization bound is explicitly retained as maximumInitializationTurns. The legacy initialization family preserves every cut0..257 and all10,000-item/CPU source stock, denial/cancellation/zero-work/heap/pointer/paid close semantics; cached cuts now correctly cost0 after the single header. It is mounted as an actual same-package target, alongside the two new dense-page-prefix public laws and the existing original-page-transfer target. The portable TS file validates the neutral contract with AJV and independent SQLite relations; it explicitly makes no native/Host authority claim.

The new neutral matrix also retains the required original Worker256-stock success, cancellation and meaningful-axis coverage obligations. Those owning families remain mandatory, as do all existing Source error/null/partial/incoming/output/close cases. SourceReady is not claimed: the bodies and updated owning targets have not yet compiled/executed, and the real Worker/NativeIO Host suite plus native CPU ingress remain open.

Reverse research evidence is in drawing-payload-dense-prefix-census.md and its generated lexical inventories. Exact current17-file packet: drawing-payload-dense-prefix-working-files.json.
