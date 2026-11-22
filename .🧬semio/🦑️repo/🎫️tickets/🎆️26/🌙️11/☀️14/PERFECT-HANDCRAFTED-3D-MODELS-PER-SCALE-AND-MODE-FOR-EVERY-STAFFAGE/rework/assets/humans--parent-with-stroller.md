# humans/parent-with-stroller

- 2026-11-22 12:2x, rig verification (not the volume author): `poses` and `clips` re-solved with `rig-reoffset.ts humans/parent-with-stroller --write` because the rig now turns a pose offset about the bone's own rest axes instead of the parent's joint axis. The child's limb bones had moved by up to 5.0 mm (checking-child, kerb-tilt and their clips). Every bone keeps the posed frame it had before the rig change (largest difference of a bone tip after the re-solve under 0.01 mm). Only `asset.poses` and `asset.clips` were written; rig, shapes, volumes untouched. Do not run the tool on this asset again (`rig-reoffset.done.json`); new poses are authored about the bone's own axes (`rig-lateral-rest.md` 1.1).

## Pass 1 (second round) — 2026-11-22, author of this asset

State found: source `volumes/humans/parent-with-stroller.json` and catalog `volumes` identical. `r3-validate.ts --catalog-only` INVALID with 193 `validateAsset` findings (all `volumes-feature`: 1:100, 1:200 and 1:500 carried the 1:50 parts list); measured rules never passed. No girdle bones: arms hung on `torso`, thighs on `pelvis`.

### Reference (adult, 1.75 m, de Leva male segment table as the rig uses it; my numbers)

| Quantity | Reference | Before | Now |
| --- | --- | --- | --- |
| hip joint height | 0.53 H = 0.93 m | 0.925 | 0.925 |
| shoulder joint (glenohumeral) height | acromion 0.818 H = 1.43 m, joint about 40 mm lower = 1.39 m | 1.452 (30 mm below C7) | 1.390 |
| C7 height | 0.86 H = 1.50 m | 1.482 | 1.482 |
| glenohumeral centres apart | biacromial 0.40 m, centre 20–25 mm inside the acromion = 0.35–0.36 m | 0.385 | 0.356 |
| acromion ends apart | 0.39–0.40 m | 0.372 (clavicle ended inside the humeral head line) | 0.370 at the acromioclavicular joint |
| hip joints apart | 0.17–0.19 m | 0.184 | 0.184 |
| upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H = 0.326 / 0.256 / 0.189 | 0.308 / 0.269 / 0.179 | unchanged |
| thigh / shank | 0.245 H / 0.246 H = 0.429 / 0.431 | 0.428 / 0.418 | unchanged |
| elbow height standing | 0.63 H = 1.10 m | 1.14 | 1.08 |
| clavicle length | 0.15–0.17 m | 0.186 capsule on the torso | 0.173 bone |

Defects found: shoulder joint 62 mm too high and 14.5 mm too far out per side (square shoulders level with the collar, no trapezius slope, arms locked straight at the push bar with 0.2° elbow flexion); no clavicle, scapula or hip-bone bones; pubis floating 39 mm from the hip; front casters not touching their forks at 1:50; coarse scales not modelled for their feature.

### Rebuilt

