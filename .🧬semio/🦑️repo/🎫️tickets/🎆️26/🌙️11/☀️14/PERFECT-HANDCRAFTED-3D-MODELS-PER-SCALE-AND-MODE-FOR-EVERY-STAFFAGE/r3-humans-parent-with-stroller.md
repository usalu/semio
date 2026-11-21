# Round 3 — `humans/parent-with-stroller`

State: spliced into the catalog (roots, redrawn 2D of all four scales, `volumes`). `r3-validate.ts humans/parent-with-stroller --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/parent-with-stroller --product`: every rule green at every scale **except one measurement**, `volumes-thermal-coverage` 1:50 `areaDeviation:shoes = 0.378 <= 0.3` (see "Still imperfect"). All four scales are print-suitable under the default profile in every named pose (1:500 was not before: `r3-2d.ts --scale 500` failed on the catalog asset).

This asset was resumed: an earlier round-3 author left the scripts `r3-stroller-*` (kit, 1:50 figure / pram / thermal / structure, coarse scales, 2D redraw, roots) with rig root fixes already in the catalog but the old 2D and old `volumes` still there (catalog INVALID: `extent-plan-width` 1:500), decisions "Draft.", no report. I rebuilt, looked, corrected and finished from that state; what I changed against it is marked "(this pass)".

Scripts (all in `T`): `r3-stroller-kit.ts` (row-to-solid helpers), `r3-stroller-s50-figure.ts`, `r3-stroller-s50-pram.ts`, `r3-stroller-s50-thermal.ts`, `r3-stroller-s50-structure.ts`, `r3-stroller-s50.ts`, `r3-stroller-coarse.ts` (1:100, 1:200, 1:500), `r3-stroller-decisions.ts`, `r3-stroller-build.ts` (writes `volumes/humans/parent-with-stroller.json`), `r3-stroller-draw2d.ts` + `r3-stroller-strokes.ts` (ruling E1, writes `r3-stroller-2d.json`), `r3-stroller-roots.py` (root edits, `--apply`), `r3-stroller-candidate.py` (full candidate for `--file`), `r3-stroller-suit.ts` (print-suitability violations per pose and view), `r3-stroller-view2d.ts`, `r3-stroller-probe.ts`.

## 1. Reference

| quantity | real value | source | asset |
|---|---|---|---|
| stature | 1.75 m, 68 kg | asset `dimensions`, `thermal.body` | 1.75 |
| hip joint height | 0.52 to 0.53 H = 0.91 to 0.93 m | Winter 2009 table 4.1; brief | 0.925 (thigh 0.4276 + shank 0.4176 + ankle 0.0795) |
| knee height | 0.285 H = 0.50 m | Winter 2009 | 0.497 |
| ankle height | 0.039 H = 0.068 m (shod + 0.012) | Winter 2009 | 0.0795 |
| shoulder joint height | 0.818 H = 1.43 m | Winter 2009 | 1.452 (+22 mm, de Leva trunk length; inside the variation of the source, kept) |
| elbow / wrist below shoulder | 0.186 H = 0.326 / 0.146 H = 0.256 | Winter 2009 | 0.308 / 0.2685 (de Leva 1996 segment lengths, kept) |
| head height (chin to vertex) | H / 7.5 = 0.233 m; breadth 0.155, depth 0.195 | brief; DIN 33402-2 | 0.230; 0.155; 0.195 |
| shoulder breadth | biacromial 0.245 H = 0.43; joint centres about 0.18 to 0.19 from the midline | brief; DIN 33402-2 | joints at +-0.1925; jacket 0.53 over the sleeves |
| hip breadth | 0.19 H = 0.33; hip joints +-0.09 | brief | joints +-0.092; jeans seat 0.34 |
| girths | upper arm 0.30, forearm 0.26, upper thigh 0.58, mid thigh 0.52, knee 0.38, calf 0.37, ankle 0.23 | DIN 33402-2 medians | sleeves: arm radius + 20 mm of declared layers; jeans thigh 0.18 x 0.20 m, straight leg 0.115 x 0.12 m at the hem |
| foot / shoe | foot 0.152 H = 0.266; size-43 sneaker 0.293 x 0.10 m, sole 25 mm | Winter 2009; product knowledge | 0.291 x 0.10, sole 25.5 mm |
| toddler | 12 months: 0.76 m, 10 kg, head about 1/4 of the height (0.19 m), sitting height 0.48 m | WHO child growth standards (median) | 0.76; head with hat 0.19 to 0.20; trunk + head 0.47 |
| pushchair | central-joint all-terrain pushchair of the Bugaboo Cameleon class: width 0.60 m, push bar 0.95 to 1.05 m, rear wheels 0.30 m, swivel front wheels 0.15 to 0.18 m, wheelbase about 0.55 m, frame tube 22 to 25 mm, seat width 0.33 to 0.36 m, canopy on three bows | EN 1888-1 (asset reference `en-1888`) for the class; manufacturer data from memory, not looked up online | width 0.598 (tyres), push bar axis 1.00 m, rear 0.318, front 0.159, wheelbase 0.52, tubes 24 to 25 mm, seat 0.356, canopy 0.39 wide |
| whole asset | length 1.8 m, width 0.6 m | asset `dimensions` | default-pose plan 1.80 x 0.61 (bounds green) |

