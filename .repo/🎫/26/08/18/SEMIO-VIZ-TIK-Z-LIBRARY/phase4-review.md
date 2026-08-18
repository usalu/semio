# Phase 4 independent review — Opus 5 High

Scope: taxonomy → manifest → gallery → chart packages → public API for ticket `26/08/18/SEMIO-VIZ-TIK-Z-LIBRARY`.
Method: `review.ts` / `review2.ts` in this ticket folder re-parse `exhaustive-taxonomy.md` independently of
`generate.ts` and diff against every generated artefact. Raw output: `review-output.txt`, `review-output-2.txt`.

**Verdict: FAIL** — no P0, three P1 defects. Everything mechanical (coverage gate, slug uniqueness, cover/demo
pairing, compilation) is green; the taxonomy→ChartKind extraction ships 36 names that are not visualization types,
and 15 dead chart packages are committed in `print/tex`.

## What is verified green

| Check | Result |
| --- | --- |
| Sections parsed | 80 (0–79) |
| `print/asset/viz-taxonomy.md` vs `.json` | identical leaf order, 2001 each |
| `wp-registry.json` vs `viz-taxonomy.json` | byte-identical |
| Duplicate global slugs | 0 |
| Collision-suffix rule (`first natural`, later `-{sectionId}[-n]`) | 205 suffixed slugs, 0 violations, 0 first-occurrences that lost the natural slug |
| Coverage keys `{section}/{slug}` → `\SemioVizDemo{slug}` | 2001/2001 match, 0 mismatches, 0 orphan covers |
| Gallery files | 81 (`viz-0`…`viz-79` + `viz-api`) |
| Loader `semio-viz-charts.sty` | requires exactly 79 packages, all present |
| Registered ChartKinds in required packages | 1946 = 2001 − 56 (section 0 marks) + 1 alias; 0 double registrations |
| Section 0 slugs vs `\g_semio_viz_mark_kinds_seq` | all 56 registered |
| Layout families used by kinds | all 32 exist in `semio-viz-layout.sty` |
| Kind rules (0 mark; 74 mark/scale; 75–77/79 layout; 78 scale/axis/layout by subgroup; 51 axis; else chart) | 0 violations |
| `test viz coverage` | `viz coverage 2001/2001 leaves, API 10/10` |
| Build | `print/dist` holds `viz-0`…`viz-79` + `viz-api` in light and dark (162 PDFs); ticket compile logs show PASS for the 60 sections they cover |

## Leaf count vs original

| Quantity | Count |
| --- | --- |
| Original bullets at depth ≥ 1 | 2060 |
| …terminal (no children) | 1956 |
| …parents (have children) | 104 |
| Generated leaves | 2001 |
| = terminals kept | 1956 (100%, nothing dropped) |
| + parents promoted to leaf-and-group | 45 |
| parents dropped as categories | 59 |

No leaf was lost. The error is one-directional: **29 of the 45 promoted parents are category headings, not named
types**, and a further **7 terminal bullets in section 79 are prose fragments**, so 36 of 2001 kinds (1.8%) are
bogus. A clean extraction lands at ~1965 leaves (~1966 if the missing section-76 `charts` namespace is added).

Correctly promoted (16): `14/flowchart`, `31/bode-plot`, `42/control-chart`, `48/combination-chart`; the seven
section-76 namespaces `hierarchy` `network` `flow` `geo` `matrix` `diagram` `scientific`; the five section-79
grammar slots `data` `transform` `mark` `encoding` `guide`.

