# `humans/person-with-white-cane` — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. `r3-validate.ts humans/person-with-white-cane --catalog-only --volumes` ends `VALID` (schema 0, validateAsset 0, checkVolumes 0). Source `T/volumes/humans/person-with-white-cane.json` and catalog `volumes` are identical.

### State found

No report existed. An earlier agent (today 13:18–13:58, cut off before reporting) had already rebuilt the asset and published it: scripts `rework-white-cane-lib.py`, `rework-white-cane-skeleton.py`, `rework-white-cane-texts.json`. Source and catalog agreed solid for solid (diff 0 in all 12 lists). What that agent left, judged by me from the sheets:

- Rig with the girdles on the lateral rest placement of `rig-lateral-rest.md`: `clavicle-l/-r` from the sternoclavicular joint [0.05, 1.392, ±0.025] to the acromion [0, 1.422, ±0.165] (0.152 m), `ilium-l/-r` from the sacroiliac joint [-0.055, 0.985, ±0.04] to the hip joint [0, 0.911, ±0.085] (0.102 m); arms and thighs re-parented without moving.
- 1:50 body 99 solids, thermal 130, structure 266 (skull with mandible and face, 6 cervical, 17 thoracic and lumbar vertebrae, ribs to the sternum, scapula with spine, acromion, coracoid and glenoid on the clavicle bone, hip bones with pubis and ischium on the ilium bone, long bones, tarsus and metatarsals, hand bones); 1:100 / 1:200 / 1:500 lists of 37 / 26 / 23 body solids, each modelled for its feature size.

### Reference (woman, 1.71 m in ankle boots, 62 kg; Drillis-Contini / DIN 33402-2 / de Leva)

| Quantity | Reference | Model | Verdict |
| --- | --- | --- | --- |
| Hip joint height | 0.530 H = 0.906 m | 0.911 m | ok |
| Thigh (hip to knee) | 0.245 H = 0.419 m | 0.412 m | ok |
| Shank (knee to ankle) | 0.246 H = 0.421 m | 0.413 m | ok |
| Ankle height incl. 25 mm heel | 0.039 H + 0.025 = 0.092 m | 0.087 m | ok |
| Foot, ankle to ball / toes | 0.16 m / 0.06 m | 0.163 m / 0.061 m | ok |
| Shoulder joint height | 0.818 H = 1.399 m | 1.394 m | ok |
| Upper arm | 0.186 H = 0.318 m | 0.3125 m | ok |
| Forearm | 0.146 H = 0.250 m | 0.245 m | ok |
| Hand | 0.108 H = 0.185 m | 0.181 m | ok |
| Shoulder joint centres apart | 0.32–0.34 m (biacromial 0.36 m, female) | 0.33 m | ok |
| Hip joint centres apart | 0.17–0.18 m | 0.17 m | ok |
| C7 height | 0.86 H = 1.47 m | 1.466 m | ok |
| Head, chin to vertex | H / 7.6 = 0.225 m | 0.235 m with hair | ok |
| Index finger phalanges | 0.043 / 0.025 / 0.019 m | 0.047 / 0.026 / 0.020 m | ok |
| Thumb | metacarpal 0.04 m and two phalanges 0.030 / 0.022 m | left ok; right had one solid for both phalanges | defect |
| Long cane | sternum-high, 1.20–1.30 m, shaft 12 mm, grip 24 mm, roller tip 27 mm | 1.30 m overall, 12 mm, 24 mm, 27 mm | ok |

### Defects found in this pass

1. Hands had no finger bones (one bone per hand), so the modelled grasp could not change: in `cane-parked` the cane stands upright through the hand while the index finger stayed stretched forward like a pointing hand, and the thumb solid passed through the grip.
2. Right thumb: one capsule for proximal and distal phalanx in body and skeleton, the left thumb had both (hands not alike).
3. No report, and the decisions texts did not mention the hands' rig.

### What I rebuilt (`rework-white-cane-hands.py`)

