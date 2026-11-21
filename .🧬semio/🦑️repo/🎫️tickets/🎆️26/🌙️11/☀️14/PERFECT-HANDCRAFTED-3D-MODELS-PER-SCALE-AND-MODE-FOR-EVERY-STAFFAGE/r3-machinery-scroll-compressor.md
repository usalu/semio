# Round 3 — machinery/scroll-compressor

State: DONE (2026-11-20). Scripts: `r3-scroll-compressor-kit.py` (constants of the real compressor, every part of every scale stated by hand with its solid and its 2D shapes, the root members, candidate build; `--apply` writes the catalog root members, the manifest entry by a scoped text edit and the volumes source), `r3-scroll-compressor-texts.json` (bilingual texts, decisions, areas, dimensions), `r3-scroll-compressor-run.sh` (build, validate the candidate, apply, check, splice, validate). Resume: `./r3-scroll-compressor-run.sh`. No earlier round-3 work on this asset existed; I started from the catalog state of 2026-11-17 (volumes identical to `T/volumes`).

## 1. Reference

Reference class: Copeland Scroll K-frame hermetic compressors (ZR*K5E / ZH*K4E, 2 to 4 ton, 1.5 to 3.5 kW input), the commonest heat-pump scroll class and the one the asset's 2.5 kW input points at. Sources: S1 = a ZR48K5 product listing found by web search (2026-11-20): mounting 7.5 x 7.5 in (190.5 mm square), height 17.17 in (436 mm). S2 = the Emerson ZH application guideline listing (search snippet): ZH12K4E height 388 mm. S3 = `copeland-2015` (asset reference: shell temperatures). K = my knowledge of the construction (not a published number). The datasheet PDFs could not be read (404 or font-encoded text); the numbers marked K are stated as such.

| quantity | real value | source |
|---|---|---|
| overall height incl. discharge stub | 383 to 436 mm across the class; 2.5 kW input class about 410 to 436 mm -> asset 0.43 m kept | S1, S2, K |
| shell | welded steel, OD 241 mm (9.5 in), wall 3.5 to 4 mm; middle cylinder, lower cap inside, top cap lapping outside with a circumferential weld | K |
| top cap | torispherical: skirt about 20 mm, knuckle about 35 mm, crown about 250 mm radius, rise about 75 mm above the lap | K |
| base | stamped plate, roughly round with four ears on the diagonals, bolt square 190.5 mm, on four hourglass rubber grommets about 40 mm across and 20 to 25 mm high, sleeve and nut on top | S1, K |
| suction fitting | 7/8 in (22.2 mm) tube with a weld boss, on the shell side about 0.18 m above the base | K |
| discharge fitting | 1/2 in (12.7 mm) stub on a boss at the top of the cap, about 30 mm above the cap | K |
| terminal box | moulded cover about 78 x 72 mm, 48 mm deep, on a welded fence at mid height, cable gland underneath | K |
| internals | motor (stator and rotor) in the lower half, main bearing frame and the scroll set (fixed and orbiting scroll) under the top cap, crankshaft between main and lower bearing, oil sump at the bottom | K |
| mass | 23 to 30 kg for 2 to 4 ton models -> 26 kg | K |
| temperatures | discharge dome 80 to 110 degC, suction cold | S3 |

## 2. Audit of the asset as found

| item | found | real | deviation |
|---|---|---|---|
| shell diameter | 0.200 m | 0.241 m | -17 %: read as a slim gas bottle ("fire hydrant") |
| height/diameter | 0.43 / 0.20 = 2.15 | 0.43 / 0.241 = 1.78 | too slender |
| base | 0.28 m square slab, 30 mm thick, flat on the ground | eared plate on four grommets, 22 mm clear under the plate | invented slab, no grommets |
| top | a half-ellipse "dome" 60 mm high on a 6 mm shoulder | lapped torispherical cap, 75 mm rise, 4 mm lap step | form wrong |
| discharge stub | 28 mm with a 42 mm flange | 12.7 mm stub on a 32 mm boss | 2.2 x too thick |
| suction stub | 26 mm with collar and crimp | 22.2 mm tube in a 34 mm weld boss | form |
| terminal box | 36 x 60 x 55 mm block, lid on top | 48 x 78 x 72 mm moulded cover with gland underneath | too small |
| sight window | at y 0.18 (motor height), ring 8 mm, plate outside the shell | authored device (no real shell has a window); the scroll set sits under the cap | moved to the scroll-set height y 0.26 |
| rig | shell length 0.34, dome joint y 0.34, rotor joint y 0.18, terminal joint (0.10, 0.25), masses 9.4/2.6/0.3/0.2 kg | seam y 0.3175, rotor y 0.26, terminal on the shell surface (0.112, 0.22) | joints off by up to 80 mm, mass 12.5 instead of about 26 kg |
| dimensions | widthM 0.30, depthM 0.28, massKg 12.5 | 0.345 (suction tube to terminal cover), 0.266 (shell to window bezel), 26 kg | see section 3 |
| structure | the shell as a closed can, the dome copied, terminal and scroll as references, two ellipsoid masses (qa-visual-2: B-, "shell as a tube, add the scroll frame, feet joints") | pressure vessel, bearing frame, crankshaft, grommets | D3 |
| thermal | references; emitter a floating ring | D2 patch on the crown | |
| 2D | slim shell, slab foot, half-ellipse dome, fat stubs | | redrawn (E1) |

