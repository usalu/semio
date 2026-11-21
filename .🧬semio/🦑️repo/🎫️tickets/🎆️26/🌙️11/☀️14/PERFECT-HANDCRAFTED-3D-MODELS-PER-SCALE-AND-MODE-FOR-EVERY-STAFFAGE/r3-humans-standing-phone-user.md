# Round 3 — `humans/standing-phone-user`

State 2026-11-20: root fixes and volumes are in the catalog asset. `r3-validate.ts humans/standing-phone-user --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/standing-phone-user --product`: every rule passes at every scale except `volumes-thermal-coverage` at 1:50 and 1:100 on three `areaM2` values that an author may not change (section 6). All four scales resolve as `authored` under the default print profile.

No earlier round-3 work existed for this asset; it was started from the round-2 state. The pipeline is a copy of the standing adult's (same person, same clothes), rewritten for this asset.

Scripts (ticket folder): `r3-standing-phone-user-apply.sh` (build, validate, write the catalog only when valid), `-build.py` (candidate document), `-roots.py` (rig, anchors, poses, clips, dimensions, provenance, shapes, texts), `-grip.py` (solver of the arm offsets of the three poses), `-model.py`, `-2d.ts` (2D redraw), `-view.ts`, `-frames.ts`, `-extents.ts`, `-suit.ts`, `-look2d.sh` (probes); modules `r3_standing_phone_user_rig.py` (rig table and forward kinematics), `_kit.py`, `_body50.py`, `_s50.py`, `_s100.py`, `_s200.py`, `_s500.py`, `_parts.py` (ponytail, headphones, hands, phone at the coarser scales), `_strokes.py`, `_texts.py`, `_fmt.py`.

## 1. Reference (slim woman of 24, stature 1.69 m barefoot, 59 kg; shod on a 20 mm sneaker sole)

| quantity | reference | value in the asset | source |
|---|---|---|---|
| hip / knee / ankle joint height | 0.52 to 0.53 H / 0.285 H / 0.039 H | 0.903 / 0.490 / 0.086 m shod (0.522 / 0.278 / 0.039 H barefoot) | brief; Winter 2009 table 4.1; de Leva 1996 (segment lengths kept, knee 12 mm low) |
| shoulder / elbow / wrist height | 0.818 H / 0.63 H / 0.485 H | 1.3975 / 1.084 / 0.837 m shod | Winter |
| C7, atlanto-occipital joint, vertex | 0.855 H, 0.912 H, H | 1.465 / 1.561 / 1.710 m shod | de Leva 1996 |
| head height, breadth, length | H / 7.5 = 0.225; 0.145 to 0.155; 0.187 to 0.197 m | 0.222 / 0.147 / 0.190 m | brief; ANSUR II (Gordon et al. 2014) |
| upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H | 0.3135 / 0.247 / 0.1805 m | Winter |
| fingers past the knuckle (index, middle, ring, little), thumb | 0.072 / 0.079 / 0.073 / 0.058 m; thumb 0.060 m | 0.082 / 0.091 / 0.085 / 0.067 m from the joint centre inside the palm; thumb 0.058 m | ANSUR II hand data; own knowledge |
| hip joint spacing, shoulder joint spacing | 0.17 to 0.19 m; biacromial 0.365 to 0.415 m | 0.17 / 0.306 m (bideltoid 0.435 nude, 0.46 with sleeves) | Bell et al. 1990; ANSUR II |
| trunk, limb girths, foot, sneaker | as the standing adult (ANSUR II, DIN 33402-2, slim build) | same sections as `humans/standing-adult` | `r3-humans-standing-adult.md` section 1 |
| head flexion while texting, standing | 33 to 45 degrees | 43 degrees against the upright trunk: trunk 3, neck 18, head 22 | Lee, Kang and Shin 2015 (added to `provenance.references`) |
| viewing distance and screen tilt | 0.30 to 0.40 m; screen roughly square to the line of sight | 0.35 m from the eyes, screen 32 degrees from the horizontal, 10 degrees off the line of sight | own knowledge |
| texting arm posture | upper arms beside the trunk, elbows 90 to 105 degrees, forearms turned palm-up, both hands on the phone | elbows 97 and 104 degrees, supination 75 and 78 degrees, upper arm spread 7 degrees and turned in 27 degrees | own knowledge; solved in `r3-standing-phone-user-grip.py` |
| smartphone | 146 to 148 x 71 to 72 x 7.4 to 7.8 mm, corner radius about 10 mm, 170 to 190 g | 147 x 71.5 x 7.8 mm, corners 10 mm, 190 g (unchanged mass) | manufacturer data of 6.1 inch phones, from memory |
| over-ear headphones | cups 80 to 95 mm across, 40 to 46 mm deep with cushion; band 30 to 35 mm wide, about 0.36 m | cups 86 x 46 mm, band 30 x 12 mm, 0.36 m | manufacturer data, from memory |
| segment masses, skin area | de Leva 1996; DuBois 1.675 m² | unchanged | asset |

