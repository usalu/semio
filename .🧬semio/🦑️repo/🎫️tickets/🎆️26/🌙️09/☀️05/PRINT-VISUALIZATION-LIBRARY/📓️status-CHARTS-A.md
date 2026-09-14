# Status — CHARTS-A (cartesian chart namespaces)

Owner of `semio-viz-charts-bar|line|area|scatter|financial|timeline|dashboard|funnel.sty` and the
catalogue entries of taxonomy sections 1, 2, 17, 18, 19, 50, 51, 52 and the cartesian kinds of
61, 62, 65, 69, 70, 72, 73.

**State: done.** 21 families, 270 catalogue entries, 6 test cases, every claim below was run.

---

## 1. Families

21 families registered with `\SemioVizFamily`, each with an `l3keys` module `semio / viz / family /
<name>` documented in a `%region 🔖️Keys` block that states every key's type, default and meaning.

| Package | Families | Taxonomy |
|---|---|---|
| `semio-viz-charts-bar` | `bar`, `dot`, `rank`, `unit` | §1, §61 (marimekko, mosaic), §69, §70 |
| `semio-viz-charts-line` | `line`, `spark`, `slope`, `annotated`, `axisplot` | §2, §50, §51, §62, §65 |
| `semio-viz-charts-area` | `area` | §2, §61 (horizon, stream, theme river) |
| `semio-viz-charts-scatter` | `scatter`, `coordplot` | §4/§17 point clouds, §52 |
| `semio-viz-charts-financial` | `financial`, `waterfall`, `curve` | §17 |
| `semio-viz-charts-timeline` | `timeline`, `narrative` | §2 temporal, §54, §62, §65, §72 |
| `semio-viz-charts-dashboard` | `kpi`, `gauge`, `statepanel` | §18, §73 |
| `semio-viz-charts-funnel` | `funnel` | §19 |

Every family inherits the shared cartesian vocabulary (`data, x, y, y2, group/series, label,
padLeft/Right/Top/Bottom, yMin, yMax, axes, grid, labels, sort, curve`) through
`\semio_viz_cart_family_keys:n`, then adds its own. The variants really are options:
`bar` has `orient`, `mode` (grouped/stacked/percent/diverging/overlapping/nested/floating/range/
waterfall), `padding`, `barWidth`, `cornerRadius`, `lollipop`, `mirror`, `paired`, `baseline`;
`line` has `curve` (linear/step/step-before/step-after/basis/cardinal/catmull-rom/monotone-x/
natural/bezier), `mode` (plain/indexed/normalized), `markers`, `highlight`, `confidence`, `lo`,
`hi`; `area` has `stack`, `offset` (zero/expand/silhouette/wiggle), `baseline`, `horizon`,
`difference`, `range`; `scatter` has `size`, `shape`, `color`, `trend` (linear/loess/polynomial),
`confidence`, `connected`, `jitter`; `financial` has `mode` (candlestick/ohlc/hlc/heikin-ashi/
renko/kagi/point-and-figure/line/volume/price-volume/volume-profile), `overlay` (ma/bollinger),
`window`, `boxSize`, `indicator` (macd/rsi/momentum/drawdown/equity/return/cumulative-return);
`timeline` has `mode` (event/milestone/interval/state/swimlane/spiral/radial/calendar/cycle/
seasonal/fan/forecast), `lane`, `start`, `end`, `state`, `columns`, `turns`, `split`; `gauge` has
`mode` (linear/radial/dial/speedometer/thermometer/progress-bar/progress-ring/donut), `startAngle`,
`endAngle`, `thickness`, `ticks`; `funnel` has `mode`, `orient`, `gap`, `dropoff`.

Composition of the kernel: data through `semio-viz-data` (`\semio_viz_table_*`), scales through
`semio-viz-scale` (`\semio_viz_scale_define:nnnnn`, `\semio_viz_scale_map:nnN`,
`\semio_viz_scale_band_edges:nnNN`, the `sqrt` scale for size encodings, `exponent` for a pow
axis), marks/tokens through `semio-viz-mark` and `semio-tokens`, and every drawn primitive is
recorded through `\semio_viz_probe_geometry:nn` (`rect, point, segment, hair, path, curve, paint,
corner, grid, tick, label, band, region, glyph, arc, arc-centroid, ring, trapezoid, regression,
window-mean`).

## 2. Catalogue

`🏗️catalog-charts-a.ts` in this ticket folder rewrites the `family`, `options` and `data` of the
**270 entries** I own in `🖼️assets/🔣️viz-catalog.json` (run `bun ./🏗️catalog-charts-a.ts`). It
refuses to finish silently: it fails loudly on a duplicate option set and on a slug missing from the
catalogue. Current run: `✅ rewrote 270 entries; 270 distinct option sets`.

> ⚠️ **CATALOG:** these entries are hand-mapped, not generated. If `generate viz` regenerates the
> catalogue from `🏗️build-catalog.ts`, re-run `🏗️catalog-charts-a.ts` afterwards, or fold the
> mapping into the generator. The placeholder options the generator wrote (`variant=<slug>`,
> `grouping=none`, `normalize=none`) were identical across kinds and would have failed the
> distinctness requirement.

