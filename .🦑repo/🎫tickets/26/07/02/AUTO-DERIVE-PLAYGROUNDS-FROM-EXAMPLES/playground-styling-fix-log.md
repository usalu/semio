# Playground Dev Styling Fix Log

## Root cause

Repo-wide `js/` subfolder migration broke playground dev:

1. **`globals.css` not loaded** — `dev/js/index.ts` imported `./globals.css` but file lives at `dev/globals.css`
2. **Wrong HTML entry** — `index.html` pointed at `./index.ts` instead of `./js/index.ts`
3. **Stale Vite aliases** — `playgroundRendererResolveAliases` still pointed at `*/index.tsx` instead of `*/js/index.tsx`
4. **CSS subpath resolution** — aliasing `@semio-tech/ui-react` to a file broke `@semio-tech/ui-react/globals.css`
5. **Broken relative imports** — packages moved under `js/` but sibling paths (`example/`, `pkg/`, `worker-client.ts`) were not updated

## Fixes applied

- `framework/product/playground/dev/index.html` → `./js/index.ts`
- `framework/product/playground/dev/js/index.ts` → `import "../globals.css"`
- `framework/product/playground/dev/globals.css` → `@import "@semio-tech/ui-react/globals.css"`
- `framework/product/playground/dev/js/vite.config.ts` — `playDir` = parent `dev/`, correct `repoRoot`
- `framework/product/playground/dev/vite.config.ts` — re-export shim for `script.ts`
- `ui/styling/vite-elements-assets.ts` — `packageEntry`/`reactEntry`, renderer at `js/index.tsx`, CSS/internal subpath aliases
- `ui/react/package.json` — fix `globals.css` export path
- `ui/react/globals.css` — `@semio-tech/ui-styling` import
- `ui/asset/js/index.ts` — fix icon import paths
- `flow/core/js/index.ts` — fix `worker-client.ts` path
- `writer/core/js/index.ts`, `note/core/js/index.ts` — fix `example/` glob paths
- `mathematical/graph/manifest/package.json` — fix exports entry

## Action required

**Restart the playground dev server** (old processes on 6016/5174 still serve broken bundles).

Use launch.json or:
`bun nx run @semio-tech/framework-playground-dev:dev -- --app flow`
