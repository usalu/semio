# Semio Style Conventions for a Static Visualization Library

Read-only exploration by a Sonnet explorer (2026-09-05), persisted by the coordinator.

## Taxonomy
- SSOT: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (`fileKinds`: `.ts→🟦️`, `.rs→🦀️`, `.py→🐍️`, `.json→🔣️` role `schema`; `langs`; `packagesDirName=📦️packages`; `targetsDirName=🎯️targets`; `semanticCollections`: `🔨️modules`, `🎮️commands`, `🛍️products`, `🏅️standards`/`🪆️subsets`, `🧬️mutations`; `testLevels`, `testModes` (`differential, conformance, round-trip, property, error`), `testComparisonProfiles`; single-emoji-grapheme path identity with VS16).
- Product shape (`📓️print`): `🎮️commands/<emoji><verb-noun>/🟦️.ts [+🧪️tests/]`, `📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts}`, `🔨️modules/<emoji><name>/🟦️.ts`, `🖼️assets/`, `🧾️template/`.
- Every `🔨️modules/` and `🎮️commands/` dir has a sibling `🔣️.json` `x-semio` collection manifest with `id`, `responsibility`, `productionConsumers`.

## Schema-first
`🧬️schema/🔣️.json` (JSON Schema with `$id`, `$defs`, `discriminator`) + hand-written typed twins `🧬️schema/🟦️.ts` and `🧬️schema/🦀️.rs` validated by differential tests (example `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/`). Ajv cross-validation example: `🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🧪️oracle/🟦️.ts`.

## Multi-implementation
`<module>/📦️packages/{🟦️typescript,🦀️rust,🐍️python}/` siblings, each with manifest, `📋️project.json`, `📜️script.ts`. Examples: `🖱️ui/🎨️styling/📦️packages/`, `💻️os/📦️packages/`, `🦑️repo/🔨️modules/🧪️test/📦️packages/` (5 languages).

## Tests
- Router `📜️script.ts test [level]` → command class `run(segments)`; `resolveTestLevel`.
- Gherkin `🥒️.feature` + `defineTestAdapter({ implementation, scenarios: { name: { oracle, subject } } })`; oracle = third-party lib (`semver` in `🎠️kernel/🧪️tests/✅️satisfy-version-requirements/🟦️.ts`) or an independent second implementation in another language (`✏️s/…/🧪️tests/✒️mutate-writer-1/{🥒️.feature,🦀️.rs,🐍️.py}`, Python registered as oracle only).
- Language-agnostic JSON fixtures beside the adapters; `markdown-it` vs own parser is print's existing differential example.

## Docstrings
`/** <emoji> … */` above exports; `//#region 🔖️Name` … `//#endregion 🔖️Name`; no comments inside definitions; `[DEBUG] ` prefix for temporary logs.

## Design tokens (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`)
`colors` (primary `#ff344f`, secondary `#34d1bf`, tertiary `#fa9500`, danger/warning/info/success, 33-step gray ramp), `presence.hues` = 12-hue categorical wheel with light `{s:0.68,l:0.32}` / dark `{s:0.72,l:0.62}`, `fontStacks` (Anta, Kelly Slab, Share Tech Mono, Noto Emoji), `typography` sizes, `strokes.grid*` weights, `appearances.{light,dark}` paint references. Print already consumes it in `🎨print-design-token-paints/🟦️.ts`.

## Reusable pure libraries
- `🧰️framework/🔨️modules/◻️2d/🟦️.ts`: dependency-free 2D scene graph (`DrawingScene`, `DrawingNode` rect/ellipse/circle/line/polygon/path/text/group, `PathSegment`, fill/stroke styles). Natural IR for a viz library.
- `🧰️framework/🔨️modules/🕸️graph/🧮️algorithms/🦀️.rs`: BFS, topo sort, components, SCC, Dijkstra.
- `🧰️framework/🔨️modules/📐️geometry/`, `🧮️math/🎯️sampling/` (Rust only).
