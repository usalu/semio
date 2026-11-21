# Round 3 — vehicles/city-bicycle (Dutch city bicycle, "omafiets")

Status: DONE on 2026-11-19 (catalog asset written, volumes spliced, validated; see "State and how to resume" at the end).

Scripts (all in the ticket folder `T`):

- `r3-city-bicycle-kit.py` — rest frames of the rig, key points of the bicycle, writers for solids, thermal entries and 2D shapes.
- `r3-city-bicycle-parts.py` — key points and hand-stated sections, profiles and paths shared by the scales.
- `r3-city-bicycle-s50.py`, `-s100.py`, `-s200.py` — one file per authored scale: every part as body solid, thermal entry, structural part and drawn shape, with the bilingual decisions.
- `r3-city-bicycle-build.py` — writes `volumes/vehicles/city-bicycle.json` and `🗑️generated/r3-city-bicycle/drawings.json`.
- `r3-city-bicycle-roots.py` — root fixes outside `volumes` (rig, anchor, poses, texts, drawings, decisions); candidate file first, `--apply` writes the catalog.
- `r3-city-bicycle-park.ts` — solves the root angle of the parked poses (front tyre patch and stand feet level).
- `r3-city-bicycle-suit.ts` — print-suitability violations per scale (the product refuses to draw an unsuitable scale; `validateAsset` does not report it).
- `r3-city-bicycle-view2d.ts` — one product 2D view rasterised large; `r3-city-bicycle-digest.py` — short form of a check output.

## 1. Reference (real subject)

Subject: a classic Dutch step-through city bicycle ("omafiets", e.g. Gazelle Classic / Batavus Old Dutch class), 28 inch wheels, closed chain case, hub gear, roller (drum) hub brakes, hub dynamo lighting, rear carrier, two-leg centre stand. No rider.

