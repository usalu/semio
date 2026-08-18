# Exhaustive Visualization Taxonomy for a TikZ-Based D3 for LaTeX Library

## 0. Foundational visualizations / primitives

### Points / marks

- Dot `dot` mark
- Circle `circle` mark
- Square `square` mark
- Rectangle `rectangle` mark
- Triangle `triangle` mark
- Diamond `diamond` mark
- Cross `cross` mark
- Plus `plus` mark
- Star `star` mark
- Custom glyph `custom-glyph` mark
- Icon mark `icon` mark
- Image mark `image` mark
- Text mark `text` mark

### Lines

- Straight line `straight-line` mark
- Polyline `polyline` mark
- Step line `step-line` mark
- Curved line `curved-line` mark
- Bezier curve `bezier` mark
- Spline `spline` mark
- Catmull-Rom spline `catmull-rom` mark
- Basis spline `basis-spline` mark
- Cardinal spline `cardinal-spline` mark
- Monotone spline `monotone-spline` mark
- Closed curve `closed-curve` mark

### Areas

- Polygon `polygon` mark
- Filled path `filled-path` mark
- Ribbon `ribbon` mark
- Band `band` mark
- Envelope `envelope` mark

### Arcs

- Circular arc `circular-arc` mark
- Elliptical arc `elliptical-arc` mark
- Annular arc `annular-arc` mark
- Sector `sector` mark
- Wedge `wedge` mark

### Connections

- Straight connector `straight-connector` mark
- Orthogonal connector `orthogonal-connector` mark
- Curved connector `curved-connector` mark
- Elbow connector `elbow-connector` mark
- Bundled connector `bundled-connector` mark
- Arrow `arrow` mark
- Bidirectional arrow `bidirectional-arrow` mark

### Regions

- Rectangular region `rectangular-region` mark
- Circular region `circular-region` mark
- Polygonal region `polygonal-region` mark
- Voronoi region `voronoi-region` mark
- Convex hull `convex-hull` mark
- Concave hull `concave-hull` mark

### Annotation primitives

- Label `label` mark
- Callout `callout` mark
- Leader line `leader-line` mark
- Bracket `bracket` mark
- Brace `brace` mark
- Highlight region `highlight-region` mark
- Reference line `reference-line` mark
- Reference band `reference-band` mark
- Reference point `reference-point` mark

## 1. Categorical comparison charts

### Bar charts

- Vertical bar chart `vertical-bar` chart
- Horizontal bar chart `horizontal-bar` chart
- Grouped bar chart `grouped-bar` chart
- Clustered bar chart `clustered-bar` chart
- Stacked bar chart `stacked-bar` chart
- 100% stacked bar chart `percent-stacked-bar` chart
- Diverging stacked bar chart `diverging-stacked-bar` chart
- Floating bar chart `floating-bar` chart
- Range bar chart `range-bar` chart
- Overlapping bar chart `overlapping-bar` chart
- Nested bar chart `nested-bar` chart
- Thin bar chart `thin-bar` chart
- Rounded bar chart `rounded-bar` chart
- Lollipop bar chart `lollipop-bar` chart
- Bullet-style bar chart `bullet-bar` chart
- Paired bar chart `paired-bar` chart
- Mirrored bar chart `mirrored-bar` chart
- Butterfly chart `butterfly` chart
- Tornado chart `tornado` chart
- Population pyramid `population-pyramid` chart

### Column charts

- Column chart `column` chart
- Grouped column chart `grouped-column` chart
- Stacked column chart `stacked-column` chart
- 100% stacked column chart `percent-stacked-column` chart
- Floating column chart `floating-column` chart
- Range column chart `range-column` chart

### Dot-based comparison

- Dot plot `dot-plot` chart
- Cleveland dot plot `cleveland-dot` chart
- Grouped dot plot `grouped-dot` chart
- Dumbbell chart `dumbbell` chart
- Arrow plot `arrow-dot` chart
- Lollipop chart `lollipop` chart
- Stem chart `stem` chart

## 2. Ranking charts

### Ordered comparison

