# Controlled Import of a Simple Embedded OpenType Font

The staged changes admit a shared indirect OpenType CFF program on ordinary larger PDFs. The PDF uses a simple Type1 font dictionary with WinAnsiEncoding, an independent descriptor and FontFile3 /Subtype /OpenType, and a direct ToUnicode CMap in its own indirect stream. The graph retains the native font data original to the PDF and records independently typed FontProgram and UnicodeMap provenance. Tj and TJ hold their original character codes. The admitted font uses the existing FontCodec, resource resolver, and glyph outline path. Raw streams and a single predictor-free Flate filter are supported.

The cursor admits one embedded font shared across pages, with up to eight font slots across the document. Font descriptors and width tables stay within the qualified shape and existing widths bounds. The program cursor hashes one borrowed decoded byte per work unit using the existing first-party SHA-256 state. Its identity covers the font dialect, program bytes and exact typed filter provenance. Direct ToUnicode accepts a one-byte codespace with bfchar and simple bfrange. Unsupported font forms, modifiers, dependent CMaps and resource layouts go through the existing reference path. Controlled CID and TrueType sfnt font admission require separate qualification.

The 48-page raw and Flate specimens contain 51,156 and 50,544 bytes. Actual native byte graph preparation used 154,530 and 154,889 one-unit work turns. Controlled semantic lift used 353,853 units for each fixture. It recorded 48 operator roles and both resource roles while leaving the original program and map in the source graph. The focused probe enforced the existing 32,768-byte per-turn transfer cap and the existing empty reservation/retirement allowances. Cancellation after 128 program reads retired all unpublished font backing. The same composed probe passed the existing DCT, sampled/Flate and Standard14 cases.

PyMuPDF 1.26.7 independently extracted the original font and verified AgRé twice on each page. The source-only integration mounted the actual PDF graph cursors, font and content codecs and native writer. Its raw export reopened in MuPDF with identical OpenType bytes, Unicode text and all 48 page raster hashes. The neutral fixture adds a Rust product test for the actual font owner cursor with native resource admission, exact typed identity, physical code round trip and visible outlines. The source-only execution passed the same assertions using a test resource adapter. The governed product test is pending. Its test asset is reproduced from the same independent PDF specimen.

## Release and verification

The digest-checked, 15-path roster produced by `controlled-font-release.py` contains the DCT, physical-code, ToUnicode and embedded-font changes together. The composed foreign-artifact source includes both owners' additions. The helper supports `--prepare`, `--verify` and `--apply-reviewed` and checks every live preimage before writing. Apply only after parent release of the live source epoch. There are no Cargo manifest or lock changes.

Run the ticket scripts from the repository using the repository tool wrapper. The stage input is the PDF 1.7 base path.

- `controlled-opentype-oracle.py` authors the neutral specimen and code, font and raster oracle with PyMuPDF. Its `verify <download>` mode checks the original program and all page receipts.
- `controlled-opentype-test.py <base>` executes the native source-only graph and semantic probe. The raw writer receipt is `🗑️generated/font-opentype/emitted.pdf`.
- `controlled-font-closures.ts <base> <ticket-generated>` invokes the repository source-closure measurement against the exact composed manifest.
- `controlled-opentype-browser.ts <mounted-url> <ticket-font-opentype-output>` opens the raw PDF by the app action, checks the accessible text and glyph outlines, saves, and repeats the same mounted proof after reopening in a fresh browser page. The downloaded PDF can be checked through the independent oracle. The browser script transpiles. A governed app build with the composed changes is pending.

The four source-closure measurements are equal on the sampled macOS, Linux, Windows and WASI target cfgs. Counts are file inputs, active lines and source bytes.

| Owner | Library | Test |
| --- | --- | --- |
| Lift | 12 / 3,787 / 207,111 | 14 / 3,948 / 213,853 |
| Fonts | 6 / 2,718 / 132,604 | 11 / 3,050 / 163,214 |
| Inferences | 24 / 2,624 / 141,159 | 32 / 3,108 / 171,156 |
| Content | 29 / 2,832 / 137,562 | 29 / 2,832 / 137,562 |

Live product sources remain at the active governed capture epoch.
