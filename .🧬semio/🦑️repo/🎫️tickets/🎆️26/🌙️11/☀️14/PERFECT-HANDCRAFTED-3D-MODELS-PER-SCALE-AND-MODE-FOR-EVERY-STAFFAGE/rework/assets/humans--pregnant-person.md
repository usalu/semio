# humans/pregnant-person — rework report

## Pass 1 (second round), 2026-11-22

Scripts (all in the ticket folder): `rework-05-pregnant-person-rig.py` (rig, re-binding of solids, 2D shapes, anchors, masses, poses, clips), `rework-05-pregnant-person-structure.py`, `rework-05-pregnant-person-girdle.py` (skeleton load path), `rework-05-pregnant-person-texts.py` (decisions). Each refuses a second run.

State found: catalog `volumes` and `T/volumes/humans/pregnant-person.json` were identical (the r3 round's full build; no half-written edit). `rig-lateral-rest.md` existed at the start and was unchanged at the end (13:19).

### Reference (woman, 1.675 m barefoot, 1.70 m on 25 mm heels, week 36, 72 kg)

| Measure | Reference | Model before | Model now |
| --- | --- | --- | --- |
| thigh (hip centre to knee centre) | 0.245 H = 0.410 m (Drillis-Contini) | 0.407 | 0.407 |
| shank (knee to ankle centre) | 0.246 H = 0.412 m | 0.397 | 0.397 |
| ankle centre above sole | 0.039 H = 0.065 m + 25 mm heel | 0.087 | 0.087 |
| foot, ankle to ball (MTP) | 0.15 to 0.16 m, 30 to 35 degrees below the horizontal | bone 0.128 m level at ankle height, toe joint offset 87 mm below its tip | 0.1546 m, 34.1 degrees down, tip = ball on the ground |
| upper arm / forearm / hand | 0.186 H = 0.312 / 0.146 H = 0.245 / 0.108 H = 0.181 m | 0.310 / 0.244 / 0.179 | same |
| lumbar spine L5/S1 to T12/L1 | 0.16 to 0.18 m | no bone (one 0.458 m torso from the sacrum) | `lumbar` 0.170 m, `torso` (thorax) 0.288 m |
| clavicle | 0.14 to 0.15 m (female), 15 to 20 degrees back, lateral end higher | no bone, arm hung from the torso at z 0.16 | `clavicle-l/-r` 0.148 m from the sternoclavicular to the acromioclavicular joint |
| biacromial breadth | 0.355 to 0.365 m (DIN 33402 female P50) | joint centres 0.32 m apart, acromion about 0.355 | unchanged |
| hip joint centres apart | 0.17 to 0.18 m | 0.17 | 0.17, each thigh on `ilium-l/-r` (0.090 m, sacroiliac joint to hip joint) |
| head height | H / 7.5 = 0.223 m | 0.225 | same |
| thigh / knee / calf / ankle girth half axes | 0.09 / 0.055 / 0.057 / 0.035 m | 0.088 x 0.095 / 0.05 / 0.053 x 0.058 / ok | same |
| abdomen, week 36 | apex 0.25 to 0.28 m ahead of the hip joints, about navel height | 0.26 m at 1.02 m | same |

### Defects found

- Rig: no lumbar segment, no clavicles, no pelvis halves; the foot bone ran level from the ankle at ankle height and the toe joint was offset 87 mm below its tip (a bone floating over the shoe in every sheet). Hands are one bone each (both alike, fingers as solids on the hand bone).
- Structure, all pre-existing in the catalog (`volumes-connected` reported 4 findings on the untouched catalog file): at 1:50 hip bones did not meet the sacrum (each leg with its hip bone was a loose group), the patella floated 8.5 mm in front of the knee, the glenoid was not carried by the scapula, the acromion ended 14 mm from the clavicle, rib 4 met neither vertebra nor sternum, the fetal skull and both fetal arms were loose and the fetal skeleton hung in the air; at 1:100 and 1:200 no rib met the spine (12 to 20 mm), the femur ended 22 mm above the knee centre, the humerus did not meet the girdle rod.
- Body: segment lengths and girths agree with the table; no solid was changed apart from the boot heel (see below). Judged at full size in the right view: the abdomen meets the sweater in a visible intersection seam at its back and its underside ends in a point where it meets the hem. Not rebuilt in this pass.

### Rebuilt

- **Rig** (20 bones before, 25 now): `lumbar` inserted with the old torso frame (its solids, the belly bone and the pose/clip offsets of the old torso moved to it; joint -30..20), `torso` is the thorax from T12/L1 (-12..8; mass split 9.728 / 10.258 kg by the de Leva middle / upper trunk shares, combined centre of mass unchanged); `clavicle-l/-r` and `ilium-l/-r` on the lateral rest placement (solved with the helpers of `rig-lateral-standing-adult.py`, joint axes back on the left and ahead on the right so the same `angleDeg` lifts the shoulder / hikes the hip on both sides); arms and thighs re-parented without moving (2e-6 m, checked with `rig-rest.ts`); foot bone ankle to ball, toe re-solved so it stays where it was. Re-bound: every torso solid of all scales and lists (translated), the five torso elevation shapes per 1:50 and one per coarser scale, foot solids, foot elevation shapes, heel and ball anchors and the foot centre of mass (turned 34.1 degrees), clavicle, scapula, scapular spine, glenoid and the coarse `shoulder-girdle` to the clavicle bones, hip bone, rami and acetabulum to the ilium bones. The extruded boot heel became a two-section loft because an extrusion cannot lie oblique in the new foot frame.
- **Structure**: 1:50 iliac body (sacroiliac joint to acetabulum), arched iliac crest, sternoclavicular and sacroiliac joint solids, acromion to the clavicle end, scapular neck to the glenoid, rib heads on their vertebral bodies, patella on the trochlea (22 mm thick) with patellar ligament, thoracolumbar joint, lumbar mass marker, fetal spine into the skull base and fetal shoulders on the rib cage, uterine wall as an 8 mm band in the median plane from the lumbar spine around the fetus. 1:100 / 1:200: lumbar and thoracic spine as two rods, iliac body, girdle rod through the glenoid, rib heads in the spine, femur down to the knee centre, patella and ligament (1:100), uterine wall at the feature size. 1:500: two spine rods, one clavicle bar per side.
- **2D**: only the re-binding above; `rig-lateral-compare.ts` before / after: at most 0.05 mm in the four poses, up to 6 mm in the clips because the breathing motion of `thermal.respiration.motion` (torso 0.9 degrees) now turns the thorax about T12/L1 instead of the whole trunk about the sacrum.
- **Decisions**: structure text of every scale extended (en, de).

### Proof

- `r3-validate.ts humans/pregnant-person --catalog-only --volumes`: `VALID humans/pregnant-person (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Read after the last geometry change: 1:50 body (six views, right view full size), 1:50 structure, 1:100 structure, 1:200 body, 1:50 thermal cut, poses `cradling-belly` and `stretching-back`, clip `waddle-walk` at 0.3 s, 2D 1:50 in `hand-on-back` and `waddle-contact`.

### Still imperfect (open list for the next pass)

- rig: no finger bones (15 per hand); `lumbar` and `torso` only split the old trunk angle (all pose and clip offsets sit on `lumbar`, the thorax never bends); no pose or clip uses the clavicles or the hip bones (no shrug, no pelvic list in the waddle).
- 1:50 body: abdomen (belly-cover, belly-drape) meets the sweater in a seam and a pointed underside instead of one stretched knit surface; sweater back hem flares like a skirt; hands, face, boots not examined in close-up.
- 1:50 structure: iliac wing is still a flat five-section plate, scapula a flat plate 8 cm behind the glenoid (bridged by an over-long neck), skull is cranium + box + mandible; the patellar ligament and uterine wall use the material `bone`; feet were not checked against the boot.
- 1:100 / 1:200 / 1:500: body and thermal lists only re-bound, not re-modelled; 1:100 hip wing reads as a square plate from behind; 1:500 body, 1:100 body, all coarse thermal sheets and 1:200 / 1:500 structure sheets were rendered but not read.
- thermal: every list only re-bound; 1:50 read once in the cut view, the others not.
- 2D: no shape redrawn; not read at 1:100, 1:200, 1:500.
- not read after the last change: pose `waddle-contact` in 3D, `waddle-walk` at 0.95 s, `rest-and-breathe` at 2 s and 4 s.

### Fundamental issues

- `volumes-insert.py` runs `main()` at import, so its `dump` cannot be imported (the rig script reads the function text instead).
- `thermal.respiration.motion` turns a whole bone about its joint; with a one-bone trunk it rocked the whole upper body about the sacrum. Assets that still have one trunk bone show that.
- 1:500 clavicle: 0.148 m is just under the 0.150 m a `slender` member needs, and no neighbouring solid lies within 75 mm of the whole bone for `members`, so the clavicle is a 75 mm bar there.
