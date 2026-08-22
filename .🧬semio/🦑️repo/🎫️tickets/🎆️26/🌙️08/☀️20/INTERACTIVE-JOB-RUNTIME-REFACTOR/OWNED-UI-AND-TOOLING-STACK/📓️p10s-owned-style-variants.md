# Phase 10 Owned Style Variants

<!-- #region Outcome -->

## Outcome

The UI styling API now owns its finite variant schema, compiler, and prop extraction type. `class-variance-authority` is absent from the UI source, package manifest, and workspace lockfile. The public UI barrel exposes `styleVariants` and the owned schema types instead of re-exporting the third-party implementation.

The migration also fixes the button-group selection path: `ButtonGroupItem` consumes `variant`, passes it to its compiler, and no longer forwards the styling-only field to the host button.

<!-- #endregion Outcome -->

<!-- #region Semantics -->

## Semantics

The owned compiler preserves the finite behavior used by the current controls:

- stable base then schema-order variant composition;
- declared default selections when a value is omitted or `undefined`;
- explicit `null` suppression of a default;
- string and boolean variant keys;
- compound conjunctions, including selector arrays;
- stable compound declaration order;
- caller `class` and `className` composition through the owned `cn` gateway;
- `StyleVariantProps` extraction without exposing implementation types.

<!-- #endregion Semantics -->

<!-- #region Validation -->

## Validation

- Focused Vitest, one worker: **14/14 passed**, one file, 417 ms on the post-migration repeat.
- Targeted TypeScript for the compiler and its matrix: **passed** with strict mode and no emit.
- UI-scoped source, manifest, and lockfile census: **zero** `class-variance-authority`, `cva`, or third-party `VariantProps` references.
- `bun install --ignore-scripts`: **passed**, 2,010 installs checked across 2,062 packages.
- UI typecheck: **passed** through the exact Nx target without cache.
- UI quick: **557/557 passed** across three files through the exact Nx target without cache.
- UI lint: **passed** through the exact Nx target without cache.
- Dependency freeze: **passed**, 162 current identities from the 238 baseline, 76 removed; `class-variance-authority` is explicitly listed among the removals.
- JavaScript manifest/source parity: **passed**, 83 manifests, 285 external rows, 136 evidenced, 149 unowned, zero undeclared imports.
- UI primitive policy: **not green** on 13 unrelated live-tree findings outside this packet's intentional files: 12 raw primitives in Compose, Sketchpad, CAD, and Hub Admin plus one stale CAD allowlist row. The owned style-variant module and all four migrated controls produced no finding.

<!-- #endregion Validation -->

<!-- #region Files -->

## Files

- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️style-variants/🟦️component.ts`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️style-variants/🧪️component.test.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ButtonGroup/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `bun.lock`

<!-- #endregion Files -->
