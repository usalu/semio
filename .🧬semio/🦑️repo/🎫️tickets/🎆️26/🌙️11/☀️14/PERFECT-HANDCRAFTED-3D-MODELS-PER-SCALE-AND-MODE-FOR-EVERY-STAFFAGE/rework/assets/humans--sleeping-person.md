# humans/sleeping-person — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Rig girdles, the skeleton of all four scales, one thermal area and the decisions were rebuilt and published; body, duvet, thermal solids and 2D drawings were judged but not rebuilt (open list at the end).

Edit script (the chisel, hand-chosen numbers, idempotent): `T/rework-sleeping-person-build.py candidate|publish`.

### State found

- No earlier report for this asset (`rework/03.md`: "not started").
- Source `T/volumes/humans/sleeping-person.json` (01:45) differed from the catalog at 1:50 body and thermal only: the source had ears, ear lobes, a nape hair mass and separate nose wings where the catalog had brow, cheeks, chin and a hair spread. Rendered the source state (the tools merge it); it reads as the better head, so the source was kept and published. Catalog and source agree now.
- Before: `r3-validate --volumes` INVALID, 17 volume findings (12 `volumes-feature` on the four hem supports, 4 `volumes-connected`, 1 `volumes-thermal-coverage`).

### Reference table (woman, stature 1.68 to 1.70 m, about 60 kg; Drillis-Contini fractions, DIN 33402 breadths, de Leva masses as in the asset)

| Measure | Reference | Model | Judgement |
| --- | --- | --- | --- |
| Thigh (hip to knee centre) | 0.245 H = 0.412 m | 0.400 m | ok (-3 %) |
| Shank (knee to ankle) | 0.246 H = 0.413 m | 0.4185 m | ok |
| Foot length / ankle height | 0.24 m / 0.066 m | 0.145 + 0.062 m bones, heel behind the ankle | ok |
| Hip joint height standing | 0.530 H = 0.890 m | 0.885 m | ok |
| Hip joint to shoulder joint | 0.288 H = 0.484 m | 0.497 m | ok |
| Upper arm | 0.186 H = 0.312 m | 0.3152 m | ok |
| Forearm | 0.146 H = 0.245 m | 0.2483 m | ok |
| Hand | 0.108 H = 0.181 m | 0.1816 m | ok |
| Neck + head above C7 | 0.182 H = 0.306 m (head height 0.218 m) | 0.105 + 0.133 m bones, vertex 2 mm beyond the head tip | ok |
| Shoulder joint centres apart | 0.31 to 0.32 m | 0.316 m | ok |
| Biacromial breadth | 0.355 to 0.365 m | no acromion, clavicle drawn on the torso bone | **defect**: no shoulder girdle; now 0.356 m over the acromia |
| Clavicle length | 0.14 to 0.155 m | — | now 0.155 m, 28° back, 2° down towards the shoulder |
| Hip joint centres apart | 0.17 to 0.18 m | 0.174 m | ok |
| Bi-iliac breadth | 0.27 to 0.29 m | ilium = two flat discs at ±0.105 m | **defect**; now 0.27 m over the crests |
| Upper arm / forearm / wrist half girth | r 0.045 / 0.038 / 0.025 m | 0.050 / 0.040 / 0.030 x 0.024 m (in sleeve) | ok |
| Thigh / calf / ankle | r 0.089 / 0.056 / 0.034 m | 0.090 / 0.056 / 0.031 m | ok |
| Left / right limbs | equal | equal (bones, solids, both hands with palm, thenar, four two-part fingers, thumb) | ok |

The sagittal chain (segment lengths, joint order pelvis – torso – neck – head, thigh – shank – foot – toe, arm – forearm – hand) was right and stays as it is. Wrong was everything lateral: no clavicle or scapula bone, no pelvis halves, and a skeleton that did not follow anatomy.

### Defects found (by rendering, 1:50 unless said)

