# Phase 10ab — Owned MDX and Single-Child Slot Boundary

## Scope

Packets A and E from `📓️p10aa-next-zero-dependency-wave.md` are implemented without touching `compose/`, Cargo, or git state.

## Implementation

- Removed the unused `MDXProvider` facade from the generated UI React barrel and removed `@mdx-js/react` from the UI React manifest.
- Replaced `@radix-ui/react-slot` in `ButtonGroupItem` with an owned exactly-one-child `Slot` beside the owned class-name composition module.
- The owned slot keeps child precedence for ordinary props and style fields, combines wrapper then child classes through `cn`, runs the child handler before the wrapper handler, suppresses the wrapper handler after `preventDefault`, and writes both child and wrapper refs.
- `ButtonGroupItem asChild` now validates one element, appends its owned label/hotkey/icon decorations inside that element, and gives the slot one cloneable child.
- Removed the direct Radix Slot rows from the UI manifest and root override, plus the stale generated-barrel import.
- Added focused fixtures for invalid child cardinality/type, class/style/ref merging, handler order, `defaultPrevented`, and decorated `ButtonGroupItem asChild` rendering.
- The historical `🔒️dependencies.json` ratchet baseline and shared `bun.lock` were intentionally not rewritten; neither is a live direct-manifest declaration.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🟦️slot.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️slot.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ButtonGroup/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `package.json`

## Focused Evidence

- `bunx vitest run --config <ui-react-vitest-config> <slot-test> --maxWorkers=1`: **PASS**, 1 file and 4 tests.
- Targeted `bunx prettier --check` over the new slot implementation/test, `ButtonGroup`, Vitest config, and both changed manifests: **PASS**.
- Parsed both changed manifests with Node JSON parsing: **PASS**.
- Outside `compose/`, `rg` over every `package.json` for `@mdx-js/react` or `@radix-ui/react-slot`: exit `1`, no matches.
- UI source and generated-barrel `rg` for both package names plus `MDXProvider`: exit `1`, no matches.

## Deferred Gates

The timing-sensitive full gates remain held for the P4 Cargo lane: full UI React Vitest, UI React typecheck, lint, dependency freeze/parity, Nx project gates, primitives verification, demonstrator/build coverage, and lockfile reconciliation. No Cargo command was run.
