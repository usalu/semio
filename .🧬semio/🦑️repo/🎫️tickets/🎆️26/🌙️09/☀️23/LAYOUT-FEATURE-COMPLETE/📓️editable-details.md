# Editable layout details

The inspector was a read-only summary. Catalogue placement already covered png, pdf, drawing, raster, and a few peers, and stopped there.

## Document, page, and frame

The inspection panel keeps the read-only summary first, then edits:

- Document: name, print target, data fields (`patchDocument`).
- Baseline grid: size, offset, and snap (`patchDocument` → `update-grid`).
- Active page: name, width, height, four margins, column count, gutter. Move earlier, move later, and delete (delete is offered when more than one page exists) go through `patchPage` → `reorder-pages` and `delete-page`.
- Selected frame: x, y, width, height, rotation, locked, visible (`patchFrame` → `set-frame-flags` for the flags). Rectangles also edit fill and stroke (`#rrggbb` keeps the previous alpha). Text frames edit columns, wrap mode, and story body. Image frames edit the link path and show the linked artifact kind.

`cargo test -p semio-s-artifact-layout-layout --lib -- update_grid_replaces set_frame_flags_locks kinds_match patch_document_updates patch_document_renames patch_page_reorders selected_frame_page` passed, including `patch_page_reorders_and_deletes_and_patch_frame_sets_flags`.

## Paragraph styles

Paragraph, character, parent-page, and spread deltas now apply on the layout diff. A paragraph style's name, font family, size, weight, leading, tracking, and alignment are editable in the inspector (`patchDocument` field `{styleId}.{key}` → `update-paragraph-style`). Alignment is left, center, right, or justify.

`cargo test -p semio-s-artifact-layout-layout --lib -- update_paragraph_style_changes patch_document_updates_a_paragraph_style selected_frame_page` — `patch_document_updates_a_paragraph_style` passed after the binary protocol gained tag 29. The focused set before that protocol line had already passed the mutation unit test, the kind catalog check, and the inspector projection.

## Text frames and layers

A selected text frame also edits inset x, y, width, and height, the story it flows, and the next frame in the thread (`patchFrame` → `update-text-frame`). A blank thread target clears the link. The next frame must be another text frame.

Each layer on the active page edits its name, visibility, and lock (`patchPage` → `update-layer`).

`cargo test -p semio-s-artifact-layout-layout --lib -- update_text_frame_sets update_layer_renames patch_frame_sets_text_inset selected_frame_page kinds_match` passed (5 tests).

## Catalogue

The catalogue also places note, writer, forms, flow, equation, puzzle 2d, block 2d, generation 2d, terrain, grid 2d, wfc 2d, presentation, sequence, and wires. Each drop stores `artifactKind` and `artifactRef` on an image-frame link and paints a typed mark when there is no proxy.

## Character styles, masters, and spreads

The inspector adds a character style (`patchDocument` → `create-character-style`), then edits its name, font family, size, weight, italic, color, and tracking (`update-character-style`). Deleting a style that a story run still uses is refused. Undo of a delete restores the style's fields.

Each spread's name, and each parent page's name, width, and height, are editable. The active page chooses its parent page, including none (`patchPage` → `set-page-parent`).

`cargo test -p semio-s-artifact-layout-layout --lib -- create_character_style_appends update_character_style_sets delete_character_style_removes update_parent_page_renames update_spread_renames set_page_parent_clears patch_document_adds_a_character selected_frame_page kinds_match` passed (9 tests).

## Page guides

The active page adds a guide (`patchPage` field `addGuide` → `set-page-guides`, binary tag 38). Each guide edits x, y, width, and height, and can be deleted. A page holds at most 64 guides. Each origin and size must be finite, and the size must be non-negative. The sheet already paints `page.guides` when chrome is on.

`cargo test -p semio-s-artifact-layout-layout --lib -- set_page_guides_adds patch_page_adds_a_guide kinds_match_the_enum pdf_export_carries export_pdf_publishes` passed (5 tests).

