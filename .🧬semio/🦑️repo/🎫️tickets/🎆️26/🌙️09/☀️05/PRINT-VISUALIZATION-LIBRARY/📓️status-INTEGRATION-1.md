# INTEGRATION-1 — status

Agent: INTEGRATION-1. Ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`.
Scope: dissolve the duplicated kernel code of the FINISHED agents, remove the legacy packages, fix
the handed-back defects, and get the real pipeline green for the finished namespaces.

---

## 1. Done

### 1.1 Bundle loads
`\usepackage{semio-viz}` loads cleanly. Every `already defined` collision is resolved:

| collision | resolution |
|---|---|
| `\l_semio_viz_coord_kind_tl`, `\l_semio_viz_coord_axes_int` (charts-scatter vs coordinate) | scatter's `coordplot` family now *uses* the kernel's coordinate state; the duplicate declarations are gone and the package requires `semio-viz-coordinate` |
| `\l_semio_viz_comp_*` (charts-statistical vs composition) | the statistical `composite` family renamed to the `composite` prefix (90 occurrences) |
| `\l_semio_viz_pie_pad_fp` (charts-polar vs mark) | the polar family's *option* variables renamed `piechart_` (52 occurrences); `pie_` stays the mark kernel's pie generator state. This also fixed a real bug: the family's `padangle` was being overwritten by the generator's own clamped pad on every ring after the first. |
| family `sunburst` registered twice (charts-polar vs hierarchy-partition) | the §6 polar approximation renamed to the family `radial-partition` (the name the schema already carried); the §7 d3 partition keeps `sunburst`. Catalogue entries `multi-level-pie-chart`, `radial-partition-chart`, `sunburst-chart` follow. |
| msg `unknown-layout`, `\l_semio_viz_width_fp`/`_height_fp` (legacy layout vs network/flowchart) | gone with the legacy file |

### 1.2 Pending regions dissolved
No `%region 🔖️Pending-…` remains in any finished agent's file.

| was | now |
|---|---|
| `charts-bar` `Pending-semio-viz-theme` | palette → `\semio_viz_theme_color:nN`, shade → `\semio_viz_theme_shade:nnN`; the `\providecolor` chrome fallback moved into `semio-viz-theme.sty` (`🔖️ChromeFallback`) |
| `charts-bar` `Pending-semio-viz-plot` / `-guide` / `-transform` / `-probe` | renamed `🔖️Frame` / `🔖️Chrome` / `🔖️CartesianStatistics` / `🔖️ProbeCanvas`; they are the namespace's own frame and chrome on top of the kernel, not a second kernel. Every hard-coded `semio-chrome-*` token in the chrome is now resolved once per chart through `\semio_viz_theme_chrome:nN` + `\semio_viz_theme_stroke:n`, so the frame follows the document appearance. |
| `charts-distribution` `Pending-semio-viz-transform` (511 lines) | **moved into `semio-viz-transform.sty`** as `🔖️SeqStatistics`, prefix `semio_viz_stat_` → `semio_viz_tr_stat_` (686 occurrences across 7 packages). Inside it, the duplicated d3 routines now delegate: `tick_increment` → `\semio_viz_scale_tick_increment:nnnN`, `ticks` → `\semio_viz_scale_tick_values:nnnN`, `nice` → `\semio_viz_scale_nice_pair:nnnNN`; `stat_power`/`stat_factor` deleted. |
| `charts-distribution` `Pending-semio-viz-scale` | `\semio_viz_scale_linear:nnnnnN`, `\semio_viz_scale_band:nnnnnn`, `\semio_viz_scale_band_at:nN` **moved into `semio-viz-scale.sty`** as `🔖️AnonymousBand` |
| `charts-distribution` `Pending-semio-viz-theme` | `\semio_viz_palette:n` → expandable `\semio_viz_theme_color:n` (added to the theme kernel) |
| `charts-distribution` `Pending-semio-viz-probe` | `\SemioVizChartsDryRun` / `…Render` **moved into `semio-viz-probe.sty`** |
| `charts-statistical` `Pending-semio-viz-transform` | **moved into `semio-viz-transform.sty`** as `🔖️ModelEvaluation`, prefix `semio_viz_tr_eval_` |
| `matrix-heatmap` `Pending-semio-viz-transform` | **moved into `semio-viz-transform.sty`** as `🔖️MatrixReshape`, prefix `semio_viz_tr_matrix_` |
| `matrix-correlation` `Pending-semio-viz-transform` | Pearson **moved into `semio-viz-transform.sty`** as `🔖️Correlation`; the hand-written d3-hexbin copy **deleted** and replaced by a 25-line adapter onto `semio-viz-spatial`'s `\semio_viz_spatial_hexbin:nn` (the verbatim d3-hexbin) |
| `charts-polar` `\semio_viz_stat_pie` | deleted; the polar and sunburst families call the mark kernel's `\semio_viz_pie_compute:nnnnn` through the new `\semio_viz_pie_from_seq:Nnnnn`. This is stricter than the copy was: d3 skips non-positive values in the pie sum, the copy did not. |
| `guide` `Pending-semio-viz-theme` | `\semio_viz_guide_chrome:nN` **moved into `semio-viz-theme.sty`** as `\semio_viz_theme_chrome:nN` (GRAMMAR-CORE request, done); `semio-viz-theme` now requires `semio-fonts` (also requested) |
| `guide` `Pending-semio-viz-scale` (159 lines) | **deleted**; `\semio_viz_guide_scale_ticks:nnN` is a one-line delegation to `\semio_viz_scale_ticks_count:nnN`, which already knows the discrete kinds, the log decades, the temporal intervals and an explicit `tickValues` |

Also removed from the kernel as dead legacy: `semio-viz-scale.sty`'s `🔖️LegacyPalette` region and
`\semio_viz_scale_band_range_prep:n` / `\semio_viz_scale_range_from_index:nN` (no caller left).

### 1.3 Legacy removal
- `semio-viz-layout.sty` and the `semio-viz-axis.sty` shim **deleted**; their `\RequirePackage` lines
  removed from `semio-viz.sty`.
- `semio-viz-mark.sty`: `\semio_viz_mark_demo:n`, `\semio_viz_path_demo:n`, `\semio_viz_polar:nn`
  **deleted**. `\SemioVizDemo` in the loader now draws through `\semio_viz_mark_draw:nnn` with the
  demo polyline as a named constant.
- The `semio viz rule` TikZ style the deleted files used to provide is now declared in the loader's
  `StyleVocabulary` region, built from the theme's own chrome colour and hairline weight.
- `🧾️template/📊️viz-api/🔓️viz-api.tex` ported off the legacy two-argument `\SemioVizLayout`:
  `{dot}` → `\SemioVizChart{dot-plot}`, `{grid}` → `\SemioVizChart{icon-array}[columns=5]`,
  `{bar}` → `\SemioVizChart{horizontal-bar-chart}`. A new section demonstrates the *new*
  `\SemioVizLayout{grid}{demo-graph}{apigrid}[columns=3]` feeding `\SemioVizPlot`. Also fixed the
  mark `draw=` → `stroke=` and the legend `legend=` → `kind=` keys.

### 1.4 The plot pipeline actually runs
`semio-viz-plot.sty` shipped a placeholder `\semio_viz_plot_kernel_use:nnn` indirection onto nine
kernel entry points that **no package ever defined** (`semio_viz_coordinate_begin:nn`,
`semio_viz_mark_series:nnn`, `semio_viz_guide_draw:n`, …), so `\SemioVizPlot` raised
`missing-kernel` on first use and the API document could not build. The Pipeline region is replaced
by a real engine (418 lines) that composes the existing kernel:

- `🔖️Frame` — the plot rectangle from the figure extent and `semio-viz-guide`'s `\l_semio_viz_pad_fp`
- `🔖️Scales` — `plot-x` (band over a non-numeric column, otherwise linear) and `plot-y` (linear,
  anchored at zero) registered with `semio-viz-scale`, so `\SemioVizAxis[scale=plot-x]` describes
  exactly what the marks drew
- `🔖️Coordinate` — `semio-viz-coordinate` sized to that rectangle and given the two data domains; a
  band domain is the half-open index interval `[0.5, n+0.5]`, so index *i* lands on the centre of
  band *i* and ticks and marks agree by construction
- `🔖️Layers` — every row mapped through `\semio_viz_coordinate_map:nnNN` and painted by the mark
  kernel: `bar` as a `rectangle` mark (area + aspect), `line`/`area` as `\semio_viz_path_draw:nn`
  over the collected series, everything else as the §0 mark of that name; `point` resolves to the
  §0 `dot`
- `🔖️Chrome` — `guide={axis=x, axis=y, grid, legend}` drawn through `\SemioVizAxis`/`\SemioVizGrid`/
  `\SemioVizLegend`, `annotation=` through `\SemioVizAnnotate`
- `transform=` → `\semio_viz_transform:nnn`, `layout=` → `\semio_viz_layout_run:nnnn`,
  `theme=` → `\semio_viz_theme_use:n`

Two bugs found on the way: the `unknown` key handler stored the literal token `\l_keys_key_str` as
the channel name instead of its value (so every encoding collapsed onto one key), and
`\semio_viz_theme_stroke:n { hairline }` never matched because `\str_case:nnF` compares the spaces
too. Both fixed.

### 1.5 Defects handed back
- `semio-viz-plot.sty` `\ProvidesPackage` description: **checked, clean** (no control sequences).
- `catalog-coverage/🟦️.ts`: `defineTestAdapter` default export added, all 8 scenarios wired
  (`schema`, `leaves`, `slugs`, `families`, `options`, `distinctness`, `languages`, `generated`).
  `leaves` is a genuine differential (markdown-it leaf counts vs the catalogue's `covers` counts);
  the conformance scenarios compare an empty finding list against the findings actually made.
- `charts-bar-layout` adapter: **verified present** (`🧪️tests/charts-bar-layout/🟦️.ts`).
- `\SemioVizProbeValuesExpanded`: the probe package already carries `\SemioVizProbeEval{key}{clist}`,
  which evaluates each expression on the emission grid — that *is* the expanding variant, and
  `probe-protocol` uses it. Nothing added; the public API stays as it is.
- `test fundamental` `unknownOptions`: the schema's `x-semio-family-options` was rebuilt
  (151 → 245 families, 2573 option keys, en+de descriptions, `x-semio-demo-tables` 11 → 77).
- `catalog-coverage` distinctness: `75/tree-layout` and `7/unordered-tree` carried the identical
  `tree` option list. The §75 kernel entry now demonstrates the layout with `node=box, labels=true`.
- The schema requires `options.variant` on every entry (architecture §4: two kinds of one family with
  different options must render differently). 1276 of 1738 entries had none; every one now carries
  `variant = <slug>`, the convention the 462 entries that already had one follow.

---

## 2. Verified runs

### `bun ./📜️script.ts generate viz`
```
print: wrote 82 visualization catalogue artifacts
```

### `bun ./📜️script.ts test fundamental`
```
[DEBUG] print: viz coverage 1966/1966 leaves through 1738 kinds, API 21/21
[DEBUG] print: 236 families still awaiting a \SemioVizFamily registration: mark-arc mark-connector mark-area mark-line mark-annotation mark-point mark-region bar…
[DEBUG] print: unit tests passed
[test] level=fundamental cases=67 executed=15 passed=15 failed=0 errored=0 parity=7/7 not-exercised=65
exit=0
```

### `bun ./📜️script.ts build viz api`
```
note: Writing `…\dist\viz-api-dark.blg` (231 B)
[DEBUG] print built 🧰️framework\🛍️products\📓️print\📦️packages\🟦️typescript\dist\viz-api-dark.pdf
```
(both themes, light and dark)

---

## 3. Left for the running agents

Recorded in `📓️integration.md` under "From INTEGRATION-1".

---

## 4. Files touched

Deleted: `🖋️latex/semio-viz-layout.sty`, `🖋️latex/semio-viz-axis.sty`.

Edited: `🖋️latex/semio-viz.sty`, `semio-viz-plot.sty`, `semio-viz-theme.sty`, `semio-viz-scale.sty`,
`semio-viz-transform.sty`, `semio-viz-mark.sty`, `semio-viz-coordinate.sty` (none), `semio-viz-guide.sty`,
`semio-viz-probe.sty`, `semio-viz-charts-bar.sty`, `semio-viz-charts-scatter.sty`,
`semio-viz-charts-distribution.sty`, `semio-viz-charts-statistical.sty`, `semio-viz-charts-polar.sty`,
`semio-viz-matrix-heatmap.sty`, `semio-viz-matrix-correlation.sty`, `semio-viz-table.sty` (prefix rename only),
`semio-viz-matrix-adjacency.sty` (prefix rename only),
`🖼️assets/🔣️viz-catalog.json`, `🧬️schema/🔣️.json`, `🧬️schema/🟦️.ts`,
`🧾️template/📊️viz-api/🔓️viz-api.tex`, `🧪️tests/catalog-coverage/🟦️.ts`,
plus the generated `semio-viz-catalog.sty`, `semio-viz-catalog-labels.sty` and the gallery templates.

Ticket scripts kept: `🔧️move-seq-statistics.py`, `🔧️move-pending-transform.py`,
`🔧️splice-plot-pipeline.py`, `🔧️cover-orphan-leaves.py`, `🔧️catalog-variant-key.py`.
