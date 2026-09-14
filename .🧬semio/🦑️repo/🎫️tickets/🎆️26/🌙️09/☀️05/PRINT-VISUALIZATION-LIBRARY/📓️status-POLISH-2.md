# POLISH-2 — status

Agent: POLISH-2. Ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`.
Continues POLISH's audit backlog (`📓️status-POLISH.md` §5) plus the kernel defects
FAMILIES-CAPABILITY handed over (`📓️status-FAMILIES-CAPABILITY.md` §5,
`📓️integration.md` `## From FAMILIES-CAPABILITY`).

Scope (owned): every `🖋️latex/semio-viz-*.sty` **except** FAMILIES-DOMAIN's set
(`text domain diagram-* scientific-* network-* infographic interactionstate`), plus
`semio-viz-showcase.sty` (FAMILIES-CAPABILITY finished), the loader `semio-viz.sty`,
the token generator and the catalogue/schema entries of the owned families.
Not owned: `🧪️tests/*`, `🔮️oracle/🔣️.json`, `🔨️modules/🧪️viz-probe` (TESTS-2).

Note: `📓️audit-consistency.md`, referenced by the briefing, does not exist in the ticket
folder; the audit classes are reconstructed from `📓️integration.md` line 126 and
`📓️status-POLISH.md` §0. The repo MCP server did not connect this session
(`repo (CONNECTION_CLOSED)`), so the ticket could not be reopened through `ticket_reopen`;
the folder was worked in directly.

---

## 1. Task list and state

| # | task | state |
|---|---|---|
| 1a | `rotate` on marks — catcode-11 colon in `rotate around={a:(x,y)}` | **done, verified** |
| 1b | `\semio@viz@font@*` undefined; showcase `Pending-semio-viz-mark` deleted | **done** |
| 1c | `semio-viz-transform` `bin` dies on the `demo` table | **done, verified** |
| 1d | `semio-viz-layout.sty` `\msg_error:nnnx` does not exist | **done** |
| 1e | geo families emit indistinguishable / no probe geometry | **done, verified** |
| 1f | six chart/matrix packages called `\semio_viz_tr_*` without requiring the transform kernel | **done** |
| 2 | docstrings — **721 gaps in 30 owned files → 0** | **done** |
| 3 | unread keys — 30 sites: implemented or deleted with their schema and catalogue uses | **done, verified** |
| 4 | hard-coded loop counts → documented keys | **done, verified** |
| 5 | split `semio-viz-hierarchy.sty` into hierarchy + tiling + packing | **done, hashes identical** |
| 6 | verification | see §5 |

---

## 2. Landed

### 2.1 `rotate` on marks (FAMILIES-CAPABILITY's first item)

`\semio_viz_mark_emit:n` built `rotate~around={ <angle> : ( <x>mm , <y>mm ) }` inside
`\ExplSyntaxOn`, where `:` is a **letter**, so TikZ never split the angle from the pivot and every
rotated mark died on `! File ended while scanning use of \tikz@doparseA`. The option is now built
into `\l_semio_viz_mark_rotate_tl` with `\tl_set:Nx` and `\c_colon_str` — a catcode-12 colon — and
handed to a new `\semio_viz_mark_scope_begin:n` (variant `V`) that opens the scope with the already
expanded text. `\semio-viz-mark.sty` is the only file touched.

**Probe check.** The rotated mark's geometry already differs from the unrotated one without any new
record: `\semio_viz_mark_draw:nnn` emits `geometry/mark/<kind>/at` = `x, y, rotate, size`
(`semio-viz-mark.sty:2168`), so the angle is in the probe stream for every §0 mark. A first draft of
this fix added a separate `mark/rotate` record; it was **removed again** as duplicate information —
the `…/at` record is the check. Verified locally
(`🗑️generated/POLISH-2/rotate.tex`, four marks each way, `grep -c '^!' rotate.log` = 0):