Structure: skull a single ellipsoid for cranium and face, mandible a strut not hinged to it; spine a smooth tube without vertebrae, 21 mm in front of the cervical spine at C7; scapulae and ilia flat ellipsoid discs, pelvis a revolved ring, sacrum a disc; no femoral head, trochanter or condyles, patella floating 4 cm proximal of the knee; fibula a free rod; hand a metacarpal disc with the finger rays 4 to 7 mm away from it; pillow core 5 mm above the mattress band; at 1:100 to 1:500 the pelvis one 0.26 m ellipsoid; hem supports 7.5 mm thin at every scale and not touching the duvet at 1:200 and 1:500.

Thermal: region `face-skin` 0.125 m² modelled against `areaM2` 0.0871 (the region also holds ears and neck).

### Rebuilt

**Rig** (catalog `asset.rig`, 583 → 587 bones), following `rig-lateral-rest.md` (read at the start; `rig-rest.ts --solve` for every placement):

| Bone | Parent | From → to (world rest, m) | Length |
| --- | --- | --- | --- |
| `clavicle-l/-r` | `torso` | sternoclavicular joint [0.5175, 0.619, ±0.024] → acromioclavicular joint [0.5215, 0.545, ±0.160] | 0.1549 m, revolute -8..35° (shrug), twist ±30° |
| `ilium-l/-r` | `pelvis` | sacroiliac joint [0.980, 0.498, ±0.045] → hip joint [1.050, 0.553, ±0.087] | 0.0984 m, revolute ±4° |
| `arm-l/-r` | now `clavicle-*` | re-parented without moving (rest frame equal to 4 decimals) | |
| `thigh-l/-r` | now `ilium-*` | re-parented without moving | |

Right side by the mirror rule (z, abduction, twist negated). Poses (`supine`, `side`, `curled`), clips (`breathing`, `turn-over`), 2D shapes, anchors, masses and thermal regions needed no re-binding because no limb frame moved; the new bones carry no mass, no 2D shape and no offset in any pose or clip.

**Structure**, every scale authored for its feature size:

- 1:50 (167 → 204 solids): cranium, skull base, facial skeleton, zygomatic arches, mandible with ramus, angle and chin hinged under the arches; 7 cervical, 12 thoracic and 5 lumbar bodies with discs on a lordosis / kyphosis line that meets the cervical column; on each clavicle bone the S-curved clavicle, scapular blade (five flat sections in the scapular plane, 25° to the frontal plane), scapular spine to the acromion, glenoid, coracoid; on each hip bone the iliac wing along the crest from ASIS to PSIS, acetabulum, ischium to the tuberosity, pubis with both rami to the symphysis; wedge sacrum with coccyx; femoral head, greater trochanter, condyles, patella on the condyles, tibial plateau, medial malleolus, fibula from lateral condyle to lateral malleolus; carpus and four metacarpals per hand ending in the knuckles where the finger rays start; sternoclavicular and sacroiliac joints as joint solids. Ribs, sternum, long-bone shafts, foot and toe rays kept.
- 1:100: cranium, base, face block, mandible; spine one bar on the same curve; straight clavicle, scapula plate with spine; iliac wing (4 sections), acetabulum, ischium, pubic arch; wedge sacrum.
- 1:200: cranium and jaw mass; spine bar; clavicle bars; hip bone as wing plus ischium; sacrum mass; hem supports as blocks that reach the duvet webs.
- 1:500: cranium and jaw mass; spine bar; slender clavicles; one 75 mm bar per hip bone; sacrum mass; 75 mm support blocks.

**Thermal** (catalog root): `face-skin` skin fraction 0.05 → 0.065 and `areaM2` 0.0871 → 0.1132 (face with ears and neck; head and neck 8.9 % of the DuBois area with the hair), taken from `top` (0.511 → 0.496).

**Decisions**: one sentence pair (en, de) per scale on skeleton and girdles appended.

### Rendered and read

Before: 1:50 body, structure sheets; body top, body left, structure top, structure left (no rig). After: 1:50 structure top, left, three-quarter (supine); 1:50 structure `curled` six views with rig; 1:50 body `side` six views; 1:50 structure `turn-over` at 4 s; 1:50 thermal three-quarter; 1:200 and 1:500 structure three-quarter; 1:100 and 1:500 body six views; 2D 1:50 supine. In `curled`, `side` and the turn the girdles stay on the trunk and the limbs leave from the acromial end and the hip joint.

