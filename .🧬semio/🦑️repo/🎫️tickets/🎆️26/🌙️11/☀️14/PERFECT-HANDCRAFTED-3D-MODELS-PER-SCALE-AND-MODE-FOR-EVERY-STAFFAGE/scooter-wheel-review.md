# Child on a kick scooter

Both original wheels had a 145 mm diameter and a 560 mm wheelbase. The 1:500 tread measured 72 mm wide against its specified 30 mm. A central fork plate cut through the front tyre. At 1:200 and 1:500 the uncut deck crossed the rear wheel. At 1:100, 1:200 and 1:500 both axles were missing.

The digital model now retains the rim, six spokes, hub, axle and 30 mm tread at every scale. The fork crown bridges two blades above the wheel. The rear deck wraps around a slot at the two coarsest scales. Both ends have 4.5 mm of nominal side clearance, with the stub axles seated in both sides of each dropout. The slotted deck leaves 9.5 mm ahead of the rear tread on the centreline. The wheel hardware follows its thermal and structural references. The model identifies the broad front-wheel mark in the coarse plan as the combined wheel and fork.

The roller support symbols now have the 12 × 20 mm plan footprint specified at the contact anchors. Their underside lies on the road, including at 1:500, where the previous symbol was 76 × 80 mm.

The [wheel fixture](scooter-wheel-case.json) and [Three.js mesh check](scooter-wheel-check.ts) verify the wheel diameter and width, wheel and axle bindings, 32 running-space samples on each of three tyre planes, and contact on both sides of every axle. The 47 poses across four scales give 376 wheel-to-road comparisons. The greatest distance between the tessellated tread and the road was 0.398 mm. The document validator, 52 volume checks and the independent twelve-scene GLB loader passed. The export is 4,627,004 bytes.

The [author](scooter-wheel-fit.py) reconstructs the candidate from the catalog source. The [inspection renderer](scooter-wheel-sheet.ts) makes side and overhead wheel-train views for body, thermal regions and structural members at every scale.
