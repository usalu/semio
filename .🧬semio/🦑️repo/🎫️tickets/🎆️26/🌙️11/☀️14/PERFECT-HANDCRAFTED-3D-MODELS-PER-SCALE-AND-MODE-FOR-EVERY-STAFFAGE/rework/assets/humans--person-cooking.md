# humans/person-cooking — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published and `VALID`; the girdles are on the lateral rest placement; the open list at the end is real.

Scripts (the chisel, hand-chosen numbers): `T/rework-person-cooking-girdle.py` (found unapplied from an agent that was cut off at 13:57, read, kept and used), `T/rework-person-cooking-pass1.py` (this pass; runs the girdle conversion and then the corrections; `--apply` writes the catalog file and `T/volumes/humans/person-cooking.json`, idempotent). `T/rework-person-cooking-check.sh` is the earlier agent's inner loop.

### State found

- Source `T/volumes/humans/person-cooking.json` and catalog `volumes` were identical in all 12 lists (no half-written edit). No earlier report; batch report `rework/05.md` lists the asset as "not started".
- The asset was already a detailed model (1:50: 126 body, 132 thermal, 179 structure solids; finger solids on both hands, face, hood, drawstrings, shoes with laces; skeleton with 12 rib pairs, sternum, cartilages, scapulae, pelvis parts, long bones with epiphyses, hand and foot bones). `r3-validate --volumes` on the girdle candidate: 6 findings, all `volumes-connected`.

### Reference table (woman, 1.71 m, 58 kg; Drillis-Contini fractions of stature H, DIN 33402-2 female p50 scaled)

| Measure | Reference | Model (rig / solids) | Verdict |
| --- | --- | --- | --- |
| Hip joint height | 0.530 H = 0.906 m | 0.905 m | ok |
| Thigh (hip to knee) | 0.245 H = 0.419 m | 0.413 m | ok (-1.4 %) |
| Shank (knee to ankle) | 0.246 H = 0.421 m | 0.404 m | ok (-4 %; ankle at 0.088 m, reference 0.039 H = 0.067 m plus 0.02 m sole) |
| Foot length | 0.152 H = 0.260 m | heel 0.046 behind + 0.125 + 0.07 toe = 0.241 m bone, shoe 0.27 m | ok |
| Shoulder joint height | 0.818 H = 1.399 m | 1.405 m | ok |
| Upper arm | 0.186 H = 0.318 m | 0.3135 m | ok |
| Forearm | 0.146 H = 0.250 m | 0.247 m | ok |
| Hand | 0.108 H = 0.185 m | 0.1805 m | ok |
| Head height (chin to vertex) | H / 7.5 = 0.228 m | about 0.23 m (cranium radii 73 x 87 x 67 mm plus face) | ok |
| Head breadth | 0.145 to 0.150 m | 0.149 m (front sheet) | ok |
| Shoulder joint centres | +/-0.155 to 0.17 m | +/-0.155 m | ok (female) |
| Biacromial breadth | 0.355 to 0.37 m | acromioclavicular joints +/-0.150 m, acromion beyond: about 0.36 m | ok |
| Bideltoid breadth, clothed | 0.42 + hoodie = 0.44 to 0.45 m | 0.45 m | ok |
| Clavicle length | 0.14 to 0.15 m | 0.1423 m, S-curve, 4 deg up, 24 deg back | ok |
| Hip joint centres | +/-0.085 to 0.09 m | +/-0.0875 m | ok |
| Bi-iliac (crest) breadth | 0.27 to 0.29 m | 0.296 m | ok, upper end |
| Crest above hip joint | 0.10 to 0.12 m | 0.124 m | ok, upper end |
| Sacrum breadth | 0.10 to 0.12 m | 0.112 m found, 0.122 m now (see below) | at the upper end |
| Coccyx behind hip axis | 0.07 to 0.08 m | 0.100 m found, 0.080 m now | corrected |
| Femur / tibia / humerus shaft radius | 13 / 11.5 to 13.5 / 10.5 to 11 mm | 1:50 and 1:100 true; 1:200 at 15.3 mm, 1:500 at 37.8 mm | corrected to true shafts with `slender` |
| Rib section | 12 to 15 mm high | 1:100 15.6 mm; 1:200 30.6 mm; 1:500 one 75.6 mm lump pair | corrected at 1:200 and 1:500 |
| Base unit | 0.60 x 0.60 x 0.90 m (DIN EN 1116), plinth 0.10 to 0.15 m | 0.60 x 0.60 x 0.90 m | ok |
| Pot | diameter 0.24 to 0.26 m, height 0.11 to 0.13 m | 0.26 m x 0.116 m | ok |
| Cooking spoon | 0.28 to 0.32 m | 0.29 m | ok |

