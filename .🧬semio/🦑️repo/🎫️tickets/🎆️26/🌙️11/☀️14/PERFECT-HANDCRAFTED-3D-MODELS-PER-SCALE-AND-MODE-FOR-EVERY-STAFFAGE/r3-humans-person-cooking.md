# Round 3 — `humans/person-cooking`

State (2026-11-20): root fixes, 2D redraw and volumes are in the catalog asset (written by `r3-person-cooking-commit.sh`: owned members replaced one by one by `r3-person-cooking-roots.ts --commit`, then `volumes-insert.py`). `r3-validate.ts humans/person-cooking --catalog-only`: `VALID ... schema 0, validateAsset 0`. With `--volumes`: `checkVolumes 6`, all six are areas (section 5). `volumes-check.ts humans/person-cooking --product`: `FAIL: 6 measurement(s) over 2 rule line(s) (21.7 s)`, only `volumes-thermal-coverage` at 1:50 and 1:100 (`jeans`, `sneakers`, `spoon`); every other rule is green at every scale (availability, silhouette, bounds, views-disagree, feature, monotonic, cross-scale, structure coverage, bones, slender, copied). Print suitability under the default profile: all four scales `authored` (smallest feature 0.33 / 0.25 / 0.32 / 0.22 mm). Manifest row updated (heightM 1.69 -> 1.71, widthM 1.088 -> 0.99).

No earlier round-3 work existed for this asset (no `r3-person-cooking-*` files, no report, catalog untouched since 2026-11-18, volumes source identical to it; the validator said VALID on the old state). Everything is from this run.

Scripts (ticket folder): `r3-person-cooking-data.ts` (landmarks, unit, pot, spoon), `-forms.ts` (trunk and seat slices), `-roots.ts` (rig, anchors, legs, arm solver `--solve`, clip keys, texts, dimensions, candidate, `--commit`), `-kit.ts` (station helpers, copied from the pregnant-person kit), `-s50.ts` (1:50 body), `-th50.ts`, `-st50.ts`, `-coarse.ts` (1:100, 1:200, 1:500), `-build.ts`, `-draw2d.ts` (2D redraw), `-texts.json`, `-dimensions.json`, `-fmt.ts`, `-io.ts`, `-fk.ts`, `-check.ts`, `-sheet.ts`, `-suit.ts`, `-run.sh`, `-commit.sh`. Caches and originals: `🗑️generated/r3-person-cooking/` (`original.json` is the catalog as found and the base of the roots script, `arms.json` the solved arm, `shapes.json` the redrawn 2D, `source-original.json` the volumes as found; keep them).

## 1. Reference (woman 1.69 m barefoot, 58 kg; canvas sneakers 24 mm heel, 16 mm ball; 600 mm hob base unit; 22 cm pot; 0.30 m cooking spoon)