| quantity | real value | source | asset before | asset now |
|---|---|---|---|---|
| tyre 37-622: outer diameter | 622 + 2 x 37 = 696 mm unloaded, about 690 mm loaded (r 0.345 m) | ETRTO (asset reference `etrto-2023`) | r 0.345 | r 0.345 (kept) |
| rim 622 x 19C/25 (alloy box rim) | bead seat 622 mm, flange top 635 mm, rim depth about 20 mm, width 25 mm | ETRTO, own knowledge | rim ring r 0.283 to 0.309, 25 wide | r 0.297 to 0.3175, 25 wide |
| tyre section | 37 mm wide, 37 mm high | ETRTO | 40 x 36 | 37 x 37 |
| wheelbase | 1.12 to 1.18 m (omafiets 57 cm frame: about 1.14 to 1.15 m) | manufacturer geometry tables (Gazelle, Batavus), own knowledge | rig 1.140 (note said 1.12) | 1.140, note corrected |
| head angle | 66 to 70 degrees | own knowledge, EN 14764 class | 70 | 70 (kept) |
| fork offset (rake) | 45 to 60 mm, curved blades | own knowledge | 44.5 mm, straight blades | 44.5 mm, blades curved forward over the last 140 mm |
| trail | 60 to 80 mm | computed: (r cos 70 - offset) / sin 70 | 78 mm | 78 mm |
| fork length axle to crown race | 390 to 410 mm for 28 inch | own knowledge | no crown race | 392 mm |
| head tube | 200 to 260 mm long, 34 mm outside diameter (1 inch steerer) | own knowledge | 132 mm x 45 mm | 220 mm x 34 mm with headset cups |
| bottom bracket height | 280 to 300 mm | own knowledge | 300 | 300 (kept) |
| crank length | 170 mm | standard | 170 | 170 (kept) |
| chainstay length | 460 to 480 mm | own knowledge | 482 | 482 (kept) |
| seat tube angle | 66 to 68 degrees | own knowledge | 69.7 | 68 |
| seat tube length (frame size) | 53 to 57 cm, 28.6 mm diameter | own knowledge | 533 mm | 550 mm |
| down tube / loop tube | 31.8 mm / 28.6 mm; classic omafiets: straight down tube plus a second tube that runs parallel about 100 mm above it and bends back into the seat tube | own knowledge | one S-shaped 38 mm tube | 31.8 mm straight down tube, 28.6 mm loop tube |
| stays | chainstay 22 to 16 mm, seat stay 16 to 13 mm; rear hub over-locknut 126 to 135 mm | own knowledge | 20 / 16 mm, +-55 mm | 22-16 / 16-13 mm, +-64 mm |
| saddle | 260 to 270 mm long, 210 to 220 mm wide, two coil springs, top 0.95 to 1.02 m | saddle makers (Lepper, Selle Royal) | 300 x 170, no springs | 265 x 214, two springs, top 1.00 m |
| handlebar | 580 to 640 mm wide, swept back, grips 1.05 to 1.10 m high, 22.2 mm tube, grips 32 mm x 120 mm | own knowledge | 620 wide, grips straight out | 620 wide, grips swept back 250 mm, top 1.074 m |
| overall length | 1.85 to 1.95 m | manufacturer data | 1.87 | 1.87 (kept) |
| overall height | 1.05 to 1.15 m (handlebar / bell) | manufacturer data | 1.09 | 1.09 (kept) |
| overall width | 0.60 to 0.64 m (handlebar) | manufacturer data | 0.62 | 0.62 (kept) |
| mass | 18 to 21 kg | manufacturer data | 19 | 19 (kept) |
| mudguards | 45 to 50 mm wide steel, about 20 mm above the tyre | own knowledge | 64 wide, 8 mm thick slabs, 5 mm above the tyre | 48 wide, real sheet, 21 mm above the tyre, with stays |
| rear carrier | deck 0.78 to 0.80 m high, about 0.40 x 0.15 m, 10 mm round steel, two legs per side to the axle | own knowledge | floating tray 0.54 x 0.18 m without legs | tube carrier 0.42 x 0.15 m with four legs and two front struts |
| chain case | closed case around the chain loop, 38-tooth ring (r 77 mm) to 18-tooth sprocket (r 36 mm), about 30 mm thick | own knowledge, asset note (36:18) | hexagon | two housings with tangent runs |
| stand | two-leg centre stand (e.g. Ursus Jumbo): bracket behind the bottom bracket under the chainstays, legs about 230 to 280 mm, fold back and up under the chain case, feet about 180 mm apart; an unladen bicycle rests on the stand and the front wheel, the rear wheel is lifted | own knowledge | "rear stand" pivot in the middle of the chainstay, folds forward, lifts the rear wheel | centre stand, see root fixes |
| hubs | front hub dynamo with roller brake (shell about 90 mm), rear 3-speed hub (shell 70 to 75 mm) with roller brake | Shimano Nexus data, own knowledge | two plain 100 to 110 mm discs | dynamo and gear hub shells with brake units and reaction arms |
| spokes | 36 per wheel, 2 mm, crossed three times | standard | 36 | 36 |

## 2. Audit against the reference (state found on 2026-11-19)

Rig:

- `stand` bone: origin (-0.42, 0.3295) in the middle of the chainstay, rest angle -2 degrees (folded forwards towards the bottom bracket), two legs. No real stand looks like this: a rear-mount stand sits at the dropout, has one leg and does not lift a wheel; the stand that lifts the rear wheel and stands upright (as the structural assumptions say) is the two-leg centre stand directly behind the bottom bracket, which folds backwards. Root defect.
- The parked poses had root angle -1.2 degrees: the rear wheel was lifted 24 mm, the description says about 4 cm.
- `frame` centre of mass x = -0.05 m: the frame bone carries saddle, carrier, rear mudguard, chain case and stays, all behind the bottom bracket; a hand sum of the parts gives about -0.22 m (see root fixes).
- Provenance note said wheelbase 1.12 m; the rig has 1.140 m (rear axle -0.58, front axle +0.560).
- Steer axis, wheel centres, bottom bracket, crank and pedal bones agree with the reference.

Dimensions: 1.87 x 1.09 x 0.62 m and 19 kg agree with the reference; unchanged.

2D drawings:

- 1:50 could not be drawn by the product at all: `resolveRepresentation` returned `unsuitable` (gaps of 0.03 and 0.075 mm between `crank-axle`/`stand-pivot` and `chainstay-l`/`stand-l`), so every 1:50 view failed with "no suitable representation" (seen in `r3-2d.ts`; `validateAsset` does not report this).
- All three scales carried dumped projections of 3D solids as shapes named after solids, on the wrong bone and with wrong regions (for example `crank-arm-n`, `stem-clamp`, `stand-foot` as `frame` shapes of region `mudguards`), next to the hand-drawn shapes: 20 to 36 such shapes per view.
- The decisions of 1:50 named view bones (`fork-leg`, `bars`, `arm-n`, `arm-f`) that do not exist in the rig.
- The drawn bicycle had the defects of the model below (S-shaped single frame tube, floating carrier tray, floating head lamp, tail lamp below the axle behind the mudguard).

Solids (1:50; the coarser scales repeated it): 

- frame: one 38 mm S-curved "main tube" instead of down tube and loop tube; head tube 132 mm long and 45 mm thick; no dropouts, no bridges.
- carrier: a flat tray floating at the top of the seat stays, no legs.
- head lamp: an ellipsoid floating 60 mm in front of the head tube with nothing holding it; tail lamp a box behind the rear mudguard 0.2 m above the ground.
- mudguards: 8 mm thick, 64 mm wide slabs 5 mm above the tyre, no stays.
- fork: straight blades, a capsule as crown.
- handlebar: straight bars angled back, grips in line; no brake levers, no bell.
- saddle: 300 x 170 mm pad on a bare post, no springs, no rails.
- hubs: plain discs; chain case a hexagon; pedals plain boxes; stand as in the rig.

## 3. Root fixes (catalog asset, outside `volumes`)

| member | old | new | reason |
|---|---|---|---|
| `rig.bones[stand]` title | Rear stand / Hinterbauständer | Two-leg centre stand / Zweibeinständer | the real stand that lifts the rear wheel |
| `rig.bones[stand]` x, y | -0.42, 0.3295 | -0.215, 0.205 | pivot of the bracket under the chainstays, 115 mm behind the bottom bracket, below the chain case |
| `rig.bones[stand]` angleDeg (folded) | -2 (forwards) | 169.6 (backwards, parallel under the chain case) | real folding direction |
| `rig.bones[stand]` lengthM | 0.32 | 0.232 | pivot to foot |
| `rig.bones[stand]` joint | revolute -100..0 | revolute 0..110 | swings down and forwards |
| `rig.bones[stand]` mass comX | 0.17 | 0.11 | middle of the shorter leg |
| `anchors[stand-foot]` | x 0.32, footprint 0.06 x 0.03, "Stand foot" | x 0.232, footprint 0.03 x 0.2, "Stand feet" | two feet 180 mm apart |
| poses `parked-on-stand`, `parked-stopped`: stand angleDeg | -90 | 106.4 (legs 6 degrees ahead of vertical, over centre) | real deployed position |
| same poses: root angleDeg | -1.2 (rear wheel lifted 24 mm) | -2.02 (front tyre patch and stand feet level, rear wheel lifted 39 mm; solved with `r3-city-bicycle-park.ts`) | the description says about 4 cm |
| `rig.bones[frame]` mass comX, comY | -0.05, 0.62 | -0.251, 0.593 | hand sum of the twelve parts of the frame bone (8.5 kg, list in `frame_com()` of `r3-city-bicycle-parts.py`). With it the parked bicycle with the 4 kg bag still rests on the front wheel (moment about the feet 2.28 kg m against 1.63 kg m) |
| summary, provenance notes, pose titles and descriptions | "rear stand", "Hinterbauständer", wheelbase 1.12 m | "two-leg centre stand", "Zweibeinständer", wheelbase 1.14 m | follow the rig |
| `thermal.regions[mudguards].areaM2` | 0.351 | 0.27 | hand value of the real object: rear guard 1.28 m and front guard 0.98 m of 51 mm developed sheet, both faces, 0.229, plus six wire stays 0.025; model 0.266 (C16: within factor 1.5) |
| `thermal.regions[lamps].areaM2` | 0.036 | 0.028 | head lamp shell 0.018 (78 mm drum, 60 mm deep) plus tail lamp box 30 x 50 x 45 mm 0.010; model 0.0283 |
| `thermal.regions[drivetrain].areaM2` | 0.2 | 0.3 | chain case two faces 0.177 and rim 0.043, two crank arms with bosses 0.03, two block pedals 0.045, axle 0.005; model 0.303. This is exactly the factor 1.5 that C16 allows; `heatW` unchanged |
| `representations[50|100|200].geometry`, `.decisions`, `.pens` | hand shapes mixed with dumped solids, 1:50 undrawable | the true views of the bodies, see section 5 | ruling E1 |

