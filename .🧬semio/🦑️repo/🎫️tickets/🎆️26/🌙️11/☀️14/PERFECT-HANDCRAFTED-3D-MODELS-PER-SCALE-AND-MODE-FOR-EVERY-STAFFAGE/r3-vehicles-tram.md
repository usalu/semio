# Round 3 — vehicles/tram (2026-11-20)

State: finished and spliced. Catalog asset `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/tram/🔣️.json`. `r3-validate.ts vehicles/tram --catalog-only --volumes`: schema 0, validateAsset 0, checkVolumes 2. Both remaining findings are the `doors` area. That value is a declared-area defect listed in `area-review.md`. I did not apply it, because the correction is beyond factor 1.5 (C16). `volumes-check.ts vehicles/tram --product`: every rule passes at every scale except `volumes-thermal-coverage` `doors` at 1:50 and 1:100 (see section 6).

Two sittings. The first author was cut off at 14:58 on 2026-11-19. They had written the kit and a 1:50 body, thermal and structure into the source. Nothing was spliced and there were no root fixes and no coarse scales. This sitting checked that 1:50 work against the reference and repaired it (section 4). It then wrote the coarse scales anew, fixed the roots, redrew all twelve drawings and spliced.

## Scripts (all state sections by hand, none guesses a form)

| file | content |
|---|---|
| `r3-tram-kit.py` | dimensions of the tram and plain solid writers (car section, nose superellipse, bogie openings, frames of the middle module and the leaning door leaves) |
| `r3-tram-s50.py` | 1:50 body, thermal, structure, decisions |
| `r3-tram-coarse.py` | 1:100, 1:200, 1:500 lists and decisions |
| `r3-tram-build.py` | writes the lists into `volumes/vehicles/tram.json` |
| `r3-tram-draw2d.py` | the 2D drawings of every scale and view as true orthographic views of that scale's body, written to `r3-tram-2d/s<N>.<view>.json` |
| `r3-tram-roots.py` | root fixes (glazing layer order, bone lengths, centre of mass, `areaM2`, drawings); candidate or `--write` |
| `r3-tram-info.py`, `r3-tram-digest.py`, `r3-tram-areas.ts`, `r3-tram-bounds.ts`, `r3-tram-sil.ts`, `r3-tram-cross.ts`, `r3-tram-cmp.py`, `r3-tram-measure.ts` | measuring aids: shape listing, check digest, raw areas, solid bounds, silhouette and cross-scale mismatch maps, outline comparison, product silhouette numbers with shapes left out |

## 1. Reference

Subject: a section of a 100 % low-floor articulated tram, about 2.40 m wide. Examples of the class are the Siemens Combino, Alstom Citadis 302 and Bombardier Flexity Classic/Outlook. The section is a cab end module on a powered bogie and a trailing bogie, the articulation with bellows, and a suspended middle module without a bogie.

Sources: manufacturer data as I know it (Siemens Combino and Alstom Citadis brochures, not fetched again in this sitting; WebSearch was not used), and the asset's `provenance.references`: EN 13803 (wheel and track), EN 50163 (contact wire), VDV 154 / BOStrab (floor height, door width, vehicle gauge) and EN 13452 (brakes).

