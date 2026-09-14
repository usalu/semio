/** 🎯 Rewrites the `family`, `options` and `data` of the catalogue entries CHARTS-A owns
 * (taxonomy sections 1, 2, 17, 18, 19, 50, 51, 52 and the cartesian kinds of 61, 62, 65, 69, 70,
 * 72, 73) so that every kind resolves to a real family of `semio-viz-charts-*` and no two kinds of
 * one family share an option set. Every other entry is left untouched.
 * Run: `bun ./🏗️catalog-charts-a.ts` from this ticket folder.
 */
import { readFileSync, writeFileSync } from "node:fs";

//#region 🔖️Mapping
type Spec = { family: string; options: Record<string, string | number | boolean>; data: string };

const D_CAT = "demo-cartesian";
const D_SER = "demo-multiseries";
const D_OHLC = "demo-ohlc";
const D_INT = "demo-interval";
const D_STAGE = "demo-stage";
const D_XY = "demo-scatter";
const D_RANGE = "demo-range";
const D_KPI = "demo-kpi";

const bar = (options: Record<string, string | number | boolean>, data = D_CAT): Spec =>
  ({ family: "bar", options, data });
const dot = (options: Record<string, string | number | boolean>, data = D_CAT): Spec =>
  ({ family: "dot", options, data });
const rank = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "rank", options, data: D_SER });
const unit = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "unit", options, data: "demo" });
const line = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "line", options, data: D_SER });
const spark = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "spark", options, data: "demo" });
const slope = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "slope", options, data: D_SER });
const area = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "area", options, data: D_SER });
const scatter = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "scatter", options, data: D_XY });
const financial = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "financial", options, data: D_OHLC });
const waterfall = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "waterfall", options, data: "demo" });
const curve = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "curve", options, data: "demo" });
const timeline = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "timeline", options, data: D_INT });
const narrative = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "narrative", options, data: D_INT });
const kpi = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "kpi", options, data: D_KPI });
const gauge = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "gauge", options, data: D_KPI });
const statepanel = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "statepanel", options, data: D_SER });
const funnel = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "funnel", options, data: D_STAGE });
const annotated = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "annotated", options, data: D_SER });
const axisplot = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "axisplot", options, data: D_XY });
const coordplot = (options: Record<string, string | number | boolean>): Spec =>
  ({ family: "coordplot", options, data: D_XY });

