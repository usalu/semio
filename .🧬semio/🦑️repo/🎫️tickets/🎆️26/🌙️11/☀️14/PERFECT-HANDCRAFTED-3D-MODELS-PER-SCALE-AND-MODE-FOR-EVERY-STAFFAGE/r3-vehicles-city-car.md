# Round 3 — vehicles/city-car

State: catalog asset written and spliced; `r3-validate.ts vehicles/city-car --volumes` = `VALID ... schema 0, validateAsset 0, checkVolumes 0`; `volumes-check.ts vehicles/city-car --product` = `PASS` at 1:50, 1:100, 1:200, 1:500.

An earlier round 3 author was cut off after the rig fix and a first 1:50 body. I kept his master-line kit, repaired and finished 1:50, authored 1:100, 1:200, 1:500 anew, redrew all twelve 2D views, corrected three areas and the texts, spliced.

Scripts (all in the ticket folder): `r3_city_car_kit.py` (master lines), `r3_city_car_s50.py`, `r3_city_car_s100.py`, `r3_city_car_s200.py`, `r3_city_car_s500.py` (lists, solid by solid), `r3-city-car-build.py` (writes `volumes/vehicles/city-car.json`), `r3_city_car_2d.py` (drawings), `r3-city-car-roots.py` (texts, areas, reference), `r3-city-car-draw.py` (candidate document or `--catalog`), `r3-city-car-rig.py` (rig fix of the first author, already applied), `r3-city-car-info.py`, `r3-city-car-shapes.py` (inspection).

## 1 Reference

| quantity | reference | asset (after) | source |
|---|---|---|---|
| length | 3.85 to 3.97 m | 3.90 m | VW Polo 6R 3970, Suzuki Swift AZG 3850, Honda Jazz GE 3900 (manufacturer data, from memory; no web lookup made) |
| body width | 1.68 to 1.70 m | 1.68 m (hull at the arches) | Polo 1682, Swift 1695, Jazz 1695 |
| width over mirrors | about 1.90 m | 1.90 m (`dimensions.depthM`) | Polo 1901 |
| height | 1.46 to 1.53 m | 1.50 m | Polo 1462, Swift 1510, Jazz 1525 |
| wheelbase | 2.43 to 2.50 m | 2.45 m | Polo 2470, Swift 2430, Jazz 2500 |
| front / rear overhang | about 0.80 to 0.83 / 0.62 to 0.68 m | 0.80 / 0.65 m | Polo about 824 / 676 |
| track | 1.46 to 1.48 m | 1.44 m (tyre centres at z = ±0.72) | Polo 1463 / 1456, Swift 1480 |
| tyre 185/60 R15 | diameter 603 mm, width 185 mm, rim 381 mm | radius 0.30, width 0.185, rim radius 0.19 to 0.208 | ETRTO 2023 (asset reference) |
| ground clearance | 0.14 to 0.17 m | 0.19 m at the sills, exhaust and floor 0.19 to 0.235 m | class value; see "imperfect" |
| belt line | about 0.95 to 1.05 m | 1.00 m | class value |
| windscreen rake | about 28 to 32 degrees from horizontal | 33.8 degrees | class value |
| tumblehome of the side glass | about 20 degrees | 20.6 degrees (0.375 m per m) | class value |
| roof width | about 1.15 to 1.2 m | 1.24 m | follows belt width and tumblehome |
| wheel arch | 60 to 70 mm over the tyre | radius 0.365 m around the 0.30 m tyre | class value |
| static front axle share | about 60 to 63 % (front engine, front drive) | 62.2 % | Gillespie 1992 (asset reference) |

The reference is now cited in the asset: `provenance.references[0]`, id `b-segment-hatchbacks`.

## 2 Audit (before this round)

- Rig: `axle-f.x` 1.25 and `axle-r.x` -1.20 gave overhangs of 0.70 m front and 0.75 m rear: reversed for a front-engined hatchback (real 0.80 / 0.65). Wheelbase 2.45 m was right. Bonnet and tailgate hinges, wheel radius, joints: right.
- `dimensions.depthM` 1.68 was the body width while the asset has door mirrors; the 2D front drew mirrors inside 1.68 m (a body of 1.48 m).
- 2D, every scale: arches and wheels at the old axle positions; arch as a flat cubic; front and plan were constructed boxes (no tumblehome, no sill, headrests drawn in plan under the roof, steering wheel drawn over the roof); 1:500 had closed arches and no mirrors.
- Solids (catalog before round 3; `model-findings.md` grade C): wheels as cylinders outside a flat side wall, no arches, no tumblehome; roof rails floating in the 1:200 and 1:500 structure; revolve profiles with implied closure on tyres and brakes (`kernel-traps.md`).
- First round 3 draft of 1:50 (found in the source): tube A and D pillars with ball ends above the roof, a roof pad not joined to the pillars, ragged windscreen base, hidden region `interior` referencing body solids (validator finding), no subframe in the body while the structure had one (0.014 m outside the silhouette). 1:100 to 1:500 were still the old lists.