Incorrectly promoted (29): `1/Icon/unit comparison`, `2/Before/after and change`, `3/Distribution curves`,
`6/Pie family`, `6/Donut family`, `6/Radial partitions`, `6/Rectangular composition`, `6/Unit composition`,
`6/Specialized`, `7/Trees`, `7/Dendrograms`, `7/Space-filling hierarchies`, `7/Organizational/human hierarchies`,
`7/Concept hierarchies`, `7/Specialized trees`, `8/General node-link graphs`, `8/Graph layouts`,
`8/Dense graph alternatives`, `10/Routes and movement`, `10/Continuous spatial fields`,
`15/Data/software architecture`, `17/Volume`, `17/Performance`, `17/Accounting/business`, `17/Economics`,
`26/Transform geometry`, `78/Shape generators`,
`79/A user should be able to describe essentially any visualization as:`, `79/This grammar then generates:`.

## Public API coverage

Complete for the contracted surface. `print/template/viz-gallery/viz-api.tex` (hand-written, untouched by this
review) uses all ten commands: `\SemioVizChart` `\SemioVizMark` `\SemioVizPath` `\SemioVizLayout` `\SemioVizAxis`
`\SemioVizGrid` `\SemioVizLegend` `\SemioVizTable` `\SemioVizRow` `\SemioVizScale`. The remaining public surface
declared across `semio-viz*.sty` is `\SemioVizChartKind` (exercised by all 79 chart packages), `\SemioVizDemo`
(exercised by all 80 numbered galleries) and the `VizFigure` environment (used everywhere), so nothing public is
unexercised. `viz-api-a.pdf` / `viz-api-b.pdf` in this ticket are byte-identical (16306 B), confirming the
determinism probe ran against `viz-api`.

Depth caveats are P2 below: three of the scales `viz-api.tex` defines are never consumed, and the
`VIZ_API_COMMANDS` gate is a substring match over the whole gallery directory.

## P0

None.

## P1

### P1-1 · 36 non-types shipped as public ChartKinds
`.repo/🎫/26/08/18/SEMIO-VIZ-TIK-Z-LIBRARY/generate.ts` (`isCategoryName`, lines 37–45) →
`print/asset/viz-taxonomy.{md,json}`, `print/template/viz-gallery/viz-{1,2,3,6,7,8,10,15,17,26,78,79}.tex`,
`print/tex/semio-viz-chart-{comparison,time-series,distribution,part-whole,hierarchical,graph-network,geospatial-cartographic,uml-software,financial-economic,geometry,d3-equivalent,ultimate-api}.sty`.

`isCategoryName` is a keyword heuristic and leaks on every heading whose wording escapes the word list. Examples
of what is now a public chart kind:

```
\SemioVizChartKind{a-user-should-be-able-to-describe-essentially-any-visualization-as}{chrome}{data=demo}
\SemioVizChartKind{this-grammar-then-generates}{chrome}{data=demo}
\SemioVizChartKind{specialized}{pie}{data=demo}
\SemioVizChartKind{performance}{line}{data=demo}
\SemioVizChartKind{trees}{tree}{data=demo}
\SemioVizChartKind{shape-generators}{chrome}{data=demo}
```

The seven children of `This grammar then generates:` come through as kinds too, keeping their source punctuation:
`conventional-named-charts` ("conventional named charts,"), `unusual-chart-variants`,
`domain-specific-scientific-figures`, `network-and-hierarchical-diagrams`, `maps` ("maps,"),
`completely-novel-visualizations`, `all-as-native-tikz-pgf-output` ("all as native TikZ/PGF output."). Those titles
are also emitted verbatim as `\begin{VizFigure}[title={maps,}]` in `viz-79.tex`.

Also inconsistent within a single section: `6/Pie family` and `6/Donut family` become kinds while `3/Box-family
plots` and `3/Violin-family plots` are dropped, purely because the latter contain a hyphen; `78/Shape generators`
becomes a kind while the seven sibling `* transforms` groups are dropped.

Fix direction: replace the heuristic with an explicit allow-list of the intermediate types that are genuinely
leaves-and-groups (Flowchart, Bode plot, Control chart, Combination chart, the section-76 namespaces, the
section-79 grammar slots) and treat every other parent as a category; drop the section-79 prose bullets from the
enumerable set. This regenerates ~36 fewer kinds across the manifest, 12 galleries and 12 packages.

