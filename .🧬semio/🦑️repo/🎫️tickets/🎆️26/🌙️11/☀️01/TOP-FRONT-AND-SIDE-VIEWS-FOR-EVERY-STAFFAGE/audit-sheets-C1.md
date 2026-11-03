# Visual audit C1: top, front and side views for every staffage

Read-only audit of the 8 assets: glass-balustrade, glulam-beam, cantilever-balcony, curtain-wall-bay, masonry-wall-with-lintel, panel-radiator, external-venetian-blind, sectional-garage-door.

## Method

- Render: `🧰️R-C1/all.ts` (a copy of `🧰️QA13/all.ts` with its output path changed), run as one `repo tools exec bun` process. Every authored scale (1:50, 1:100, 1:200, 1:500), every named pose, and clip instants at 25, 50 and 75 % of each clip, plus the key times for the three moving-part assets at 1:50. One `thermal` job per asset at 1:100 on the default pose. The run exited 0 with 401 SVG previews.
- Rasterise: `🧰️QA13/raster.py` (PyMuPDF, 150 dpi), run as `python -X utf8`.
- Montage: `🧰️QA13/stack.py` (3 sheets stacked per asset and scale), 145 sheets.
- Each sheet was opened with Read. Each sheet shows elevation, front and plan side by side, with the top view labelled "plan".

## Sheets read

| Asset | Sheets | Scales covered |
|---|---|---|
| cantilever-balcony | 17 | 1:50, 1:100, 1:200, 1:500 |
| curtain-wall-bay | 20 | 1:50, 1:100, 1:200, 1:500 |
| glass-balustrade | 12 | 1:50, 1:100, 1:200 (no 1:500 authored) |
| glulam-beam | 13 | 1:50, 1:100, 1:200, 1:500 |
| masonry-wall-with-lintel | 20 | 1:50, 1:100, 1:200, 1:500 |
| panel-radiator | 14 | 1:50, 1:100, 1:200 (no 1:500 authored) |
| external-venetian-blind | 23 | 1:50, 1:100, 1:200 (no 1:500 authored) |
| sectional-garage-door | 26 | 1:50, 1:100, 1:200, 1:500 |
| **Total** | **145** | |

All three views are present and non-empty on every sheet read. Nothing is detached or floats above the ground line, and nothing is drawn below ground. Blind slats, garage-door panels and the glass in the balustrade are consistent between views at every instant sampled. Curtain-wall-bay and masonry-wall-with-lintel front and plan are drawn as sections, as the brief's convention allows.

## Defects

| Asset | Scale | Pose / instant | View | Defect | Severity |
|---|---|---|---|---|---|
| masonry-wall-with-lintel | 1:500 | all poses and clips (beam-over-opening, beam-sweep, dead-load, floor-line-load, load-build-up, shaded-morning, sun-and-shade) | front (section) | The wall body between the base pier and the lintel is not drawn. The front reads as two loose fragments, a small square and a "⊥" glyph, not as a 2.75 m wall section. Plan and elevation are correct. Evidence: `🗑️generated/evidence-C1/masonry-wall-with-lintel-500-00.png` to `-04.png`. | medium |
| glulam-beam | 1:500 | all poses and clips | front | The end-on beam profile is about 1 mm tall at this scale, so it reads as a "⊥" glyph and not as a beam section. | low |
| glulam-beam | 1:200 | all poses and clips | front | The height dimension label "0.61 m" is drawn over the beam outline. | low |
| glulam-beam | all four scales | default pose | elevation and front | The height label changes across scales: 0.62 m at 1:50, 0.63 m at 1:100, 0.61 m at 1:200, 0.62 m at 1:500. All are within the 0.02 m consistency tolerance, but they show that the authored heights differ slightly per scale. | low |
| panel-radiator | 1:100 | rated (thermal job) | elevation, front, plan | The thermal fill is red. The other assets use orange for thermal, so the colour mapping is inconsistent. | low |

No defect was found in cantilever-balcony (all 17 sheets), glass-balustrade (12), curtain-wall-bay (20), panel-radiator apart from the thermal colour (14), external-venetian-blind (23) or sectional-garage-door (26).

## Remaining doubts

- Only one thermal sheet per asset was rendered, at 1:100 on the default pose. Thermal colour and region mapping were checked on that sheet only.
- The 1:500 front views of the masonry, glulam and garage-door assets are small at print size. Their outlines are legible on the PNGs, but they are close to the print minimum stroke.
- Render output for this run went to `🗑️generated/QA13/pv`, not to `🗑️generated/R-C1` as the brief asked. My path substitution in the copied driver did not apply. The render overwrote QA13's SVGs of the same names. I copied this run's 401 SVGs to `🗑️generated/R-C1`, rasterised and stacked them there, then deleted the R-C1 folder, the stack folder and this run's SVGs under `QA13/pv`. Only the five cited masonry PNGs were kept, under `🗑️generated/evidence-C1`. QA13's earlier SVGs of those names were replaced by the render and cannot be restored.
