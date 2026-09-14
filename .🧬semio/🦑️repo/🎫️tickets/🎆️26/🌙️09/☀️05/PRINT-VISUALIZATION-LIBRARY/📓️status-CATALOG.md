# Status — CATALOG

Owner of: `🧬️schema/🔣️.json` + `🧬️schema/🟦️.ts`, `🖼️assets/🔣️viz-catalog.json`, `🖼️assets/📊️viz-taxonomy.md`,
`🖼️assets/🔣️viz-taxonomy.json`, `🖋️latex/semio-viz.sty`, `semio-viz-plot.sty`, `semio-viz-family.sty`,
generated `semio-viz-catalog.sty` / `semio-viz-catalog-labels.sty`, generated `🧾️template/📊️viz-gallery/*.tex`,
`🔨️modules/📊️visualization-gallery/🟦️.ts`, the `generate viz` wiring, legacy dissolution.

## Done

### Taxonomy (`🖼️assets/📊️viz-taxonomy.md`)

- **De-suffixed.** All 205 numeric section suffixes stripped from leaf slugs (`treemap-7` → `treemap`,
  `band-78` → `band`, …). Ids stay `section/slug`. Five within-section collisions got handcrafted names:
  `74/width-74` → `line-width`, `74/rotation-74-2` → `text-rotation`, `76/matrix` (network group) →
  `network-matrix`, `78/quantile-78` → `quantile-transform`, `79/text` (mark group) → `text-mark`.
- **Homonym splits.** Five leaves whose de-suffixed name collided *semantically* with a different
  concept got handcrafted slugs and titles: `43/heatmap` → `sports-heatmap` (pitch heatmap, not a
  matrix heatmap), `74/rotation` → `rotation-encoding` (encoding channel, not the `26/rotation`
  geometry transform), `74/area` → `area-encoding` (size channel, not the area mark),
  `78/band` → `band-scale`, `78/point` → `point-scale` (scales, not the marks of the same name).
- 1,966 leaves, 80 sections, unchanged in count and meaning.

### Catalogue (`🖼️assets/🔣️viz-catalog.json`, 748 KB, 1,770 entries)

- Every leaf mapped onto exactly one chart kind; 161 kinds cover more than one leaf
  (`treemap` covers `6/treemap 7/treemap 76/treemap 78/treemap`; `chord-diagram` five sections).
  Where a merge would have been wrong the leaves were split instead (see above).
- Entry shape exactly as architecture §4: `id, slug, title{en,de}, kind, namespace, family, options,
  data, covers`. Sorted by numeric section, then slug.
- Titles: English from the taxonomy, German handcrafted through a compositional renderer with a
  lexicon of ~900 heads/modifiers/adjectives plus explicit overrides. German conventions: strong
  adjective endings agreeing with the head noun's gender (`Gestapeltes Balkendiagramm`,
  `Kraftbasierter Graph`), noun compounds concatenated (`Balken` + `Diagramm` → `Balkendiagramm`,
  `Windrose`, `Kantenbündelung`), proper names and established anglicisms hyphenated and kept
  (`Sankey-Diagramm`, `ROC-Kurve`, `Nyquist-Diagramm`, `Boxplot`, `Treemap`, `Heatmap`).
  **Zero titles have an untranslated head word.** The 80 chapter and 99 group titles are handcrafted
  one by one and live in the schema (`x-semio-taxonomy-sections`, `x-semio-taxonomy-groups`).
- `🖼️assets/🔣️viz-taxonomy.json` regenerated from the same source, now carrying the catalogue family.

### Schema (`🧬️schema/🔣️.json`) and typed twin (`🧬️schema/🟦️.ts`)

- JSON Schema draft 2020-12 for the catalogue document: `CatalogEntry`, `LocalizedText`,
  `TaxonomyLeafId`, `Slug`, `Kind`, `Namespace`, `DemoTable`, `FamilyOptions`,
  `DemoTableDeclaration`, `TaxonomySection`, `TaxonomyGroup`, `ProbeRecord`.
- The per-family vocabulary that pure JSON Schema cannot express rides as annotations:
  `x-semio-family-options` (146 families, each `{owner, options}`), `x-semio-demo-tables` (11 tables
  with their columns), `x-semio-taxonomy-sections`, `x-semio-taxonomy-groups`. The TS module enforces
  them; `ajv` enforces the entry shape.
- `🧬️schema/🟦️.ts` restates the contract for TypeScript and carries the shared pure readers
  (`parseVizTaxonomy`, `vizTitle`, `vizOptionList`).

### LaTeX

