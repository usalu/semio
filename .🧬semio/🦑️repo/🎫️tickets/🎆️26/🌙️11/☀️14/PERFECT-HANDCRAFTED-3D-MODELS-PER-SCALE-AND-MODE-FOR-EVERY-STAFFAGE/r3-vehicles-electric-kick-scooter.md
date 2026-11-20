# Round 3 — vehicles/electric-kick-scooter

State: shipped. Catalog asset valid (`r3-validate.ts … --volumes`: schema 0, validateAsset 0, checkVolumes 0), `volumes-check.ts --product` PASS at 1:50, 1:100, 1:200 (1:500 is unavailable in 2D and in 3D), volumes spliced. No earlier round-3 work existed for this asset (no scripts, no report, catalog `volumes` equal to the source), so it was authored from the start.

Scripts (all in `T`): `r3-electric-kick-scooter-kit.py` (frames, writers), `-parts.py` (hand-stated sections and outlines), `-s50.py`, `-s100.py`, `-s200.py` (one file per scale: solids, thermal, structure, 2D, decisions), `-build.py`, `-roots.py` (root fixes, `--apply`), `-run.sh`, `-ship.sh`, `-sheet.ts` (private copy of `r3-sheet.ts` reading the candidate), `-suit.ts` (print suitability). Rebuild and ship: `./r3-electric-kick-scooter-ship.sh`.

## 1. Reference

| quantity | real subject | source | asset now |
|---|---|---|---|
| overall L x W x H unfolded | 1080 x 430 x 1140 mm | Xiaomi Mi Electric Scooter (M365) specifications, mi.com (web search 2026-11-20) | 1.09 x 0.43 x 1.11 m (class value, unchanged) |
| folded height | 490 mm | same | 0.43 to 0.48 m (hinge foot and fold lever are the top) |
| mass | 12.5 kg | same | 12.5 kg (unchanged) |
| tyre | 8 1/2 x 2 pneumatic, outer diameter 216 mm, section about 50 mm | same; ETRTO designation (`etrto-2023`) | radius 0.108, section 0.050 (was 0.045) |
| drive | 250 W hub motor in the front wheel, the motor shell is the rim | same | motor shell with side covers and rim flanges, 66 mm wide, flange diameter 154 mm |
| brakes | regenerative front, mechanical disc rear (110 mm rotor on the left) | same; my knowledge of the model | rotor ring 110/70 x 3 mm at z +0.031, caliper on the left stay |
| wheelbase | about 0.84 to 0.86 m | my knowledge (not in the manufacturer sheet) | 0.86 m (unchanged, class value) |
| deck | extruded aluminium, about 0.15 m wide, standing length about 0.45 to 0.5 m, battery inside, ground clearance about 0.09 m, ends between the wheels | my knowledge | 0.53 m extrusion + nose and tail castings (x -0.316 to 0.268), 0.15 wide, 0.095 to 0.150 high, mat 0.484 x 0.132 |
| frame front | gooseneck from the deck nose over the rear of the front wheel to a short head tube | construction of every folding e-scooter | six-section neck, head tube 0.262 to 0.345 m on the steering axis, headset cups |
| fork | steerer, crown, two blades around the motor, almost no offset | my knowledge | steerer 29 mm, crown 102 mm wide, two flat blades at z ±0.036 to 0.047, offset 2.7 mm |
| steering axis | raked back about 8 to 14 degrees | my knowledge | 8 degrees (unchanged, class value) |
| folding hinge | directly above the head tube, pin at the rear, stem folds back until the bell latches on the rear mudguard | my knowledge; height from the stacking tyre 0.216 + mudguard + crown + head tube | pin at (0.365, 0.377), hinge plane 0.38 m, fold 95 degrees |
| stem and bar | stem tube about 40 to 45 mm, handlebar 22 mm with 120 mm grips, brake lever and bell left, thumb throttle right, display on top, headlight in front | my knowledge | stem 44 to 38 mm, bar 22 mm, grips 31 to 36 mm, all parts modelled |
| rear | two stays to the axle, channel mudguard with tail light, cast spoked wheel | my knowledge | as stated |

## 2. Audit of the state before (numbers)

