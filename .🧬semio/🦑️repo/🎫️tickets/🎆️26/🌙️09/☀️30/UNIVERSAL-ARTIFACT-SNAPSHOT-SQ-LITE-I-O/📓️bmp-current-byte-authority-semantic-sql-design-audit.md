# BMP Current Byte Authority Semantic SQL Design

Read-only design against current authored owner, no implementation/test execution. Preserve BmpSnapshot {schema,bytes}; relational decoding must reconstruct those exact bytes, not resurrect the former decoded pixels Snapshot API.

Primary authority is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs`. profile195–206 admits BI_RGB1/4/8/16/24/32 and BI_BITFIELDS16/32 only. layout235–339 requires BM signature, exactly40-byte DIB, nonnegative signed width, representable height, both dimensions zero together, planes1, valid checked strides/ranges. Masks210–224 require three nonzero contiguous nonoverlapping RGB masks in sample width; no fourth alpha mask is admitted. Other DIB12/52/56/108/124 and compressed profiles remain refused in this standard.

Layout reads every file-header field and all40 DIB bytes: declared file size, both reserved words, pixel offset, signed width/height, planes, bits per pixel, compression, image size, signed x/y resolution, colors used/important. Declared file size may be0 or between checked pixel end and source length. Image size may be0 or greater than actual row storage. Preserve these exact declarations; do not silently normalize to derived lengths. Indexed colorsUsed0 means full2^bpp palette; otherwise declared count<=capacity. Palette entries own all four BGRA octets, including duplicate entries and reserved byte. Direct profiles do not interpret colorsUsed as palette authority. Metadata ends54 or66 for bitfields plus indexed palette. Gap to pixel offset and all bytes after checked pixel end are accepted arbitrary native octets. These require separate exact ordinal ownership, including bytes beyond declared file size or image-size declarations.

## Handwritten Relational Plan

| Table | Full semantic ownership / reconstruction |
|---|---|
| bmp_document | singleton schema identity only; no native file BLOB |
| bmp_file_header | singleton declared_file_size u32, reserved1/2 u16, pixel_offset u32; fixed BM signature derived from selected standard |
| bmp_info_header | singleton header_size=40, width i32 nonnegative, signed_height i32 excluding MIN, planes1, bpp, compression, declared_image_size u32, signed x/y resolution, colors_used/important u32 |
| bmp_channel_mask | document FK + ordinal0..2/channel closed RGB + mask u32, only BI_BITFIELDS; implicit BI_RGB masks remain derived, not additional editable duplicate authority |
| bmp_palette_entry | document FK + contiguous palette index; blue/green/red/reserved u8; exact occurrence identity, duplicate colors allowed |
| bmp_pixel_index | document FK + logical y/x unique + raw index fitting bpp, indexed profiles only; palette relationship may be unresolved as described below |
| bmp_pixel_sample | document FK + logical y/x unique; direct raw red/green/blue integer channel samples at original mask precision, and unused_bits word constrained disjoint from masks; RGB24 exact three8-bit channels; RGB16 high unused bit and RGB32 unused high octet preserved |
| bmp_row_tail_bits | document FK + logical y; original unused low bits in final indexed partial byte, bit count derived from width/bpp; avoids hiding addressable pixels in raw row bytes |
| bmp_row_padding_octet | document FK + logical y + contiguous padding ordinal + u8; stride-derived exact cardinality |
| bmp_gap_octet | document FK + contiguous ordinal + u8 for metadata_end..data_offset |
| bmp_trailer_octet | document FK + contiguous ordinal + u8 for pixel_end..source length |

Logical y indexes top-down visual rows; signed height determines physical row ordering at reconstruction. Reassemble each packed sample/index at its actual bit width; OR only separately constrained unused sample bits/tail bits, then row padding. This permits exact independent SQL resolution/header/palette/index/channel edits while preserving every unaffected physical octet. No RGBA8 normalization:16-bit sample conversion to8-bit can lose source precision. No whole-row or whole-file BLOB competing authority. uint32 fits SQLite signed64 exactly; guard integer storage class and range. Every relation needs unique identity, FK, ordinal completeness, variant exclusivity, checked arithmetic and cross-row cardinality validation before paid construction; enforce supported layout with existing bmp_layout_bytes after reconstruction.

A palette index beyond actual palette length is currently accepted by layout/decode but rejected by rgba preview at461–463. Therefore hard FK from every pixel index to palette would narrow actual Snapshot acceptance. Preserve such index as typed unresolved relationship with range<=bpp capacity; preview retains existing rejection. Do not invent default color. Default/DSL/intermediate bytes may be malformed even though ordinary native Pack validates layout: current plain struct allows arbitrary Vec bytes. Exact relational projection of these states needs explicit domain decision. Refuse malformed input with existing typed error and document narrower valid-native capability, or author separate meaningful byte-span grammar for malformed states; a generic hidden payload fallback would violate this task. Do not claim complete owner-domain coverage until resolved.

## Independent Test Authorities

Current canonical corpus is `🧫️fixtures/🧬️canonical-byte-authority/🔣️.json` plus its closed schema and named BMP files. It includes duplicate indexed palettes,16-bit555/565,32-bit bitfields, row padding/gap/trailer and explicit wrong-DIB refusals. Actual IO unit tests19 onward assert exact native bytes/layout and byte-preserving edits. Existing independent third-party image BMP oracle is `🔮️oracles/🦀️.rs`, with bmp decoder palette/raw-pixel API and image encoder. Its old docstrings describe retired Snapshot pixels/palette and cannot serve as current public owner assertions unchanged; use image/bmp solely behind test interface to compare decoded visual samples where supported. Its encoder normalizes resolution/row-order/palette details, so not an exact-byte oracle. Independent BunSQLite must assert all authored relational cells/ordinals, mutate selected semantic fields, then compare exact independently authored changed bytes and third-party visual meaning. Include header declarations0/noncanonical lengths, padding/tail unused bits, orphan palette indexes and malformed refusal laws. Preserve existing exact byte corpus before adding cases.

No runtime GREEN, SQL implementation or new command registration is claimed.
