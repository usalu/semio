# TypeScript Audit Repairs And Eight Semantic Providers

This follow-up implements the TypeScript findings in the semantic relational audit and adds TSV, JSON, XML, STL and OBJ mirrors beside their stable handwritten native SQL schemas. CSV/TXT/BINARY remain explicit domain providers. No snapshot JSON/BLOB carrier, native-codec fallback, external runtime library or filesystem import was added.

## Audit Repairs

The new independent SQLite regression reproduces `value TEXT "NOT" "NULL"` as a valid different type declaration with `notnull=0`, then asserts it cannot satisfy the expected `value TEXT NOT NULL` schema. Table/column/reference identifier quoting still accepts valid equivalent DDL. The comparator preserves quote distinctions for syntax/type/constraint tokens and permits quote differences only at trusted expected identifier positions.

The new TextEncoder allocation-observation regression demonstrates export used to encode an oversized text cell and oversized schema before rejecting their aggregate budgets. Export now performs a cancellable row/schema/value preflight before any page or encoded-cell allocation. Owned `sqliteValueByteLength` validates and counts scalar semantic bytes without an encoded copy; UTF-16 surrogates reject before encoding.

CSV/TXT/BINARY reconstruction regressions independently serialize through Bun SQLite and reproduce a successful nonempty reconstruction with maxValueBytes=0. All reconstructors now await the shared helper's aggregate typed-cell budget scan before allocating their domain output. The helper scans at bounded cancellation intervals, validates the complete handwritten schema and row identities, and returns expected table rows. Projection retains preallocation value estimates and bounded scans.

## Added Domain Facets

| Artifact | Explicit SQL domain |
| --- | --- |
| TSV | Document newline properties, ordered records and fields; no quoting or header semantics |
| JSON RFC8259 base | Document, typed primitive/container values, ordered object members and array elements; duplicate keys and exact arbitrary-size number lexemes retained |
| XML 1.0 base | Thirteen document/node/typed component/attribute/child/misc/declaration/doctype/entity tables |
| STL ASCII | Named solid, persisted facet normals, exactly three ordered owned vertices per facet |
| OBJ geometry | Fourteen document/geometry/corner/group/object/membership/boundary/material/smoothing/unknown statement tables |

All five schema constants are asserted byte-equal to the adjacent SQL assets. All providers have independent Bun SQLite integrity/foreign-key checks, entity queries, SQL edits and malformed relational checks. JSON rejects bad number grammar, dangling/multiple-owner/disconnected-cycle graphs and ordinal gaps; iterative projection/reconstruction handles 3,000 nesting levels. XML validates typed component ownership, declarations, optional external-ID variants and boundaries; iterative graph reconstruction handles 2,000 levels. STL preserves uncomputed normals and validates finite coordinates, positive facet IDs and exactly three ordered vertices. OBJ preserves nullable coordinate/reference fields, ordered multiple memberships, source statements and explicit ranges beginning at the final face boundary.

The existing JSON snapshot parser called an undefined `parseJsonValue`. Its runtime ReferenceError was reproduced before adding its owned union validator; the new semantic fixture test verifies the typed parser preserves the exact native model.

## Executed Checks

- Full explicit-path Bun test run over the physical engine and all eight semantic provider test files: **60 passed, 0 failed, 356 assertions, 9 files, 6.91 seconds**. Earlier runs saw the separately owned IO ownership-signature regression during active edits; the final run includes the corrected owned-transfer/trait-borrow test and passes it.
- Existing registered script seam `bun ./🧰️framework/📦️packages/🦀️rust/📜️script.ts test-snapshot-sqlite source`: **26 passed, 0 failed, 155 assertions, 1.225 seconds**.
- Registered package check `NX_DAEMON=false bun nx run @semio-tech/framework:typecheck --skip-nx-cache`: **passed**, 1 minute 49 seconds.
- Expanded compiler-API check using the framework package tsconfig with core/public entrypoint/public consumer/helper/interop and all eight providers, tests and snapshot typed boundaries: **0 TypeScript diagnostics**, exit 0.
- Preserved ticket public package consumer probe rerun: **passed**, runtime `[DEBUG] public relational SQLite API {"tables":["entity","relation"],"rows":[1,1],"bytes":12288,"phases":["writePages","readPages"]}`, including independent SQLite join and integrity verification.
- Runtime import review: physical core imports nothing; provider imports resolve only to owned core/helper and owned snapshot declarations/validator. Independent Bun SQLite is confined to tests.

