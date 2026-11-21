# Round 3 — furniture/folding-deck-chair

State: DONE

Asset: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/folding-deck-chair/🔣️.json` (index row in `🗂️catalog/🔣️.json` updated: `heightM`, `widthM`).
No earlier round-3 work existed; the catalog `volumes` equalled `volumes/furniture/folding-deck-chair.json` at the start (no bulk-pass change on this asset).

Scripts (all hand-stated, kept): `r3_folding_deck_chair_geom.py` (chair, kinematics, profiles), `r3_folding_deck_chair_parts.py` (part writer: one statement per part gives the solid, its thermal and structural references and its 2D shapes), `r3_folding_deck_chair_scales.py` (the four scales), `r3_folding_deck_chair_roots.py` (rig, anchors, poses, clips, thermal, structure, symbol, 2D decisions), `r3-folding-deck-chair-build.py [--apply]` (volumes source, candidate document, catalog write), `r3-folding-deck-chair-print.ts` (print suitability probe).

## 1. Reference

| quantity | real deck chair | source | model |
|---|---|---|---|
| height of the head end, by notch | 0.78 to 1.06 m | Harbour Housewares beech deck chair (rear height 78-106 cm), Wimborne beech 0.80 m, DIY.com 0.92 m | 0.86 / 0.90 / 0.94 / 0.97 m (notch 1 to 4), default 0.936 m |
| height of the knee rail | 0.33 to 0.44 m | Harbour Housewares (front height 33-44 cm), colonial beech chair seat height 0.42 m | 0.38 to 0.43 m, default 0.41 m |
| length unfolded | 0.90 to 1.08 m | Wimborne 1.01 m, colonial 1.02 m, Donner 1.08 m, DIY.com 0.90 m | 1.016 m |
| width | 0.56 to 0.66 m | Wimborne and Harbour 0.56 m, Donner 0.62 m | 0.58 m over the prop arms, 0.597 m over the dome nuts (`depthM` 0.59) |
| mass | 4.0 to 4.4 kg (beech, one 9 kg outlier with cushion) | DIY.com 4 kg, Wimborne 4.4 kg | 4.85 kg (timber volume of the modelled joinery x 720 kg/m3 + canvas + bolts) |
| construction | three hinged rectangular frames: long back frame from the front feet to the head rail, seat frame from the rear feet to the knee rail, pivoted like scissors; a short prop frame hinged on the back frame whose cross bar drops into one of several notches in the upper edge of the rear part of the seat rails; rails of about 20 x 40 mm beech on edge with rounded ends; canvas sling about 0.43 m wide lapped and tacked round the head rail and the knee rail; the canvas is as long as the rail distance of the folded chair, so it lies taut when folded and hangs as a deep sling when open | own knowledge of the traditional British deck chair, consistent with the trade pages above | modelled as stated in section 4 |

Trade pages found by web search (2026-11-20): willowandstone.co.uk (Wimborne deck chair, beech), rinkit.com (Harbour Housewares beech deck chair), chairish.com (colonial beech deck chair), wayfair (Donner), diy.com (traditional folding wood deck chairs). None states rail lengths or the sling cut size; those follow from the construction above and were fitted to the cited overall dimensions.

## 2. Audit of what was there

- **Rig (root defect).** `rear` ran from the REAR foot up and FORWARD (62 degrees) to the top rail at x = 0.49 m, the canvas hung from there backward and down to a sag at x = 0.32 m: the "back" of the chair leaned forward by 0.17 m over 0.58 m, nobody can sit in it. The real chair has the head rail behind the sag and the back frame standing on the FRONT feet. The seat frame was a 0.495 m stub hinged at the scissor pivot; in reality it is the second leg of the scissors and stands on the rear feet. The prop frame with the notches, the part that makes a deck chair adjustable, did not exist; a prismatic `front-slide` bone stood in for it.
- **Dimensions.** 0.787 m long (real about 1.0 m), 5.95 kg (real 4 to 4.4 kg), members 45 x 45 mm in the 1:50 body against "25 x 45 mm" in the thermal layer (real about 20 x 40 mm).
- **2D drawings.** All four scales drew that wrong object: three bars as an A with a stub, no prop, no cross rails on the ground, canvas as a V leaning forward.
- **Solids.** 9 body solids at 1:50 (three box pairs, two dowels, two bolts, two canvas bands), structure with cubes for joints and masses; coarser scales bars and slabs. No notches, no prop, no foot rails, no laps of the canvas, frames not nested (270 / 235 / 180 mm).
- **Poses.** `folded` stood upright on four feet at 84 degrees, which a deck chair cannot do.

## 3. Root fixes (old -> new)

- `dimensions`: heightM 0.93 -> 0.936, widthM 0.787 -> 1.016, depthM 0.58 -> 0.59, massKg 5.95 -> 4.85 (reference table).
- `rig`: bones `ground, rear, front-slide, front, seat, canvas-a, canvas-b` -> `ground` (0.02 m marker, no longer a member), `seat` (seat frame from the rear foot round, 23.39 degrees, 1.04 m), `back` (child of `seat` at the pivot 0.565 m, 106 degrees to the seat frame, 0.90 m to the head end, 0.30 m down to the front feet), `strut` (prop, child of `back` 0.45 m above the pivot, 0.4906 m to the dowel), `canvas-a`, `canvas-a2` (back canvas), `canvas-b`, `canvas-b2` (seat canvas): four inextensible canvas bones of 0.4815, 0.290, 0.227, 0.2665 m whose sum 1.265 m is the rail distance of the folded chair. Frame bones carry real sections (two rails 20 x 40 mm, prop arms 20 x 32 mm) and masses from the joinery (back 2.07, seat 1.72, prop 0.82, canvas 0.24 kg).
- `anchors`: the four foot contacts moved to the lowest points of the real foot rounds (rear feet on `seat`, front feet on `back`), footprint 20 x 20 mm; new contacts `head-end-left/right` (the head ends of the back rails, on which the folded chair lies); `sling` load anchor at the sag (tip of `canvas-a2`).
- `poses`: all five re-solved by `r3_folding_deck_chair_geom.py`: `unfolded` third notch (frame angle 74 degrees), `reclined-seated` first notch (66.2 degrees) with the sling pulled into two straight runs to the sitting point, `sling-proof-load` third notch with the loaded sling, `half-folded-key` 37 degrees with the prop trailing on the ground, `folded` flat on the ground (frames coplanar, 1.43 m long, resting on the head ends and the planted foot rail, canvas taut). `clips`: texts, and contact spans for the head ends at the end of `fold-chair` and the start of `unfold-chair`.
- `thermal`: areas by hand calculation of the corrected object, each within a factor 1.5 of the old value (C16): frame-timber 0.88 -> 0.90 m2 (7.5 m of rail with 0.12 m girth, dowel, end grain; measured 0.895), canvas-back 0.49 -> 0.73 m2 ((0.772 + 0.10 lap) x 0.42 x 2; measured 0.771 with the edges of the 7.5 mm band), canvas-seat 0.368 -> 0.50 m2 ((0.493 + 0.10) x 0.42 x 2; measured 0.532). Timber layer 25 x 45 half thickness 12.5 mm -> 20 x 40 half thickness 10 mm (resistance 0.0556 m2K/W, 11520 J/m2K); characteristic lengths follow the parts.
- `structure.assumptions`, `summary`, `provenance.notes`: rewritten for the real mechanism.
- `representations` 1:50, 1:100, 1:200, 1:500 and `symbols[0]`: redrawn (ruling E1): every shape is the side profile of a body part of that scale in its bone frame; front and plan are the same profiles with their lateral spans, so they are the true views by construction (silhouette IoU 0.990 to 1.000 in every view). Decisions rewritten. Canvas pens raised to 0.18 mm at 1:100 and 0.2 mm at 1:500 so every shape meets the 0.2 mm print feature.

## 4. What is modelled

| scale | body | thermal | structure | content |
|---|---|---|---|---|
| 1:50 | 24 | 24 | 30 | Three nested frames of 20 x 40 mm rails with half-round ends and 2 mm washer gaps (prop 270-290, back 248-268, seat 226-246 mm), four round-bottomed 24 mm notches in the seat rails, 32 mm prop arms with a 22 mm chamfered dowel in the third notch, four cross rails 40 x 20 mm with 4 mm arrises tenoned into the rails, the foot rail planted on the rear edges of the front legs (so the seat frame folds through), four turned bolts with low heads, gap washers and dome nuts, the sling as the true unloaded catenary (1.277 m) in four panels with nine woven stripes, lapped as a U round head rail and knee rail. |
| 1:100 | 20 | 20 | 30 | Same frames at true size as plain joinery: coarse half-round ends, square notches, sharp cross rails, plain dowel and 15 mm pins, sling 15 mm with eleven facets and stripes, no laps. |
| 1:200 | 12 | 12 | 28 | Bars 40 x 30 mm with chamfered ends side by side (260-290 / 230-260 / 200-230 mm), four cross bars and the prop bar, sling 30 mm in four sheets 380 mm wide. |
| 1:500 | 10 | 10 | 26 | Square 40 mm bars with chamfered ends (250-290 / 210-250 / 170-210 mm), one cross bar per frame, sling as four straight 40 mm sheets. |

Thermal: every timber and steel part references the body in `frame-timber`, back panels and head lap in `canvas-back`, seat panels and knee lap in `canvas-seat` (single-layer regions, no emitters, no hidden regions). Structure: rails, cross rails, prop arms and dowel are the members by reference (they are the load path); four full-width canvas tension members; joints are the pivot and prop bolts (sleeves at 1:200 and 1:500), the notch seat, the rear foot rounds and five canvas hinges; each frame mass is two half masses inside its rails at the centre of mass; four pads under the foot rounds.

Final checks (2026-11-20): `volumes-check.ts furniture/folding-deck-chair --product` PASS (all 43 rule lines ok); `r3-validate.ts furniture/folding-deck-chair --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0), the same with `--catalog-only`; print suitability suitable at all four scales (smallest feature 0.248 / 0.212 / 0.258 / 0.232 mm against 0.2 mm). Spliced with `volumes-insert.py`.

