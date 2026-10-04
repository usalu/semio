# PNG Native Profile Paint Plan

Date: 2026-10-03

## Current-source finding

The persisted PNG authority is already correct: `PngSnapshot` owns only the schema identifier and exact source bytes. IHDR/profile metadata, chunks, native samples and RGBA8 preview are derived. No-op save is byte exact.

The remaining editing gap is narrower. `paint_rgba8_region_controlled` admits only non-interlaced 8-bit RGBA and edits the RGBA8 projection before `author_png_projection` writes a new color-type-6, depth-8, non-interlaced IDAT stream. That restriction protects indexed identity, packed samples, 16-bit low bytes and Adam7, but leaves those profiles without natural sample editing.

The replacement must edit native samples directly. It must not add a persisted decoded plane or route indexed/16-bit/Adam7 sources through RGBA8.

## Native edit model

Keep exact source bytes as the sole persisted authority. Add an ephemeral `PngSampleRaster` cursor that owns:

- the checked IHDR profile;
- the original consecutive IDAT addresses and payload lengths;
- decompressed pass rows;
- each row's original filter byte;
- Adam7 pass geometry;
- the original packed-row tail bits.

The cursor defilters rows without widening them into a second full-image RGBA or `u32` grid. Addressed writes update only native sample bit ranges:

- depth 1/2/4 writes preserve every neighboring packed sample and unused tail bit;
- depth 8 writes one byte per native channel;
- depth 16 writes the exact big-endian sample word, including its low byte;
- Adam7 maps each addressed image coordinate to its owning pass and row;
- palette paint writes an index, retaining duplicate-palette identity.

After edits, refilter each affected pass with the original filter type for every row from the first changed row onward. This is required because Up/Average/Paeth rows depend on the preceding reconstructed row. Preserve unchanged pass rows and their filter choices. Recompress through the repository's retained/cancellable zlib encoder. Replace only the consecutive IDAT run, preserving the IHDR and every PLTE, tRNS, bKGD, text, private and unknown chunk byte-for-byte and in order.

Keep the original number of IDAT chunks. Deterministically partition the new zlib stream using each original payload length in order, with the final chunk owning the remainder; when the stream shrinks, later chunks may be empty, which PNG permits. CRCs change only for authored IDAT chunks. This retains the source's IDAT topology while acknowledging that an image edit necessarily changes compressed payload bytes.

## Schema-first actions

Preserve the seven existing 8-bit RGBA region laws and the existing `set-pixel-region` command as the specialized color-type-6/depth-8 path. Route it through the same native row engine after parity is proven.

Add profile-specific action facets instead of one ambiguous RGBA conversion:

- `paint-index-region`: geometry plus `paletteIndex`;
- `paint-grayscale-region`: geometry plus `gray`;
- `paint-grayscale-alpha-region`: geometry plus `gray` and `alpha`;
- `paint-rgb-region`: geometry plus `red`, `green`, `blue`;
- `paint-rgba-native-region`: geometry plus `red`, `green`, `blue`, `alpha`.

Native sample fields are unsigned 16-bit values. Runtime profile checks enforce the current depth maximum, palette cardinality and exact color-type/action pairing. RGB and grayscale actions retain existing tRNS color-key semantics; they cannot invent partial alpha. Indexed paint selects an authored palette entry and never infers an index from RGBA, because duplicate entries make that mapping non-unique.

The main image window uses the existing additive accessory seam to render only the action matching the checked source profile. Controls carry English/German native labels, native accessibility names, exact integer bounds and a status that identifies color type, depth and Adam7. A refused/malformed profile keeps Details/source access and the mounted unavailable preview.

## Retained execution

Use one registered factory for the native profile paint family and one Artifact publication contract per action. The retained work stages are:

1. validate source revision, profile, geometry and bounded raw size;
2. collect and stream-inflate the IDAT run;
3. defilter one pass row per bounded step;
4. apply addressed native sample writes by pass row;
5. refilter affected pass rows;
6. stream-compress through the current retained deflate engine;
7. partition and validate the authored IDAT run;
8. publish the compact revision-guarded semantic mutation.

Every owned buffer has explicit close/retirement steps. Cancellation before publication discards the cursor and leaves the source unchanged. The maximum admitted decompressed raster should be 128 MiB, enough for DCI 4K 16-bit RGBA without a dense widened sample plane. Extent derives from pass-row work plus the deflate engine's exact progress, rather than counting unchanged pixels.

The semantic mutation remains small: source revision, region and native sample value. Its diff uses the same deterministic native row engine, and its inverse is the exact prior snapshot. Before mounting, confirm that Store application does not hide a second uninterruptible full encode; if it does, publish the retained work's checked authored-IDAT receipt in bounded pages and make mutation application a cheap verified splice.

## Neutral red/green corpus

Add language-neutral fixtures and schemas for:

- indexed 2-bit with duplicate palette entries, painting the second duplicate index;
- grayscale 1-bit spanning a packed-byte boundary with nonzero unused tail bits;
- RGB16 and RGBA16 with distinct low bytes;
- grayscale-alpha 16-bit;
- Adam7 indexed and Adam7 RGBA16 regions crossing multiple passes;
- invalid sample maxima, invalid palette index, profile/action mismatch, stale revision and cancellation.

For every accepted case require:

- unchanged IHDR profile and interlace method;
- exact unaddressed native samples;
- exact packed tail bits;
- exact non-IDAT chunk bytes and order;
- retained IDAT chunk count;
- exact inverse back to the original source bytes;
- save/reopen sample agreement.

The independent `png` crate dev oracle must decode with identity transformations. It can verify palette indices and big-endian 16-bit samples before and after save without an RGBA8 conversion. pngjs remains the independent rendered-pixel oracle for 8-bit cases and the existing TypeScript twin. The Adam7 laws compare source profile/interlace metadata and addressed samples through both implementations.

## Profile coverage after this cut

The common native sample edit matrix will be:

| Color type | Depths | Non-interlaced | Adam7 | Paint value |
| --- | --- | --- | --- | --- |
| Grayscale | 1, 2, 4, 8, 16 | yes | yes | native gray |
| RGB | 8, 16 | yes | yes | native RGB |
| Indexed | 1, 2, 4, 8 | yes | yes | palette index |
| Grayscale + alpha | 8, 16 | yes | yes | native gray + alpha |
| RGBA | 8, 16 | yes | yes | native RGBA |

Palette entry editing, tRNS/bKGD editing, explicit profile conversion, animation and nonstandard critical chunks remain separate addressed commands. They must not be folded into paint or inferred from preview colors.

## Mount order

1. Commit neutral fixture schemas and red first-party/independent-oracle laws.
2. Implement the ephemeral exact-sample cursor and checked IDAT replacement.
3. Add semantic mutation facets and codecs for the five profile families.
4. Mount retained factories, progress/cancellation and exact retirement.
5. Add profile-specific accessible accessory controls.
6. Run the seven existing region laws unchanged, the new profile laws, independent `png`/pngjs oracles, SQLite/source authority laws, package TypeScript checks and the full registered native target with `component-app-assembly`.

No PNG production source was changed during this design pass.
