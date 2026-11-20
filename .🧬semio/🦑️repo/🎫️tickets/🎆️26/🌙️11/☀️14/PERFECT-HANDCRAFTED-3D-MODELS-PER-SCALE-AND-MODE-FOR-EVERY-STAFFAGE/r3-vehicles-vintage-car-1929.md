# Round 3 — vehicles/vintage-car-1929 (2026-11-19)

Resumed from an earlier author who was cut off: the scripts `r3-vintage-car-1929-*.py`, the root fixes of rig and dimensions and the 1:50 drawing were in the catalog, the volumes source was not spliced, decisions were placeholders, the drawings of 1:100 to 1:500 were written but never applied (5 validation errors), 15 check measurements failed. Everything below is the state after this session; what I kept from the earlier work I looked at and judged first.

Final state: `r3-validate.ts vehicles/vintage-car-1929 --volumes` = `VALID ... schema 0, validateAsset 0, checkVolumes 0`; `volumes-check.ts vehicles/vintage-car-1929 --product` = `PASS (41.9 s)`; print suitability (own probe `r3-vintage-car-1929-suit.ts`) `authored 0` violations at all four scales; catalog `volumes` equal to the source (spliced).

## 1. Reference

| quantity | real subject | source | model |
|---|---|---|---|
| wheelbase | 3213 mm (Voisin C11), 3220 mm (C14) | `provenance.references[voisin-c11-c14-data]` (added by the earlier author from conceptcarz.com / supercars.net profiles; I did not open the pages again in this session) | 3.20 m (axles at x = +1.70 and -1.50) |
| overall length | 4450 mm (C14, 1931) | same | 4.45 m over the bumpers |
| overall width | 1630 mm body data, class value 1.7 m over the wings | same, `voisin-c7-c11-class-data` | 1.70 m over the wings, body 1.24 m |
| overall height | 1.70 to 1.75 m | same | 1.70 m |
| track | 1.40 m front, 1.35 m rear (C14), 1.42 m (C11) | same | 1.34 m (tyre centres at z = +-0.67); 4 to 6 cm narrow, kept because the 1.70 m width over 0.29 m wings fixes it |
| tyre | 5.00-19 balloon: rim 483 mm + 2 x 127 mm = 0.737 m | tyre size arithmetic | radius 0.37 m, section 0.13 m, rim seat radius 0.245 m |
| radiator position | over the front axle on every car of the period | general knowledge of 1920s chassis layout | shell at x = 1.68 to 1.80, axle at 1.70 |
| front overhang / rear overhang | short front (frame horns and bumper only), long tail behind the axle | general | 0.525 m / 0.725 m to the bumpers |
| frame | ladder frame of two channel rails, upswept over the rear axle, 5 to 7 cross-members, top about 0.5 m above ground | general construction | rails 45 mm wide, 50 to 110 mm deep, top at 0.52 m, kick-up to 0.63 m, six cross-members |
| suspension | semi-elliptic leaf springs on both rigid axles, dropped I-beam front axle, banjo rear axle | general | modelled so |
| body construction | coachbuilt on an ash frame; Voisin saloons of the family have flat fabric or aluminium panels with sharp arrises, an upright six-light cabin, a visor and a fabric roof | general knowledge of the type | flat panels, sharp corners kept on purpose, tumblehome 40 mm, windscreen raked 5 degrees, rear wall 12 degrees |
| belt, bonnet, radiator, roof heights | belt and bonnet line about 1.15 to 1.2 m, radiator top just above the bonnet | proportion of period saloons | belt 1.16, bonnet 1.185 level, radiator 1.20, cant rail 1.63, roof 1.70 |
| bonnet plan | tapers from the scuttle (about 1.0 m wide) to the radiator (about 0.6 m) | proportion of period saloons | 1.00 m at the scuttle, 0.566 m at the radiator |
| mass | 1.2 to 1.4 t | class data | 1300 kg (unchanged) |

## 2. Audit against the reference

