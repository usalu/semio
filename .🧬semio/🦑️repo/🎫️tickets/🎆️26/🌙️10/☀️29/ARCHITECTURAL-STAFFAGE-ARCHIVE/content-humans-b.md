# Content notes: humans-b (6 assets)

Deliverable: `T/🧰️scripts/humans-b.json` (CategoryDocument, category `humans`). Check: all 6 print ✓ (`--file T/🧰️scripts/humans-b.json`).
Authoring aid (scratch, `T/🧰️scripts/humans-b/`): `lib.py`, `body.py`, `gait.py` plus one script per asset. All content values (width tables, paths, poses, masses,
thermal data) are hand-entered data in these scripts; the aids only convert profile coordinates, fit splines through hand-entered width tables,
mirror/boilerplate, and solve inverse kinematics for poses (hands on objects, planted feet, moment balance). Preview CLI: `humans-b-preview.ts`;
sheets in `T/🗑️generated/humans-b/`.

Common body: 15-bone anatomical rig (pelvis root, trunk, head, 2 x upper arm/forearm/hand, 2 x thigh/shank/foot), segment masses de Leva 1996 (male), DuBois area
and Lund-Browder-style region split, ISO 9920 insulation, ISO 8996 / Compendium met values, breath emitter at the mouth (`directionDeg -90` relative to the head bone = forward).
Contacts: `heel-l/r` and `contact-l/r` (ball of foot) per foot. Gait key frames come from a planted-foot inverse-kinematics gait generator: the planted
anchor stays fixed, advance equals the plant-solved travel to < 1e-6 m. Gait key poses are marked `"key": true`.

| asset | height / width / mass | named poses | key poses | clips |
|---|---|---|---|---|
| worker-carrying-box | 1.81 / 0.68 / 104 kg (84 + 20 box) | squat-grip, lift-mid, carrying-box-low, carrying-box-chest, setting-down | 10 `walk-*` gait phases | squat-lift (load-on at 0.7 s), carry-walk (loop, advance 1.1 m), set-down |
| parent-with-stroller | 1.75 / 1.70 / 89.4 kg | pushing-stroller, pausing-with-stroller, checking-child, kerb-tilt | 10 `push-*` gait phases | push-walk (loop, advance 0.99987 m), stop-and-check-child, kerb-tilt-up |
| jogger | 1.82 / 1.04 / 72 kg | initial-contact-l, midstance-l, flight-l, stretch-stand, stretch-quad-r, stretch-hamstring | 9 other run phases | run-cycle (loop, advance 1.695 m, 2 ballistic phases), stretch |
| person-with-umbrella | 2.10 / 1.05 / 70.46 kg | umbrella-up-still, umbrella-tilted-to-wind, umbrella-furled | gust-light, gust-heavy | rain-gust-sway (track `load:wind`), close-umbrella |
| leaning-person | 1.72 / 0.60 / 66 kg | leaning-shoulder-wall, leaning-arms-folded, phone-call, pushing-off, standing-away | none | phone-call-idle (loop), push-off |
| tour-guide | 2.275 / 1.254 / 78.59 kg | standing-pointing, gesturing-to-group, holding-flag, waving | wave-swing-a/b, turning-to-group | gesture-point, turn-and-wave |

Pose and clip ids differ from the manifest (manifest rows must be synced by the coordinator): worker clips lift-and-carry/set-down -> squat-lift, carry-walk, set-down;
parent poses kerb-tilt kept, clips push-and-pause/kerb-crossing -> push-walk, stop-and-check-child, kerb-tilt-up; jogger poses strike-phase/airborne/recovery ->
initial-contact-l, midstance-l, flight-l (+ stretch poses), clips run-cycle, stretch (cool-down-walk dropped); umbrella poses unchanged, clips rain-gust-sway, close-umbrella;
leaning poses leaning-arms-folded kept, slipping-from-wall -> pushing-off/standing-away, clips shift-weight/slide-down -> phone-call-idle, push-off;
tour-guide poses standing-pointing, gesturing-to-group, holding-flag kept (+ waving), clips gesture-explain/turn-to-group -> gesture-point, turn-and-wave.
Heights/widths in the manifest must follow the evaluated default-pose bounds above (e.g. tour-guide 2.275 because the flag pole is included, parent 1.70 with stroller).

## Per asset