No web lookup was made; phone, headphone and sneaker values are from memory.

## 2. Audit findings (round-2 state)

Rig
- Skeleton barefoot on the ground under drawn sneakers (ankle 0.066 m, no sole).
- `torso` straight up from the pelvis tip: C7 at 0.840 H (25 mm low), no offset of the spine behind the hip joints. Head joint at 1.515 m, 26 mm below the atlanto-occipital joint, inside the jaw.
- Texting pose planar: both arms in the sagittal planes through the shoulders, hands 0.31 m apart, so "both hands holding the phone" was not true in front view and plan. Knees flexed 20 degrees and the whole figure leaning back for a pose called standing.
- `phone` a fixed bone starting at the phone's centre with its screen normal in the hand's plane; the phone could not turn in the hand, so the call pose held it across the cheek.
- `phone-call`: elbow at 148 degrees with the wrist 6 cm from the shoulder joint, not reachable with these segment lengths in three dimensions.
- Anchors `mouth`, `nostrils` 4 to 5 cm off; `ear` on the midline; heel and ball for a bare foot.

Dimensions: 1.67 x 0.64 x 0.40 m (height, forward, lateral). Forward 0.64 m came from a ponytail sticking out behind and outstretched forearms; lateral 0.40 m is narrower than the figure's own shoulders with sleeves (0.46 m).

2D (12 drawings): constructed, not views of one body. Elevation of `path` outlines; front with an ellipse head, hands hanging at the sides while the side view holds them at the chest; plan of capsules and ellipses with the arms pointing straight ahead; headphones bound to the neck bone; 1:500 without a far arm.

