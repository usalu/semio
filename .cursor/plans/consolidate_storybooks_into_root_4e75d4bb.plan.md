---
name: Consolidate Storybooks Into Root
overview: One root Storybook under `.storybook/` with all duplicate config and helpers merged; all story modules live under a single top-level `.storybook/story/<technology>/<bundle>/` tree (not merged into fewer story files).
todos:
  - id: ticket
    content: Open repo ticket for the consolidation
    status: completed
  - id: move-stories
    content: Move *.stories.* into .storybook/story/<tech>/<bundle>/ (single top-level stories tree); non-story helpers to fixtures/ and compose/algorithm/kit-store/
    status: completed
  - id: consolidate-helpers
    content: Single root withLevel.tsx (globals.level + optional args.level), withTheme.tsx, vitest.setup.ts; delete all bundle main/preview/with*
    status: completed
  - id: update-main-preview
    content: Update .storybook/main.ts globs and .storybook/preview.ts decorator imports
    status: completed
  - id: fix-imports
    content: Rewrite relative imports in moved stories to use @compose/* and @elements/* aliases
    status: completed
  - id: delete-old
    content: Delete elements/client/lib/react/.storybook, compose/client/lib/react/rendering/.storybook, compose/dev/algorithm/.storybook
    status: completed
  - id: verify
    content: Run dev:storybook and build:storybook; fix breakages
    status: completed
  - id: close-ticket
    content: Close the ticket with summary and file list
    status: completed
isProject: false
---

## Consolidation principle

- **Consolidate (single copy at repo root `.storybook/`):** `main.ts`, `preview.ts`, `withLevel.tsx`, `withTheme.tsx`, `vitest.setup.ts`, `playwright.config.ts`, `monorepo.spec.ts`, shared story fixtures (e.g. `nakagin.ts`), and algorithm KitStore shell components that are not story modules.
- **Do not consolidate:** each `*.stories.*` file remains its own module (many files); only relocate them. No merging multiple stories into one file.

Technologies = `elements`, `compose` (coda has no stories; skip). Bundle folder names from `AGENTS.md` (`bundle.name`): `ui`, `algorithms` under each technology.

## Target layout

```
.storybook/
  main.ts
  preview.ts              (union: theme + level toolbars like current root; both globals.css)
  playwright.config.ts
  monorepo.spec.ts
  withLevel.tsx           (one file: globals.level for elements + args.level branch for compose stories)
  withTheme.tsx
  vitest.setup.ts
  fixtures/
    nakagin.ts            (was elements-only; imported by elements stories)
  stories/                (one general tree for all story modules)
    elements/
      ui/
        *.stories.tsx
    compose/
      ui/
        *.stories.tsx
      algorithms/
        *.stories.tsx
  compose/
    algorithms/
      kit-store/*         (TSX/helpers for KitStore story; not *.stories.*)
```

Rationale: **No per-bundle `stories/` folder** under `elements/` or `compose/` inside `.storybook`. Instead, **one** top-level `.storybook/story/` groups by technology then bundle. Non-story support code stays outside that tree (`fixtures/`, `compose/algorithm/kit-store/`).

## Steps

1. **Move story modules only** (glob: `**/*.stories.@(ts|tsx|mdx|...)`):
   - `elements/client/lib/react/.storybook/story/**` → `.storybook/story/elements/ui/`
   - `compose/client/lib/react/rendering/.storybook/story/**` → `.storybook/story/compose/ui/`
   - `compose/dev/algorithm/.storybook/story/**/*.stories.*` → `.storybook/story/compose/algorithm/`

2. **Move consolidatable non-story files:**
   - `elements/.../.storybook/nakagin.ts` → `.storybook/fixture/nakagin.ts`; update element story imports (e.g. `from "../../../fixture/nakagin"` from `.storybook/story/elements/ui/`).
   - `compose/dev/algorithm/.storybook/story/kit-store/**` → `.storybook/compose/algorithm/kit-store/**`; update `KitStore.stories.tsx` and any other imports to the new path.

3. **Single root helpers** (delete per-bundle duplicates):
   - **withLevel:** merge behaviors: when `context.args.level` is set (compose ui / algorithms pattern), wrap like current compose `LevelWrapper`; otherwise apply `context.globals.level` like elements (toolbar stays in unified `preview.ts`).
   - **withTheme:** one shared implementation (prefer the slightly richer elements/system branch unless a regression appears in smoke).
   - **vitest.setup:** one file at `.storybook/vitest.setup.ts` importing root `./preview`; remove `elements/.../vitest.setup.ts` after references updated.

4. **Unified preview:** one [.storybook/preview.ts](.storybook/preview.ts) with **both** `theme` and `level` `globalTypes` + `initialGlobals` (today’s root already does this; bundle compose previews omitted level—root wins for consistency). Import both `globals.css` from elements and compose rendering packages.

5. **Update [.storybook/main.ts](.storybook/main.ts)** `stories` glob to a single tree, e.g. `./story/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)` (or equivalent). Keep existing Vite aliases in `viteFinal`.

6. **Import fixes in moved stories:** replace broken relatives with `@semio-tech/compose-algorithm`, `@semio-tech/compose-asset`, `@compose/ui`, `@semio-tech/compose-react`, `@elements/ui`, and stable relatives from `.storybook/story/...` (e.g. `../../../fixture/nakagin`, `../../../compose/algorithm/kit-store/...`) or optional Vite aliases if you add them in `main.ts`.

7. **Delete** entire `elements/client/lib/react/.storybook/`, `compose/client/lib/react/rendering/.storybook/`, `compose/dev/algorithm/.storybook/` after nothing references them.

8. **Verify:** `bun run dev:storybook`, `bun run build:storybook`; grep repo for stale `.storybook` paths under `elements/` or `compose/client|dev/`.

## Open ticket

Open repo ticket `2026/05/15/CONSOLIDATE-STORYBOOKS-INTO-MONOREPO-ROOT` (title "Consolidate Storybooks Into Monorepo Root") associated with the most appropriate goal from `repo://goals`, do the work inside it, and close it on completion with the file list.
