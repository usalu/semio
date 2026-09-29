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

## Page overrides

A selected frame that belongs to the page parent is edited on the active page (`patchFrame` x, y, width, height, rotation, visible, or locked to `set-page-overrides`, binary tag 41). The master frame stays where it is. The page holds at most 64 overrides. Each override names a frame on that parent, and each box is finite with a non-negative size. Undo clears the overrides back to the previous list.

The sheet and the PDF use the override. Moving the demo master frame from x 50 to x 90 prints `90.000 370.000 100.000 80.000`. Hiding it removes the frame from the sheet. A page with no overrides still prints the master at x 50.

`cargo test -p semio-s-artifact-layout-layout --lib -- set_page_overrides_moves patch_frame_overrides page_override_moves pdf_export_prints_a_page_override pdf_export_carries_every_page kinds_match_the_enum` passed (6 tests).

## Hidden layers

A layer that is hidden drops its frames from the sheet and from the PDF. Frames on another layer stay. The demo content layer holds the story, the image, and the white rectangle. Hiding it prints no story glyphs and omits that rectangle. The master frame stays, because it sits on the parent layer.

`cargo test -p semio-s-artifact-layout-layout --lib -- hidden_layer_drops pdf_export_omits_a_hidden_layer pdf_export_carries_every_page` passed (3 tests). The full-document PDF test still matches while the content layer is visible.

## Frame layers

The active page adds a layer (`patchPage` field `addLayer` to `create-layer`, binary tag 42). A page holds at most 16 layers. Each new layer needs a fresh id and a short name. Undo removes that layer while it holds no frames.

A selected page frame chooses its layer (`patchFrame` field `layerId` to `set-frame-layer`, binary tag 43). The frame leaves the previous layer membership. Undo puts it back. A frame moves only onto a layer of its own page.

Moving the white rectangle onto a hidden layer drops that rectangle from the sheet and from the PDF. The story on the content layer still prints.

`cargo test -p semio-s-artifact-layout-layout --lib -- create_layer_appends set_frame_layer_moves patch_page_adds_a_layer moving_a_frame_onto_a_hidden_layer pdf_export_omits_a_frame_on_a_hidden_layer kinds_match_the_enum` passed (6 tests).

## Open a placed source

A selected image frame whose link names a placeable kind and a document id offers Open (`patchFrame` field `open`). That asks the shell to open `os.open-artifact` for the source editor. The relay carries the kind coordinate, the document id, the document schema, and the frame id. A rectangle, an empty document id, and an unknown kind do nothing. Undo is not involved: the layout document does not change.

`cargo test -p semio-s-artifact-layout-layout --lib -- patch_frame_opens_the_placed_source patch_frame_open_refuses every_placeable_kind_can_open` passed (3 tests). A PDF link `doc-pdf` opens `s.stdio.pdf@1.7/*` with schema `stdio.pdf`.

## Imported drawing on the sheet and in print

A document that keeps a background drawing paints that drawing on every page, fitted to the page, behind the frames. A hidden drawing layer is omitted. The same strokes are in the PDF, under the frames. An offset plan whose box is 100 by 50, sitting at (10, 20), lands on a 100 by 50 page from (0, 0) to (100, 50).

`cargo test -p semio-s-artifact-layout-layout --lib -- background_drawing_fits pdf_export_prints_the_background pdf_export_carries_every_page` passed (3 tests). The full-document PDF test still matches when the document has no background drawing.

## Placed drawing inside its frame

A placed frame whose link is a drawing, DWG, DXF, SVG, or CAD kind, and whose document owns a background drawing, fits that plan inside the frame. The sheet paints those strokes on top of the frame. The PDF prints the same path. A proxy image keeps the pixels and does not also draw the plan. The demo image frame is 60 by 40 at (136, 435); a unit square lands at (146, 435), (186, 435), (186, 475), and (146, 475).

`cargo test -p semio-s-artifact-layout-layout --lib -- a_placed_drawing_frame_fits pdf_export_prints_a_placed_drawing_inside pdf_export_prints_the_background placed_drawing_shapes pdf_export_prints_a_placed_drawing_mark` passed (5 tests). A placed kind with no plan still shapes its mark.

## Locks

A locked frame stays where it is. Move, rotate, scale, and geometry edits do nothing. Unlocking the frame still works. A locked layer blocks every frame on it, including unlock, until the layer itself is unlocked. Another frame on an unlocked layer still moves. The frame stays on the sheet and in the print.

`cargo test -p semio-s-artifact-layout-layout --lib -- a_locked_frame translate_selection_moves rotate_selection_adds scale_selection_resizes` passed (5 tests).

## Drawing text

Text on the imported plan is shaped on the page and again inside a placed drawing frame. A label at the plan origin sits at (0, 50) on the demo page and at (146, 435) inside the demo image frame. The PDF prints the same glyphs.

`cargo test -p semio-s-artifact-layout-layout --lib -- drawing_text_lands pdf_export_prints_drawing_text a_placed_drawing_frame_fits` passed (3 tests).

## Embedded drawing image

A PNG embedded in the imported plan is fitted with the plan. On the demo page it covers (0, 50) at 400 by 400. Inside the placed drawing frame it covers (146, 435) at 40 by 40. The host canvas sends that PNG, and the PDF prints the red pixel in the frame.

`cargo test -p semio-s-artifact-layout-layout --lib -- an_embedded_drawing_png pdf_export_prints_an_embedded_drawing_png canvas_layers_emits_an_embedded_drawing_png` passed (3 tests).

