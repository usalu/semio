# Volume agreement: silhouette and bounding box of all 152 volumes in their default pose

Ticket `ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE`. Script `volume-agreement.ts` (this folder; extends `volume-check.ts` and `volume-lib.ts`): for each asset the pose `presentation.defaultPose` is evaluated with `evaluate` at 1:50 (so authored shapes and bone frames are those of the pose, not the rest pose), every volume solid not hidden by `maxDenominator` is tessellated by the kernel and carried by the pose's bone frames, projected through the staffage cameras and compared with the authored closed `body`, `detail` and `glazing` shapes on one shared 640-cell grid. **Open strokes are stroked with their pen width** (`shape.penMm`, else the pen of the role in the 1:50 representation, paper mm times 50) instead of being ignored: 1971 open strokes entered the masks. Gate: bounding box at most 8 % per axis, IoU at least 0.80 per view. Raw rows: `🗑️generated/verification/agreement2.jsonl` (bbox, IoU, recall, precision, worst shapes per view).

Result: 137 of 152 pass, 15 miss (before the catalog fix and at rest the same measure failed 38). The in-package guard `🧪️tests/🧊️model-silhouette` repeats the measure with polygon-clipping as the third-party oracle for the sample figure and three catalog assets.

## Misses with reason

| asset | pose | category | evidence |
|---|---|---|---|
| agave | summer-turgid | 2D views mutually inconsistent | plan draws the rosette leaves as wide flat blades (recall 0.16, volume precision 1.00) while elevation and front draw them as curved profiles; the volume follows the profiles. Inferred from the worst shapes (leaf-r1-a, leaf-l1-a). |
| bamboo-clump | summer-calm | 2D drawn as filled area | foliage masses (`foliage-*`, `mass-*`) are drawn as filled blobs, the volume models sparse leaf sprays; recall 0.8, precision 0.55 in elevation. |
| date-palm | calm | 2D views mutually inconsistent | the elevation draws frond pair 1 foreshortened (x 0.06 to 0.59 m) in one plane, the volume fans six copies about the stem; precision 0.56 / 0.53 / 0.29, recall 0.98 / 0.75 / 0.97. Extent fixed (see fixes). |
| miscanthus-grass | calm | 2D drawn as filled area | culms and blades drawn as filled strips and `crown-fill-*` areas (plan recall 0.31); the volume is the individual culms. |
| tower-crane | lifting-load | 2D lattice drawn as filled area | `jib-outline`, `mast-chords`, `jib-diagonals` are lattice members drawn as one filled outline (missing area 68 m2 of the jib outline); the volume models the bars. |
| hedgehog | foraging | small asset, absolute floor | height +11.4 % is 14.8 mm of a 0.13 m asset, inside the 15 mm floor of the guard; all IoU pass. |
| person-with-umbrella | umbrella-up-still | 2D views mutually inconsistent | umbrella side view in the elevation (catalog-fix: single-asset artwork decision); precision 0.78, recall 0.98. |
| cargo-bike | loaded-on-stand | 2D drawn as filled area | wheels drawn as filled discs (`tyre-r` 52 %, `rim-r` 33 % covered) against the spoked volume wheels. |
| city-bicycle | riding | 2D views mutually inconsistent | handlebar width in the front view (`handlebar` 62 % covered; catalog-fix lists the hub and handlebar sizes). |
| panel-radiator | rated | 2D drawn as section/cut | elevation draws the cut through the panel; the volume panel and fins project outside it (precision 0.49, recall 0.99). |
| sectional-garage-door | closed | 2D drawn as section/cut | elevation is the side section with the tracks as strokes (`track` 49 % covered); the volume panels project outside. |
| spiral-stair | self-weight | 2D lattice drawn as filled area | helical handrail and stringer drawn as outline strokes (`handrail` 71 %). |
| cooking-pot | boiling | 2D drawn as section/cut | elevation is the section of the pot (catalog-fix: cooking-pot section); precision 0.65, recall 1.00. |
| market-parasol | open-still | 2D views mutually inconsistent | scalloped ribs and crank in the open pose (`rib-*-scallop` 35 %); precision 0.58, recall 0.99. |
| retractable-bollard | raised | 2D drawn as section/cut | elevation and front are the cuts through the sleeve (`sleeve` 64 % covered; catalog-fix: retractable-bollard cuts). |