- Rig (committed state): `axle-f` at x = 1.50 with the radiator at 1.68 to 1.80: the radiator hung 0.2 m ahead of the axle, wheelbase 3.00 m against 3.21 to 3.22 m. Root defect. `chassis.lengthM` 4.39 against the body length. Axle height 0.37 = tyre radius: right. Wheel bones revolute on the axle bones: right.
- `dimensions.widthM` (the length) 4.38 against 4.45; height and depth right.
- Anchor `tailpipe` at (-2.08, 0.25, -0.35): 0.17 m below the pipe that the drawing showed.
- 2D: all four scales drew the 3.00 m wheelbase; front and plan were constructed, not projected (plain boxes for bonnet and roof); print suitability already failed in the committed file at 1:50 (`running-bracket-3-r` / `spare-mount` gap 0.05 mm) and 1:100 (gap 0.07 mm, `axle-f-mid` feature 0.154 mm): the product refused to draw these scales ("no suitable representation").
- Solids (state I found in the source): bonnet top a 60 mm slab over flat sides that overhung the rounded shoulders of the radiator shell by up to 50 mm; bonnet 0.86 m wide at the scuttle against a 1.24 m body, so the cowl necked in 35 degrees; roof a flat plate; wings flat strips; 1:100 and 1:200 tub and cabin solid blocks behind the glass (cross-scale volume band failed by 0.245 and 0.037); 1:500 structure outside the body in front view (0.16 m), monotonic counts failing, `interior` referenced body solids although hidden, hood area -17.6 %, tailpipe outlet 45 mm at 1:500; structure without engine bearers; decisions placeholders.

## 3. Root fixes (catalog file, written by `r3-vintage-car-1929-root.py`, members `summary`, `provenance`, `dimensions`, `rig`, `anchors`, `representations`, `thermal`, `structure`)

| member | old | new |
|---|---|---|
| `rig.bones[axle-f].x` | 1.5 | 1.7 |
| `rig.bones[chassis].lengthM`, title | 4.39, "... spare wheels" | 4.45, "Body, frame, fenders and spare wheel" |
| `dimensions.widthM` | 4.38 | 4.45 |
| `anchors[tailpipe]` | (-2.08, 0.25, -0.35) | (-2.126, 0.42, -0.35), the mouth of the modelled tail pipe |
| `thermal.regions[radiator].areaM2` | 0.512 | 0.75 (factor 1.46, within C16; hand value: shell band 2.2 m x 0.12 m outside 0.26 + inside 0.23 + two face rings 0.17 + neck and cap 0.03 = 0.69 to 0.75 m2; the old value was itself a validator read-out of the previous model) |
| `structure.assumptions[0]` | front share 48 % | 44 % (centre of mass 1.40 m ahead of the rear axle on 3.20 m) |
| `summary`, `provenance.notes`, new reference `voisin-c11-c14-data` | 4.4 m, wheelbase 3.0 m, spare wheels | 4.45 m, wheelbase 3.20 m, one spare wheel |
| `representations[50..500].geometry` | constructed views on the short wheelbase | every view redrawn from the same hand-stated design lines as the solids (`r3-vintage-car-1929-2d.py`), ids, bones, roles and regions kept where the shape existed |
| `representations[50].decisions.silhouette`, `[500].decisions.omitted/retained` | old wording | wording of what is drawn now |

Poses, clips, contact anchors and the wheel rolling rule (theta = -x / r) do not depend on the wheelbase and are unchanged; checked on `arrive-and-park` t = 1.3 s (3D sheet) and `drive-by` t = 0.7 s (2D). `structure.supports` stays empty: tyre contacts are pose contacts in this asset family, and the validator refuses a support solid whose anchor is not in `structure.supports` (tried, `volume-anchor`), so no support solids exist. I did not turn pose contacts into permanent supports, that would change the structural semantics of all vehicles.

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 213 | 82 | 74 |
| 1:100 | 112 | 47 | 54 |
| 1:200 | 77 | 39 | 29 |
| 1:500 | 21 | 26 | 23 |