## Print path

`pdf_export_carries_every_page_with_the_embedded_font_and_shaped_glyphs` and `export_pdf_publishes_a_segmented_download_the_host_can_drain_into_a_whole_pdf` passed in that same run. The `exportPdf` command still returns `layout-export-job-only`; the PDF bytes come from the export engine the host drains.

## Placed artifacts on the sheet and in print

A placed link with an artifact kind and no proxy paints a mark inside the frame and shapes the kind as glyphs. Drawings, DWG, DXF, and CAD use a diagonal stroke. PDF, note, writer, forms, sequence, and presentation use a page rectangle. Maps and terrain use a cross. SVG, FEM, equation, flow, and wires use a peak. Other kinds use a grid. The same mark is in the PDF content stream, and the kind is one positioned glyph per character.

`cargo test -p semio-s-artifact-layout-layout --lib -- placed_drawing_shapes pdf_export_prints_a_placed_drawing pdf_export_carries_every_page` passed (3 tests). The existing full-document PDF test still matches, because the demo image link has an empty kind.

## Proxy pixels in print

A ready link whose `proxyDataUrl` is `data:image/png;base64,...` is decoded with the stdio PNG codec and painted into the PDF as an inline RGB image, fitted to the frame. Alpha is composited on white. A proxy the codec cannot read stays the flat fill. The host canvas already sends that same data URL as an image layer.

`cargo test -p semio-s-artifact-layout-layout --lib -- pdf_export_prints_proxy_png_pixels pdf_export_carries_every_page` passed (2 tests). The 2×2 proxy is red, green, blue, and white, and the existing full-document PDF test still matches.

A rotated frame turns that image in the PDF. The matrix uses the same center rotation as the sheet. An upright frame keeps the axis-aligned box.

`cargo test -p semio-s-artifact-layout-layout --lib -- pdf_export_rotates_a_proxy pdf_export_prints_proxy_png_pixels` passed (2 tests).

## Rotated proxy on the host canvas

The host raster quad is axis-aligned. A rotated proxy whose PNG decodes is resampled into the rotated bounding box and sent as an upright image layer covering that box. A payload that does not decode stays the flat rotated outline, so the original data URL is not painted upright.

`cargo test -p semio-s-artifact-layout-layout --lib -- canvas_layers_turns_a_rotated_proxy a_rotated_proxy_is_not_painted canvas_layers_paints_a_ready_proxy` passed (3 tests). A 2×1 red-then-blue proxy in a quarter-turn frame comes back 1×2, red on top and blue below.

## Character style runs

A selected text frame chooses a character style (`patchFrame` field `characterStyle` → `set-story-runs`, binary tag 39). The choice covers the whole story. A blank choice clears the runs. A run stays inside the story, on character boundaries, and names a style that exists. Undo restores the previous runs.

The sheet shapes each run with that character style's size and color, and the PDF prints the same size and a fill color when the run is not black. The rest of the story keeps the paragraph style.

`cargo test -p semio-s-artifact-layout-layout --lib -- set_story_runs_applies patch_frame_applies_a_character_style character_style_run_changes pdf_export_prints_a_character_style kinds_match_the_enum pdf_export_carries_every_page` passed (6 tests). The first five characters of "Hello layout" print at 24pt in red; the rest stay 12pt black. The full-document PDF test still matches.

## Link print details

A selected image frame edits the link pixel width, pixel height, resolution, and color profile (`patchFrame` to `update-link`, binary tag 40). Resolution must be from 1 to 9600 dpi. Width and height stay positive and inside the print envelope. A blank color profile clears it. Undo restores the previous values.

Raising a 72 dpi link to 300 dpi removes the low-resolution preflight warning. Switching that link from RGB to CMYK on a print document removes the RGB-in-print warning.

`cargo test -p semio-s-artifact-layout-layout --lib -- update_link_raises patch_frame_sets_link_resolution kinds_match_the_enum` passed (3 tests).

## Still open

The source artifact is not opened as a live editor inside the frame.