The parent owns registration, native sources and IO. Foundation registration already includes the first six providers; geometry registration needs the STL/OBJ TypeScript tests added alongside its native tests.

## Model Boundary To Resolve

OBJ's TypeScript `unknownStatement.lineIndex` now uses bigint in its artifact, snapshot and diff models, with mutation aliases inheriting the same owned type. Its artifact and diff parsers require unsigned64 bigint without accepting numbers or strings. All four GraphQL assets use an explicit ObjSourceLineUInt64 scalar, and the JSON schemas constrain the complete unsigned64 domain. Native numeric JSON fixtures remain language-neutral; TypeScript fixture loading explicitly lifts their ordinals into bigint. The adjacent full-width fixture uses decimal strings as language-neutral exact test input.

The SQLite mapping splits ordinals into source_line_ordinal_high/source_line_ordinal_low INTEGER columns bounded to unsigned32, reconstructing with bigint shifts. This preserves values beyond signed64, including 18446744073709551615, while retaining numeric query/order semantics. Geometry references and smoothing groups remain unsigned32 and fit JavaScript exactly.

## PLY And Precision Follow-Up

The ninth TypeScript semantic provider mirrors PLY's seven handcrafted tables: document, comment, element, property, row, cell and list item. Scalar payloads remain typed INTEGER/REAL cells with all eight native widths. Declarations preserve ordered generic properties, list count/value kinds, element counts, all three format modes and Unicode comments. Reconstruction rejects orphan/cross-element ownership, duplicate or missing cells, invalid scalar width, noncanonical float32 values, count overflow, conflicting primitive payloads and ordinal gaps. Projection preflights aggregate bytes/rows before entity allocation and yields during large list scans.

Five new PLY tests first failed because the provider was absent; a separate runtime probe reproduced the pre-existing missing parsePlyProperty implementation. The missing parsePlyProperty and parsePlyValue guards were implemented next to their existing snapshot declarations. The PLY independent Bun SQLite suite now passes **5 tests, 31 assertions**. It checks schema bytes, integrity/foreign keys, typed SQL queries, both-direction reconstruction, edited SQL cells/list items, resource bounds and cancellation.

The expanded compiler program covering core/public consumer, nine providers/tests/snapshot boundaries plus the adjacent OBJ artifact/diff/mutation types reports **0 diagnostics in the owned relational targets**. It exposes **26 existing diagnostics in OBJ's unrelated diff parsers**: undefined coordinate diff parsers, generic object guards returned as typed geometry, and optional values returned for required arrays. No diagnostic concerns the widened source-line field; the two strict unsigned64 parser boundaries are exercised directly by the OBJ tests. These unrelated diff parser repairs are outside this execution assignment.

Independent GraphQL parsing confirms all four OBJ assets parse and define their owned unsigned64 scalar. The public package consumer runtime probe was rerun successfully and logged two queryable tables, one relationship, 12,288 database bytes and writePages/readPages progress.

The fresh registered framework package check, `NX_DAEMON=false bun nx run @semio-tech/framework:typecheck --skip-nx-cache`, passed in **23.6 seconds**. Dedicated full-width OBJ oracle/parser/invalid-word tests pass **3 tests, 22 assertions**. They include unsigned64 maximum, a SQL edit to maximum minus one, exact values above 2^53, integrity/foreign-key checks and independently edited out-of-range high/low words. The complete geometry suite will be rerun after the concurrently authored native SQL asset adopts the same two word columns.

