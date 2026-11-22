# Segmented PDF Preparation and Compressed Reads

The staged parser accepts a borrowed stable `ByteReadSource` at `PdfFilePreparationCursor::advance` and returns the existing `PdfByteGraphReceipt`. The source stays borrowed for each turn. Its exact extent and owned storage are supplied by the caller. Slice and `Vec<u8>` callers continue through the same cursor. The prepared result owns its graph, position index and trailer and can move to semantic lift after the source has been released.

The focused Rust executable took `PreparedPdfFile` from the 9,437,877-byte fixture with a seven-byte logical preamble and a non-contiguous file source. It retained the correct original header location, five objects, their exact position index, the root reference, and the 9,437,184-byte image. Preparation removed the structural size field and reported no source reads after graph transfer. The prepared result was inspected after releasing the source. The highest paid backing observed at preparation was 9,515,012 bytes under a 10 MiB aggregate cap. Invoking cancellation during the text phase returned the complete charged balance after an initial zero-release refusal.

The compressed candidate also carries the byte reader through the xref stream frame, checked payload range and controlled filter chain. Flate, predicted Flate and LZW, ASCIIHex, ASCII85 and RunLength consume the bounded source through the same interface. The Flate terminator verifies its four-byte Adler-32 checksum through the source reader. The codec laws compare outputs and retirement totals under single-work-unit grants. They also cover admitted bytes disappearing from the source, checksum failure and the existing raw retention for an unknown filter.

The actual prepared-file path passed three further genuine segmented fixture cases under one-work-unit grants. The classic and Flate incremental documents both produced the typed title `Updated` and kept the free-object tombstone out of the graph. The compressed-object fixture supplied its indirect stream length from the decoded object stream and retained both payloads containing literal `endstream` marker text. MuPDF independently verified the title, tombstone, compressed length and exact two stream payloads. Python zlib expanded the 29-byte compressed xref payload to the expected 28 row bytes. The focused executable links the real repository DEFLATE engine, xref rows, graph preparation, text-scalar and date code. It uses isolated model declarations without serialization derives.

The composed overlay includes 38 exact source preimages and has SHA-256 `fb34cc17f48be09e58664d7c989e863c681f55ae5fd17cdf45838bab413795d3`. The Cargo source reader admitted all measured libraries and 88 of 96 library and test selections. The xref stream test closure has 31 files, 3,998 active lines and 187,002 bytes across Linux, macOS, Windows and WASI. The same eight mutation-family-11 test selections exceed the source count in the live and staged trees at 34 files. Product source remains untouched.

The install overlay moves the existing VT trim-box text codec tests to the package that already owns both codecs. Their test files and assertions stay in place. The mutation-family-11 mirror continues reexporting the same two codec modules. The install overlay has 40 exact source preimages and SHA-256 `d02724ef5993d32baa128766ae38ba282ac37f47a181121bc65270cf8b150ef0`. All 112 source closure selections pass. The former test owner now has 32 files, 2,730 active lines and 151,214 bytes. The owning VT test closure has 30 files, 830 active lines and 38,306 bytes.

Both original text codec round trips linked to the repository’s existing VT WASI library and ran successfully through jco. The focused command invoked the original assertion bodies directly because WASI cannot spawn the children required by the abort-mode Rust test harness.

The mounted natural-file owner has yet to supply paged source custody at this seam. Intermediate decoded streams and snapshot stream data use owned vectors, and a 9,437,184-byte decoded image still requires one final vector allocation and matching retirement credit. The synchronous compatibility reader is still based on a slice. No full application build or editor open was run against the overlay.

- [Compressed source producer](📜️pdf-compressed-byte-stage.ts)
- [Focused linker](📜️pdf-graph-byte-link.ts)
- [File preparation law](📜️pdf-prepared-byte-law.rs)
- [Codec law](📜️pdf-compressed-byte-law.rs)
- [VT test owner producer](📜️pdf-vt-test-owner-stage.ts)
- [VT test owner law](📜️pdf-vt-test-owner-law.py)
- [Earlier file and graph evidence](segmented-pdf-graph-import.md)

