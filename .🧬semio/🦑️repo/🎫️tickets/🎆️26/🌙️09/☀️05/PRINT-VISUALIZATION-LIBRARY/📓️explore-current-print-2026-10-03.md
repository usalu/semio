# Current Print Visualization Architecture

Read-only exploration on 2026-10-03. No production files edited, no git mutations performed, and no tests run.

## Scope and Instructions

Applicable instructions are root AGENTS.md and 🧰️framework/🛍️products/AGENTS.md. There are no AGENTS.md files below the print product. Root rules require schema-first changes, language-neutral features with third-party oracle adapters, bun/Nx script routes, mutation/inference ownership, no modifying git commands and no worktrees. Products instructions describe the framework/platform/playground/OS ownership but add no print-specific restrictions.

## Existing Surface

Print root: `🧰️framework/🛍️products/📓️print`.

- `README.md` documents the grammar: data → transform → scale → coordinate → mark → guide → annotation → theme.
- `🖼️assets/📊️viz-taxonomy.md` is the human taxonomy.
- `🖼️assets/🔣️viz-catalog.json` parsed successfully and contains **1,738 kinds**.
- `🧬️schema/🔣️.json` owns catalogue validation, family option vocabulary, demo tables and probe protocol. `🧬️schema/🟦️.ts` is its typed twin.
- `🖋️latex/semio-viz.sty` loads grammar packages, opens millimetre TikZ canvases and dispatches public commands. `semio-viz-family.sty` and chart namespace packages register chart families. README documents 246 families.
- Family customization uses l3keys; schema family-option descriptions are bilingual English/German. Themes supply colours and stroke tokens.
- `🧾️template/📊️viz-api/🔓️viz-api.tex` is the authored public reference.
- `🔨️modules/📊️visualization-gallery/🟦️.ts` owns catalogue loading, family-key extraction, coverage reports and generated catalogues/gallery/API assets.
- Generated catalogue and labels stylesheets, API JSON and gallery sources must be regenerated through existing `generate viz` route rather than hand edited.

## Existing TypeScript Twin

`🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts` is a barrel over scale, format, transform, mark, shape, coordinate, hierarchy, network, flow, geo, spatial, theme and render domain modules.

Its schema twin declares:

- 15 scale kinds, 19 curve kinds, 13 symbol kinds.
- Cartesian, polar, ternary, barycentric, parallel, logpolar and geographic coordinates.
- 24 layout algorithms, including stack/bin/hexbin, hierarchy, force/DAG, chord/sankey, Voronoi/Delaunay, contour/density and projection.
- 16 encoding channels, with transform/layout options on layers and axis/legend/grid guides.
- Declarative `VizChartSpecification`, `VizLayerSpec`, data table and scale shapes.

`🔨️modules/📊️viz-kernel/🖼️render/🟦️.ts` resolves primitives and emits both scene nodes and TikZ. Its actual layer switch supports only bar, point, line, area, arc, text and rule. The current planner does not apply layer transform/layout declarations. It handles only a subset of channels and hardcodes guide drawing to bottom/left despite top/right schema orientations. The public spec breadth consequently exceeds the execution breadth.

## Mutation and Inference Routing

A text scan of print TypeScript, JSON and stylesheets found no actual mutation/inference runtime owner routes; matches were documentation, catalogue words and oracle rationale. A repository path scan found no print/viz mutation or inference implementations. Existing print behavior lives in separate modules. This is the central architecture mismatch with the requested mutation/inference-only implementation.

The global OS inference/service module at `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🟦️.ts` is a bounded installed-service/document-HTTP operation layer, so it should not be mistaken for a chart-specific inference owner. Canonical mutation contracts elsewhere live under artifact `🧬️schema/🧬️mutations/...` taxonomy paths. The inference exploration agent owns identification of the appropriate canonical pattern.

## Validation Infrastructure

- `🧪️tests/📚️catalog-coverage/🥒️.feature` and `🟦️.ts` validate shape with Ajv, taxonomy with markdown-it, unique slugs, registered families, option vocabulary, demo tables and generated-file freshness.
- `🧪️tests` already has language-neutral cases and TypeScript adapters for scales, formats, curves/symbols, hierarchy, graph/flow, geography, spatial contour/Delaunay and chart families.
- `🔮️oracles/🔣️.json` registers test-only d3 packages and explicit numerical no-oracle decisions.
- `🔨️modules/🧪️viz-probe/🟦️.ts` compiles TeX probe fixtures and reads protocol output; `🖋️latex/semio-viz-probe.sty` owns the TeX protocol.
- Existing test levels: fundamental (no TeX), quick (kernel probes), long (family probes/gallery), exhaustive (all kinds, appearances and languages).
- Entry scripts and Nx ownership live at `📦️packages/🟦️typescript/📜️script.ts` and `📋️project.json`; existing launch seed configurations live in `.vscode/🧩️launch.seed.jsonc`.

## Implementation Implication

The catalogue is already extensive. Prioritize canonical schema-backed mutation/inference ownership and making declared grammar features executable and customizable. Adding more catalogue names alone cannot provide D3-equivalent behavior. Reuse existing numerical twin/oracle coverage behind canonical ownership rather than introducing another independent visualization module or API.
