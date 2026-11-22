# PDF 1.4 page rendering

## Finding
The 1.4 snapshot is `PdfSnapshot { pages: Vec<PageDoc { width, height, text }> }`, and the 1.4 natural-file codec opens only files its own writer produced (`decode_pdf_lossless`). So every 1.4 document is exactly "N pages, one Helvetica 12 pt text object at (72, height - 72)". It has no object graph, so a native 1.4 scene builder would have nothing to paint beyond that.

## Route: one shared page path
PDF 1.4 is a subset of 1.7, so the 1.4 pages are lifted into the 1.7 page model, and the 1.7 scene builder, view lane, thumbnails, text layer and selection serve both dialects.

- New crate `semio-s-artifact-stdio-pdf14-canvas` at `4.1.4/base/editor/page/canvas` (the 1.4 page crate is `semio-s-artifact-stdio-pdf-schema-3`; it cannot depend on the lift, because `graph-projection -> schema-2 -> schema-3` would form a cycle, so the lift sits above it).
- `lift` = `encode_pdf` (1.4 writer) followed by the 1.7 `decode_pdf`. What is painted is therefore exactly what a reader opens after save. There is no second page model and no cache; each render lifts again (not measured).
- Canvas window = the 1.7 `pdf.page` window definition, filtered to the actions the 1.4 domain can publish (pointer, set-text, insert/remove/move page, set-page-size, ten view actions). The thumbnails and text-layer windows are reused unchanged. The 1.4 text-draft window stays (renamed "Page Text" / "Seitentext") for revision-guarded text edits. It keeps only `replace-page-text`, and page structure moved to the canvas window.
- Page edits use the 1.7 payload (JSON): `emit_page_edit` maps `set-text` to `ReplacePageText`, `insert-page`, `remove-page`, `move-page` and `set-page-size` to the 1.4 mutations, `canvasPointerDown` to the 1.7 selection effect over the lifted pages, and refuses everything else (fills, move, resize, images and so on do not exist in the 1.4 model).
- View lane (`semio-s-artifact-stdio-pdf-view-lane`): `ViewCommandWork` now holds a `PageLens`, so a dialect can reduce zoom, fit and search over a lifted page model (`lifted_view_work`; 1.7 passes `own_page`). `render_page_body_of` takes a page model and a lazy publication revision. The three 1.4 editors (base, x, a) gained the typed transient (`PdfViewTransient`), `transient_tools`, `render_with_request_context`, the object selection domain and the new layout (thumbnails, canvas, text layer, text drafts, details).
- No inspector window for 1.4: its fields (fill, move, font, images) have no 1.4 mutation. Text is edited with `set-text` on the selected object, or in the text-draft window.

## Tests (language-agnostic)
`canvas/tests/unit/pdf14-canvas.json` holds three cases (11-char letter page; two pages of different size; blank page), the expected window actions and search expectations. `pdf14-canvas-oracle.py` (`uv run --with pymupdf==1.26.5 python -I ... write|check`) builds each page as the 1.4 writer does, renders it with MuPDF at 8x, stores 26 probe colours (white paper, centres of black 3x3 pixel blocks inside glyphs) and verifies the MuPDF search hit counts. `check` passed (0 failures). The Rust tests composite the scene layers of the lifted pages at the same probe points and compare with the tolerance in the fixture. They also cover selection, `set-text` with its inverse, page structure, search, the view reducer, thumbnails (en and de) and the layout.

## Not done
- 1.4 viewers (read-only) are unchanged.
- Hub and other workspace `Cargo.lock` files were not updated by hand.
- Zoom and pan by wheel stay with the host, as for 1.7.
