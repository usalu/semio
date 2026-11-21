# Round 3 — vehicles/city-bus (2026-11-19)

State: finished, spliced. Catalog asset `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/city-bus/🔣️.json` validates (`r3-validate.ts vehicles/city-bus --catalog-only --volumes`: schema 0, validateAsset 0, checkVolumes 0) and `volumes-check.ts vehicles/city-bus --product` is `PASS` at all four scales.

This asset was authored in two sittings. The first author (cut off by a usage limit at 01:55) wrote the kit, the 1:50 body, all lists of all scales and the root script and had spliced an intermediate state; the check then failed at every scale. This sitting rebuilt from the modules, looked at every scale, repaired what was wrong (listed under "Second sitting") and spliced.

## Scripts (all handwritten statements of sections, no guessing generator)

| file | content |
|---|---|
| `r3_city_bus_kit.py` | the stated dimensions of the bus and the plain solid writers |
| `r3_city_bus_body50.py` | 1:50 body, assembly by assembly |
| `r3_city_bus_lists50.py` | 1:50 thermal and structure, 1:50 decisions |
| `r3_city_bus_coarse.py` | 1:100, 1:200, 1:500 lists and decisions, each authored for its feature size |
| `r3-city-bus-build.py` | writes the lists into `volumes/vehicles/city-bus.json` |
| `r3-city-bus-roots.py` | root fixes and the 2D drawings of every scale and view (`--write` replaces only the changed members of the catalog file) |
| `r3-city-bus-probe.ts`, `r3-city-bus-view2d.ts`, `r3-city-bus-suit.ts` | measuring aids (silhouette with groups left out, 2D view, print suitability violations) |

## 1. Reference

Subject: 12 m two-axle low-floor battery-electric city bus with two double doors, roof batteries and a roof heat pump (the class of Mercedes-Benz eCitaro, MAN Lion's City 12 E, Solaris Urbino 12 electric). Sources: manufacturer data as I know them (not fetched again in this sitting, WebSearch was not used), tyre and wheel standards (ETRTO size 275/70 R 22.5 on 22.5 x 8.25 rims, ten-stud 335 mm bolt circle), EU 2001/85/EC and UN R107 for door and floor dimensions (the asset's `provenance.references`), ZF AxTrax AVE portal axle for the drive axle layout.

| quantity | real class | model |
|---|---|---|
| length | 12.0 to 12.2 m | 12.00 m |
| body width (without mirrors) | 2.55 m (legal limit) | 2.55 m |
| width over mirrors | about 2.9 to 3.0 m | 2.95 m |
| height over roof equipment | 3.25 to 3.40 m | 3.30 m (roof skin 2.98 m + battery fairing 0.32 m) |
| wheelbase | 5.90 m (eCitaro, Urbino 12), 6.0 m (MAN) | 5.90 m (axles at x 3.45 and 9.35) |
| front / rear overhang | 2.70 to 2.80 m / 3.40 to 3.45 m | 2.65 m / 3.45 m |
| tyre 275/70 R 22.5 | diameter 0.957 m, section 0.276 m, rim 0.572 m | 0.96 m, 0.275 m, rim flange 0.604 m, bead seat 0.572 m |
| rear axle | twin tyres, portal axle with two hub motors | twins at z 0.755 and 1.075 (0.32 m spacing), dished against each other |
| track front | about 2.10 m | 2.10 m |
| floor height at the doors | 0.32 to 0.34 m, kneeling about 70 mm | 0.34 m |
| door clear width / height | 1.20 to 1.25 m / about 2.0 m | 1.25 m / 2.02 m (0.34 to 2.36 m), four leaves of 0.625 m |
| waist (lower window edge) | about 0.9 to 1.1 m | belt 1.10 m, glass from 1.12 to 2.50 m |
| side wall tumblehome above the belt | 30 to 40 mm | 35 mm between 1.10 and 2.62 m |
| roof cant radius | about 0.25 to 0.35 m | 0.30 m, crown 3 cm |
| windscreen rake | 5 to 10 degrees, bowed in plan | 7 degrees, one curved pane |
| wall build-up | tube frame 40 to 50 mm, 1.5 mm skin, lining | 41.5 mm (1.5 mm skin + 40 mm lining) |
| frame | welded square tube ring frames at the window posts, 1.3 to 1.5 m pitch | posts at 0.4, 1.75, 3.05, 4.35, 5.65, 6.95, 8.25, 9.55, 10.75, 11.4 m |
| roof battery | several modules under one fairing, about 0.3 m high | 6.3 x 2.08 x 0.32 m fairing, six module lids |
| roof heat pump | about 2.5 x 1.8 x 0.25 m | 2.5 x 1.8 x 0.25 m with fan and louvres |

