# Ticket File Inventory Preflight — 2026-10-04

This read-only preflight resolves explicit owned inventory entries against the repository and ticket folder. It does not infer ownership from the shared Git diff or register disposable compiler outputs. Historical removed/relocated paths require their retained ownership evidence, not current file existence.

| Observed bucket | Count |
| --- | --- |
| Raw bullet entries | 909 |
| Unique entries | 881 |
| Existing repository paths | 587 |
| Existing ticket-only paths | 164 |
| Absent historical candidates | 130 |
| Ambiguous repository/ticket resolution | 0 |

## Historical Candidates To Confirm

- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️merge-contract.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/✒️mark/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🌊flow/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🌍geo/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🌳hierarchy/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🎨theme/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📍spatial/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📐scale/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🔬️probes/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🔢format/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🕸️network/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🖼️render/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🥧shape/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🧭coordinate/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🧮transform/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔮️oracle/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-axis.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-matrix-adjacency.sty`
- `🧰️framework/🛍️products/📓️print/🧪️tests/⏱️charts-kpi-gauge/🧫️fixtures/progress-ring.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/⏱️charts-kpi-gauge/🧫️fixtures/radial-gauge.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/⏳️scale-temporal/🧫️fixtures/scale-temporal.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/✒️mark-geometry/🧫️fixtures/mark-geometry.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/✒️mark-geometry/🧫️fixtures/mark-rotation.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/❄️geometry-tilings-fractals/🧫️fixtures/geometry-tilings-fractals.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/➗️math-functions-sampling/🧫️fixtures/math-functions-sampling.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/➰️shape-curves/🧫️fixtures/shape-curves.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🌊️field-streamlines/🧫️fixtures/streamlines.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🌌️charts-scatter-trend/🧫️fixtures/bubble-size.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🌌️charts-scatter-trend/🧫️fixtures/linear-trend.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🌍️geo-projections/🧫️fixtures/fit.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🌍️geo-projections/🧫️fixtures/forward.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🌍️geo-projections/🧫️fixtures/inverse.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🌗️shape-arc-pie/🧫️fixtures/shape-arc-pie.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🌳️hierarchy-aggregates/🧫️fixtures/hierarchy-aggregates.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🌴️hierarchy-tree-cluster/🧫️fixtures/hierarchy-tree-cluster.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎀️shape-links-ribbons/🧫️fixtures/shape-links-ribbons.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎡️charts-polar-radar/🧫️fixtures/coxcomb.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎡️charts-polar-radar/🧫️fixtures/polar-bars.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎡️charts-polar-radar/🧫️fixtures/polar-scatter.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎡️charts-polar-radar/🧫️fixtures/radar.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎨️scale-color/🧫️fixtures/scale-color.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎪️showcase-families/🧫️fixtures/showcase-capabilities.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎪️showcase-families/🧫️fixtures/showcase-namespaces.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎯️charts-evaluation-curves/🧫️fixtures/confusion.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎯️charts-evaluation-curves/🧫️fixtures/gain.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎯️charts-evaluation-curves/🧫️fixtures/pr.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎯️charts-evaluation-curves/🧫️fixtures/roc.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎻️charts-box-violin/🧫️fixtures/letter.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎻️charts-box-violin/🧫️fixtures/quartiles.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎻️charts-box-violin/🧫️fixtures/violin.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎻️charts-box-violin/🧫️fixtures/whiskers.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏔️spatial-contours-density/🧫️fixtures/contours.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏔️spatial-contours-density/🧫️fixtures/density.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏷️annotation-placement/🧫️fixtures/bracket.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏷️annotation-placement/🧫️fixtures/data-space.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏹️physics-projectile-rk4/🧫️fixtures/physics-projectile-rk4.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🐝️spatial-hexbin/🧫️fixtures/hexbin.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/💹️charts-financial/🧫️fixtures/candlestick.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/💹️charts-financial/🧫️fixtures/moving-average.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📈️charts-line-area/🧫️fixtures/area-stack.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📈️charts-line-area/🧫️fixtures/linear.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📈️charts-line-area/🧫️fixtures/step-after.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📈️charts-line-area/🧫️fixtures/step-before.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📊️charts-bar-layout/🧫️fixtures/diverging.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📊️charts-bar-layout/🧫️fixtures/grouped.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📊️charts-bar-layout/🧫️fixtures/horizontal.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📊️charts-bar-layout/🧫️fixtures/percent.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📊️charts-bar-layout/🧫️fixtures/stacked.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📋️transform-statistics/🧫️fixtures/transform-statistics.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📏️guide-axis-ticks/🧫️fixtures/axis-geometry.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📏️guide-axis-ticks/🧫️fixtures/band-ticks.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📏️guide-axis-ticks/🧫️fixtures/linear-ticks.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📏️guide-axis-ticks/🧫️fixtures/log-ticks.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📏️guide-axis-ticks/🧫️fixtures/tick-format-labels.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📐️scale-continuous/🧫️fixtures/scale-continuous.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📡️signal-dft-bode/🧫️fixtures/signal-dft-bode.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📶️charts-histogram-density/🧫️fixtures/bins-coarse.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📶️charts-histogram-density/🧫️fixtures/bins.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📶️charts-histogram-density/🧫️fixtures/density.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📶️charts-histogram-density/🧫️fixtures/ecdf.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📶️charts-histogram-density/🧫️fixtures/ticks.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔠️scale-discrete/🧫️fixtures/scale-discrete.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔢️format-number/🧫️fixtures/format-number.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔥️charts-heatmap-matrix/🧫️fixtures/cells.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔥️charts-heatmap-matrix/🧫️fixtures/heatmap-padded.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔥️charts-heatmap-matrix/🧫️fixtures/heatmap.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔬️probe-protocol/🧫️fixtures/power-mapping.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔲️charts-quadrant-table/🧫️fixtures/quadrant.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔲️charts-quadrant-table/🧫️fixtures/risk.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔲️charts-quadrant-table/🧫️fixtures/table-bars.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔲️charts-quadrant-table/🧫️fixtures/table.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔷️shape-symbols/🧫️fixtures/shape-symbols.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔺️spatial-delaunay-voronoi/🧫️fixtures/delaunay.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔺️spatial-delaunay-voronoi/🧫️fixtures/voronoi.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🕰️format-time/🧫️fixtures/format-time.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️gallery-render/🧫️fixtures/🖼️gallery-render.json`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗃️data-csv/🧫️fixtures/cities.csv`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗃️data-csv/🧫️fixtures/data-csv.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗃️data-csv/🧫️fixtures/places.tsv`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗓️charts-timeline/🧫️fixtures/interval.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗓️charts-timeline/🧫️fixtures/swimlane.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗝️guide-legend/🧫️fixtures/categorical.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗝️guide-legend/🧫️fixtures/size.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗺️geo-path-graticule/🧫️fixtures/graticule.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗺️geo-path-graticule/🧫️fixtures/path-none.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗺️geo-path-graticule/🧫️fixtures/path-resampled.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🗾️hierarchy-treemap/🧫️fixtures/hierarchy-treemap.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🥚️spatial-hull/🧫️fixtures/hull.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🥞️transform-stack/🧫️fixtures/transform-stack.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🥧️charts-pie-donut/🧫️fixtures/donut.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🥧️charts-pie-donut/🧫️fixtures/pie-padded.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🥧️charts-pie-donut/🧫️fixtures/pie-unsorted.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🥧️charts-pie-donut/🧫️fixtures/pie.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧊️3d-projection/🧫️fixtures/3d-projection.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧩️composition-concat-inset/🧫️fixtures/concat.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧩️composition-concat-inset/🧫️fixtures/dashboard.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧩️composition-concat-inset/🧫️fixtures/inset.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️biology-kaplan-meier/🧫️fixtures/biology-kaplan-meier.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧭️coordinate-polar-ternary/🧫️fixtures/coordinate-polar-ternary.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧱️hierarchy-partition/🧫️fixtures/hierarchy-partition.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🪟️facet-layout/🧫️fixtures/wrap.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🪣️transform-bin/🧫️fixtures/transform-bin.tex`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🫧️hierarchy-pack/🧫️fixtures/hierarchy-pack.tex`
- `🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🔓️viz-api.tex`

The final closing file list will prefix ticket-relative retained reports/inputs, deduplicate exact repository paths, retain proven removed source paths and exclude disposable generated outputs. It will be refreshed after the final full consumer and last audit reports. The ticket remains open.
