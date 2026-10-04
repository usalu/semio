# Final Catalogue Visual QA Shortlist

Inspect only PDFs published by the final source-frozen catalogue gate, in both light and dark. Earlier seven discovery pairs are not final artifacts. All paths below resolve from `C:/git/semio/🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/dist/documents`.

Select the first PDF page containing the precise localized figure title below as a figure title line, then rasterize that page with Poppler. Do not match a substring in a section/chapter heading (for example, `Kreisdiagramm-Familie` does not identify the `Kreisdiagramm` figure). These titles identify actual authored charts rather than cover/chapter pages. The gallery source uses `language=de` in both themes; English equivalents are provided for identification. Page numbers must be located in the final PDFs, since content and page breaks can change.

| Area | Document and PDF basename | First chart title to locate | Expected authored geometry and light/dark checks |
|---|---|---|---|
| Categorical chart | `viz-1/📊️viz-1.pdf` | `Vertikales Balkendiagramm` / Vertical bar chart | Vertical grouped bars have distinct heights, shared baseline and category spacing. Axes/text remain legible; fill and thin outlines remain distinct from each theme background. |
| Hierarchy | `viz-7/🌳️viz-7.pdf` | `Gewurzelter Baum` / Rooted tree | Top-down tree has circular nodes and straight parent-child links. Root-to-child levels and node spacing are visible; connectors and text are readable in both themes. |
| Network | `viz-8/🕸️viz-8.pdf` | `Ungerichteter Graph` / Undirected graph | Force layout has visible connected nodes, lines without directed arrowheads, and a single node color (`colorBy=none`). Edges remain distinct from page/window backgrounds. |
| Geography | `viz-10/🗺️viz-10.pdf` | `Politische Karte` / Political map | Region polygons, borders and region labels render with neutral palette. Adjacent regions stay separable and labels remain legible in both themes. |
| Scientific | `viz-30/⚛️viz-30.pdf` | `Freikörperbild` / Free-body diagram | Force arrows and body geometry are visible, with clear arrowheads/directions. Thin scientific notation and diagram outlines remain readable on both backgrounds. |
| Annotation | `viz-50/💬️viz-50.pdf` | `Annotiertes Diagramm` / Annotated chart | Callout annotation is attached to the plotted chart with visible text and connector. Annotation must remain inside the figure canvas and distinguishable from marks in both themes. |
| Radial regression | `viz-6/🍰️viz-6.pdf` | `Kreisdiagramm` / Pie chart | Filled wedges join at a common centre and form the complete disc (`inner=0`), without reversed/offset arcs. Adjacent category fills remain distinguishable in both themes. |
| Flow regression | `viz-9/🌊️viz-9.pdf` | `Alluvialdiagramm` / Alluvial diagram | Bands connect successive node columns with visible widths and gaps (`nodePadding=2`); no missing flow geometry after structure-backend dispatch repair. Bands and nodes remain readable in both themes. |
| Ordering regression | `viz-64/⛓️viz-64.pdf` | `Commit-Zeitstrahl` / Commit timeline | Linear arc layout shows named commit labels ordered by `order=name`, plus directed connectors. Inspect this specifically located chart page, since it is the fifth authored figure rather than the first. Numeric/probe ordering remains covered independently by the native D3 regression. |

Dark PDF basenames append `-dark` before `.pdf`. For example, the chart pair is `viz-1/📊️viz-1.pdf` and `viz-1/📊️viz-1-dark.pdf`.

Source validation: document IDs and PDF basenames were read from the current compiler catalogue, first chart ordering from the authored gallery TeX, and family/options/title expectations from current `🖼️assets/🔣️viz-catalog.json`. This is a planned visual QA checklist, not a claim that final images have already been inspected.