const SPECS: Record<string, Spec> = {
  // §1 bar charts
  "vertical-bar-chart": bar({ orient: "vertical", mode: "grouped" }),
  "horizontal-bar-chart": bar({ orient: "horizontal", mode: "grouped" }),
  "column-chart": bar({ orient: "vertical", mode: "grouped", padding: 0.35 }),
  "grouped-bar-chart": bar({ orient: "horizontal", mode: "grouped", group: "grp" }),
  "grouped-column-chart": bar({ orient: "vertical", mode: "grouped", group: "grp" }),
  "clustered-bar-chart": bar({ orient: "vertical", mode: "grouped", group: "grp", padding: 0.1 }),
  "stacked-bar-chart": bar({ orient: "horizontal", mode: "stacked", group: "grp" }),
  "stacked-column-chart": bar({ orient: "vertical", mode: "stacked", group: "grp" }),
  "100percent-stacked-bar-chart": bar({ orient: "horizontal", mode: "percent", group: "grp" }),
  "100percent-stacked-column-chart": bar({ orient: "vertical", mode: "percent", group: "grp" }),
  "diverging-stacked-bar-chart": bar({ orient: "horizontal", mode: "diverging", group: "grp" }),
  "floating-bar-chart": bar({ orient: "horizontal", mode: "floating", y: "lo", y2: "hi" }, D_RANGE),
  "floating-column-chart": bar({ orient: "vertical", mode: "floating", y: "lo", y2: "hi" }, D_RANGE),
  "range-bar-chart": bar({ orient: "horizontal", mode: "range", y: "lo", y2: "hi" }, D_RANGE),
  "range-column-chart": bar({ orient: "vertical", mode: "range", y: "lo", y2: "hi" }, D_RANGE),
  "overlapping-bar-chart": bar({ orient: "vertical", mode: "overlapping", group: "grp" }),
  "nested-bar-chart": bar({ orient: "vertical", mode: "nested", group: "grp" }),
  "thin-bar-chart": bar({ orient: "vertical", mode: "grouped", padding: 0.7 }),
  "rounded-bar-chart": bar({ orient: "vertical", mode: "grouped", cornerRadius: 1.2 }),
  "lollipop-bar-chart": bar({ orient: "vertical", mode: "grouped", lollipop: true }),
  "bullet-style-bar-chart": bar({ orient: "horizontal", mode: "grouped", barWidth: 2, labels: "value" }),
  "paired-bar-chart": bar({ orient: "horizontal", mode: "grouped", group: "grp", paired: true }),
  "mirrored-bar-chart": bar({ orient: "vertical", mode: "grouped", mirror: true }),
  "butterfly-chart": bar({ orient: "horizontal", mode: "diverging", group: "grp", padding: 0.35 }),
  "tornado-chart": bar({ orient: "horizontal", mode: "diverging", group: "grp", sort: "descending" }),
  "population-pyramid": bar({ orient: "horizontal", mode: "diverging", group: "grp", padding: 0.05 }),
  "ranked-bar-chart": bar({ orient: "horizontal", mode: "grouped", sort: "descending" }),
  // §1 dot comparisons
  "dot-plot": dot({ mode: "dot" }),
  "cleveland-dot-plot": dot({ mode: "cleveland", group: "grp" }),
  "grouped-dot-plot": dot({ mode: "dot", group: "grp" }),
  "paired-dot-plot": dot({ mode: "dot", group: "grp", connect: false, size: 1.6 }),
  "dumbbell-chart": dot({ mode: "dumbbell", group: "grp" }),
  "connected-dot-plot": dot({ mode: "dumbbell", group: "grp", orient: "vertical" }),
  "lollipop-chart": dot({ mode: "lollipop" }),
  "needle-chart": dot({ mode: "needle", orient: "vertical" }),
  "ranked-dot-plot": dot({ mode: "cleveland", sort: "descending" }),
  "dumbbell-plot": dot({ mode: "dumbbell", group: "grp", size: 1.6 }),
  // §1 ranking
  "bump-chart": rank({ mode: "bump" }),
  "rank-evolution-chart": rank({ mode: "bump", marker: false }),
  "podium-chart": { family: "rank", options: { mode: "podium", n: 3, x: "cat", y: "val", group: "grp" }, data: D_CAT },
  "top-n-chart": { family: "rank", options: { mode: "top-n", n: 3, x: "cat", y: "val", group: "grp" }, data: D_CAT },
  // §1 unit comparisons
  "pictogram-chart": unit({ mode: "pictogram", glyph: "person" }),
  "icon-array": unit({ mode: "icon-array", glyph: "square", columns: 12 }),
  "unit-chart": unit({ mode: "unit", glyph: "square" }),
  "isotype-chart": unit({ mode: "isotype", glyph: "triangle" }),
  "repeated-symbol-chart": unit({ mode: "unit", glyph: "dot" }),
  "waffle-comparison-chart": unit({ mode: "waffle", columns: 10 }),
  // §2 line charts
  "basic-line-chart": line({ group: "", curve: "linear" }),
  "multi-line-chart": line({ curve: "linear" }),
  "grouped-line-chart": line({ curve: "linear", markers: true }),
  "highlighted-line-chart": line({ highlight: "Alpha" }),
  "indexed-line-chart": line({ mode: "indexed" }),
  "normalized-line-chart": line({ mode: "normalized" }),
  "step-chart": line({ curve: "step" }),
  "step-before-chart": line({ curve: "step-before" }),
  "step-after-chart": line({ curve: "step-after" }),
  "smoothed-line-chart": line({ curve: "catmull-rom" }),
  "spline-chart": line({ curve: "basis" }),
  "connected-scatter-time-series": line({ group: "", curve: "linear", markers: true }),
  "confidence-band-chart": line({ confidence: true }),
  // §2 area charts
  "area-chart": area({ group: "", stack: "none" }),
  "stacked-area-chart": area({ stack: "stack" }),
  "100percent-stacked-area-chart": area({ stack: "stack", offset: "expand" }),
  "overlapping-area-chart": area({ stack: "none" }),
  "streamgraph": area({ stack: "stack", offset: "wiggle", curve: "catmull-rom" }),
  "theme-river": area({ stack: "stack", offset: "silhouette", curve: "basis" }),
  "horizon-chart": area({ horizon: 4 }),
  "horizon-time-series-chart": area({ horizon: 3 }),
  "horizon-graph": area({ horizon: 5 }),
  "difference-chart": area({ difference: true }),
  "range-area-chart": area({ range: true }),
  // §2 sparks and change
  "sparkline": spark({ mode: "line" }),
  "spark-area-chart": spark({ mode: "area" }),
  "spark-bar-chart": spark({ mode: "bar" }),
  "slope-chart": slope({ mode: "slope" }),
  "slopegraph": slope({ mode: "slope", markers: false }),
  "before-after-plot": slope({ mode: "before-after" }),
  "change-bar": slope({ mode: "change" }),
  "delta-chart": slope({ mode: "delta", markers: false }),
  // §2 timelines
  "timeline": timeline({ mode: "interval" }),
  "interval-timeline": timeline({ mode: "interval", state: "lane" }),
  "event-timeline": timeline({ mode: "event" }),
  "chronology-chart": timeline({ mode: "event", lane: "state" }),
  "event-sequence-chart": timeline({ mode: "event", lane: "label" }),
  "milestone-timeline": timeline({ mode: "milestone" }),
  "state-timeline": timeline({ mode: "state" }),
  "status-timeline": timeline({ mode: "state", state: "label" }),
  "lifeline-chart": timeline({ mode: "swimlane", state: "lane" }),
  "swimlane-timeline": timeline({ mode: "swimlane" }),
  "calendar-chart": timeline({ mode: "calendar", columns: 7 }),
  "calendar-heatmap": timeline({ mode: "calendar", columns: 14 }),
  "year-heatmap": timeline({ mode: "calendar", columns: 26 }),
  "clock-chart": timeline({ mode: "radial", turns: 1 }),
  "radial-timeline": timeline({ mode: "radial" }),
  "circular-timeline": timeline({ mode: "radial", state: "lane" }),
  "spiral-timeline": timeline({ mode: "spiral" }),
  "spiral-time-series-chart": timeline({ mode: "spiral", turns: 3.5 }),
  "spiral-chart": timeline({ mode: "spiral", turns: 4.5 }),
  "cycle-plot": timeline({ mode: "cycle" }),
  "seasonal-plot": timeline({ mode: "seasonal" }),
  "seasonal-subseries-plot": timeline({ mode: "seasonal", lane: "state" }),
  "fan-chart": timeline({ mode: "fan" }),
  "forecast-chart": timeline({ mode: "forecast", split: 5 }),
  "forecast-cone": timeline({ mode: "forecast", split: 7 }),
  "polar-time-series-chart": coordplot({ coordinate: "polar", axesCount: 6 }),
  "control-timeline": annotated({ mode: "reference-band" }),
  "waterfall-chart": waterfall({ mode: "waterfall" }),
  "bridge-chart": waterfall({ mode: "bridge", connector: true }),
  // §17 financial
  "line-price-chart": financial({ mode: "line" }),
  "candlestick-chart": financial({ mode: "candlestick" }),
  "ohlc-chart": financial({ mode: "ohlc" }),
  "hlc-chart": financial({ mode: "hlc" }),
  "kagi-chart": financial({ mode: "kagi", boxSize: 1 }),
  "renko-chart": financial({ mode: "renko", boxSize: 1 }),
  "point-and-figure-chart": financial({ mode: "point-and-figure", boxSize: 1 }),
  "heikin-ashi-chart": financial({ mode: "heikin-ashi" }),
  "volume-bar-chart": financial({ mode: "volume" }),
  "volume-profile": financial({ mode: "volume-profile" }),
  "price-volume-chart": financial({ mode: "price-volume" }),
  "moving-average-chart": financial({ mode: "line", overlay: "ma", window: 3 }),
  "bollinger-band-chart": financial({ mode: "line", overlay: "bollinger", window: 3 }),
  "macd-plot": financial({ indicator: "macd" }),
  "rsi-plot": financial({ indicator: "rsi" }),
  "momentum-plot": financial({ indicator: "momentum" }),
  "drawdown-chart": financial({ indicator: "drawdown" }),
  "equity-curve": financial({ indicator: "equity" }),
  "return-chart": financial({ indicator: "return" }),
  "cumulative-return-chart": financial({ indicator: "cumulative-return" }),
  "risk-return-scatterplot": scatter({ trend: "linear", size: "size", color: "grp" }),
  "efficient-frontier": curve({ mode: "ppf" }),
  "production-possibility-frontier": curve({ mode: "ppf", samples: 32 }),
  "portfolio-allocation-chart": bar({ orient: "vertical", mode: "percent", group: "grp", cornerRadius: 0.6 }),
  "pandl-bridge": waterfall({ mode: "bridge", connector: true, total: false }),
  "revenue-bridge": waterfall({ mode: "bridge", connector: true, padding: 0.55 }),
  "variance-chart": waterfall({ mode: "variance" }),
  "actual-vs-budget-chart": waterfall({ mode: "actual-budget" }),
  "bullet-chart": kpi({ mode: "bullet" }),
  "kpi-chart": kpi({ mode: "card", columns: 1 }),
  "supply-demand-graph": curve({ mode: "supply-demand" }),
  "phillips-curve": curve({ mode: "phillips" }),
  "indifference-curve": curve({ mode: "indifference" }),
  "engel-curve": curve({ mode: "engel" }),
  "yield-curve": curve({ mode: "yield" }),
  "term-structure-chart": curve({ mode: "term-structure" }),
  // §18 KPI and gauges
  "kpi-card": kpi({ mode: "card" }),
  "big-number-display": kpi({ mode: "big-number" }),
  "delta-indicator": kpi({ mode: "delta" }),
  "trend-indicator": kpi({ mode: "trend" }),
  "sparkline-kpi": kpi({ mode: "sparkline" }),
  "bullet-graph": kpi({ mode: "bullet", columns: 1 }),
  "traffic-light-indicator": kpi({ mode: "traffic-light" }),
  "status-badge": kpi({ mode: "badge" }),
  "scorecard": kpi({ mode: "scorecard" }),
  "balanced-scorecard": kpi({ mode: "scorecard", columns: 2 }),
  "dashboard-tile": kpi({ mode: "tile" }),
  "small-multiple-dashboard": kpi({ mode: "tile", columns: 2 }),
  "linear-gauge": gauge({ mode: "linear" }),
  "radial-gauge": gauge({ mode: "radial" }),
  "dial": gauge({ mode: "dial" }),
  "speedometer": gauge({ mode: "speedometer" }),
  "thermometer": gauge({ mode: "thermometer" }),
  "progress-bar": gauge({ mode: "progress-bar", thickness: 4 }),
  "progress-ring": gauge({ mode: "progress-ring" }),
  "completion-donut": gauge({ mode: "donut" }),
  // §19 funnels
  "funnel-chart": funnel({ mode: "funnel" }),
  "inverted-funnel": funnel({ mode: "inverted" }),
  "conversion-funnel": funnel({ mode: "funnel", labels: "category" }),
  "sales-funnel": funnel({ mode: "funnel", labels: "value" }),
  "recruitment-funnel": funnel({ mode: "funnel", gap: 2 }),
  "marketing-funnel": funnel({ mode: "funnel", gap: 0 }),
  "funnel-with-drop-off": funnel({ mode: "drop-off", dropoff: true }),
  "pyramid-funnel": funnel({ mode: "pyramid" }),
  "sankey-funnel": funnel({ mode: "sankey" }),
  "stage-flow-funnel": funnel({ mode: "stage-flow" }),
  // §50 annotation-driven
  "annotated-chart": annotated({ mode: "callout" }),
  "callout-chart": annotated({ mode: "callout", at: 4 }),
  "explainer-diagram": annotated({ mode: "callout", at: 2, value: 8 }),
  "highlight-chart": annotated({ mode: "highlight" }),
  "threshold-chart": annotated({ mode: "threshold" }),
  "reference-line-chart": annotated({ mode: "reference-line" }),
  "reference-band-chart": annotated({ mode: "reference-band", value: 5 }),
  "event-marker-chart": annotated({ mode: "event-marker" }),
  "label-rich-chart": annotated({ mode: "label-rich" }),
  "directly-labeled-chart": annotated({ mode: "direct-label" }),
  "story-chart": narrative({ mode: "storyline", panels: 5 }),
  "narrative-chart": narrative({ mode: "arc", panels: 6 }),
  "scrollytelling-style-static-sequence": statepanel({ mode: "storyboard", frames: 8 }),
  // §51 axis-specialised plots
  "linear-axis-plot": axisplot({ scale: "linear" }),
  "log-axis-plot": axisplot({ scale: "log" }),
  "symmetric-log-plot": axisplot({ scale: "symlog" }),
  "power-scale-plot": axisplot({ scale: "pow" }),
  "square-root-scale-plot": axisplot({ scale: "sqrt" }),
  "reciprocal-axis-plot": axisplot({ scale: "reciprocal" }),
  "probability-axis-plot": axisplot({ scale: "probability" }),
  "date-time-axis-plot": axisplot({ scale: "time" }),
  "categorical-axis-plot": axisplot({ scale: "categorical" }),
  "discontinuous-axis-plot": axisplot({ mode: "discontinuous" }),
  "broken-axis-plot": axisplot({ mode: "broken" }),
  "mirrored-axis-plot": axisplot({ mode: "mirrored" }),
  "dual-axis-plot": axisplot({ mode: "dual" }),
  "multiple-axis-plot": axisplot({ mode: "multiple" }),
  "polar-axis-plot": coordplot({ coordinate: "polar", axesCount: 2 }),
  "ternary-axis-plot": coordplot({ coordinate: "ternary", grid: "none" }),
  // §52 coordinate-system variants
  "cartesian-plot": coordplot({ coordinate: "cartesian" }),
  "polar-plot": coordplot({ coordinate: "polar" }),
  "radial-plot": coordplot({ coordinate: "radial" }),
  "log-polar-plot": coordplot({ coordinate: "logpolar" }),
  "ternary-plot": coordplot({ coordinate: "ternary" }),
  "barycentric-plot": coordplot({ coordinate: "barycentric" }),
  "parallel-coordinate-plot": coordplot({ coordinate: "parallel", axesCount: 4 }),
  "spherical-coordinate-plot": coordplot({ coordinate: "spherical" }),
  "cylindrical-coordinate-plot": coordplot({ coordinate: "cylindrical" }),
  "3d-cartesian-plot": coordplot({ coordinate: "cartesian", axesCount: 3, grid: "x" }),
  // §61 cartesian niche kinds
  "marimekko": bar({ orient: "vertical", mode: "percent", group: "grp", padding: 0.02 }),
  "mosaic-chart": bar({ orient: "vertical", mode: "percent", group: "grp", padding: 0.08 }),
  // §62 software performance (cartesian kinds)
  "trace-timeline": timeline({ mode: "interval", lane: "state" }),
  "span-timeline": timeline({ mode: "state", lane: "label" }),
  "waterfall-request-chart": timeline({ mode: "interval", lane: "label", state: "lane" }),
  "allocation-timeline": area({ stack: "stack", curve: "step-after" }),
  "garbage-collection-timeline": timeline({ mode: "milestone", lane: "state" }),
  "cpu-profile-chart": line({ curve: "monotone-x", grid: "y" }),
  "memory-profile-chart": line({ curve: "natural", grid: "both" }),
  "latency-percentile-chart": line({ curve: "linear", markers: true, mode: "indexed" }),
  // §65 monitoring (cartesian kinds)
  "traffic-time-series": line({ curve: "linear", markers: true, grid: "y" }),
  "error-rate-chart": line({ curve: "catmull-rom", grid: "y" }),
  "throughput-chart": line({ curve: "step-after", grid: "y" }),
  "slo-burn-rate-chart": line({ curve: "linear", markers: true, grid: "both" }),
  "request-waterfall": timeline({ mode: "swimlane", lane: "label" }),
  "availability-timeline": timeline({ mode: "state", state: "lane" }),
  "status-history": timeline({ mode: "state", lane: "state" }),
  // §69 survey and polling (bar/line kinds)
  "likert-chart": bar({ orient: "horizontal", mode: "stacked", group: "grp", padding: 0.15 }),
  "diverging-likert-chart": bar({ orient: "horizontal", mode: "diverging", group: "grp", padding: 0.15 }),
  "stacked-response-chart": bar({ orient: "horizontal", mode: "percent", group: "grp", padding: 0.15 }),
  "top-two-box-chart": bar({ orient: "horizontal", mode: "grouped", group: "grp", sort: "descending", padding: 0.15 }),
  "net-promoter-chart": bar({ orient: "vertical", mode: "diverging", group: "grp", cornerRadius: 0.8 }),
  "margin-of-error-interval-chart": dot({ mode: "cleveland", group: "grp", size: 1 }),
  "polling-trend": line({ curve: "catmull-rom", markers: true }),
  "election-polling-average": line({ curve: "basis", confidence: true }),
  "seat-projection": bar({ orient: "horizontal", mode: "stacked", group: "grp", cornerRadius: 0.8 }),
  // §70 election (bar and vote-share kinds)
  "election-result-bar-chart": bar({ orient: "horizontal", mode: "grouped", sort: "descending", cornerRadius: 0.4 }),
  "vote-share-chart": bar({ orient: "horizontal", mode: "percent", group: "grp", cornerRadius: 0.4 }),
  "coalition-diagram": bar({ orient: "horizontal", mode: "stacked", group: "grp", padding: 0.5 }),
  "seat-projection-chart": bar({ orient: "horizontal", mode: "stacked", group: "grp", labels: "value" }),
  "swingometer": gauge({ mode: "dial", startAngle: 200, endAngle: -20 }),
  // §72 narrative
  "storyline-chart": narrative({ mode: "storyline" }),
  "story-arc-visualization": narrative({ mode: "arc" }),
  "story-map": narrative({ mode: "story-map" }),
  "scene-graph": narrative({ mode: "scene-graph" }),
  "event-sequence-diagram": narrative({ mode: "sequence" }),
  "narrative-flow-diagram": narrative({ mode: "storyline", panels: 3 }),
  "character-interaction-timeline": narrative({ mode: "scene-graph", panels: 6 }),
  "comic-panel-data-narrative": narrative({ mode: "panels", panels: 4 }),
  "progressive-reveal-panels": narrative({ mode: "reveal", panels: 5 }),
  "before-after-sequence": narrative({ mode: "before-after", panels: 2 }),
  "annotated-timeline": timeline({ mode: "milestone", state: "lane" }),
  // §73 static equivalents of interactive patterns
  "overview-plus-detail-panel": statepanel({ mode: "overview-detail" }),
  "focus-plus-context-chart": statepanel({ mode: "focus-context" }),
  "brush-selection-depiction": statepanel({ mode: "brush" }),
  "zoom-inset": statepanel({ mode: "zoom-inset" }),
  "magnified-region": statepanel({ mode: "magnify" }),
  "hover-state-callout-rendered-statically": statepanel({ mode: "hover" }),
  "selected-node-network-state": statepanel({ mode: "selected-node" }),
  "filtered-vs-unfiltered-comparison": statepanel({ mode: "filter-compare" }),
  "before-after-filtering-panels": statepanel({ mode: "filter-panels", frames: 2 }),
  "small-multiple-animation-frames": statepanel({ mode: "frames", frames: 4 }),
  "keyframe-sequence": statepanel({ mode: "keyframe", frames: 3 }),
  "transition-sequence": statepanel({ mode: "transition", frames: 5 }),
  "animated-path-frames": statepanel({ mode: "animated-path", frames: 7 }),
  "morph-sequence": statepanel({ mode: "morph", frames: 6 }),
  "interactive-state-storyboard": statepanel({ mode: "storyboard", frames: 9 }),
};
//#endregion 🔖️Mapping

