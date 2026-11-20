# Round 3 — `humans/leaning-person`

State: root fixes, 2D redraw and volumes are in the catalog asset (`r3-leaning-person-commit.sh`). `r3-validate.ts humans/leaning-person --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/leaning-person --product`: `FAIL: 6 measurement(s) over 2 rule line(s)` — only `volumes-thermal-coverage` at 1:50 and 1:100 for `arm-clothing`, `leg-clothing`, `shoes` (honest garment surfaces against declared skin shares, filed in `author-requests.md`, "2026-11-20 r3 humans/leaning-person"). Every other rule is green at every scale. Manifest row `leaning-person` updated (heightM 1.74, widthM 0.54).

No earlier round-3 work existed for this asset (no `r3-leaning-person-*` files, catalog as of 2026-11-17); everything is from this run.

Scripts (ticket folder): `r3-leaning-person-roots.ts` (rig, anchors, pose solving, clips, texts, dimensions, candidate document, `--fit`, `--commit`), `-data.ts` (segment lengths, phone in hand), `-kit.ts` (station helpers, anatomical frame `sa`/`pa`, copied from the walking-commuter kit and adapted), `-s50.ts` (every section of the 1:50 body, thermal, skeleton), `-coarse.ts` (1:100, 1:200, 1:500 stated anew), `-build.ts` (writes `volumes/humans/leaning-person.json`), `-draw2d.ts` (2D redraw), `-texts.json` (decisions), `-dimensions.json`, `-fmt.ts` (catalog JSON style), `-io.ts`, `-fk.ts`, `-check.ts` (candidate check), `-sheet.ts` (r3-sheet on the candidate, `--no-wall`, `--crop y0:y1`), `-run.sh` (rebuild + validate), `-commit.sh` (catalog write + splice).

## 1. Reference (slim man, 1.72 m barefoot, 66 kg; 21 mm sneaker sole)

| quantity | reference | asset now | source |
|---|---|---|---|
| stature shod / with beanie | 1.72 + 0.021 sole; beanie +12 mm over the vertex | vertex 1.741 m standing; 1.736 m bbox in the 11-degree lean | asset summary; sneaker data |
| hip joint height | 0.52 H = 0.894 m barefoot | 0.898 m barefoot (straight leg), 0.919 shod | Winter 2009 table 4.1 (asset reference) |
| knee | 0.285 H = 0.490 m | 0.477 m barefoot (shank 0.4105 kept, 13 mm low) | Winter 2009 |
| ankle | 0.039 H = 0.067 m barefoot | 0.088 m shod | Winter 2009 + sole |
| C7 (neck base) | 0.855 H = 1.471 m | 1.471 m barefoot | de Leva 1996 (asset reference) |
| shoulder joint | 0.818 H = 1.407 m | 1.416 m barefoot | Winter 2009 |
| upper arm / forearm / hand | 0.186 / 0.146 / 0.108 H = 0.320 / 0.251 / 0.186 m | 0.318 / 0.252 / 0.186 m | Winter 2009 |
| thigh / shank | 0.245 / 0.246 H | 0.4202 / 0.4105 m (kept) | Winter 2009 |
| head chin–vertex, length, breadth | H/7.5 = 0.229; 0.195; 0.152 m | 0.229 / 0.197 / 0.15 m (skull), beanie 0.217 x 0.17 | brief; ANSUR II (Gordon 2014, added to references) |
| face landmarks above menton | stomion 0.048, subnasale 0.07, nasion 0.12 m | 0.048 / 0.07 / 0.12 m | anthropometric face proportions |
| shoulder joint spacing, bideltoid | joints 0.33–0.36; bideltoid 0.44–0.46 nude | 0.35 m (kept); 0.47 m over the sleeves | ANSUR II |
| chest / waist / hip breadth x depth (nude) | 0.30 x 0.22 / 0.27 x 0.19 / 0.34 x 0.22 m | hoodie 0.344 x 0.261, 0.342 x 0.247, jeans seat 0.348 x 0.225 m | ANSUR II, slim build, garment ease 15–25 mm per side |
| neck | 0.115 x 0.12 m | 0.12 x 0.12 m | ANSUR II |
| thigh / calf / ankle girth | 0.53 / 0.36 / 0.22 m | jeans 0.17 x 0.18, 0.116 x 0.124, sock 0.07 x 0.084 m | ANSUR II |
| hand length / breadth / thickness | 0.186 / 0.085 / 0.028 m | 0.186 / 0.084 / 0.029–0.035 m | ANSUR II |
| sneaker | low top EU 42–43: 0.285 x 0.105 m, heel collar 75 mm, sole 20–25 mm | 0.286 x 0.108 m, collar 77 mm, tongue 99 mm, sole 24 mm | manufacturer size tables (from memory, no web lookup) |
| hoodie | hem at the upper seat, rib 50 mm, kangaroo pocket 0.30 x 0.15 m, hood folded down | hem 0.83 m, rib 54 mm, pocket 0.30 x 0.153 m | product knowledge |
| phone | 147 x 71.5 x 7.8 mm | same | product data (iPhone-size) |
| wall sample | 365 mm brick + 15 mm plaster | 0.38 x 1.9 x 1.2 m (kept) | asset thermal layers |
| skeleton landmarks | iliac crest h 0.13 m above the hip joint, biiliac 0.27 m, rib cage 0.27 m wide, sternal notch 0.82 H | crest h 0.138, ilium ±0.135, ribs ±0.134, notch 1.43 m shod | Gray's anatomy (own knowledge) |

