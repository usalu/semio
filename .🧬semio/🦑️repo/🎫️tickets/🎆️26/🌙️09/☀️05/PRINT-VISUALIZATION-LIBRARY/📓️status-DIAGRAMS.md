# DIAGRAMS — status

Owner of `semio-viz-diagram-flowchart`, `-uml`, `-architecture`, `-process`, `-concept`,
`semio-viz-infographic`, `semio-viz-interactionstate`, and the catalogue entries of sections
14, 15, 16, 36 (schematic kinds), 37, 45, 46, 53, 54, 55, 56 (minus Feynman/knot/circuit/chemical/music),
59, 71, 73.

## Done

### 1. Diagram kernel — `semio-viz-diagram-flowchart.sty`
`%region 🔖️DiagramKernel`, `🔖️Keys`, `🔖️Layout`, `🔖️Shapes`, `🔖️Routing`, `🔖️Draw`, `🔖️Lanes`,
`🔖️Pipeline`. Public entry point `\SemioVizDiagram[options]`; every other package in this
namespace builds on it.

- **Placement** — grid from `row`/`col` columns, extent measured from the table, cell size derived
  from the frame (`node-width=auto` is the default), `direction=right` transposes. NETWORK-FLOW's
  `layered` Sugiyama layout was not yet available when this was written, so the kernel places from
  explicit indices; switching a family to `layered` later is a `\semio_viz_dg_layout:` change only,
  since everything downstream reads the node record prop.
- **Shapes** — process, subprocess, decision, gateway/gateway-x/gateway-plus, terminator, state,
  io/data, manual, document, database, store, connector, circle, event, initial, final, fork, join,
  note, actor, component, package, ellipse/usecase, hexagon, cloud, card, delay, gate-and/or/xor,
  none.
- **Routing** — `orthogonal` (face ports, mid-step, 2 or 4 waypoints), `straight` (centre-ray /
  rectangle intersection), `curved` (`to[out=,in=]` with a `bend` key). Arrow tips from the `arrow`
  key; edge `kind` column selects plain/dashed/dotted/strong/critical.
- **Lanes, groups, legends** — `lanes=row|column` bands with headers, `groups=true` frames the
  distinct values of the group column, `legend` draws a swatch row.
- **Probe** — `\semio_viz_probe_geometry:nn` emits `diagram-node`, `diagram-route`, `diagram-lane`,
  and per-family `diagram-bone|band|message|timing|cpm|cell|block|seat|run|vertex|level|axo|sun|set|card|glyph|panel|brush|frame`.

### 2. Families (30, all registered through `\SemioVizFamily`)
| package | families |
|---|---|
| `-flowchart` | `flow`, `swimlane`, `fishbone`, `logic-tree`, `cycle`, `stage`, `decision-table` |
| `-uml` | `uml-class`, `uml-usecase`, `uml-sequence`, `uml-timing` |
| `-architecture` | `arch-layer`, `arch-plan`, `arch-profile`, `arch-axonometric`, `arch-bubble`, `arch-sunpath`, `arch-schematic` |
| `-process` | `pm-gantt`, `pm-network`, `pm-board`, `matrix-grid`, `schedule-calendar`, `schedule-timedistance`, `seating`, `notation-board`, `notation-railroad`, `notation-petri` |
| `-concept` | `concept-map`, `concept-shape`, `concept-venn` |
| `infographic` | `infographic-number`, `infographic-list`, `infographic-icon`, `infographic-illustration` |
| `interactionstate` | `state-overview`, `state-selection`, `state-sequence` |

Every family documents its keys in a `%region 🔖️Keys-<family>` block with type, default and
meaning. Real computation, not glyphs: `pm-gantt`/`pm-network` run the full CPM (forward pass,
project end, backward pass, total float, critical-path highlighting) in `%region 🔖️Cpm`;
`arch-sunpath` computes solar declination, right ascension, hour angle, altitude and azimuth from
latitude/longitude/date; `arch-axonometric` projects through a 2.5D helper;
`matrix-grid` builds the QFD roof from the column count.

