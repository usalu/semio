# Round 3 — `humans/worker-carrying-box`

State (2026-11-20): root fixes and volumes are in the catalog asset. `r3-validate.ts humans/worker-carrying-box --catalog-only`: `VALID ... schema 0, validateAsset 0`. With `--volumes`: `checkVolumes 6`, all six are body-share thermal areas (section 5). `volumes-check.ts humans/worker-carrying-box --product`: every rule `ok` at every scale (silhouette, bounds, feature, monotonic, cross-scale, structure, bones, copied, views-disagree, availability) except `volumes-thermal-coverage` at 1:50 and 1:100 (`FAIL: 6 measurement(s) over 2 rule line(s)`). Print suitability under the default profile: all four scales `authored` (min feature 0.31 / 0.29 / 0.22 / 0.21 mm).

An earlier author had stopped after the rig draft (`r3-worker-carrying-box-roots.py`, `r3_worker_carrying_box_ik.py`, `_kit.py`, `_texts.py`), the 1:50 body and lists (`r3_worker_carrying_box_body50.py`, `_s50.py`) and a few probes; nothing was in the catalog, the source held a broken 1:50 (no thermal, no structure). I ran the validator and the check, repaired `face_rows` (complex power of a negative base), and continued from that state.

Scripts (ticket folder): `r3-worker-carrying-box-build.sh` (roots draft, volumes, 2D redraw, roots again, validate the candidate), `r3-worker-carrying-box-apply.sh` (build, write catalog only when the candidate is VALID, splice, validate), `r3-worker-carrying-box-roots.py` (rig, anchors, poses, clips, texts, dimensions, manifest, shapes), `r3_worker_carrying_box_ik.py` (arm solver onto the hand holes), `r3-worker-carrying-box-model.py` with `r3_worker_carrying_box_body50.py`, `_s50.py`, `_s100.py`, `_s200.py`, `_s500.py` (every section stated by hand), `_kit.py`, `_texts.py` (all bilingual texts), `r3-worker-carrying-box-draw2d.ts` (E1 redraw), probes `-armscan.py`, `-probe.ts`, `-measure.ts`, `-extent.ts`, `-suit.ts`, `-polycheck.py`, `-sheet.ts`.

## 1. Reference (stocky man, stature 1.741 m, 84 kg body; 30 mm boot sole; carton 20 kg)