| quantity | real class | model |
|---|---|---|
| car body width | 2.30 to 2.65 m (2.40 m common) | 2.40 m over the belt |
| roof height without equipment | 3.3 to 3.6 m | 3.40 m crown, 3.10 m cant rail |
| height over roof units | 3.6 to 3.8 m | 3.66 m (HVAC), 3.64 m (brake resistor) |
| contact wire, collector raised | 4.0 to 6.0 m (BOStrab), 5.0 m typical | collector at 5.00 m |
| floor height | 300 to 350 mm at the doors | 350 mm floor, 280 mm skirt, podests over the bogies at 0.74 m |
| tumblehome above the waist | 30 to 50 mm | 40 mm between belt 1.15 m and cant 3.10 m |
| wheel | 600 mm new, independent wheels or cranked axles in low-floor bogies | 600 mm tread, 644 mm flange, recessed web, stub axles (no axle under the floor) |
| bogie wheelbase | 1.80 to 1.85 m | 1.80 m |
| track (rail centres) | 1435 mm gauge, contact at about +-0.72 m | +-0.715 m contact anchors |
| traction | one longitudinal motor per side driving both wheels through bevel gearboxes (Combino type) | motor of 0.9 x 0.38 m per side, two gearboxes |
| brakes | wheel discs, calipers, magnetic track brakes | disc 0.4 m on each wheel, calipers, track brakes 0.92 m |
| double door | 1.30 m clear, about 2.0 to 2.1 m high, plug-sliding | two leaves of 0.65 m, 0.30 to 2.45 m, 40 mm sandwich, tilted with the tumblehome |
| window band | sill about 1.1 to 1.2 m, head about 2.3 to 2.5 m | 1.20 to 2.40 m |
| windscreen | raked 15 to 20 degrees, wrapped round the corners | raked 16.7 degrees (tan 0.3), superelliptic plan (n = 3) |
| bellows | about 0.6 to 1.0 m long, folded rubberised fabric round the gangway turntable | 0.90 m, V folds at 50 mm pitch, turntable and articulation bearing below |
| single-arm pantograph | lower and upper arm about 1.6 to 2.0 m, head about 1.6 m wide with two carbon strips and horns | arms 1.90 m, strips 1.05 m, horns to +-0.84 m, frame on four insulators |
| roof units | HVAC about 3.5 to 4 x 1.8 m; brake resistor about 2 x 1.7 m | 3.8 x 1.8 x 0.28 m; 2.0 x 1.7 x 0.24 m on feet |
| car body construction | welded aluminium or steel frame (sole bars, bolsters, posts, rails, roof bows, end rings), skin about 1.5 to 2 mm, lining | frame members 50 to 85 mm, skin 1.5 mm, lining 50 mm |

## 2. Audit (before round 3)

| item | found | real / required |
|---|---|---|
| rig `body-mid.lengthM` | 5.55 (tip 0.45 m short of the rear end at x = -6.0) | 6.0 from the articulation to the rear end face |
| rig `body-mid.mass.comX` | 2.8 | 3.2 (module centre -3.23 m, roof HVAC at -3.3 m) |
| rig `body-end.lengthM` | 9.0 (tip 0.42 m short of the coupler face) | 9.42 to the nose |
| other bones (bogies at 7.2 / 1.9, wheelsets +-0.9, wheel radius 0.3, pantograph 22 / 136 / -158 degrees, doors 0.65 m) | correct against the reference | unchanged |
| `dimensions` 15.45 x 2.4 x 5.0 m | correct | unchanged |
| `thermal.regions[glazing].layers` | pane-out, gap, pane-in: listed outermost first, so the validator counted the inner pane (74.9 m2 against 32.8) | innermost first: pane-in, gap, pane-out |
| `body-panels.areaM2` 79.82 | the hand calculation used a 2.2 m wall height and left out the bogie wells | 105.7 (honest 1:50 outer area, factor 1.32) |
| `doors.areaM2` 8.89 | double the leaves' outer area | about 4.4: listed for the reviewer, not applied |
| 2D 1:50 front | rectangle 2.4 x 3.4 m, no tumblehome, no cant curve | true section |
| 2D 1:50 elevation | cab with a step at the roof, no arches, windows and doors at wrong heights (1.25 to 2.75 m, doors 0.28 to 2.70 m), bogie beams 0.31 to 0.45 m (real 0.50 to 0.62 m), axle drawn straight across under the low floor | redrawn |
| 2D plan | nose as a chamfered box, middle module with chamfered corners | superelliptic nose, plain module |
| 1:50 source (first author) | windscreen and cab good; skirt arches had a 40 mm notch at their feet (kit `cutout` started the arc at y 0.32 instead of 0.28); cab sill a filled 60 mm plate across the cab; door skin both faces; structure header ring a filled plate; masses poking out of the members | fixed (section 4) |
| 1:100, 1:200, 1:500 (old) | qa-visual-2 grade C: windscreen a visor plate over a recessed face, pantograph a plate on a stick, 1:500 wheels buried in blocks, structure only sills, portals and a gantry with the pantograph floating | authored anew |

