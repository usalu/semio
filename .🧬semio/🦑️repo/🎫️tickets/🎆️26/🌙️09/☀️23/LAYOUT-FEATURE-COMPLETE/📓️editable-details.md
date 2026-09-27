# Editable layout details

The inspector was a read-only summary. Catalogue placement already covered png, pdf, drawing, raster, and a few peers, and stopped there.

## This pass

The inspection panel keeps the read-only summary, then edits:

- Document: name, print target, data fields (`patchDocument` → `rename-layout`, `change-print-target`, `change-data-fields`).
- Active page: name, width, height, four margins, column count, gutter (`patchPage`).
- Selected frame: x, y, width, height, rotation. Rectangles also edit fill and stroke (color input, `#rrggbb` keeps the previous alpha). Text frames edit columns, wrap mode, and story body. Image frames edit the link path and show the linked artifact kind.

The catalogue now also places note, writer, forms, flow, equation, puzzle 2d, block 2d, generation 2d, terrain, grid 2d, wfc 2d, presentation, sequence, and wires. Each drop still stores `artifactKind` and `artifactRef` on an image-frame link and paints a typed mark when there is no proxy.

`cargo test -p semio-s-artifact-layout-layout --lib -- patch_document_renames selected_frame text_to_rgba catalogue_roster command_ids the_inspector layout_inspection window_kind_actions every_printed every_placeable every_command_round` — the focused set passed, including `patch_document_renames_sets_print_target_and_clears_it` and `selected_frame_page_and_document_expose_edit_inputs`.

## Still not an end-user editor

These document fields have no inspector control and no command, because there is no semantic mutation for them yet: grid (baseline, offset, snap), paragraph and character styles, layers (name, visible, locked), frame locked/visible, text inset and threading, parent pages, spreads, guides, and which parent a page uses.

Placed 2D artifacts are links with a preview mark or a proxy image. They are not live embedded editors of the source artifact.

Page and frame deletion, page reorder, and print export already exist as commands or mutations, but deletion of a page and reordering are not on the inspector. Print was not re-run in this pass.