## BMP Follow-Up

BMP is the tenth semantic TypeScript provider. Its three handcrafted tables expose every typed header field, palette byte/reserved field and one explicit RGBA pixel per canonical grid position. Header108, noncanonical schema text, 32-bit pixels with alpha17, signed pixel-density metadata and full unsigned header maxima roundtrip independently without invoking a BMP codec. Both row orders and empty grids retain their typed values. Reconstruction requires a complete unique in-bounds grid and valid byte/word/dword metadata. Aggregate budgets and cancellation precede grid/entity allocations.

The missing-provider red test was reproduced before implementation. The resulting BMP suite passes **4 tests, 35 assertions** through independent Bun SQLite queries, integrity/foreign-key checks, palette/pixel SQL edits and malformed grid/header checks. The expanded ten-provider compiler program reports **0 owned diagnostics**, with the same 26 adjacent existing OBJ diff parser diagnostics recorded above.

## HTML, SVG And PNG Follow-Up

HTML independently authors eight tables for its own node union, ordered children, optional attribute values and raw script/style text. SQL NULL boolean attributes remain distinct from empty-string values. Scalar roots remain valid native HTML values. Projection/reconstruction reject cyclic/disconnected or multiply owned graphs, dangling typed components/owners and ordinal gaps; iterative traversal handles 2,500 levels. The missing-provider red and missing parseHtmlNode runtime ReferenceError were reproduced first, then the adjacent guard implemented. Its suite passes **4 tests, 28 assertions**.

SVG authors thirteen independent svg_* tables. XML now exports typed projectXmlDocument/reconstructXmlDocument helpers with an explicit thirteen-name XmlSqliteTables vocabulary, matching the native design. SVG calls those projections directly with its own handcrafted DDL; no completed database is renamed or transformed. Both directions enforce a root element named svg or ending in :svg. Declaration, DTD/entities, prolog/epilog, attributes, text/CDATA/comments/PI remain persisted typed fields. The SVG missing-provider red preceded implementation. XML and SVG together pass **8 tests, 64 assertions** after the shared typed-helper refactor.

PNG authors sixteen tables exposing the complete typed ancillary set, decoded RGBA coordinates, duplicate-keyword text chunks, exact ordered chunk references and intrinsic unknown chunk type/data octets. Separate palette presence preserves None versus Some(empty). One-to-one typed components, scalar ranges and unused nullable payloads are validated. Both directions enforce full unique in-bounds grids and all owned byte/alpha/marker relationships. No PNG codec is invoked. Its missing-provider red preceded implementation. The initial compiler check exposed seven existing PNG union-guard type errors; a runtime red proved a truncated RGB payload was accepted. Transparency/background/chunk-marker guards now require only the actual selected variant fields. The final PNG suite passes **6 tests, 57 assertions**.

The combined XML/HTML/SVG/BMP/PNG semantic suite before the final PNG guard regression passed **21 tests, 175 assertions, five files, 3.74 seconds**. The final separate PNG six-test run validates the subsequent guard repair. The fresh thirteen-provider compiler check after repair reports **0 owned diagnostics**, exit 0; the same 26 adjacent existing OBJ diff parser errors remain separately identified.

## Remaining TypeScript Checklist

Parent reassigned this executor to native DWG AC1024/full and the AC1018 shared owned model after PNG. Keep the thirteen authored TypeScript mirrors and their independent oracle tests intact. Subsequent TS work still includes DXF (full signed64 DxfValue::Int needs bigint), newly authored native Deflate/ZIP/Brep/shared blob semantics, Note/PDF/DWG/Semio and any still-unimplemented exact dialect families. Snapshot-specific schemas remain mandatory; no opaque carrier or generic JSON/entity fallback is authorized.
