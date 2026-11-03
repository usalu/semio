# Catalogue bridge v4 (wave 2): every design case in 3D

Status: UNCOMPILED, nothing built or run except the Python generator. All Rust is unverified by a compiler. `demo-values.md` did not exist; the demo values were read from the per-artifact `🌱️demo/🔣️.json` and the vocabulary examples.

## 1. Schema-first declaration
- `⚖️compliance/🧰️design-case`: `CaseGeometry` is now `{ form: CaseForm, width, length, height, load }` (keys, empty = nominal), new `CaseForm` (Block, Member, Column, Plate, Wall, Cylinder, Connection, Load, Gauge), `GeometryRow`, `geometry_in`. `geometry_m` kept (needs both keys).
- Per artifact `…/🗂️cases/📐️geometry/{🔣️.json,🦀️.rs}`: one row per registry entry (EN 1990: 30, EN 1991: 98, EN 1992: 174, EN 1995: 70, EN 1996: 40; EN 1997 keeps its 6 rows, now with forms). Sole source is the JSON; generator `🧰️scripts/catalogue-bridge-v4/geometry_gen.py` classifies the form from the clause title (word-prefix keywords) and picks the dimension inputs (priority lists over the declared length-unit inputs, valued inputs first) and the load input (force-like unit or load-like name, any numeric for Gauge/Load) from the vocabulary units; hand overrides for EN 1990 (E90.01/08/10/11/15/19/24/27). `case_entry` of all six catalogues reads `geometry::geometry_of(e.id)`; the EN 1995 heuristic (`b`,`h`) and the EN 1997 const table are gone. The EN 1996 module is private (the facade glob re-exports `cases::*`).
- Form counts: EN 1990 Gauge 17, Load 8, Member 3, Block 2; EN 1991 Load 56, Block 11, Gauge 11, Member 11, Plate 5, Connection 2, Wall 1, Column 1; EN 1992 Member 74, Block 42, Gauge 20, Plate 13, Connection 11, Load 6, Column 5, Wall 2, Cylinder 1; EN 1995 Block 22, Member 20, Connection 14, Gauge 5, Plate 4, Column 3, Load 1, Wall 1; EN 1996 Wall 16, Member 9, Block 6, Connection 3, Column 2, Load 2, Cylinder 1, Gauge 1. 37 entries have no number input at all (text-only: they are drawn nominal, movable and coloured).

## 2. Generic drawing (projection kit)
`projection-kit/🗂️design-case/🧊️forms`: `CasePose`, `size`/`nominal`, `driver` (load bar or gauge pillar, 2 m at the vocabulary example), `parts` (member on two supports, cylinder, lapped plates with fastener, gauge pillar with reference mark, load bar), `extent`, `origins` (rows of 10, packed), `handles` and `draw` (scene entities with utilisation binding of the worst check of the case, bilingual names/details, label `id entry - load label value unit`). Grips: width/length (face offset, factor 2), height and load (scalar along Z), `move-x`/`move-y`; each write is `size_binding` (stored unit, unit-aware) or the load driver binding on `cases[id=…].numbers[id=KEY].value`, so one release is one typed `change-case-number`; an unset input has no grip, the placement move always exists. Tests `🧊️forms/🧪️tests/🔬️unit` (size in stored units, nominal fallback, parts, grip tags/maps/paths, gauge, every form finite without input, packing). `test_support::assert_case_laws` is the shared law: finite positive solids, bilingual names, body lists exactly its grips, move grips exist, every write is a case number or placement, finite non-zero maps, limits, bilingual label.

## 3. Scenes
| code | edits |
|---|---|
| EN 1990 | `🗂️cases` module, kit (entity kind `case`, layer, `rows_of` 12, `Out::push_entity`), layout (`cases`, row y = 20), handles (`case_handles`), root (project call, layout stamps), law test `🧪️tests/🗂️cases` |
| EN 1991 (`🎬️scene-members`) | same plus `Rows` 15, `extra` stamp of the case pose, `entity_kind_of`; test in the facade |
| EN 1992 | same; grips go to `out.handles`; law test builds the report from `evaluate_design_case` |
| EN 1996 | same; `case_ref`, private `geometry` module; law test |
| EN 1995 | keeps its own case scene (groups with notch, fasteners, plates, modes; handles on the case numbers); only the registry declaration and a law test (`🧪️tests/🗂️cases`: every demo case in the scene, draggable, grips write only own rows, live result) were added |
| EN 1997 | `keys()`/`body()` read the declared form instead of the id prefix; the 65 other entries stay table rows (not asked) |
- The four editors' insert palette skips `case` (`🧊️viewport`), as EN 1997 does.
- Layers: `cases` ("Design cases", "Bemessungsfälle") in each kit. Role legends untouched (existing goldens unchanged, no document without cases changes).

## 4. Laws (per scene test file)
Every demo case projects (scene validates, finite positive solids), every handle binds a case number or placement (kind `change-case-number` or placement), EN and DE names complete (`assert_scene_laws`, `assert_handle_laws`, `assert_case_laws`), a persisted placement moves the case, every registry entry declares a form with keys that are declared inputs.

## Open / risks
- Nothing compiled: expect small type errors (the new code mirrors the EN 1997 v3 pattern; closures in `forms::handles`, `Out::push_entity` per kit, the test `report` construction per artifact).
- EN 1992 demo-document fixture has 115 cases against 174 registry entries (stale; owner v5): the committed JSON test pins them, the scene law uses `catalogue_demo_cases()`.
- Heuristic forms are plausible, not curated (e.g. T-71/T-72 bolts read as Member); curate in `geometry_gen.py` OVERRIDES or the JSON.
- Entries whose declared inputs have no vocabulary example and no demo value have no grip for that dimension (nominal size).
- Goldens of documents with cases (none today) and the EN 1997 scene tests were not re-run.
