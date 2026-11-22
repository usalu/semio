# machinery/concrete-mixer-truck — rework report

Chisel: `T/rework-10-concrete-mixer-truck-p1.py` (hand-chosen numbers, uses `rework-10-lib.py`). Earlier batch report `T/rework/10.md` only reached the excavator; for this asset it holds one audit row (116 of 185, 78 of 141, 60 of 104 body solids byte-identical to 1:50; 32 solids under the feature at 1:500).

## Pass 1

### Reference (four-axle 8x4 truck mixer, 9 m3 nominal, 32 t GVW, e.g. MAN TGS 32.xxx / Mercedes Arocs 3240 with Liebherr HTM 904 or Stetter AM 9 C)

| quantity | real machine | model before this pass (rig, solids, sheet) | verdict |
|---|---|---|---|
| length x width x height | 9.0 to 9.4 x 2.50 to 2.55 x 3.75 to 3.95 m | 9.0 x 2.55 x 3.65 m | ok (height at the low end) |
| unladen mass / drum with flights | 13 to 14 t / 3.4 to 3.8 t | 13.5 t / 3.6 t | ok |
| axle spacing 1-2 / 2-3 / 3-4 | 1.70 to 2.00 / 2.3 to 2.9 / 1.35 to 1.40 m | 1.795 / 2.505 / 1.40 m | ok |
| front overhang / cab length (day cab) | 1.45 to 1.50 / 1.6 to 1.9 m | 1.47 / 1.9 m | ok |
| tyre 315/80 R 22.5 (13 R 22.5): diameter / width / rim | 1.08 to 1.12 / 0.315 / 0.572 m | 1.04 / 0.315 / 0.576 m | ok (diameter 4 % small, kept: all drawings, anchors and fenders are built on it) |
| track front (tyre centres) / over twin tyres rear | 2.03 to 2.05 / 2.50 m | 2.04 / 2.505 m | ok |
| twin tyre centre distance | 0.35 m | 0.35 m | ok |
| frame: rail height x flange, outer width | 270 x 85 mm, 0.76 to 0.77 m | 270 x 85 mm, 0.762 m | ok |
| drum: largest diameter / length / axis slope | 2.30 to 2.40 m / 4.9 to 5.4 m / 10.5 to 12 degrees | 2.30 m / 5.4 m / 11 degrees | ok |
| drum mouth diameter / track ring diameter | 1.1 to 1.2 / 1.5 to 1.6 m | 1.19 / 1.60 m | ok |
| mixing flights | 2 spirals, 0.35 to 0.45 m high in the belly | 2, tapered | ok |
| hopper top over ground / rim | 3.6 to 3.9 m / about 1.0 x 0.9 m | 3.65 m / 1.0 x 0.54 m | ok |
| swivel chute / folding chute length | 1.2 to 1.4 / 0.8 to 1.0 m, trough 0.5 m wide | 1.25 / 0.85 m, 0.52 m | ok |
| chute support | lift cylinder or spindle between swivel bracket and trough, stroke for about -8 to +15 degrees | none: the trough hung from the swivel ring alone; joint limits -15 to +25 degrees | DEFECT |
| steering | axles 1 and 2 steer on king pins about 1.80 m apart, tie rod and drag link | one rigid bone per axle, no king pins | DEFECT (open, see below) |
| front suspension | parabolic or semi-elliptic leaf springs 1.5 to 1.7 m, cambered | straight 90 x 90 mm bars | DEFECT |
| cab hardware | wipers, indicators, number plate, beacons, roof hatch, mirrors, steps, lamps | mirrors, steps, lamps, grille, visor only | DEFECT |
| wheels at 1:100 / 1:200 / 1:500 | modelled for 15 / 30 / 75 mm | byte copies of the 1:50 wheels (14 mm grooves, 14 mm discs with hand holes, 12 mm flanges) at every scale | DEFECT |
| axle tubes at 1:500 | reach the brake drums | ended 35 to 40 mm short of the drums: wheels floated | DEFECT |

### Judgement of the state found

- 1:50 body: a part-by-part model (259 solids) with right overall proportions from all six directions; the defects are the ones in the table.
- Coarse scales: every non-wheel part had already been re-authored per scale; all 60 identical solids at 1:500 (and 78 / 116 at 1:200 / 1:100) were wheel parts and a few parts that are legitimately the same at 1:100.
- Structure: real load path (frame, subframe, pedestals, rear frame, springs, bogie, axles, wheels, drum shell with ring and flights, chute). Thermal: region solids nested, drum skin closed.

### Rebuilt in this pass

Rig (catalog `asset.rig`, poses, clips):
- Two new bones for the chute lift cylinder: `chute-lift` (barrel, child of `chute-swivel`, foot pin 0.70 m below the chute hinge on the swivel axis, rest 56.48 degrees to the swivel bone, revolute -12 to +12) and `chute-lift-rod` (rod, child of `chute`, pin 0.55 m along and 0.23 m under the trough line, revolute -25 to +25). Pin distance 0.508 m at rest, 0.426 m at 8 degrees lowered, 0.353 m at +15, 0.589 m at -8: barrel 0.31 m and rod 0.33 m overlap over the whole range.
- `chute` limits corrected from -15..+25 to -8..+15 degrees (the stroke a real single-stage chute cylinder covers).
- Poses `discharging`, `chute-swing` and every key of `discharge-sequence` that moves the chute carry the two solved angles (barrel and rod stay on the pin line; between keys the linear interpolation error is below 0.1 degree).

