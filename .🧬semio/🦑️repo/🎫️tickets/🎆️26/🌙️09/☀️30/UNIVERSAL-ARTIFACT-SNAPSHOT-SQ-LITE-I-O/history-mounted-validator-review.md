# Mounted History Validator Review

Read-only audit of current source, 2026-10-13. No builds or test execution. Current producer: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🦀️.rs`.

## Production Coverage Reviewed

Both actual directional trait overrides mount this module. Its retained Frame owns seven id vectors, five ordered vectors and the actual IoResult slot. Port prices vector backing and index moves/sorting; allocation receipts reach SQL through Port.receipt. No candidate P/text clone is constructed. Receiving owner owns scratch through result publication and close; binder drops Port and RefCell borrow before finish_body enables close-time settlement. No concrete RefCell lifetime/borrow conflict found in that sequence.

All seven authored SQL declarations are validated with the shared borrowed schema parser, including exact table census and duplicate table rejection. Exact row arity, SQL cell types and id-cell=rowid are checked. Surrogate ids remain signed; no positivity restriction is introduced. Duplicate surrogate ids and duplicate parent/ordinal pairs are rejected. Parent existence checks use sorted signed indexes; per-source-parent ranges require exact child counts and ordinals 0..n-1. Timestamp rows require unique one-to-one checkpoint ownership plus all six u32 words.

Full source fields compared: activeAlternativeId presence/text; checkpoint id/parentId/message/order; actor/physical_ms/logical exact words; author id/name/avatar presence/order; member documentId/checkpointId/alternativeId/order; alternative id/name/order; every ordered repeated checkpointId pin. Duplicate semantic IDs and unresolved semantic references are preserved. Empty children and optional Null versus empty string remain distinct. Text comparison uses borrowed 256-byte chunks with original native work and SQL checkpoints; no unbounded candidate-copy comparison remains.

Shared `TypedFileSettlement` captures original before/remaining once, reconciles cumulative delta idempotently, and checkpoints after binder borrows are dropped. Its inner validation observer settles actual index/frame/close admissions; the outer typed file observer is still in body mode during nested validation and does not borrow its SQL RefCell. SQL scalar census is max-based (`admit_reconstruction_scalar_bytes`), so repeat borrowed validation does not add phantom scalar bytes. No production field omission or allocation double charge was proven by this source review.

## Concrete Authored Test Gaps

1. `🛂️validation/🧪️tests/🟦️.ts` independently inserts all seven baseline tables and signed renumberings into SQLite, but its malformed edit loop only checks `accepted === false` and table-name membership. It never applies the edit or verifies the independent SQLite/query result. Thus malformed rejection has Native implementation coverage authored, not independent Source behavior coverage. Apply each fixture edit to independently inspected rows; structural SQL-invalid edits may be checked with constraints, semantic unequal edits with exact expected query rows.

2. Native `run` rearms `live.store(true)` before zero/funded cleanup. It does not attempt a funded close while the same original native observer is still canceled. Add that refusal and unchanged original destination/index receipts before rearming the same AtomicBool. Current zero-close test checks zero receipt/birth/release, but after rearm.

3. Cancellation laws compare `ValueRefusalKind::Canceled` only. They do not distinguish exact SQL observer cause from original native cause, nor assert original borrowed message identity through the owner wrapper. Add distinct expected literal causes and pointer custody where the genuine binder supplies a borrowed cause; preserve promoted full owner receipts.

4. Prior progress fixture covers items/copy only; retainedCapacityBytes and releasedBytes are zero. Current test asserts capacity receipt equals new observed births, so it cannot prove cumulative prior capacity preservation. Add a separate actual original prior allocation/receipt law and compare new delta separately from total progress.

5. Root fixture's malformed text values use literal `\\x00`; genuine NUL appears in Native long-message cancellation input, but independent Source baseline does not establish NUL semantic equality. Add an existing-corpus true `\u0000` field case checked through SQLite and both actual validators. No copied whole corpus is needed.

Tests are mounted at parent snapshot tests lines 701–702. Source declarations/independent baseline checks and authored Native laws remain distinct from executed qualification. Runtime outcome is unknown to this audit.
