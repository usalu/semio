# Round 3 — vehicles/delivery-van

State: catalog asset written and spliced. `r3-validate.ts vehicles/delivery-van --volumes` = `VALID ... schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only` = `VALID`; `volumes-check.ts vehicles/delivery-van --product`: every rule line `ok` at 1:50, 1:100, 1:200, 1:500 (table in section 5).

No round 3 work existed for this asset when I started (no scripts, no report; the source equalled the catalog and validated). Everything below is from this run.

Scripts (ticket folder): `r3_delivery_van_kit.py` (master lines and solid writers), `r3_delivery_van_s50.py`, `r3_delivery_van_s100.py`, `r3_delivery_van_s200.py`, `r3_delivery_van_s500.py` (lists, solid by solid), `r3-delivery-van-build.py` (writes `volumes/vehicles/delivery-van.json`), `r3_delivery_van_rig.py`, `r3_delivery_van_roots.py`, `r3_delivery_van_drawings.py` (root stages), `r3-delivery-van-roots.py` (candidate document, or `--catalog`).

## 1 Reference

Figures are manufacturer data from memory; no web lookup was made.

| quantity | reference | asset (after) | source |
|---|---|---|---|
| length | 5.14 to 5.40 m | 5.34 m | Ford Transit Custom L2 5340, VW T6 long 5304, Mercedes Vito long 5140, Renault Trafic L2 5399 |
| body width | 1.90 to 1.99 m | 1.97 m | T6 1904, Vito 1928, Trafic 1956, Transit Custom 1986 |
| width over mirrors | 2.24 to 2.30 m | 2.27 m (`dimensions.depthM`) | Vito 2244, Transit Custom 2272, T6 2297 |
| height | 1.91 to 2.00 m | 1.99 m | T6 1990, Transit Custom H1 1972 to 2000 |
| wheelbase | 3.20 to 3.50 m | 3.30 m | Transit Custom L2 3300 |
| front / rear overhang | 0.90 to 1.01 / 0.97 to 1.05 m | 0.97 / 1.07 m | Transit Custom 1011 / 1029, T6 908 / 996, Vito 895 / 1045 |
| track | 1.63 to 1.74 m | 1.68 m (tyre centres z = ±0.84) | T6 1628, Vito 1666 / 1646, Transit Custom 1736 / 1720 |
| tyre 235/65 R16 | diameter 712 mm, width 235 mm, rim 406 mm | radius 0.356, width 0.235, rim radius 0.203 to 0.221 | ETRTO 2023 (asset reference) |
| wheel arch | 50 to 70 mm over the tyre | radius 0.42 m around the 0.356 m tyre | class value |
| tyre inside the body side | 15 to 30 mm | 27 mm (tyre face 0.9575, body 0.985) | class value |
| belt line (lower edge of the side glass) | 1.15 to 1.20 m | 1.17 m | class value |
| windscreen rake | 40 to 45 degrees from horizontal | 44.5 degrees | class value |
| tumblehome above the belt | 60 to 90 mm to the roof line | 80 mm (6.3 degrees) | class value |
| cargo floor height | 0.53 to 0.59 m | 0.56 m | Transit Custom loading height |
| side door opening | about 1.03 m wide | 1.05 m (slider x -0.80 to 0.25) | Transit Custom |
| rear leaves | about 0.8 to 0.86 m each, 1.3 to 1.4 m high | 0.86 m, 1.35 m | class value |
| static front axle share, empty | 55 to 60 % (front engine, front drive) | 57.4 % | type-approval axle loads of the class; Gillespie 1992 (asset reference) for the method |

The reference is cited in the asset: `provenance.references[0]`, id `medium-panel-vans`.

## 2 Audit (before this round)

- Width: the body was 1.82 m wide and `dimensions.depthM` 1.99 m was called the width over the mirrors. Real vans of this length are 1.90 to 1.99 m in the body and 2.24 to 2.30 m over the mirrors. With the track of 1.68 m the tyre faces stood at z = 0.9575, 47 mm outside the 0.91 m body side: the reason the wheels hung outside the body (`qa-visual-2.md`, grade C).
- Rig: wheelbase 3.30 m, wheel radius 0.356 m, axle positions (overhangs 0.97 / 1.07 m) are inside the class and stay. The rear door hinges stood at x = -2.67 (the bumper face, not the door plane) and z = ±0.88 (30 mm from the old body side). `chassis.mass.comX` 0.15 gave a static front axle share of 50.5 % for the empty van; the class has 55 to 60 %.
- Anchor `tailpipe` at (x -2.55, y 0.42) lay inside the volume of a rear bumper.
- 2D, every scale: body 1.82 m wide, front and plan constructed from boxes (no tumblehome, tyres drawn in front of the bumper, windscreen in the side view as a triangle that no real projection gives, roof as a rectangle), the 1:500 body without arches.
- Solids (catalog before round 3): one lumpy loft for the lower body with arch cheeks, a second loft for everything above the belt (the cargo hold and the cab were solid), doors as plates on a closed body (an open sliding door or rear leaf showed painted body behind it), cab with a windscreen and a tiny side window, no interior; tyres as revolves with implied closure (`kernel-traps.md`); hidden region `exhaust-system` referenced a body solid at no scale but had one straight pipe.