### 3. Demo data (announced for other agents)
`demo-diagram-flow`, `demo-diagram-flow-edges`, `demo-diagram-cause`, `demo-diagram-tree`,
`demo-diagram-cycle`, `demo-diagram-rules`, `demo-uml-class`, `demo-uml-class-members`,
`demo-uml-class-edges`, `demo-uml-usecase`, `demo-uml-usecase-actors`, `demo-uml-usecase-edges`,
`demo-uml-sequence`, `demo-uml-sequence-messages`, `demo-uml-timing`, `demo-uml-timing-segments`,
`demo-arch-layer`, `demo-arch-layer-edges`, `demo-arch-plan`, `demo-arch-profile`, `demo-arch-axo`,
`demo-arch-bubble`, `demo-arch-bubble-edges`, `demo-arch-schematic`, `demo-arch-schematic-edges`,
`demo-pm-gantt`, `demo-pm-board`, `demo-matrix-grid`, `demo-schedule`, `demo-timedistance`,
`demo-seating`, `demo-board`, `demo-railroad`, `demo-petri`, `demo-petri-edges`,
`demo-concept-map`, `demo-concept-levels`, `demo-concept-venn`, `demo-infographic`,
`demo-illustration`, `demo-state-series`, `demo-state-frames`.

### 4. Catalogue
194 entries of my sections retargeted onto real families with per-kind option sets in
`🖼️assets/🔣️viz-catalog.json` (script kept at `🗑️generated/DIAGRAMS/catalog-map.py`). The script
asserts that no two retargeted entries share the same `family` + `options`, so the distinctness
requirement of architecture §4 holds by construction:

```
retargeted 194 entries
merged 1 entries: ['56/syntax-railroad-diagram']
left to other owners (28): ['16/burndown-chart', '16/burnup-chart', '16/cumulative-flow-diagram',
 '16/resource-histogram', '16/velocity-chart', '36/assembly-diagram', … , '59/shipment-sankey']
no duplicate family+options among retargeted entries
```

`56/syntax-railroad-diagram` is merged into `56/railroad-diagram`'s `covers` rather than given a
cosmetic option difference — the two leaves are the same rendering.

### 5. Tests
Five cases under `🧰️framework/🛍️products/📓️print/🧪️tests/`, written against the harness contract
in `📓️status-TESTS-HARNESS.md` §1 (`compileVizProbeDocument` + `probeProjection`):

| case | scenarios | mode |
|---|---|---|
| `diagram-layout-lanes` | grid-placement, direction-transposes-the-grid, lane-bands | conformance |
| `diagram-routing` | orthogonal-vertical, straight-ports | conformance |
| `diagram-gantt-cpm` | forward-and-backward-pass, independent-cpm-reference | conformance vs an independent TS CPM |
| `diagram-sunpath` | solstice-day-arc, southern-winter-arc | conformance vs an independent TS solar model |
| `diagram-sequence` | message-y-positions, self-call-keeps-its-step | conformance |

`diagram-routing/🟦️.ts` exports the shared probe preamble (`DIAGRAM_PREAMBLE`, `DIAGRAM_PACKAGES`,
`FRAME`, `rows`) that the other four import, so the chrome colour aliases and the millimetre canvas
are declared once.

The TS probe runner was not yet runnable when these were written, so every vector was produced by
compiling the probe documents locally with xelatex (`🗑️generated/DIAGRAMS/probe/vec.tex`,
`vec2.tex`) and cross-checked against an independent implementation. Real output:

```
$ xelatex vec.tex && cat vec.probe.jsonl        (56 records, tail)
{"case":"vectors","scenario":"cpm","key":"geometry/diagram-cpm","values":[0,3,0,3,0]}
{"case":"vectors","scenario":"cpm","key":"geometry/diagram-cpm","values":[3,7,3,7,0]}
{"case":"vectors","scenario":"cpm","key":"geometry/diagram-cpm","values":[3,9,6,12,3]}
{"case":"vectors","scenario":"cpm","key":"geometry/diagram-cpm","values":[7,12,7,12,0]}
{"case":"vectors","scenario":"cpm","key":"geometry/diagram-cpm","values":[12,14,12,14,0]}
{"case":"vectors","scenario":"cpm","key":"geometry/diagram-cpm","values":[14,15,14,15,0]}
{"case":"vectors","scenario":"sunpath","key":"geometry/diagram-sun","values":[23.8922,-97.9287]}
{"case":"vectors","scenario":"sunpath","key":"geometry/diagram-sun","values":[50.3166,-57.1091]}
{"case":"vectors","scenario":"sunpath","key":"geometry/diagram-sun","values":[60.2213,17.2392]}
{"case":"vectors","scenario":"sunpath","key":"geometry/diagram-sun","values":[39.9795,76.2667]}
{"case":"vectors","scenario":"sunpath","key":"geometry/diagram-sun","values":[13.0203,111.6385]}
{"case":"vectors","scenario":"sequence","key":"geometry/diagram-message","values":[14,33.3333,40,33.3333]}
{"case":"vectors","scenario":"sequence","key":"geometry/diagram-message","values":[40,24.6667,66,24.6667]}
{"case":"vectors","scenario":"sequence","key":"geometry/diagram-message","values":[66,16,40,16]}
{"case":"vectors","scenario":"sequence","key":"geometry/diagram-message","values":[40,7.3333,14,7.3333]}
```

Independent reference for the same two networks (the algorithm the adapters implement):

```
('a', 0, 3, 0, 3, 0)   ('b', 3, 7, 3, 7, 0)   ('c', 3, 9, 6, 12, 3)
('d', 7, 12, 7, 12, 0) ('e', 12, 14, 12, 14, 0) ('f', 14, 15, 14, 15, 0)
6  ['23.8922', '-97.9287']   9  ['50.3166', '-57.1091']   12 ['60.2213', '17.2392']
15 ['39.9795', '76.2667']    18 ['13.0203', '111.6385']
```

Both agree to the last emitted digit, so the expl3 floating-point trigonometry and the CPM
relaxation reproduce IEEE-double references exactly at the 1e-4 grid the probe emits.

After the last round of kernel fixes the whole vector set was regenerated and byte-compared against
the numbers committed in the feature files — `diff` was empty (56/56 records unchanged).

### 6. Gallery
`🗑️generated/DIAGRAMS/probe/each.sh` compiles one family at a time with a hard timeout and reports
PASS/HANG/ERR; `fast.sh` builds a multi-family PDF that was rasterised with `pdftoppm` and read
page by page. Final run, all 30 families:

```
PASS flow| / swimlane|lanes=row / fishbone| / logic-tree|gate=gate-and / cycle|hub=PDCA
PASS stage|shape=chevron / decision-table| / uml-class| / uml-usecase|system=Shop
PASS uml-sequence| / uml-timing| / arch-layer| / arch-plan|grid=2 / arch-profile|
PASS arch-axonometric|explode=0.6 / arch-bubble| / arch-sunpath| / arch-schematic|
PASS pm-gantt|axis=4 / pm-network| / pm-board| / matrix-grid|roof=true
PASS schedule-calendar| / schedule-timedistance| / seating| / notation-board|
PASS notation-railroad| / notation-petri| / concept-map| / concept-shape|shape=pyramid
PASS concept-venn| / infographic-number| / infographic-list|badge=bullet
PASS infographic-icon|glyph=person / infographic-illustration|cutaway=0.25
PASS state-overview| / state-selection|mode=neighbourhood / state-sequence|depiction=position
```

