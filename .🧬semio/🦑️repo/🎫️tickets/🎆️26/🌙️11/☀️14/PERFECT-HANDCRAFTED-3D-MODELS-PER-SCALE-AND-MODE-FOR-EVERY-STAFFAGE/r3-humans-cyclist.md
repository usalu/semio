# Round 3 — humans/cyclist

Asset: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/cyclist/🔣️.json`; volumes source `volumes/humans/cyclist.json`. Fresh start (no earlier round-3 scripts, report or edits of this asset existed). State: catalog written and spliced, `r3-validate.ts humans/cyclist` and `--catalog-only` VALID (schema 0, validateAsset 0), print-suitable at all four scales, `volumes-check.ts --product` green on every rule except the helmet's body-share area (C18, below).

## 1. Reference (real subject)

| quantity | value | source | asset before | asset now |
|---|---|---|---|---|
| rider | woman, H = 1.72 m, 66 kg + 2.4 kg clothing | asset provenance (de Leva 1996 female) | — | kept |
| hip joint / knee / ankle | 0.52 H = 0.894, 0.285 H = 0.490, 0.039 H = 0.067 m | brief shares, Drillis and Contini 1964 | thigh 0.420, shank 0.411 | thigh 0.4042, shank 0.4231 |
| shoulder / elbow / wrist | 0.818 H, 0.63 H, 0.485 H: upper arm 0.32, forearm 0.25, hand 0.186 m | brief, de Leva | 0.319 / 0.251 / 0.184 | kept |
| hip-joint spacing | 0.17–0.18 m (bitrochanteric 0.33 m minus 2 x 0.075) | Snyder/de Leva female breadths | ±0.095 | ±0.088 |
| atlanto-occipital joint, vertex | AO about 0.113 m above C7, vertex 0.155–0.158 m above AO | head height H/7.5 = 0.229, tragion 0.93 H | neck 0.0966, head 0.174 | neck 0.112, head 0.158 |
| foot in a sneaker (EU 39) | ankle 0.092 m above ground in the shoe, ball joint 0.035 m, 0.125 m ahead: ankle-to-ball 0.137 m, toe 0.066 m, shoe 0.27 m long | foot length 0.14 H, ball at 0.72 of foot length | foot bone 0.1795 (+31 %) | 0.1374, rest angle 65.5 deg |
| girths (with clothing) | thigh 0.58 → 0.52 m, knee 0.33, calf 0.36, ankle 0.22, upper arm 0.31, forearm 0.27, neck 0.33; chest breadth 0.30, waist 0.26, hips 0.36 m | ANSUR II female 50th percentile plus 4–8 mm garments | box torso, tube limbs | sections stated by hand |
| bicycle | 700 x 40C (ETRTO 40-622): outer r 0.34, section 40 x 38 mm, rim 622 box 24 mm; wheelbase 1.05, chainstay 0.45, BB height 0.30 (rig, hybrid/city class) | ETRTO 2023, EN ISO 4210, Wilson and Papadopoulos 2004 | kept | kept |
| head angle / fork | 70–72 deg, offset 40–50 mm, axle-to-crown 0.39–0.40 m | city/trekking geometry tables | fork 82 deg, head tube 0.12 m ahead of a real one | 71 deg, 45 mm, 0.392 m |
| frame | women's city bicycles are low-step ("wave") frames: one 45 x 40 mm oval down tube from the head tube sweeping low (step-over about 0.42 m) into the seat tube above the BB; seat tube 73 deg, 0.50 m | Gazelle Esprit, Kalkhoff Image, Batavus Fonk geometry (50 cm) | diamond frame with 0.87 m top tube | wave frame |
| controls | swept-back city bar 600–640 mm, grips 115 mm x 33 mm at about 0.07 m above the saddle, grip centres ±0.25 m | city bar makers (Humpert, Ergotec) | grips at ±0.307, 0.05 m below and 0.12 m outboard of the hands | grips at (0.80, 0.99, ±0.25), hands on them |
| pedals | Q-factor: crank faces ±0.085, platform pedals 0.08 wide, pedal centre ±0.135 m | ISO 4210-8 crank clearance, city cranks | feet at ±0.095, pedals at ±0.1425 (feet beside the pedals, inside the crank arms) | feet at ±0.132 on the pedals |
| equipment | fenders 50–56 mm, 20 mm clearance; rear carrier 0.78–0.80 m high; chain guard; side stand; hub-dynamo lamp; sprung comfort saddle 265 x 204 mm | product knowledge | fenders as slabs, no carrier, guard, stand | modelled |
| overall | length 1.05 + 2 x fender radius 0.367 = 1.785 m; width 0.63 m (bar ends); height 1.79 m (helmet top) | from the above | 1.74 x 1.79 x 0.65 | 1.785 x 1.79 x 0.63 |

## 2. Audit (catalog before this round)

- Rig: thigh 4 % long and shank 3 % short against the shares; foot bone 31 % long (ankle to ball 0.18 m); hips 7 mm too wide; neck–head joint 17 mm low. Bike: diamond frame with an 0.87 m top tube — in the pose `dismounted-astride` the rider's crotch (0.79 m) went through it; fork nearly vertical (82 deg) and the head tube 0.12 m too far ahead.
- Poses and clips: hands 0.117 m inboard and 0.05 m above the grips (they gripped air); upper arms hanging vertical with forearms horizontal (a tray-carrying posture, not "straight arms on the bars"); feet 0.05 m inboard of the pedals, inside the crank arms; the top-of-stroke ankle beyond its joint limit.
- Anchors: grips not on the bar, mouth/nose/crown not on the face and helmet (crown 40 mm above the helmet).
- `dimensions` 1.74 x 0.65: the fenders were not counted, the bar is 0.63.
- 2D: mixed front/plan shapes bound to `pelvis` (legs did not follow the clips), stadium limbs, the diamond frame.
- Solids (qa-visual-3 grades B/B-): box torso, tube limbs, ball shoulder caps, capsule tyres 18 mm thick (tyre area finding), flat fenders, no carrier, thermal one grey region from 1:100 on, structure a body copy at coarse scales.

## 3. Root fixes (old → new)

- `rig.bones`: thigh 0.42021 → 0.4042 (comX 0.1623 → 0.1561), shank x/length 0.4202/0.41055 → 0.4042/0.4231 (comX 0.1729 → 0.1782), foot x/length/angle 0.4106/0.1794828/66.2 → 0.4231/0.1374/65.487 (com 0.0764/-0.0033 → 0.0591/-0.012), toe x/angle/length 0.1795/23.8/0.06279 → 0.1374/24.513/0.066, thigh z ±0.095 → ±0.088, neck 0.0966 → 0.112, head x/length 0.0966/0.17388 → 0.112/0.158 (com 0.075/0 → 0.052/-0.012), frame mass centre (0.5, 0.65) → (0.51, 0.59) (hand sum of frame, fork, bar, fenders, carrier, saddle, guard, lamps, stand, brakes).
- `poses` (4) and `clips` (160 + 17 keys) re-solved by inverse kinematics (`r3-cyclist-root.py`): torso +4 deg while riding (trunk lean 21 deg); arms reach the grips with abduction 5–12 deg and 38–60 deg elbow flexion; legs reach the ball-over-spindle target with abduction 2–8 deg and a thigh twist that keeps the knee over the hip–ankle line; where the ankle would leave its range the foot stops at the limit and the free pedal turns to the sole; `dismounted-astride` feet re-planted on the ground. Residual of every key below 1e-7 m.
- `anchors`: heel/ball on the new sole, grips (0.084, -0.024) on the bar axis, saddle (0.25, 0.92), mouth (-0.024, -0.113), nose (0, -0.117), crown (0.181, 0.037) on the helmet top.
- `dimensions`: widthM 1.74 → 1.785, depthM 0.65 → 0.63 (heightM 1.79 and mass kept).
- `thermal.regions[].areaM2` (C16, within factor 1.5, from the 1:50 model and a hand calculation): `bike-metal` 1.22 → 1.62 (frame tubes 0.33, fork/stays 0.17, rims 0.30, 72 spokes 0.13, fenders both faces 0.29, carrier 0.10, hubs, chainring, guard, cranks, pedals, grips, lamps, stand 0.30), `tyres` 0.6 → 0.49 (2 x 2π x 0.321 m x 0.122 m section perimeter of a 40-622 tyre), `saddle` 0.07 → 0.086 (top 0.042, underside 0.036, rim 0.008 m²).
- `representations` (ruling E1): every shape of every scale and view redrawn from that scale's body (`r3-cyclist-draw2d.py` + `-project2d.ts`); one shape per drawn part with the same id in all three views; limbs as swept solids, crank/pedal/wheel parts as extruded side profiles, frame and trunk as plain outlines, slender parts as strokes; paint order by default-pose depth; hatches `softshell` and `far-limb` kept; decision texts rewritten (`r3-cyclist-texts.py`). Shape counts elevation/front/plan: 164/84/91, 83/60/60, 55/43/43, 25/25/25. Removed with the parts they drew: `tube-top`, the pelvis-bound front/plan leg shapes, stadium arms, the 1:500 toe shapes.

## 4. Models (body / thermal / structure solids)

- **1:50 — 135 / 201 / 96.** Rider: trousers seat, jacket with waist, bust, shoulder slope and a hem flaring over the seated hips, collar, zip, reflective band; sleeves with deltoid and elbow; gloves closed round the swept grips (palm, four fingers wrapped about the true bar axis, thumb under the bar); thighs with quadriceps mass, knees, calves, trouser clips; sneakers with soles and toe boxes; neck, head with jaw, nose, lips, eyes, ears; vented helmet (open shell) with straps; ponytail. Bicycle: wave down tube, seat tube with collar, post, head tube, headset, steerer, stem, clamp, swept bar, grips, levers, bell, crown and curved fork blades, dropouts, chainstays, seatstays with bridge, revolved sheet fenders with stays, carrier, tail lamp, chain guard, chainring, cog, chain, cranks, platform pedals, sprung saddle on rails, head lamp, side stand, V-brakes, tyres, box rims, 36 crossed spokes, gear and dynamo hubs. Thermal: nested shells for jacket (4), trousers (3), gloves (3), shoes (3), helmet (4), hair; face skin an open skin below the helmet rim; outlets at nostrils, lips and helmet crown. Structure: skull, mandible, cervical spine, thoracolumbar spine with lordosis and kyphosis, ribcage, clavicles, scapulae, pelvis, femur with head and neck, patella, tibia, fibula, tarsus, toes, humerus, ulna, radius, metacarpus; 24 joints (rig joints of the rider, steering head, bottom bracket, hubs, pedal spindles); masses of frame, pelvis, torso, head, thighs; bicycle members by reference.
- **1:100 — 73 / 76 / 90.** Fewer sections, finger roll, merged shoes, plain tubes and hubs, carrier side frames with one bar loop, twelve spokes; thermal one surface per part; skeleton with shafts ≥ 15 mm.
- **1:200 — 46 / 49 / 64.** Fists, merged stays and fork plates, carrier box on two legs, tyre-and-rim rings with six spokes; skeleton one bone per segment ≥ 30 mm.
- **1:500 — 25 / 30 / 55.** Body masses at the 75 mm floor, head and helmet one form; the bicycle as slender members of true diameter (tyres, fenders, fork-and-stem, bar, wave-and-seat tube, stays) and a saddle block; thermal adds own face and hair solids; stick skeleton, wheel spokes and crank sticks as members.
- `decisions`: three bilingual sentences per scale (body, thermal, structure).

Final check line (`volumes-check.ts humans/cyclist --product`): `FAIL: 2 measurement(s) over 2 rule line(s)` — only `areaDeviation:helmet` 0.88 (1:50) and 0.43 (1:100); every other rule ok. Silhouette IoU elevation/front/plan: 1:50 0.977/0.961/0.946, 1:100 0.970/0.962/0.942, 1:200 0.971/0.965/0.951, 1:500 0.934/0.931/0.883. Body areas at 1:50: face-skin +17 %, trousers +17 %, shoes +25 %, gloves +5 %, jacket +1 %, bike-metal +1 %, tyres +2 %, saddle 0 %. `r3-validate.ts` (with source and `--catalog-only`): VALID. `r3-cyclist-suit.ts`: all four scales authored (print-suitable).

## 5. What I saw (renders opened with the image viewer)

- 1:50 body sheet (front, right, back, left, top, three-quarter) and full-size right, front, top and three-quarter views: reads at once as a woman on a low-step city bicycle. Right side: wave tube low between the wheels, curved fork ahead of the head tube, head lamp on the crown, carrier over the rear fender, chain guard, hands closed on the grips, right foot on the pedal at 4 o'clock, left knee up, ponytail under the helmet. Front: shoulders now continue into the sleeves without caps (first render showed epaulette caps, fixed), hands on the bar ends, feet on the pedals outside the cranks. Top: swept bar, arms reaching out to the grips, saddle under the seat. Back: buttocks on the saddle, jacket hem over them.
- Poses `dismounted-astride` (stands astride the low frame, hands on the grips, feet planted) and `standing-on-pedals` (out of the saddle, cranks level) and clip `pedal-cycle` at 0.4 s (hands and feet stay on grips and pedals): no joint opens.
- 1:100, 1:200, 1:500 body sheets: same figure, simpler; 1:500 a clean stick bicycle with a massed rider.
- 1:50 thermal (cut): regions and nested shells in trunk, limbs, shoes, helmet; the face skin as an open skin. 1:50 structure: a skeleton inside the ghost, frame tubes, wheels, joints, masses.
- 2D (`r3-2d.ts`): 1:50 (default and `dismounted-astride`), 1:100, 1:200, 1:500 — elevation, front and plan agree with the model, the swept legs follow the dismount.

## 6. Still imperfect

- Grade as I see it: 1:50 body A-, 1:100 A-, 1:200 B+, 1:500 B+; thermal A-; structure A- (1:50, 1:100), B+ (coarser).
- `helmet` area fails C18 (body share 0.07 against a real 0.13 m² helmet); filed in `author-requests.md`.
- Seams: where the flared jacket hem crosses the thighs and at the knees the crossing lofts leave a slightly ragged seam line; the face is plain (dot eyes, wedge nose); no cloth folds; the ribcage is one barrel; the pelvis bone a basin.
- Not modelled: cables, valves, tread, reflectors, lock. Spokes are texture solids.
- The thigh shapes of the plan carry the swept outline; in strongly bent poses their straight ends show.
- `r3-cyclist-root.py` re-solves from `🗑️generated/r3-cyclist/asset.before.json` (kept on purpose); deleting it makes the roots non-reproducible; `asset.applied.json` beside it is the snapshot `r3-cyclist-apply.ts` uses to refuse writing over someone else's later edit. All other generated output of this asset was deleted.

## 7. Files

`r3-cyclist-ref.py` (reference numbers), `-fk.py`, `-root.py` (rig, anchors, IK of poses and clips), `-kit.py` (solid writers), `-body50.py`, `-coarse.py` (1:100, 1:200, 1:500), `-volumes.py` (thermal, structure, decisions), `-build.py`, `-project2d.ts`, `-draw2d.py`, `-texts.py`, `-sheet.ts` (sheet of the candidate), `-slender.ts`, `-suit.ts` (print suitability), `-apply.ts`, `-ship.sh` (rebuild, validate, write catalog, splice, validate). Appended: `author-requests.md`.

## Round 5 handcraft (2026-11-21)

State: catalog written (`r5-cyclist-roots.ts --write`) and volumes spliced. `r3-validate.ts humans/cyclist --catalog-only`: `VALID ... schema 0, validateAsset 0` (before this round the catalog was INVALID: four `PRINT range-end-feature` findings). `volumes-check.ts humans/cyclist --product`: every rule ok except `areaDeviation:helmet` 0.87 (1:50) and 0.43 (1:100), the known C18/C27 body-share finding (`author-requests.md`, `area-review.md`). Silhouette IoU elevation / front / plan: 1:50 0.968 / 0.960 / 0.939, 1:100 0.962 / 0.959 / 0.935, 1:200 0.964 / 0.966 / 0.948, 1:500 0.927 / 0.894 / 0.912.

Scripts: `r5-cyclist-remodel.py` (jacket rows of every scale through `r4-g11-kit.edit`, thermal shells re-drawn), `r5-cyclist-roots.ts` (2D print root fixes), `r5-cyclist-ranges.ts` (print violations at preset and range ends); `r5-human-measure.ts`.

### Reference and model (rig of round 3, bone lengths; woman, H = 1.72 m)

| quantity | reference | model | deviation |
|---|---|---|---|
| shank (knee to ankle) | 0.246 H = 0.423 | 0.4231 | 0.0 % |
| thigh (hip to knee) | 0.235 to 0.245 H = 0.404 to 0.421 (hip 0.52 to 0.53 H) | 0.4042 | in range |
| upper arm | 0.186 H = 0.320 | 0.3188 | -0.4 % |
| forearm | 0.146 H = 0.251 | 0.2512 | 0.0 % |
| hand | 0.108 H = 0.186 | 0.1835 | -1.3 % |
| head chin to vertex | H / 7.5 = 0.229 | 0.226 (lips at 1.543 to 1.561, vertex 1.754 in the rest frame) | -1.3 % |
| ankle in the shoe | 0.039 H + 25 mm = 0.092 | 0.092 | 0.0 % |
| shoe length EU 39 | 0.27 | 0.274 | +1.5 % |
| wheel 40-622 outer radius, wheelbase | 0.34, 1.05 | 0.34, 1.05 | 0 % |

The rig's rest pose is not upright (trunk and neck carry the riding lean), so stature and breadths are taken from the bone chain, not from the rest-pose mesh (`r5-human-measure.ts` output kept only as a cross-check).

### Grades before -> after

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | B+ -> A- | A- -> A- | A- -> A- |
| 1:100 | B+ -> A- | A- -> A- | A- -> A- |
| 1:200 | B+ -> B+ | B+ -> B+ | B+ -> B+ |
| 1:500 | B -> B | B+ -> B+ | B+ -> B+ |

Defects seen before: the softshell jacket pinched into the lumbar hollow (back 0.048 m behind the trunk axis at L3) and flared out over the seat, so the hem read as a peplum from the side and the three-quarter views; the catalog failed the print range ends (1:100 front `carrier` slivers at 1:75, 1:500 tyre bands at 1:1000).

### Changes

- Jacket at every scale: back 0.062 to 0.074 m behind the axis between L5 and T12 (a loose softshell bridges the lordosis); hem rows kept because they cover the seated buttocks (0.117 m behind the hip joint); nested thermal shells re-drawn.
- 2D roots: 1:50 front carrier stays 10.5 mm and platform 12 mm (real tubes 10 mm); 1:100 front drops the `carrier` detail block that only showed 14.5 mm slivers between the rack bars behind the rider; 1:500 front and plan tyre bands drawn 72 mm (the scale's feature) instead of 40 mm.

### Looked at (opened)

1:50 body sheet, right side large, torso crop, three-quarter from the front right, pose `dismounted-astride`, thermal cut, structure; 1:200 and 1:500 body sheets; `r3-2d` 1:500. Rendered, not opened: clip `pedal-cycle` at 0.4 s.

### Open points

- `helmet` area 0.07 m² declared against 0.13 m² modelled (C27, reviewer's decision pending).
- 1:500 bicycle is drawn and modelled as true-diameter slender tubes (the 2D draws strokes); the fenders stand 16 mm off the tyres as separate rings, below the 75 mm feature.
- No cloth folds; face plain; 1:200 / 1:500 not remodelled beyond the jacket.

The rear carrier is now a welded tubular rack at all four scales. The pair of 330 mm deck rails is 136 mm apart at 735 mm above the ground. The steelwork provides at least 23 mm vertical clearance over the rear mudguard envelope. The legs reach the dropout mounting points, two stays connect to the frame, and each crossbar connects both rails. The models retain five, five, four and three crossbars at the four scales. Their thermal and load-bearing references follow the actual stock. The saddle and rear carrier remain distinct in the 1:350 drawing. All twelve outlines, the four scale, thermal and structural checks and the full paper document passed. Independent GLB readback measured 6,025,592 bytes across twelve scenes.
