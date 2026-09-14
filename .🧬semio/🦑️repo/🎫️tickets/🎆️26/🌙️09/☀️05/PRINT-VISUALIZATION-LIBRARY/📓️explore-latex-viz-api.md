# Print LaTeX Visualization Library — API Inventory

Read-only exploration by a Sonnet explorer (2026-09-05), persisted by the coordinator. Scope: `🧰️framework/🛍️products/📓️print/🖋️latex/` (94 `.sty` + `semio.cls` + `zukunftbau.cls`, 11,844 lines).

## 1. Architecture

Load graph: `semio.cls` → `semio-tokens` → `semio-fonts` → `semio-core` → `semio-logo` → `semio-table`; `semio-window` (tcolorbox, fancyhdr, eso-pic, tikz); `semio-components`; `semio-graph`; `semio-tree`; `semio-viz` (→ `semio-viz-data` → `semio-viz-scale` → `semio-viz-mark` → `semio-viz-axis` → `semio-viz-layout`) → `semio-viz-charts` (aggregator, `semio-viz.sty:343`) → 78 `semio-viz-chart-*.sty`.

- Strictly layered viz chain; the 78 chart files contain only `\SemioVizChartKind` calls and are not standalone-loadable.
- **Broken reference:** `semio-viz-charts.sty:5` requires `semio-viz-chart-distribution`, which does not exist on disk. Every document loading `semio-viz` fails unless this is fixed.
- Engine: `expl3`/`xparse`, `tikz` (`calc, arrows.meta, shapes.geometric, decorations.pathreplacing`, `semio-viz.sty:14`), `l3keys` for all options, `xcolor`, `fontspec`. **No pgfplots**; all plotting is hand-rolled TikZ with `fp_eval`.
- Tokens: `semio-tokens.sty` is generated from `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`; theme aliases (`semio-chrome-*`) resolved in `semio-core.sty:56-92` at `begindocument`. Downstream code consumes only aliases plus brand colours, so dark mode works except the categorical palette.

## 2. Public API surface

| File | Commands / environments | Notes |
|---|---|---|
| `semio-viz.sty` | `VizFigure` env (`title, window, width=80, height=40/auto`, :50-71), `VizSection` (`title, aside, rule`), `VizColumn` (`width, title, note`), `\SemioVizNoteBelow{anchor}{width}{text}`, `\SemioVizChartKind{name}{family}{opts}` (:308), `\SemioVizChart{kind}[opts]` (:336), `\SemioVizDemo{kind}` (:338) | |
| `semio-viz-data.sty` | `\SemioVizTable{name}{cols}`, `\SemioVizRow{name}{row}`; global seq/clist registers; column lookup by name (:74); aggregates min/max/sum/mean (:95-143); built-in `demo` table (:145-160, 5 rows, cols `cat,val,val2,grp,t,label,value,from,to,row,col,chip,s1,s2,s3`) | |
| `semio-viz-scale.sty` | `\SemioVizScale{name}{kind}{domain}{range}` (:263); kinds `linear, log, symlog, pow, sqrt, band, point, ordinal, quantize, quantile, threshold` (:34-44); band helpers (:266-322); hard-coded 7-colour categorical cycle (:224-228) | most d3-like piece |
| `semio-viz-mark.sty` | `\SemioVizMark{kind}[opts]{x,y}` (`size=2.2, fill, draw, points`), `\SemioVizPath{kind}[opts]`, `\SemioVizText{kind}[opts]{x,y}{text}` (kinds `cell,module,chip,label,title,value`); 47 mark kinds (:84-96) | spline variants are `smooth` with different tensions (:246-250) |
| `semio-viz-axis.sty` | `\SemioVizAxis[scale=x, orient=bottom, legend]`, `\SemioVizGrid` (4 fixed gridlines, :101-108), `\SemioVizLegend[legend=swatch\|gradient\|size]` | no titles, formatting, minor ticks |
| `semio-viz-layout.sty` | `\SemioVizLayout{family}[opts]`, 22 keys (`data=demo, x=cat, y=val, legend, scale, xscale, yscale, orient, label, value, key, from, to, row, col, chip, unit, stages, columns, boxheight=8, gutter=1.5, labelwidth, frame, header, domain, anchor`, :71-131); 34 registered families, 31 used | |
| `semio-graph.sty` | `GraphFigure` (`title, window, width=181, height=50`), `\SemioGraphNode[state,image]{xy}{label}`, `\SemioGraphEdge[kind]{from}{to}`, `\SemioGraphState`, `\SemioGraphEdgeKind`, `\SemioGraphLegend`, `GraphSpread` | states `plain, focal, attested, hypo`; edges `plain, muted, synth, hypo, directed` |
| `semio-tree.sty` | `TreeFigure`, `\SemioTreeKind` (7 args), `\SemioTreeBox/Row/Root/Group/Leaf`, `\SemioTreeMarks`, `\SemioTreeLegend` | kinds `root, group, leaf` |
| `semio-window.sty` (~3,880 lines) | `\SemioImage`, `Panel`, `Window`, `WindowItem/Row/Column/Arrangement`, chrome scale macros | |
| `semio-components.sty` | front/back matter (`\makecoverpages`, `\maketableofcontents`, appendices, registers) | |
| `semio-table.sty` | `\SemioTableHeaderRow`, long-table machinery | |

