# Runtime Variant Carve (koordinator->cad, aggregator->puzzle3d, bearbeiten->process3d, verfolgen->gis2d)

## Changed
- `🧫️pipeline.json` variants: aussuchen, cad, energy, fem3d, generation3d, generator, gis2d, process3d, puzzle3d.
- `📋️project.json` prepare-prototype/-release dependsOn: same nine lanes (`prepare-<variant>-react-<profile>`). `activate-lanes-prototype` unchanged (energy, fem3d, generator): cad/puzzle/process/gis/procedural stay inside the primary lane's closure.
- `🔨️modules/🧩️runtime/🟦️.ts`: `DEMONSTRATOR_RUNTIME_TARGETS` now derives from `pipeline.primaryVariant` + pane `runtimeVariant`s only (no longer the branded pane variants, which all resolved to pluginId `demonstrator`).
- `🔨️modules/🧩️runtime/🧫️cases.json`: additionalBuildVariants = generation3d, cad, puzzle3d, process3d, gis2d, energy, fem3d.
- `📇️catalog/🧫️cases.json`: brandImports gains `@semio-tech/framework/assets/published-page` (pre-existing stale fixture; brand.ts already imported it).
- `🧪️tests/🧪️demonstratorcompileclosure/🎭️e2e/🧫️cases.json`, `README.md` (EN+DE), repo-lib `🎪️demonstrator-targets` test: lane lists updated.

## Tests (repo test-file)
- runtime/🧪️tests/🟦️.ts pass; demonstrator-targets pass; serveclosure pass. vitest in-source suites (panebootvariants, unionreceipt, examples) run 0 tests under bun (pass vacuously); their data is unaffected (unionreceipt is synthetic, panebootvariants koordinator row stays valid via the catalog).

## Examples
Brand default exampleIds (cad demo, puzzle3d concrete-forest, process3d concrete-forest, gis2d demo) all exist as owner playground examples (cad: demo, demo-session; puzzle3d: capsule-dream, concrete-forest, demo-session, nakagin; process3d: concrete-forest, demo, demo-session; gis2d: demo, demo-session). The demonstrator plugin offered its own 14-example list for every pane; owner variants offer only their own, so the example pickers shrink to those lists (no default changes). Content of same-named examples comes from the owner artifacts (the examples test already sources them there).

## Withheld extensions and bearbeiten runtime variant

- `withheldExtensions` (8 ids) is required in `🔨️modules/🧩️runtime/🧬️schema/🔣️.json` and `🔨️modules/🧩️runtime/📇️catalog/🧬️schema/🔣️.json`; the catalog test now asserts that `demonstratorShippedComponents` drops exactly those ids and that each is a known extension id.
- `bearbeiten` replaces `process3d` in the pipeline variants, the prototype and release preparation dependencies (`📋️project.json`, compile-closure e2e fixture), the README (EN and DE), and the repo-lib `🎪️demonstrator-targets` expectation. `bearbeiten` resolves to the `demonstrator` plugin, so it adds no extra build variant.
- Runtime fixture: `additionalBuildVariants` drops `process3d`; `extensionIds` drops the 8 withheld ids; `pluginIds` unchanged.
- Results: `repo test-file "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️tests/🟦️.ts"` passed (1 pass); `repo test-file` on `🎪️demonstrator-targets/🟦️.ts` passed (3 pass).
