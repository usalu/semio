# Segmented PDF File Structure and Graph Import

The staged candidate carries the repository’s checked byte ranges from PDF header and classic revision discovery through object projection. The file structure and graph use the same source view, including a checked header-relative origin when the absolute directory does not resolve. Classic section scanning, typed trailers, normal-object framing, indirect numeric lengths and stream-boundary inspection consume bounded byte reads. The source interface can optionally lend an already-contiguous span to an encoded decoder. That query never assembles storage.

The existing identity decoder now reads its original body from a checked range into its 256-byte scratch buffer. It accounts for input and output work with the existing grant and quotes the exact final extent on its first output allocation. This avoids reallocating and migrating the existing output as it grows. The fallback for a filter the decoder already treats as unsupported copies from the checked range directly into its admitted retained stream buffer. A short read fails before the partial copy can be published. Ordinary contiguous encoded inputs retain their existing codec path. A segmented recognized encoded body requires a contiguous-backed codec or returns an ownership refusal. A segmented xref stream enters the existing explicit reference-required path.

The focused Rust executable drives the staged `PdfByteGraphCursor` on the 9,437,877-byte neutral fixture through a non-contiguous disk source with a seven-byte logical preamble. The real xref and trailer cursors settle at the relative origin, then the real graph cursor transfers all five indexed objects. The law compares every byte of the retained 9,437,184-byte image against the neutral quadrant values. MuPDF independently returned the same full payload and SHA-256 `bd7a3ff2839c399aee46ad902507846a0f579242a5645b3811bb5dd74bddbeae`.

The successful graph run read 9,447,455 physical bytes and peaked at 9,488,956 grant-accounted bytes under a 10 MiB aggregate cap. The current snapshot represents stream data as `Vec<u8>`, so the transfer still requires one allocation for the final image and a grant large enough to release it as one backing. A 9 MiB aggregate cap rejects the output allocation because the other graph owners consume part of the grant. An output limit one byte below the image length rejects the stream before pulling image data. Successful and partially copied cancelled graph owners retire all charged bytes. The focused laws also cover an indirect generation-three length, marker text inside the stream body, preservation of a vendor-filter stream, existing ASCIIHex decode on contiguous storage, and explicit refusal for segmented recognized encoded bodies and compressed cross-references.

The red focused link fails at the prior sliced file and graph input signatures. The green link uses the staged production file, xref, normal-object, stream-data and graph cursors. Its PDF model declarations omit serialization derives for the isolated build. Unexercised Flate, LZW, predictor and xref-stream implementations are outside the link. The coded-packet law uses the real ASCIIHex reader.

The canonical Cargo source reader measured 96 library and test selections across Linux, macOS, Windows and WASI. It admitted 88, including all measured libraries, and found an owner for each of the 32 changed source paths. The xref-stream test closure has 31 files, 4,000 active lines and 186,962 bytes. Eight selections of the mutation-family-11 tests still exceed the source file limit. Both the live and candidate measurements have 34 files. All staged edits are guarded by exact source hashes. Product sources remain untouched.

The mounted natural-file decoder still owns its original input as a `Vec<u8>`. Receiving and transferring a paged source into that owner, retiring the source by pages, and storing large PDF streams in the snapshot without an indivisible output allocation remain separate end-to-end barriers. No mounted editor run was made against this candidate.

- [Staged source producer](📜️pdf-graph-byte-stage.ts)
- [Focused graph linker](📜️pdf-graph-byte-link.ts)
- [Focused native law](📜️pdf-graph-byte-law.rs)
- [Cargo source closure checks](📜️pdf-xref-byte-bounds.ts)
- [Earlier typed trailer and frame proof](segmented-pdf-typed-read.md)

