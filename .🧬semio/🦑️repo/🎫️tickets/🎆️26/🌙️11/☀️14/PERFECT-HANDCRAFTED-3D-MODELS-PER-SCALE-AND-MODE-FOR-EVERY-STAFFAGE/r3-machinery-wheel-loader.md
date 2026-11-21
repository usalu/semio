# Round 3 — machinery/wheel-loader

State: DONE (2026-11-20), with the open points of section 8. Two authors: the first (2026-11-19, cut off by a usage limit before a report existed) researched the reference, rebuilt rig, kinematics, 2D and all four models and wrote them to the catalog; the second (this report) found that state valid but failing the check and not drawable at 1:200, repaired it, looked at everything and finished.

Scripts (all in `T`): `r3_wheel_loader_kin.py` (pin geometry and closed-loop solution of the Z-bar linkage, pose table), `r3-wheel-loader-kinsearch.py` (scan that led to the pin positions), `r3_wheel_loader_lib.py` (solid helpers; every prism, revolve, loft and capsule also gives its own orthographic outline for the three views), `r3_wheel_loader_model.py` (every part of every scale stated by hand), `r3_wheel_loader_texts.py` (bilingual texts), `r3-wheel-loader-build.py` (candidate document under `🗑️generated/r3-wheel-loader/` and the volumes source), `r3-wheel-loader-write.py` (root members into the catalog file, member by member, re-read right before), `r3-wheel-loader-areas.json` (area corrections), `r3-wheel-loader-run.sh` (build, validate, print suitability, write, splice, product check), diagnostics `r3-wheel-loader-cross.ts`, `-areas.ts`, `-glass.ts`, `-suit.ts`, `-fails.py`, `-dump2d.py`. Resume: `./r3-wheel-loader-run.sh`.

## 1. Reference

Reference machine: Caterpillar 962M (20 t articulated wheel loader, Z-bar linkage, 3.5 m3 general-purpose bucket). S1 = `provenance.references[cat-962m-2017]` (dealer data sheets read by the first author on 2026-11-19), S2 = ISO 6746-1 (dimension definitions), S3 = ETRTO size arithmetic of 23.5R25, K = knowledge of the construction of this machine class (not a published number).

| quantity | real value | source | model |
|---|---|---|---|
| wheelbase | 3,350 mm | S1 | 3.350 (axles at x -1.675 / +1.675, articulation at 0) |
| tread | 2,140 mm | S1 | 2.140 (tyre centres z 1.07) |
| tyres 23.5R25 | 1,615 mm diameter, 595 mm wide | S1, S3 | 1.615 x 0.60, rim 25 in = 0.635 m bead seat, flange 0.73 m |
| width over tyres | 2,140 + 595 = 2,735 mm | derived | 2.74 |
| rear axle to edge of counterweight | 2,147 mm | S1 | 2.147 (rear end x -3.822) |
| height to top of ROPS | 3,444 to 3,458 mm | S1 | 3.45 |
| hinge pin height at carry / maximum lift | 669 / 4,235 mm | S1 | 0.670 / 4.237 (arm angle -36.5 / +45 degrees, arm 2.74 m, pivot at 2.30 m) |
| overall length without bucket | 7,228 mm | S1 | hinge pin at carry x 3.263 - (-3.822) = 7.09 (pin, not the arm nose; see section 8) |
| operating mass | about 20.2 to 20.9 t | S1 class | 20,500 kg (kept) |
| bucket | 3.5 m3 class, about 2.93 m wide | S1 class, K | 2.95 wide, profile 1.32 m deep, 1.35 m high |
| rack-back at carry / dump angle at full lift | about 45 to 50 / 45 to 50 degrees | S2 definitions, K | 45 / 45 |
| linkage | Z-bar: one tilt cylinder on the tower, bell crank pivoted on the arm cross tube, tilt link to the bucket, two lift cylinders | K | as real: five pins solved as a closed loop |
| frames | two box / plate frames joined by an upper and a lower hitch pin on a vertical axis; rear axle oscillating in a cradle; planetary hubs | K | as real |