- Ranked bar chart `ranked-bar` chart
- Ranked column chart `ranked-column` chart
- Slope chart `slope` chart
- Bump chart `bump` chart
- Ranked lollipop `ranked-lollipop` chart
- Ordered dot plot `ordered-dot` chart
- Unit chart `unit-chart` chart
- Pictogram chart `pictogram` chart
- Isotype chart `isotype` chart

## 3. Distribution charts

### Frequency

- Histogram `histogram` chart
- Variable-width histogram `variable-width-histogram` chart
- Density plot `density` chart
- Kernel density estimate `kernel-density` chart
- Ridgeline plot `ridgeline` chart
- Joy plot `joy` chart
- Frequency polygon `frequency-polygon` chart
- Cumulative frequency `cumulative-freq` chart
- Ogive `ogive` chart

### Summary

- Box plot `boxplot` chart
- Letter-value box plot `letter-value-box` chart
- Violin plot `violin` chart
- Bean plot `bean` chart
- Beeswarm plot `bee-swarm` chart
- Strip plot `strip` chart
- Jitter plot `jitter` chart
- Sina plot `sina` chart
- Raincloud plot `raincloud` chart
- Quantile box plot `quantile-box` chart

### Binned 2d

- Hexbin chart `hexbin` chart
- Rectbin chart `rectbin` chart
- Contour plot `contour` chart
- Filled contour `filled-contour` chart

## 4. Part-to-whole charts

### Circular

- Pie chart `pie` chart
- Donut chart `donut` chart
- Exploded pie chart `exploded-pie` chart
- Half pie / gauge pie `half-pie` chart
- Sunburst `sunburst` chart
- Multi-level donut `multi-level-donut` chart
- Rose chart `rose` chart
- Nightingale rose `nightingale` chart
- Polar area chart `polar-area` chart

### Rectangular

- Treemap `treemap` chart
- Squarified treemap `squarify-treemap` chart
- Slice-and-dice treemap `slice-dice-treemap` chart
- Icicle chart `icicle` chart
- Mosaic plot `mosaic` chart
- Marimekko / Mekko `marimekko` chart
- Waffle chart `waffle` chart
- Grid plot `gridplot` chart
- 100% stacked area `stacked-percent-area` chart

### Progress

- Funnel chart `funnel` chart
- Pyramid chart `pyramid-part` chart
- Gauge `gauge` chart
- Bullet chart `bullet` chart
- Progress bar `progress-bar` chart
- Radial progress `radial-progress` chart

## 5. Trend and time series charts

### Lines

- Line chart `line` chart
- Multi-line chart `multi-line` chart
- Step chart `step` chart
- Spline line chart `spline-line` chart
- Area chart `area` chart
- Stacked area chart `stacked-area` chart
- Streamgraph `streamgraph` chart
- Difference chart `difference` chart
- Range area chart `range-area` chart
- Horizon chart `horizon` chart
- Sparkline `sparkline` chart
- Sparkband `sparklines-band` chart

### Temporal markers

- Connected scatter `connected-scatter` chart
- Cycle plot `cycle` chart
- Seasonal plot `seasonal` chart
- Index chart `index-chart` chart
- Fan chart `fan` chart
- Control chart `control` chart

## 6. Correlation and relationship charts

### Point

- Scatter plot `scatter` chart
- Bubble chart `bubble` chart
- Scatterplot matrix `scatter-matrix` chart
- Q-Q plot `qq` chart
- P-P plot `pp` chart
- Residual plot `residual` chart
- Lag plot `lag` chart

### Bivariate surface

- Heatmap `heatmap` chart
- Correlation matrix `correlation-matrix` chart
- Voronoi scatter `voronoi-scatter` chart
- 2d density `density-2d` chart

## 7. Flow charts

### Quantity flow

- Sankey diagram `sankey` chart
- Alluvial diagram `alluvial` chart
- Parallel sets `parallel-sets` chart
- Chord diagram `chord` chart
- Arc diagram `arc-flow` chart
- Hive plot `hive` chart
- Origin-destination matrix `od-matrix` chart

## 8. Hierarchy charts

### Trees