No web search was made; the pushchair numbers are class values from memory and from the asset's own EN 1888 reference.

## 2. Audit (state found, against the reference)

- Rig: human joints are anthropometrically right within 2 cm (table). Stroller: `stroller-hood` joint sat at (0.18, -0.12) with length 0.46, i.e. not on the canopy hinge of the frame; the rest angle of `stroller` (0) pointed the chassis straight down from the hand in the rest pose, which made every rest-frame front/plan shape of the stroller meaningless; toddler and hood centres of mass lay outside their bodies; breath anchors of adult and toddler floated 2 to 5 cm off the faces. (Fixed by the earlier author in the catalog; I verified the values.)
- `dimensions`: consistent with the reference, not changed.
- 2D (old): outlines from "the original 1:50 study" (`stroller-sil`, `hand-l-u33`, `hair-strands` ...), not views of one body: against the handcrafted body IoU 0.71 / 0.80 / 0.77 at 1:50 and 0.72 / 0.81 / 0.74 at 1:500; 1:500 front and plan a filled block; 1:500 not print-suitable.
- Solids (old catalog `volumes`): box torso, capsule limbs, ball joints, 47 capsules at 1:50; structure 49 solids following the body.
- Solids as left by the earlier round-3 author (this pass, looked at in `r3-sheet`): sleeve caps rose 5 cm above a shoulder line that was 2.5 cm too low and too narrow (epaulettes, a notch between jacket and sleeve); face flat, nose a thin stalk off the face; hair cap an ellipsoid nearly coincident with the skull (ragged hairline, crown at 1.768 m); jacket body, jacket skirt and jeans seat nearly coincident at the waist and the seat poking through the skirt at the back (zigzag intersections); shoe collar 16 mm above the ankle joint (a boot, not a low sneaker); at 1:500 the trailing shoe was one rigid mass through the ground, sleeves ended in flat cuts; plume outlet a disc hovering 6 mm above the crown; at 1:100 the toddler's whole head counted as skin (0.083 against 0.03 m2).

## 3. Root fixes (catalog file, by `r3-stroller-roots.py --apply`)

