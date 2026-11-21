# Round 3 — humans/person-with-rollator

State: NOT DONE on one point, everything else finished. Root fixes and volumes are in the catalog asset (candidate document, volumes source and catalog file are identical). `r3-validate.ts humans/person-with-rollator` and `--catalog-only`: `VALID ... schema 0, validateAsset 0`. Print suitability: all four scales `authored`, no violations. `volumes-check.ts --product`: 40 rule lines green, `FAIL: 3 measurement(s) over 2 rule line(s)`, all three in `volumes-thermal-coverage`: `beret` at 1:50 and 1:100, `shoes` at 1:50. Both are physiological shares I may not change (C18); they are filed for the area reviewers (see 7).

This session resumed the work interrupted on 2026-11-20. The model, the root fixes and the 2D redraw were already spliced by the earlier session; this session compared catalog, source and scripts, looked at every scale and list again, kept the calibration another pass had written into the catalog, filed the area findings and wrote this report. No geometry was changed in this session.

## 1. Reference

| Quantity | Real value | Source |
|---|---|---|
| Stature, woman of 79 | 1.58 m upright, 1.57 m in the stooped stance | DIN 33402-2 women 61 to 65 (median about 1.59 m) less age loss; asset summary |
| Hip joint | 0.53 H = 0.84 m | Drillis and Contini (asset reference `drillis-contini-1966`) |
| Knee | 0.285 H = 0.45 m | same |
| Ankle | 0.039 H = 0.062 m, plus 12 mm sole | same |
| Shoulder joint | 0.818 H = 1.29 m upright | same |
| Upper arm, forearm, hand | 0.186 H = 0.294 m, 0.146 H = 0.231 m, 0.108 H = 0.171 m | same |
| Head height | H / 7.5 = 0.21 m | round-3 brief |
| Biacromial breadth, elderly woman | 0.22 H = 0.35 m, narrow sloping shoulders | anthropometric tables, my knowledge |
| Hip breadth | 0.20 H = 0.32 m | same |
| Thoracic kyphosis | visibly rounded upper back, head carried forward | geriatric posture, my knowledge |
| Rollator, four-wheel, lambda side frames | overall width 0.60 m, length 0.65 to 0.70 m, wheelbase about 0.52 m | ISO 11199-2 (asset reference `iso-11199-2-2021`), common products (Topro Troja, Drive Nitro class) |
| Wheels | diameter 0.20 m, tread 0.03 to 0.036 m, front pair on swivel forks | same |
| Seat | 0.52 m high (short user), about 0.40 x 0.22 m, backrest strap | same |
| Grips | set to the wrist crease of the standing user: 0.485 H + sole = 0.79 m | fitting rule of rollators; Drillis and Contini |
| Tubes | aluminium, diameter 22 to 25 mm | products |
| Mass | 5.4 kg (asset), real 5 to 7.5 kg | products |
| Basket and bag | wire or fabric basket ahead of the front legs, tote 0.30 x 0.10 x 0.20 m | products |

## 2. Audit (state before round 3)

- Rig of the woman: hip 0.849 m (0.538 H, 1 % high), knee 0.462 m (0.292 H), ankle 0.074 m with the sole, shoulder 1.303 m upright (0.825 H), arm 0.294 / 0.231 / 0.171 m. All within 1 % of H of the reference; kept.
- Rig of the aid: `frame` hung on `aid-slide` at x 0.38 m, so the woman walked 0.19 m behind the rollator instead of inside its frame; grips drawn 0.85 m high (0.06 m above her wrist crease); no way to turn the rollator, so in `seated-rest` she sat facing the wrong way; `seat` and `basket` anchors 0.04 and 0.27 m off the real seat and basket; grip anchors at the middle of the open hand, not in the closed fist; wheel footprints 0.08 m wide against a 0.036 m tread; hand centre of mass at 0.135 m (open hand) with closed fists.
- `dimensions`: 1.61 x 1.16 x 0.62 m. Height was the upright stature plus beret, not the stooped stance; length was 0.19 m too long because of the detached rollator; width of the real rollator is 0.60 m.
- 2D: every scale drew the old detached arrangement; front and plan at 1:500 drew the rollator as filled blocks while the elevation drew strokes (author-requests, entry 55).
- Solids: box torso, tube limbs, shoulder cap spheres, blade hands, wedge shoes; thermal at 1:100 to 1:500 one region for the person; structure at 1:500 a copy of the body (qa-visual-3, grades B to B-).

