# humans/worker-carrying-box — running rework report

## Pass 1 (second round)

Two agents worked under this heading. The first one (cut off at 04:12 without a report) left `rework-wcb-girdles.py` and `rework-wcb-texts.py`: clavicles and hip bones as rig bones on the lateral rest placement, girdle solids re-expressed in their frames, long bones as `slender` members, decisions. The second one (this text) found source and catalog identical and `VALID`, judged that state by rendering, and continued with `rework-wcb-form.py`, `rework-wcb-coarse500.py` and `rework-wcb-montage.ts` (reading tool: panels of several `r3-sheet` PNGs in one image).

### Reference (male, stature 1.76 m barefoot, 84 kg, plus 30 mm sole and helmet = 1.82 m; DIN 33402-2 P50, Drillis-Contini, de Leva 1996)

| Measure | Reality | Model (rig rest frames, `rig-rest.ts`) | Verdict |
|---|---|---|---|
| Hip joint height (0.530 H) | 0.93 m | 0.94 m above the toe joint line | right |
| Hip joint spacing | 0.17 to 0.18 m | 0.179 m (thigh origins at z ±0.0895) | right |
| Hip joint to shoulder joint (0.288 H) | 0.507 m | 0.513 m | right |
| Shoulder joint centres | ±0.17 to 0.18 m | ±0.180 m | right |
| Acromioclavicular joints / biacromial | ±0.18 / 0.40 m | clavicle tip ±0.176 m, yoke 0.40 m | right |
| Clavicle length | 0.15 to 0.17 m | 0.166 m, sternal end 22 mm beside the midline, running 19 degrees back and 10 degrees up | right |
| Hip bone, sacroiliac to hip joint | 0.10 to 0.11 m | 0.106 m, from z ±0.046 down and out to ±0.0895 | right |
| Upper arm / forearm / hand | 0.327 / 0.257 / 0.190 m (Drillis), 0.285 / 0.272 / 0.190 m (de Leva) | 0.3135 / 0.2731 / 0.182 m, sum 0.769 m against 0.774 m | right, between the two tables |
| Thigh / shank (0.245 H / 0.246 H) | 0.431 / 0.433 m | 0.420 / 0.425 m | 2 % short, see ankle |
| Ankle height barefoot (0.039 H) | 0.07 m | 0.096 m from ankle to toe joint line | about 20 mm high; thigh and shank are short by the same sum |
| Neck (C7 to skull base) / head | 0.11 / 0.24 m to the vertex | 0.115 m / head bone 0.129 m to the head centre line, vertex at 1.76 m | right |
| Joint order, both sides | pelvis, hip bone, thigh, shank, foot, forefoot; trunk, clavicle, upper arm, forearm, hand | as listed, left and right mirror images (z, abduction, twist negated) | right |
| Trunk | lumbar and thoracic spine move separately | one bone from L5/S1 to C7 | wrong, open |
| Gluteal mass | 0.20 m high, 0.17 m wide, 20 to 30 mm proud of the sacrum | two near-spheres of radius 0.08 to 0.09 m reaching beyond the seat at the side | wrong, rebuilt |
| Rib cage | twelve rib pairs, 12 to 15 mm deep, closed basket 0.28 m wide and 0.19 m deep at rib 7 | 1:50 twelve pairs; 1:100 four hoops; 1:200 three hoops; 1:500 a wire outline of 9 mm | coarse scales wrong, rebuilt |
| Carton | 0.40 x 0.30 x 0.30 m, 20 kg, hand holes | same, one half of the mass on each hand bone | right |

### Looked at (every image read at the size stated)

- Before any change: six-view sheets of 1:50 body, thermal and structure at full size; 1:100, 1:200 and 1:500 of all three lists as four-panel montages (front, right, back, three-quarter); all 14 other poses and two instants of each of the three clips at 1:50 as montages (two to four panels per pose); 1:50 front and rear three-quarter alone at full size; 1:500 three-quarter alone; the 2D drawings 1:50.
- After the last change: the same set again (all four scales and three lists, 14 poses, six clip instants, structure in `squat-grip`, thermal 1:50 with `--cut`). The walking poses were read in side and back view only the second time.

### Defects found