### P1-2 · 15 stale chart packages committed in `print/tex`
`print/tex/semio-viz-chart-{calendar,chrome,correlation,financial,geo,hierarchy,network,part,process,ranking,science,statistical,text,trend,uncertainty}.sty`

94 `semio-viz-chart-*.sty` files exist; `semio-viz-charts.sty` requires 79. The extra 15 are tracked by git
(`git status --porcelain -- print/tex` is empty) and come from an earlier category-naming scheme with a conflicting
slug vocabulary — e.g. `semio-viz-chart-financial.sty` registers `candlestick`, `ohlc`, `volume`, `waterfall`,
`equity-curve`; `semio-viz-chart-part.sty` registers `pie`, `donut`, `sunburst`. Several of those slugs collide with
current taxonomy slugs under different families, so loading one of these files re-binds live kinds. `generate.ts`
writes packages but never prunes, so any future category rename silently adds more. Fix: delete the 15 files and
make the generator prune `semio-viz-chart-*.sty` files not in the current package set.

### P1-3 · Section 76 `charts` namespace missing
`generate.ts` line 40 (`/(charts|…)$/i`) → `print/asset/viz-taxonomy.md` section 76.

Section 76 lists eight top-level namespaces with children. Seven (`hierarchy`, `network`, `flow`, `geo`, `matrix`,
`diagram`, `scientific`) are leaves; `charts` alone is swallowed by the plural-suffix rule. Section 76 is the
"recommended top-level namespaces" deliverable, so its single most important entry is the one absent from the
manifest, the gallery and `semio-viz-chart-recommended-top.sty`. The other 60 section-76 leaves are present.

## P2

1. **Mark kinds shadow three ChartKinds.** `print/tex/semio-viz.sty:146-155` checks
   `\g_semio_viz_mark_kinds_seq` before `\g_semio_viz_chart_kinds_seq`, and
   `print/tex/semio-viz-mark.sty:32-45` registers the extra aliases `icon`, `image`, `text`, `bezier`,
   `catmull-rom`. So `\SemioVizDemo{icon}` (74/icon), `{text}` (79/text) and `{image}` (79/image) render mark
   primitives; the `chrome` kinds registered for them in `semio-viz-chart-visual-encodings.sty` and
   `semio-viz-chart-ultimate-api.sty` are dead. The output happens to be more apt than a plot frame would be, but
   the coverage gate is satisfied by a different code path than the manifest claims.
2. **Three section-51 plots lose their defining scale.** `symmetric-log-plot`, `power-scale-plot` and
   `square-root-scale-plot` fall outside `generate.ts`'s `/\baxis\b/` test, so they get `kind=chart` and register
   as bare `{chrome}{data=demo}` — i.e. the default linear y axis. The three charts named after a scale are the
   only ones in section 51 that do not show one. `extraOpts` (lines 239–247) only reaches `kind==="axis"`.
3. **Chrome coverage is nominal.** 247 of 269 `chrome`-family kinds and 39 of 40 `scale`-kind leaves register with
   bare `data=demo`, so ~250 gallery figures are the identical default frame+axis+grid+legend. Sections 74, 76, 77,
   78 and 79 are almost entirely this. Passing coverage says the slug resolves, not that anything distinguishes it.
4. **`vertical-bar` alias duplicates a taxonomy slug.** `generate.ts:251-253` injects it into
   `semio-viz-chart-comparison.sty` purely so `viz-api.tex` can call it, next to the real `vertical-bar-chart`.
   Point `viz-api.tex` at the taxonomy slug and drop `PUBLIC_CHART_ALIASES`.
5. **`viz-api.tex` defines scales it never consumes.** `api-sym` (symlog) and `api-q` (quantize) are declared but
   nothing maps through them, so the symlog/quantize/quantile/threshold branches fixed in phase 3 are not exercised
   at render time. Binding one of them to `\SemioVizAxis[scale=api-sym]` would close that.