**worker-carrying-box.** Hi-vis vest (ISO 20471 class 2, retroreflective bands), hard hat (EN 397), work boots, gloves. The 20 kg box (NIOSH load constant 23 kg) is two 10 kg
half-bones (`box-l`, `box-r`), one per hand, because both hands coincide in elevation; each box bone counter-rotates the hand so the box stays level. In `squat-grip` the
box rests on the ground (`box-base` contact) and shares its weight with the feet; the clip ends the contact at 0.7 s with a `load-on` event, after which the arms carry all of it.
Lumbar joint moment (static): -203 N m squat, -196 mid lift (trunk 50 deg), about -53 N m carrying at hip height, -47 N m at chest height; shoulder moment grows from 7 to 41 N m with the
chest carry. Simplification: far arm lumped with the near arm, box centre at the grip point.

**parent-with-stroller.** Stroller = bones `stroller` (frame, 8 kg), `stroller-hood` (hinged, 0.6 kg, folds back 95 deg), `wheel-rear`/`wheel-front` (free revolute, rolling without slip:
rear wheel one turn, front wheel two turns per stride via a clip key at the cycle end), `child` (fixed, 10 kg), attached to the right hand and counter-rotated so it stays level.
Wheel contacts `wheel-rear`/`wheel-front` (rolling friction 0.02) are in every pose and clip span set; in `kerb-tilt` only the rear wheel is in contact (front wheels 12 cm up).
Second emitter `child-breath`. Simplification: equal-stiffness load sharing puts part of the stroller weight on the feet.

**jogger.** 2.35 m/s, step rate 2.8 Hz, stance 1/3 of the cycle, flight about 0.12 s twice per cycle (`ballistic` phases), pelvis path a sinusoid of amplitude g T^2/(16 pi^2) x 1.34 so the
vertical COM acceleration in flight is close to -g (ground force about 1.9 body weights at midstance, Cavanagh). 8 met, breath period 1.5 s, skin wettedness 0.3. Flight poses have no
contact by design; horizontal COM residual in flight stays about 45 N (limb motion not mass-consistent). Ankle limit raised to -50..38 deg for running dorsiflexion; running shoe friction 0.8.

**person-with-umbrella.** Shaft `umbrella` and ribs `rib-front`/`rib-back` (open -114/+114 deg from the shaft, furl by -66/+66). `wind` load at the canopy centre (F = 0.5 rho cd A v^2, rho 1.25,
cd 0.5, A 0.55 m2, plus 35 % uplift; EN 1991-1-4, Hoerner), pose loads 3/8/12/16 m/s, clip track `load:wind` multiplies it in the gust. The lean angle for each wind speed (root angle -3.8/-5.7/-8.3 deg)
is solved so that the moment of weight and wind about the foot middle vanishes. Rain environment 8 C, 95 % RH, 4 m/s wind; wet coat (wettedness up to 0.45).

**leaning-person.** The wall is a world-fixed rig branch (`wall-frame`, `wall-x`, `wall` compensate root angle, x and y) so it stays vertical. Anchors `wall-shoulder` and `wall-seat`
(kind `support`, support `contact`, `normalDeg 0`, friction 0.4) take the horizontal wall reaction; feet resist by friction. Lean and foot positions solved so that the wall reaction is 8 % of the weight at the shoulder height.
Wall shapes use role `ground`/`texture` so they do not enter the bounds. Simplification: wall base follows the solved root height within a few mm during blends.

**tour-guide.** Telescopic pointer (fixed to the right hand), flag pole `flag-pole` (counter-rotated to stay vertical) and swaying `flag` (+-25 deg). The straight arm raised 40 deg (pointer 0.12 kg) loads the shoulder
visibly in the moments view. Turning is expressed by weight shift, foot change and head turn (the turn about the vertical axis is not visible in elevation).

## Known issues for the coordinator

- Parent/worker walking and jogger running rely on the engine plant model with overlapping double-support spans; check output is clean (✓), but the jogger flight poses `flight-l/r` are evaluable only
  as ballistic poses (static structural residual = weight, as expected for free flight).
- Legs of toe-off poses penetrate the ground by up to 4 cm at the toe tip (rigid foot, no toe joint); invisible at 1:50 and smaller.
- Hand silhouette is a simple mitten; hands holding boxes/handles are drawn over the object.
- Source tags: segment masses `literature` (de Leva), hand COM `authored-estimate`; wind coefficients `literature` with authored area; clothing insulation `literature` (ISO 9920) with authored fractions.
