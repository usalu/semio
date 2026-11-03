# Audit M2: Visual Check of the Eight Machinery Assets

Read-only audit. No asset or source file was changed.

## Method

- Driver: `T/🧰️QA6/all-r-m2.ts`, a copy of `all.ts` in the ticket folder. Only the output folder differs (`T/🗑️generated/R-M2`). The copy was needed because the original hardcodes `QA6`.
- Run: one `bun` process from `🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript`. It produced 294 `--views` sheets (dark-line and thermal), 0 preview failures.
- Rasterised with `T/🧰️QA6/raster.py` and stacked with the same script into 94 montages (3 sheets per montage at 1:50 and 1:100, 5 at 1:200 and 1:500).
- Every one of the 94 montages was opened with Read. Montages are downscaled, so fine detail at 1:50 was judged at montage size only.

## Sheets read

**94 montages read** (covering all 294 sheets), per asset and scale:

| Asset | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| oil-pumpjack | 4 | 4 | 2 | 2 |
| plate-heat-exchanger | 4 | 4 | 2 | - |
| scissor-lift | 4 | 4 | 3 | - |
| scroll-compressor | 4 | 4 | 2 | - |
| solar-thermal-collector | 5 | 5 | 3 | - |
| sorption-wheel | 4 | 4 | 2 | - |
| tower-crane | 4 | 4 | 3 | 3 |
| wheel-loader | 4 | 4 | 3 | 3 |

Scales authored: pumpjack 50/100/200/500, tower-crane and wheel-loader 50/100/200/500, the others 50/100/200 only.

## Findings

| Asset | Scale | Pose / instant | View | Defect | Severity |
|---|---|---|---|---|---|
| wheel-loader | 1:50, 1:100, 1:200, 1:500 | start-and-lift@5 (also raising, load-cycle@6, start-and-lift@9 at 1:50 and 1:500) | front | The bucket is a horizontal box high above the body, with no visible lift arms in the front view. It reads as floating and detached. | moderate |
| wheel-loader | 1:50 | start-and-lift@5 | front | Same as above, at full detail. | moderate |
| scissor-lift | 1:50 | rest / lift poses | front | Arms appear as small separate rectangles beside the platform. They may read as detached parts. Not verified at higher zoom. | low (to verify) |

Checked and found correct at montage size (no defect seen):
- Three views present and non-empty in every montage.
- Scroll-compressor, sorption-wheel and solar-thermal-collector: front and plan follow the elevation in every pose and clip instant. Moving parts (spin, stagnation, start-up, rotor) move consistently. Nothing below ground.
- Tower-crane: slew pose `parked-weathervane` turns the jib in plan and front as designed. Load and hook travel with the trolley in all three views.
- Wheel-loader: the body, wheels, bucket and arms agree between views in carry, dig, dump-high, load-cycle, parked-off and raising. Wheels stay on the ground line.
- Thermal sheets: regions and colours match the dark-line views on all eight assets.

## Remaining doubts

- The scissor-lift front arms need a zoomed check at 1:50 before the low finding can be confirmed or closed.
- The bucket-to-arm connection in the wheel-loader front view is the main open point. It is a readability issue, not a missing view.
- Montages are downscaled, so 1:50 detail (small hatches, bolt rings) was not inspected individually.
- Plan views of scissor-lift arms are hidden by design (noted in QA6), so they are not counted as defects.

## Cleanup

- `T/🗑️generated/R-M2` deleted except the cited PNG montage `wheel-loader/m50-1.png` (start-and-lift@5 front at 1:50, bucket detached), `wheel-loader/m200-1.png` (1:200) and `wheel-loader/m500-1.png` (1:500).
- One side effect to note: the first run wrote its output to `T/🗑️generated/QA6` (the `sed` path edit did not apply). That folder did not exist before the run. Its contents were moved into `R-M2` and the empty folder removed.