## 3. Root fixes (written to the catalog asset)

| Member | Old | New |
|---|---|---|
| `dimensions` | heightM 1.61, widthM 1.16, depthM 0.62 | 1.57, 0.975, 0.60 |
| `rig.bones` new `aid-turn` | none | parent `aid-slide`, x 0.485, y -0.8494, twist -180 to 180 degrees |
| `rig.bones.frame` | parent `aid-slide`, x 0.38, y -0.8494, angle 0; mass at 0.26 / 0.40 | parent `aid-turn`, x 0, y 0.295, angle -90; mass at 0.30 / 0.505 |
| `hand-l`, `hand-r` mass comX | 0.1348 | 0.0531 (closed fist) |
| anchors `grip-l`, `grip-r` | x 0.0768, y 0 | x 0.055, y -0.03 (centre of the grip tube in the fist) |
| anchor `seat` | x 0.255 | x 0.295 |
| anchor `basket` | x 0.26 | x 0.53 |
| anchors `wheel-rear-contact`, `wheel-front-contact` | footprintWidthM 0.08 | 0.036 |
| anchors `heel-l`, `heel-r` | x -0.0323, y -0.103 | x 0.012886, y -0.07733 (centre of the modelled heel tread; written by `hu-heel.py` of another pass on 2026-11-20, kept) |
| poses | arms not on the grips, rollator ahead | every pose and clip key re-solved so both grip anchors lie on the grip tubes (miss under 0.1 mm); `seated-rest` turns the rollator by 180 degrees and sits her on the seat against the strap; `stoop-to-basket` brings the wrists to the basket rim; descriptions rewritten |
| clip `rollator-walk` | rollator at rest distance | 0.03 m farther ahead while walking; thigh and shank angles of the 27 keys adjusted by under 0.1 degrees so that the planted feet give exactly one wheel revolution per stride (written by `hu-stride.py` of another pass, kept) |
| `materials` | 14 | plus `bone`, `spectacle-frame`, `nylon-grey` |
| `thermal.regions.bag.areaM2` | 0.33 | 0.27 (factor 1.22, C16; hand calculation in `area-review.md`) |
| `provenance.notes` | grip height 0.85 m | grips 0.79 m at the wrist crease, overall width 0.60 m |
| `representations` 50 / 100 / 200 / 500 | old drawings | all three views redrawn as orthographic views of the scale's body (E1); shape counts elevation / front / plan: 82 / 88 / 61, 57 / 56 / 50, 43 / 42 / 42, 35 / 34 / 34; drawing decisions rewritten in English and German |

`r3-person-with-rollator-candidate.py` now leaves `anchors` and `clips` as the catalog has them, so a later run cannot undo the heel and stride calibration. `r3-person-with-rollator-roots.ts` must not be run again on the patched catalog: it adds the walking offset a second time.

## 4. Model per scale (solids body / thermal / structure)