Unchanged on purpose: `dimensions`, all other bones, the steer axis, wheel radius, clips (keys, contacts, tracks), thermal layers and heat, `structure.supports`, the symbol, the unavailable 1:500 decision.

## 4. Models per scale

Every scale has its own file; coarser scales are authored anew (own sections, merged masses), not filtered.

| scale | body | thermal | structure | what is modelled |
|---|---|---|---|---|
| 1:50 | 111 | 197 | 77 | all assemblies: tube frame with lug, post, headset cups, dropouts, bridges and stand bracket; tube carrier with spine, cross bars, four legs, two struts; sprung saddle on rails with clamp; wheels with tyre, box rim, 36 crossed spokes, gear hub / dynamo hub with roller brake units, axle and nuts; sheet mudguards (0.72 mm) with wire stays; chain case; crank axle, bosses, arms, block pedals with plates and spindles; fork crown, curved oval blades, fork ends; quill stem with head and clamp; swept-back handlebar, grips with flanges, brake levers, bell; head lamp with lens and bracket, tail lamp with lens; two-leg stand with pivot, bent legs and rubber feet. Thermal: outer layer on the body surface, steel wall as second solid for the four painted regions. Structure: hollow sections with real walls, steerer, bearings, 36 spokes, masses, support |
| 1:100 | 53 | 53 | 58 | merged: cups into head tube, lug into seat tube, springs into a yoke, hub with brake unit, axle with nuts, one block per pedal, lamp with bracket, straight stand legs; carrier as one hoop on four legs; twelve spokes; no dropouts, bridges, rails, stays, levers, lenses |
| 1:200 | 36 | 38 | 47 | wheel ring (tyre and rim), hub cylinder, six spokes; seat tube with post, down tube, loop tube, head tube, four stays; carrier hoop on two legs; saddle pad; chain case; bottom bracket cylinder, crank bars, pedal blocks; folded-sheet mudguards; crown, blades, one stem bar, handlebar that thickens into the grip; two lamps; two stand legs |
| 1:500 | unavailable | | | the representation is unavailable (below printable feature), unchanged |

Final check (`volumes-check.ts vehicles/city-bicycle --product`, 124 s): `FAIL: 2 measurement(s) over 2 rule line(s)`. Everything is green except `volumes-structure-coverage` `outsideBodyM:elevation` at 1:50 (0.0345 against 0.0075) and 1:100 (0.0333 against 0.015): the mass marker of the frame at its true centre of mass lies in the air between seat tube and rear mudguard (proved by removing it: 0.0046). Filed in `author-requests.md` ("r3 vehicles/city-bicycle"); the marker was not moved.

| rule | 1:50 | 1:100 | 1:200 |
|---|---|---|---|
| silhouette IoU elevation / front / plan | 0.962 / 0.966 / 0.966 | 0.968 / 0.984 / 0.987 | 0.972 / 0.956 / 0.981 |
| bounds deviation length / height / width (mm) | 2.9 / 1.7 / 6.3 | 0.3 / 2.2 / 13.6 | 3.3 / 19.5 / 5.1 |
| thermal area deviation, largest region | 7.0 % (hub-brakes) | 25.6 % (rims-spokes, twelve spokes) | total 0.7 % |
| cross-scale IoU to the finer scale | | 0.989 / 0.998 / 0.992 | 0.992 / 1.0 / 1.0 |

`r3-validate.ts vehicles/city-bicycle --catalog-only`: `VALID ... schema 0, validateAsset 0`. `r3-city-bicycle-suit.ts`: 1:50, 1:100 and 1:200 `authored` (drawable), smallest feature 0.214 / 0.213 / 0.200 mm.

## 5. Drawings (ruling E1)

All three views of all three scales are written by the same statements as the solids, so they are the orthographic views of that scale's body by construction: straight tubes are `member` shapes, plates, bent tubes and round parts about the lateral axis are profiles with an `extruded` solid hint, only the saddle (and the carrier hoop at 1:100 and 1:200) has a hand outline per view. The paint order of each view is the real depth order. Shape ids are the part names; the old ids of dumped solids are gone. Shapes on the steer bone and the front wheel are solids or members, so the steered poses are drawn right (no `lateral-ambiguous`).

