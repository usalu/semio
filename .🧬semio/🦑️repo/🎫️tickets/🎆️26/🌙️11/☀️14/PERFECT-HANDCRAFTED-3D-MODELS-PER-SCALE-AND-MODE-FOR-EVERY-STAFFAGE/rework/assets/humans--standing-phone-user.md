# humans/standing-phone-user — rework report

## Pass 1 (second round), 2026-11-22

State found: no earlier rework report for this asset (batch report `rework/01.md` names it in its title only; that author did not reach it). The source `volumes/humans/standing-phone-user.json` and the catalog `asset.volumes` were identical (all four scales, all three lists). The asset is the round-3 build (`r3-humans-standing-phone-user.md`, scripts `r3-standing-phone-user-*`, `r3_standing_phone_user_*`), 21 bones, sagittal rig. `r3-validate.ts` at the start: INVALID, 60 `volumes-feature` findings (26 at 1:200, 34 at 1:500, structure list: limb bones copied with their 1:50 section into the coarse lists without `slender`, epiphyses and joint markers under the feature).

### Reference (slim woman of 24, stature 1.69 m barefoot, 59 kg; 1.68 m as drawn with the bowed head; own knowledge: DIN 33402-2 women 18–25 P50, Drillis-Contini ratios, de Leva 1996)

| Quantity | Real | Rig / model as found | Verdict |
|---|---|---|---|
| Hip-joint height | 0.530 H = 0.896 + 0.02 sole | pelvis origin 0.903 | right |
| Thigh (hip centre – knee) | 0.245 H = 0.414 | 0.41325 both sides | right |
| Shank (knee – ankle) | 0.246 H = 0.416 | 0.40375 both sides | 12 mm short, inside the spread; kept |
| Ankle height / foot ankle–ball / toes | 0.039 H = 0.066 + sole / 0.17 / 0.055 | 0.17251 / 0.0572 | right |
| Upper arm / forearm / hand | 0.186 H = 0.314 / 0.146 H = 0.247 / 0.108 H = 0.183 | 0.3135 / 0.247 / 0.1805, both sides alike | right |
| Trunk hip centre – C7 | 0.288 H + … = 0.56 | 0.0951 + 0.467 = 0.562 | right |
| Neck C7 – atlanto-occipital / head AO – vertex | 0.10 / 0.15 (head height vertex–menton 0.225 = H/7.5) | 0.1016 / 0.149 | right |
| Shoulder-joint centres apart / biacromial | 0.30–0.31 / 0.355 (women) | ±0.153 / acromion tips ±0.176 = 0.352 | right |
| Hip-joint centres apart / bi-iliac / bitrochanteric | 0.17 / 0.28 / 0.33 | ±0.085 / crest ±0.108 + plate / — | right |
| Clavicle (sternoclavicular – acromioclavicular, straight) | 0.14–0.15 | solid ran 0.161 from z 0.022 | 10 mm long; the solid's ends were kept as the bone's ends |
| Hip bone (sacroiliac joint – hip centre, straight) | 0.09–0.10 | no bone | added, 0.096 |
| Joint order arm | clavicle/scapula – humerus – radius/ulna – carpus – metacarpus – phalanges | arm hung from `torso` 0.153 m beside the plane, no girdle bone | wrong topology: fixed |
| Joint order leg | hip bone – femur – tibia/fibula – tarsus – metatarsus – toes | thigh hung from `pelvis` 0.085 m beside the plane | wrong topology: fixed |
| Spine | lumbar (L5/S1–T12, 0.17) and thoracic (0.30) segments | one `torso` bone 0.467 | missing lumbar joint: open |
| Fingers | 3 phalanges per finger, 2 for the thumb, each a joint | solids only, all on `hand-l` / `hand-r` (the grip is modelled, not rigged) | open |
| Upper-arm sleeve (slim arm girth 0.26 = Ø 0.083, plus fleece and ease) | 0.105–0.11 at the deltoid, 0.095–0.10 mid, 0.088–0.09 at the elbow | 0.117 × 0.129 deltoid, 0.106 × 0.117 mid, 0.097 × 0.103 elbow: a fat tube | 12 % too thick: taken in |
| Thigh breadth top / knee / calf / hem (front sheet) | 0.16 / 0.105 / 0.107 / 0.085 | 0.160 / 0.106 / 0.107 / 0.088 | right |
| Head breadth with hair | 0.150 + hair = 0.158 | 0.159 | right |
| Breadth over the sleeves, texting pose (arms 7° out) | 0.46 | `dimensions.depthM` 0.48, drawn 0.47 | 0.465 after the sleeves |

