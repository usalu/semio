# PNG Native Profile Paint Implementation

## Persisted authority

`PngSnapshot.bytes` remains the sole persisted authority. Native paint derives an ephemeral cursor over the checked IHDR, the authored IDAT addresses, decompressed pass rows, each row's original filter byte, and Adam7 geometry. It never stores an RGBA mirror.

The editor changes native sample bits in reconstructed pass rows:

- packed grayscale and indexed depths 1, 2, and 4 replace only the addressed high-order bit range and preserve neighbouring samples and unused tail bits;
- depth 8 replaces one authored byte;
- depth 16 replaces the exact big-endian word, including the low byte;
- Adam7 maps each global coordinate to its authored pass row before changing the sample;
- indexed paint writes a palette index, preserving duplicate-palette identity.

Rows are filtered again with their original filter kind. All rows are rebuilt because Up, Average, and Paeth filters depend on prior reconstructed rows. The resulting zlib stream replaces only the consecutive IDAT run. The original number of IDAT chunks is retained and the new stream is partitioned by the original payload lengths, with the final chunk owning the remainder. Every non-IDAT byte remains exact.

The existing RGBA8 `set-pixel-region` path now delegates to this exact sample editor after retaining its narrower 8-bit non-interlaced profile refusal. Its existing command and mutation wire remain stable.

## Public edit surface

The atomic `paint-native-samples` mutation carries a source revision, region, and one closed native profile with four bounded `u16` sample slots. Text and binary codecs, JSON Schema, TypeScript, GraphQL, Protobuf, neutral wire witness, aggregate roster, and exact inverse are paired.

Five retained editor actions expose profile-shaped argument forms:

- `paint-index-region`
- `paint-grayscale-region`
- `paint-grayscale-alpha-region`
- `paint-rgb-native-region`
- `paint-rgba-native-region`

Every coordinate and sample argument has English and German labels. Runtime validation applies the IHDR bit-depth maximum and PLTE entry count, rejects a mismatched action/profile, guards the exact source revision, validates nonempty in-bounds regions, and publishes only after cancellable retained preparation completes. The image window identifies the active native profile, precision, and Adam7 preservation in English and German.

## Supported profiles

The native editor admits the complete PNG 1.2 sample matrix already admitted by the checked decoder:

| Color type | Bit depths | Non-interlaced | Adam7 |
| --- | --- | --- | --- |
| Grayscale | 1, 2, 4, 8, 16 | yes | yes |
| RGB | 8, 16 | yes | yes |
| Indexed | 1, 2, 4, 8 | yes | yes |
| Grayscale + alpha | 8, 16 | yes | yes |
| RGBA | 8, 16 | yes | yes |

Palette contents, transparency chunks, metadata, private chunks, chunk order, authored filters, packing, precision, and interlace remain unchanged. Only the IDAT run changes: payload bytes, payload lengths where needed, and their CRCs.

## Neutral and independent laws

The neutral fixture covers duplicate palette identity, packed grayscale, 16-bit low bytes, Adam7, and the full profile matrix. Rust laws generate every profile/depth in both interlace modes, paint an addressed sample, and require:

- exact native sample values after reopen;
- exact non-IDAT chunks;
- retained IDAT chunk count;
- exact packed tail bits;
- stale revision, bounds, profile, palette cardinality, and cancellation refusal;
- exact undo back to the original source;
- text and binary mutation round-trip.

The independent `png` 0.18 decoder runs with identity transformations and must accept every edited file while reporting the original color type and bit depth. Targeted laws assert exact indexed packed bytes and 16-bit RGB, grayscale-alpha, and Adam7 RGBA words.

## Validation state

The first private Nx check reached Cargo after generation but waited about eleven minutes on the shared package-cache/native-owner lock with zero compiler CPU and no PNG diagnostic. It was interrupted cleanly to release the native-owner lane for other queued artifact gates. Its blocked receipt is `🗑️generated/png-native-profile-check-1.log`; it is not a passing result. A fresh full native gate remains required after the coordinated Cargo queue advances.