- **Rig** (`rig-lateral-rest.md` followed): new bones `clavicle-l/-r` (sternoclavicular joint [0.066, 0.490, ±0.025] to acromioclavicular joint [0, 0.502, ±0.185], 0.1735 m, limits −8..35) and `ilium-l/-r` (sacroiliac joint [−0.055, 0.070, ±0.035] to hip joint [0, 0, ±0.092], 0.1057 m), solved with `rig-rest.ts --solve`. `upper-arm-*` re-parented to the clavicle with the joint centre moved to [0, 0.4654, ±0.178]; `thigh-*` re-parented to the ilium without moving (rest frames checked with `rig-rest.ts --file`). `stroller.z` −0.0145 keeps the stroller on the centre line. The `upper-arm-r` solve of the tool gave an inconsistent origin for the right side; I used the mirror rule of the guide and checked the frames.
- **Poses and clips**: all 14 poses and the one clip key with overrides re-solved by two-bone reach so that the wrist and the hand angle in the torso frame stay exactly where they were (stroller, wheel contacts and the child do not move); elbows now flex 25°–61° instead of 0.2°–45°. Rendered and read after the change: `pushing-stroller` (six views, 1:50/200/500), `checking-child` (six views). Rendered but NOT read: `kerb-tilt`, `push-walk` t 0.3. Not rendered: `pausing-with-stroller`, `stop-and-check-child`, `kerb-tilt-up`, other instants.
- **Body**: `jacket-body` upper sections re-modelled at all four scales (shoulder slope from the neck to the deltoid); the three nested thermal jacket layers of 1:50 follow with their own inset. 1:500 upper sleeve 6 mm thicker (print minimum of the band).
- **Structure**: clavicle (S-curved loft at 1:50, bar at 1:100, slender true-section bar at 1:200/1:500), scapula, scapular spine, new scapula neck, acromion (1:200) on the clavicle bone; ilium, ischium and new pubic rami on the ilium bone; hip-bone masses at 1:200/1:500; the V-shaped fake `shoulder-girdle` bar removed.
- **Coarse scales** (`rework-05-parent-with-stroller-coarse.py`): 1:100 fork prongs and spokes at 15 mm, small long bones declared slender; 1:200 fist instead of palm and fingers, hand skeleton as one plate, fork as one strut, wheel web / disc, tubes 30 mm; 1:500 wheels as discs, tubes 39 mm, toddler limbs 75 mm, fist; thermal `of` entries and structure `of` entries re-pointed.
- **2D**: torso front outline redrawn at all four scales for the sloped shoulder; 1:500 upper-arm profiles widened. Arm front/plan shapes are bone-local solids and follow the rig. Plan and elevation outlines of the torso not redrawn.
- **Decisions**: one sentence per scale added (en, de); the stale shoulder height in the 1:50 text corrected.
- Front axle pins added at 1:50 (body, thermal, structure).

Scripts (run once, in this order, already applied): `rework-05-parent-with-stroller-girdle.py`, `-coarse.py`, `-publish.py`, `-acromion.py`.

### Validation

`r3-validate.ts humans/parent-with-stroller --catalog-only --volumes`: `INVALID … schema 0, validateAsset 0, checkVolumes 8` (before: validateAsset 193, measured rules not run). The asset is published in this state because it is better in every list than what was there; it is not VALID.

### Remaining (not done, stated plainly)

1. 1:500 body vs elevation drawing: IoU 0.899 (limit 0.9), largest distance 92 mm; cause not located (wheel discs, thicker tubes or the bent elbow against the old elevation shapes). The 1:500 elevation needs redrawing with `r3-project2d`.
2. Toddler skeleton, all scales, structure: pelvis halves, pubis, clavicles, humeri and hand bones do not touch (16 mm at 1:50, 39–48 mm at 1:100/1:200, 16 mm at 1:500); the toddler was not reworked at all in this pass (rig, body, skeleton untouched except coarse thickening).
3. 1:100 body: `child-hand-*` 6.2 mm from the forearm; 1:200 body: toddler group 11.7 mm clear of the stroller.
4. Stroller structure: `hood-bow` 53 mm from the push bar at 1:200 (21 mm seat rail at 1:100); stroller not re-modelled in this pass beyond the coarse merges.
5. 1:100 structure: patellae 10.3 mm from the femoral condyles.
6. Not looked at after the change: thermal sheets at any scale, structure sheets at 1:100/1:200/1:500, 1:100 body sheet, 2D drawings (`r3-2d` at 1:50 was overwritten by another agent's candidate of the same file name, 1:500 was refused before the sleeve fix and not re-rendered). `rig-lateral-compare.ts` crashes on this asset in `unionPolygons`, so "the stroller did not move" rests on the reach solve and on the sheets, not on a measurement.
7. The right ilium blade looks edge-on and long in the three-quarter structure view; the re-bound hip-bone lofts were not checked in a close-up.
8. Upper arm 0.308 m is 18 mm short of 0.186 H, forearm 13 mm long; left as the de Leva table has them.
9. The catalog file was re-serialised by my writer; a round trip of the unchanged file was not byte-identical (cause not examined), so the diff of this asset is larger than the content change.
10. Head, hands, feet, legs of the adult at 1:50 were judged from the six-view sheet only, no close-ups; I do not sign the model yet.

### Fundamental issues

- `rig-rest.ts --solve BONE --parent <mirrored lateral bone> --at …` printed x −0.164 / z −0.139 for `upper-arm-r` where the mirror of the left solution is x 0.1645 / z −0.0037.
- `rig-lateral-compare.ts` throws in `unionPolygons` (`🔺️polygon`) via `placeShape` for this asset.
- `r3-sheet.ts --file` / `r3-2d.ts --file` name their output after the candidate's folder; two agents with a folder `cand` overwrite each other's PNG.
- The `PRINT nominal-feature` message names the shape but not the view or the place, which made the 1:500 sleeve finding expensive to locate.
