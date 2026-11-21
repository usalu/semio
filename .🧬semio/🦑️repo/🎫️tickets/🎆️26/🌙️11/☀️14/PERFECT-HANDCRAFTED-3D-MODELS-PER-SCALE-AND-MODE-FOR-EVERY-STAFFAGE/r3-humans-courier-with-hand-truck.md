# Round 3 — humans/courier-with-hand-truck

State: catalog asset written (root fixes and `volumes` spliced), `r3-validate.ts` VALID with and without the source, `volumes-check.ts --product` green at 1:200 and 1:500 and green at 1:50 and 1:100 except four physiological area lines (`boots`, `gloves`, `cap`; see "Still imperfect"). This run continued the work of an earlier author who was cut off: his scripts were read, the half-finished candidate was repaired (the catalog itself was still untouched), the forms were judged again and partly remodelled, the 2D pipeline was repaired and everything was applied.

Scripts (all in `T`, re-runnable in this order): `r3-courier-roots.ts` (rig, anchors, poses, clips, texts, reference; candidate document), `r3-courier-kit.py` + `r3-courier-s50.py`, `-s100.py`, `-s200.py`, `-s500.py` + `r3-courier-build.py` (every section stated by hand), `r3-courier-draw2d.ts` + `r3-courier-decisions2d.json` (2D of all scales and views, dimensions), `r3-courier-apply.py` (catalog members other than `volumes`), then `volumes-insert.py`. Diagnostics: `r3-courier-probe.ts`, `r3-courier-view2d.ts`, `r3-courier-suit.ts` (print suitability per pose and view), `r3-courier-outside.ts` (structure solids leaving the body silhouette).

## 1. Reference

| quantity | real value | source |
|---|---|---|
| stature | 1.82 m, male, 34 years, 84.5 kg (133.5 kg with truck 11 kg and load 38 kg) | asset `thermal.body`, summary |
| hip joint height | 0.52-0.53 H = 0.95-0.96 m | Drillis and Contini 1966; Winter 2009 table 4.1 |
| knee / ankle height | 0.285 H = 0.52 m / 0.039 H = 0.071 m (+ sole) | same |
| shoulder joint / elbow / wrist height | 0.818 H = 1.49 m / 0.63 H = 1.15 m / 0.485 H = 0.88 m | same |
| upper arm / forearm / hand length | 0.186 H = 0.339 m / 0.146 H = 0.266 m / 0.108 H = 0.197 m | same |
| head height (chin to vertex) | H / 7.5 = 0.243 m; breadth 0.155 m, length 0.197 m | brief; DIN 33402-2 (median man), my knowledge |
| biacromial breadth | 0.245 H = 0.446 m as breadth over the shoulders; acromion to acromion about 0.39-0.40 m; glenohumeral centres about 0.37 m apart | brief; DIN 33402-2; de Leva 1996 |
| bideltoid breadth | about 0.48-0.50 m nude, 0.52 m in a jacket | DIN 33402-2 |
| hip joint centres | 0.17-0.19 m apart; hip breadth 0.19 H = 0.35 m | brief; de Leva 1996 |
| chest depth / waist depth | 0.23-0.25 m / 0.22-0.25 m nude at BMI 25.5 | DIN 33402-2, my knowledge |
| neck | girth 0.39 m, diameter 0.12 m | my knowledge |
| thigh / calf / ankle girth | 0.56 m / 0.38 m / 0.23 m | my knowledge |
| upper arm / forearm / wrist girth | 0.31 m / 0.28 m / 0.175 m | my knowledge |
| foot / work boot | foot 0.152 H = 0.277 m; 6 inch safety boot size 45 about 0.31 m long, 0.105 m wide, 30 mm sole with heel | Drillis and Contini; my knowledge |
| hand truck | fetra B1125L steel tube truck: 570 x 533 x 1150 mm (W x D x H), toe plate 320 x 250 mm, pneumatic wheels 260 x 85 mm, 200 kg capacity, 11 kg | manufacturer data, checked by web search on 2026-11-19 (links below) |
| tube | 27 mm steel tube bow frame, three cross braces, axle on brackets behind the rail plane | product pictures, my knowledge |
| balance | truck and load balance when their centre of gravity stands over the axle; with this stack 48.6 degrees back from vertical | computed from the asset's masses |
| cartons | 460 x 408 x 400, 400 x 368 x 300, 360 x 292 x 234 mm (38 kg together) | asset (authored) |