```
geometry/mark/square/at","values":[10,10,0,40]      geometry/mark/square/at","values":[10,10,30,40]
geometry/mark/triangle/at","values":[10,10,0,40]    geometry/mark/triangle/at","values":[10,10,30,40]
geometry/mark/cross/at","values":[10,10,0,40]       geometry/mark/cross/at","values":[10,10,30,40]
geometry/mark/star/at","values":[10,10,0,40]        geometry/mark/star/at","values":[10,10,30,40]
```

The fixture that turns this into a committed scenario belongs in TESTS-2's `✒️mark-geometry`; the
request is in `📓️integration.md` under `## From POLISH-2`.

### 2.2 `\semio@viz@font@{cell,chip,label,title}` — used 141 times, defined in one rarely loaded file

`semio-viz-theme.sty` gained a `%region 🔖️FontSwitches` with the four switches, each one line and
each nothing but the `\c_semio_viz_theme_font_*_tl` constant beside it. `semio-viz-scientific-field.sty`
requires `semio-viz-theme` **before** its own three `\providecommand`s, so those become no-ops and
nothing is redefined; the definition order needs no guard. FAMILIES-CAPABILITY's
`%region 🔖️Pending-semio-viz-mark` in `semio-viz-showcase.sty` is deleted.

These four switches are a compatibility surface and come out of the theme the moment FAMILIES-DOMAIN's
files read the constants; the exact per-file counts are in `📓️integration.md` under `## From POLISH-2`.

### 2.3 `bin` on the `demo` table — a d3 conformance defect, not a table defect

Reproduced and then narrowed: the table is irrelevant, the trigger is `thresholds = count: 8` over a
short integer column. `\semio_viz_scale_tick_increment:nnnN` returns d3's **negative** step
convention (−2 for a half-unit step), and `\semio_viz_tr_bin_ticks:` read it as

```
xhi = -( ( floor( xhi * -step ) + 1 ) / -step )     % (8) -> -8.5
```

so the upper bound went negative, the trim loop emptied the edge sequence and the run died on an
expandable-error marker (`! Use of \??? doesn't match its definition`). d3-array's own line is
`x1 = (Math.ceil(x1 * -step) + 1) / -step` — **ceil, and not negated**; the file's own second copy of
the routine, `\semio_viz_tr_stat_bin_extend:n`, already had it right. Corrected, and the `step < 0`
guard of the twin adopted so a zero step cannot divide.

```
$ xelatex bin.tex           # demo-distribution and demo, thresholds = count: 8
[DEBUG] t1 ok
[DEBUG] t2 ok               # was: ! Use of \??? doesn't match its definition.
```

### 2.4 `semio-viz-layout.sty`

`\msg_error:nnnx` does not exist; `\msg_error:nnnn` does, and l3msg expands the message arguments
when the message is typeset, so the registered-algorithm list still expands. One-token fix.

### 2.5 Geo families were indistinguishable to the probe

`geo-basemap`, `geo-choropleth`, `geo-symbol` and `geo-dotdensity` emitted only the `geometry/path`
records of the shared outline, and `geo-field` emitted nothing at all. Each family now emits the
layer that **is** the family:

| family | records |
|---|---|
| `geo-basemap` | `geo/basemap/layers` (the five layer flags and the opacity), `geo/basemap/land` per part (centroid, value, relief ramp position or −1), `geo/basemap/place`, `geo/basemap/label` |
| `geo-choropleth` | `geo/choropleth/scale` (class rule, step count, bivariate flag), `geo/choropleth/class` per part (centroid, value, ramp position) |
| `geo-symbol` | `geo/symbol` per anchor (x, y, symbol size, value) |
| `geo-dotdensity` | `geo/dot` per dot (x, y, radius) |
| `geo-field` | `geo/field/ring` per isoline ring (level, level count, vertex count, the vertices) |