## 2. Audit of the asset as found before round 3 (`🗑️generated/r3-wheel-loader/catalog-before-r3.json`)

| item | found | real | deviation |
|---|---|---|---|
| `dimensions` | height 3.4, length 8.9, width 2.95 m | 3.45, 8.42 at carry, 2.95 | height -0.05, length +0.48 m |
| wheelbase / tyre radius / tread | 3.40 m / 0.875 m / 2.20 m | 3.35 / 0.8075 / 2.14 | tyres 135 mm too large in diameter |
| lift arm | pivot (1.1, 2.0), 2.7 m | pivot about 2.3 m high, 2.74 m | pin heights at carry and full lift not reachable |
| tilt cylinder | a bone of the lift arm, from the arm to the bucket | Z-bar: cylinder on the tower, bell crank on the arm, link to the bucket | linkage of another machine type; no bell crank, no link |
| joint ranges | bucket -75..70, rods -0.5..0.9 m | from the solved linkage | free ranges without a mechanism behind them |
| masses | rear 8,190, front 4,100, arm 1,700, bucket 1,550 | sum 20,500 | redistributed onto the new parts |
| anchors | contacts at x 1.7, z 1.1, footprint 0.8 m; stack mouth at (-1.23, 3.15, -0.9) | 1.675, 1.07, 0.5 m; stack on the hood | stack stood beside the cab |
| 2D | 42 / 25 / 39 shapes at 1:50, following the wrong rig | | redrawn (E1) |
| thermal `areaM2` | hand values of the former drawing | | section 3 |
| solids | no `volumes` in the file in the earlier rounds (`model-findings.md`), not reviewed by `qa-visual-2` | | modelled anew |

Audit of the state the first author left (found by this author):