Segment lengths, joint order and left/right symmetry of the sagittal chain were right as found (pelvis – torso – neck – head; arm – forearm – hand; thigh – shank – foot – toe; both sides identical with mirrored `z`). The dev's "wrong proportions" does not apply to the lengths of this asset; what was wrong is listed next.

### Defects found (by rendering and measuring)

1. Rig: no shoulder girdle and no pelvic girdle; arms hung from `torso`, thighs from `pelvis` with a bare `z` offset.
2. Structure 1:50: hip bones 3.0 mm off the sacrum (open sacroiliac joints), radius 1.5 mm short of the carpus, femoral shafts 3.9 mm short of the condyles, coccyx 0.10 m behind the hip axis (2 cm too far, a spur towards the trouser surface).
3. Structure 1:100: atlas end of the cervical column 3.3 mm short of the skull; sacroiliac gap 6.7 mm.
4. Structure 1:200: pot stood over the hob cut-out of the worktop member, no member carried it (25.8 mm to the carcass side); long bones, spine top, ribs and shoulder girdle thickened to the feature (15.3 mm radius) instead of their true section.
5. Structure 1:500: every member a 75.6 mm sausage (femur, tibia, humerus, forearm, spine), the rib cage two fat lumps on the chest; contrary to rule 2 and 11 of `validator.md`.
6. Body 1:50: drawstrings floated 2.6 mm in front of the sweatshirt; sleeve caps stood 7 to 10 mm above the shoulder slope of the sweatshirt (an epaulette corner in the front view).

### What was rebuilt

- **Rig** (`rework-person-cooking-girdle.py`, following `T/rig-lateral-rest.md` 1.2 to 1.6): `clavicle-l/-r` under `torso` from the sternoclavicular joint [0.045, 1.417, +/-0.020] to the acromioclavicular joint [-0.012, 1.427, +/-0.150] (0.1423 m, revolute -8..35, twist -40..5 mirrored), `ilium-l/-r` under `pelvis` from the sacroiliac joint [-0.035, 0.985, +/-0.045] to the hip joint (0.0971 m, revolute -3..3). Arms and thighs re-parented without moving (asserted under 2e-5 m and 2e-5 in the frame); `rig-lateral-compare.ts` catalog before / after: largest move of any 2D shape or anchor in every pose and clip instant 0.0047 mm. Poses, clips, 2D shapes, anchors, thermal regions and masses therefore keep their meaning unchanged.
- **Girdle solids re-bound** at all four scales, catalog and source: clavicle (now its S-curve through six stations), scapula, scapular spine, scapular neck (new at 1:50), glenoid, shoulder-girdle (1:100, 1:200; new member at 1:500) to `clavicle-*`; hip bone, pubic and ischial ramus, acetabulum to `ilium-*`.
- **Structure 1:50**: sacrum alae 61 / 53 mm half breadth (sacroiliac joints closed), coccyx pulled to 0.080 m behind the axis; radius runs into the carpus at [0.2495, 0.004, +/-0.008]; femoral shaft runs into the condyles (end at 0.392); hob plate added as the member that carries the pot onto the worktop.
- **Structure 1:100**: atlas end at 0.113 into the skull base; sacrum 62 mm, coccyx as above; hob plate member.
- **Structure 1:200**: femur, tibia, humerus, forearm and spine at the 1:100 (true) sections with `slender: true`; three rib hoop pairs at 6.5 mm radius with `slender` (bent rods, rule 11); shoulder girdle 8 mm with `slender`; hob plate member; coccyx.
- **Structure 1:500**: femur, tibia, humerus, forearm, spine at true sections with `slender` (femur and humerus run to their joints again: 0.416, 0.3135); two rib hoop pairs (upper and lower chest) at 6.5 mm instead of the lump pair; coccyx. Skull, cervical column, hip bones, sacrum, hand and foot stay feature-sized compact solids (not long enough for `slender`).
- **Body 1:50**: drawstrings and their tips laid 3.5 mm back onto the sweatshirt (thermal twins alike); sleeve cap lowered 7 mm at its three top stations on both arms.
- **Thermal**: only what follows the body (`of` references, drawstring twins). Lists read at 1:50 (default pose) as the same object with nested layers.
- **2D**: not redrawn; the body changes are under 7 mm (under one feature of 1:50) and `volumes-silhouette` passes at every scale. 1:50 drawing read after the change: agrees with the model in all three views.
- **Decisions texts**: unchanged (they still describe what is modelled; the girdle is not mentioned in them).

