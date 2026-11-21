# Round 3: appliances/duct-fan

State: finished and spliced on 2026-11-20. No earlier round-3 work existed for this asset, so it was rebuilt from the roots. Everything below describes the catalog file as it is now.

Scripts in the ticket folder:
- `r3-duct-fan-kit.py`: the measures, the blade sections (pitched, twisted, swept lens lofts with a buried root boss) and a principal-extent check.
- `r3-duct-fan-volumes.py`: every solid of every scale and list, stated by hand. It writes `volumes/appliances/duct-fan.json`.
- `r3-duct-fan-roots.py`: the root fixes and the elevations of 1:50, 1:100 and 1:200 drawn from each scale's body. It writes the candidate `🗑️generated/r3-duct-fan/🔣️.json`. It can be run again on the fixed catalog and gives the same result.
- `r3-duct-fan-apply.py`: writes the candidate's root members into the catalog in the file's own formatting. It is a copy of the split-ac-indoor-unit apply script.

To resume, run in order:
1. `python3 r3-duct-fan-volumes.py`
2. `python3 r3-duct-fan-roots.py`
3. `$BUN r3-validate.ts --file 🗑️generated/r3-duct-fan/🔣️.json`
4. `python3 r3-duct-fan-apply.py`
5. `python3 volumes-insert.py volumes/appliances/duct-fan.json <catalog>`

## 1. Reference

| Quantity | Real value | Source | Model |
|---|---|---|---|
| Class | Plastic axial wall or duct fan: 0.8 to 0.9 kg, 24 W, about 295 m3/h at 2400 rpm (150 mm class) | Vents 150 M and 150 VKO product pages (web search 2026-11-20) | Kept: 0.9 kg, 28 W, 180 m3/h at 1400 rpm (slower, larger rotor) |
| Front panel and depth | 150 M: panel 206 x 182 mm, 106 mm deep, spigot 150 mm | Vents 150 M | Kept: plate 220 x 220 mm, 90 mm deep |
| Spigot | Outside diameter equals the duct size, thin wall, a slightly larger ring (VKO: D 150, D1 154) | Vents 150 VKO | 200 mm outside, 195 mm bore, 208 mm sealing bead |
| Plate-type wall-ring fan | A square wall plate (or a steel ring) with an inlet nozzle in the bore. The impeller runs in the nozzle throat; an external-rotor motor is held behind it | Maico EZQ and EZS product pages (square plate against steel ring); product knowledge | Same construction |
| Rotor | External-rotor motor: the rotating can is the hub, with the blades moulded onto it. Tip clearance 1 to 2 mm | Product knowledge | Can 68 mm, 50 mm long, with a domed front and a 24 mm spinner cap. Tip corners run 1 mm inside the 176 mm throat |
| Blades | Low-pressure axial blades: pitch about 35 to 40 degrees at the root and 20 to 25 at the tip, chord growing outwards, swept, 2 to 3 mm thick | Fan design practice (ISO 5801 class of fan) | 5 blades: pitch 38 to 24 degrees, chord 40 to 62 mm, 22 degrees forward sweep, lens sections 3.2 to 2 mm thick |
| Fixing | 4 screws in the plate corners, 4 to 5 mm with wall plugs | Installation practice | 4 domed 10 mm caps at (+-0.092, +-0.092) |
| Mass split | The motor dominates: about 0.45 to 0.55 kg of a 0.9 kg fan. The ABS mouldings weigh about 0.3 kg | Product class estimate | Plate 0.2 kg, housing with stator 0.45 kg, rotor 0.25 kg |