- Node-link tree `tree` chart
- Cluster dendrogram `cluster-tree` chart
- Tidy tree `tidy-tree` chart
- Radial tree `radial-tree` chart
- Circular dendrogram `circular-dendrogram` chart
- Circle pack `pack` chart
- Nested circles `nested-circles` chart
- Enclosure diagram `enclosure` chart
- Indented tree `indented-tree` chart
- Sunburst hierarchy `sunburst-hierarchy` chart
- Icicle hierarchy `icicle-hierarchy` chart

## 9. Network charts

### Graphs

- Force-directed graph `force` chart
- Arc graph `arc` chart
- Adjacency matrix `adjacency` chart
- Radial network `radial-network` chart
- Layered / Sugiyama graph `layered-graph` chart
- Bipartite graph `bipartite` chart
- Ego network `ego` chart

## 10. Geospatial charts

### Maps

- Choropleth `choropleth` chart
- Proportional symbol map `proportional-symbol` chart
- Dot density map `dot-density-map` chart
- Isoline map `isoline` chart
- Isopleth map `isopleth` chart
- Cartogram `cartogram` chart
- Hexbin map `hexbin-map` chart
- Tile grid map `tile-grid-map` chart
- Flow map `flow-map` chart
- Connection map `connection-map` chart

## 11. Multivariate charts

### High dimensional

- Parallel coordinates `parallel-coordinates` chart
- Radar chart `radar` chart
- Spider chart `spider` chart
- Star plot `star-plot` chart
- Andrews plot `andrews` chart
- Chernoff faces `chernoff` chart
- Profile plot `profile` chart
- Table lens `table-lens` chart

## 12. Uncertainty charts

### Error and interval

- Error bar chart `error-bar` chart
- Confidence band `confidence-band` chart
- Prediction interval `prediction-interval` chart
- Gradient interval `gradient-interval` chart
- Quantile dot plot `quantile-dot` chart
- Fan uncertainty `fan-uncertainty` chart
- Hypothetical outcome plot `hypothetical-outcome` chart

## 13. Statistical charts

### Inference

- Normal Q-Q `qq-stat` chart
- Residual vs leverage `residual-leverage` chart
- Cook's distance `cooks` chart
- Autocorrelation `acf` chart
- Partial autocorrelation `pacf` chart
- ROC curve `roc` chart
- Precision-recall `precision-recall` chart
- Forest plot `forest` chart
- Bland-Altman `bland-altman` chart

## 14. Financial charts

### Markets

- Candlestick `candlestick` chart
- OHLC bar `ohlc` chart
- Kagi `kagi` chart
- Renko `renko` chart
- Point and figure `point-figure` chart
- Waterfall chart `waterfall` chart
- Volume bars `volume` chart
- Equity curve `equity-curve` chart

## 15. Calendar and temporal charts

### Calendars

- Calendar heatmap `calendar-heatmap` chart
- Contribution calendar `github-contrib` chart
- Gantt chart `gantt` chart
- Timeline `timeline` chart
- Swimlane `swimlane` chart
- Spiral chart `spiral` chart
- Polar clock `polar-clock` chart

## 16. Text and qualitative charts

### Words

- Word cloud `word-cloud` chart
- Word bar chart `bar-of-words` chart
- Phrase net `phrase-net` chart
- Text alluvial `alluvial-text` chart

## 17. Scientific charts

### Lab

- Function plot `function` chart
- Parametric plot `parametric` chart
- Polar plot `polar` chart
- Vector field `vector-field` chart
- Streamline plot `streamline` chart
- Phase portrait `phase` chart
- Ternary plot `ternary` chart
- Smith chart `smith` chart
- Bode plot `bode` chart
- Nyquist plot `nyquist` chart

## 18. Process and state charts

### Process

- Flowchart `flowchart` chart
- State machine `state-machine` chart
- Activity diagram `activity` chart
- Sequence diagram `sequence-diagram` chart
- Event storm `event-storm` chart

## 19. Axes, legends, and annotation charts

### Chrome

- Linear axis `axis-linear` axis
- Log axis `axis-log` axis
- Band axis `axis-band` axis
- Cartesian grid `grid-cartesian` axis
- Swatch legend `legend-swatch` axis
- Gradient legend `legend-gradient` axis
- Size legend `legend-size` axis