The two probe passes that were added (`\semio_viz_geomap_probe:`, `\semio_viz_chor_probe:`) run only
under `\g_semio_viz_probe_geometry_bool`, so a normal document does not pay for them. Verified with
`🗑️generated/POLISH-2/geo.tex`, five scenarios, `grep -c '^!' geo.log` = 0:

```
basemap    geometry/geo/basemap/land 4   geometry/geo/basemap/layers 1   geometry/path 7
choropleth geometry/geo/choropleth/class 4  geometry/geo/choropleth/scale 1  geometry/path 4
dotdensity geometry/geo/dot 25              geometry/path 4
field      geometry/geo/field/ring 10
symbol     geometry/geo/symbol 4            geometry/path 4
```

### 2.6 Six packages called the transform kernel without requiring it

FAMILIES-CAPABILITY's fifteen `parity long` failures. `\RequirePackage{semio-viz-transform}` added to
`semio-viz-charts-distribution`, `-charts-statistical`, `-charts-polar`, `-matrix-heatmap`,
`-matrix-correlation`, `-matrix-adjacency` — each of the six calls `\semio_viz_tr_*` itself, so each
requires it itself rather than relying on the chain through `charts-distribution`.

### 2.7 Docstrings: 721 → 0

The corrected checker was rewritten as `🗑️generated/POLISH-2/docstrings.py` (POLISH's rule: walk the
whole comment block upwards, step over blank lines, variable declarations and variant generation,
stop at a `%region` marker or at another definition; only definitions at column 0 count). It runs
over the POLISH-2 owned set — POLISH's 52 files plus `semio-viz-showcase.sty`.

| stage | gaps |
|---|---|
| start | 721 in 30 files |
| after the small and geo files | 421 |
| after the chart, composition, flow and data files | 277 |
| after `semio-viz-mark.sty` and `-spatial.sty` | 124 |
| **now** | **0** |

Every note is authored against the definition it sits above; none is templated, and none sits inside
a definition body. Six notes that opened with a **non-emoji** glyph were corrected on the way through
(`⣿`, `┴`, `⋮`, `▦`, `▬`, `‖`, `∥`-style mathematical and box-drawing characters are not emoji, and
the checker is right to reject them).

### 2.8 Unread keys (task 3)

30 declared keys never reached a renderer. Each was either implemented — preferred wherever the
schema documented the option — or deleted together with its `x-semio-family-options` entry and every
catalogue use.

Implemented: `density` (hexbin, plus `\l_semio_viz_hex_total_int`), `sparse` (heatmap, empty cells
outlined when false), `labels` (scatter-matrix outer captions), `split` (quadrant), `smooth`
(residual), `curve` (parallel), `hulls` (embedding), `dasymetric` + `weight` (choropleth),
`kernel` (density plot), `distance-max` (force layout), `value` (geo ring), `label`
(directly-labeled chart), and the layer name in the composition probe record.

Deleted: `coxcomb.angle`, `parliament.party`, `marimekko.part`, `waffle.part`, `parallel.label`,
`glyph.label`, `radial-partition.node`, `radial-partition.parent`, `uncertainty.orient`,
`control.timecol`, `facet.shared`, `hexbin.levels`, `funnel.orient`, `geo-cartogram.iterations`,
`grammar.source`, `grammar.target`, `encoding.category`, `shape.x`, and the package-only
`coordinate` of `semio-viz-network.sty` (no schema entry existed for it).
`showcase`'s `namespace` and `data` are deliberately kept: the catalogue binds them.

Every default was chosen so that the currently rendered output does not move (`density=false`,
`sparse=true`, `split=0.5`, `curve=false`, `hulls=false`, `dasymetric=false`, `distance-max=-1`).

### 2.9 Hard-coded loop counts (task 4)

