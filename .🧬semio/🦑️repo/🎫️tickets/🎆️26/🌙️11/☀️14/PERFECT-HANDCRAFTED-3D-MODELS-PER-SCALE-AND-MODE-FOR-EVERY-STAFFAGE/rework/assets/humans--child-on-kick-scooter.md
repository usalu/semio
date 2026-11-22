# humans/child-on-kick-scooter

## Pass 1 (second round), 2026-11-22

State found: source and catalog `volumes` identical (untouched since 21 Nov), no earlier report; the asset was the last of author 02 and never started. `r3-validate.ts --volumes`: INVALID, validateAsset 69, checkVolumes 73 (69 `volumes-feature` at 1:100 to 1:500, 4 `volumes-connected` on the structure lists).

State left: `r3-validate.ts humans/child-on-kick-scooter --catalog-only --volumes` and the same run with the source merged: `VALID … schema 0, validateAsset 0, checkVolumes 0`.

### Reference (girl, 8 years, 1.28 m, about 26 kg; child anthropometry after Snyder 1977 and DIN 33402, written from memory; scooter: 145 mm children's two-wheeler)

| Measure | Reference | Model | Verdict |
|---|---|---|---|
| Stature (rest chain) | 1.28 m | 1.289 m | ok |
| Head height chin to vertex / breadth / length | 0.19–0.20 / 0.14 / 0.18 m | 0.197 / 0.140 / 0.184 m | ok |
| Hip joint height | 0.51 H = 0.655 m | 0.658 m | ok |
| Thigh (hip to knee centre) | 0.235 H = 0.30 m | 0.302 m | ok |
| Shank (knee to ankle centre) | 0.23 H = 0.295 m | 0.293 m | ok |
| Foot length | 0.20 m | 0.19 m with heel | ok |
| Upper arm (joint centres) | 0.23 m | 0.229 m | ok |
| Forearm | 0.19 m | 0.179 m | 6 % short, kept |
| Hand | 0.14 m | 0.133 m | 5 % short, kept |
| Shoulder joint height | 0.80 H = 1.03 m | 1.028 m | ok |
| Hip joint spacing | 0.13 m | 0.130 m | ok |
| Biacromial breadth (skeleton) | 0.275 m | was 0.256 m, now 0.278 m (acromion overhangs the humeral head) | fixed |
| Bideltoid breadth, clothed | 0.30 + jacket = 0.32 m | 0.32 m | ok |
| Clavicle | 0.11–0.12 m | no bone before; now 0.120 m | fixed |
| Chest breadth / depth, clothed | 0.23 / 0.16 m | 0.24 / 0.158 m | ok |
| Mid-thigh, knee, calf, ankle girth (leggings) | 0.36 / 0.27 / 0.26 / 0.18 m | 0.37 / 0.27 / 0.27 / 0.17 m | ok |
| Upper arm, forearm girth (jacket sleeve) | 0.25 / 0.21 m | 0.26 / 0.22 m | ok |
| Neck girth | 0.27 m | 0.26 m | ok |
| Wheel diameter × width, wheelbase, bar height, bar width | 145 × 30 mm, 0.55–0.58 m, 0.75–0.80 m, 0.34 m | 145 × 30 mm, 0.56 m, 0.78 m, 0.344 m | ok |

### Defects found

- Rig: no shoulder girdle and no pelvic girdle; arms hung from `torso` and thighs from `pelvis` at a bare `z` offset. Segment lengths, joint order and left/right symmetry of the limbs were right (table).
- Structure 1:50 / 1:100: the skeleton was 10 / 7 loose groups: clavicles ended 15 mm from the sternum (no manubrium), scapulae floated, mandible 4.9 mm below the skull, ilium, pubis and ischium did not meet each other, the sacrum or the femoral head, patella 5.8 mm off the femur, toes 2.3 mm off the metatarsals, finger bones 5 mm off the bar with no grip in the structure.
- Structure 1:200 / 1:500: the 1:50 parts list with thickened blocks (76 mm spine block, 22 mm joint balls, patella); at 1:200 the upper body was not connected to the legs.
- Body 1:100 to 1:500: wheels, fork and axles were the 1:50 parts (9 mm spokes, 8 mm fork blades, 16 mm axles, rims, hubs) at every scale.
- Body 1:50: judged from six directions in `gliding`, `pushing-off`, the braking instant and one kick instant; proportions and girths agree with the table, no body solid was rebuilt.

### Rebuilt

