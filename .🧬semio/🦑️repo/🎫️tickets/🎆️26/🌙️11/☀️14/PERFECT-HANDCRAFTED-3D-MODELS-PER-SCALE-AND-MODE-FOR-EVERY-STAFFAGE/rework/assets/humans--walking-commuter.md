# humans/walking-commuter — rework report

## Pass 1 (second round), 2026-11-22

Scripts (ticket folder): `rework-walking-commuter-build.ts` (rig, poses, clips, anchors, coat, strap, hands, thermal lists, carrying of 2D frames), `rework-walking-commuter-skeleton.ts` (structure list of every scale), `rework-walking-commuter-draw2d.ts` (2D redraw by projection), `rework-walking-commuter-texts.py` (decisions), `rework-walking-commuter-run.sh` (all of them and the validation), `rework-walking-commuter-publish.py`, `rework-walking-commuter-dump.ts` (ruler), `rework-walking-commuter-base.json` (the catalog document as it stood before this pass, taken from the git index, byte size equal to the file of 2026-11-21 16:30; the build starts from it, so it can be run again).

Source and catalog were identical at the start (96 / 122 / 181 solids at 1:50); no earlier report on this asset existed (`rework/01.md` names it only in its title).

### Reference (man, 1.78 m barefoot, 72 kg; 1.80 m with shoes and hair)

| quantity | reference | model before | model now |
|---|---|---|---|
| head, chin to vertex | H / 7.5 = 0.237 m; breadth 0.152, length 0.195 (DIN 33402-2) | 0.233; 0.147 x 0.19 | unchanged |
| hip joint height, shod | 0.530 H + 0.02 = 0.963 (Drillis and Contini) | 0.959 | 0.959 |
| thigh / shank | 0.245 H / 0.246 H = 0.436 / 0.438 | 0.435 / 0.425 | unchanged |
| ankle height, shod | 0.039 H + 0.025 = 0.094 | 0.099 | unchanged |
| L5/S1 above hip axis | 0.095 to 0.10 | 0.10 | 0.10 |
| lumbar spine L5/S1 to T12/L1 | 0.18 | none (one torso bone of 0.495) | `lumbar` 0.18 |
| thoracic spine T12/L1 to C7/T1 | 0.28 to 0.30 | none | `torso` 0.295 |
| C7/T1 height, shod | 0.863 H minus 1 cm + 0.02 = 1.535 | 1.554 | 1.534 |
| cervical spine C7/T1 to atlanto-occipital joint | 0.115 to 0.125 | 0.10 | 0.1198 |
| atlanto-occipital joint to vertex | 0.15 | 0.15 | 0.15 |
| shoulder joint (glenohumeral centre) height, shod | acromion 0.818 H + 0.02 = 1.476 to 1.49, centre 4 cm lower = 1.44 to 1.45 | 1.479 (in the acromion) | 1.449 |
| elbow / wrist height hanging, shod | 1.12 / 0.86 | 1.149 / 0.889 | 1.119 / 0.859 |
| upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H = 0.331 / 0.260 / 0.192 | 0.33 / 0.26 / one hand bone 0.19 | 0.33 / 0.26 / metacarpus 0.095 + middle finger 0.094 |
| phalanges index, middle, ring, little (Buryanov and Kotiuk, scaled to a 0.19 m hand) | 42/24/19, 46/28/20, 43/27/19, 34/19/17 mm; thumb 46/33/25 | capsules glued to the hand bone, right hand without bones | 15 finger bones per hand with these lengths |
| shoulder joint spacing | biacromial 0.40, joints 0.36 to 0.37 | 0.38 | 0.38 (not changed, see remaining) |
| clavicle, sternoclavicular joint to acromion | 0.15 to 0.165 chord, 15 to 20 degrees back | bone absent | 0.175, 16 degrees back, level |
| hip joint spacing | 0.17 to 0.18 | 0.179 | 0.179 |
| bi-iliac breadth / crest height | 0.29 / 1.10 | plates | 0.30 / 1.097 |
| chest depth x breadth, clothed in jacket and overcoat | 0.235 x 0.32 body + 3 cm = 0.27 x 0.36 | 0.265 x 0.345, same depth from hem to shoulder (slab) | 0.283 x 0.36 at 1.36 m |
| back profile (x behind the hip axis) | shoulder blades farthest back, hollow at L3, seat | -0.12 everywhere | -0.146 at 1.43 m, -0.104 at 1.19 m, -0.140 at 0.975 m |
| thigh / knee / calf / hem, trousers | 0.20 / 0.13 / 0.13 / 0.125 deep | 0.20 / 0.136 / 0.136 / 0.125 | unchanged |
| shoe EU 44 | 0.295 x 0.104 | 0.295 x 0.104 | unchanged |
| vertebra pitch C / T / L | 0.017 / 0.020 to 0.029 / 0.036 | one loft with 17 pegs | 0.0171 / 0.020 to 0.029 / 0.036 |
| rib cage | 12 pairs, 1 to 7 on the sternum, 8 to 10 on the arch, 11 and 12 free; greatest breadth 0.29 at rib 8; sternum 0.20 | lofts of an older pass, not touching the spine | as stated, half breadths 0.058 to 0.147 |

