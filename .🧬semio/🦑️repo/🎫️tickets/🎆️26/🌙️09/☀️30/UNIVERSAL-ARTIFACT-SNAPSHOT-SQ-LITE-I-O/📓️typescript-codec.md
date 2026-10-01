# TypeScript SQLite Snapshot Codec

## Design

The runtime codec writes SQLite 3 bytes directly and reads table B-trees without importing a SQLite dependency. The fixed schema precedes implementation. SQLite's [authoritative file format](https://sqlite.org/fileformat.html) defines table leaf payload placement, overflow chains, record serial types, and page headers. Exports use 4096-byte pages with sqlite_schema at page 1 and the snapshot table at page 2; imports discover the snapshot root from sqlite_schema and accept all supported SQLite page sizes. Imports inspect the SQL column contract and the stored row types independently.

Progress and cancellation run at visited or emitted page boundaries. The reader bounds file, snapshot, coordinate, page count, record headers and schema records before allocating. It rejects duplicate page ownership, cycles, truncated cells, invalid UTF-8, invalid identities, versions, row counts and dialect mismatches.

## Validation

Tests are authored before the codec and drive the language-neutral shared corpus. Bun's SQLite engine independently validates every exported database and creates databases imported by the codec, including alternate page sizes, moved schema roots and overflow chains. Results will be appended after execution.

The initial explicit-path Bun invocation failed because the codec module did not yet exist, confirming the red stage. After implementation and hardening, `bun test ./🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🟦️.ts` passed **32 tests, 215 assertions, 0 failures**. Every shared payload fixture passed exporter -> independent Bun SQLite `integrity_check` and row/schema verification, independent Bun SQLite -> importer, and direct codec roundtrip.

The final run consumed the shared corpus's `malformedCoordinates` collection in both exporter and independently created SQLite importer refusals, with the same **32 passing tests and 215 assertions** (154 ms). Root coordination reported an earlier registered Nx run covering the initial 26-test suite; the coordinator will rerun the registered combined target for the final 32-test suite and cross-language interoperability.

Additional runtime validation covers page sizes 512, 1024, 2048, 8192, 16384, 32768 and 65536; more than 90 unrelated tables causing schema interior pages and moved snapshot roots; real schema freeblocks after deletion; snapshot freelist pages after 500 deletions; nullable SQL for autoindexes and zero roots for views; differently quoted SQL identifiers; native text BOM/Unicode/NUL; sliced byte buffers; malformed file identity/version/header, row counts/types, coordinate control characters, cell arrays, freeblock pointers, cycles, overflow termination, resource limits and callback/timer-driven cancellation.

An isolated runtime command logged `[DEBUG] sqlite snapshot runtime {"header":"SQLite format 3","fileBytes":8192,"dialect":"s.test.snapshot@1/*","payload":[0,255]}` after export and import. No debug logging was added to permanent runtime source.

The first Bun invocation without `./` treated the filename as a broad repository filter and crashed in Bun's runner after scanning the huge repository. Explicit paths execute the intended file immediately; the registered runner must retain the `./` prefix. No temporary generated files were created by these in-memory tests.

## API

`exportSnapshotSqlite(value, options?)` returns `Promise<Uint8Array>`. `importSnapshotSqlite(bytes, expectedDialect?, options?)` returns `Promise<SqliteSnapshot>`. A value contains exact `dialect`, `encoding: "binary" | "text"` and `snapshot: Uint8Array`. Options include `signal`, `onProgress`, `maxFileBytes`, `maxSnapshotBytes`, `maxCoordinateBytes`, and `maxPages`. Defaults are 272 MiB file, 256 MiB snapshot, 65536 coordinate bytes, 1000000 pages. A canonical coordinate needs nonempty kind, standard and subset around the first `@` and last `/`; C0/C1 controls and invalid UTF-8 are rejected. Cancellation throws an error named `AbortError`.

The exporter copies native bytes directly into database pages without building a second complete record buffer. The reader returns a view of its independently allocated record payload, avoiding another complete snapshot copy and never exposing a view into the caller's database file buffer.

## Independent Audit Repair