## 3. Root fixes (old -> new)

- `dimensions`: widthM 0.30 -> 0.345, depthM 0.28 -> 0.266, massKg 12.5 -> 26 (heightM 0.43 kept). Manifest entry: widthM 0.3 -> 0.345, `distinction` rewritten (grommets).
- `rig`: shell lengthM 0.34 -> 0.3175, mass 9.4 -> 22.6 kg (com 0, 0.17), title "Hermetic shell on base plate and grommets"; dome joint (0, 0.34) -> (0, 0.3175), lengthM 0.09 -> 0.1125, mass 2.6 -> 2.2 kg (com y 0.03 -> 0.04), title "Top cap with discharge stub"; terminal joint (0.1, 0.25) -> (0.112, 0.22), lengthM 0.06 -> 0.048, mass 0.3 -> 0.4 (com 0.024, 0.039); rotor joint (0, 0.18) -> (0, 0.26), mass 0.2 -> 0.8 kg.
- `anchors`: base-contact footprint 0.26 x 0.26 -> 0.23 x 0.23 (grommet square); dome-top (0, 0.09) -> (0, 0.1125) (top of the stub).
- `materials`: added `epdm-rubber` (grommets) and `borosilicate-glass` (sight glass), Incropera values.
- `provenance`: reference `copeland-zr-k5` added; notes rewritten (en, de). `summary` rewritten. `structure.assumptions` rewritten (26 kg, four grommets, 0.23 m).
- `thermal.regions[].areaM2` (1:50 model, all within factor 1.5, C16): discharge 0.006 -> 0.0044, terminal 0.015 -> 0.019, feet 0.15 -> 0.106, scroll 0.016 -> 0.0123; shell 0.28, dome 0.079, suction 0.009 kept (measured 0.282, 0.080, 0.009).
- `representations` 50/100/200: all three views redrawn from the parts (below), decisions rewritten; 500 `reason`, `limitingFeature`, `limitingSizeM` 0.028 -> 0.0127 (the real stub).

## 4. 2D drawings (E1)

Every shape is the true orthographic outline of its part, computed in the kit from the same constants as the solids. 1:50 elevation: one outline of plate and the two visible hourglass grommets, bolt washers and nuts (detail), shell, suction boss+tube, window bezel, scroll disc (glazing) with the 1.8-turn wrap stroke, terminal cover with gland, cap profile (skirt, knuckle, crown), discharge boss+stub. Front: the same foot outline, shell, window edge-on on the left, terminal cover with gland, cap, stub. Plan: eared plate, four bolt washers, shell, cap, discharge boss and tube, suction, cover, window. 1:100: grommets as pads, 15 mm plate, 2+2 point cap, no bolts/gland, wrap 1.3 turns. 1:200: one foot block 0.22 square with chamfered corners, shell, cap, 30 mm stubs, box, window pad. Shape counts elevation/front/plan: 11/8/9, 9/6/7, 7/6/7. The `spin` clip at t 0.4 s (r3-2d) turns the wrap in the window; the rest stays.

## 5. Models per scale

| scale | body / thermal / structure | what is modelled |
|---|---|---|
| 1:50 | 16 / 16 / 15 | hourglass grommets (repeat 4), washer-and-nut mounts, eared base plate (extrude, 104 mm disc + four round ears on the 190.5 mm square), shell (2 mm foot radius), torispherical top cap with the 4 mm lap step, discharge boss and open 1/2 in stub, suction weld boss and open 7/8 in tube, terminal cover with 6 mm radii and gland, window boss, bezel, 3 mm glass, orbiting scroll plate and its wrap (slender loft, 1.8 turns). Thermal: references per region, shell top and cap underside as section faces (weld joint between the pressure parts), terminal back as section face (on the shell, C25 b), crown outlet patch for the plume (D2). Structure: 3.5 mm pressure vessel, lower cap, base plate, main bearing frame, fixed scroll, crankshaft, lower bearing bridge (cross), base weld and lap weld joints, cap shell, terminal fence, orbiting scroll with hub bearing joint, motor mass at the com (y 0.11 to 0.23), four grommet supports. |
| 1:100 | 10 / 11 / 11 | grommet pads, 15 mm plate, plain shell, 3+3 point cap, boss and tube one body per fitting (discharge 15 mm), cover without gland, window as one ring, 15 mm scroll plate and a 1.3-turn wrap. Structure without weld joints, fixed scroll merged into the frame. |
| 1:200 | 7 / 9 / 9 | 29.5 mm foot block, shell, 2+2 point cap, 30 mm stubs, box, window pad. Thermal adds a 30 mm scroll disc at the window. Structure: vessel, foot frame, frame, crankshaft, cap shell, fence, orbiting scroll, motor mass, one contact pad. |
| 1:500 | unavailable | 12.7 mm stub, 0.7 mm wide on paper |