## 3 Root fixes (catalog file)

| member | old | new |
|---|---|---|
| `dimensions.depthM` | 1.99 | 2.27 (over the mirrors; body 1.97) |
| `rig.bones[rear-door-r].x`, `.z` | -2.67, -0.88 | -2.61, -0.86 |
| `rig.bones[rear-door-l].x`, `.z` | -2.67, 0.88 | -2.61, 0.86 |
| `rig.bones[chassis].mass.comX` | 0.15 | 0.48 (front share 57.4 %; total mass unchanged, checked by hand: 1366 x 2.08 + 60 x 1.3 - 42 x 1.01 + 276 x 3.3 = 3787 kg m over 3.3 m = 1148 kg of 2000 kg) |
| `anchors[tailpipe].x`, `.y` | -2.55, 0.42 | -2.63, 0.30 (the pipe mouth below the rear bumper) |
| `provenance.references` | 8 entries | + `medium-panel-vans` (first); `provenance.notes` names the widths |
| `thermal.regions[body-panels].areaM2` | 22.0 | 27.0 |
| `thermal.regions[roof].areaM2` | 6.41 | 7.4 |
| `thermal.regions[cab-glazing].areaM2` | 3.0 | 3.3 |
| `poses[rear-doors-open].description` | "0.9 m behind the rear panel" | "0.86 m" |
| `symbols[van-symbol]` | 1.82 m body, mirrors to 1.99 m | 1.97 m body, mirrors to 2.27 m, tyres 0.235 m wide at z = ±0.84, windscreen as the real band; note text |
| `representations[*].geometry` | 12 views | all redrawn (below) |
| `representations[*].decisions` | texts of the old drawings | `silhouette`, `retained`, `merged`, `omitted`, `thinElements` rewritten |

Areas (C16, each within a factor 1.5 of the old value; all three regions are passive, so no flux density changes). Measured on the 1:50 model: 28.8 / 7.98 / 3.43 m2. Hand values: body panels: both sides 2 x 5.34 x 1.6 = 17.1 less slider 1.5, glass 1.2, arch openings 1.1 = 13.3; front 1.8; rear posts 0.5; underbody 5.34 x 1.97 x 0.85 = 8.9; four wheel wells 2.4; fender tops 0.5; together 27.4, set 27.0. Roof: 3.76 m x 1.93 m girth = 7.26 plus the rounded front and rear edges = 7.4. Glazing: windscreen 1.76 x 0.90 = 1.58, two door panes with quarter lights 2 x 0.62 = 1.24, edges and the strips behind the frames 0.4 = 3.2 to 3.3. The body grew from 1.82 to 1.97 m, which is the physical reason for all three. These corrections are for the area reviewer to confirm; they are not in `area-review.md`.

The product measure (`🧊️volumes/🧮️measures`) has no reachability flood (C31): the inner faces of the closed cab and cargo hold counted as outer surface (body panels 47 m2, roof 14.6 m2). I marked the inward faces of the panels, the roof underside, the floor top and the inner faces of doors and glass with `sectionFaces` on the thermal solids, as vehicles/city-car does. When C31 is implemented these marks can go.

Poses, clips, anchors: unchanged except the values above; contacts sit on the axle bones. Rendered and opened: `parked`, `rear-doors-open` (3D and 2D), `side-door-open` (3D), clip `stop-and-unload` at 8 s (3D).

