# `humans/skateboarder` — running report

## Pass 1 (second round), 2026-11-22

### State found

- No report existed. `rework/02.md` never reached this asset. Six scripts of an earlier agent were in `T` (`rework-02-skateboarder-{pass1,pass2,rig,skeleton,coarse,draw}.py`, last written 13:58); it was cut off before it wrote a report.
- Source `T/volumes/humans/skateboarder.json` and catalog `asset.volumes` were identical (all four scales), `r3-validate.ts humans/skateboarder --volumes` ended `VALID` (schema 0, validateAsset 0, checkVolumes 0) before I changed anything.
- `T/rig-lateral-rest.md` exists (read at the start and before finishing). The girdles were already on the lateral rest placement: `clavicle-l/-r` under `torso` from the sternoclavicular joint [0.016, 1.192, ±0.022] to the acromioclavicular joint [-0.012, 1.245, ±0.176] with `abductionDeg ±68.7`, `twistDeg ±116.2`; `ilium-l/-r` under `pelvis` from the sacroiliac joint [-0.058, 0.760, ±0.040] to the hip joint [0, 0.685, ±0.087]; `upper-arm-*` hang from the clavicles, `thigh-*` from the hip bones; the former sagittal stub survives only as the fixed `hood-yoke` frame of hood and collar. I checked this in the rig and in the structure sheets (front, back, top: the bones run joint to joint) and in the poses below; I did not redo it.

### Reference (young man, 1.78 m in skate shoes, lean, about 68 kg; Drillis and Contini ratios, DIN 33402-2 P50 male, street deck 8.0 in)

| Measure | Reality | Model (rig / solids / sheet) | Verdict |
|---|---|---|---|
| Thigh, hip joint to knee | 0.245 H = 0.43 m | 0.418 m | ok |
| Shank, knee to ankle | 0.246 H = 0.43 m | 0.438 m | ok |
| Upper arm, shoulder to elbow | 0.186 H = 0.33 m | 0.335 m | ok |
| Forearm, elbow to wrist | 0.146 H = 0.26 m | 0.258 m | ok |
| Hand | 0.108 H = 0.19 m | 0.19 m, palm, four fingers and thumb on both sides alike | ok |
| Foot / shoe length | 0.152 H = 0.27 m, shoe 0.29 m | foot 0.154 m + forefoot 0.075 m, shoe about 0.29 m | ok |
| Head height / breadth | 0.235 / 0.155 m (1 / 7.6 of stature) | 0.235 / 0.157 m under a beanie | ok |
| Biacromial breadth | 0.39 to 0.40 m | 0.352 m (acromioclavicular joints at ±0.176) | 40 mm narrow, open |
| Bideltoid breadth in a hoodie | 0.47 m + cloth = 0.49 to 0.50 m | 0.49 m | ok |
| Distance of the hip joints | 0.17 to 0.18 m | 0.174 m | ok |
| Fingertip height, arm hanging | mid thigh | mid thigh | ok |
| Deck | 0.81 x 0.203 m, 11.5 mm, kicks about 20 degrees | 0.81 x 0.203 m, 11.5 mm halves, both kicks | ok |
| Wheels, track | 54 mm, 171 mm between wheel centres | 54 mm, 171 mm | ok |
| Wheelbase | 0.36 to 0.37 m (14.25 in) | 0.41 m | 40 mm long, open |

### What I looked at

Read as images at full sheet size before the changes: 1:50 body, thermal, structure (default pose); 1:50 body in `pushing`, `crouch`, `ollie-air`; 1:500 body and structure; 1:200 body and structure; 1:100 structure; 2D at 1:50 and 1:500. Rendered but not read before the changes: the other six poses, the six clip instants, the remaining coarse lists.

### Defects found and what I did

