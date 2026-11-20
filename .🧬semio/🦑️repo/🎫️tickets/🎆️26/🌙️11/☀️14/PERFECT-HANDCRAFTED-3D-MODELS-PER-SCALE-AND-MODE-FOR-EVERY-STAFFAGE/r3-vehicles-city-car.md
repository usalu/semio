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
