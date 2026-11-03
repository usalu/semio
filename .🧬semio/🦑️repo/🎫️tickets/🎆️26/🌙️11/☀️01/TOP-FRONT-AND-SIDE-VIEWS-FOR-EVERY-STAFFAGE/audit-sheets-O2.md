# Audit O2: Visual Check of Three Views for the Seven Staffage Objects

Scope: retractable-bollard, shopping-trolley, stepladder, storage-crate-stack, street-lamp, tealight, waste-bin.
Render: `🧰️QA10/all.ts` with output `🗑️generated/R-O2/`. Check and consistency output: 28 of 28 authored asset-scale pairs `ok`, no problems reported by `check.txt`.
Rasterise: `raster.py` (150 dpi). Montage: `montage.py`, 5 per sheet, one sheet per asset, scale and five-item group.
Sheets read: 56 montage sheets (every sheet in `🗑️generated/R-O2/_m`, each opened with Read).
Limit: each review was at montage resolution, not zoomed on the individual PNGs.

## Findings

| Asset | Scale | Pose / instant | View | Defect | Severity |
|---|---|---|---|---|---|
| all seven with authored 1:500 (bollard, trolley, stepladder, street-lamp, waste-bin) | 1:500 | all poses and clip instants | elevation, front, plan | Dimension leaders and ticks run across the glyph, so the label and the object overlap | medium |
| retractable-bollard | 1:500 | all | all | Glyph is about 3 mm; the bollard, lid and bracket detail is lost | low |
| retractable-bollard, shopping-trolley, stepladder, waste-bin | 1:200 | all | plan | Plan dimension leader crosses the plan glyph | low |
| tealight | 1:50, 1:100 | all, flicker and burn instants | elevation, front | Candle reads as a hairline on a long ground line; flame and wick are barely visible | low |
| tealight | 1:50, 1:100, 1:200 | all | plan | Flame is a single dot | low |
| waste-bin | 1:50 | lid-open, lid-open-close | plan | Open lid appears as a second outline around the body; reads correctly but is busy | low |

Checked with no defect found:
- All three views present and non-empty in every sheet.
- Nothing floating or detached, and nothing below ground. Ground lines sit at y = 0 in every view.
- Moving parts agree across views: bollard raise height (1.26 to 1.90 m), trolley gate and nesting swing, stepladder spread, storage stack height (0.62 to 1.50 m), lamp head and arm, waste-bin lid.
- Street lamp lean (pole-sway, lit-in-wind) is forward-only and therefore absent from the front view, as the coordinate contract requires; plan and elevation agree.
- Scale detail is sensible at 1:50 and 1:100.

## Kept evidence (PNG cited above)
- `🗑️generated/R-O2/_m/retractable-bollard-500-0.png`
- `🗑️generated/R-O2/_m/shopping-trolley-500-0.png`
- `🗑️generated/R-O2/_m/stepladder-500-0.png`
- `🗑️generated/R-O2/_m/street-lamp-500-0.png`
- `🗑️generated/R-O2/_m/waste-bin-500-0.png`
- `🗑️generated/R-O2/_m/tealight-50-0.png`
