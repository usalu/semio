# Round 3 — vehicles/cargo-bike (two-wheeled long-john, Bakfiets.nl Cargobike Long class)

Status: DONE on 2026-11-20. Catalog asset written, volumes spliced, `r3-validate.ts vehicles/cargo-bike --catalog-only` VALID (schema 0, validateAsset 0), all four scales printable. Nothing from an earlier author of this asset existed (no `r3-cargo-bike-*` scripts, no partial report), so the work started from the catalog state.

Scripts (all in `T`, inputs kept):

- `r3-cargo-bike-kit.py` — rest frames of the corrected rig, steering-axis helpers, writers for solids, thermal entries, structural parts and 2D shapes (an own adaptation of the city-bicycle kit; the region of a shape is optional).
- `r3-cargo-bike-parts.py` — key points, profiles and paths stated by hand (wheels, beam with fillet and gooseneck, box, crates, handlebar, chain, steering cables, frame centre of mass).
- `r3-cargo-bike-s50.py`, `-s100.py`, `-s200.py`, `-s500.py` — one file per scale: each part as body solid, thermal entry, structural part and drawn shape, with bilingual decisions.
- `r3-cargo-bike-build.py` — writes `volumes/vehicles/cargo-bike.json` and `🗑️generated/r3-cargo-bike/drawings.json`.
- `r3-cargo-bike-roots.py` — every root fix outside `volumes` (rig, anchors, poses, clip keys, texts, references, dimensions, supports, areas, drawings, decisions); candidate first, `--apply` writes the catalog.
- `r3-cargo-bike-sheet.ts` — private copy of `r3-sheet.ts` that renders the candidate document before the rig is in the catalog; `r3-cargo-bike-suit.ts` — print suitability of every scale (the product refuses to draw an unsuitable scale; `validateAsset` does not say so).

Rebuild: `python3 r3-cargo-bike-build.py && python3 r3-cargo-bike-roots.py`, `$BUN r3-validate.ts --file 🗑️generated/r3-cargo-bike/🔣️.json`, `$BUN r3-cargo-bike-suit.ts --file <same>`, then `python3 r3-cargo-bike-roots.py --apply` and `python3 volumes-insert.py volumes/vehicles/cargo-bike.json <catalog file>`. The generated output of this asset (candidate, drawings, renders, check logs) was deleted at the end; `r3-cargo-bike-roots.py` writes the candidate again for `r3-cargo-bike-sheet.ts`. Source `volumes/vehicles/cargo-bike.json` equals the catalog `volumes`.

## 1. Reference (real subject)

Subject: a riderless two-wheeled long-john e-cargo bike of the Bakfiets.nl Cargobike Long class (aluminium frame, Shimano EP6 mid drive, 504 Wh battery, wooden box with child bench), 26 inch rear and 20 inch front wheel, steered by cables from the handlebar column to the fork, two-leg stand under the box.

