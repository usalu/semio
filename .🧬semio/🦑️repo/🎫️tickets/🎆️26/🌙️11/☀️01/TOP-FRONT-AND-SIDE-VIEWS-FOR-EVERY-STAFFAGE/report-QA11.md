# Report QA11: Visual QA of Vehicles

- **Checks:** the check CLI is clean for 9 of 9. The sweep and consistency are ok for 34 of 34 combinations. The three-views vitest passes 18 of 18. `consistency.ts` ran directly with no findings.
- **delivery-van:** the single prismatic rear door is replaced by two revolute vertical hinges, `rear-door-r` and `rear-door-l` (`Bone.z` ∓0.88), swung with `twistDeg` ±90. The elevation leaf is corrected to 1.35 m.
- **Steering:** the steer bones lie along the head tube with its rake, and the yaw is a real `twistDeg`.
  - city-bicycle: 70° head angle, 4.5 cm rake.
  - kick-scooter: 98° stem.
  - motor-scooter: 73°.
  - cargo-bike: vertical bar and a fork along its axis.
  - Wheels, forks and fenders are extruded solids.
  - New `turning-left` poses for the motor-scooter and cargo-bike.
- **city-bus 1:100:** the front is enriched with windscreen halves, the destination sign and mirrors.
- **Writes:** atomic. `T/🧰️QA11/run.sh` rebuilds all nine vehicles.
- **Open:**
  - Yawed poses not inspected at 1:200 and 1:500.
  - The motor-scooter and cargo-bike 1:200 fronts lost the fender top.
  - No clip opens the van doors.
  - Rigid plan parts away from the origin height yaw about a slightly wrong pivot. Plan points are taken at the bone origin's height, which is a framework limit; solids fix it, and TS has been told.

## Last Round
- **Fender tops:** at 1:200 the front wheel stays a disc solid, and a small `<id>-top` solid on the steer or fork bone carries the fender top, so it steers with the bone and no longer spins (motor-scooter, cargo-bike). `T/🧰️QA11/rig.py` `wheel_solids`.
- **Yawed plan parts:** scooter fork legs are capsule solids along the steering axis (`leg_solids`). Cowl, fender, fork and headlamp were already solids. Handlebar, grips and head blocks stay flat; they sit within about 0.07 m of the bone origin height (a few mm of pivot error, and TS's twin-height rule now carries them at their elevation height).
- **Van:** `stop-and-unload` opens the sliding door at 5 to 7 s; rear doors stay closed because the clip unloads through the side door. Accepted by the coordinator.
- **Yawed sheets:** cargo-bike and motor-scooter `turning-left` at 1:200 and 1:500 read correctly; at 1:500 the turn is only a slight skew (physical size limit).
- **Verification:** check ok for all nine vehicles, consistency ok for 34 asset/scale combinations, three-views vitest 18 of 18.
