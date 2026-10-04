# Inferred Layer Transform and Layout Contract

The chart's authored table and options feed `inferVizLayerTable(spec, layer)`. The function is pure and deterministic: it copies the source table, applies transforms in declaration order, then resolves its declared layout. Missing tables, absent required columns, invalid enumerations and invalid numerical controls produce actionable errors for the owning chart inference to convert to diagnostic data.

The authored transformation vocabulary is filter, sort, group, aggregate, rollup, summary, fold, pivot, join, window, normalize, cumulative, bin, stack, quantile, kde and regression. Filtering uses a structured column/operator/value predicate, without evaluation of arbitrary source text. Grouping supports multiple columns and first-appearance order. Aggregates support sum, mean, min, max, count, median, sample variance and deviation. Normalization supports sum and extent modes. Regression supports linear, polynomial, exponential, logarithmic and power fits.

Every layout in the existing schema is dispatched: stack, bin, hexbin, beeswarm, jitter, pie, arc, chord, sankey, alluvial, treemap, partition, pack, force, tree, cluster, dag, bundling, voronoi, delaunay, hull, contour, density and projection. Geometrical coordinates are measured in the chart's inner rectangle in millimetres. The renderer owns placement of that rectangle.

## Output Columns

| Computation | Inferred columns |
| --- | --- |
| Histogram bin | x, x2, y, x0, x1, count |
| Stack | original columns, key, index, y, y2, y0, y1, value |
| Pie | original columns, angular spans, padAngle, innerRadius, outerRadius, x, y |
| Hierarchy | original columns, value, depth, x, y, x2, y2, rectangular bounds, radius, size, parentX, parentY |
| Graph node | id, x, y |
| Graph link | source, target, x, y, x2, y2 |
| Sankey/alluvial link | source, target, value, x, y, x2, y2, width |
| Sankey/alluvial node | id, value, x, y, x2, y2, depth |
| Chord | source, target, value, angular spans and target angular spans, radius |
| Polygon | x, y, detail, order, value, points (JSON polygon) |
| Hexbin | x, y, count, value, radius |
| Projection | original columns, x, y |

Options that select lists currently accept comma/semicolon-separated schema scalar strings; transforms additionally accept authored arrays. Deterministic random layouts use an explicit seed. Numerical implementations remain free of external runtime dependencies. The integration lane is relocating the previous numerical modules into owned inference leaves; these computations are authored there rather than establishing another runtime engine.

## Customization Controls

Unrecognized transform and layout controls are rejected rather than silently ignored. Transform controls use these exact spellings:

| Transform | Controls |
| --- | --- |
| filter | column, operator (eq/ne/lt/lte/gt/gte/in/valid), value |
| sort | column/columns, descending, order |
| group | column/columns, as |
| aggregate/rollup/summary | column, groupby/group, operation, as |
| fold | columns, key, value |
| pivot | index, key, value, operation |
| join | table, left, right, prefix |
| window | column, as, groupby, size, operation, centred |
| normalize | column, as, groupby, mode (sum/extent) |
| cumulative | column, as, groupby |
| bin | column, thresholds, min, max |
| stack | keys, order, offset |
| quantile | column, probabilities |
| kde | column, min, max, samples, bandwidth, kernel |
| regression | x, y, method (linear/poly/exp/log/pow), order |

Every geometric layout accepts width and height. Hierarchy layouts accept id, parent, value, sort and leaves. Tree, cluster and bundling additionally accept separation and cousinSeparation. Treemap accepts tile, round, paddingInner, paddingOuter and each edge's padding. Partition accepts padding and round; pack accepts padding. Pie accepts value, sort, angular endpoints and padding, inner/outer radii and centre coordinates.

Graph layouts accept source/target or id columns. Arc accepts origin, vertical and output; DAG accepts layerGap, nodeGap, sweeps and output. Force accepts seed, alpha, velocityDecay, charge, distance, radius, iterations and output. Flow layouts accept value, nodeWidth, nodePadding, iterations, align and output; alluvial additionally requires stages. Chord accepts value, padAngle, transpose, output and radius.

Spatial layouts accept x/y selectors. Voronoi/density accept explicit x0/y0/x1/y1 bounds. Hexbin and beeswarm accept radius; beeswarm accepts axis. Jitter accepts seed and amount. Contour requires value, integer columns/rows describing its complete raster and thresholds. Density accepts cellSize, bandwidth, kernel and thresholds, and projects its raster coordinates into figure millimetres. Geographic projection accepts its kind, scale, translation, rotation, centre and reflection controls.

## Verification

Language-neutral scenarios are in `🧪️tests/🧮️inferred-layer-pipeline/🥒️.feature`, with all 24 layouts represented as authored JSON fixture inputs in `🔣️.json` and an adjacent TypeScript adapter. The actual differential runner invokes that adapter for every JSON scenario; these fixture assertions test determinism, unchanged authored snapshots and nonempty output. Independent D3 comparisons are separately registered as `layer` checks in the existing differential probe table.

The unchanged numerical transform checks passed via `bun nx run @semio-tech/print-viz-kernel:test -- quick transform` (20 passed, zero failures). The initial layer Nx run found three projection issues: undefined D3 aggregates must map to schema-null, and Voronoi ring rotations need geometric canonicalization. Those were corrected. A direct diagnostic invocation of the actual check table then exercised all 76 layer checks against independent D3 and the authored fixtures: 76 passed, zero failures. Runtime console output used `[DEBUG]` and its generated log resides under the ticket's generated directory for final cleanup.

The optional third argument, `VizLayerInferenceControl`, exposes AbortSignal and an onProgress(completed,total) callback. Cancellation is checked before source resolution, during row copying, between transform/layout phases and at every force iteration. Progress is monotone and bounded within the call. Dedicated cancellation and progress assertions passed among the 76 cases.

The final Nx retry stalled before any test output during graph construction and was cancelled to release the graph mutex. The coordinating agent must run the canonical Nx route after relocation and include its outcome separately; the direct diagnostic result does not substitute for a claimed final Nx pass.
