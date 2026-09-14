# Goal Verification (Sonnet, read-only, 2026-09-06 ~16:40)

Measured on `🧰️framework/🛍️products/📓️print/` (ripgrep-verified counts).

## Catalogue
- 1,738 kinds; 1,966 taxonomy leaves (mark 64, chart 1,633, axis 24, scale 40, layout 205); every leaf covered exactly once; 0 dangling covers.
- 245 families referenced, 253 registered, 0 unregistered; 8 registered-but-unused (`annotated-chart`, `concept-venn`, `notation-petri`, `adjacency`, `sci-forest`, `sci-timing`, `sci-field`, `sci-surface`).
- 0 kinds with an empty option set; 0 duplicate family+options signatures.
- Families without a `%region 🔖️Keys` block: `alluvial` (4 kinds), `parallel-sets` (1).

## Configurability
- 135 families declare their own `\keys_define` block; the rest delegate.
- Schema `x-semio-family-options` documents 245 families, but 117 of them list far fewer keys than the family implements (e.g. `text-viz` 2/30, `uml-class` 2/22, `monitoring` 2/20, `optimization` 2/20, `engineering-diagram` 2/19, `scale` 2/19, `spatial-layout` 2/18, `encoding` 2/17, `performance` 2/17, `pm-gantt` 2/17). Root cause: `catalog-coverage` only checks that keys used by kinds are declared, never that every implemented key is documented.
- 6 families only in `.sty`, not in schema: `annotated-chart`, `common`, `concept-venn`, `notation-petri`, `mark`, `adjacency`.

## Tests
- 100 cases, 316 scenarios (245 differential, 70 conformance, 2 property); 29 oracles (28 third-party + 1 cross-semio twin) and 19 no-oracle decisions.
- Differential coverage: scale, format, transform, mark/shape/curves, hierarchy, network (+dagre), flow, geo, spatial, guide, annotation (all also with the TS twin). Specification-only: coordinate, facet, composition. No dedicated case: theme, plot; namespaces `infographic`, `interactionstate` (7 kinds) untested.

## d3-likeness
- 137 public `\SemioViz…` commands. Full equivalents: array, scale, shape, hierarchy, force, chord, sankey, geo, delaunay, contour, hexbin, format, time-format, axis, dsv. Partial/internal: scale-chromatic, path, color, polygon, random. Missing public surface: interpolate, quadtree. DOM-only modules intentionally absent.

## Semio style
- 81 `semio-viz*.sty`, 75,559 lines; 0 `Pending-` regions; 4 legacy `\semio@viz@` tokens left (diagram-flowchart); legacy chart files gone; `semio-viz-layout.sty` is the 51-line dispatcher. Modules/commands registered in the `🔣️.json` manifests; nx targets and 18 launch entries present.

## TypeScript twin
- 13 modules, 283 exports, no runtime dependencies, own harness (237 checks per its feature).

## Documentation
- `🔓️viz-api.tex` 4,855 lines, 21 chapters, test-gated freshness; `README.md` present.

## Gaps to close (→ GAPS agent) — all four closed, details in `📓️status-GAPS.md`

1. **Done.** The debt was larger than this file measured: not 117 families short of a few keys each
   but **2 878 key documentation slots**, because the delegated vocabularies (`family / common` in 33
   families, `geo / base` in 17, `scale` in 12, the diagram kernel behind `flow`/`swimlane`, and the
   `keys_set_known` routing of the hierarchy families) were invisible to the earlier count. All are
   now documented, from 296 authored en+de descriptions fanned out over the declaration blocks they
   belong to. One *phantom* key was found too — `points` on `spatial-vector-field`, documented and
   set by 16 catalogue kinds and read by nothing — and removed. `@id-implemented-keys-documented`
   (`catalog-coverage`, fundamental, conformance) now asserts both directions against a key
   extractor ported into `vizImplementedFamilyKeys()`.
2. **Done.** `📰️infographic-kinds`, `🖱️interactionstate-kinds` (conformance, one new `@no-oracle-`
   decision each), `🌈️theme-palettes` (6 differential scenarios against `d3-scale`,
   `d3-interpolate`, `d3-color`) and `🪶️plot-grammar` (1 conformance + 3 differential against
   `d3-scale` and the TypeScript twin). Writing the first two found five defects nothing else could
   see, including three §45 and four §44 kinds that emitted no geometry at all, two kinds that drew
   the identical picture, and `\SemioVizChart{selected-node-network-state}` failing outright because
   `state-selection` rejected the `data` binding every catalogue kind passes it.
3. **Done.** `notation-petri` kept and `petri-net` re-pointed onto it (it is the only renderer that
   draws the token marking); `concept-venn`, `sci-forest`, `sci-timing`, `sci-field`, `sci-surface`,
   `adjacency` and `annotated-chart` deleted, each against the family that already serves its
   catalogue kinds better. `%region 🔖️Keys` added to `alluvial` and `parallel-sets` (37 keys each).
   `grep -rn 'semio@viz@' 🖋️latex/*.sty` is empty: the last four are now
   `\c_semio_viz_theme_frame_width_fp` / `…_height_fp` in `semio-viz-theme.sty`.
4. **Done.** `\SemioVizInterpolate{kind}{a}{b}{t}` (number, round, rgb, lab, hcl, oklab, array) with
   `🔀️interpolate-kinds` against `d3-interpolate`; `\SemioVizQuadtree{table}[x=,y=]` and
   `\SemioVizQuadtreeFind{x}{y}[radius]` with `🌳️spatial-quadtree` against `d3-quadtree` — a real
   quadtree, agreeing with d3 not only on the neighbours it finds but on the square it covers and on
   the number of nodes in the tree.

Measured after the work: **106 cases**, `parity quick` **550/550 with 259/259 pairs**, `parity long`
**630/630 with 294/294 pairs**, `test fundamental` **17/17 with 8/8**, `contract` **0 print
breaches**, `generate viz` 83 artifacts and no stale generated file.
