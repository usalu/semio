# Labrador forequarter

The fitted short coat follows the cranial ribs, covers the upper scapula and triceps, and widens the caudal neck at the thoracic inlet. The upper arm and scapula keep their independent joints. Each scale uses its own lofts and three paper outlines traced from those lofts. The 1:500 upper forelegs now identify the fur layer explicitly.

| Scale | Standing mean shoulder protrusion before → after (mm) | Largest seated width loss per 8 mm before → after (mm) |
| --- | ---: | ---: |
| 1:50 | 23.47 → 0 | 32 → 4 |
| 1:100 | 21.94 → 0 | 32 → 5 |
| 1:200 | 18.22 → 0.02 | 20 → 5 |
| 1:500 | 28.13 → 0 | 30 → 4 |

The standing metric compares near-side ray hits on the arm against the trunk and neck within a 98 × 111 mm overlap patch. The seated metric samples the plan half-width along the shoulder-to-neck transition. A Three.js ray/triangle test independently confirms 304 overlapping pairs across all 19 named poses at all four scales. Buried upper-arm surfaces transfer the external exchange area toward the trunk. The fitted standing mesh exposes 0.49814 m² of trunk fur and 0.15032 m² of forelimb fur. The two regions declare 0.4981 and 0.1503 m² respectively.

All 13 volume rules at each of the four scales passed on the shoulder-only candidate, as did schema, semantic validation and the independent loader for the twelve-scene GLB (5,030,212 bytes). The visual comparison covers the standing body, seated plan and regional heat geometry during trot. Smooth attachment around the thoracic inlet is apparent in the seated plan. Small surface boundaries remain at moving joints. The heat view uses region colours.

The [author](lab-shoulder-fit.ts) accepts an input and output document. The [contour test](lab-shoulder-probe.ts), [pose test](lab-shoulder-joints.ts), [exchange area measurement](lab-shoulder-heat.ts) and [reference geometry](lab-shoulder-case.json) run against the output. The [comparison renderer](lab-shoulder-sheet.ts) takes the original, candidate and output prefix and produces one sheet per scale. Integration with the lumbar revision is tracked by the parent ticket.