| quantity | reference | asset now | source |
|---|---|---|---|
| sneaker lift at the ankle | heel 24, ball 16 mm, heel contact 0.04 m behind, ball 0.125 m ahead of the ankle | 22.1 mm | low canvas sneaker, vulcanised sole (own knowledge) |
| ankle | 0.039 H = 0.066 m barefoot | 0.088 m shod | Winter 2009 tab. 4.1 |
| knee | 0.285 H = 0.482 m | 0.470 barefoot (shank 0.404 of de Leva kept, 12 mm low), 0.492 shod | Winter 2009; de Leva 1996 |
| hip joint | 0.52 to 0.53 H = 0.879 to 0.896 m | 0.883 barefoot (0.522 H), 0.905 shod | Winter 2009 |
| lumbosacral joint | 9 to 11 cm above, 2 to 4 cm behind the hip axis | 95 mm above, 30 mm behind | sagittal pelvic geometry (own knowledge) |
| C7 | 0.855 H = 1.445 m | 1.445 barefoot, 1.467 shod, 20 mm behind the hip joints | de Leva 1996 |
| atlanto-occipital joint, vertex | vertex H; joint about 0.145 m below it | joint 1.567 shod, head bone 0.145 | ANSUR II head height (Gordon 2014) |
| head height, length, breadth (female) | 0.216 / 0.185 / 0.145 m | chin -0.070 to vertex +0.146 about the joint; skull 0.183 x 0.144 m | ANSUR II |
| shoulder joint | 0.818 H = 1.382 m; female biacromial 0.365 m, joints 2 to 3 cm inside | 1.4045 shod, joints 0.31 m apart | brief, Winter 2009, ANSUR II |
| hip joint spacing | 0.16 to 0.18 m | 0.175 m | Bell 1990 (own knowledge) |
| upper arm / forearm / hand | 0.186 / 0.146 / 0.108 H = 0.314 / 0.247 / 0.183 | 0.3135 / 0.247 / 0.1805 (kept) | Winter 2009 |
| chest, waist, hip breadth x depth (slim, nude) | 0.27 x 0.22 (bust) / 0.24 x 0.17 / 0.345 x 0.21 m; bideltoid 0.415 | sweatshirt 0.37 to 0.39 wide, 0.23 to 0.27 deep; seat of the jeans 0.352 x 0.216; over the sleeves 0.43 | ANSUR II female, scaled to 58 kg, + 15 to 25 mm ease |
| thigh / knee / calf / ankle girth | 0.54 / 0.35 / 0.345 / 0.21 m | jeans 0.545 / 0.33 / 0.33 / hem 0.27 | ANSUR II + 3 mm denim |
| upper arm / forearm / wrist girth | 0.26 / 0.23 / 0.15 m | sleeve 0.33 / 0.27, cuff 0.18 (relaxed fleece) | ANSUR II + ease |
| hand length / breadth / thickness | 0.18 / 0.075 / 0.026 m | 0.18 / 0.076 / 0.023 to 0.033 | ANSUR II |
| foot; canvas sneaker EU 39 | 0.245 x 0.09 m; shoe 0.26 to 0.27 x 0.095, collar 0.10 high, rubber toe cap | 0.265 x 0.095, collar 0.104, sole 27 to 20 mm with foxing | ANSUR II; own knowledge |
| working posture at a hob | trunk 5 to 10 degrees forward, head and neck 25 to 35 degrees down, toes 5 to 10 cm from the unit, forearm about level over a 0.90 m worktop | trunk 7, neck 10 + head 20 degrees; toes 0.08 m before the worktop edge; forearm level | DIN EN 1116 working heights, ergonomics texts (own knowledge) |
| base unit | 600 mm module: carcass 560 deep x 720 high of 16 mm board, 19 mm fronts, plinth 100 to 150 high and 50 mm back, worktop 600 x 38, top at 0.90 m | 0.56 x 0.72 x 0.60 carcass, 19 mm fronts, plinth 0.142 set back 50 mm, worktop 0.60 x 0.60 x 0.038, top 0.90 | DIN EN 1116 (added to references), own knowledge |
| hob | four-zone glass-ceramic 580 x 510 mm, cut-out 560 x 490, built-in depth 45 to 50 mm, zones 210 and 145 mm at +-0.14 m, 0.19 and 0.41 m from the worktop front | 0.58 x 0.51, 6.5 mm proud, tray 46 mm, zones 210 / 145 at those centres | manufacturers' standard cut-out (own knowledge) |
| pot | 22 cm stockpot (asset notes), 0.13 m high, 1 mm wall, sandwich base 200 x 6 mm, two riveted ear handles; 2.5 l fill 68 mm deep | as stated; water surface 74 mm above the hob | asset notes; own knowledge |
| cooking spoon | beech, 0.30 m, handle 11 to 12 mm, bowl 60 x 43 x 10 mm | as stated, grip 85 mm from the end | own knowledge |
| skin area, masses | DuBois 1.675 m2; de Leva female fractions | unchanged (C18) | asset |

No web lookup was made; all values are from the asset's references and my knowledge.

## 2. Audit findings (before)

