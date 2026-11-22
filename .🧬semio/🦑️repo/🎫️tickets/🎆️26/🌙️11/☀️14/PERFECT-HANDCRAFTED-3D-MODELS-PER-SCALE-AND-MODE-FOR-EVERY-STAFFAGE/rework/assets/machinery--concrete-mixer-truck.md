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