- **Rig**: `fork` ran from (0.40, 0.30) UP the stem for 0.8 m (it duplicated `stem`); the real fork is 0.275 m long and runs down to the axle. The hinge at 0.30 m left 84 mm between tyre top and hinge for mudguard, crown, head tube and hinge base, which cannot be built. The folded pose turned the stem by 72 degrees, leaving the handlebar 0.2 m above the mudguard (bar centre y 0.44), while `structure.assumptions` said 82 degrees.
- **2D, every scale**: the deck ran from x -0.41 to 0.38 and so 57 mm INTO the front tyre and across the rear wheel; nothing connected the deck with the steering head (no neck, no head tube); the fork was a single one-sided blade (z 0.044 to 0.064) from the hinge to the axle; the front mudguard named in the decisions and in the thermal region did not exist; the brake lever pointed straight ahead from the grip instead of lying along it; tyre 45 mm instead of 50 mm; the rear rim was bound to the `tyres` region at 1:50 and to `brake-disc` at 1:100.
- **Solids**: copies of that drawing (deck plates through the wheels, one-sided fork capsule, head "block", plain disc wheels); 1:100 body reused the 1:50 deck plates; structure referenced the body tube and blade; brake rotor was a 60 mm solid disc 6 mm thick.
- **Thermal data**: `areaM2` values had been set to the outer areas of that wrong model.
- **Dimensions**: 1.09 x 0.43 x 1.11 m and 12.5 kg agree with the manufacturer class; not changed.

## 3. Root fixes (old -> new) in the catalog file

- `rig.bones.fork`: x 0.4 -> 0.389, y 0.3 -> 0.38, angleDeg 98 -> -82 (down the steering axis), lengthM 0.8 -> 0.2751, com (-0.099, 0.0139) -> (0.15, 0), title.
- `rig.bones.stem`: y 0 -> -0.024 (hinge pin 24 mm behind the axis), angleDeg 0 -> 180, lengthM 0.8 -> 0.719, joint max 90 -> 95, com (0.4, 0) -> (0.36, -0.024), title.
- `rig.bones.wheel-f`: (x, y, angle) (-0.1943, -0.003, -98) -> (0.2751, 0.0027, 82); the axle stays at world (0.43, 0.108).
- `anchors.contact-f`: (-0.3013, 0.012) -> (0.382, -0.0123) (same world point).
- `poses.braking-stop.fork.twistDeg` 10 -> -10 (same steering side on the reversed bone); `poses.folded-charging.stem.angleDeg` 72 -> 95 with a new description; every pose gets the `rear-rim` binding.
- `thermal.regions`: new passive region `rear-rim` (cast rear wheel), `areaM2` deck-battery 0.41 -> 0.34, deck-grip 0.195 -> 0.137, hub-motor 0.059 -> 0.067, stem-bars 0.29 -> 0.208, fenders 0.05 -> 0.073, tyres 0.15 -> 0.167, brake-disc 0.009 -> 0.0126 (all within factor 1.5, from the 1:50 model; listed in `area-review.md`).
- `structure.assumptions[1]` (82 degrees, "in-plane rotation") rewritten; `provenance`: reference `xiaomi-m365-2019` added, notes rewritten.
- `symbols.kick-scooter-symbol`: deck, wheels, steering line and bars moved to the corrected body.
- `representations.500`: reason text and `limitingSizeM` 0.035 -> 0.038; `representations.50/100/200`: geometry of all three views and all decisions replaced (ruling E1: every shape is stated from the same numbers as its solid; fork, stem and wheel shapes are extruded profiles so that folding, steering and rolling carry them in all views).

## 4. What was modelled

| scale | body | thermal | structure | check |
|---|---|---|---|---|
| 1:50 | 46 solids: hollow deck extrusion, nose, tail, mat, 2 stays, six-section neck, hollow head tube, 2 cups, side stand, channel mudguards (revolved channel section with sweep), tail light, latch catch, tyres, hub motor, axle, rim, five-spoke web (extrude with five windows), hub, rotor, caliper, hinge base, pin, steerer, crown, 2 blades, hinge foot, hollow stem tube, cockpit head, fold lever, display, headlight, bar clamp, bar, 2 flanged grips, perch, brake lever loft, throttle, paddle, bell | 47: references per region, closed twins for the four hollow parts, hidden controller box | 27: deck girder, nose, tail, stays, neck, head tube, steerer, crown, blades, stem foot, tube, head, bar, motor shell, rim, web, hub; joints: 2 headset cups, hinge pin, 2 axles; masses: battery, motor, stem | PASS; IoU 0.980 / 0.959 / 0.988 (elevation / front / plan), area deviation per region <= 0.006 |
| 1:100 | 19 solids: one deck plate body, 2 plain stays, four-section neck, head cylinder, hinge base, crown, 2 plates, 2 arc mudguards, plain tyres, dished motor and rear wheel disc, one turned stem, bar, 2 grips | 22: references, 1.5 mm mat layer, controller, rotor ring | 21: hollow box girder, stays, neck, head tube, steerer, crown, blades, stem, bar, wheel bodies; 5 joints; 3 masses | PASS; IoU 0.993 / 0.977 / 0.995, area deviation <= 0.108 |
| 1:200 | 7 solids: deck plate with rear arms, neck with steering head, fork yoke, 2 wheel discs, stem, handlebar | 14: references plus own mat, controller, 2 mudguard arcs, motor, rim, rotor | 15: box girder, 2 arms, neck, yoke, stem, bars, 2 rim cylinders; 3 joints; 3 masses | PASS; IoU 0.995 / 0.957 / 0.997, total area deviation 0.001 |
| 1:500 | unavailable (2D unavailable) | | | ok |