| file | was | key | default |
|---|---|---|---|
| `semio-viz-annotation.sty` | `3` ×3 (explainer, story, sequence) | `steps` | 3 |
| `semio-viz-charts-bar.sty` | `5` ×2 (grid x, grid y) | `gridLines` | 5 |
| `semio-viz-charts-bar.sty` | `5` (value ticks) | `ticks` | 5 |
| `semio-viz-composition.sty` | `6` cells, `columns = 3 , rows = 2` | `panels`, `panelColumns` | 6, 3 |
| `semio-viz-geo.sty` | `12` Newton steps of the equal-earth inverse | `invertSteps` | 12 |
| `semio-viz-geo.sty` | `48` halvings of the clip bisection | `clipSteps` | 48 |
| `semio-viz-guide.sty` | `2` slashes of a broken axis | `breakMarks` | 2 |
| `semio-viz-guide.sty` | `3` hatch lines of a pattern swatch | `hatchLines` | 3 |
| `semio-viz-scale.sty` | `10` rounds of `nice()` | `niceIterations` | 10 |

Each spacing divisor moved with its count (`/ 4` → `/ max(1, n - 1)`, `/ 4` → `/ (n + 1)`,
`- 1.5` → `- (n + 1) / 2`), so the default value reproduces today's geometry exactly, and the
dashboard grid derives its row count from `panels` and `panelColumns` instead of naming it twice.
The two solver counts are not per-family options: they are set through the new
`\SemioVizGeoSolver[…]` and `\SemioVizScaleSolver[…]`, whose key modules are
`semio / viz / geo / solver` and `semio / viz / scale / solver`.

`gridLines` and `ticks` were added to the schema for all 21 families that inherit the shared
cartesian vocabulary (`ticks` not for `gauge`, which declares its own arc-tick count and therefore
overrides the shared key). `annotated-chart` has no `x-semio-family-options` entry — it is not a
catalogue family — so `steps` is documented in the package only.

Out of scope, left alone and deliberately: the fixed `3` of `semio-viz-showcase.sty:1057` and
`:2129` (a three-column record grid and a three-bar mini chart of the illustration itself), and the
literal counts in `semio-viz-charts-dashboard.sty`, `-charts-scatter.sty` and `-charts-timeline.sty`,
which belong to CHARTS-A.

### 2.10 Splitting `semio-viz-hierarchy.sty` (task 5)

3 173 lines → 1 855 + `semio-viz-hierarchy-tiling.sty` (635: `LayoutTreemap`, `LayoutPartition`,
`VoronoiTreemap`) + `semio-viz-hierarchy-packing.sty` (712: `LayoutPack`). The regions were moved
byte-for-byte; `🗑️generated/POLISH-2/split_check.py` proved beforehand that the three chunks contain
no load-time code at all and that the only edges between them are

```
treemap -> voronoi (1): \semio_viz_hierarchy_tm_voronoi:nn
voronoi -> treemap (5): \l_semio_viz_hierarchy_tm_{n_int,xa_fp,xb_fp,ya_fp,yb_fp}
rest    -> treemap (2): \semio_viz_hierarchy_layout_{partition,treemap}:n
rest    -> pack    (1): \semio_viz_hierarchy_layout_pack:n
```

so treemap and voronoi had to stay in one file and the parts can be required before the body.
`\RequirePackage{semio-viz-hierarchy-tiling}` and `…-packing` sit in `semio-viz-hierarchy.sty`'s own
header; the loader `semio-viz.sty` is untouched and pulls both in transitively. Construction,
aggregates, traversal, the tree and cluster layouts, the layout registrations, the dispatch, the
public API and the renderer stay in `semio-viz-hierarchy.sty`.

## 3. Decisions

- **Rotated marks stay recorded in d3 space.** `\l_semio_viz_path_cmd_tl` is deliberately the token
  stream of d3's own SVG path string (`semio-viz-mark.sty:52-57`) and the oracles tokenise it;
  rotating those numbers would make every symbol oracle disagree. The rotation is testable through
  the `…/at` record instead.
- **The four `\semio@viz@font@…` switches live in the theme, not in four shims.** One definition
  each, beside the constant each one names, and deleted once FAMILIES-DOMAIN migrates.