## 2. Audit (before round 3)

| item | found | real |
|---|---|---|
| `dimensions.heightM` | 3.14 | 3.30 (roof 2.98 + 0.32 fairing; class 3.25 to 3.40) |
| `dimensions.depthM` | 2.55 (body only) | 2.95 over the mirrors, which the drawings and the model show |
| door bones | 0.55 m leaves at x 6.35 / 6.9 / 10.35 / 10.9, travel 0.45 / 0.40 m | 0.625 m leaves closing a 1.25 m opening (6.275 to 7.525 and 10.2 to 11.45), travel 0.60 / 0.55 m so the opening is clear |
| rear contact anchors | z = +-1.05 (front track) | +-0.915, the middle of the twin tyres |
| `hvac-vent` anchor | y 0.16 | 0.25, the top of the housing where the fan is |
| battery / hvac centre of mass | comY 0.08 | 0.15 / 0.12, the middle of the housings |
| glazing layers | pane-out, gap, pane-in (outermost first) | innermost first as every other region: pane-in, gap, pane-out |
| structure assumption text | door travel 0.45 m | 0.6 m |
| 1:50 body (103 solids) | box shell without tumblehome, 12-gon tyres, blank rear, tiny mirrors (qa-visual-2: A-/B) | see section 4 |
| 1:100 structure | roof structure a flat slab (model-findings) | tube frame |
| 1:500 | tyres, drums and e-axle buried in the block | wheels in arches |
| 2D drawings | did not agree with a real bus section (no cant radius in front view, wheel arches missing at coarse scales, plain rectangles for the roof units) | redrawn as the orthographic views of the body |

## 3. Root fixes written to the catalog asset

| member | old | new |
|---|---|---|
| `dimensions.heightM` | 3.14 | 3.3 |
| `dimensions.depthM` | 2.55 | 2.95 |
| bone `door-m-aft` | x 6.35, length 0.55, comX 0.275, maxM 0.45 | x 6.275, length 0.625, comX 0.3125, maxM 0.6 |
| bone `door-m-fwd` | length 0.55, comX 0.275, maxM 0.45 | length 0.625, comX 0.3125, maxM 0.6 |
| bone `door-f-aft` | x 10.35, length 0.55, comX 0.275, maxM 0.4 | x 10.2, length 0.625, comX 0.3125, maxM 0.55 |
| bone `door-f-fwd` | x 10.9, length 0.55, comX 0.275, maxM 0.4 | x 10.825, length 0.625, comX 0.3125, maxM 0.55 |
| bone `battery` mass comY | 0.08 | 0.15 |
| bone `hvac` mass comY | 0.08 | 0.12 |
| anchors `contact-r-l`, `contact-r-r` z | +-1.05 | +-0.915 |
| anchor `hvac-vent` y | 0.16 | 0.25 |
| pose `kneeling-at-stop` door dx | -0.45, 0.45, -0.4, 0.4 | -0.6, 0.6, -0.55, 0.55 |
| clip `depart` | one key with the old door travel | new travel |
| `thermal.regions[glazing].layers` order | pane-out, gap, pane-in | pane-in, gap, pane-out |
| `structure.assumptions` | "by up to 0.45 m" / "um bis zu 0,45 m" | 0.6 m |
| `representations` 50 / 100 / 200 / 500 | elevation 53 / 46 / 34 / 10, front 16 / 15 / 7 / 7, plan 17 / 13 / 10 / 10 shapes | 53 / 47 / 35 / 12, 21 / 20 / 14 / 12, 19 / 15 / 14 / 12 shapes, every existing shape keeps id, bone, role, z, tone, region; geometry redrawn (ruling E1) |

No `areaM2` was changed. `massKg` unchanged.

2D redraw: side outline with tail radius, header lip, raked windscreen, bumper steps and both arches; front section with tumblehome and cant radius; plan with the bowed nose and tight tail corners; bays, door openings, leaves, seat backs, lamps and mirrors at the positions of the solids. New shapes: front `leaf-f-fwd`, `axle-beam-f`, `e-axle-casting`, mirrors and their arms (front and plan), elevation `well-front` / `well-rear` at 1:100, 1:200 and 1:500 (the dark wheelhouse seen through the arch). Shapes that differ between scales because their solids differ are drawn per scale (`variant()` in the roots script): axle outlines, roof unit sections, tilted door panels of 1:200 and 1:500, roof unit ends, mirror arms as thin filled quads.