| quantity | reference | value in the asset | source |
|---|---|---|---|
| hip joint height | 0.52 to 0.53 H = 0.905 to 0.923 | 0.9106 barefoot, 0.9406 shod (0.523 H) | brief; Winter 2009 tab. 4.1 |
| knee / ankle joint | 0.285 H = 0.496 / 0.039 H = 0.068 | 0.4906 / 0.0658 barefoot (0.282 H / 0.038 H) | Winter; de Leva shank 0.4248 kept |
| shoulder joint | 0.818 H = 1.424 | 1.424 barefoot, 1.454 shod | brief, Winter |
| C7 (cervicale) | 0.855 to 0.86 H | 1.498 barefoot (0.8605 H), 1.528 shod | de Leva 1996 |
| vertex / helmet crown | H; EN 397 shell 30 to 45 mm above the scalp | 1.771 / 1.815 shod | EN 397, own knowledge |
| head chin to vertex, length, breadth | 0.232 / 0.197 / 0.153 m (H/7.5 = 0.232) | 0.232 / 0.197 / 0.153 | ANSUR II means (Gordon 2014) |
| biacromial, bideltoid; shoulder joint spacing | 0.41 / 0.51 m; joints 2 to 3 cm inside the acromia | joints 0.36 m apart, sleeves 0.51 m over the deltoids | ANSUR II |
| chest, waist, hip breadth x depth (nude) | 0.33 x 0.255 / 0.33 x 0.25 / 0.353 x 0.25 | shirt 0.33 to 0.40 wide, vest 0.35 to 0.39 x 0.28 deep, seat 0.38 x 0.26 | ANSUR II, DIN 33402-2 |
| thigh girth top / mid / knee; calf; ankle | 0.625 / 0.54 / 0.41; 0.39; 0.225 m | trouser sections 0.19 x 0.22 / 0.17 x 0.20 / 0.13 x 0.14; 0.13 x 0.14 | ANSUR II + 10 mm ease |
| upper arm / forearm / wrist girth | 0.33 / 0.29 / 0.176 m | sleeve 0.128 x 0.12 at the deltoid, forearm 0.10 x 0.094 | ANSUR II + 6 mm ease |
| upper arm / forearm / hand length | 0.186 / 0.146 / 0.108 H | 0.3135 / 0.2731 (de Leva, kept) / 0.182 | Winter; de Leva |
| hand length x breadth x thickness | 0.193 x 0.089 x 0.030 | glove 0.19 x 0.092 x 0.027 to 0.048 | ANSUR II + glove |
| foot; safety boot | 0.271 x 0.102 m; EU 44 ankle boot 0.30 to 0.31 x 0.11, shaft 0.17 to 0.19, cleated sole 30 mm, steel toe cap | 0.305 x 0.112, shaft top 0.176, sole 30 / 19 mm | EN ISO 20345, own knowledge |
| hard hat | EN 397: shell 0.26 x 0.22 m plan, 0.14 m high, front peak 40 to 60 mm, crown ridge | 0.262 x 0.22, peak 0.08 ahead, ridge | EN 397, own knowledge |
| hi-vis vest | ISO 20471 class 2: two 50 mm bands round the trunk, braces over the shoulders | two 50 mm bands at 1.08 and 1.21, braces | ISO 20471 |
| carton | FEFCO 0201 regular slotted case 0.40 x 0.30 x 0.30, BC double wall about 6.5 to 7.5 mm, die-cut hand holes about 105 x 36 mm, 65 mm below the lid | 0.40 x 0.30 x 0.30, 7.5 mm board, 105 x 36 mm holes centred 65 mm below the lid | FEFCO code, own knowledge |
| squat lift | trunk inclination comes mostly from hip flexion with a near-neutral lumbar spine; pelvis tilts forward 25 to 40 degrees | pelvis -38 / -30 / -25 degrees in squat, mid lift, setting down; lumbar flexion 28 / 20 / 17 degrees | biomechanics texts (own knowledge) |
| segment masses and centres | de Leva 1996 | unchanged; centres re-expressed in the new bone frames | asset references |

No web lookup was made; product and anthropometric values are from the asset's references and my knowledge.

## 2. Audit (before)