Things I had to learn and respect: the number of `member` shapes per bone must not exceed the structural member solids of that bone (so only real bars are members and the handlebar is discretised alike in drawing and structure); filled shapes need 0.2 mm visible width including the pen (thin hidden parts get a wider pen or no fill); a ring with a hole projects wrongly in front and plan (the tyre is two half rings there, so a steered wheel stays open); coarse-scale spokes are `texture` strokes with their own pen, else they count in the silhouette.

## 6. What I saw in the renders

- 1:50 body, six-view sheet and full-size right, top, three-quarter from front-right and from rear-left: reads at once as a Dutch omafiets. Right side: loop tube parallel above the down tube and bending into the seat tube, long head tube with cups, curved fork, chain case with round housings, folded stand below the case, carrier on V legs, mudguards with stays, tail lamp on the rear guard, head lamp on its bracket above the front guard. Top: swept-back bar with splayed grips, levers and bell on the left, saddle pear shape, carrier grid, block pedals, cranks on both sides. Front and back: narrow, symmetric, saddle springs, stand feet 180 mm apart. Tyres and rims are faceted by the renderer's own tessellation.
- 1:50 structure: hollow frame, steerer in the head tube, spokes, blue members only where steel carries load; magenta masses at hubs, steer and frame (the frame one in the air, as stated). 1:50 thermal (also cut): nine regions in their colours, wall and paint coincide, nothing pokes out.
- 1:100 and 1:200 body, thermal and structure sheets: same proportions, fewer and simpler parts; 1:200 is still a bicycle at a glance (wheels with six spokes, frame, saddle pad, bar, lamps).
- Pose `parked-on-stand` (3D and 2D): stand down under the bottom bracket, rear wheel off the ground, front wheel and handlebar turned 15 degrees, front view shows the turned wheel as an open ring. Clip `brake-and-park` at 3.4 s: stand half way down. Clip `ride-loop` at 0.3 s: cranks turned, pedals level, in 3D and in the drawing.
- 2D 1:50 elevation, front and plan large (`r3-city-bicycle-view2d.ts`), 1:100 and 1:200 in the six-panel sheets and the elevation large: clean line drawings, tubes as double lines at 1:50 and as single strokes from 1:100, even spokes, correct overlaps (chain case over the right chainstay, mudguard over the tyre).

## 7. Still imperfect

- The frame mass marker fails `volumes-structure-coverage` at 1:50 and 1:100 (above; author request filed).
- Not modelled: brake and gear cables, roller-brake reaction arms, chain and chainring inside the closed case, coat guards on the rear wheel (a flat 2 mm plate cannot pass `volumes-feature`), valve, reflectors, carrier spring clip, tyre tread.
- The handlebar halves are drawn as five straight members along the bend; at 1:50 the joints show as short ticks across the 0.44 mm bar in line views.
- In front and plan the tyre is a prism (square shoulders), and its two half rings meet in a seam at axle height.
- The right chainstay passes through the chain case solid, as it does through the cut-out of a real case; the case is modelled closed.
- Head angle 70 degrees and tyre radius 0.345 m were kept from the rig (classic omafiets: 66 to 68 degrees, 0.348 m unloaded).
- `drivetrain` area sits exactly at the C16 limit (factor 1.5); an area reviewer may want to confirm 0.3 m².

## State and how to resume

Done. Catalog file `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/city-bicycle/🔣️.json` carries the root fixes, the drawings and the spliced volumes; source `volumes/vehicles/city-bicycle.json` equals the catalog volumes. To rebuild: `python3 r3-city-bicycle-build.py && python3 r3-city-bicycle-roots.py --park -2.02` (candidate under `🗑️generated/r3-city-bicycle/`), `$BUN r3-validate.ts --file <candidate>`, `$BUN r3-city-bicycle-suit.ts --file <candidate>`, then `python3 r3-city-bicycle-roots.py --park -2.02 --apply` and `python3 volumes-insert.py volumes/vehicles/city-bicycle.json <catalog file>`. Generated output of this asset was deleted at the end.

The rack hoop and legs, fender, hub ends, stand pivot, tail light and pedal outlines now use scale-specific print widths. The full asset document passed.