- **Rig** (`rework-child-on-kick-scooter-build.py`, `rig`): four new bones on lateral rest placement. `clavicle-l/-r` (0.120 m) from the sternoclavicular joint [0.048, 1.038, ±0.014] to the acromioclavicular joint [-0.004, 1.044, ±0.122], revolute -8..30; `ilium-l/-r` (0.072 m) from the sacroiliac joint [-0.024, 0.716, ±0.030] to the hip joint [0, 0.658, ±0.065], revolute -4..4. `arm-l/-r` and `thigh-l/-r` are re-parented to their tips without moving (the script refuses a drift above 2e-5 of any old bone frame; frames checked with `rig-rest.ts`). Poses, clips, 2D shapes, anchors, masses and thermal regions keep their meaning unchanged; no pose uses the new bones yet.
- **Structure 1:50 / 1:100**: clavicle and scapula re-expressed on the clavicle bones; ilium, pubis, ischium and ischiopubic ramus on the hip bones. New: manubrium; scapular spine, acromion and glenoid per side; mandibular condyles; arcuate line (sacroiliac joint to acetabulum, the real load path) and acetabulum per side; metatarsal heads on the tarsus; patella set into the femoral groove; grips as frame members at 1:50; fibula `slender` at 1:100.
- **Structure 1:200**: one clavicle per clavicle bone, manubrium with the sternum drawn up to it, arcuate line and acetabulum, long bones at true section with `slender: true`, patella left out, joint balls and supports at the feature.
- **Structure 1:500**: one spine rod at true section from sacrum to atlas (`members: [neck]`), one bar across both clavicles (`members`), pelvis mass spanning both hip bones, long bones at true section `slender`, joints and patella left out, supports at the feature.
- **Body / thermal 1:100**: the six spoke arms become one 15 mm web disc. **1:200**: tyre on one centre disc, one fork solid over both blades; axle, hub and spokes merged. **1:500**: one disc per wheel (72 mm, the width the drawings give it), one fork; rim, hub, spokes, axles and fork crown merged. Thermal and structure entries of merged parts were removed with them.
- **2D**: not redrawn. The outer form did not change at 1:50, and `volumes-silhouette` and `volumes-views-disagree` pass at all four scales with the merged wheels.
- **Decisions**: structure texts of all scales, body texts of 1:100 to 1:500, wheel texts and the counts (en, de).

### Rendered and read

- Before the edits: body 1:50 `gliding` and `pushing-off`; structure 1:50; body and structure 1:500.
- After the edits: structure 1:50 (six views, three-quarter and front close-up); structure 1:50 `pushing-off`; structure 1:500 and 1:200; body 1:500; body 1:50 front close-up; body 1:50 `glide-and-brake` 2.2 s (the braking pose) and `kick-and-glide` 0.5 s; thermal 1:50; 2D 1:500 and 1:100.
- Rendered but not read: body 1:100 and 1:200; structure 1:100; thermal 1:100 cut, 1:200 and 1:500; structure of `kick-and-glide` 1.4 s, `glide-and-brake` 3.5 s and `braking`; 2D 1:200. Never rendered: 2D 1:50 after the edits, the remaining 43 key poses of `kick-and-glide` one by one.

### What remains imperfect

- Hands: one bone per hand, fingers modelled as solids closed round the grip (both sides alike); no finger bones.
- Scooter rig: no steering pivot (stem, bar, fork and front wheel are solids of `frame`) and no brake-spoon pivot.
- No pose or clip uses the new girdle bones (no shrug, no pelvic drop on the pushing leg).
- Forearm 6 % and hand 5 % shorter than the reference; left as authored because poses and clips are solved for these lengths.
- The 1:500 wheel is 72 mm wide against 30 mm in reality, because `volumes-feature` holds it to the thinnest drawing of its bone.
- The 1:50 shoulder cap is slightly angular in the front view; not re-lofted.
- Body, thermal and 2D of 1:100 and 1:200 are validated but not all read as images (list above).

### Files

- `rework-child-on-kick-scooter-build.py` (rig, re-binding, coarse lists) and `rework-child-on-kick-scooter-fix.py` (joint anatomy, decisions). They build from the pre-edit copies in `🗑️generated/rework-child-on-kick-scooter/orig/`, kept there for the next pass; the scripts must not be run on the published state.
- `volumes/humans/child-on-kick-scooter.json` and the catalog file `👤️humans/child-on-kick-scooter/🔣️.json` (`rig`, `volumes`).
- Not deleted: nine `🗑️generated/r3-sheet/file/cand.*.png`; seven are mine, but the folder name `cand` may be shared with another author, so I left them all.

### Fundamental issues

- `volumes-feature` forces a wheel solid at 1:500 up to the pen width of its bone's drawing (72 mm for a 30 mm wheel), so the 3D model is typified to the 2D pen.
- `r3-sheet.ts --file` names its output after the folder of the candidate only (`file/cand.*`), so two authors with a folder `cand` overwrite each other's renders.
- The semio MCP server did not connect in this session; the work ran with the native tools of the brief only.
