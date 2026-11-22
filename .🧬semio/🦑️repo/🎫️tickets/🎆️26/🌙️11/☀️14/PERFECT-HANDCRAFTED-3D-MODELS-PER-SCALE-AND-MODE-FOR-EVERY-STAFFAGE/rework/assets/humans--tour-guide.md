# humans/tour-guide — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published and `VALID` (`r3-validate.ts humans/tour-guide --catalog-only --volumes`: schema 0, validateAsset 0, checkVolumes 0). Source `volumes/humans/tour-guide.json` and catalog agree (they also agreed at the start; nothing was left half-edited by earlier passes).

Chisel of this pass: `rework-tour-guide-girdle.py` (run after `rework-tour-guide-skeleton.py`, which rebuilds the structure lists on the old frames; the girdle script is idempotent and then converts them again).

### Reference table (man, 1.75 m, 78.6 kg, walking shoes with 15 mm sole)

| Measure | Reference | Model | Verdict |
| --- | --- | --- | --- |
| Stature (with shoes) | 1.765 m | crown of the skull 1.764 m | ok |
| Head, chin to vertex | 1/7.5 of stature = 0.233 m | about 0.235 m | ok |
| Hip joint height | 0.530 H + sole = 0.943 m (Drillis-Contini) | 0.925 m | 18 mm low |
| Thigh (hip joint to knee) | 0.245 H = 0.429 m (Drillis-Contini), 0.424 m (de Leva) | 0.411 m | 3–4 % short, not corrected |
| Shank (knee to ankle) | 0.246 H = 0.431 m | 0.431 m | ok |
| Ankle height | 0.039 H + sole = 0.083 m | 0.083 m | ok |
| Upper arm | 0.186 H = 0.326 m | 0.3255 m | ok |
| Forearm | 0.146 H = 0.256 m | 0.2555 m | ok |
| Hand | 0.108 H = 0.189 m | 0.189 m | ok |
| Shoulder joint height | 0.818 H + sole = 1.447 m | 1.4465 m | ok |
| Glenohumeral centres | ±0.165–0.175 m (biacromial 0.39–0.40 m) | ±0.175 m | ok |
| Hip joint centres | ±0.085–0.09 m | ±0.088 m | ok |
| Clavicle | 0.15–0.165 m, runs about 20° back and 7–10° up | 0.1624 m, sternoclavicular [0.060, 0.5276, ±0.022] to acromioclavicular [-0.004, 0.5473, ±0.170] (y from the hip joint) | new |
| Hip bone, sacroiliac to hip joint | about 0.10 m | 0.0997 m, [-0.060, 0.070, ±0.050] to [0, 0, ±0.088] | new |
| Iliac crest above hip joint | 0.13–0.14 m | 0.137 m (+ 8 mm rod radius) | new |
| Anterior superior iliac spines | ±0.12 m, 0.05 m ahead, 0.08 m above the hip joint | [0.052, 0.082, ±0.122] | new |
| Mid-thigh girth (in trousers) | 0.55 m + cloth | about 0.585 m | ok |
| Calf girth (in trousers) | 0.38 m + cloth | about 0.40 m | ok |
| Clavicle lift at 127° arm flexion | 6–11° (Ludewig 2009) | 8° | new |

### Defects found

- Rig: no shoulder girdle and no pelvic girdle; arms hung from the trunk bone 0.175 m beside the plane, thighs from the pelvis bone. Clavicle, scapula, acromion, ilium, pubis and ischium solids were bound to trunk and pelvis, so a raised arm never lifted a shoulder.
- 1:50 / 1:100 structure: clavicle a straight capsule; hip bone an upright oval disc beside the sacrum without flare, crest, acetabulum or closed obturator foramen; tibia lofts pinched to an hourglass in the lower third (loft without `rotation` along `u`, the section frame turns between sections).
- 1:200 structure: one bar from shoulder to shoulder instead of clavicles; tibia and forearm as 35 mm rods.
- 1:500 structure: every bone a 60–78 mm thick rod (femur 78 mm), against rule change 2 of `validator.md`.
- 1:100 structure: scapula had no link to the clavicle (no acromion).
- Body, thermal, 2D, poses and clips: read as a credible clothed man at all four scales; no defect that I would rebuild found in this pass (see "Remaining").

### Rebuilt