- **`\semio_viz_tr_bin_ticks:` was corrected against d3, not against the symptom.** The file's own
  `_stat_bin_extend:n` is the reference implementation, and the two now agree line for line.
- Geo probe passes are guarded by the probe flag, so the added records cost a normal document nothing.

## 4. Files touched

LaTeX (all through Write/Edit, never a shell heredoc; `grep -c '^\\'` checked after the functional
edits):

- `semio-viz-mark.sty` — rotate scope built with `\c_colon_str`; 89 docstrings.
- `semio-viz-theme.sty` — `%region 🔖️FontSwitches`; 3 docstrings.
- `semio-viz-showcase.sty` — `Pending-semio-viz-mark` deleted; 9 docstrings.
- `semio-viz-layout.sty` — `\msg_error:nnnn`.
- `semio-viz-transform.sty` — `bin` upper bound corrected, `\l_semio_viz_tr_stat_datamax_fp` so the
  uniform-width rescue reads the data maximum and not the niced bound (§5.4), kernel switch of the
  KDE; 9 docstrings.
- Task 4 keys: `semio-viz-annotation.sty` (`steps`), `-charts-bar.sty` (`gridLines`, `ticks`),
  `-composition.sty` (`panels`, `panelColumns`), `-geo.sty` (`semio / viz / geo / solver`,
  `\SemioVizGeoSolver`), `-guide.sty` (`breakMarks`, `hatchLines`), `-scale.sty`
  (`semio / viz / scale / solver`, `\SemioVizScaleSolver`).
- Task 5: `semio-viz-hierarchy.sty` trimmed to its own regions and given the two new requires.
- `semio-viz-geo-map.sty`, `-geo-choropleth.sty`, `-geo-symbols.sty`, `-geo-contours.sty` — probe
  geometry per family; docstrings.
- `semio-viz-charts-distribution.sty`, `-charts-statistical.sty`, `-charts-polar.sty`,
  `-matrix-heatmap.sty`, `-matrix-correlation.sty`, `-matrix-adjacency.sty` — `\RequirePackage`.
- Docstrings only: `semio-viz-data.sty`, `-scale`-adjacent files, `-shape.sty`, `-guide.sty`,
  `-annotation.sty`, `-label.sty`, `-composition.sty`, `-plot.sty`, `-hierarchy.sty`, `-network.sty`,
  `-flow.sty`, `-geo.sty`, `-spatial.sty`, `-geo-projection.sty`, `-geo-routes.sty`, `-flow-sankey.sty`,
  `-charts-area.sty`, `-charts-bar.sty`, `-charts-dashboard.sty`, `-charts-financial.sty`,
  `-charts-funnel.sty`, `-charts-line.sty`, `-charts-scatter.sty`, `-charts-timeline.sty`.

Ticket:

- `📓️status-POLISH-2.md` (this file), `📓️integration.md` `## From POLISH-2`.
- `🗑️generated/POLISH-2/` — `docstrings.py` (the checker), `split_check.py` (the dependency proof of
  task 5), `split_hierarchy.py` and `split_trim.py` (the split itself), `schema_loopkeys.py` and
  `catalog_dropkeys.py` (the order-preserving JSON edits), and the reproductions `bin.tex`,
  `bin2.tex`, `bin3.tex`, `rotate.tex`, `geo.tex`, `keys.tex`, `loops.tex`, `hist/extend.tex`.
  Every compiler output (logs, aux, pdf, probe streams, command tails) was deleted again.

New LaTeX packages:

- `🖋️latex/semio-viz-hierarchy-tiling.sty`, `🖋️latex/semio-viz-hierarchy-packing.sty`.

Assets regenerated (through `generate viz`, never by hand):

- `🖋️latex/semio-viz-catalog.sty`, `🖋️latex/semio-viz-catalog-labels.sty`,
  `🖼️assets/🔣️viz-api.json`, `🧾️template/📊️viz-gallery/*.tex` — 83 artifacts.

Assets edited:

- `🧬️schema/🔣️.json` (task 3 deletions, task 4 additions), `🖼️assets/🔣️viz-catalog.json`
  (three option uses of deleted keys).

## 5. Verification runs

The final state of the tree is §5.3; the runs below are the intermediate ones, kept because each
task was re-verified as it landed.

`bun ./📜️script.ts build viz api`, after the whole of task 1, exit code 0:

```
[DEBUG] print built …\dist\viz-api.pdf
[DEBUG] print built …\dist\viz-api-dark.pdf
exit=0
```

Local probes (MiKTeX, `TEXINPUTS` at `🖋️latex`, `TTFONTS` at `.🧬semio/🦑️repo/⚡️cache/print-fonts` —
without that second variable every compile fails on `The font "Anta-Regular" cannot be found`, which
is not a LaTeX defect):

```
$ xelatex rotate.tex ; grep -c '^!' rotate.log   -> 0
$ xelatex geo.tex    ; grep -c '^!' geo.log      -> 0
$ xelatex bin.tex    ; grep -c '^!' bin.log      -> 0
```

### 5.1 Task 4 — every new key confirmed at runtime, not by reading

`🗑️generated/POLISH-2/loops.tex` draws each affected renderer twice, once on the default and once on
a changed count. Temporary `[DEBUG]` `\typeout`s were added inside the five loops that emit no probe
record, the fixture was compiled, and the logs then removed again (`grep DEBUG` over the five files
is empty):

```
$ xelatex loops.tex ; grep -c '^!' loops.log      -> 0
[DEBUG] cart tick 1 of 5 … 5 of 5 / 1 of 4 … 4 of 4
[DEBUG] break mark 1 of 2, 2 of 2 / 1 of 4 … 4 of 4
[DEBUG] hatch 1 of 3 … 3 of 3 / 1 of 5 … 5 of 5
[DEBUG] invert step 1 of 12 … / 1 of 20 …
[DEBUG] clip step … 52 of 52          (208 crossings on a rotated orthographic globe)
[DEBUG] nice round 1 of 4 … 4 of 4
```

The counts that do reach the probe were read out of `loops.probe.jsonl` instead:

```
geometry/grid            [8,0] [15,0] [22,0] [29,0] [36,0]   then   [8,0] [22,0] [36,0]
geometry/dashboard-cell  6 cells, 3 columns, w=22            then   4 cells, 2 columns, w=38
geometry/annotation-callout  3 records                       then   5 records
```

Endpoints are unchanged in every case, which is what the `max(1, n - 1)` and `(n + 1)` divisors buy.

### 5.2 Task 5 — the split changed no geometry

The five hierarchy fixtures were compiled from the unsplit file, then from the split one, and their
probe streams hashed:

```
                                    before                            after
hierarchy-aggregates.probe.jsonl    fd7e8756640b4bccb41e684e4d5bfdfc  fd7e8756640b4bccb41e684e4d5bfdfc
hierarchy-pack.probe.jsonl          f66d1c6f4f738696b982c6a98e8b2847  f66d1c6f4f738696b982c6a98e8b2847
hierarchy-partition.probe.jsonl     479a22e86e31983c802bfa5a096a5fa7  479a22e86e31983c802bfa5a096a5fa7
hierarchy-tree-cluster.probe.jsonl  90482a269feff6e5084ae0f7d0af7755  90482a269feff6e5084ae0f7d0af7755
hierarchy-treemap.probe.jsonl       3f0813f219c5072b9d977e7616882130  3f0813f219c5072b9d977e7616882130
```

`🧪️tests/🎪️showcase-families/🧫️fixtures/showcase-namespaces.tex`, which goes through the loader,
also compiles with zero errors after the split, so `semio-viz.sty` still reaches both new packages.

### 5.3 Platform

