# Print Visualization Library — Architecture Contract

Coordinator document (Fable 5.1, 2026-09-05). Every execution agent reads this before touching code and reports in its own `📓️status-<agent>.md` in this ticket folder. The reference for scope and naming is `🧰️framework/🛍️products/📓️print/🖼️assets/📊️viz-taxonomy.md`: sections 0–73 are the chart catalogue, 74 the encodings, 75 the layout algorithms, 76 the package namespaces, 77 composition, 78 the d3-equivalent kernel, 79 the grammar every chart is expressed in.

## 1. Verdict on the current implementation

Findings are in `📓️explore-latex-viz-api.md`, `📓️explore-gallery-taxonomy.md`, `📓️explore-test-build-infra.md`. In short: 1,857 `\SemioVizChartKind` registrations collapse onto 32 family routines, 96.6 % with the identical option string `data=demo`; ~70 % of chart leaves render a fixed glyph that ignores data; names carry section suffixes (`band-78`, `treemap-7`); no per-component test exists; the aggregator requires a missing file. The library is rebuilt as a grammar-of-graphics engine (taxonomy §79) with the catalogue as declarative presets. No compatibility layer, no legacy support, no migration scripts left behind.

## 2. Product layout (semio style)

```
🧰️framework/🛍️products/📓️print/
  🧬️schema/🔣️.json                      JSON Schema: catalog entry, family option vocabularies, data-table spec, probe protocol
  🧬️schema/🟦️.ts                        typed twin of the schema
  🖼️assets/📊️viz-taxonomy.md            human taxonomy (SSOT for sections/leaves), leaves de-suffixed
  🖼️assets/🔣️viz-catalog.json           machine catalogue: one entry per chart kind (handcrafted, validated by schema)
  🖋️latex/semio-viz.sty                  bundle loader (like the d3 bundle); users may load single packages
  🖋️latex/semio-viz-<module>.sty         kernel and namespace packages (section 4)
  🖋️latex/semio-viz-catalog.sty          GENERATED from 🔣️viz-catalog.json by `📜️script.ts generate` (like semio-tokens.sty)
  🖋️latex/semio-viz-catalog-labels.sty   GENERATED localized kind titles (en, de) for gallery and legends
  🧾️template/📊️viz-gallery/*.tex         GENERATED gallery documents, one per taxonomy section, plus 🔓️viz-api.tex (hand-written)
  🔨️modules/📊️visualization-gallery/    catalog loading, generation, coverage checks (TS, no runtime deps)
  🔨️modules/🧪️viz-probe/                 TS side of the numeric probe harness (compile probe .tex, parse JSON lines)
  🎮️commands/…                           build, watch, generate, test routers (existing pattern)
  🧪️tests/<case>/🥒️.feature + 🟦️.ts     language-neutral cases run by the repo test platform (`🦑️repo/🔨️modules/🧪️test`)
  🔣️oracle.json                          owner oracle registry: d3 packages as test-only references
```

Rules: emoji docstrings, `%region 🔖️Name` in TeX / `//#region 🔖️Name` in TS, no comments inside definitions, every module registered in the sibling `🔣️.json` collection manifest with `id`, `responsibility`, `productionConsumers`.

## 3. The grammar (taxonomy §79) — public LaTeX API

All public commands are `\SemioViz…`; all internals `\semio_viz_<module>_…` in expl3 with `l3keys` families `semio / viz / <module>`. Coordinates are millimetres inside a `VizFigure` (`x=1mm,y=1mm`). Numbers are plain millimetre numbers (no units), validated.