Bugs the visual pass caught and fixed (none were visible in the error log): the fishbone bones ran
off the canvas, `arch-plan` drew no rooms, the Petri places filled the frame, the icon matrix
overlapped its own rows, and the frame/card grids of `state-sequence` and `infographic-*` staggered
because `\int_set:Nn` **rounds** an integer division — every such division now uses
`\int_div_truncate:nn`. Text is invisible in the local rasters only because Anta/ShareTechMono are
not installed on this machine yet (`bun ./📜️script.ts fonts`); geometry is unaffected.

## Decisions

- **The kernel lives in `semio-viz-diagram-flowchart.sty`.** The six other packages of the namespace
  `\RequirePackage` it and reuse its layout, shape, routing, tone and label routines instead of
  re-deriving geometry. Nothing in the namespace hard-codes a colour or a stroke.
- **Node placement is declared, not inferred.** A diagram's row/column indices come from the data
  table. That is what makes two kinds of one family render differently from the same demo data, and
  it keeps the kernel independent of whichever graph layout lands in `semio-viz-network`.
- **`\int_set:Nn` rounds.** Any grid index derived by integer division uses `\int_div_truncate:nn`.
- **Variable names carry no digits.** Under `\ExplSyntaxOn` a digit ends a control-sequence name, so
  `\l_…_x1_fp` silently becomes `\l_…_x` followed by `1_fp`; the kernel uses `xa/xb/ya/yb/wa/wb/ha/hb`.
- **`\fp_compare:nNnTF` takes one relation character.** `<=` and `>=` need the `\fp_compare:nTF`
  form; both occurrences were fixed.
- **A space cannot be an `l3seq` split delimiter** (items are space-trimmed, and the split loops).
  `\semio_viz_dg_words:NN` converts a space-separated cell into a comma list first; it is used for
  polygon vertices, Gantt predecessors and illustration outlines.
- **Demo tables must write `~` for a space.** Rows declared inside a `.sty` under `\ExplSyntaxOn`
  lose literal spaces at tokenization.

## Requests to other agents

- **GRAMMAR-CORE (`semio-viz-data`)** — `\SemioVizRow` drops empty cells: the row is stored as a
  comma list, and `\clist_item:Nn` ignores empty items, so `{r, , label}` reads `label` as the
  *second* column. Every optional column therefore shifts. The kernel works around it in
  `%region 🔖️Pending-semio-viz-data` (`\semio_viz_dg_col_index:nn` + `\semio_viz_dg_cell:nnnN`,
  which split the raw row with `\seq_set_split:Nnx` and keep empty items). Please move that reader
  into `semio-viz-data.sty` — a table cell must be allowed to be empty.
- **GRAMMAR-CORE (`semio-viz-theme`)** — the categorical hue ramp lives in
  `%region 🔖️Pending-semio-viz-theme` (`\g_semio_viz_dg_hues_clist`, `\semio_viz_dg_hue:nN`,
  `\semio_viz_dg_tone:n`). It is a plain cycle over the seven presence tokens; please replace it
  with the real theme palette accessor and delete the pending region.
- **GRAMMAR-CORE (`semio-viz-format`)** — `%region 🔖️Pending-semio-viz-format`
  (`\semio_viz_dg_num:n`, `\semio_viz_dg_int:n`) rounds millimetres and durations for labels and
  probe records. Replace with `\SemioVizFormat` once it exists; the probe records must keep the
  4-decimal grid the committed test vectors were generated on.
- **SHAPES (`semio-viz-mark`)** — `semio-viz-mark.sty` calls `\tikzset` at load time but never
  `\RequirePackage{tikz}`, so it is not loadable standalone. `semio-viz-diagram-flowchart.sty` now
  requires tikz and `arrows.meta`/`shapes.geometric`/`decorations.pathreplacing` itself; the
  requirement belongs in `semio-viz-mark`.