2D, ruling E1: not dumped from `r3-project2d.ts`; every shape is written in `r3_delivery_van_drawings.py` from the master lines of the solids (arches as two cubic quarter circles, outlines of 4 to 45 points). Kept ids: `body-shell`, `tyre-*`, `rim-*`, `disc-*`, `hub-*`, `slider-panel`, `rear-leaf-r/l` (with their `solid` spans moved to ±0.86 / ±0.815), `windscreen`, `door-window`, `tail-pipe`, `door-front`, `door-rear`, `headlamp*`, `tail-lamp*`, `mirror*`, `rub-strip`, `handle-slider`, `roof`, `roof-cap`, `bonnet`, `bonnet-top`, `bumper`, `grille`, `exhaust-tip`, `glazing`, `wheel-f/r`, `tyres-f/r`. New: `undertray`, `wheelhouse`, `bumper-front`, `bumper-rear`, `sill`, `belt-line`, `spare-wheel`, `mirror-arm*`, `quarter-bar`, `seat-back`, `headrest`, `steering-wheel`, `door-handle`, `slider-track` (side), `rub-strip-slider`, `slider-belt`, `hole-*`, `side-glass-l/r`, `cowl`, `wiper-*`, `intake`, `plate-front`, `grille-bar-*`, `slider-panel` in the front view. Removed: front-view `handle-slider`, `slider-track-low/high` (constructed boxes), plan `slider-track`. The tyres in the front view now lie behind the body, so only what shows below the bumper is seen.

## 4 What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 157 | 99 | 109 |
| 1:100 | 88 | 49 | 71 |
| 1:200 | 44 | 38 | 54 |
| 1:500 | 33 | 32 | 30 |

Construction, all scales: a hollow panel van. Side panels, doors and posts are slabs (side-view outlines pushed through the wall thickness, the rear arches cut into the outline); above the belt they are plates in the leaning side plane; the front end is one loft with the front arches as full-width tunnels over an undertray; the roof is one loft with crown and rounded edges; floor, wheel houses and bulkhead close the hold. The sliding door and both rear leaves fill real apertures and move on their bones, so the open poses show the cargo hold.

- **1:50 body**: side panels left and right, B-post, lintel, front doors as their own slabs (their joints are real seams), cant rails, A-pillars, quarter-light bars, door glass, sill covers, door steps, corner posts with tail lamps and hinges, roof, front end, front bumper, bonnet skin, grille with three bars, intake, number plates, wrapped headlamps, mirrors with arm and glass, handles, rub strips, slider track, fuel flap, swept windscreen, cowl, two wipers; dashboard, binnacle, steering wheel with column and spokes, driver seat and double bench (base, cushion, back, headrests on stems), interior mirror; sliding door (lower slab, leaning upper panel, handle, strip, roller arm), rear leaves with recess, handle and plate; chassis rails, five crossmembers, fuel tank, spare wheel, tail pipe; per wheel tyre, rim, wheel disc with eight real holes, hub, five nuts, brake disc (vented in front), hat, caliper; knuckles, lower arms, drive shafts, subframe, rear beam axle, leaf springs, dampers.
- **1:50 thermal**: paint film and steel sheet as nested shells on every panel solid, glass, engine block, gearbox and manifold as hidden solids in the front end, exhaust run (down pipe, particulate filter, pipe, silencer, pipe over the axle, tail pipe) as hidden solids, the pipe mouth as outlet patch (D2), tyres, brake discs, hats and calipers.
- **1:50 structure**: unibody frame (rails, crossmembers, sills, front rails with kick-up, hinge, A, B, C, D pillars, cant and waist rails, roof bows, headers, cowl and bulkhead rails, strut towers, upper rails, wheelhouse hoops, bumper beams), frames of the slider and of both leaves with rollers and hinges as joints, subframe, arms, struts with top mounts, beam axle, springs with eyes, wheels on bearings, masses. `structure.supports` is empty, so there are no support solids.
- **1:100**: the same construction for 15 mm: arches of 16 chords, one-piece corner posts, sill and step as one L section, seats as base and back, plain steering ring, wheel as tyre, one dished wheel, hub and disc; no handles, strips, wipers, plates, nuts, calipers.
- **1:200**: front doors merged into the side panels, bumper merged into the front end, arches of 8 chords, glass up to the roof, no interior; brake discs and the exhaust as hidden thermal solids; frame reduced to rails, three crossmembers, sills, four pillar pairs, cant rails, bows.
- **1:500**: 80 mm panels (feature 75 mm), arches of 6 chords, corner posts merged into the side panels, one-piece wheels on two axle beams, rear bumper, mirrors; frame of two rails, three crossmembers, three pillar pairs, cant rails, three bows, door stiles.

`decisions`: three bilingual sentences per scale.

## 5 Check (final)

- `r3-validate.ts vehicles/delivery-van --volumes`: `VALID vehicles/delivery-van (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`.
- `volumes-check.ts vehicles/delivery-van --product`: all rule lines `ok` (availability, silhouette, views-disagree, bounds, feature, monotonic, cross-scale, thermal-coverage, structure-coverage, bones, slender, copied).

