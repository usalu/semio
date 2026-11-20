# Glyph Outline Clipping

The page geometry accumulates paths from glyphs shown with embedded horizontal TrueType outlines when PDF text rendering mode includes clipping. The outlines use the same font codec, byte widths, glyph outlines and text transform as page paint. The end of the text object intersects a single nonzero-winding path with the current graphics clipping path. Restoring graphics state restores the preceding path. Empty glyph outlines leave the current clip unchanged. If a text object mixes unsupported outlines, the preview does not apply a partial clip.

Type 3 glyph procedures still paint in mode 7 and remain invisible in mode 3. PDF.js independently matches that behavior. MuPDF 1.24.14 hides the Type 3 glyph procedure in mode 7. Normal Type 3 rendering continues to agree with both oracles.

MuPDF 1.24.14 verified two separated saturated red regions from the TrueType glyph clip, the white gap, and an unclipped blue rectangle after restoring graphics state. The authored native page regression checks that the red layer receives one merged nonzero-winding clip with both glyph outlines and that the later blue layer receives no text clip. The isolated native outline and viewport transform fed the actual production Canvas2d painter, which matched both glyph regions, the white gap and the later blue pixel. Rust 2021 parses the changed production modules. The registered page test is queued with the current manifest graph.

- [TrueType MuPDF oracle](📜️embedded-truetype-oracle.py)
- [Native outline projection](📜️glyph-clip-native.py)
- [Production Canvas2d probe](📜️glyph-clip-canvas.ts)
- [Type 3 PDF.js oracle](📜️type3-mode-oracle.ts)
- [ISO 32000-1, §9.3.6](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf)
- [PDF.js Type 3 renderer](https://github.com/mozilla/pdf.js/blob/master/src/display/canvas.js)
