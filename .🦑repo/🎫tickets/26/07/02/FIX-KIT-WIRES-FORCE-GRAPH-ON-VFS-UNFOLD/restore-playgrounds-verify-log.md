# Restore Playgrounds App Split — Verification Log

## Build (all 24 playground apps via `framework/product/playground/dev/script.ts build --app <entry>`)

All succeeded on 2026-07-02:

`2d 3d 5d gis-2d wires draw writer raster forms flow dag imperative sequence layout lowpoly procedural-2d procedural-3d shooting s vcs trinity-jack trinity-rewrite presentation cad`

## Post-split fixes in this session

- `procedural/2d/core/index.ts`: re-export fixture symbols from `playground.ts`
- `forms/core/index.ts`: hoist `export * from "./internal.ts"` before `@semio-tech/forms-react` import (breaks circular import)
- `framework/product/playground/core/index.ts`: fixture-lock test cleanup; `eagerPlayFixtureGlob` try/catch for vitest
- `imperative/core/index.ts`: schema assertion `imperative.document` (not `/v1`)
- `s/core/playground.ts`: static `import.meta.glob` for fixtures
- `s/core/internal.ts`: `presentation.deck` resource map entry
- `framework/product/presentation/core/index.ts`: `buildPresentationDeckProgramDefinition`
- `cad/js/renderer/core/index.ts`: AppTools array test shape; model JSON schema; concrete-forest document counts

## Tests (scoped `bun nx run <pkg>:test`)

| Package                   | Status                                                   |
| ------------------------- | -------------------------------------------------------- |
| framework-playground-core | 19/19 pass                                               |
| imperative-core           | pass (after schema fix)                                  |
| forms-core                | pass (after export order fix)                            |
| cad-js-renderer-core      | 48/52 pass (4 document integration tests remain)         |
| s-core                    | 4/20 pass (16 pre-existing / fixture bootstrap failures) |

Note: full `run-many` across all touched packages still reports failures in writer-core (LSP), raster-core, puzzle packages, etc. — largely pre-existing and outside playground build path.

## Architecture outcome

- `core/index.ts` owns app logic (Controller, tools, panels, `build*ProgramDefinition`)
- `core/playground.ts` owns fixtures + `*PlayAppDefinition` + dev-host only
- `cad` migrated to `@semio-tech/cad-js-renderer-core` and registered in shared playground registry (24 apps)