The categories are inferred from the worst shapes of each view (the rows of the raw file) and from the findings of `catalog-fix.md`; none of the misses is a volume that is wrong against `dimensions` (every bounding box is inside the guard, see `verification.md`). Genuine modelling errors found: **date-palm** (see below); no other miss is a volume error: the bodies are physically right and the 2D drawing is the simplification (section, lattice as area, filled foliage).

## Fixes

- `date-palm` (`🌳️vegetation/date-palm/🔣️.json`, mirrored in `volumes/vegetation/date-palm.json`): the distal fronds of pair 1 (`fr-r1b`, `fr-l1b`) reached x -3.554 to 3.459 m (7.013 m, +11.3 % against `widthM` 6.3) and z 3.33 m: the fan of six copies about the stem made the crown round (x 7.01, z 7.03) where the asset is 6.3 by 7.0. The solids `fr-r1b-blade`, `fr-r1b-rachis`, `fr-l1b-blade`, `fr-l1b-rachis` were shortened to 85 %: `capsule.p1[0]` 1.7595 to 1.49558 and every loft section `at[0]` times 0.85 (blade sections 0.26393, 0.87975, 1.49557, 1.7595 to 0.22434, 0.74779, 1.27123, 1.49558). Result x 6.53 m (+3.7 %), y 12.49 m (-0.1 %), z 6.55 m (-6.4 %), all inside the 8 % gate.

## red-fox, pose `curled-sleeping`: authored abduction, not a binding error

Evidence (script `verification-fox.ts`, `verification-extent.ts`): the pose authors `lumbar` abduction 22 degrees, `neck` 40 degrees, `head` 25 degrees and `tail-1` 45 degrees, all towards +z (left). The bone chain accumulates it: `thorax` origin z 0.082, `neck` 0.235, `head` 0.216 to 0.248 m, so the whole forebody lies at z > 0 and so do both pairs of limbs, ears and eyes. The volume follows the bones exactly: near solids sit consistently to the right of their far twins (`ear-n` z 0.173 to 0.225 against `ear-f` 0.242 to 0.294; `eye-n` 0.200 to 0.216 against `eye-f` 0.254 to 0.270; `forearm-n` 0.091 to 0.189 against `forearm-f` 0.193 to 0.329; `paw-f-n` z 0.102 to 0.189 against `paw-f-f` 0.239 to 0.326). The near/far order is intact; the "far side" impression is the curl towards the left flank. No solid, bone or `w` sign was changed. The 2D plan of the same pose (`evaluate`) shows the same bones.