## 5. What I saw in the final renders

- 1:50 body, six directions (`r3-sheet`): from the side the crossed frames, the prop standing almost vertically under the back with its dowel in the third notch and two free notches behind and one in front, the sling leaving the head rail, hanging behind the back rails and rising to the knee rail, lowest point 0.27 m above the ground; from the front and back the striped canvas between the three nested frames, the two ground rails low; from above the frames stepping inward and the stripes; three-quarter view reads as a deck chair at first sight.
- 1:50 poses: `reclined-seated` lower and flatter with the sling as two straight runs meeting at the sitting point; `half-folded-key` and `fold-chair` at 3 s with the scissors sinking and the prop trailing; `folded` flat, head ends and foot rail on the ground line, canvas lying on the frames with 18 mm waves, prop extending past the head end.
- 1:50 thermal: frame red, back canvas green, seat canvas violet, change of region at the sag. 1:50 structure: the frame as blue members with orange bolts and hinges, green pads, body ghost only visible where the stripes were.
- 1:100, 1:200, 1:500 body, thermal 1:200 and 1:500, structure 1:200 and 1:500, 1:100 folded, 1:500 reclined: each scale is its own simplification and still the same chair in proportion; feet stand on the ground line after the square bar ends were chamfered.
- 2D (`r3-2d`) at 1:50, 1:100, 1:200, 1:500 and 1:50 folded: side view with notches, bolts and planted foot rail, front and plan with stripes at 1:50 and 1:100, a clean sign at 1:500.

