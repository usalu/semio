# appliances/kitchen-extractor-hood — rework report

## Pass 1 (2026-11-22)

Chisel: `rework-kitchen-extractor-hood-model.py` (`candidate` writes the source and a candidate document, `publish` writes `rig` and `representations` into the catalog file; volumes go in with `volumes-insert.py`).

### Reference table (wall chimney hood, 90 cm class, Bosch DWP96BC50B data sheet plus trade knowledge)

| Feature | Reality | Model before | Model after |
|---|---|---|---|
| Canopy footprint | 900 x 500 mm | 0.90 x 0.50 m | same |
| Canopy height without / with spigot | 246 / 319 mm | 246 mm, no spigot | 246 mm plus spigot 73 mm (1:50) |
| Rim band | 40–50 mm, hemmed 10–12 mm | 45 mm, 12 mm return | same |
| Top plate | about 260 x 200 mm | 0.26 x 0.20 m | same |
| Front slope / side slopes | about 34° / 32° from horizontal | exact trapezoids | same |
| Sheet gauge | 0.5–0.7 mm stainless | 0.6 mm at every scale | 0.6 mm at 1:50, plates of 15 / 30 mm and masses at 1:100 / 1:200 / 1:500 |
| Chimney lower / upper | 250 x 190 mm, upper 6 mm narrower, 553–730 mm in all | 0.4 m + 0.254 m visible, joint fixed | same sizes, joint prismatic -101 … +76 mm |
| Duct | 150 mm, outlet centre 85 mm from wall | one pipe on the upper channel | two pipes (lower on the spigot, upper sleeved, follows the telescope) |
| Blower | scroll housing, forward-curved wheel 130–150 mm x 60–70 mm, external-rotor motor about 80 mm | scroll, 24 vanes, wheel 134 x 65 mm, motor 78 mm, wheel 80 mm left of centre | same at 1:50; housing, drum and motor cylinder at 1:100 |
| Backdraft flap | two half discs in the 150 mm spigot | two leaves r 72 mm on a spindle | same |
| Filters | three cassettes about 280 x 320 mm | 3 x 0.28 x 0.32 m | same at 1:50, one field 0.856 x 0.32 m coarser |
| Lamps | two LED modules | 2 x 120 x 50 mm | same |
| Fixing | two hanging screws 270 mm apart at 190 mm, two at 25–30 mm | screws and pads | same |
| Mass | 10.3 kg | 10.3 kg | same |

### Defects found

- 1:100, 1:200 and 1:500 body lists were the 1:50 list with decimated profiles (27 solids each: 0.6 mm sheets, 24 vanes, 1 mm spindle, cassette meshes), not models for 15 / 30 / 75 mm; their `decisions` described lists that did not exist.
- Structure masses of the fan (1:50 `mass-fan-volute`, `-motor`, `-outlet`; coarser `mass-fan-unit`) sat on the centre line although the blower is 80 mm to the left.
- No duct spigot; duct one piece bound to the upper channel; telescope joint `fixed`.
- 2D "Longitudinal blower section" at 1:50 / 1:100 / 1:200 drew only two slivers of 1 mm and 1.3 mm (0.005 mm on paper at 1:200).
- 1:50 body, thermal and the rest of the structure were judged right from six directions, from below and in the cut: no change.

### Rebuilt

