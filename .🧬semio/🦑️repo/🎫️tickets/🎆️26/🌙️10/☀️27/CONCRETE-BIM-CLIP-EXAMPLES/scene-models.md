# Scene Report: 🧩️models (`Beat4_PartialModels`)

## Round 2

Scene file `C/🧩️models/🐍️.py` (full rewrite, 19 narration clauses, en + de). Audit: `audit-r2-models.md`.

### Audit defects fixed

- Numbers (D-N1..N7): every class row of each model is listed and the rows are summed on screen (`5+4+4+1+2+4+2+6 = 28`, `25+2+4+2 = 33`, `1+1+4+7 = 13`); the scene raises if rows and model totals differ. Members come from `translate_structure()` (25 curve + 24 surface = 49), loss counts from `loss_groups` (dropped 40 + kept 26 = the 66 unique objects of beat 1, asserted). The piloti count is labelled "teaching choice" with "the sources disagree". `A1–E5 ×25` is derived from the tags; fire rating and external flag are two rows. q = 6.25 + 1.50 + 2.00 = 9.75 kN/m² is drawn: slab section with 0.25 m and 25 kN/m³, stacked bar sized by value, formula with coloured terms, arrows over the analysis axon that follow the same tracker, and 380.42 m² × 9.75 kN/m² = 3,709 kN.
- Abstract (D-A1..A7): the analysis mesh sits on the real first-floor plan (slab outline, grid bubbles, dimension chain 1.25 | 4.75 ×4 | 1.25, ramp void) with 7 × 5 nodes and 6 × 4 shells derived from the mesh lines; the check plan is the ground-floor plan with poché, curved glass wall, ramp and pilotis in discipline colours; federation leaders end on the same slab point; authors are bilingual teaching roles (`author_label` in the model).
- Layout, sync, German (D-L*, D-S*, D-G*): no overlaps in either language; "render" is listed in the structure omissions; loss elements stay orange; revision term is `rev N` / `Rev. N`; German title is "Modelle teilen ein Schema"; all 19 keys exist in en and de.
- Depth (D-P*): dimension lines with values (0.18 m, 0.125 m, 0.25 m, 4.75 m, 0.30 m), poché and hatch on plan, wall cut and slab sections, ValueTracker + live drawings for counts, wall layer growth, clearance ring, mid-surface idealisation, load build-up, explode 1 → 0.

### Final beat list (19 clauses)

| # | key | what changes on screen |
| --- | --- | --- |
| 1 | whole | axon in three colours, three question cards, 28 + 33 + 13 = 74 entries, 66 unique (8 shared) |
| 2 | architecture | eight class rows light the villa, sum 28 |
| 3 | rooms | three storey plans, six spaces hatched with their areas |
| 4 | buildup | plan cut through the south wall, three layers grow to 0.30 m, LoadBearing FALSE |
| 5 | structure | structure over architecture ghost, rows sum 33, LoadBearing chip with leader to C3, omissions incl. render |
| 6 | services | services over ghost, rows sum 13, 110 m² garden hatch, drops run in the downpipes |
| 7 | schema | 13 classes × 3 models matrix with thumbnails, column totals count up |
| 8 | federation | explode 1 → 0 on one origin and grid, both leaders meet at the slab, GlobalId chip |
| 9 | slab | 408.50 − 28.08 = 380.42 m² in plan, 0.25 m section, two property cards on one GlobalId |
| 10 | owners | ground-floor plan, every line in its model colour, object counts per model |
| 11 | check | clash on the plan, zoom inset with dashed 0.25 m ring and 0.18 m gap |
| 12 | translation | structure axon → analysis axon, three mapping rules with counts |
| 13 | members | mesh on the slab outline: 25 line members, 24 shells, 35 nodes |
| 14 | midsurface | section at C3, solids idealise: member 3.250 + 0.125 = 3.375 m |
| 15 | supports | 25 fixed supports pop in, fixed versus pinned foot |
| 16 | loads | q builds up 6.25 + 1.50 + 2.00 → arrows, bar, formula, total reaction |
| 17 | loss | 12 rows (incl. rooms and services) with leaders, elements turn orange, 40 + 26 objects |
| 18 | properties | C3 as kept (line, section, material, support, GlobalId) versus dropped psets |
| 19 | trace | source GlobalId equal on both sides, one-way translation, 82 records in 13 kinds |

### Shared-file additions (all additive, with tests)

- Model JSON/schema/`partial_model`: `author_label {en, de}` per discipline model; test `test_author_labels_are_bilingual_teaching_roles`.
- Analysis: `IfcSpace` loss rule and `_losses` also scans `spaces` (rooms were missing from the loss list); test extended (`test_losses_name_the_source_element…`).