### Validation

- `r3-validate.ts humans/person-cooking --catalog-only --volumes` after the last write: `VALID humans/person-cooking (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Source and catalog `volumes` compared equal after the apply.

### Rendered and read (images deleted after publishing)

- 1:50 default pose: body, structure, thermal (six views); structure front, back, left close-ups; body front close-up before the sleeve change.
- 1:50 poses on the girdle rig: `tasting` and `standing-by` (six views each); after the last change `tasting` three-quarter (body and structure) and `stir-and-taste` at 2.9 s (six views).
- 1:100 structure, 1:200 body, 1:500 structure before and after the rebuild, 2D of 1:50.
- Rendered but **not read**: `stir-and-taste` 1.3 s, `stirring` 0.6 s and 1.5 s, 1:100 body and thermal, 1:200 structure after the change and thermal, 1:500 body and thermal, 2D of 1:500, the front close-up after the sleeve change. For these the claim rests on the 0.0047 mm rig comparison and the validator only.

### What remains imperfect (for the next pass)

- Rig: one `torso` bone from L5 to C7 (no lumbar / thoracic segments); one rigid `hand` bone per side, fingers are solids on it (right hand modelled closed round the spoon, left relaxed), no finger joints; no pose uses the new clavicles (no shoulder lift in `tasting`).
- Structure 1:50: the hip bone is one flat trapezoid blade per side (reads as a funnel from the side, not as a curved iliac fossa with crest, ASIS and ischial tuberosity); the spine is one curved column with eight spinous processes, no individual vertebrae or discs; skull is a cranial ellipsoid with a face block, mandible and nasal bone, no orbits or zygomatic arches.
- Structure 1:200 / 1:500: hip bones 30.6 / 75.6 mm thick plates; cervical column and shoulder girdle at 1:500 feature-thick; 1:200 cervical column 15.3 mm instead of 12 mm.
- Body 1:50: the sleeve cap / shoulder transition was only lowered, not re-read after the change; in `tasting` the spoon handle crosses the left drawstring.
- Body, thermal and 2D of 1:100, 1:200, 1:500 were not re-modelled in this pass and only 1:200 body was read; the unread images above.
- Decisions texts do not mention the skeleton simplification per scale.

### Fundamental issues (outside this asset)

- None met in kernel, validator or tools. Host has no image montage tool (no PIL, ImageMagick, ffmpeg), so every sheet costs one read; a `r3-sheet --contact` mode that tiles several lists / poses into one PNG would let an author read all 30 plus images of the loop within a pass.