- **Rig**: fifteen bones per hand (`thumb-metacarpal/-proximal/-distal`, `index|middle|ring|little-proximal/-middle/-distal`, `-l` and `-r`), each with its joint axis as the flexion axis (fingers about the hand's breadth, thumb about the palm normal), so `angleDeg` flexes. Their rest placement is the modelled hand (right: constant-contact grasp, index along the grip; left: relaxed), so no existing pose or clip changed. Rig is now 53 bones.
- **1:50 body and structure**: the 29 finger capsules of the body and the 29 phalanx members of the skeleton are bound to these bones (re-expressed in the bone frames, nothing moved in the default pose; checked in the three-quarter close-up before and after). Right thumb split into proximal and distal phalanx lying on top of the grip (body, skeleton, thermal entry `th-hand-r-thumb-mid`). Body 100, thermal 131, structure 267 solids.
- **1:100**: `hand-r-index` (body, with its thermal twin) and `index-bones-r` (structure) bound to `index-proximal-r`.
- **Pose `cane-parked`**: index finger and thumb close round the upright grip (offsets solved from hand-stated fingertip stations in the hand frame: index tip passes over the grip and ends behind it, thumb crosses in front of it). Rendered and read at 1:50 body and structure.
- **Decisions**: one sentence pair (en, de) on the hands added to `volumes.50.decisions` and `volumes.100.decisions`.

### Rendered and read in this pass

Before the change: 1:50 body (six views, front and right close-ups), 1:50 structure, poses `walking-midstance`, `cane-parked`, `diagonal-technique`, clip `constant-contact-walk` 0.7 s, 1:50 thermal cut, 1:100 body, 1:500 body, 1:200 structure, 2D 1:50. After the change: 1:50 body default and `cane-parked` (three-quarter close-ups), 1:50 structure `cane-parked`.

Rendered but not read: 1:50 thermal uncut, 1:100 / 1:200 / 1:500 thermal, 1:100 / 1:500 structure, 1:200 body, clip `constant-contact-walk` 0.15 s, both instants of `listen-and-scan`, 2D 1:100 / 1:200 / 1:500. Not re-rendered after the change: `walking-midstance`, `diagonal-technique`, the clips (their finger offsets are zero, so the hands are the rest hands seen in the default pose).

### Validation

`VALID humans/person-with-white-cane (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (full `--catalog-only --volumes`, after publishing).

### What remains imperfect

- 2D, all scales, `hand-r` shapes (elevation, front, plan): drawn on the hand bone, so in `cane-parked` the drawing still shows the index stretched; the 2D of that pose could not be rendered at all after my change (`r3-2d.ts` fails, see fundamental issues).
- 1:200 and 1:500 body, `hand-r` (one ellipsoid): no index at these scales, nothing to close; fine for the feature size, stated for completeness.
- 1:50 body, right hand in `cane-parked`: middle, ring and little finger keep their rest curl, which was sculpted for the cane lying along the hand; they touch the upright grip but do not wrap it as tightly as the index. Judged only in a three-quarter view at sheet size, not in a hand close-up.
- 1:50 body, torso and legs: correct proportions, but the side view is plain (little bust-to-waist and gluteal depth, calf barely fuller than the ankle under the jeans); the face is a mask with nose, lips and chin and no ears under the bob. Not rebuilt in this pass.
- 1:200 structure, rib cage: three rib hoops round the spine; valid and connected, but it reads as hoops, not as a cage.
- Rig: the trunk is one `torso` bone (no lumbar / thoracic segments), which no pose needs (largest trunk offset 2°) but the brief lists spine segments.
- Thermal sheets of 1:100, 1:200, 1:500 and the unread items above were not judged by me.
- `walk` at 0.7 s: the left arm swings about 0.35 m ahead of the shoulder, at the upper end of a natural swing; left as authored.

### Fundamental issues (outside this asset)

- `r3-2d.ts` stopped working during this pass (it ran at 14:28, failed at about 15:00): `Cannot find module '../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts' from '🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🌊️flow-view/📐️model/🟦️.ts'`. The new flow-view depiction module imports a framework file that does not exist at that path; every 2D render of every author is blocked until it is fixed.
- None else. My PNGs and old logs of this asset under `🗑️generated` are deleted; the candidate documents `🗑️generated/rework-person-with-white-cane/{cand,wcane}/🔣️.json` are left (both older than the catalog state, safe to delete).
