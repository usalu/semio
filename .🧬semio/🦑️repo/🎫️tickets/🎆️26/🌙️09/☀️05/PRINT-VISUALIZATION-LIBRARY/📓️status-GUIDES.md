# Status — GUIDES

Owner of `semio-viz-guide`, `semio-viz-annotation`, `semio-viz-label`, `semio-viz-facet`,
`semio-viz-composition`, the legacy `semio-viz-axis` shim, and the catalogue entries of taxonomy
sections 50 (annotation-driven), 77 (composition) and the `axis` kinds of section 78.

## Done

### 1. `semio-viz-guide.sty` — axis, legend, grid (+ the frame)
- **Frame** (moved out of legacy `semio-viz-axis.sty`): `\l_semio_viz_pad_fp`,
  `\l_semio_viz_width_fp`, `\l_semio_viz_height_fp`, `\semio_viz_plot_frame:nn`.
- `\SemioVizAxis[scale=,orient=bottom|top|left|right|segment|angular|radial,at=,offset=,from=,to=,
  center=,innerRadius=,outerRadius=,startAngle=,endAngle=,angle=,title=,titleAnchor=,titleGap=,
  ticks=,tickValues=,tickFormat=,tickSize=,tickSizeOuter=,tickPadding=,minor=,minorTicks=,
  minorSize=,grid=,gridLength=,domainLine=,labelRotate=,labelAlign=,labels=,broken=,breakGap=]`.
  Every key is documented with type and default in the `%region 🔖️Keys` block.
- `\SemioVizLegend[scale=,kind=swatch|line|symbol|gradient|size|pattern,title=,orient=,columns=,at=,
  anchor=,itemGap=,swatchSize=,labelGap=,length=,format=,ticks=,symbol=]` — categorical entries from
  the scale domain, gradient/size entries from the scale ticks; horizontal legends advance by the
  label's *measured* width (`\hbox_set:Nn` + `\box_wd:N`), not a guessed column width.
- `\SemioVizGrid[x=,y=,minor=,ticks=,style=]`.
- **d3 tick algorithms**, in `%region 🔖️Pending-semio-viz-scale`:
  `\semio_viz_guide_tick_increment:nnnN` (d3-array's 1/2/5/10 × 10^k step with the
  √2/√10/√50 thresholds), `\semio_viz_guide_ticks_linear:nnnN` (d3-array `ticks`),
  `\semio_viz_guide_ticks_log:nnnnN` (d3-scale log ticks, both branches),
  `\semio_viz_guide_scale_ticks:nnN` (dispatch by scale kind; delegates to
  `\semio_viz_scale_ticks_count:nnN` when GRAMMAR-CORE provides one).
- **One geometry core.** `bottom|top|left|right|segment` all resolve to a segment plus an outward
  unit normal; `angular` and `radial` are the two polar cases. Ternary axes are three `segment`
  axes, so they cost no extra renderer. Broken axes collapse `[lo,hi]` to `breakGap` and draw the
  break glyph; dual axes are `orient=right` plus `offset=`.
- **Family `axis`** (§78 capabilities as catalogue kinds): variants `cartesian`, `polar`, `ternary`,
  `geographic`, `multiple-axes`, `broken-axes`, each consuming the bound table's own columns.

### 2. `semio-viz-annotation.sty`
`\SemioVizAnnotate{kind}[…]` for `label, callout, leader, bracket, brace, highlight (rect|ellipse|
polygon), reference-line, reference-band, reference-point, event-marker, threshold, badge`.
Positioning is data-space first (`x=`/`y=`/`x2=`/`y2=` through `xScale=`/`yScale=`), with `at=`/`to=`/
`from=` as the millimetre escape hatch. Reference lines span the *opposite scale's range*, not the
frame. Family `annotated-chart` renders the thirteen §50 kinds.

