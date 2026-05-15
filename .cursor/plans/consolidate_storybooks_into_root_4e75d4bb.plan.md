---
name: Consolidate Storybooks Into Root
overview: Collapse the three bundle-local Storybook configs into the existing root `.storybook` workspace, grouped as `.storybook/<technology>/<bundle>/stories/...`, and remove the per-bundle `.storybook` folders.
todos:
  - id: ticket
    content: Open repo ticket for the consolidation
    status: pending
  - id: move-stories
    content: Move stories/nakagin from the 3 bundle .storybook folders into .storybook/<tech>/<bundle>/
    status: pending
  - id: consolidate-helpers
    content: Create single .storybook/withLevel.tsx, withTheme.tsx, vitest.setup.ts
    status: pending
  - id: update-main-preview
    content: Update .storybook/main.ts globs and .storybook/preview.ts decorator imports
    status: pending
  - id: fix-imports
    content: Rewrite relative imports in moved stories to use @semio/* and @elements/* aliases
    status: pending
  - id: delete-old
    content: Delete elements/client/lib/react/.storybook, semio/client/lib/react/rendering/.storybook, semio/dev/algorithms/.storybook
    status: pending
  - id: verify
    content: Run dev:storybook and build:storybook; fix breakages
    status: pending
  - id: close-ticket
    content: Close the ticket with summary and file list
    status: pending
isProject: false
---


## Target layout

```
.storybook/
  main.ts                 (update story globs)
  preview.ts              (update decorator imports)
  playwright.config.ts
  monorepo.spec.ts
  withLevel.tsx           (consolidated; uses @elements/ui)
  withTheme.tsx           (consolidated)
  vitest.setup.ts         (consolidated)
  elements/
    ui/
      nakagin.ts
      stories/*.stories.tsx
  semio/
    ui/
      stories/*.stories.tsx
    algorithms/
      stories/
        *.stories.tsx
        kit-store/*       (helpers)
```

Technologies = `elements`, `semio` (coda has no stories yet; no folder created). Bundle names taken from each bundle's `AGENTS.md` frontmatter (`bundle.name`): `elements/ui`, `semio/ui`, `semio/algorithms`.

## Steps

1. **Move stories** into new locations (preserve subfolders like `kit-store/`):
   - `elements/client/lib/react/.storybook/stories/**` → [.storybook/elements/ui/stories/](.storybook/elements/ui/stories/)
   - `elements/client/lib/react/.storybook/nakagin.ts` → [.storybook/elements/ui/nakagin.ts](.storybook/elements/ui/nakagin.ts)
   - `semio/client/lib/react/rendering/.storybook/stories/**` → [.storybook/semio/ui/stories/](.storybook/semio/ui/stories/)
   - `semio/dev/algorithms/.storybook/stories/**` → [.storybook/semio/algorithms/stories/](.storybook/semio/algorithms/stories/)

2. **Consolidate helpers** (the three copies are near-identical) into root `.storybook/`:
   - [.storybook/withLevel.tsx](.storybook/withLevel.tsx) — single decorator using `@elements/ui` (`Level`, `LevelProvider`, `getLevelBgClass`), context-globals based to keep elements-style toolbar working.
   - [.storybook/withTheme.tsx](.storybook/withTheme.tsx) — single theme decorator (already identical across the three).
   - [.storybook/vitest.setup.ts](.storybook/vitest.setup.ts) — moved from `elements/client/lib/react/.storybook/`.

3. **Update [.storybook/main.ts](.storybook/main.ts)** story globs to:
   ```ts
   stories: [
     "./elements/**/stories/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)",
     "./semio/**/stories/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)",
   ]
   ```
   Keep existing Vite aliases (`@elements/ui`, `@semio/ui`, `@semio/react`, `@semio/js`, `@semio/algorithms`, `@semio/assets`, `@semio/rs-wasm`) so moved stories keep working.

4. **Update [.storybook/preview.ts](.storybook/preview.ts)** to import the local consolidated decorators (`./withLevel`, `./withTheme`) instead of `../elements/client/lib/react/.storybook/...`. CSS imports stay (`../elements/client/lib/react/globals.css`, `../semio/client/lib/react/rendering/globals.css`).

5. **Rewrite relative imports in moved stories** so they don't break after the move. Replace package-internal relatives with the existing aliases:
   - In `elements/ui/stories/*`: `from "../nakagin"` → `from "../nakagin"` (still works, nakagin lives one level up at `.storybook/elements/ui/nakagin.ts`).
   - In `semio/algorithms/stories/*`: `from "../../index"` → `from "@semio/algorithms"`; `from "../../../../assets/index"` → `from "@semio/assets"`; `from "../../../../assets/fixtures/..."` → `from "@semio/assets/fixtures/..."`.
   - In `semio/ui/stories/*`: similar rewrites if any relatives reach into `semio/client/...` — replace with `@semio/ui` / `@semio/react`.

6. **Delete obsolete bundle-local Storybook folders** entirely:
   - `elements/client/lib/react/.storybook/`
   - `semio/client/lib/react/rendering/.storybook/`
   - `semio/dev/algorithms/.storybook/`

7. **Verify** by running `bun run dev:storybook` and `bun run build:storybook`; fix any remaining import paths surfaced by Vite. Existing `dev.script.ts` / `build.script.ts` / `test.script.ts` already point at the root `.storybook` directory, so no script changes needed.

## Open ticket

Open repo ticket `2026/05/15/CONSOLIDATE-STORYBOOKS-INTO-MONOREPO-ROOT` (title "Consolidate Storybooks Into Monorepo Root") associated with the most appropriate goal from `repo://goals`, do the work inside it, and close it on completion with the file list.