| Grammar element | Command(s) | Package |
|---|---|---|
| data | `\SemioVizTable{name}{cols}`, `\SemioVizRow{name}{row}`, `\SemioVizTableFromCSV{name}{file}[opts]`, `\SemioVizHierarchy{name}[id=,parent=,value=]`, `\SemioVizGraph{name}[nodes=,edges=,source=,target=,weight=]`, `\SemioVizMatrix{name}[row=,col=,value=]`, `\SemioVizGeometry{name}[...]` (rings/points with lon,lat), `\SemioVizFunction{name}{expr}[domain=,samples=]` | `semio-viz-data` |
| transform | `\SemioVizTransform{out}{in}[filter=,sort=,group=,rollup=,pivot=,fold=,join=,window=,bin=,stack=,normalize=,kde=,quantile=,regression=,contour=]` | `semio-viz-transform` |
| scale | `\SemioVizScale{name}{kind}{domain}{range}[opts]`; kinds `linear, log, pow, sqrt, symlog, identity, ordinal, band, point, quantile, quantize, threshold, sequential, diverging, temporal`; opts `nice, clamp, padding, paddingInner, paddingOuter, align, base, exponent, constant, ticks, tickValues, tickFormat, interpolator, scheme, reverse` | `semio-viz-scale` |
| format | `\SemioVizFormat{spec}{value}` (d3-format grammar subset: fill/align/sign/symbol/zero/width/comma/precision/type `d f e g s % r`), `\SemioVizTimeFormat{spec}{value}` (`%Y %m %d %H %M %S %b %B %a %A %j %U %W %p %Z`), locale `en`/`de` from the document language | `semio-viz-format` |
| coordinate system | `\SemioVizCoordinate{kind}[opts]` inside a plot: `cartesian` (x,y), `polar` (angle, radius, startAngle, endAngle, innerRadius), `ternary`, `barycentric`, `parallel`, `logpolar`, `geographic` (projection) | `semio-viz-coordinate` |
| mark | `\SemioVizMark{kind}[opts]{point}`; kinds = every §0 primitive; `\SemioVizPath{kind}[points=,curve=,closed=]`; curves `linear, step, step-before, step-after, basis, basis-closed, bundle, cardinal, cardinal-closed, catmull-rom, catmull-rom-closed, monotone-x, monotone-y, natural, bezier` (true interpolators, no cosmetic aliases) | `semio-viz-mark` |
| shape generators | `\SemioVizLine`, `\SemioVizArea`, `\SemioVizArc[innerRadius=,outerRadius=,startAngle=,endAngle=,padAngle=,cornerRadius=]`, `\SemioVizPie[value=,sort=,startAngle=,endAngle=,padAngle=]`, `\SemioVizLink[kind=horizontal\|vertical\|radial]`, `\SemioVizSymbol[type=,size=]`, `\SemioVizRibbon` | `semio-viz-shape` |
| layout algorithms (§75, §78) | `\SemioVizLayout{algorithm}{in}{out}[opts]`: `stack` (order `none\|ascending\|descending\|inside-out\|reverse\|appearance`, offset `none\|expand\|diverging\|silhouette\|wiggle`), `bin`, `hexbin`, `beeswarm`, `jitter`, `pie`, `arc`, `chord`, `sankey`, `treemap` (tile `squarify\|slice\|dice\|slice-dice\|binary\|resquarify`), `partition`, `pack`, `force` (deterministic: fixed iterations, seeded jitter, forces `link, many-body, center, collide, x, y, radial`), `tree` (Reingold–Tilford/Buchheim), `cluster`, `dag` (Sugiyama layering, ordering, coordinate assignment), `bundling`, `voronoi`, `delaunay`, `hull`, `contour` (marching squares), `density` (KDE 1D/2D), `projection` | `semio-viz-layout` (kernel) with per-domain files `semio-viz-hierarchy`, `semio-viz-network`, `semio-viz-flow`, `semio-viz-geo`, `semio-viz-spatial` |
| encoding | keys of `\SemioVizPlot`: `x, y, x2, y2, angle, radius, size, shape, fill, stroke, opacity, text, dash, width, order, detail, tooltip`(ignored in print) each `= <column>` or `= { column, scale=<name> }` | `semio-viz-plot` |
| plot (the ultimate API) | `\SemioVizPlot[data=, mark=, coordinate=, <encodings>, transform=, layout=, facet=, guide=, annotation=, theme=]` and the environment form `\begin{VizPlot}[...] … layers … \end{VizPlot}` with `\SemioVizLayer[...]` | `semio-viz-plot` |
| facet | `\SemioVizFacet[row=,column=,wrap=,columns=,sharedX=,sharedY=,gap=]` | `semio-viz-facet` |
| guide | `\SemioVizAxis[scale=,orient=,title=,ticks=,tickValues=,tickFormat=,tickSize=,minor=,grid=,offset=]`, `\SemioVizLegend[scale=,kind=swatch\|line\|symbol\|gradient\|size,title=,orient=,columns=,at=]`, `\SemioVizGrid[x=,y=,minor=]` | `semio-viz-guide` |
| annotation | `\SemioVizAnnotate{kind}[...]`: label, callout, leader, bracket, brace, highlight, reference-line, reference-band, reference-point, event-marker, threshold | `semio-viz-annotation` |
| theme | `\SemioVizTheme{name}` / keys `semio / viz / theme`: categorical palette from design tokens (`presence.hues`, light/dark aware), sequential and diverging schemes, stroke widths (`strokes.grid*`), fonts, grayscale-safe and pattern encodings | `semio-viz-theme` |
| composition (§77) | `VizFigure`, `VizSection`, `VizColumn` (exist), plus `\SemioVizLayer`, `\SemioVizOverlay`, `\SemioVizConcat[direction=]`, `\SemioVizInset[at=,width=,height=,zoom=]`, shared/independent axes and legends, small multiples via facet | `semio-viz-composition` |
| chart kinds | `\SemioVizChart{kind}[overrides]` = preset from the catalogue: `family` + default options, rendered through the grammar | `semio-viz-catalog` (generated) |
| families | `\SemioVizFamily{name}{code}` registers a renderer that composes grammar calls from `semio / viz / family / <name>` keys; every chart kind belongs to exactly one family | `semio-viz-family` (registry) + namespace packages |