Rendered but not read: 1:100 structure, 1:50 structure `side` top. Not rendered after the change: `breathing` (two instants), a second instant of `turn-over`, thermal and body at 1:200, structure sheets of 1:100 to 1:500 in the other poses, 2D at 1:100 to 1:500 and in the other poses.

### Validation

- Candidate (`r3-validate.ts --file … --volumes`): `VALID file/cand (file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Published with `volumes-insert.py` (all four scales spliced). `r3-validate.ts humans/sleeping-person --catalog-only --volumes` after the first publish (rig, thermal, structure): `VALID … schema 0, validateAsset 0, checkVolumes 0`. After the second publish (decisions texts added) the same run, about 11 minutes on the loaded host: `VALID humans/sleeping-person (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### What remains imperfect (for the next pass)

- **Rig**: `torso` is still one rigid bone (no lumbar / thoracic split), hands have no finger bones (fingers are solids on the hand bone, alike on both sides), so `curled` bends the trunk only at the lumbosacral joint and the neck.
- **Clavicle limits / poses**: no pose or clip uses the new girdle bones yet (side lying should protract the upper shoulder by some degrees); scapula rides on the clavicle bone, no scapulothoracic motion of its own.
- **1:50 structure**: no spinous or transverse processes, ribs meet the sternum through straight costal bars (rib cage reads boxy from the three-quarter view), no orbits or nasal opening in the facial block, radius / ulna and foot bones as found, `member-bed-surface` is a 1.68 m band through the body that hides the spine from above. Whether every new bone lies inside the skin in every pose was read from the ghost in three renders, not measured.
- **1:500 structure**: support blocks of 75 mm are larger than the hem they stand for; hip bar and femur neck read as a cone.
- **Body, all scales**: not rebuilt. The duvet is 112 flat-faced cells on 560 cloth bones at every scale, with black slivers between cells in several views and the same cell count at 1:500 as at 1:50 (the decisions text of 1:100 to 1:500 describes mounds and slabs that are not what is modelled); side drapes are flat vertical sheets. Head at 1:50: nose a long ridge, no brow, eyelids as two ovals; at 1:500 the hair is a thin slab under the head. Legs and left arm under the duvet exist only at 1:50.
- **Thermal**: the plume emitter over the chest renders as a 0.3 m disc lying on the duvet; lists at 1:100 to 1:500 were not looked at.
- **2D**: unchanged; elevation and plan draw every quilt cell with its mesh diagonal, which is not a seam of the duvet.
- Head breadth: the head loft states 0.188 m along the section's first radius and 0.148 m along the second; the render shows a head about 0.16 m wide, so the proportions look right, but I did not settle which radius is lateral.

### Fundamental issues (outside this asset)

- `r3-validate.ts --catalog-only --volumes` on this asset (587 bones, 9.8 MB) takes from 1 to more than 10 minutes depending on host load; a run cannot be told from a hang.
- `volumes-connected` reports a hem support that is 30 mm below the first duvet web as a gap only at the scales whose feature is larger than the gap, so the same geometry passes at 1:50 and 1:100 and fails at 1:200 and 1:500; the support had to be drawn as a block there.
- `r3-sheet.ts` has no zoom on a region (`--only` enlarges the whole asset), so a face of 0.2 m on a 2 m asset is about 240 px wide.

## Pass 1 (second round), continuation — 2026-11-22 afternoon

Status: **partial**. This run took over where the run above stopped (girdles on the lateral rest placement and the skeleton were already published and are kept; `rig-lateral-rest.md` read at the start, nothing in it asks for more on this asset). Done now: the body under the duvet at all four scales, closed hips and knees in every pose, the duvet laid again over the changed body including the feet, one clip defect, the decisions texts. Not done: the 2D drawings, a duvet per scale, torso split and finger bones, thermal of the bare legs (open list at the end).

Edit scripts (hand-chosen numbers, idempotent), in the order they run:

1. `T/rework-sleeping-person-3-body.py candidate|publish` — body solids.
2. `T/rework-sleeping-person-4-duvet.ts <in> <out>` — copy of `digital-sleeping-duvet.ts` (the cloth of an earlier ticket step) with feet and toes among the parts the cloth lies on; `T/rework-sleeping-person-4-duvet-merge.py` takes only cloth bones, their pose and clip offsets, cells, webs and cell drawings from its output (thermal areas, hem supports, anchors, skeleton stay).
3. `T/rework-sleeping-person-6-2d-decisions.py` — decisions texts.
4. `T/rework-sleeping-person-5-publish.py <candidate> [root keys]`, then `volumes-insert.py`.

### State found

Catalog and source `T/volumes/humans/sleeping-person.json` were identical (all four scales). `r3-validate --catalog-only --volumes` of the earlier run: VALID.

### Judged by rendering (before any change)

Read: 1:50 body six views `supine`, three-quarter alone, `curled` six views; 1:500 body six views; the body without the duvet (a candidate with the quilt cells removed) at 1:50 in `supine` and `curled`.

Reference numbers are the table of the run above (woman 1.68–1.70 m); added for this run:

| Measure | Reference | Model found | Judgement |
| --- | --- | --- | --- |
| Hip breadth over the trochanters | 0.35–0.37 m | 0.36 m (pelvis block half width 0.18) | ok |
| Perineum / gluteal fold below the hip joint centre | 0.07–0.10 m | pelvis block ran 0.15 m below the hip joints and ended in a flat cap 0.31 x 0.14 m | **defect**: a flat disc stood out behind the thighs as soon as the hips flexed (`side` 35°, `curled` 70°) |
| Thigh root | gluteal mass around the hip joint, follows the femur | flat cap of 0.18 m at the joint centre | **defect** |
| Knee | condyles 0.09–0.10 m across, rounded, closed at 105° | thigh and shank ended in flat caps 20 mm past the joint | **defect** (open wedge at the knee in `curled`) |
| Body under the duvet at 1:100, 1:200, 1:500 | whole figure | only what shows: upper chest, right arm, ankles and feet; no pelvis, thighs, shanks, lower trunk; left hand missing | **defect**: the duvet was a hollow shell, the feet hung in the air behind it |
| Duvet over the feet | cloth lies on the feet | cloth was laid over trunk, pelvis, thighs and shanks only | **defect**: in `curled` the toes of the upper foot came through the duvet |
| `breathing` at 2.4 s | hand lifts by half a degree | key `forearm-r.angleDeg -0.5` replaces the 71.22° of the pose | **defect**: the elbow straightens by 72° and the arm sinks into the duvet at the top of every breath |

### Rebuilt

**Body 1:50.** `shorts` (pelvis block) re-lofted in seven sections from the rounded perineum 0.108 m below the hip line to the waist (half widths 0.045 → 0.18 at the trochanters → 0.14, the back flat on the mattress at 0.093 m). `thigh-l/-r` and `shorts-leg-l/-r` start with three sections of a rounded gluteal head around the hip joint and the thigh ends in two sections of a rounded knee; `shank-l/-r` start with a rounded head. Hips and knees are closed in `side`, `curled` and during the turn.

**Body 1:100 / 1:200 / 1:500.** Trunk lofts extended from the shoulders down to the pelvis (three / three / two new sections); new `shorts`, `thigh-l/-r`, `shank-l/-r` per scale with 5–6 / 4–5 / 3 sections of the same proportions; `hand-l` (and `thumb-l` at 1:100) as the mirror of the right hand. At 1:500 the left hand is 76 mm thick (the feature).

**Duvet.** Laid again with the cloth step of this ticket over the new body with feet and toes: 560 cloth bones, their offsets in the three poses and the 72 keys of `turn-over`, the 112 cells at each scale, the quilt webs of the structure lists and the cell drawings were replaced; nothing else of that step's output was taken. The toes no longer come through in `curled`; the cloth forms a mound over the feet.

**Decisions.** First text (cloth) and the body texts of 1:100, 1:200, 1:500 rewritten to what is modelled (they described mounds, slabs and ellipsoids that do not exist); 1:50 body text: duvet part corrected, hips and knees added. English and German.

### Rendered and read after the changes

1:50 body six views: `supine`, `side`, `curled`, `turn-over` at 3 s and 5.5 s; `breathing` at 2.4 s three-quarter; body without duvet 1:50 `curled` and 1:200 `side`; 1:500 body `curled`; thermal 1:50 cut (six views); 2D 1:50 `curled` and 1:200 `supine`.

Rendered, not read: 1:100 and 1:200 body `curled`, thermal 1:50 three-quarter, `breathing` at 1.2 s. Not rendered in this run: structure at any scale (only the quilt webs changed there), thermal at 1:100 to 1:500, 2D at 1:100 and 1:500.

### What remains imperfect (for the next pass)

- **2D, all scales, all views**: the duvet is drawn as 112 triangles with every mesh diagonal at 0.22 mm; at 1:200 and 1:500 the line drawing is a black blot. A thinner pen is refused by the print profile (tried 0.13 mm: `PRINT unsuitable`, min stroke 0.2 mm). Needed: quilt seams as a rectangular grid and one outline per view, per pose. In `curled` and `side` the head is not drawn in elevation and plan (only hair strokes and a neck stub show).
- **Duvet per scale**: the same 112 cells and 560 cloth bones at 1:500 as at 1:50 (each cell needs its bones to follow the poses). Black slivers between cells remain along the steep flank beside the trunk (`supine`, three-quarter).
- **Duvet, `curled`, from the left**: a triangular opening between the raised cells over the knees and the fixed side hem shows the knee; the knees reach to 0.5 m beside the axis, the duvet edge is at 0.515 m.
- **Rig**: `torso` is one rigid bone (no lumbar / thoracic split), hands have no finger bones (fingers are solids on the hand bone, alike on both sides). No pose uses the clavicle or hip bones.
- **Thermal**: thighs and shanks under the duvet have no thermal solid at any scale (they show as ghost in the cut sheet; the rule `thermal-coverage` passes because the duvet layer covers them). The plume emitter still lies as a disc on the chest.
- **Body 1:50**: head as found by the run above (nose a long ridge, no brow); no patella, no clavicle or sternal notch relief, the right arm in `curled` lies straight along the trunk with the hand hanging in front of the hip.
- **1:500 body**: the trunk is a short stack of flat-sided sections at the shoulders.
- **Structure**: as left by the run above (list there); not looked at again.

### Fundamental issues (outside this asset)

- A clip key that names one field of a bone (`forearm-r.angleDeg -0.5`) replaces the pose value of that field instead of adding to it; nothing warns when a key moves a bone by 70° within a "breathing" clip. A validator rule on the largest joint travel per second of a clip would have caught it.
- The print profile's minimum stroke (0.2 mm) applies to every role, so a drawing cannot subordinate seams to outlines by pen weight at any scale; `texture` has no pen at 1:200.
- `r3-sheet.ts` has no switch to leave out solids by material or bone; judging a covered body needs a candidate file without the cover.

### Clip fix and validation (same run)

- `breathing`, key at 2.4 s: `forearm-r.angleDeg` -0.5 → 70.72 (`T/rework-sleeping-person-7-breathing.py`: the supine 71.22° less the half degree the hand lifts). Rendered and read at 2.4 s: the elbow stays bent, the hand stays on the duvet.
- Structure 1:50 `curled` rendered and read after the last change: the skeleton lies inside the body and the re-laid duvet, girdles on the trunk, limbs from the acromial end and the hip joint. The contact band of the mattress stays where the supine body lay (it is a fixed band on `bed-surface`), so in `curled` the legs lie beside it.
- Candidate with body and re-laid duvet: `VALID file/cand2 (file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- After the publish of body, duvet and decisions: `VALID humans/sleeping-person (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`r3-validate.ts humans/sleeping-person --catalog-only --volumes`).
- After the clip fix, same command: `VALID humans/sleeping-person (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Catalog and source agree (published through `volumes-insert.py`, all four scales spliced). Tool output of this run was deleted.