Sources: [Vents 150 VKO](https://ventilation-system.com/product/150-vko), [Vents 150 M](https://ventilation-system.com/product/150-m), [Maico axial wall fans DN 200](https://www.maico-ventilatoren.com/en/products/p/dn-200-g5900/ezs-20-4-e-p6743).

## 2. Audit before round 3

- **Body (the worst defect).**
  - The plate was a solid 12 mm slab with no bore. Seen from the room (the elevation, from -w) the 3D fan was a blank square: the ring, blades and hub sat hidden behind the plate.
  - The four screw heads sat on the back of the plate (z -0.033), on the wall side.
  - The blades were flat 7.8 mm planform slabs with no pitch or twist.
  - The hub was a plain cylinder. The motor spider was 4 capsules of 8 mm.
  - The ring was a 12 mm thick extruded tube with no inlet nozzle.
  - The silhouette checks were green only because the solid plate filled the drawn square.
- **Rig and anchors.**
  - Both wall screws (the support anchors) sat at (0, +-0.092). That point is inside the 0.2 m bore, where no screw can be: the real screws are in the corners.
  - The mass split put 0.5 kg into the plate and 0.12 kg into the rotor. That is the wrong way round for a fan whose motor is the heaviest part.
- **2D.**
  - The elevation drew the plate as a full filled square and the ring as a full disc. A true view shows a plate with a bore.
  - The screws were 16 mm, and the hub cap was drawn off centre (+4 mm, +4 mm).
  - The spider arms were missing from the view.
  - At 1:50 and 1:100 the blade tips reached r 0.091 to 0.093, beyond the 88 mm throat radius.
- **Thermal.** The blades and hub were references to the slabs. The emitter was a solid disc cutting through the blades and hub.
- **Structure.**
  - The rotor member was a 16 mm bar.
  - The supports were at the wrong anchors, and the plate mass was a flat disc.
  - qa-visual graded body B, thermal A/B and structure B. It asked for a motor in the hub and for supports inside the plate.
- **Areas.**
  - `casing` was 0.14 m2. The reviewer's hand figure counted the back of the plate as the full square (0.048) and left out the inner walls of the open nozzle and spigot. The honest 1:50 outer area is 0.183.
  - `hub` was 0.0145 m2, for a can 30 mm long; the modelled can is 50 mm long.

## 3. Root fixes (catalog outside `volumes`)

| Member | Old | New |
|---|---|---|
| `anchors.wall-screw-top` | (0, 0.092), "Upper screw" | (-0.092, 0.092), "Upper left screw" (pin) |
| `anchors.wall-screw-bottom` | (0, -0.092), "Lower screw" | (0.092, -0.092), "Lower right screw" (roller) |
| `rig.bones[].mass.kg` | plate 0.5, ring 0.28, rotor 0.12 | plate 0.2, ring 0.45, rotor 0.25 (sum 0.9 unchanged) |
| `rig.ring.title`, `rig.rotor.title` | ring with motor spider; rotor with blades | housing with nozzle, spigot, motor spider and stator; external rotor with blades |
| `thermal.regions.casing.areaM2` | 0.14 | 0.18: the measured 1:50 outer area is 0.183, factor 1.29 (within C16) |
| `thermal.regions.hub.areaM2` | 0.0145 | 0.0165: the measured 1:50 can, factor 1.14 |
| `thermal.regions.casing.title` | Plate and housing ring | Plate, nozzle, spigot and motor spider |
| `structure.assumptions` | 2 screws | 4 corner screws, idealised as the upper left pin and the lower right roller; the motor hangs on the spider |
| `provenance` | 3 references | Added `vents-150-m` and `maico-ezq-ezs`. Notes rewritten (en, de): construction, frame (plate face at w -0.045, wall at -0.033), mass split |
| `summary` | | Now names the nozzle, the external-rotor motor and the spider |
| `representations.50/100/200.geometry.elevation` | see audit | Redrawn as the true view (E1) |
| `representations.50.front/plan` | lip 7 mm | 7.5 mm (the modelled bead) |
| `representations.200.front/plan` | one T poly | Plate (15 mm) and spigot as two rects, as at 1:100 |
| `representations.*.pens.texture` | null | 0.1 (only for the opening tone) |
| `representations.*.decisions` | | Rewritten (en, de) |

The elevation redraw in detail:
- **Plate:** a path with the bore as a hole.
- **Ring:** an annulus from 0.1 to 0.088.
- **Opening:** the dark tone of the duct seen through the fan. Its role is now `texture`, because it is not a part.
- **Blades:** drawn from the lofts' true plan-form, with leading-edge and trailing-edge points and the straight tip cut.
- **Hub and cap:** circles, the cap centred.
- **Screws:** 10 mm caps.
- **Spider:** 4 new `strut-*` arms (8 / 15 / 30 mm).

Unchanged: `dimensions`, poses, clips, emitter values, the 1:500 reason.

## 4. What was modelled (solids per list; expanded counts after repeats in brackets)

### 1:50: body 14 (20), thermal 15, structure 14

- **Body:**
  - Plate: 12 mm, 220 square, R 14 corners, 0.2 m bore.
  - 4 domed screw caps (repeat).
  - Nozzle moulding (`ring`): a revolve with a 12 mm bell mouth into the 176 mm throat, ending at the seam at w -0.015.
  - Spigot sleeve: the throat, a 45 degree diffuser to the 195 mm bore, out to w 0.045.
  - Sealing bead: 208 mm by 7.5 mm.
  - 4 spider arms, 8 x 16 mm with rounded edges, on a 52 mm stator flange.
  - 5 blades, lofted through 9 lens sections. The root boss is 7.6 mm inside the can; the visible blade is 3.2 to 2 mm thick, pitched 38 to 24 degrees with the leading edge upstream (-w) and ahead, chord 40 to 62 mm, swept 22 degrees.
  - The 68 mm external-rotor can, with a domed front and a spinner cap.
- **Thermal:**
  - References for every body solid, in their single layers (shell, blade, can).
  - The `extract` outlet: a 2.5 mm annulus across the throat exit, between the hub and the throat wall (D2, tag `extract`).
- **Structure:**
  - Members: plate, nozzle, spigot, spider and flange (references).
  - Rotor: the can (reference) on an 8 mm steel shaft.
  - Joints: the plate clip ring and the 18 mm bearing.
  - Masses: plate, housing with stator, and rotor, each at its centre.
  - Supports: the pin is an 8 mm screw from its cap into the plug; the roller is an 8 mm cylinder in the plate at the lower right anchor.

### 1:100: body 9 (12), thermal 10, structure 11

- **Body:**
  - Plate 14 mm (its drawn thickness).
  - One housing tube, with the nozzle as a 12 mm chamfer and the diffuser.
  - 5 blades with the same pitch, twist and sweep. They are generalised to 5 to 4 mm sections with a 15 mm buried root boss.
  - A 72 mm can with a chamfered front.
  - 4 arms, 15 x 16 mm, crossing behind the hub.
- **Thermal:** references and the outlet annulus.
- **Structure:**
  - Plate, tube, arms, and the can on a 15 mm shaft.
  - The bearing joint.
  - 3 masses.
  - Pin 15 mm and roller 14 mm.

### 1:200: body 7 (10), thermal 8, structure 7

- **Body:**
  - Plate 15 mm with the 192 mm bore.
  - A thin tube.
  - 4 arms of 30 mm reaching under the rim of the 88 mm hub.
  - 3 blade plan-forms through the 30 mm blade stage.
- **Thermal:** references and the outlet annulus.
- **Structure:**
  - Plate, tube, arms and hub.
  - One mass of the whole fan at w -0.016.
  - Pin and roller of 30 / 15 mm.

1:500 stays unavailable. Each authored scale has three bilingual decision sentences.

## 5. Checks

- `$BUN volumes-check.ts appliances/duct-fan --product`: **PASS** (2.2 s).
- **Silhouette IoU** (elevation / front / plan), with p95 0 to 2 mm:

  | Scale | Elevation | Front | Plan |
  |---|---|---|---|
  | 1:50 | 0.998 | 0.994 | 0.994 |
  | 1:100 | 0.997 | 1.000 | 1.000 |
  | 1:200 | 1.000 | 1.000 | 1.000 |

- **Area deviation:**

  | Scale | Casing | Blades | Hub | Total |
  |---|---|---|---|---|
  | 1:50 | 1.8 % | 4.3 % | 0.3 % | |
  | 1:100 | 2.0 % | 13 % | 9.8 % | |
  | 1:200 | | | | 10.8 % |

- Feature, monotonic, cross-scale, copied, bones, bounds, views-disagree and structure coverage (supports at their anchors) are all ok.
- `$BUN r3-validate.ts appliances/duct-fan --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.

## 6. What the renders show (opened with the Read tool)

Seen:
- `r3-sheet` body at 1:50 (twice: before the 2D fix and after it), 1:100 and 1:200;
- 1:50 thermal `--cut` and 1:50 structure;
- 1:50 clip `spin` at t 2.2 s, right side;
- `volumes-render` at 1:50 and 1:200;
- `r3-2d` at 1:50 (candidate, light-line and light-solid enlarged), 1:100 and 1:200 (enlarged), and 1:50 in pose `off`.

Rendered but not opened: 1:100 and 1:200 thermal and structure sheets, and the 1:100 `volumes-render` sheet. Their content was judged through the sheets of the neighbouring scales and the check numbers.

What the views show:
- **Right side (the room view, from -w):**
  - The rounded plate with four domed caps, and the bell mouth as a soft ring into the throat.
  - Five swept blades, each shaded by its pitch: the blades turned away from the light are darker.
  - The 68 mm can with its spinner cap, and the spider arms visible in the gaps.
  - At t 2.2 s the rotor has turned rigidly and every part stays closed.
- **Front and back (side-on):** the 12 mm plate, the spigot with the seam line and the bead; the screw caps stand 4 mm proud.
- **Left (from the wall or duct, +w):** the spigot bore, the diffuser step, and the cross of the spider on the flange in front of the can.
- **Top:** the plate and spigot, with the bead at the duct end.
- **Three-quarter:** reads at once as a plate-type wall fan.
- **Thermal cut:**
  - The red casing is cut through the nozzle and spigot.
  - The green blades show their twist, and the hub is violet.
  - The outlet annulus fills the throat exit.
- **Structure:**
  - Blue members: plate, nozzle, spigot, arms, flange, and the can on its shaft.
  - The orange clip ring at the plate back.
  - The supports are inside the plate (the pin's cap shows at the upper left); masses are magenta.
- **1:100:** the same fan with the chamfered mouth, 15 mm arms and thicker blades.
- **1:200:** a square with a large dark bore, three broad blades, the 88 mm hub and a heavy cross. It reads as a fan at a glance.
- **2D at 1:50:**
  - Light-solid: the plate with its bore, the ring, the dark duct tone in the gaps, light blades, the spider, the hub and the cap.
  - Light-line: blade outlines over the spider lines.

## 7. Still imperfect

- **Blade thickness and the feature rule.** The product's `volumes-feature` measures a loft by its thickest section, not by its extents. The real 3.2 to 2 mm blades pass only through the root boss buried in the hub (7.6 mm at 1:50, 15.2 mm at 1:100). That boss is a real hub fillet and is invisible, but it exists for the rule. The blade's true principal extent is 9.9 mm. Filed in `author-requests.md` (2026-11-20 r3 appliances/duct-fan).
- **1:100 blades.** They are 5 to 4 mm against 3.2 to 2 mm real: a generalisation that does not show at 1:100.
- **Facets.** Small revolves tessellate coarsely in the sheets: the hub shows about 12 facets. Segment counts come from the kernel quality, not from the data.
- **Tip cut.** The blade tip is a planar cut, because loft sections are planar. The tip middle runs 4.7 mm inside the corners instead of following the throat circle.
- **Opening tone.** The opening is a `texture` shape with a 0.1 mm texture pen, so it is not counted as body. `volumes-render`'s owned overlay still paints it red as "drawn only"; the product validator, which judges, excludes it (IoU 0.998).
- **Screw caps in front and plan.** These views do not draw the 4 mm proud caps (0.08 mm on paper).
- **Rounded edges.** The plate's 3 mm edge radius is drawn in front and plan (rect r 0.003) but not modelled: it is below the 7.5 mm feature.
- **Plate as a solid slab.** The plate and the nozzle wall are solid envelopes. The real mouldings are 2.5 mm shells with ribs, but their cavities are closed (C31), so the outer area is the same.
- **No backdraft shutter or cable gland.** Not modelled. In the default pose `high` a shutter would stand open anyway.
- **Area figures.** The 1:200 blades (0.044 against 0.0285) and the hub (+31 %) are high because the 30 mm floor thickens them. Only the 1:200 total is judged, at 10.8 %.
- **Area corrections.** Both are within the factor 1.5 of C16 and are listed here, not in `area-review.md`.

## Round 5 handcraft (2026-11-21)

Scripts: `r5-duct-fan-500.py` (new 1:500, uses the round-3 solid writers). Shared root fix: `r3-lib.ts cornerNormals` (see below).

### Reference (model to) and model vs reference, 1:50

| Quantity | Reference | Model | Deviation |
|---|---|---|---|
| Wall plate | 0.22 x 0.22 m, 12 mm ABS (DN 200 plate fan class: Maico EZQ/EZS, Vents 150 M scaled to DN 200) | 0.22 x 0.22 x 0.012 m | 0 % |
| Duct spigot / bore | DN 200: 200 mm outside, thin wall | 200 mm, bore 195 mm, bead 208 mm | 0 % |
| Depth over spigot | 90 mm (asset dimension) | 90 mm (bbox deviation 4 mm = the 4 mm proud screw caps) | within feature |
| Throat / tip clearance | 1 to 2 mm | 176 mm throat, tips 1 mm inside | ok |
| Blades | 5, pitch 35-40 deg root / 20-25 tip, 2-3 mm thick | 5, 38 -> 24 deg, 3.2 -> 2 mm, chord 40 -> 62 mm | ok |
| External-rotor can | 60-70 mm | 68 mm | ok |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- (plate faces shaded with diagonal wedges by the sheet renderer) | A- / A- / A- |
| 1:100 | B+ / A- / A- (same wedges) | A- / A- / A- |
| 1:200 | B / B+ / B+ | B+ / B+ / B+ (unchanged solids; wedges gone) |
| 1:500 | C+ / C+ / C+ (round-1 leftover: 12-gon bore, plank blades, 4 mm plate, boxy spider) | A- / A- / A- |

Opened: body sheets 1:50, 1:100, 1:200, 1:500 (before and after), thermal cut and structure at every scale, poses `off`, `low` (three-quarter), clip `speed-steps` t 1 s (right), 2D 1:50, 1:100, 1:200.

### Changes

- 1:500 authored anew: rounded 0.22 m plate 12 mm with a true 32-sided 192 mm bore, housing tube to the duct end, a two-bar motor cross, the 80 mm can and three swept 8 mm blade planforms; thermal by reference with the outlet annulus; structure plate, tube, cross, can, one mass, pin and roller at the corner screws.
- Root fix of the review tool: `r3-lib.ts cornerNormals` averaged every face at a welded vertex, so flat caps of extrudes with holes or fillets (this plate, many others) were shaded with light and dark wedges (open requests in `tool-requests.md`, 2026-11-19/20). Each corner now averages only the faces within the crease angle of its own face, area-weighted; flat caps are flat.

### Checks

`volumes-check.ts --product` PASS; `r3-validate.ts --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); spliced.

### Remaining

- 1:200 blades are three planforms through the 30 mm blade stage with visible tip gaps in the zoomed sheet (0.02 mm on paper): the drawing's abstraction, kept.
- Supports at 1:100 to 1:500 are buried in the plate and show as z-fighting dots in the structure sheets (renderer).
