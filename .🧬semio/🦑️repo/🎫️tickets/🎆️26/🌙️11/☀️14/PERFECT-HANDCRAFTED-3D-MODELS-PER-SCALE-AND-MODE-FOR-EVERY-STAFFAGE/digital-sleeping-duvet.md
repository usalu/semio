# Sleeping person and fitted bedding

## Source and physical observations

On 21 November 2026 the four-scale review used the supine, side-lying and curled poses. The original catalog SHA-256 was `cc64e3d44913c78572e5a2283ca8db4c5e15cc2b5ae3245dcf2f8fe1ebbadcad`. In the original 1:500 model, rays through the side-lying and curled bodies found bedding above only 3.76% and about 8.1% of the occupied sample locations, respectively. The 1:50 curled model also left 4.3% of those locations uncovered. At the empty foot of the bed, its blanket rose 186 mm above the mattress.

The 1.21 m quilt has a 52 mm filled section, a sewn top and lining, and two bound sides that hang below the mattress top. Its 112 panels share edges and move with the sleeper's trunk, pelvis, knees and calves. When the lower legs draw back, the foot end drops onto the mattress. Four pads register the actual support at the side hems. The thermal surface follows the cloth. The upper pyjama fabric and the duvet over the chest are separate thermal surfaces. At 1:100–1:500 the previously omitted left sleeve now follows the upper arm and forearm beside the quilt. The foot-end contact temperature is 29 °C with legs extended or side-lying and 24 °C with the legs drawn up.

The published catalog SHA-256 is `d83c849d3ac6a66d3297f0f0afd80e46e64ee335a51295f9e0db3c32cc77ccbd`. The installer rebuilt the validated candidate and updated the catalog and editable volume source. Native document validation and all 52 scale checks passed.

## Independent geometric checks

The Three.js fixture casts vertical rays through the full 1:50 tissue mesh of the torso, pelvis, thighs and shanks in each pose. Every denominated cover is evaluated against that same tissue reference. This keeps a missing body part at a coarse scale from artificially improving a coverage measurement. The coverage threshold allows 4 mm for textile intersections. The near-body fraction counts points up to 110 mm above the tissue. Separate downward rays test for holes in the quilt's projected footprint.

| Pose | Covered tissue at all four scales | Near-body fraction | Holes | Maximum empty foot height |
|---|---:|---:|---:|---:|
| Supine | 100% | 99.7% | 0 | — |
| Side-lying | 100% | 87.3–88.3% | 0 | — |
| Curled | 100% | 90.5% | 0 | 70.3 mm |

At 1:500 four additional frames during the turn at 2.625, 3.375, 5.375 and 5.8125 seconds also covered every sampled tissue point with zero holes. An independent world-space corner trace measured maximum separations of 0.055, 0.064 and 0.064 mm in the three resting poses and 2.000 mm over 97 evenly spaced turnover samples. Another check projected each panel's three weighted corners into the native front, elevation and plan drawings at all four scales.

The geometric fixtures are `digital-sleeping-duvet-case.json`, `digital-sleeping-duvet-check.ts`, `digital-duvet-seams.ts` and `digital-duvet-paper.ts`. The asset-specific cloth author is `digital-sleeping-duvet.ts`. The side and knee folds still show individual facets in the shaded views, especially along the steepest ridges.

## Structural export

The previous static exporter allocated a moment arc for each unloaded cloth control and hid it by scaling it down. The finest structural scene then contained 825,190 triangles, exceeding the existing 400,000-triangle ceiling. The frame builder now creates the arc on its first visible frame. A registered arc still vanishes when the joint unloads. The native Gherkin and Vitest case reproduced the old behavior and checks activation and release against Three.js. All 14 model-look tests and all 43 native model-digest cases passed. The regression fixtures include six static sample-figure GLBs and 36 FBX exports covering three clips, four scales and three views. The GLBs received native and independent glTF readback; the FBXs were independently opened by Three.js and Assimp. The cloth package contains twelve validated scenes and occupies 6,103,776 bytes under the original 8 MiB limit.
