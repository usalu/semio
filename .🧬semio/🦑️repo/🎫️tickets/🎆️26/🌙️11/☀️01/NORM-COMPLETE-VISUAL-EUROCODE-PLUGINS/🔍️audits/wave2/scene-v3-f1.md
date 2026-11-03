# SCENE v3 F1 - en1991 + en1995 persisted placements (nothing compiled)

## Task 1: `placements` collection (id, x, y, z, yaw), flat shape identical to en1990/1996/1997/1999
Tools: `🧰️scripts/placements-adder/en1991.py`, `en1995.py` (idempotent, `--dry`; padd.py untouched). Placed last in struct/twin order so positional record indices of existing fields do not shift.

| Layer | en1991 (template `roofSurfaces`) | en1995 (template `connections`) |
|---|---|---|
| model, root artifact (+From/To snapshot) | done | done (field moved behind `cases`) |
| twins snapshot + root: ts, graphql, proto (65 / 6), json | done (snapshot json by text insertion: file has mixed inline enums) | done (root twins opaque JSON like the file) |
| diff lane rs (patch/delta/key, absorb, commit_onto, is_empty, inverse), ts, graphql, proto, json | done (json by text clone, compact entries) | done (+ removed leftover `nest actions` of the clone) |
| SQLite | table `en1991_placement`, 12 tables, admission field 85, rs/ts code + tests, semantic.json (schemaBytes 13183 -> 13677, width 16) | table `en1995_placement`, 9 tables, admission field 5, rs/ts + tests, semantic.json (6589 -> recomputed, width 16) |
| field-meta EN+DE, outline (field list + entry count), duplicate-id check, struct literals, fixtures (`"placements": []`) | done | done (en1995 fixtures: 224, en1991: 298 touched) |

en1992 and en1999 not touched (they own placements; flat shape unverified by me here).

## Task 2: scene reads + drag
- Scene already read `placements_of(document)` in all editor/viewer viewports; with the field in the model they now get real rows.
- New translate grips `<entity>:move-x|move-y` (writes `placements[id=E].x|y`, quantised 0.01/0.05 m) for entities that own a row: en1991 building/bridge/crane/silo/roofs 2..n (`handles::moves`), en1995 members/connections/cases (`handles::move_handles`).
- Law test per artifact (end of scene unit tests): drag -> `PlacementCommit::of` yields exactly one op -> persisted rows in the document -> `placements_of` reloads the identical rows -> `Layout::of` pose equals the dragged pose -> same release is an empty commit -> the grips write the placement leaves -> a second drag is one `Set` op and its `inverse` restores the previous rows -> inverse of the first commit empties the rows.

## Open / unverified
- Nothing compiled. Rows `ordered(..., 1, ..)`, `row.text(3)`/`real(4..7)` follow the en1996 code. Check `Placement` DSL derive in en1995/en1991 snapshot (copied attrs of the template struct) on first build.
- DSL assets/packs and `schema-document-index.json` need regeneration (first test run / infra).
- `io/📝️text/🔺️diff/{🔗️.graphql,🛰️.proto}` of en1995 are stale copies (miss `ulsEquation`, `cases`), not extended.
- Missing host: a drag on an entity without row cannot create the row (needs app-surface wiring using `PlacementCommit::of`); leaf-kinds need `add-placement`/`remove-placement`/`change-placement-*` for both (requests.md).
- en1995 ts sqlite test `tables.length` 5 -> 9 (old value looked wrong, DDL has 8 tables).
- Concurrent leaf-kinds E touches en1991 collections (sqlite 11 tables in their note); my edits assumed the files as they stood (all regex asserts passed), re-check on merge.
- Task 3 (open visuals) and task 4 are not in this fork's directive.
