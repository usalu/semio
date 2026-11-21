# Natural PDF Segmented Save

The normal shell Save now resolves the declared export action against the exact native binary format and `artifact:native` output port. When the editor declares that producer, the shell passes its generated natural filename to the existing revision-bound segmented download. It still persists and verifies the attached archive before requesting output. The sink accepts the bytes privately, rechecks the exact persisted archive and document revision, and commits only after both checks. A cancellation while that archive check is in progress aborts the sink. Existing natural codecs without a declared binary producer keep their current delivery.

The focused replay reproduced both failures before the change. A caller that refused the archive or cancelled during the pre-delivery check still closed the sink. The same replay passed after the change. Ajv independently admitted the neutral delivery vectors. The repository's real renderer test target then ran the checked-media-export and natural-scope suites through its canonical Vitest runner. Both suites passed, with 52 tests total. Natural-scope is now included in the engine suite list. Its actual Save handler submitted the exact app, port, document and revision, wrote the segmented bytes, and refused the download when the archive changed during the operation.

The registered PDF producer is still staged. It must use the existing factory registry, the matching Export Media File action and both PDF native format lists. An ordinary PdfSnapshot graph reconciler/emitter is in progress. The current host collect route retains separate chunks until it checks the descriptor and initial revision; there is no final contiguous buffer in that segment handoff. The mounted PDF path remains to be verified.

- [Focused red and green replay](📜️natural-media-save-probe.ts)
- [Renderer test runner](📜️natural-media-save-renderer.ts)
- [Source application guard](📜️natural-media-save-stage.ts)
- [Original PDF producer stage](pdf-registered-save.md)

The subsequent renderer change opens the private sink after verifying the exact descriptor and source identity, then awaits each sink write before taking the next page. The default shell sink retains immutable Blob pages and delivers the final Blob directly through an anchor. Its former implementation retained both the completed transport list and the copied sink pages, then built a contiguous JavaScript array. Three registered renderer suites passed with 63 tests, including the new drain ordering test and actual scope tests for successful Save and a document edit before delivery. The focused replay delivered an independently framed two-page PDF byte for byte. MuPDF independently reopened both pages, with matching text, page rectangles and rasters.