- Rig: no sole (ankle 0.066 under a drawn sneaker, stature drawn 1.69 shod); torso straight up from the hip joints, C7 at 1.420 (0.840 H, 25 mm low), no lumbosacral offset; neck 0.095 and head 0.175 with the head joint in the jaw; shoulder joints 0.372 m apart (wider than the biacromial breadth); foot bone 66 degrees into the arch; torso titled "with coat". The unit hung 0.34 m ahead with the pot on the centre line of the bone 0.32 m into it (no hob zone there, 0.66 m from the hip joints, so the arm was stretched), the pot and the unit shared one origin, the spoon bone lay along the hand.
- Poses: knees bent 14 degrees in every pose, feet together; the spoon turned 185 degrees through the hand between stirring and tasting; the head twisted on flat shapes.
- Dimensions: 1.69 x 1.088 x 0.62 (height, forward, lateral).
- 2D: front and plan constructed (capsules, ellipses, rectangles, `fit-*` patches), a hood triangle beside the head in the front view, different ids per view.
- Solids (1:50 114 / 124 / 42): box torso, tube limbs, a block as unit, the structure the body again, 32 kernel traps (`kernel-traps.md`), the steam emitter tagged to the face region.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `pelvis` y | 0.883 | 0.90497 |
| `torso` y / angleDeg / lengthM | 0 / 0 / 0.4422 | 0.03 / -1.2269 / 0.46705, title "Torso with hooded sweatshirt"; centre of mass re-expressed |
| `neck` x / angleDeg / lengthM | 0.4422 / -8 / 0.0951 | 0.46705 / -16.5095 / 0.10504 |
| `head` x / angleDeg / lengthM | 0.0951 / 8 / 0.1746 | 0.10504 / 17.7364 / 0.145 |
| `arm-*` x / y / z / angleDeg | 0.39941 / 0 / +-0.1859 / 180 | 0.40496 / -0.02133 / +-0.155 / 181.2269 |
| `thigh-*` z | +-0.0888 | +-0.0875 |
| `foot-*` angleDeg / lengthM | 66.193 / 0.1635 | 90 / 0.125 (to the ground line under the ball) |
| `toe-*` x / y / angleDeg / lengthM | 0.1635 / 0 / 23.807 / 0.0572 | 0.125 / -0.08797 / 0 / 0.07 |
| `counter-lift` x / y | -0.883 / -0.34 | -0.90497 / -0.25 (worktop edge 0.25 m ahead of the hip joints) |
| `pot` x / y / z / lengthM | 0 / 0 / - / 0.14 | 0.19 / 0.9065 / -0.14 / 0.11 (centre of the base on the front right zone) |
| `spoon` y / z / angleDeg / lengthM / joint | 0 / - / 0 / 0.22 / -180..180 | 0.004 / 0.026 / -55 / 0.215 / -60..150 (at the grip, inside the curled fingers) |
| anchors heel / ball | (-0.0096, -0.0764) / (0.1635, 0) | (-0.04, -0.08797) 0.07 x 0.062 / (0.125, -0.08797) 0.10 x 0.09 |
| anchors mouth / nostrils / crown / ear | on the old head | (-0.0215, -0.1) / (-0.0005, -0.107) / (0.156, 0.012) / (0.03, -0.002, z -0.071) on the modelled face, hair and right ear |
| anchors counter-contact / pot-mouth / grip-r / spoon-cog | (0, 0) 0.6 x 0.6 / (0.32, 1.05) / (0.09, 0) / (0.05, 0) | (0.335, 0) 0.53 x 0.6 / (0, 0.13) / (0.09, 0.004, z 0.026) / (0.075, 0) |
| all poses, legs | thigh 6, shank -14, foot 8 | thigh 0.22, shank -4, foot 3.78 (ankle 25 mm behind the hip, sole level), `counter-lift` dx 0.9 mm for the lost height |
| `stirring` | torso -9, neck -4, head -12, arm 42.7 / forearm 64.5 | torso -7, neck -10, head -20; right arm solved: arm 13.43 / abduction -7.52 / twist -3.3, forearm 94.21 / twist 10.41, hand -20, spoon -14.33 (bowl 35 mm above the pot base within 2 mm) |
| `tasting` | spoon 106.5 degrees through the hand | arm 30.06 / -23.3 / -6.19, forearm 135.85 / twist -32.32, hand 8, spoon 106.75 from the new rest (bowl hollow up, 30 mm before the lips within 2.3 mm) |
| `standing-by` | neck twist -10, head twist -25 | no twist (flat head shapes), head bowed 12 degrees, spoon hanging beside the thigh |
| clip `stirring` | 9 keys of arm angles on an ellipse | 9 keys (8 + loop), each solved with abduction and twist so the bowl runs on a 50 mm circle in the pot; trunk sway 0.8 degrees |
| `dimensions` heightM / widthM / depthM | 1.69 / 1.088 / 0.62 | 1.71 / 0.99 / 0.60 (default-pose body); manifest likewise |
| materials | - | `bone`, `water` added |
| summary, pose and clip descriptions, `structure.assumptions`, provenance notes, references, `counter` region title | adult, counter | woman, hob base unit, round-3 note (en, de); `gordon-2014`, `din-en-1116` added |
| `representations[*].geometry` | 163 / 109 / 83 / 67 shapes | 240 / 149 / 96 / 66 shapes, redrawn (below) |
| `representations[*].decisions` silhouette, retained, merged, omitted, thinElements | constructed drawings | the views of the body |

