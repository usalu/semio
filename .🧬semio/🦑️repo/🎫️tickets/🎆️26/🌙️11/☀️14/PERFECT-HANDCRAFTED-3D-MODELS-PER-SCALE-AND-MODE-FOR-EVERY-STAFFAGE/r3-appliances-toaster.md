# Round 3: appliances/toaster

State on 2026-11-20: finished and spliced. No earlier round-3 work existed for this asset (only `author-13-toaster.py` and `author-57-toaster.py` of rounds 1 and 2), so it was started from scratch. The catalog file holds the corrected roots, the redrawn 2D of every scale and view, and the new volumes of all four scales (1:500 is a digital model without a print drawing, as the catalog declared it since the bulk pass of 03:40 to 04:10).

Scripts in the ticket folder:
- `r3-toaster-kit.py`: measures, the housing section functions, bread outlines and the solid writers.
- `r3-toaster-model.py`: states every solid of every scale and list and writes `volumes/appliances/toaster.json`.
- `r3-toaster-draw.py`: the 2D of every scale and view from the same measures (ruling E1).
- `r3-toaster-roots.py`: candidate document `🗑️generated/r3-toaster/🔣️.json`; `--apply` writes only the changed members into the catalog file.
- `r3-toaster-texts.json`: provenance notes, volume decisions and drawing decisions, en and de.
- `r3-toaster-ship.sh`: model, candidate, validation with volume rules and print suitability, roots, splice, validation of the catalog file.
- `r3-toaster-suit.ts` (print suitability), `r3-toaster-zoom.ts` (2D panels cropped to the object).

## 1. Reference

| Quantity | Real subject | Source | Model now |
|---|---|---|---|
| Overall L x D x H | 0.26 to 0.32 m x 0.16 to 0.19 m x 0.18 to 0.21 m, 800 to 1000 W, 1.1 to 2 kg | De'Longhi Icona CTO2003: 320 x 185 x 205 mm, 900 W, 2 kg, two slots (manufacturer and retailer data read 2026-11-20, new reference `retail-two-slice-toasters-2026`); IEC 60335-2-9 for the class | 0.30 x 0.17 x 0.20 m with bread, 900 W, 1.3 kg (`dimensions` unchanged, inside the class) |
| Housing | chromed sheet wrap on a plastic base moulding, dark top with the slots, large end radii on retro bodies | product knowledge | wrap 0.30 x 0.152 m, end shoulders 36 x 38 mm, long shoulders r 22 mm, black plinth 12 to 30 mm inset 6 mm, dark cap from 174 to 188 mm |
| Slots | 135 to 145 mm long, 28 to 36 mm wide, 55 to 65 mm apart | product knowledge | 136 x 30 mm, r 6 mm, 60 mm apart, rims 8 mm deep |
| Bread | sandwich slice 100 to 120 mm square, 12 to 14 mm thick | product knowledge, asset notes | 104 x 115 x 13 mm, crust ring 3.5 mm, 12 mm above the cap when lowered |
| Carriage | lifter under each slice, yoke on a vertical guide rod in the end compartment, lever rigidly on the yoke, travel 35 to 45 mm | product knowledge | lifters, yoke, sleeve on a rod at x 0.112, lever arm through the long face, travel 40 mm |
| Controls | lever knob 15 to 25 mm proud, dial of 25 to 35 mm, cancel button | product knowledge | knob 30 x 24 mm and 14 mm proud in a 20 x 68 mm bezel, dial 30 mm and 12.5 mm proud, button 24 x 14 mm |
| Feet | four rubber feet of 20 to 27 mm, 6 to 12 mm high | product knowledge; `area-decisions.md` rows 239 and 1106 (o25) | four tapered feet 22 to 27 mm, 12 mm high |

The control layout on the long face was kept, not moved to the narrow end where most toasters have it. The rig is planar: the dial can only turn about the depth axis, so it has to sit on a face that looks along that axis. Toasters with the controls on the long side exist; the drawing was authored as that view.

## 2. Audit of the state before round 3