- Rig: `chimney-top` joint prismatic, `axisDeg` 90, -0.101 … +0.076 m (0.799 … 0.975 m total height).
- 1:50 body: `duct-spigot` (canopy), `duct-lower` (chimney), `duct` (chimney-top, sleeved). Structure: the three fan masses are references to scroll, motor and transition.
- 1:100 (15 solids body, 13 thermal, 18 structure): rim block with filter field and lamp openings, five plates of 15 mm, volute housing with wheel bore, wheel drum on its bone, motor cylinder, two U channels with 15 mm walls, solid duct.
- 1:200 (8 / 11 / 13): rim slab with filter field, pyramid as three wedges and a core (a single polyhedron was shaded smooth across hips and top edge), two chimney blocks.
- 1:500 (7 / 11 / 12): closed rim slab, the same four pyramid masses, two chimney blocks.
- Thermal at the coarse scales: shell from the body solids, hidden filter field and 2 mm diffuser sheets, emitter disc, motor.
- Structure at the coarse scales: fan mass moved to the blower, wheel mass on its bone, damper leaves as half-octagon plates (the validator asks a member for both damper bones at every scale).
- 2D: section at 1:50 and 1:100 draws housing, wheel and motor outlines (1:50 also the spigot, the two cut sheets stay); slivers removed at 1:100 and 1:200; `sections` removed at 1:200.
- `decisions` of all four volumes rewritten (en, de).

### Validation

`r3-validate.ts appliances/kitchen-extractor-hood --catalog-only --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0` (after publish).

### Rendered and read after the last change

1:50 body cut, poses `off` and `low` (top), clip `blower-inspection` t 0.3 (side cut), structure front; 1:100 body, body cut, underside, thermal cut, structure, 2D; 1:200 body, structure, thermal cut; 1:500 structure; 2D 1:50.

### Remaining (not signed)

- Rendered but not read: 1:50 pose `medium`, clip frames `blower-inspection` t 1.1, `switch-on-warm-up` t 2, `step-down-and-off` t 100, 1:50 thermal after the change (list unchanged), 1:500 thermal cut, 1:500 body after the pyramid split, 2D 1:200.
- 1:500 body after the split into rim slab and four pyramid masses was validated but its sheet was not read.
- The prismatic joint is in the rig but no pose or clip moves it; its axis sense (`axisDeg` 90 = up) is unverified in a render.
- 1:500 structure: hips are 60 mm angles (kept from the earlier list) and look heavy against the body; not checked for poking through the slope faces.
- 1:200 / 1:500 structure hips and rim frame were kept from the earlier lists, only read in the sheets.
- 2D `decisions` texts of 1:50 / 1:100 / 1:200 were not revised for the redrawn section; the 1:100 section outlines are 0.7–1.3 mm on paper.
- 1:50: lamps are plain 120 x 50 mm sheets, no bezel; no mains cable; chimney fixing screws not modelled.

### Fundamental issues

- `r3-sheet.ts` shades one `polyhedron` smooth across edges under 48° (hips 45°, top edge 34°), and a polyhedron with per-face vertices is refused (`volume-size`: one closed shell), so a faceted sheet-metal mass cannot be one solid with crisp edges.
- `volumes-structure-coverage` treats `damper-left` / `damper-right` (no mass, hidden flaps) as load-bearing and wants a member solid at every scale, also at 1:500.
- `volumes-feature` accepts 0.6 mm sheets at any scale when the thickness equals a thermal layer gauge; that let the 1:50 sheet list pass as 1:100 / 1:200 model.
- `volumes-insert.py` runs on import (no `__main__` guard), and the catalog file is not in its `dump` format, so root members cannot be rewritten byte-stable with it.

## Pass 1 (second round) (2026-11-22)

Chisel: `rework-kitchen-extractor-hood-pass2.py` (`source` edits the volume source in place, `root` rewrites `representations` of the catalog file). Source and catalog were identical at the start; the reference table of pass 1 stands (re-measured on the sheets: 0.90 x 0.50 m canopy, 246 mm, rim 45 mm, top plate 0.26 x 0.20 m, chimney 250 x 190 mm, duct 150 mm at 85 mm from the wall, wheel 134 mm, 80 mm left of centre, 0.915 m in all).

### Found (the state of pass 1 was INVALID under the rules of 01:23 and later)

