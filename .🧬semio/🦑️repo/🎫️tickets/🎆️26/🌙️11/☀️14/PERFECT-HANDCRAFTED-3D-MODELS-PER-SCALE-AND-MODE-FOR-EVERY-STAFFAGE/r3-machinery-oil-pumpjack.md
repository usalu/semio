# Round 3 — machinery/oil-pumpjack

Status: DONE, spliced (round-3 author, 2026-11-20). Scripts: `T/r3-oil-pumpjack-kit.py` (the table of real dimensions and the hand-stated outlines and sections), `T/r3-oil-pumpjack-draw.py` (the 2D drawings of every scale and view), `T/r3-oil-pumpjack-solids.py` (body, thermal, structure, decisions per scale), `T/r3-oil-pumpjack-build.py` (root fixes, candidate document, apply and splice), `T/r3-oil-pumpjack-run.sh` (build, validate candidate, apply, check), `T/r3-oil-pumpjack-print.ts` (print suitability per scale). Output under `T/🗑️generated/r3-oil-pumpjack/`.

No earlier round-3 author state existed for this asset (no `r3-oil-pumpjack-*` scripts, no report); the starting point was round 2 (`author-35-pumpjack.py`, spliced 2026-11-15, `r3-validate` VALID).

## 1. Reference (real subject)

A conventional (class I lever, crank-balanced) beam pumping unit with a 74 to 75 in stroke. API Spec 11E letter dimensions: A = saddle bearing to horsehead face (horsehead radius), C = saddle bearing to equalizer bearing, I = horizontal saddle to crankshaft, P = pitman length, H = saddle bearing height above the base bottom, G = crankshaft height, R = crank radius (wrist-pin hole).

| quantity | real reference | source | asset (round 2) | verdict |
|---|---|---|---|---|
| stroke | 74 in = 1.88 m | Lufkin C-228D-173-74 / C-228D-200-74 | 2 R A / C = 1.89 m | ok |
| A (horsehead radius) | 96 in = 2.44 m (74 in units); 111-129 in for 86-100 in units | Lufkin API geometry table (catalog p. 6-7) | 3.0 m | 23 % long for 74 in, inside the API family; kept (see audit) |
| C (saddle to equalizer) | 84.05-96.05 in = 2.13-2.44 m | same | 2.7 m | 11-27 % long; kept with A (A/C 1.11, real 1.0-1.14) |
| I (saddle to crankshaft, horizontal) | I = C (84-96 in) | same | 2.7 = C | ok |
| P (pitman) | 96-114 in = 2.44-2.90 m; P ≈ 3.16 R | same; Lufkin patent US 2019/0040857 ratio table | 2.65 m = 3.12 R | ok |
| H (saddle bearing height) | 168.25-196.13 in = 4.27-4.98 m | same | 4.40 m | ok |
| G (crankshaft height) | 69.25-79.13 in = 1.76-2.01 m | same | 1.75 m | ok |
| H - G | 2.46-2.97 m (≈ 1.03 P) | same | 2.65 m (= P) | ok |
| R (wrist pin) | 32-37 in = 0.81-0.94 m | same | 0.85 m | ok |
| counterweight phase | in phase with the wrist pin (offset angle 0 on conventional units: the weights hang down when the pin is down and the horsehead is up) | Takacs 2015 ch. 3 (torque factor of conventional geometry); asset's own pose text "counterweights hang down" at crank 270 | weights OPPOSITE the pin (comX -0.3, sector at -u) | ROOT DEFECT |
| walking beam | rolled wide-flange beam of constant depth, ≈ W24 (0.61 m deep, 0.33 m flange) for a 228-class unit; centre (saddle) bearing and equalizer bearing bolted UNDER the bottom flange | Lufkin catalog structure data; AISC W24x104 d 611 mm, bf 325 mm, tf 19 mm, tw 12.7 mm | fish-belly plate outline 0.4-0.64 m centred on the pivot | redrawn |
| horsehead | curved face plate of radius A (rope groove), two flat side plates, top/bottom/rear closing plates, hinged to the beam end; width ≈ 0.6-0.7 m | Lufkin / API 11E | ±0.35 m wide wedge | kept outline, rebuilt as box + flanged face |
| equalizer | transverse beam under the beam tail on the equalizer bearing, carries both pitman arms | API 11E | absent (pitman tops floating) | added |
| pitman arms | two arms OUTBOARD of the crank arms (wrist pin bearing on the outer face of the crank) | API 11E, Lufkin assembly drawings | arms at z ±0.5..0.65, inside the crank arms (±0.45..0.7) — interpenetrating | relaid |
| crank arms | one cast arm per side on the low-speed shaft, counterweights bolted to it | same | box 0.95 x 0.28 x 0.25 | rebuilt |
| gear reducer | double-reduction cast housing, low-speed (crank) shaft at the high front end, input shaft at the low rear end beyond the crank circle, base flange on a sub-base; ≈ 1.8-2.3 m long, 0.8-0.9 m wide for 228 class | Lufkin reducer drawings | box 1.4 x 1.3 x 1.2 m around the crankshaft | redrawn |
| prime mover | 30 kW (40 hp) 4-pole TEFC NEMA 324T: shaft height 203 mm, frame Ø ≈ 0.50 m over fins, length ≈ 0.75 m with fan cover, ≈ 250 kg; shaft PARALLEL to the reducer input shaft (transverse), on a slide base for belt tension | NEMA MG-1 frame 324T; WEG/Baldor catalogues | 1.1 x 0.8 x 0.7 m box with its axis along x and the sheave beside it (inconsistent) | redrawn |
| belt guard | sheet-metal enclosure around the reducer sheave (Ø ≈ 1.0-1.2 m) and the motor sheave, outboard on one side | Lufkin | trapezoid | redrawn |
| Samson post | four legs (two front, two rear) splayed in x and z, braced, a cap with the saddle bearing blocks | Lufkin | ok, but bone `post` lay along +x | bone fixed |
| carrier bar | transverse bar (along the lateral axis) hanging on the two bridle ropes, polished-rod clamp on top | API 11B | bar 0.6 m along x | redrawn transverse |
| wellhead | casing head with flange, pumping tee with flowline, stuffing box, polished rod Ø 1.25-1.5 in | API 6A / 11B | ok | kept |
| skid and slab | two longitudinal I-beams with cross members on a concrete foundation | Lufkin | ok | kept, I-beam profiles at 1:50 |