## 3. Root fixes written to the catalog asset

| member | old | new |
|---|---|---|
| `rig.bones[body-mid].lengthM` | 5.55 | 6.0 |
| `rig.bones[body-mid].mass.comX` | 2.8 | 3.2 |
| `rig.bones[body-end].lengthM` | 9.0 | 9.42 |
| `thermal.regions[glazing].layers` order | pane-out, gap, pane-in | pane-in, gap, pane-out |
| `thermal.regions[body-panels].areaM2` | 79.82 | 105.7 |
| `representations` 50 / 100 / 200 / 500 elevation | 52 / 38 / 27 / 19 shapes | 73 / 56 / 35 / 23 |
| front | 16 / 14 / 9 / 8 | 24 / 23 / 13 / 10 |
| plan | 20 / 17 / 14 / 12 | 21 / 18 / 15 / 12 |

Every existing shape keeps its id, bone, role, z, fill, tone, region and material. Only its geometry is redrawn as the orthographic view of that scale's body (E1).

New shapes:
- wheelhouses `well-r` and `well-f` behind the arches
- `bogie-legs-*` (uprights and track brakes) and `gearbox-f` / `axle-box-r`
- `articulation-arm` in elevation and front, and `bellows-folds-*` (texture)
- `destination-sign` and `wiper` in elevation
- `a-pillar-l/r` in front
- `drive-f`: the motors seen below the skirt
- pantograph: `pan-upper-arms` (the V of the upper arms) in front and plan, plus `pan-horns`, `pan-brace`, `pan-knee`, and `pan-lower-rod` / `pan-upper-rod`
- 1:500: `bogie-frame-*`, front `roof-boxes` and `pan-base`

`r3-validate.ts --file` was VALID on every candidate before it was written. The poses `at-stop-doors-open` and `depot-pantograph-down` were rendered (section 7).

## 4. What is modelled

### 1:50 — body 238, thermal 214, structure 201

Body:
- 51.5 mm side walls: a plumb lower panel with the bogie openings, and a leaning upper frame (40 mm tumblehome) with the real window openings and recessed 24 mm panes.
- Roof with rolled cant rails, floor with the bogie wells, podests, end walls with gangway portals, and a blanked rear portal on the middle module.
- Cab: superelliptic nose, bumper, raked windscreen loft, domed header, sill band, A and B pillars, headlamps, indicators, destination sign, coupler hatch, wiper.
- Folded bellows with turntable, articulation arm and bearing.
- Bogies part by part: 600 mm wheels with flange and recessed web on stub axles, brake discs and calipers, side beams on primary springs, secondary springs, uprights, two cross members, track brakes. The powered bogie has a longitudinal motor per side with pinions, couplings and two gearboxes; the trailing bogie has axle boxes and brake units.
- Plug-sliding leaves with glass, seals and buttons, tilted to follow the tumblehome.
- Pantograph: insulators, frame, bearings, shaft, drive, lower and upper arms with push rods, brace, knee shaft, head with apexes, carriers, strips and horns.
- HVAC with fans and louvres, brake resistor on feet with fan and louvres.

Thermal:
- Skin and lining plates of every wall, roof strip, floor, end wall and the cab.
- Three-layer glazing.
- Door leaves as sandwich skin with core; the cabin face is a section face (C25 c).
- Bellows as folded outer skins with flat backs.
- Operating regions; the two fans are outlet patches on the roof units.

Structure:
- Welded car-body frame: sole bars, bolsters, bolster posts, cross-bearers, lower and upper posts, door posts and lintels, waist and cant rails, roof bows, end rings with portal posts.
- Cab anticlimber, floor ring, waist ring, collision posts, A and B posts, header ring.
- Articulation links and pins, bogie frames with springs as joints, stub axles, bearings, wheels.
- Pantograph arms, brace and hinges; door leaf frames with carriages; masses.

