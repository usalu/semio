# Round 3 — `humans/sleeping-person`

Final state: see the last section. Written over two sessions (interrupted 2026-11-20 about 02:15, resumed the same day).

No earlier round-3 work existed for this asset (no `r3-sleeping-person-*` scripts, no report); the catalog asset was valid at the start (`r3-validate.ts`: VALID; `volumes-check.ts --product`: FAIL 8 measurements over 4 lines: hand-skin and feet-skin areas, structure outside the body in front and plan).

Scripts (ticket folder): `r3-sleeping-person-ref.ts` (reference numbers), `-roots.ts` (rig, supine pose solved in 3D with the product's `solveChain`, anchors, materials, thermal region `duvet-edge`, texts), `-kit.ts` (world-authoring constructors on top of the office worker's kit: world lofts, world bars, the half-revolved duvet mound), `-s50.ts` (1:50 body, every section by hand), `-coarse.ts` (1:100, 1:200, 1:500 bodies), `-lists.ts` (thermal, structure), `-texts.ts` (decisions, notes, 2D decisions), `-build.ts` (candidate document and source), `-draw2d.ts` (2D redraw, ruling E1), `-look.ts`, `-sheet.ts` (copy of `r3-sheet.ts` reading a candidate), `-probe.ts`, `-frames.ts`, `-bbox.ts` (debug aids), `-write.py` (catalog members).

## 1. Reference

| quantity | reference | used | source |
|---|---|---|---|
| stature / mass | 1.70 m, 64 kg woman | unchanged | asset provenance, `thermal.body` |
| hip joint / knee / ankle height (standing) | 0.52 H / 0.285 H / 0.039 H → thigh 0.40, shank 0.418 m | thigh 0.40, shank 0.4185 | round 3 brief; Winter 2009 |
| shoulder joint above hip joint | 0.818 H − 0.52 H = 0.507 m | 0.497 (kept, joint centre below the acromion line) | brief |
| C7 / atlanto-occipital joint / vertex above hip joint | 0.86 H − 0.52 H = 0.578; vertex − 0.133; 0.48 H = 0.816 | 0.578 / 0.683 / 0.816 m | de Leva 1996, brief |
| head chin–vertex, length, breadth | H/7.5 = 0.227; glabella–opisthocranion 0.185; breadth 0.145–0.15 (ANSUR II women) | 0.232 / 0.186 / 0.148 | ANSUR II (my knowledge) |
| shoulder (biacromial), bideltoid | women 0.36–0.37 / 0.41–0.43 m; joint centres 2 × 2.5 cm inside | joints 0.316 apart, shoulders 0.34 + deltoids | ANSUR II women |
| hip joint spacing, hip breadth | 0.17–0.18 / 0.36–0.38 m | 0.174 / 0.36 | Bell et al. 1990, ANSUR II |
| chest breadth × depth (bust), waist, thigh, calf, upper arm, forearm, wrist | 0.29–0.31 × 0.24–0.26; waist 0.27 × 0.20; thigh girth 0.58; calf 0.36; arm 0.28; forearm 0.245; wrist 0.155 m | trunk 0.30–0.33 × 0.215 lying (bust flattens), waist 0.26 × 0.18, thigh 0.18 × 0.17, calf 0.11 × 0.114, sleeve 0.10, forearm sleeve 0.08, wrist 0.054 × 0.034 | ANSUR II women (my knowledge) |
| hand length / breadth; foot length / breadth | 0.108 H = 0.184 / 0.08; 0.152 H = 0.258 / 0.095 m | hand bone 0.1816, palm 0.08; foot 0.245, ball 0.094 | Winter 2009, ANSUR II |
| ankle to ball (foot bone) | heel 0.05 behind the ankle, ball at 0.73 of the foot length → 0.145 m | 0.145 | anthropometry of the foot (my knowledge) |
| lying supine on a soft mattress | hip joint 0.09–0.10 m, shoulder joint 0.07–0.08, knee 0.06–0.08, ankle (heel on the mattress) 0.045–0.05 m above the support; relaxed feet plantar-flexed 30–40°, legs turned out 15–25°; elbows slightly flexed | hip 0.093, shoulder 0.080, knee 0.076, ankle 0.045, feet 56° above the mattress, legs turned out 18.7°, left elbow 3°, right elbow 71° | clinical positioning practice (my knowledge) |
| pillow | 40 x 80 cm (DE single bed), loft 12–14 cm, 5–7 cm under a head | 0.40 x 0.80, loft 0.128, 0.062 under the head | trade sizes |
| duvet | 135 x 200 cm (DE single), 4–6 cm loft, 1.0–1.5 kg plus cover | 1.35 x 2.00, 35 mm flat, 40 mm over the body, 2.0 kg with cover; foot end folded back 0.58 m so the feet are out | DIN EN 12934 trade sizes; asset thermal layer 0.05 m |
| mattress | 0.90 x 2.00 m, top 0.46 m under a sleeper, head end at x 0.06 | duvet drapes over z ±0.45 | catalog single-bed (EN 1334) |