- Rig: no sole anywhere (ankle 0.0809 with a boot drawn round it); torso straight up from the pelvis tip, C7 at 1.507 (21 mm low) and no offset of the lumbosacral joint behind the hips; head bone 0.2731 from inside the neck; shoulder joints 0.401 m apart (wider than the 0.41 m biacromial minus the 2 x 25 mm to the joint centres); box as two 0.4 m bones lying along the hands, so the carton turned with every wrist angle.
- Poses: arms hung straight with the hands beside the carton, not in it; pelvis never tilted, so the squat bent the lumbar spine 75 degrees; squat legs (thigh 88, shank -113) put the knees ahead of the toes by 0.2 m.
- Anchors: contacts and heels of a bare foot; mouth and nose 4 to 6 cm off the face; box base 0.15 m below the bone, inside the carton.
- Dimensions: 1.815 x 0.68 x 0.50 (height, forward, lateral); the drawn 1:50 front was 0.729 wide.
- 2D: constructed views, not one body: plan of capsules and ellipses, front arms without lateral solids (`lateral-ambiguous` once the arms were posed honestly), 1:500 front without the far arm.
- Solids (1:50 27 / 64 / 36): qa-visual-3 grades B- to C+; box torso, tube limbs, one hand gripping air, splayed long boots, structure as the body again.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `torso` y / angleDeg / lengthM / comX | 0 / 0 / 0.4652 / 0.2124 | 0.045 / -1.767 / 0.48653 / 0.2221 (L5/S1 45 mm behind the hip joints, to C7 at 1.528) |
| `head` x / angleDeg / lengthM / comX | 0.4652 / 0 / 0.2731 / 0.1365 | 0.48653 / -4.107 / 0.24428 / 0.1222 (C7 to vertex 1.771) |
| `upper-arm-*` x / y / z / angleDeg | 0.4349 / 0 / ±0.2005 / -180 | 0.41318 / -0.02227 / ±0.18 / -178.233 (joint at 1.454, 0.36 m apart) |
| `thigh-*` lengthM / comX, `shank-*` x | 0.4349 / 0.1781, 0.4349 | 0.42 / 0.172, 0.42 (knee at 0.521 shod) |
| `foot-*` comY, `toe-*` y / lengthM | -0.0405, -0.0809 / 0.0991 | -0.0479, -0.0958 / 0.09 (30 mm sole) |
| `box-*` angleDeg / lengthM / title | 0 / 0.4 | 90 / 0.15, "carton half hanging on the left/right hand" |
| anchors contact / heel | y -0.0809, 0.2023 x 0.1011 / x -0.0303, 0.0809 x 0.0809 | y -0.0958, 0.10 x 0.108 / x -0.04, 0.085 x 0.088 |
| anchors mouth / nose / crown | (0.1153, -0.0961) / (0.1526, -0.1216) / (0.3206, -0.0166) | (0.0679, -0.1227) / (0.0904, -0.1264) / (0.2873, 0.0115) on the lips, nostrils, helmet crown |
| anchor `box-base` | (0, -0.15), 0.4 x 0.3 | (0, -0.235, z 0.217), 0.30 x 0.40 under the carton |
| all poses, arms and box bones | hands beside the carton | solved by `r3_worker_carrying_box_ik.py`: knuckles in the hand holes, carton level and square in every pose |
| `carrying-box-chest` | upper arms horizontal sideways, elbows behind the back | grip (0.30, 1.16), elbows down and out at 1.27 m |
| `squat-grip` legs and trunk | thigh 88, shank -113, foot 25, torso -75 | pelvis -38, thigh 110, shank -102, foot 30, torso -28 |
| `lift-mid`, `setting-down` | no pelvis tilt | pelvis -30 / -25, thigh and torso compensated (same world segments) |
| clip `carry-walk` advanceM | 1.10000629 | 1.097274 (measured planted travel) |
| `dimensions` heightM / widthM / depthM | 1.815 / 0.68 / 0.50 | 1.82 / 0.65 / 0.52 (default-pose 1:50 body 1.818 x 0.650 x 0.516); manifest height and width likewise |
| summary, provenance notes, `structure.assumptions`, pose and clip descriptions | box, no hand holes | carton with hand holes, half bones, round-3 note (en, de); references winter-2009, gordon-2014, din-33402-2, en-iso-20345, fefco-0201 |
| `representations[*].geometry` | 148 / 81 / 59 / 45 shapes | 114 / 97 / 77 / 58 shapes, redrawn (below) |
| `representations[*].decisions` silhouette, retained, merged, omitted, thinElements | constructed drawings | the views of the body |

2D redraw (E1, `r3-worker-carrying-box-draw2d.ts`): head, neck, hair, ears, eyes, brows, nose, helmet, ridge, shirt, vest, vest hem, bands, braces, seat, thighs, shanks, pockets, knee pads, boots, soles, toes are the outlines of their solids (bone-local in the elevation, rest-world in front and plan), traced on a raster of 1 / 1 / 2 / 3 mm and reduced at 1.2 / 2.5 / 5 / 10 mm; sleeves, forearms and glove cuffs are `revolved` profiles of their loft sections (lateral scale from the sections), gloves and every carton part `extruded` profiles, so poses that turn the arms and the carton stay right. One id per part in all three views (`sleeve-*`, `waist`, `trunk`, `shoulders`, `face`, finger lines and fold strokes are gone; new: `neck`, `vest-hem`, `vest-strap-*`, `cargo-pocket-*`, `sole-*`, `collar` at 1:500). Mouth, jaw, zip and collar lines hand-stated at the model's landmarks. Workarounds, both stated in the decisions: thumbs merged into the glove outline and extruded profiles closed by 4 mm (the product's polygon union threw on the notched profile, `tool-requests.md`); bands outlined, not filled, at 1:100 and 1:200, and at 1:500 the front face merged into the helmet piece and the seat left behind the carton (otherwise slivers under the 0.2 mm print minimum in the chest carry and the squat).

## 4. What was modelled (body / thermal / structure solids)