## 2. Audit findings (before)

Rig
- Torso to C7 0.4495 m: C7 at 0.840 H, 27 mm low; the combined head-and-neck bone 0.2639 m put the vertex only at 1.709 m barefoot.
- Upper arm 0.303 m (15 mm short), forearm 0.2639 m (13 mm long): elbow at 0.647 H instead of 0.63 H.
- Ankle at 0.078 m: an 11 mm sole under a drawn sneaker.
- Every pose bent the knees 20 degrees (shank -19.9 to -21.7), including "standing-away", described as upright.
- Default pose: hands hanging, the phone hanging at the thigh, although the thermal state says "screen on" (phone 0.3) and the summary "looking at a phone".
- `phone-call`: the hand 0.2 m in front of the face and 0.1 m outside the ear (planar arm); "one hand in the pocket" was a hand hanging in the air.
- `leaning-arms-folded`: forearms pointing straight ahead side by side, nothing folded.
- `pushing-off`: both hands 5–11 cm inside the wall.
- Anchors: wall contact, mouth, nostrils, crown off the old body; phone-ear at the old hand.

Dimensions: 1.73 x 0.53 x 0.45 m; the posed figure measures 1.736 x 0.538 x 0.498 m.

2D (all 12 drawings): constructed, not views of one body (old proportions, hands hanging, upright front view with square shoulders, plan of ellipses and capsules, wall plan 0.12 m deep against a 0.38 m wall); arm shapes flat on bones that spread and twist.

