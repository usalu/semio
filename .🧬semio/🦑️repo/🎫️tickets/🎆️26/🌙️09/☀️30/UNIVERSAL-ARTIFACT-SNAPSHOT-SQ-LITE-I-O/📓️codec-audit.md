# SQLite Snapshot Codec Audit

Read-only review of the Rust/TypeScript codec, shared schema, corpus and interoperability tests. Reviewed live source on 2026-10-01; implementation workers may subsequently resolve findings.

## Actionable Findings

1. **TypeScript accepts invalid B-tree separator keys (confirmed at runtime).** `Reader.table` decodes rowids and interior separator varints only to advance the cursor; it does not enforce ascending signed rowids or inherited child bounds. Rust already carries lower/upper bounds. Reproduction: independently create an in-memory Bun SQLite database with page_size=512, application_id=1397576526, user_version=1, ninety `auxiliary_i(value TEXT)` tables, then the canonical artifact_snapshot table and one binary row. Serialize, read the first schema interior cell pointer with `new DataView(bytes.buffer).getUint16(112)`, and set `bytes[pointer + 4] = 0` (first separator key). TypeScript successfully returns the snapshot; Bun SQLite `PRAGMA integrity_check` reports `Tree 1 page 8 cell 5: Rowid 6 out of order`. Repair with signed 64-bit rowid decoding, ascending keys and child-bound propagation; add independent-engine mutation regression.

2. **Rust does not validate fragmented-byte accounting (source-confirmed).** `Reader.table` only rejects overlapping ranges after sorting. It does not ensure every untracked gap is at most three bytes or that all fragment bytes total the page header value. Canonical export has no fragments; changing offset 4103 (page 2 header byte 7) from 0 to 1 remains accepted by the reviewed Rust code, whereas TypeScript rejects it. Rust worker has been informed by coordinator. Include independently inspected mutation coverage and align checks with TypeScript.

3. **Header acceptance differs for valid SQLite stale-size metadata (source-confirmed).** TypeScript observes the SQLite rule that offset 28 page count is authoritative only when nonzero and offsets 24/92 agree; Rust requires a matching page count and matching counters unconditionally. Old but valid standalone SQLite files therefore have different import behavior. Either align with SQLite validity rules or explicitly describe a narrower transfer contract in the schema; current API promises independent SQLite files.

## Coverage and Limits

The shared corpus feeds both language implementations, and the interoperability test exercises both export directions plus Bun SQLite integrity/schema/value checks and independently generated files at several page sizes. This is strong representation validation. The test currently checks round trips but does not assert canonical Rust-export bytes equal TypeScript-export bytes. Add a byte equality assertion if deterministic parity is part of the contract.

The JSON coordinate schema `^.+@.+/.+$` also permits control-bearing coordinates that both implementations reject; the shared schema should express the actual coordinate contract. No obvious unchecked malformed-file indexing panic was found in the Rust reader after its minimum usable-page and cell-boundary checks. Default allocation ceilings are checked before payload allocation. Rust cancellation callbacks are page based; UTF-8 validation is a synchronous whole-buffer scan, so cancellation cannot interrupt that scan itself.

Only the separator-key reproduction was executed during this audit. No full Nx test suite result is claimed.

## Follow-up Review After Repairs

Re-read the repaired source. Findings 1–3 above are resolved in implementation: TypeScript carries signed 64-bit lower/upper rowid bounds, Rust validates free-space accounting, and Rust observes conditional authoritative header page count. Coordinator is adding direct canonical byte equality and corruption-corpus coverage. No new Rust panic defect found.

Two remaining TypeScript schema parity defects were independently reproduced at runtime:

- **Valid uppercase table names rejected:** create the canonical SQL with table name `ARTIFACT_SNAPSHOT` using Bun SQLite. SQLite integrity_check returns `ok`, but TypeScript import throws `artifact_snapshot schema missing` because schema name and tbl_name comparisons are case-sensitive. Rust now uses ASCII case-insensitive matching. Match ASCII case consistently, preserving exact snapshot dialect identity.
- **Malformed quoted SQL keyword accepted:** export a canonical SQLite snapshot, find the SCHEMA_SQL bytes, and replace `CREATE` with `"CREATE"` while removing the spaces after both commas to preserve SQL byte length. TypeScript imports the snapshot successfully, but Bun SQLite throws `malformed database schema (artifact_snapshot)`. TypeScript removes quoting from all tokens; Rust now restricts quoted tokens to table/column identifier positions. Apply that same restriction in TypeScript and regress against the independent engine.