## 6. Limits that remain, stated plainly

- The 3 mm arris rounding of the rails, tacks, hems and screws are not modelled: they are below the 7.5 mm feature of 1:50.
- The canvas is 7.5 / 15 / 30 / 40 mm thick instead of 0.7 mm (the print floor of each scale); its edges add about 5 % to the measured canvas areas.
- The rig is an open tree: the notch closure and the ground contact are exact at the five poses and interpolated between clip keys. In `folded` the four feet stay listed as contacts (supports are permanent) while the chair really lies on the head ends and the foot rail; the rear foot rounds are 7 mm above the ground there.
- The mass 4.85 kg is 10 % above the heaviest cited plain beech chair (4.4 kg); it is the honest volume of 20 x 40 mm rails.

## Round 5 handcraft (2026-11-21)

No new script: the round-3 model was judged against the reference again and left unchanged (catalog VALID, `volumes` equal to `volumes/furniture/folding-deck-chair.json`).

### Reference (beech deck chairs: Harbour Housewares, Wimborne, colonial beech, Donner, DIY.com; traditional construction)

| Quantity | Reference | Model 1:50 | Deviation |
| --- | --- | --- | --- |
| Head-end height by notch | 0.78-1.06 m | 0.86-0.97 m, default 0.936 m | inside |
| Knee-rail height | 0.33-0.44 m | 0.38-0.43 m, default 0.41 m | inside |
| Length unfolded | 0.90-1.08 m | 1.016 m | inside |
| Width | 0.56-0.66 m | 0.58 m (0.597 over the nuts) | inside |
| Rails | about 20 x 40 mm beech on edge, rounded ends | 20 x 40 mm, half-round ends | 0 % |
| Sling | about 0.43 m wide, length = rail distance folded | 0.42 m, 1.265 m (four canvas bones) | -2.3 % |
| Mass | 4.0-4.4 kg | 4.85 kg (honest timber volume) | +10 % to the heaviest cited chair |