| scale | IoU elevation / front / plan | p95 elevation / front / plan (mm) |
|---|---|---|
| 1:50 | 0.995 / 0.993 / 0.996 | 13 / 15 / 11 |
| 1:100 | 0.995 / 0.994 / 0.996 | 15 / 15 / 15 |
| 1:200 | 0.995 / 0.991 / 0.998 | 8 / 15 / 8 |
| 1:500 | 0.994 / 0.998 / 0.999 | 19 / 0 / 0 |

Thermal areas at 1:50 within 1 to 13 % per region (slider door 13 %, bonnet 11 %), at 1:100 within 26 % (brakes), totals 3.2 % (1:200) and 5.1 % (1:500). The 1:50 elevation p95 of 13 mm is close to its limit of 15 mm.

## 6 What I saw

Opened: 1:50 body sheet (six views, twice), right side alone, three-quarter front right, three-quarter front left, rear three-quarter with `rear-doors-open`, front right with `side-door-open`, rear right of clip `stop-and-unload` at 8 s, structure sheet, thermal cut sheet, overlay sheet of `volumes-render.ts`; 1:100 body sheet and thermal cut sheet; 1:200 body sheet; 1:500 body sheet (before the windscreen fix) and structure sheet; 2D at 1:50 (candidate, then `rear-doors-open` from the catalog) and 1:200. Not opened: 2D at 1:100 and 1:500 and of `side-door-open`, structure at 1:100 and 1:200, thermal at 1:200 and 1:500, the 1:500 body after the windscreen and arch change, the single-list renders at four directions each that the brief asks for (I used the six-view sheets and five three-quarter views instead).

- 1:50 side: reads as a medium panel van at first sight: round arches with the wheel house behind the tyre, sill, belt seam, sliding door with handle, strip and track, cab door with glass and quarter-light bar, raked windscreen in front of the A-pillar, short sloping bonnet, blunt nose, dark bumpers. Front: lean above the belt, crowned roof, mirrors on arms, grille with bars, wrapped headlamps, plate. Top: swept windscreen with cowl and wipers, leaning walls beside the roof. Rear: two leaves with recess, handle, plate, tail lamps on the posts, hinges, bumper.
- Open poses: the leaves swing out on the corner hinges and show floor, wheel house and ceiling; the slider runs back outside the quarter panel and shows the hold, the step and the far wheel house.
- Structure: a recognisable unibody cage on rails with the door frames in their openings. Thermal cut: shells, engine and gearbox in the nose, the exhaust run under the floor over the rear axle.
- 1:100 and 1:200: the same van, lighter; 1:500: boxier, arches visibly polygonal up close, still a van whose doors open.

## 7 Still imperfect

- Not grade A in the body at 1:50. The front end is one smooth loft: the bumper is a 7 mm skin over it, there are no arch lips, the bonnet shut line is the edge of a 14 mm skin, headlamps are lens strips. The mirror housings are plain ellipsoids. Side panels are flat slabs with square lower edges: no sill tuck-under, no swage lines, no roof ribs.
- The sliding door stands 8 mm proud of the body when closed (15 mm at 1:100 and 1:200): the rig has one prismatic axis, so the door cannot move outwards before it slides; flush, it would disappear inside the quarter panel when open. A second axis or a guided path in the rig would be the real fix.
- The front arches are full-width tunnels in the front-end loft, closed by a dark undertray, not real inner wings with strut towers; the front doors are slabs without bones; the cab floor is the cargo floor level.
- The upper track of the slider and its lower track are not modelled; the centre track is a texture strip.
- `sectionFaces` on inward faces stands in for the missing C31 flood (section 3). The wheel-house shells are marked `+v` only, which removes about a quarter of their inner (hold-side) face and leaves the rest counted.
- The exhaust run passes 30 mm under crossmember 1 at the tail and through the plane of the rear bumper beam of the structure list; both are hidden thermal solids.
- Roof, glazing and body-panel areas were corrected from the model with hand checks and need the area reviewer. `slider-door` (2.05 declared, 1.78 measured, hand value 1.75) and `brakes` (0.93 declared for "drum brakes", the model has four discs with calipers at 0.85) were left.
- Reference figures are from memory, not looked up; the front axle share has no table citation.
- `r3-sheet.ts` shows a lighter wedge on the flat side panels around the rear arch (area-weighted vertex normals of the tool, not geometry) and draws layerless thermal solids grey; both are in `tool-requests.md`.
- The three coarser lists were written after the 1:50 list from the same kit; each has its own sections, chord counts and merged parts, but they share the panel layout.
