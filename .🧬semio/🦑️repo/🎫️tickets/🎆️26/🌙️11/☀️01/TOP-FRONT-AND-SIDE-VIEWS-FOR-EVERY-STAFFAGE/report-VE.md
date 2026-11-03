# Report VE: Vehicles

**Assets:** electric-kick-scooter, city-bicycle, cargo-bike, motor-scooter, delivery-van, city-bus, tram, city-car, vintage-car-1929.

## State
- Front and plan exist at every authored scale. city-bicycle and electric-kick-scooter 1:500 stay unavailable. `presentation.viewpoints` is removed, and the en/de decisions are extended.
- **Last round** (`T/🧰️VE/all.ts`, one process):
  - The consistency sweep is ok for all 34 asset/scale combinations, covering every pose and clip instant.
  - The check CLI reports no problems. The city-bicycle 1:200 stand was thickened to a 0.263 mm minimum feature.
  - 88 previews were rendered.
- **Helper bones** `bars`, `fork-leg`, `arm-n` and `arm-f` are deleted. Their shapes are now `extruded` solids on the stem tip, `steer` and `crank` bones.
- **Other solids:** stand legs (bicycle, cargo bike, motor scooter), pantograph arms and head, city-car bonnet and tailgate, motor-scooter seat.
- **Builders:** `T/🧰️VE/build_<asset>.py` and `solidify.py`, with pristine copies in `orig/`.

## Inspected
- kick-scooter: all poses and clip instants at 1:50 and 1:100.
- city-car: parked at all four scales, plus bonnet-open and hatch-open.
- tram: running and pantograph-down.
- One pose each for the other vehicles.
- **Not inspected:** the vintage car, door-open poses, and most clip instants.

## Doubts (handed to QA11)
- Wheels hidden under bodies or fenders are omitted in plan (physically correct).
- Doors slide as rigid strips on prismatic bones.
- The bicycle `steer` is only an elevation pitch, so steering is not a yaw.