## 3 Root fixes (catalog file)

| member | old | new |
|---|---|---|
| `rig.bones[axle-f].x` | 1.25 | 1.15 (first author) |
| `rig.bones[axle-r].x` | -1.20 | -1.30 (first author) |
| `rig.bones[chassis].mass.comX` | 0.38 | 0.28 (keeps the front share at 62.2 %; checked by hand) |
| `dimensions.depthM` | 1.68 | 1.90 (over the mirrors, Polo 1901) |
| `provenance.references` | 7 entries | + `b-segment-hatchbacks` (first) |
| `thermal.regions[body-panels].areaM2` | 13.0 | 15.4 |
| `thermal.regions[roof].areaM2` | 1.78 | 2.5 |
| `thermal.regions[tailgate].areaM2` | 0.75 | 0.9 |
| `thermal.regions[interior].hidden` | "only drawn at 1:50" | drawn at 1:50 and 1:100 |
| `representations[*].geometry` | 12 views | all redrawn (below) |
| `representations[*].decisions` | texts of the old drawings | `silhouette`, `retained`, `merged`, `omitted`, `thinElements`, `overlays` rewritten where they were no longer true |

Areas (C16, each within a factor 1.5, all three regions passive, so no flux density changes): measured on the 1:50 model 15.44 / 2.50 / 0.90 m2. Hand check: sides 2 x 3.9 x 0.75 = 5.9, front and rear 2.0, underside 3.9 x 1.6 = 6.2, arches and pillars about 1.3 = 15.4; roof 1.86 x 1.24 = 2.3 plus the edge band 0.2 = 2.5. These three corrections should be confirmed by the area reviewer; they are not in `area-review.md`.

Poses, clips, anchors: unchanged and still right (contacts sit on the axle bones, wheel roll does not depend on the axle position, `boot-floor` at x = -1.5 is behind the rear axle). Rendered: `parked`, `hatch-open` (3D and 2D), clip `drive-by` at 3.5 s (rendered, not opened).

2D, ruling E1: not dumped from `r3-project2d.ts`; every shape is written in `r3_city_car_2d.py` from the same master lines as the solids (arches as two cubic quarter circles, outlines of 10 to 45 points). Ids, roles, bones, z order and regions of the old drawings are kept where the part still exists. New ids: `resonator`, `axle-pipe`, `plate-front`, `subframe`, `lower-arm-l/r`, `wiper-driver/passenger`, `cowl`, `binnacle`, `side-glass-*` in plan, mirrors and wells at 1:500. Removed: `grille-line`, `bumper-rear-lamp`, headrests in plan (they are under the roof), `seat-rear-headrest` at 1:100, all interior shapes at 1:200. `tail-lamp` moved from the hatch bone to the chassis (the lamps are on the body). Bonnet and tailgate are their section profiles, swept (`solid.extruded`) in front and plan.

## 4 What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 137 | 82 | 96 |
| 1:100 | 67 | 47 | 62 |
| 1:200 | 32 | 31 | 38 |
| 1:500 | 14 | 20 | 19 |

