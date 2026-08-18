# Visual / family mapping (post-P1)

Coverage is slug-resolution: each leaf is `\SemioVizChartKind{slug}{family}{opts}` or a section-0 mark. Geometry lives in 32 layout families. Distinct drawings equal families, not 1966 leaves.

## Families used

All 32 families registered in `semio-viz-layout.sty` appear in the manifest.

| family | leaves | drawing |
| --- | ---: | --- |
| science | 215 | vector arrows |
| geo | 209 | map-like frame |
| chrome | 259 | default plot frame + axis + grid + legend |
| process | 170 | three boxes and arrows |
| dist | 140 | distribution family |
| heat | 137 | heatmap |
| net | 130 | network |
| gantt | 77 | gantt |
| line | 88 | line |
| tree | 66 | tree |
| bar | 57 | bar |
| pack | 46 | pack |
| path | 40 | polyline |
| dot | 45 | scatter |
| bullet | 38 | bullet |
| pie | 34 | pie |
| chord | 30 | chord |
| text | 30 | word marks |
| radar | 26 | radar |
| area | 22 | area |
| anno | 21 | callout |
| funnel | 16 | funnel |
| mark | 13 | dot mark |
| waffle | 11 | waffle |
| special | 7 | radar fallback |
| force | 6 | force |
| calendar | 6 | heatmap alias |
| parallel | 5 | parallel coords |
| voronoi | 4 | voronoi |
| flow | 4 | flow |
| sankey | 12 | sankey |
| hexbin | 2 | hexbin |

`special` is 7 leaves (section fallback). Chrome is 259 leaves: sections 74 and 76–79 are almost entirely the same default frame. That is accepted (plan freeze: do not invent a family per leaf).

## Mark shadow

`\SemioVizDemo` tests `\g_semio_viz_mark_kinds_seq` first. Extra mark aliases `icon`, `image`, `text` (not the section-0 slugs `icon-mark` / `image-mark` / `text-mark`) therefore swallow:

- `74/icon` (scale / chrome)
- `79/text` (layout / chrome)
- `79/image` (layout / chrome)

The mark drawing is the better picture for those names. Chart-kind chrome registrations for those three slugs are dead. Coverage keys still match. Left as documented P2; demo dispatch is not inverted.

## Sample

Light PDF text extraction:

- `viz-1`: bar/column/dot/ranking/icon-unit sections, 16 pages
- `viz-7`: tree/dendrogram titles, 16 pages
- `viz-10`: political/physical/choropleth map titles, 20 pages
- `viz-74`: chrome encodings with `A B C D E` on most frames; `74/icon` draws a `+` mark (mark shadow)
- `viz-76`: namespace chrome frames (`timeline`, `table`, then `A B C D E` on annotation/dashboard/…)
- `viz-79`: grammar slots; `79/text` draws `Aa`; `79/image` is a mark, not a chrome frame

Underfull `\vbox` only on short figures.

**Verdict: PASS** as a family-alias library. Chrome-nominal coverage remains P2 documentation, not a fail.
