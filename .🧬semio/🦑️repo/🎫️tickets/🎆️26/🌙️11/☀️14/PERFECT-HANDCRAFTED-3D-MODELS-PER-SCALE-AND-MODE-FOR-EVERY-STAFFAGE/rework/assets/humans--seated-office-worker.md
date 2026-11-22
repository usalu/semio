# humans/seated-office-worker

Running report of the asset. Earlier work: author 03 of the first round (`rework/03.md`, scripts `rework-03-seated-office-worker-1…8`), cut off before publishing.

## Pass 1 (second round), 2026-11-22

### State found

- Source `volumes/humans/seated-office-worker.json` and catalog differed: the source held the whole rebuild of author 03 (163 / 72 / 62 / 40 body solids; the catalog missed 31 body, 17 thermal and 58 structure solids at 1:50 and had 43 / 12 / 73 older ones). Rendered both: the source was better in every view (lumbar segment, face, chair mechanism, 1200 x 800 desk, anatomical skeleton), so the source was kept and is now published.
- Catalog alone: `INVALID` (bone `lumbar` drawn at 1:100/200/500 without a solid). Source merged: `INVALID` with three findings (1:500 drawing 1.337 / 1.347 m long against 1.308 m; `mass-desk-top` 24.0 mm at 1:200). During the pass eight `lateral-ambiguous` findings appeared on the star base (the kernel now counts the rest spread of a bone; see Fundamental issues).

### Reference (man 1.78 m, 76 kg, about P50 DIN 33402-2; segment ratios Drillis-Contini, masses de Leva)

| Quantity | Reality | Rig / model | Verdict |
|---|---|---|---|
| Thigh, hip joint to knee axis | 0.245 H = 0.436 m | 0.435 | right |
| Shank, knee to ankle | 0.246 H = 0.438 m | 0.438 | right |
| Ankle above floor in a shoe | 0.039 H + 25 mm = 0.094 m | 0.092 | right |
| Foot in a shoe (ankle to ball 0.17, toes 0.065) | 0.29 m overall | 0.1675 + 0.065 | right |
| Hip joint to L5/S1 / lumbar / thoracic to C7 | 0.10 / 0.18 / 0.32 m (hip to cervicale 0.60 m) | 0.10 / 0.18 / 0.325 | right |
| C7 to atlas / atlas to vertex | 0.115 / 0.135 m | 0.118 / 0.132 | right |
| Shoulder joint above hip joint | 0.818 H - 0.530 H = 0.51 to 0.53 m | 0.532 | right |
| Shoulder joints apart (biacromial 0.39 to 0.40) | 0.35 to 0.36 m | 0.352 (z +-0.176) | right, but the arms hung from the thorax bone, no clavicle bone (found); clavicle bones now |
| Upper arm / forearm | 0.186 H = 0.331 / 0.146 H = 0.26 m | 0.33 / 0.26 | right |
| Hand, wrist to tip of the middle finger | 0.108 H = 0.19 m; metacarpophalangeal joint of the middle finger 0.095 m from the wrist | one 0.19 m bone, fingers baked into it | wrong skeleton: no finger joints |
| Middle finger phalanges (proximal / middle / distal with pulp) | 44 to 45 / 26 to 27 / 20 to 21 mm | none | missing |
| Index / ring / little finger | 82 / 87 / 68 mm from the joint | none | missing |
| Thumb: metacarpal / proximal / distal | 46 / 32 / 27 mm | none | missing |
| Finger breadth at the proximal joint | index 19, middle 20, ring 18.5, little 16.5 mm; thumb 25 mm | lofts of the same breadths on the hand bone | form right, binding wrong |
| Hip joints apart | 0.17 to 0.18 m | 0.176 | right, but the thighs hung from the pelvis bone directly (found); hip bones now |
| Sitting height / seat height | 0.92 m / 0.46 to 0.48 m | 0.91 (bowed) / 0.48 | right |
| Chair depth behind the backrest pivot (pad 45 mm + shell + J-bar) | 55 to 60 mm | 58 mm at 1:50; 95 mm at 1:500 | 1:500 too deep |