- **Rig**: the `slices` joint sat at the worktop (0, 0), 85 mm below the carriage. The lever travelled 45 mm in `toasting` while the carriage travels 40 mm, although both are one rigid part. Joint limits (60 mm) were wider than the mechanism.
- **Anchors**: foot footprints 50 x 30 mm for feet that are 25 mm in reality.
- **Dimensions**: inside the class, unchanged.
- **2D**: feet drawn as 50 x 12 mm slabs; the housing started at the feet with no plinth; slots only 120 mm long; the rear slice was drawn narrower than the front one so that it "peeks out"; lever, dial and button were missing in front and plan although they stand proud of the long face; the housing was 0.17 m deep with the lever only 1.5 mm proud; at 1:100 the feet were thickened to 20 mm and the body raised by 8 mm; at 1:200 both slices were one 90 mm slab.
- **Solids** (`qa-visual.md`: B / B / B, 1:200 thermal C): one banded superellipse loft, feet as o50 revolves with an implied closure (`kernel-traps.md`), lever and button as 8 mm inlays, no plinth, no chamber. Thermal: a slab protruding above the housing at 1:100 and 1:200, the emitter as a box inside the housing. Structure: a ring block, a slab and two floating capsules.
- **Thermal areas**: the bulk pass had set `top-plate` 0.077 (both faces of a closed plate), `control` 0.0053 and `feet` 0.02 (the o50 feet) against the area reviewers' 0.0549, 0.0044 and 0.007.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `rig.slices` | y 0, comY 0.14, joint -0.06 to 0.06 | y 0.085 (the seat of the bread on the carriage), comY 0.055 (same world height), joint -0.04 to 0.04 |
| `rig.lever.joint.minM` | -0.06 | -0.04 |
| `poses.toasting.bones.lever.dy` | -0.045 | -0.04 (same travel as the carriage) |
| `anchors.foot-left`, `foot-right` | footprint 0.05 x 0.03 | 0.027 x 0.027 |
| `thermal.regions.casing.areaM2` | 0.24 | 0.181: measured outer envelope of plinth and wrap up to the cap seam (factor 1.33); the old value fitted a 0.17 m deep box |
| `thermal.regions.top-plate.areaM2` | 0.077 (HEAD 0.0549, notes 0.045) | 0.048: rim, deck and slot rims, underside not counted as the reviewers ruled. Factor 1.6 against the bulk value, 1.14 against the reviewed 0.0549 |
| `thermal.regions.control.areaM2` | 0.0053 (HEAD 0.0044) | 0.0057: lever, dial and button as modelled |
| `thermal.regions.feet.areaM2` | 0.02 (HEAD 0.007) | 0.0079: four real feet; the reviewers called the o50 feet a model defect. Factor 2.5 against the bulk value, 1.13 against the reviewed one |
| `thermal.assumptions[0]` | "cool-touch at about 60 degC ... 95 W on 0.24 m2, about 400 W/m2" | "touchable at about 70 degC ... 95 W on 0.181 m2, about 520 W/m2" (en and de) |
| `provenance` | 3 references, old notes | + `retail-two-slice-toasters-2026`; notes rewritten (construction, 0.048 m2 cap at 520 W/m2) |
| `representations[50,100,200].geometry` | old drawings | redrawn at every scale and view; new shapes `base-tray` (1:50, 1:100), `lever`, `dial`, `cancel-button`, `cord-bush` in front and plan (1:50; lever and dial at 1:100; lever at 1:200), `slice-front` and `slice-rear` instead of `slice` at 1:200 |
| `representations[*].decisions` | old | rewritten per scale, en and de |

`bread` stays 0.054. `dimensions`, materials, emitters, supports, clips and the other poses are unchanged. The two top-plate and feet corrections exceed the C16 factor against the bulk-pass values; they return to the reviewed values, so I applied them and list them here for the coordinator.

## 4. What was modelled

Solid counts body / thermal / structure: 1:50 19 / 16 / 16, 1:100 11 / 10 / 10, 1:200 5 / 8 / 8, 1:500 2 / 6 / 7.