| quantity | real value | source | asset before | asset now |
|---|---|---|---|---|
| overall length x width x height | 260 x 64 x 120 cm (Bakfiets.nl Long); Urban Arrow Family 263 x 68.5 cm | Bakfiets.nl Product Data Sheet 2026 (`bakfiets-pds-2026`, added), Urban Arrow service centre (`urban-arrow-2025`, added) | 2.69 x 0.65 x 1.16 m | 2.60 x 0.64 x 1.16 m (height kept: bar set 4 cm below the 1.20 m maximum) |
| wheelbase | 200 cm | Bakfiets.nl PDS 2026 | 2.10 m | 2.00 m |
| mass | 45 to 50 kg (Urban Arrow 48 to 50 kg) | Urban Arrow, Bosch listing | 45.8 kg | 45.8 kg (kept) |
| rear tyre 50-559 | outer diameter 659 mm, bead seat 559 | ETRTO (`etrto-2023`) | r 0.33 | r 0.33, section 50 x 47 mm on a 30 mm double-wall rim |
| front tyre 50-406 | outer diameter 506 to 510 mm | ETRTO | r 0.255 | r 0.255 |
| front head angle / fork offset / trail | 68 to 72 degrees, 30 to 45 mm offset, trail 50 to 70 mm for a 20 inch cargo fork | own knowledge (long-john geometry) | steering axis 55.6 degrees (34 degrees from vertical), fork bone not on the steering axis | 70 degrees, 35 mm offset, trail 56 mm, fork bone on the axis |
| fork crown to axle | 300 to 330 mm along the axis for 20 x 2.0 | own knowledge | none | 288 to 310 mm crown, 35 mm tyre clearance |
| handlebar column | long upright steering column at the rear of the box, 75 to 82 degrees, swept-back bar 0.60 to 0.64 m wide, grips 1.10 to 1.15 m | own knowledge, manufacturer photos | vertical bone at x 0.74 standing on the frame line, no head tube | 80 degree column with 355 mm head tube on the beam, riser, stem, bar 0.60 m, grips at 1.13 m |
| bottom bracket | 0.45 to 0.50 m ahead of the rear axle, 0.28 to 0.31 m high | own knowledge | 0.55 / 0.32 m | 0.47 / 0.30 m |
| seat tube | 66 to 70 degrees, saddle top 0.95 to 1.02 m | own knowledge | drawn post 0.2 m, saddle at 0.98 | 68 degrees, post to 0.958 m, saddle top 1.00 m |
| main beam | one large aluminium tube or box section 50 to 60 x 70 to 90 mm, straight under the box, bending up behind the front wheel into the front head tube | own knowledge (Bakfiets.nl, Urban Arrow, Bullitt) | 52 x 60 mm straight beam plus a separate riser | 50 x 70 mm box section, 9 cm fillet, gooseneck 32 to 33.5 cm from the front axle |
| box | Bakfiets.nl Long box 148 l, about 0.75 to 1.0 m long, 0.6 m wide, sides about 0.3 m, floor 0.28 to 0.30 m, child bench at the rear, front panel raked over the wheel | Bakfiets.nl PDS (148 l), WorkCycles review (about 100 x 60 cm at its widest), own knowledge | 1.0 x 0.65 x 0.65 m, floor 0.35 m, front notch | 0.74 to 0.83 x 0.64 x 0.348 m outside, floor top 0.302 m, inside 0.72 x 0.616 x 0.318 m = 141 l, bench 0.26 m deep at 0.50 m |
| stand | long-johns stand on a wide two-leg stand under the front of the box that lifts the front wheel a few centimetres | own knowledge (Urban Arrow, Babboe, Christiania support under the box) | "rear stand" behind the rear axle lifting the rear wheel | two-leg stand under the box 1.55 m ahead of the rear axle, legs 0.235 m splayed to 0.41 m, front wheel lifted 3.3 cm |
| mid-drive motor | housing about 180 x 140 x 85 mm around the crank axle, chainline 47 to 50 mm | Shimano EP6/EP8 data (own knowledge) | ellipse at 0.55 m | 85 mm housing under a cast bracket, chainline 49.5 mm, 38:20 chain |
| battery | down-tube battery about 0.35 x 0.09 x 0.08 m | Shimano BT-E6010 class (own knowledge) | box on a bone at x 0.4, y 0.69 | 0.34 x 0.085 x 0.092 m on the down tube |
| brakes | hydraulic discs 180 mm on 100 mm front / 135 mm rear hubs, rotor faces 15.5 mm inside the lock nut | IS/post-mount standard (own knowledge) | two 160 mm discs | 180 mm rotors at z 33.5 and 52 mm, calipers |
| crates | Euronorm stacking container 600 x 400 x 270 mm with lid, two stacked, 60 kg | own knowledge (Euronorm module) | 0.50 x 0.52 x 0.28 m blocks | two lidded tapered containers 600 x 400 x 270 mm |

## 2. Audit against the reference (state found on 2026-11-20)

Rig:

- `fork`: origin (1.85, 0.62), angle -55.58 degrees, length 0.4424 to the axle: the bone ran from the head tube top straight to the axle, so its twist axis was not the steering axis (the fork swung the wheel about a line 20 degrees off the real one) and the "head angle" read 55.6 degrees. Root defect.
- `bar`: a vertical bone at x 0.74 standing on the frame with no head tube; the handlebar turned about a vertical line in the air. Root defect.
- `stand`: "rear stand" at (-0.08, 0.30) behind the rear axle, lifting the rear wheel; the long-john's real stand is the wide two-leg stand under the front of the box. Root defect; contacts, poses, clip contacts, the structure assumption and the supports followed it.
- Wheelbase 2.10 m (reference 2.00 m), bottom bracket 0.55 m ahead of the rear axle (0.47 m), box bone at floor 0.35 m and 1.0 m long, box front at 1.95 m overlapping the front wheel (tyre back at 1.745 m), crate bones at floor 0.37 m.
- `frame` centre of mass (0.9, 0.45) was a guess; hand sum of the parts gives (0.714, 0.458).