Sources: Lufkin Industries, Pumping Units catalog (API geometry table, conventional units, pp. 5-9), text copy at https://pdfcoffee.com/download/lufkin-9-pdf-free.html; Lufkin patent application US 2019/0040857 (API dimension ratios to R) https://patents.justia.com/patent/20190040857; PetroWiki "Sucker-rod pumping units" https://petrowiki.spe.org/Sucker-rod_pumping_units; the asset's `api-11e-2013` and `takacs-2015` references; AISC Steel Construction Manual (W24x104); NEMA MG-1 (frame 324T).

## 2. Audit (round 2 against the reference)

Rig (`asset.rig.bones`), measured in the rest pose:
- `post`: angle 0, so the bone ran HORIZONTALLY from the post foot 3.6 m towards the reducer while the Samson post stands vertical (its shapes were drawn along the bone's local y). The structural member diagram of the post ran along the wrong line.
- `beam`: one bone from the saddle 5.0 m along +x, i.e. 2.3 m past the equalizer (2.7 m) into the air, while the front arm (saddle to horsehead, 3.0 m) lay outside the bone: the member diagram dropped the horsehead side and the beam's own weight (`comX` -0.3 is outside 0..L).
- `pitman`: angle 0 relative to the beam, the bone pointed along the beam (world -18.6 degrees at mid-stroke) while the pitman hangs down to the wrist pin (world -108.6 degrees): the shapes were drawn along local -y, `comY` -1.4. Tip of the bone 3.6 m away from the wrist pin.
- `crank`: `comX` -0.3 (counterweights opposite the wrist pin) - contradicts the reference and the asset's own pose text (stopped at the top of the stroke with the pin down: "the counterweights hang down").
- `rod`: bone along +x (0.6 m towards the post) for a vertical polished rod; `base` 7.8 m long from x = 0 although the slab ends at x = 5.9 (1.9 m of bone in the air); `motor` 1.1 m along x for a motor whose shaft must be transverse; `gearbox` 1.4 m for a 2.35 m reducer.
- Kinematics A, C, I, P, H, G, R: within the API 11E family (table above); kept, so poses and clips are unchanged.

Dimensions: height 5.57 m (horsehead top at mid-stroke) and depth 2.4 m (slab) correct; width 9.2 m against the modelled 9.1 m (wellhead flange -3.2 m to slab end 5.9 m) is inside the 2 % tolerance, kept.

2D drawings (all scales, all views): fish-belly beam 0.40 to 0.64 m centred on the pivot (real: straight W24, 0.61 m, above the bearings); counterweight sector (r 0.35 to 1.12 m, +-52 degrees) on the side opposite the wrist pin; crank arm a 0.95 x 0.28 m bar; pitman arms at z +-0.50..0.65 inside the crank arms (z +-0.45..0.70): the two interpenetrate; no equalizer (pitman tops floating at z +-0.57 under a 0.36 m beam); reducer a 1.4 x 1.3 m rounded box centred on the crankshaft; belt guard a trapezoid ending at a motor sheave beside a motor whose axis was along x (front 0.8 m wide, plan 1.1 m long in x): the drive could not exist; carrier bar 0.6 m long along x (real: transverse); saddle bearing a dot on the post top. The print suitability of 1:100 and 1:500 failed already (round 2 drawings refused by `resolveRepresentation`, "no suitable representation").

Solids (round 2): reasonably complete but built on the defects above (fish-belly plates 40 mm, counterweights opposite the pin, pitman inside the crank arms, motor box along x), boxes for the skid "I-beams", one pitman arm visible from one side (QA), structure a mixture of body references with masses as boxes.

## 3. Root fixes (in the catalog file)

| member | old | new |
|---|---|---|
| `rig.bones.base.lengthM` | 7.8 | 5.9 (bone ends at the slab end) |
| `post` | angle 0, mass com (0, 1.75) | angle 90 (vertical), com (1.75, 0); title names legs, cap and saddle blocks |
| `beam` | x 0, y 3.85 in the post frame, angle 0, length 5.0, mass 2900 kg com (-0.3, 0.1) | x 3.85, y 0, angle -90 (same world frame), length 2.7 (saddle to equalizer bearing), mass 1867 kg com (0.81, 0.4) |
| `horsehead` (new, parent `beam`, fixed) | - | x 0, y 0, angle 180, length 3.0 (saddle to horsehead face = A), 1033 kg com (2.3, -0.2); total beam mass 2900 kg and its moment about the saddle (-870 kg m, the head-heavy structural unbalance of about 290 kg at the rod) unchanged |
| `pitman` | angle 0, com (0, -1.4) | angle -90 (bone runs down the arm to the wrist pin in every pose), com (1.4, 0) |
| `gearbox` | length 1.4, com (0, -0.45) | length 1.65 (to the rear face), com (0.5, -0.4) |
| `crank` | com (-0.3, 0) | com (0.6, 0): counterweights in phase with the wrist pin |
| `rod` | angle 0, length 0.6 | angle 90, length 0.16 (carrier bar and rod clamp, vertical) |
| `motor` | length 1.1, com (0, 0.05) | length 0.27 (frame radius), com (0, 0) on the shaft axis |
| poses, clips, anchors | - | unchanged: offsets are relative to rest, the joint origins and world frames of every moving bone are the same; the four-bar still closes (checked: pitman tip on the wrist pin at crank 270, 180 and 90 degrees) |
| `representations.*.geometry` | round-2 drawings | redrawn at all four scales and three views from the same dimension table as the solids (E1): ids kept where the part exists (`slab`, `skid`, wellhead ids, `polished-rod`, `leg-*`, `brace`, `saddle`, `beam`, `centre-bearing`, `tail-pin`, `horsehead`, `horsehead-web` = face plate, `pitman*`, `carrier`, `counterweight*`, `crank-arm*`, `crank-hub`, `gearbox`, `pedestal`, `belt-guard`, `motor*`, `terminal-box`); new: `equalizer`, `rod-clamp`, `input-shaft`, `motor-shaft`, `guard-bracket-*`; dropped where hidden: `crank-pin` (behind the pitman eye), `motor-sheave` (inside the guard), elevation `motor-cowl` (behind the frame) |
| `representations.*.decisions` | round-2 text | silhouette, retained, merged, omitted, thinElements (all scales), styles (1:50) and overlays rewritten en/de |
| `thermal.regions.areaM2` (C16, within factor 1.5, also in `area-review.md`) | gearbox 10.28, crank 9.7, motor 7.0 | 13.2, 6.65, 4.7 |
| `provenance.notes` | "API 114 class", origin "at the wellhead axis" (wrong: the origin is under the saddle) | the 74-75 in class of a Lufkin C-228D-173-74 with the API letters, corrected origin, transverse motor; references `lufkin-catalog` and `nema-mg1` added |

Print suitability (office-laser) is now met at all four scales (round 2: 1:100 and 1:500 refused).

## 4. Models per scale and list

Every part is stated by hand in world rest coordinates and converted to its bone frame; front and plan of the moving parts are their elevation profiles extruded across their real lateral spans (z, left +): gearbox +-0.42, counterweights 0.43..0.59, crank arms 0.59..0.75, hubs 0.48..0.78, pitman arms 0.77..0.89, eyes 0.75..0.91, equalizer +-0.91, belt guard -1.13..-0.93, beam flange +-0.1625, horsehead +-0.28 with the face plate +-0.35. The pitman sweeps outboard of the crank and counterweights, the reducer input shaft (1.37 m behind the crankshaft) lies outside the 1.12 m crank circle, and the counterweights clear the skid deck by 80 mm with the pin down: the mechanism can turn without collision.

| scale | body / thermal / structure | body |
|---|---|---|
| 1:50 | 73 / 73 / 47 | slab; two rolled I-beams (240 mm, 15/12 mm) with five channel cross members and 10 mm deck; casing head with flange, pumping tee, stuffing box, flowline; polished rod (slender, 50 mm) to the horsehead tangent; four 170 mm legs (rounded-rect lofts, buried in the skid), square brace frame, cap plate, two saddle blocks, saddle shaft; W24 beam (611 x 325 mm, tf 19, tw 12.7) with four web stiffeners over the bearings; centre bearing housing; equalizer hanger plates and top plate; horsehead box (r 2.97) with the 30 mm face plate of radius 3.0 overhanging the sides; per side pitman arm, upper fitting, eye housing; equalizer in two arms and its bearing; per side crank arm (hull of two circles), counterweight sector (r 0.55-1.12, +-38 degrees, chamfered), hub, wrist pin; reducer housing profile (arch r 0.6 over the bull gear, slope to the input end, base flange), sub-base, two bearing caps, input cap, input shaft; belt guard (hull of the 1.16 m and 0.40 m sheave circles) on two brackets; motor: 36-fin frame, drive-end shield, fan cowl, shaft, terminal box, two feet, slide base; carrier bar (transverse, 0.56 m) and rod clamp |
| 1:100 | 62 / 62 / 47 | same assemblies; I-beams with 15 mm web and flanges, bar cross members, 15 mm deck; wellhead two stepped revolves; beam I with 20/16 mm; horsehead one extrude +-0.35; equalizer one piece; no stiffeners, no guard brackets; motor 18 fins of 15 mm, shield, cowl, shaft, terminal box, merged foot block |
| 1:200 | 41 / 41 / 42 | skid two box beams, three cross bars, 30 mm deck; wellhead one stepped revolve; beam I generalised to 30 mm flanges and web; hanger one block; pitman arms with the eyes in their outline; crank arms, counterweights, hubs; reducer coarse arcs, input shaft; motor one cylinder over frame and cowl, shaft, foot block |
| 1:500 | 31 / 31 / 35 | skid two beams under a 75 mm deck; beam its 611 x 325 envelope; cap 75 mm; no braces, hubs, hanger (the equalizer reaches up to the beam flange); polished rod 0.1 m, carrier 0.16 m and shafts widened to the 75 mm feature as drawn; reducer polygon |

Thermal: every solid of the body is referenced in its region (base, wellhead, steel, gearbox, crank, motor); all regions have one layer, no hidden regions, no emitters in the asset, so there are no nested shells. Structure (D3): members = slab, skid beams and cross members, legs, braces and cap, polished rod, carrier bar, the beam split at the saddle into its own I-section tail arm (`beam`) and front arm (`horsehead`) plus the horsehead box, pitman arms with fittings and equalizer, crank arms, reducer housing with sub-base, motor frame; 7 joints (saddle shaft, centre bearing, equalizer bearing, two wrist pins, two crank hubs; own cylinders where a coarse body has no pin); 7 masses (two counterweights, and compact solids at the centres of mass of foundation, reducer, motor, beam and horsehead, sized per scale). The asset declares no `structure.supports` (it stands on four contact anchors in the poses), so there are no support solids.

## 5. Check lines

`$BUN volumes-check.ts machinery/oil-pumpjack --product` (`🗑️generated/r3-oil-pumpjack/check-<scale>.txt`): 1:50 PASS, 1:100 PASS, 1:200 PASS, 1:500 PASS.
- Silhouette IoU elevation / front / plan, p95: 1:50 0.998 / 0.989 / 0.999 (0 / 24 / 0 mm); 1:100 0.999 / 0.993 / 0.999 (0 / 26 / 0 mm); 1:200 0.999 / 0.997 / 0.999 (0 / 0 / 0); 1:500 0.999 / 0.951 / 0.999 (0 / 112 / 0 mm, limit 300).
- Areas 1:50 (outer, against areaM2): base 84.96 / 84.9, wellhead 1.57 / 1.45 (+8 %), steel 44.3 / 47 (-6 %), gearbox 13.19 / 13.2, crank 6.63 / 6.65, motor 4.44 / 4.7 (-6 %); 1:100 within 30 % (motor -27 %, wellhead +14 %); totals 1:200 12 %, 1:500 15 %.
- Bounds, feature, monotonic, cross-scale, structure coverage (outside body 1 / 2 / 3 / 8 mm), bones, slender, copied: ok at every scale.
- `$BUN r3-validate.ts machinery/oil-pumpjack --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0; `--catalog-only`: VALID (spliced).
- Print suitability (`r3-oil-pumpjack-print.ts`, office-laser): suitable at 1:50, 1:100, 1:200, 1:500.

## 6. What I saw (final renders)

- 1:50 sheet (`r3-sheet`, six views with rig): the rig now lies on the machine: post vertical up the A, beam from the saddle to the equalizer, horsehead from the saddle to the face, pitman from the equalizer down to the wrist pin, crank from the shaft to the pin, rod bone short and vertical on the carrier. Right side (elevation): a straight W-beam above the centre bearing and the equalizer hanger, curved horsehead face over the wellhead, A-frame with brace and saddle, the long reducer with its arch and sloping top, the teardrop belt guard in front of it and the motor's finned end at its tail; the counterweight sits on the wrist-pin side under the pitman eye. Front: legs splayed to the skid, the equalizer across the full width with both pitman arms outboard of the crank arms and counterweights, the reducer between them, guard and transverse motor at the -z side. Top: X of the legs and the brace frame, beam centred, equalizer and both arms, reducer, guard on one side, motor across.
- Three-quarter from the +z side (turn 230, tilt 20) and from the guard side (turn 40, tilt 22): reads like a Lufkin conventional unit: flanged face plate on a side-plate horsehead, I-beam with stiffeners, pillow block saddle on the cap, cast crank arm with the counterweight on its inner face, wrist pin and pitman eye outboard, bearing caps on the reducer, guard over both sheaves with the motor shaft entering it, 36-fin motor on its slide base, I-beam skid with deck on the slab.
- Poses: `stopped` (crank 270): pitman straight down to the pin, both counterweights hanging below the shaft 80 mm above the deck, horsehead up, carrier at the top of the rod line; `bottom-of-stroke` (2D 1:100): pin and counterweights up, horsehead down, carrier low; the rope line always meets the face at saddle height. Clip `pump-cycle` t 1.0 s at 1:200 (right side with rig, crank 390 degrees): beam tilted head-down, pitman from the equalizer to the pin, counterweight on the pin side rising, carrier low on the rod line, the rope line touching the face at saddle height.
- 1:100, 1:200, 1:500 sheets: the same machine in coarser sections; at 1:500 the beam is a box and the parts are thicker but every assembly is still there in proportion.
- Thermal 1:50 and 1:100 (six views each): six region colours on the right parts (red base, green wellhead, violet steel, yellow reducer, cyan crank, magenta motor), no stale or protruding solids. Structure 1:50 and 1:100: blue members along the real load path (legs, braces, beam arms, horsehead, pitman arms, equalizer, crank arms, housing, skid, slab), orange joints at the saddle, equalizer, wrist pins and hubs, magenta counterweights and small mass bodies inside beam, horsehead, reducer, motor and slab.
- Overlays (`volumes-render` 1:50, final): elevation 0.999, plan 1.000 all green; front 0.989 with red hairlines only at the 15 mm slits between the skid flanges and the cross members, and a few blue pixels at the wellhead flowline.

## 7. Still imperfect

- The beam geometry A = 3.0 m and C = 2.7 m are about a quarter longer than the Lufkin 74-in unit (2.44 / 2.13 m); proportions and stroke are right and poses, clips and the wellhead position depend on them, so I kept them and say so in the notes.
- The bridle (two wire ropes from the horsehead groove to the carrier bar) is modelled, as drawn, as the one static polished-rod line from the stuffing box to the tangent point: a rope of changing length cannot follow both the horsehead and the carrier with rigid bones. The polished rod is static on `base`; only the carrier bar moves along it.
- Not modelled: ladder and saddle platform, brake drum and lever, belts and sheaves inside the guard (the guard is its closed envelope), bolts, the horsehead latch. The bearings (saddle shaft in the centre housing, equalizer bearing between the hanger plates, wrist pin in the eye) are solids that overlap as shafts in bores.
- The skid deck spans the full width under the cranks, so the counterweights are limited to r 1.12 m (real units let them swing beside a narrower base); the crank mass of 2600 kg is consistent with this geometry.
- Motor areaM2 4.7 is the C16 limit (factor 1.5 from 7.0); the model measures 4.44 at 1:50 and 3.44 at 1:100 (no fins finer than 15 mm), both inside the tolerance.
- No `structure.supports` in the asset, hence no support solids (the four foundation contacts live in the poses).