2D redraw (E1, `r3-person-cooking-draw2d.ts`): every filled shape is the outline of the solids it stands for in the default pose, solved into its bone frame by the product's placement (`inverse` of `r3-project2d.ts`), one id per part in all three views with tone and hatch of the old drawing. Sleeves, forearms, cuffs and the spoon are `revolved` solids (the bowl with a lateral scale), hands `extruded` prisms, so the turned arm reads in every pose; nose, lips, eyes, brows and ears have elevation twins so the bowed head carries them. Strokes stated by hand: drawstrings, laces and mouth at 1:50, zone rings in plan at 1:50 and 1:100, drawer handles in plan at 1:100. At 1:500 the spoon is not drawn (0.02 mm on paper, the profile refused it) and the pot shape lists the spoon region.

## 4. What was modelled (body / thermal / structure solids)

- **1:50 — 126 / 132 / 167.** As in its decisions: sweatshirt of 16 slices with rib hem, kangaroo pocket, hood pouch on the back, rolled rim and two slender drawstrings; sleeves with deltoid cap, rounded elbow, folds and cuffs; jeans with seat, thighs, knees, calves, turned-up hems; sneakers with sole, upper, tongue, laces, toe cap; head of twelve slices with face parts, neck, short hair; hands with pads, three-phalanx fingers and thumb, the right one closed round the handle. Unit built as made: two cheeks, bottom, back, two rails, three fronts with bar handles on posts, plinth front and returns, worktop with cut-out, hob tray, glass plate with four zone rings and control strip. Pot as revolved 1 mm shell with base and rim, water, two bent-rod handles; spoon with handle, bowl and hollow. Thermal: nested shells (T-shirt, briefs, socks), the unit as closed envelope, four outlets on surfaces. Structure: full skeleton stretched to the segment lengths with curled finger bones, carcass boards and worktop on four legs, pot shell, spoon, joints and masses.
- **1:100 — 50 / 52 / 72.** New slices; mitten hands with thumbs; carcass as one hollow shell with back, three fronts, slender handles, plinth, worktop with cut-out, tray and plate; pot with lugs; spoon with slender handle and bowl. Skeleton with six rib hoops; unit as cheeks, bottom and worktop on legs.
- **1:200 — 29 / 32 / 59.** One mass per part; hollow carcass with front slab and 30 mm back, plinth block, worktop with the hob inlaid; plain pot with water; spoon as one rod. Bar skeleton with three rib hoops.
- **1:500 — 22 / 27 / 44.** Trunk with the hood merged, two-part limbs, fists; unit as one block with the hob as its top 75 mm; pot cylinder; the spoon only in thermal and structure. Stick skeleton and the unit as a portal on four legs.
- Decisions: three bilingual sentences at 1:50, two at the coarser scales.

Final check line: `FAIL: 6 measurement(s) over 2 rule line(s) (21.7 s)`. Silhouette 1:50 elevation / front / plan IoU 0.997 / 0.995 / 0.990 (p95 2 / 6 / 11 mm); cross-scale IoU 1.000 at every step.

## 5. Areas (not changed)