- **1:50 body**: plinth with chamfered foot edge, four tapered feet, crumb-tray front, cord bush, wrap as a 12-section skin, cap rim of 10 sections, deck with two real slots, toasting chamber with two wells over a chromed crumb pan, crust ring and crumb per slice, lever knob with stem, bezel, dial with flared skirt and pointer, cancel button.
- **1:50 thermal**: closed envelope of plinth and wrap (top face under the cap marked), tray front and bush; cap rim and deck with slot rims (underside marked); bread, controls, feet by reference; two outlet patches in the slot mouths around the slices (D2).
- **1:50 structure**: plinth as base frame on the foot pairs (supports), chamber tube with 1 mm walls, guide rod, two lifters, yoke, sleeve as carriage joint, lever arm and its joint on the yoke, dial spindle in its bush on a bracket, masses of appliance and bread.
- **1:100**: plain plinth, o25 feet, wrap of 7 sections, rim of 4, deck 15 mm deep over a plain chamber tube, 15 mm slices, block lever, plain dial. Structure: plinth, tube with 2 mm walls, rod, carriage as one angle, lever arm, spindle and bracket, supports, mass.
- **1:200**: one housing skin from the worktop, 30 mm deck with both slots, 30 mm slices, lever block. Thermal: references plus dial skin, foot runners, one outlet patch. Structure: base frame, tube, carriage angle, lever arm, round spindle, support runners, mass.
- **1:500**: one closed housing mass of four sections and one bread dome. Thermal: skins for cap, lever, feet and outlet. Structure: frame tube on runners, carriage plate, lever bar, spindle stub, mass.
- **2D (E1)**: every outline comes from the same functions as the solids (housing half length and half depth by height, superellipse plan, bread outlines).

## 5. Final check

- `$BUN volumes-check.ts appliances/toaster --product`: PASS, every rule at every scale.
- Silhouette IoU (elevation / front / plan): 1:50 0.998 / 0.990 / 0.989; 1:100 0.995 / 0.987 / 0.990; 1:200 1.000 / 0.996 / 0.997.
- Thermal area deviation (casing / top-plate / bread / control / feet): 1:50 0.2 / 0.5 / 9.1 / 0.6 / 0.2 %; 1:100 0.7 / 17.6 / 12.2 / 7.7 / 1.4 %; totals 1:200 19 %, 1:500 8 %.
- `$BUN r3-validate.ts appliances/toaster`: VALID; `--catalog-only --volumes`: VALID, checkVolumes 0.
- `r3-toaster-suit.ts`: no print-suitability violation at 1:50, 1:100, 1:200.
- Not run: the product test suites (`repo test`), including `🧊️model-frame`, which tessellates the toaster's 1:50 body and requires every solid wound outward.

## 6. What the renders show

- **1:50 body, six views** (`r3-sheet`): front and back show the 152 mm wrap with 22 mm shoulders on the inset plinth, knob, dial and button proud on the control side, the bush on the other. The long side shows dial, button, bezel with the knob at the bottom of its travel, and the tray front. The top shows the cap with its panel seam, both slots with dark rims and the bread. Three-quarter views from both sides read at once as a retro toaster.
- **1:50 pose `popped`**: knob at the top of the bezel, slices 52 mm out with crust and crumb distinct, nothing opens or floats.
- **1:50 thermal, cut**: bread nested in the red envelope, green cap, yellow controls, blue feet.
- **1:50 structure, cut**: chamber tube on the plinth, lifters, yoke with orange sleeve on the rod, lever arm to the knob, spindle with bush, masses.
- **1:100, 1:200, 1:500**: body, thermal and cut structure sheets opened at each scale; the same object with fewer parts. A rotated square spindle at 1:200 was found there and replaced by a round one.
- **Overlays** (`volumes-render`): green at 1:50 except sub-feature specks at dial and bush; 1:100 to 1:500 rendered, numbers as above.
- **2D**: zoomed panels of 1:50, 1:100 and 1:200 opened; controls sit on the correct side in front and plan and follow the pose.

## 7. Still imperfect

- **Idle pose**: the asset hides the bread by sinking the carriage 40 mm with the lever up. The schema cannot remove a part per pose, so in `idle` carriage and lever disagree inside the housing.
- **C31 stand-ins**: the envelope's top face and the deck's underside are marked `sectionFaces` because the product has no cavity flood.
- **Hidden deck edge**: the deck's outer edge inside the rim counts 0.005 m2 towards `top-plate`.
- **Not modelled**: bread guard wires, heating element cards, mains cable, logo, screws; the 2D omits them too.
- **Through the slots at 1:100** the plinth shows as the floor (no crumb pan at that scale).
- **1:500 structure**: the frame tube is wider than the real chamber so that it reaches the support runners.
- **Controls on the long face** is the less common layout (see section 1).

State: DONE