### Defects found (sheets read at full size: 1:50 body and structure in the default pose, front and left close-ups, `phone-call`, `photo-pose`)

1. Rig topology: no shoulder girdle and no pelvic girdle; arms and thighs were children of `torso` / `pelvis` floating 0.153 m / 0.085 m beside the plane with nothing between.
2. Structure 1:50: hip bones 5.0 mm clear of the sacrum (no sacroiliac contact), mandible 2.0 mm clear of the maxilla, phone frame 3.6 mm clear of every bone. 1:100: the same sacroiliac gap, metacarpus 4.9 mm from the radius (no carpus). 1:200: shoulder girdle 9.2 mm from sternum and ribs, ulna/radius/hand apart. 1:500: one bar across both shoulders and one across the pelvis instead of girdles; limb bones were the 1:50 parts list.
3. Body 1:100 and 1:200: shoe 9.9 / 10.9 mm below the jeans hem with nothing between (the sock existed only at 1:50).
4. Body, all scales: upper sleeves 12 % too thick, a constant tube (table).
5. Not defects after measuring: segment lengths, joint order of the sagittal chain, left/right symmetry (identical numbers), leg and trunk form, head, hands with all fingers on both sides, shoes.

### Rebuilt

Scripts (ticket folder, hand-chosen numbers at the top of each): `rework-standing-phone-user-girdle.py`, `-sleeve.py`, `-texts.py`.

- **Rig** (24 bones, was 20): `clavicle-l/-r` (child of `torso`, sternoclavicular joint [0.050, 1.404, ±0.022] to acromioclavicular joint, 0.161 m, running 23° back and 9° up; limits −8…35°, twist −40…5° / mirrored) and `ilium-l/-r` (child of `pelvis`, sacroiliac joint [−0.045, 0.978, ±0.045] to the hip-joint centre [0, 0.903, ±0.085], 0.096 m; ±5°) on the lateral rest placement (`abductionDeg`, `twistDeg`, solved with the helpers of `rig-lateral-standing-adult.py`). `arm-*` re-parented to the clavicles, `thigh-*` to the hip bones without moving: every pre-existing bone keeps its world rest frame to 2e-5 (checked in the script), `rig-lateral-compare.ts` candidate against catalog: largest move 0.0000 mm. Poses, clips, anchors, 2D shapes, thermal regions and masses of the limbs therefore needed no re-binding. Right side = left with `z`, `abductionDeg`, `twistDeg` negated.
- **Structure**: 30 solids re-expressed in the new bone frames at all four scales (clavicle, scapula, scapular spine, acromion, glenoid, shoulder-girdle bar on `clavicle-*`; hip bone, pubic and ischial ramus on `ilium-*`). 1:50 and 1:100: `posterior-ilium-l/-r` from the auricular surface to the blade (sacroiliac contact); 1:50: mandible raised 2.5 mm into occlusion, `grip-pad-r` (joint solid, finger pad under the phone); 1:100: carpus both sides. 1:200: long shafts `slender` at their true section, sub-feature epiphyses left out, humerus continued to the elbow and radius to the carpus, `elbow-*` and `carpus-*` balls, girdle bars reach the manubrium, joint markers at 31 mm. 1:500: `shoulder-girdle` and `pelvic-girdle` bars replaced by slender clavicles, a scapula mass per side, sacrum and two hip-bone masses; femur and tibia shafts kept (`slender`), `proximal-femur-*` and `knee-*` masses, humerus and radius joint to joint, sternum extended to the manubrium.
- **Body**: sleeves tapered at 1:50, 1:100, 1:200, 1:500 (now 0.108 × 0.120 at the deltoid, 0.092 × 0.102 mid, 0.087 × 0.093 at the elbow); `sock-l/-r` added at 1:100 and 1:200.
- **Thermal**: `th-sock-*` (region sneakers, layer socks) at 1:100 and 1:200; tee sleeve at 1:50 kept 4 mm inside the new sleeve. Outer sleeve regions follow the body solid.
- **2D**: revolved sleeve profiles (`sleeve-l/-r`, at 1:500 `sil-arm-l/-r`) redrawn with the same taper in elevation, front and plan at every scale; `dimensions.depthM` 0.48 → 0.465.
- **Decisions**: girdle, load-path and sleeve paragraphs at every scale; structure texts of 1:200 and 1:500 rewritten (en, de).

