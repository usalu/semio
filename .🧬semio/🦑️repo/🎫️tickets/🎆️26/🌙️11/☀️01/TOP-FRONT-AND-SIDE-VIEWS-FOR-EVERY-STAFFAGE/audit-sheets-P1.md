# 🔍️ Audit Sheets P1: Top, Front and Side Views (read-only)

Scope: the 11 appliance assets (ceiling-diffuser, circulation-pump, decentralised-hrv-fan, desktop-monitor, duct-fan, electric-fan-heater, electric-kettle, electricity-meter, gas-boiler, gas-hob, gas-meter).

Method: `🧰️QA2/all.ts` rendered 1134 sheets (378 per scale at 1:50, 1:100 and 1:200; every pose, every clip at 3 instants per segment, plus a thermal sheet per asset). `raster.py` produced 1134 PNGs and `montage.py` 171 montages (7 sheets each). All 171 montages were opened with Read.

Sheets read: **171 montages, covering 1134 sheets** (per asset: ceiling-diffuser 12, circulation-pump 18, decentralised-hrv-fan 24, desktop-monitor 12, duct-fan 15, electric-fan-heater 12, electric-kettle 15, electricity-meter 15, gas-boiler 21, gas-hob 15, gas-meter 12).

Driver check output: all 11 assets report `check: ok`; the sweep and consistency lines in the log are empty; no render errors.

## Findings

| Asset | Scale | Pose / instant | View | Defect | Severity |
|---|---|---|---|---|---|
| gas-boiler | 1:200 | heating, heating-cycle@0 and similar | E (elevation) | Title "Wall-Hung Condensing Gas Boiler" and subtitle run past the right edge of the sheet; the title is clipped. Label only, geometry intact. | Low (cosmetic). Cited: 🗑️generated/R-P1/gas-boiler/_m200-0.png |
| gas-hob | 1:200 | flicker@0, flicker@0.05 and similar | E and F | Subtitle "flicker @ 0.00 s" runs past the cell edge. Label only. | Low (cosmetic). Cited: 🗑️generated/R-P1/gas-hob/_m200-0.png |
| gas-hob | 1:100 | light-and-burn@150, @151.5, @285 | F | Bottom scale-bar label ("0 1 m") sits at the edge of the sheet, clipped. Label only. | Low (cosmetic) |
| ceiling-diffuser | 1:50 | first pose sheet | F (front) | Title text overruns the front cell. Label only. | Low (cosmetic) |

No geometry defects found: no detached or floating parts, nothing below ground, all three views (E, F, P) non-empty in every montage, and the moving parts (fan blades, door, valve, dial pointers, burner flame, cycle states) read consistently across the three views at each scale.

## Remaining doubts

- The montage is downscaled (about 420 px per view), so this is a visual audit. Fine detail (1:50 bolts, thin hatches, sub-millimetre gaps) was not measured. Only the orthographic and print-profile checks from the driver measure those.
- The label overflows look like raster-crop artefacts of the montage cell rather than authoring errors. Worth confirming on the original SVG before fixing.
- The thermal sheets (`thermal-*`) show a single pose. They were checked for presence and colour fill only.
