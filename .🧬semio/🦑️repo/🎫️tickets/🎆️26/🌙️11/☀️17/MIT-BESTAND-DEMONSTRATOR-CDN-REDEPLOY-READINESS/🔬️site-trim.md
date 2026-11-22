# Site Trim

Sizes are computed from the existing output `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out` by applying the new selection rules to it. The full release build was not run (memory); `MEM=4096 bash native-wasm.sh site` still has to confirm the after numbers.

## Changes

| Source | Change |
|---|---|
| `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts` | `semioAssetsVitePlugin` replaced by `semioReferencedAssetsVitePlugin`; new `semioKatexWoff2OnlyVitePlugin` |
| `🗃️assets/🟦️.ts` | `semioReferencedAssetsVitePlugin` split into a serve plugin and an `apply: "build"` plugin (its `closeBundle` also ran on `serve`) |
| `🏗️builder/🌐️vite/🟦️.ts` | `staticDirVitePlugin` copy filters with `semioAssetShipsToBrowserV1` (no `AGENTS.md`, `README.md`, sources, tests) |
| `🔡️woff2-only/🟦️.ts` (new) | strips the `woff` and `ttf` fallbacks from `katex/dist/katex.min.css` through `load`, so Vite never emits them |
| `📦️packages/🟦️typescript/🏗️builder/🌐️vite/🟦️.ts` | exports the two plugins |
| `🧪️tests/🧩️suite/📦️asset-delivery/🟦️.ts` | 3 new tests ("trimmed site delivery"); 5 asset-delivery tests pass |

## Evidence for the `🖼️assets` trim (item 3)

Every file of the output outside `🖼️assets/` was scanned (js, css, html, json, `.semio` descriptors, all 37 plugin and extension wasm, the app wasm), for the raw directory name, its percent-encoded form, `assetPathFromRequest`, `assetTransportUrl` and `directoryName:"🖼️assets"`.

- CSS `assets/🌐️-VPVqfxkN.css` is the only runtime requester: 42 cursor `url(/🖼️assets/👆️cursor/...)` and 27 `woff2` font urls, 45 distinct files (1.12 MB).
- JS mentions `🖼️assets` only inside repo paths (`🧰️framework/🔨️modules/🖼️assets/...`: mesh catalog `source` and `root` fields, the font and icon-pack directories). Mesh requests go `/mesh/<public id>` and resolve to `/mesh/<catalog path>` (`fB` in the bundle); the `source` is never fetched.
- No wasm holds `/🖼️assets/` or any encoded form, and no bundle contains the resolver (`assetPathFromRequest`, `assetTransportUrl`), so no runtime path can be built for the route. Wasm and descriptors that mention `🖼️assets` do so for source paths (`📚️examples/🖼️assets`, `🖼️canvas/🖼️assets`, `🌙️dream/🗣️.dsl.semio`), served by `/cad-assets`, `renderer-modules` and the plugin modules.
- The 14 GIF, 3dm, png and mp4 hits in the GIS descriptor are format ids such as `s.stdio.gif`, not files.
- The manifest `.semio-artifact.json` is written after the build from the files present; it is not an input.
- The plugin keeps the reference rule of the existing `semioReferencedAssetsVitePlugin`: a route opening a token after a quote, bracket, `=` or whitespace, so repo paths never count. A referenced file missing in the source tree throws.

Excluded as unreachable: everything not in the 45 (images 98 MB, metabolism 3dm and glb 43 MB, logos, list, badge, icons, other fonts, `LICENSE.md`).

## Before and after

| Item | Before | After |
|---|---|---|
| `🖼️assets` | 174.1 MB, 1,380 files | 1.12 MB, 45 files |
| `mesh` (canonical, catalog-owned) | 22.95 MB, 93 files | unchanged |
| `🖼️assets` twin glbs (94) | in output | gone (item 1: `/mesh` is the only location, no reference repointed because none pointed at the twins) |
| KaTeX fonts in `assets/` | 1,072,948 B, 59 files | 256,168 B, 20 woff2 files (-0.82 MB) |
| `cad-assets/AGENTS.md` | shipped | not shipped |
| Output total | 2,793,119,247 B, 3,479 files | about 2,619 MB, 3,364 files (-173.8 MB, -6.2 percent) |

The wasm (2.33 GB) is untouched; compression at the CDN is the remaining lever.

## Not done (item 4)

The 50 byte-identical 753.6 KB glbs (36.9 MB) are 50 separate catalog entries, each with its own public id, source and path. `parseMeshDeliveryCatalog` rejects a duplicate `path` or `source` and its docstring says "without aliases", so sharing one transport path would need an alias concept in the catalog schema. After item 1 the twin half (24 plus 1) is gone; the remaining 25 stay as distinct files.