- **Rig**: `clavicle-l/-r` (parent `torso`, lateral rest placement solved with the frames of `rig-lateral-rest.md`, joint -8..35°, twist limits mirrored) and `ilium-l/-r` (parent `pelvis`, joint ±3°). `upper-arm-*` and `thigh-*` re-parented without moving (rest frames equal within 2e-5), so their solids, 2D shapes, anchors, masses and thermal regions are untouched. 25 bones, both sides mirror images.
- **Poses**: clavicle lift by scapulohumeral rhythm — `standing-pointing` 8° (right), `waving` / `wave-swing-a` / `wave-swing-b` 12° (right), `holding-flag` 4° (left), `gesturing-to-group` 3° (right); the upper-arm offsets were solved again (angle, abduction, twist) so every posed arm keeps its direction (hand frames within 4e-4). Clips reference poses only and needed no key change.
- **Structure 1:50 and 1:100**: clavicle as S-curved loft (sternal end 22 mm, shaft 13 mm, acromial end 16 mm) on the clavicle bone; scapula and acromion re-bound to it (same world geometry), acromion added at 1:100. Hip bone on the ilium bone: iliac wing flared 20° and turned from the posterior to the anterior spine, iliac crest rod, acetabulum, ischiopubic ramus; pubis and ischium re-bound. Tibia lofts state their section rotation (pinch gone).
- **Structure 1:200**: two clavicle rods at the true section (`slender`), tibia, fibula, ulna and radius at the 1:50 shaft section (`slender`), pelvic ring names both hip bones (`members`).
- **Structure 1:500**: spine from the sacrum, cervical column, clavicles, femur (bent through its neck into the hip joint), tibia, humerus, ulna and radius at the true shaft section (`slender`); pelvic bar names both hip bones; joint markers at shoulders and ankles close the load path.
- **Decisions**: structure text of all four scales rewritten (en, de).
- `dimensions`, 2D `representations`, body and thermal lists: unchanged.

### Rendered and read after the last geometry change

Structure 1:50 (three-quarter close-up of the candidate before the tibia fix, back view, waving front), 1:100, 1:200, 1:500 six views; body 1:50 `standing-pointing` front, `holding-flag`, `waving`, clip `gesture-point` t 1.9 s and `turn-and-wave` t 1.7 s; thermal 1:50 cut; 2D 1:50 default pose. Rendered but **not read**: body 1:50 `gesturing-to-group`, `turning-to-group`, `wave-swing-a`, `wave-swing-b` after the rig change (I read the sheets of `gesturing-to-group` from before the change by mistake), clip instants `gesture-point` 5.3 s and `turn-and-wave` 3.1 s, structure 1:50 six views after the tibia fix. Body and thermal of 1:100, 1:200, 1:500 were read from the renders at the start (those lists did not change); thermal 1:100–1:500 not read. All PNGs are deleted.

### Remaining (for the next pass)

- 1:50–1:500, body and thermal, shoulders: no jacket yoke on the clavicle bones, so a lifted clavicle raises the sleeve but not the shoulder line of the jacket; in `waving` (12°) scapula and acromion may leave the jacket (the rule measures the default pose only, where they stay inside by about 1 mm at 8°). Needs a body and thermal solid per side on `clavicle-*` and its 2D shapes (`solid` shapes in the elevation).
- Rig, thigh: 0.411 m is 13–18 mm short against the table and the trunk correspondingly long; correcting it moves hip height and every trunk solid, 2D shape and anchor.
- Rig, trunk: one `torso` bone of 0.457 m, no lumbar / thoracic split; no finger bones (both hands alike, fingers are solids on the hand bone).
- 1:50 structure: scapula is an ellipsoid blade without spine, glenoid and coracoid; iliac wing renders as a faceted disc; skull is cranium, face and mandible bars without orbit or zygomatic arch.
- 1:50 body: knee shows as a seam between thigh and shank lofts (no patella form); face is mask-like.
- Poses and clips not read after the change (list above); 2D drawings of 1:100, 1:200, 1:500 and of the poses other than the default not rendered in this pass.
- `rework-tour-guide-skeleton.py` still writes the old girdle on trunk and pelvis; the girdle script must run after it.
- `🧪️tests/🧊️model-skeleton` and the other suites were not run.

### Fundamental issues

- Kernel, loft: a loft without `rotation` on a path nearly along `u` turns its section frame between sections; even a circular section then pinches to an hourglass (tibia of this asset). Documented in `validator.md` item 7 for `rx` / `ry`, but the pinch of circular sections is a visible defect that authors will not expect.
- `r3-sheet.ts`: `--no-labels` changes the output file name (`….no-labels.png`) while `--no-rig` does not, so a sheet with and without rig overwrite each other and stale sheets are easy to read by mistake.
- `volumes-structure-coverage` (members inside the body silhouette) is measured in the default pose only; a girdle bone that lifts in other poses is not checked there.
