# Round 3 — appliances/shower-head

State on 2026-11-20: resumed, finished and written to the catalog. One check line is not green on purpose: `volumes-thermal-coverage` 1:50 for region `head` (17 % against 15 %), because C16 forbids the honest area (section 6).

## 0. Resume

- Earlier round-3 work existed: `r3-shower-head-kit.py`, `r3-shower-head-model.py`, `r3-shower-head-roots.py` (03:17 to 03:19) and a candidate under `🗑️generated/r3-shower-head/`. There was no report, and the roots had never reached the catalog.
- At the start the catalog still had the old rig (rail bone lying horizontal, no head bone). Its `volumes` were the 03:55 bulk pass (`digital-models.py`): 46 / 38 / 15 / 11 body solids on the old rig, boxes for brackets, a square head, a capsule-chain hose, and a new authored 1:500.
- Kept from the bulk pass: the fact that 1:500 is now an authored digital scale. Nothing else: its solids sit on the wrong rig and are the forms the dev rejected.
- Re-running the earlier chain gave 8 `validateAsset` findings (lateral dimension) and 11 volume findings (silhouette at 1:100 and 1:200, bounds, five areas). All were worked through below.

## 1. Reference

| measure | value | source |
|---|---|---|
| spray disc | 110 mm across | hansgrohe Croma Select S 110 (art. 26560400), `provenance.references` |
| disc thickness | 26 mm with domed back and rounded rim | product class, authored estimate (the asset's notes said 27 mm) |
| rail | 22 mm tube, 1.2 mm wall, 0.73 m between the end caps | hansgrohe Unica'Croma 0.65 m (art. 26503000, pipe 669 mm) |
| bracket axes | 0.686 m apart, rail centre 60 mm from the wall | same product data |
| rosette, end socket | 44 mm, 30 mm | product class, authored estimate |
| holder | 34 mm sleeve, clamp button, tilting conical socket 45 mm in front of the rail | product class |
| handle | 25 to 29 mm, 0.19 m from the tilt axis to the disc centre | product class |
| handle to wall | 72 degrees above the horizontal at rest; disc cranked 15 degrees, spray 33 degrees below the horizontal | the asset's own emitter direction (-33 degrees), kept |
| hose | 13 mm outside, 9 mm bore, 1.25 m, conical G 1/2 nuts | hansgrohe Isiflex (art. 28272000) |
| wall elbow | 60 mm escutcheon, 34 mm body projecting 60 mm, outlet G 1/2 down, quarter-turn lever | hansgrohe Fixfit Stop class, authored estimate |
| flow, temperature | 12 and 9 L/min, 38 degC | EN 1112, DIN 1988-300 (unchanged) |

## 2. Audit against the reference

- **Rig**: the `rail` bone ran horizontally out of the wall (angle 0, length 0.81) while the rail stands vertical. `holder`, `shower` and `elbow` hung on it with world offsets; the `shower` bone had angle 0 and length 0.2, so the handle was not on its bone. There was no bone for the spray axis, so the emitter anchors sat at computed points in the air.
- **Dimensions**: `widthM` 0.238 and `depthM` 0.2 belonged to a hose loop standing 0.24 m out of the wall.
- **Hose**: looped forward out of the wall plane. A hose hanging between two nuts lies in the vertical plane through them.
- **2D**: drawn for that rig, with box brackets and a square head in plan.
- **Solids** (qa-visual B/C/B): box brackets, square head block, capsule hose with corners, floating supports and mass, thermal with nothing inside.

## 3. Root fixes (old to new)

| member | old | new |
|---|---|---|
| `rig.bones.rail` | angle 0 | angle 90 (up the wall), length 0.81 |
| `rig.bones.elbow` | x 0.03, y 0.04 | on the wall at height 0.04, 0.1 m right of the rail (`z` -0.1), pointing out of the wall |
| `rig.bones.lever` | at the elbow origin | on the valve axis 40 mm from the wall |
| `rig.bones.holder` | x 0.09, y 0.62, prismatic axis 90 | on the rail axis at 0.62, length 0.045, prismatic along the rail (-0.47 to 0.12 m) |
| `rig.bones.shower` | angle 0, length 0.2, revolute -40 to 70 | at the tilt axis, 72 degrees, length 0.24, revolute -40 to 10 |
| `rig.bones.head` | none | new, fixed on `shower` at (0.188, -0.021), along the spray axis |
| bone masses | centres off the parts | rail (0.445, -0.055), elbow (0.0545, 0.0168), holder (0.015, 0), shower (0.134, -0.008) |
| `anchors` | wall-low y 0.095, wall-top y 0.79, nozzle and steam-top on `shower` | wall-low 0.102 and wall-top 0.788 on the bracket axes (footprint 44 mm), nozzle at the face centre and steam-top on the upper rim, both on `head` |
| `dimensions` | 1.12 / 0.238 / 0.2 | 1.12 / 0.245 / 0.191 (envelope of the set in the running pose; elbow 0.1 m beside the rail) |
| `poses[*].root.y` | 0.9 | 1.1 (elbow 1.14 m, disc about 1.9 m above the floor) |
| `thermal.emitters[*].directionDeg` | -33, 90 | 0 and 123 in the frame of the `head` bone (the same world directions) |
| `thermal.regions[*].areaM2` | valve 0.025, hose 0.052, handle 0.022, head 0.0454 | 0.0167, 0.051, 0.0147, 0.0303 (section 6); rail 0.083 unchanged |
| `thermal.regions.hose.characteristicLengthM` | 0.012 | 0.013 |
| `summary`, `provenance.notes`, `provenance.references`, `structure.assumptions` | old layout | rewritten for the corrected set, hansgrohe reference with article numbers |
| `representations[50,100,200].geometry` and `decisions` | 27 / 23 / 15 shapes | 54 / 34 / 21 shapes redrawn under E1 as the views of each scale's body; texts rewritten (en, de) |

`representations.500` stays unavailable (22 mm rail is 0.04 mm on paper); `volumes.500` is an authored digital model.

## 4. What was modelled

Every profile and section is stated by hand in `r3-shower-head-model.py`, with the measures and frames in `r3-shower-head-kit.py`.

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 26 | 33 | 19 |
| 1:100 | 13 | 17 | 14 |
| 1:200 | 8 | 11 | 11 |
| 1:500 | 6 | 9 | 8 |

### 1:50
- **Body**
  - Rail tube; two brackets, each with rosette, arm and end socket with a domed cap.
  - Holder: sleeve, clamp button, a forked arm (U-shaped extrude) and two pivot pins.
  - Socket: a conical clip with a 14 mm slot at the front (C-section extrude with taper), turning between the fork cheeks.
  - Hand shower: conical hose nut, handle seated in the clip, a neck loft that flattens from 26 mm to a 54 mm tongue on the back of the disc, the disc with domed back and rounded rim, and the nozzle field as a texture plate.
  - Wall elbow: escutcheon, valve body with domed end, outlet, conical nut, lever boss, hub and lever.
  - Hose: one loft through 42 circles along a hand-set Bezier bight of 1.2 m.
- **Thermal**
  - Each of the five surface regions is its body solids (every region has one layer). Nuts count to handle and valve, the neck to the head.
  - The hidden `spray` region is the water path: valve bore, outlet, hose bore of 9 mm, handle and neck, chamber behind the nozzle plate.
  - Emitters: the nozzle field as a plate 1 mm proud of the face, and a patch on the upper rim for the plume.
- **Structure**
  - Rail tube 22 x 1.2 mm (hollow), two bracket studs, pin at the lower and roller at the upper bracket.
  - Holder ring and fork, pivot pin as joint, handle tube 21 x 2 mm, neck spine, nozzle plate.
  - Elbow tube with outlet, lever axis as joint, hose as hanging member, three masses inside their parts.

### 1:100 (15 mm)
- Each bracket is one turned piece plus a plain socket; the elbow is one turned body with a 25 mm outlet cylinder.
- Holder: sleeve, button and one block for arm and hinge.
- Hand shower: one turned handle (taking in socket and nut) that ends as a dome in the lower back of a plain lens.
- Hose 15 mm, not slender (section 7).
- Structure: rod rail on two studs, holder bar, handle rod, hose, two joints, three masses.

### 1:200 (30 mm)
- Pole of 30 mm with two wall stubs, stepped elbow block, holder block, handle rod with domed end, disc of 110 x 30 mm.
- Hose 30 mm, run on up into the elbow because the outlet is merged there.
- Thermal: the hose surface is its own true 13 mm solid inside the 30 mm body hose, so the area total stays honest.
- Structure: slender rail of its true 22 mm on two stubs, elbow stub, holder bar, slender handle rod, three masses.

### 1:500 (digital, no drawing)
- Six solids: rail and both brackets as one bent bar of 22 mm with mitred bends (slender), elbow cone stump, holder block, handle, flat disc, hose as a slender line of 13 mm.
- Structure: the bent bar itself, holder block, elbow stub, slender shower rod, two supports, two masses.

## 5. Checks

- `r3-validate.ts appliances/shower-head`: `VALID ... schema 0, validateAsset 0`.
- `r3-validate.ts appliances/shower-head --volumes`: one finding, the `head` area at 1:50 (section 6).
- `volumes-check.ts appliances/shower-head --product`: `FAIL: 1 measurement(s) over 1 rule line(s)`; every other rule is ok at all four scales.

| scale | IoU elevation / front / plan | outer areas rail / valve / hose / handle / head |
|---|---|---|
| 1:50 | 0.978 / 0.984 / 0.978 | 0.0808 / 0.0161 / 0.0489 / 0.0138 / 0.0251 |
| 1:100 | 0.988 / 0.982 / 0.988 | 0.0772 / 0.0137 / 0.0564 / 0.0177 / 0.0226 |
| 1:200 | 0.994 / 0.990 / 0.988 | total deviation 1.0 % |
| 1:500 | no drawing | total deviation 14.1 % |

- Written to the catalog with `r3-shower-head-apply.py`, not `volumes-insert.py`: rig, drawings and volumes had to change in one write because the volumes are stated in the corrected bone frames. The catalog file equals the regenerated candidate, and its `volumes` equal `volumes/appliances/shower-head.json`.

## 6. Areas (C16)

Honest outer areas of the 1:50 model against the values found, with hand calculations in `area-review.md` (entry of 2026-11-20):

| region | before | honest | applied |
|---|---|---|---|
| rail | 0.083 | 0.0808 | unchanged |
| valve | 0.025 | 0.0161 | 0.0167 (factor 1.5, the limit) |
| hose | 0.052 | 0.0489 (13 mm x 1.25 m = 0.051) | 0.051 |
| handle | 0.022 | 0.0138 | 0.0147 (the limit) |
| head | 0.0454 | 0.0251 | 0.0303 (the limit) |

- The old head value came from a round-1 loft of 137 mm; the product's disc is 110 mm.
- With the limit value the head deviates 17 % and the check fails. Applying 0.0251 makes it pass; that is the area reviewers' decision, not mine.

## 7. Tool findings (`tool-requests.md`, 2026-11-20)

- **Slender bent hose.** The product's centre line of the hose loft wanders 3 to 4 mm and has side twigs. With `slender: true` the 1:100 elevation read IoU 0.882; the same loft not slender reads 0.986. So the hose is at the smallest feature where a drawing exists (15 and 30 mm) and slender at its true 13 mm only at 1:500. Evidence script: `r3-shower-head-centreline.ts`.
- **C31 not measured.** The bore of the hollow rail tube was counted as outer surface (0.112 against 0.081). The thermal rail tube is therefore the body rod; the hollow tube is in the structure list.

## 8. What I saw

Tools: `r3-sheet.ts`, `volumes-render.ts` overlays, `r3-2d.ts`, and two own close-up tools because the set is 1.1 m tall and 22 mm thick (`r3-shower-head-zoom.ts` for solids, `r3-shower-head-view2d.ts` for the drawings).

- **Baseline (bulk pass, 1:50 sheet)**: rail bone lying horizontal, box brackets, square head, hose looping forward.
- **1:50 body**, six directions plus close-ups of hand shower, holder and elbow (turn 0, 90, 50/28, 230/20, 270, top):
  - Side: rail between two brackets, forked holder with pivot pin, handle seated in the slotted clip, neck running onto the back of the disc.
  - Front: round disc with dark nozzle field, handle, fork with both pins, clamp button on the left.
  - Back and top: the tongue lies flat on the dome and ends blunt; clip slot open to the front; fork clear of the nut.
  - Elbow: escutcheon, domed body, lever on its boss at the right, conical nut on the outlet.
  - First neck attempt was twisted (section planes flipped); fixed with stated section planes and looked at again.
- **1:50 thermal**, cut from the side and front, and outside from two directions: water core through valve, hose, handle, neck and head chamber; regions coloured as bound; emitter plate on the face and patch on the rim.
- **1:50 structure**, four directions: continuous path wall, stud, tube, ring, fork, pin, handle tube, neck spine, plate; masses inside the parts.
- **1:50 pose `off`** (two close-ups) and **clip `eco-and-off` at 70 s** (sheet): lever down and level, shower upright, everything stays closed.
- **1:100 body** (hand shower and elbow, four directions each), **thermal cut**, **structure**: reads as the same set with merged parts; the handle dome meets the lower back of the lens.
- **1:200 body** (four directions and whole), **thermal cut**, **structure**: pole, stubs, block holder, fat hose with its bight. Fixed after looking: a handle spike through the back of the disc, the water chamber poking 2 mm out of the head, rail member without wall stubs.
- **1:500 body** (sheet and close-ups), **thermal cut**, **structure**: bent bar, holder, handle, disc, elbow, hose line. Fixed after looking: the bent bar pinched in the middle (twisted sections), hose not reaching the elbow, shower member and rail mass floating.
- **2D** at 1:50 (enlarged, all three views and crops), 1:100 and 1:200 (enlarged): clean outlines, true to the body.

## 9. Still imperfect

- `head` area check at 1:50 fails by 2 points until the reviewers apply 0.0251 (section 6).
- The hose is 15 mm at 1:100 and 30 mm at 1:200 instead of 13 mm (section 7); at 1:200 it is as thick as the pole.
- The hose is rigid to the elbow: when the shower tilts 8 degrees the nut moves 5 mm inside the hose end; when the holder slides, the hose does not follow.
- The mode selector, bracket screws and single nozzles are not modelled (below 7.5 mm).
- The renderers tessellate the 110 mm disc as a 14-gon at 1:50; that is the tools' quality, not the solid (a revolve).
- At 1:200 the product draws the light-solid shapes without outlines and the hose as a heavy stroke, so the hose dominates that drawing.

## 10. Files

- Scripts: `r3-shower-head-kit.py`, `r3-shower-head-model.py`, `r3-shower-head-roots.py`, `r3-shower-head-apply.py`, `r3-shower-head-zoom.ts`, `r3-shower-head-view2d.ts`, `r3-shower-head-crop.ts`, `r3-shower-head-stack.ts`, `r3-shower-head-centreline.ts`.
- Source: `volumes/appliances/shower-head.json`.
- Catalog: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/shower-head/🔣️.json`.
- Appended: `area-review.md`, `tool-requests.md`.
- `🗑️generated/r3-shower-head/` is removed; `python3 r3-shower-head-model.py && python3 r3-shower-head-roots.py` rebuilds the candidate.

State: finished and in the catalog, with the one failing area line of section 6 and the limits of section 9.

## Round 5 handcraft (2026-11-21)

Script: `r5-shower-head-model.py` (imports the round-3 model, replaces the 1:50 neck, writes the volumes source). No root change outside `volumes`.

### Reference and model against it

| quantity | reference | model 1:50 | deviation |
|---|---|---|---|
| spray disc | 110 mm (hansgrohe Croma Select S 110, 26560400) | 110 mm | 0 % |
| rail tube | 22 mm (Unica'Croma 0.65 m, 26503000) | 22 mm | 0 % |
| bracket axes | 0.686 m | 0.686 m | 0 % |
| rail centre from wall | 60 mm | 60 mm | 0 % |
| hose | 13 mm, 1.25 m (Isiflex 28272000) | 13 mm, 1.20 m bight between the nuts | -4 % length (the nuts take the rest) |
| handle | 25 to 29 mm | 25 to 29 mm | 0 % |
| neck to disc | flows onto the lower back of the head | 26 to 31 mm, sinks into the dome | - |

`volumes-bounds` 1:50: 1.4 / 0.6 / 0 mm.

### Grades (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | B+ (neck a flat 54 mm tongue plate lying 6 mm proud on the back of the disc, polygon outline visible) / A- / A- | A- / A- / A- |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | B+ (hose at 30 mm as thick as the pole) / A- / B+ | unchanged |
| 1:500 | B+ / B+ / B+ | unchanged |

### Changes

- 1:50 neck: six stations stated against the back surface of the disc (dome at v -0.0017 at u 0.139, 0.0 at 0.152, -0.0008 at 0.169): it widens from 26 to 31 mm, flattens to 9 mm and ends 2 mm under the dome, so it runs onto the lower back and sinks in. Decision text en/de updated.

### Looked at (opened)

Before: every scale's body, thermal cut, structure, poses off and eco, clips turn-on-and-run t 150 s, eco-and-off t 400 s, 1:50 close-ups of hand shower and elbow. After: 1:50 close-up (four directions), 1:50 pose off, 1:50 thermal cut. 2D unchanged (round-3 enlargements; `volumes-silhouette` 1:50 0.973 / 0.989 / 0.986).

### Checks

`volumes-check.ts --product`: FAIL on one line only, `volumes-thermal-coverage` 1:50 `head` 0.0252 against 0.0303 (16.9 %; C16 cap of round 3, proposal 0.0252 in `area-review.md`). `r3-validate.ts --catalog-only --volumes`: schema 0, validateAsset 0, checkVolumes 1 (the same line). Spliced.

### Remaining

- Rig: bone `rail` runs along the wall plane, 60 mm behind the tube axis (the sliding `holder` joint is on the axis, so motion is right); moving the root bone onto the axis means re-framing every rail solid and 2D shape, not done.
- Hose 15 mm at 1:100 and 30 mm at 1:200 (slender centre-line tool defect, round 3 section 7); hose rigid when the holder slides.
- `head` area (area reviewers).
