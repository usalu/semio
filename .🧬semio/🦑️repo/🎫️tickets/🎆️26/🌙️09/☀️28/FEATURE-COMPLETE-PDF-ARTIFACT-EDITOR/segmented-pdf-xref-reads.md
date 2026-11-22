# Segmented PDF Cross-Reference Reads

The staged candidate adds byte-addressed reads to the classic cross-reference table and normal object offset validation. The table keeps up to 20 token bytes in the cursor as it advances, so token parsing can cross page boundaries without borrowing a contiguous source span or reserving heap space. A missing byte consumes the work unit used to inspect it. Offset validation forwards the same byte reader into the original indirect-object header cursor and checks both object number and generation. Existing slice callers supply local read adapters.

The two staged producers compose 19 exact source preimages. Shared changes to offset validation, the structured xref cursor, and the xref test root merge into the prior probe candidate. The resulting overlay lives under the ticket’s generated directory. Product source remained untouched.

The focused Rust executable uses the real staged footer, classic table and offset cursors alongside the repository’s checked range, sparse directory and revision merge. Its source wraps a 9,437,877-byte disk fixture through a checked range with a seven-byte logical prefix. The old classic and offset methods reject the segmented reader at compile time, even with the earlier footer and indirect-header adaptation present. The candidate validated each live entry and object generation after a paid revision merge. It handled a zero-work turn, allocation and retirement refusal, cancellation, malformed truncation, comments and numeric field widths. The directory returned all allocated backing on close. A 20-byte numeric token parsed and a 21-byte token failed.

| Segment bytes | Physical bytes read |
| ---: | ---: |
| 1 | 223 |
| 3 | 240 |
| 13 | 300 |
| 4,096 | 5,482 |

MuPDF 1.26.7 independently opened the fixture without repairing it and found five live objects, catalog object 1 and page object 3. The existing oracle reproduced the file checksum, image size, page geometry, and grayscale quadrant samples. It also accepted the separate 90-degree reference output.

The canonical Cargo closure reader accepted 32 measurements across Linux, macOS, Windows and WASI configurations. The xref-stream test closure had 3,998 active lines before the candidate. The first candidate had 4,005 and exceeded the 4,000-line cap. The final test closure has 31 files, 3,999 active lines and 186,237 bytes. The lexer test closure has 29 files, 3,658 lines and 155,680 bytes. The graph projection test closure has 28 files, 3,954 lines and 228,166 bytes with and without its testkit feature. These measurements assess source admission. The focused Rust executable exercises cursor and directory behavior. Application builds and an end-to-end segmented open remain unverified.

The structured parser still uses a slice to hand a classic trailer to the COS cursor. COS atom scanning retains offsets and later revisits the token span, so the next handoff has to preserve the scalar limit and account for any retained token backing. The compressed xref and retained graph readers additionally borrow raw stream spans. They still need bounded source and filter integration, and the parsed graph’s retained stream backing still needs bounded ownership and retirement.

- [Probe producer](📜️pdf-byte-probe-stage.ts)
- [Composed xref producer](📜️pdf-xref-byte-stage.ts)
- [Focused Rust law](📜️pdf-xref-byte-law.rs)
- [Cargo closure checks](📜️pdf-xref-byte-bounds.ts)
- [Fixture and MuPDF verification](segmented-pdf-source-fixture.md)

