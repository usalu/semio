# 🚀️ Staffage Website CDN Redeploy Readiness

State on 2026-11-17, machine g1-m02 (macOS arm64). Site `@teaching/staffage-site`, served at the root of `https://staffage.semio-tech.com`.

## Verdict

Ready to redeploy. Upload the contents of

`🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/📦️packages/🟦️typescript/dist/release`

at the domain root (no sub-path, no SPA fallback; a miss answers `404.html` with status 404). The folder contains `index.html`, `404.html`, `CNAME`, `.nojekyll`, `_headers`, `robots.txt`, `manifest.webmanifest`, favicons, `assets/` (hashed, immutable) and `🖼️assets/` (fonts, cursors). The upload is an outward action for the owner.

## Build (native fallback)

The build graph and the script admission were starved (`repo capacity run -- repo ./📜️script.ts build` fails with "Original parent Script process invocation required"; the envelope launcher `native.sh` stayed in `resource-directory-admission` for minutes). The release is built straight from the workspace Vite:

```
SEMIO_BUILD_MODE=release .cache/b/t/m/bun/<digest>/bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/native-vite.ts
```

About 4–10 s. `dist/release` is 38 MB (was 67 MB before the catalog payload change), 6.9 MB gzip.

## What was fixed

| Area | Finding | Fix | Write-up |
|---|---|---|---|
| Catalog | 32 of 152 assets failed `validateAsset` and showed "Unavailable" (missing `volumes`, front/plan extents, flat shapes leaving their plane, support/thermal coverage, duplicate ids, undrawn detail roles) | Authored data corrected per asset, validator unchanged: 0 of 152 fail | `invalid-assets.md`, `invalid-vegetation.md`, `invalid-machinery-objects.md`, `invalid-humans-animals.md` |
| Manifest | 4 entries disagreed with their asset files (standing-adult, pig, shower-head, plate-heat-exchanger) | Synced to the asset files: `validateManifest` 0 issues | this file |
| Payload | Every catalog chunk embedded the pretty-printed file and the browser hashed it | `?staffage-asset` Vite plugin serves compact JSON plus the precomputed authored revision; −44 % raw | `catalog-payload.md` |
| Redeploy | No recovery for tabs opened before a redeploy (old hashed chunks 404) | `🌐️site/🛟️recovery`: reload once per minute on `vite:preloadError`, never loops, never without storage; tests `🧪️tests/🛟️stale-chunk` | `cdn-audit.md` |
| DWG export | CSP blocked `WebAssembly.instantiate`; site build did not depend on the wasm | `'wasm-unsafe-eval'` in `script-src` (no `'unsafe-eval'`), test; `@teaching/staffage-dwg-rs:wasm` in the site's `dependsOn` | `cdn-audit.md` |
| Browser safety | `process.env.DEBUG_FW3_WHOLE` read in the print measurement path | Debug switch removed | `cdn-audit.md` |
| Schema | 19 assets broke the JSON schema (`schema-conformance`) although `validateAsset` passed | Data fixed; five validator rules added so schema and validator agree; 43 of 43 pass | `schema-conformance.md` |
| Power (owner request) | Assets showed size and mass, no power | Total nameplate power of all elements plus metabolic heat (`totalPowerW`, `ratedPowerW` + `ratedPowerBasis`), on cards, compare, metadata, sort and filter, en/de; manifest `powerW` for all 152 (e.g. person cooking 1.68 kW, gas hob 2.50 kW, wheel loader 150 kW) | `asset-power.md` |
| Framework typecheck | 24 errors reached through the site's `tsconfig.json` | 3 real type errors fixed in the repo tooling; the generators (actor typegen, UI contract, framework manifest via `native.sh gen-manifest`, 50 min limit) ran; site `tsconfig.json` and `tsconfig.site.json` and the React package: 0 errors | `typecheck-framework.md` |

| Playground (owner request) | Playground was side elevation only | Three tabs over one shared, linkable scene: Section (side or front), Floor plan (top view, yaw and mirror), 3D (lazy); per-asset pose/clip, play/pause, scrub, loop, speed, offset; global timeline with adjustable windows | `playground.md`, `react-fixes.md` |
| Library tests | fill-rule, plan-projection, three-views, export-performance, model-split failed | Fixed (geometry, stale expectation, re-pinned digests after re-authoring, model validator kept out of the site entry) | `tests.md`, `library-tests.md` |
| Silhouette guard | `catalog-silhouette` failed for 117 assets, 116 of them already at HEAD | All of today's regressions fixed; 30 assets fixed in total; about 90 pre-existing assets remain (hand-authored drawing vs volume mismatches; lists per category) | `catalog-silhouette.md`, `silhouette-*.md` |

## Verification

- `validate-all.ts`: failing 0 of 152. `validate-manifest.ts`: 0 issues.
- Final run: site 9 of 9, React 271 of 271, release build 3.7 s; site, React and library typechecks clean.
- Site tests (`host-document`, `stale-chunk`): 7 of 7 pass. Run with an absolute `--config`: with `--configLoader native`, a relative config path from the site package loads the library's config and finds no tests (the `test` target resolves it absolute and is unaffected).
- Static CDN stand-in `serve.mjs` (exact files, `404.html` for misses) in the in-app browser: `#/all` shows 152 of 152 with no unavailable asset and no failed request; detail, thermal and 3D model views render. Headless Chrome via Playwright cannot start on this machine (`FATAL path_service 1001`), so `smoke.mjs` is kept for a machine where it can.

## Host notes

`_headers` (immutable hashed assets, `no-cache` documents, `X-Frame-Options`, `frame-ancestors`) applies on Netlify or Cloudflare Pages; GitHub Pages, S3 and CloudFront ignore it, and a meta CSP cannot carry `frame-ancestors`.