### Verification

`capture.py` en and de: layout conflicts only the toolkit grid-bubble "object under" items in federation; strict findings 0 / 19 frames in both languages. Remaining non-strict items judged false positives: `0.18 m` label lies fully inside the inset circle (check); `0,125 m` dimension label next to its own extension line (de midsurface); `±0.00` marker placed clear of piloti and bubbles.

### Remaining risks

- Federation keeps toolkit grid-bubble labels over the villa (11 bbox-level "object under" reports, nothing visually overlapping).
- Loss leaders converge on the roof slab; readable but dense.
- Real render timing not measured; narration holds follow the existing `step` mechanism, longest beats 5–6 s of animation.
- The title chip is no longer drawn by `ClipScene.present` (toolkit change by others), not by this scene.

## Round 3

Source: `verify-r2-models.md` (2 blocker, 9 major, 14 minor; the "no title" remark ignored on purpose).

### Changes

- V-B1 plans: `owners` and `check` use the toolkit ground-floor plan as 📊️information does (curved glass wall, entrance hall, garage and boiler room outlines, ramp, pilotis, services), every object in the colour of its model, plus scale bar and north arrow. Section/plan scale bars added to `slab`, `members`, `supports`, `midsurface`, `owners`, `check`; the 4.75 m bay dimension on every plan.
- V-B2 analysis views: `translation`, `loads` and `trace` use `_villa_analysis` = structure axon (slab outline with ramp void, grid bubbles A–E / 1–5) under the analysis members, shells and supports on the piloti grid, with a labelled C3 leader.
- V-M1: translation row reads "1 IfcSlab · SL-1 → 1 IfcStructuralSurfaceMember, meshed into 24 shell elements" (de: "vernetzt in 24 Schalenelemente"); `members` counts 25 line members, 1 surface member, 24 shell elements, 35 nodes.
- V-M2: loss rows with an object count are windows 4, openings 4, walls 4, curtain wall 1, screen 1, ramps 2, flights 4, spaces 6, roof slab 1, services 13 = 40; render layer and ramp void (28.08 m²) are uncounted rows; SL-1 is kept (25 line members + 1 surface member = 26). The scene asserts counted rows do not share objects and 40 + 26 = 66.
- V-M3 (model fix, Edit only): `SP-TERRACE` (roof garden) now belongs to the roof storey (`storey: "roof"`, `IfcRelAggregates` REL-005 → salon only, REL-006 → terrace + solarium). The first-floor plan shows only the salon; the roof plan shows roof garden 110 m² and solarium 70 m² (sourced reference values, whole m²). The `_plan_spaces` toolkit rule already drew the terrace on the roof, so nothing else changed; all model tests stay green.
- V-M4/M5: Ø 0.30 m and 25 kN/m³ tagged assumed on `properties`; `IfcBoundaryNodeCondition` shown with "translations and rotations restrained" and an assumed badge, "fixed" only in the teaching sketch.
- V-M6: architecture beat shows a ramp section inset with the four flights highlighted as inclined members.
- V-M7: midsurface section drawn with architecture layer, level marks, SL-1 and C3 labels and a scale bar; detail frame widened so no label leaves it.
- V-M9: the 82 loss records are drawn as 82 orange squares grouped by the 13 kinds.
- V-m1..m14: "teaching levels, not sourced" on rooms, "teaching values" on the build-up, LoadBearing of the slab unchanged but the conflict note "sources report other footprints" added, brackets removed from all strings (a `text` override turns bracketed remarks into comma clauses), services narrated with the 110 m² roof garden, federation narration includes "In this teaching model", clash inset label clear of the ring and of the plan marker, structure chip shortened with a leader that ends on the C3 top, radiator and boiler labelled with leaders, schema header widths fixed, dashed first-floor rectangle explained as "roof garden opening above".

### Beat list

Unchanged: 19 keys, same order (whole … trace).

### Verification

`capture.py` en and de: strict findings 0 in 19/19 frames in both languages; layout reports only the toolkit grid-bubble "object under" items (federation) and the false-positive dimension labels. Model tests: 138 run, 1 error (`test_source_discovery_skips_browser_runtime`, needs the missing `repo` binary, unrelated). Presentation tests: see final result below.

### Remaining risks

- The roof plan in the first-floor beats shows the dashed roof-garden outline because the toolkit plan draws the terrace hatch only on the roof level; the legend names it.
- `IfcSlab` roof counted as dropped (1) while SL-1 is kept: consistent with the model but visually the roof slab is also a slab.

