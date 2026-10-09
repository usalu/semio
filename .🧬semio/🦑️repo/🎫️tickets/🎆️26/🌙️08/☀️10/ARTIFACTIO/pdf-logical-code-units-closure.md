# PDF Logical Character Code Ownership

The schema now represents text operands with `codes: u32[]`. Raw native string octets and font codespace width grouping belong to physical content admission/emission. Snapshot Rust, JSON, GraphQL, proto and TypeScript contracts use the integer owner. Native JSON TypeScript admission checks the unsigned word range. Rust and TypeScript SQLite project codes as ordered integer rows in `pdf_operation_code` and `pdf_array_item_code`; parent rows hold explicit counts. The native lowerer now maps admitted words directly to glyphs rather than serialize and decode them again.

Three language-neutral fixtures cover one-byte simple, two-byte Identity-H, and four-byte embedded codespaces, including zero, 0x80ff and 0xffffffff. Native stream fragments are joined before content parsing. The new Rust test compares code grouping with lopdf through a first-party oracle interface, re-emits typed operators, and repeats independent code comparison. TypeScript independently checks SQLite integer rows and edits one code to the full unsigned maximum.

The complete-domain SQLite fixture's authored expected census changed from 1274 to 1278 rows and from 336218 to 336358 owned value bytes: four integer code rows replace two two-byte BLOB payloads with integer count fields. This expected census remains to be confirmed by the registered tests.

No fresh runtime pass has been observed. Earlier registered jobs disappeared during host resets before producing outcomes, and ticket generated output was removed externally. The implementation remains unverified until a fresh registered run completes.

## Remaining Retained Graph Work

`PdfObject::Str` and `Stream.data` still retain native known-role representations, and semantic `carry_graph_edit` still reaches the native lifter. Full closure must author typed text/date, contextual complete operator sequences, CMap/ToUnicode, native-sample image records, calculator operator programs and font code mappings. Uninterpreted attachments and program bodies may retain explicitly opaque bytes. The new character-code owner is one prerequisite; it does not establish the retained graph closure.

## Retained Text/Date Admission And Fresh Runtime Evidence

The canonical COS owner now has explicit Text(String) and Date(PdfDate), with authored JSON/GraphQL/proto contracts and TypeScript twins. Native admission resolves known direct/indirect string roles including indirect arrays, preserves indirect identities, and keeps unknown octets opaque. Physical text emission and encryption bind the admitted values; consumers of destinations, filespecs, actions, forms, name trees and CID system fields accept logical text. Text/date SQL uses explicit date_id foreign keys into pdf_date; standalone COS owns the same date relation. A neutral fixture covers PDFDocEncoding/UTF16 text, native dates and indirect ownership; lopdf supplies an independent grammar/text decoder. Rust admission and direct SQL tests are authored but have not yet executed in the current queue.

Fresh registered @semio-tech/stdio-pdf:test session 77516 executed PDF1.4 7 pass/0 fail (199 assertions), then PDF1.7 COS 3 pass/2 fail (1054 assertions). The new neutral retained Text/Date direct SQLite query, foreign-key, exact roundtrip and independent SQL edit test passed. The two failures were a stale table-count assertion (7 to 8 tables) and an omitted semioPrimitive i64 annotation after copying the canonical authored schema into the TypeScript literal. Both source defects were repaired, including the authored schemas, and a fresh registered retry was launched. Later suites did not run in this failing invocation. No aggregate pass is claimed.

Current PNG8/BMP10/PDF logical3 native sessions remained live preparing when inspected. Their redirected logs were removed externally while processes retained open inodes. Their session streams contain no recovered runtime output; no result is claimed from those jobs. Future verification retains console tool output instead of redirecting it away.

The full retained PDF program/CMap/image/calculator/font mapping closure is still required; schema diff and conformance-support continue importing carry_graph_edit from physical IO. This stage does not establish full graph purity.

## Fresh canonical contract runtime verification

Registered `@semio-tech/stdio-pdf:test` session 96585 completed with exit 0 in 33.6 seconds after authored Binary64 word contracts and explicit u64 annotation indices were synchronized with TypeScript owned model admission. All eleven registered suites executed. PDF1.4: 7 passed, 199 assertions. COS: 5 passed, 1058 assertions, including the new neutral Text/Date typed-row test. Annotation: 5 passed. Metadata: 5 passed, 167 assertions. Form: 3 passed, 10 assertions. Full document: 7 passed, 52 assertions; the independent complete SQL census passed. This confirms the current TypeScript/SQL stage, not the full Rust or retained stream projection architecture.

The preceding 53284 run reached annotation admission and failed one test because canonical popup/inReplyTo/Popup.parent schema nodes still expected JavaScript numbers. The source repair declares the actual u64 owned admission role and full range. No rejection check was weakened. Current canonical real JSON contracts require an exact 16-hex bit word and the TypeScript semantic admission requires the first-party Binary64 bigint owner; native transport number admission stays under I/O.