- **1:50 — 100 / 179 / 178.** Body as in its decisions: skull loft with jaw and chin, nose, lips, eyes, brows, ears, hair at the nape, neck; hard hat dome, rim, peak, ridge, headband; shirt and collar, vest on the torso and vest hem on the pelvis (so the hem follows the hips in the lift), bands, braces and zip as texture; seat with gluteal masses, thighs, cargo pockets, knee pads, knees, shanks over the boot shafts; boot shaft, vamp, 30 mm sole, toe cap, toe sole, laces; sleeves (cap lowered into the shoulder slope this round), forearms, glove cuffs, palm, thenar, four three-segment fingers hooked into the hand hole, two-segment thumb; carton sleeve of 7.5 mm board, two end panels with 105 x 36 mm hand holes, tapes, label. Thermal: nested shells of real thickness per layer, face solid, breath outlets (7.8 mm) and plume patch on the helmet crown. Structure: full skeleton (cranium to phalanges, 12 ribs per side, sacrum, hip bones, scapulae), 17 joints, masses, carton walls and half masses.
- **1:100 — 48 / 44 / 87.** New sections; mitten gloves with one curled finger block and a thumb; carton as closed body with the hand holes as 15 mm recesses in its end blocks; thermal one surface per region (face as its own solid, vest as open shells), structure with four rib hoops, metatarsal fan, finger row.
- **1:200 — 32 / 33 / 44.** One mass per part, helmet with peak, collar above the vest, carton block; skeleton of 30 mm bones with three rib hoops, shoulder and pelvic bars, ball joints at hip, knee, shoulder, elbow.
- **1:500 — 21 / 25 / 25.** Head, helmet dome with front brim, collar ring, vest trunk, seat, two-part limbs with fists and boots, carton block; stick skeleton of 75 mm bars.
- Decisions: three bilingual sentences at 1:50, two at the coarser scales.

Final check line (`volumes-check.ts --product`): `FAIL: 6 measurement(s) over 2 rule line(s) (17.5 s)`, only `volumes-thermal-coverage` helmet / leg-clothing / shoes at 1:50 and 1:100. Silhouette 1:50 elevation / front / plan IoU 0.993 / 0.989 / 0.971 (p95 2 / 6 / 11 mm); cross-scale IoU 1.000 at every step.

## 5. Areas (not changed, C18)

`helmet` 0.102 (1:50) and 0.187 (1:100) against 0.067, `leg-clothing` 1.042 / 1.033 against 0.73, `shoes` 0.353 / 0.325 against 0.141: the declared values are skin shares of scalp, legs and feet; the honest hard hat, trousers over the boot shafts and EU 44 safety boots are larger. Filed in `author-requests.md` ("2026-11-20 r3 humans/worker-carrying-box"). Passing: head-skin +25 %, gloves +25 %, arm-clothing +22 %, vest +17 %, torso-clothing +7 %, box-surface 0 %.

## 6. What I saw in the renders

- 1:50 body sheet, carrying at hip height (candidate sheet, then the same model in the catalog): front reads as a worker with hard hat, class-2 vest with braces and two bands, arms hanging to the carton ends; right and left sides show the forward hands hooked into the hand holes, the carton level in front of the thighs, boots with soles; back shows braces crossing the shoulders; top shows helmet, shoulders, arms to the carton ends; three-quarter (turn 60, tilt 15) after lowering the sleeve caps: continuous shoulder line, no ball shoulders.
- 1:50 squat (before and after the pelvis tilt): before, the vest bottom stood out behind the seat as a tube and the lumbar spine bent 66 degrees; after, the pelvis tilts, the back is straight, hands in the holes, carton on the floor ahead of the toes.
- 1:50 chest carry (before and after): before, upper arms horizontal sideways with elbows behind the back; after, elbows down and out at 1.27 m, hands hooked into the holes from above, carton at lower chest.
- 1:50 walk clip at 0.3 s: stride, carton clears the thighs, both hands in the holes.
- 1:50 structure sheet: skeleton reads at once, inside the body everywhere; iliac wings appear as two ovals from behind. 1:50 thermal cut sheet: vest over shirt and air, socks in the boots, face red under the green hair layer, plume patch on the crown.
- 1:100 body and structure sheets: same figure simplified; the lowest rib hoop reads as a loop in the front view. 1:200 body (before the band fix: bands z-fighting with the vest; fixed) and structure sheets. 1:500 body sheet before and after the brim and nape fix: before, the hard hat read as a military helmet and the nape ball stood out; after, the brim reaches forward as a peak and the nape mass shows only as a small bump in the side views; the figure with helmet, vest trunk and carton on hanging arms reads at a glance.
- `r3-2d` 1:50 carrying (candidate) and chest carry (catalog): line and tone views read as the same worker in all three views; revolved arms correct in the turned chest pose. 1:500 2D was first unsuitable (head 0.17 mm, seat 0.14 mm in the squat front), fixed and measured suitable, not re-opened.
- `volumes-render` 1:50 overlay sheet: green almost everywhere, thin red and blue edges in the plan only.
- Not opened: 1:100 and 1:200 2D, 1:500 structure and thermal, 1:100 thermal, the list-only `volumes-render --az` series; `lift-mid`, `setting-down`, `squat-lift` and `set-down` frames after the pelvis tilt were checked by the validator only.

