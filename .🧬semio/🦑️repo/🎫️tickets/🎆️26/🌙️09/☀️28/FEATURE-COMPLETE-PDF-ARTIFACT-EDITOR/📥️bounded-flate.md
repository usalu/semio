# Bounded PDF Flate decoding

The cursor reads a stable caller-owned compressed stream slice and stages expanded bytes into the caller’s supplied output span. It uses the repository’s byte-resumable DEFLATE engine and retains only the admitted RFC 1950 history window. The work receipt counts frame initialization, history allocation, and each DEFLATE transition, including turns that fail. The output receipt reports the exact staged prefix. The caller can publish that prefix after terminal Adler-32 validation succeeds.

The cursor exposes compressed and expanded byte progress, returns an explicit history allocation demand, and can yield for either fuel or output space. Cancellation stops further decoding and hands the retained inflater to bounded retirement. The decoder reports a separate receipt for physical history release.

The language-neutral fixture has stored, fixed, and dynamic DEFLATE blocks carrying PDF content commands. Python’s independently implemented zlib decoder reproduced all three logical streams and rejected their corrupted checksums. Rust tests exercise small grants, output backpressure, expansion limits, cancellation, exact release admission, truncation, and trailing payloads.

The production PDF stream decoder uses the cursor for wrapped Flate and raw recovery, then applies the declared predictor in the same pipeline. The ordinary synchronous entry point collects the staged bytes for its existing callers. Other PDF filters still use their existing synchronous stages.

The combined predictor cursor stages one inflated byte at a time and grants PNG row buffers and TIFF component history separately. PNG supports every row filter. TIFF decodes packed 1-, 2-, and 4-bit components and 8- and 16-bit components. It counts compressed, inflated, and final output bytes independently. Its close drive releases each row or component allocation before retiring the inflater history. The shared encoder uses the same TIFF component arithmetic in reverse so exported predicted data can reopen correctly.

The bounded filter implementation and its contract tests are owned by `semio-s-artifact-stdio-pdf-filters`. The inferences package keeps the stream-dictionary integration test and reexports its incremental cursor API. The repository's Rust source closure reports 8 inputs with 1,022 active lines for the filters test target and 30 inputs with 2,535 active lines for the inferences test target after the lexer extraction.

The dedicated filters Cargo suite passed all seven tests, including TIFF writer round trips, exact output limits, and split sixteen-bit sample handling. The same tests passed when compiled directly against repository-built dependencies. The first focused PDF inferences package command passed seven tests, covering framing recovery, predictor dispatch, one-byte output, and cancellation. Python zlib matched six framing and recovery cases. MuPDF matched both shared pipeline predictor outputs and all six bounded predictor fixtures.

The controlled Flate and predictor stages are available to the ordinary-stream and object-stream importer. Full mounted import still needs the enclosing parser and publication work.