Repairs to the first author's 1:50:
- Arch foot notch fixed in the kit, at every scale.
- Cab sill changed to a 70 mm ring band.
- Leaves tilted with the tumblehome. Before, they stood 36 mm proud of the wall at the top in the front view.
- Door and middle-rear section faces (C25).
- Header ring changed to a band.
- Mass marks kept inside their members.

### 1:100 — body 143, thermal 115, structure 100

Body:
- The same shell build at 15 mm. Window openings are merged where the drawing merges them; panes are flush 20 mm.
- Wells as one U-profile per side.
- Cab with nose, windscreen, header, sill, bumper, A pillars, headlamps and sign.
- Bellows with nine folds as four skins, with arm and bearing.
- Bogies: beams with secondary springs, legs, two cross members, track brakes, wheels on stub axles, discs, motors with gearboxes or brake units.
- Leaves with glass, tilted.
- Pantograph: insulators, frame, shaft, arms, knee shaft, brace, two strips with apex and horns.
- Roof units with fans; resistor on two feet.

Thermal: references by region. Cabin faces are section faces. Layers are merged into their plates (B2).

Structure:
- Stepped side sills, cant rails, door and end posts plus two window posts per side.
- Four roof bows and four cross-bearers per module, end rings.
- Cab ring and posts, articulation link and pin, bogie frames and pivots, stub axles, wheels, pantograph, door leaves, masses.

### 1:200 — body 104, thermal 95, structure 95

Body:
- Closed 60 mm shells with plumb walls; the 40 mm tumblehome is under 1.5 features and is dropped.
- Lower panels with arches and door gaps, roof cap with the cant curve, floor with wells, end walls.
- Panes 8 mm proud.
- Cab with nose, windscreen, header, bumper and lamps.
- Four-fold bellows.
- Bogies with beams, legs, cross member, track brakes, wheels with flange, stub axles, motors with gearboxes or brake units.
- Solid leaves with applied panes.
- Pantograph with one-piece head and horns; plain roof boxes.

Thermal: brake discs as 30 mm discs inside the wheels.

Structure: as 1:100 with three bows.

### 1:500 — body 55, thermal 65, structure 71

Body:
- Closed 100 mm shells with arches, roof cap with the cant curve, floor and wells.
- Window bands split at the doors; 75 mm door panels.
- Cab nose, windscreen and header.
- Rounded bellows block.
- Per bogie: a frame block per side above plain 600 mm wheel discs.
- Pantograph base, two slender arms and a 1.6 m collector bar; the two roof boxes.

Thermal: motors and discs as compact solids inside the bogie.

Structure: stepped sills, cant rails, door and end posts, two roof bows and one cross-bearer per module, end rings, cab ring and posts, articulation link and pin, bogie frame blocks with pivots, stub axles, wheels, pantograph base and arms, door panels, masses.

Decisions: three bilingual groups (en, de) per scale, updated for everything above.

## 5. Silhouette after the redraw (product numbers)

| scale | elevation IoU / p95 | front IoU / p95 | plan IoU / p95 |
|---|---|---|---|
| 1:50 | 0.998 / 9 mm (limit 15) | 0.980 / 19 mm (30) | 0.999 / 0 |
| 1:100 | 0.998 / 8 mm (30) | 0.981 / 19 mm (60) | 0.999 / 0 |
| 1:200 | 0.995 / 8 mm (60) | 0.980 / 23 mm (120) | 0.993 / 8 mm |
| 1:500 | 0.997 / 19 mm (150) | 0.994 / 19 mm (300) | 0.993 / 19 mm |

Cross-scale, bounds, feature, monotonic, copied, structure coverage and solids all pass at every scale.

## 6. Final check

`$BUN volumes-check.ts vehicles/tram --product`: `FAIL: 2 measurement(s) over 2 rule line(s) (91.7 s)`. Both are `volumes-thermal-coverage` `doors`: 4.36 m2 against 8.89 at 1:50 (limit 15 %), and 4.33 at 1:100 (limit 30 %).