## 4. What is modelled

### 1:50 — body 198, thermal 166, structure 189 solids

- Body: each side a flat lower panel with the two arches (kerb side with the door notches) and a leaning upper frame with one opening per bay; arched roof skin; nose and tail as curved walls on the plan outline with floors, header with a 25 mm lip over the screen, tail cap; curved raked windscreen, 20 mm panes set into the openings with hopper transoms, rear window, hatch, lamp clusters, high brake lamp, plates, wipers, destination display, rub rails, arch trims, charging flap, vent grille, mirrors on arms; four kinked door leaves with two panes and seals each; floor with wheel-well cut-outs, four wheelhouses with spring seats, seats, grab rails, stanchions, driver's seat, partition, dashboard, steering wheel; tyres with real section and grooves, disc wheels with ten hand holes and nuts, hubs, brake discs, dropped front beam with stub axles, carriers and bellows, portal drive axle with motor housings, stub spindles, drop arms, carriers and four bellows; battery fairing with six lids, heat pump with fan and louvres.
- Thermal: skin and lining plates of walls, nose, tail, floor, roof (cut in cant strips and crown), header and tail cap; three layers per pane; door leaves; operating regions (battery, heat pump with the fan disc as outlet patch, drive axle, tyres in quadrants, brake discs).
- Structure: welded tube frame (longerons, sills, ten ring frames with posts, cross members and five-piece roof bows, waist and cant rails, door headers, bumper beams, screen beams, A-pillar tubes, corner posts), roof unit rails and mounts, leaf frames with carriages, axle beams, springs and king pins as joints, disc wheels and hubs, one mass per bone. `structure.supports` is empty in this asset, so there is no support solid.

### 1:100 — body 123, thermal 74, structure 134

60 mm walls, one flush pane per bay with transom, 30 mm windscreen, bumpers, lamps, destination display, mirrors on arms, solid kinked leaves with applied panes, wheels with tyre section, disc wheel, hub and brake disc, plain spring cylinders on carriers, plain fairings. Thermal one solid per part and region. Structure eight ring frames in 60 x 50 tube, leaf stiles, two rails under each roof unit.

### 1:200 — body 79, thermal 58, structure 88

80 mm walls, one glazing band per window run with 90 mm pillar strips and one transom strip, plain nose and tail, rear window, tilted leaf panels with one pane, tyre and dished disc per wheel (twins separate), axle beams with motor housings and spindles, four-section fairings. Structure five ring frames in 70 x 60 tube.

### 1:500 — body 40, thermal 34, structure 61

The box with its real cross-section in 100 mm walls, both arches, three glazing bands, windscreen, rear window, header with lip and tail cap, four 75 mm door panels, roof units with rounded section, four wheel discs (twins merged), two axle beams, mirrors as ears. Structure two longerons, two cant rails, three ring frames in 90 x 80 tube, rim cylinders, spring and hub pins.

Decisions: three bilingual sentences groups per scale (en, de), updated for everything changed in this sitting.

## 5. Second sitting: what was repaired after the cut-off

| finding | fix |
|---|---|
| check failed at all scales (elevation p95 94 / 188 / 469 mm at 1:100 / 1:200 / 1:500) | the coarse elevations had no wheelhouse behind the arch gap; `well-front` / `well-rear` added there |
| front p95 over the limit at 1:50, 1:100, 1:200 | front drawings of axles, drive axle, roof units, tilted doors and mirror arms redrawn per scale from the solids |
| thermal `e-axle` area 22 % under the declared 3.0 m2 at 1:50 | a real gap: nothing connected the motor housings to the twin wheel hubs. Stub spindles (r 0.10 m, z 0.66 to 0.915) added at 1:50, 1:100, 1:200 in body and thermal; deviation now 11 % |
| front air springs floated 0.2 m above the beam at 1:100 | spring carriers added at 1:100 |
| header and windscreen nearly coincident, jagged intersection line | header lowered to 2.66 m and set 25 mm proud as a lip, at every scale; destination display moved under it; side, front and plan drawings follow |
| 1:200 pillar and transom strips 18 mm proud of the wall | set 4 mm proud |
| 1:500 had no rear window | added (body, thermal) |
| 1:100 / 1:200 / 1:500 heat pump carried by a 1.2 m wide slab in `structure` | two rails as at 1:50 |
| monotonic: 1:200 structure and 1:500 thermal had more triangles than the finer scale | ellipsoid joints at 1:200 / 1:500 replaced by short pin cylinders; 1:500 roof units as two-section lofts instead of rounded boxes |
| 1:500 drawings were rejected by the print profile after the redraw (`e-axle-casting` 0.176 mm < 0.2 mm) | 1:500 drive axle beam 195 mm deep, drawn 0.105 to 0.30 m; all four scales resolve as `authored` without violations (`r3-city-bus-suit.ts`) |
| 1:500 door panels 50 mm under the feature limit once the drawing was true | panels 75 mm (the feature of the scale), carriage 75 mm |