Body:
- 1:50: lift cylinder (two hanger cheeks from the swivel ring on the side away from the concrete flow, foot pin, 70 mm barrel with base lug and gland, barrel eye, 36 mm rod, rod eye, lug pair and pin under the trough); cab hardware (two parked wipers with arms, corner indicators, number plate, two beacons, roof hatch); the four front leaf springs as cambered five-leaf packs seated on the axle beams instead of straight bars. 259 -> 279 solids.
- 1:100: wheels for 15 mm (tyres without the 14 mm grooves, 15 mm rim flanges, 16 mm hand-hole discs); lift cylinder as hanger plate, barrel, rod, lug; indicators, plate, beacons, hatch.
- 1:200: all wheels re-authored for 30 mm (eight-point tyre, plain rim band, closed 30 mm disc, turned hub, no spacer bands); lift cylinder; beacons, hatch.
- 1:500: all wheels re-authored for 75 mm (six-point tyre, one turned face of disc and hub boss per wheel end, web disc in the inner twin tyres); axle tubes lengthened into the brake drums (they ended 35 to 40 mm short, the wheels floated); lift cylinder as 80 mm strut. 104 -> 80 solids.

Thermal: every new or re-authored body solid is referenced in its region (`chute`, `cab`, `tyres`, `running-gear`, `brakes`) at every scale; nothing else changed.

Structure: hanger, barrel, rod and lugs are members of `chute-swivel`, `chute-lift`, `chute-lift-rod`, `chute`, with the two pins as joints (own joint solids at 1:100 to 1:500); wheel members re-bound to the re-authored tyres, rims, discs and hubs per scale.

2D (`representations`): lift hanger, barrel and rod in elevation and plan at 1:50, 1:100, 1:200 (revolved / extruded shapes on the new bones, so they follow every pose); beacons and roof hatch in elevation, front and plan; number plate, indicators (1:50, 1:100) and wipers (1:50) in the front view.

Decisions: one new sentence pair per scale (en, de); the sentence that claimed identical wheels at 1:200 and 1:500 replaced.

### Read after the last change

1:50 body three-quarter front and rear, back, `discharging` (sheet, rear close-up), 1:200 and 1:500 `discharging` rear close-ups, 2D 1:50 `discharging`. Lift cylinder closes on both pins in `transit-agitating` and `discharging`; cab hardware sits on the surfaces; coarse wheels read as tyre, dish and hub.

## Pass 1 (second round)

Chisels (hand-chosen numbers, all in `T`): `rework-concrete-mixer-truck-rig.py` (rig, steering solids, bindings, 2D bindings), `-join.py`, `-join2.py`, `-join3.py` (carriers at 1:100, 1:200, 1:500), `-plan50.py`, `-elev.py` (2D). Probes written for this asset: `rework-concrete-mixer-truck-gaps.ts` (every loose group of the product rule `volumes-connected` with its three nearest solids and exact gaps), `rework-concrete-mixer-truck-plandiff.py` (coarse raster comparison of a drawn view with `r3-project2d` output).

### State found

- Source `T/volumes/machinery/concrete-mixer-truck.json` and catalog `volumes` were identical (all 12 lists). The catalog was `INVALID`: 8 `checkVolumes` findings, 7 of them `volumes-connected` at 1:100, 1:200 and 1:500 (17 / 20 / 7 floating body groups, open load paths in all three structure lists) and the 1:50 plan silhouette (p95 46.9 mm, limit 30). The earlier pass had joined the parts at 1:50 only.
- 1:50 body judged at full size from six directions and in `discharging`: proportions right against the reference table of pass 1 (9.0 x 2.55 x 3.65 m, axles 1.795 / 2.505 / 1.40 m, drum 2.30 x 5.4 m at 11 degrees). Rig defect left open by pass 1: one rigid bone per axle carried the axle beam and both wheel ends (a spinning wheel bone would have spun the beam), no king pins.

### Reference additions (steering and running gear of an 8x4 chassis)

| quantity | real | model now |
|---|---|---|
| king pin spacing front axle (track 2.04 m) | 1.70 to 1.78 m | 1.72 m |
| king pin offset to wheel centre plane | 0.12 to 0.17 m | 0.16 m |
| king pin diameter x length | 50 to 55 x 230 to 260 mm | 52 x 240 mm |
| lock angle axle 1 / axle 2 (inner wheel) | 40 to 45 / 28 to 32 degrees | limits +-42 / +-30 degrees |
| tie rod, drag link diameter | 40 to 50 mm | 40 mm |
| tie rod position | behind the beam, below the sump | 0.24 m behind, 0.06 m below the axle centre |

### Rebuilt