- 1:50 body: ladder frame (lofted tapered rails with kick-up, six cross-members), leaf springs with shackles and hangers, dropped front axle with kingpins, stub axles, back plates, tie rod, steering arms, drop arm and drag link, banjo rear axle, propeller shaft, engine (block, head, sump, bell housing, gearbox, fan, dynamo, manifolds, carburettor, bearers), exhaust with silencer and hangers, fuel tank; hollow tub (sides with arches, turned-under tail, wheelhouse plates, floor, heel board, seat pan), cowl, tilted cabin frames with radiused openings and glass, windscreen frame, rear wall with light, crowned roof with visor (one loft, superellipse 2.6), bonnet with rounded shoulders matched to the radiator shell (new), louvred side panels, hinge and catches, radiator shell, core, neck and cap, crowned wings (new section), running boards, aprons, brackets, wing stays, head lamps on a tie bar, fender lamps, tail lamp, plate, twin-blade bumpers on irons with clamps, door gaps, hinges, handles, belt moulding, spare wheel, benches, steering column, wheel and box, dashboard with dials, mirror (new); wheels with tyre, rim, twelve spokes, hub, cap, drum, valve.
- 1:50 thermal: sheet and paint shells for panels, bonnet and wings, roof, glazing, interior (hidden, own solids), radiator, engine, exhaust line with outlet patch, tyres, drums. Areas against `areaM2`: body-panels -5.5 %, roof +3.8 %, hood -8.7 %, fenders +5.4 %, glazing +13.9 %, interior -2.2 %, radiator -0.6 %, engine-bay +4.7 %, exhaust -10.4 %, tyres +9.3 %, brakes +4.2 %.
- 1:50 structure: wheels on bearings, axles, springs with clamps, hangers and shackles, frame with cross-members and engine bearers (new), ash body frame (sills, 4 pillars a side, cant rails, 5 roof bows, 4 floor bearers), masses in the body and the wheels.
- 1:100: authored with its own lines; tub and cabin now hollow (25 mm skins, floor, benches, steering wheel visible through the glass) instead of a block; crowned roof, shouldered bonnet with six louvres.
- 1:200: hollow shell with 30 mm walls and glazed frames (new), bonnet one mass, wheels with a spoke cross, body frame with three pillars a side.
- 1:500: closed masses, one wing-and-running-board band a side, axle beams (new, drawn in front and plan), wheel discs; structure with rails moved inside the underbody, box masses in the wheels.
- Coarse thermal shells of layered regions now name the outer layer `paint`, so the views colour them by region.

## 5. What I saw (Read tool, after the last substantial change unless said)

- 1:50 body sheet (six views), three-quarter from front right (turn 50) and front left (turn 120), rear three-quarter (turn 250, before the bonnet change), front, right side: reads at first sight as a late-1920s six-light saloon; radiator over the axle, wheels centred in the wing arcs, bonnet shoulders run into the shell, cowl widens evenly, roof crowned with a visor lip, nothing floats. Tyres and lamp bowls look faceted in the sheets: that is the viewer's fixed 16-segment tessellation, the solids are revolves.
- 1:50 thermal cut and structure sheets: regions distinct, engine, exhaust and benches inside; frame, body cage, axles and wheels connected; the final structure three-quarter view, opened last, shows the four engine bearers on the rails.
- 1:100, 1:200, 1:500 body sheets; 1:100 structure sheet and thermal cut; `volumes-render` sheets of 1:100, 1:200, 1:500 (overlays green, IoU 0.984 to 0.998).
- Product 2D (`r3-2d.ts`) of all four scales, line and solid: true views of the bodies; clip frame `arrive-and-park` 1.3 s in 3D: wheels turn about the axle joints, body closed. The 2D clip frame was rendered but not opened.

## 6. Still imperfect

- Track 1.34 m against 1.35 to 1.42 m; body corners are sharp by intent (type), a car designer who expects a pressed-steel body will find them hard.
- The bonnet is a closed loft over flat side panels, not a 1 mm sheet; a small step shows where its rear section meets the cowl corner.
- Wheels are twelve tapered spokes (artillery pattern); Voisins usually ran disc or wire wheels (the 1:50 2D decision text now says tapered spokes).
- 1:100 elevation overlay shows a thin red dashed ring inside both wheels (an old wheel shape kept from the drawing, IoU 0.995); not cleaned.
- 1:200 wheels are a two-bar cross, as the drawing; honest for the scale but not a real wheel pattern.
- No support solids (see 3). No wipers, mascot, brake rods, shock absorbers, pedals.
- The web sources of the new reference were not re-opened by me.

Scripts: `r3-vintage-car-1929.py` (driver), `-kit.py`, `-s50.py`, `-coarse.py`, `-decisions.py`, `-2d.py`, `-root.py`, `-suit.ts`, `-go.sh` (one iteration: source, candidate, validate, root apply, splice, suitability). Output under `🗑️generated/r3-vintage-car-1929/`.
