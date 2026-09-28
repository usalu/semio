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
- Output intents, interactive form fields, the open action, the document id pair, and a font program (Type 1, TrueType, CFF, CID CFF, or OpenType hex) publish as mutations.
- Graphics-state alpha and blend mode, tiling patterns, device and separation color spaces, property lists, and simple-font encoding, widths, and ascent publish as mutations.
- Image masks, form content streams, page transitions and display duration, and catalog and trailer name entries publish as mutations.
- Annotation appearances, Type 3 glyph procedures, indirect objects, and mesh vertex bytes publish as mutations.
- `set-page` publishes once, refuses a stale revision, and undoes and redoes against the loaded document.

`editor::page::tests` — 18 passed. `editor::` — 90 passed, including the 1.7 registered-app laws.

The page surface now publishes an edit for every typed PDF collection. Snapshot details still accepts a raw field when an action does not cover that one value.

A canvas pointer-down hit-tests the top object and redispatches `interactionSelect` on the `objects` domain (granularity `object`, merge `replace`, or `invertive` when shift, ctrl, or meta is held). A miss redispatches `clearSelection`. Pointer move and pointer up change nothing. The PDF snapshot is not mutated. All ten editors install that domain after the app definition is built, because the builder validates window interaction refs before `build_definition` returns. `render_with_request_context` paints that selected id as a blue outline on the page canvas. A render that has no interaction view still paints with no outline.

`set-info-field` writes keywords, creator, producer, trapped, and `D:YYYYMMDDHHmmSS` creation and modification dates. `set-page-extra` writes a page thumbnail, page metadata, transparency group, struct parents, an additional action, or a page dictionary entry. `set-annotation-style` writes color, flags, name, appearance state, or the modified time. Viewer preferences now also cover direction, view and print boxes, print scaling, duplex, pick-tray, copy count, print page range, and the non-full-screen page mode.

Still without its own action: annotation border, markup, and optional content; form signature flags, default appearance, and quadding; and the leftover extra dictionaries on info, the viewer, and the form.