1. Body 1:50: the buttocks were two ellipsoids that showed as balls below the vest hem from behind and beside the hips in the three-quarter view, and as two spheres in the squat.
2. Structure 1:100 and 1:200: ribs as four and three loose hoops per side. Structure 1:500: the rib cage as a 9 mm wire outline.
3. Body 1:500: sleeve starting as a faceted wedge at the shoulder, a step at the knee between two-section thigh and shank, a boxy seat.
4. Not defects, checked: segment lengths and joint order (table), both hands with four three-part fingers and a thumb, boots with shaft, sole, toe cap and laces, face with nose, lips, chin, ears, eyes and brows, helmet with rim, peak, ridge and headband, shoulder yoke and sleeve caps, skeleton 1:50 (skull with mandible, spine with processes, twelve rib pairs with cartilages, sternum, clavicles, scapulae, sacrum, hip bones, long bones with epiphyses, hand and foot bones).

### Rebuilt in this pass

- Rig (first agent): `clavicle-l/-r` under `torso` and `hip-bone-l/-r` under `pelvis` on the lateral rest placement; upper arms and thighs re-parented without moving. I checked the rest frames with `rig-rest.ts` (table above) and rendered every pose and clip: the limbs leave from the tips of the girdle bones in all of them.
- Body 1:50, thermal 1:50 (by reference): each buttock is a loft through eight sections from the gluteal fold to the iliac crest, centre 82 mm beside the midline, widest 0.18 m, 23 mm proud of the seat, inside the breadth of the seat (`rework-wcb-form.py`).
- Structure 1:100: all twelve rib pairs, seven costal cartilages per side and the costal arches at a 15 mm section in place of the four hoops.
- Structure 1:200 and 1:500: the rib cage as one closed basket (eight and five sections from the first rib to the costal margin, 0.276 m wide and 0.188 m deep at rib 7) around the thoracic spine; the lumbar spine stays visible between basket and pelvis.
- Body 1:500 (thermal by reference): sleeves with a rounded shoulder cap (five sections), thigh and shank through three sections each without a knee step, seat through four rounded sections (`rework-wcb-coarse500.py`).
- Decisions: the sentences on ribs, rib cage, gluteal masses and the 1:500 limbs in English and German.
- 2D drawings: not redrawn. The silhouette rules pass against the changed body and the 1:50 drawings were read after the change; 1:100, 1:200 and 1:500 drawings were not rendered.

### Validation

`r3-validate.ts humans/worker-carrying-box --catalog-only --volumes` → `VALID humans/worker-carrying-box (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (after the last publish with `volumes-insert.py`). One finding on the way (`volumes-monotonic`, 1:500 thermal not simpler than 1:200) was met by using fewer sections at 1:500, not by touching the rule.

### Still imperfect (open list for the next pass)

- Rig, all scales and lists: one trunk bone from L5/S1 to C7. A lumbar and a thoracic bone are missing; in the squat and set-down poses the back bends at the lumbosacral joint only and the skeleton shows a rigid straight spine. Splitting it means re-binding every trunk solid of three lists at four scales, the trunk 2D shapes, anchors, masses and all 15 poses; not started.
- Rig: ankle about 20 mm too high (0.096 m to the toe joint line against 0.07 to 0.08 m), thigh and shank together 20 mm short. Not changed because every walk pose and foot contact is solved on these lengths.
- Pose `carrying-box-chest`: the elbows stand about 30 degrees out from the trunk. Plausible for a grip in side hand holes, but not re-solved or compared with a tighter carry.
- Body 1:50, squat poses from behind: the gluteal lofts still read as two pads on the seat; seat and buttocks are not one continuous form with the thighs.
- Body 1:50: face is narrow with very small eyes; trouser legs are near-straight tubes below the knee (work trousers, calf form not shown).
- Structure 1:200: the rib basket ends in a flat cut at the costal margin (no infrasternal angle).
- Structure, hip bones (first-round state, kept): two flat plates per side at 1:50, no acetabular rim.
- Thermal 1:100, 1:200, 1:500: one surface per region by reference to the body solids, clothing layers not nested (they are 1 to 3 mm, below those features). At 1:500 the breath outlets of 75 mm read as a ball on the face.
- 2D: drawings of 1:100, 1:200, 1:500 and of the other poses not rendered in this pass.
- `r5_worker_carrying_box_*.py`, `r3_worker_carrying_box_*.py` and `rework-06-worker-carrying-box.py` (earlier generators) describe the state before the girdles; running them again would undo this pass.

### Fundamental issues (outside this asset)

- `r3-sheet.ts --list thermal` draws every region that has no `layer` on its solids in grey (coarse scales), so regions cannot be told apart there; only head skin, hair and the carton keep a colour.
- `python3` on this host has no imaging module and no ImageMagick; reading many sheets at once needed `rework-wcb-montage.ts`. A `--panels` or contact-sheet option in `r3-sheet.ts` would serve every author.
