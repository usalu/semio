# Fix Demonstrator End to End Boot Hang — Summary

> Historical boot-chrome observation only. This is not end-to-end completion: visible authored content, panels, interactions, and console cleanliness for all six apps remain pending in `📓️browser-proof-2026-08-27.md`.

## Verdict
Demonstrator panes leave the loading state and show live app chrome (e.g. cad editor windows). Confirmed via Playwright against `http://127.0.0.1:6029/` (`🧪️playwright-boot-4.txt`): `bootFailed=0`, `workers≥9` including `cad`/`gis`/`puzzle`/`process`/`sourcing`, `canvas=4`, German UI body with “Entwerfen mit Bestand · cad”.

## Root causes fixed
1. **`expandPluginRegistry` omitted transitive `dependsOn`** — non-host boots (all demonstrator panes) only kept primary + consume-matched extensions, so cad/flow/stdio/… were “not installed” and dropped from load order.
2. **Demonstrator Vite missing `semioPluginHotSwapVitePlugin`** — `/plugin-modules/watch` 404’d; PluginSource never snapped staged wasm, so dependency crates never installed.
3. **Session established only on primary plugin** — pane `appId`s live on dependency crates (`s.cad.cad@1/*#editor`, etc.); primary `demonstrator` manifest does not list them → boot failed.
4. **Backbone worker decoded every `postMessage`** — React DevTools noise crashed `parseBackboneWorkerWire`.
5. **Duplicate Canvas corner re-exports** — Vite transform 500 blocked the whole page (collateral from corner-window work).
6. **Puzzle 2d wasm missing engine imports** — restored so plugin rebuilds can succeed.

## Files
- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` — dependency closure + unit test
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts` — ignore non-wire messages
- `♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts` — hot-swap plugin + transitive plugin-module dirs
- `ShellHost/🟦️component.tsx` — establish session on plugin that owns `appId`
- `🎨️Canvas/🟦️component.tsx` — remove duplicate re-exports
- puzzle 2d `🌉️wasm/🦀️component.rs` — restore imports

## Residual (non-blocking)
Flow-extension workers crash / `/extensions/...` fetch fails and fall back to main-thread / fail soft. Does not block pane apps.