## 7. Still imperfect

- Grade as I see it: 1:50 body B+ (no cloth folds beyond hems and cuffs, elbow and knee seams are overlaps of two rigid lofts, face plain); coarse scales B+; structure A- at 1:50, B at 1:500 (stick figure forced by the 75 mm feature).
- The chest carry by hand holes is inherently elbows-out; the solver cannot find a wrist position within the joint limits for fingers inserted horizontally, so the hands hook from above.
- Three body-share areas stay red (section 5). The 1:100 helmet thermal is the closed shell with its base, honest for a merged single surface but counting the base disc.
- Front and plan of 1:500 and 1:100/1:200 band outlines are compromises for the print profile (section 3).
- Not run: the staffage test suites; fixtures that embed this asset may need regenerating after the rig and 2D change.

## Round 5 handcraft (2026-11-21)

Scripts (ticket folder): `r5_worker_carrying_box_body50.py`, `r5_worker_carrying_box_s50.py` … `_s500.py` (round-3 modules carried over and reshaped where named below), `r5-worker-carrying-box-model.py`, `r5-worker-carrying-box-roots.py` (round-3 roots with two fixes), `r5-worker-carrying-box-draw2d.ts` (2D redraw), `r5-worker-carrying-box-ship.sh` (roots → volumes → redraw → roots → volumes → validate the candidate; `--apply` writes the catalog only when VALID, splices, runs the product check and the full validation); shared with the courier: `r5-courier-with-hand-truck-measure.ts`, `-ends.ts`, `-render.sh`. Output: `🗑️generated/r5-worker-carrying-box/` (`before/`, `after/`, `check.txt`, `measure.txt`).

### Reference

The round-3 table above stands (stocky man 1.741 m, 84 kg, ANSUR II means, EN 397 hard hat, ISO 20471 vest, EN ISO 20345 boots, FEFCO 0201 carton); added: bideltoid breadth 0.51 m (ANSUR II male mean 0.513) plus 6 mm shirt, and the shoulder slope from the neck base to the acromion of about 18-20 degrees.

### Model against reference (1:50 body, rest pose; `measure.txt`)

| quantity | model | reference | deviation |
|---|---|---|---|
| hip / knee / ankle joint (shod) | 0.941 / 0.521 / 0.096 | 0.935-0.953 / 0.526 / 0.098 | in range / -1.0 % / -2 % |
| shoulder joint / C7 / vertex | 1.454 / 1.528 / 1.771 | 1.454 / 1.528 / 1.771 | 0 |
| upper arm (joint to joint) / forearm / gloved hand length | 0.314 / 0.273 / 0.190 | 0.324 (Winter 0.186 H; de Leva 0.282) / 0.269 (de Leva 1996, male of exactly 1.741 m) / 0.193 (ANSUR II hand length) | between the two sources / +1.6 / -1.6 % |
| elbow / wrist joint height (barefoot) | 1.110 (0.638 H) / 0.837 (0.481 H) | 0.63 H / 0.485 H | +1.3 / -0.8 % |
| head chin to vertex | 0.232 | 0.232 | 0 |
| bideltoid over the shirt | 0.511 | 0.51-0.525 | in range |
| shirt chest breadth x depth | 0.340 x 0.259 | 0.342 x 0.267 | -0.6 / -3.0 % |
| vest waist breadth x depth | 0.355 x 0.283 | 0.354 x 0.274 | +0.3 / +3.3 % (vest pocket fill) |
| hip over the seat | 0.378 | 0.373 | +1.3 % |
| trousered thigh girth (mid) / calf girth | 0.582 / 0.436 | 0.58 / 0.43 | +0.3 / +1.4 % |
| shirt upper arm girth | 0.339 | 0.35 | -3.1 % |
| hard hat over everything | 1.815 | 1.81-1.82 | in range |
| carton | 0.30 x 0.30 x 0.40 | FEFCO 0201 0.30 x 0.30 x 0.40 | 0 |

