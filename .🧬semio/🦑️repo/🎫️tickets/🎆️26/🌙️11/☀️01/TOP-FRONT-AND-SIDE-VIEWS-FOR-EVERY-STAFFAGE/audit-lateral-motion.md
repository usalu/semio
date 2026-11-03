# Audit: Poses Needing Lateral Spread or Axial Twist

Source: a read-only Haiku audit of every asset's pose and clip names, descriptions and decisions texts. `abductionDeg` is lateral spread. `twistDeg` is axial twist; for a vertical bone it is a yaw.

| Asset | Pose / clip | Bones | DOF | Present | Owner |
|---|---|---|---|---|---|
| herring-gull | glide/landing poses | arm-n/f | spread | yes | QA1 (also update the stale "wingspan not drawn" text at 1:50 and 1:200) |
| horse | graze-and-swish (no tail track); standing-rest ears | tail-1..4; ear-n/f | spread; twist | no | QA1 |
| hedgehog | rolled, rolling-half | coat-r/f | spread | no | QA1 |
| sleeping-person | side, curled, turn-over | limbs; torso | spread; twist | limbs only | QA5 |
| tour-guide | waving and pointing; turning-to-group | arm-r; torso, head | spread; twist | spread only | QA5 |
| standing-adult | pointing | arm-r | spread (−10 is too small) | weak | QA5 |
| runner-with-blade-prosthesis | hands-on-hips | arms | spread | no | H3 |
| skateboarder | cruising, ollie-air | arms | spread | no | H3 |
| jogger | stretch-stand | arms | spread | no | QA4 |
| cyclist | dismounted-astride | thighs | spread | no | QA4 |
| person-with-white-cane | diagonal-technique, constant-contact | arm-r, forearm-r, cane | spread | no | H2 |
| person-cooking | standing-by | head | twist | no | H2 |
| mallard-duck | wing-stretch | wing-n/f | spread | no | QA8 |
| pigeon | wings-half-open | humerus, forearm, hand | spread | no | QA8 |
| red-fox | mousing-pounce head cock | head | twist | no | QA8 |
| office-swivel-chair | swivel-rotation | seat subtree | twist | no | QA7 |
| bar-stool | turned-*, swivel-turn, sit-and-turn | seat subtree | twist | no | QA7 |
| tower-crane | parked-weathervane | turntable (new physical slewing ring) | twist | yes (TS) | QA6 |
| concrete-mixer-truck | chute-swing, discharge-sequence | chute | twist | no | QA12 |
| delivery-van | rear-doors-open | rear-door | twist (vertical hinge) | no | QA11 |
| electric-kick-scooter, city-bicycle | steering | steer/fork | twist | no | QA11 |
| shopping-trolley | gate-raised | gate | check | no | QA10 |
| glass-balustrade | wind-pressure | panel | deflection under load, not a pose: excluded | — | — |

Excluded as in-plane or sagittal: excavator, stepladder, dining table, sliding doors, split AC, pigeon take-off, gaits, vegetation sway.