- `semio-viz-plot.sty` — the grammar entry point *and* the chart-kind registry:
  `\SemioVizChartKind`, `\SemioVizKindCovers`, `\SemioVizChart{slug}[overrides]` resolving through
  `\semio_viz_family_run:VV`; `\SemioVizPlot[...]`, the `VizPlot` environment and `\SemioVizLayer` with
  the full §79 key set (`data, mark, coordinate, transform, layout, facet, guide, annotation, theme,
  curve`) plus the 16 encoding channels of §74 as unknown-key bindings that accept either a bare
  column or `{column, scale=…}`. Rendering delegates to the kernel entry points
  (`semio_viz_transform_apply:nn`, `semio_viz_layout_apply:nn`, `semio_viz_coordinate_begin:nn`,
  `semio_viz_mark_series:nnn`, `semio_viz_facet_map:nn`, `semio_viz_guide_draw:n`,
  `semio_viz_annotation_draw:n`, `semio_viz_theme_select:n`) through `\semio_viz_plot_kernel_use:nnn`,
  which raises a named `missing-kernel` error rather than silently drawing nothing.
- `semio-viz.sty` — the chart-kind registry moved out to `semio-viz-plot.sty`; `\SemioVizDemo` now
  dispatches marks to `semio-viz-mark` and everything else through the catalogue. Load order
  untouched apart from dropping the deleted legacy aggregator.
- `semio-viz-family.sty` — unchanged (was already implemented by the coordinator).

### Generation

- `bun ./📜️script.ts generate viz` (nx `generate-viz`, launch seed `📦️generate🖨️print📊️viz`,
  command `🎮️commands/📊️viz-catalog-generation`, registered in `🎮️commands/🔣️.json`) emits **82 files**:
  - `semio-viz-catalog.sty` — 1,770 `\SemioVizChartKind{slug}{family}{data=…,variant=…,…}` plus 1,770
    `\SemioVizKindCovers{slug}{leaf ids}`.
  - `semio-viz-catalog-labels.sty` — its own expl3 registry plus 1,770 `\SemioVizKindLabel{slug}{en}{de}`;
    `\SemioVizKindTitle{slug}` and `\SemioVizLocalized{en}{de}` resolve through `\l_semio_language_tl`,
    with no default language.
  - 80 gallery documents, one per section, with `% viz-covers:` per leaf, `VizFigure` titled by
    `\SemioVizKindTitle`, and `\SemioVizChart{slug}` on the entry's demo data. Chapter and section
    headings go through `\SemioVizLocalized`.
- All generated files carry the `% 🤖 Generated …` header. `bun ./📜️script.ts generate` (no segment)
  writes the token stylesheet *and* the viz artifacts; `preview-generated` emits all **83 nodes**.
- The repo generator contract `print-latex-tokens` (`🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`) now
  declares the two generated `.sty` files and the gallery directory as output roots and the catalogue,
  taxonomy, schema, module and command as inputs.
- To keep that output root purely generated, the hand-written API reference moved to
  `🧾️template/📊️viz-api/🔓️viz-api.tex` (still enumerated by `visualizationTemplates()`, still the
  `viz-api` template id) and now also demonstrates `\SemioVizPlot`, `VizPlot`, `\SemioVizLayer` and
  `\SemioVizKindTitle`.

### Checks and tests

- `🔨️modules/📊️visualization-gallery/🟦️.ts` rewritten: catalogue/schema loading, the three renderers,
  `generateVizArtifacts`, `registeredVizFamilies` (parses `\SemioVizFamily{…}` across `🖋️latex`) and
  `vizCoverageReport` — leaves covered exactly once, no over-coverage, every option key declared for
  its family, every `data` a declared demo table, no two kinds sharing family+options, both languages
  titled.
- Gherkin case `🧪️tests/catalog-coverage/` (`🥒️.feature` + `🟦️.ts`): 7 `@level-fundamental` scenarios
  (`schema, leaves, slugs, options, distinctness, languages, generated`) and 1 `@level-long`
  (`families`). Oracles: `ajv` 8.20.0 in draft 2020-12 mode validates the catalogue against the schema;
  `markdown-it` 14.3.0 reads the leaf set out of a CommonMark token stream. Both declared in the print
  package's `devDependencies` and in the new `🔣️oracle.json`.
- Fixtures updated: `🧫️merge-contract.json` `vizApiCommands` 17 → 21; the quick test's chart-package
  existence check now walks `semio-viz.sty`'s `\RequirePackage` list instead of the deleted aggregator.

### Legacy dissolution

- Deleted `semio-viz-charts.sty` and all **78** `semio-viz-chart-*.sty` files, and removed the
  aggregator from the bundle. Their 1,857 registrations are replaced by the generated catalogue.
