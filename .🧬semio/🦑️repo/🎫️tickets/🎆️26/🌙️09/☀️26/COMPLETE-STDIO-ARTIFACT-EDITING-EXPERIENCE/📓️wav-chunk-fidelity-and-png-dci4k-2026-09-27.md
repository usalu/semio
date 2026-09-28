# WAV Chunk Fidelity and PNG DCI 4K Boundary

Date: 2026-09-27

## Implemented

### WAV RIFF/WAVE fidelity

- Replaced the lossy fixed `fmt ` / `data` / auxiliary rewrite model with a complete top-level sequence:
  - `WavChunkRef::Format` references the editable typed primary `fmt ` chunk.
  - `WavChunkRef::Samples` references the editable typed primary `data` chunk.
  - `WavChunkRef::Other(index)` references an auxiliary chunk or a later duplicate `fmt ` / `data` chunk retained verbatim.
- Added `fmt_pad_byte`, `data_pad_byte`, and per-`RiffChunk` `pad_byte` state. Odd RIFF word-alignment bytes now survive decode, semantic editing, diffing, and encode instead of being normalized to zero.
- The decoder selects the first on-disk `fmt ` and `data` chunks as typed primaries and retains every later canonical duplicate as raw chunk data. It records the exact admitted order and rejects an odd chunk that omits its required pad byte.
- The encoder emits each represented chunk once in recorded order and appends unreferenced payloads rather than discarding them. If a hand-edited order places or leaves a raw duplicate before its typed primary, the encoder moves the typed primary immediately before that duplicate. This keeps edited format/sample values authoritative when the result is reopened.
- `WavDiff`, its text/binary codec, aggregate mutations, inverse behavior, artifact state, and editor detail mutations now include chunk order and pad bytes. `SetOtherChunks` reconciles auxiliary references, and its inverse restores the complete prior snapshot.
- The details provider exposes chunk order and alignment bytes through the existing generic snapshot details surface while retaining the compact sample-lane provider for large sample arrays.
- JSON Schema, TypeScript, GraphQL, and protobuf facets now describe the typed WAV snapshot/artifact/diff/mutation state instead of generic key/value placeholders. Existing WAV fixtures now declare their full canonical chunk order.

The language-neutral fixture `🧫️fixtures/🧭️preserve-chunk-sequence/🔣️.json` covers an odd `JUNK` chunk with a nonzero pad byte, typed primaries, duplicate `fmt ` and `data` chunks, and interleaved `LIST` metadata. The native codec test uses the independent test-only `riff` crate to verify child IDs and payloads. It also verifies that primary format/sample edits are visible to that oracle while duplicate payloads stay byte-exact, and that a misordered duplicate cannot mask edited primary values.

### PNG DCI 4K boundary

- Corrected the RGBA8 interactive raster byte ceiling from 32 MiB to `4096 × 2160 × 4 = 35,389,440` bytes.
- The refusal message states that this is a byte ceiling equal to a 4096×2160 RGBA8 raster.
- The native boundary test constructs the exact DCI 4K raster size, applies a one-pixel bottom-right patch, asserts the mutation remains below the one-item store limit, and checks that all untouched pixels remain unchanged.

This is a bounded small-region capability at the DCI 4K byte boundary. It does not establish efficient full-frame painting or broad larger-raster support. A full DCI 4K fill can exceed the retained work capacity, and applying a compact patch still clones the snapshot pixel vector in the current generic store/diff path. Larger common rasters need tiled or structurally shared storage and a scaling patch path before they can be claimed as complete.

## Validation

- `bun nx test @semio-tech/stdio-wav --skip-nx-cache`: passed, 2 tests, 11 expectations.
- `bun nx test @semio-tech/stdio-png --skip-nx-cache`: passed, 2 tests, 6 expectations.
- Parsed every JSON file in the WAV artifact subtree with Bun: passed.
- `git diff --check` over the WAV and PNG artifact subtrees: passed.

The Nx targets above execute the TypeScript facet suites. Native WAV/PNG tests were authored but were not run in this slice because shared full-component and native Cargo sessions were already active. No native pass is claimed here.