Solids (1:50 85 / 79 / 36): figure-kit tubes with ball caps, skin lofts under every garment, a box pocket, structure = tube members plus three invented contact pads for the wall bones; coarse scales were coarsenings of 1:50 (1:500 a hull).

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones[torso].lengthM`, `mass.comX` | 0.4495, 0.2052 | 0.4755, 0.2171 |
| `rig.bones[head]` x / lengthM / comX | 0.4495 / 0.2639 / 0.1319 | 0.4755 / 0.249 / 0.1245 |
| `rig.bones[upper-arm-*]` lengthM / comX | 0.303 / 0.1749 | 0.318 / 0.1835 |
| `rig.bones[forearm-*]` x / lengthM / comX | 0.303 / 0.2639 / 0.1207 | 0.318 / 0.252 / 0.1153 |
| `rig.bones[hand-*]` x / lengthM / comX | 0.2639 / 0.1759 / 0.088 | 0.252 / 0.186 / 0.093 |
| `rig.bones[toe-*].y`, `foot-*.mass.comY` | -0.0782, -0.0391 | -0.088, -0.044 (21 mm sole) |
| anchors contact / heel | y -0.0782, heel x -0.0293 | y -0.088, heel x -0.03; footprints 0.11 x 0.095, 0.08 x 0.07 |
| anchors mouth / nostrils / crown | (0.1114, -0.0928) / (0.1399, -0.0976) / (0.2718, 0) | (0.06, -0.1) / (0.083, -0.112) / (0.249, 0.012) |
| anchors grip-l/r, phone-ear | (0.0733, 0), (0.0977, 0.02) | (0.075, 0), (0.097, 0.09, z 0.0255) on the phone's speaker |
| anchor wall-shoulder, wall-seat | (0.3225, 0.1251), (-0.0049, 0.1319) | (0.37, 0.142) upper back on the folded hood, (-0.02, 0.125); titles "upper back" |
| all poses, legs | knee 20–22 degrees | two-link solve: knee 3–14 degrees, soles level, ankles 0.05–0.15 m ahead of the hip (standing-away 0.04 behind) |
| all poses, root x / angle | 0.203/11, 0.1649/4, 0.1893/9, 0.1573/-2, 0.3251/0 | 0.2375/11, 0.129/-0.1, 0.2174/9, 0.1396/-2, 0.3297/0 (fitted from the 1:50 body: upper back on the wall; folded: upper back and seat; pushing-off 2 mm off; standing-away 0.2 m off) |
| default pose arms, head | hanging, head -6.6 | right arm solved (abduction, twist, forearm pronation 87 degrees) so the phone faces the eyes 0.32 m away, 52 degrees down; head -32; left arm hanging with 3 degrees spread |
| `phone-call` | planar arm, head -14 | right arm solved to put the phone at the ear (elbow flexed to the 145-degree limit), left hand solved into the kangaroo-pocket opening; head -14 |
| `leaning-arms-folded` | forearms straight ahead | both arms solved so the forearms cross in front of the chest, each hand at the other upper arm |
| `pushing-off`, `standing-away` arms | arms behind into the wall / planar | arms forward 10–12 degrees, 6 degrees spread, no wall contact |
| wall bones in all poses | hand-tuned | `wall-frame` = -root, `wall-x` / `wall` = -hip position (wall face at x = 0, foot on the ground) |
| clip `phone-call-idle` | head -9/-18/-11, upper-arm-r 42/36, forearm-l 44 | head -9/-19/-11, upper-arm-r solved ±3, upper-arm-l +3 (shrug in the pocket); description rewritten |
| clip `push-off` | — | description and contact-off event title rewritten (upper back) |
| poses' descriptions, summary, provenance notes, structure assumption | shoulder blades, "person" | rewritten (en, de); reference `gordon-2014` added |
| `materials` | 7 | + `bone` (skeleton, authored estimate) |
| `dimensions` heightM / widthM / depthM | 1.73 / 0.53 / 0.45 | 1.74 / 0.54 / 0.50 (measured on the posed 1:50 body: beanie crown in the lean, wall face to toe tips, right elbow to left hand) |
| `representations[*].geometry` | 318 shapes | 422 shapes, redrawn (below) |
| `representations[*].decisions` | old drawings | all eight keys rewritten per scale |

2D redraw (E1, `r3-leaning-person-draw2d.ts`): every filled shape is the outline of the body solids of its part at that scale in the default pose, Douglas-Peucker 0.9 / 1.8 / 3.5 / 7 mm. Trunk, legs, feet, head shapes are solved into the bone frame with the product's placement (`inverse`). Arm segments are solids of revolution about their bone, hands and phone extruded prisms along the hand axis most aligned with the view (the validator requires solids on bones that spread or twist); head abduction was removed from `phone-call` so head shapes stay flat. Parts inside the outline of their own bone (pocket, hem, beanie edge, eyes, brows, lips, nose, hood, far ear) also carry their outline as a detail stroke, because the line views only draw depth steps. Strokes: drawstrings (0.18 / 0.13 mm), mouth line (1:50 front). Wall sample redrawn at its true 0.38 m depth in side view and plan. Shapes and points elevation / front / plan: 1:50 45, 580 / 59, 878 / 48, 723; 1:100 32, 293 / 37, 403 / 35, 380; 1:200 25, 157 / 25, 227 / 25, 256; 1:500 24, 122 / 24, 159 / 24, 182.

## 4. What was modelled (solids body / thermal / structure)

- **1:50 — 85 / 107 / 145.** Body: hoodie of 14 trunk slices (seat, waist, chest, shoulder blades, sloping shoulders, neckline), rib hem, kangaroo pocket as a laterally lofted patch following the belly, drawstrings (slender) with aglets, hood pouch lying on the upper back and an elliptic roll round the neckline; sleeves with deltoid and taper, rib cuffs, bare wrists; jeans seat, thighs (trochanter, quadriceps), knee, calves, hem resting on the shoe lower at the heel; socks; sneakers: sole (rear and toe part), upper with heel counter, collar and tongue, toe box, toe cap, laces; head of 14 slices (chin, jaw, maxilla, cheekbones, forehead, crown) with nose, lips, brow ridge, eyes, ears, neck, short hair at nape and sides, beanie with a turned-up cuff, lower at the back; relaxed left hand with thenar, thumb (3 phalanges), four fingers of three phalanges; right hand with the four fingers under the phone and round its far edge, thumb on its near edge; phone with screen. Thermal: nested shells for hoodie (air, fleece), sleeves, jeans (air, denim), shoes (sock, air, upper), hair under the beanie; face ring chin to beanie edge with nose, lips, eyes, brows, ears (neck left out); open wrist bands; phone; wall as brick slab plus plaster with cut top, base and heated face as section faces; outlets under the nose, on the lips and on the beanie crown. Structure: see decisions — full skeleton (spine with both curves, 7 spinous processes, 12 rib pairs, sternum, clavicles, scapulae with spine, sacrum, flared ilia, pubic and ischial rami, cranium, facial skeleton, mandible, cervical spine, femur with head/neck/trochanter/condyles, patella, tibia, malleolus, fibula, talus, calcaneus, midfoot, 5 metatarsals and 5 toes per foot, humerus, ulna, radius, carpus, finger and thumb rays), 21 joints, 7 masses (segments of 2.5 kg and more), the wall as member of its three bones.
- **1:100 — 33 / 35 / 62.** Hoodie of 9 slices with hem merged, hood pouch and roll, drawstrings, legs of 4 stations, sleeves with cuff merged, wrists, shoes with soles merged, mitten hands with thumbs, right mitten on the phone, head of 7 slices with nose and ears, hair, beanie. Merged skeleton (6 rib hoops per side, girdle bars, hip bones, sacrum, skull and facial block, femur, tibia, fibula, patella, foot and toe bars, humerus, forearm bar, hand plates), 14 joints, masses.
- **1:200 — 24 / 26 / 50.** Hood and roll merged into the back of the hoodie (7 slices), no strings, nose, ears or thumbs; legs of 3 stations, mittens, phone slab 9.5 mm, head of 5 slices. Skeleton of bars with 3 rib hoops per side.
- **1:500 — 24 / 26 / 36.** Hoodie of 5 slices, one loft per limb segment, thickened mittens, round head of 4 slices, 3-slice beanie. Stick skeleton (38 mm bars, 1 rib hoop per side), 10 joints, masses of 9 kg and more. Body solid count equals 1:200; `volumes-monotonic` passes on the triangle count.
- Decisions: three bilingual sentences at 1:50, two at each coarser scale.

Final check line: `FAIL: 6 measurement(s) over 2 rule line(s) (12.9 s)`. Silhouette IoU elevation / front / plan: 1:50 0.991 / 0.984 / 0.946, 1:100 0.989 / 0.986 / 0.950, 1:200 0.992 / 0.982 / 0.954, 1:500 0.986 / 0.984 / 0.943. Outer areas at 1:50 (declared): head-skin 0.083 (0.07), torso-clothing 0.867 (0.8), arm-clothing 0.358 (0.27), arm-skin 0.007 (0.009), hand-skin 0.089 (0.089), leg-clothing 0.856 (0.61), shoes 0.203 (0.125), phone 0.023 (0.025), wall-face 3.667 (3.5). Totals 1:200 / 1:500 +10 %.

## 5. What I saw in the renders

- 1:50 body, default pose, six views with rig (catalog, final): reads at first sight as a young man in a hoodie and beanie leaning back against the wall looking down at his phone. Front: sloping shoulders, hood roll, drawstrings, pocket, hem, slim straight jeans, sneakers at hip width, right forearm across the belly with the phone, left hand hanging beside the thigh. Right side: upper back (folded hood) on the wall, trunk 11 degrees back, head bent to the phone, almost straight legs, ankles ahead of the hips, shoes flat. Back: hood pouch on the upper back. Top: shoulders, beanie, phone in the hand, feet ahead. Joints of the rig sit at hip, knee, ankle, shoulder, elbow, wrist of the figure.
- 1:50 upper-body and head close-ups (four and six directions): face with brow ridge, eyes, nose, lips, chin; ears beside the head; beanie lower at the back with a cuff; hair at the nape; hand round the phone with the screen to the face. Before the fixes I saw and removed: a hood standing off the back, a beanie tilted the wrong way, hair poking through the beanie, a keel-shaped chin, brow ends and eye ellipsoids sticking out of the cheeks, a cranium bigger than the head.
- 1:50 lower body (six views): hem rib, legs, knees, calves, hems on the shoes, shoe sole, toe box, toe cap, tongue; the knee overlap of thigh and shank shows as a seam.
- 1:50 `phone-call` (full and close-ups): phone at the right ear with the elbow forward and down; left hand enters the side opening of the pocket with the thumb outside. `leaning-arms-folded`: back and seat on the wall, forearms crossed in front of the chest, hands at the opposite upper arms; the right hand with the phone pokes a little out beyond the left arm. `pushing-off` (full): upright, arms forward, nothing in the wall. Clip `push-off` at 1.1 s (catalog sheet): the right arm half lowered with the phone, figure leaving the wall.
- 1:50 structure (six views, pelvis close-up): a skeleton at first sight, everything inside the body ghost; the ilium plates read as flat paddles rather than bowls.
- 1:50 thermal cut (six views): shells nested, nothing outside the body; 1:100 thermal cut (catalog): all regions coloured by their outer layer.
- 1:100, 1:200, 1:500 body (six views each): the same man progressively plainer; 1:500 had a fez-like beanie and the seat poking through the hem at the back — fixed and 1:200 re-looked; 1:500 three-quarter view after the fix: rounded beanie, plain figure with the phone; a small jagged patch remains where the seat meets the hoodie hem at the left hip.
- 1:200 structure (six views): bar skeleton with hoops, girdles, hip bones inside the ghost.
- 2D: `r3-2d` 1:50 candidate (line and tone, crops of side, front and plan) and 1:100 `phone-call` (tone): the drawings show the same person in all three views; the phone call reads in all three.
- Rendered but not opened: 1:500 structure sheet (catalog), 2D at 1:200 and 1:500, `standing-away` body, the `phone-call-idle` clip.

## 6. Still imperfect

- `volumes-thermal-coverage` red at 1:50 and 1:100 on `arm-clothing` (+32 / +34 %), `leg-clothing` (+40 / +41 %), `shoes` (+62 / +35 %): the declared values are skin shares; filed for the area reviewers (C18 forbids the author to change them).
- The 1:50 face is plain (no eyelids or nostrils), sections are superellipses (no pectoral or scapular relief, ilium as flat plates); no cloth folds beyond hem, cuffs, hood roll and the jeans hem; knee and elbow are overlaps of two rigid segments with a visible seam.
- The hood pouch is rigid: in `leaning-arms-folded` the back with the hood touches the wall, which needed a near-vertical trunk (-0.1 degrees).
- The phone hand: forearm pronation 87 degrees and the elbow at the 145-degree flexion limit in `phone-call`; the optimiser's residuals (phone 3–5 cm from the ideal ear spot) are within what a hand holding a phone covers.
- In the `push-off` transition the wall bones are blended linearly while the root is grounded per frame, so the wall drawing tilts by about 1 degree mid-clip (pre-existing behaviour of the wall chain).
- Knee 13 mm below 0.285 H (shank length kept); sneaker and anthropometric values from memory and the asset's references, no web lookup.
- `structure.supports` is empty in the asset, so no support solids; the wall is the member of `wall`, `wall-x` and `wall-frame` (a reference to the wall body solid) instead of invented contact pads.
- The neck under the collar is left out of `head-skin`; the face ring includes the skin behind the ears up to the beanie edge.
- Not run: the staffage test suites; fixtures that embed this asset (`structure-projection`, area fixtures) may need regenerating after the rig, dimension and 2D change.
