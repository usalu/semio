# 🌐️ Static Hosting Audit

The release bundle must be served at the root of its host, the `CNAME` domain the build writes. A sub-path deploy breaks.

| Finding | Where | Effect on a root deploy |
|---|---|---|
| Plugin and extension routes are root-absolute (`/🔌️plugin-modules`, `/🧩️extension-modules`); `publishedPageUrl` rewrites only when `SEMIO_PLAY_PAGE_ORIGINS` is set | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🛣️routes.json` | none; breaks under a sub-path |
| Brand logos use `/${DEMONSTRATOR_ASSETS_DIR}/…` | `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts` | none; breaks under a sub-path |
| Map tiles `/osm/{z}/{x}/{y}.png`, `/vt/{z}/{x}/{y}.pbf`; only Switzerland z0–10 is bundled | tiled map scene, `🔨️modules/📦️site/🗺️map-tiles` | tiles outside the bundled set 404 |
| Extension install and remove call `/🧩️extension-modules/install`, answered only by the serve-mode Vite plugin | plugin lifecycle | those two UI actions 404 |
| `/semio-backbone?uri=` exists only as dev middleware | backbone envelope IO | remote backbone reads 404 and retry for 15 s, if reached |
| `/_semio/dev/local-session`, hub service worker | identity state, host ports | inert: gated on `VITE_S_HUB_URL`, which is unset |

The host needs no COOP/COEP headers, because SharedArrayBuffer is optional. It needs no SPA fallback either: there is one page, and a missing module JSON must answer 404, not `index.html`.
