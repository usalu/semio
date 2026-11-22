# humans/seated-guitarist — rework report

Scripts: `T/rework-03-seated-guitarist-rig.py` (girdles, shoulder breadth, right hand, pose re-solve), `T/rework-03-seated-guitarist-coarse.py` (1:100, 1:200, 1:500 volumes). Batch report `T/rework/03.md` lists this asset as "not started"; no earlier per-asset report existed.

## Pass 1 (second round), 2026-11-22

### State found

- Source `T/volumes/humans/seated-guitarist.json` and catalog `asset.volumes` were identical (no half-written edit).
- `r3-validate.ts humans/seated-guitarist --catalog-only --no-print`: INVALID, 80 findings, all `volumes-feature` at 1:100, 1:200 and 1:500 (the coarse lists carried the 1:50 guitar and hand parts: fretboard 7.8 mm, tuner buttons 11 mm, posts 8.6 mm, metacarpals 8.4 mm, carpals, thenar, occipital condyles, joint markers).

### Reference (written before any edit)

Subject: man, about 1.76 m barefoot (1.78 m in boots), 74 kg, flannel shirt over T-shirt, jeans, boots; on a 24 inch (0.61 m) wooden stool with rungs; dreadnought guitar (scale 645 mm, body 505 x 400 x 100-125 mm, neck 43 mm at the nut).

| Quantity | Reality (Drillis-Contini x 1.76 m, DIN 33402-2 P50 male) | Model found | Verdict |
|---|---|---|---|
| Thigh, hip centre to knee axis | 0.245 H = 0.431 m | 0.430 | right |
| Shank, knee to ankle | 0.246 H = 0.433 m | 0.435 | right |
| Ankle above sole (boot) | 0.039 H + 15 mm = 0.084 m | 0.080 | right |
| Foot in boot | 0.152 H + 15 mm = 0.283 m | 0.03 heel + 0.130 + 0.092 toe + cap = 0.28 | right |
| Upper arm, shoulder centre to elbow | 0.186 H = 0.327 m | 0.327 | right |
| Forearm, elbow to wrist | 0.146 H = 0.257 m | 0.257 | right |
| Hand, wrist to middle finger tip | 0.108 H = 0.190 m | left 0.105 palm + 0.045 + 0.028 + 0.019 = 0.197; right one 0.19 m bone without fingers | left right, right hand not articulated |
| Hip centre to L5/S1, L5/S1 to C7/T1, C7 to atlas, atlas to vertex | 0.10 / 0.47-0.50 / 0.10-0.12 / 0.13-0.15 m | 0.10 / 0.475 / 0.10 / 0.155 (vertex 0.149 above the atlas) | lengths right; trunk is one rigid bone |
| Head: chin to vertex, breadth, depth | 0.23 (1/7.6 H), 0.155, 0.195 m | 0.227, 0.152, 0.194 | right |
| Hip joint centres apart | 0.17-0.18 m | 0.18 | right |
| Shoulder joint centres apart (biacromial 0.395) | 0.35-0.36 m | 0.38 | 12 mm per side too wide |
| Clavicle, sternoclavicular to acromioclavicular joint | 0.15-0.165 m, runs 15-20 degrees back and 10-15 degrees up, S-curved | no bone; a loft on the torso 0.177 m long | missing bone, too long |
| Hip bone, sacroiliac joint to hip centre | 0.10-0.11 m, 70 mm down, 55 mm forward, 50 mm outward | no bone; ilium, pubis, ischium on the pelvis bone | missing bone |
| Bideltoid breadth, clothed | 0.47-0.49 m | 0.494 before, 0.47 after narrowing | right |
| Thigh girth top / above knee (jeans) | 0.56-0.60 / 0.40-0.42 m | 0.57 / 0.40 | right |
| Calf / ankle in jeans leg | 0.38-0.40 / 0.34 (hem) | 0.39 / 0.34 | right |
| Upper arm / forearm / wrist in flannel | 0.36 / 0.28 / 0.17-0.18 m | 0.37 / 0.27 / 0.16 | right |

### Defects found

1. Rig: no shoulder girdle (arms hung from the torso at +-0.19 m), no pelvis halves (thighs hung from the pelvis bone), right hand a single 0.19 m bone while the left has 15 finger bones, trunk one rigid bone from L5/S1 to C7.
2. Shoulder joint centres 0.38 m apart instead of 0.356 m; clavicle solid 0.177 m.
3. 1:50 body: right hand five static finger lofts on the hand bone (cannot follow a pose); limbs, trunk, head, boots, stool and guitar read correctly from all six sides.
4. 1:100, 1:200, 1:500: 1:50 parts lists with entries deleted (guitar with 20 fret wires, six strings, tuner posts and spindles at 1:500; left hand with 15 finger pads at 1:500; structure rods of 75 mm at 1:500 beside 8 mm metacarpals).

### Rebuilt in this pass

**Rig** (`asset.rig`, 37 -> 56 bones):

- `clavicle-l/-r` under `torso`, lateral rest placement from the sternoclavicular joint (torso frame 0.412 up, 58 mm ahead, +-20 mm) to the acromioclavicular joint (0.452 up, 4 mm back, +-166 mm): 0.1636 m, 21 degrees back, 14 degrees up; joint axis back / ahead so `angleDeg` shrugs on both sides; limits -8..35, twist as `standing-adult`.
- `upper-arm-l/-r` re-parented to the clavicles; shoulder joint centres moved from +-0.190 to +-0.178 m (0.356 m apart); the arm rest frame is the old sagittal one.
- `hip-bone-l/-r` under `pelvis`, from the sacroiliac joint (pelvis frame 0.07 up, 55 mm back, +-40 mm) to the hip centre: 0.1021 m; `thigh-l/-r` re-parented without moving.
- Right hand: `hand-r` 0.105 m (palm) plus the 15 finger bones of the left hand mirrored (`z` negated, same limits).
- Poses: all five re-solved for both arms (upper arm 3 angles, forearm 2, hand 2; damped Gauss-Newton on the hand frame) so both hands keep their old world frame on the guitar to 0.000 mm; the right hand got a pick grip (index 35/68/32, thumb 24/-11/-60 twist, other fingers curled) in every pose. Clips reference poses only and needed no change.
- 2D: the hand-r polygon is replaced by the mirrored palm and finger-pad shapes of the left hand at every scale; `sweep: "w"` spans of the arm shapes shifted 12 mm inwards.

**1:50 volumes**: clavicle re-authored as an S-curved loft in its own bone frame (sternal end 25 x 23 mm, shaft 12.4 x 12 mm, acromial end 23 x 11 mm); scapula and acromion drawn in to the new breadth and bound to the clavicle bone; ilium, pubis and ischium bound to the hip bones; joint markers for sternoclavicular and sacroiliac joints; right hand body, thermal and structure solids are the mirrored left hand (palm, thenar, 15 pads; carpals, 4 metacarpals, 15 phalanx shafts and joints).

**1:100 / 1:200 / 1:500 volumes**: guitar neck and fretboard one solid (27 mm deep), two tuner rows instead of six buttons, twelve posts and spindles; fret wires and nut removed, strings kept at 1:100 only; metacarpals declared `slender` at 1:100 and 1:200 and merged with the carpals into one hand member at 1:500; thenar merged into the palm at 1:200 and 1:500; occipital condyles left out at 1:200 and 1:500; joint markers of hand and head at the feature of the scale; shoulder bar and pelvic bar at 1:500 name the girdle bones (`members`).

### Validation and what was looked at

See the next section (appended after the runs).