The read-only codec audit identified that the TypeScript table reader skipped interior separator and leaf rowid values. An independently generated 512-byte-page database with 90 auxiliary tables demonstrated that changing the first schema separator to zero produced SQLite's `Rowid ... out of order` integrity error while the importer accepted the database. The regression was added first and failed because the import promise resolved.

The reader now interprets rowids and separators as signed 64-bit `BigInt` values with `BigInt.asIntN(64, ...)`, checks strict ascending keys within every page, and propagates each interior child's exclusive lower and inclusive upper bounds. The rightmost child inherits the last separator as its lower bound and its parent's upper bound. Traversal processes children in table order.

The independent-engine regression includes four separately corrupted inputs: first separator violating its child's upper bound, duplicate interior separator, duplicate leaf rowid, and a rightmost leaf rowid violating its inherited lower bound. Bun SQLite independently reports `out of order` for all four, and the codec rejects all four. Independent valid snapshot tables use rowids `-9223372036854775808`, `-1`, `0`, and `9223372036854775807`; all import successfully. Two-row native tables exercise negative-to-positive ordering and adjacent maximum signed integers, and correctly fail only the snapshot row-count contract rather than rowid ordering. Direct writable_schema manipulation was unavailable in Bun's defensive SQLite mode, so the valid signed-boundary test uses native table rowids generated by SQLite.

After the audit repair, the direct explicit-path Bun suite passed **35 tests, 240 assertions, 0 failures** in **114 ms**. The coordinator owns the final registered Nx and combined interoperability rerun.

## Final Schema Audit Repair

The final audit found that valid SQLite uppercase snapshot table names were skipped and that quoted SQL keywords were incorrectly normalized as ordinary keywords. Two independent-engine regressions were added before changing the parser. The uppercase file returned `integrity_check: ok` but the importer rejected it as missing; the byte-preserving quoted `"CREATE"` mutation caused Bun SQLite to throw `malformed schema`, while the importer accepted it. Both regressions failed as expected before the repair.

Schema table identifiers and SQL tokens now use ASCII-only case folding. Matching `name` and `tbl_name` accepts SQLite's uppercase and mixed-case table naming. Quoted tokens are permitted only in the fixed table-name and three column-name positions; the CREATE/TABLE/type/NOT/NULL keyword positions must contain unquoted keywords. Existing valid quoted identifier tests continue to pass.

The final direct explicit-path Bun suite passed **37 tests, 246 assertions, 0 failures** in **110 ms**. This repair changed only the TypeScript codec, its unit test source, and this report. The coordinator owns the final combined registered validation.

## Public Package Verification

A consumer probe is preserved as this ticket's `📜️script.ts`. It imports `exportSnapshotSqlite`, `importSnapshotSqlite`, `SqliteSnapshot`, `SqliteSnapshotOptions`, and `SqliteSnapshotProgress` from the actual `@semio-tech/framework` package name, assigns both functions to their explicit public async signatures, supplies all four resource-limit fields plus a typed progress callback, and asserts a native-byte roundtrip. The package entrypoint explicitly reexports both functions and all three owned interfaces. The codec source has no runtime imports; its public signatures use owned interfaces and platform `Uint8Array`/`AbortSignal` types only.

`bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📜️script.ts` exited 0 and logged `[DEBUG] public framework SQLite snapshot API {"dialect":"s.test.snapshot@1/*","payload":[0,128,255],"bytes":8192,"progressEvents":5}`.

An isolated `bun -e` TypeScript compiler-API check loaded the existing framework package tsconfig, created a program rooted at the consumer probe, codec, and package entrypoint, and checked all pre-emit diagnostics associated with those three files plus configuration-level diagnostics. It exited 0 with `[DEBUG] public framework SQLite API type check: 0 diagnostics in consumer probe, codec, and package entrypoint`. This targeted result does not claim every unrelated imported framework module typechecks. The existing registered package-wide target `NX_DAEMON=false bun nx run @semio-tech/framework:typecheck --skip-nx-cache` is recorded separately once complete.

