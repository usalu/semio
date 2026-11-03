# Audit O12: Visual Re-check of Three Views after the Single 3D Placement Path

Scope: O1 (bicycle-stand, chalkboard, charcoal-grill, cooking-pot, drinking-fountain, market-parasol, patio-heater, playground-swing) and O2 (retractable-bollard, shopping-trolley, stepladder, storage-crate-stack, street-lamp, tealight, waste-bin). Read-only; no catalog or source file edited.

Render: `🗑️generated/R-O12/render.ts`, the preview CLI (`runPreview`, the O1 batch and O2 all.ts mechanism) with `--views --no-png`, SVG converted to PNG by sharp at 150 dpi. Output confined to `🗑️generated/R-O12`, a folder that did not exist before the run. One bun process, Windows-form paths.

Coverage: 528 three-view sheets, every authored scale (50/100/200, plus 500 where authored), every pose, and three instants per clip (0, ½, 1 of duration). No render failures. Sheets were read as 56 contact sheets (one per asset and scale, two instants per row), so reading was at reduced resolution, not zoomed on each PNG. Dimension labels cited below were confirmed against the SVG text.

## Findings

| Asset | Scale | Pose / instant | View | Finding |
|---|---|---|---|---|
| playground-swing | 1:50 | occupant-push@0, rest and all instants | plan | Plan depth label 1.27 m |
| playground-swing | 1:100, 1:200, 1:500 | same | plan | Plan depth label 1.24 m (1:100 confirmed in SVG). A 3 cm scale-dependent drift for the same pose. Suggests the front/plan extent differs with scale. Needs a follow-up. |
| playground-swing | all | pendulum-forward | elevation | Chain and seat not visible in side view (already recorded in the O1 report) |
| stepladder | 1:50, 1:200 | all instants | elevation, front | Height label 1.51 m |
| stepladder | 1:100 | all instants | elevation, front | Height label 1.50 m |
| stepladder | 1:500 | all instants | elevation, front | Height label 1.49 m. A 2 cm drift across scales. |
| stepladder | 1:50 to 1:500 | all | plan | Plan depth 0.47 m, 0.48 m at 1:500 (confirmed in SVG) |
| shopping-trolley | 1:500 | all instants | elevation, front | Height label 0.99 m against 1.00 m at 1:50 to 1:200 (confirmed in SVG). Rounding-level drift. |
| tealight | 1:50 to 1:200 | all, flicker and burn instants | elevation, front | Candle reads as a hairline; wick and flame small. Plan flame is a dot. Same as the O2 audit. |
| retractable-bollard | all | raised, lowered, moving-up, nudge and raise-and-lower instants | all | Raise heights (1.26, 1.58, 1.90 m) agree across elevation, front and plan. Plan shows the rise as the section only, as expected. |
| waste-bin | 1:200, 1:500 | lid-open | elevation, front | Lid-open height label 1.50 m at 1:200 against 1.49 m at 1:500 |
| charcoal-grill | all | lid open and closed, lid-lift cycle | all | Lid, handle and body present in all three views. Heights 0.97 m and 1.37 m agree across views. |
| cooking-pot | 1:50 to 1:200 | cold, heating, boiling | all | Handles and water surface present. Top-view outline at 1:200 looked less round than at 1:50; not confirmed at full resolution. |
| drinking-fountain | all | idle, drinking | all | Height 1.00 m and plan diameter 0.40 m agree. Plan shows the basin, not the jet (as designed). |
| market-parasol | all | open-still, sun-tilt, wind-flap, tilted-to-sun, furled | all | Canopy tilt visible in side view; eight spokes in plan. Furled plan extent 2.09 m consistent across scales. |
| patio-heater | all | off, low, high, high-in-breeze, ignite and turn-down clips | all | Height 2.25 m and plan ring 0.80 m agree at every scale; burner and base attached. |
| bicycle-stand | all | bike-leaning, lock-pulled, clips | all | Not individually flagged; sheets consistent with the O1 report. |
| chalkboard | all | leaf-left, leaf-middle, leaf-right, clips | all | Not individually flagged; leaves attached and extents consistent with the depth. |
| storage-crate-stack | all | high, half, low, load and unload clips | all | Stack heights 0.62, 1.06, 1.11, 1.50 m agree across views; plan shows top crate only (as designed). |
| street-lamp | all | lean and pole-sway poses, dusk, day and night poses | all | Lean visible in side and plan views; front view untilted (as the coordinate contract requires). Height 6.00 m at every scale. |

## Checked with no defect found

- All three views present and non-empty in every sheet (528 of 528).
- No part displaced or detached by the new carry: lids, chains, handles, ribs, rings and seats remain attached in every pose and instant I read.
- Ground lines sit at the base in every view; nothing floats or sinks.
- Moving parts agree across views for the bollard raise, trolley gate and nesting swing, stepladder spread, storage stack height, lamp lean and heater height.

## Limits

- Contact sheets at reduced size. Small parts (wicks, chain links, handle detail) were read at low resolution, so any finding below the sheet's pixel size is unverified.
- Four drifts are confirmed from the SVG labels. The cooking-pot 1:200 plan outline is not.

## Verdict

**Needs follow-up (minor).** No visual displacement or missing part from the single 3D placement path across the 15 objects, scales, poses and instants. Scale-dependent dimension drift is present: playground-swing plan depth (3 cm between 1:50 and 1:100), stepladder height and plan depth (2 cm), shopping-trolley height (1 cm at 1:500). These need a check of whether the front/plan extents still follow the authored geometry at each scale. The pendulum-forward chain gap in the playground swing side view was already recorded in the O1 report and remains open.

## Kept evidence (in `🗑️generated/R-O12-cited`)

- `🗑️generated/R-O12-cited/playground-swing-1-50.png`
- `🗑️generated/R-O12-cited/playground-swing-1-100.png`
- `🗑️generated/R-O12-cited/stepladder-1-500.png`
- `🗑️generated/R-O12-cited/shopping-trolley-1-500.png`
- `🗑️generated/R-O12-cited/tealight-1-50.png`

Scripts kept in `🗑️generated/R-O12`: `render.ts` (render) and `montage.ts` (contact sheets). Render SVG and PNG outputs and logs were deleted.
