# Thermal breath markers

Exact-file validation found undersized marker solids for goat, pig, sheep and pigeon. The 1:100 goat and pig marker lists also duplicated those of the finer representation.

The affected representations use grouped markers where the printable figure no longer resolves both nostrils separately. The fine mammal scales retain distinct ellipsoids and the coarser scales use the scale-specific extruded profiles. At the coarser affected goat and sheep scales, the nose and mouth share a single muzzle marker whose tags retain both respiratory outlets. Pigeon keeps its paired nostril location through a centrally grouped cere marker at the affected scales.

The edits affect marker geometry only. The physical respiration inputs, emitter diameters and anchors, body solids and exposed thermal surfaces retain their authored values. Exact-file catalog validation checked all four assets and reported zero failures.

## Pigeon in the 10:18 release build

The browser's unavailable card came from an embedded Pigeon payload with an `extent-front-height` error. The source had subsequently changed both feet in the 1:200 front view from short open strokes to filled outlines reaching the ground. The actual placement evaluator measured 0.207274 m for the embedded payload and 0.220584 m for the updated source against a declared height of 0.225 m. The updated source passed the Vite payload validator with zero issues. It also passed print suitability at 1:150, 1:200, 1:250 and 1:350. No additional animal geometry or breath-marker policy change was needed for this issue.
