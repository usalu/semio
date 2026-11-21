# Remaining PDF Product Acceptance

Source audit; no browser mount or build was performed. Native retest and browser component gates remain active. Existing browser scripts are authored assertions, not passing mounted evidence. The registered factory report records passing native page edit history and draft publication, but Open/media, picker and Text Layer required a fresh runtime retest after fixes.

## Live surface and mounted coverage

Paths are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor`.

| Surface | Live source | Authored proof and gap |
| --- | --- | --- |
| Document properties | `🖼️page/🔎️inspector/🦀️.rs` exposes title, author, subject, keywords, creator, producer, dates, trapped and language; `🖼️page/🎬️actions/🦀️.rs` emits SetInfo and SetLanguage. | `📜️pdf-browser-smoke.ts edit-content` edits title, checks author, saves and reopens. Other fields and language are not covered there. The Stdio natural-file PDF acceptance row separately specifies metadata, save, history, close, reopen, reload, export and malformed refusal. |
| Text and vector | Inspector exposes text, font, position and size; vectors expose geometry, line width, fill and stroke. `🖼️page/🖱️object-edits/🦀️.rs` emits SetPageContent and font mutations; actions include insert-text, insert-rectangle and insert-line. | Edit-content selects accessible `Text: …` and `Vector graphic`, edits `pdf-object-text` and `pdf-object-x`, then saves and reopens. It does not prove insertion, deletion, font, size, colors or independent rendering. Large-text and large-vector inspect the final page. |
| Pages | Inspector exposes size, rotation, user unit and five boxes; actions emit InsertPage, RemovePage, MovePage and box mutations. | Default smoke selects page 2 via canvas, changes `pdf-page-rotation`, saves and reopens. Edit-content checks crop and trim, with undo/redo only for trim. Insert/remove/move and independent preservation of other pages need mounted checks. |
| Images | Inspector exposes placement and image attributes; `📄️document/🖼️image-authoring/🎛️replace-button.rs` exposes “Replace Image with PNG”; `insert-image` is a page action. | Edit-content changes image width. `image-authoring-browser.ts` scripts PNG insertion, selected shared and inline replacement, undo/redo, picker cancellation, save and reopen. `image-authoring-cancel-browser.ts` scripts active decode cancellation and retry with byte equality after cancellation and undo. Neither has a successful mounted run yet. Large-images opens and saves four pages and checks names and geometry; it does not author a large image. |

## Earliest proofs after both current components publish

1. Run the natural-file PDF acceptance row and default rotation smoke against the actual shell. Require visible inspector and downloaded PDF, then run the existing independent MuPDF download oracle. This establishes real Open → edit → Save → reopen.
2. Run `📜️pdf-browser-smoke.ts` with `edit-content` and `🔣️mounted-content-edit.json`. Verify the download independently as well as the reopened inspector: title and untouched author, text, vector position, image width, crop and trim; check untouched page and contents. The script verifies trim history only.
3. Run `image-authoring-browser.ts` with prepared `image-source.pdf` and the independent image oracle, then `image-authoring-cancel-browser.ts`. Check shared-resource isolation, inline replacement, pixels and cancellation. Selectors are action `insert-image`, `insert-image.arg.{page,x,y,width,height}`, “Execute”, image Text Layer buttons, and “Replace Image with PNG”.
4. Run `large-vector`, `large-text`, `large-images` and `cancel-large` smoke modes on exact fixtures. These check visible output paths, not all semantic edits. Cancellation requires positive task progress, task cancellation, unchanged baseline windows and successful retry.
5. Add mounted page insertion/move/removal and undo/redo proof using the registered action rail, then save/reopen and independently compare page order, dimensions and unchanged content. Add property, text/font and vector style coverage for claims beyond the small edit scenario.

## Separate staged boundaries

The normal PNG picker currently has the 24,546-byte source and 4,096-pixel sampled model described in `📋️pdf-editor-media-integration.md`. Staged paged transport still needs a lifetime-bound operation and authenticated ingress; large JPEG preview needs revision/source/viewport binding and paid output closure. Standard Save still produces a contiguous native payload; segmented producer and delivery wiring are staged, not demonstrated by the small Save smoke. Mounted large-image history also needs the custom wire source and receiver installed together and a revision-bound edit/reopen/undo/redo proof. Four-page image readback does not establish those paths.

References: `📋️mounted-pdf-e2e.md`, `📋️pdf-editor-media-integration.md`, `📋️registered-pdf-factory-retest.md`, `📜️pdf-browser-smoke.ts`, `image-authoring-browser.ts`, `image-authoring-cancel-browser.ts`.

The ticket now has an authored `page-history` smoke mode and `🔣️mounted-page-history.json`. It operates on an independently framed PDF with distinct 216×216 and 240×260 pages, runs the actual Action rail commands for insert, move and remove, checks each of the six undo/redo transitions, and saves and reopens. `📜️pdf-page-history-oracle.py` checks the delivered page order, dimensions, text and untouched-page raster in MuPDF. The fixture was independently read and the TypeScript parsed. Mounted execution still awaits the real browser components.

The separate `style-properties` mode covers actual text font size, vector fill, document language and subject, with an unchanged author and second-page raster. Its original PDF passed the independent MuPDF field/metrics check. Source inspection exposed a functional defect in the existing font size control. It called generic resize, which modified the text matrix while the inspector displayed the unchanged Tf value. The dedicated text-size mutation path is now live after an independent MuPDF position and metric comparison. The mounted style mode is prepared for a component build that consumes those exact sources.
