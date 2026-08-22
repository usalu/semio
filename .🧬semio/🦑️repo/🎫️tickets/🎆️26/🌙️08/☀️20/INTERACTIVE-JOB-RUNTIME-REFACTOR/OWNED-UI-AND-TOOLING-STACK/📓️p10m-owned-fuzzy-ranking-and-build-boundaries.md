# P10m Owned Fuzzy Ranking and Build Boundaries

## Outcome

- Replaced the remaining `fuse.js` product dependency with the workspace-owned `@semio-tech/ui` fuzzy-ranking boundary.
- Migrated Shell Search and Find to deterministic owned ranking with weighted fields, Unicode NFKD/diacritic normalization, prefix/substring/subsequence matching, bounded Damerau typo scoring, stable ordering, and result limits.
- Removed `fuse.js` from the UI and renderer manifests, the lockfile, and both Vite dependency-prewarm lists.
- Hardened the OS backbone worker boundary so Window/jsdom is never mistaken for a dedicated worker merely because `self` exists.
- Made the demonstrator and OS Vite configurations preserve explicit `@semio-tech/ui-react/runtime` and `@semio-tech/ui-react/test` subpath aliases before the package-root alias, and use ES-format worker chunks.
- Removed Node TypeScript strip-mode parameter properties from the actor shard client and turn scheduler.
- Corrected the demonstrator stylesheet to import the live renderer-engine global stylesheet.

## Exact Validation

- `bun nx run @semio-tech/ui:typecheck --skip-nx-cache` — passed.
- `bun nx run @semio-tech/ui:test --args=quick --skip-nx-cache` — 536/536 passed.
- `bun nx run @semio-tech/ui:lint --skip-nx-cache` — passed.
- Renderer ShellSearch focused test — 1/1 passed.
- Renderer UIFind focused test — 1/1 passed.
- `bun nx run @semio-tech/framework-actor:test --args=quick --skip-nx-cache` — 46/46 passed.
- `bun ./📜️script.ts verify dependencies` — passed at 177 external identities, including the recorded `js:fuse.js` removal.
- `bun ./📜️script.ts verify dependencies parity js` — passed with 83 manifests, 299 external rows, 143 evidenced, 156 unowned, and 0 undeclared.
- `bun install --ignore-scripts` — passed after the manifest/lockfile update.

## Build Gate

The renderer repair packet restored the missing owned Interpreter exports and the demonstrator production build is now green. `SKIP_PLUGIN_BUILD=1 SKIP_ENGINE_BUILD=1 bun nx run @semio-tech/mit-bestand-demonstrator:build --skip-nx-cache` transformed 1,944 modules and completed the Vite production bundle in 11.52 seconds.

## Scope Notes

- The fuzzy implementation exports workspace-owned values and contracts only.
- No compatibility layer or legacy import path was added.
- Existing Vite asset-resolution, Node externalization, and CSS-selector warnings remain visible diagnostics even though the production bundle completes; they are not silently represented as clean warning gates.

## Manifest-only Tooling Follow-up

- Removed `tsx` from the assets and repo-coordinator packages; every permanent TypeScript entrypoint is already run by Bun through the existing `📜️script.ts`/Nx routes.
- Removed `@types/pixelmatch` from OS dev because Pixelmatch 7 ships its own declaration.
- `bun install --ignore-scripts` passed and updated the lockfile without package changes.
- `@semio-tech/assets:build` passed and regenerated 1,891 emoji, 249 catalog shortcodes/icons, and 29 metabolism icons.
- `@semio-tech/repo-coordinator:test-quick` passed (no test files).
- `@semio-tech/framework-os-dev:test-quick` passed 27/27.
- Replaced the direct `jsdom` and `pngjs` declaration-package boundaries with concise workspace-owned structural contracts loaded through `createRequire`, then removed `@types/jsdom` and `@types/pngjs`.
- The post-removal assets build passed and regenerated 1,891 emoji, 249 catalog shortcodes/icons, and 29 metabolism icons.
- The post-removal OS dev quick suite passed 27/27.
- The dependency freeze passed at **173** identities, down 65 from baseline; `js:tsx`, `js:@types/pixelmatch`, `js:@types/jsdom`, and `js:@types/pngjs` are recorded removals.
- The JS parity gate passed with 83 manifests, 294 external rows, 141 evidenced, 153 unowned, and 0 undeclared imports.