- `r3-validate.ts`: VALID, but `volumes-check.ts --product` failed: thermal areas of eight regions at 1:50 and seven at 1:100, cross-scale front and plan at 1:200.
- The 1:200 representation (the asset's `defaultScale`) was not drawable: "no suitable representation at 1:200", five print-feature violations (visible slivers of lift rods, lever brackets and tilt link, 0.145 to 0.199 mm against 0.2 mm). `validateAsset` does not report this; `r3-2d.ts` showed it.
- Line views: every filled detail inside a body of its own region (rims, hubs, steps, ladder, cab posts, louvres, grille, lamps, pins, bucket edge and hinge plates) lost its outline, because the product merges inner outlines of filled shapes of one region; the rim circles appeared as broken arcs.
- The cab glazing counted both pane faces (12.97 m2 against the reviewers' 6.0 for the outward faces).
- 1:500 structure referenced the whole rear body solid (D3).

## 3. Root fixes (old -> new)

By the first author (in the catalog since 2026-11-19, verified here):

- `dimensions`: heightM 3.4 -> 3.45 (S1), widthM (length) 8.9 -> 8.421 (rear end -3.822 to the bucket tip 4.599 at carry), depthM 2.95 kept, massKg 20,500 kept. Manifest entry updated.
- `rig`: bones re-placed, two bones added. rear-frame length 3.5 -> 3.8, front-frame 2.4 -> 2.3; lift-arm (1.1, 2.0) -18.4 deg 2.7 m -> (1.06, 2.30) -36.5 deg 2.74 m, range -8..83; bucket at 2.74, 81.5 deg, 1.47 m; lift-cyl (0.45, 1.0) -> (0.45, 0.90), barrel 1.1 -> 1.3; tilt-cyl moved from the lift arm to the front frame at (0.70, 1.95), barrel 1.62; new `tilt-lever` (bell crank on the arm at (1.90, 0.50), arms 0.80 and 0.70 m at 205 degrees) and `tilt-link` (0.90 m); wheels (-/+1.7, 0.875) -> (-/+1.675, 0.8075). Masses 9,000 / 2,740 / 1,500 / 1,700 / 230 / 140 / 150 / 80 / 260 / 100 / 2,300 / 2,300 = 20,500 kg.
- `poses` and both `clips`: every pose and key carries the solved angles of the closed loop (`offsets()`): carry (-36.5, 45), dig (-39.8, -5, edge on the ground), dump-high (45, -45, pin at 4.237 m), parked-off (bucket flat on the ground), raising; joint limits follow the solved extremes.
- `anchors`: contacts x 1.7 -> 1.675, z 1.1 -> 1.07, footprint length 0.8 -> 0.5; stack-mouth (-1.23, 3.15, -0.9) -> (-2.2, 3.3, -0.4); radiator-outlet (-3.5, 1.6) -> (-3.72, 1.92); bucket-cog (0.6, -0.1) -> (0.75, 0.15).
- `summary`, `provenance` (reference `cat-962m-2017`, notes), `symbols[0]`, `structure.assumptions`, thermal: `front-frame` retitled "Chassis: front and rear frame, axles, rims, fenders and ladders", tyres `characteristicLengthM` 1.75 -> 1.615.
- `representations` 50 / 100 / 200 / 500: all views redrawn as the true outlines of the parts (E1), decisions rewritten.

By this author:

- `thermal.regions[].areaM2`: engine-bay 21.0 -> 14.1, exhaust-stack 0.524 -> 0.61, lift-arm 12.51 -> 8.4, bucket 33 -> 22.1 (all within the factor 1.5 of C16); counterweight 14.8 -> 5.3, front-frame 25.8 -> 65.0, hydraulics 2.79 -> 4.3 (beyond the factor 1.5: the committed values described the former drawing, not this machine; applied so that object and area agree and listed with hand calculations in `area-review.md`, 2026-11-20, for the reviewers to confirm or revert). Unchanged: cab 18.98, cab-glazing 6.0, tyres 35.0.
- `representations`: every filled detail gets an unfilled outline twin (`<id>-edge`) right above it at 1:50, 1:100 and 1:200, so the line views show rims, hubs, steps, posts, louvres, lamps and pins; 1:200: handrails drawn (34 mm tube, 0.17 mm), front fenders over the tyres only, lift rods and lever brackets in the side view only, tilt link 0.16 m deep (print-suitable again); 2D decisions of 1:200 corrected accordingly.

## 4. 2D drawings (E1)

Each part is stated once and gives its solid and its shapes; front and plan of moving parts are `extruded` hints on the elevation profile, of fixed parts hand-stated true outlines. Order is far to near (true hidden lines: the near tyre covers the arm, the fender the tyre). Shape counts elevation / front / plan: 1:50 139 / 120 / 119, 1:100 108 / 112 / 109, 1:200 48 / 51 / 46, 1:500 18 / 25 / 25 (before: 42 / 25 / 39, 35 / 21 / 28, 17 / 16 / 18, 9 / 10 / 13). Print suitability (`r3-wheel-loader-suit.ts`): 0 violations at all four scales. Old shape ids could not be kept where the machine changed (new lever, link, tower); `tyre-*`, `rim-*`, `hub-*`, `cab`, `bucket`, `lift-cyl-barrel`, `lift-rod`, `tilt-cyl-barrel` and the region bindings continue.

## 5. Models per scale

| scale | body | thermal | structure | what is modelled |
|---|---|---|---|---|
| 1:50 | 138 | 137 | 61 | rear frame rails, cross-member, hitch bulkhead and plates, axle housing with differential bulge on a cradle, fuel tank, hood as a rounded four-section loft with grille, louvres, tail lamps, air cleaner, hollow stack, counterweight with chamfered plan corners and skirt, 8 mm decks and fenders, two ladders with three steps, handrails and grab rails (34 mm), cab as sill, six posts, rounded roof, 6 mm panes all round, rear panel, seat, column, door handle, mirrors, four work lamps; front frame: 40 mm tower plates, belly, nose and bulkhead plates, hitch plates and pins, pin tubes, axle housing, steering cylinders with rods, arched fenders, head lamps; 60 mm cranked arm plates with bosses, lift lugs, torque tube, lever brackets; bell crank, forked link, pins; lift and tilt cylinders as barrel with gland and base eye, rod with rod eye sliding in the barrel; bucket as a 20 mm rolled shell with side plates, cutting edge, paired hinge plates, tilt lug, top beam, back ribs; tyres as carcass rings with two staggered rows of 24 lugs, rims with flanges and disc, planetary hubs, twelve nuts. |
| 1:100 | 105 | 107 | 60 | authored for 15 mm: three-section hood, plain axle housings, 15 mm plates, thicker steps, cab with 90 mm posts without interior and lamps, mirrors kept, one hinge block per side, plain tyres (lugs merged) on rims with hubs. |
| 1:200 | 57 | 59 | 40 | authored for 30 mm: one rear frame beam, hood and counterweight prisms, deck fenders, one ladder block per side, handrails, cab as sill, glass, four posts and flat roof, tower block, simplified cranked arms with torque tube, lever, solid link bar, plain cylinders and rods, 35 mm bucket shell with edge, tyres with a rim disc. |
| 1:500 | 24 | 26 | 25 | authored for 75 mm: one rear body with decks, hood, counterweight, solid stack, cab as sill, glass band and roof, one front body, 80 mm arm plates, lever, link bar, cylinders, bucket of two cheeks and a 75 mm shell, four plain tyres. |

Thermal at every scale: `{ of }` references per region (ten regions, one layer each, no hidden regions in this asset), the inner faces of the cab panes marked `sectionFaces` (closed cab, C31 not yet in the product measurement), two outlet patches (D2): the bore disc at the stack mouth and a patch on the rear grille. Structure: the load path only: rails, cross-members, hitch plates, axle housings, tower plates and pin tubes, arms, torque tube, bucket shell with hinge plates, lever, link, cylinder barrels and rods, rims; pins as joints at the linkage, the two hitch pins and the axle ends (1:50, 1:100; fewer at 1:200, none at 1:500); four contact pads at the support anchors; masses at the centres of rear frame, front frame and both wheel pairs. 1:500 structure now has its own rear frame beam of the rail profile instead of the rear body. `decisions`: bilingual paragraphs per scale (four at 1:50, two at the others).

## 6. Check

- `volumes-check.ts machinery/wheel-loader --product`: PASS at 1:50, 1:100, 1:200, 1:500 (per-scale runs 62 / 26 / 17 / 4 s; the full run before the last text and 1:500 structure change: PASS, 82 s).
- Silhouette IoU elevation / front / plan (p95 mm): 1:50 0.9985 / 0.9933 / 0.9966 (1.8 / 15 / 9.3), 1:100 0.9970 / 0.9928 / 0.9977 (7.5 / 11 / 11), 1:200 0.9973 / 0.9936 / 0.9991 (7.5 / 7.5 / 0), 1:500 0.9973 / 1.000 / 0.9994 (19 / 0 / 0).
- Cross-scale 1:200 against 1:100: p95 0.105 / 0.1125 / 0.135 m within 0.135 / 0.1425 / 0.1575 (before: front and plan 0.285 against 0.2175).
- Outer areas 1:50 against declared: engine-bay 14.08 / 14.1, counterweight 5.31 / 5.3, exhaust-stack 0.61 / 0.61, cab 17.01 / 18.98, cab-glazing 6.56 / 6.0, chassis 64.98 / 65.0, lift-arm 8.37 / 8.4, bucket 22.13 / 22.1, hydraulics 4.32 / 4.3, tyres 39.48 / 35.0.
- `r3-validate.ts machinery/wheel-loader`: VALID (schema 0, validateAsset 0), also `--catalog-only`. Spliced with `volumes-insert.py`.

## 7. What I saw (images opened with the Read tool)

- 1:50 body sheet, six views with rig (state as found): reads at first sight as an articulated loader of the 962 class; bones lie on the pins of the linkage. Right side alone: sloping hood with louvres and grille, stack and air cleaner, cab with posts and door handle, ladder, steering cylinder across the open hitch with both pins, tower plate, cranked arm, the Z of cylinder, bell crank and link, bucket rolled back, tyres with lugs, rims with twelve nuts and hub. Front three-quarter (turn 50) and rear three-quarter (turn 230) alone: rounded cab roof with work lamps and mirrors, deck plates with handrails, arched front fenders with lamps, torque tube and bell crank between the arms, bucket interior with ribs; from behind the grille, tail lamps, counterweight with skirt, fuel tank under the rails.
- 1:50 poses `dump-high` and `dig` (six views with rig): linkage closed in both, rods inside their barrels, bucket dumping 45 degrees at a pin height of 4.24 m, cutting edge on the ground in `dig`; no part passes through another.
- 1:100 clip `load-cycle` at t = 5 s (six views): between two keys the loop stays closed to the eye.
- 1:50 structure and 1:100 structure sheets: blue rails, tower, arms, lever, link, cylinders, axle housings and rims inside the pale body, orange pins, green pads under the tyres, masses in the rear frame, front frame and wheels. 1:500 structure (before the change): the rear member was the whole rear body; replaced by a rail-profile beam; the three-quarter view was reopened after the change: beam with the hitch step under the pale body, tower, arms, cylinders, bucket, hub discs, four pads.
- 1:50 thermal `--cut` and 1:100 thermal `--cut`: ten regions in their colours, the cab hollow behind its panes, cylinders orange, chassis magenta, no stale parts.
- 1:100, 1:200, 1:500 body sheets (as found) and the 1:200 body sheet after the changes: each scale is its own abstraction and stays recognisable; 1:200 now has handrails and front fenders over the tyres only.
- `r3-2d` 1:50 before and after the outline twins (and an enlarged crop of the side elevation before): rims, hubs, steps, posts and louvres now drawn in the line view. `r3-2d` 1:100, 1:200 (first "no suitable representation", then drawn), 1:500: true projections, the three views agree.
- Not reopened after the last build: 1:50, 1:100 and 1:500 body sheets (their body lists did not change after they were looked at), 1:200 and 1:500 thermal, 1:200 structure (never opened).

## 8. Still imperfect, stated plainly

- Grade as I judge it: body A- at 1:50 and 1:100, B+ at 1:200 and 1:500; thermal A-; structure A- (B+ at 1:500). Not "perfect": the cab is a correct but plain ROPS cab (no door seam, wipers, roof gutter, interior beyond seat and column), the hood has no door seams or radiator guard frame, head lamps sit as boxes on the front fenders, hoses and drive shafts are missing, the 1:500 model has no front fenders.
- Parts beyond the data sheet (pin positions of the linkage, cylinder sizes, hood, cab, tower, counterweight shape, bucket profile) are knowledge of the class, not published numbers. The tilt cylinder is long for its 0.57 m stroke (2.08 m closed, barrel 1.62 m); a real one is probably shorter with its tower pin further forward. The overall length without bucket (7,228 mm, S1) is met only to about 0.1 m (pin at 7.09 m plus the arm nose).
- Three `areaM2` values were changed beyond the factor 1.5 of C16 (counterweight, chassis, hydraulics): applied and listed in `area-review.md`; the reviewers decide. The hand calculation of the chassis is an approximate breakdown of open plate work with both faces.
- The inner pane faces of the cab are `sectionFaces` on the thermal references: a workaround for the missing C31 flood in the product measurement (already in `tool-requests.md`), not a C25 case.
- Clips interpolate joint angles between solved keys; between keys the closed loop is only approximately closed (not visible at the frame looked at).
- Tyre lugs are boxes on the carcass, not moulded into it; the tyre does not bulge at the contact patch.

The paired mirror contours have a 0.20 mm paper stroke in the 1:100 drawing. The official document now passes its nominal and band-end paper checks.
