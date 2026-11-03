# SCENE v3 / F2: en1993 (nothing compiled)

## 1. Placements
Driver `🧰️scripts/placements-adder/en1993.py` (idempotent, uses padd.py only): model + root/snapshot twins (rs/ts/graphql/proto/json "lines" form), diff lane `En1993PlacementDelta` + 4 twins, inferences reads/outline/duplicate ids, field-meta EN+DE, binary/text io (import lists, ts interface, graphql type), SQLite (sql, rs, ts, admission field 26, rs/ts tests), 1544 fixtures.
SQLite: 35 tables (was 34), schemaBytes 46049 (check_schema formula), width `en1993_placement` 16; tests `max_tables` 34->35 / 33->34. Pre-existing stale `tableNames` fixture (25 of 34 tables) regenerated from the DDL (35).

## 2. Scene read + law test
Scene already reads `placements_of(document)` (editor/viewer unchanged); the persisted rows now exist. Law test `dragging_an_entity_commits_one_undoable_event_and_the_reload_stands_at_the_same_pose` (scene unit tests): drag -> one `PlacementOp` -> folded into `document.placements` -> `placements_of` -> `Layout` pose equals target -> second identical drag is empty -> inverse restores.

## 3. Visual (new module `🎬️scene/🧩️detail-sketches`, hooks: scene `🦀️.rs` mount + `project` line, `📈️diagrams` one line)
- Effective width: plated panel in uniform compression (psi=1) via `effective_width::effective_internal`; 3D overlay `<panel>:ineffective`, diagram `effective-width:<panel>` (plan, edge zones, hatched ineffective strip), table twin. Worked: pp-1 (600 mm, 12 mm, 355 MPa): rho 0.73633, b_eff 441.80 mm, b_e1 220.90 mm.
- Gusset: Whitmore 30° spread through bolt rows; 3D `<joint>:gusset`, diagram `gusset:<joint>`; worked 60 mm -> 129.28 mm.
- Haunch: length 0.1 span, depth = beam depth; 3D `<joint>:haunch`, diagram `haunch:<joint>`, only when `beamSpan > 0`.
Tests: four new unit tests (values above, 3D+diagram presence, table twins, focus).

## 4. en1996 sqlite assertion
`database(&empty).tables.len()` is the DDL table count (tables are created from the SQL, not by rows): was 5 (wrong already at HEAD with 12 tables), set to 13.

## Unverified / open
- Haunch proportion and the 30° spread are drawing rules (labelled so in the diagram descriptions), no stored geometry; real fields would need a field-adder profile for en1993.
- Plate overlay frame handedness for the haunch (x=-beam x, y=-z, z=-y) not run.
- psi=1 is assumed for the effective-width sketch (no stress ratio in the row).
- Scene golden re-bless, DSL asset/pack regeneration, leaf-kinds for placements (see requests.md).
- Appended `- ready for rebuild: scene v3 en1993 ...` to build-en1993/latest.md.