6. **`VIZ_API_COMMANDS` is incomplete and loosely matched.** `print/script.ts:94-117` omits `\SemioVizChartKind`,
   `\SemioVizDemo` and the `VizFigure` environment, and greps the whole gallery directory, so a command mentioned in
   a comment anywhere would satisfy it. Scope the check to `viz-api.tex` and include the full public surface.
7. **`unknown-layout` falls through.** `print/tex/semio-viz-layout.sty:303-309` raises the error and then still
   expands `\use:c { semio_viz_family_<x>: }`, producing a second undefined-control-sequence error.
8. **`api-contracts.md` has drifted.** It documents `\SemioVizRow{name}{a}{1}{2}` (actual: `{ m m }`, cells as one
   clist — `print/tex/semio-viz-data.sty:161`) and `\SemioVizLegend[kind=swatch, items={a,b,c}]` (actual key is
   `legend`, no `items` — `print/tex/semio-viz-axis.sty:26,138`), and its families list omits `path`, `mark`,
   `anno`, `chrome`, which are four of the 32 live families.
9. **Ticket compile logs cover 60 of 80 sections.** Sections 0, 24–32, 48–51, 63, 64 and 74–78 have no
   `compile-*.log`. `print/dist` shows all 81×2 PDFs exist, so this is a record gap rather than an untested build.

## Files reviewed, not modified

`.repo/🎫/26/08/18/SEMIO-VIZ-TIK-Z-LIBRARY/{exhaustive-taxonomy.md,generate.ts,api-contracts.md,phase4-notes.md,category-packages.json,wp-registry.json,compile-*.log}`,
`print/asset/viz-taxonomy.{md,json}`, `print/template/viz-gallery/*.tex`, `print/tex/semio-viz*.sty`,
`print/script.ts`.

Added by this review (ticket-local, no source edits): `review.ts`, `review2.ts`, `review-output.txt`,
`review-output-2.txt`, `phase4-review.md`.

## Post-P1 addendum (multi-perspective re-review)

The FAIL above judged a **2001-leaf** snapshot. After the P1 parser (allow-list + prose skip + `76/charts`), stale-package prune, and gallery regen, that verdict is **stale**. This addendum reviews the live tree.

| Check | Post-P1 result |
| --- | --- |
| Independent catalogue parse | 1966 leaves, 80 sections, 0 cover/demo mismatches, 0 duplicate slugs, `76/charts` present, no `7/trees` / `6/pie-family` / `79/maps` / sentence kinds (`review-catalogue.md`) |
| Packages | 79 `semio-viz-chart-*.sty`; loader matches disk |
| Public API | `viz-api.tex` exercises 13 tokens including `VizFigure`, `\SemioVizChartKind`, `\SemioVizDemo`; `apiq` consumed as a bottom axis (`review-architecture.md`) |
| Contracts | `\SemioVizRow{name}{clist}`, `\SemioVizLegend[legend=…]`, 32 families |
| `unknown-layout` | `\seq_if_in:NnTF` then `\use:c`, else error — no missing-family fallthrough |
| Coverage | `bun ./print/script.ts test viz` → `1966/1966 leaves, API 13/13` |
| Full compile | `bun ./print/script.ts test viz full` exit 0: 162 PDFs, deterministic hash `2e7209f8ba5b` (`review-compile.md`) |
| Visual | 32 families used; `special` = 7; chrome-nominal ~259 accepted; mark-shadow of `74/icon`, `79/text`, `79/image` documented not inverted (`review-visual.md`) |

**Verdict: PASS.** No new P0/P1.

Remaining P2 documentation only:

1. Chrome kinds share one default frame (plan freeze: no family per leaf).
2. `\SemioVizDemo` prefers mark registry; `74/icon` / `79/text` / `79/image` draw marks.
3. Custom scale `apisym` with domain `-10,10` as an axis still hits `Invalid operation (0)/(0)` (scale storage via expl3 cs names). `apiq` with domain `0,10` compiles.