Dimensions: length 2.69 m against 2.60 m and width 0.65 against 0.64 m in the manufacturer sheet; corrected.

2D (all four scales): drawings of the old geometry (box 0.65 m tall, front notch, vertical bar post, rear stand, crank arms through view bones `arm-n`/`arm-f` that no rig has), 1:100, 1:200 and 1:500 were unprintable (`resolveRepresentation` unsuitable: motor-housing feature 0.159 mm, rack/fender gap 0.09 mm, box wall 0.148 mm, steer-rod/box-wall gap 0.106 mm).

Solids (old 1:50, 84 body solids; coarser scales alike): box torso of the box, a flat floating tray as rear rack, a vertical bar post with a horizontal stub, segmented-cylinder tubes, crates as rounded blocks, discs as hubs, the stand at the rear wheel, no steering linkage, battery hanging in the air ahead of the seat tube.

## 3. Root fixes (catalog asset, outside `volumes`)

| member | old | new | reason |
|---|---|---|---|
| `rig.bones[frame]` lengthM, title, mass com | 1.85, "Frame with rear triangle and main beam", (0.9, 0.45) | 2.0, "... main beam and gooseneck", (0.714, 0.458) | wheelbase; hand sum of 14 parts (`frame_com()`) |
| `rig.bones[fork]` x, y, angleDeg, lengthM, title, com | 1.85, 0.62, -55.58, 0.4424 | 1.7961, 0.7129, -70, 0.50, "Front steering head, fork and cable drum", (0.30, 0.02) | bone on the real steering axis from the steerer top to the foot opposite the axle |
| `rig.bones[wheel-f]` x, y, angleDeg | 0.4424, -0.0001, 55.58 | 0.50, 0.035, 70 | axle 35 mm ahead of the axis (fork offset); axle at (2.00, 0.255) |
| `rig.bones[bar]` x, y, angleDeg, lengthM, title, com | 0.74, 0.62, 90, 0.43 | 0.82, 0.29, 100, 0.855, "Handlebar column, stem and handlebar", (0.70, 0.02) | the 80 degree handlebar column from the beam to the bar clamp |
| `rig.bones[box]` x, y, lengthM, com | 0.95, 0.35, 1.0, (0.5, 0.25) | 0.86, 0.29, 0.84, (0.38, 0.13) | box on the beam between column and gooseneck |
| `rig.bones[battery]` x, y, angleDeg, lengthM, com | 0.4, 0.69, 0, 0.4, (0, 0) | 0.5407, 0.3955, 51.552, 0.34, (0.16, 0.0665) | battery on the down tube |
| `rig.bones[motor]`, `[crank]` x, y; motor com | 0.55, 0.32; (0, 0) | 0.47, 0.30; (0.02, -0.01) | bottom bracket |
| `rig.bones[stand]` x, y, angleDeg, lengthM, joint, title, com | -0.08, 0.30, -95, 0.3433, revolute -10..90, "Rear stand", 0.17 | 1.55, 0.205, 178 (folded back under the beam), 0.235, revolute 0..105, "Two-leg stand under the box", 0.12 | the real long-john stand |
| `rig.bones[crate-x]`, `[crate-y]`, `[crate]` | (1.15, 0.37), lengths 0.5 / 0.56 / 0.5 | (1.18, 0.302), 0.40 / 0.54 / 0.40 | crates on the new floor, real container size |
| `anchors` | contact-f (0.6528, -0.1442); stand-pad x 0.3433, 0.10 x 0.08, "Stand foot pad"; crate-cog (0.25, 0.28) | contact-f (0.7396, -0.0522); stand-pad x 0.235, 0.05 x 0.46, "Stand feet"; crate-cog (0.20, 0.27) | new rig |
| poses `parked-on-stand`, `loaded-on-stand` | root -1.09 degrees, stand -95, contacts contact-f + stand-pad, crates dx -2.35 dy -0.393 | root +0.952 degrees (stand feet and rear patch level, front wheel 3.3 cm up; closed form in `park_angle()`), stand +102 (feet 10 degrees ahead of vertical, over centre), contacts contact-r + stand-pad, crates dx -2.48 dy -0.2804 (on the ground 1.3 m behind the axle) | real parking |
| other poses | stand 80 | stand key removed (folded rest) | |
| clip `load-box` | contacts contact-f + stand-pad, carry height dy 0.75, crate keys of the old floor | contacts contact-r + stand-pad, carry dy 0.90 (crates pass 2 cm over the handlebar), ground keys as the pose | collision-free carry |
| texts | summary, provenance notes, pose titles and descriptions, load-box description, structure assumption with "rear stand" and "front tyre carries most" | rewritten (en, de): two-leg stand under the box, front wheel lifted, stand carries about 73 % | follow the rig |
| `provenance.references` | | + `bakfiets-pds-2026`, `urban-arrow-2025` | cited dimensions |
| `dimensions` widthM, depthM | 2.69, 0.65 | 2.60, 0.64 | Bakfiets.nl PDS |
| `structure.supports` | ["contact-f"] | ["contact-r"] | the rear patch is the contact present in every pose and clip |
| `thermal.regions` areaM2 | frame 1.6, box-wood 5.4, battery-pack 0.15, tyres 0.635 | 1.35, 3.61, 0.125, 0.51 | honest 1:50 outer areas (1.348, 3.52, 0.123, 0.509); box-wood limited by C16 to factor 1.5 and listed in `area-review.md` |
| `representations[50,100,200,500]` geometry, pens, decisions | drawings of the old body | the true views of each scale's body, see section 5 | ruling E1 |

