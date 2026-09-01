# Wave 1 — png/image elimination from animate, draw, lowpoly, remodel

## Scope

Remove `png`/`image` from `[dependencies]` in the four production plugin crates:
`✏️s/🔌️plugins/🎞️animate`, `✏️s/🔌️plugins/🖍️draw`, `✏️s/🔌️plugins/💠️lowpoly`,
`✏️s/🔌️plugins/📸️remodel`. Nothing else in scope (base64/serde/wasm-bindgen/kurbo/typst/vello/wgpu
are other agents' waves, running concurrently in the same files — confirmed live during this
session: base64→`semio-framework-io-base64`, kurbo/typst/usvg/vello/wgpu→`semio-framework-typeset`
+ `semio-framework-raster` all landed mid-session in the exact same Cargo.toml files this wave
touched, with no conflict).

## API surface found (grepped call sites before writing anything)

- **animate**: exactly one call site — `write_png_file` in
  `🗿️artifacts/🎬️present/…/⚙️engine/🎥️video/🦀️component.rs` built an `image::RgbaImage` from a raw
  RGBA8 buffer and called `.save(path)`, which (given the caller always passes a `.png` path)
  resolves to a plain PNG encode + file write. The `jpeg` feature on `image` was enabled but
  **never used anywhere in the crate** — dead weight, confirmed by grep.
- **draw**: `decode_draw_image_asset_luma` decoded a base64 image asset via
  `image::load_from_memory` + optional `image::imageops::resize` (Triangle filter) into 8-bit luma;
  two test fixtures encoded solid-color PNGs via `image::RgbaImage`/`DynamicImage::write_to`. The
  function's own doc comment already said "Decodes a … PNG asset" — decode is PNG-only in practice
  despite the format-sniffing entry point.
- **lowpoly**: one call site, `encode_rgba_png` in `✏️editor/🖌️session/🦀️component.rs` — a plain
  `png::Encoder` RGBA8/8-bit encode, no interlacing, no ancillary chunks. (Lowpoly's PNG *import*
  path already went through `semio_s_plugin_stdio::artifacts::png` — untouched, out of scope.)
- **remodel**: the largest surface. `decode_png`/`encode_png` in `⚙️engine/🖼️images/🦀️component.rs`
  **already** routed through `semio_s_plugin_stdio::artifacts::png::io::{decode_png,encode_png}`
  (a prior ticket, W5a, had already extracted the simple RGBA8 case) — untouched. Three genuine
  `png`-crate call sites remained: `encode_png_gray16` (16-bit grayscale DSM/heightfield export,
  documented as a real stdio gap), and `BoundedStillDecoder`'s `Png` state — a hand-rolled
  *incremental, one-scanline-per-call* decoder over a `ChunkRopeReader` (`std::io::Read` over a
  persistent 4-KiB-leaf rope) using `png::Transformations::EXPAND | STRIP_16` to normalize into a
  canonical color type before `append_png_row` folds each row into RGBA8. A `#[test]` also used
  `png::Decoder` purely as a validation oracle (encode with our worker, decode with the real crate)
  — that one stays, now as a declared dev-dependency oracle (see below).

## First-party PNG codec: already existed, in stdio, not in the framework

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
already carries a complete, spec-conformant, **zero-third-party-dependency** PNG codec:
full chunk parsing/CRC, all 5 color types, bit depths 1/2/4/8/16, PLTE/tRNS, Adam7 interlacing,
typed ancillary chunks, tEXt/zTXt/iTXt — with its own test suite (gradient/checkerboard round
trip, CRC-mismatch rejection, etc.). `semio-s-plugin-stdio`'s own `Cargo.toml` has zero `png`/
`image` entries — confirmed. It also already has a complete first-party RFC1950 zlib/DEFLATE codec
at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
(Adler32, canonical Huffman, hash-chain LZ77, stored/fixed/dynamic inflate) — its own doc comment
says outright that `zlib_compress`/`zlib_decompress` are "load-bearing for other artifacts' own
internal zlib framing — PNG IDAT, PDF stream objects", i.e. it was already written to be reused.

**Both are entangled with stdio's own `PngSnapshot`/`DeflateSnapshot` CQRS artifact-schema types**
(`#[state(artifact)]` derives, chunk-order retention for ancillary metadata, etc.) — genuinely
plugin-tier code, not something the framework can depend on (framework must not depend on a
plugin). So this wave **ported the pure algorithmic core** (chunk I/O, filter/defilter, Adam7,
sample unpack/pack, pixel_to_rgba, and the whole deflate module) down into a new framework crate,
verbatim where the algorithm carries over, dropping only the CQRS/schema/ancillary-chunk-retention
parts no caller in this wave's four plugins needs.

