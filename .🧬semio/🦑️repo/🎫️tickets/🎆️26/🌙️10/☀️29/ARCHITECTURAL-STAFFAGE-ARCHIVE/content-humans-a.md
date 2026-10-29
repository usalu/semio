# Content report: humans-a (6 assets)

File: `S/🗂️catalog/👤️humans/🔣️.json` (CategoryDocument, S = `🎓️teaching/🏛️architecture/🧍️staffage`), about 1 MB. Design sources and converter in
`T/🧰️scripts/humans-a/` (`*.mjs`, `build.mjs`, `cycle.sh`); previews in `T/🗑️generated/humans-a/*.png` (own sketch renderer, the official preview CLI did not exist yet).

## Authoring method (honest disclosure)
Outlines, rigs, poses and all physical numbers are hand-designed in rest-world coordinates (f forward, u up, metres) in the `.mjs` sources; `build.mjs` only converts them to
bone-local JSON (coordinate transform, no content generation). Gait/pedal/push/jump key frames are computed by two-link inverse kinematics from authored foot paths
(`common.mjs`: `makeGait`, `ik2`, `armIK`), as the engine's reference walker does. Shapes of limbs are scaled/rotated copies of one authored pattern (`PAT`); clothing,
heads, hair, accessories and mechanisms are distinct per asset.

## Check status
`repo tools exec bun -- ...🩺️check... --file ...👤️humans/🔣️.json`: walking-commuter, seated-reader, wheelchair-user, elderly-with-cane, cyclist print ✓.
running-child is ✗ ONLY at instants that are genuinely flight phases (poses airborne/toe-off/jump-apex/flight-1..3, sprint-cycle@0.195 and @0.455, standing-jump@0.787 and
@1.049): "no active support" and force residual = body weight (234 N, correct for free flight). Needs a coordinator decision: the check should accept `contacts = []`
when the sampled phase is a declared flight phase. All other instants of the child are clean (it uses `quasi-static`; with `inertial` the plant seam of a flight clip
produces spurious 0.3 m/s root-velocity steps).

## Engine observations worth knowing
- Spinning bones (wheels, crank) cannot loop: sampled offsets are plain numbers, the engine drops the closing key and Hermite-wraps value 0 after the last value, so a
  monotonic spin breaks at the seam. All wheel/crank clips are therefore `loop: false` with 4 cycles; spoke/crank symmetry makes a restart seamless. `advanceM` on these
  `rootMotion: "none"` clips is documentation (ground travel, treadmill frame) and is not validated.
- Locomotion with `plant` needs C2-smooth foot paths (quintic ease) and dense keys; smoothstep eases caused 1-3 mm kinks at heel strike and 2600 N residuals at the seam.
- Friction: with the elastic-support solver, a lightly loaded trailing foot exceeded mu 0.6 in double support; sole friction is 0.9 (rubber/leather on dry asphalt, authored).
- ik2 clamps overreach silently below 6 mm.

## walking-commuter (1.78 m, 74.7 kg, 19 bones, 60 shapes)
- Poses (named): standing, check-watch, contact-left, passing-right; key poses: loading-left, passing-left, contact-right, loading-right.
- Clips: walk-cycle (loop, plant, T 1.2 s, 48 smooth keys, advanceM 1.200013 m, heel and ball contacts with overlapping double support, worst anchor slip 0.2 mm),
  glance-at-watch (4 s), idle-breathing (4 s, loop).
- Rig: pelvis, torso, neck, head, arm/forearm/hand, thigh/shank/foot/toe (ball joint, toe flat at push-off) per side, bag on strap (revolute, 3.5 kg carried load at `bag-cog`).
- Physics: de Leva 1996 male fractions of 72 kg + 2.7 kg clothing; DuBois 1.89 m2; ISO 9920 ensemble ~1.7 clo; walking 4 km/h = 2.8 met (ISO 8996); breath at mouth, 3 s period.
- Silhouette: overcoat to the knee, scarf, bag; tapered sleeves with elbow caps, fingered gloveless hands, rounded knee caps (no joint circles); the near arm counter-swings.
- Simplifications: manifest width 0.62 -> dimensions width 0.40 (default pose standing); stop-and-glance became glance-at-watch (no walk-to-stop IK).

## seated-reader (1.34 m seated, 64.5 kg incl. 0.6 kg notebook; 24 bones, 56 shapes)
- Poses: seated-upright, reclined-reading, reaching-for-cup, standing (named); leaning-forward, rising (key).
- Clips: page-turn (3.6 s, near hand follows the free edge of a top-bound page, page bone rotates -178 deg), stand-up (4 s, sit-to-stand, feet planted, seat contacts released at 1.3 s),
  lean-back (5 s, backrest contact then reach back to the cup).
