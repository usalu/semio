# TypeScript Relational Engine And First Three Semantic Providers

Later audit repairs and the eight-provider implementation are documented in [TypeScript Audit Repairs And Eight Semantic Providers](./📓️typescript-audit-and-domain-facets.md). Its executed checks supersede the initial three-provider totals below.

The updated goal requires explicit artifact entities and relationships. The obsolete whole-snapshot carrier API was removed; the TypeScript public package exposes `exportSqliteDatabase`, `importSqliteDatabase`, `parseSqliteDatabaseSchema`, and `validateSqliteDatabaseSchema` with owned typed scalar cells, row identities, handcrafted SQL, cancellation, aggregate limits and physical-page progress.

## Delivered Semantic Facets

CSV declares `csv_document`, `csv_record` and `csv_field`, retaining document schema, header flag, field quoting, text and explicit collection ordinals. TXT declares `text_document` and `text_line`, retaining LF/CRLF, trailing newline and text including empty lines, Unicode and NUL. BINARY declares `binary_document` and `binary_byte`; each byte is a constrained integer entity. The three TypeScript schema constants are tested byte-equal to the adjacent handcrafted native SQL assets. Facets share owned validation/progress helpers and have no filesystem or external-library runtime imports.

All reconstruction validates complete expected DDL and row identities, one identity-1 document, references, contiguous unique collection ordinals and typed constrained cells. Binary import reads the byte from column index 3 and rejects values outside 0..255. The existing binary snapshot parser was corrected to require integer byte arrays, matching its public interface rather than accepting a string.

Projection checks maxRows and aggregate maxValueBytes before allocating relational entity arrays. Estimates match native semantic cell accounting: binary 32 bytes per byte entity plus schema UTF-8 and document identity; TXT 24 bytes per line entity plus line text and schema/document properties; CSV 24 bytes per record and 32 bytes per field plus text and schema/document properties. Counting scans yield to cancellation every 256 entities. Shared progress phases are projectSnapshot/reconstructSnapshot, with physical readPages/writePages retained.

## Physical SQLite Coverage

The runtime writes and reads real SQLite pages without a SQLite runtime dependency. It handles signed 64-bit rowids, INTEGER PRIMARY KEY aliases, NULL/REAL/TEXT/intrinsic binary cells, schema and row trees with multiple interior levels, overflow chains, independent SQLite page sizes 512..65536, empty tables, wide serial-header varints and a schema root with one child. It rejects unsupported indexed/UNIQUE/WITHOUT ROWID schemas and corrupt row ordering, separator bounds, pages, fragments, overlaps, overflow cycles, headers and UTF-8.

Unicode scalar strings roundtrip through independent SQLite. Unpaired UTF-16 strings reject for cells, SQL and table names before TextEncoder allocation. Independent Unicode tests include NUL, supplementary-plane scalars, combining-compatible text and BOM; no replacement-character normalization is permitted.

## TDD And Independent Oracle

Initial tests failed before the physical relational API and schema helpers existed. Provider tests initially failed with absent TypeScript facets. The aggregate projection limit regressions failed in all three providers before byte-budget checks were implemented. UTF-16 rejection already existed via encode/decode identity checking; a new regression independently validates scalar roundtrip and all malformed string positions, then rejection was moved before encoded allocation.

Each provider consumes the shared language-neutral native fixture. Bun SQLite independently checks integrity, foreign keys and entity queries, edits a semantic field/line/byte and verifies reconstructed native semantics, then edits a dangling relationship, ordinal, boolean or byte constraint and verifies rejection. These databases remain directly queryable without invoking any native artifact codec.

## Executed Checks

- Final explicit-path Bun run of physical unit tests and CSV/TXT/BINARY SQLite facet tests: **34 passed, 0 failed, 167 assertions, 4 files, 1185 ms**. Includes semantic byte-budget rejection and cancellation during preallocation counting scans.
- `NX_DAEMON=false bun nx run @semio-tech/framework:typecheck --skip-nx-cache`: **passed**, 28.9 seconds. The task waited for an existing shared graph construction, then completed the registered package script successfully.
- The preserved ticket `📜️script.ts` public consumer probe imports the actual `@semio-tech/framework` APIs and types, exports entity/relation tables, imports them, and executes an independent SQLite semantic join and integrity check. Runtime output: `[DEBUG] public relational SQLite API {"tables":["entity","relation"],"rows":[1,1],"bytes":12288,"phases":["writePages","readPages"]}`.
- Source import review: physical core has zero runtime imports. The semantic helper imports only the owned core. All three facets import own snapshot types and owned framework helper/core; Bun SQLite appears only in tests.
- Expanded targeted compiler validation using the framework package tsconfig and TypeScript compiler API on core, public entrypoint, public consumer, helper, all three facets, their tests, snapshot typed boundaries and root interoperability harness: **0 diagnostics**, exit 0. Earlier interoperability safeIntegers typing was fixed by the harness owner using a local test-only statement interface.

Scope delivered here is the general TypeScript physical engine and first three explicit semantic facets. Other artifact families and universal IO dispatch are coordinated by the parent ticket.