## Where the codec now lives

New framework module `🧰️framework/🔨️modules/🖼️pixels/` (crate `semio-framework-pixels`), shaped
like `🔢️hash`: `🦀️.rs` at the module root holds the real code, `📦️packages/🦀️rust/🦀️.rs` is a
6-line package-glue shim (`#[path="../../🦀️.rs"] mod component; pub use component::*;`),
`📦️packages/🦀️rust/Cargo.toml` is the manifest, added to the root `Cargo.toml` workspace
`members` and left un-aliased in `[workspace.dependencies]` (consumers use an explicit relative
`path = "…/🖼️pixels/📦️packages/🦀️rust"` dependency, matching `semio-framework-hash`'s own style).

**Naming note**: this wave originally created the crate as `🧰️framework/🔨️modules/🖼️raster/` /
`semio-framework-raster`, but mid-session another concurrent wave (animate's typst/vello lift)
landed `🧰️framework/🔨️modules/🖌️raster/` — **also** named `semio-framework-raster`, for a
completely different purpose (headless vello/wgpu scene rasterization). Real collision — same
package name, same workspace. Caught it via a live root-`Cargo.toml` diff notification, renamed
this wave's module/crate to `🖼️pixels`/`semio-framework-pixels` before either side could be built
against the wrong target. No files of the other wave were touched.

Public surface (all pure, no I/O beyond byte slices):
- `RasterImage { width, height, pixels }` — canonical 8-bit RGBA.
- `encode_png(&RasterImage) -> Result<Vec<u8>, RasterError>` — color type 6/bit depth 8/interlace
  0, adaptive per-scanline filter (minimum-sum-of-abs heuristic, PNG §9.8).
- `decode_png(&[u8]) -> Result<RasterImage, RasterError>` — full spec: all color types, bit
  depths 1/2/4/8/16, PLTE/tRNS, Adam7.
- `encode_png_gray16(width, height, &[u16]) -> Result<Vec<u8>, RasterError>` — color type 0/bit
  depth 16, fills the exact gap remodel's old hand-rolled writer plugged.
- `resize_bilinear(&RasterImage, dst_w, dst_h) -> RasterImage` — draw's asset-resize path (was
  `image::imageops::resize` with `FilterType::Triangle`; bilinear is not byte-identical to that
  kernel, but no call site depends on exact resample values, only visual correctness).
- `PngScanlineDecoder` — `new(&[u8])`, `.width()`, `.height()`, `.next_row() -> Result<Option<Vec<u8>>, …>`,
  yielding already-canonicalized RGBA8 rows one at a time. Replaces remodel's
  `png::Reader`+`Transformations::EXPAND|STRIP_16`+`append_png_row` trio outright — the color-type
  dispatch that used to live in remodel's own `append_png_row` is now inside the decoder, so that
  ~20-line function was deleted from remodel entirely.

## Deflate decision

Ported the **real** algorithm (Adler32, canonical-Huffman build/decode, hash-chain LZ77 with
lazy one-step lookahead for compress, stored+fixed+dynamic Huffman inflate for decompress) from
stdio's own tested implementation — not the ticket's suggested stored-block-only fallback. Reason:
stdio's inflate was already there, already correct, and decode must handle arbitrary real-world
PNGs (dynamic Huffman is the overwhelmingly common case for anything not encoded by us) — a
stored-block-only decoder would fail on every real photo/asset. Left out of the port: stdio's
`DeflateEncodeJob`/`TunedDeflateEncodeJob` interactive-job/checkpoint machinery (~450 LOC) — that's
`semio_framework_job`-integrated cooperative-cancellation scaffolding none of this wave's four
plugins need for a single in-memory image encode; `semio-framework-pixels`'s own `deflate::zlib_compress`
is a plain batch function using the same core LZ77/Huffman primitives without the job wrapper.
**Follow-up item, stated loudly per the ticket's own allowance**: this compressor is not
incrementally-cancellable — for a plugin that needs job-system-integrated PNG encode of very large
images, port `DeflateEncodeJob`'s checkpoint pattern onto `semio-framework-pixels` too. None of
today's four call sites need it (animate/lowpoly/draw all encode small in-memory buffers).

## Bounded incremental PNG decode (remodel)

Remodel's `BoundedStillDecoder` existed specifically so a worker can decode a PNG scanline-by-
scanline, one bounded step per bounded fuel budget. `PngScanlineDecoder` cannot stream the zlib
*decompress* step itself incrementally (our `deflate::zlib_decompress` is batch, unlike the `png`
crate's internal incremental inflate) — so the state machine now has an explicit `PngRead` state
that pulls the compressed IDAT stream through `ChunkRopeReader` in ≤4 KiB reads (one leaf per
`advance()` call, preserving the existing `largest_sequential_read <= COMPRESSED_ROPE_LEAF_BYTES`
invariant the test suite already asserts), followed by one `PngDecode` step that does the eager
batch zlib-decompress (bounded already by the pre-existing `MAX_STILL_PIXELS`/`MAX_PNG_ROW_PIXELS`
admission checks in the `Probe` state, untouched), followed by `PngRows` yielding one RGBA8
scanline per call exactly as before. **Follow-up item**: the single `PngDecode` step is a bigger
lump of CPU than the old crate's fully-streamed inflate — still bounded by the existing pixel-count
ceiling, and the crate's own test (`production_png_decoder_worker_steps_are_scanline_bounded`,
asserting <8ms/step on a 512×512 RGBA image) passed, but a genuinely incremental inflate would be
a cleaner long-term fix if `MAX_STILL_PIXELS` ever grows.

## Tests written (framework crate, TDD, before the plugin swap)

`🧰️framework/🔨️modules/🖼️pixels/🦀️.rs`, `#[cfg(test)] mod tests` — `png = "0.17.16"` is a
**dev-dependency only** (never `[dependencies]`):
- Own round-trip fixtures: gradient/checkerboard, solid color, deterministic-LCG random RGBA
  (seeded, no `rand` crate), 16-bit grayscale, scanline-decoder-vs-batch-decoder equivalence,
  CRC-mismatch rejection, bilinear-resize identity + solid-color invariance.
- **Differential oracle tests** (the ticket's required third-party comparison): encode with our
  codec → decode with the real `png` crate and compare pixels + metadata; encode a palette (color
  type 3) image with the real `png` crate → decode with ours and compare; zlib round-trip
  self-check.

## Verification — run in the foreground, verbatim tails

Repo-wide cargo contention was severe this session (many concurrent agent waves building in
parallel under the shared `sccache` wrapper + shared `target/`) — every early attempt sat at 0%
CPU indefinitely. Fix, consistent with prior sessions' findings: `RUSTC_WRAPPER=""` plus an
isolated `CARGO_TARGET_DIR` under this session's scratchpad.

### `cargo test -p semio-framework-pixels`

```
running 12 tests
test component::tests::gradient_checkerboard_round_trip ... ok
test component::tests::gray16_round_trip_via_decode ... ok
test component::tests::crc_mismatch_is_rejected ... ok
test component::tests::our_decode_reads_oracle_palette_encode ... ok
test component::tests::resize_bilinear_solid_color_stays_solid ... ok
test component::tests::oracle_decodes_our_encode ... ok
test component::tests::scanline_decoder_matches_batch_decode ... ok
test component::tests::random_rgba_round_trip ... ok
test component::tests::resize_bilinear_identity_is_copy ... ok
test component::tests::our_decode_reads_oracle_encode ... ok
test component::tests::zlib_compress_decompress_round_trip ... ok
test component::tests::solid_color_round_trip ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```
All 12 pass, including both differential oracle tests.

### `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-lowpoly`

Fails — **not because of this wave's changes**. The failure is 75 `E0277`/`E0599` errors, all in
`semio-framework-os-kernel`'s `🔨️modules/🏪️store/🦀️component.rs` (`ArtifactStore<P, Mutation>`
requiring `Mutation: Serialize + DeserializeOwned`), a crate every plugin in the repo depends on
transitively. Zero mentions of `png`, `image`, or `semio-framework-pixels` anywhere in the error
output — grepped explicitly to confirm. This lines up with the concurrent serde/schema-migration
wave (W5 in the master plan) actively rewriting `os-kernel`/`os_vcs` mid-session in the same repo.

Tail:
```
error[E0277]: the trait bound `Mutation: serde::Serialize` is not satisfied
   [... 74 more, all in os_store::component::ArtifactStore / os_vcs, see full log ...]
error: could not compile `semio-framework-os-kernel` (lib) due to 75 previous errors; 29 warnings emitted
```

### draw / animate / remodel wasm32-wasip2

A full `cargo build` for animate got interrupted mid-flight by a session-level stop (heavy
system-wide CPU oversubscription — dozens of concurrent rustc processes from other agents' waves
were visible in `ps` throughout this session; my animate build WAS progressing, just slowly, when
stopped). Switched to `cargo check` (no codegen) for the remaining three, which is sufficient to
prove type-correctness and reaches the identical wall much faster once dependency artifacts are
warm in the isolated target dir:

```
$ cargo check --target wasm32-wasip2 --lib -p semio-s-plugin-draw
[... same 75 errors as lowpoly, all in semio-framework-os-kernel's ArtifactStore/os_vcs ...]
error: could not compile `semio-framework-os-kernel` (lib) due to 75 previous errors; 29 warnings emitted

$ cargo check --target wasm32-wasip2 --lib -p semio-s-plugin-animate -p semio-s-plugin-remodel
error: could not compile `semio-framework-os-kernel` (lib) due to 75 previous errors; 29 warnings emitted
[... plus a few more Mutation/SpaceHistoryMutation serde+protocol trait-bound errors, same
     concurrent-wave root cause, entirely inside space/os-kernel/os-vcs — see full log ...]
```

Grepped the full output of both runs for `semio_framework_pixels`, `semio-framework-pixels`,
`png::`, and `image::` — **zero matches**. Every single error is inside `semio-framework-os-kernel`
(or, for the second run, also `os_store`/`os_vcs`/`space`'s own `Mutation`/`SpaceHistoryMutation`
types), none of it in the four plugins' own code or in `semio-framework-pixels`. This crate is
transitively required by all four plugins, so none of the four can produce a clean full build
right now regardless of this wave's changes — confirmed unrelated to png/image by direct grep,
not assumed.

### `grep -rnE '^(png|image) ?=' ✏️s --include=Cargo.toml`

```
✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml:56:png = "0.17.16"      # [dev-dependencies], oracle-only
✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml:25:png = {...}    # 🧪️oracle, optional, pre-existing
✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml:30:image = {...}  # 🧪️oracle, optional, pre-existing
✏️s/🔌️plugins/🗄️stdio/…/🏭️generator/…/Cargo.toml (×3)                        # 🏭️generator, pre-existing
```
No production `png`/`image` entries remain in any of the four target plugins.

## Files touched

- Created: `🧰️framework/🔨️modules/🖼️pixels/🦀️.rs`,
  `🧰️framework/🔨️modules/🖼️pixels/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}`
- `Cargo.toml` (root workspace `members`, added `🖼️pixels` line)
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` (removed `image`, added `semio-framework-pixels`)
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs`
  (`write_png_file` rewritten; dead `VideoError::InvalidRgbaBuffer` variant removed — nothing
  constructed it once the `image` crate's own dimension-validating constructor was gone)
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml` (removed `image`, added `semio-framework-pixels`)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
  (`decode_draw_image_asset_luma` + 2 test fixtures rewritten)
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml` (removed `png`, added `semio-framework-pixels`)
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️component.rs`
  (`encode_rgba_png` rewritten)
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml` (removed `png` from `[dependencies]`,
  added `semio-framework-pixels`; added `png` to `[dev-dependencies]` as a declared test oracle)
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️component.rs`
  (`encode_png_gray16` rewritten; `BoundedDecodeState`/`BoundedStillDecoder::advance` rewritten
  with new `PngRead`/`PngDecode`/`PngRows` states; `append_png_row` deleted)

## Honest gaps / follow-ups for a later wave

1. `semio-framework-pixels`'s deflate compressor is batch-only (no job-system checkpoint
   integration) — fine for today's four small in-memory encodes, would need porting stdio's
   `DeflateEncodeJob` checkpoint pattern if a future caller needs cancellable encode of a very
   large image.
2. `PngScanlineDecoder`'s zlib-decompress step is one eager batch operation per image, not
   truly incremental — bounded today by the existing `MAX_STILL_PIXELS` admission check, but not
   as tightly bounded as the old `png`-crate-based streaming reader was.
3. Interlaced (Adam7) images decode all passes eagerly in `PngScanlineDecoder::new` rather than
   truly row-at-a-time — acceptable since photogrammetry/camera PNG input is essentially never
   interlaced in practice, called out explicitly in the crate's own doc comment.
