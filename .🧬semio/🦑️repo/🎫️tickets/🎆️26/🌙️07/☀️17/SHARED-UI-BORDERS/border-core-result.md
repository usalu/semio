# Border Core Result

## Changed Scope

All UI paths relative to `🧰️framework/🔨️modules/🖱️ui/`:

- `🔨️modules/📏️border-presentation/🟦️.ts`: pure axis-aligned sweep resolver and public types, exact approved API in `border-core-api.md`. Stable owner/ID tiebreak; hover exceeds every transient effect; immutable input; rejects malformed segments and omits zero-length points.
- Same directory: `segments.schema.json`, `cases.json`, `segments.test.ts`. Neutral fixtures include partial parent/child overlap, reversed endpoints/order, ties, gaps, and hover over all effects. Independent brute-force implementation uses installed lodash group/interval/order utilities and reproduces fixture outputs; Ajv validates fixture input; 40 generated fractional mixed-axis sets compare sweep against oracle in both input orders. No added runtime dependencies.
- `🔨️modules/🎛️chrome-control-presentation/🟦️.ts`: canonical shared-frame/shared-cell markers replacing border utility seams.
- `🔨️modules/🎨️chrome-class-names/🟦️.ts`: single shared-cell divider, removes active extra border/negative overlap.
- `🎨️styling/🖌️ui/🎨️.css`: eliminates inset-shadow seams, supports attribute and class cell contracts, vertical groups, next-sibling hover independent of order; earliest important ownership/hover layers defeat active utilities. Attached sides preserve layout widths and suppress paint only. Navbar/footer pseudo-edges suppress only after runtime ownership. Broad child border reset removed; wrapper-only reset and keyboard focus preserved. Silhouette geometry and effect styles untouched.

## Validation

Test-driven initial test failed because resolver export did not exist, then implemented. Final focused result: **9 tests pass, 137 assertions**.

Current runnable command after lead integrated these tests into the normal Vitest suite (the original Bun validation below is historical):

```sh
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light node -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS/run-ui-tests.mjs' 'segments.test.ts'
```

Focused source/test TypeScript check passes:

```sh
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light bun -- ./node_modules/typescript/bin/tsc --noEmit --skipLibCheck --target ES2022 --module preserve --moduleResolution bundler --resolveJsonModule --allowImportingTsExtensions --types bun './🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/🟦️.ts' './🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/segments.test.ts'
```

PostCSS parsing/structural rules tested. Canonical CSS also compiled in memory using existing `@tailwindcss/node` (183253 characters with selected candidate utilities). Read-only final diff inspected; `repo tools exec --light git -- diff --check` passes. No generated output files retained.

## Integration / Remaining Verification

- Lead owns integrated browser validation and full UI suites. No browser/layout/forced-color runtime claims made here.
- Existing `🧪️tests/🧪️owned-locale-detector-retirement/🛠️control-chrome-2/🟦️.tsx` lines 31–36 assert obsolete `border-e`/`!border-normal` divider utility names. Lead should update those assertions to `shared-border-cell` (outside this agent's ownership).
- Runtime must set `data-shared-border-attached` only for sides replaced in SVG, as approved. CSS deliberately does not infer attachment or suppress arbitrary chip children. Cells use one DOM border seam; rectangular outer frames use compositor registration. Geometry/menu integration belongs to their owners.
- Root CSS layer ordering is intentional: important ownership suppression wins over hover; important hover paint wins over active/focus utilities and effects, except forced-colors chooses system Highlight.