Thermal areas at 1:50, measured against declared:

| region | measured (m2) | declared (m2) | deviation |
|---|---|---|---|
| body-panels | 105.9 | 105.7 | 0 % |
| roof | 38.7 | 36 | 7 % |
| glazing | 36.8 | 32.83 | 12 % |
| gangway | 18.2 | 16.7 | 9 % |
| traction-motors | 3.98 | 4.5 | 12 % |
| wheelsets | 6.86 | 7.8 | 12 % |
| brake-discs | 2.10 | 1.89 | 11 % |
| brake-resistor | 8.27 | 8.5 | 3 % |
| hvac-unit | 9.68 | 8.96 | 8 % |
| pantograph | 3.39 | 3.61 | 6 % |

At 1:100 every region except `doors` is within 30 % (worst pantograph, 17 %). The 1:200 and 1:500 totals are within 12 %.

The `doors` value is a declared-area defect, not a model defect. The leaves' outer face less their glass, plus edges and reveals, is 4.0 to 4.4 m2. The cabin face is a section face by C25 (c). The entry is in `area-review.md` (2026-11-20 r3 vehicles/tram).

## 7. What I saw in the final renders

Opened with the Read tool:
- `r3-sheet` 1:50 body (six views and the three-quarter view)
- 1:50 structure and thermal cut
- 1:100 and 1:500 body three-quarter, and 1:100, 1:200 and 1:500 body sheets
- 1:100 and 1:500 structure three-quarter, 1:100 thermal cut, 1:500 thermal cut sheet
- poses `at-stop-doors-open` (three-quarter, turn 300) and `depot-pantograph-down` (right side)
- `volumes-render` overlays at 1:50 and 1:500
- `r3-2d` of 1:50, 1:100 and 1:500

Direction by direction:
- **Front (1:50):** cab with wrap-round screen between A pillars, header with sign, headlamps and indicators, sill band, bumper, wiper. Below the skirt: track brakes, cross member, motors. Pantograph V with the head and horns.
- **Right side:** two double doors with tilted plug leaves, windows at 1.2 to 2.4 m, bogie openings with wheels, beams, springs, motor with couplings and gearboxes (front) and brake unit with axle boxes (rear). Folded bellows; raked nose with domed header.
- **Left side:** the window band without doors; the cab side pane behind the raked edge.
- **Back:** the rear end wall of the middle module with the blanked portal, and the bogie undersides.
- **Top:** roof units, pantograph frame, the plan curve of the nose.
- **Poses:** with the doors open the leaves slide clear along the wall and show the floor and interior. With the pantograph down it folds flat over its frame on the roof.
- **Thermal cut:** skin and lining plates, glazing layers, bellows skins and operating regions, nothing outside the body.
- **Structure:** a real welded frame at 1:50 and 1:100, still a frame at 1:500, with the pantograph and articulation carried by members.
- **Coarse scales:** they read as the same tram at a glance. 1:500 keeps the arches, wheels, bands, doors, roof units and pantograph.
- **2D:** clean outlines with the true front section (tumblehome, cant curve). The bogie openings show the dark wheelhouse; the pantograph V and horns appear in the front view.

## 8. Still imperfect

- **`doors` area:** fails until the area reviewer decides (section 6).
- **Thermal sheet colours:** `r3-sheet --list thermal` paints solids without a `layer` (all coarse thermal solids, by B2) in one neutral grey instead of the region colour. The geometry is right; this is the tool's palette, so I judged the coarse thermal lists from the region legend and the overlay renders.
- **Upper pantograph arms in 2D:** in the front and plan views they are drawn as a narrow extruded capsule (the `solid` member cannot show a V), plus the new `pan-upper-arms` V shape. In the lowered pose the V shape is carried by the bone but is exact only in the default pose.
- **Mirrors and camera arms:** none modelled, because the declared width of 2.40 m includes nothing outside the body. Many current trams use cameras.
- **Reference values:** from memory of manufacturer data, not fetched again.
- **Structure assumptions text:** unchanged. It does not mention the 6.0 m length of the middle module.
