# humans/seated-reader — rework report

Asset: a woman of 1.68 m (61 kg) reading a top-bound A5 notebook on a slatted park bench, paper cup beside her. Source `T/volumes/humans/seated-reader.json`, catalog `S/🗂️catalog/👤️humans/seated-reader/🔣️.json`. Edit scripts: `T/rework-seated-reader-*.py`.

## Pass 1 (second round), 2026-11-22

### State found

- Source and catalog `volumes` were identical (no half-written edit). No earlier report on this asset (`rework/03.md`: "not started").
- Rendered and read before any edit: 1:50 body and structure, default pose.
- Body 1:50 was already stated by hand (lofts with real sections, both hands with four fingers and thumb, sneakers with soles and laces, bench with posts, rails, slats and bolts) and reads as a seated woman from all six sides.
- Rig: 29 bones, sagittal chain only. One rigid 0.477 m `torso` from L5/S1 to C7/T1 (no lumbar joint), no clavicles, no hip bones: arms hung from the trunk 165 mm beside the plane, thighs from the pelvis 85 mm beside it. Hands and feet alike on both sides; fingers are baked into the hand bone (no finger bones).
- Structure 1:50: spine one smooth rod without vertebrae, clavicle, scapula, acromion, ilium, pubis and ischium all bound to `torso` / `pelvis`; at 1:500 one transverse shoulder bar and one pelvic bar.

### Reference (woman 1.68 m; Drillis-Contini shares of stature H, de Leva 1996 female, DIN 33402-2 female P50) against the rig

| Quantity | Reality | Model | Verdict |
|---|---|---|---|
| Thigh, hip joint to knee axis | 0.245 H = 0.412 m | 0.4106 | right |
| Shank, knee to ankle | 0.246 H = 0.413 m | 0.413 | right |
| Foot, ankle to ball / ball to toe tip | 0.15 / 0.06 m | 0.1516 / 0.0614 | right |
| Upper arm, shoulder to elbow | 0.186 H = 0.312 m | 0.3115 | right |
| Forearm, elbow to wrist | 0.146 H = 0.245 m | 0.2454 | right |
| Hand, wrist to finger tip | 0.108 H = 0.181 m | 0.1794 | right |
| Hip joint to L5/S1 | 0.09-0.10 m | 0.0944 (`pelvis`) | right |
| L5/S1 to T12/L1 (lumbar, lordotic) | 0.16-0.18 m | none, part of a rigid 0.477 m bone | wrong skeleton -> `lumbar` 0.17 |
| T12/L1 to C7/T1 (thoracic, kyphotic) | 0.29-0.31 m | part of the same bone | wrong skeleton -> `torso` 0.307 |
| C7 to atlas / atlas to vertex | 0.11 / 0.13 m | 0.111 / 0.125 + skull cap | right |
| Hip joint centres apart | 0.16-0.18 m | 0.17 | right |
| Shoulder joint centres apart (biacromial 0.355 P50, 0.37 P75) | 0.31-0.33 m | 0.33 | upper end, kept (thick sweater) |
| Clavicle | 0.14-0.15 m | none -> 0.148 | added |
| Hip bone, sacroiliac joint to hip joint | 0.09-0.10 m | none -> 0.093 | added |
| Sitting height, seat to vertex | 0.52 H = 0.87-0.89 m (bowed head less) | 0.87 | right |
| Head height / breadth | 0.22 / 0.15 m | 0.22 / 0.15 | right |
| Bench: seat height / depth / length, back top | 0.43-0.45 / 0.40-0.45 / 1.2-1.8 m, 0.80-0.85 | 0.44 / 0.43 / 1.2, 0.85 | right |

### Stage 1 — rig (`rework-seated-reader-1-rig.py`)