- `semio-viz-layout.sty` and `semio-viz-axis.sty` were **not** deleted: architecture §3 gives those two
  file names to the kernel (`semio-viz-layout` = the §75/§78 layout algorithms, GRAMMAR-CORE;
  `semio-viz-axis` superseded by `semio-viz-guide`, GUIDES). Emptying them now would destroy work in
  flight. Their old `\SemioVizLayout{family}` registry is a separate register from
  `semio-viz-family`, so it cannot collide with the catalogue. **Request: GRAMMAR-CORE replaces
  `semio-viz-layout.sty` with the kernel layout package; GUIDES folds `semio-viz-axis.sty` into
  `semio-viz-guide.sty` and the loader line goes with it.**

## Verification actually run

```
bun ./📜️script.ts generate            → print: wrote latex/semio-tokens.sty
                                         print: wrote 82 visualization catalogue artifacts
bun ./📜️script.ts preview-generated   → contractId print-latex-tokens, 83 nodes
bun ./📜️script.ts test quick          → [DEBUG] print: viz coverage 1966/1966 leaves through 1770 kinds, API 21/21
                                         [DEBUG] print: 146 families still awaiting a \SemioVizFamily registration: …
                                         [DEBUG] print: unit tests passed
                                         [DEBUG] print: catalog-coverage — 7 fundamental scenarios passed
```

Local xelatex probe of the generated packages (`TEXINPUTS` → `🖋️latex`, `\usepackage{semio-viz-catalog}`
`\usepackage{semio-viz-catalog-labels}`):

```
[DEBUG] kind count=1770
[DEBUG] treemap family=space-filling
[DEBUG] treemap options=data=demo-hierarchy,variant=treemap,tile=squarify,padding=0.4,depth=2
[DEBUG] treemap covers=6/treemap,7/treemap,76/treemap,78/treemap
[DEBUG] title de=Gestapeltes Balkendiagramm
```

Dispatch probe (a stub family registered with `\SemioVizFamily`, then `\SemioVizChart`):

```
[DEBUG] probe-family ran with: data=demo-hierarchy,variant=treemap,tile=squarify,padding=0.4,depth=2
[DEBUG] probe-family ran with: data=demo,variant=stacked-bar-chart,orient=vertical,…,orient=horizontal
```

Remaining xelatex errors in the probe are the missing Anta/ShareTechMono fonts and `semio-core` loaded
outside its class — probe scaffolding, not the generated packages.

## Decisions the other owners must adopt

- **Family names come from the taxonomy `(section, group)` pair.** 146 families, each owned by one
  agent; the full list with owners is in `🧬️schema/🔣️.json` → `x-semio-family-options`. Read it before
  naming a family: registering a name that is not in the catalogue leaves chart kinds unrenderable, and
  registering a catalogue name in two packages fails the `@id-families` scenario.
- **Every family takes a required `variant` key** whose value is the catalogue slug of the kind being
  drawn. This is what makes `family + options` unique per kind (architecture §4) without inventing
  fake dimensions. **The family must draw a different geometry for each variant it serves** — that is
  what the exhaustive distinctness test measures.
- **Real dimensions are declared on top of `variant`** for the families where the taxonomy expresses
  them, and are overridable through `\SemioVizChart{kind}[key=value]`:
  `bar{orient,grouping,normalize,baseline}`, `line{curve,points}`, `area{curve,offset}`,
  `pie{startAngle,endAngle,padAngle,explode}`, `donut{innerRadius,rings,padAngle}`,
  `scatter{symbol,size,jitter}`, `histogram{bins,normalize}`, `density{bandwidth,fill}`,
  `box{whisker,notch,orient}`, `violin{bandwidth,side}`, `heatmap{scheme,cellGap,labels}`,
  `space-filling{tile,padding,depth}`, `tree{orient,link}`, `graph{layout,directed}`,
  `flow{align,nodePadding}`, `map{projection,graticule}`, `choropleth{projection,classes,classify}`,
  `polar{startAngle,innerRadius}`, `funnel{orient,neck}`, `kpi{format,trend}`,
  `scale{domain,range,nice}`, `axis{orient,ticks,grid}`. Add your own by extending
  `x-semio-family-options` for **your** families only, then rerun `generate viz`.
- **Demo tables** (declared in `x-semio-demo-tables`, to be provided in `semio-viz-data.sty` /
  your own `%region 🔖️DemoData`): `demo{cat,val,val2,grp}`, `demo-series{t,s1,s2,s3}`,
  `demo-time{date,open,high,low,close,volume}`, `demo-distribution{group,value}`,
  `demo-parts{part,value}`, `demo-hierarchy{id,parent,value}`, `demo-graph{source,target,weight}`,
  `demo-flow{source,target,value,stage}`, `demo-matrix{row,col,value}`,
  `demo-geo{region,lon,lat,value}`, `demo-field{x,y,u,v}`.