## Static and non-native checkpoint

The source audit before Cargo found two real test compilation defects and repaired them: the committed Adam7 RGBA fixture constant no longer shadows the seven-pass geometry constant, and all newly authored `include_bytes!`/`include_str!` paths resolve to their intended PNG fixture owners. Thirteen relevant include paths resolve, all 149 PNG JSON documents parse, and binary mutation tag 20 is unique in the PNG mutation protocol.

The registered TypeScript target passed its two pngjs region comparisons. Receipt: `🗑️generated/png-native-profile-typescript-1.log`.

The earlier `png-native-profile-check-1.log` remained blocked on the shared native owner with no compiler activity and was interrupted cleanly; it is not a pass. The fresh Rust profile/oracle matrix is still pending the coordinated shared build-directory release.

The TypeScript mutation-input resolver also audited the complete nested mutation schema with zero findings, including English/German labels for revision, region geometry, sample profile, enum options and all four native sample slots. Receipt: `🗑️generated/png-native-profile-input-schema-audit-1.log`.

## Read-only ownership, cancellation, and chunk-order audit

The native sample/profile surface is complete, but its expensive physical pipeline does not yet satisfy the repository's cumulative native ownership contract. `MAXIMUM_RAW_BYTES` bounds only the command wire. `png_layout` still calls the legacy RGBA projection, whose IDAT concatenation, zlib expansion, scanline owners, unpacked sample vectors, and `width * height * 4` derivative are not charged to one caller-owned allocation budget. Native paint then independently owns concatenated IDAT bytes, a second zlib expansion, pass-row vectors, rebuilt scanlines, compressed output, replacement chunks, and the final snapshot. No domain ceiling currently connects IHDR dimensions or the declared decompressed scanline extent to those cumulative live owners.

The existing Deflate artifact already exposes the correct lower-level seams: `decompress_zlib(bytes, maximum_output, NativeDecodeControl)` performs a measured/materialized two-pass inflate with interior cancellation and cumulative admission, while `compress_zlib(bytes, maximum_file, NativeEncodeControl)` checkpoints controlled compression, output emission, and Adler calculation. PNG native paint still calls the older `zlib_decompress` and `zlib_compress` functions. Its callback begins only after the entire inflate and pass-row construction have completed, and its final callback occurs before rebuilding/refiltering/compressing the full image. The retained command publishes cancellable preparation ticks, but the mutation later executes the codec synchronously with an always-continue callback. Cancellation therefore prevents publication during preparation and preserves the base snapshot, but cannot interrupt the actual inflate or deflate work. This is an explicit unsupported execution frontier, not a claimed complete progress/cancellation implementation.

Chunk preservation is stronger than the present focused law states. Checked structure requires one consecutive IDAT run. `replace_idat_run` splices exactly from the first IDAT start through the last IDAT end, so the entire byte prefix and suffix, including private `prIV`, text, palette, transparency, metadata, and their relative order, remain source exact. It retains the IDAT chunk count and reuses each authored payload length until compressed output is exhausted, with the final IDAT owning the remainder. The current multi-IDAT/private fixture has the exact order `IHDR, prIV, tEXt, IDAT, IDAT, IEND`; current laws compare all non-IDAT tags and payloads in order and the IDAT count, but do not yet assert the complete tag sequence or exact prefix/suffix byte spans. That missing explicit law remains part of the native validation follow-up.

Closing this frontier requires one PNG-owned control that derives the exact packed scanline extent with checked arithmetic, admits every simultaneously live owner, bridges the retained job's cancellation token through controlled Deflate and row materialization, and emits the mutation only after the controlled output exists. It must use the existing Deflate controlled APIs rather than add another compressor or duplicate their accounting. This control work is intentionally not mounted while shared native/SDK extraction is compiling; the current full native profile/oracle gate remains the immediate validation checkpoint.