```
$ bun ./📜️script.ts build viz api
[DEBUG] print built …\dist\viz-api.pdf
[DEBUG] print built …\dist\viz-api-dark.pdf
exit=0

$ bun ./📜️script.ts test fundamental
[test] level=fundamental cases=100 executed=15 passed=15 failed=0 errored=0 parity=7/7 not-exercised=98
exit=0

$ bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print"
[test] level=quick cases=100 executed=513 passed=513 failed=0 errored=0 parity=241/241 not-exercised=12
exit=0

$ bun ./📜️script.ts build viz api            # after the binning fix of §5.4
[DEBUG] print built …\dist\viz-api.pdf
[DEBUG] print built …\dist\viz-api-dark.pdf
exit=0
```

The whole owner's quick level runs 100 cases, which covers every package POLISH-2 touched, so no
per-case invocation list is needed; `parity quick` was run over the owner rather than case by case.
`241/241` is the first time this owner has been fully green (TESTS-2 finished at `239/241`).

The first `test fundamental` run failed on two gates that the key deletions of task 3 had opened, and
both were closed rather than weakened:

1. `verifyVisualizationCoverage` reported `contour-scatter-plot:levels`, `kde-contour-plot:levels`
   and `contiguous-cartogram:iterations` — three catalogue kinds still passing options whose schema
   entries had gone. The three options were deleted from `🖼️assets/🔣️viz-catalog.json`
   (`🗑️generated/POLISH-2/catalog_dropkeys.py`; the file round-trips through
   `json.dumps(…, indent=2)` byte-for-byte, so nothing but those three lines moved).
2. `📚️catalog-coverage::generated` then reported `semio-viz-catalog.sty` and `🔣️viz-api.json` stale.
   `bun ./📜️script.ts generate viz` rewrote the 83 catalogue artifacts; the staleness check is now
   empty for every generated file.

### 5.4 `\semio_viz_tr_stat_bin:Nn` appended a bin above the data maximum (TESTS-2's hand-over)

`parity quick` over the whole owner came back `239/241`, failing
`📶️charts-histogram-density::bins` and `::bins-coarse` with six differences each — the defect
TESTS-2 handed to POLISH-2. The cause is a clobbered variable, not the binning rule:

```
\semio_viz_tr_stat_max:NN #1 \l_semio_viz_tr_stat_b_fp        % b_fp = the data maximum
\semio_viz_tr_stat_nice:nnn { …a_fp } { …b_fp } {#2}          % nice() writes its result INTO b_fp
…
\fp_compare:nTF { \l_semio_viz_tr_stat_b_fp >= \g_semio_viz_tr_stat_hi_fp }   % always true
```

d3's `bin.js` compares the **data** maximum (`max`, captured before nicing) against the niced upper
bound, and only extends the bound when the two meet; otherwise it pops the last threshold. Because
`\semio_viz_tr_stat_nice:nnn` hands its result back through the same `…_b_fp`, the comparison was
`hi >= hi` and the rescue branch always won, so every histogram carried one empty bin too many.
The data maximum is now kept in its own `\l_semio_viz_tr_stat_datamax_fp`.

```
                       before (ours)              after (ours) = d3-array
bins (10 thresholds)   lower 2..10, 9 bins        lower 2..9, 8 bins,  counts 4,4,5,7,6,8,9,5
bins-coarse (4)        lower 2,4,6,8,10           lower 2,4,6,8,       counts 8,12,14,14
extend (0..10, 10)     lower 0..10, upper 1..11   unchanged — the rescue branch still fires
```

`🗑️generated/POLISH-2/hist/extend.tex` is the new reproduction of the branch that must keep
extending (`max` exactly on the niced bound); its output equals d3's for the same input.
`parity quick --case 📶️charts-histogram-density` is then `10/10, parity=5/5`.

## 6. Open

Nothing of tasks 1-5. Handed over in `📓️integration.md` `## From POLISH-2`: the
`\semio@viz@font@…` migration table for FAMILIES-DOMAIN (the four aliases can be deleted the moment
those 17 files stop using the legacy names), and one rotated-mark scenario for TESTS-2.
