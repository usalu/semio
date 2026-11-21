# Round 3: objects/patio-heater

State: DONE (with the open points listed under "Still imperfect").

Resumed on 2026-11-20 from the interrupted session (scripts `r3-patio-heater-*.py` of 02:00 to 02:14 existed, catalog, source and candidate were identical; the bulk passes of 03:40 to 04:10 did not touch this asset: every member outside my own equals `🗑️generated/r3-patio-heater/catalog-before-r3.json`). The catalog declares 1:50, 1:100, 1:200, 1:500; all four are handcrafted.

Scripts (all in the ticket folder): `r3-patio-heater-kit.py` (real dimensions, profiles, bone frames), `-volumes.py` (every solid), `-draw.py` (2D shapes and representation decisions), `-roots.py` (rig, anchors, poses, materials, thermal, structure, provenance), `-texts.py` (volume decisions), `-areas.json`, `-build.py` (candidate, `--source`, `--apply`). Ship chain: `python3 r3-patio-heater-build.py --source --apply`, then `python3 volumes-insert.py volumes/objects/patio-heater.json <catalog file>`.

## Reference

| Item | Value used | Source |
|---|---|---|
| Overall height / dome diameter | 2.25 m / 0.80 m | Amazon Basics 46,000 BTU commercial patio heater, retailer data 32.1 x 32.1 x 91.3 in (0.815 m, 2.32 m); class value kept at the asset's 2.25 m / 0.80 m |
| Base pan | 470 mm diameter, 40 mm high | same product: base 18.4 in (467 mm) |
| Bottle housing | 420 mm diameter, top at 0.73 m, 1.2 mm sheet | same product: base assembly 28.76 in (0.73 m) high |
| Post | 60 x 1.5 mm stainless tube, three sections, gas hose inside | Crown Verity CV3050 owner's manual (three-part post); 57 to 60 mm is the common tube |
| Head | valve housing 170 mm, burner deck, emitter screen 220 mm x 210 mm with cap 240 mm, knob 47 mm, piezo igniter | Crown Verity manual part list; my knowledge of the product class |
| Reflector | one-piece aluminised dome, 1 mm, 150 mm rise, rolled rim, on three studs with spacers and dome nuts | Crown Verity manual (three reflector spacers on studs with wing nuts) |
| Wheels | two 90 mm wheels on a rear axle | Amazon Basics (wheel kit) |
| Bottle | 11 kg propane cylinder, 300 mm x 580 mm with collar, 2.5 mm shell | EN 1442 cylinder class |
| Mass | 45 kg with full bottle (17 kg heater, 13.5 kg tare, 11 kg gas, rest fittings) | Amazon Basics 37 lb empty |
| Heat input | 13 kW high, 6 kW low | 46,000 BTU/h = 13.5 kW, EN 14543 product family |

The retailer and manual data were fetched by the interrupted session (references `amazonbasics-46k`, `crown-verity-cv3050` in `provenance.references`); I did not fetch them again.

## Audit of the state before round 3

- Rig: every bone lay along +X on the ground line (angle 0) with the joint offsets written as if upright: `pole` at (0, 0.62), `burner-head` at (0, 1.36), `reflector` at (0, 0.08), `knob` a child of `base` at (0.17, 0.46), 1.2 m below the real knob. Anchors `flue-vent` (0, -1.86) and `reflector-top` (0, -1.87) were compensating offsets.
- Body 1:50: 22 solids, a square box housing on a plinth, no wheels, no door, a solid pole, the head as stacked cylinders, a faceted dome without rim, studs or finial, no bottle.
- Thermal: an empty housing hull, no bottle region (`qa-visual.md`). Structure: a thin pole line and a cylinder for the bottle.
- 2D: square housing, knob on the housing, no wheel, no door, no guard rods.

## Root fixes (old to new)