| asset | default pose | bbox error x / y / z in % | IoU elevation | IoU front | IoU plan | verdict |
|---|---|---|---|---|---|---|
| agave | summer-turgid | -0.9 / -6.4 / -1.3 | 0.58 | 0.58 | 0.16 | FAIL |
| apple-tree | summer-fruit-set | +0.4 / -0.0 / +0.0 | 0.91 | 0.85 | 0.92 | pass |
| bamboo-clump | summer-calm | -4.6 / +0.1 / -4.8 | 0.48 | 0.68 | 0.87 | FAIL |
| date-palm | calm | +3.7 / -0.1 / -6.4 | 0.56 | 0.46 | 0.30 | FAIL |
| english-oak | calm | +0.1 / +0.2 / +0.0 | 0.87 | 0.89 | 0.91 | pass |
| hornbeam-hedge | calm | +0.0 / +2.5 / +0.0 | 0.94 | 0.87 | 0.98 | pass |
| hosta-clump | summer-full | +1.4 / -5.2 / -1.0 | 0.87 | 0.87 | 0.88 | pass |
| ivy-on-trellis | summer-calm | +1.0 / -0.5 / +0.0 | 0.90 | 0.92 | 0.82 | pass |
| japanese-maple | summer-calm | -0.5 / -0.1 / +0.2 | 0.85 | 0.91 | 0.95 | pass |
| lombardy-poplar | calm | -5.0 / -1.9 / +6.2 | 0.80 | 0.85 | 0.92 | pass |
| london-plane-pollarded | summer-calm | +0.3 / -0.0 / +3.6 | 0.90 | 0.94 | 0.93 | pass |
| miscanthus-grass | calm | -0.6 / -1.9 / +0.8 | 0.54 | 0.41 | 0.29 | FAIL |
| multi-stem-birch | calm | -3.7 / -0.5 / -3.4 | 0.81 | 0.92 | 0.86 | pass |
| scots-pine | calm | -3.8 / -0.4 / -4.6 | 0.83 | 0.84 | 0.82 | pass |
| small-leaved-lime | summer-calm | -0.2 / -0.1 / +0.6 | 0.91 | 0.93 | 0.96 | pass |
| weeping-willow | calm | +0.1 / +0.0 / +0.3 | 0.84 | 0.94 | 0.94 | pass |
| concrete-mixer-truck | transit-agitating | +0.1 / +0.6 / +0.2 | 0.98 | 0.99 | 0.94 | pass |
| diesel-generator | off | +0.2 / +0.0 / +0.9 | 0.98 | 0.98 | 1.00 | pass |
| evaporative-humidifier | running | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| excavator | stowed | -0.0 / +0.9 / +0.0 | 0.94 | 0.92 | 0.99 | pass |
| expansion-valve | open | +3.2 / -0.9 / +0.0 | 0.92 | 0.96 | 0.96 | pass |
| finned-heat-exchanger | hot | +0.0 / +0.0 / +1.6 | 1.00 | 0.98 | 1.00 | pass |
| forklift | lowered-forks | +0.2 / -0.1 / +4.0 | 0.95 | 0.98 | 0.99 | pass |
| heat-pump-outdoor-unit | heating | -1.2 / +0.0 / +1.8 | 0.97 | 1.00 | 0.99 | pass |
| oil-pumpjack | mid-stroke | +0.0 / -0.0 / +0.0 | 0.99 | 0.94 | 1.00 | pass |
| plate-heat-exchanger | full | +0.0 / -2.2 / +0.0 | 0.95 | 0.99 | 1.00 | pass |
| scissor-lift | stowed | +0.0 / -0.1 / +0.0 | 1.00 | 0.96 | 0.97 | pass |
| scroll-compressor | full-load | -3.3 / +0.0 / +0.0 | 0.99 | 1.00 | 1.00 | pass |
| solar-thermal-collector | operating | -0.0 / -0.0 / +0.0 | 0.81 | 1.00 | 1.00 | pass |
| sorption-wheel | rated | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| tower-crane | lifting-load | +0.1 / +0.0 / +0.0 | 0.36 | 0.47 | 0.58 | FAIL |
| wheel-loader | carry | +0.2 / +0.0 / +0.0 | 0.99 | 0.99 | 0.99 | pass |
| dairy-cow | standing-chewing | +0.1 / +0.2 / -0.6 | 0.99 | 0.92 | 0.87 | pass |
| domestic-cat | standing-alert | -0.1 / -0.3 / +1.2 | 0.98 | 0.91 | 0.95 | pass |
| goat | standing-alert | -0.3 / -0.3 / -0.6 | 0.97 | 0.86 | 0.90 | pass |
| hedgehog | foraging | -3.4 / +11.4 / +0.0 | 0.93 | 0.92 | 0.91 | FAIL |
| herring-gull | standing | -0.7 / +1.2 / -3.0 | 0.98 | 0.83 | 0.95 | pass |
| horse | standing-alert | -0.3 / +0.1 / +1.5 | 0.99 | 0.90 | 0.90 | pass |
| labrador-dog | standing-alert | -0.3 / -0.4 / -0.2 | 0.99 | 0.93 | 0.90 | pass |
| mallard-duck | standing | -1.2 / -0.4 / +2.5 | 0.97 | 0.88 | 0.87 | pass |
| pig | standing-pig | -0.0 / +0.2 / +1.3 | 0.99 | 0.91 | 0.93 | pass |
| pigeon | standing-alert | +0.4 / +0.7 / +0.0 | 0.97 | 0.84 | 0.87 | pass |
| red-fox | standing-alert | +0.0 / +0.5 / +0.0 | 0.99 | 0.83 | 0.93 | pass |
| red-squirrel | sitting-eating | -6.4 / +2.2 / +3.0 | 0.98 | 0.92 | 0.97 | pass |
| roe-deer | alert | +0.1 / +0.3 / +2.3 | 0.99 | 0.86 | 0.89 | pass |
| sheep | standing-resting | +0.2 / +0.5 / -0.1 | 0.99 | 0.92 | 0.93 | pass |
| child-on-kick-scooter | gliding | -0.2 / -0.2 / -1.3 | 0.98 | 0.92 | 0.89 | pass |
| courier-with-hand-truck | waiting | -1.0 / +0.4 / +1.7 | 0.98 | 0.95 | 0.95 | pass |
| cyclist | riding-upright | +0.1 / -0.2 / +0.0 | 0.95 | 0.88 | 0.95 | pass |
| elderly-with-cane | stooped-standing | +0.1 / -1.1 / -1.0 | 0.98 | 0.96 | 0.93 | pass |
| jogger | initial-contact-l | -0.0 / -0.2 / +3.4 | 0.98 | 0.89 | 0.87 | pass |
| kneeling-gardener | kneeling-weeding | -0.3 / +0.9 / +0.0 | 0.96 | 0.97 | 0.96 | pass |
| leaning-person | leaning-shoulder-wall | -1.3 / -0.2 / +0.9 | 0.98 | 0.93 | 0.92 | pass |
| parent-carrying-toddler | standing-carry | -0.0 / +0.2 / +0.0 | 0.97 | 0.94 | 0.93 | pass |
| parent-with-stroller | pushing-stroller | -0.2 / -0.6 / +0.3 | 0.97 | 0.91 | 0.89 | pass |
| person-cooking | stirring | -0.1 / -0.3 / +0.0 | 0.98 | 0.91 | 0.95 | pass |
| person-on-crutches | tripod-stand | -0.8 / +0.2 / +0.9 | 0.93 | 0.90 | 0.87 | pass |
| person-with-rollator | stand-at-grips | +0.1 / +0.1 / +0.6 | 0.97 | 0.86 | 0.87 | pass |
| person-with-umbrella | umbrella-up-still | -3.2 / +0.2 / -1.6 | 0.77 | 0.92 | 0.95 | FAIL |
| person-with-white-cane | constant-contact-stance | +0.6 / -0.2 / +0.0 | 0.96 | 0.90 | 0.89 | pass |
| pregnant-person | hand-on-back | +1.1 / +0.2 / +2.7 | 0.96 | 0.94 | 0.92 | pass |
| runner-with-blade-prosthesis | standing-ready | +0.9 / +0.8 / +2.9 | 0.97 | 0.95 | 0.90 | pass |
| running-child | standing-ready | +3.6 / -0.6 / +0.6 | 0.95 | 0.98 | 0.92 | pass |
| seated-guitarist | strumming | -0.5 / +0.4 / -0.5 | 0.92 | 0.87 | 0.95 | pass |
| seated-office-worker | typing | +0.0 / +1.3 / +1.7 | 0.96 | 0.93 | 0.97 | pass |
| seated-reader | seated-upright | -0.2 / +0.2 / +0.0 | 0.96 | 0.98 | 0.99 | pass |
| skateboarder | cruising | -1.5 / +0.0 / -0.9 | 0.97 | 0.91 | 0.90 | pass |
| sleeping-person | supine | -1.3 / -0.0 / +0.0 | 0.98 | 0.95 | 0.97 | pass |
| standing-adult | relaxed | +3.6 / -0.0 / -0.0 | 0.97 | 0.95 | 0.92 | pass |
| standing-phone-user | texting | -0.2 / +0.2 / -0.0 | 0.97 | 0.94 | 0.87 | pass |
| tour-guide | standing-pointing | +0.4 / -0.0 / +1.2 | 0.93 | 0.91 | 0.83 | pass |
| walking-commuter | standing | -0.3 / -0.1 / +1.9 | 0.92 | 0.96 | 0.87 | pass |
| wheelchair-user | seated-level | -0.6 / -0.1 / +0.7 | 0.92 | 0.94 | 0.96 | pass |
| worker-carrying-box | carrying-box-low | +0.0 / +0.8 / +1.2 | 0.96 | 0.97 | 0.95 | pass |
| ceiling-diffuser | design | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| circulation-pump | high | +0.0 / -0.2 / +0.0 | 0.98 | 0.95 | 0.99 | pass |
| decentralised-hrv-fan | extract | +0.0 / +0.0 / +0.6 | 0.89 | 1.00 | 1.00 | pass |
| desktop-monitor | on | -0.5 / -0.0 / +0.0 | 0.98 | 0.98 | 0.95 | pass |
| duct-fan | high | +0.0 / +0.0 / +1.1 | 0.83 | 1.00 | 1.00 | pass |
| electric-fan-heater | heating | +0.2 / +0.0 / +0.8 | 0.95 | 0.99 | 0.99 | pass |
| electric-kettle | boiling | -1.6 / +0.0 / -1.0 | 0.99 | 0.99 | 0.99 | pass |
| electricity-meter | full-load | +0.0 / +0.0 / +0.4 | 1.00 | 1.00 | 0.99 | pass |
| gas-boiler | hot-water | +0.0 / +0.0 / +0.1 | 1.00 | 0.99 | 1.00 | pass |
| gas-hob | high | +0.0 / +0.0 / +0.0 | 0.97 | 0.98 | 1.00 | pass |
| gas-meter | rated-flow | +0.0 / +0.0 / +0.9 | 0.98 | 0.99 | 1.00 | pass |
| hot-water-cylinder | half-charged | +0.0 / +0.0 / +0.4 | 0.99 | 1.00 | 0.96 | pass |
| kitchen-extractor-hood | extracting | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| laptop | open | -0.4 / -0.1 / +0.0 | 0.83 | 1.00 | 1.00 | pass |
| mvhr-unit | normal | +0.0 / -0.5 / +1.0 | 1.00 | 0.99 | 0.99 | pass |
| pendant-lamp | on | +0.0 / +0.0 / -0.7 | 1.00 | 0.99 | 0.99 | pass |
| room-thermostat | calling | +0.0 / -0.0 / +0.0 | 1.00 | 0.95 | 0.96 | pass |
| server-rack | full-load | +0.2 / +0.0 / +1.2 | 1.00 | 1.00 | 1.00 | pass |
| shower-head | running | +2.6 / -0.6 / +2.2 | 0.80 | 0.91 | 0.89 | pass |
| split-ac-indoor-unit | off | +0.2 / +1.1 / +0.0 | 0.99 | 0.98 | 1.00 | pass |
| thermostatic-radiator-valve | open | -1.5 / -0.2 / +5.6 | 0.98 | 0.98 | 0.97 | pass |
| toaster | toasting | +0.0 / +0.3 / +2.1 | 1.00 | 0.98 | 0.98 | pass |
| tower-server | typical | +0.9 / +0.0 / +0.8 | 0.99 | 1.00 | 0.99 | pass |
| vacuum-cleaner | running | +0.1 / +0.2 / +3.0 | 0.99 | 0.98 | 0.98 | pass |
| cargo-bike | loaded-on-stand | -0.3 / +1.7 / +0.0 | 0.79 | 0.88 | 0.95 | FAIL |
| city-bicycle | riding | +0.3 / -0.3 / +0.0 | 0.85 | 0.78 | 0.88 | FAIL |
| city-bus | level-cruise | +0.0 / +0.2 / +1.9 | 0.97 | 0.98 | 0.99 | pass |
| city-car | parked | +2.2 / -0.2 / +1.2 | 0.95 | 0.96 | 0.98 | pass |
| delivery-van | parked | +0.7 / -0.1 / +0.0 | 0.99 | 0.96 | 1.00 | pass |
| electric-kick-scooter | riding-cruise | +0.2 / +0.9 / +0.0 | 0.96 | 0.93 | 0.96 | pass |
| motor-scooter | cruising | +2.8 / -0.1 / +0.0 | 0.94 | 0.91 | 0.94 | pass |
| tram | running | +0.1 / +0.4 / +2.0 | 0.99 | 0.97 | 0.99 | pass |
| vintage-car-1929 | parked | +0.1 / -0.1 / +0.0 | 0.97 | 0.94 | 0.96 | pass |
| bar-stool | unoccupied | +1.2 / +0.0 / +1.2 | 0.99 | 0.99 | 0.99 | pass |
| bookshelf | empty | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| cafe-chair | upright-empty | -0.3 / +0.3 / +0.4 | 0.99 | 0.97 | 0.97 | pass |
| dining-table | closed | +0.0 / +0.0 / +0.0 | 1.00 | 0.98 | 1.00 | pass |
| folding-deck-chair | unfolded | +1.2 / -0.9 / +0.9 | 0.94 | 1.00 | 0.99 | pass |
| height-adjustable-desk | seated-height | +0.0 / +0.0 / +0.0 | 0.96 | 0.97 | 1.00 | pass |
| office-swivel-chair | seat-low | -0.6 / -0.2 / +2.4 | 0.95 | 0.95 | 0.98 | pass |
| operating-table | light-on | -0.0 / +0.0 / +0.0 | 0.99 | 0.99 | 0.99 | pass |
| park-bench | unloaded | +0.0 / +0.0 / -0.2 | 0.99 | 0.99 | 0.94 | pass |
| picnic-table | deployed | +0.0 / +0.0 / +0.0 | 1.00 | 0.97 | 0.99 | pass |
| single-bed | flat | -0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| three-seat-sofa | unoccupied | +0.0 / +0.0 / -0.0 | 0.99 | 0.99 | 0.98 | pass |
| cantilever-balcony | distributed-live-load | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| curtain-wall-bay | dead-load | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| etics-brick-wall-12 | winter-night | +0.0 / +0.0 / +0.4 | 1.00 | 1.00 | 1.00 | pass |
| etics-brick-wall-30 | winter-night | +0.0 / +0.0 / +0.4 | 1.00 | 1.00 | 1.00 | pass |
| external-venetian-blind | lowered-horizontal | +0.0 / +0.0 / +0.0 | 0.91 | 1.00 | 1.00 | pass |
| glass-balustrade | handrail-line-load | +2.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| glulam-beam | distributed-live-load | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| masonry-wall-with-lintel | dead-load | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 0.90 | pass |
| panel-radiator | rated | +0.0 / +0.0 / +0.0 | 0.48 | 1.00 | 1.00 | FAIL |
| sectional-garage-door | closed | +0.1 / +0.4 / +0.0 | 0.41 | 0.98 | 0.94 | FAIL |
| sliding-door | closed | -0.0 / +0.0 / +2.0 | 1.00 | 1.00 | 1.00 | pass |
| solid-brick-wall-1960 | winter-night | +0.0 / +0.0 / +0.4 | 1.00 | 1.00 | 1.00 | pass |
| spiral-stair | self-weight | +0.0 / +0.4 / +0.0 | 0.79 | 0.82 | 1.00 | FAIL |
| stair-flight | crowd-live-load | +0.0 / -0.0 / +0.5 | 1.00 | 1.00 | 1.00 | pass |
| steel-column | eccentric-load | +0.0 / +0.0 / +0.0 | 0.99 | 1.00 | 1.00 | pass |
| tied-canopy | snow-load | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| tilt-window | closed | +0.0 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| wheelchair-ramp | self-weight | +0.2 / +0.0 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
| bicycle-stand | empty-sun | +0.1 / +0.0 / +2.1 | 0.99 | 0.99 | 0.99 | pass |
| chalkboard | leaf-left | +0.0 / +0.0 / +3.1 | 1.00 | 0.99 | 1.00 | pass |
| charcoal-grill | grilling-closed | +1.1 / +0.0 / +0.6 | 0.98 | 0.99 | 1.00 | pass |
| cooking-pot | boiling | +0.0 / +0.0 / +0.0 | 0.65 | 1.00 | 1.00 | FAIL |
| drinking-fountain | drinking | +2.5 / -0.0 / +2.5 | 0.98 | 0.98 | 0.95 | pass |
| market-parasol | open-still | -1.2 / +0.1 / -1.4 | 0.58 | 0.93 | 0.99 | FAIL |
| patio-heater | high | +0.0 / +0.0 / +0.0 | 0.99 | 0.99 | 1.00 | pass |
| playground-swing | rest | +0.0 / -0.0 / +0.0 | 0.99 | 0.93 | 0.99 | pass |
| retractable-bollard | raised | +0.0 / +0.0 / +0.0 | 0.67 | 0.70 | 1.00 | FAIL |
| shopping-trolley | empty | +0.6 / +0.0 / +2.1 | 0.97 | 0.96 | 0.96 | pass |
| stepladder | open-empty | -0.0 / +0.0 / +2.2 | 0.82 | 0.93 | 0.94 | pass |
| storage-crate-stack | high | +0.0 / +0.0 / +0.0 | 0.99 | 0.99 | 1.00 | pass |
| street-lamp | dusk-off | +0.2 / +0.0 / +0.0 | 0.99 | 0.93 | 0.96 | pass |
| tealight | lit | +0.0 / +0.0 / +0.0 | 0.83 | 0.83 | 1.00 | pass |
| waste-bin | closed-empty | +0.8 / +0.3 / +0.0 | 1.00 | 1.00 | 1.00 | pass |