Presentation tests (final): 14 run; all 13 contract tests pass including the models lifecycle in en and de. The only error is the class teardown guard "BIM authored inputs changed during this focused run" raised because another agent edited `🔗️consistency/🐍️.py` during the 21-minute run, not a models failure.

## Round 4

Coordinator correction (fleet-r2 "Hanging garden vs solarium"): the Round 3 move of the 110 m² garden to the roof was wrong.

- Model (`C/🧱️model/🔣️.json`, Edit only): `SP-TERRACE` is back on the first-floor storey, named "Hanging garden (first floor, open air)" / "Hängender Garten (1. OG, offen)", sourced reference area 110 m²; aggregates REL-005 = salon + hanging garden, REL-006 = solarium only. `SP-SOLARIUM` stays the roof storey's own `IfcSpace`, named "Solarium (roof)" / "Solarium (Dach)", 70 m². The `garden-fit` note names both storeys. No model function, test or case changed with the revert.
- Scene: rooms beat shows the hanging garden hatched on the first-floor plan with its qualifier line "first floor, open air" / "1. OG, offen" and the solarium on the roof plan with "roof" / "Dach"; areas come from the model reference values (110 and 70 m²). Services narration, row and architecture row say "hanging garden" / "hängender Garten (1. OG)". The dashed outline on first-floor plans is the roof-slab opening above the garden and is labelled "dashed: roof slab opening above the garden" / "gestrichelt: Öffnung der Dachdecke über dem Garten".
- Verification: `capture.py` en and de, strict findings 0 in 19/19 frames; rooms (en, de), services (en, de), architecture, members (de) read. Model tests: 124 tests OK; language and source contracts OK; presentation beat/renderer-interface checks OK. `BimClipContract` (a long render) and the 20-minute presentation lifecycle were not rerun in Round 4 (green in Round 3; only labels and one JSON storey changed since).

## Round 5

Source: `final-verify-representations-models.md` (models section: 1 blocker, 4 major, 8 minor).

- M-B1 walls: `owners` and `check` use `_ground_plan` (scale 0.22): the toolkit ground-floor plan in model colours plus an overlay of the real cut walls from `ground_walls` as heavy hatched poché (plot weight 0.05 screen units half-width), the curved glass wall as its double line, and every piloti as a filled circle; zoomed crop read.
- M-M1 piloti count: "pilotis in this teaching model*" in the structure row, `25*` in the schema matrix, owners, translation, members, supports, loss and properties, with the legend "* sources differ: 15–25" (de "* Quellen uneins: 15–25") on each of those beats via `_legend`.
- M-M2 services: narration back to the original ("boiler, radiator, roof drains and downpipes"); the garden row says "110 m² hanging garden, first floor, open air" and "its floor drains are not modelled; the 4 roof drains belong to the roof level", so the garden is no longer attached to roof drains.
- M-M3 headings: `HEADINGS` (en, de) and `_fonts.section_heading` swapped per beat inside `_show`.
- M-M4 stars: every assumed value carries "*" (levels +3.50 / +6.70, decimal room areas, build-up 0.03 / 0.24 / 0.03 / 0.30 m, piloti height 3.250 m, slab 0.25 / 0.125 / 3.375 m, Ø 0.30 m, 25 kN/m³, 6.25 / 1.50 / 2.00 / 9.75 kN/m², required clearance 0.25 m, boundary condition) with the legend "* assumed in the teaching model"; the word tags "assumed", "angenommen", "Lehrwerte" are gone.
- Minors: "belegt" → "Quelle" (rooms, owners; the slab cards no longer use the toolkit basis column and show `Qto_SlabBaseQuantities.NetArea`, `Pset_SlabCommon.LoadBearing` starred); one radiator in the narration; the trace shows a 13-bar chart (one labelled bar per loss kind, heights = record counts, sum 82) instead of unlabelled squares; German translation chip "IFC4X3_ADD2 · Strukturanalyse-Domäne"; ramp flights labelled in the section; own grid chains with the 1.25 labels raised above the extension lines; the 0.18 m leader starts clear of its label.
- Verification: `capture.py` en and de: strict findings 0 in 19/19 frames in both languages; changed frames read (owners, check, rooms, structure, schema, services, slab, midsurface, properties, loads, loss, trace, translation; de owners, check, trace, services, translation). Tests: 124 model-contract and language/source tests OK; presentation lifecycle for `models` in en and de, dispatch, analysis-beat and renderer-interface checks OK (`SEMIO_TEST_BIM_TOPICS=models`).