### Namespaces (taxonomy §76) → packages and owners

| Namespace | Packages | Owner agent |
|---|---|---|
| kernel | `semio-viz-data`, `semio-viz-transform`, `semio-viz-scale`, `semio-viz-format`, `semio-viz-theme` | GRAMMAR-CORE |
| kernel | `semio-viz-mark`, `semio-viz-shape`, `semio-viz-coordinate` | SHAPES |
| kernel | `semio-viz-guide`, `semio-viz-annotation`, `semio-viz-facet`, `semio-viz-composition`, `semio-viz-label` | GUIDES |
| kernel | `semio-viz-plot` (encoding + layer engine), `semio-viz-family` (registry), `semio-viz-catalog` (generated), `semio-viz-catalog-labels` (generated) | CATALOG |
| charts | `semio-viz-charts-bar`, `-line`, `-area`, `-scatter`, `-financial`, `-timeline`, `-dashboard` (KPI, gauges, bullets), `-funnel` | CHARTS-A |
| charts | `semio-viz-charts-distribution`, `-statistical` (inference, uncertainty, model evaluation, control charts), `-polar`, `semio-viz-matrix-heatmap`, `-correlation`, `-adjacency`, `semio-viz-table` | CHARTS-B |
| hierarchy | `semio-viz-hierarchy-tree`, `-dendrogram`, `-treemap`, `-partition`, `-pack` | HIERARCHY |
| network + flow | `semio-viz-network-graph`, `-matrix`, `-arc`, `-chord`, `-bundling`, `semio-viz-flow-sankey`, `-alluvial`, `-parallelsets` | NETWORK-FLOW |
| geo + spatial | `semio-viz-geo-map`, `-projection`, `-choropleth`, `-symbols`, `-contours`, `-routes`, `semio-viz-spatial` (voronoi, delaunay, hull, hexbin, contour, density) | GEO-SPATIAL |
| diagram | `semio-viz-diagram-flowchart`, `-uml`, `-architecture`, `-process`, `-concept`, `semio-viz-infographic`, `semio-viz-interactionstate` | DIAGRAMS |
| scientific | `semio-viz-scientific-field`, `-surface`, `-signal`, `-physics`, `-chemistry`, `-biology`, `-engineering`, `-mathematics`, `-geometry`, `-3d` | SCIENTIFIC |
| tests | `semio-viz-probe`, `🔣️oracle.json`, `🧪️tests/`, `🔨️modules/🧪️viz-probe`, nx/launch wiring | TESTS-HARNESS |
| toolchain | generated TS layer, tectonic on Windows | BOOTSTRAP |