## Requests to other agents

- **GUIDES — `\SemioVizLayer`.** `semio-viz-composition.sty` defines a command form
  `\SemioVizLayer[keys]{content}` of its `VizLayer` environment. Architecture §3 gives that name to the
  plot grammar (`\SemioVizLayer[keys]` inside `VizPlot`), so `semio-viz-plot.sty` now claims it with
  `\DeclareDocumentCommand` and an emoji note. Please withdraw the command form from composition — the
  `VizLayer` environment and `\SemioVizOverlay` already cover figure-level layering.
- **GRAMMAR-CORE — `semio-viz-data.sty` double definitions.** A local xelatex load reports
  `\l_semio_viz_data_{id,parent,value,source,target,weight,nodes,edges,row,col,ring,lon,lat,label}_tl`,
  `…_domain_clist`, `…_samples_int`, `…_delimiter_tl`, `…_quote_tl`, `…_header_bool`,
  `…_columns_clist` as *already defined* — the same variables are `\…_new:N`-ed twice in that file.
- **GRAMMAR-CORE / SHAPES / GUIDES — kernel entry points `semio-viz-plot` calls.** Names implemented
  against architecture §3: `\semio_viz_transform_apply:nn {table}{keys}`,
  `\semio_viz_layout_apply:nn {table}{keys}`, `\semio_viz_coordinate_begin:nn {kind}{keys}` /
  `\semio_viz_coordinate_end:`, `\semio_viz_mark_series:nnn {mark}{table}{keys}`,
  `\semio_viz_facet_map:nn {keys}{body}`, `\semio_viz_guide_draw:n {keys}`,
  `\semio_viz_annotation_draw:n {keys}`, `\semio_viz_theme_select:n {name}`. Tell me if you name them
  differently and I will follow.

## Open

- 146 families are still awaiting their `\SemioVizFamily` registration — that is the namespace owners'
  milestone 3. `test quick` reports the count, `test viz full` / `test long` fails on it
  (`runCatalogFamilyRegistration`), which is the intended gate.
- `bun ./📜️script.ts generate viz` intermittently fails with `EUNKNOWN errno -134` on the first gallery
  file when run twice in quick succession — a Windows file-lock during the on-access scan of a
  just-written emoji-named file, not a code defect. Re-running succeeds.

## Files touched

Product (`🧰️framework/🛍️products/📓️print/`):

- `🧬️schema/🔣️.json` (rewritten), `🧬️schema/🟦️.ts` (new)
- `🖼️assets/📊️viz-taxonomy.md` (de-suffix, homonym splits, 10 handcrafted titles)
- `🖼️assets/🔣️viz-catalog.json` (new), `🖼️assets/🔣️viz-taxonomy.json` (regenerated)
- `🖋️latex/semio-viz.sty`, `🖋️latex/semio-viz-plot.sty`
- `🖋️latex/semio-viz-catalog.sty`, `🖋️latex/semio-viz-catalog-labels.sty` (generated)
- `🧾️template/📊️viz-gallery/*.tex` (80 generated), `🧾️template/📊️viz-api/🔓️viz-api.tex` (moved, extended)
- `🔨️modules/📊️visualization-gallery/🟦️.ts`, `🔨️modules/🔣️.json`
- `🎮️commands/📊️viz-catalog-generation/🟦️.ts` (new), `🎮️commands/🔣️.json`
- `🎮️commands/🧪️print-pipeline-verification/🟦️.ts`
- `🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts`, `…/🧫️merge-contract.json`
- `🧪️tests/catalog-coverage/🥒️.feature`, `🧪️tests/catalog-coverage/🟦️.ts` (new)
- `🔣️oracle.json` (new)
- `📦️packages/🟦️typescript/📜️script.ts`, `📋️project.json`, `package.json`
- **deleted:** `🖋️latex/semio-viz-charts.sty` and 78 `🖋️latex/semio-viz-chart-*.sty`

Repository:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (generator contract inputs/outputs)
- `.vscode/🧩️launch.seed.jsonc` (`📦️generate🖨️print📊️viz`)

Ticket-local build helpers (kept, not product code):
`🔧️desuffix-taxonomy.ts`, `🔬analyze-taxonomy.ts`, `🏗️catalog-families.ts`, `🏗️catalog-german.ts`,
`🏗️catalog-sections-de.ts`, `🏗️catalog-groups-de.ts`, `🏗️build-catalog.ts`, `🏗️build-schema.ts`.
