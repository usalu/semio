# Round 3 — `humans/seated-office-worker`

State (2026-11-20): written to the catalog (rig, anchors, poses, clip keys, dimensions, materials, thermal areas, structure assumptions, provenance note, 2D geometry and drawing decisions of all four scales, `volumes`) and the manifest row (height, width). `r3-validate.ts humans/seated-office-worker --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/seated-office-worker --product`: `FAIL: 3 measurement(s) over 2 rule line(s)` — every rule green at every scale except `volumes-thermal-coverage` on two body-model areas the author may not change (C18): `trousers` 1:50 +64 % and 1:100 +74 %, `shoes` 1:50 +48 % (request to the area reviewers in `author-requests.md`, 2026-11-20). Print suitability (office-laser) holds at every scale and at both ends of every valid range.

Resumed from an earlier author who was cut off after writing four scripts (`-kit`, `-ref`, `-roots`, `-sheet`, adapted from the seated guitarist); nothing had been written to the catalog or the source and the asset was valid. I kept the kit and the sheet copy, kept the structure of the roots script and corrected its numbers and solver (see 3), and wrote everything else.

Scripts (ticket folder): `r3-seated-office-worker-ref.ts` (reference numbers), `-roots.ts` (rig, anchors, poses solved with the product's `solveChain`, clip keys, texts), `-frame.ts` (rest frames, world-to-bone conversion, aiming of the star arms), `-kit.ts` (constructors), `-s50.ts` (1:50 body, every section by hand), `-coarse.ts` (1:100, 1:200, 1:500 bodies, each stated anew), `-lists.ts` (thermal, structure, root areas), `-texts.ts` (3D and 2D decisions, provenance), `-build.ts` (candidate document and source), `-draw2d.ts` (2D redraw, ruling E1), `-write.py` (catalog members and manifest row), `-run.sh` (draw, build, validate), `-look.sh`, `-sheet.ts`, `-suit.ts`, `-probe.ts`.

## 1. Reference

| quantity | reference | used | source |
|---|---|---|---|
| stature / mass | 1.78 m, 76 kg | unchanged | asset `provenance`, `thermal.body` |
| hip joint above seat surface (sitting) | 0.085 to 0.10 m (ischial tuberosities about 0.09 below the joint, 1 cm foam set) | hip 0.570 m, seat top 0.485 m, buttocks set 1 cm into the cushion | Winter 2009; ANSUR II seated heights (my knowledge) |
| shoulder joint above hip joint | 0.818 H − 0.52 H = 0.53 m | 0.53 m (rest), 0.042 m ahead of the spine line | brief; Winter 2009 |
| C7 above hip joint | 0.86 H − 0.52 H = 0.60 m | 0.605 m | de Leva 1996 |
| head: C1 joint to vertex / chin to vertex | ≈0.13 / H/7.5 = 0.237 m | 0.132 / 0.24 m | brief; ANSUR II male |
| shoulder joint spacing / bideltoid | biacromial 0.245 H = 0.436 (ANSUR II 0.40) minus 2 × 2.5 cm; bideltoid 0.48 m | 0.352 m / 0.48 m in the shirt | ANSUR II male |
| hip joint spacing / seated hip breadth | 0.17 to 0.18 / 0.38 to 0.41 m | 0.176 / 0.41 m in trousers | Bell et al. 1990; ANSUR II |
| upper arm / forearm / hand | 0.188 H / 0.145 H / 0.108 H = 0.335 / 0.258 / 0.192 | 0.33 / 0.26 / 0.19 (bones kept) | Winter 2009 |
| thigh / shank / ankle height | 0.245 H / 0.246 H / 0.039 H = 0.436 / 0.438 / 0.069 | 0.435 / 0.438 / 0.092 in a shoe with a 23 mm sole | Winter 2009 |
| chest breadth × depth, waist, thigh, calf | 0.33 × 0.24, waist 0.31 × 0.23, thigh girth 0.58, calf 0.38 m | shirt 0.34 × 0.26 at the chest, 0.31 × 0.22 at the waist, trouser thigh 0.19 × 0.18, leg 0.115 × 0.125 | ANSUR II male (my knowledge) |
| foot / shoe | 0.152 H = 0.27 m; oxford EU 43 0.29 × 0.10 m, heel 25 mm | shoe 0.29 × 0.105 m, heel block 25 mm, sole 9 mm | my knowledge |
| task chair | EN 1335-1 type A: seat height 420–510, depth 380–440 usable (pan 470), width ≥ 400, armrest 200–250 above seat, backrest 360+ above seat; base Ø 0.6–0.7 m; EN 12529 twin castors Ø 50 mm | seat top 0.485, pan 0.47 × 0.49, armrests 0.195 above seat 0.53 apart, backrest pad 0.47 tall to 1.0 m, base Ø 0.61, castor Ø 50 twin | EN 1335-1, EN 12529 (values from my knowledge) |
| desk | EN 527-1: height 720–750 fixed, depth 800 | 780 × 600 × 720, 25 mm board, 35 mm square tube legs, 60 × 20 rails, open front | EN 527-1 (my knowledge); depth and width kept from the asset |
| notebook | 15.6 inch business notebook ≈ 357 × 233 × 19 mm, lid 6.5 mm, 1.8 kg | 357 × 235 × 19 mm, lid 7.5 mm, 1.3 + 0.5 kg | manufacturer ranges (my knowledge, no lookup) |
| segment masses, centres | de Leva 1996 | unchanged except trunk centre scaled to the new trunk length | asset references |

No web search was made; "my knowledge" values are not verified against a datasheet.

## 2. Audit (old asset)

Rig: trunk 0.465 m from a lumbosacral joint on the hip axis (no offset behind the hip joints), neck 0.10 and head 0.18 m (vertex 0.845 m above the hip); shoulder joints only 0.29 m apart (`z` ±0.145) and on the spine line; **both thighs on the symmetry plane** (no `z`: the hip joints coincided, the legs were spread only by their solids); shank 0.425 m (−13 mm); foot bone 0.186 m at 66° so the ankle sat 0.077 m high with no sole under it; star arms "projected" as bones of 0.093 / 0.243 m in the side plane with the castors on the arm tips in that plane, i.e. the side and back castors of a five-star base were drawn and modelled on the centre line; hub at 25 mm height; gas lift 0.42 m; notebook 0.32 m deep with the hinge at 0.74 (a 15 inch notebook is 0.23 to 0.25 m deep); a desk "end panel" bone at the far edge while the model had four legs.
Poses: forearms with up to ±43° abduction at the elbow (a hinge joint), the typing hands without pronation; the hand behind the head was 34 mm off its goal.
Anchors: mouth and nostrils inside the face, crown inside the hair, laptop vent in the base.
2D: hand-drawn constructed views, a single leg in the elevation, arms as tubes, mitten hands, castors on the centre line; front and plan did not agree with any one body.
Solids (old 71 / 96 / 63 at 1:50): box-like torso with ball shoulders, tube limbs, mitten hands, a hub revolve filled by implied closure (kernel-traps 163–170), self-intersecting lofts at neck, head, collar and shoe toes (kernel-traps 531–550), a structure of body-like member lofts.

## 3. Root fixes (old → new)

| member | old | new |
|---|---|---|
| `torso` x / y / angle / length / com | 0.1 / 0 / 0 / 0.465 / 0.2119 | 0.1 / 0.035 / −2° / 0.505 / 0.2301 |
| `neck` angle / length | −8° / 0.10 | −16° / 0.118 |
| `head` angle / length / com | 8° / 0.1801 / (0.0742, 0) | 18° / 0.132 / (0.026, −0.016) |
| `arm-l/r` x / y / z / angle | 0.42 / 0 / ±0.145 / 180° | 0.432 / −0.042 / ±0.176 / 182° |
| `thigh-l/r` z / angle | none / −93.9° | ±0.088 / −95.28° |
| `shank-l/r` length / angle | 0.425 / −86.1° | 0.438 / −84.72° |
| `foot-l/r` length / angle / com; `toe` angle | 0.1858 / 66.2° / 0.0791; 23.8° | 0.1675 / 56.69° / 0.0713; 33.31° (ankle 92 mm above the floor in the shoe) |
| `hip-lift` y; `chair-column` length; `chair-seat` x / length | 0.125; 0.42; 0.42 / 0.46 | 0.17; 0.30; 0.30 / 0.235 |
| `chair-base` y / length; star arms | 0.025 / 0.1; front 0.3, side 0.093, back 0.243 in the side plane | 0.10 / 0.05; arms to sockets at 0.30 m radius on true yaw 0°, ±72°, ±144° (projected bones 0.30 / 0.12 / 0.24 m at their slope) |
| `chair-castor-*` | on the arm tips in the side plane | under their sockets, 20 mm trail inward, wheel centre 25 mm high, side and back castors at z ±0.266 / ±0.165 |
| `chair-backrest` x / length / com | −0.23 / 0.62 / (0.31, 0) | −0.21 / 0.60 / (0.36, 0) |
| `laptop` x / y / length / mass; `screen` length / mass | 0.74 / 0.74 / 0.32 / 1.2; 0.215 / 0.5 | 0.665 / 0.7335 / 0.235 / 1.3; 0.226 / 0.5 |
| `desk-top` length / mass; `desk-leg` x / mass | 0.78 / 11; 1.02 / 5 | 0.78 / 7.6; 0.67 / 6.2 (frame of four legs and rails) |
| bone titles | "projected" arms, "end panel", "fragment" | true arms, frame, notebook, desk 780 × 600 |
| anchors | seat (−0.0847, 0.0318), backrest (0.2178, 0.1271), heels (−0.0109, −0.0868), mouth (−0.0051, −0.0801), nostrils (0.0301, −0.1041), crown (0.1748, 0), vent (0.03, 0) | seat (−0.095, 0.025), backrest (0.2, 0.092), heels on the heel of the shoe, mouth (−0.06, −0.104), nostrils (−0.036, −0.10), crown (0.142, 0.008), vent (−0.008, 0.004) on the rear face of the base |
| poses | arms by hand, elbows abducted | all three solved in 3D (shoulder angle, abduction and twist, elbow flexion with at most ±8° carrying angle, wrist): typing hands pronated, wrists 30 mm over the palm rest, finger tips on the home row; reading: left forearm on the thigh, right hand on the touchpad; leaning back: left palm on the occiput with the elbow out (multi-start solve, 1.5 mm), right forearm in the lap. Legs solved so the feet stay flat (heel and ball on the floor). |
| clip keys | old arm angles | shifted by the same pose deltas (`typing` loop), `settle-back` "she" → "he" |
| `structure.assumptions` | "notebook is a 0.6 kg bone mass on the far hand" | notebook 1.8 kg and desk 13.8 kg stand on their own |
| `thermal.regions` `chair-fabric` / `castor-rubber` | 0.40 / 0.036 | 0.51 / 0.053 (C16, within 1.5: cover on seat top, edges, backrest front and edges with the glued faces as C25 b section faces; ten tyre rings with their bores on the wheel cores) |
| `materials` | — | `bone` added (structure) |
| `dimensions` | 1.411 / 1.291 / 0.60 / 111.65 kg | 1.413 / 1.278 / 0.61 / 109.55 kg (bbox of the 1:50 body in the typing pose; mass = rig sum) |
| manifest row | 1.411 / 1.291 | 1.413 / 1.278 |
| `provenance.notes`, all `representations[*].decisions` | old construction | rewritten (en, de) |
| `representations[*].geometry` | 207 / 136 / 114 / 70 shapes | redrawn: 289 / 192 / 156 / 114 (1:50 / 100 / 200 / 500) |

2D redraw (ruling E1, `-draw2d.ts`): each shape is the outline of the body solids of its part in the typing pose; convex single solids (boxes, ellipsoids, capsules) by the exact convex hull, all others traced at a third of the tolerance and Douglas-Peucker at 0.9 / 1.8 / 3.5 / 7 mm. Furniture, trunk, neck, head and legs (pitch-only in every pose) are flat outlines solved into their bone frames through the product's placement (`inverse`, every residual under 0.01 mm); sleeves, forearms and wrists are revolved profiles, hands thin plates, so the spread arms of every pose project. Layer order by depth along the view. Strokes by hand: button placket (1:50 two lines, 1:100 one) and breast pocket (1:50) in the front view. One id, tone, region and material per part in all views.

## 4. What was modelled (body / thermal / structure)

- **1:50 — 131 / 131 / 170.** Chair: polyamide hub (closed revolve), five star arms as tapered lofts aimed along their true yaw, sockets, ten castor wheels (polyamide core + rubber tyre ring), hoods, steel stems, telescopic column cover, chrome column, synchro mechanism, paddle, seat shell, cushion with waterfall front, backrest pad with lumbar curve on its shell, steel J-bar (extruded profile), T-armrests (bent posts, pads). Desk: board, four legs with glides, two side rails and a back rail. Notebook: base, keyboard field, touchpad, hinge barrel, lid, display. Man: trouser seat, belt and buckle, shirt (11 hand rows from tucked hem to neck), collar band, placket, buttons, pocket, neck, head (12 rows chin to vertex), nose, mouth, eyes, brows, ears, hair with hairline, sleeves with deltoid caps, forearms, cuffs, wrists, palms, thenars, four three-part fingers and a thumb per hand curled on the keys, thighs, trouser legs, shoes (upper, heel block, sole, toe cap, toe sole). Thermal: nested shells (vest/shirt, briefs only under the seat, socks only in the uppers, foam/fabric), face ellipsoid and open neck skin for `face-skin`, glued faces as section faces, four outlet patches (nostrils, lips, crown, rear of the base). Structure: full skeleton (see the 1:50 decisions) with 19 joints and masses at the de Leva centres, chair and desk members by reference, desk frame mass on its four legs, floor pads under nine contacts.
- **1:100 — 67 / 68 / 128**, **1:200 — 52 / 56 / 87**, **1:500 — 38 / 42 / 65**: authored anew per scale (`-coarse.ts`), what was merged is in each scale's decisions (mitten hands at 1:100 and 1:200, one shoe form, twin wheels as one wheel, no shell at 1:200, slabs at 1:500, 77 mm star arms, castor balls; skeleton with 5 / 3 / 0 rib hoops plus a rib cage loft at 1:500; 1:500 chair column, seat and backrest as members inside their slabs).

Final check: `volumes-check.ts --product` silhouette IoU elevation / front / plan 1:50 0.995 / 0.995 / 0.998, 1:100 0.994 / 0.995 / 0.995, 1:200 0.987 / 0.992 / 0.999, 1:500 0.984 / 0.992 / 0.991; bounds, feature, monotonic, cross-scale, structure coverage, bones, copied green at all scales; thermal per region at 1:50: face-skin +17 %, hand-skin 3 %, shirt −23 %, laptop −11 %, desk +14 %, chair-plastic 4 %, chair-fabric 0 %, chair-steel 2 %, castor-rubber 0 %, trousers **+64 %**, shoes **+48 %**; at 1:100 trousers **+74 %**, all others within 30 %; totals 1:200 +0.8 %, 1:500 +5 %. Line: `FAIL: 3 measurement(s) over 2 rule line(s)`.

## 5. What I saw (Read on the PNGs)

- `r3-sheet` 1:50 body, six views (catalog state) and the large right side and front: a man of normal proportions typing; side view: S-shaped back free of the backrest in the forward lean, head bowed to the screen, curled fingers on the keys, wrists over the palm rest, thighs level on the cushion, knees under the open desk front, shins vertical, feet flat in oxfords; chair with five arms sloping to castors standing under their sockets, column in its cover, J-bar behind the backrest. Front: shoulders slope into the deltoid caps (the first state had ball shoulders; the shirt shoulder was widened and the caps lowered), collar around the neck, face with brows, eyes, nose, mouth, ears.
- 1:50 three-quarter (turn 60/75): belt, placket, buttons, pocket, armrests, notebook; reads at first sight.
- 1:50 `leaning-back` six views and turn 120: backrest tilted with the trunk and in contact, left palm on the back of the head with the elbow out, right forearm towards the lap; `typing` clip at 1.5 s: arms keep their twist and abduction through the loop.
- 1:50 structure six views: skull, mandible, spine with curvature, rib hoops, scapulae, pelvis, long bones, finger bones on the keys, chair and desk frames; thermal cut six views: nested shells inside the garments, face and neck skin, nothing outside the body.
- 1:100 body six views, 1:200 body six views, 1:500 body six views and structure: the same figure in fewer masses, still a typist at a desk; 1:500 floor pads read as balls under the desk legs.
- `r3-2d` 1:50 typing and leaning-back (candidate), 1:500 typing: clean line and solid drawings; side 1.42 m, plan 0.61 m wide; in the leaning pose the hand plate behind the head reads as a ragged shape in the side elevation.
- Rendered, not opened in the final state: 1:100 thermal and structure sheets, 1:200 structure, 1:500 2D after the last seat change (the change was 8 mm at the hips).

## 6. Still imperfect

- `volumes-check` stays red on `trousers` and `shoes` (body-model areas, C18); request filed (C27).
- Body grade as I see it: 1:50 A− (the thigh taper seen end-on in the front view shades as dark zig-zags at the knees, a rendering of grazing faces of the overlapping thigh and knee lofts; no cloth folds; superellipse sections, no separate pectorals or scapulae under the shirt; hair a 12 mm cap with a step at the hairline), 1:100 A−, 1:200 B+, 1:500 B+ (generalised slabs). Chair and desk A−: the backrest is flat in plan (no concave curve), the star arms are solid tapered sections instead of ribbed U-profiles.
- The left hand behind the head is a plate in 2D; in the side elevation of that pose it is a ragged outline.
- Chair, desk and notebook dimensions are from my knowledge of the standards and typical products, not from a looked-up datasheet. Desk depth and width (780 × 600) were kept from the asset; a full EN 527-1 work desk would be wider.
- Tests and fixtures that embed this asset were not regenerated or run.

## Round 5 handcraft

2026-11-21. Brief `r5-brief.md`. The round-3 scripts (`r3-seated-office-worker-*.ts`) still reproduced the catalog byte for byte at the start, so the model was reworked in them at the root (`-s50.ts`, `-coarse.ts`, `-lists.ts`); new round-5 tools shared with the seated reader: `r5-seated-measure.ts`, `r5-seated-look.sh`, `r5-seated-montage.ts`.

### Reference table (modelled to) and model vs reference, 1:50, rest frames (`bun r5-seated-measure.ts office --rest`)

| quantity | reference | model | deviation | source |
|---|---|---|---|---|
| sitting height (seat top to vertex) | 0.925 | 0.940 (round 4: 0.928) | +1.6 % | DIN 33402-2 m P50 0.907 at 1.75 m, scaled to 1.78 m |
| head height chin to vertex (H/7.5) | 0.237 | 0.238 (round 4: 0.249, +4.9 %) | +0.4 % | brief; ISO 7250 m P50 |
| head breadth over the hair | 0.160 | 0.159 (round 4: 0.171, +6.9 %) | -0.6 % | ISO 7250 m P50 0.155 + hair |
| shoulder joint above the seat top | 0.615 | 0.615 | 0 % | 0.818 H - 0.52 H + hip 0.085 above the seat |
| shoulder joint spacing | 0.352 | 0.352 | 0 % | biacromial 0.40 - 2 x 0.024 |
| bideltoid breadth in the shirt | 0.480 | 0.472 (round 4: 0.491) | -1.7 % | DIN 33402 m P50 0.477 |
| upper arm / forearm | 0.335 / 0.258 | 0.330 / 0.260 | -1.5 / +0.8 % | Winter 0.188 H / 0.145 H |
| hand length, fingers straightened | 0.192 | 0.190 | -1.0 % | 0.108 H; knuckle 0.096 + phalanges 0.045 / 0.027 / 0.022 (curled on the keys in the model) |
| hip breadth seated in trousers | 0.410 | 0.413 | +0.7 % | DIN 33402 m P50 0.375 + trousers |
| thigh clearance | 0.160 | 0.161 (round 4: 0.169, +5.5 %) | +0.4 % | DIN 33402 m P50 0.155 + cloth |
| knee height (floor to knee top) | 0.585 | 0.573 | -2.1 % | DIN 33402 m P50 0.555 barefoot + 23 mm sole + cloth |
| knee joint height | 0.530 | 0.530 | 0 % | Winter 0.285 H + 23 mm sole |
| buttock-knee length | 0.620 | 0.628 (round 4: 0.643, +3.7 %) | +1.4 % | DIN 33402 m P50 0.605 + cloth |
| calf breadth in trousers | 0.120 | 0.122 | +1.7 % | ANSUR II m 0.11 + 2 x 5 mm cloth |
| shoe length | 0.290 | 0.298 | +2.8 % | 0.152 H foot + 0.02; EU 43 |
| ankle joint height in the shoe | 0.092 | 0.092 | 0 % | Winter 0.039 H + 23 mm sole |

Every quantity within 3 % at 1:50. Chair, desk and notebook keep the round-3 table (EN 1335-1, EN 12529, EN 527-1; unchanged). The rig was audited against the table and needed no change.

### Grades (my own look at every sheet; before = catalog state of round 4)

| scale | body before | body after | structure before / after | thermal before / after |
|---|---|---|---|---|
| 1:50 | B (hair a mushroom helmet with a brim 1 cm proud of the temples; sleeve caps as balls on the shoulders with a seam; equal knee domes of thigh and shank z-fighting into jagged patches; equal elbow domes; chin 8 mm long, thighs 8 mm thick, knees 15 mm far forward) | A | A / A (mandible raised with the chin, coverage 7.5 mm) | A / A |
| 1:100 | B (same hair, caps, knees) | A | A / A | A / A |
| 1:200 | B (helmet hair, ball caps) | A- | A- / A- | A- / A- |
| 1:500 | B- (helmet hair; flat discs on top of the upper arms standing out of the shirt; ragged seam between trouser seat and thigh roots) | A- | A- / A- | A- / A- |
| poses 1:50 (typing, reading-screen, leaning-back) and clip `settle-back` at 4.2 s | A- | A | | |

### Changes

- Head: lower face shortened (chin -0.108 -> -0.100 on the head bone, same share at every scale), skull breadth 0.151 m; mandible of the structure raised with it.
- Hair restated at every scale as short men's hair: 7 mm on the crown, 8 to 10 mm at the back, 3 to 6 mm at the temples, down to the nape behind the ears, hairline crossing the forehead about 6 cm above the brows at a clear angle; no brim.
- Shoulders: the sleeve starts with five stated rows leaning in under the shirt shoulder (trapezius slope into the deltoid) instead of a quarter-sphere round the joint, at all scales; at 1:500 the shirt shoulder gets one more row so the arm top stays inside it.
- Elbows: forearm elbow dome 7 mm smaller than the sleeve's (olecranon one surface) at 1:50, 1:100, 1:200.
- Thighs: 4 mm thinner at the root (thigh clearance), knee 15 mm shorter (buttock-knee), knee formed by the thigh end with the shank top inside it, at every scale; at 1:500 the thigh root sits inside the trouser seat.
- 2D of all four scales redrawn from the new bodies (`r3-seated-office-worker-draw2d.ts`, ruling E1); dimensions and manifest height 1.413 -> 1.409 m.

Final: `r3-validate.ts humans/seated-office-worker --catalog-only` VALID (schema 0, validateAsset 0, print suitability at every scale and range end). `volumes-check.ts --product`: `FAIL: 3 measurement(s) over 2 rule line(s)`, only `volumes-thermal-coverage` on the body-model areas the author may not change (C18, filed in round 3): trousers +63 % (1:50) and +73 % (1:100), shoes +49 % (1:50). Silhouette, bounds, feature, monotonic, cross-scale, structure coverage, bones, copied green at every scale.

Looked at (Read): body of the four scales (montage `🗑️generated/r5-seated/office/v2-body.png`, single front and three-quarter views after each change), structure and thermal cut of the four scales, the three poses and the clip frame, the product 2D of all four scales.

### Remaining issues

- The black jagged rings round the knees in the front views are the sheet renderer inking grazing surfaces (thigh tops within 8 degrees of the view) as depth steps; the geometry is closed and smooth (`tool-requests.md` 2026-11-21).
- No cloth folds; the shirt is one smooth mass with placket, buttons and pocket. Chair star arms are solid tapered sections, the backrest flat in plan (round-3 note stands).
- 1:500 keeps 75 mm logs for star arms and shoes (feature rule).
- `trousers` and `shoes` stay red in `volumes-thermal-coverage` (body-model areas, C18; filed in round 3).