### 3. `semio-viz-label.sty`
`\SemioVizLabel[…]` queues, `\SemioVizLabelSeries[points=,labels=,values=,labelFormat=,…]` queues one
label per point, `\SemioVizLabelsPlace[…]` resolves and draws. Greedy solver: descending priority,
candidate 0 is the wanted position and candidates 1… walk up/down/right/left at growing multiples of
`step=`; the first candidate clearing every placed box wins, a label that never clears is hidden
(and reported as `geometry/label-hidden`), and a displaced label gets a leader line.
Text extents are measured, not estimated.

### 4. `semio-viz-facet.sty`
`\SemioVizFacet[wrap=,row=,column=,columns=,sharedX=,sharedY=,gap=,header=,headerGap=,width=,height=,
xScale=,yScale=,pad=]{body}` (and environment `VizFacet`). Each panel is a scope with the frame
variables rebound to the cell and the panel scales re-ranged onto it, so a panel body is the same
code as a whole-figure body. `\SemioVizFacetValue`, `\SemioVizFacetIndex`, `\SemioVizFacetAxes`.

### 5. `semio-viz-composition.sty`
`VizLayer`/`\SemioVizLayer`, `VizOverlay`/`\SemioVizOverlay`, `VizConcat` + `\SemioVizConcatItem`,
`\SemioVizInset[at=,width=,height=,zoom=,frame=,link=,pad=]`, `VizDashboard` + `\SemioVizCell`
(with `colspan`/`rowspan`), `\SemioVizCrossLine`. All five operators share one sub-frame mechanism
(`\semio_viz_comp_push:nnnn` / `\semio_viz_comp_pop:`, stack-based so compositions nest).
Family `composition` renders the nineteen §77 kinds.

### 6. `semio-viz-axis.sty`
Reduced to a one-line `\RequirePackage{semio-viz-guide}` shim. **Delete it** together with
`semio-viz-layout.sty` — legacy layout is its only remaining consumer.

### 7. Catalogue
`🗂️catalog-update-guides.py` (ticket root, idempotent, reads-then-writes atomically) rewrote the 38
GUIDES-owned entries of `🖼️assets/🔣️viz-catalog.json`:
- §78 `axis` kinds → `family: axis`, `namespace: kernel/guide`, one distinct option set each;
  **added the missing `78/polar`** (the taxonomy lists it, the catalogue did not).
- §50 → `family: annotated-chart`, `namespace: kernel/annotation`.
- §77 → `family: composition`, `namespace: kernel/composition`; **added the missing
  `77/zoom-inset` and `77/small-multiples`**.
Every entry's `options` differ per kind, and the gallery probe below shows the projections differ.

### 8. Tests (`🧰️framework/🛍️products/📓️print/🧪️tests/`)
| case | scenarios | mode |
|---|---|---|
| `guide-axis-ticks` | `linear-ticks`, `log-ticks`, `band-ticks` (differential vs d3-array/d3-scale), `axis-geometry` (conformance), `tick-format-labels` (differential vs d3-format, en + de) | mixed |
| `guide-legend` | `categorical-entries` (differential vs d3-scale), `vertical-layout`, `size-entries` | mixed |
| `annotation-placement` | `data-space-anchors` (differential vs d3-scale), `reference-extent`, `bracket-normal` | mixed |
| `facet-layout` | `wrap-grid`, `shared-axes` (`@no-oracle-facet-grid`) | conformance |
| `composition-concat-inset` | `concat-horizontal`, `inset-rectangle`, `dashboard-span` (`@no-oracle-composition-geometry`) | conformance |

Each case has `🥒️.feature` (vectors in the Gherkin table), `🟦️.ts` against the TESTS-HARNESS §1
contract, and committed `🧫️fixtures/*.tex`.

## Verification actually run

Local xelatex (MiKTeX), `TEXINPUTS` at `🖋️latex`, until `🔨️modules/🧪️viz-probe/🟦️.ts` exists.
All thirteen committed fixtures compile with **0 errors** and emit the expected records:

```
linear-ticks          errors=0 records=9
log-ticks             errors=0 records=2
band-ticks            errors=0 records=2
tick-format-labels    errors=0 records=12
axis-geometry         errors=0 records=15
categorical           errors=0 records=11
size                  errors=0 records=3
data-space            errors=0 records=6
bracket               errors=0 records=1
wrap                  errors=0 records=56
concat                errors=0 records=2
inset                 errors=0 records=2
dashboard             errors=0 records=5
```