`jeans` 0.728 (1:50) and 0.735 (1:100) against 0.5452, `sneakers` 0.174 / 0.150 against 0.1005: the declared values are skin shares of legs and feet (C18); filed in `author-requests.md` ("2026-11-20 r3 humans/person-cooking"). `spoon` 0.0131 against 0.026: the declared value described the old 20 mm rod; hand calculation in `area-review.md`, factor 2.0, not applied (C16). Passing: face-skin 18 %, hand-skin 5 %, hoodie 3 %, counter 3.02 against 2.96, hob 0.605 against 0.6, pot 0.221 against 0.2. The product has no C31 flood: a carcass of boards read 5.0 m2, so the thermal list carries the closed envelope (`th-carcass`, `th-worktop`) as its own solids; the thermal jeans are cut at the sweatshirt hem with a section face.

## 6. What I saw in the renders

- Before (catalog as found): 1:50 sheet with a hood triangle beside the head, box trunk, the unit a block, the arm stretched to a pot in the middle of the hob; 2D with a rectangular head and a hood wedge in the front view.
- 1:50 body sheet (candidate several times, then the catalog): front shows the cook facing the viewer behind pot and unit back, head bowed, drawstrings, pocket, right fist on the spoon; right and left side show the forward lean, level forearm, hand above the pot, spoon in the water, hood pouch, hem, slim legs, sneakers; back shows the hood and the fronts with handles between the legs; top shows the four zone rings, the pot with both handles, the hand over it; three-quarter reads at once as a person stirring at a hob.
- 1:50 upper-body crop (0.85 to 1.75 m) twice, head crop (1.3 to 1.75 m) and a rear three-quarter (turn 315): face with nose, lips, eyes, brows, ears; the hair went from an overhanging cap to a closer crop but still reads as a cap with a hard lower edge; shoulders show the sleeve seam as a step; knees show a jagged intersection line.
- 1:50 poses: tasting (bowl at the lips, hollow up, elbow bent), standing-by before and after the fix (before, the spoon pointed into the drawer handles; after, it hangs beside the thigh); clip `stirring` at 1.0 s (hand on the handle, bowl at the far side of the pot).
- 1:50 structure sheet: skeleton inside the body everywhere, curled finger bones on the handle, carcass on four legs with the tray opening; the hip bones read as flat plates. 1:50 thermal cut sheet: T-shirt inside the fleece, briefs, pot orange, hob blue, the unit as envelope.
- 1:100 and 1:500 body sheets (candidate), 1:200 body sheet (catalog, before the back panels were added: the carcass was open behind), 1:500 structure sheet (stick skeleton, portal on legs).
- `r3-2d`: 1:50 (candidate, before the seat hatch fix: the seat showed as a block with the wrong hatch), 1:100 and 1:500 (catalog): the three views are one figure; hands are prisms in plan.
- `volumes-render` 1:50 overlay sheet: green, thin red at the left hand and forearm in plan.
- Rendered but not opened: 1:100 structure and thermal sheets, 1:50 tasting 2D, 1:200 2D; not rendered: 1:200 and 1:500 thermal, 1:200 structure, the list-only `volumes-render --az` series, the 1:100 and 1:200 sheets after the back panels (checked by the validator only).

## 7. Still imperfect

- Grade as I see it: 1:50 body B+ (hair still a cap, sleeve seam step at the shoulder, no cloth folds beyond hem, cuffs and two forearm folds, elbow and knee are overlaps of rigid lofts); coarse bodies B+; 1:50 structure A- (hip bones plate-like), 1:500 structure B; thermal A- at 1:50.
- The spoon swivels about 12 degrees in the fingers during the stirring loop and 120 degrees between stirring and tasting; the fingers do not regrip. The left arm only hangs.
- `standing-by` lost its head turn towards the pot, because the head shapes are flat outlines and a twist would draw them wrong.
- Hands and the spoon bowl are prisms and a scaled revolve in 2D, exact in the default pose only for the hand outline.
- Six area measurements stay red (section 5). The knee joint stays 12 mm low against Winter (de Leva shank kept).
- Not run: the staffage test suites and exports; fixtures that embed this asset may need regenerating after the rig and 2D change.
