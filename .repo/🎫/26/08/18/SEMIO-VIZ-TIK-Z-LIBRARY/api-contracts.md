# semio-viz API contracts

Frozen so parallel workers do not negotiate interfaces.

## Layers

| Module | File | Owns |
| --- | --- | --- |
| umbrella | `print/tex/semio-viz.sty` | load order, VizFigure, chart-kind registry/dispatch, `\SemioVizDemo` |
| data | `print/tex/semio-viz-data.sty` | named tables, rows, column accessors, stats, group, bin, sort, stack |
| scale | `print/tex/semio-viz-scale.sty` | named scales, map, ticks |
| mark | `print/tex/semio-viz-mark.sty` | mark/generator/region/annotation primitives (taxonomy 0) |
| axis | `print/tex/semio-viz-axis.sty` | axes, grid, legends, plot frame |
| layout | `print/tex/semio-viz-layout.sty` | geometry families consumed by chart recipes |
| chart-* | `print/tex/semio-viz-chart-<category>.sty` | named kind aliases only, calling layout families |

## Units

Picture: `x=1mm, y=1mm`, origin bottom-left. Keys `width`/`height` are plain millimetre numbers (same contract as `semio-graph`).

## Data

```
\SemioVizTable{name}{colA, colB, colC}
\SemioVizRow{name}{a, 1, 2}
```

Internal:

- `\semio_viz_table_new:nn {name} {col-clist}`
- `\semio_viz_table_row:nV {name} \l_clist` (cells positional)
- `\semio_viz_table_nrows:nN {name} \l_int`
- `\semio_viz_table_cell:nnnN {name} {row-1-based} {col-name} \l_tl`
- `\semio_viz_table_col_min:nnN {name} {col} \l_fp`
- `\semio_viz_table_col_max:nnN {name} {col} \l_fp`
- `\semio_viz_table_col_sum:nnN {name} {col} \l_fp`
- `\semio_viz_table_stack:nnn {name} {group-col} {value-col}` writes `stack0`/`stack1` columns (normal stack)
- Built-in fixture table `demo` (cats A–E, values, groups, times) created by `\semio_viz_data_demo:`

## Scale

```
\SemioVizScale{name}{linear}{0,10}{0,80}
```

Kinds: `linear`, `log`, `symlog`, `pow`, `sqrt`, `band`, `point`, `ordinal`, `quantize`, `quantile`, `threshold`.

- `\semio_viz_scale_map:nnN {name} {value} \l_fp` — numeric range
- `\semio_viz_scale_color:nnN {name} {value} \l_tl` — color token/name
- `\semio_viz_scale_ticks:nN {name} \l_seq` — tick values as tokens
- Color palettes: `categorical`, `sequential`, `diverging` from semio tokens only

## Mark

```
\SemioVizMark{dot}[size=1.2]{x,y}
\SemioVizPath{polyline}[points={0,0;10,8;20,4}]
```

Mark names are taxonomy-0 slugs. Path generators take `points=` as `x,y;x,y;...`.

## Axis

```
\SemioVizAxis[scale=x, orient=bottom]
\SemioVizGrid
\SemioVizLegend[legend=swatch]
```

## Layout families

`\semio_viz_layout:nn {family} {key-vals}` writes geometry into `\g_semio_viz_geom_seq` (each item `x0,y0,x1,y1,cx,cy,label,value,color`).

Families: `bar`, `dot`, `line`, `area`, `pie`, `dist`, `tree`, `net`, `flow`, `geo`, `heat`, `radar`, `parallel`, `waffle`, `funnel`, `gantt`, `bullet`, `pack`, `chord`, `sankey`, `force`, `voronoi`, `hexbin`, `calendar`, `text`, `science`, `process`, `special`, `path`, `mark`, `anno`, `chrome`.

## Chart kinds

```
\semio_viz_chart_kind_define:nnn {vertical-bar-chart} {bar} {data=demo}
\SemioVizChart{vertical-bar-chart}[data=demo, x=cat, y=val]
\SemioVizDemo{vertical-bar-chart}
```

Every taxonomy leaf slug is a chart kind or a mark kind. Gallery uses `\SemioVizDemo{slug}` plus `% viz-covers: <section>/<slug>`.

## Figure

```
\begin{VizFigure}[title=..., window=..., width=80, height=45]
  ...
\end{VizFigure}
```

Wraps `Figure` with `break=false` and `tikzpicture[semio viz picture]`.

## Diagnostics

Package `semio-viz`. Messages: `unknown-table`, `unknown-column`, `unknown-scale`, `unknown-scale-kind`, `unknown-mark`, `unknown-chart`, `unknown-layout`, `outside-figure`, `unit-in-number`, `empty-data`.