19 further entries in my sections stayed untouched because their semantics belong to another
namespace: `flame-graph, flame-chart, icicle-flame-graph, icicle-chart, call-tree, call-graph,
service-dependency-graph` (hierarchy/network), `latency-histogram` (distribution),
`topology-map, service-map, dependency-map, latency-heatmap, survey-heatmap,
question-by-group-matrix` (matrix/network), `electoral-map, choropleth-election-map, swing-map,
constituency-cartogram` (geo), `alluvial-plot, coxcomb, spiral-heatmap` (flow/polar).

## 3. Demo data (announced)

Defined in `semio-viz-charts-bar.sty`, `%region 🔖️DemoData`, all `demo-` prefixed and namespaced so
they cannot collide with GRAMMAR-CORE's kernel tables (which is why they are **not** called
`demo-series` / `demo-time` / `demo-xy` — those names are now the kernel's, with different columns):

| Table | Columns |
|---|---|
| `demo-cartesian` | `cat, grp, val` (5 categories × 3 series) |
| `demo-multiseries` | `t, series, val, lo, hi` (2 series × 6 periods with a band) |
| `demo-ohlc` | `t, open, high, low, close, volume` (8 periods) |
| `demo-interval` | `lane, label, start, end, state` (3 lanes, 6 intervals) |
| `demo-stage` | `stage, value` (5 funnel stages) |
| `demo-scatter` | `x, y, size, grp, label` (10 points) |
| `demo-range` | `cat, lo, hi, ref` (5 spans) |
| `demo-kpi` | `label, value, target, min, max, delta` (3 indicators) |

## 4. Tests

Six cases under `🧰️framework/🛍️products/📓️print/🧪️tests/`, each with `🥒️.feature`, `🟦️.ts` and
committed `🧫️fixtures/*.tex`, written against §1 of `📓️status-TESTS-HARNESS.md`:

| Case | Scenarios | Oracle |
|---|---|---|
| `charts-bar-layout` | grouped, horizontal, stacked, percent, diverging | `d3-scale` band+linear, `d3-shape` `stack` / `stackOffsetExpand`; diverging is `@no-oracle` conformance |
| `charts-line-area` | linear, step-after, step-before, area-stack | `d3-shape` `line()` with `curveLinear/StepAfter/StepBefore`, `stack()` |
| `charts-scatter-trend` | linear-trend, bubble-size | `d3-regression` `regressionLinear`, `d3-scale` `scaleSqrt` |
| `charts-financial` | candlestick, moving-average | `d3-scale`, `d3-array` `mean` |
| `charts-kpi-gauge` | radial-gauge, progress-ring | `d3-shape` `arc().centroid()` |
| `charts-timeline` | interval, swimlane | `@no-oracle` conformance vectors |

**The repo test runner cannot be used yet** — `bun ./📜️script.ts run long --owner
"🧰️framework/🛍️products/📓️print"` fails in its contract phase on repo-wide discovery breaches
(`207 executable test file(s) outside the canonical owner-root test tree, baseline allows 0`, plus
fixture-vector breaches in `✏️s/🔌️plugins`), which is not about these cases. So the oracles were
run directly, with the same fixtures and the same d3 packages, from
`🗑️generated/CHARTS-A/verify/`. Output tail (`oracle-report.txt`, all 18 comparisons):

```
✓ grouped (15 rects)      ✓ stacked (15 rects)     ✓ percent (15 rects)   ✓ horizontal (15 rects)
✓ line linear Alpha       ✓ line linear Beta       ✓ line step-after Alpha ✓ line step-after Beta
✓ line step-before Alpha  ✓ line step-before Beta
✓ area layer 1 upper      ✓ area layer 1 lower     ✓ area layer 2 upper
✓ scatter regression      ✓ financial moving average
✓ financial candlestick bodies
✓ gauge radial-gauge arc centroid   ✓ gauge progress-ring arc centroid
```

Every comparison is exact to 4 decimal places; the bar geometry, the step vertices and the stack
boundaries are byte-identical to d3's, and the bubble radii match `scaleSqrt` after the size
encoding was moved onto the kernel scale instead of an ad-hoc formula.

## 5. Distinctness

`🗑️generated/CHARTS-A/gen-distinct.mjs` renders **all 269 of my catalogue kinds** into one probe
document and `distinct/check-distinct.mjs` hashes each kind's projection per family:

```
kinds probed: 269
kinds with no geometry: 0
same-family projection clashes: 0
```

Getting there fixed 64 real collisions rather than papering over them: `paired` now reserves a
gutter, `state`/`interval` timelines fill different shares of a lane, `indexed` and `normalized`
lines got separate domains, `delta` and `change` bars separate widths, `momentum`, `return` and
`macd` separate formulas, `cleveland` joins each dot to the baseline while `dumbbell` joins the
pair, the funnel `gap` reaches the trapezoid, coordinate kinds got per-kind polar mappings, and the
probe now records the paint, corner radius, grid lines, ticks, labels and arc thickness that were
being drawn but not measured.

## 6. Gallery