Final check `$BUN volumes-check.ts machinery/scroll-compressor --product`: `PASS (9.9 s)`, 33 rule lines ok. Silhouette IoU elevation/front/plan 1:50 0.998/0.995/0.996, 1:100 0.996/0.997/0.997, 1:200 1.000/0.997/1.000; bounds, feature, monotonic, cross-scale, thermal coverage (1:50 areas within 2 %), structure coverage, bones, slender, copied all ok. `$BUN r3-validate.ts machinery/scroll-compressor --catalog-only --volumes`: `VALID (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` after the splice.

## 6. What the final renders show (r3-sheet)

- 1:50 front: a stocky Copeland-type compressor on four grommets with nuts, the eared plate visible as a thin band, the terminal cover with its gland on the shell, the window boss in profile on the left, the lapped cap with a clear step, the thin stub on its boss.
- 1:50 right side (product elevation): the bolted window with glass at the scroll-set height under the cap, suction boss and tube at the back, cover at the front. Back: the open bore of the suction tube end-on, window in profile. Left: plain shell, cover and suction tube. Top: the four ears with washers on the diagonals, cap, stub, cover, window, suction. Three-quarter: reads at first sight as a hermetic scroll compressor.
- 1:50 thermal (cut): shell red, cap green, stubs, cover, grommets/plate and the scroll in the window coloured per region, the plume patch on the crown. Structure (cut): vessel with lower cap, cross bridge, crankshaft, bearing frame and fixed scroll, motor mass, cap shell with lap-weld ring, fence, orbiting scroll with bearing, green grommet supports.
- 1:100: same reading; the wrap shows in the window (no glass at this scale). 1:200: cylinder, rounded cap, foot block with corner ears, stubs, box, window pad.

## 7. Still imperfect

- The window is an authored didactic device (the asset's concept and its `spin` clip depend on it): real hermetic shells are closed, the wraps lie horizontal and the orbiting scroll orbits without turning. Kept and stated in provenance; moved to the true height of the scroll set.
- In the `r3-sheet` shading the 1:50 glass is opaque, so the wrap behind it does not show in the 3D panels (it shows in the 2D, where the disc is glazing). A renderer matter, not a model one.
- Weld beads, bolt heads of the window, the terminal cover clip and the cable are below or at the scale and are not modelled.
- Reference numbers marked K could not be confirmed against the datasheets (the PDFs were not readable).
- Areas: the honest discharge area is 0.0044 m2 (1/2 in stub on a boss), smaller than the earlier 0.006; changed within factor 1.5, nothing for `area-review.md`.

## Round 5 handcraft

Script: `r5-scroll-compressor-model.py` (edits the volumes source; splice with `volumes-insert.py`). 1:50 to 1:200 unchanged; the model-only 1:500 (coordinator's digital scale) re-authored.

### Reference (Copeland ZR K5 class) and model at 1:50

| quantity | reference | model 1:50 | deviation |
|---|---|---|---|
| overall height incl. discharge stub | 0.43 m (asset; class 0.388-0.436) | 0.43 m | 0 % |
| shell OD | 241 mm (9.5 in) | 241 mm | 0 % |
| bolt square | 190.5 mm (7.5 in) | 190.5 mm | 0 % |
| cap rise above lap | about 75 mm (K) | 75 mm | 0 % |
| discharge stub | 1/2 in = 12.7 mm | 12.7 mm | 0 % |
| suction tube | 7/8 in = 22.2 mm | 22.2 mm | 0 % |
| suction height above base | about 0.18 m (K) | 0.18 m | 0 % |
| terminal cover | 78 x 72 x 48 mm (K) | 78 x 72 x 48 mm | 0 % |

### Grades (body / thermal / structure), before -> after

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- (default `full-load`, `off`, `part-load` and clip `spin` t 0.4: only the rotor turns, hidden behind the window) |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | B+ / A- / A- | unchanged (foot block with chamfered corners at the 30 mm feature) |
| 1:500 | B- / B / B+ | B+ / A- / B+: the cap was a faceted two-slope cone with a hat-brim sliver, the base a square slab, the window thermal shell and the scroll disc stood 26 to 53 mm out of a body without a window. Now a smooth torispherical crown (75 mm rise), an octagonal base plate whose chamfers stand for the ears, no window shell in thermal, the scroll region a 90 mm disc inside the shell at the scroll-set height. |

### Checks

`volumes-check.ts machinery/scroll-compressor --product`: PASS. `r3-validate.ts`: VALID.

### Remaining

- The window remains an authored didactic device (concept of the asset); real hermetic shells are closed.
- 1:50 structure reads as the closed vessel from outside (it is the real pressure vessel; the frame, crankshaft and bearings are inside and show only in the cut).