### Defects found (sheets read at full size before the work: 1:50 body and 1:50 structure, default pose)

- Rig: 19 bones. One rigid torso from L5/S1 to C7, no shoulder girdle, no pelvis halves, hands of one bone (the fingers were capsules on the hand bone, the right hand an ellipsoid with a thumb of two parts), neck 2 cm short and C7 2 cm high, the shoulder joint 3 cm high (inside the acromion), so elbow and wrist hung 3 cm high.
- Body 1:50: the coat a slab in the side view (same depth from hem to shoulder, no shoulder blades, no hollow of the back); the right hand lay flat on the chest with the forearm bent 145 degrees and the hand bone pointing up, so the fingers could not close round a vertical strap.
- Structure: with the rule `volumes-connected` 17 loose groups at 1:50 (fibulae, bag lugs, ribs, skull parts), 5 at 1:100, 5 at 1:500; spine one loft; skull two ellipsoids and four capsules; pelvis plates.
- 2D: nothing wrong before the rig changed.

### Rebuilt

**Rig** (19 bones before, 54 now). `pelvis` → `lumbar` (new) → `torso` (thorax, id kept) → `neck` → `head`; `torso` → `clavicle-l|r` (new, lateral rest placement from `rig-rest.ts --solve`: sternoclavicular joint [0.038, 1.490, ±0.020] to acromion [-0.012, 1.490, ±0.188]) → `arm` → `forearm` → `hand` (metacarpus 0.095 m, 31 mm lateral of the forearm line where the wrist already was) → thumb (metacarpal, proximal, distal) and four fingers of three phalanges; `pelvis` → `ilium-l|r` (new, sacroiliac joint [-0.053, 1.030, ±0.052] to hip joint) → `thigh` → `shank` → `foot` → `toe`; `bag` on `torso`. Arms and thighs were re-parented with `--solve BONE --parent`, their rest frames are unchanged to 0.1 mm (checked with `rig-rest.ts`). Both sides have the same topology; the right side is the left with `z`, `abductionDeg`, `twistDeg` negated. Finger flexion is rest abduction per phalanx: relaxed on the left (14 to 32 degrees), a fist on the right (84 to 98 degrees at the two first joints). Trunk mass 24.45 kg split 12.35 / 12.1 kg.
- Poses and clip keys: the torso angle is split 40 % lumbar / 60 % thorax; the strap arm (was arm -7, elbow 145, hand +28) is now arm -23, elbow 147, hand -25, so the metacarpus lies 8 degrees above the horizontal and the fist stands upright round the strap; `check-watch` keeps its own arm. Anchor `strap-grip` moved into the fist.
- `dimensions.widthM` 0.40 → 0.425 (the elbow of the strap arm now points back; measured by the product), manifest row too.