| Scale, list | Part | Defect |
| --- | --- | --- |
| 1:50 body | base panel with cassettes, latches, meshes, lamps (12 solids) | floated 5.4 mm above the return flange of the rim, touching nothing (`volumes-connected`) |
| 1:50 body | `wheel-inlet-ring` | revolved strip of 3 x 17 mm, below the feature and no sheet (`volumes-feature`) |
| 1:50 / 1:100 structure | `member-hip-left/right` | angle bars ending 6.9 / 3.1 mm short of the rim: open load path; a square-cut angle cannot reach the rim corner without leaving the slope planes (hip meets the rim at 48 degrees) |
| 1:200 / 1:500 structure | hips | the same invented bars inside solid wedges |
| 1:100 body | `blower-motor` | floated 6 mm in the bore of the scroll, touching neither wheel nor housing |
| 2D 1:50 / 1:100 elevation | `blower-wheel`, `rotor-disc-section` | drawn on the spinning wheel bone (`lateral-ambiguous`, up to 1.34 mm wrong on paper) |

### Rebuilt

- 1:50 body: `rim-upstand`, the 5.4 mm fold of the return flange up to the base panel (0.6 mm sheet, ring 0.876 x 0.476 m), so rim, hem, base panel, cassettes and lamps are one object; the same solid in the thermal shell (`th-rim-upstand`).
- 1:50 body: `wheel-inlet-ring` is the flat shroud annulus it is (r 51 to 67 mm, 1.5 mm, declared `sheet`); the 3 mm bell-mouth lip is below the feature and left out.
- Structure at all four scales: the hip bars are gone; the stressed skin is the structure. 1:50 and 1:100 take the three sloping sheets / plates as members (`member-skin-front/left/right`), 1:200 and 1:500 the three pyramid wedges. Back plate, rim, top plate, chimney channels, joints, supports and masses unchanged.
- 1:100 body: the motor has its 40 mm hub through the housing wall to the wheel drum.
- 2D: the two wheel outlines of the section are bound to the canopy (a turned part looks the same at every angle), same position on paper.
- `decisions`: 1:50 underside text (hem) and the structure text of all four scales rewritten (en, de).

### Validation

`r3-validate.ts appliances/kitchen-extractor-hood --catalog-only --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0` (after `volumes-insert.py`).

### Rendered and read after the last change

1:50 body (six views, before), body cut from below (hem, cassettes, wheel, dampers), structure, thermal cut, pose `off` cut (six views), `low` top (leaves at 46 degrees), `medium` front cut, clips `blower-inspection` t 1.1 front cut and `step-down-and-off` t 100 top; 1:100 body cut, structure, thermal cut; 1:200 body, structure, thermal cut; 1:500 body, structure, thermal cut; 2D 1:50 and 1:100.

### Remaining (not signed)

- Rendered, not read: 1:100 body uncut, clip `blower-inspection` t 0.3, clip `switch-on-warm-up` t 2, 2D 1:200.
- Rig: the prismatic `chimney-top` joint (-101 … +76 mm) is still moved by no pose or clip; its axis sense is unproven in a render.
- 1:50 body: lamps are plain 120 x 50 mm diffuser sheets without bezel (a 6 mm bezel is below the feature); no mains cable, no chimney fixing screws; `filter-latch-*` sit 0.5 mm above the rim edge, 5.5 mm below the cassette face (recess not modelled).
- Structure 1:100 to 1:500: the fan and wheel masses lie inside the closed skin and show only in a cut; 1:500 `member-chimney-top` is a channel with 20 mm walls kept from pass 1.
- 2D: front view `chimney-upper` is drawn 0.25 m wide, the model is 0.244 m (0.06 mm on paper at 1:50); 2D `decisions` of 1:50 / 1:100 do not mention the section outlines.

### Fundamental issues (in addition to pass 1)

- `volumes-feature`: a revolved thin ring (shroud, washer, flange) cannot be declared `sheet` (it is measured by its 3 x 17 mm section as a strip), while the same ring as an extrusion with a hole passes; the rule should measure a revolve by the ring it sweeps.
- The validity of a published asset silently changed with the rule changes of 01:23 to 03:30 (this asset went from VALID to 1 + 4 findings without any edit); there is no catalog-wide re-validation list after a rule change.