- Rig: fixed root `seat` (bench), prismatic `hip-lift` / `hip-shift` carry the pelvis (so sit and stand share one rig); rest configuration is seated. Supports: seat, thigh-support,
  backrest, heels, balls. Seated exposedFraction overrides: jeans 0.85 (0.92 when leaning), sweater 0.86 reclined.
- Physics: de Leva female fractions of 61 kg + 2.9 kg; DuBois 1.69 m2; seated reading 1.0 met; breath period 5 s.
- Simplifications: height 1.34 m (manifest 1.22 was inconsistent with a 1.68 m person on a 0.42 m bench); notebook is top-bound so that the page-turn is a true planar revolute;
  the bench is a minimal fragment (seat, legs, back slats), not the park-bench asset.

## running-child (1.15 m, ~5.8 heads, 22.1 kg; 19 bones, 41 shapes)
- Poses (named): standing-ready, mid-stride, airborne, crouch, jump-takeoff, jump-apex; key: toe-off, landing, flight-1..3 (carry the authored hip height used in flight).
- Clips: sprint-cycle (loop, plant, T 0.52 s, 3.8 steps/s, 0.18 s stance per foot, 0.08 s flight between steps, advanceM = plant-solved), standing-jump (2.4 s, 0.28 m vertical jump,
  0.48 s flight, planted root so feet do not slide).
- Physics: child segment fractions rounded after Jensen 1986 (literature-based estimate), 22 kg with clothing/backpack (1.4 kg bone mass); DuBois 0.86 m2; running 6 met, jump 8 met; breath 1.3 s.
- Simplifications: stance with heel then ball contact (no forefoot-strike path), hip arc in flight is a smooth sinusoid, not an exact parabola; ankle dorsiflexion up to 50 deg at mid-stance
  (limit raised for this asset); jump drifts forward about 0.3 m because plant continues the last velocity.

## wheelchair-user (1.45 m, 1.09 m long, 90.2 kg; 21 bones, 69 shapes)
- Poses: seated-level, hands-on-rims, push-end, ramp-ascent (chair tilted 4.8 deg about the rear axle, ground line follows the 1:12 ramp).
- Clips: push-cycle (4 s, 4 strokes, 45 deg push arc, rear wheels 240 deg and castors 3 turns per stroke = 1.278 m ground advance per stroke), ramp-roll (2 s), rest-in-chair (5 s, loop).
- Rig: mechanical, frame root at the rear axle; free revolute rear-wheel and castor bones; person attached to the frame; contacts = tyre patches (rear, castor).
- Physics: 75 kg man (de Leva) + 2.8 kg clothing, chair 12.4 kg (authored product values), DuBois 1.90 m2, propulsion 2.2-3.2 met, 12-spoke symmetry.
- Simplifications: wheel pair and castor pair are single bones; push force on the rim is internal; no loop (see engine observations); manifest poses kerb-turn replaced by ramp-ascent.

## elderly-with-cane (1.65 m stooped, 73.8 kg; 19 bones, 46 shapes)
- Poses (named): stooped-standing, cane-planted, resting-on-cane, contact-left, passing-left; key: loading-left, contact-right, loading-right, passing-right.
- Clips: cane-gait (loop, plant, T 1.7 s, 60 keys, three-point gait: cane tip lands with the weak left foot, cane tip anchored in world during left stance (slip 8 mm), then swings with the left leg),
  rest-on-cane (6 s, cane tip fixed, slip 22 mm from blended IK).
- Physics: male 70 kg + 2.3 kg clothing, de Leva fractions, 16 deg forward torso inclination plus flexed neck, DuBois 1.80 m2, 2 km/h = 2.0 met, core 36.4 C.
- Simplifications: no seated-rest pose (replaced by resting-on-cane); cane load share comes from the solver's equal-stiffness model, not a clinical measurement.

## cyclist (1.79 m, 1.74 m long, 83 kg; 27 bones, 102 shapes)
- Poses: riding-upright, standing-on-pedals, coasting, dismounted-astride.
- Clips: pedal-cycle (4 s, 4 crank revolutions at 60 rpm, 2:1 gear, 4.27 m per revolution, pedals level, feet follow the pedals), brake-and-stop (1.6 s), dismount (2.4 s, feet planted from 1.5 s).
- Rig: frame root, free revolute rear/front wheels, cranks and level pedals, prismatic hip-lift/shift; feet attached to pedals by IK; contacts = tyre patches (+ feet when dismounted).
- Physics: female 66 kg + 2.4 kg, bicycle 14 kg (700C, 170 mm cranks), 15 km/h = 4 met, relative wind 4.3 m/s in the reference environment.
- Simplifications: no steering; chain static; riding dismount skips the leg swing over the saddle (rider slides forward and steps down); no loop.

## Manifest differences to sync
heightM/widthM of the dimensions: commuter 1.78/0.40, reader 1.34/1.02, child 1.15/0.27, wheelchair 1.45/1.09, elderly 1.65/0.54, cyclist 1.79/1.74. Pose/clip ids differ from the plan:
see per-asset lists above.
