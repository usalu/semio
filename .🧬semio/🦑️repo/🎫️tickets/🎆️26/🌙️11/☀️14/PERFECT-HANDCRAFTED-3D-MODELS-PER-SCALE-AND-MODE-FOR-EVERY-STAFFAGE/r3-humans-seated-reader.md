# Round 3 — `humans/seated-reader`

State (2026-11-20): written to the catalog (summary, provenance, dimensions, materials, rig, anchors, representations with redrawn 2D and decisions, symbols, poses, clips, structure assumptions, `volumes`) and the manifest row. `r3-validate.ts humans/seated-reader --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/seated-reader --product`: `FAIL: 7 measurement(s) over 4 rule line(s)`; every rule is green at every scale except `volumes-thermal-coverage`, and there only on areas the author may not change: `jeans` and `shoes` (body model, C18) and `bench` (factor 2.7, C16) with the totals at 1:200 and 1:500 that follow from `bench`. Print suitability (default profile) holds at every scale and at both ends of every valid range; before this round the asset was unsuitable at 1:350, 1:500 and 1:1000, so the product drew nothing at 1:500.

No earlier round-3 work existed for this asset. The scripts follow the seated office worker's layout; kit, sheet copy, frame helper and write script were copied, everything else is new.

Scripts (ticket folder): `r3-seated-reader-ref.ts` (reference numbers), `-roots.ts` (rig, anchors, poses and clip keys solved with the product's `solveChain`, texts), `-frame.ts` (rest frames, world to bone), `-kit.ts` (constructors), `-s50.ts` (1:50 body), `-coarse.ts` (1:100, 1:200, 1:500 bodies), `-lists.ts` (thermal, structure, materials, references), `-texts.ts` (3D and 2D decisions, provenance note), `-build.ts` (candidate document and source), `-draw2d.ts` (2D redraw, ruling E1), `-write.py` (catalog members with `volumes` in one write, manifest row), `-run.sh` (draw, build, validate), `-look.sh`, `-sheet.ts`, `-suit.ts`, `-probe.ts`. `🗑️generated/r3-seated-reader/` was deleted; the first `rooted()` call after that solves the poses again (about 12 s) and caches them there.

## 1. Reference

| quantity | reference | used | source |
|---|---|---|---|
| stature / mass | 1.68 m, 61 kg | unchanged | asset `provenance`, `thermal.body` |
| hip joint above a hard seat | 0.085 to 0.10 m | 0.09 m: hip joint at 0.504 m | Winter 2009; ANSUR II seated heights (my knowledge) |
| shoulder joint above hip joint | 0.818 H − 0.52 H = 0.50 m | 0.50 m, 0.04 m ahead of the spine line | brief; Winter 2009 |
| C7 above hip joint | 0.34 H = 0.571 m | 0.571 m (pelvis 0.094 + trunk 0.477) | de Leva 1996 |
| head: joint to vertex / chin to vertex | 0.125 / H/7.5 = 0.224 m | 0.125 / 0.224 m | brief; ANSUR II female |
| shoulder joint spacing / bideltoid | biacromial 0.365 − 2 × 0.02; bideltoid 0.45 m | 0.33 m / 0.45 m in the sweater | ANSUR II female (my knowledge) |
| hip joint spacing / seated hip breadth | 0.17 / 0.40 m | 0.17 / 0.406 m in jeans | Bell et al. 1990; ANSUR II |
| upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H = 0.312 / 0.245 / 0.181 | 0.3115 / 0.2454 / 0.1794 (kept) | Winter 2009 |
| thigh / shank / ankle | 0.245 H / 0.246 H / 0.039 H = 0.412 / 0.413 / 0.066 | 0.4106 (kept) / 0.413 / 0.078 in a sneaker with a 12 mm sole | Winter 2009 |
| chest, waist, thigh, calf | chest 0.28 × 0.25, waist 0.27 × 0.20, thigh girth 0.60, calf 0.36 m | sweater 0.32 × 0.27 at the bust, 0.30 × 0.22 at the waist; thigh 0.194 × 0.172; jeans leg 0.116 × 0.128 | ANSUR II female (my knowledge) |
| foot / shoe | foot 0.245 m; canvas sneaker EU 39: 0.265 × 0.095 m, sole and foxing 19 mm | 0.265 × 0.098 m | my knowledge |
| park bench | length 1.2 / 1.5 / 1.8 m; seat 440–490 mm high, 380–450 deep, raked 2–10°, seat-to-back 95–105°, back at least 300 mm above the seat; 40–50 mm slats 80–120 wide, gaps 8–20 mm, frames set in from the slat ends, carriage bolts | 1.20 m; seat 0.44 m at the front, 0.43 deep, raked 5°; back leaning 8° to 0.85 m (103°); seat slats 4 × 100 × 40 with 10 mm gaps, back slats 3 × 80 × 30; end frames in 45 mm timber 0.13 m in | BS 8300-1:2018, Inclusive Mobility 2021 §13, catalogue practice as collected in `r3-furniture-park-bench.md` |
| notebook | A5 top-bound pad 148 × 210 × 12 mm, about 0.25 kg | 148 × 210 × 12, 0.25 kg, sheet 2.5 g | ISO 216; my knowledge |
| cup | 12 oz paper cup: rim 90, base 60, height 108 to 110 mm, lid | r 30 to 44 mm, 108 + 12 mm, 0.27 kg with drink | my knowledge |

No web search was made. "My knowledge" values are not verified against a datasheet; the bench numbers rest on the sources the park-bench author collected.

## 2. Audit (old asset)

- Rig: the root bone `seat` lay on the floor (0.48 m long at y 0) and stood for the whole bench; both thighs on the symmetry plane (no `z`), shoulder joints 0.28 m apart and on the spine line; no lumbosacral offset; neck 0.094 and head 0.17 m; shank 0.401 m (−12 mm); ankle 0.071 m with no sole. The notebook hung from the left finger tips (pad origin at the tip of the hand), weighed 0.6 kg, and its sheet swung through the back of the pad; the cup weighed 0 kg and stood on the centre line behind her pelvis in 3D but beside her in the plan drawing.
- Bench: 0.5 m wide, a 40 mm plank on four square legs with a tilted panel; not a bench.
- Poses: forearms abducted up to 14° at the elbow, hands without supination; stand-up keys with toes bent up to 30° on flat feet.
- 2D: constructed views that did not agree with one body; a single leg in the side view; height 1.34, depth 0.50.
- Solids (44 at 1:50): tube limbs, ball shoulders and knees, mitten hands, self-intersecting lofts at cowl, neck, head, hair and toes and a revolve closed by implied closure (`kernel-traps.md` 171, 551–577); structure of 1:50 one member for the seat.
- Print: unsuitable at 1:350, 1:500 and 1:1000.

## 3. Root fixes (old → new)

| member | old | new |
|---|---|---|
| rig root and bench bones | root `seat` (0, 0), 0.48 m | root `ground` (length 0); `frame-l` / `frame-r` (rear foot to post top, 0.85 m, z +0.31 / −0.63), `seat` (rear edge (−0.155, 0.4025), 5°, 0.43 m), `backrest` ((−0.187, 0.56), 98°, 0.29 m), all fixed |
| `hip-lift` y | 0.5 on `seat` | 0.504 on `ground` |
| `torso` x / y / angle / length | 0.0944 / 0 / 0 / 0.439 | 0.0944 / 0.033 / −2° / 0.477 |
| `neck` angle / length; `head` angle / length / com | −8° / 0.0944; 8° / 0.17 / (0.07, 0) | −16° / 0.111; 18° / 0.125 / (0.0245, −0.015) |
| `arm-l/r` x / y / z / angle | 0.3965 / 0 / ±0.14 / 180° | 0.406 / −0.04 / ±0.165 / 182° |
| `thigh-l/r` z / angle | none / −93.9° | ±0.085 / −91.81° |
| `shank-l/r` length / angle | 0.4012 / −86.1° | 0.413 / −88.19° |
| `foot-l/r` length / angle; `toe` angle | 0.1754 / 66.2°; 23.8° | 0.1516 / 59.04°; 30.96° (ankle 78 mm above the floor) |
| `pad` | at the tip of the hand, 0.2 m, 0.6 kg | in the palm (0.0101, −0.022, z 0.0633 in the hand frame), 0.21 m, 0.25 kg, turned 22° against the hand in every pose |
| `page` | 0.19 m, −185°..5°, 0.01 kg | 0.205 m, −5°..360° (over the binding towards the reader and round behind the board), 2.5 g |
| `cup` | (−0.215, 0.42) on the root, centre line, 0 kg | 0.07 m along the seat, z −0.34, 0.12 m, 0.27 kg |
| anchors | seat (−0.08, 0.03), thigh support (0.105, −0.073), backrest (0.206, 0.12), heels (−0.0103, −0.0819), balls (0.1754, 0), mouth (−0.0048, −0.0756), nostrils (0.0284, −0.0983), crown (0.165, 0), pad grip (0.0999, 0) | seat (−0.09, 0.02), thigh support (0.17, −0.07, z 0.085), backrest (0.3, 0.09), heels on the heel of the sneaker, balls (0.1516, 0), mouth (−0.056, −0.097), nostrils (−0.034, −0.094), crown (0.131, 0.008), pad grip (0.105, 0) |
| poses | arms by hand in the side plane plus abduction | all six solved in 3D (shoulder angle, abduction and twist, elbow flexion with at most ±8° carrying angle, forearm supination, wrist): both open hands under the notebook, fingers 22° inwards; reclined: slid 3 cm back, trunk 15.5°, back on the slats; cup: right hand over the cup beside her; legs solved with the feet flat on the same spot; standing 0.43 m ahead and 0.39 m up |
| clips | `page-turn` right arm in the side plane, sheet to −178°; 13 baked `stand-up` keys | `page-turn`: right finger tips follow the free edge of the sheet (0°, 50°, 110°), the sheet falls to 352°; `stand-up`: the 13 keys solved again (old lift, travel and trunk course carried to the new legs, flat feet, both hands under the notebook) |
| `summary`, pose and clip descriptions, `structure.assumptions`, `provenance.notes`, references | edge of the bench, 0.6 kg notebook | rewritten; references `ansur-ii-2014`, `bs-8300-1-2018`, `inclusive-mobility-2021`, `timber-bench-catalogues` added |
| `materials` | — | `bone`, `rubber-sole`, `steel-zinc` added |
| `dimensions` | 1.34 / 1.02 / 0.50 / 64.5 kg | 1.337 / 0.898 / 1.20 / 64.41 kg |
| manifest row | 1.34 / 1.02 | 1.337 / 0.898 |
| symbols | dot on `seat` | dot on `ground` (same place) |
| `representations[*].geometry` and decisions | constructed drawings | redrawn from the bodies (E1) |
| `thermal.regions[*].areaM2` | — | unchanged (see 6) |

2D redraw (`-draw2d.ts`): one shape per part and view with one id, tone, region and material; bench, cup, trunk, head and feet as flat outlines solved into their bone frames; sleeves, forearms, cuffs, wrists, thighs and shanks as revolved profiles; hands and notebook as thin plates; the turning sheet as a 2 mm strip. Thighs and shanks had to become solids: a flat front outline of a thigh seen end-on collapses to a line when she stands, which is also what made the print check fail at 1:350 to 1:1000. Knit hatch kept on sweater trunk, sleeves and forearms (1:50) and on the trunk (1:100).

## 4. What was modelled (body / thermal / structure)

- **1:50 — 95 / 76 / 135.** Bench: two end frames (cranked rear post, front leg cut on the rake, seat rail, lower stretcher), long stretcher, four seat slats, three back slats, 14 bolt heads. Cup: body, sleeve, lid. Notebook: block, board back, wire comb, sheet. Woman: jeans seat, sweater (12 rows), ribbed hem, cowl, neck, head (12 rows), nose, mouth, eyes, brows, ears, hair, bun, sleeves with deltoid caps, forearms, cuffs, wrists, palms, thenars, four three-part fingers and a thumb per hand, thighs, jeans legs, sneakers (upper, sole with foxing, toe part, toe sole, four lace bars). Thermal: nested shells where each layer lies, face ellipsoid, ears and neck band for `face-skin`, three outlet patches. Structure: full skeleton, 17 joints, masses at the de Leva centres, bench timbers, notebook, sheet and cup as members.
- **1:100 — 43 / 44 / 100**, **1:200 — 28 / 30 / 57**, **1:500 — 25 / 27 / 39**: stated anew per scale; what was merged is in each scale's decisions. The notebook is 30 mm thick at 1:200 and 78 mm at 1:500 (the feature rule and the print check ask for it).

Final check: silhouette IoU elevation / front / plan 1:50 0.986 / 0.996 / 0.995, 1:100 0.986 / 0.995 / 0.992, 1:200 0.987 / 0.995 / 0.996, 1:500 0.971 / 0.997 / 0.997; bounds, feature, monotonic, cross-scale, structure coverage, bones, copied green at all scales. Thermal per region at 1:50: face-skin 14 %, hand-skin 17 %, sweater 10 %, notebook 11 %, cup 14.5 % (limit 15), jeans **+64 %**, shoes **+39 %**, bench **+172 %**; at 1:100 jeans **+63 %**, bench **+156 %**; totals 1:200 **+56 %**, 1:500 **+71 %**. Line: `FAIL: 7 measurement(s) over 4 rule line(s)`.

## 5. What I saw (Read on the PNGs)

- 1:50 body, six views, then front and three-quarter (turn 60) large: a woman of normal proportions on a slatted bench, head bowed to the notebook on both open palms, thumbs over its edges, elbows out, thighs level on the slats, shins vertical, sneakers flat, cup beside her. First state had sleeve caps standing proud of the shoulders; shoulders widened and caps lowered, then looked again.
- 1:50 poses, six views each: reclined (back on the slats, notebook at eye level), reaching for the cup (right hand over the cup, notebook on the left thigh), standing (straight legs, notebook at the waist). Clip frames, right side large: `page-turn` 1.7 s (finger tips on the edge of the raised sheet), `stand-up` 1.9 s (pelvis off the seat, feet planted).
- 1:100, 1:200 and 1:500 body, six views each; 1:500 again after the hair cone, the frames and the shoes were restated, plus its top view large.
- 1:50 structure, six views: skull, mandible, curved spine, rib hoops, scapulae, pelvis, long bones, finger bones under the notebook, bench timbers. 1:50 thermal cut: nested shells inside the sweater, face and hair, jeans, socks in the sneakers, nothing outside the body.
- `r3-2d` 1:50 reading pose and standing pose (candidate), 1:500 (catalog): clean drawings; standing reads 1.73 m with full legs; 1:500 is drawn now.
- These renders were made before the cup moved 13 cm back along the seat (last change, for nothing else): only the 1:500 2D shows the final cup place.
- Rendered or not rendered, and not opened: thermal and structure at 1:100, 1:200, 1:500; 2D at 1:100 and 1:200; `leaning-forward` and `rising` as poses.

## 6. Still imperfect

- `volumes-check` stays red on `jeans`, `shoes` (body-model shares, C18/C27) and on `bench` with the two totals. The bench area describes the old 0.5 m sample (1.108 m²); the real bench measures 3.01 m². I did not apply it (factor 2.7, C16); hand calculation in `area-review.md`, request in `author-requests.md` (both 2026-11-20). Until a reviewer applies it the thermal solver computes the bench with a third of its surface.
- The bench is 1.20 m long instead of 0.50 m: the asset is 1.20 m deep in front view and plan, and she sits 0.44 m from its left end, so it is not symmetric about her.
- Body grade as I see it: 1:50 A− (no cloth folds; one bust mass under the knit; the knee seam of thigh and jeans leg shows as a ragged line in shaded views; hair is a smooth cap; the buttock mass shaped for sitting reads low when she stands), 1:100 A−, 1:200 B+, 1:500 B+ (sleeve caps read as flaps, shoes as blocks, notebook as a brick). Bench A−: no armrests, bolts only as heads, no chamfers on frame timbers.
- Knees stay 0.17 m apart in every pose (legs pitch only).
- The right hand misses its place under the notebook by 2 to 4 mm in the stand-up keys (wrist limits); the left hand carries it exactly.
- Thighs and shanks are circular in 2D (revolved profiles), 6 % off their true sections.
- `kernel-traps` was not re-run on the new solids. Tests and fixtures that embed this asset were not regenerated or run.

## Round 5 handcraft

2026-11-21. Brief `r5-brief.md`. The round-3 scripts (`r3-seated-reader-*.ts`) are the root of the model and still reproduced the catalog byte for byte at the start (all three lists of all four scales and every other member), so they were extended in place; new round-5 tools: `r5-seated-measure.ts` (model against the reference table, any scale, pose or rest frames), `r5-seated-look.sh` (body, structure, thermal cut of the four scales and every pose plus one clip frame as montages), `r5-seated-montage.ts`.

### Reference table (modelled to) and model vs reference, 1:50, rest frames (`bun r5-seated-measure.ts reader --rest`)

| quantity | reference | model | deviation | source |
|---|---|---|---|---|
| sitting height (seat to vertex) | 0.875 | 0.896 | +2.4 % | DIN 33402-2 f P50 0.857 at 1.65 m, scaled to 1.68 m |
| head height chin to vertex (H/7.5) | 0.224 | 0.224 (round 4: 0.232, +3.8 %) | -0.2 % | brief; ISO 7250 f P50 |
| head breadth over the hair | 0.155 | 0.153 (round 4: 0.160, +3.2 %) | -1.3 % | ISO 7250 f P50 0.147 + hair |
| shoulder joint above the seat | 0.590 | 0.589 | -0.2 % | 0.818 H - 0.52 H + hip 0.09 above the seat |
| shoulder joint spacing | 0.330 | 0.330 | 0 % | biacromial 0.365 - 2 x 0.02 |
| bideltoid breadth in the sweater | 0.450 | 0.438 | -2.7 % | DIN 33402 f P50 0.43 + knit |
| upper arm / forearm | 0.312 / 0.245 | 0.311 / 0.245 | -0.2 / +0.2 % | Winter 0.186 H / 0.146 H |
| hand length | 0.181 | 0.178 | -1.4 % | 0.108 H; DIN 33402 |
| hip breadth seated in jeans | 0.405 | 0.405 | -0.1 % | DIN 33402 f P50 0.39 + jeans |
| thigh clearance over the raked slats | 0.155 | 0.156 | +0.6 % | DIN 33402 f P50 0.15 + cloth |
| knee joint height | 0.491 | 0.491 | 0 % | Winter 0.285 H + 12 mm sole |
| buttock-knee length | 0.595 | 0.603 | +1.4 % | DIN 33402 f P50 0.585 + cloth |
| calf breadth in jeans | 0.115 | 0.116 | +0.9 % | ANSUR II f 0.107 + 2 x 4 mm denim |
| shoe length | 0.265 | 0.271 | +2.3 % | 0.152 H foot + 0.02; EU 39 |
| ankle joint height in the shoe | 0.078 | 0.078 | 0 % | Winter 0.039 H + 12 mm sole |

Every quantity within 3 % at 1:50. The bench, notebook and cup keep the round-3 reference table above (unchanged objects). The rig (joint centres, spine curvature, lumbosacral offset, shoulder joints ahead of the spine) was audited against the table and needed no change.

### Grades (my own look at every sheet; before = catalog state of round 4)

| scale | body before | body after | structure before / after | thermal before / after |
|---|---|---|---|---|
| 1:50 | B (hair a thick beanie with a brim over the temples; sleeve caps as balls standing on the shoulders; elbows z-fighting where two equal domes met; chin 8 mm long) | A | A / A (mandible raised with the chin) | A / A |
| 1:100 | B (same hair and caps; elbow seam) | A | A / A | A / A |
| 1:200 | B (beanie hair; ragged knee seam where the shank top stood out of the pointed thigh end) | A- | A- / A- | A- / A- |
| 1:500 | B- (beanie hair; knee seams with dark slivers) | A- | A- / A- | A- / A- |
| poses 1:50 (seated-upright, reclined-reading, reaching-for-cup, leaning-forward, rising, standing) and clip `lean-back` at 3 s | A- (closed, natural; same hair and cap defects as the body) | A | | |

### Changes

- Head: lower face shortened (chin -0.101 -> -0.094 m on the head bone at 1:50, the same share at 1:100, 1:200, 1:500) so that chin to vertex is H/7.5; mandible of the structure raised with it at 1:50 and 1:100.
- Hair restated at every scale as drawn-back hair: 5 to 9 mm over the skull, 2 to 4 mm at the temples, down to the nape at the back, hairline crossing the forehead at about 7 cm above the brows at a clear angle (no grazing seam), bun unchanged. At 1:200 and 1:500 the hairline sits a little higher so that the visible face keeps the 0.2 mm printable feature down to 1:350 (the first try failed `PRINT range-end-feature` at 0.186 mm).
- Shoulders: the sleeve no longer starts with a quarter-sphere dome round the shoulder joint; five stated rows lean the cap inwards under the sweater shoulder (trapezius slope into the deltoid), at 1:50, 1:100, 1:200 and 1:500.
- Elbows: the forearm's elbow dome is 7 mm smaller than the sleeve's, so the olecranon is one surface (no z-fight) at 1:50 and 1:100.
- Knees at 1:200 and 1:500: thigh ends full and rounded, shank tops smaller and inside them.
- 2D of all four scales redrawn from the new bodies (`r3-seated-reader-draw2d.ts`, ruling E1) and written with the volumes (`r3-seated-reader-write.py`, `volumes-insert.py`).

Final: `r3-validate.ts humans/seated-reader --catalog-only` VALID (schema 0, validateAsset 0, print suitability at every scale and range end). `volumes-check.ts --product`: `FAIL: 8 measurement(s) over 4 rule line(s)`, all `volumes-thermal-coverage` on areas the author may not change and already in `area-review.md`: jeans +64 % / +63 %, shoes +40 % (body-model areas, C18), bench +172 % / +156 % and the 1:200 / 1:500 totals that follow from it (C16, filed 2026-11-20), cup +16.3 % at 1:50 (limit 15 %; the cup was not touched, the measured value moved with the shared tessellation change; row already in `area-review.md`). Silhouette, bounds, feature, monotonic, cross-scale, structure coverage, bones, copied green.

Looked at (Read): body sheets of 1:50, 1:100, 1:200, 1:500 (montage `🗑️generated/r5-seated/reader/v2-body.png` and single three-quarter views after each change), head close-ups 1:100 and 1:200, structure and thermal cut of all four scales, the six poses and the clip frame (montage), the product 2D of all four scales.

### Remaining issues

- The black jagged rings round the knees in the front views and the black bar under the raked seat slats are the sheet renderer inking grazing surfaces as depth steps (geometry closed and smooth; `tool-requests.md` 2026-11-21).
- No cloth folds; the sweater is one smooth knit mass with a single bust mass. The notebook reads thin in the side view because it is held nearly edge-on (A5 pad, 12 mm).
- Area rows above stay red until the area reviewers decide (not forced, C16/C18).
