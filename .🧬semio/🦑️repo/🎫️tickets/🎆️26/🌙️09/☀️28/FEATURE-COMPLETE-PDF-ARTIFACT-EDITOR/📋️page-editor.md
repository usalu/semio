# Page editor

The ten PDF subset editors share one page surface, `pdf.page`. It paints text, vectors, images, forms, shadings, and annotations, and publishes edits through the existing PDF mutations. `set-page` stays the revisioned text draft. Snapshot details remains the editor for fields that have no page geometry.

Proven by `editor::page::tests` and the 1.7 registered-app laws:

- Text, rectangles, lines, and images can be inserted, moved, resized, restyled, and deleted.
- `set-image` replaces an image's raw samples from hex (gray or RGB) and keeps its placement.
- A form moves by its paint transform.
- An axial shading moves by its coordinates and bbox.
- Document info, page insert, remove, reorder, and size, and annotation contents and rectangle publish as mutations.
- `set-font` inserts a font operator on the addressed text run and adds a standard font when the base name is new.
- `set-outline` appends a bookmark or renames one by index.
- Page rotation (0/90/180/270), crop/bleed/trim/art boxes, user unit, document language, page layout, and page mode publish as mutations. A 90° or 270° rotation swaps the canvas page size.
- Optional content groups, embedded files, named destinations, page labels, mark info, XMP metadata, viewer preference flags, and encryption publish as mutations.
- Moving a mesh shading shifts its decode range and bbox. The vertex bitstream itself stays as stored.
- `set-page` publishes once, refuses a stale revision, and undoes and redoes against the loaded document.

`editor::page::tests` — 11 passed. `editor::` — 83 passed, including the 1.7 registered-app laws.

Still outside the page surface: font programs, output intents, the interactive form, open action, and document id. Those stay on snapshot details.