Sources for the truck: [contorion data sheet](https://media.contorion.de/content/pdf/fetra/at/B1125L.pdf), [plate.de data sheet](https://static.plate.de//downloads/91125/136941026/91125%20Fetra%20Stahlrohrkarre%20B1125L.pdf), [boxic24](https://boxic24.com/english/fetra-b1125l.html), [eisenwolff](https://eisenwolff-online.de/stahlrohrkarre-b1125l-200kg-schau-250x320-luft).

## 2. Audit against the reference (state before round 3)

| item | found | reference | verdict |
|---|---|---|---|
| hip joint y | 0.9575 m (0.526 H) | 0.52-0.53 H | right |
| knee / ankle | 0.5138 m (0.282 H) / 0.0803 m (0.044 H with sole) | 0.285 H / 0.039 H + sole | right |
| shoulder / elbow / wrist height | 1.4912 / 1.1546 / 0.8894 m (0.819 / 0.634 / 0.489 H) | 0.818 / 0.63 / 0.485 H | right |
| segment lengths | arm 0.3366, forearm 0.2652, hand 0.1938, thigh 0.4437, shank 0.4335 m | as table | right |
| shoulder joint z | +-0.160 m (0.32 m apart) | about 0.37 m apart | **wrong, 25 mm too narrow per side** |
| hip joint z | +-0.095 m | 0.17-0.19 m apart | right |
| truck bone | rest angle 112 degrees (22 degrees back), length 1.12 m, axle on the rail line, handle anchor at (0.96, +0.02) | 1.15 m high truck, axle 0.14 m behind the rail plane, balance near 45-49 degrees | **wrong: no real truck geometry; at 22 degrees the 49 kg load would pull the courier forward** |
| truck reach | truck can only move vertically (`truck-lift`), so the parked truck could not stand upright on plate and wheels in front of the feet | horizontal travel needed | **missing joint** |
| hands | grip anchors on the hand axis, hands 20-24 degrees extended, not on the handle in `parked-upright` (hand 0.05 m off) | hands closed around the bar | **wrong** |
| `dimensions` | 1.82 x 1.152 x 0.60 m | pose box with a real truck 1.81 x 1.58 x 0.578 m | **wrong length and width** |
| 2D 1:50 to 1:500 | truck with 32 mm rails, 50 mm tyres, plate 26 mm lip, handle 1.06 m; figure drawn per view (front and plan constructed, arms as stadiums) | E1 | **redrawn** |
| body 1:50 (115 solids) | fitted skin lofts with garment shells, arms as tubes with ball shoulders, box-like trunk | brief | **replaced** |
| structure | long bones as tubes, no ribcage, no scapulae, no clavicles | D3 | **replaced** |
| thermal | eight area lines failing (cap +149 %, boots +155 %, trousers +46 %, gloves +36 % at 1:50 and 1:100) | C29 | **replaced, four lines remain** |
| `tyres.areaM2` | 0.26 m² (for a 50 mm tyre) | 260 x 85 mm: 2 x (0.069 tread + 0.0865 sidewalls) = 0.31 | **corrected (factor 1.19, passive region, C16)** |

## 3. Root fixes (old -> new)

- Rig: `arm-l/-r.z` +-0.16 -> +-0.185. New bone `truck-reach` (prismatic, horizontal, -0.1 to 0.6 m) between `pelvis` and `truck-lift`, origin (-0.82745, -0.66) -> (-0.82745, -0.95). `hand-truck` rest angle 112 -> 135 degrees, length 1.12 -> 1.02 m (axle to handle bar), joint range (-27..70) -> (-45..45), centre of mass (0.42, -0.01) -> (0.14, -0.12). `parcels` length 0.9 -> 0.812 m, centre of mass (0.36, -0.19) -> (0.2909, -0.3415).
- Anchors: `handle-grip` (0.96, 0.02) -> (1.0065, -0.14); `grip-l/-r` (0.0918, 0) -> (0.082, -0.032) (centre of the closed fist); `parcels-cog` (0.36, -0.19) -> (0.2909, -0.3415); `mouth` (-0.005, -0.0801) -> (-0.019, -0.103); `nostrils` (0.03, -0.104) -> (-0.001, -0.114); `crown` (0.175, 0) -> (0.18, -0.002).
- Poses: arms of all four poses and of every clip key solved onto the handle bar (grip anchor on the bar at +-0.07 m, residual below 0.01 mm, wrist 12 degrees extended), e.g. `waiting` arm -8.1 -> 9.7 degrees with 11.3 degrees adduction, forearm 72.8 -> 47.4, hand 20.4 -> 12. Truck: `tilted-back` 9 -> 6 (51 degrees back, past the balance point), `parked-upright` -10 -> -45 (upright on plate and wheels) with `truck-reach` 0.53 m and the left arm hanging; `tilt-and-settle` keys 2 / 3.3 / 6 degrees. Descriptions of summary, poses and clips rewritten (en, de).
- `dimensions`: 1.82 x 1.152 x 0.60 -> 1.81 x 1.579 x 0.578 m (box of the 1:50 body in the default pose; the truck is 570 mm over the tyres and 578 mm over the axle ends).
- `thermal.regions.tyres.areaM2` 0.26 -> 0.31. No other area was touched (C18).
- `provenance.references`: `fetra-b1125l` added.
- 2D (`representations` 50, 100, 200, 500; elevation, front, plan): redrawn as the orthographic views of each scale's body (E1). Trunk, head, legs and boots are outlines per bone (bone-local in the elevation, rest-world in front and plan); arms, forearms, cuff straps and fists are revolved or extruded `solid` profiles, the truck and the cartons extruded profiles, the wheels direct, the struts open detail strokes. Ids, layer order, tones, regions and hatches of the previous drawing kept where a shape of the same id existed. 1:500: rails 36 mm, tyres 100 mm wide, cartons one stepped stack of mean width (print profile: 0.2 mm with the 0.13 mm pen, also for the visible part of a covered shape). `decisions` of all four representations rewritten (en, de). The outline tracer of the earlier author dropped half of the front trunk where pixels touched diagonally; repaired and guarded (it throws on an open outline).

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 81 solids | 83 | 117 |
| 1:100 | 43 | 43 | 76 |
| 1:200 | 30 | 32 | 35 |
| 1:500 | 25 | 28 | 25 |

- **1:50 body**: jacket loft of eleven sections (hip, waist, chest, armpit, yoke widening to the acromion at 0.222 m half width, trapezius slope to the neck), stand-up collar, two 50 mm reflective bands, trousers seat, neck, head loft with jaw and chin, wedge nose (loft), ears, brow, eyes, hair at the nape, knit cap with cuff and brim; per arm a sleeve of eight sections whose head sits inside the yoke, forearm nested in the sleeve end, cuff strap, glove cuff, palm, four fingers of three phalanges closed around the bar and a two-part thumb; per leg thigh, cargo pocket, shank nested in the trouser knee, boot shaft, upper, sole with heel and arch, toe cap and toe sole. Truck: two 27 mm rails, bow corners of three segments each, handle bar, three braces, two 22 mm struts per side to the axle 0.14 m behind the rails, 8 mm toe plate 320 x 250, axle, tyres 260 x 85 with round shoulders on dished rims; three cartons with tape (texture).
- **1:50 thermal**: jacket, collar, sleeves, forearms as three nested shells (shirt 1.2 mm, fleece 4 mm, softshell 2 mm), trousers with briefs under the upper thigh, boots with socks, gloves, cap dome (inner face a section face), face skin, truck, tyres, cartons; breath outlets under the nose and at the mouth, the body plume as a patch on the crown.
- **1:50 structure**: cranium, facial skeleton, mandible with rami, atlas, cervical spine, C7, thoracolumbar spine with lordosis and kyphosis, six rib hoops inclined 20 degrees, sternum inclined 12 degrees, clavicles to the acromion, scapulae, sacrum, ilia, pubic and ischial rami; per arm humeral head, humerus, condyles, ulna, radius, carpus, metacarpals, curled phalanges, thumb; per leg femoral head and neck, femur, condyles, patella, tibial plateau, tibia, fibula, talus, calcaneus, metatarsals, toe joints and phalanges; truck frame, axle, wheels and cartons as their own members (they are the members), axle pivot joint, masses at the de Leva centres and at the centres of truck and cartons. `structure.supports` is empty in the asset, so there is no support solid.
- **1:100** (15 mm): one mass per garment segment, one band zone, mitten fist with a finger roll, boot of upper, sole and toe, head with nose and padded brim; truck with chamfer bows, three braces, one strut per side, plain tyres and conical rims; skeleton with four rib hoops, merged forearm and hand bones.
- **1:200** (30 mm): trunk, seat, neck, head, cap and three sections per limb segment, ellipsoid fist, one-piece boot; rails and axle slender at their true size, handle, one strut per side, plate, disc wheels; skeleton of spine, two rib hoops, girdles and one bar per segment.
- **1:500** (75 mm): trunk, neck, ellipsoid head under the cap, two-section limbs, fists, boots; two slender rails and the axle, two wheels, three cartons; handle bar (inside the fists), toe plate, struts and braces are below the feature and omitted; stick skeleton.
- `decisions` of every scale are bilingual sentences (en, de) that say what is merged and omitted.

## 5. Checks

- `$BUN r3-validate.ts humans/courier-with-hand-truck`: `VALID ... schema 0, validateAsset 0` (also with `--catalog-only`).
- Print suitability (`r3-courier-suit.ts`): all four scales `authored` (before the repair 1:50 and 1:500 were `unsuitable` in the candidate, so the product could not draw them).
- `$BUN volumes-check.ts humans/courier-with-hand-truck --product`: 1:200 PASS, 1:500 PASS; 1:50 and 1:100: `FAIL: 4 measurement(s) over 2 rule line(s)`, all in `volumes-thermal-coverage`: cap 0.326 and gloves 0.348 at 1:50, boots 1.229 at 1:50 and 1.090 at 1:100 (limit 0.3). Every other rule line is ok.
- Silhouette (IoU elevation / front / plan): 1:50 0.987 / 0.994 / 0.978; 1:100 0.991 / 0.994 / 0.982; 1:200 0.991 / 0.992 / 0.979; 1:500 0.985 / 0.947 / 0.936 (the generalised stack and tyres). Bounds deviation at most 4 mm. Structure outside the body: 0 to 4 mm.

## 6. What I saw (images opened with the Read tool)

- `r3-sheet` 1:50 body, default pose (six directions, with and without rig; close-ups of right side, front, three-quarter): from the front a man with sloping shoulders, deltoid caps, arms converging on the handle, face with nose and jaw under the cap; from the right side upright trunk with chest and seat, flexed elbow, fist around the bar, cargo pocket, boot with heel and toe cap; the truck leans 45 degrees with the wheel behind the rails, struts and plate, cartons stepped on the plate; back view shows the seat under the hip-length jacket; top view shows shoulders, cap, both forearms to the bow handle, wheels beside the stack. Rig joints lie inside the shoulders, elbows, wrists, hips, knees and ankles. Earlier states that I rejected and fixed: sleeve heads standing up as horns beside a narrow trunk, a drop-shaped nose and V jaw, torn seams at knee and hem, a band cutting into the jacket.
- 1:50 `parked-upright`: truck upright on plate and wheels, right hand on the bar, left arm hanging, all joints closed. 1:100 `push-walk` at 0.3 s: stride reads, hands stay on the bar, knees and elbows closed.
- 1:50 thermal with cut: nested jacket shells visible in the cut, regions separate and complete, no shell pokes out. 1:50 and 1:100 structure: a recognisable skeleton with curved spine, inclined ribs, girdles and long bones inside the ghost, truck and cartons as members.
- 1:100, 1:200, 1:500 body sheets: the same figure and truck in fewer, simpler masses, proportions unchanged, recognisable at a glance.
- `r3-2d` of the catalog asset: 1:50 default pose and `push-walk` at 0.3 s, 1:500 default pose: side, front and plan are the views of the model; the walk animates correctly in all three. 1:100 and 1:200 2D sheets and the 1:100 thermal, 1:500 structure and `volumes-render` overlay sheets were rendered but not opened; for them I rely on the numbers above.

## 7. Still imperfect

- **Four area lines stay red** (`boots` +123 % / +109 %, `gloves` +35 %, `cap` +33 % at 1:50). The declared values are skin shares of feet, hands and scalp; the honestly modelled boots, fists and cap are larger. C18 forbids changing them; filed in `author-requests.md` ("r3 humans/courier-with-hand-truck") with measurements. Nothing was distorted to meet the numbers. The state before round 3 failed eight such lines.
- `jacket` -29 % and `trousers` +27 % pass narrowly: the trousers seat over the pelvis is not a thermal solid because the physiological partition books the trunk on the jacket region.
- The default pose now holds the truck at 45 degrees instead of 22: a deliberate change (balance of this load), but it changes the look of the asset and its length (1.15 -> 1.58 m).
- Garments are rigid per bone: in the walk the thigh still passes close under the jacket hem, and elbows and knees are nested sleeves, not cloth folds.
- The face is simple (no lips, eyes hidden under the brim); the cap is the asset's "knitted cap with brim", a hybrid I kept because the thermal layer names it.
- 2D outlines of trunk, head, legs and boots are traced from the solids and reduced (jacket 26 points at 1:50); they are clean but not hand-inked curves. In the front and plan line views the product draws the cartons and arms as merged outlines.
- 1:500: the drawn stack is one block of mean width and the tyres are 100 mm, so front and plan IoU are 0.95 and 0.94.

## Round 5 handcraft (2026-11-21)

Scripts (ticket folder): `r5-courier-with-hand-truck-forms.py` (the garment forms, one table per segment, stated once and picked per scale), `-s50.py` … `-s500.py` (the four scales; head, hands, boots, truck and lists as in round 3 except where named below), `-build.py`, `-draw2d.ts` + `-decisions2d.json` (2D redraw), `-ship.sh` (build → `r3-courier-roots.ts` → redraw → validate the candidate → `--apply`: `r3-courier-apply.py`, `volumes-insert.py`, product check, full validation), `-measure.ts` (rig and slice measurements), `-ends.ts` (print suitability per band, pose and view at any denominator). Output: `🗑️generated/r5-courier-with-hand-truck/` (`before/`, `after/`, `check.txt`, `measure.txt`).

### Reference (1.82 m man in a softshell jacket, cargo trousers, EU 45 safety boots)

| quantity | reference | source |
|---|---|---|
| stature (shod) / hip / knee / ankle joint | 1.82 / 0.52-0.53 H / 0.285 H / 0.039 H + sole | Winter 2009 tab. 4.1, Drillis & Contini |
| shoulder / elbow / wrist joint height | 0.818 / 0.63 / 0.485 H | same |
| upper arm / forearm / hand | 0.186 / 0.146 / 0.108 H | same |
| head chin to vertex | 0.232 m (H/7.5 canon 0.243) | ANSUR II male mean (Gordon 2014), DIN 33402-2 |
| bideltoid breadth | 0.49-0.50 nude, about 0.52 in a jacket | DIN 33402-2, ANSUR II |
| chest / waist / hip breadth x depth (nude) | 0.33 x 0.24-0.25 / 0.30 x 0.22 / 0.35-0.36 x 0.24; + 2 x 12 mm jacket and fleece, + 15 mm trousers | DIN 33402-2 |
| girths: mid thigh / calf / upper arm (nude) | 0.53-0.56 / 0.38 / 0.31 m; + 40 mm ease of work trousers or jacket sleeve | DIN 33402-2 |
| shoulder slope neck base to acromion | about 18-20 degrees (drop 35-40 mm over 0.13 m) | anatomical canon |
| safety boot EU 45 | 0.31-0.325 m long, 0.105-0.11 m wide | EN ISO 20345 boots, own knowledge |
| hand truck | fetra B1125L: 1150 x 570 mm, toe plate 320 x 250, wheels 260 x 85 | manufacturer data (round 3) |

### Model against reference (1:50 body, rest pose; `measure.txt`)

| quantity | model | reference | deviation |
|---|---|---|---|
| hip / knee / ankle joint | 0.957 / 0.514 / 0.080 | 0.946-0.965 / 0.519 / 0.071 + sole 0.010 | in range / -1.0 % / 0 |
| shoulder / elbow / wrist joint | 1.491 / 1.155 / 0.889 | 1.489 / 1.147 / 0.883 | +0.1 / +0.7 / +0.7 % |
| upper arm / forearm / hand | 0.337 / 0.265 / 0.194 | 0.339 / 0.266 / 0.197 | -0.6 / -0.4 / -1.5 % |
| head chin to vertex | 0.230 | 0.232 | -0.9 % |
| bideltoid over the jacket | 0.508 | 0.52 | -2.3 % |
| jacket chest breadth x depth | 0.360 x 0.268 | 0.354 x 0.264-0.274 | +1.7 % / in range |
| jacket waist breadth x depth | 0.328 x 0.238 | 0.324 x 0.244 | +1.2 / -2.5 % |
| hip over jacket hem | 0.384 | 0.38-0.39 | in range |
| trousered thigh girth (mid) / calf girth | 0.579 / 0.429 | 0.57-0.60 / 0.42 | in range / +2.1 % |
| jacket upper arm girth | 0.348 | 0.35 | -0.6 % |
| boot length | 0.323 | 0.31-0.325 | in range |

### Grades (body / thermal / structure)

| scale | before (round 4 state) | after |
|---|---|---|
| 1:50 | C+ / B+ / A- | A- / A- / A- |
| 1:100 | C / B / B+ | A- / B+ / B+ |
| 1:200 | C / B / B | B+ / B+ / B |
| 1:500 | C+ / B / B | B+ / B / B |
| poses 1:50 (waiting, tilted-back, parked-upright, walking-contact) | B (knee steps, hem poke-through) | A- (closed at knee, elbow, hem) |

Defects seen before (sheets `before/`): a box jacket with straight sides from hem to chest and a flat shoulder plateau ending in a corner, sleeve heads as humps with a flat disc on top, reflective bands standing 5-10 mm proud as belts, the thigh tops piercing the jacket hem at the side, a 6 mm step at every knee (thigh hem wider than the shank top), the cargo pocket a box slab, the 1:500 cap a bucket, the 1:200 knees torn open (shank started below the knee) and the round-4 source (1:500 handle bar) unspliced and unprintable.

### Changes

- Garment forms rebuilt from the reference (`-forms.py`): jacket of 13 sections (hem 0.194 half breadth, waist 0.164, chest 0.180 x 0.134 deep with the chest ahead of the back, armpit 0.186, a trapezius slope from the neck base at 0.078 to the acromion at 0.200 over 70 mm of height, exponents 2.4 at the hem to 2.0 at the neck: no box corners); sleeve of 10 sections with a domed deltoid cap that starts 42 mm above the joint inside the yoke, deltoid 0.063 at 40 mm, biceps forward, 0.047 at the elbow; forearm flattened (0.047 x 0.044 to 0.036 x 0.031); thigh oval with the quadriceps forward and valgus to the knee, its hem identical to the shank section at the same place (no knee step), calf bulge behind (0.064 x 0.071 at 130 mm); trousers seat rounded (exponent 2.3); cargo pocket 12 mm proud with rounded edges; bands interpolated on the jacket and only 2 mm proud; cap 5 mm higher at its rim (cap still a beanie with brim); ears 2 mm closer to the head.
- Every coarser scale picks fewer rows of the same tables (1:100 10/7/4/5/5 sections, 1:200 7/5/3/4/4, 1:500 6/4/2/3/4), so the proportions are the same at every scale; the 1:200 and 1:500 shank now starts inside the thigh (closed knees in every pose); 1:500 cap a dome; 1:500 handle bar 112 mm and boot heel and toe 72-88 mm so that the band prints.
- 2D (E1, `-draw2d.ts`): all scales and views retraced from the new bodies; zip line on the new jacket front; 1:200 rails 44 mm wide (print at 1:350); 1:500 rails one 80 mm strip in the elevation and one 365 mm frame panel in front (with the handle) and plan, starting where the stack covers its foot; decisions of 1:200 and 1:500 updated (en, de). The catalog was INVALID under the print rule (four range-end findings) and is now VALID.
- No rig change: joints, segment lengths and the truck geometry measured within 1.5 % of the reference. The single torso bone stays (pelvis tip to C7, through the vertebral bodies); the curvature of the spine (lumbar lordosis, thoracic kyphosis, dorsal position) is in the structure list, which was already right.

### Checks

`r3-validate.ts humans/courier-with-hand-truck --volumes`: schema 0, validateAsset 0 (print clean at every band and range end), checkVolumes 2. `volumes-check.ts --product`: every rule ok except `volumes-thermal-coverage` boots at 1:50 (0.271 against 0.122 m², +122 %) and 1:100 (+109 %): honest EU 45 safety boots against the skin share of the feet; C18 forbids the change, listed in `area-review.md` since round 3 (and again below). The cap (0.325 in round 3, over the limit) passes after its rim was raised 5 mm; trousers 1:100 pass at 0.299 after the 1:100 thigh dropped its widest row.

### Remaining

- Boots area (C18/C27, area reviewer).
- Garments are rigid per bone: no cloth folds; elbows and knees are nested lofts, closed but without creases.
- Plain face; the cap is the asset's knit cap with a brim.
- 1:500 structure is a stick skeleton of 75 mm bars (the feature floor), with the curved dorsal spine, both girdles and the skull.
- Opened: before and after sheets of every scale (body, structure, thermal cut), the three poses at 1:50, 2D 1:50 and 1:200; 1:100/1:500 2D only through the validator.

The sack truck now has a turned 578 × 20 mm steel axle, with two machined snap-ring seats at the fine scales. The formerly solid wheel hubs have 20.4 mm through bores, giving 0.2 mm of radial clearance. All four physical scales show the separate steel rims and rubber tyres at the original 485 mm wheel spacing. The steel retaining rings fit the end grooves and their true 1.1 mm section appears in the fine drawing. The thermal and structural models refer to these parts. An independent Three.js section check measured each scale. The whole drawing, feature, cross-scale and four-scale model checks passed with the fine ring drawing in place.