- 29 -> 34 bones. `lumbar` (L5/S1 to T12/L1, 0.17 m, -45..20 degrees, 9.23 kg) and `torso` as thorax (0.307 m, -25..15 degrees, 9.73 kg; the two masses are de Leva's middle and upper trunk and sum to the former 18.96 kg).
- `clavicle-l|r`: sternoclavicular joint 21 mm beside the midline on the manubrium to the acromioclavicular joint 157 mm out, 55 mm further back, 19 mm higher; 0.148 m; lateral rest placement (`abductionDeg` ±66.8, `twistDeg` ±157.3); -8..35 degrees. `ilium-l|r`: sacroiliac joint (35 mm out, 60 mm above and 50 mm behind the hip axis) to the hip joint centre 85 mm out; 0.093 m; -5..5 degrees. `arm-*` and `thigh-*` re-parented without moving (script stops if any rest frame drifts more than 2e-5).
- Poses and clips: every trunk lean is shared 65 % lumbar / 35 % thorax when bending forward and is all lumbar when reclining (the backrest contact keeps its place). 14 keys of `stand-up` rewritten the same way.
- Re-bound: `backrest` anchor, trunk masses and joints, every torso solid at four scales and three lists (thorax frame), structure solids of each girdle half written in the frame of their bone (clavicle, scapula, acromion, shoulder bar; ilium, pubis, ischium, hip bar); 1:500 shoulder and pelvic bars replaced by one bar per clavicle and hip bone, joint to joint.
- Body, thermal, 2D at 1:50, 1:100, 1:200: the sweater is a waist part on `lumbar` and a chest part on `torso`, overlapping 65 mm at T12, so a bent trunk stays closed; hem on `lumbar`. Elevation and front shapes cut the same way (`sweater-waist`).
- Structure 1:50: lumbar column of 5 vertebral bodies with discs (48 x 36 mm tapering to 42 x 32) and thoracic column of 12 (38 x 30 to 26 x 22) along the authored curve (lordosis 21 mm, kyphosis 23 mm), one spinous process per vertebra. 1:100 to 1:500: the spine rod cut at T12 into a lumbar and a thoracic member.

### Stage 2 — skeleton as one load path (`rework-seated-reader-2-skeleton.py`)

The measured rule `volumes-connected` found the skeleton in pieces at every scale (arms, scapulae, iliac blades, finger bones, notebook, legs at 1:100, pelvis against the seat at 1:200 and 1:500) and the shoes loose from the shanks at 1:100 and 1:200. Rebuilt by hand:

- 1:50 and 1:100: manubrium (36 x 52 x 16 mm) under the sternoclavicular joints; acromion carried on to the acromioclavicular joint (new at 1:100); spine of the scapula (1:50); glenoid with neck from inside the blade to the humeral head; body of the ilium from the blade to an acetabulum of 42 mm, auricular part from the sacroiliac joint to the blade.
- Hands 1:50: metacarpals with base 10.4, shaft 8, head 11.2 mm reaching the knuckle; phalanges swell at the joints (10 / 9.2 mm) over 8 mm shafts; thumbs end on the notebook board. 1:100: one thumb bone per hand (15 mm) that lies on the board.
- Ischial tuberosities bear on the seat: 7 mm lower at 1:100; at 1:200 the hip bar runs 16.5 mm further down; at 1:500 the bar per hip bone runs sacroiliac joint, hip joint, seat.
- Body 1:100, 1:200: the shank runs 27 mm into the shoe collar (ankle 72 x 80 mm).

### Stage 3 — 1:500 printable (`rework-seated-reader-3-thigh500.py`)

The product refused to draw 1:500 (`PRINT unsuitable`): in the plan the visible strip of the left thigh between forearm and notebook was 97 mm (0.194 mm on paper). Found by removing the thigh from one view at a time. Thigh restated at 154 mm above the knee, 134 mm at the knee, 112 mm at the rounded end, in the solid and in the profile of all three drawings.

### Stage 4 — decisions (`rework-seated-reader-4-texts.py`)

One decision per scale (en, de) on trunk, spine, girdles, hands and seat bearing.

### Proof

- `r3-validate.ts humans/seated-reader --catalog-only --volumes`: `VALID humans/seated-reader (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (after the last publish). Before the pass the same command gave 2 print findings (1:500 not drawable) and 8 `volumes-connected` findings.
- `rig-lateral-rest.md` existed at the start and at the end; the girdles follow its sections 1.3, 1.4 and 1.6.
- Read after the change that affects them: structure 1:50 six views (stage 1), back and three-quarter close-ups (stage 2), structure 1:100 and 1:500; body 1:50 `leaning-forward`, `reclined-reading`, `stand-up` at 2 s; body 1:500; thermal 1:50 cut; 2D 1:50 default and 2D 1:100 `leaning-forward`.
- Rendered but **not read**: body 1:50 `reaching-for-cup`, `rising`, `standing`, `page-turn` at 1.2 / 1.7 s, `stand-up` at 1.2 s, `lean-back` at 0.8 / 3 s, body 1:100 and 1:200, structure 1:200, thermal 1:500, 2D 1:500. Not rendered: thermal 1:100 and 1:200, 2D 1:200. The structure close-ups were read before the manubrium was moved 8 mm back, the 1:500 body before the thigh was thickened.

### Still imperfect (for the next pass)

- Rig: no finger bones (fingers are baked into `hand-*`, alike on both sides); `standing-adult` and `seated-office-worker` have 14 phalanx bones per hand. Shoulder joint centres 0.33 m apart are the upper end for a 1.68 m woman (0.31-0.32 typical); narrowing means shifting the arm front and plan shapes.
- Poses: no pose or clip moves `clavicle-*` or `ilium-*` (the raised arms of `reclined-reading` should lift the clavicles by about 10 degrees with the arm offsets re-solved). The trunk lean is still taken by `lumbar` and `torso` only; `pelvis` stays at 0 in `leaning-forward` (38 degrees forward realistically includes 15-20 degrees of pelvic tilt with the thighs compensated).
- 1:50 structure: skull without orbits, nasal opening or teeth; vertebrae are bodies, discs and spinous processes only (no arches, transverse processes or facet joints); ribs are 10 mm round rods; iliac blade is a flat ellipsoid without crest or spines; carpals are one ellipsoid; hand phalanges are one bent rod per finger; foot is calcaneus, tarsal block and toe block (no metatarsals); patella, femoral condyles and tibial plateau not re-judged in close-up.
- 1:100 structure: the ischium was lowered 7 mm and at 1:200 the hip bar 16.5 mm so that the skeleton touches the seat (`volumes-connected` allows no gap under one feature); in reality 10-20 mm of tissue and cloth lie between.
- 1:50 body: sweater shows a crease line at T12 where waist and chest part overlap (visible from behind in `leaning-forward`); hands, face and shoes were not re-judged in close-up in this pass.
- 1:200 body and thermal split of the sweater, 1:500 thermal, 2D 1:200 and 1:500: valid by the rules but not looked at (see Proof).
- 2D: `sweater-waist` exists in elevation and front only (plan keeps the one `sweater` shape on `torso`); at 1:500 the sweater drawing is one shape on `torso`.
- Thermal 1:50 cut sheet shows a small dark wedge near the ground in the three-quarter panel (a cut bench part); not investigated.

### Fundamental issues (outside this asset)

- `rig-lateral-standing-adult.py` `splice` raises "unknown layout" for `representations` of this catalog file (neither `catalog_text` nor `volumes-insert` layout); `rework-seated-reader-1-rig.py` falls back to the `volumes-insert` layout for that member, which reformats it.
- `PRINT nominal-feature` names only the shape, not the view or the place of the thin visible part; finding it took three trial candidates.
- A loft whose sections have `rx` different from `ry` and no `rotation` swaps its axes when the path leans (validator note 7): appending one leaning section to the existing `sternum` turned its plate by 90 degrees. Such solids exist in this asset (`sternum`) and are stable only as long as nobody edits their path.
- Under load `r3-validate.ts --volumes` took 3 to 5 minutes per run for this asset.
