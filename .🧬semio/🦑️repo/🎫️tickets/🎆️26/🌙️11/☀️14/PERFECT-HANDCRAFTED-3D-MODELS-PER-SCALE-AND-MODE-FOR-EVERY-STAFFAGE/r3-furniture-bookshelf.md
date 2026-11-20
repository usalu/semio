# Round 3 — furniture/bookshelf

State: DONE (2026-11-20), with the open points of section 7. No earlier round-3 work existed (no `r3-bookshelf-*` scripts, no report, no `🗑️generated/r3-bookshelf/`); the catalog `volumes` were identical to `volumes/furniture/bookshelf.json` at the start (the bulk passes of 03:40-04:10 had not changed this asset's volumes). The asset was rebuilt from the roots; everything below describes the catalog file as it is now.

Scripts (all in the ticket folder): `r3-bookshelf-kit.py` (measures, helpers, masses), `r3-bookshelf-model.py` (every solid of every scale and list, decisions), `r3-bookshelf-roots.py` (rig, anchors, material, areas, texts, 2D of all scales, symbol), `r3-bookshelf-build.py` (candidate; `--apply` writes the roots into the catalog file and the volumes source). Ship chain: `python3 r3-bookshelf-build.py --apply && python3 volumes-insert.py volumes/furniture/bookshelf.json "<catalog file>"`.

## 1. Reference

| item | value | source |
|---|---|---|
| overall | 0.80 x 0.30 x 1.80 m, 34.9 kg | asset `dimensions` (unchanged); IKEA BILLY 80 x 28 x 202 / 106 cm as the type (from my knowledge of the product and its assembly instructions, not fetched) |
| board | 18 mm melamine-faced chipboard P5, 650 kg/m3 | asset material, EN 312 |
| carcass | side panels run down to the floor; top, fixed bottom between the sides; kick board under the bottom, set back; back of 3 mm hardboard in grooves | BILLY construction (knowledge) |
| carcass joints | cam-and-bolt fittings: 15 mm cam drum, 12.5 mm deep, drilling distance 34 mm, bolt 7 mm | Häfele Minifix 15 (knowledge) |
| back groove | 4 mm wide, 8 mm deep, 10 mm in front of the rear edge | usual carcass practice (knowledge; authored value) |
| shelf supports | 5 mm steel pins with collar in System 32 rows: 32 mm pitch, first row 37 mm behind the front edge | asset reference `system-32` |
| shelves | 762 x 280 x 18 mm (1 mm play each side), 4 pins each | rig section "280 x 18 mm", I = 1.3608e-7 m4; BILLY shelf 76 x 26 cm for 28 cm depth |
| shelf pitch | a multiple of 32 mm | System 32 |

WebSearch was not used; the two new provenance references (`ikea-billy-80`, `haefele-fittings`) are cited from knowledge and say what they are taken for.

## 2. Audit of the state before round 3

- **3D mirrored in depth**: the symbol note and the 2D say the back panel is at +z and the open front at -z; the solids had the back at z = -0.146, so the product elevation (seen from -z) looked at the back of the unit. All validators were green because the outline is nearly symmetric.
- **Construction wrong**: sides started at y = 0.06 and the carcass sat on a solid plinth block 0.772 x 0.075 x 0.25 m (model-findings: 0.539 m2 against a 0.147 m2 kick board). No real bookcase of this type is built so; mass 22.4 kg of the frame bone only adds up with sides of 1.80 m.
- **Shelf heights off the System 32 raster**: undersides at 0.36, 0.66 ... 1.56 m, pitch 300 mm = 9.375 holes, while the asset cites the 32 mm raster and re-pins by 4 x 32 mm.
- **Rig**: `ground.lengthM` 0.8 (a floor reference with a length), `frame.lengthM` 1.8 along x (the bone stuck 1 m out of the 0.80 m unit), prismatic travel +-0.2 m (not a multiple of 32).
- **Anchors**: "plinth feet" at x = 0.06 / 0.74 with a 0.1 x 0.28 m footprint, on a plinth that does not carry.
- **2D**: sides from 0.06, top across the sides, plinth 0.25 m deep in the section, back drawn 18 mm thick, pins drawn as 20 x 13 mm blocks, 1:100 boards thickened to 22 mm, plan lines "front-edge / back-edge" at +-0.132 that match no part, 1:500 plinth narrower than the carcass.
- **Solids**: back 7.5 / 15 / 30 / 75 mm thick, pins 13 mm studs of chipboard, 1:200 and 1:500 the 1:100 list with fatter boards (qa-visual: B), structure = body references plus one wedge support and a mass block.

## 3. Root fixes (catalog file, outside `volumes`)

| member | old | new |
|---|---|---|
| `materials` | 2 | + `zinc-plated-steel` (pins, cam fittings; ISO 10456 steel values, emissivity 0.25) |
| `rig` ground `lengthM` | 0.8 | 0 |
| `rig` frame `lengthM`, `mass.comY` | 1.8, 0.9 | 0.8 (across the width, the axis of the `books-base` load), 0.891 (from the boards); mass 22.4 kg confirmed by the board volumes |
| `rig` shelf-1..5 `y` | 0.36, 0.66, 0.96, 1.26, 1.56 | 0.352, 0.64, 0.928, 1.216, 1.504 (pitch 288 mm = 9 x 32) |
| `rig` shelf-N joint travel | +-0.2 | +-0.192 (6 holes) |
| `anchors` | `plinth-left` x 0.06, `plinth-right` x 0.74, footprint 0.1 x 0.28 | `foot-left` x 0.009, `foot-right` x 0.791, footprint 0.018 x 0.30 (the floor edges of the side panels); renamed in every pose, clip and `structure.supports` |
| `structure.assumptions[2]` | "rigid body on the plinth feet" | stands on the floor edges of the side panels, back braces against racking |
| `thermal` `carcass.areaM2` | 2.952 | 3.26 (hand: 2 sides 2 x 1.1556 + top and bottom 2 x 0.4859 - 4 butt patches 0.0216) |
| `thermal` `shelves.areaM2` | 2.294 | 2.32 (5 x (0.762 x 0.28 x 2 + edges 0.0375)) |
| `thermal` `plinth.areaM2`, title | 0.153, "Plinth" | 0.121 (kick board 0.764 x 0.06 x 0.018: 0.0917 + 0.0275 + 0.0022), "Kick board of the plinth" |
| `provenance` | | 2 references added, notes rewritten to the construction |
| `representations` 50/100/200/500 | | geometry of all three views and all eight decision texts redrawn (E1), see below |
| `symbols[0]` | | geometry and description redrawn to the construction |

All area changes are within the factor 1.5 of C16 and come from the hand calculation of the corrected object; `back-panel` 2.616 stays. `dimensions` unchanged.

2D as redrawn: elevation = open front (sides 0..1.80 m, top and bottom between them, kick board between the sides, back visible between the boards, shelf halves on their bones, pins 9 x 5 mm at true size at 1:50); front = section A-A at mid width (pale side panel, cut top, bottom, 3 mm back 10 mm in front of the rear edge, 18 mm kick board 30 mm behind the front, cut shelves 280 mm deep, the two pins of each shelf beyond as 5 mm circles at 1:50); plan = top between the two side ends. 1:100 boards are now drawn at their true 18 mm (the validator accepts it). 1:200 / 1:500: carcass block to the floor, kick board between the feet, one stroke per shelf on its carrier bone `shelf-N` (was two strokes on the half bones).

## 4. What was modelled (listed / expanded)

- **1:50** body 30 / 44: two side panels as extrusions of their plan section with the 4 x 8 mm back groove; top and bottom as extrusions with the groove; 3 mm back sitting 7.5 mm in the grooves; kick board; 10 shelf halves (1 mm play, the rig bends a shelf about its supports); 20 steel pins (8 mm in the hole, 1 mm collar, 9 mm under the board; rows 37 mm behind the front and 224 mm further); 8 cam drums in the undersides of top and bottom. Thermal 16 references (carcass 4, shelves 10, back, kick board; fittings belong to no region). Structure 37 / 41: boards as frame and beams, pins as members of the carrier bones, cam drums + 8 bolts as joints, two floor strips, carcass mass at 0.891 m.
- **1:100** body 16: plain boards, back as 3 mm sheet between the sides, no fittings. Thermal 16. Structure 33 / 47: + a 15 mm block under each board end at both rows, a 30 mm block in each inner carcass corner at the fittings.
- **1:200** body 8: sides, top and bottom merged into one carcass frame of 30 mm boards with the kick opening, 30 mm back slab, kick board, one 30 mm board per shelf. Thermal 8. Structure 26: + two beam halves per shelf on their own bones, a cleat under each board end.
- **1:500** body 8 (decision starts "Minimal:"): the same abstraction with 75 mm boards. Thermal 8. Structure 26.

Final check lines: `volumes-check.ts furniture/bookshelf --product` PASS (silhouette IoU 1 / 1 / 1 at 1:50 and 1:100, front 0.998 at 1:200 and 0.988 at 1:500; areas at 1:50 carcass +3.2 %, shelves 0.05 %, back +3.0 %, kick board 0.3 %; totals 0.8 % and 0.3 % at 1:200 / 1:500). `r3-validate.ts furniture/bookshelf --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Spliced.

## 5. What the renders show (opened with the Read tool)

- 1:50 body sheet (six views) and two close three-quarter views (front turn 25, back turn 205): a flat-pack bookcase at first sight: sides to the floor, recessed kick board, bottom, five evenly spaced shelves, back panel set in from the rear with the groove shadow all round, plinth void open at the back. The mid-span seam of each shelf is visible (two rig halves).
- 1:50 pose `shelf-adjusted`, elevation: all shelves sag closed at mid span, shelf 3 sits 128 mm higher with its pins under it.
- 1:50 thermal (cut) and structure sheets; 1:100 structure (sheet and three-quarter); 1:200 body; 1:500 body and structure: each reads as the open shelf unit; supports are green strips under both sides, the mass sits in the third compartment.
- 2D of the candidate at 1:50 and 1:200 (`r3-2d.ts`): as described in section 3.
- Not opened: 1:100 body and thermal sheets, 1:200 thermal and structure, 1:500 thermal, 2D at 1:100 and 1:500 (rendered without error; judged only through the checks).

## 6. Decisions worth knowing

- The 32 mm hole rows (5 mm holes = 0.1 mm at 1:50), edge banding, dowels and a wall restraint are not modelled; said in the decisions.
- Pins, cam drums and the 3 mm back are below the 7.5 mm feature and kept at true size; `volumes-feature` accepts them (its limit follows the thinnest drawing, 3 mm).

## 7. Still imperfect

- 1:500 (and less so 1:200): boards of 75 / 30 mm make a heavy caricature of an 18 mm carcass, and the 75 mm cleats under the shelf ends in the 1:500 structure read as brackets. This follows the smallest-feature rule; I would grade 1:500 body and structure B+, not A.
- The carcass frame of 1:200 / 1:500 (one extrusion with a hole) shows a soft shading gradient on its flat faces in `r3-sheet`; geometry is flat, it is the sheet's smoothing.
- 1:50 structure: the cam joints lie inside the boards and are invisible without the ghost; the list is otherwise the boards themselves (D3 allows it for carcass furniture, but it looks like the body in blue).
- References for BILLY and Häfele are from knowledge, not fetched.