Chart-kind distribution across families (1,857 registrations): chrome 259, science 215, geo 209, process 170, heat 136, net 130, dist 90, line 88, gantt 77, tree 66, bar 57, pack 46, dot 43, bullet 38, pie 34, text 30, chord 30, radar 26, area 22, funnel 16, sankey 12, anno 12, waffle 11, special 7, path 7, force 6, calendar 6, parallel 5, flow 4, voronoi 3, hexbin 2.

## 3. Configurability audit

- Only **5 distinct option strings** across 1,857 registrations: `data=demo` (1,793), `data=demo,scale=y,orient=left` (59), `…scale=y-log…` (2), `…scale=x,orient=bottom` (2), `data=demo,legend=gradient` (1). Chart kinds are labels, not behaviour: `grouped-bar-chart` and `100percent-stacked-bar-chart` render byte-identical output via `\semio_viz_family_bar_vertical:` (`semio-viz-layout.sty:218-231`).
- `semio-viz-chart-d3-equivalent.sty:4-30` maps every scale/generator demo to the `chrome` family.
- Data-driven families: `bar, dot, line, area, pie, dist(=bar), grid, heat(+hexbin, calendar), gantt, process`. Hard-coded decorative glyphs ignoring data: `tree(=pack=net), force, flow(=sankey), chord, geo, radar(=special), parallel, waffle, funnel, bullet, text, science, voronoi, path/mark/anno`. ≈825 kinds (~44%) render a fixed glyph unrelated to input.
- 205 names carry a numeric section suffix (`band-78`, `polar-78`, `adjacency-matrix-11`): collision papering from generation.
- Hard-coded constants: grid lines 4 (`semio-viz-axis.sty:103`), ticks 5 (`semio-viz-scale.sty:256`), quantize buckets 4 (:166), waffle 32 cells / 8 cols (`semio-viz-layout.sty:464-465`), pie radius 14mm. Log ticks reuse the linear stepper (numerically wrong).
- Duplicated palette call sites (`semio-viz-scale.sty:224-228` vs `semio-viz-layout.sty:149-153`); polar node idiom repeated six times.

## 4. Data input model

Inline `\SemioVizTable`/`\SemioVizRow` only; no CSV/JSON ingestion, no pgfplotstable. Scale abstraction is genuinely d3-like; axis generator is minimal. Layout families named after algorithms (tree, pack, force, sankey, chord, voronoi, geo) compute nothing.

## 5. Gaps vs d3

| d3 concept | Status |
|---|---|
| linear/log/pow/sqrt/symlog scales | present (log ticks wrong) |
| band/point/ordinal | present |
| quantize/quantile/threshold | present, bucket count fixed |
| time scale, time format, number format | absent |
| sequential/diverging colour scales, schemes | absent |
| axis generator | partial |
| legend generator | minimal |
| line/area/curve generators | coupled to families; curves cosmetic |
| arc/pie generator | fixed radius, no donut/pad/corner |
| symbol generator | good (47 kinds) |
| hierarchy (tree, cluster, pack, treemap, partition) | absent as computation |
| force, sankey, chord, voronoi/Delaunay, geo projections | absent as computation |
| CSV/JSON loading | absent |
| faceting engine | absent (manual `VizColumn`) |