- **1:50 body**: hull as one loft of about 50 superellipse sections with both arches cut as circles; underbody tub between the wheel houses; sills; waist rail; roof as a loft with crown and swept header; A, B, C pillars as plates in the tumblehome plane, D posts as lofts into the roof; windscreen bent in plan, four side panes, tailgate glass; cowl and two wipers; bonnet skin; tailgate (panel, lip, header, rails, glass, handle, plate); grille, intake, number plate, headlamps on the skin, tail lamps, mirrors with foot and glass, handles, door seams and fuel flap as texture; cabin deck, seat backs, headrests with stems, steering wheel with column and spokes, binnacle, interior mirror; exhaust run with resonator, silencer, tail pipe; per wheel tyre, rim, hub, five spokes, disc with hat, caliper; knuckles, struts, lower arms, drive shafts, subframe, twist beam, trailing arms, dampers.
- **1:50 thermal**: paint film and steel sheet as nested shells on every panel solid, glass, seat foam with hidden cushions, engine block, gearbox and manifold as hidden solids, exhaust run, tyres, discs and calipers, tailpipe mouth as outlet patch (D2).
- **1:50 structure**: unibody frame (sills, front and rear rails with kick-ups, floor rails, seven crossmembers, hinge, A, B, C, D pillars, roof rails, five roof bows and headers, strut towers, upper rails), bonnet and tailgate frames with hinges, struts, arms, subframe, twist beam, springs, wheels on bearings, masses. `structure.supports` is empty in the asset, so there are no support solids.
- **1:100**: same design for 15 mm; one-piece seats, one exhaust pipe, plain discs, no seams, handles, plates, wipers, calipers.
- **1:200**: arches as four chords, the glasshouse as one glass mass with plate pillars on it, two-part wheels; cabin, engine, exhaust pipe and discs as hidden thermal solids; structure reduced to platform, pillar pairs, roof frame.
- **1:500**: hull, glass mass, roof, rear quarter pillars, bonnet, tailgate, one-piece wheels, mirrors; structure: two longitudinals with kick-ups, three crossmembers, B-pillar hoop, side rails of bonnet and tailgate, axle beams, wheels.

`decisions`: three bilingual sentences per scale (1:50 from the first author, the others new).

## 5 Check (final)

- `r3-validate.ts vehicles/city-car --volumes`: `VALID vehicles/city-car (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only`: `VALID`.
- `volumes-check.ts vehicles/city-car --product`: `PASS (62.9 s)`.

| scale | IoU elevation / front / plan | p95 elevation / front / plan (mm) |
|---|---|---|
| 1:50 | 0.993 / 0.992 / 0.997 | 11 / 8 / 11 |
| 1:100 | 0.993 / 0.992 / 0.996 | 8 / 11 / 8 |
| 1:200 | 0.993 / 0.993 / 0.994 | 15 / 8 / 15 |
| 1:500 | 0.992 / 0.990 / 0.997 | 19 / 19 / 19 |

Thermal areas at 1:50 within 0.1 to 10 % per region; 1:100 within 25 %; totals 10.7 % (1:200) and 6.6 % (1:500). The maximum boundary distance in the 1:50 elevation is 0.156 m (reported, not limited); I did not locate it.

## 6 What I saw

Opened: 1:50 body sheet (six views), front and rear three-quarter, `hatch-open` sheet, structure sheet, thermal cut sheet, overlay sheet; 1:100 body and structure sheets; 1:200 body sheet and thermal cut sheet; 1:500 body sheet, structure sheet, overlay sheet; 2D at 1:50 (parked and hatch-open), 1:100, 1:500. Not opened: 2D at 1:200, thermal at 1:100 and 1:500, structure at 1:200, the clip frame, the final 1:50 sheet after the last small changes (subframe, fuel flap).

- 1:50 side: reads as a five-door hatchback at first sight; round arches with a dark gap over the tyres, sill, flat belt, raked screen, roof crown, sloped tailgate. Front: tumblehome, mirrors on stalks, headlamps on the corners, grille and intake. Top: swept screen, cowl with wipers, glass sloping out under the roof, pillars meeting the roof without gaps. Rear three-quarter: tailgate with glass in its frame; the lip and rails stand slightly proud with hard edges.
- `hatch-open`: the tailgate swings up on its hinge, the seats show through the opening; 2D follows in all three views (2.01 m high).
- 1:50 structure: a real unibody cage. Thermal cut: shells, seats, engine, exhaust, brakes inside.
- 1:100, 1:200, 1:500: each is recognisably the same car, lighter in detail; the 1:200 and 1:500 arches are visibly polygonal up close.

## 7 Still imperfect

- Not grade A in the body at 1:50. The front end is one smooth loft: no modelled bumper, no wheel-arch lips, no door frames around the glass, headlamps and tail lamps are simple lens bodies. The tailgate edges are hard. A car designer would accept the proportions, not the surface detail.
- The hull is a closed solid up to the belt: the cabin exists only above the belt (seats pass through a dark deck), the boot is not hollow when the tailgate is open, the engine bay is not hollow when the bonnet is open.
- Ground clearance 0.19 m is 20 to 50 mm above the class; lowering it means lowering sills, floor and exhaust together and was not done.
- Track 1.44 m is 20 to 40 mm narrow; windscreen rake 33.8 degrees is 2 to 4 degrees steep. Both left (the track is tied to the contact anchors at z = ±0.72).
- The roof is 1.24 m wide, a little wide for the class.
- Side doors stay panel lines (no door bones), as before.
- `sectionFaces` on the undersides of roof and bonnet skins and the inner faces of pillars and glass follow the first author's "outward faces only" reading; under C25/C31 a reviewer may want the cabin closed instead.
- Reference figures are from memory, not looked up.
- Earlier renders of the first author under `🗑️generated/r3-city-car/` and my candidate document there were deleted; sheets remain under `🗑️generated/r3-sheet`, `r3-2d`, `volumes-tools/render`.

