# PDF editor and viewer: i18n and accessibility fixes

Fixes the findings of `🌐️pdf-i18n-a11y-audit.md`, modelled on `xlsx-i18n-fixes.md`. Base is PDF 1.7 `🧱️base`; shared code reaches every 1.7 subset and the 1.4 editors and viewers.

## Findings and fixes

- H1 numbers. `pdf_decimal_separator`, `format_pdf_number`, `parse_pdf_number` and `canonical_quad` live in `✏️editor/🖼️page/🧰️arguments`. Only the locale's own separator is a decimal mark (German `1,5`; English `1,5` is refused; a repeated or foreign separator is refused). The inspector now renders numeric fields as text inputs showing the locale notation, stamps the session locale (`locale`) on every field commit, and `req_num`/`opt_num` parse only the committed typed value with that locale. Agent and stored values stay canonical (`None`). The page box quad no longer uses the host `blurDecimalQuad` commit (the host validator accepts only dots); it commits on blur, shows `612,5` in German and is canonicalized in `edit_from_action`.
- H2 viewer summary. `page_summary(index, page, locale)` uses the `PdfViewerLabels` catalog (Page/Seite, MediaBox/Medienrahmen, CropBox/Beschnittrahmen, empty-text note) and locale decimals. All seven 1.7 viewer `main` copies and the three 1.4 viewer mains (dimensions with locale decimals) are updated; `render` takes the locale.
- H3 layout titles. All 20 edit and view layouts (1.7 and 1.4) and `document_layout` use `title: None`.
- H4 faults. `PdfFault` (document crate, `🚨️faults`) is a closed enum with code, English and German notice in exhaustive matches. `pdf_fault_notices()` is declared as `fault_notices()` by the base editor and both kit macros. Coded refusals: argument-missing, argument-not-finite, number-invalid, the three cancellation phases, stale-target, invalid-item, image-placement-invalid, png-import-job, list-needs-numbers (class reserved), unhandled-action, and the generic page-edit class. Details stay English developer text.
- H5 canvas name. Another agent concurrently changed the framework `scene_surface(id, kind, scene, name, description)` to require an accessible name (replacing my interim `named_scene_surface`) and updated the PDF callers; the PDF page canvas passes the catalog name and description. The PDF page canvas passes "Page canvas / Seitenfläche" with a description pointing to the text layer and inspector.
- M1/L2 `if De else en` closures now use the exhaustive `pick`/`pick_label`; the fixed texts live in `PdfEditorLabels` (`🖼️page/🗣️labels`).
- M2 `image_replacement_button` takes `Locale`.
- M3 unknown body is resolved from `PdfEditorLabels`/`PdfViewerLabels` in all editors (base, kits for 1.7 and 1.4) and all ten viewers.
- M4 no heading primitive exists in the framework; headings are emphasized text and entry forms carry the heading as the group's accessible name.
- M5 German boundary help text now states the input format.
- M6 cancellation messages are coded per phase and bilingual.
- L1 "Untitled / Ohne Titel" capitalization.

## Deliberate deviations and residuals

- The `scene_surface` signature change is owned by another agent; at last check `semio-plugin-window-kits` (text, mesh) was not yet migrated, which blocks `cargo test` of the PDF crates.
- About 180 internal `fault(message)` calls keep the generic `stdio.pdf.page-edit` code with a fixed bilingual notice; they were not split per message.
- The inspector raw dictionary key labels (L3) and technical key names (L4) are unchanged on purpose.
- The Details window title remains a shared contract.
- PDF 1.4 own `refusal` code (`stdio.pdf.1.4.page-edit`) is not a valid notice code (digit segments) and stays unlocalized.
- Number fields are no longer `InputKind::Number`, so they lose host steppers; accessibility name and value remain.

## Tests

Page editor tests: German decimal edit, English refusal, number trio, quad canonicalization, German vs English inspector box text, fault notice catalog, layout titles, canvas accessible name. Viewer tests: German/English summary, label catalog, layout titles, 1.4 dimension decimals.

## Verification

`repo cargo` stalled (44 min) so the native fallback was used: `check -p semio-s-artifact-stdio-pdf --lib --tests` passes; the later `test -p semio-s-artifact-stdio-pdf-page-editor-tests` run failed to compile only in `semio-plugin-window-kits` (`scene_surface` takes 5 arguments, 3 supplied in `📝️text` and `🧊️mesh`), an unfinished framework migration outside the PDF code. The new tests are therefore compiled-checked but not yet executed.

## JPX preview and boundary boxes (follow-up)

