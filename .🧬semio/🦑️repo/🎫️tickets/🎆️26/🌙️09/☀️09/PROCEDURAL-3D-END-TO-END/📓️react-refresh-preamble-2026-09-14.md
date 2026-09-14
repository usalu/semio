# React refresh preamble blank boot (2026-09-14)

## Symptom

Chrome on procedural 3d react shows an empty page. Console:

```
[vite] connected.
Uncaught Error: @vitejs/plugin-react can't detect preamble. Something is wrong.
    at react-three-fiber.esm.js:…
```

## Root cause

`semioHostHtmlVitePlugin` replaces the entire `index.html` on every request (`transformIndexHtml` `order: "pre"`). `@vitejs/plugin-react` injects the fast-refresh preamble in a later pass; when that ordering fails or the browser serves a document without `injectIntoGlobalHook`, any JSX module (notably `@react-three/fiber` via `optimizeDeps`) still gets `$RefreshReg$` wrappers and throws before React mounts.

With `SEMIO_VITE_HMR=0`, the preamble is intentionally omitted, but Vite 7 / Rolldown can still leave OXC `jsx.refresh` enabled for `command: "serve"`, producing the same throw without `window.$RefreshReg$`.

## Fix

`semioPlaygroundReactRefreshCoherenceVitePlugin` in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts`, mounted from the os-dev Vite config for react renderers:

1. **`order: "post"` `transformIndexHtml`** — injects the react-refresh preamble when `server.hmr !== false` and the HTML does not already contain `injectIntoGlobalHook`.
2. **`enforce: "post"` `config`** — when `server.hmr === false`, sets `esbuild.jsxDev: false`, `oxc.jsx.refresh: false`, and `optimizeDeps.esbuildOptions.jsxDev: false`.

Vitest: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts` (`semioPlaygroundReactRefreshCoherenceVitePlugin`).

## Verify

Restart the generation3d react dev server (launch `🛠️dev🔧️procedural🏙️3d⚛️react` or ticket serve script), hard-refresh Chrome, open `http://127.0.0.1:6018/?plugin=generation3d`. Shell should render; console must not show the preamble error.