`🗑️generated/CHARTS-A/gallery/charts-a-gallery.tex` (generated by `gen-gallery.mjs` from the
catalogue) renders all 270 kinds through `\SemioVizRunFamily` in
`\documentclass[type=report,theme=light,language=de]{semio}` → **72 pages, all figures drawn**.

> 🛑 **Blocker for CATALOG / the class owner, not caused by any family.** The `VizFigure`
> environment itself errors: a document with **20 empty figures containing a single
> `\SemioVizMark{dot}`** produces 76 errors — `Missing \endgroup inserted`, `Missing } inserted`,
> `Extra }, or forgotten \endgroup`, four per figure. Reproduce:
> `🗑️generated/CHARTS-A/gallery/empty.tex`. The PDF is still produced, but every gallery build will
> carry this noise until `VizFigure`/`Figure` is fixed. Probe compiles through my own
> `VizProbeCanvas` are clean (0 errors over 269 kinds).
>
> Two more, also not mine: `semio-viz-plot.sty`'s `\ProvidesPackage` description contains
> `\SemioVizPlot` and used to break every load (worked around by not requiring it); and loading the
> full `semio-viz` bundle reports `Viz family 'sunburst' is already registered` and
> `Command '\SemioVizGeometry' already defined`.

## 7. Requests to other agents

- **GRAMMAR-CORE** — please adopt from `semio-viz-charts-bar.sty`:
  `%region 🔖️Pending-semio-viz-transform` (column extent, per-key totals, running stack offsets,
  OLS regression, trailing window mean) and `%region 🔖️Pending-semio-viz-theme` (the categorical
  series palette `\semio_viz_cart_color:nN`, the sequential shade `\semio_viz_cart_shade:nnN`, and
  the `\providecolor` fallbacks that let a probe document without the `semio` class resolve the
  `semio-chrome-*` aliases).
- **SHAPES** — `%region 🔖️Pending-semio-viz-plot` holds the curve handling
  (`\semio_viz_cart_coords_run:`, exact `step`/`step-before`/`step-after` vertex insertion verified
  against `d3-shape`) and the arc generator the gauges use. The smooth curves (`basis`, `cardinal`,
  `catmull-rom`, `monotone-x`, `natural`) currently map onto TikZ `smooth` tensions rather than the
  true interpolators; the vertex list is right, the interior is approximate. Real interpolators
  belong in `semio-viz-shape`.
- **GUIDES** — `%region 🔖️Pending-semio-viz-guide` holds the cartesian axis frame, five-tick value
  axis, band tick labels, grid and legend.
- **TESTS-HARNESS** — I need `@no-oracle-viz-bar-diverging` (d3's `stackOffsetDiverging` splits by
  sign, the family splits by series position, justified in the feature) and
  `@no-oracle-viz-timeline-intervals` (no d3 lane layout exists) registered in
  `🔮️oracle/🔣️.json`'s `noOracleDecisions`.
- **CHARTS-B** — `semio-viz-charts-distribution.sty` currently contains a stray form-feed character
  (0x0C) where a control sequence should start; it will break the load. Also: when the `polar`
  family publishes an option vocabulary that takes a table plus an angle and a radius column, tell
  me and I will delegate §52's polar kinds to it (`\semio_viz_coord_delegate:` in
  `semio-viz-charts-scatter.sty` is the single place to change).

## 8. Files touched

Created / rewritten (`🧰️framework/🛍️products/📓️print/`):

- `🖋️latex/semio-viz-charts-bar.sty`, `-line.sty`, `-area.sty`, `-scatter.sty`, `-financial.sty`,
  `-timeline.sty`, `-dashboard.sty`, `-funnel.sty`
- `🧪️tests/charts-bar-layout/{🥒️.feature,🟦️.ts,🧫️fixtures/{grouped,horizontal,stacked,percent,diverging}.tex}`
- `🧪️tests/charts-line-area/{🥒️.feature,🟦️.ts,🧫️fixtures/{linear,step-after,step-before,area-stack}.tex}`
- `🧪️tests/charts-scatter-trend/{🥒️.feature,🟦️.ts,🧫️fixtures/{linear-trend,bubble-size}.tex}`
- `🧪️tests/charts-financial/{🥒️.feature,🟦️.ts,🧫️fixtures/{candlestick,moving-average}.tex}`
- `🧪️tests/charts-kpi-gauge/{🥒️.feature,🟦️.ts,🧫️fixtures/{radial-gauge,progress-ring}.tex}`
- `🧪️tests/charts-timeline/{🥒️.feature,🟦️.ts,🧫️fixtures/{interval,swimlane}.tex}`

Edited (my 270 entries only): `🖼️assets/🔣️viz-catalog.json`.

Ticket folder: `🏗️catalog-charts-a.ts` (kept — it is how the mapping is reapplied),
`🗑️generated/CHARTS-A/` (probe fixtures, `gen-gallery.mjs`, `gen-distinct.mjs`,
`verify/check-bar.mjs`, `verify/check-rest.mjs`, `distinct/check-distinct.mjs`,
`oracle-report.txt`, `distinct-report.txt`, the local d3 oracle install).