Rig: seat frame from the rear foot round, back frame hinged at the scissor pivot, prop hinged on the back frame with its dowel in a notch, four inextensible canvas bones (unchanged since round 3, checked against the solids and poses).

### Grades (opened with the Read tool)

| Scale | Body | Thermal | Structure |
| --- | --- | --- | --- |
| 1:50 | A- (crossed frames, notched rails, prop and dowel, striped sling lapped round head and knee rails) | A- | A- |
| 1:100 | A- | A- | A- |
| 1:200 | B+ | B+ | B+ |
| 1:500 | B+ (crisp square bars, the sling four straight sheets) | B+ | B+ |
| pose `folded` | B (the sling panels keep their unloaded catenary sag and lie in 18 mm waves on the flat frames) | - | - |

Opened: 1:50 body, a low three-quarter close view at 1:50, 1:500 body, the `folded` pose. Rendered, not opened: the other poses, the `fold-chair` frame at 3 s, thermal cuts and structure of every scale, the 2D (`🗑️generated/r5-folding-deck-chair/before/`).

`volumes-check.ts furniture/folding-deck-chair --product`: PASS (0 fail).

### Remaining

- Folded and half-folded: the canvas panels are rigid lofts on their bones, so the catenary sag of the open chair shows as waves when the cloth is taut (needs a deformable sling or per-pose panel shapes).
- Canvas thickness at the print floor of each scale (7.5 / 15 / 30 / 40 mm instead of 0.7 mm); arrises, tacks and hems below the feature.
- Mass 10 % above the heaviest cited chair.

All four digital chairs retain the paired 20 × 32 mm folding props and the turned cross dowel, 580 mm long with 19 mm ends and a 22 mm shaft. The rear and foot rails have separate thermal and structural members. The 1:200 and 1:500 paper views now project the same working sections as the digital model. Three.js independently measured all four scales.