Rig (13 -> 25 bones): `wheel-1..4` replaced by `axle-1..4` (fixed, carry beam or drive axle housing with differential, 380 kg each), `kingpin-1-l|r`, `kingpin-2-l|r` (vertical, at z +-0.86, steering by twist, limits +-42 and +-30 degrees) and `wheel-N-l|r` (origin at the wheel centre, or between the twin tyres; spin about the lateral axis; 110 kg each; mass total unchanged 13 500 kg). `chute-swivel` got twist limits +-100 degrees (it had none). All wheel solids, structure entries and 2D shapes re-bound per side; wheel solids re-expressed in the new wheel frames. Steering tested with a candidate pose (king pins 30 / 38 / 21 / 26 degrees): wheels turn about their pins, read from above. No pose of the asset steers yet.

Body 1:50: king pins, forked knuckles (body and two lugs), steering arms, tie rods behind both front beams, steering box on the left rail, pitman arm, drag link to axle 1, relay rod with lever and second drag link to axle 2; front beams shortened to end in the king pin boss; oil sump moved 60 mm back so the beam of axle 1 no longer passes through it. 310 -> 340 solids.

Body 1:100 / 1:200 / 1:500 (each sized for 15 / 30 / 75 mm): steering reduced per scale (1:100 pin, knuckle block, arms, rods, box; 1:200 knuckle, arm, tie rod; 1:500 knuckle block). Carriers that were missing: rear frame sill across the subframe, hopper gussets, pedestal foot plates, bogie hangers, bogie trunnion (1:200), spring seats on all axles, front spring hangers (1:200), fender stays, spare chute cradles, battery / silencer / air filter / fuel tank / air tank / oil cooler / sun visor brackets, exhaust outlet, intake elbow, mirror arms, ladder foot, platform stay, lamp beam at its true depth, swivel beam and air filter block (1:500), support rollers at 1:200 and rollers of 240 mm that touch the track ring. Result of the probe: one body group and no unsupported structure group at 1:100, 1:200 and 1:500.

Thermal: every new solid referenced in the region of the part it belongs to (`running-gear`, `chassis`, or the region of its neighbour); the 1:100 thermal sheet reads as the same object.

Structure: axle beams are members of `axle-N`, king pins joints of `kingpin-N-s`, knuckles, arms, rods members; wheel members and wheel bearings on `wheel-N-s`; contact supports stay on the axle bones. Sill, gussets, feet, hangers, seats, trunnion are members, so the path drum -> rollers / gearbox -> pedestals -> subframe -> rails -> springs -> axles -> wheels -> contact patches is closed at every scale.

2D: wheel shapes bound per side (elevation: near wheel, plus the far tyre behind it; front and plan by side); knuckles in all three views at 1:50 and 1:100, tie rods in front and plan at 1:50; 1:50 plan: battery box, air tanks, ladder foot, rear frame gussets, relay rod, second drag link, lamp beam at 100 mm (plan silhouette finding gone); 1:50 and 1:100 elevation: relay rod, relay lever, steering box.

Decisions: one sentence pair per scale for the kinematics, one per coarse scale for the carriers (en, de).

### Read after the last change

1:50 body (six views, three-quarter from below with rig, right side), `discharging` sheet, steering test from above, 1:200 and 1:500 body sheets, 1:50 structure sheet, 1:100 thermal sheet, 1:50 2D drawings (before the plan and elevation additions).

### Open, stated plainly

- 1:50 `volumes-silhouette` elevation: p95 16.9 mm against 15.0 (largest distance 324 mm). Not located. It is unchanged to the digit when the far tyres, the knuckle drawings, all steering solids, the sump move or the swivel limits are taken out (five candidates measured), so it does not come from this pass's parts; the first full run of this pass reported only the plan at 1:50.
- The engine outline of the 1:50 elevation still shows the sump at its old place (60 mm forward, behind the tyre).
- Not rendered after the last change: 1:100 body sheet, structure at 1:100 / 1:200 / 1:500, thermal at 1:50 / 1:200 / 1:500, poses `parked-off`, `charging`, `chute-swing`, clip `agitate-at-rest`, second instant of `discharge-sequence`, 2D of 1:100 / 1:200 / 1:500.
- No pose or clip uses the steering; in a steered pose tie rods and drag links (fixed to axle and chassis) would not follow the arms, and the flat wheel elevations would need `solid` shapes.
- Axles are fixed bones: no suspension travel, no bogie rocking about the trunnion.
- Tyre diameter 1.04 m (4 % under 315/80 R 22.5), kept from pass 1.
- Front wheel and king pin drawings are absent from the 2D of 1:200 and 1:500 by decision (under 0.5 mm on paper); tie rods are not drawn at 1:100 and 1:200.

### Validation (end of this pass)

`r3-validate.ts machinery/concrete-mixer-truck --catalog-only --volumes`: `INVALID … schema 0, validateAsset 0, checkVolumes 1` (was 8). The one finding: `volumes-silhouette 1:50 … disagrees with the elevation drawing: p95M 16.9 mm (at most 15.0 mm); the largest boundary distance is 324.4 mm`. Source and catalog `volumes` agree (published with `volumes-insert.py` after every stage).
