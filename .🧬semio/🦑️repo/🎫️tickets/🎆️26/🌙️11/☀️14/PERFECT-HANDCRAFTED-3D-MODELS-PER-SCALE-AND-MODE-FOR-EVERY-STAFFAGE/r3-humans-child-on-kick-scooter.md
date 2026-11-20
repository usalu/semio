# Round 3 — humans/child-on-kick-scooter

Asset: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/child-on-kick-scooter/🔣️.json`. Source of the volumes: `volumes/humans/child-on-kick-scooter.json`. This round was resumed from an earlier author's scripts (reference, rig IK, kit, bodies, volumes, a first 2D redraw); the catalog file had not been touched yet. I rebuilt the candidate, repaired what failed (91 validator findings), finished the 2D, looked, corrected, and wrote the catalog.

State now: catalog file VALID (schema 0, validateAsset 0), print-suitable at all four scales in the three named poses, every pose and 23 instants of each clip evaluate, `volumes-check.ts --product` green on every rule except `volumes-thermal-coverage` at 1:50 and 1:100 for three declared areas that the rules forbid me to change (below).

## 1. Reference

| quantity | value | source |
|---|---|---|
| subject | girl, 8 years, stature 1.274 m, 26 kg | asset `provenance` (Fryar 2021, NCHS) |
| head height (chin to vertex) | 0.195 m = H / 6.5 | Snyder 1977 (UM-HSRI-77-17), 8-year-olds |
| hip joint | 0.505 H = 0.643 m (+ sole 15 mm) | Snyder 1977 trochanteric height; adults 0.52 H (brief) |
| knee joint | 0.268 H = 0.341 m | Snyder 1977 tibiale height |
| ankle joint | 0.048 m above the foot sole | Snyder 1977 sphyrion height |
| shoulder joint | 0.795 H = 1.013 m, 0.115 m off the midline | Snyder 1977 acromion height 1.02 m, biacromial 0.277 m, joint centre 25 mm inside |
| C7 | 0.83 H = 1.057 m | Snyder 1977 cervicale height |
| hip joint spacing | 0.13 m | bitrochanteric 0.215 m, joint centres 42 mm inside |
| upper arm / forearm / hand | 0.2286 / 0.1794 / 0.1327 m (0.179 / 0.141 / 0.104 H) | Drillis and Contini 1964, kept from the asset |
| foot | 0.207 m shoe (EU 33), ball 0.092 m ahead of the ankle, toes 0.062 m | Snyder 1977 foot length 0.196 m + shoe |
| girths | thorax 0.21 x 0.16 m, hip breadth 0.24 m, thigh 0.13, calf 0.086, upper arm 0.09 (sleeve), forearm 0.07, neck 0.08 m | Snyder 1977 breadths and circumferences, clothing added |
| scooter | 145 mm PU wheels 30 mm wide, wheelbase 0.56 m, overall 0.705 m, deck 0.33 x 0.12 x 0.028 m, top 85 mm above ground, rake 10 degrees, T-bar 0.34 m wide at 0.78 m, stem 34 / 29 mm with clamp, folding block with gooseneck, spoon brake, 3.3 kg + rider | Micro Speed / Micro Sprite data sheets (145 mm class), my knowledge of the product |
| helmet | skate helmet, 25 mm EPS + shell, 242 x 194 mm | EN 1078 child size S |

## 2. Audit (catalog before this round)

- Rig: hip 0.665 m (0.522 H, adult share), knee 0.364 m, head bone 0.166 m with a neck of 0.060 m, shoulders 0.125 m off the midline, hips 0.06 m; thigh 0.3011 and shank 0.3023 (equal, adult-like). No lateral motion in any pose: both feet stood 0.12 m apart on a deck that is 0.12 m wide.
- Scooter: wheelbase 0.78 m, overall length 0.925 m, deck top 0.11 m, deck 0.59 m long: an adult scooter under an 8-year-old. `dimensions` 1.274 x 0.925 x 0.45 m: the height was the child's stature, not the envelope of child, shoes, helmet and deck.
- 2D: drawn to that rig and that scooter; front and plan limbs as stretched stadiums.
- Solids: box torso, tube limbs, mitten hands (qa-visual-3: B- / A- / B at 1:50, child proportions missing); thermal at coarse scales one region, structure a body copy at 1:500.

## 3. Root fixes (old → new)

- `rig.bones`: pelvis y 0.6653 → 0.6584; torso 0.3264 → 0.35; neck 0.0602 → 0.0926; head 0.1661 → 0.124 (joint at the atlas, vertex 1.289 m with sole); arm z 0.125 → 0.115, x 0.3036 → 0.3054; thigh z 0.06 → 0.065, 0.3011 → 0.302; shank 0.3023 → 0.2934; foot 0.1219 → 0.1115 (rest 59.462 → 55.597 degrees); toe 0.0461 → 0.062; frame x -0.15 → -0.14, length 0.78 → 0.56, mass centre (0.33, 0.22); wheel-front x 0.78 → 0.56; segment mass centres rescaled with the lengths.
- `anchors`: heels, balls, grips (now on the grip axis in the closed hand), nose, mouth, crown, front wheel contact 0.78 → 0.56, deck, fender, backpack moved with the rig and the scooter.
- `poses` (47) and `clips` (2): every pose re-solved by inverse kinematics on the new rig (`r3-child-on-kick-scooter-root.py`): hands on the grips, front foot on the deck at z +0.020, rear foot on the deck at z -0.020 with the heel raised, pushing foot on the ground at z -0.125, braking foot on the fender; legs now carry `abductionDeg` (up to 10 degrees). Clip keys follow their poses; contacts, phases, tracks, wheel turns unchanged.
- `dimensions`: 1.274 x 0.925 x 0.45 → 1.344 x 0.705 x 0.344 m (envelope in the gliding pose: helmet top, 0.56 m wheelbase + 0.145 m wheel, 0.34 m bar). Mass unchanged.
- `thermal.regions.grips.areaM2` 0.022 → 0.027 (C16, factor 1.23, measured 0.0269).
- `representations` (ruling E1): all shapes of all scales and views redrawn from that scale's body (`r3-child-on-kick-scooter-draw2d.py` + `-project2d.ts`): same ids, bones, roles, z, tones, regions, pens. Points 686 / 716 / 535 at 1:50 (elevation / front / plan), 386 / 388 / 373 at 1:100, 197 / 248 / 241 at 1:200, 102 / 139 / 111 at 1:500. Because the legs now leave the sagittal plane, every thigh, shank, shoe and toe shape in every view (and the arm shapes in the front view, whose bones point at the viewer) is a `solid` (extruded, `sweep` w in the elevation, v in front and plan, with a `zTip` taper), the knee an ellipse swept the same way, the sole stroke the swept outline of the sole. Plan arms stay stadiums. Changes to the shape lists: 1:500 plan loses `toe-l` / `toe-r` (toe box merged into the shoe, bend 0.1 mm on paper), 1:500 front gains `wheel-rear` (behind the front wheel) and draws `stem-column` as a stroke (the stem is slender; a filled column hid both shanks down to unprintable slivers). Decision texts of the four scales updated where they described stadiums and mittens.

## 4. Models (body / thermal / structure solids)

- **1:50 — 95 / 126 / 87.** Body: lofts with hand-stated sections for seat, jacket hem, trunk with shoulder slope, collar, neck, head with jaw, nose, ears, eyes, mouth, ponytail and nape hair, skate helmet with straps, sleeves with deltoid and elbow overlap, hands closed round the grips with four fingers and thumb, thighs, shanks with calf, sneakers with separate sole, toe box, laces, backpack with pocket, shoulder and sternum straps; scooter with extruded deck, grip tape, dropout plates, axles, spoon fender with pad, folding block, hinge pin, gooseneck, head tube, crown, fork blades, two-part stem, clamp with lever, T-bar, flanged grips, tyres, rims, spokes, hubs. This pass: fork profile repaired, clamp lever attached, knee and elbow inserts set inside the upper segment (clean seam), jacket hem raised to the hip line and the blouson overlap separated from the hem band, face sections rounded. Thermal: every region, nested shells per layer for jacket (air 4, T-shirt 1, jacket 4 mm), leggings, sneakers, helmet (20 mm air, 25 mm EPS), hair pelt; face skin as a one-sided patch; outlets at nostrils, mouth, helmet crown. Structure: skull, jaw, cervical and thoracolumbar spine with curvature, ribcage, scapulae, clavicles, pelvis, femur with head and neck, patella, tibia, fibula, tarsus, toes, humerus, radius, ulna, metacarpus, phalanges round the grip, 17 joints, 6 masses, the scooter frame parts and wheels, two roller supports.
- **1:100 — 55 / 58 / 75.** Fewer sections, finger roll with thumb, one upper per shoe with toe box, plain scooter parts; thermal one surface solid per region part; skeleton as at 1:50 with every shaft at least 15 mm.
- **1:200 — 32 / 35 / 48.** One loft per limb segment, fists, hip merged with seat, deck slab, one stem tube, 18 mm fork blades (printable in plan), grips 116 mm; skeleton with one shoulder-girdle bar, one forearm bone, no toes, shafts at least 30 mm.
- **1:500 — 24 / 28 / 31.** Three to five sections per mass, shoe with toe as one form, fists at the 75 mm floor, 36 mm wheel discs, stem a slender 32 mm tube, fork and gooseneck one plate; thermal adds a nape-hair solid for the `hair` region; structure a stick skeleton at the 75 mm floor (skull, spine, ribcage, pelvis, one bone per segment), masses, deck, stem, fork, bar, wheels, supports.
- `decisions`: three bilingual sentences per scale (body, thermal, structure).

Final check line: `FAIL: 6 measurement(s) over 2 rule line(s)`: `areaDeviation` helmet 1.65 / 1.33, sneakers 0.52 / 0.36, deck 0.48 / 0.48 (1:50 / 1:100); everything else ok. Silhouette IoU elevation / front / plan: 1:50 0.971 / 0.971 / 0.930, 1:100 0.983 / 0.976 / 0.942, 1:200 0.924 / 0.981 / 0.974, 1:500 0.918 / 0.943 / 0.942. `r3-validate.ts` (with source and `--catalog-only`): VALID.

Why three areas stay red: `helmet` (0.0484) and `sneakers` (0.0677) are skin shares of scalp and feet, the helmet and the shoes are larger (C18 forbids a change; `author-requests.md`); `deck` 0.262 was decided for the old 0.59 m deck, the real deck measures 0.137 (factor 1.91, C16 forbids me to apply it; `area-review.md`).

## 5. What I saw (final renders, opened with the image viewer)

- 1:50 body, six directions and two three-quarter views: reads at first sight as a school child on a small folding scooter. Front: head about 1/6.5 of the figure, shoulders 0.32 m, both hands closed on the grips, feet one behind the other on the deck. Side: slight forward lean, front leg nearly straight, rear leg bent with raised heel above the fender, bar at hip height, backpack high on the back, ponytail below the helmet rim. Back: backpack with pocket, seat of the leggings below the jacket hem. Top: bar, hands, head, pack in line. Poses `pushing-off`, `braking` and the clip at 0.3 s: the pushing foot is on the ground beside the deck, the standing knee bends, hands stay on the grips, no joint opens.
- 1:100, 1:200: same figure, visibly simpler, proportions unchanged. 1:500: recognisable (helmet, pack, bent legs, deck, two discs, yellow slender stem).
- 1:50 thermal (cut): region colours, nested shells in trunk, limbs, shoes and helmet, the red face patch. 1:50 structure: a skeleton inside the ghost, the scooter frame; 1:500 structure: thick sticks.
- 2D (`r3-2d.ts`, all four scales, gliding and pushing-off): elevation, front and plan agree with the model; the 1.34 m height dimension is right.

## 6. Still imperfect

- Grade as I see it: 1:50 body A-, 1:100 A-, 1:200 B+, 1:500 B+; thermal A- / B+; structure B+ (1:50, 1:100), B (1:200, 1:500).
- 1:50: the waist ring between blouson and hem band and the knee seam are slightly wavy lines (two faceted lofts crossing); the jacket has no cloth folds; the face is plain (wedge nose, dot eyes); the helmet is a smooth dome without vents; the ribcage is one barrel, not single ribs; scapulae are flat ellipsoids.
- 1:500: structure sticks of 75 mm are almost as thick as the limbs, fists are 75 mm, both by `volumes-feature`; the fork plate runs through the wheel disc.
- The scooter's centre of mass (0.33, 0.22 m in the rig) lies in the air between deck and stem; the `mass-frame` solid sits in the deck below it (y 0.071), because the rule keeps structure inside the body silhouette.
- Front and plan leg shapes are tapered prisms: true envelope in every pose, but their ends are straight, not rounded.
- Three thermal areas fail the check (above); `face-skin` is +16 % / +18 %.
- `r3-child-on-kick-scooter-root.py` re-solves the poses from the pre-round asset kept as `🗑️generated/r3-child-on-kick-scooter/asset.before.json`; deleting that file makes the roots non-reproducible (the catalog itself is complete).
- Product defect met and worked around (`tool-requests.md`): the polygon union throws on swept shapes seen along a profile axis; the validator does not evaluate non-default poses and does not report print unsuitability.

## 7. Files

Scripts in the ticket folder: `r3-child-on-kick-scooter-ref.py` (reference numbers), `-fk.py`, `-root.py` (rig, anchors, IK), `-kit.py`, `-body50.py`, `-coarse.py`, `-volumes.py` (thermal, structure, decisions), `-build.py`, `-project2d.ts`, `-draw2d.py` (2D, dimensions, texts), `-probe.ts` (all poses and clips), `-suit.ts` (print suitability per view and pose), `-sheet.ts`, `-run.sh` (rebuild + validate candidate), `-apply.ts`, `-ship.sh` (rebuild, probe, write catalog, splice, validate). Appended: `author-requests.md`, `area-review.md`, `tool-requests.md`.
