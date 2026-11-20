# Inline PDF Image Framing

The content parser now checks positive integer width and height against the typed image range before decoding. Unfiltered images share the packed-image row calculation with sample packing and unpacking. The calculation checks byte length, per-row alignment, and addressable sample storage. The parser consumes a CRLF pair or one other PDF whitespace byte after `ID`, preserving whitespace that belongs to the sample payload.

For a known packed grid, the image closes only at the declared byte length followed by whitespace and a complete `EI` operator. This preserves a delimited `EI` inside a sample row. A malformed boundary returns a typed error without trying an earlier marker or treating following page operators as sample data.

## Verification

- The language-agnostic `🧪️inline-images.json` fixture covers valid CRLF with embedded `EI`, a leading whitespace sample, byte-padded 1-bit rows, and ASCIIHex, plus truncated data, an oversized grid, out-of-range width, and missing or malformed operator separators.
- Before the change, the isolated production parser failed on the CRLF and embedded `EI` case. After the change, it passed the initial nine fixture cases. The same test printed each accepted image and reparsed the content to verify the typed operators and samples round-trip.
- The expanded fixture passed an isolated compile of the production framing and geometry helpers. The updated full-parser re-run requires the repo backing libraries that were removed from the build cache during verification.
- `📜️inline-images-oracle.py` placed the four valid cases inside actual PDF 1.7 pages with xref tables. MuPDF matched all 14 rendered sample pixels, including the four bytes around the embedded marker. The harness `📜️inline-images-native.py` runs the production parser and image helpers against the repo-built dependency artifacts.

For filtered images with unknown encoded size, the parser still uses whitespace-delimited `EI` scanning. A codec-aware end marker is needed to remove ambiguity when filtered binary bytes contain that delimiter sequence.

The encoded ASCII85 terminator now owns the inline-image boundary. This prevents a whitespace-delimited `EI` inside valid encoded data from truncating the image. The original regression failed at the shorter body length. The reference reader now drains a resumable frame cursor and a sample cursor whose output and physical close respect grants. The combined production-source harness passed 86 tests. MuPDF rendered all five image fixtures and matched their 18 sample pixels. Python’s independent ASCII85 decoder also produced [0, 4, 5, 76] for the new row.