## 6. Final check

`$BUN volumes-check.ts vehicles/city-bus --product`: `PASS (61.8 s)`.

| scale | elevation IoU / p95 | front IoU / p95 | plan IoU / p95 | thermal area |
|---|---|---|---|---|
| 1:50 | 0.998 / 15 mm (limit 15) | 0.995 / 30 mm (limit 30) | 0.993 / 11 mm | worst region e-axle 11 %, limit 15 % |
| 1:100 | 0.998 / 11 mm | 0.996 / 11 mm | 0.999 / 4 mm | worst tyres 23 %, limit 30 % |
| 1:200 | 0.998 / 8 mm | 0.995 / 15 mm | 0.998 / 8 mm | total 14 %, limit 30 % |
| 1:500 | 0.997 / 19 mm | 0.993 / 56 mm | 0.997 / 19 mm | total 11 %, limit 30 % |

`$BUN r3-validate.ts vehicles/city-bus --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.

## 7. What I saw in the final renders

Opened with the Read tool: `r3-sheet` 1:50 body (six views, then front, rear three-quarter turn 230, front three-quarter turn 70, from below turn 60 tilt -12), 1:50 thermal cut, 1:50 structure, 1:50 pose `kneeling-at-stop`, 1:100 body, thermal cut and structure, 1:200 body, 1:500 body and structure, `volumes-render` overlays of 1:50, 1:100, 1:200, `r3-2d` of 1:50, 1:200, 1:500 and 1:100 in the kneeling pose.

- Front: bowed nose, raked curved screen under a clean header lip, display, wipers, lamp clusters with indicators, bumper, grille slot, plate, mirrors (hanging arm on the kerb side, short arm on the driver's side), twin rear tyres and the dropped front beam under the floor.
- Right side: seven bays and two double doors with two panes per leaf, belt, rub rails, charging flap, arch trims, wheels with hand holes, nuts and hubs, roof units with ramps.
- Left side: eight bays, vent grille at the rear, driver's window.
- Back: rear window, hatch, lamp strips, high brake lamp, bumper, twins.
- Top: roof skin with cant radius, six battery lids, heat pump with fan and two louvre fields, doors standing 3 mm outside the wall.
- From below: flat floor with four wheel-well cut-outs, front beam, drive axle with motors, carriers and bellows.
- Pose: body kneels, leaves slide clear of the openings along the wall, interior (seats, stanchions) shows through the openings.
- Thermal cut: skin and lining plates, glazing layers, roof strips, tyres, axle, brake discs inside; nothing pokes out of the body.
- Structure: a recognisable bus skeleton at 1:50, 1:100; reduced but still a frame at 1:500.
- Coarse scales read as the same bus at a glance; 1:500 keeps arches, bands, doors, roof units, mirrors.

## 8. Still imperfect

- Tyres and rims look 12-sided in the `r3-sheet` panels. The solids are true revolves (the arches drawn with 28 segments next to them are smooth); the facets are the sheet's tessellation, not the model. Not requested as a tool change.
- 1:50 silhouette p95 sits exactly on the limit in elevation (15 mm) and front (30 mm). The remainder is real geometry that the drawing does not outline: rub rails, arch trims, vent grille and door seals stand 8 to 11 mm outside the body section.
- The four leaves slide parallel to the wall (prismatic bones as before). A real outward sliding-plug door first moves out about 50 mm; the forward leaf of the front door then travels along the curved nose corner. The plug motion is not rigged.
- 1:100 leaves out seats and interior; the 1:100 elevation does not draw them either.
- Glass is shown opaque by the sheet tool, so the interior was judged only through the open doors of the kneeling pose and from the solids' coordinates.
- Reference values are from memory of manufacturer data, not re-fetched.
- Front overhang 2.65 m is 5 to 15 cm shorter than the class (2.70 to 2.80 m) because the wheelbase (5.90 m) and rear overhang (3.45 m) were kept at their real values within the given 12.00 m length (`widthM`).

## Round 5 handcraft (2026-11-21)

Scripts: `r3_city_bus_coarse.py` (edited in place: door leaf frames at 1:200 and 1:500, thicker mirror arms and a narrower kerb-side mirror at 1:100 and 1:200), `r3-city-bus-build.py` (unchanged, rebuilds the source), `r5-vehicles-patch.py` with `r5-city-bus-patch.json` (the 2D root patch, shape by shape), `r5-vehicles-render.sh`, `r5-vehicles-ship.sh`. Renders: `🗑️generated/r5-city-bus/before/`.

### Reference (Solaris Urbino 12 electric, Solaris press specification UITP 2023, fetched 2026-11-21) and model

| quantity | reference | model 1:50 | deviation |
|---|---|---|---|
| length | 12.000 m | 12.000 m | 0 % |
| width (body) | 2.550 m | 2.550 m | 0 % |
| height without pantograph | 3.300 m | 3.300 m | 0 % |
| wheelbase | 5.900 m | 5.900 m | 0 % |
| front overhang | 2.700 m | 2.650 m | -1.9 % |
| rear overhang | 3.400 m | 3.450 m | +1.5 % |
| door arrangement | 2-2-2 (three double doors) | two double doors (2-2-0, the asset's class choice) | see open points |
| tyre 275/70 R 22.5 | 0.957 m | 0.960 m | +0.3 % |
| floor at the doors (UN R107, class) | 0.32 to 0.34 m | 0.34 m | in range |

### Grades (opened: body, structure and thermal cut at 1:50, 1:100, 1:200, 1:500; all seven poses and the three clip frames at 1:50; 2D at every scale)

| scale | body before / after | thermal before / after | structure before / after |
|---|---|---|---|
| 1:50 | A- / A- | A- / A- | A- / A- |
| 1:100 | A- / A- | A- / A- | A- / A- |
| 1:200 | A- / A- | B+ (grey palette, documented) / B+ | B+ (door leaves as solid panel references) / A- |
| 1:500 | B+ / B+ | B+ / B+ | B (four 75 mm door slabs in the frame) / A- |
| poses, clips | A- except kneeling-at-stop B+ (forward leaf of the front door slides past the nose corner) | | |

Catalog before: INVALID under the print rule (`mirror-arm-l` 0.168 mm at 1:150 for 1:100 and 1:200; `door-opening-m/f` 0.192 mm at 1:1000).

### Changes

- Structure 1:200 and 1:500: each of the four door leaves is now a frame of two stiles and two rails (40 mm / 75 mm sections) on its carriage, the real leaf frame, instead of the leaf panel referenced as one member slab. Decisions (en, de) updated.
- Mirrors 1:100 and 1:200 (3D and 2D front and plan): arms r 17/20 -> 22/25 mm (real mirror arm tubes 40 to 50 mm); the kerb-side driver's mirror housing 180 -> 140 mm wide at the same outer edge (overall width 2.95 m unchanged), so its arm shows 35 mm beyond the wall.
- 2D 1:500 plan: the two door reveals `door-opening-m/f` drawn 100 mm deep instead of 41.5 mm (a 0.08 mm sliver on paper).
- Tried and reverted: a deeper 1:500 drive axle (0.26 m) made the axle a sliver under the kneeling skirt and the product then refused 1:500 at its nominal scale; the round-3 axle stays. Tried and reverted: narrowing the 1:500 range to 1:900 (the door-reveal measure is dominated by the pen and barely moves with the denominator).

Check: `VALID vehicles/city-bus (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; `volumes-check.ts --product` PASS (671 s).

### Open points

- Front door, pose `kneeling-at-stop` and clip `arrive-kneel-doors` (B+): the forward leaf of the front door slides 0.55 m forward on the prismatic `door-f-fwd` bone and stands proud of the curved nose corner. A real front door of this class swings (inward or outward plug) about vertical columns at the jambs; that needs revolute door bones with a vertical axis, re-authored leaf solids, 2D shapes, poses and clips at every scale. Not done in this round.
- Door arrangement 2-2-0 against the reference 2-2-2: the asset was authored as a two-door 12 m bus (the class exists); a third door would be a new layout of the rear overhang, not a correction.
- Coarse thermal sheets show the coarse solids in one grey: the sheet tool's palette for solids without `layer` (documented in round 3), not the model.
- 1:500 body (B+): wheels are plain discs with the twins merged, roof units plain slabs; minimal at 75 mm.

The 1:200 side and front views of the electric axle casting use a 0.16 mm pen. The 1:350 endpoint and all affected feature and silhouette checks passed.
