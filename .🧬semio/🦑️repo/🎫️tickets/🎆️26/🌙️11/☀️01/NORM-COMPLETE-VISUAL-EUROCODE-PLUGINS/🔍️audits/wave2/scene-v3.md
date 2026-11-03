# SCENE v3 report (wave 2, nothing compiled, no cargo/build run)

Sub-reports: `scene-v3-f1.md` (en1991, en1995), `scene-v3-f2.md` (en1993), `scene-v3-f3.md` (en1994 slip arrows), `scene-v3-f5.md` (en1992 crack pattern).

## 1. Persisted `placements` (id, x, y, z, yaw; last field of each snapshot)
Tool `🧰️scripts/placements-adder/` (`padd.py` + `en1990/91/93/94/95/96/97/98.py`, idempotent, `--dry`). Additive padd changes: `new_model` naming for prefixed models, a `lines` JSON form, and an import-guard fix.
| Artifact | State |
|---|---|
| en1990, en1996, en1997 | done in v2 |
| en1991, en1995 | done (F1): SQLite 12 / 9 tables, schemaBytes recomputed, TS empty-collection test 5 -> 9 |
| en1993 | done (F2): 35 tables, schemaBytes 46049, admission field 26, binary/text io, 1544 fixtures |
| en1994 | done: `Placement` model, twins, positional diff lane `En1994PlacementsRows/Patch/Inserted` (new `🧬️schema/🔺️diff/📍️placement-rows`, mounted in `DIFF_LEAVES`, apply/absorb/inverse/is_empty wired) + 4 diff twins, SQLite table `en1994_placement` (12 tables, admission field 9, rs/ts/tests, schemaBytes via the `check_schema` formula), field-meta EN+DE, outline, fixtures |
| en1998 | done: `En1998Placement`, keyed `En1998PlacementDelta` + twins, SQLite `en1998_placement` (17 tables), field-meta EN+DE, outline, duplicate-id check, 565 fixtures |
| en1999 | has it (flat id,x,y,z,yaw, `list_delta!` lane): same shape |
| en1992 | DIFFERENT SHAPE: pose is inline (`members[].placementX|Y|Z|Yaw`, `anchors[]...`), no `placements` collection. Not converted (would remove 8 leaf kinds); instead `stored_placements`/`store_placements` in the scene layout, and the editor/viewer viewport call sites use `stored_placements` (previously `placements_of` was always empty for en1992) |
The placements field was put LAST in en1994 so the DSL field ids used by the SQLite admission (`n::field(root, 5..8)`) did not shift.

## 2. Law tests
`dragging_an_entity_commits_one_undoable_event_and_the_reload_stands_at_the_same_pose` (drag -> one `PlacementCommit` -> fold into the document -> same pose from the layout -> repeated release is empty -> inverse restores) appended to the scene unit tests of en1990, en1992 (inline-pose adapter), en1991, en1993, en1994, en1995, en1996, en1997, en1998, en1999. Scenes already read rows through `placements_of`. Open for shared: nothing turns a drag on an entity without a row into `PlacementCommit::of` + insert in the host (en1991/en1995 offer move grips only for entities with a row).

## 3. Visuals
- EN 1993: `🎬️scene/🧩️detail-sketches` panel effective width, Whitmore gusset zone, eaves haunch (3D + diagram + table twin, 4 tests); haunch proportion and 30 deg spread are drawing rules.
- EN 1994: `🎬️scene/🧷️slip-arrows` (layer `slip`, <= 12 arrows per beam, length/colour = |delta|/delta_uk, delta_uk = 6 mm, from the slip-distribution diagram inputs).
- EN 1992: `🎬️scene/🪨️crack-pattern`, diagram `<member>/crack-pattern` (lines at s_r,max, w_k from the crack check stations, EN+DE legend, table twin). 3D crack triangles are not yet width-toned.

## 4. SQLite check
en1996 empty-walls `tables.len()` was 5 (stale even before placements); the database always carries every DDL table, so it is now 13 (= 12 + placement). en1998 17, en1994 12 follow the same rule.

## Unverified / follow-ups
Uncompiled everything. Scene golden re-bless (en1992, en1993, en1994), DSL asset/pack regeneration, `schema-document-index.json`, leaf-kinds `add/remove-placement` + `change-placement-x|y|z|yaw` (en1994 index-addressed). Editor viewport en1992 edit overlaps SHARED v3's `live_scene!` switch (only the `placements_of` lines and the import changed). en1994/en1995 text-io diff twins are stale copies and were not touched.
