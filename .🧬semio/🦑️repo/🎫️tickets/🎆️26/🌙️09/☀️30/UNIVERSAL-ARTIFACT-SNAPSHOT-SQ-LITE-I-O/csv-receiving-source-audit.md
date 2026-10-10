# CSV Receiving Source Audit

Read-only current mounted source, 2026-10-09. No builds/tests run. Root reports source90060 new law passed with 11 tests/153 assertions but an asset URL failure; repaired replay65716 and native1238 were pending at assignment. This audit does not claim their eventual results.

## Actual Custody

Mounted CSV snapshot impl now explicitly calls receiving::project_receiving/reconstruct_receiving; it does not forward to direct backing methods or Store defaults. `🫴️receiving/🦀️.rs` declares derived RetireOwned Projection and Reconstruction. Both receive real native/body owners, narrow the same native allowance through scoped_native, and reconcile receiving wrappers/cleanup admission once with settle. Original allocation direction and recipient stay attached; the extra always-true observer is composed with the original native callback rather than replacing it.

Projection owns actual database, scratch SqliteRow and cumulative value_bytes. Table vector is paid before reserve; each empty table enters database before its name/schema/rows allocations. Scratch row values are paid before push, text cells enter scratch as empty owned String before copy, and completed rows move into paid table storage. A cancelled field copy therefore remains in scratch row or database rather than a local temporary. Only primitive or empty String SqliteValue construction enters the cell helper before its work check, so no existing heap backing is lost on that check.

Reconstruction owns CsvSnapshot and four paid usize index vectors. Records/fields identity and ordinal validation borrows original database, sorts paid index storage in place, and checkpoints work before index swaps. Snapshot schema String stays in frame during copying. Each empty logical CsvRecord/CsvField enters paid destination before vector/text allocation, so cancellation retains its genuine prefix. Late quoted/ordinal errors keep already copied fields inside the frame. Source database remains borrowed, not copied or retired by this receiving child.

Port methods inspect original SQL remaining allowance before birth, invoke first-party body destination helpers, then settle actual cumulative native allocation even when copy returns refusal. Body.work settles admitted item/copy receipts before its caller effect; successful receipt-before-effect ordering is safe because no fallible action intervenes between helper success and the inline push/write. Frame/row/index fields have real supported retirement via derives; final owned output transfer remains the existing receiving mechanism's responsibility.

## Actionable Gaps Sent to Execution Agent

1. receiving::schema removes all whitespace from both declarations and compares case-insensitively. It will accept malformed SQL token splitting such as `CREATE TA BLE csv_document` or `csv _document`, since removing spaces reproduces the authored string. This does not prove authentic SQL schema equality and differs from independent SQLite parsing. Add a neutral malformed-token law with bun:sqlite refusal; preserve token boundaries through the existing first-party schema tokenizer/borrowed validation port. The authored CSV schema has no string literals, so literal-case corruption is not currently the main case, but collapsing tokens is already a concrete hole.
2. New native laws cancel every SQL control callback checkpoint, including text prefixes, but original native callback is always true. Add genuine original native observer cancellation during body allocation and especially receiving post-body cleanup/publication, where SQL callback need not run. Verify same retained output/frame, cumulative native accounting and physical custody before retry close.
3. close_decode/close_encode assert per-turn fits and eventual absence of has_retirement_owner, but do not accumulate exact released bytes against body backing plus cleanup births/receiving wrapper release. This proves bounded terminal traversal if executed, not an exact physical byte conservation witness. Add exact release/birth accounting where that stronger claim is desired. Success outputs are returned and ordinarily dropped by fixture code; their later retirement is outside this helper's physical close proof.

The secondary-refusal test intentionally retains prior published output only as a local test value; a bare later Err drops that output normally. It proves cumulative wallet preservation, not a whole consumer session preserving the previous output on later refusal. The generic codec/hop custodian work remains necessary independently of these relational receiving methods.

No direct allocation leak was found in the inspected per-field/table/index birth code. This is source evidence only; pending Rust replay must qualify actual supported retirement and cleanup behavior.

## Concurrent Correction Readback

Execution agent reported that the schema comparison was concurrently corrected before receiving this audit message. Current source readback shows `struct Tokens` and receiving::schema using its next token method: borrowed ASCII word/punctuation boundaries are retained and compared under paid work checks. The whitespace-collapse finding describes the earlier source read, not the final current token implementation. Execution agent is adding independent malformed-token witnesses and genuine native callback cleanup cancellation.

Execution agent reports registered Source65716 and Native1238 ended130 in actual generation prerequisites, rather than reaching native assertions; the prior Source90060 new receiving oracle passed while the existing URL slash failure was repaired. No final physical byte-conservation qualification was claimed by that agent. Runtime qualification remains open.

The corrected Tokens::next still scans an arbitrarily long whitespace span or identifier through trim_start_matches/take_while before the next port.work checkpoint. Requested an interior per256-octet paid scan checkpoint for large received formatting/token spans; borrowed slices can remain allocation-free. This interaction frontier is independent of the resolved token-boundary correctness hole.