### Validation

- `r3-validate.ts humans/standing-phone-user --catalog-only --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0` (run after the sleeve stage). After that only the `decisions` texts changed; `--catalog-only` without `--volumes` afterwards: `VALID`. Start of the pass: INVALID with 60 analytic findings, and 9 `volumes-connected` findings on the first candidate.
- Source and catalog `volumes` agree (checked after the last publish).
- Sheets read after the changes: 1:50 structure (default), 1:500 structure, 1:50 body default (six views), 1:100 body, 1:500 body, 1:50 thermal with `--cut`, clip `answer-call` t = 2 s, 2D 1:50. Rendered but not read: 1:200 body and structure, 1:100 structure, 1:500 thermal, `scroll-and-sway`, `photo-pose` structure. `phone-call` and `photo-pose` body were read before the sleeve change only.

### What remains imperfect (for the next pass)

1. Rig: one `torso` bone from L5/S1 to C7; no lumbar joint (standard of `standing-adult`: `lumbar` 0.170 + thorax 0.297). Needs the torso-bound solids, shapes and poses split.
2. Rig: no finger bones (15 per hand in the standard). The phalanx solids of body and structure sit on `hand-l/-r` in the fixed grip; the grip cannot open in `answer-call`.
3. Clavicle bone 0.161 m straight (anatomy 0.14–0.15 for this woman): its sternal end follows the old solid at z ±0.022; move the sternoclavicular joint out to ±0.028 and re-solve.
4. 1:500 body: a visible slit between jeans hem and shoe in the front and back views (inside the 15 mm contact tolerance, so no finding); shoulders of the 1:500 trunk mass are square.
5. 1:500 structure: the scapula mass (75 mm capsule) reads heavy beside the true-section shafts; no elbow mass (humerus, ulna and radius meet end to end).
6. 1:50 structure `grip-pad-r` is a sleeve of 8.5 mm around the last phalanx of the little finger; a pad under the fingertip only would be truer.
7. Poses `phone-call`, `photo-pose` and clip `scroll-and-sway` not read after the sleeve change; 1:200 sheets and thermal at 1:100/1:200/1:500 not read in this pass. 2D drawings at 1:100, 1:200, 1:500 not rendered after the profile change (validated only).
8. Hoodie trunk, jeans, head, hands, shoes at 1:50 were judged from the sheets and left as found; no close-ups of face, hands and feet were made.

### Fundamental issues

- `r3-validate.ts … --volumes` needs 2 to 4 minutes for this asset with 19 authors on the host; the bash tool's 120 s limit is exceeded, so it has to run in the background.
- `volumes-connected` on a hand-held object: the structure solid of the phone (`of: phone`) cannot touch bone in reality (soft tissue between). The rule accepts a path "through one body solid" only for standing on the ground; a held object needs a joint solid invented for the contact (item 6). A rule for `contact` between a held object and the hand's body solid would be truer.