- `paint_fidelity` case `codec-jpx-rgb` (probe [3,3] black vs MuPDF 5,6,3) was not a JPEG 2000 decode, colour or component-mapping bug. The in-house decoder is bit-identical to Apple's ImageIO JPEG 2000 decoder on this stream (all probed samples equal), and the MuPDF probes are reproduced to within 1 by a bilinear sample at the device pixel centre (for example [3,3] blends samples (1,1), (2,1), (1,2), (2,2) into 6,6,4). Root cause: the canvas image preview rasterized at source density and was then sampled nearest, while MuPDF smooths a modest upscale (this image is 2x). The earlier fixtures only probed exact sample centres or upscales of 4x and more, which MuPDF keeps crisp.
- Fix: `PdfImagePreviewSamples::rgba_smooth` (`🖼️images/🎭️sampled-alpha`) samples bilinearly between the four nearest sample centres, weights colour by opacity and clamps at the borders. `image_layer` (`🖼️page/🔆️image-preview`) uses it when the image is upscaled by at least 1x and less than `SMOOTH_UPSCALE_LIMIT` (4x) on both axes, and then rasterizes at device density so the canvas does not add its own nearest scaling; larger upscales and downscales keep the previous crisp sampling.
- Boundary boxes: all five page boxes (media, crop, bleed, trim, art) now use the same locale text input with blur commit as the other numeric fields. The inspector no longer selects the host dot-only `blurDecimalQuad` commit for any field; the quad is shown in the session notation (`612,5`) and canonicalized in `edit_from_action`. The page-boundary test now asserts zero `blurDecimalQuad` inputs; the German inspector test sets all four optional boxes to decimals, asserts the German commas (and absence of the English dots), the English dots (and absence of commas), no `blurDecimalQuad`, and a German quad canonicalization.
- Residual: the 4x threshold is derived from the fixture set (2x smoothed, 4x and larger crisp); MuPDF's exact rule for factors between 2x and 4x is not pinned by a probe.

## One-based page numbers

Every page a person reads or types is now one-based; the model, the mutations and the payload-to-mutation boundary keep zero-based indices. `insert-text` with page `1` on a one-page document succeeds; page `0` and page `2` are refused.

- Single conversion: `page_index(count, number)` and `page_number(index)` (`🔭️view/🔢️page-number`, the only place that adds or subtracts one). `resolve_page` (`📄️document`) wraps it with the coded refusal, `page_of` is the matching lookup of a page by index, `page_slots` widens the range by one for the actions that insert a page (`insert-page`, `merge-pick`, `merge-document`).
- Coded fault `PdfFault::PageNumberOutOfRange` (`stdio.pdf.page-number-out-of-range`, params `page` and `last`): en "Page {page} does not exist. Enter a page number from 1 to {last}." / de "Seite {page} existiert nicht. Geben Sie eine Seitenzahl von 1 bis {last} ein." It replaces the English `pdf page N does not exist` at the three lookup sites (`insert_object`, `create_annotation`, `replace_page_with`) and the `set-page-size` lookup.
- `edit_from_action` keeps the number the person typed in the replayable payload (`page` is `null` for actions without a page); `apply_payload`, the retained authoring work (`fields`), the go-to-page view work, the presence pointer (`presence_intent` now takes the page count) and the 1.4 canvas (`emit_page_edit`) each resolve it once through `resolve_page` against the live snapshot, so a stale or out-of-range number is refused where the document is known.
- Declared arguments: every `page` argument of the page window, the view action `go-to-page`, `presence-pointer` and the PNG actions is labelled "Page number" / "Seitenzahl"; `move-page` takes the target page number in `x` ("Target page number" / "Zielseitenzahl"), the link aspect `page` of `set-annotation-kind` and the `page:N` link text are one-based as well. Out-of-range targets are refused now (the old `move-page` clamp for `x: 9` is gone and its fixture case removed).
- Emitters: the inspector (`object_args`), the page-structure buttons, the thumbnails (move earlier/later, drag data, drop target) and the replace-image button emit `page_number(index)`; displays (inspector heading, thumbnail rows, text layer) use the same function.
- Decision for agents and automation: machine tool arguments match what the UI shows, so they are one-based as well; the schema labels say so. The one deliberate index is the draft-binding commit `set-page` (and the 1.4 `replace-page-text`): its `page` is the host's shared document-text draft address (also used by the pptx editors) and is declared `ActionArgDef::index` with the label "Page index" / "Seitenindex". The PNG callback `import-local-image` also echoes the zero-based index its picker resolved; only the user-facing `insert-image` / `replace-image` take the number.
- Dirty scope: insert-text publishes `UiDirtyScope::Full` through the typed-operation completion (the page, thumbnails, text layer and inspector windows are all refreshed); the inspector-only `operation_progress_scope` is used for progress ticks and cancellation only. The new test asserts the inserted text is a page object and a layer of the canvas scene right after the edit; the end-to-end browser check needs a rebuilt wasm, which was not done here.

### Tests
New: `insert_text_on_page_one_of_a_one_page_document_succeeds_in_every_language` (invariant, `en`, `de` typed decimals), `page_numbers_outside_the_document_are_refused_with_the_localized_valid_range` (page 0 and 2, en and de message text), `page_number_conversion_is_one_based_and_total_over_every_page_action` (insert slot, move target), `pointer_page_is_a_one_based_number` (presence). Updated by hand: the page editor unit tests (55 `page` arguments, move-page target, annotation-kind helper), the authoring tests and `authoring.json` (insert/move/remove/merge/replace cases, clamp case dropped), the native-page-edit and natural-file-edit rotation fixtures, the 1.4 canvas tests and the three 1.4 editor tests, the root editor tests, the journey `pdf-pane-journey.ts` (page `1`). Verified with filtered native runs: `page-editor-tests` 65 passed, `page-editor-authoring-tests` 20 passed, `presence-lane` 10 passed.