- Rig: `base` angle 0 to 90 (upright), length 0.46 to 0.73; `bottle` joint to the pan top (0.04 along the base), length 0.58; `pole` joint 0.73 (lid socket), length 0.93; `burner-head` joint at the post top (0.93), length 0.365; `reflector` joint on the emitter cap (0.365), length 0.225; `knob` now a child of `burner-head` at (0.082, -0.089), pointing ahead, revolute with twist 0 to 120 degrees. Masses and centres of mass per bone rewritten (8.7, 24.5, 4.5, 3.5, 3.5, 0.3 kg = 45 kg).
- Anchors: `flue-vent` (0.365, 0) on the cap, `reflector-top` (0.2, 0) on the crown, `base-contact` footprint 0.47 x 0.47 m.
- Poses: the knob turns by `twistDeg`; every pose carries the new `gas-bottle` region; wind loads of `high-in-breeze` follow the real areas (dome 4 N, head 3 N, post 3 N, housing 10 N).
- Materials: `bottle-steel` added. Thermal: hidden passive region `gas-bottle` (2.5 mm shell, heat capacity of shell plus 11 kg propane). Structure assumptions and provenance notes rewritten.
- `areaM2` (total outer area of the honest 1:50 model, C29): base-housing 1.257 to 1.35, pole-stainless 0.26 to 0.31 (now with the valve housing), emitter-head 0.198 to 0.283, reflector 0.694 to 1.15, control 0.011 to 0.0127, gas-bottle new 0.72. The reflector is a 1 mm dish that the air reaches on both faces: 2 x 0.533 m2 (integrated by hand) plus rim bead, finial, studs = 1.15. These are for the area reviewer to confirm.
- `dimensions` unchanged (2.25 / 0.80 / 0.80 m, 45 kg).
- 2D of all scales and all three views redrawn from the kit's geometry (E1): 1:50 elevation 21 to 40 shapes, front 23 to 25, plan 12 to 8; 1:100 12/11/3 to 19/18/3; 1:200 6/5/2 to 10/8/3; 1:500 5/4/1 kept in count, redrawn. Representation decisions rewritten.

This session added: the pressed stiffening rib of the dome (3D 1:50 and its 2D strokes; the former "contour ring" of the plan had no counterpart in the model and is gone at 1:100), the access panel 3 mm proud (at 1.5 mm it interleaved with the wall facets and showed stripes), outline strokes for finial and latch so that the line views show them, the tapered structural valve housing at 1:200, the volume decisions, the reflector area.

## What is modelled

| Scale | Body | Thermal | Structure |
|---|---|---|---|
| 1:50 | 44 solids: pan with rounded edges, sheet wall, pressed lid, socket, two tyres with hubs, axle, two brackets, access panel with 12 louvres and latch, post tube with two couplings, valve housing, screen tube, two bands, cap, eight guard rods, knob boss, igniter, knob with grip, dome shell, rim bead, rib, finial, three studs with dome nuts | 34: envelopes of housing, post, emitter; references to every open sheet; bottle (foot ring, body, collar, valve); flue and plume outlet patches | 23: pan, wall, lid, socket joint, ground pad, bottle shell and foot, post, two couplings, valve housing shell, head sleeve, guard rods, cap, dome, studs and stud joints, knob stem, five masses |
| 1:100 | 17, authored anew for 15 mm | 19 | 21 |
| 1:200 | 6: one housing sheet, post, one head, knob, dome with finial, stud ring | 9 | 15 |
| 1:500 | 5: one solid per bone | 8 | 14 |

Final checks: `volumes-check.ts objects/patio-heater --product` PASS at 1:50, 1:100, 1:200, 1:500; `r3-validate.ts objects/patio-heater --volumes`: `VALID objects/patio-heater (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only` VALID; catalog equals the candidate document.

## What I saw in the final renders

Opened after their last change: `🗑️generated/r3-sheet/objects/patio-heater.s50.body.png`, `.s50.thermal.cut.png`, `.s50.structure.cut.png`, `.s50.body.three-quarter.png` (turn 300), `.s50.body.back.png`, `.s50.body.off-afternoon.front.png`, `.s100.body.png`, `.s100.thermal.cut.png`, `.s100.structure.cut.png`, `.s200.body.png`, `.s200.thermal.cut.png`, `.s200.structure.cut.png`, `.s500.thermal.cut.png`, `.s500.structure.cut.png`, and `🗑️generated/r3-2d/objects/patio-heater.s50.png`, `.s100.png`, `.s200.png`, `.s500.png`. The 1:500 body sheet was opened before the last changes; the 1:500 solids were not changed afterwards.

- 1:50 body: front and side read as a commercial mushroom heater at first sight; the rig runs up the axis with joints at pan, lid socket, post top, cap and knob. Rear: wheels, axle and brackets sit on the pan rim. Side: access panel with louvre rows and latch, clean after the 3 mm change. Top and three-quarter: rim bead, rib, three dome nuts, finial. Pose `off-afternoon`: knob turned, everything closed.
- 1:50 thermal cut: the bottle stands in the housing on the pan with collar and valve; post, head, dome coloured by region. Structure cut: sheet housing, post tube, shells, studs, masses at the centres of mass, green pad under the pan.
- 1:100: the same object with merged parts; guard rods as slender rods; bottle and masses present.
- 1:200: one head, a solid post, stud ring; structure with the true tube and a tapered housing shell.
- 1:500: a mushroom sign, one solid per bone; the outlet patches are 75 mm thick by rule D2 and stand visibly on cap and crown.
- 2D: elevation, front and plan agree with the model at all four scales; the plan line view now shows rib and finial.