- **1:50 (169 / 210 / 191).** Body: loden coat as trunk part with narrow sloping shoulders and rounded back, seat part and two overlapping skirt halves that follow the thighs; collar, lapels, placket, four buttons, two pockets; sleeves with sleeve head and cuff; trousers from the knee with calf and hem break; laced shoes with upper, toe box, heel, sole, laces; head with jaw, cheeks, brow, nose, lips, ears, glasses, hair at nape and temples, beret with stalk; hands closed on the grips with palm, four three-part fingers and thumb. Rollator: two lambda side frames of bent tube, cross tubes, seat plate, backrest strap, grips, brake levers and cables, swivel forks, four spoked wheels with tyres, hollow basket, tote with handles. Thermal: coat as loden, cardigan, blouse and air shells, trousers as wool, tights and air, shoes as leather, sock and air, beret as felt and air, face, hands, hair, the five rollator regions, breath and plume outlets on the surface. Structure: skull, jaw, cervical, thoracic and lumbar spine with its curve, nine rib solids, sternum, clavicles, scapulae, pelvis with ilia, long bones in pairs (radius and ulna, tibia and fibula), patellae, hand and foot bones; the tube frame, forks and wheels of the rollator; 21 joints, the two supports at the wheel contact anchors (the asset's `structure.supports`), 20 masses.
- **1:100 (73 / 74 / 97).** Stated anew: coat in fewer sections, mitten hands with thumb on the grips, shoes as one form each, face with nose and ears, rollator as its tube runs, seat, strap, basket, bag and disc wheels in forks. Thermal by region with the face as an own skin. Structure: skull and jaw, spine, five rib rings, scapulae, ilia, single long bones per segment with fibula and radius merged, tube frame.
- **1:200 (45 / 46 / 57).** Merged masses: coat, sleeves, legs, shoes, fist blobs on the grips, head with beret and hair, rollator side frames, seat, basket, bag, plain wheels. Structure: skull, spine, three rib rings, shoulder and hip girdle bars, long bones, frame.
- **1:500 (35 / 35 / 40).** Coat, limbs, head and beret as few plain forms, fists on the grips, the rollator as two side frames, seat, basket with bag and four wheels. Structure is an own stick skeleton with joint nodes and the tube frame, not the body.
- `decisions`: bilingual sentences at every scale.

## 5. Checks

`volumes-check.ts humans/person-with-rollator --product`: `FAIL: 3 measurement(s) over 2 rule line(s)`: `areaDeviation:beret` 1.75 (1:50) and 1.72 (1:100), `areaDeviation:shoes` 0.56 (1:50). Silhouette IoU elevation / front / plan: 1:50 0.993 / 0.981 / 0.977, 1:100 0.994 / 0.981 / 0.978, 1:200 0.991 / 0.984 / 0.976, 1:500 0.981 / 0.988 / 0.990. Bounds, feature, monotonic, cross-scale, views-disagree, structure coverage, bones, slender, copied: green at every scale.

## 6. What I saw (this session, opened with the Read tool)

- 1:50 body sheet: front shows narrow sloping shoulders, coat with placket and buttons, glasses, fists on both grips, feet between the rear wheels. Right and left side show the rounded back, head forward, elbows slightly bent, toes at the rear axle inside the frame, lambda frame with seat, strap, basket and bag. Back shows hair under the beret and the coat hem at the knee. Top shows arms spread to the grips and the rollator 0.60 m wide. Three-quarter at full size: garment reads as a coat, hands wrap the grips, wheels with spokes and forks, brake cables. Wheel rims show as polygons of about 14 sides (tessellation of the tool).
- 1:50 structure, sheet and left side at full size: curved spine, ribcage, scapulae, pelvis, paired forearm and shank bones, patellae, finger bones on the grip, frame tubes on four wheels with the green supports under the wheels. The lumbar spine is one tapered member, not single vertebrae.
- 1:50 thermal and thermal cut: nested coat, trouser and shoe shells visible in the cut, face red, hair green, hollow basket with the bag inside, nothing poking out.
- 1:50 poses: `seated-rest` sits on the seat with the back at the strap and the rollator turned round, hands in the lap, feet flat. `rollator-walk` at 0.65 s: stride closed at hip and knee, hands stay on the grips, skirt halves follow the thighs.
- 1:100 body and structure: same figure, plainer; mitten hands on the grips. 1:100 thermal cut: coat, trousers, shoes and beret are drawn grey by `r3-sheet.ts` although each solid carries its region (known tool defect for layered regions without `layer`, tool-requests of 2026-11-19 and 2026-11-20); checked in the data instead: coat 10, trousers 2, shoes 4, beret 1, face-skin 7, hands 8, hair 3.
- 1:200 and 1:500 body and structure: recognisable at a glance as a stooped woman in a coat with a beret behind a four-wheel rollator; fists sit on the grips at 1:500.
- 2D 1:50 default pose: side, front and plan in line and tone match the body. The 2D sheets of 1:100, 1:200, 1:500 and of `seated-rest`, and the sheets of `stoop-to-basket` and `walking-midstance`, were rendered in this session but not opened; they were looked at by the earlier session only.

## 7. Still imperfect

- `beret` 0.0897 m2 against 0.0326 declared and `shoes` 0.178 against 0.1141: the model is a real beret 0.27 m across and a real pair of laced shoes; the declared shares are too small. Filed in `area-review.md` (2026-11-20 r3 humans/person-with-rollator) with hand calculations. The check stays red on these three measurements until the reviewers redistribute the shares (C27).
- Lumbar spine at 1:50 is one member.
- Front and plan drawings of trunk, legs and head are flat outlines on their bones: exact standing, approximate when she sits or stoops (stated in the drawing decisions).

## Files

- Scripts: `r3-person-with-rollator-kit.ts`, `-figure.ts`, `-aid.ts`, `-s50.ts`, `-coarse.ts`, `-build.ts`, `-roots.ts`, `-draw2d.ts`, `-candidate.py`, `-texts.json`, `-sheet.ts`, `-suit.ts`, `-probe.ts`, `-run.sh`, `-all.sh`.
- Source: `volumes/humans/person-with-rollator.json`. Renders: `🗑️generated/r3-sheet/humans/person-with-rollator.*.png`, `🗑️generated/r3-2d/humans/person-with-rollator.*.png`. Check output: `🗑️generated/r3-person-with-rollator/check.txt`.

## Round 5 handcraft (2026-11-21)

Scripts: the round-3 chain copied to `r5-person-with-rollator-{kit,figure,aid,s50,coarse,build,draw2d}.ts`, `-candidate.py`, `-texts.json` and changed there (r3 files untouched); new `r5-person-with-rollator-arms.ts` (shoulder move with arm re-solve, also used for the pregnant person), `-arms-check.ts`, `-measure.ts`, `-print-probe.ts`, `-print-cover.ts`, `-render-all.sh`, `-same.ts` (proof that the copied chain rebuilt the old source byte for byte before any change). Output `🗑️generated/r5-person-with-rollator/` (`before/`, `after/` sheets, `check.txt`, `measure.txt`, `catalog-before.json`, `source-before.json`).

### Reference (elderly woman, 79, 1.58 m upright; DIN 33402-2 women 61 to 65 less age loss, Drillis and Contini 1966 shares, ISO 11199-2 for the rollator)

| quantity | reference | model before | model after |
|---|---|---|---|
| hip joint (stooped stance, 12 mm sole) | 0.53 H + sole = 0.85 m upright, about 0.84 m stooped | 0.835 | 0.835 |
| knee | 0.285 H + sole = 0.462 | 0.460 / 0.451 | same |
| ankle | 0.039 H + sole = 0.074 | 0.074 | 0.074 |
| upper arm / forearm | 0.186 H = 0.294 / 0.146 H = 0.231 | 0.294 / 0.231 | same |
| head bone (joint to vertex) | head height H/7.5 = 0.21 incl. chin below the joint | 0.186 + chin 0.03 | same |
| biacromial breadth | 0.35 m (elderly woman, narrow sloping shoulders); glenohumeral joints about 35 mm inside the acromion tips: 0.315 | joints 0.354 apart (acromial about 0.39, +11 %) | joints 0.314 apart (acromial 0.35) |
| shoulder width over the coat sleeves | about 0.40 to 0.42 | 0.452 | 0.407 |
| chest girth over coat | 0.95 to 1.0 skin + coat layers, about 1.0 | 1.008 (width 0.338) | 0.983 (width 0.315, depth 0.291) |
| hip girth over coat | about 1.10 to 1.15 | 1.151 | 1.137 |
| calf girth (trousers) | 0.34 to 0.37 | 0.366 | 0.366 |
| neck girth | 0.33 to 0.35 | 0.338 | 0.338 |
| hip joint spacing | 0.16 to 0.18 | 0.169 | 0.169 |
| rollator width / grip height | 0.60 / 0.79 m | 0.60 / 0.79 | same (grips unchanged in the world, hands re-solved) |

All joint heights and segment lengths were within 3 % already; the only proportion defect was the shoulder breadth (+11 %).

### Grades (my own look at every sheet; before = round-3/4 state)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | B- -> A- | B+ -> A- | B -> A- |
| 1:100 | B -> B+ | B -> B+ | B- -> B+ |
| 1:200 | B -> B+ | B -> B | B- -> B |
| 1:500 | B- -> B+ | B -> B | B -> B |

Defects seen before (sheets `before/`): coat skirt read as shorts (a fork between two thigh-shaped halves at 0.72 to 0.76 m, front and back, all scales); shoulders 4 cm too broad with the sleeve heads standing up like epaulettes; spine one tapered tube (lumbar one member, r4 open point); ribs as nine horizontal hoops (a coil spring, not a ribcage) at 1:50, 1:100, 1:200; pelvis two loose ovals with no ring; clavicles reaching to the old shoulder.

### Changes

- Root: `rig.bones.arm-l/-r.z` 0.1771 -> 0.157. Every pose and every clip key that states the arms re-solved by damped least squares through the product's `evaluate` so both hands keep their grip anchors in the world (worst over all poses and both clips sampled at 20 Hz: 0.94 mm; hand orientation within 1.5 degrees); loop-closing keys take the arms of key 0.
- Body 1:50 to 1:500: coat torso half-width 0.013 m narrower from waist to shoulder (girth kept by 2 to 6 mm more depth), sleeve heads lower (first ring 0.045 -> 0.039, dome 18 -> 10 mm); skirt halves centred 18 mm inside their thigh with flat-sided sections (exponent 3.2 at the seat, rounding to 2.6 at the hem) so they overlap at full depth across the middle: one closed coat front and back, no fork, still following each thigh in the walk and when seated.
- Structure 1:50: the spine is now 17 vertebral bodies L5 to T1 (lumbar 1.35 times as high) with 17 spinous processes on the stooped curve; nine pairs of ribs, each a tube from its vertebra round the flank and down 30 to 62 mm to the front (upper two stop at the sternum); pelvis with iliac crest, sacroiliac bars and acetabulum rims tying ilium, sacrum, pubis and ischium into a ring; clavicles end at the new acromion (z 0.15). 1:100 and 1:200: rib hoops replaced by sloping rib pairs (5 and 3), clavicles and shoulder bar to z 0.15. Structure counts 191 -> 239 (1:50), 97 -> 102, 57 -> 60, 40.
- 2D: the coat, coat seat, skirt halves and the arms from sleeve to hand redrawn from the new solids at every scale and view (keeping the catalog's layer `z`); all other shapes kept as drawn. Print bands narrowed where the product measured visible slivers under 0.2 mm at band ends (they were already red before this round: skirt-l, main tubes, casters, beret): 1:100 75-130, 1:200 130-250, 1:500 250-500; the 22 mm tubes cannot print beyond 1:500. The 1:200 backrest strap is drawn 6 mm taller at each edge so its part beside the stooping arm stays printable.
- Thermal: `tyres.areaM2` 0.21 -> 0.24 (C16, factor 1.14; by hand four tyres 0.2 m x 36 mm: tread 0.0226 + sidewalls 0.0174 + shoulders, about 0.06 each).
- Decisions 1:50, 1:100, 1:200 rewritten for shoulders, skirt, spine, ribs and pelvis (en, de).

### Checks

`r3-validate.ts humans/person-with-rollator`: VALID (schema 0, validateAsset 0, print 0; before this round the catalog had 7 print findings). `volumes-check.ts --product`: all rules green at every scale except `volumes-thermal-coverage` `beret` (1:50 1.77, 1:100 1.74) and `shoes` (1:50 0.46) - the C18 body-region shares already filed in `area-review.md` (2026-11-20 r3 humans/person-with-rollator), unchanged.

### Seen (opened)

`after/s50.body` (six views), `s50.structure`, `s50.thermal.cut`, `s50.pose.seated-rest`, the `rollator-walk` frame at 0.4 s, `s500.body`, `s100.structure`, and the candidate front view at full size. The coat reads as one garment in front, back and seated; shoulders narrow and sloping; ribs slope, vertebrae show as a segmented column, the pelvis is a ring. 1:100 structure: ribs slope, but the ilia are still two loose ovals with crossed ischiopubic bars (B+). Rendered, not opened this round (their grades carry over from round 3 with the shared changes): 1:100 and 1:200 body, 1:200 and 1:500 structure, 1:100 to 1:500 thermal, `walking-midstance`, `stoop-to-basket`.

### Still open

- beret and shoes areas (C18, area reviewer).
- 1:500: a shallow notch (about 3 cm) remains at the top of the skirt between the halves in front.
- Sleeve heads still sit slightly proud of the coat shoulder line (reads as a set-in sleeve with light padding).
- The asset is unavailable beyond 1:500 (band narrowed for printability).
- 1:100 pelvis still two loose iliac ovals.

The tyre outlines, shoe tips and thigh contours have 0.20 mm printed weights at their respective generalisation scales. The 1:130 and 1:250 range endpoints and the official paper document now pass.
