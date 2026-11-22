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