## Drawing text edit

A selected drawing frame edits the plan's first text label (`patchFrame` field `drawingText` to `set-drawing-text`, binary tag 44). The sheet shapes the new words. Undo restores the previous label. A label longer than 256 characters is refused.

`cargo test -p semio-s-artifact-layout-layout --lib -- set_drawing_text_replaces patch_frame_renames_drawing_text kinds_match_the_enum` passed (3 tests). Setting "Plan" to "Title" puts "Title" on the sheet. The host canvas emits that string, and the PDF prints the same glyphs.

`cargo test -p semio-s-artifact-layout-layout --lib -- pdf_export_prints_edited_drawing_text canvas_layers_emits_an_embedded_drawing_png` passed. The embedded plan path is a host stroke (`drawing.plan.0`, fitted origin `(0, 50)`) and again inside the drawing frame (`drawing.frame.0`).

## Plan stroke style

A named drawing style keeps its stroke color, width, and opacity on the sheet and in the print. An unnamed path stays 1pt `[0.15, 0.2, 0.28]`. A style named `red` with stroke red, width 2, and opacity 0.5 paints `[1, 0, 0, 0.5]` at width 2. The PDF uses `1.0000 0.0000 0.0000 RG 2.000 w`.

`cargo test -p semio-s-artifact-layout-layout --lib -- background_drawing_fits pdf_export_prints_a_styled_plan_stroke` passed.

## Plan fill and arcs

A named style with a fill paints that color on the sheet and in the PDF. Opacity multiplies the fill as well as the stroke. An unnamed path stays unfilled. An SVG arc is sampled as quarter-turn cubics, so a semicircle leaves its chord instead of drawing a straight line.

`cargo test -p semio-s-artifact-layout-layout --lib -- pdf_export_prints_a_filled_plan_path an_imported_arc_bulges pdf_export_prints_the_background_drawing` passed (3 tests).

## Rotated drawing frame

A quarter-turn drawing frame rotates the plan in place. The unit square's first corner moves from (146, 435) to (186, 435). The embedded PNG stays in the frame and prints with the rotated image matrix `0.000 -40.000 40.000 0.000 146.000 65.000`. The host canvas still receives a PNG for that frame.

`cargo test -p semio-s-artifact-layout-layout --lib -- rotated_drawing embedded_drawing_png a_placed_drawing_frame_fits` passed (6 tests).

## Every plan label

Each text label on the imported plan has its own inspector field, `drawingText` and then `drawingText.1` through `drawingText.31`. Editing one label leaves the others in place. Undo restores that label only. The sheet shapes every changed label.

`cargo test -p semio-s-artifact-layout-layout --lib -- set_drawing_text patch_frame_renames_drawing_text` passed (3 tests).

## Stack order

A page frame can move one step forward or backward. Later frames paint above earlier ones. Bring Forward swaps the frame with the next frame; Send Backward swaps it with the previous one. A frame already at that end of the stack stays put. Undo walks the same step the other way. A locked frame, or a frame on a locked layer, stays where it is. The inspector shows Bring Forward and Send Backward for a frame that lives on the page.

`cargo test -p semio-s-artifact-layout-layout --lib -- reorder_frame kinds_match_the_enum` passed (3 tests). Two rectangles on the page paint in the new order on the sheet.

The sheet, the host canvas, and the PDF follow that same order across kinds. A rectangle brought in front of an image paints after the image, so it covers the image. Page-level plan strokes stay behind the frames. Hit testing returns the front frame.

`cargo test -p semio-s-artifact-layout-layout --lib -- front` passed, including `a_rect_brought_to_the_front_paints_after_the_image` and `pdf_export_prints_a_front_rect_after_the_image`. `cargo test -p semio-s-artifact-layout-layout --lib -- drawing` passed (23 tests).

## Character style span

A character style can cover part of a story. Style Start and Style End are character offsets. Setting Style End to 5 on "Hello layout" keeps the emphasis on "Hello" and leaves the rest in the paragraph style. The sheet paints those glyphs at the character style's size. Clearing the character style removes the runs. A locked frame still refuses the range edit.

`cargo test -p semio-s-artifact-layout-layout --lib -- limits_a_character` passed (1 test).

## Character style tracking and italic

A character style's tracking and italic reach the shaped sheet and the PDF. Tracking adds space between the glyphs of that span. Italic prints those glyphs with an oblique text matrix `1 0 0.250 1`, and the rest of the story stays upright. Font weight from the character style is passed into the shaper with the span.

`cargo test -p semio-s-artifact-layout-layout --lib -- obliques_and_tracks` passed (1 test).

## Styled story on the host canvas

The host canvas splits a styled story into one text layer per span. The first five characters of "Hello layout" are a layer at 24px in the character color; the rest is a layer at the paragraph size. Tracking shifts where the second span starts. The canvas painter draws text with the layer fill, so that color is the ink.

`cargo test -p semio-s-artifact-layout-layout --lib -- canvas_layers_splits_a_styled_story` passed (1 test). `cargo check -p semio-framework-os-renderer-wgpu --offline` passed after a cache-write retry.

## Still open




The source editor is the shell session for that document. The sheet shows the imported plan, including its styled strokes, fills, arcs, text, and embedded PNG, inside a drawing frame, including when that frame is rotated, and each plan label can be edited from the layout. It does not open that artifact's editor inside the frame.