**Body.** 1:50: 101 solids. New: `coat` (thorax) and `coat-waist` (lumbar) from 13 and 7 horizontal slices with the profile of the table, joined by hidden tongues; both hands (palm loft, ball of the thumb, ball of the little finger, one capsule per phalanx on its bone); the strap laid from the front lug through the fist, on the chest, over the shoulder, down the back to the rear lug. Carried without change of form into the new frames: collar, scarf, neck, lapels, pockets, buttons (those below the waist now on `lumbar`). Untouched because they measured right: head, hair, face, sleeves, forearms, cuffs, trousers, shoes, bag. 1:100: same coat slices without densifying, hands with real phalanges floored at 15.6 mm (81 solids). 1:200 / 1:500: coat from 4+7 / 3+5 slices, right hand a fist of 100 x 88 x 72 mm / 112 x 92 x 75 mm on the strap, left hand as before.

**Thermal.** References follow the body lists; the three inner coat layers at 1:50 are shells of both coat pieces at the former thicknesses. `areaM2` values untouched; all thermal coverage rules pass.

**Structure** (`rework-walking-commuter-skeleton.ts`, every list written anew): 273 / 141 / 58 / 37 solids. 1:50: braincase loft, facial block, mandible, zygomatic arches, orbits; C1 to C7, T1 to T12, L1 to L5 as bodies with discs, spinous processes and (thoracic, lumbar) transverse processes, on the curve the old spine loft had (lordosis 12 mm, kyphosis 55 mm behind the bone axis), bound to `neck`, `torso`, `lumbar`; 12 rib pairs from inside their vertebral body to the rib angle, round the flank, falling 45 to 100 mm to the front, 1 to 7 to the sternum, 8 to 10 to the costal arch, 11 and 12 free; sternum 0.20 m; clavicle, scapula and scapular spine on the clavicle bones; sacrum on the pelvis; iliac blade, body over the socket, pubic and ischial rami on the ilium bones; femur with head, neck, trochanter, condyles; patella; tibia with plateau; fibula; malleolus; talus, calcaneus, midfoot, 5 metatarsals, 5 toe rays; humerus with head and condyles; ulna and radius running out to the wrist; carpus, 4 metacarpals, every phalanx on its finger bone; heel and sole pads in the shoes; joints and masses. Coarser scales merge (1:100 vertebra bodies without processes, six rib pairs, plates for metatarsus, toes, metacarpus, fingers; 1:200 and 1:500 one rod per spine bone, one rib-cage form, one bar per clavicle and hip bone, one bone per limb segment; at 1:500 femur, tibia and humerus are slender rods of real thickness).

**2D** (`-draw2d.ts`): every filled closed shape of the 12 drawings that stands for body solids was redrawn as the outline of those solids in the default pose and solved into its bone with the product's placement (`inverse` of `r3-project2d.ts`); layer, tone, region, hatch and pen kept. 35/42/27 of 40/49/30 shapes at 1:50. Kept as drawn: strokes (vent, seams, mouth, brows, front edge, fringe) and the strap.

**Texts.** Structure decision of every scale, 1:100 body decision, one added 1:50 decision on rig, coat cut and grip, provenance note (en, de).

### Proof

