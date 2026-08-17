# Wave 2 fix-up #2 — gave `🟦️catalog.ts` a real package home

The catalog-injection agent created
`💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️catalog.ts` one level
above the `📇️registry/` package boundary (no `package.json` there), so its 4
in-product importers all used relative paths (fine — same technology) but
`♻️mit-bestand/🧺️demonstrator/📦️index.tsx` (a DIFFERENT technology, must
import cross-technology only via an `@semio-tech/*` package per
`.dependency-cruiser.cjs`'s `no-cross-technology-*` rule) still called the
OLD 1-arg `resolvePlaygroundBoot(bootVariant)` signature — a real regression
this ticket caused, just in a technology outside the framework/s/plugin
layering scope.

## Fix
- `git mv`'d `🟦️catalog.ts` into `📇️registry/🟦️catalog.ts` (the existing
  `@semio-tech/plugin-registry` package boundary) and fixed its now
  one-level-shorter relative imports of the generated files.
- Added a `"./catalog": "./🟦️catalog.ts"` subpath export to
  `📇️registry/package.json`.
- Repointed all 4 existing in-product relative importers (dev
  `🟦️component.ts`, `🧩️multi.tsx`, `ShellHost/🟦️component.tsx`, wgpu
  `🟦️boot.ts`) to the new one-level-deeper path — verified every one
  actually resolves to the moved file (wrote a small Python path-resolution
  check rather than trusting sed/perl output, since Wave 1's rename already
  taught the lesson that macOS `sed` regex mistakes fail silently).
- `♻️mit-bestand/🧺️demonstrator/📦️index.tsx` now imports
  `{ PLUGIN_CATALOG }` from `@semio-tech/plugin-registry/catalog` and passes
  it as `resolvePlaygroundBoot`'s new first argument.

## Verification
Scoped `tsc --noEmit` including both the demonstrator file and the moved
`🟦️catalog.ts` (config kept at `w2fixup-tsconfig.jsonc` in this folder for
reference) — zero errors mention `resolvePlaygroundBoot`, `PLUGIN_CATALOG`,
`catalog.ts`, or `plugin-registry`, and zero `TS2554`-class argument-count
errors anywhere in the demonstrator file. The ~90 other diagnostics the
scoped check surfaced are all pre-existing framework-wide debt (the same
"type exported only via `export *`, never locally imported" pattern the
wave-2 agents already found in `glue.ts`/`manifest.ts`), unconnected to this
edit — confirmed none reference any file this fix touched.

Did not verify `@semio-tech/plugin-registry` resolves at actual Vite
bundle-time for the demonstrator (no dev server run) — `@semio-tech/framework`
and `@semio-tech/framework-renderer-react` are already imported by this same
file with no explicit `dependencies` entry in its `package.json`, so
whatever workspace-resolution mechanism makes those work should apply
identically to the new import. Flagging as a residual assumption, not
independently confirmed at runtime.
