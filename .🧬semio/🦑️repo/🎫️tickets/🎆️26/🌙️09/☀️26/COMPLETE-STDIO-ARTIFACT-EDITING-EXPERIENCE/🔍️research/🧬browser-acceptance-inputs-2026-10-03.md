# Browser Acceptance Inputs

The retained CSV input exercises quoted delimiters, Unicode, a multiline value, empty fields, and a trailing field. The hand-authored DOCX ZIP input exercises an empty paragraph with existing properties/comment, a styled Unicode run, a sibling run, populated and empty table cells, page metadata, foreign run properties, a custom XML part, and a ZIP archive comment.

Python standard-library ZipFile reopened the DOCX input, verified all four authored entries and ZIP CRC integrity. This is input validation only; application import, editing, undo/redo, saved-file fidelity, locale, and keyboard acceptance remain pending the current preview build.

Use browser-native file chooser upload to the local test app. Inspect all visible changes, command failures, console errors, and saved/reopened results. Do not treat fixture creation or a component build as browser success.


## Spreadsheet and Presentation Inputs

Added hand-authored `📊️workbook.xlsx` (8XMLparts) and `📽️presentation.pptx` (6XMLparts). Python standard-library ElementTree parsed every authored XML body; ZipFile reopened both archives, validated CRCs and exact entry bytes/order. XLSX covers sparse addressing, rich shared strings, Unicode, formula/cached value, boolean, blank styled cell, merge range and column width. PPTX covers a positioned textbox, independently styled Unicode runs, an empty paragraph with paragraph properties and slide dimensions. Both include unrelated custom XML and an archive comment. These are input-level checks, not application acceptance or Office schema conformance claims. Current browser execution is still pending a coherent component build.


## PNG Precision Witness

Added hand-authored `📷️precision.png`:2×1grayscale16 with sample words1and255, tRNS=1, bKGD=255, a text chunk and two IDAT chunks. Python struct/zlib independently checked every chunk CRC, decompressed the concatenated IDAT stream and recovered the exact five-byte scanline. The two samples have the same high byte and different low bytes, exposing RGBA8 precision collapse on save. This is a valid binary-input/deflate receipt, not a third-party PNG rendering or application round-trip receipt. Use it when completing the PNG source-authority cut.

## Raster Inputs Added

Copied three hand-authored canonical BMP witnesses for RGB24 padding/gap/trailer, duplicate1bit palette identities, and16bit565 masks. Pillow independently decoded all three with dimensions and RGBA output observed. Authored a classic little-endian two-page TIFF: first page2×2gray8 in two strips, DPI144×72 and unknown exact byte tag65000; second page2×1RGBA with alpha0and128. Pillow independently read both pages, exact gray/RGBA samples and unknown tag bytes. These are input-integrity checks; browser acceptance is still pending.

- ('direct-rgb24-padding-gap-trailer.bmp', (3, 2), [(255, 0, 0, 255), (0, 255, 0, 255), (0, 0, 255, 255), (255, 255, 0, 255), (255, 0, 255, 255), (255, 255, 255, 255)])
- ('indexed-rgb1-duplicate-palette.bmp', (9, 1), [(255, 0, 0, 255), (255, 0, 0, 255), (255, 0, 0, 255), (255, 0, 0, 255), (255, 0, 0, 255), (255, 0, 0, 255), (255, 0, 0, 255), (255, 0, 0, 255), (255, 0, 0, 255)])
- ('direct-bitfields16-565.bmp', (2, 1), [(255, 0, 0, 255), (0, 255, 0, 255)])
- ('📚️pages.tiff', '2 pages', 'gray8 strips + RGBA8 with alpha0/128')