Final check line: `PASS (5.7 s)`; `VALID vehicles/electric-kick-scooter (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`. Print suitability (`-suit.ts`): 1:50, 1:100, 1:200 authored, no violations.

## 5. What I saw in the final renders

- **1:50 body, six views** (`r3-sheet/…s50.body.png`): side: a folding e-scooter at first sight; the deck sits between the wheels, the neck rises over the rear of the front tyre into the head tube, the fork blade runs from the crown to the axle, both mudguards hug the tyres, the stem leans back 8 degrees, the hinge foot and lever sit above the head tube. Front and back: tyre between two blades under the crown, deck behind, T handlebar with grips, lever on the left, bell next to the stem. Top: rounded deck with mat, tapered tail with the wheel between the stays, neck narrowing to the head, bar across. Three-quarter: channel mudguards, five-spoke rear wheel, caliper, side stand under the left deck edge.
- **1:50 folded** (`…body.folded-charging.png`, 2D `r3-2d/…s50.folded-charging.png`): the stem turns about the rear pin, lies over the deck sloping down to the rear, the handlebar rests over the rear mudguard with the bell at the catch; nothing intersects; 2D and 3D agree, folded height 0.43 m in the drawing.
- **1:50 braking-stop** (3D and 2D): wheel, fork, mudguard, stem and bar turn together by 10 degrees about the raked axis; the clip frame `fold-and-charge` t 1.8 was rendered and is mid-fold without gaps.
- **1:50 thermal** (cut and uncut): deck red with green mat and the violet controller box inside the nose, steering assembly cyan, motor yellow, tyres green, mudguards magenta, rear wheel orange with the blue rotor; no shell pokes out. **1:50 structure**: frame, fork, stem and wheel bodies blue, cups, pin and axles orange; the mudguards, mat, grips and levers are ghost only.
- **1:100** (body and structure sheets opened): same proportions with plain parts; still clearly the same scooter; structure shows the box girder, stays, neck, steerer, crown, blades, stem and bar with the grips as ghost. **1:200** (body and thermal cut opened): seven masses; wheels, deck bar with arms, neck, yoke, stem, T bar; reads as a kick scooter in every panel. Thermal 1:200 shows the mudguard arcs, mat and controller as own solids.
- Rendered but not opened (checked by numbers only): 1:100 thermal, 1:200 structure, the folded sheets of 1:100 and 1:200, the 2D folded drawings of 1:100 and 1:200.
- **2D** (`r3-2d/…s50|s100|s200.png`) and overlays (`volumes-tools/render/…`): drawings are clean true views, overlays almost entirely green.

## 6. Still imperfect

- Geometry outside the manufacturer sheet (wheelbase 0.86 m, rake 8 degrees, deck and hinge heights, tube diameters) is class knowledge, not measured data; height 1.11 m and folded height 0.43 to 0.48 m stay 30 mm and 10 to 60 mm under the M365 figures because `dimensions` were kept.
- The latch is shown as a catch on the mudguard meeting the bell; the hook mechanism itself is below the feature size and not modelled. Cables, bolts, reflectors, charging port and tread are omitted.
- The shaded sheets tessellate round parts coarsely (visible facets on the tyres); that is the renderer's quality, not the solids.
- The validator counts sealed cavities as outer surface (measured, `tool-requests.md`): the hollow deck, head tube, steerer and stem therefore have closed twins in the thermal list instead of references.
- `deck-grip` 0.137 m2 still counts the glued under-face of the mat (as the previous value did); one face would be 0.073 m2, beyond the factor 1.5, left to the area reviewer (`area-review.md`).
- The structure lists have no support solids because `structure.supports` is empty in the asset; I did not add structural supports (physical modelling decision outside this task).
- At 1:200 the deck bar with its arms crosses the rear wheel centre in the side view (true view of the merged arms); the mudguards are not in the 1:200 body.