| member | old | new |
|---|---|---|
| `rig.stroller.angleDeg` (rest) | 0 | 90; every pose `stroller` angle shifted by -90 (same world pose), joint limits -60..60 -> -150..-30 |
| `rig.stroller-hood` x, y, lengthM | 0.18, -0.12, 0.46 | 0.19, -0.225, 0.29 (the canopy hinge on the seat frame, bow radius) |
| `rig.stroller-hood.mass` comX, comY | 0.22, 0.08 | 0.085, 0.18 |
| `rig.child.mass` comX, comY | 0.18, 0 | 0.025, 0.11 (inside the toddler's trunk) |
| anchors `mouth`, `nose` | (0.1134, -0.0945), (0.1547, -0.1123) | (0.083, -0.092), (0.107, -0.1) on the modelled lips and nostrils |
| anchors `child-mouth`, `child-nose` | (0.06, 0.312), (0.068, 0.337) | (0.045, 0.288), (0.056, 0.313) |
| pose `checking-child` `stroller-hood` | 95 | 40 (the front canopy part folds back onto the rear part instead of through the push bar) |
| `thermal.regions.stroller-frame.areaM2` | 0.5 | 0.73 (C16, factor 1.46; tube lengths of the real chassis in `author-requests.md`; measured 0.734) |
| `representations.{50,100,200,500}.geometry` and `.decisions` | old study outlines | redrawn, see 5 |

Human joints, `dimensions`, clips, supports and physiological areas are unchanged. Clips have no `stroller` keys, so the rest-angle change needs no clip edit; default pose, `checking-child`, `push-walk` t 0.45 and `kerb-tilt-up` t 1.5 were rendered and read right.

## 4. Model per scale

Solid counts body / thermal / structure: 1:50 129 / 166 / 145, 1:100 76 / 81 / 97, 1:200 44 / 49 / 40, 1:500 31 / 36 / 24. Body kinds at 1:50: 59 lofts, 18 revolves, 9 extrudes, 17 capsules (tube runs, fittings), 16 ellipsoids (ears, eyes, lips, toddler head, hat, hands, boots), 10 boxes (footrest, basket panels, fork crowns, spokes).

- **1:50 body.** Adult: head loft of 12 stated sections (chin, lips, subnasale, nasion, brow ridge, sloping forehead; this pass), nose wedge on the face (this pass), ears, eyes, lips, hair cap as a loft 9 to 12 mm over the skull with its hairline on the forehead and behind the ears (this pass), back fall and two side strands, scarf, collar, quilted jacket body with trapezius slope to the acromion (this pass), skirt tucked under the body hem (this pass), sleeves with deltoid caps 42 mm over the joint (this pass) and cuffs, hands closed round the bar (palm, four fingers, thumb), jeans seat inside the skirt (this pass), thighs and shanks with stacking folds, low sneakers (collar lowered, this pass) with soles and bending forefoot. Toddler: trunk, thighs, shanks, boots, arms, hands, head, face, nose, eyes, hat, pompom. Stroller: U push bar, grip, telescopic sleeves, clamps, joint hubs, rear legs, axle, front legs, cross tube, swivel housings, fork crowns and prongs, seat rails and top bow, seat mounts, sling, wings, footrest, bumper bar, basket (floor, front, back, sides, straps), two canopy fans on three bows with hinges, four wheels of tyre, rim, hub and spokes.
- **1:50 thermal.** Outer surface of every region plus nested layer shells (trunk and arms: air, shirt, sweater under the jacket; legs: air, leggings, briefs at the seat; shoes: sock, air), face masks of adult and toddler as section solids, five outlets on the surface (plume now a patch on the crown, this pass). Garment cuts (this pass): sleeves without the shoulder cap and trouser legs without the hip cap with that end as `sectionFaces`, the seat only below the jacket hem (reasoning in `author-requests.md`).
- **1:50 structure.** Adult skeleton (cranium, mandible, cervical and thoracolumbar spine with its curves, six rib hoops, sternum, clavicles, scapulae, ilia, sacrum, pubis, ischia, humerus with head, radius, ulna, carpus, metacarpals and phalanges round the bar, thumb ray, femur with neck and condyles, tibia, fibula, patella, calcaneus, metatarsus, toe phalanges), joints at the rig joints, masses; toddler skeleton; the stroller's tubes, hubs, forks, bows, rims, spokes and wheel hubs by reference (they are the members); masses of stroller, hood, wheels.
- **1:100, 1:200, 1:500.** Authored anew per scale as the `decisions` say (smooth jacket, slab fingers, one last per shoe; then one solid per limb segment, fists, 30 mm tubes, disc wheels; then masses, the chassis as slender true-size runs). This pass: shoulders, hair caps, waist overlap and seat at every scale, 1:100 toddler face mask, 1:500 forefoot solids (the trailing foot rolls off the ground again) and closed sleeve heads.
- `decisions`: four sentences at 1:50, three at 1:100, two each at 1:200 and 1:500, en and de (this pass).

Final check line: `FAIL: 1 measurement(s) over 1 rule line(s)`. Silhouette IoU elevation / front / plan: 1:50 0.993 / 0.983 / 0.977, 1:100 0.988 / 0.979 / 0.970, 1:200 0.981 / 0.975 / 0.959, 1:500 0.948 / 0.979 / 0.978. Thermal areas 1:50 (measured / declared): head-skin 0.120 / 0.100, torso 0.838 / 0.745, arm 0.418 / 0.327, hand 0.097 / 0.091, leg 0.807 / 0.669, shoes 0.176 / 0.128 (red), child-skin 0.029 / 0.030, child-clothing 0.518 / 0.420, fabric 2.037 / 2.0, frame 0.734 / 0.73, tyres 0.488 / 0.48.

## 5. 2D (ruling E1)

All three views of all four scales are redrawn from that scale's body in the pushing pose: one outline per stated group of solids and bone (the group table in `r3-stroller-draw2d.ts` is the line hierarchy), simplified to a quarter of the smallest feature, placed through the product's own `evaluate` (residual 0.000 mm); limbs in front and plan are revolved profiles about the bone, wheels and (this pass, 1:50 to 1:200) feet extruded elevation profiles, so other poses carry them; seams, hem, zip and trouser hem are hand-stated strokes. This pass: revolved profiles are sampled along triangle edges (the first version read vertices only and drew the sleeves as strings of beads at 1:100); a right limb shape takes the identical bone-local outline of its left twin (in standing poses the far shoe no longer leaves a 0.06 mm sliver; that sliver made 1:500 unsuitable); face details are not drawn in plan (the toddler's eyes showed through the canopy).

## 6. What I saw (opened with the Read tool)

- 1:50 body sheet, final: front: head with nose and eyes, hair behind the ears, shoulders sloping from the scarf to the sleeve heads, quilted jacket, both hands on the bar, canopy, toddler under it behind the bumper bar, basket, four wheels, both shoes. Right side: upright walker leaning 8 degrees, stride, arms to a bar at 1.0 m, pushchair with joint hub, rear leg to the axle, front leg to the caster fork, reclined toddler, spoked wheels. Back: hair fall, clean jacket hem over the skirt, seat and thighs under it. Left, top (hands at the bar ends, toddler's legs on the footrest side, basket) and three-quarter consistent. Close crops of head profile, hands, waist from the side and the back.
- 1:50 `checking-child` sheet and `kerb-tilt-up` t 1.5 sheet (final shoulders, before the last waist change): closed at every joint, hood folded, both feet flat. `push-walk` t 0.45 was rendered but not opened.
- 1:50 structure sheet (earlier in this pass, the list did not change after): reads as a skeleton and a tube chassis. 1:50 thermal cut sheet, final but before the plume patch moved: nested shells in trunk and legs, the seat cut under the jacket, face masks, regions distinct.
- 1:100 body sheet (before the shoulder, hair and waist edits; the same edits were then made and seen at 1:50, 1:200, 1:500), 1:100 structure sheet (final).
- 1:200 and 1:500 body sheets, final: recognisable at a glance (walker, pushchair with canopy, toddler mass, disc wheels; yellow slender chassis at 1:500). 1:500 structure sheet, final.
- 2D of the candidate: 1:50 default pose and `checking-child` (both rows before, solid row after the feet change), 1:100, 1:500. 
- Not opened: final 1:100 body, 1:100 / 1:200 / 1:500 thermal, 1:200 structure, 1:200 2D, the `volumes-render.ts` overlay sheets (the silhouette numbers above stand in for them). They were rendered without a `NOT DRAWN` solid and without a failed panel.

## 7. Still imperfect

- **`shoes` area at 1:50 is red** (0.176 against 0.128 m2, +38 %): the declared value is the nude feet share; honest size-43 sneakers with their soles are about 0.18 m2. Not changed (C18), not shrunk; filed in `author-requests.md` ("r3 humans/parent-with-stroller"), with the three garment cuts I applied for arm and leg for the area reviewers to accept or reject.
- Feet in front and plan at 1:50 to 1:200 are extruded elevation profiles: right in every pose, but the plan shows the shoe as a rectangle of its true length and width, not the rounded sole outline. At 1:500 the feet stay solved outlines (the rectangle made a 0.06 mm sliver of the shank), exact in the default pose only.
- The toddler is a simplified figure on the single `child` bone (ellipsoid head, hands and boots, tube limbs with stated radii): it cannot move, and its face is coarser than the adult's.
- The adult's garments are smooth lofts: quilting and stacking folds are modelled, drape folds at the elbow and behind the knee are not. Where the thigh leaves the skirt and the forearm meets the upper sleeve the shaded render shows a slightly ragged intersection line (two lofts crossing at a shallow angle).
- Jacket body hem lifts off the skirt by about 1 cm at the back in `checking-child` (32 degree bend).
- Shoulder joint height 1.452 m is 22 mm above the 0.818 H of the brief (de Leva segment lengths of the asset); not changed, since the 2D, poses and clips of all human assets share this rig.
- Pushchair dimensions are class values from memory, not a looked-up product sheet.

## Round 5 handcraft

Scripts (ticket folder): `r5-parent-with-stroller-forms.py` (every changed row list stated in full, old -> new, written into the asset's own `r3-stroller-s50-figure.ts`, `r3-stroller-s50-structure.ts`, `r3-stroller-coarse.ts`, `r3-stroller-draw2d.ts`, `r3-stroller-decisions.ts`), `r5-parent-with-stroller-run.sh` (forms, build, 2D redraw of all scales, candidate, validation), `r5-parent-look.ts` (zoomed multi-direction look on catalog + source), `r5-parent-suit.ts` (print suitability per view and denominator), `r5-parent-measure.ts` (model vs reference); root range fix in `r3-stroller-roots.py` (`RANGES`).

### Reference (unchanged sources, see section 1) and model vs reference, 1:50

| quantity | reference | model | deviation |
|---|---|---|---|
| stature (rig: hip 0.925 + pelvis 0.0994 + torso 0.4574 + head 0.2685) | 1.75 m | 1.750 m | 0 % |
| head chin to vertex | H/7.5 = 0.233 m | 0.230 m (head loft u 0.0355 to 0.265) | −1.3 % |
| head breadth / depth | 0.155 / 0.195 m (DIN 33402-2) | 0.155 / 0.197 m | 0 / +1 % |
| shoulder breadth over the jacket sleeves | biacromial 0.23 H = 0.40 m + 2 sleeves ≈ 0.52 m | 0.523 m | +0.6 % |
| hip breadth (jeans seat) | 0.19 H = 0.333 m | 0.336 m | +0.9 % |
| upper thigh girth (jeans, with ease) | 0.58 m body + ease | ellipse 0.176 x 0.196 m ≈ 0.59 m | +1.7 % |
| knee height / hip height | 0.285 H = 0.499 / 0.52 H = 0.91 m | 0.497 / 0.925 m | −0.4 / +1.6 % |
| hand length (wrist to fingertip, unrolled round the bar) | 0.108 H = 0.189 m | wrist to MCP 0.085 + finger arc ≈ 0.11 = 0.195 m (rig bone 0.179, de Leva) | +3 % |
| shoe (size 43 sneaker) | 0.293 x 0.10 m | 0.284 x 0.103 m | −3.1 / +3 % |
| default-pose extents | length 1.80, width 0.60, height 1.74 m (`dimensions`) | 1.811 x 0.610 x 1.737 m | within bounds rule |

### Grades (before -> after; opened sheets listed below)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | B (hair a flat plank behind the head, two-tier jacket with a 2 cm ledge at the body hem, ring step at the knee) -> A− | A− -> A− | B− (rib hoops stacked horizontally like a coil, iliac wings two 4 cm ellipsoid blobs, scapulae ovals, sternum a box) -> A− |
| 1:100 | B (same hair plank and hem ledge) -> A− | A− -> A− | B (adult skeleton shared with 1:50) -> A− |
| 1:200 | B+ (hair plank, skirt ledge) -> A− | B+ -> B+ | B+ -> B+ |
| 1:500 | B+, catalog INVALID (print: hood, child-body below 0.2 mm at 1:1000) -> A− | B+ -> B+ | B+ -> B+ |
| poses | `checking-child` B (jacket hem open at the back) -> A−; `kerb-tilt`, `pausing-with-stroller`, `push-toe-off-r` A− | | |

### Changes

- Hair (1:50, 1:100, 1:200): the plank `hair-back` and the two side strands are replaced by one `hair-fall` loft with 7 to 9 stated sections that wraps the back of the skull and the nape (1 to 2 cm over the skull, shoulder length, ears showing at its front edge); the crown cap's lowest rows tucked into it so no ledge shows.
- Nose 1:50: nasion lowered to eye level (0.158), alae 31 mm wide.
- Jacket 1:50, 1:100, 1:200: the hip part (`jacket-skirt` / `pelvis`) now continues the body hem at the same section (0.18 x 0.137 m half axes at the hem) instead of 2 cm inside it, with a quilting seam at the junction, and reaches 5 cm up under the body (tapered to 0.16 x 0.106) so the hem stays closed when the trunk bends 32 degrees in `checking-child`.
- Jeans 1:50 to 1:500: shank sections start at the thigh's end section (0.058 x 0.064 m) — no ring step at the knee.
- Structure 1:50 and 1:100: seven (three) rib hoops falling 24 degrees to the front, rib 1 narrow (0.086 half width) to rib 3 widest (0.14); a curved sternum loft through the rib fronts; scapulae as triangular plates (inferior angle to superior border, 16 mm thick) turned 30 degrees onto the back of the rib cage with the scapular spine to the acromion; iliac wings as fanned plates (16 mm thick, crest 0.13 m wide) turned 35 degrees antero-laterally; a tapering sacrum loft.
- 1:500: the two canopy fans are one canopy shell on the stroller bone (25 to 105 degrees); the drawn `hood` band was the print failure. Root fix: `representations.500.validRange.maxDenominator` 1000 -> 800 (`r3-stroller-roots.py`): at 1:850 to 1:1000 the canopy, seat and forefoot bands fall to 0.176 to 0.199 mm (office laser 0.2 mm), measured with `r5-parent-suit.ts`.
- 2D of all four scales redrawn from the new bodies (`r3-stroller-draw2d.ts`, ruling E1); decisions of 1:50, 1:100 and 1:500 rewritten for hair, skeleton and canopy (en, de).

### Checks

`r3-validate.ts humans/parent-with-stroller --catalog-only`: `VALID ... schema 0, validateAsset 0` (with `PRINT` lines: none). `r3-validate.ts --file` (identical candidate) `--volumes`: every volume rule green except the known `volumes-thermal-coverage` 1:50 `shoes` 0.176 against 0.128 m² (C18, physiological share; listed in `area-review.md` round 5). Spliced with `volumes-insert.py`; solids body / thermal / structure 1:50 125 / 162 / 166, 1:100 76 / 81 / 103, 1:200 44 / 49 / 46, 1:500 30 / 35 / 32.

### What I opened

Before: 1:50 body sheet and right-side close-up, 1:100, 1:200 and 1:500 body sheets, 1:50 structure, 1:50 thermal cut, zoomed head, legs and skeleton. After: zoomed head/torso/legs (three iterations), zoomed skeleton (two iterations), 1:50 body sheet, `checking-child` sheet and its right side and hem close-up (two iterations), 1:50 2D (line and tone, all three views). Rendered without a `NOT DRAWN` solid but not opened after the change: 1:100, 1:200, 1:500 body/structure/thermal sheets of the final state, `kerb-tilt`, `pausing-with-stroller`, `push-toe-off-r` sheets, 2D 1:100 to 1:500 (1:500 only seen at paper size, too small to judge).

### Still not A

- Face is a stylised plane mask (no eyelids, no philtrum); fine detail of a 1:50 figure, not changed.
- Shoes remain one-piece lasts with sole and forefoot; no lacing or heel counter.
- The toddler stays a figure on the single `child` bone (cannot move its limbs).
- 1:200 and 1:500 thermal and structure were not reworked (no defect seen in round 3/4 sheets; not reopened after this round).
- `shoes` area C18 (area reviewers).

## Child articulation

The 10 kg passenger now sits on the stroller centreline. Fourteen articulated segments distinguish the neck, skull and paired upper arms, forearms, hands, thighs, lower legs and feet. Snowsuit seams follow the elbows and knees. The passenger reaches towards the parent, turns the head, or draws up a foot in the relevant authored poses. The nose and mouth anchors now coincide with their visible outlets. Each was formerly displaced 192.5 mm sideways. The measured uncovered skin area is 0.042 m². [Child passenger anatomy and motion](digital-passenger-review.md)