Section → owner map for the catalogue: CHARTS-A: 1, 2, 17, 18, 19, 50, 51, 52, 61(cartesian ones), 62, 65, 69, 70(bar/share), 72, 73. CHARTS-B: 3, 4, 5, 6, 11, 12, 20, 23, 24, 42, 48, 49, 60, 68. HIERARCHY: 7, 47(packing). NETWORK-FLOW: 8, 9, 13(networks/trees), 57(automata, data structures), 58, 63, 64, 66, 67. GEO-SPATIAL: 10, 26(voronoi/delaunay/hull), 27, 28, 35(maps). DIAGRAMS: 14, 15, 16, 36(schematics), 37, 45, 46, 53, 54, 55, 56, 59, 71. SCIENTIFIC: 21, 22, 25, 26(geometry), 29, 30, 31, 32, 33, 34, 35(diagrams), 38, 39, 40, 41, 43, 44. Section 0 marks: SHAPES. Sections 74–79: kernel owners (they are capabilities, not charts; the catalogue marks them `kind: scale|axis|layout` and the probe tests cover them).

## 4. File ownership and integration rules

- The loader `semio-viz.sty` and the family registry `semio-viz-family.sty` are created by the coordinator with every planned `\RequirePackage` line already present and every package existing as a stub. **Nobody edits the loader**; add your code inside the packages you own. Load order is fixed in the loader (kernel → plot → namespaces → catalog).
- Legacy `semio-viz-layout.sty` and `semio-viz-axis.sty` stay loadable until every family they still serve is reimplemented; CATALOG deletes them at the end together with the 78 `semio-viz-chart-*.sty` files (replaced by the generated `semio-viz-catalog.sty`). Do not extend legacy files.
- Register families only through `\SemioVizFamily{name}{code}` in your own package. Family option keys live in `semio / viz / family / <name>`; document each key with type and default in a `%region 🔖️Keys` block; CATALOG copies the vocabulary into the schema (`🧬️schema/🔣️.json`) — agree on names by reading the schema if it already exists, otherwise state them in your status file.
- Chart kinds: you own the catalogue entries of your sections in `🖼️assets/🔣️viz-catalog.json` (add or edit entries for your sections only; CATALOG owns the file structure and the generator). Entry shape: `{ "id": "7/treemap", "slug": "treemap", "title": { "en": "Treemap", "de": "Kacheldiagramm" }, "kind": "chart", "namespace": "hierarchy/treemap", "family": "treemap", "options": { "tile": "squarify" }, "data": "demo-hierarchy", "covers": ["6/treemap", "7/treemap", "61/treemap"] }`. Slugs are global chart-kind names without section suffixes; a kind may cover several taxonomy leaves; distinct semantics get distinct handcrafted slugs (`sports-heatmap`, not `heatmap-43`).
- Every family must actually consume its data and options: two kinds of the same family with different options must render differently, and the probe projection must differ. This is verified by the exhaustive test.
- Demo data: CATALOG provides named demo tables in `semio-viz-data.sty` (`demo`, `demo-series`, `demo-hierarchy`, `demo-graph`, `demo-matrix`, `demo-geo`, `demo-time`, `demo-distribution`, …). Add demo tables you need in your own package under `%region 🔖️DemoData` with the `demo-` prefix and announce them.
- Shared helper code belongs in the kernel packages; if you need a kernel capability that does not exist yet, write it in your own package under `%region 🔖️Pending-<kernel-package>` and note it in your status file. The kernel owner moves it.
- No comments inside definitions; a `%` note above a definition is fine and must start with an emoji.

## 5. Tests (language-agnostic + third-party oracle)

Owner of every case: `🧰️framework/🛍️products/📓️print` (cases under `🧪️tests/<kebab-case>/`). Runner: the repo test platform (`bun ./📜️script.ts run <level> --owner "🧰️framework/🛍️products/📓️print"` in `🦑️repo/🔨️modules/🧪️test`), also exposed through print's `📜️script.ts test` and nx targets `test-quick/long/exhaustive`.

