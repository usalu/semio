# WAL Next Owner and Capacity Work

## Active Assignments

Root owns the replacement of one-shot `open`/`open_with_control` with a caller-retained `ArtifactWalOpenCursor` and the pre-submit capacity fence. Fermat owns `SegmentWriter.writer: Option`, deterministic successful `ArtifactWal` terminal close, and necessary adaptations to current helpers; root must preserve those overlapping edits. Pascal is independently auditing current recovery branches and capacity limits. Arendt owns Home process and the exact Space guest build.

The already-run native receipt `wal-recovery-exact/exact-cargo-laws-iNHATx/00` passed both root recovery laws and failed the third replay-close law during its larger seed flush. Root fixed `copy_range` and `copy_verified_prefix` partial writes, added four language-neutral CRC alignment cases, and reran source GREEN39. Fermat owns the next warmed native recovery rerun after terminal-close edits, then the separate all-features storage-state gate. Do not queue duplicate native builds against that target.

## Capacity Fence: Exact Next Test

Storage's one-read/append bound is 496 KiB = 507904, below the current WAL default 512 KiB. Memory and Neo4j also reject oversized resulting segments; FS can append a too-large total that then cannot be read back. The fix is a shared storage-owned public byte ceiling and a pre-write transaction frame-size calculation. Merely lowering the soft rotate threshold is insufficient: `submit` currently writes and flushes before testing it.

For document `d`, the committed genesis header is 129 bytes and a chained successor header is 161 bytes. A transaction holding one 250000-byte command has 250055 record-frame bytes plus a 75-byte physical commit. Three Fsync submissions must use segments [0,0,1], with final lengths [500389,250291]. The same three submissions under a fully deferred group policy must rotate before submission three and finally have lengths [500314,250291] after explicit force-flush. Every segment must reopen and replay under the real FS bound as well as Memory.

A single 507646-byte command is one byte too large even for empty genesis: 129 + (payload + 13) + 19 + 23 + 75 = 507905. Reject it before transaction-id advancement, record writes, flush, seal, or creation. Conversely 507645 bytes fills genesis exactly to 507904; it may be accepted and then rotated without making the sealed segment unreadable. Preflight must reserve the eventual physical commit even while group-committing in memory. Calculate exact chained-header capacity before rotating; if a transaction cannot fit a fresh segment, return without changing the old one. Keep the test-only soft `max_segment_bytes` behavior distinct from the hard readable bound.

## Open Cursor Direction

The required owner state transitions are list → scalar segment metadata → retained read → generic SPR page scan → WAL semantic page scan → transaction/frame observation → verified prefix copy → bounded repair epoch → source close → advance → list close → ready. Each opportunity gates useful progress with caller control, while terminal cleanup remains unconditional. Store partial scanner, source pages, buffer and list in the cursor across yields or control exhaustion; never rely on emergency Drop retirement for normal cancellation. `take_result` must be one-shot only after all source/list owners are empty. Backend I/O may only be canceled before starting, not during an in-flight operation with the existing storage trait.

Use a fixed non-owning transaction frame-span gate, not a decoded `WalRecordBatch`, for semantic validation and later artifact/sync replay. A decoded 64-record batch can exhaust the operation page budget before its valid transaction is emitted. The detailed independent contract and consumer map are in `📓️terra-wal-committed-transaction-replay-frontier.md` (fully read by root).

The present one-shot recovery and explicit-control variant are an intermediate implementation, not a compatibility API to retain. Migrate the small set of production open callsites in artifact, cluster and CLI and the test helpers when the retained cursor exists. Fix CLI's now-stale “rewritten” recovery text in that same lifecycle change.