## Still imperfect

- The emitter screen is a plain tube; its perforation (3 mm holes) is below every feature size and exists only as the mesh hatch of the 1:50 drawing.
- Gas hose, regulator, burner venturi and pilot inside housing, post and head are not modelled in any list (only the bottle is); the thermal model has no region for them.
- The housing is the access-panel type that carries the post through its lid; the common lift-up housing with three post brackets on the pan would have a different load path. The choice is stated in `structure.assumptions`.
- 1:500 structural valve housing is a plain cylinder inside the conical body head; the inner corners of the 1:200 shell are chamfered by the offset.
- The reference dimensions rest on retailer data and one manual plus my knowledge; no manufacturer drawing was available, and the area changes above await the area reviewer.
- The clips were validated but no clip frame was rendered; only the default pose and `off-afternoon`.

## Round 5 handcraft (2026-11-21)

Scripts: `r5-patio-heater-kit.py`, `-volumes.py`, `-draw.py`, `-roots.py`, `-texts.py`, `-areas.json`, `-build.py` (the round-3 chain carried over; `python3 r5-patio-heater-build.py --source --apply`, then `volumes-insert.py`). The r5 chain reproduced the catalog byte for byte before any change. Backup: `🗑️generated/r5-patio-heater/catalog-before-r5.json`.

### Reference (modelled to)

| quantity | reference | source |
|---|---|---|
| overall height | 2.32 m (class 2.25-2.32 m) | Amazon Basics 46,000 BTU commercial heater, 91.3 in |
| reflector diameter | 0.815 m | same, 32.1 in |
| base pan | 0.467 m | same, 18.4 in |
| housing height | 0.73 m | same, base assembly 28.76 in |
| post | 57-60 mm stainless tube, three sections | Crown Verity CV3050 manual |
| control knob | about 47 mm | product class |
| bottle | 11 kg propane, 300 x 580 mm | EN 1442 class |

### Model against reference (1:50)

| measure | model | reference | deviation |
|---|---|---|---|
| height | 2.25 m | 2.32 m | -3.0 % (class value of the asset kept; at the limit) |
| reflector | 0.80 m | 0.815 m | -1.8 % |
| pan | 0.47 m | 0.467 m | +0.6 % |
| housing height | 0.73 m | 0.73 m | 0 % |
| post | 60 mm | 57-60 mm | 0 % |
| knob | 47 mm | 47 mm | 0 % |
| 1:500 knob projection from the valve housing | 49 mm (75 mm boss, half sunk) | 47 mm knob on its boss | +4 % (75 mm size floor) |

### Grades (opened: body, thermal cut, structure at 1:50, 1:100, 1:200, 1:500; poses high, low, off-afternoon, high-in-breeze; clips ignite-and-warm-up t 360 s, turn-down-and-shut-off t 720 s; 2D 1:50 to 1:500)

| scale | body before → after | thermal | structure |
|---|---|---|---|
| 1:50 | A- → A- | A- | A- |
| 1:100 | A- → A- | A- | A- |
| 1:200 | A- → A- | A- | A- |
| 1:500 | B (knob a 75 mm cylinder sticking 69 mm out of the head; housing lid a cone rising 55 mm to the post) → A- | B+ → A- (housing envelope follows the new lid) | B+ → A- |

### Changes

- 1:500 knob: a rounded boss of 75 mm (the size floor) from -30 to +45 mm along the knob bone, half sunk into the valve housing, with a rounded face (body and the elevation 2D, which takes the same profile).
- 1:500 housing: the lid has the rounded shoulder and nearly flat top of the pressed 1:50 lid (0.695/0.225, 0.718/0.21, 0.73/0.18, 0.735/0.075 m) instead of a cone; the post socket rises from it to 0.755 m. Body, structure (the same profile) and the 2D of all three views changed together.
- Decisions and the 1:500 silhouette text rewritten in both languages.
- `r3-validate.ts --file` of the candidate with `--volumes`: VALID; catalog written and spliced.

### Remaining

- Height 2.25 m against 2.32 m of the retail reference is 3.0 %; the asset's class value is kept because no data sheet of a 2.25 m model was fetched.
- The emitter screen perforation, hose, regulator and burner internals stay unmodelled (below every feature or hidden, see round 3).
- The 1:500 dome is a four-segment cone shell; the slight kinks read at sheet size, not on paper.

The gas control now measures 47 mm across and 27 mm along its real axis at 1:50, 1:100, 1:200 and 1:500. Its thermally modeled knob, grip and boss remain separate from the 8 mm steel spindle. Three.js measured the shaft axis and control dimensions; all four product checks and the twelve-scene export passed, at 2,737,896 bytes.