The registered package-wide command completed in 1 minute 15 seconds with exit 1. Its only reported diagnostics were TS2769 matcher generic mismatches at `🪶️sqlite-snapshot/🧪️tests/🤝️interoperability/🟦️.ts` lines 38, 64 and 80: expected `Uint8Array<ArrayBuffer>`, supplied `Uint8Array<ArrayBufferLike>`. No codec/entrypoint/consumer-probe error was reported. The coordinator owns those interoperability assertions and was asked to normalize byte comparisons. This worker did not change IO, Rust or catalog code during the public API follow-up.

The user subsequently superseded the opaque carrier goal with per-artifact semantic relational SQLite schemas. The carrier API verification is now parser/public-access evidence, not satisfaction of the updated goal. The separate `📓️relational-typescript-plan.md` records the foundation's reusable capabilities, missing relational primitives and recommended schema-first provider boundary. No additional carrier features were added after the scope change.

## Relational Physical Engine

The implementation now replaces the obsolete opaque carrier with `exportSqliteDatabase` and `importSqliteDatabase`, plus owned scalar/row/table/database/options/progress interfaces. The public framework entrypoint was updated without compatibility APIs. Typed cells preserve NULL, signed64 INTEGER, finite REAL, UTF-8 TEXT and intrinsic binary scalars. INTEGER PRIMARY KEY aliases store NULL physically and reconstruct the signed rowid; REAL-affinity integer storage reconstructs a number, matching SQLite.

The writer reserves schema/table roots, packs many records into leaves, emits overflow pages and builds multi-level interior B-trees. The reader preserves independently audited freeblock, fragment, alias/cycle, UTF-8 and rowid/separator-bound checks. Trusted handcrafted DDL supports quoted identifiers, comments, inline and single-column table INTEGER PRIMARY KEY; indexed/UNIQUE/non-INTEGER PK/DESC/AUTOINCREMENT/WITHOUT ROWID/generated schemas fail explicitly. The current physical foundation accepts semantic tables and performs no native artifact codec operation.

Fresh resource options match native: maxFileBytes272MiB, maxValueBytes256MiB aggregate semantic cell data, maxSchemaBytes4MiB aggregate SQL/name UTF-8, maxRows1000000 aggregate entities, maxColumns1024 per table, maxTables4096, maxPages1000000. Numeric cells including reconstructed aliases count8 bytes, NULL0, text UTF-8 bytes, binary length. Progress reports writePages/readPages; planning yields every64 rows/pages for cancellation.

The new unit suite failed before implementation due missing general API exports. Final direct command `bun test ./🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🟦️.ts` passed **18 tests, 94 assertions, 0 failures** in **998ms**. Coverage includes the shared language-neutral building/room corpus and independent joins/FK checks; typed scalar both-direction interoperability; SQL-edited entity import and reexport; 10000 long-label rows forcing multiple interior levels and100 schema tables; all page sizes512..65536; moved roots and overflow; signed64 boundaries; empty databases/tables; quoted uppercase identifiers; table-level aliases; single-child schema root;600-column records and header varints; allocation limits; independently malformed separators/quoted CREATE; fragment corruption; truncated/cyclic/aliased overflow; sliced buffers; callback/timer cancellation.

The updated ticket consumer probe imports the actual public package, exports/imports entity/relation tables and independently runs a SQL join/integrity check. It exited0 with `[DEBUG] public relational SQLite API {"tables":["entity","relation"],"rows":[1,1],"bytes":12288,"phases":["writePages","readPages"]}`. Targeted TypeScript diagnostics for the probe, engine, unit tests and entrypoint returned0. One missed overflow progress phase and stale Bun safeIntegers type surface were fixed; the oracle uses `CAST(integer_value AS TEXT)` to independently verify exact signed64 values. The coordinator owns combined Nx/native/provider validation.
# Relational Scope Completion Follow-Up

The current general physical engine and first three explicit semantic providers supersede historical whole-snapshot carrier checks below. See [TypeScript Relational Engine And First Three Semantic Providers](./📓️typescript-providers.md) for the current API, handwritten schema contracts, independent SQLite oracles, preallocation resource/cancellation bounds, Unicode integrity and exact successful Nx/typecheck/test results.