- `r3-validate.ts --file <candidate> --volumes`: `VALID file/rework-walking-commuter (file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Published with `rework-walking-commuter-publish.py` (members dimensions, rig, anchors, representations, poses, clips, provenance + volumes through `volumes-insert.py`; manifest row). `r3-validate.ts humans/walking-commuter --catalog-only --volumes`: `VALID humans/walking-commuter (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- `validator.md` has no section "Rule changes authors must know" yet; no finding was left open as a rule defect.

### What was looked at, and what was not

Read at full size: 1:50 body and 1:50 structure before the work; after the first build the 1:50 body six views with the rig, the 1:50 three-quarter close-up without rig, and the 1:50 structure six views of the first new skeleton. From these: the back profile, the fist on the strap and the skeleton read as intended; the thumb stood out like a pointing finger (its angles were then changed from 52/-14/-16 with curl 34/30/26 to 34/-22/-24 with curl 48/38/30), and the facial bones floated in front of the braincase (then moved until `volumes-connected` passed).

**Not read: everything after that.** The image reader of this session stopped answering (six attempts over the rest of the pass), so the following were rendered after the last change but never seen by me: the corrected thumb and skull, the girdle bones in the rig overlay, 1:50 thermal, 1:100 / 1:200 / 1:500 body and structure, the poses `check-watch`, `contact-left`, `passing-right`, the clip instants `walk-cycle` 0.3 s and 0.9 s, `glance-at-watch` 1.6 s, `idle-breathing` 1 s, and all four 2D sheets. They are validated only. I do not sign this model.

### Remaining (for the next pass)

1. Look: read every sheet listed above; the pass is unsigned until then.
2. 1:50 body, right hand: thumb position on the fist unseen; the strap stands 4 to 8 cm off the chest between fist and shoulder (the hand pulls it forward); check that it reads as held, not floating.
3. 1:50 body, right arm: the elbow points 13 cm behind the trunk axis and the back run of the strap passes through the upper arm between 1.20 and 1.30 m (hidden inside the sleeve); check `passing-*` and `walk-cycle`, where the trunk leans 4 degrees.
4. Rig: shoulder joints at z ±0.19 (reference ±0.18 to ±0.185), clavicle chord 0.175 m (reference up to 0.165); moving them needs the front and plan shapes of both arm chains shifted by the same amount.
5. 1:50 body, coat: the skirt still swings as two halves on the thighs with a centre strip on the pelvis (older design, kept); the waist cut has a ring seam at 1.235 m; lapels, pockets and buttons were not re-seated on the new front surface (front line kept within 3 mm of the old one, unseen).
6. 1:50 body, head, sleeves, trousers, shoes, bag: measured against the table and kept; not re-sculpted (the face is a sliced head with added nose, chin, lips, lids).
7. Structure all scales: skull is braincase + facial block + mandible (no maxilla/nasal opening); scapula a tapering blade without coracoid; iliac blades are four-section plates; 1:500 spine, clavicle, forearm rods are 75 mm thick by the feature rule.
8. Thermal at 1:100 to 1:500 and the 1:50 shells: by reference and by offset only, never looked at in section.
9. 2D: outlines are projections simplified at 0.9 / 1.8 / 3.5 / 7 mm, not hand-drawn economical polygons; the strap shape is the old drawing (front run now up to 5 cm off); strokes on the coat (vent, side seam, front edge) were carried, not redrawn; drawing decisions texts of the representations still describe the hand "in front of the chest".
10. Clips: finger bones have no keys (hands rigid in every clip); no shrug or pelvis tilt uses the new girdle bones yet.

### Fundamental issues

- The Read tool (image reading) failed with "PreToolUse hook did not respond before its timeout" from about the middle of this pass to its end; a pass that cannot look cannot sign.
- `🧪️tests/🧊️model-skeleton` ("adultLongBones", already reported in `rework/04.md`) may list this asset: at 1:200 and 1:500 `radius-l|r` is merged into `ulna-l|r` and coarse bones are floored at the feature size, as `volumes-feature` demands; the vitest suite was not run.
- `volumes-structure-coverage` demands a member per load-bearing bone at 1:500, and `volumes-slender` refuses a slender member under 300 mm unless 20 times as long as thick: lumbar, thoracic and cervical spine and the clavicles can therefore only be 75 mm rods at 1:500. A rule that let consecutive collinear bones share one slender member would give a true 1:500 skeleton.
- `rework-04-publish.py` hard-codes its candidate folder; a shared `publish <asset> <candidate>` would save every author a copy.

## Pass 2 (second round), 2026-11-22

`rig-lateral-rest.md` exists (read at the start and again before finishing): the girdles of pass 1 (`clavicle-l|r`, `ilium-l|r` on lateral rest placement, arms and thighs hanging from them) are what it describes; nothing had to be re-bound.

### Read at full size this pass (the image reader worked until the last step)

From the catalog as pass 1 left it: 1:50 body (six views with rig, three-quarter and right side without rig), 1:50 structure (six views, three-quarter), 1:50 thermal (six views and the cut sheet), 1:100 / 1:200 / 1:500 body (six views each), 1:100 / 1:200 / 1:500 structure and 1:200 thermal (front, side, three-quarter), all seven non-default poses (front, side, three-quarter), the 2D sheets of 1:50 (candidate), 1:200 and 1:500.

After this pass's skeleton change: 1:50 structure front and right side without rig, 1:100 / 1:200 / 1:500 structure (front, side, back, three-quarter), clips `walk-cycle` 0.3 s and 0.9 s, `glance-at-watch` 1.6 s, `idle-breathing` 1 s, pose `check-watch`.

### Judged

- Rig and 1:50 body: proportions agree with the reference table of pass 1 from all six directions (fingertip of the hanging hand at 0.68 m, coat shoulders 0.52 m, head 0.233 m). The girdle bones sit where the clavicles and hip bones are. The thumb lies on the fist; the fist stands upright round the strap. The elbow of the strap arm is 13 cm behind the trunk axis: with the fist on the chest in front of the shoulder and an elbow that cannot flex beyond 147 degrees this is where the elbow has to be (15 degrees less would need 156 degrees of flexion), kept. The strap stands 4 to 8 cm off the chest between fist and shoulder, as a strap does that a hand pulls forward; kept. Walk poses and clip: natural, the skirt halves follow the thighs and show a stepped hem at the front in the contact poses.
- 1:50 structure before: the face was a block with two balls (zygomatic ellipsoids and orbit lumps standing proud of the braincase), no eye sockets, no nasal aperture. Coarse scales: spine, clavicle and forearm rods thickened to the feature, radius missing at 1:200 and 1:500.
- Thermal 1:50: reads as the same object; in the cut the tee, shirt, jacket and overcoat shells nest in the trunk and in the sleeves, briefs inside the trousers, socks inside the shoes. 1:200 thermal reads as the same object.
- 2D 1:50, 1:200, 1:500: agree with the model (strap through the fist, coat profile, bag at the hip).

### Rebuilt

- **Skull, 1:50** (`rework-walking-commuter-skeleton.ts`): the braincase retreats below the brow (front of the loft at -0.038 / -0.053 m at 8 / 30 mm above the atlanto-occipital joint, forehead from 47 mm up), and the face is built bone by bone in front of it: two maxillae, their frontal processes beside a nasal aperture of 20 x 35 mm, alveolar arch, zygomatic bones with the lateral orbital rim, infraorbital rims, zygomatic arches to the temporal bone, brow ridges, nasal bones, vomer; the mandible body is 32 mm high at the chin. The orbits are now open sockets between brow, zygoma, nasal process and maxilla. 285 solids.
- **Skull, 1:100**: maxillary block ending under the orbits, zygomatic bars, brow bar and nasal bridge of 15 mm (147 solids).
- **Long bones, every scale**: humerus, ulna, radius, femur and tibia keep one shaft section from 1:50 to 1:500 (`slender: true` on the coarse scales, validator item 2); radius and ulna are both present at 1:200 and 1:500.
- **Spine, 1:500**: one rod of true section (21 to 12.5 mm) from L5/S1 to the atlas on `torso` with `members: ["lumbar", "neck"]` (validator item 9) instead of three 75 mm rods; clavicles at 1:200 and 1:500 at their true section (`slender`).
- **Clip** `idle-breathing`: both clavicles rise 1.5 degrees at the top of the breath (first use of the girdle bones in a clip).
- Pose `check-watch`: a lower, more level forearm (16 / 98, 19 / 104, 20 / 108 degrees) was tried and read better, but the far hand then prints 0.169 to 0.195 mm at 1:1000 (`range-end-feature`); a turn of the upper arm about its axis is refused by `lateral-ambiguous` (flat 2D shapes on the forearm). The pose stays at 22 / 112 degrees.

### Proof

`r3-validate.ts --file <candidate> --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`; published; `r3-validate.ts humans/walking-commuter --catalog-only --volumes`: `VALID humans/walking-commuter (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

One publish of this pass went out with two print findings (the 16 / 98 watch pose; the shell chain did not stop on `INVALID`); it was replaced about ten minutes later by the valid state above.

### Not read after the last publish

The image reader stopped answering again at the very end (four attempts). The last sheets (all seven poses, `walk-cycle` 0.3 / 0.9 s, `glance-at-watch` 1.6 / 3.4 s, `idle-breathing` 2 s, rendered from the published catalog) were therefore not read. Against what was read earlier in this pass the published state differs only in the 1.5 degree clavicle lift of `idle-breathing` at 2 s; `check-watch` is back at the values seen in `glance-at-watch` 1.6 s. Also never read: 1:100 and 1:500 thermal, the 1:100 2D sheet, the coarse-scale poses. I do not sign the model yet.

### Remaining (for the next pass)

1. Look: `idle-breathing` at 2 s (clavicle lift), 1:100 and 1:500 thermal sheets and cuts, the 1:100 2D sheet.
2. 1:50 body, pose `check-watch` / clip `glance-at-watch`: the far forearm rises to chin height (22 / 112 degrees) and reads as a raised hand rather than a wrist held level; a level forearm fails `range-end-feature` at 1:1000 for `hand-l` (0.17 to 0.195 mm) unless the 1:500 front and elevation shapes of the far hand and forearm are redrawn wider.
3. Rig: shoulder joints at z ±0.19 m (reference ±0.18 to 0.185), clavicle chord 0.175 m (reference up to 0.165); moving them means shifting front and plan shapes of both arm chains.
4. 1:50 body, coat: ring seam at the waist cut (1.235 m) shows as a line; skirt halves on the thighs give a stepped hem in the contact poses; the strap is a polyline of 16 stations (angular over the shoulder in the side view) and its back run passes through the upper arm between 1.20 and 1.30 m inside the sleeve.
5. 1:50 body, head, face, sleeves, trousers, shoes, bag: measured and seen, kept, not re-sculpted.
6. Structure 1:50: iliac blades are bands of eight sections without the fan of the wing; scapula a tapering blade; ribs of 1:100 are six pairs of 15 mm and read as a lattice; rib cage of 1:200 and 1:500 is one shield-shaped form; joints at 1:500 are 75 mm balls.
7. 2D: outlines are simplified projections, not hand-drawn economical polygons; coat strokes (vent, side seam, front edge) carried, not redrawn.
8. Clips: finger bones have no keys; `walk-cycle` does not use the clavicle or ilium bones (no counter-rotation of shoulder girdle and pelvis; the rig's poses are sagittal, a transverse rotation would need solids instead of flat 2D shapes on the limbs, see `lateral-ambiguous`).

### Fundamental issues (pass 2)

- The Read tool for images fails intermittently with "PreToolUse hook did not respond before its timeout"; it worked for 25 sheets and then stopped.
- `r3-sheet.ts` exits with a bare "Bun v1.4.2" line now and then under load (19 authors on one host); a retry succeeds. `r3-2d.ts` hung once at 0 % CPU for 15 minutes on the catalog form (other authors' `r3-2d` processes hung at the same time); the `--file` form ran in seconds.
- `rework-walking-commuter-publish.py` does not validate before it writes; a publish tool that refuses an `INVALID` candidate would have prevented the ten-minute invalid state of this pass.
- `lateral-ambiguous` forbids every pose that turns a limb out of the sagittal plane as long as the 2D shapes of that limb are flat: a watch read across the body, a shoulder-girdle counter-rotation in the gait and a pelvic rotation cannot be authored for this asset without redrawing all limb shapes as solids.