### Defects found (all sheets read: 1:50 body / thermal / structure, three poses, 1:100 body and structure, 1:500 body, 2D 1:50 and 1:500)

1. Rig: hands without finger bones (both alike, but one rigid 0.19 m bone each); the typing clip could only rock the whole hand.
2. Pose `leaning-back`: the back stood 10 cm in front of the reclined backrest; the pose leaned on nothing.
3. Pose `reading-screen`: left wrist extended 38 degrees, fingers inside the front rail of the desk.
4. 1:50 body: thigh sections with superellipse exponents up to 2.7 (box corners, black crease patches in the front view); shirt yoke exponents 2.6 / 2.7 (a board with an edge down each shoulder blade from behind).
5. 1:500: backrest pad and J-bar each 75 mm thick one behind the other, 37 mm beyond the real back of the chair in solid and drawing.
6. 1:200 structure: `mass-desk-top` 24.0 mm thin.
7. 2D: the four star arms and castors that leave the side plane were flat elevation shapes without a lateral span.
8. Decisions text of 1:50 still named a 780 x 600 mm desk.

### Rebuilt

- **Rig, girdles** (`rework-seated-office-worker-14-girdles.py`, after `rig-lateral-rest.md` appeared at the end of the pass): 71 -> 75 bones. `clavicle-l|r` from the sternoclavicular joint (22 mm beside the midline on the manubrium) to the acromioclavicular joint (165 mm out, 56 mm further back, 31 mm higher along the thorax), 0.157 m, parent `torso`, revolute -8..35 degrees; `ilium-l|r` from the sacroiliac joint (35 mm out, 55 mm above and behind the hip axis) to the hip joint centre (88 mm out), 0.094 m, parent `pelvis`, -5..5 degrees. `arm-*` and `thigh-*` re-parented to them without moving. Structure solids re-expressed in the new frames and made their members at every scale: clavicle, scapula, acromion, glenoid (1:50, 1:100), shoulder bar (1:200), a stated bar (1:500); ilium, ilium body, pubis, ischium (1:50, 1:100), ilium and hip bar (1:200), a stated bar (1:500). No 2D shape, anchor, thermal region, pose or clip is bound to the new bones.
- **Rig, hands** (`rework-seated-office-worker-9-fingers.py`): 43 -> 71 bones. Per hand `thumb-metacarpal|proximal|distal` (46 / 32 / 27 mm, saddle joint 18 mm from the wrist, 32 degrees of rest spread towards the midline) and `index|middle|ring|little-proximal|middle|distal` (40/23/19, 45/27/21, 41/26/20, 32/18/18 mm; knuckles 93 / 96 / 92 / 84 mm from the wrist, 19.5 mm apart), revolute with flexion limits, identical left and right (mirrored z and spread). Rest is the relaxed curl of the typing hand (16 / 42 / 26 degrees), so the three poses need no finger keys and the drawn hand stays true.
- **Body 1:50**: the five baked finger lofts per hand replaced by 14 phalanx lofts on their bones (knuckle ball over the joint, waist, next knuckle or flattened pad; breadths as in the table), 163 -> 183 solids. Thigh exponents capped at 2.3, shirt yoke 2.35 / 2.3 (`-13-yoke.py`).
- **Thermal 1:50**: one `hand-skin` surface per phalanx (149 -> 169); the vest shell under the yoke follows the shirt. `areaM2` unchanged, coverage rules green.
- **Structure 1:50**: the four bent "phalanges" rods and the thumb rod per hand replaced by 14 phalanx bones with base, shaft and head on their own members; metacarpals end in the knuckles (248 -> 268).
- **1:100**: palm ends at the knuckles; three finger blocks on the middle-finger bones and a two-part thumb on its bones, in body, thermal and structure (72 / 73 / 175 -> 80 / 81 / 185). 1:200 and 1:500 keep one blunt hand (feature 30 / 75 mm).
- **1:500 backrest** (`-10-backrest500.py`): pad ends at the real back face (58 mm), the J-bar rises inside the same 75 mm; solid and the three drawings restated. **1:200** desk mass marker 25 mm.
- **Poses and clips**: `leaning-back` pelvis +5, lumbar +8, thorax +8 degrees, hip 85 mm back, thighs and right arm compensated: lumbar and upper back lie on the pad, right hand on the armrest. `reading-screen` (`-12-poses.py`): left upper arm plumb, forearm 57, wrist 20 degrees: the hand lies on the thigh clear of the desk. `typing` clip: index, middle and ring fingers and the right thumb tap (3 to 9 degrees at the knuckle) in every key.
- **2D** (`-11-star2d.py`): every elevation shape of the four spread star arms and their castors states its lateral span as an extrusion along w at all four scales; 1:500 backrest shapes redrawn. The drawn hands were checked against the new fingers (same curl) and kept.
- **Decisions**: 1:50 (desk 1200 x 800, fingers on bones) and 1:100 (finger blocks) in en and de.