Unchanged on purpose: heightM 1.16, massKg 45.8, materials, wheel radii, crank length, clip `roll-by` and the ride keys (wheel angles follow the unchanged radii), thermal layers, heat and operating keys.

## 4. Models per scale

| scale | body | thermal | structure | what is modelled |
|---|---|---|---|---|
| 1:50 | 136 | 93 | 108 | tyres (torus sections), double-wall rims, 36 crossed spokes per wheel, gear hub and front hub with flanges, 180 mm rotors (texture), axles, nuts, sprocket; 50 x 70 beam with fillet and gooseneck, both head tubes with headset cups, strut, oval down tube, seat tube with clamp and post, chain- and seatstays to dropouts, cast motor bracket, chainstay bridge, three box rails, stand bracket; tube carrier with rails, five cross bars, four legs, two struts, tail plate and lamp; comfort saddle (superelliptic loft) with clamp and rails; 65 mm plastic mudguards (2.5 mm arched sheet) with wire stays; motor housing; battery with lock; chain runs, chainring and guard (texture), crank arms (lofts), spindles, pedals; fork steerer cap, crown, straight blades, ends, cable drum, caliper, head lamp with lens and bracket; column steerer, drum, riser, stem, bar clamp, swept-back bar, grips, levers, display, bell; two housed steering cables; plywood box (floor, sides, rear wall, raked front wall, four capping strips, bench seat and panel); two lidded Euro containers; folded U stand with pivot, bent legs, cross tube and feet. Thermal: every region references its body solids. Structure: hollow sections with real walls, box-section beam (extrude with hole) and gooseneck loft, bearings, 36 spokes, cables, box panels, crate base and post, masses, rear patch support |
| 1:100 | 78 | 58 | 82 | merged: cups into head tubes, hub/flanges/nuts one shell, rotor and chainring as drawn rings, twelve spokes, carrier as hoop + two legs + two struts, saddle without rails, lamp on the crown, levers and bell into the grips, capping into the walls, bench one block, each crate one tub with lid, straight stand legs |
| 1:200 | 47 | 39 | 66 | wheel ring (tyre and rim) with plain hub (carries the brakes region) and six spokes; beam with three-step gooseneck, head tubes, strut, down tube, seat tube to the saddle, stays, fork blades, handlebar and stand legs generalised to 30 mm; motor, bracket, battery, saddle pad, carrier hoop, mudguards, crank bars, pedals, crown block, lamp, riser, display; box with 20 mm walls and bench; crates as blocks |
| 1:500 | 33 | 31 | 60 | mass model at 75 mm: 80 mm wheel rings with hub drums, 30 mm mudguard bands, every tube a 75 mm bar on its real centre line, one chain- and seatstay per side, motor, battery, saddle pad, carrier deck, crank bars, crown block and two blades, riser and handlebar narrowed to 0.64 m, the box one block with the upper crate standing out, two stand legs |

Final check (`$BUN volumes-check.ts vehicles/cargo-bike --product`, 54 s): `FAIL: 4 measurement(s) over 3 rule line(s)`. Everything is green except `volumes-structure-coverage` `outsideBodyM` at 1:50 (elevation 0.066 against 0.0075, plan 0.013), 1:100 (0.065 against 0.015) and 1:200 (0.057 against 0.03): the `mass-frame` marker at the true centre of mass of the open frame (0.714, 0.458) lies in the air 76 mm from the nearest tube. Proved by a probe without it: 0.0065 / 0.0069 / 0.0093 m elevation, all within limits. Filed in `author-requests.md` ("r3 vehicles/cargo-bike"); the marker was not moved (D3).

