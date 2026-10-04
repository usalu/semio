# TIFF Tiled 8-Bit Preview And Editing

Date: 2026-10-03

## Canonical design

The existing per-IFD `TiffStorage` remains the sole raster authority. Preview derives RGBA8 pixels from the selected IFD without persisting that projection. Tiled painting addresses one IFD and canonical snapshot revision, then patches only the raw sample spans inside the addressed uncompressed tiles. It leaves the IFD tag sequence, every unmodeled tag, storage partition, edge-tile padding, unrelated samples, other IFDs and all other canonical bytes unchanged.

The neutral fixture is a hand-authored 17×17 RGB page split into four 16×16 tiles. Its one-pixel right and bottom edge tiles carry full authored padding, so the laws distinguish visible samples from stored tile extent and prove that a region crossing both tile boundaries changes only four addressed pixels.

## Supported preview profiles

The checked projection accepts classic TIFF pages with orientation 1, chunky planar configuration, unsigned 8-bit samples, predictor 1, and authored strip or row-major tile storage. It supports:

- WhiteIsZero and BlackIsZero grayscale with one sample.
- RGB with three samples.
- RGB with exactly one unassociated alpha extra sample and four samples total.
- Uncompressed and PackBits chunks.

The projection validates dimensions, BitsPerSample cardinality, sample count, photometric interpretation, sample format, storage geometry, chunk cardinality and exact decoded chunk extent before allocating the RGBA8 derivative. Tiles are decoded at their full authored extent and clipped only while copying visible edge pixels. The editor and viewer continue to encode that derivative with the first-party PNG codec and render the localized mounted-unavailable state when a profile is refused.

## Supported editing profiles

`paint-region` is an atomic schema-owned mutation and an accessible English/German editor action. It accepts the same grayscale, RGB and unassociated-alpha sample profiles only when storage is tiled and uncompressed. RGB and grayscale refuse non-opaque paint; grayscale also requires equal RGB channels. The action validates page and region bounds, limits work to 65,536 rows, captures the canonical revision, uses the retained command route for prepare/row progress and cancellation, refuses stale snapshots, and publishes one exact-inverse history mutation.

The raw authoring operation exposes a controlled progress callback and returns no partially edited snapshot when cancelled. Its inverse is the exact prior snapshot. Text, binary, JSON Schema, TypeScript, GraphQL, Protobuf, EBNF, ANTLR, repository grammar, ABNF, Spicy and Kaitai facets use the same `paint-region` identity; binary tag 7 is followed by the existing set-snapshot and patch-snapshot tags 8 and 9.

## Independent evidence

Image-rs reopens the saved neutral tiled TIFF before and after editing and observes the exact expected samples. A separate PackBits tiled law compares the first-party projection with image-rs. WhiteIsZero inversion and unassociated alpha are checked with explicit sample witnesses. The language-neutral fixture and schema are consumed by the native laws.

## Explicit remaining frontier

The preview refuses packed and non-8-bit samples, palette color, signed or floating samples, planar storage, associated or unspecified alpha, predictors other than 1, orientations other than 1, and compressions other than uncompressed or PackBits. Editing additionally refuses strips and PackBits tiles. These profiles retain their canonical bytes and remain mounted with an explicit unavailable preview.

The public projection and command address any IFD ordinal, while the current main image window still displays IFD 0. Page selection, native-profile editing for the refused formats, and reversible profile conversion remain separate work. No precision or multi-page conversion is implied by this checkpoint.

## Validation

- `🗑️generated/tiff-tiled-core-green-6.log`: nine focused tiled projection, edit, cancellation, oracle, action and viewer laws passed.
- `🗑️generated/tiff-tiled-full-native-1.log`: genuine red full run; 93 passed before the schema input resolver found the missing `ifdIndex` localized label.
- `🗑️generated/tiff-tiled-full-native-2.log`: full registered component-enabled suite passed 111/111 with zero skipped.
- `🗑️generated/tiff-tiled-typescript-check-2.log`: all four TIFF TypeScript package suites checked after the final wire-facet alignment.

No browser receipt is claimed here. Root owns the full Stdio wasm/browser integration gate.
