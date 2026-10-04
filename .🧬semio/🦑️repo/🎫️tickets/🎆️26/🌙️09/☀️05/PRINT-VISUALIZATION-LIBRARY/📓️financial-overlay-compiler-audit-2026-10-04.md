# Financial Overlay Compiler Audit — 2026-10-04

The actual neutral native baseline failed at moving-average-chart with `Invalid operation fp_to_decimal(nan)` before the Bollinger case. Existing finance passed an unevaluated arithmetic expression into the canonical numeric scale guard. The fixed financial owner evaluates the numeric mapping boundary. Its earlier Bollinger implementation used constant offsets ±1.4; it now derives trailing population standard deviation and multiplies it by the owned nonnegative deviation option (default 2).

Schema was updated first by root: financial.window has minimum 1 and financial.deviation is a number with minimum 0/default 2 and bilingual descriptions. The existing native financial owner rejects invalid values, retains means, deviations, upper/lower values in dedicated local sequences, maps each value to its actual authored period, and expands the price scale to contain both bands. General cartesian, scale, and statistical owners are unchanged.

The first successful numerical implementation replay terminated 0 with 50 actual records across 7 neutral cases. The current strengthened replay, session 19277, terminated 0. It uses explicit light/dark theme selection and passed all 88 records including actual mapped x/y positions against D3 scaleLinear. Independent D3 mean and variance provide the population deviation (`sample variance * (n-1)/n`). Cases include stock input, windows 1, 2, 4 and 20, and multipliers 0, 1.5 and 2. Early native implementation runs caught and corrected integer iterator rebinding and an unavailable clist variant; failed runs are preserved as failed diagnostics.

The separate focused stock compiler replay, session 42104, terminated 0 and compiled all 39 authored stock kinds in taxonomy 17 in explicit light and dark themes. Both numerical and stock helpers are called by the existing registered full native runner, so no additional production executable or runtime library is introduced.

Exact source inventory, rooted at `🧰️framework/🛍️products/📓️print/`:

- `🖋️latex/semio-viz-charts-financial.sty`
- `🧪️tests/🧬️native-chart-grammar/💹️financial/🔣️.json`
- `🧪️tests/🧬️native-chart-grammar/💹️financial/🟦️.ts`
- `🧪️tests/🧬️native-chart-grammar/🟦️.ts`

Root separately owns `🧬️schema/🔣️.json` metadata. Temporary implementation probes and logs remain inside `🗑️generated` until ticket cleanup. Current final log paths are `financial-native-green5.log` and `financial-native-stock-final.log`; both the numerical and stock smoke gates are verified terminal 0. Poppler page 3 of the current light numerical PDF was inspected and shows the varying-width stock Bollinger band, mean and price lines within the frame. Raster: `🗑️generated/legend-visual-qa/financial-current-03.png`.
The registered current strict build terminated 1 in 1 minute 18 seconds. Canonical inference passed; the native-runner branch failed only on the missing private SunCalc test declaration in the concurrently owned family helper (TS7016, line 8). No financial or legend source typing errors were reported. The solar owner is correcting that oracle boundary before the combined strict replay. Log: `🗑️generated/legend-finance-current-strict.log`.

Current registered strict after the private SunCalc oracle boundary repair: session 45280 terminated 0, both canonical/browser and native-runner checks passed, 250 exported symbols, 1 minute 10 seconds. Log: `🗑️generated/legend-finance-current-strict-after.log`. This includes the latest finance test owner and title/label line-box implementation.

Final canonical replay after the axis role/default corrections terminated 0: exhaustive 630 / 630 checks (171 render checks) in 31.7 seconds, and strict canonical/browser plus native runner/helpers with 250 exports in 1 minute 26 seconds. These supersede the earlier successful 1 minute 10 second strict gate as current combined TypeScript closure. Logs are `🗑️generated/canonical-current-final-exhaustive.log` and `🗑️generated/axis-role-current-strict.log`. The full current native/Rust compiler gate remains held for the concurrent final shared-family freeze; focused native results above remain valid evidence for their recorded source versions.

Latest registered canonical closure after the final legend foreground and shared text-opacity correction: strict session 49810 terminated 0 in 1 minute 37 seconds, checking canonical browser/worker inference and native runner/helpers with 250 exports. Exhaustive session 52999 terminated 0 with 633 / 633 checks in 39.4 seconds, task cache explicitly skipped. Logs: `🗑️generated/canonical-final13-strict.log` and `🗑️generated/canonical-final13-exhaustive.log`. These supersede the prior 630-check/1 minute 26 second gates as current TypeScript proof. The actual focused axis opacity compiler and independent PDF label oracle also terminated 0 after the correction; all 30 paired label colors/opacities passed.
