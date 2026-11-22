# Draw PDF Original Funding

The registered Draw serializer reaches the PDF writer through `ExportProducer` in the `pdf-write` phase. It previously spent one fuel unit per step and used the standalone writer. That path grew three paged lists, allocated 3,072-byte byte pages, and built owned stream dictionaries without a retained allocation receipt.

The registered phase now advances the shared writer with the original caller's context. Each step quotes the paged list backing, body page, or byte append before entering the effect and settles the first-party retained ledger through the same effect machinery as scene preparation. A page allocated before its list slot is published remains inside the writer's retirement owners. Typed stream dictionary operands replace owned strings and render the same COS tokens and spacing into the existing fixed fragment.

The neutral rectangle fixture gives the writer one fuel unit at a time. It denies a page with 3,071 bytes of turn capacity and checks unchanged progress and zero allocator effects. It then funds the 3,072-byte allocation, compares physical allocation and release observations with the retained receipts on every writing turn, compares the completed PDF bytes with both standalone grants, and sends the bytes to the existing independent PDFium raster oracle. Existing fixture rows cover gradients, masks, blending, and drawing paths.

The scoped whitespace check passed. The repository-provisioned PDFium runtime reported 5.15.2. The focused native test is `pdf_write_neutral_styles_match_pdfium_and_exact_physical_close` in `semio-s-artifact-draw-drawing-write`. The repository Cargo ingress admitted one compiler core with an isolated store and a 2,550 MB job reservation. Cargo is compiling dependencies. The test has yet to produce PDF bytes for the oracle.

The first native capture failed before writer tests because the Draw IO `testkit` feature did not expose the optional job crate to `physical_close` (`E0432`). The Draw IO manifest was corrected in the shared workspace. A fresh repository Cargo capture with the shared store and `write_` filter has been admitted to compile and run both writers’ fixtures.