- **Probe protocol** (`semio-viz-probe.sty`, TESTS-HARNESS): a probe document loads `semio-viz`, runs kernel macros on fixture vectors and appends JSON lines to `\jobname.probe.jsonl` via `\iow`: `{"case":"<case>","scenario":"<id>","key":"<vector-id>","values":[…numbers or strings…]}`. Public macros: `\SemioVizProbeBegin{case}{scenario}`, `\SemioVizProbeValues{key}{clist}`, `\SemioVizProbePoints{key}{coordinate list}`, `\SemioVizProbeEnd`. Marks and families also emit their geometry through `\semio_viz_probe_geometry:nn` when probing is on (`\SemioVizProbeOn`), so a family's rendered primitives (rect/circle/path/text with coordinates) are testable numerically without rasterising.
- **Adapter** (`🟦️.ts` per case): `subject` compiles the case's probe `.tex` (fixture) with tectonic through `🔨️modules/🧪️viz-probe/🟦️.ts` and returns the parsed projection; `oracle` computes the same projection with the d3 package named by `@oracle-<id>` (test-only dependency, declared in print's `🔣️oracle.json` and `package.json` devDependencies). Comparison profile `floating-point-v1` with the tolerance declared in the feature.
- **Levels**: `fundamental` = schema/catalog/taxonomy structure and coverage (no TeX); `quick` = kernel probes (scales, formats, shapes, transforms, layouts) on small vectors; `long` = every family probe + gallery build; `exhaustive` = every chart kind rendered in both themes and both languages, per-kind PDF text and stable hash fixtures, plus distinctness (no two kinds of one family with different options share a projection).
- **Oracles**: `d3-scale`, `d3-scale-chromatic`, `d3-array`, `d3-format`, `d3-time-format`, `d3-time`, `d3-shape`, `d3-hierarchy`, `d3-force`, `d3-chord`, `d3-sankey`, `d3-geo`, `d3-delaunay`, `d3-hexbin`, `d3-contour`, `d3-interpolate`, `d3-color`, `d3-path`; plus `markdown-it` (taxonomy) and `ajv` (schema). Where d3 has no equivalent (Sugiyama, Kamada–Kawai, marching squares variants, projection variants), use a second reference (`dagre`, `elkjs`, `proj4`) or a `@no-oracle` decision with specification vectors, justified in the feature file.
- Fixtures are immutable JSON/TeX under `🧪️tests/<case>/🧫️fixtures/`; vectors preferably live in the Gherkin data table.
- Gherkin tags: `@capability-…`, `@oracle-<id>` / `@no-oracle-<id>`, `@comparison-floating-point-v1`, per scenario `@id-…`, one `@level-…`, one `@mode-…` (`differential`, `conformance`, `property`, `error`).

## 6. Toolchain

- `bun` + `nx`; print scripts route through `📦️packages/🟦️typescript/📜️script.ts` (`generate | fonts | build | watch | test | preview-generated`). Add `generate viz` (catalog → `.sty` + gallery), `test viz …` already exists and becomes the platform run.
- Tectonic 0.16.9 auto-downloaded to `.🧬semio/🦑️repo/⚡️cache/tectonic/`; Windows extraction must fall back to `tar` when `unzip` is absent (TESTS-HARNESS fixes). MiKTeX `xelatex` exists on this machine for local probing (`TEXINPUTS` to `🖋️latex` and the font cache), but the tests must use the repo's tectonic path.
- Launch entries: add to `.vscode/🧩️launch.seed.jsonc` (never `launch.json` directly) following `🧪️test🖨️print📊️viz…` naming, then regenerate.
- Multi-language: every user-visible string (kind titles, axis/legend defaults, gallery chapter titles) exists in `en` and `de`; the document language selects it; no default language in code.

## 7. Milestones

1. Foundation (coordinator): loader, registry, stubs, schema skeleton, this document. Toolchain bootstrap (BOOTSTRAP).
2. Kernel + harness (GRAMMAR-CORE, SHAPES, GUIDES, CATALOG, TESTS-HARNESS) while namespace owners build on the contract.
3. Namespaces (CHARTS-A, CHARTS-B, HIERARCHY, NETWORK-FLOW, GEO-SPATIAL, DIAGRAMS, SCIENTIFIC): families, catalogue entries, demo data, probes, gallery.
4. Integration: generated catalog + gallery, exhaustive run in both themes and languages, legacy deletion, docs (`🔓️viz-api.tex` becomes the API reference), TS twin kernel (`🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript`) as a second subject rendering into `🧰️framework/🔨️modules/◻️2d`.
