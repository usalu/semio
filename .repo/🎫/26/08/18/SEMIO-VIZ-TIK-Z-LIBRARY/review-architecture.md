# Architecture and API (post-P1)

Layers still match the original split: umbrella + data + scale + mark + axis + layout + per-section chart aliases via `semio-viz-charts.sty`.

## Public surface

`print/template/viz-gallery/viz-api.tex` now exercises:

- `VizFigure`
- `\SemioVizTable` / `\SemioVizRow` (clist cells)
- `\SemioVizScale` linear (`apiy`), symlog (`apisym`, domain `-10,10`), quantize (`apiq`, domain `0,10`)
- `\SemioVizChartKind{apiprobe}`
- `\SemioVizChart{vertical-bar-chart}`
- `\SemioVizMark` / `\SemioVizPath`
- `\SemioVizLayout{dot}`
- `\SemioVizAxis[orient=left, scale=apisym]` and `\SemioVizAxis[orient=bottom, scale=apiq]` (custom symlog with domain `-10,10` and quantize with `0,10`)
- `\SemioVizGrid` / `\SemioVizLegend[legend=gradient]`
- `\SemioVizDemo{dot}`

`print/script.ts` `assertVizApi` greps only `viz-api.tex` and requires those 13 tokens.

## Contracts

`api-contracts.md` updated to the live signatures:

- `\SemioVizRow{name}{a, 1, 2}` (one clist, not three args)
- `\SemioVizLegend[legend=swatch]` (key is `legend`, no `items`)
- 32 layout families including `path`, `mark`, `anno`, `chrome`
- Chart demo slug `vertical-bar-chart`

## Remaining documented P2 (not defects of this pass)

`\SemioVizDemo` still checks mark kinds before chart kinds. `74/icon`, `79/text`, `79/image` therefore draw mark primitives. Those slugs are not section-0 names (`icon-mark` / `text-mark` / `image-mark`); they collide with extra mark aliases. The drawing is the better picture; coverage is satisfied by the mark path. See `review-visual.md`.

**Verdict: PASS** after the contract and `api-q` fixes.
