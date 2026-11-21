# Child passenger anatomy and motion

## Parent Carrying a Toddler

The carried two-year-old has a 60.86 mm lower cervical segment, a 0.21 kg neck mass, and paired occipital joints 17 mm apart. The head and neck retain their combined 1.83 kg mass and neutral centre of mass. The nose, mouth, eyes and wool hat follow the skull. The warm neck surface ends at the collar.

The source at `649ef3e8684851ce` passed the schema, paper and four-scale geometry checks. Three.js independently checked both respiratory outlets against the skull in 10 poses. All 12 GLB scenes reopened from 7,959,096 bytes.

## Parent with Stroller

The 10 kg passenger now sits on the stroller centreline. Fourteen articulated segments distinguish the neck, skull and paired upper arms, forearms, hands, thighs, lower legs and feet. Snowsuit seams follow the elbows and knees. The passenger reaches towards the parent, turns the head, or draws up a foot in the relevant authored poses. The nose and mouth anchors now coincide with their visible outlets. Each was formerly displaced 192.5 mm sideways. The measured uncovered skin area is 0.042 m².

The source at `1ae000d3e4f23fa6` passed the schema, paper and four-scale geometry checks. Three.js independently checked both respiratory outlets against the skull in 14 poses. All 12 GLB scenes reopened from 7,902,568 bytes.

## Animation export

The FBX readback used Three.js AnimationMixer and compared all passenger joint positions at five times per export. Every authored passenger clip was checked at 1:500. The lifting and stopping clips were also checked at 1:50.

| Asset | Clip | Scale | Joint positions | Maximum displacement (mm) |
| --- | --- | --- | ---: | ---: |
| parent-carrying-toddler | walk-carrying | 1:500 | 65 | 0.3820 |
| parent-carrying-toddler | shift-toddler | 1:500 | 65 | 0.0297 |
| parent-carrying-toddler | shift-toddler | 1:50 | 65 | 0.0297 |
| parent-with-stroller | push-walk | 1:500 | 70 | 1.3482 |
| parent-with-stroller | stop-and-check-child | 1:500 | 70 | 0.0001 |
| parent-with-stroller | stop-and-check-child | 1:50 | 70 | 0.0001 |
| parent-with-stroller | kerb-tilt-up | 1:500 | 70 | 0.0340 |

## Garment and skin surface accounting

Articulating the neck and garment seams preserves the subject's original declared surface area. The leaning adult retains 2.268 m², including 0.092 m² exposed head skin. The adult carrying the toddler retains 2.7339 m², including 0.098 m² exposed facial skin. The stroller passenger retains 0.450 m² across 0.408 m² clothing and 0.042 m² skin. Closed garment ends at the stroller passenger’s elbows and knees are excluded from exposed heat exchange. The independent [surface budget fixture](digital-clothing-balance.json) checked all three installed models.
