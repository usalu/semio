# Silhouette guard for the vehicles category

Result: all nine assets of `🚜️vehicles` pass `catalog-silhouette` (bbox within one feature, IoU >= 0.92, Hausdorff <= 2 features, per authored scale, elevation, front and plan) and `validate-some.ts` reports 0 for each. The guard was never loosened.

Assets: city-car, motor-scooter, vintage-car-1929, delivery-van, tram, city-bus, city-bicycle, cargo-bike, electric-kick-scooter.

## Causes found

1. Authored drawings were hand-simplified unions while the volumes were detailed (shell, mounts, hangers, brackets, mirrors, lamps, axles, brake discs, exhaust). Fix: trace the drawing from the volume per solid (outline of the union, Douglas-Peucker, tolerance 0.004 / 0.006 / 0.005 / 0.03 m at 1:50 / 1:100 / 1:200 / 1:500) and add one shape per volume solid that had no drawn twin.
2. Bone pose. Elevation shapes are bone-local, so traces use the default-pose bone frame. Front and plan shapes are carried from the rest frame to the pose through a depth profile taken from the elevation twins; for rotated roots (cargo-bike, -1.09 degrees and 0.04 m) a world trace drifts by up to 0.045 m. Fix: iterate the local points until the evaluated ring equals the traced world ring (`vehicles-cb4.py`).
3. Slender volume members take the width of the drawn shape of the same bone. Replacing a drawn shape therefore changes the volume; the front and plan trace is run twice so the volume settles before it is traced.
4. Hairline gaps: a gap between two solids that is thinner than a pen or a hole threshold exists in the volume but not in the drawing (or the reverse). Fixed by overlapping solids (cargo-bike fork legs moved inward at 1:50, fenders lowered, stand shortened) or by shortening a stroke that ran into air (cargo-bike box-planks at 1:100 and 1:200).
5. Declared extents: bbox excess from tail pipes, bumpers, fenders, door leaves, stands. Fixed by value-only shortening of the offending solid at the failing scale.
6. Outline traces only the outer ring, so unions of tubes fill their holes; one polygon per solid is drawn instead.

## Validation follow-ups made

- `shape-role-undrawn`: delivery-van 1:500 mirror and tram 1:500 front axle moved from role detail to body (detail pen is null there).
- `lateral-ambiguous`: flat shapes on the leaving-plane bones got extruded solids again (city-bicycle wheel-f tyre, rim and wheel at 1:50, 1:100, 1:200; cargo-bike fork shapes at 1:50 and 1:100) because the retrace removes the solid of the replaced shape.
- `extent-front-height` cargo-bike 1:500: the stand capsule (-0.02 m) plus the traced bar envelope exceeded the real height; the stand is now traced from the stand-web solid.

## Tools (this folder, prefix `vehicles-`)

`vehicles-run.sh` dispatches triage, diag (`vehicles-diag.ts`), owner, scan, who, outline, tiles, bones, evalall. `vehicles-lib.py` holds the span-edit helpers on top of the old `mvlib.py`. Cargo-bike is rebuilt reproducibly with `vehicles-cbrun.sh <scales>` (restore the representations from HEAD, re-add the solids, apply volume fixes `cb6`, trace `cb1`/`cb2`, extras `cb5`/`cb7-*`/`cb8`/`cb9`/`cb10`/`cb12`, fit front and plan `cb4`).

## Remaining

Nothing failing in vehicles. Borderline results worth watching if the guard changes: tram 1:50 elevation (H 0.0146 of 0.015), city-bus 1:500 elevation (IoU 0.1428 vs tile measure), cargo-bike 1:50 elevation (H 0.0131 of 0.015).
