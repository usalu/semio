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