Reproduction output is temporarily in `🗑️generated/codec-followup.txt`; coordinator should delete generated outputs with ticket cleanup. No additional material container defects identified during this bounded follow-up review. Nx results are owned by coordinator; this audit does not claim a fresh full-suite run.

## Final Repair Verification

Re-read TypeScript `asciiLower`, `validSchema`, and schema-table selection after the final repairs. Both follow-up defects are resolved: only table/column identifier positions admit quoting, and schema names/tbl_name use ASCII case folding. Reviewed the existing independent SQLite regression tests for both cases. Executed the full focused TypeScript unit file successfully with `bun test ./🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🟦️.ts`; execution output is under ticket `🗑️generated/codec-final-audit.txt`. No remaining concrete blocker identified within the container-codec audit scope. Coordinator owns broader combined Nx and interoperability results.

## Semantic Relational Engine Audit

This section supersedes the earlier opaque-carrier scope. Reviewed current general Rust/TypeScript physical engines and explicit CSV/TXT/BINARY/TSV native providers plus the three TypeScript mirrors. CSV preserves header, record/field ordinals, quote flags and text; TXT preserves schema, ordered lines, LF/CRLF and trailing newline; BINARY stores each byte as an integer; TSV preserves schema, record/field order, line ending and trailing newline. Relationship and contiguous-ordinal reconstruction checks are explicit. No opaque snapshot BLOB/JSON fallback appeared in these providers.

### Actionable Findings

1. **TypeScript schema comparison loses quoted-keyword semantics (runtime-confirmed).** `validateSqliteDatabaseSchema` normalizes every nonliteral token identically, dropping its quoted flag. Expected `CREATE TABLE t (id INTEGER PRIMARY KEY, value TEXT NOT NULL)` accepts actual `CREATE TABLE t (id INTEGER PRIMARY KEY, value TEXT "NOT" "NULL")`, including a null value. Bun SQLite accepts that actual DDL as a different type declaration: independent `PRAGMA table_info(t)` reports type `TEXT "NOT" "NULL"` and `notnull=0`. Keep quoting distinctions for syntax/constraint keywords while allowing quoted identifiers. This is a real weakened-schema acceptance issue, although individual typed artifact reconstruction will still reject null where it requires TEXT.

2. **Semantic reconstruction omits aggregate value budget (runtime-confirmed TypeScript; source-confirmed native).** `csvSnapshotFromSqliteDatabase(validProjectedDb, {maxValueBytes:0})` returns a nonempty snapshot rather than rejecting. Shared TypeScript `artifactSqliteTables` validates schema, rows and aliases but does not count cell bytes; CSV/TXT/BINARY reconstructors allocate output without a value-budget pass. Reviewed native CSV/TXT/TSV reconstructors similarly check rows but do not call `check_value_bytes` before cloning fields. File-import limits upstream do not cover direct public projection/reconstruction calls. Bound/count semantic values before allocating or cloning output.

3. **Rust physical import buffers table payloads before value limits (source-confirmed).** `import_sqlite_database` calls `reader.table(root, limits.max_file_bytes, remaining_rows)`; that function collects every record payload into a Vec before `read_record` or aggregate `value_bytes` checks run. A very small max_value_bytes still permits allocation of all large table payloads up to the file limit. Apply aggregate remaining-value/record-header limits during traversal or stream rows so limits are enforced before payload allocation. TypeScript export also calls `encodedField(value)` (allocating UTF-8 bytes) before adding/checking aggregate `dataBytes`, unlike native export's preflight.

4. **Native TSV preflight cancellation can stall on one large record (source-confirmed).** Its value-byte counting loop checkpoints per record, but its inner field loop has no checkpoints. A single very large record scans all fields before observing cancellation. Native TXT's value-byte `try_fold` similarly scans every line without checkpoints. Add bounded checkpoints during expensive preallocation scans.

Reproductions for findings 1–2 executed with Bun and an independent SQLite engine; retained temporary output is `🗑️generated/semantic-codec-audit.txt`. No fresh general-engine test-suite run is claimed here; coordinator owns test evidence. No obvious ownership cycle can occur in these four flat document/record/field provider schemas because parent entities live in separate fixed-level tables. This review does not establish completion for the retained full artifact inventory or ownership graphs in other providers.
