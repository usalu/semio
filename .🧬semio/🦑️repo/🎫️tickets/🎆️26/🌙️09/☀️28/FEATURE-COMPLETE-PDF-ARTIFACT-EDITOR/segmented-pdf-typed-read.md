# Segmented PDF Trailers and Indirect Object Frames

The staged classical cross-reference reader now parses its complete typed trailer through the same byte callback as its rows. The indirect frame reader uses that callback for the header, typed object and stream introducer. The existing slice entry points for the shared COS, string and frame readers delegate to the byte-addressed parser. The surrounding classic revision dispatcher adapts its current slice at the boundary.

The COS reader retains the existing 256-byte maximum scalar token in a fixed inline buffer. A segmented name or number resumes without rereading the token span. The buffer adds no separate heap allocation. The existing allocation and release grants still control token values, strings and dictionary backing. A COS work unit performs at most two byte lookups, including angle-bracket and reference lookahead. Escaped string scanning skips the former duplicate lookups. The focused law tested escaped names, octal escapes, CRLF continuation, hex nibbles, references, the complete 256-byte scalar and rejection of a 257-byte scalar.

On the 9,437,877-byte neutral fixture, the actual classic section emitted five live rows and one free row and returned the typed trailer with the correct root and size. The actual revision-link reader found no previous or hybrid section. The law denied the first trailer allocation, resumed under a one-unit work grant, and drove close after both completion and partial cancellation. A zero release grant refused retirement and a later paid grant returned all allocation charged to each owner. The complete and cancelled trailer attempts together read 342, 351, 378 and 693 physical bytes with source segments of 1, 3, 13 and 4,096 bytes respectively.

The law then used the image offset reported by the same xref parser to frame object 5. Before checking the payload it inspected fewer than 256 bytes. The original dictionary reported 3,072 × 3,072 samples and a 9,437,184-byte body. A checked source range verified the four grayscale quadrant samples, the end bound and the adjacent stream terminator. MuPDF independently returned the same typed trailer, stream length and raw samples. The resulting source range was exercised in the focused law. The graph projection cursor has yet to consume that range.

The red focused link against the prior probes and classic-table stage failed specifically at the sliced classic trailer API and the missing COS and indirect frame byte-addressed entry points. The complete candidate passed. The focused link used the repository’s COS and frame implementations and PDF model declarations with serialization derives removed for the isolated build. It did not build the application.

The canonical Cargo source reader accepted 48 library and test measurements over the Linux, macOS, Windows and WASI configurations, including both testkit selections where declared. All 24 product source preimages matched. The largest active-line count was the xref-stream test closure at 3,999, with 31 files and 186,270 bytes. The lexer test closure has 3,682 lines, 29 files and 157,068 bytes. Graph projection tests have 3,954 lines, and the conformance-support test closure has 3,182. Product sources remain untouched.

The remaining graph handoffs still carry contiguous slices. The normal projection and xref-stream decode need a bounded original-stream range, and the current retained graph reserves the full raw stream in one allocation before copying. The next source ownership change has to address that allocation and the storage consumed by the snapshot, in addition to the source transport.

- [Staged producer](📜️pdf-cos-byte-stage.ts)
- [Focused Rust law](📜️pdf-cos-byte-law.rs)
- [Cargo source measurement](📜️pdf-xref-byte-bounds.ts)
- [Earlier cross-reference work](segmented-pdf-xref-reads.md)

