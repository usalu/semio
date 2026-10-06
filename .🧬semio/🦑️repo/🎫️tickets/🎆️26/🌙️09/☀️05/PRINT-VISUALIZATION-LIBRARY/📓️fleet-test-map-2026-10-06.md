# Fleet Test Map — 2026-10-06

Read-only exploration; no tests executed and no implementation files changed.

## Rules

Root `AGENTS.md` and `🧰️framework/🛍️products/AGENTS.md` are relevant ancestors. No print-local AGENTS file was found. Use Bun/Nx, schema-first changes, language-neutral test vectors and an independent third-party oracle, explicit en/de, ticket-local generated output, and existing script routers. Do not modify git, create worktrees, or edit AGENTS.

## Owned Paths

All paths below are relative to `C:/git/semio/🧰️framework/🛍️products/📓️print`.

- `🧬️schema/🔣️.json`: chart, catalogue, probe contract and `x-semio-family-options`.
- `🧬️schema/📸️snapshot/📊️chart/🟦️.ts`: authored typed chart twin.
- `🧬️schema/🧬️mutations/🟦️.ts`, `🧬️schema/🔀️diff/🟦️.ts`: mutation and replay/inverse contract.
- `🖼️assets/🔣️viz-catalog.json`, `🖼️assets/📊️viz-taxonomy.md`: catalogue and taxonomy.
- `🖋️latex/semio-viz-*.sty`: native renderer and chart families.
- `🧪️tests/🧬️native-chart-grammar/🔣️.json`, `🥒️.feature`, `🟦️.ts`, `🧬️schema/`: neutral fixtures, language-agnostic acceptance scenarios and actual compiled output checks.
- `🧪️tests/🎬️render-scene/{🔣️customization.json,🔣️baseline.json,🔣️axis.json,🔣️legend.json,🥒️.feature,🟦️.ts}`: numerical scene customization and baseline fixtures.
- `🔨️modules/🧪️viz-probe/🟦️.ts`: compiled Tectonic probes, structured JSONL and PDF paths.
- `🔨️modules/🖨️tectonic-template-compilation/🟦️.ts`: repository compiler integration.
- `🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/{🔣️.json,📜️script.ts}`: pinned Tectonic 0.16.9 cross-platform provisioning.
- `🎮️commands/🧪️print-pipeline-verification/🟦️.ts`: quick/coverage/full verification and fixture routing.
- `📦️packages/🟦️typescript/{📜️script.ts,📋️project.json,package.json}`: Bun router, Nx commands and existing independent-oracle dev dependencies.

## Existing Commands

`bun nx run @semio-tech/print:test-native-grammar --skip-nx-cache` depends on deps-tectonic, deps-tex and generate. `PRINT_NATIVE_GRAMMAR_PHASE` chooses a focused native suite. `PRINT_NATIVE_GRAMMAR_WORK_DIR` redirects its artifacts; set it under this ticket's `🗑️generated/native-grammar` to honor ticket output rules. Native router also supports `--grammar-only` and `--families-only`.

Other existing Nx targets are `test-viz`, `test-viz-fixtures`, `test-viz-full`, `test-toolchain`, `build-viz`, `test-fundamental`, `test-measurement` and `test-macro-staging`. `.vscode/launch.json` already registers many focused phases around lines 27258–28059 and whole-gallery commands around lines 42556–42648. Extend the corresponding group if adding executable phases.

## Validation Strategy

For each newly admitted control, add schema-first neutral JSON vectors plus Gherkin, including authored zero, omitted value, invalid value, explicit empty reference and real alternate settings. Validate mutation acceptance, replay, inverse and purity. Compile inferred TikZ for explicit English/German and light/dark variants. Compare numeric probe values and actual painted PDF operators with independent D3 scales/shapes/geometry. Check that changing the control changes the relevant painted result, not merely the emitted source. Run a focused native phase followed by gallery coverage; broaden only where affected contracts require it.

The package already has D3 modules as devDependencies (array, chord, color, contour, delaunay, dsv, force, format, geo, hexbin, hierarchy, interpolate, path, polygon, quadtree, random, regression, sankey, scale, scale-chromatic, shape, time, time-format), Ajv and pdfjs-dist. No new runtime oracle dependency is needed. Native tests already inspect marked PDF content, geometry and alpha, rather than relying only on source-string comparisons.

## Observed Parse Blocker

At exploration time, `🧪️tests/🧬️native-chart-grammar/🟦️.ts` contained literal conflict markers at lines 8–12. The alternatives import `inferVizChart` from the owned host worker versus `inferVizChart,validateVizChartSpecification` from schema inference. This prevents parsing native suites until the concurrent work resolves those markers. Explorer did not edit them or run tests.