## Round 5 handcraft (2026-11-21)

Scripts: `r5_city_car_kit.py` (body-in-white writers on the round 3 master lines: nose, tail panel, side walls, stepped floor, wheel houses, arch lips, bumpers, wrap-around headlamps, mirror housings), `r5_city_car_s50.py`, `r5_city_car_s100.py`, `r5_city_car_s200.py`, `r5_city_car_s500.py`, `r5-city-car-build.py` (source), `r5_city_car_2d.py` + `r5-city-car-draw.py` (drawings), `r5-city-car-roots.py` (areas). Output under `🗑️generated/r5-city-car/`.

### Reference (spec sheets; from memory, no web lookup in this round)

| quantity | reference | model 1:50 | deviation |
|---|---|---|---|
| length | VW Polo 6R 3970, Swift AZG 3850, Jazz GE 3900 mm | 3.90 m body, 3.916 m over the bumper covers | in class |
| body width | 1682 to 1695 mm | 1.68 m | 0 to -0.9 % |
| width over mirrors | Polo 1901 mm | 1.90 m (mirror tips at z = +-0.95) | 0 % |
| height | 1462 to 1525 mm | 1.50 m | in class |
| wheelbase | 2430 to 2500 mm | 2.45 m | in class |
| overhang front / rear | Polo 824 / 676 mm | 0.80 + 0.008 / 0.65 + 0.008 m | -2.9 % / -2.7 % |
| track | Polo 1463, Swift 1480 mm | 1.44 m | -1.6 % (Polo) |
| tyre 185/60 R15 | 603 mm, 185 mm wide | 0.60 m, 0.185 m | 0 % |
| front floor (heel point) | about 0.30 m | 0.33 m top of floor | +3 cm (floor 85 mm thick incl. carpet) |
| front seat cushion top | H-point about 0.52 m | cushion top 0.585 m | in range |
| boot floor / loading sill | about 0.58 / 0.72 m | 0.58 m / tail panel top 0.825 m behind the hatch lip | |
| bumper shut lines | front about 0.50 m, rear about 0.60 m | 0.50 m / 0.60 m (also the 2D lines) | 0 % |
| headlamp height | lower edge 0.67, upper 0.76 m, under the bonnet shut line | 0.67 to 0.76 m, x 1.64 to 1.91 m | |

### Grades (opened sheets: 1:50 body, hatch-open, bonnet-open, braking-nose-down, driving, lane-start, lane-cruise-entry, lane-exit, crawl-in, clip drive-by t 2 s, clip park-and-open-hatch t 6 s, front three-quarter close-up, structure, thermal cut, overlays; 1:100 body, structure, thermal cut; 1:200 body, structure, thermal cut; 1:500 body, structure, thermal cut; 2D 1:50, 1:100, 1:200, 1:500)

| scale | list | before | after |
|---|---|---|---|
| 1:50 | body | B (closed hull to the belt, nose a smooth loft without bumpers, oval lamp blobs low on the corners, ball mirrors, no arch lips, boot solid in hatch-open) | A- (hollow body in white, bumper covers with shut lines, wrap-around lamps, mirror housings with glass, arch lips, dust shields, seats with cushions and bases, boot open in hatch-open) |
| 1:50 | thermal | C (cut shows a solid red block up to the belt with seats standing on it) | A- (cabin, floor, wheel houses, seats, dash, engine, exhaust as real spaces and solids) |
| 1:50 | structure | A- (unibody tube cage) | A- (unchanged) |
| 1:100 | body / thermal / structure | B / C / A- | A- / A- / A- |
| 1:200 | body / thermal / structure | B / B- (floating cabin slab) / B+ | B+ / B+ / B+ |
| 1:500 | body / thermal / structure | B / B / B | B+ (mirror housings) / B / B |
| poses, clips | all | B (solid boot) | A- (all nine poses and two clip frames closed, no tearing) |

### Changes

