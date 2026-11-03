# W3-EDITOR-ACTIONS report (2026-11-03)

Static work only (no cargo, no repo run). freeze.txt absent at every edit batch. All code is shared, in `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/`; no per-artifact code was touched.

## What already existed (wave 2, verified by reading)
- `entity-editing/clipboard`: `copy_rows`, `paste_edits`, `duplicate_edits`, fresh document-unique ids (`a-copy`, `a-copy-2`), reference rewrite among copied rows, clip as JSON (works across documents because the clip is text).
- Commands `copyEntities/duplicateEntities/pasteEntities` and `moveEntity`: wired through `norm_live_commands!`/`norm_live_editor!` into all ten editors (tool ids, EN+DE action labels and descriptions, palette actions); `live_entity_edit` resolves a whole paste/duplicate into ONE emit (one undo step).
- Chords Ctrl+C/V/D, Select all, undo/redo (`shortcut_route`), long-press touch menu (Copy/Paste/Duplicate/Select all, EN+DE), bulk edit (`bulk_writes`, `bulk_delta_writes`, `bulk_rows` "Mixed"/"Gemischt", one `setFields` bundle), outliner reorder.
- All of it is schema-agnostic: it works on the document value tree by collection path and id, so any collection in `ENTITY_KINDS` (via `insert_specs()` -> `live_palette()`) is covered with no registration beyond the kind row.

## Gaps found and fixed
1. Offset placement was never applied: `live_entity_edit` passed `&[]` offsets, so a duplicate sat exactly on its original. Also the persisted pose (`placements[id=ENTITY]`) did not follow the fresh id, so a copy of a moved entity fell back to the derived layout pose. Fix (clipboard): `PASTE_STEP` = 0.5 m, `paste_offsets()` (x and y), `Clip.poses` (the placement rows of the copied rows, carried in the clip JSON key `placements`, so a paste into another document of the same code keeps the pose), `paste_edits` inserts the pose row under the new id with the offset in the same edit batch (documents without a `placements` list take no pose). Both wiring paths now pass `paste_offsets()`; rows with inline `x`/`y` fields are shifted as well.
2. Selection tie bug: an entity id is also the id of its placement row, so `selected_rows` could pick `placements` over the entity collection on a tie and copy/duplicate the pose row instead of the entity. Ties now go to the non-placement collection.

## Law test
`norm_live_acceptance!` (stamped into all ten editors) gained `every_registered_entity_kind_supports_duplicate_copy_paste_and_bulk_edit_as_one_undoable_batch`: for every kind of `live_palette()` (= `ENTITY_KINDS`) except `placements` (rows are poses of other kinds, copied with their owner): row present (inserted by the insert tool when the default document has none and the document must still load), duplicate adds exactly one row with a fresh id and the result loads, `live_entity_edit(Duplicate)` and a paste into another (default) document resolve into a non-empty single emission, `live_copy_entities` works, and at least one field is bulk editable through the edit rules (`bulk_writes` + `resolve_set_fields`). Unit tests added in `entity-editing/tests` for pose-carrying duplicate, cross-document paste, pose-less documents and the selection tie.

## Not done / open
- No desktop right-click context menu exists in the outliner or viewport (only the long-press touch menu and the palette/chords); adding one needs a framework trigger for secondary click, not done here.
- Entities without a persisted placement row (derived layout) get no explicit offset; their copy is laid out by the projection's own default.
- Compile-unverified: `.and_then($crate::object_id_field)`, `$crate::resolve_set_fields`/`$crate::live_copy_entities` root paths, `name.as_str() != PLACEMENTS_FIELD` closure. If the law test fails on a specific kind, the panic message names the kind (typically an insert default that does not load, or a collection with no editable scalar leaf).

"- ready for rebuild: editor actions" appended to all ten `build-en19xx/latest.md`.
