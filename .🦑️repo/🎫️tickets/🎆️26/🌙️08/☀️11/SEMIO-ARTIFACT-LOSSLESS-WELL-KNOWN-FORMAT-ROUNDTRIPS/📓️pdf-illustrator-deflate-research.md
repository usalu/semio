# PDF Illustrator DEFLATE Producer Analysis

## Fixture Stream

The first unresolved filtered stream in `temp/📄️bachelor-thesis.pdf` belongs to a Form XObject
whose logical dictionary contains `/PieceInfo << /Illustrator ... >>`. It is 3,362 compressed bytes,
decodes to 16,316 bytes, starts with RFC 1950 header `48 89`, and ends with Adler-32 `4eb38157`.
The header declares a 4 KiB window.

## Backend Matrix

- `miniz_oxide` levels 0 through 10 emitted 16,327, 4,236, 3,648, 3,434, 3,256, 3,237,
  3,225, 3,223, 3,222, 3,222, and 3,222 bytes. Every result differs at byte zero because its
  deterministic wrapper selects `08` or `78` rather than `48`.
- `zlib-rs` was exhaustively checked across window bits 9–15, levels 0–9, memory levels 1–9,
  and Default, Filtered, Huffman-only, RLE, and Fixed strategies. Its best prefix came from
  window 12, level 6, memory level 5, Default: 3,269 bytes with header `48 89`, differing at byte 3.
- Zopfli at 1, 5, and 15 iterations emitted 3,015, 2,976, and 2,974 bytes. All use `78 da` and
  differ at byte zero.
- System libz 1.2.12 was exhaustively checked over the same window/level/memory/strategy matrix.
  Window 12, level 6, memory level 5, Default emits 3,360 bytes and matches the fixture through
  byte 2,691. Its decoded value and Adler-32 are identical.

The detailed results are retained in `🧪️pdf-deflate-backends.log`.

## Block Analysis

RFC 1951 `Z_BLOCK` tracing proves that the fixture and system libz candidate emit identical data
blocks: block one ends at compressed bit 21,538 after 14,152 decoded bytes, and block two ends at
bit 26,841 after all 16,316 decoded bytes. The ordinary system candidate marks block two final.
The Illustrator producer leaves it non-final and emits a ten-bit empty non-final fixed block plus
a ten-bit empty final fixed block before the Adler-32 trailer.

Calling system libz with window 12, level 6, memory level 5, Default strategy, `Z_PARTIAL_FLUSH`,
then `Z_FINISH` reproduces every one of the fixture's 3,362 bytes. `Z_SYNC_FLUSH` and
`Z_FULL_FLUSH` both produce 3,366 bytes and diverge at byte 3,355. The block trace is retained in
`🧪️pdf-deflate-blocks.log`.

The same algorithm was then applied independently to the first thirty Illustrator private-data
streams declared as `/Filter [/FlateDecode]`, through fixture offset 1.1 MB. Every decoded logical
value regenerated its complete compressed stream exactly, including lengths from 3,563 to 63,559
bytes. This validates an Illustrator object-family policy rather than a one-stream special case.

## Implementation Decision

The shared RFC 1950 boundary now owns an internal libz-sys implementation of the exact Adobe
algorithm. At serialization, PDF derives the Illustrator object family by traversing logical COS
references from each typed Illustrator PieceInfo root; this transient set is never persisted. The
snapshot retains only decoded stream bytes, typed filters, and the logical PDF dictionary; it
stores no compressed bytes, source bytes, layout tokens, producer flags, or per-stream encoding
state.