Solids (110 / 114 / 38 at 1:50): box-like trunk, tube limbs, ball joints, ponytail as a horizontal sausage, hands as flat mittens with stick fingers that hold nothing, phone thickened to 15.75, 31.5 and 76 mm at the coarser scales, structure of 38 body-like lofts.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones[pelvis].y` | 0.883 | 0.903 (20 mm sole) |
| `rig.bones[torso]` y / angleDeg / lengthM / comX, comY | 0 / 0 / 0.4422 / 0.2043, 0 | 0.035 / -1.84 / 0.46709 / 0.2053, -0.0284 |
| `rig.bones[neck]` x / angleDeg / lengthM / comX | 0.4422 / -8 / 0.0951 / 0.0476 | 0.46709 / -17.228 / 0.10163 / 0.0508 |
| `rig.bones[head]` x / angleDeg / lengthM / comX | 0.0951 / 8 / 0.1746 / 0.0728 | 0.10163 / 19.068 / 0.149 / 0.0463 |
| `rig.bones[arm-*]` x / y / angleDeg | 0.39941 / 0 / 180 | 0.40033 / -0.02216 / 181.84 (same shoulder joint) |
| `rig.bones[foot-*]`, `[toe-*]` | 66.193, 0.1635, comX 0.0703; toe x 0.1635, 23.807 | 60.133, 0.17251, comX 0.0742; toe x 0.17251, 29.867 |
| `rig.bones[phone]` | x 0.0807, y 0.022, 18 degrees, joint fixed, comX 0 | x 0.073, y -0.038, z 0.024, 40 degrees, joint revolute -75..30, comX 0.0735 (origin at the middle of the lower edge, screen normal = palm normal) |
| `anchors` heel / ball | (-0.0096, -0.0764) / (0.1635, 0) | (0.015, -0.0904), 0.07 x 0.062 / (0.17251, 0), 0.09 x 0.092 |
| `anchors` mouth / nostrils / crown / ear | (-0.0049, -0.0777) / (0.0291, -0.1009) / (0.1697, 0) / (0.045, 0) | (-0.031, -0.101) / (-0.011, -0.107) / (0.149, 0) / (0.021, 0.01, z -0.079) |
| `anchors` phone-cog / grip-r | (0, 0) / (0.09, 0) | (0.0735, 0) / (0.1237, 0.0346, z 0.0366), the tip of the right thumb |
| all poses, legs | thigh 8 to 13, shank -20 to -26, foot 11 to 13 | thigh -1, shank -5, foot 6 |
| pose `texting` | torso -4, neck -16, head -24; arms 6.1 / 105.3 / -5.5, planar | torso -3, neck -18, head -22; right arm -1.4, spread -7, twist -27.2; forearm 103.9, twist 75.2; hand -8.9, spread 21.6; left arm 0.9 / 7.2 / 27.7, 97.0 / -78.3, -8.5 / -25.8 / 3.0 (the left hand 26 mm behind the right) |
| pose `phone-call` | right arm 35.6 / 148.5 / -6.1, left -13.8 / 96.2 / -2.4, planar | right arm 55.4, 0.5, -19.8; forearm 150, -13.5; hand 38.8, -5.4, -8.7; phone -62.3; left arm 27.8, 12.3, 90; forearm 88.7, -27.3; hand 9.3 |
| pose `photo-pose` | both arms 77.4 / 61.3 / 16.3, planar | right 66.5, -12.1, -38; 84.1, 90; -0.5, 22.3; left 68.1, 10.7, 38.4; 76.2, -90; -0.5, -27.9 |
| pose descriptions, `answer-call` description, `summary` | "Feet together", "arms nearly straight", "weight on one leg" | describe the new poses; "feet at hip width" |
| clip `scroll-and-sway` keys | head -27 / -21 / -26, neck -18 / -15 / -17, torso -5 / -2 / -4.5, hand-l about -5.45 +-5 | head -25 / -19 / -24, neck -19 / -16.5 / -18.5, torso -4 / -2 / -3.5, hand-l -7 / -10 / -7.5 |
| clip `answer-call` keys | leg overrides, planar arm keys | no leg overrides; key 0.8 s neck -7, head -8, torso -1; key 2.2 s a stated half-way arm posture with the phone turned -38 degrees |
| `dimensions` heightM / widthM / depthM | 1.67 / 0.64 / 0.40 | 1.68 / 0.50 / 0.48 (envelope of the texting pose: crown of the bowed head; hood to fingertips; elbow to elbow) |
| manifest entry heightM / widthM | 1.67 / 0.64 | 1.68 / 0.5 |
| `provenance` | — | references `gordon-2014`, `din-33402-2-2005`, `lee-2015`; round-3 note (en, de) |
| `structure.assumptions[1]` | "close together" | "at hip-joint width" |
| `representations[*].geometry` | 354 shapes | 421 shapes, redrawn |
| `representations[*].decisions` | described the constructed drawings | `silhouette`, `retained`, `merged`, `omitted`, `thinElements` at every scale, `overlays` at 1:200 and 1:500 rewritten |
| headphone shapes | bone `neck` | bone `torso` (they rest on shoulders and collar) |

The arm offsets were solved, not guessed: `r3-standing-phone-user-grip.py` finds the posture nearest to a stated natural one that puts the phone where stated (residual under 0.5 mm in texting).

2D redraw (E1). Shapes and points, elevation / front / plan: 1:50 68, 802 / 72, 881 / 48, 729; 1:100 38, 411 / 40, 495 / 29, 356; 1:200 25, 229 / 25, 283 / 23, 213; 1:500 18, 125 / 18, 135 / 17, 117. Because arms, hands and phone now leave the sagittal plane, they are drawn as the product's 3D shape solids so that they follow every pose: sleeves, forearms and cuffs as solids of revolution about their bone; hands, fingers, thumbs, ponytail and phone as plates in their own plane; head and hair in front and plan as prisms with stated depth spans that give the true height and length of the bowed head. Old ids kept where the part exists. New ids: `jeans-seat`, `sock-*`, `toe-cap-*`, `hood-roll`, `hair-fringe`, `fingers-*`, `headphone-band-l/-r`, `headphone-cup-l/-r` and `headphone-cushion-l/-r` in every view. Gone: `waist`, `trunk`, `shoulders`, `neckwear`, `face`, `upper-arm-*`, `shoe-heel-*`, the `finger-*-n` separation strokes, `sil-pony`.

## 4. What was modelled (solids body / thermal / structure)

- **1:50 — 102 / 107 / 185.** Body: head with jaw, chin, nose, lips, brows, eyes, ears; hair as a cap 8 mm over the skull ending at a stated hairline from forehead over the ears to the nape; hair tie; ponytail hanging under gravity from the bowed head onto the nape; neck; sweatshirt, hem rib, collar rib, pocket, drawstrings, hood with rolled edge; headphone band in two halves, yokes, cup shells, cushions; sleeves, forearms, cuffs; per hand palm, thenar, thumb of three segments, four fingers of three segments in the two-hand texting grip (right fingers flat on the back of the phone, thumb on the screen; left hand 26 mm further behind, index and middle finger under those of the right); phone plate with rounded corners and glass; jeans seat, buttocks, thighs, kneecaps, shanks; socks; sneakers. Thermal: outer surfaces by reference; T-shirt shell on trunk and upper arms; briefs under the pelvis; socks in the uppers; face ahead of the hairline; outlet patches at nose, lips, crown. Structure: the standing adult's skeleton (cranium to toes, 12 ribs each side) with the hand bones along the grip (carpus, five metacarpals, phalanges), the phone as its own member, 17 joints, 4 masses. `structure.supports` is empty, so there are no support solids.
- **1:100 — 35 / 38 / 82.** New sections; ponytail of six sections, headphone band 15 mm thick and one form per cup, hand as palm, finger slab and thumb, phone as one plate.
- **1:200 — 24 / 27 / 44.** One mass per part; ponytail, one ellipsoid per cup, no band, one mass per hand, phone plate at true thickness.
- **1:500 — 19 / 25 / 24.** Fewest masses; hands clasped in front of the chest; no phone, headphones or ponytail in the body. Thermal keeps headphones and phone as solids at the 75 mm size floor (C21); structure is a stick figure with the phone as a block.
- Decisions: three bilingual sentences per scale.

Final check line: `FAIL: 6 measurement(s) over 2 rule line(s)`, all six in `volumes-thermal-coverage` (section 6); every other rule `ok` at every scale. Silhouette IoU elevation / front / plan: 1:50 0.987 / 0.980 / 0.980; 1:100 0.988 / 0.978 / 0.967; 1:200 0.986 / 0.971 / 0.961; 1:500 0.976 / 0.966 / 0.965. Outer areas at 1:50 against declared: face 0.1167 / 0.1173, hands 0.0860 / 0.0838, hoodie 0.913 / 0.9283, phone 0.0240 / 0.022, jeans 0.784 / 0.5452, sneakers 0.184 / 0.1005, headphones 0.0764 / 0.0208.

## 5. What I saw in the final renders

- `r3-sheet` 1:50 body, texting, six views with and without rig: joints at hip, knee, ankle, shoulder, elbow, wrist of the figure. Front: bowed head showing mostly hair, cups on the collarbones, elbows at the sides, forearms converging on the phone at the lower chest. Sides: head forward and down, ponytail on the nape, hood on the back, forearm level, phone tilted towards the face. Back: hood, band behind the neck. Top: band round the neck, forearms meeting at the phone. Three-quarter: reads at once as a young woman texting.
- 1:50 body `phone-call` and `photo-pose` sheets: phone at the right ear with the elbow forward and the left forearm across the waist; phone upright at eye height in both hands. In both upright poses the ponytail stands back and up by about 40 degrees.
- 1:50 `answer-call` at 2.2 s (three-quarter): phone half-way up in the right hand, left hand lowered.
- Hand close-ups (six directions, flat-shaded probe): phone lies on the crossed fingers, thumbs over both edges onto the screen, no part passes through the phone.
- 1:50 structure sheet: a skeleton at first sight, inside the body ghost, skull bowed, hand bones under the phone. 1:50 thermal cut sheet: T-shirt inside the sweatshirt, briefs, socks, red face under green hair, headphones, phone.
- 1:100, 1:200, 1:500 body sheets: the same person, simpler each time; at 1:500 two clasped hand masses. 1:100 and 1:500 structure sheets: merged skeleton; stick figure with the phone block.
- 2D, 1:50 texting, enlarged line and tone drawings of all three views: side reads as the figure texting with ponytail, hood and headphone cup; front shows hair to the hairline, the foreshortened face, cups, crossed hands under the phone; plan shows the triangle of shoulders and converging forearms, band, head, ponytail, phone. 2D 1:50 `phone-call` and 2D 1:500 sheets opened: both read right.
- Overlay sheet 1:50 (`volumes-render`): green almost everywhere; thin red and blue bands at the hands and behind the trunk.
- Rendered, not opened: 1:200 structure; thermal cut sheets at 1:100, 1:200, 1:500; `scroll-and-sway` at 3 s; 2D sheets at 1:100 and 1:200; 2D `photo-pose`.

## 6. Still imperfect

- `volumes-thermal-coverage` stays red at 1:50 and 1:100 on three regions. `jeans` (+44 %) and `sneakers` (+83 %) are body-model areas (C18) and were red before this round (+42 %, +99 %); the standing adult passes with the same solids because its shares were redistributed by the area reviewer (C27). `headphones` is new: the real-size headphones measure 0.076 m² against 0.0208, beyond the factor 1.5 of C16. Nothing was changed; measurements and proposals are in `area-review.md` and `author-requests.md` (2026-11-20).
- The ponytail is rigid on the head bone and is modelled hanging under gravity for the bowed default pose, so it stands back and up in `phone-call` and `photo-pose`.
- Grade as I see it: body 1:50 B+ to A-. Lofts are superellipses: no cloth folds at the bent elbows, fingers are straight capsule chains without knuckle relief, the face is plain, elbow and knee are overlaps of rigid segments. The chin dips about 1 cm into the collar front in the bowed pose.
- Left and right hand are not mirror images (the left lies under the right); at 1:500 the two hand masses overlap by design.
- 2D: head and hair in front and plan are prisms, exact in height and length but with straighter sides than the true outline; the plan ponytail is a rectangle; hand plates are thinned to 60 % to stand for rounded forms; finger separations are not drawn.
- Knee joint 12 mm below 0.285 H (de Leva segment lengths kept). Phone, headphone and sneaker dimensions are from memory, without a web lookup.
- Structure solids use material `skin` (the asset has no bone material); at 1:500 the hand member is as thick as the hand (same finding as the standing adult).
- Not run: the staffage test suites. Fixtures or tests that embed this asset may need regenerating after the rig, dimension and 2D change (only the catalog manifest refers to it by name).

## Round 5 handcraft (2026-11-21)

Scripts changed (re-runnable, absolute values): `r3_standing_phone_user_rig.py` (bone `ponytail`), `r3-standing-phone-user-roots.py` (bone insert, pose and clip keys of `ponytail`, `fix_ranges`), `r3_standing_phone_user_parts.py` (ponytail per scale on its own bone in the rest world, 1:500 hand masses and hand bones), `r3_standing_phone_user_body50.py` (shank, kneecap, ponytail), `r3_standing_phone_user_s100.py`, `_s200.py`, `_s500.py` (knee, 1:500 toe cap). Renders: `🗑️generated/r5-standing-phone-user/before|after|after1/`.

### Reference (stature 1.69 m barefoot, slim woman, 20 mm sole) and model against it

| quantity | reference | model (barefoot) | deviation |
|---|---|---|---|
| hip joint | 0.52 to 0.53 H = 0.879 to 0.896 m (Winter 2009) | 0.883 m = 0.522 H | in band |
| knee joint | 0.285 H = 0.482 m (Winter) | 0.470 m = 0.278 H | -2.4 % |
| ankle | 0.039 H = 0.066 m | 0.066 m | 0 % |
| shoulder joint | 0.818 H = 1.382 m | 1.3775 m = 0.815 H | -0.3 % |
| elbow | 0.630 H = 1.065 m | 1.064 m | -0.1 % |
| wrist | 0.485 H = 0.820 m | 0.817 m | -0.4 % |
| head height | H / 7.5 = 0.225 m | 0.222 m | -1.3 % |
| hand | 0.108 H = 0.183 m | 0.1805 m | -1.2 % |
| foot | 0.15 H = 0.254 m | 0.250 m | -1.6 % |
| biacromial (female) | 0.224 H = 0.379 m (ANSUR II female 36.5 cm at 162.9 cm) | 0.375 m | -1.1 % |
| ponytail | 0.12 to 0.18 m of tied shoulder-length hair, hangs from the tie under gravity | 0.15 m path, its own bone, every pose keeps it hanging | — |

All within 3 %; the rig needed no proportion change. The brief's 0.23 H biacromial is a male value; the female ANSUR value is used.

### Grades (as seen in the opened renders)

| scale | body before / after | thermal before / after | structure before / after |
|---|---|---|---|
| 1:50 | B (knee blob and step, ponytail stuck out of the nape in phone-call and photo-pose, rigid) / A- | A- / A- | A- / A- |
| 1:100 | B+ (knee step) / A- | A- / A- | A- / A- |
| 1:200 | B+ (knee step) / A- | B+ / B+ (not re-opened) | B+ / B+ (not re-opened) |
| 1:500 | B+ / A- (crisp massing, knee continuous, hands and toe caps a feature thicker) | B+ / B+ | B (stick figure, forced) / B |

### Changes

- Rig root: new bone `ponytail` (parent `head`, joint at the hair tie behind the occiput, 0.075 m, revolute -60..60, no mass of its own: the hair is in the de Leva head mass). Pose offsets that keep it falling under gravity and lying on the nape: `texting` 40, `phone-call` -1, `photo-pose` -3; clip keys `scroll-and-sway` 45 / 34.5 / 43, `answer-call` 16 / 5. The ponytail solids of 1:50, 1:100, 1:200 are restated on that bone in the upright rest world (from the tie down the back of the head, the end resting on the hood). Before, the ponytail was rigid on the head and stood out backwards or cut into the neck whenever the head was not bowed.
- Knee at every scale: the shank top is now 0.5 to 1 mm fuller than the thigh end, so the thigh closes inside it without a step; the 1:50 kneecap is a flat relief (14 x 28 x 26 mm half axes, 2 mm proud) instead of a ball that stood 10 mm off the shin.
- Print suitability (new `PRINT` rule, catalog was INVALID): 1:500 hands and toe caps drawn a little fuller, the 1:500 range ends at 1:650 (the visible width of the clasped hands behind the forearms falls under 0.2 mm beyond); the 1:500 hand bones grow to 49 mm (forced by `volumes-feature` through the thinnest hand drawing).
- 2D redrawn by the pipeline from the new solids (ruling E1), ponytail shapes now on bone `ponytail`.

### Checks

`r3-validate.ts humans/standing-phone-user --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts --product`: every rule `ok` at every scale except `volumes-thermal-coverage` at 1:50 / 1:100 on `jeans` (+45 %), `sneakers` (+75 %), `headphones` (outer 0.078 against 0.0208): C18 / C16 values the author may not change, unchanged since round 3 and listed in `area-review.md` and `author-requests.md`.

### Seen (opened)

1:50 body sheet, three-quarter close-up, right side in `texting`, `phone-call`, `photo-pose`, `answer-call` at 1.5 s; 1:100, 1:200, 1:500 body sheets; 1:50 structure and thermal cut; 2D 1:50. Rendered, not opened: 1:100 to 1:500 structure and thermal sheets after the change, 2D 1:200 and 1:500.

### Remaining

- Hairline is a planar cut (reads slightly like a cap from the side); face plain; lofts are superellipses without cloth folds at elbows; fingers capsule chains. Grade A- not A at 1:50 for these reasons.
- Knee joint 2.4 % below 0.285 H (de Leva segment lengths kept).
- Three thermal areas red (C18, C16, area reviewer).
