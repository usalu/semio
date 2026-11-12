# Authoring report: vehicles (9 assets)

Sources: `volumes/vehicles/<slug>.py` (generators, explicit data) and `volumes/vehicles/<slug>.json` (spliced with `volume-insert.py`); helpers `volumes/vehicles/_lib.py`, `run-vehicle.sh`, `authoring-vehicles-{inspect,rings,extent,validate}.ts`. Tool output in `🗑️generated/authoring-vehicles/`.
Every loft section carries an explicit `rotation` (rx on w, ry perpendicular to the path in the u-v plane; fixed vertical slices `VERT` where the centre line is steep). `lateralScale` is only used on capsules built exactly along u. Bone-local coordinates were computed from the bone frames (offsets applied once).

| slug | solids | bbox err % x / y / z | IoU elev / front / plan | validator issues (Ajv + volume rules) | poses checked | remarks |
|---|---|---|---|---|---|---|
| city-car | 60 | +2.2 / -0.2 / +1.2 | 0.953 / 0.961 / 0.984 | 0 | parked, hatch-open, bonnet-open, braking-nose-down | hatch and bonnet on their hinge bones; open hatch reads as a bay (quarter panels, floor) |
| vintage-car-1929 | 75 | +0.1 / -0.1 / 0.0 | 0.972 / 0.940 / 0.960 | 0 | parked, driving, braking | running boards 0.23 m (front view draws 0.28, plan 0.18); louvres plan 0.26-0.31 vs front 0.30-0.34 |
| city-bicycle | 65 | +0.3 / -0.3 / 0.0 | **0.739** / **0.782** / 0.883 | 0 | parked-on-stand, riding, braking-stop | misses two IoU gates, see below |
| cargo-bike | 71 | -0.2 / +2.9 / 0.0 | **0.788** / 0.890 / 0.966 | 0 | parked-on-stand, loaded-on-stand, rolling-loaded, turning-left | misses elevation gate, see below |
| electric-kick-scooter | 24 | +0.2 / +0.9 / 0.0 | 0.963 / 0.932 / 0.957 | 0 | riding-cruise, braking-stop, folded-charging | deck split into rails so the rear wheel runs in a slot (2D wheel overlaps deck) |
| motor-scooter | 40 | +2.8 / -0.1 / 0.0 | 0.940 / 0.909 / 0.943 | 0 | cruising, idling-seat-open, turning-left, parked-on-stand | rear body = top tray + two side panels, wheel in slot; 2D x extent 1.88 m vs dimension 1.85 |
| delivery-van | 46 | **+17.0** / -0.1 / 0.0 | 0.990 / 0.958 / 0.995 | 0 | parked, rear-doors-open, side-door-open, braking-nose-down | rest has rear leaves open; see below |
| city-bus | 78 | 0.0 / +0.2 / +1.9 | 0.971 / 0.985 / 0.990 | 0 | level-cruise, kneeling-at-stop, braking | open wheel arches across the width as drawn; left-side windows at door gaps invented |
| tram | 75 | +0.1 / +0.4 / +2.0 | 0.991 / 0.968 / 0.989 | 0 | running, at-stop-doors-open, depot-pantograph-down | door leaves 3 cm proud (2D draws them 6 cm inside the half width, hidden by a solid body) |

Poses: in every checked pose all solids stay in one bounding-box-connected component (rest check of the script is meaningless for offset bones). Exceptions by design: cargo-bike `parked-on-stand` moves the crate 3 m aside (crate-x joint), so the crate is a separate component there. Doors open/close, hatch/bonnet open upward, pantograph folds (height 5.02 -> 3.69), kick-scooter stem folds (1.12 -> 0.54), wheels sit on y = 0 in all poses.

## Gates not met (physical body kept)
- delivery-van, bbox x +17.0 % (6.248 m vs 5.340 m): the rest frames draw both rear door leaves swung 90 degrees out (elevation/plan `rear-leaf-*` reach x = -3.57), the poses twist them shut. In the default pose `parked` the extent is 5.378 m (+0.7 %). IoUs pass.
- city-bicycle, elevation 0.739 and front 0.782: the frame tubes, stays, rack and stand are open polylines in the elevation (not counted by `volume-check`, which only rasterises closed shapes), so my 32 mm tubes lie outside the drawing (seat/down tube, stays: 8 to 24 % inside); front handlebar is drawn as the bounding rect of the swept bar (.049 m high, true tube about .028 m, 62 % covered); hub drawn R 0.055 m in elevation vs 0.04 x 0.02 m in plan/front.
- cargo-bike, elevation 0.788: `tyre-r/f` and `rim-r/f` are filled discs in the elevation (0.16 and 0.08 m2 each) while the volume has hollow tyre rings with spokes (52 to 33 % covered); rack and fender are open lines. Bbox y +2.9 % because the stand foot is drawn at y = -0.058 m below the contact line (dimension 1.16 m, drawn 1.194 m).
- Other 2D inconsistencies: electric-kick-scooter deck overlaps the rear wheel; motor-scooter and tram outlines as listed above; vintage-car and cargo-bike widths noted in the table.

## Notes for a reviewer
- Revolves use open polylines only (tyres are rounded-shoulder solid cylinders, two-wheelers use rings of capsules with `repeat`); the validator rejected closed loops (`volume-profile-self-intersecting`), no such finding remains in the spliced files.
- `lateralScale` appears only on capsules along u (silencers, main beam via transform).
- Wheel spin: all wheel solids hang on `wheel-*`/`ws-*` bones, tyres and spokes are rotationally periodic via `repeat` about the axle.
