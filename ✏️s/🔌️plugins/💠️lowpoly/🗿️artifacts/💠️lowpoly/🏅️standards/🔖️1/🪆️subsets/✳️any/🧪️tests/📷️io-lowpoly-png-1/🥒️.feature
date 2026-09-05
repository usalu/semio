@capability-lowpoly-1-io
@oracle-lowpoly-io-png-pillow
@oracle-input-subject-raw
@comparison-ordered-json-v1
Feature: Decode the lowpoly PNG export with Pillow
  This case splits the PNG row from `🔀️io-lowpoly-1`: the other eight declared `stdio.*` formats retain
  that case's native round-trip law and no-oracle decision, while this one gives `png` a genuine
  third-party decoder. The Rust subject exports the committed `LowpolySnapshot` fixture through
  `serialize_bytes()`, checks its own import round trip, then gives those exact produced bytes to the
  Python oracle. The oracle calls `PIL.Image.open` on that subject artifact; it never builds a PNG or
  re-derives the lowpoly DSL text.

  Pillow independently validates PNG decoding, dimensions, the RGBA pixel format and byte length, and
  the existence of the `semio-lowpoly-dsl` PNG `tEXt` entry. The fixture's eight-byte paint layer does
  not meet `LOWPOLY_PAINT_TEXTURE_SIZE² × 4`, so this exercises the exporter’s documented `1×1`
  opaque-white raster fallback while the full lossless document remains in the metadata chunk.

  @id-roundtrip-png
  @level-long
  @mode-differential
  Scenario: Export the committed lowpoly document as PNG and independently decode its bytes with Pillow
    Given the committed lowpoly document
      """
      {
        "format": "png",
        "document": "local://💠️lowpoly-snapshot.json"
      }
      """
    When the Rust subject exports it through `serialize_bytes` and Pillow opens those exact PNG bytes
    Then both report the same width, height, RGBA pixels and lowpoly metadata keyword