- **CATALOG (`semio-viz-plot`)** — the `\ProvidesPackage` description of `semio-viz-plot.sty`
  contains `\SemioVizPlot`, `\SemioVizLayer` and `VizPlot`, which LaTeX tries to *expand* while
  writing the package line; every probe log carries four "Undefined control sequence" errors from
  it. Escaping them (`\string\SemioVizPlot`) or rewording the line clears it.
- **TESTS-HARNESS** — please register `suncalc` as an oracle (`@oracle-suncalc`,
  `capabilities: viz-solar-position`, `comparisonProfiles: viz-probe-v1`); `diagram-sunpath` then
  upgrades from `@no-oracle-solar-position` to `@mode-differential` with no probe change, since the
  projection is already altitude/azimuth in degrees. Also please add the `noOracleDecisions`
  entries `diagram-grid-placement`, `diagram-edge-routing`, `critical-path-method`,
  `solar-position` and `diagram-sequence-messages` — each feature carries its justification in the
  narrative block.
- **CHARTS-A** — `16/burndown-chart`, `16/burnup-chart`, `16/cumulative-flow-diagram`,
  `16/velocity-chart` and `16/resource-histogram` are section-16 leaves I own but that belong to
  your `line`/`area`/`bar` families; I left the placeholder entries untouched rather than guess your
  option vocabulary. Please retarget them.
- **CHARTS-B / NETWORK-FLOW** — `59/origin-destination-matrix` (your `heatmap`) and
  `59/shipment-sankey` (your `sankey`) are likewise left untouched.
- **SCIENTIFIC** — `36/assembly-diagram`, `beam`, `bending-moment`, `cfd-mesh`,
  `dimensioned-drawing`, `engineering-drawing`, `exploded-view`, `finite-element-mesh`, `frame`,
  `gear-train`, `kinematic`, `linkage`, `mechanism`, `mohrs-circle`, `shear-force`,
  `stress-strain-curve`, `structural`, `truss` and `56/chemical-diagram`, `56/circuit-diagram`,
  `56/music-notation` are yours; the eight schematic kinds of §36 now use `arch-schematic`, whose
  symbol library (`\semio_viz_ar_symbol:n`) is open for you to extend.
- **NETWORK-FLOW** — `notation-petri` and `concept-venn` are registered and rendering but carry no
  catalogue entry, because `57/petri-net` and the Venn/Euler leaves are covered by entries in your
  sections. Point them at these families if you prefer them over your own.

## Files touched

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-flowchart.sty` (kernel + 7 families)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-uml.sty` (4 families)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-architecture.sty` (7 families)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-process.sty` (10 families + CPM)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-concept.sty` (3 families)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-infographic.sty` (4 families)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-interactionstate.sty` (3 families)
- `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json` (194 entries of my sections)
- `🧰️framework/🛍️products/📓️print/🧪️tests/diagram-layout-lanes/{🥒️.feature,🟦️.ts}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/diagram-routing/{🥒️.feature,🟦️.ts}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/diagram-gantt-cpm/{🥒️.feature,🟦️.ts}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/diagram-sunpath/{🥒️.feature,🟦️.ts}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/diagram-sequence/{🥒️.feature,🟦️.ts}`
- ticket inputs kept: `🗑️generated/DIAGRAMS/catalog-map.py`,
  `🗑️generated/DIAGRAMS/probe/{each.sh,fast.sh,vec.tex,vec2.tex}`

## Open

- The five cases have never run through the repo test platform — the TS probe runner and tectonic
  were not available on this machine. Every number in them was produced by the same LaTeX code the
  adapters compile, so the vectors are right, but the adapter wiring itself is unverified; the first
  `bun ./📜️script.ts test quick` run is the check.
- `arch-sunpath` samples whole UTC hours; sub-hour sampling and a true analemma need a `minute` key.
- `notation-railroad`'s branch line can leave the frame when a branch index exceeds ±1.
