# Print Visualization Contracts Exploration

Read-only exploration on 2026-10-03. No tests were run and no production files were changed.

## Applicable Instructions

Root AGENTS.md requires schema-first, multi-implementation, language-agnostic feature tests with third-party output comparison, Bun/Nx routing, launch registration, ticket-owned reports, and concurrent work without modifying Git commands. Products AGENTS.md contains brief framework descriptions; there is no print descendant AGENTS.md.

## Existing Surface

Owner: `🧰️framework/🛍️products/📓️print`.

- `🧬️schema/🔣️.json` is the normative catalogue, family option, demo table and probe schema; its TypeScript twin is `🧬️schema/🟦️.ts`.
- `🖼️assets/🔣️viz-catalog.json` currently has 1,738 kinds and 246 unique families. `🔣️viz-taxonomy.json` and `📊️viz-taxonomy.md` provide the taxonomy.
- `🔨️modules/📊️viz-kernel/🧬️schema/🟦️.ts` describes chart specifications, layers, channels, scales, transforms and layout choices. Primitive types use millimetres; labels have English and German fields.
- Numerical TypeScript twins occupy existing `viz-kernel` children: scale, transform, mark, shape, format, coordinate, hierarchy, network, flow, spatial, geo, theme and render.
- The LaTeX implementation is in `🖋️latex/semio-viz*.sty`. The public API reference is `🧾️template/📊️viz-api/🔓️viz-api.tex`.
- Generated catalogue styles, API JSON and gallery documents must not be edited manually; the existing `generate viz` route owns them.

## Confirmed Architecture Gaps

`🔨️modules/📊️viz-kernel/🖼️render/🟦️.ts:70` implements `planVizChart` as direct calls into separate numerical modules. That production route does not satisfy the requested mutation/inference-only architecture. Existing numerical implementations can inform leaf relocation, but a new facade that still calls them would preserve the forbidden route.

The renderer handles only bar, point, line/area, arc, text and rule. Per-layer `transform` and `layout` are declared by the typed chart schema but never applied. Most fill, stroke, opacity and other declared encodings are ignored. Guides mostly collapse to bottom/left placement despite declared top/right orientations; declared title, formatting and legend semantics are not fully rendered. Annotations are absent from the chart specification rather than a declared-but-unused property. `language` is optional, which permits a chart without an explicit language. These are concrete integration gaps despite the large catalogue.

## Tests and Oracles

Language-neutral cases live under `🧪️tests/<case>/🥒️.feature`, with TypeScript adapters in the adjacent `🟦️.ts`. The scatter trend case imports d3-regression and d3-scale and projects compiled TeX numeric probe records. `🪶️plot-grammar` separately compares placement with d3-scale, then checks TypeScript/TeX parity. Twin parity supplements independent evidence; a twin must not become its own sole oracle.

`🔨️modules/🧪️viz-probe/🟦️.ts` supplies `compileVizProbe`, `compileVizProbeDocument`, projection and rounding. `🖋️latex/semio-viz-probe.sty` supplies the other implementation of the probe protocol.

`🔮️oracles/🔣️.json` already registers d3-array, chord, color, contour, delaunay, dsv, force, format, geo, hexbin, hierarchy, interpolate, path, quadtree, regression, sankey, scale, scale-chromatic, shape, time and time-format. Its test-only metadata should be preserved. Existing render-scene tests use handcrafted conformance outputs, so they do not alone meet the request for third-party output validation of new numerical features.

## Execution Routes and Caveats

`📦️packages/🟦️typescript/📋️project.json` names `@semio-tech/print` and registers generate-viz, test-fundamental, test-quick, test-long, test-exhaustive, test-viz, test-viz-full, build-viz and toolchain preparation through existing script routes.

`🎮️commands/🧪️print-pipeline-verification/🟦️.ts` currently sends quick/fundamental testing to `verifyPrintPipelineQuick`. It does not dispatch the Gherkin D3 numerical cases, contrary to the README description. Do not treat a passing print quick command as verification of newly authored numerical cases.

The independent `@semio-tech/print-viz-kernel` Nx project has quick/long/exhaustive routes. Its `📜️script.ts` calls the custom differential check table in `🔬️probes/🟦️.ts`, with optional module filters. Its build command merely imports the barrel; it is not a static TypeScript type-check despite its docstring wording.

Launch configurations are registered in `.vscode/🧩️launch.seed.jsonc`; existing print viz launch names include dev watch and generate viz. New executable verification routes should be added consistently there and routed through the owner script.

## Feasible Test Plan

1. Define mutation input, inferred chart specification, render primitive and TikZ projection JSON fixtures before implementation; retain explicit language and customization values.
2. Add one Gherkin feature per new mutation/inference capability using JSON vectors. Validate normative JSON with existing AJV oracle.
3. Compare scale, curve, pie, hierarchy, graph, geospatial, contour and flow numeric projections to their already registered D3 package. Include non-default customization and degenerate values, not just default presets.
4. Replay mutations, verify inferred results without separate module imports, and verify cancellation/progress where computation is expensive.
5. Check scene/TikZ primitive parity as a second implementation assertion after independently validating numeric output.
6. Compile representative TeX projections with the existing ticket work directory and probe harness. Verify both appearances and explicit English/German variants.
7. Register and execute the actual domain test runner through Bun/Nx. Separate schema validation, numerical differential output, TeX compilation and raster evidence in reporting.

No external runtime dependency is needed for these steps; D3 remains test-only.
