# Print Visualization Gallery & Taxonomy — Reality Check

Read-only exploration by a Sonnet explorer (2026-09-05), persisted by the coordinator.

## 1. Taxonomy structure
- Source of truth: `🖼️assets/📊️viz-taxonomy.md` (2,617 lines); `🖼️assets/🔣️viz-taxonomy.json` is a derived mirror: 1,966 entries `{id, slug, title, kind, family, section}`.
- 80 sections `## 0.` … `## 79.`; sections 74–79 are meta (encodings, layout algorithms, namespaces, composition, d3-equivalents, API objective).
- Leaf grammar: `` - Title `slug` (mark|chart|layout|axis|scale) ``. id = `section/slug` (`🔨️modules/📊️visualization-gallery/🟦️.ts:19-32`). 205 slugs carry `-<section>` suffixes; 166 base names still collide across sections.
- Kind counts: chart 1,633, layout 205, mark 64, scale 40, axis 24. `family` (32 values) is a rendering tag.

## 2. Coverage reality
`% viz-covers:` is a presence claim only (`🟦️.ts:34-42`). All 1,966 leaves are covered 1:1, but rendering is family-level: `\SemioVizDemo{slug}` → chart kind → family → one of 32 `\semio_viz_family_<name>:` routines in `semio-viz-layout.sty`, all on the same 5-row demo table.

| Family (leaves) | Renderer | Draws |
|---|---|---|
| chrome (259) | :576-582 | frame + axis + grid + legend, no marks |
| science (215) | :520-524 | 3 fixed arrows |
| geo (209) | :364-368 | one fixed pentagon + 2 dots |
| process (170) | :526-569 | data-driven chip chain, identical shape |
| heat/calendar/hexbin (145) | :403-, :444, :512 | data-driven matrix |
| net/pack (176), tree (66) | :335-345 | fixed tree glyph |
| dist (90) | :331-333 | alias of bar |
| bar (57) | :218-231 | 5 bars, no grouping/stacking |
| bullet (38), text (30), chord (30), funnel, sankey/flow, force, special, path, mark, anno | fixed glyphs | |

Estimate: ~68–70% of chart leaves render a hard-coded, data-independent glyph; the rest are data-driven but indistinguishable within a family. Only the 64 marks are individually distinct.

## 3. Gallery document structure
- 81 `.tex` files: `viz-0` … `viz-79` (one per section) + `viz-api`. No `-dark` sources; dark variants derive at build (`🖨️tectonic-template-compilation/🟦️.ts:73-81`, written to `.semio-dark/`).
- Each file: `\documentclass[type=report,theme=light,language=de]{semio}`, one `\chapter`, one `\section` per subsection, then `% viz-covers:` + `VizFigure[title, width=80, height=40]` + `\SemioVizDemo{slug}` per leaf. `VizSection`/`VizColumn` only appear in `viz-api`.
- Figures per file range 8 (`viz-39`) to 76 (`viz-10`). Build runs tectonic with `--reruns 2` plus a panel pass.

## 4. `viz-api.tex`
5 sections exercising tables/scales/charts, marks/paths, layout/axis/grid/legend, sections/columns/text, text kinds; enforced by `assertVizApi` (17 tokens textually present).

## 5. Template registration
- `TEMPLATES` (`🖨️tectonic-template-compilation/🟦️.ts:38-45`): report, paper, flyer, forschungsbericht, zwischenbericht, kompaktbericht. Gallery documents enumerated separately by `visualizationTemplates()`.
- `resolveRegisteredPrintTemplates(filter)`: `viz` first segment switches catalog.
- Production templates contain no `\SemioViz*` usage; the viz library is a self-contained showcase. External consumers of print LaTeX: `♻️mit-bestand/📋️bericht/**` (three reports).