### Proof

- `r3-validate.ts humans/seated-office-worker --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`; after `volumes-insert.py`: `r3-validate.ts humans/seated-office-worker --catalog-only --volumes`: `VALID (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (run again after the girdle stage, same line).
- Read after the changes: 1:50 body three-quarter from two sides and from behind (before the yoke change), structure three-quarter, `leaning-back` full sheet, `reading-screen` left side, `typing` at 1.2 s, `settle-back` at 4 s, 1:100 body and structure, 1:500 body, 2D 1:50 and 1:500.
- Not read: the image tool stopped answering at the end of the pass. The 1:50 back view after the yoke change, the 1:50 thermal cut, 1:100 thermal and 1:200 body were rendered but not looked at (the measured rules are green for them).

### Still imperfect (for the next pass)

- Girdles (stage 14) were built after the image tool had stopped answering: the structure sheets at 1:50 (default and `leaning-back`), and 1:500 front were rendered but **not looked at**. What is proven by numbers only: arm and thigh rest frames unchanged (2e-5), 2D shapes and anchors moved at most 0.004 mm in every pose and clip instant (`rig-lateral-compare.ts`), the 130 points of the re-expressed solids moved at most 0.006 mm in the world, the measured rules are green. The orientation of the flat sections (scapula, iliac blade) after the quaternion change must be checked on a sheet by the next pass.
- Girdle motion is not used yet: no pose or clip moves `clavicle-*` or `ilium-*` (the raised left arm of `leaning-back` should lift its clavicle by about 20 degrees with the arm offsets re-solved; left out because the result could not be looked at).
- 1:500 structure: the stated shoulder and hip bars (75 mm rods, joint to joint) were never seen on a sheet.
- 1:50 body: sleeve head is a rounded cap with a visible seam step on the shoulder from behind; cuffs show crease patches at the wrist; heel counters of the shoes show crease lines; front view of the knees not re-read after the exponent change.
- 1:50 body: no finger nails, no knuckle creases (below 7.5 mm).
- Clip `settle-back` at 4 s: the left arm passes through a raised side position on its way behind the head (interpolated, plausible but not keyed).
- 1:200 / 1:500: hand is one blunt solid on the hand bone; the finger bones carry nothing there (by feature size).
- 2D 1:100 / 1:200 hands not re-read against the new finger blocks (silhouette rule green).

### Fundamental issues (outside this asset)

- `lateral-ambiguous` started to fire during the pass on flat elevation shapes of bones with a rest `abductionDeg` (the star arms): the rule changed under the authors while `rig-lateral-rest.md` was not yet written. Fixed here in the drawing (extrusions), but every asset with rest-spread bones and flat shapes will fail the same way.
- `validator.md` has no section "Rule changes authors must know" (the task pointed to it).
- `r3-2d.ts` shows 1.42 m as the height dimension at 1:50 and 1.41 m at 1:500 for `dimensions.heightM` 1.409 (the dimension seems to be measured on the drawing, not read from `dimensions`).
- The Read tool (image viewing) stopped responding near the end of the pass (hook timeout), so the last two sheets could not be judged.