| Scale, list | Defect | Change |
|---|---|---|
| 1:500 structure | ribcage = two loose hoops and a 6 mm sternum wire floating in front of the spine | one thoracic cage (eight hand-set sections, 0.27 m wide, 0.15 m deep, apex at the first rib) around the spine; hoops and wire removed |
| 1:200 structure | three loose hoops that read as a bow tie | five true rib pairs (1, 3, 5, 7, 9) on the paths of the 1:100 skeleton at their 9 mm section (`slender`), sternum at the 30 mm feature; ids now `costal-N-l/-r`, `sternum` as at the finer scales |
| 1:100, 1:200, 1:500 body (thermal follows through `of`) | limb segments ended in flat cuts inside elbow, knee, shoulder and hip | rounded ends inside the segment's own length on sleeve, forearm (elbow end), thigh, shank (knee end); hems and cuffs stay open cuts |
| 1:500 body | thigh ran 74 mm past the knee and stood out behind the bent knee as a block | thigh ends 50 mm past the knee centre |
| 1:50 and 1:100 body, 1:50 thermal shells | sleeve heads stood 60 mm above the shoulder joint as separate lobes with a notch towards the neck | trunk carries the shoulder seam out over the acromion (half breadth 0.19 m at seam height), sleeve head flattened to 50 mm; thermal upper-arm shells moved with the same factor |
| 1:200 and 1:500 decisions | text described open ribs and, at 1:500, a deck with trucks and wheels that the list does not hold | rewritten (en, de) to what is modelled and why |

Scripts (hand-chosen numbers, edit the source in place): `rework-skateboarder-r2-coarse.py`, `rework-skateboarder-r2-shoulder.py` (run once each, in this order; the second is not idempotent).

### Validation and publishing

- `r3-validate.ts humans/skateboarder --volumes` after the coarse stage: `VALID` (schema 0, validateAsset 0, checkVolumes 0); published with `volumes-insert.py`.
- After the shoulder stage: `VALID` again with `--volumes`; published; `r3-validate.ts humans/skateboarder --catalog-only --volumes`: `VALID humans/skateboarder (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Rig, poses, clips, 2D, anchors, `thermal`, `structure` of the catalog file were not written by me.

### Rendered and read after the last change

All nine other poses, two instants of each of the three clips and all twelve scale and list sheets were rendered again after the last change without a `NOT DRAWN` line. Read as images after the last change: 1:50 body `pop`, 1:50 body `push-cycle` at 1.3 s, 1:100 structure `crouch`, 1:500 thermal. Read after the coarse stage and before the shoulder stage: 1:500 body (sheet and left side alone), 1:500 structure, 1:200 structure, 1:100 body, 1:50 body right side, 1:50 structure three-quarter. Not read after the last change: `pushing`, `crouch`, `ollie-air`, `push-plant`, `push-lift`, `push-swing`, `push-reach`, `landing` at 1:50, five of the six clip instants, 1:50 thermal, 1:100 and 1:200 thermal, 2D at any scale. The poses I read before the change (`pushing`, `crouch`, `ollie-air`) were natural; the shoulder change moves the sleeve head by 10 mm, so I expect them unchanged but did not look.

### What remains imperfect

1. Rig: biacromial breadth 0.352 m, about 40 mm narrow for a 1.78 m man. Widening moves the arms; front and plan 2D shapes and `sweep: "w"` spans then have to be shifted by the same vector (`rig-lateral-rest.md` 1.6 item 3).
2. Rig: `board` starts at the middle of the deck and is 0.81 m long, so the bone ends 0.4 m ahead of the nose. It should start at the tail or be 0.405 m; every child, solid and 2D shape of the board is written against the present origin.
3. Board, all scales: wheelbase 0.41 m against 0.36 to 0.37 m of a street deck (trucks at ±0.205 m).
4. 2D 1:50 side elevation: the hoodie is drawn with the old lobed sleeve heads and a square neck opening and without hood or pocket; the 3D shoulder line changed, the drawing was not redrawn. 2D of 1:100 and 1:200 not looked at in this pass.
5. 1:50 body: the seat of the trousers and the two thighs are three solids whose seams read as briefs in the frontal view; the pelvis of the 1:50 skeleton has paddle-shaped iliac blades and the lumbar column is one rod without vertebrae.
6. 1:500 thermal sheet: every garment region is drawn grey while the legend lists their colours (regions given as `of` references); the wheel boxes stand out below the board form and the truck boxes are hidden inside it. Whether the grey is a defect of `r3-sheet.ts` or of the list was not investigated.
7. 1:500 body and structure: the board is one 90 mm high form; a true 11.5 mm deck was not attempted because the print rule asks for 0.2 mm on paper.
8. The scripts of the earlier agent (`rework-02-skateboarder-*.py`) do not know the changes of this pass; running them again would undo them.
