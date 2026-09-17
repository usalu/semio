# Demonstrator generator — ship vs dev plugin boot

## Symptom (deployed build)

- Flow graph: truncated nodes, `! …` port labels, missing wires.
- Preview: **Geometrie-Erweiterung nicht verfügbar** (`flow.extension-not-contributed` / geometry extension unavailable).
- Dev server (`bun nx run @semio-tech/mit-bestand-demonstrator:dev`) works.

## Root cause

`FrameworkOsShell` loads the **primary** plugin directly, but every other plugin and **all flow extensions** depend on a `PluginSource` subscription fed by **SSE** at:

- `/🔌️plugin-modules/watch`
- `/🧩️extension-modules/watch`

Those endpoints exist only under the Vite dev middleware (`semioPluginHotSwapVitePlugin`, `semioExtensionStoreVitePlugin`). A static `dist/site` bundle has the wasm artifacts under `/🔌️plugin-modules` and `/🧩️extension-modules`, but **no watch stream**, so only `procedural` (generator primary) loads. Flow extensions (`flow-extension-brep`, `flow-extension-math`, …) never install → empty `kind_infos`, broken graph, no brep preview.

## Fix

1. **`createBundledPluginSource(registry)`** (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`): on `subscribe`, replay one immediate `snapshot` for the full expanded registry (same event shape as dev SSE connect).
2. **`ShellHost`**: use bundled source when `import.meta.env.PROD`, keep dev multiplexed SSE sources otherwise.
3. **Demonstrator Vite**: `base: "./"` so asset and module URLs stay relative on static hosts; add `public/.nojekyll` for GitHub Pages.

## Redeploy checklist

1. `bun nx run @semio-tech/mit-bestand-demonstrator:build`
2. Publish entire `♻️mit-bestand/🧺️demonstrator/dist/site` (must include `🔌️plugin-modules/` and `🧩️extension-modules/` trees, not only `assets/`).
3. Confirm `GET …/🔌️plugin-modules/🌀️procedural/🌉️bridge.js` returns JS (not SPA 404 HTML).

## Note on live host (2026-09-17)

`demonstrator.entwerfen.mit-bestand.de` still served an **old** Aggregator build (July 2026) with no plugin-module tree — separate from this code fix; redeploy required.