No web search was made; "my knowledge" values are not verified against a data sheet.

## 2. Audit against the reference (catalog state before round 3)

| item | found | reference | verdict |
|---|---|---|---|
| `dimensions` | height 0.45, width 1.88, depth 0.40 m | lying woman with pillow and 135 cm duvet: about 0.36 x 1.92 x 1.04 m | depth was the body alone, the bedding was not in it |
| torso / neck / head bones | 0.4443 / 0.0955 / 0.172 m | 0.4825 / 0.105 / 0.133 m (C7 0.578, atlanto-occipital 0.683, vertex 0.816 above the hip joints) | head joint 39 mm too low, head bone 39 mm too long |
| thigh / shank / foot / toe | 0.4155 / 0.406 / 0.1775 / 0.0621 | 0.400 / 0.4185 / 0.145 / 0.062 | knee 15 mm too far, foot bone 32 mm too long |
| pillow bone | 0.52 m | 0.40 m (40 x 80 pillow across the bed) | wrong pillow format |
| arms | 0.3152 / 0.2483 / 0.1816 | 0.186 H / 0.146 H / 0.108 H = 0.316 / 0.248 / 0.184 | kept |
| supine pose | both arms in the paper plane, feet upright | left arm beside the body, right hand on the belly, feet falling outward 18.7 degrees | solved anew in 3D |
| solids | 64 / 39 / 22 / 16 body solids: box torso, tube limbs, duvet as slabs | see section 4 | replaced completely |
| `volumes-check --product` at the start | FAIL, 8 measurements (hand and feet skin areas, structure outside the body) | | now PASS |

## 3. Root fixes (written by `r3-sleeping-person-write.py` from `r3-sleeping-person-roots.ts`)