- 1:50 and 1:100 lower body rebuilt as a body in white: `nose` (master hull ahead of the firewall at x 0.78), `tail-panel` (behind x -1.79), `side-wall-l/r` (50 / 60 mm walls whose outer face is the master hull side, from sill or arch line to the belt), `floor` stepped 0.33 / 0.45 / 0.58 m that rests on the rear arch line and narrows to the wheel-house plates only where the arch rises through it, `wheelhouse-l/r` bands and `wheelhouse-plate-*` inner plates (front ones close the front arch tunnel), `belt-rail-l/r/rear`, `dashboard`; dropped `hull`, `underbody`, the full-width `waist` deck and `cabin-deck`.
- Details: `bumper-front` / `bumper-rear` covers (8 mm proud at 1:50, 15 mm at 1:100) with shut lines at 0.50 / 0.60 m, `intake`, `fog-lamp-*`, `diffuser-rear`, `reflector-rear-*`; `headlamp-*` as lens lofts laid on the hull from the wing round the corner (x 1.64 to 1.905); `mirror-*` as lofted housings with `mirror-glass-*` on the rear face; `arch-lip-*` beads on every arch; `dust-shield-*` behind every wheel (the dark disc seen through the spokes; it also restores the elevation silhouette); body seat cushions and bases.
- 1:200: same open lower shell for 30 mm features (60 mm walls, 30 mm floor and plates), mirror housings, lamp lofts; the hidden cabin slab of round 3 that floated at mid-height in the hollow body replaced by hidden cushions and dashboard. 1:500: mirror housings.
- Thermal: interior is a hidden region, so its solids are own copies, not references (validator rule). Faces that look into cabin and boot carry `sectionFaces` as in round 3: the plate pillars and the hinged tailgate leave seams wider than the C31 flood cell (1.9 mm), so without them the flood reaches the cabin and every region read double (measured body-panels 29.4 against 15.4, roof 4.74 against 2.5).
- 2D: headlamps redrawn in elevation, front and plan at 1:50, 1:100, 1:200 as the true views of the new lamp lofts (`r5_city_car_2d.py`); nothing else in the drawings changed. Bumper lines at 0.50 / 0.60 m already matched.
- Root areas (C16, factor within 1.5, passive regions): `body-panels` 15.4 -> 20.2 m2 (the hollow body adds floor underside, wheel-house liners, bumper covers, arch lips and mirror housings: 1:50 measures 20.17), `interior` 5 -> 7.2 m2 (seat cushions, bases, backs, headrests, dashboard and steering wheel: 1:50 measures 7.23, hand value 2 front seats 3.0 + bench 2.4 + dash 1.4 + wheel and headrests 0.4 = 7.2).

### Check

`volumes-check.ts vehicles/city-car --product`: PASS (609.7 s), every rule line ok at all four scales (silhouette IoU 1:50 0.994 / 0.986 / 0.987, p95 11 / 15 / 23 mm; thermal 1:50 body-panels +0.2 %, every region within 15 %); `r3-validate.ts vehicles/city-car --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0); spliced with `volumes-insert.py`.

### Remaining

- The nose stays a closed loft under the bonnet (engine bay not hollow in `bonnet-open`; the engine is a hidden thermal region and may not be a body solid). Door shut lines are texture capsules, doors have no bones.
- Side walls are flat-faced per station (exponent 8) where the round 3 hull rounded into sill and belt (exponent 12): a faint wave shows along the lower side between the arches.
- The cabin is not airtight for the C31 flood, so inward faces are marked as section faces; sealing pillars, glass and tailgate seams would let the marks go.
- 1:200 and 1:500 stay a step below 1:50 in detail (polygonal arches by design, glass mass at 1:200 and 1:500); structure lists unchanged from round 3 (tube idealisation).

The 1:100 exhaust outlet uses a 0.20 mm paper outline. The complete drawing passed at 1:55, 1:75, 1:100, 1:125 and 1:150.

## November 21 road wheels and body framing

Five swept cast spokes attach each aluminium rim well to its hub. The steel A, B, C and D pillars and connecting roof rails complete the coarse cabin cage. The thermal view distinguishes the tyre carcass, wheel metal and brake. The measured metal surface totals 2.1191 m² across the four wheels. The full model passed schema and paper endpoint checks, all 52 scale rules and a twelve-scene independent GLB readback of 8,378,128 bytes. The outer, oblique and inner wheel detail views were inspected in the 1:500 body.

Published source `fc967fdf7bcef9c2e3cbe245296a7fae3b77a2d881299bc10e62de0786eed8b9`.