Segment lengths follow de Leva 1996, whose reference male is 1.741 m tall like this worker (the mass data belong to the same segments); Winter's 0.146 H forearm (0.254 m) uses another landmark, and the joint heights it implies are met within 1.3 %.

### Grades (body / thermal / structure)

| scale | before (catalog) | after |
|---|---|---|
| 1:50 | B / B+ (z-fighting yoke shells) / A- | A- / A- / A- |
| 1:100 | B- (brace flaps off the vest) / B+ / B+ | A- / B+ / B+ |
| 1:200 | B / B+ / B | B+ / B+ / B |
| 1:500 | B- (helmet read as a cap, square trunk) / B / B | B+ / B / B |
| poses 1:50 | squat, lift, set-down B+; chest carry C+ (upper arms horizontal) | squat, lift, set-down A-; chest carry B (elbows lower, still hooked from above) |

### Changes

- Shoulders: the shirt yoke now falls from the neck base (half breadth 0.074 at 1.534) to the acromion (0.198 at 1.44) over seven rows instead of a flat plateau at 1.48-1.50; collar raised to sit round the neck (1.505-1.556); sleeve caps start 40 mm above the joint as a dome (12 x 16 mm top) inside the yoke, deltoid 0.066 wide with its lateral bulge, so the bideltoid breadth is 0.511 m; the same at 1:100, 1:200 (four rows) and 1:500 (three rows, cap 34 mm above).
- Trunk sections: vest, vest hem, seat and bands from exponent 2.5-2.6 to 2.35 (no box corners); the braces are traced on the vest with the same exponent; at 1:100 bands and braces now follow the 1:100 vest itself (they followed the 1:50 rows and stood off the vest as flaps).
- 1:500: hard hat a dome of six rows with the forward brim (was a flat-topped cap), trunk with the shoulder slope, collar ring narrowed under the helmet.
- Thermal 1:50: the torso-clothing yoke shell lies 1.5 mm outside the vest's shirt layer (the two regions shared one surface and z-fought in the thermal view).
- Roots: `r5-worker-carrying-box-roots.py` fixes a round-3 script defect that would have broken the squat (re-run on the catalog it set torso -66 and thigh 72 without the -38 pelvis tilt: trunk folded 104 degrees); the chest carry solves the elbows towards (-0.2, -0.7, 0.9) (down and out) instead of (0, -0.5, 1) (sideways).
- 2D (E1): every scale and view retraced from the new bodies by `r5-worker-carrying-box-draw2d.ts`; collar lines at the new collar.

### Checks

`r3-validate.ts humans/worker-carrying-box --volumes`: schema 0, validateAsset 0 (print clean at every band and range end), checkVolumes 6. `volumes-check.ts --product`: every rule ok except `volumes-thermal-coverage` helmet (+53 % / +181 %), leg-clothing (+43 % / +42 %) and shoes (+143 % / +119 %) at 1:50 / 1:100: the same three physiological shares as in round 3 (C18; listed in `area-review.md`). Gloves pass at +29 % (1:50).

### Remaining

- The three area lines (area reviewer, C27).
- Chest carry: hands hooked into the hand holes from above force the elbows out (B); a carry with the palms under the carton would need a different grip anchor.
- Garments rigid per bone, no cloth folds; the vest hem shows a seam crease at the lumbar bend in the squat.
- Rig: no neck bone (the head bone runs from C7 to the vertex); the cervical curve is in the structure list only.
- Opened: before and after sheets at 1:50 (body, thermal cut), 1:100 and 1:500 three-quarter views, the squat and chest carry candidates, 2D 1:50; the other after sheets were rendered (`after/`) and checked by the validator.