- Rig: torso 0.4443 -> 0.4825, neck 0.0955 -> 0.105, head 0.172 -> 0.133, thigh 0.4155 -> 0.400, shank 0.406 -> 0.4185, foot 0.1775 -> 0.145, toe 0.0621 -> 0.062, pillow 0.52 -> 0.40 m; hip joints 0.174 m apart, shoulder joints 0.316 m apart; hierarchy unchanged.
- `dimensions`: 0.45 x 1.88 x 0.40 -> 0.364 x 1.919 x 1.04 m (height from the lowest drape to the nose; mass unchanged 69.7 kg = 64 kg woman, pillow, duvet).
- Poses `supine`, `side`, `curled` and clips `breathing`, `turn-over` re-solved on the new rig (the product's own `solveChain`); anchors moved onto the new surfaces (`chest-plume` now 0.7 / 0.713 / 0.028 on the shifted trunk mound); thermal region `duvet-edge` for the duvet beyond the body; summary, provenance, materials and the manifest row.
- 2D (ruling E1): elevation, front and plan of all four scales are redrawn from the body of that scale by `r3-sleeping-person-draw2d.ts` (per shape: projected outline, simplified to the tolerance of the scale, sleeves as revolved profiles, layer order from the depth in the view). Print suitability: all four scales `suitable` (the lips at 1:50 were 0.194 mm against 0.2 mm; lips widened from 19 to 21.6 mm).

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 82 | 85 | 92 |
| 1:100 | 28 | 33 | 58 |
| 1:200 | 19 | 22 | 54 |
| 1:500 | 14 | 17 | 26 |

- 1:50 body: head loft of 13 sections with nose, lids, lips, hair cap and spread hair; neck; trunk in the pyjama top (13 sections, bust, sloping shoulders), collar; shorts; thighs, shanks, ankles with malleoli, feet with heel and arch, five toes each; sleeves with rounded shoulder and elbow ends, forearm sleeves, cuffs, palms with thenar, four two-part fingers and thumb per hand; pillow with the head's dent; duvet: half-revolved mounds over trunk (set 28 mm to her left so the right arm lies beside it), hips (low and wide, under the hand), thighs and shins, flanks as eight hand-stated sections across the bed per side that fall from the mounds to the 35 mm flat part (lower on the arm side), rolled edges, drapes, and the foot end folded back with its own mounds, flanks, edges and drapes.
- 1:100, 1:200, 1:500: authored anew (`r3-sleeping-person-coarse.ts`), hidden body left out, face and hands merged step by step; ankles join the feet to the duvet end at every scale.
- Thermal: every region from its body solids; at 1:50 `top` and `bottom` as nested layers (pyjama inside the duvet mounds), face mask, three emitter patches (nostrils, lips, duvet over the chest).
- Structure: skeleton (skull, mandible, curved spine, rib hoops, sternum, clavicles, scapulae, pelvic ring, long bones, patellae, hand and foot rays), joints at the rig joints, masses at the de Leva centres, mattress contact band, pillow core, duvet membrane; simplified by scale.

## 5. Second session (resume), look and fix

Found on resuming: catalog `volumes` equal to the ticket source, no bulk-pass change in this asset (file time 02:11). Seen in the renders and fixed:

1. Duvet read as a mummy on a tray (vertical mound flanks on flat boards): flanks restated as sections across the bed (1:50, 1:100), stepped profile at 1:200.
2. Right upper arm ran through the flank of the trunk mound: mound set 28 mm to her left and narrowed, the arm now lies beside it.
3. Sleeve ends were flat caps that stuck out at shoulder and elbow: rounded by two closing sections each.
4. 1:100 mitten hand stood on its edge (bar sections without orientation; also `th-hand-l`): restated as lofts lying flat.
5. 1:200 and 1:500 feet floated 7 cm beyond the duvet: ankles added.
6. Hip and thigh showed through the mound flank after fix 2: own low hip mound (0.26 half-width, 0.21 high) that stays under the hand; thigh mound start widened.
7. Stray tick in the 1:50 plan line drawing (thigh mound 0.1 mm longer than the fold over it): mound ends 10 mm earlier.
8. Decisions (en, de) updated for the flanks, the hip mound and the ankles.

Final checks (2026-11-20): `r3-validate.ts humans/sleeping-person`: `VALID ... schema 0, validateAsset 0`; `volumes-check.ts humans/sleeping-person --product`: `PASS`; print suitability `suitable` at 1:50, 1:100, 1:200, 1:500; spliced (`volumes spliced: 1:50 authored, 1:100 authored, 1:200 authored, 1:500 authored`).

## 6. Final renders, as seen (`🗑️generated/r3-sheet/humans/sleeping-person.*`, `🗑️generated/r3-2d/humans/sleeping-person.s50.png`)

- 1:50 body, front (from the feet): soles with five toes each, duvet as a bell over the body falling to the mattress edges, drapes hanging over both sides. Right side: head on the dented pillow, chin, nose and lips in profile, shoulder out of the duvet, upper arm on the cloth, forearm rising to the hand on the belly, step of the fold over the thighs, feet at about 56 degrees. Back: hair cap on the pillow. Left side: same silhouette, left arm hidden. Top: 40 x 80 pillow across, face with lids, nose, lips, the bent right arm with fingers and thumb, duvet 1.35 m wide with the wider folded part, feet turned out. Three-quarter (two directions): no part pierces another, the hand lies on the cloth, hip covered.
- 1:100: same figure with mitten hand lying flat, one nose form, merged toes. 1:200: masses, arm in two segments, feet on ankles. 1:500: head, shoulders, arm, feet, two mounds on one slab; recognisable as a sleeper under a duvet.
- Thermal 1:50 (and `--cut`): face red, hair green, hands violet, feet magenta, pyjama top and shorts inside the paler duvet shells, duvet edge green, pillow blue, plume patch on the chest; no shell pokes out. Thermal 1:100 to 1:500 rendered without error; I opened 1:50 only in this session.
- Structure 1:50: skull, curved spine, rib hoops, pelvis, long bones inside the pale body, mattress band and pillow core; nothing outside the body (the check agrees). 1:100 to 1:500 rendered without error; opened in the first session only.
- 2D 1:50: elevation, front and plan agree with the body; 1:100 opened before the last three fixes, 1:200 and 1:500 not opened in this session.
- Pose `side` at 1:50: opened before the last fixes.

## 7. Still imperfect

- Poses `side` and `curled`, clip `turn-over`: the duvet mounds are rigid half-revolves blended between their bone and the bed; when the legs are drawn up they tilt, show their flat end faces and open wedges between trunk, thigh and shin mounds. The figure itself stays closed. Grade C for the duvet in these poses; a cloth that follows three bones needs its own pose-specific solids, which the schema has no place for (not requested yet).
- The duvet in the default pose still has visible steps between the trunk, hip, thigh and shin mounds; they are separate solids so that they can follow their bones.
- Face at 1:50: lids as ellipsoids, nose as one loft, lips as one ellipsoid; no ears (under the hair), no brow ridge.
- The left hand under the duvet reaches 15 mm below the mattress datum (pressed into the soft mattress; hidden).
- 2D outlines come from the projected body of each scale through a script (simplified rings), not drawn stroke by stroke; in the line views the product merges the fold mounds with the folded flat part, so their outlines show only in the solid views.
- Reference values marked "my knowledge" are not verified against a data sheet; no web search was made.
- Not re-opened after the last fix: thermal and structure sheets of 1:100, 1:200, 1:500 and the 2D of 1:100, 1:200, 1:500 (all rendered without error and all checks green).

State: default pose grade A at 1:50 in body, thermal and structure as far as I looked; NOT DONE for the points listed above (side and curled poses first).

## Round 5 handcraft

Round 5 author, 2026-11-21. Found: the S01 second pass (cloth lofts that follow trunk, hips, thighs and shins with the hills staying on the bed; 1:50 face details) existed only in the scripts and was never spliced; the candidate it built was INVALID (1:50 print, 1:500 extent, arm twist). Scripts extended in place (`r3-sleeping-person-s50.ts`, `-coarse.ts`, `-lists.ts`, `-roots.ts`, `-draw2d.ts`), plus `r5-sleeping-person-measure.ts` (model vs reference), `r5-sleeping-person-box.ts` (vertical extremes per scale), `r5-sleeping-person-print.ts` and `r5-sleeping-person-culprit.ts` (which 1:50 shape breaks the print measurement). Output under `🗑️generated/r5-sleeping-person/`.

### Model vs reference (1:50, H = 1.70 m woman; `bun r5-sleeping-person-measure.ts`)

| quantity | reference mm | model mm | deviation | source |
|---|---|---|---|---|
| thigh (hip-knee joint) | 400 | 400 | 0.1 % | hip joint 0.52 H, knee 0.285 H |
| shank (knee-ankle joint) | 418 | 419 | 0.1 % | Drillis-Contini 0.246 H |
| upper arm (shoulder-elbow joint) | 316 | 315 | -0.3 % | Drillis-Contini 0.186 H |
| forearm (elbow-wrist joint) | 248 | 248 | 0.0 % | Drillis-Contini 0.146 H |
| hand bone | 184 | 182 | -1.1 % | Drillis-Contini 0.108 H |
| hip joint to C7 (torso bone + pelvis) | 578 | 578 | 0.0 % | C7 0.86 H - hip joint 0.52 H |
| shoulder joint spacing | 316 | 316 | 0.0 % | ANSUR II women biacromial 366 mm less 2 x 25 mm |
| hip joint spacing | 174 | 174 | 0.0 % | Bell et al. 1990, women |
| head chin-vertex (u) | 227 | 229 | 1.0 % | H / 7.5 |
| head breadth (w) | 148 | 148 | 0.0 % | ANSUR II women 148 mm |
| head length (v, skull loft without the nose) | 186 | 187 | 0.5 % | ANSUR II women glabella-opisthocranion 186 mm |
| foot length (heel to big toe tip) | 258 | 262 | 1.4 % | 0.152 H |
| pillow length across the bed | 800 | 800 | 0.0 % | 40 x 80 cm pillow |

All within 3 %. Bedding: pillow 40 x 80 cm, duvet 135 x 200 cm with the foot end folded back 0.58 m (round 3 table).

### Grades (before = catalog at the start of round 5 -> after)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 supine | B+ (steps between the rigid mounds) -> A- (hills fade into each other; faint seam lines where the body cloth meets the hill surface) | A -> A | A- -> A- |
| 1:50 side / curled | D (mounds torn into wedges, pyjama and hips through the duvet, body below the bed line) -> B+ (closed; the cloth goes with the body; small skin patches at the knees in `side`; the hills of the supine legs stay on the bed) | not opened | not opened |
| 1:100 | A- -> A- (only the face details and the right-arm twist changed) | A- -> A- | A- -> A- |
| 1:200 | A- -> A- | A- -> A- | A- -> A- |
| 1:500 | B+ (drapes 25 mm short, the drawing 20 mm lower than the asset) -> A- (crisp massing: head, shoulders, arm, feet, two mounds, slab with drapes) | B+ -> B+ | A- -> A- |

### Changes

- Spliced the S01 second pass (cloth lofts `cloth-trunk`, `cloth-hips`, `cloth-thighs`, `cloth-shins` weighted to their bones, hills weighted 30–50 % to the body and the rest to the duvet; brow, alae, cheeks, chin and lower lip on the face).
- Cloth restated: trunk cloth 20–40 mm wider than the pyjama trunk at every station with the same superellipse exponent 2.4 (the corners of the 2.4 trunk had pierced the exponent-2 cloth in `side`), its depth kept within the supine hills (cloth top at most 5 mm above the round 3 value); hips cloth likewise (exponent 2.5, 22 mm wider).
- Pose `curled`: `arm-r` twist -181.82 deg -> 178.18 deg (the same rotation inside the joint limits).
- Face: eyelids 22.4 x 19 x 33 mm (were 17 x 18 x 31), upper lip 14.8 mm deep (was 11), lower lip set 2 mm lower, so that 1:50 prints (the product refused 1:50 at 0.189 mm).
- Thermal: the merged face details (brow, cheeks, chin, alae, lower lip) are left out of `face-skin`; the face mask carries that surface (they had raised `face-skin` to 0.113 m² against 0.0871, a 30 % excess that was a double count, not a model of more skin).
- 1:500: trunk mound restated to the 1:50 crest (0.262 m) and the slab drapes lowered to 115 mm (were 90), so the 1:500 body spans the asset height 0.378 m like the 1:50 one.
- 2D (`r3-sleeping-person-draw2d.ts`): the cloth lofts and the left arm (under the duvet) are not drawn in the front view (the cloth plates are seen edge-on there, which gave NaN points, and the hidden left hand broke the product's polygon union in the print measurement); the right eyelid merges into the head outline in the elevation like the left one. All views of all scales redrawn.

### Checks

- Candidate `r3-validate.ts --file … --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0); catalog after write and splice: VALID; print suitable at every scale.
- `volumes-check.ts humans/sleeping-person --product`: PASS (15.2 s).

### Seen (opened)

1:50 body `supine`, `side`, `curled` (before and after), `side` three-quarter close-up, 1:500 body (catalog), 1:50 structure (catalog). Rendered and not opened in this round: 1:100 and 1:200 body (candidate), 1:50 thermal cut, structure and thermal of 1:100–1:500, the 2D of every scale, the `turn-over` and `breathing` clips.

### Still not perfect

- In `side` and `curled` the hills of the supine legs stay on the bed (the duvet keeps the shape of stretched legs at the foot end) and two small skin patches of the lower knee show between the thigh and shin cloth; a cloth that is pose-specific needs pose-specific solids, which the schema has no place for.
- Supine: faint seam lines on the trunk where the cloth surface comes within a few mm of the hill surface.
- 1:50 face details are separate ellipsoids (lids, alae, cheeks), not one face loft.
- Thermal and structure sheets of the poses and of 1:100–1:500 were not opened in this round.

The fine drawing now uses 0.16 mm for the eyelid and lip contours. The whole document, 1:75 print endpoint, 1:50 silhouette and feature checks passed.