| rule | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| silhouette IoU elevation / front / plan | 0.959 / 0.967 / 0.985 | 0.960 / 0.960 / 0.917 | 0.982 / 0.965 / 0.986 | 0.989 / 0.978 / 0.980 |
| bounds deviation length / height / width (mm) | 5 / 4 / 8 | 5 / 4 / 0 | 5 / 12 / 0 | 38 / 4 / 0 |
| thermal area deviation, largest region | 13.2 % (touch-parts) | 22.4 % (box-wood) | total 14.8 % | total 13.9 % |
| cross-scale IoU to the finer scale | | 0.988 / 0.993 / 0.986 | 0.994 / 0.992 / 0.998 | passes (C14 limits) |
| print suitability (min feature) | 0.202 mm | 0.211 mm | 0.210 mm | 0.254 mm |

## 5. Drawings (ruling E1)

All three views of all four scales are written by the same statements as the solids: straight tubes are `member` shapes, plates, bent tubes, the beam with gooseneck and round parts about the lateral axis are profiles with an `extruded` solid hint, only saddle and crates have their own outline per view. Shapes on `fork` and `bar` are members or solids, so the steered pose draws right. Paint order is the real depth order. Hidden parts that only made slivers were dropped per view (box rails 1 and 2, the caliper and the floor at 1:200 in the front view, the left stays at 1:200 and 1:500 in the side view), which is what made every scale printable.

## 6. What I saw in the renders

- 1:50 body, six-view sheet, right side alone, three-quarter from front-right (turn 125) and from front-left (turn 30): reads at once as a Dutch long-john. Side: low beam under the box sweeping up behind the 20 inch wheel into the short head tube, the upright column with riser and swept-back bar, battery on the down tube, motor under its bracket, chain to the rear hub, carrier over the 26 inch wheel, the raked plywood box with the upper crate standing out of it, the stand down under the box, front wheel lifted. Front and back: box 0.64 m wide, bar narrower, stand feet splayed. Top: bar sweep, saddle pear, carrier grid, bench line in the box, crate, fork and lamp. Tyres show the renderer's facets.
- 1:50 structure: blue members follow every tube, the beam and gooseneck, box panels and crates; magenta masses at hubs, motor, battery, box, crates, fork and column; the frame mass floats between down tube and beam (as stated). 1:50 thermal: nine regions in their colours on the body surface, nothing pokes out.
- 1:100, 1:200, 1:500 sheets: the same proportions with fewer and simpler parts; 1:500 is a clean mass model, still a long-john at a glance.
- Pose `turning-left` (3D and 2D): column and bar turned, front wheel turned about the 70 degree axis, stand folded under the beam, both tyres down. Clip `load-box` at 4 s (1:100): crate stack carried above the handlebar, stand down, front wheel lifted.
- 2D 1:50, 1:200 and 1:500 sheets (light-line and light-solid): clean drawings, tubes as filled bars at 1:50, even spokes, mudguard bands, box with capping and raked front, stand under the box; 1:500 is the generalised outline.

## 7. Still imperfect

- `mass-frame` fails `volumes-structure-coverage` at 1:50, 1:100 and 1:200 (author request filed; same issue as city-bicycle).
- `box-wood` `areaM2` set to 3.61, the C16 limit; the honest value of the modelled box is 3.52 m² (listed in `area-review.md` for the reviewers).
- Not modelled: brake and gear cables, cable housings at the drums (the steering cables run beside the beam and up the gooseneck, their ends meet the drums only approximately), spoke nipples, tyre tread, valves, reflectors, display wiring, the crates' handle recesses as geometry (drawn as texture).
- The rotors, chainring, chain guard and sprocket are `texture` solids (2 to 4 mm sheet below the 7.5 mm feature); spider cut-outs are not modelled.
- Steering cables are housed push-pull cables (as on Douze-type long-johns); Bakfiets.nl itself uses a steering rod. A straight rod cannot pass the box and the gooseneck in this geometry without a bell crank, so the cable layout was chosen.
- Head angle 70 degrees, fork offset 35 mm and the 80 degree column are own-knowledge values (manufacturers do not publish long-john geometry tables); the box is 141 l against the 148 l of the Bakfiets.nl sheet.
- At 1:200 and 1:500 thin tubes are generalised to 30 mm and 75 mm bars (smallest printable feature), the 1:500 box is a closed block.
