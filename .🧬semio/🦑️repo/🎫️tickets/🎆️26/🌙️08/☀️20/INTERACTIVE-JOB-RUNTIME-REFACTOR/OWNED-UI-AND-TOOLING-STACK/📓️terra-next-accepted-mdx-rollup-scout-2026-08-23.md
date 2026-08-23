# Terra Next Direct Dependency Scout — `@mdx-js/rollup` — 2026-08-23

## Decision

**ACCEPTED candidate: `@mdx-js/rollup`.** Expected boundary: **131 = 68 JS + 63 Rust** → **130 = 67 JS + 63 Rust**. This is a narrow Phase 10 tooling wave, not Phase 10 acceptance.

## Fresh Binding And Reachability Review

The sole root Storybook use is `.storybook/main.ts`: it removes the pre-existing `@mdx-js/rollup`/`storybook:mdx-plugin` entries from Vite plugins, then dynamically imports `@mdx-js/rollup` and pushes `mdx.default({})`. The options are now empty after all four retired plugins. Removing this adapter must remove both the dynamic import and empty `config.plugins.push`, while retaining the existing removal loop; that preserves the deliberate removal of Storybook's internal MDX plugin and does not reorder any surviving plugin.

Installed `@mdx-js/rollup/lib/index.js` creates a VFile and calls its processor only if `formatAwareProcessors.extnames` contains the input extension. Installed `@storybook/addon-docs` registers `storybook:mdx-plugin` with `/\.mdx$/`; Storybook core makes Autodocs entries by retaining the CSF input `importPath`, while its separate `extractDocs` route reads actual MDX. The current permanent guard/index provides the live proof: `0` owned MDX files and Markdown/MDX module edges; `231 = 170 stories + 61 docs`, 61 unique `.tsx` inputs and 61 Autodocs, zero MDX; reliable SHA-256 `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`.

Thus generated docs do not reach either removed internal MDX plugin or the empty root adapter. No replacement, fallback, compatibility shim, externalization, or @mdx-js subdependency removal is allowed.

## Owners And Lock Boundary

The two in-scope direct owners are root `package.json:171` and UI React `package.json:51`, each at `^3.1.1`. They have matching Bun workspace tuples at lines 12 and 525. Compose `client/lib/sketchpad/js` (line 23 / lock 126), `client/ui/vscode` (line 54 / lock 238), and `dev/algorithm` (line 28 / lock 270) are the three excluded direct owners. All three Compose tuples and the one `@mdx-js/rollup@3.1.1` resolution at lock line 1361 must remain after only the two root/UI tuples are removed. That resolution retains `@mdx-js/mdx`, `@rollup/pluginutils`, `source-map`, and `vfile`; none are in this wave.

## Executor Packet And Gates

1. Record the pre-edit zero-input scan, complete uncached UI Storybook index/hash, and current Vite plugin ordering.
2. Remove only `.storybook/main.ts`'s dynamic import and `config.plugins.push(mdx.default({}))`, then root/UI direct manifest rows. Keep the loop that removes `@mdx-js/rollup` and `storybook:mdx-plugin`, as well as every remaining Vite plugin, in its existing order.
3. Run `bun install`; retain the three Compose rows/tuples and one shared resolution. Do not touch Storybook/addon-docs, @mdx-js subdependencies, Dagre, guard, or P3/P8/Cargo paths.
4. Require post-edit byte-identical index/hash; UI build, quick 724, lint, typecheck, frozen install, ratchet/list/parity (`67 JS`, `63 Rust`), exact source/manifest absence, Compose retention, syntax, honest Prettier baseline, and scoped/whole working/staged/HEAD diff checks.

## Independent Audit Gates

```text
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun x nx run @semio-tech/ui-react:lint --skip-nx-cache
bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache
bun install
bun install --frozen-lockfile
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js
find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print | wc -l
rg --hidden -n -P "\b(?:from\s+|import\s*\()['\"][^'\"]+\.mdx?['\"]" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg --hidden -n -P "\brequire\s*\(['\"][^'\"]+\.mdx?['\"]\)" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg -n -F 'await import("@mdx-js/rollup")' .storybook/main.ts
rg -n -F 'mdx.default({})' .storybook/main.ts
rg -n -F '@mdx-js/rollup' package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
rg -n -F '@mdx-js/rollup' compose/client/lib/sketchpad/js/package.json compose/client/ui/vscode/package.json compose/dev/algorithm/package.json bun.lock
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check .storybook/main.ts package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json' 📜️script.ts
git diff --check
git diff --cached --check
git diff HEAD --check
```

Acceptance requires no dynamic root adapter import/push or root/UI manifest declaration; retention of the plugin-removal loop is intentional. It also requires exactly three Compose declarations/workspace tuples plus the shared `3.1.1` resolution, the exact permanent guard/index/hash differential, and clean scoped and whole working/staged/HEAD diff checks.

The closest competing direct tooling leaf, `@tailwindcss/typography`, is not safe: it remains actively imported by `ui/styling/tailwind.config.ts` and registered by `ui/styling/ui.css`; it is therefore a real build input rather than an empty-domain adapter. `eslint-plugin-react-hooks` has no active non-manifest binding, so it lacks the required truthful active-use proof. Storybook/addon-docs is infrastructure; Compose is excluded; Dagre remains held; runtime/tooling leaves require their own direct-use or replacement packet. No production, lock, Git, ticket-lifecycle, or Cargo input was changed by this scout.