Tick values against d3 (all agreeing):

```
ticks/0  d3.ticks(0,10,5)      [0,2,4,6,8,10]
ticks/1  d3.ticks(0,1,5)       [0,0.2,0.4,0.6,0.8,1]
ticks/3  d3.ticks(-5,5,4)      [-4,-2,0,2,4]
ticks/5  d3.ticks(0,100,7)     [0,20,40,60,80,100]
ticks/7  d3.ticks(10,0,5)      [10,8,6,4,2,0]
ticks/8  d3.ticks(2,7,3)       [2,4,6]
log/0    scaleLog([1,100])     [1..10,20..100]
band     scaleBand(A..E,8..78) [15,29,43,57,71]
labels   .1f  en 0.0 0.2 …     de "0,0" "0,2" …
```

Geometry (80×40 frame, pad 8, `scale=x` linear 0..10 → 8..78):
`axis-domain [8,8,78,8]`, `axis-tick-positions [8,22,36,50,64,78]`, `grid-line [8,8,8,38]`.
Facet on a 120×60 block, `wrap={A,B,C,D} columns=2 gap=4`:
panels `[0,0,32,58,28] [1,62,32,58,28] [2,0,0,58,28] [3,62,0,58,28]`, 4 axes drawn (shared).
Dashboard `columns=3 rows=2 gap=3` with one `colspan=2`: cells `0,1,3,4,5` — the spanning cell
consumes two slots, as specified.
Label solver on five colliding points: 8 placed, 0 hidden, 3 leaders drawn.

Gallery probe of **all 38 GUIDES catalogue kinds** (13 §50 + 19 §77 + 6 §78 axis):
`gallery.tex` → 12 pages, 0 errors, 1364 probe records, every variant with distinct geometry.

Coexistence check: `semio-viz-layout` (legacy) + `semio-viz-composition` in one document → no
`already defined` clash, 1 page. Each of the six packages also loads standalone on a bare
`\documentclass{article}` with 0 errors.

## Decisions

- **Frame state lives in `semio-viz-guide`.** The guides are what turns a canvas into a plot area.
  `\l_semio_viz_width_fp` / `\l_semio_viz_height_fp` are created in an `\AddToHook{begindocument}`
  with `\fp_zero_new:N` because legacy `semio-viz-layout.sty` declares the same two variables and is
  loaded *after* the guide; the hook runs after every package and only creates what legacy did not.
  **Delete that hook and declare them plainly once `semio-viz-layout.sty` is gone.**
- **One geometry core for every straight axis** (see §1). An orthogonal axis reproduces
  `\semio_viz_scale_map:nnN` exactly; a ternary edge or an arbitrary segment costs nothing extra.
- **Polar arcs are sampled polylines**, not TikZ `arc[]`: with `x=1mm,y=1mm` a unitless polar
  coordinate is not portable, and a sampled arc is what the probe can measure numerically.
- **Composition operators share one sub-frame stack**, so a plot in a concat cell, a dashboard cell,
  an inset or a facet panel is literally the same code as a plot in a whole figure.
- **Theme access.** Guides read colour, stroke and font through `semio-viz-theme`
  (`\semio_viz_theme_color:nN`, `\semio_viz_theme_ramp_color:nN`, `\semio_viz_theme_stroke:n`),
  never through a hard-coded token. The one exception is the chrome role (border/foreground), which
  the theme does not expose yet — see the request below.

## Requests to other agents

1. **GRAMMAR-CORE — `semio-viz-theme`: add a chrome-role accessor.** The theme exposes categorical
   colours, ramps, strokes, fonts and patterns, but no chrome role, so the guides resolve
   border/foreground themselves in `semio-viz-guide.sty`'s `%region 🔖️Pending-semio-viz-theme`
   (`\semio_viz_guide_chrome:nN {border-normal|border-emphasized|foreground|canvas} \tl`, resolved
   for `\semio_viz_theme_appearance:N` straight from `semio-tokens`). Please move it in as
   `\semio_viz_theme_chrome:nN` and I will delete mine.
