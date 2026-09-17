# Deployed Websites Out Of Box

## Problem

Static hosts (GitHub Pages, generic object storage, `python -m http.server`) expect conventional entry and icon filenames. Vite builds used the constitutional `🌐️.html` rollup input only, so deploy roots had no `index.html` / `404.html` and browsers probing `/favicon.ico` got 404 unless middleware was present.

## Change

Central Vite plugins in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`:

- `semioEmojiIndexHtmlVitePlugin` — on `closeBundle`, copies the built emoji entry to `index.html` and `404.html` (GitHub Pages SPA fallback).
- `faviconVitePlugins` — serves and copies `favicon.svg` / `favicon.ico` aliases alongside `🛡️favicon.svg` / `🔖️favicon.ico`.

Hub admin Vite config now wires `semioFaviconVitePlugin` and declares favicon `<link>` tags in its `🌐️.html`.

## Verification

See [📓️verification.md](./📓️verification.md) for per-site build evidence.

- UI styling tests: favicon delivery, declared HTML entry, build output write authority.
- `runViteBuild` now invokes `vite.js` with `--configLoader bundle` (aligned with `buildViteArtifact`).
- Projektetage: repaired corrupted Vite config, added `🌐️.html` + `🟦️.ts` entry, fixed `🎨️globals.css` UI import path.

## Consumers using the shared plugin (inherit deploy aliases automatically)

- `♻️mit-bestand/🧺️demonstrator`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage`
- `🌎️hub/🔨️modules/🛡️admin`
- `🧰️framework/🛍️products/💻️os` dev shell (local)
