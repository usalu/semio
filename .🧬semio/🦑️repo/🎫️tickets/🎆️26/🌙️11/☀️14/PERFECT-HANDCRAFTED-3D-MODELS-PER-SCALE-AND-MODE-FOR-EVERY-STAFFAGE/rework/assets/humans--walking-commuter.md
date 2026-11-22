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