//#region 🔖️Apply
const CATALOG =
  "C:/git/semio/🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json";

const doc = JSON.parse(readFileSync(CATALOG, "utf8")) as {
  kinds: { id: string; slug: string; family: string; options: Record<string, unknown>; data: string }[];
};

let touched = 0;
const seen = new Map<string, string>();
for (const entry of doc.kinds) {
  const spec = SPECS[entry.slug];
  if (!spec) continue;
  entry.family = spec.family;
  entry.options = { ...spec.options };
  entry.data = spec.data;
  touched += 1;
  const signature = `${spec.family}|${JSON.stringify(spec.options)}`;
  const clash = seen.get(signature);
  if (clash && clash !== entry.slug) {
    console.error(`❌ duplicate option set: ${entry.slug} and ${clash} → ${signature}`);
    process.exitCode = 1;
  }
  seen.set(signature, entry.slug);
}

const unused = Object.keys(SPECS).filter((slug) => !doc.kinds.some((k) => k.slug === slug));
if (unused.length > 0) console.error(`❌ slugs not present in the catalogue: ${unused.join(", ")}`);

writeFileSync(CATALOG, `${JSON.stringify(doc, null, 2)}\n`, "utf8");
console.log(`✅ rewrote ${touched} entries; ${seen.size} distinct option sets`);
//#endregion 🔖️Apply
