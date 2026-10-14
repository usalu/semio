# TXT Whole Snapshot File IO Scope Audit

Read-only current source proof; no builds/runtime assertions. Parent recorded original TXT Native96029 terminal1, actual Plugin177compilererrors/168warnings and zero TXT laws. That outcome does not establish any raw/semantic roundtrip.

## Actual distinct public routes

`store::os_store::serialization::ArtifactSqliteSnapshotCodec` is explicitly an erased native-payload bridge: export accepts IoPayload and decode owner and returns SqliteDatabase; import accepts original Option<SqliteDatabase>, encoding and encode owner and returns IoPayload. Generic native-retirement export decodes P from payload before P::to_sqlite_database_receiving; import reconstructs P then native encodes it. TXT ArtifactPack encodes to_body raw text and decodes from_body; its native receiving decode parses the same raw body, and new original encoding emits the same raw body. Thus that payload bridge cannot represent every legal TxtSnapshot.

However there already is a separate direct typed file API: `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs:2611::io_export_sqlite_snapshot<P>` directly calls snapshot.to_sqlite_database, attaches semantic file metadata and export_sqlite_database_controlled; :2624::io_import_sqlite_snapshot<P> imports SQLite bytes, removes metadata, verifies exact dialect then directly calls P::from_sqlite_database and returns P. It does not force a typed snapshot through raw native payload. It uses SqliteSnapshotControl::run_as_original_caller and cold P/R plus typed cold validation, and takes no supplied NativeSnapshotDecode/EncodeOwner. Therefore its semantic reachability differs from genuine supplied-owner custody qualification. Payload graph SnapshotHopFrame paths instead invoke erased codec and remain raw-limited.

## Exact legal counterexamples

TXT schema struct persists schema:String,lines:Vec<String>,trailing_newline:bool,line_ending:LineEnding; no String restriction bans separators or custom schema.

- `{schema:"literal custom",lines:["a"],trailing:false,ending:Lf}` and the same object with a different schema both encode raw `a`; raw decode installs STDIO_TXT_DOCUMENT_SCHEMA. Schema authority is erased.
- lines[] and lines[""] with trailing=false both encode empty raw bytes; from_body empty always gives lines[] and Lf. Original one-empty-line identity is erased.
- lines["a\nb"] with Lf/trailing=false and lines["a","b"] encode identical `a\nb`; raw decode yields two lines. Embedded separator placement is erased.
- empty raw payload cannot encode chosen CrLf without trailing; from_body chooses Lf. A line containing CRLF can also cause raw decoder to choose CrLf despite originally chosen Lf.

These are deterministic source collisions, not executed tests. No original grant, better cancellation law or paid envelope can recover information that raw native encoding omitted. Direct semantic SQLite rows preserve schema, exact line records/count, boolean and chosen ending, so their typed route has sufficient information.

## Minimum Full-Scope Original API

Replace the current typed file functions' cold implementation with genuine supplied-owner typed functions taking original SQL control and DecodeOwner for snapshot→SQLite bytes, EncodeOwner for SQLite bytes→typed snapshot, plus exact dialect. Retain registered typed_snapshot_codec identity validation. Borrow original source P; do not clone it or invoke ArtifactPack/ArtifactDsl. Call explicit P::to_sqlite_database_receiving/from_sqlite_database_receiving and directional validators under the same original nested owner.

Export original receiving frame holds Option<SqliteDatabase>, diagnostics/validation and Option<Vec<u8>>/IoOutcome file output before any birth; import frame holds original input backing/adoption state, parsed database/metadata/dialect, Option<P> and diagnostic/output slots. Attach/take file metadata through actual receiving helpers. export_sqlite_database_controlled/import_sqlite_database_controlled must use the original native.with_encoding_receiver allocation port and SQL with_original_allocation plus actual workspace recipient; retire scratch workspace through original remaining grant. No fresh controller, invented authority or post-success local output settlement is permitted.

Keep all fallible identity/schema/metadata/SQL accounting/validation/retirement work inside receiving custody before output take. Settle accepted copy/capacity/release and original cause at every refused seam; original observer cancellation during parser/writer work must reach its exact typed cause, then same caller observer can deny/rearm funded close. Successful native payload bridge remains a representation bridge; it must not be advertised as complete typed snapshot semantics.

Author schema-first neutral full-snapshot law with the four collisions above producing distinct semantic row sets and direct SQLite files, then imports compare all4 original fields exactly. Independent SQLite queries witness exact schema/document flags/line ordinal/content and Buffer bytes; actual Native laws invoke supplied-owner typed file API in both directions with original grants, cumulative prior receipts, canceled schema/line/file prefix and funded cleanup. Existing raw native semantics and legal TxtSnapshot schema remain unchanged; no fallback or legacy codec branch is needed.