2. **GRAMMAR-CORE — `semio-viz-theme` must `\RequirePackage{semio-fonts}`.** It calls `\SemioSans`,
   `\SemioSerif`, `\SemioMono` in `\semio_viz_theme_font:n` but does not load them. I load
   `semio-fonts` from `semio-viz-guide` for now (the guide typesets, so it needs them either way).
3. **GRAMMAR-CORE — beware Python heredocs mangling backslashes on this machine.** Three of your
   files carried damage from it while I was building against them: literal `0x08` bytes where `\b`
   was meant in `semio-viz-format.sty` (6 occurrences at one point — `\bool_new` became `^^Hool_new`),
   `#RRGGBB` unescaped inside a `\msg_new:nnn` in `semio-viz-theme.sty`, and
   `\c_semio_viz_scale_e5` in `semio-viz-scale.sty` (digits are *not* letters under expl3 catcodes,
   so a digit inside a variable name splits the control sequence). All three appear fixed now; use
   the Write tool rather than a `python - <<EOF` heredoc for anything containing backslashes.
4. **CATALOG — delete `semio-viz-axis.sty` together with `semio-viz-layout.sty`.** The shim exists
   only for legacy layout's `\RequirePackage`. Tell me in `📓️status-CATALOG.md` when legacy is gone
   and I will drop the shim and the begindocument hook named under Decisions.
5. **CATALOG — `50/threshold-chart` (and siblings) now carry `"data": "demo-multiseries"`.** My
   `annotated-chart` family binds `data=`, `x=cat`, `y=val`; whatever table the catalogue binds must
   carry those columns (or the entry must bind its own `x`/`y`). Ping me if `demo-multiseries` has a
   different column vocabulary and I will adjust the family defaults.
6. **TESTS-HARNESS — the five cases above are written against §1 as published.** Two declare
   `@no-oracle-facet-grid` / `@no-oracle-composition-geometry`: d3 has no faceting and no
   composition layer, so the panel and cell arithmetic is specified in the feature's data table
   rather than adjudicated. Please register those `noOracleDecisions`.

## Files touched

Created / rewritten:
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-guide.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-annotation.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-label.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-facet.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-composition.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-axis.sty` (reduced to a shim)
- `🧰️framework/🛍️products/📓️print/🧪️tests/guide-axis-ticks/{🥒️.feature,🟦️.ts,🧫️fixtures/*.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/guide-legend/{🥒️.feature,🟦️.ts,🧫️fixtures/*.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/annotation-placement/{🥒️.feature,🟦️.ts,🧫️fixtures/*.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/facet-layout/{🥒️.feature,🟦️.ts,🧫️fixtures/wrap.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/composition-concat-inset/{🥒️.feature,🟦️.ts,🧫️fixtures/*.tex}`

Edited (only the GUIDES-owned entries):
- `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json`

Ticket scripts (kept):
- `🗂️catalog-update-guides.py`, `🗂️theme-rewire-guides.py`, `🗂️fixture-rewire-guides.py`

Ticket probes kept under `🗑️generated/GUIDES/` (inputs, no outputs left):
`axis.tex`, `fam.tex`, `full.tex`, `gallery.tex`, `render.tex`.

## Open issues

- The five cases cannot be run through the platform until `🔨️modules/🧪️viz-probe/🟦️.ts` and the
  repo tectonic path exist; everything was verified by compiling the same fixtures with xelatex and
  reading `\jobname.probe.jsonl`. The numbers above are those files' real contents.
- No temporal scale exists yet, so there is no time-axis tick scenario. `\SemioVizAxis` already
  routes to `\semio_viz_scale_ticks_count:nnN` when the scale package offers it — the day
  `semio-viz-scale` gains `temporal`, add a `time-ticks` scenario against `d3-time`/`d3-time-format`
  and nothing in the guide needs to change.
- `geographic` axes are currently a lon/lat graticule over linear scales; a real projection needs
  GEO-SPATIAL's `\SemioVizCoordinate{geographic}`.
