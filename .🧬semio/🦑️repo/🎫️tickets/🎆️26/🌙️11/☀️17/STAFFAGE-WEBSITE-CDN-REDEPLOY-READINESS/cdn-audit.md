# Staffage website CDN redeploy audit

Scope: static CDN deploy of `https://staffage.semio-tech.com` with the strict CSP from `🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts`.
Method: read-only static analysis of the browser graph (entry `🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🟦️.ts`, then `mountArchive` in `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/📦️packages/🟦️typescript/📚️app/🟦️.tsx`), plus an import-graph trace (223 files reachable from the entry, no build run, no files written). No `vite build` and no browser run were performed, so items marked "verify" are unconfirmed.

All paths are relative to the repository root `/Users/ueli/Documents/semio`. Vite is 7.3.7 (`node_modules/vite`).

## Ranked blockers

### 1. Stale deploys cannot recover from failed chunk loads (high)

No `vite:preloadError` handler exists anywhere in the staffage packages or `🧰️framework/🔨️modules/🖱️ui` (grep: no hits). Every lazy chunk is an ordinary dynamic `import()` with a retry that re-imports the same URL, and that retry is defeated by caching:

- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧊️spatial/🚪️gate/🟦️.tsx:12` declares `lazy(() => import(...))` at module scope. React caches the rejected lazy result, so the retry button at `:20-28` re-renders the same rejected component and fails again.
- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts:62` does `axonometricLoading ??= import(...)`. A rejected promise is kept forever, so `loadAxonometric` never retries.
- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/📚️catalog/🟦️.ts:25,30,142-145`: every category is a hashed chunk from `import.meta.glob`, and `retryCategory` calls the same `documents[key]()` again. Browsers cache a failed module fetch per document, so this also fails until reload.

Why it matters on a CDN: a tab opened before a redeploy still references old hashed chunk names, and those files are gone after the deploy. `index.html` is `no-cache` (`🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts:33-35`), so new visitors are fine, but open tabs are not.

Fix: register `vite:preloadError` in the entry (`🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🟦️.ts`) that calls `event.preventDefault()` and reloads once (guard the reload with a session flag to avoid loops), and show a bilingual "new version, reload" notice (EN first, then DE) when the failure happens without a reload available.

### 2. Unguarded `process.env` read in the browser print path (high, verify)

`🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts:105` reads `process.env.DEBUG_FW3_WHOLE` inside `measureDrawn` (`:61`). `measureDrawn` is reached from `measureSuitability` (`:55`) and `resolveRepresentation` (`:171-175`), and the depiction and catalog code calls those.

Nothing defines `process` for the browser:
- the staffage Vite `define` contains only `import.meta.vitest` (`🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts:148`);
- Vite only replaces `process.env.NODE_ENV`;
- the framework build factory (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts`) adds no define or shim.

Every print profile has `minFeatureMm > 0` (`🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🖨️print-profiles/🔣️.json:15,29,43,57`), so the branch runs on the measured path. Expected result: `ReferenceError: process is not defined` when a drawing is measured against a print profile. This would also fail under `vite dev`, so either the path is not exercised today or the error is swallowed. Verify in the browser console on a planar figure with a print profile.

Fix: delete the debug switch. Product code must not read environment variables.

### 3. DWG export cannot run on the CDN (medium)

- The WASM module is git-ignored (`.gitignore:105`, `**/*.wasm`), and no plugin in the site list copies it (`🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts:125-143`; `publicDir: false` at `:123`).
- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖊️dwg/🟦️.ts:23` uses `new URL("./staffage_dwg.wasm", import.meta.url)`. When the file is missing at build time, Vite leaves the URL unchanged with a warning (`node_modules/vite/dist/node/chunks/config.js` around line 27985), so the browser requests a file that was never emitted and gets a 404.
- The CSP `script-src` (`🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts:45`) lacks `'wasm-unsafe-eval'`, so `WebAssembly.instantiate` at `dwg/🟦️.ts:53` is blocked even when the bytes arrive.
- Both failures are handled (`dwg/🟦️.ts:78`): the export panel reports DWG as unavailable. This is a feature gap, not a crash.

Fix: build the WASM before the site build, copy it into `dist`, add `'wasm-unsafe-eval'` to `script-src`, and serve `.wasm` as `application/wasm` through the host configuration.

### 4. Headers and CSP depend on a Netlify or Cloudflare Pages host (medium, depends on host choice)

`_headers` is the only header source (`🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts:24-38`, written at `:111`). It uses the Netlify and Cloudflare Pages format. On S3, CloudFront, GitHub Pages or a plain static server it is ignored, which means:
- no immutable caching for `/assets/*` and no `no-cache` for documents;
- no `X-Frame-Options` / `frame-ancestors` header. The CSP `frame-ancestors` directive in the meta tag is ignored by browsers, so clickjacking protection disappears;
- no 404 status handling for `/404.html`.

The only CSP is the meta tag produced at `:41-56` and `:100`. It is correct for scripts and styles, but it cannot carry `frame-ancestors`.

Fix: decide the host now. Either make the host's native header config the source of truth and generate it from `STAFFAGE_SITE_HEADERS`, or keep `_headers` and confirm the host reads it.

### 5. Inline `<style>` injection is blocked by `style-src` (low)

`style-src` (`:46-47`) has no `'unsafe-inline'`, so runtime `<style>` elements are refused:
- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧊️spatial/🖥️stage/🟦️.ts:88-89` is the fallback when `adoptedStyleSheets` is unavailable (older Safari and Firefox). Modern browsers take the `adoptedStyleSheets` path, so this is low risk.
- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧩️element/🟦️.ts:247,289,320` writes `<style>` through `innerHTML`. The element is a public export (`🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx:8`) that the archive app never registers, so it is off the archive path.

Fix (if the fallback matters): use `CSSStyleSheet` construction or `replaceSync` instead of `<style>` text.

## Passes and caveats

- Node-only modules: the browser graph reaches Node built-ins only through the cast exporter (`🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts:13-18`) and the catalog `files` module (`🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts:7-9,19`). The cast exporter is replaced by `cast-stub.ts` through a `resolveId` match on the path suffix (`🌐️site/🏗️builder/🌐️vite/🟦️.ts:126-134`). Caveat: the exclusion is fragile. Any new static import of the real cast module breaks the build or the site at runtime.
- Fetch and dynamic URLs: no runtime `fetch` in `🎯️targets/⚛️react/🔨️modules` (grep). The catalog is bundled: manifest JSON import (`📚️catalog/🟦️.ts:13`) and category raw imports (`:25`). Only the DWG WASM (blocker 3) is a runtime fetch.
- Dev-only endpoints, localhost, ports, `/@fs`, `/_semio`, `/semio-backbone`: none in product code (grep).
- External origins and CSP: no Google Fonts or CDN references. Fonts are local `woff2` files under `🧰️framework/🔨️modules/🖼️assets/🔤️fonts` (committed), referenced as root-absolute `/🖼️assets/...` from `🎨️styling/🤖️generated/🔤️palette-fonts/🎨️.css:4` and copied into `dist` by `semioReferencedAssetsVitePlugin` (`🎨️styling/🏗️builder/🌐️vite/🗃️assets/🟦️.ts:65-`). `font-src 'self'` is correct. The `🤖️generated` directory is git-ignored (`.gitignore:97`), but the site build target declares `@semio-tech/ui-styling-tokens:generate` in `dependsOn`, so repo builds regenerate it. A bare `vite build` does not.
- Workers and WASM loading: no `new Worker`, no `eval`, no `new Function` in the browser graph. WASM is covered by blocker 3.
- CSP hashes: Vite 7 does not minify HTML, and it inlines `<style>` contents before the post-transform hooks run (`node_modules/vite/dist/node/chunks/config.js` around lines 24380-24388). The hashes from `staffageContentSecurityPolicy` should therefore match the output. Verify on a real `dist`.
- Favicon: emitted as `favicon.svg`, `favicon.ico` and the emoji-named variants (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🔖️favicon/🟦️.ts:86-94`). `manifest.webmanifest` references `/favicon.svg` (`🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts:112`), so it resolves. Emoji-named files in `dist` are a minor deployment risk only.
- 404 page: `404.html` is written as a copy of the app shell (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🌐️site-html/🟦️.ts:67,96-98`). Unknown paths therefore return the app with HTTP 404. Routing keeps its state in `URLSearchParams` (`🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🗺️routing/🟦️.ts:32,43,73`), so deep links work from `/` without rewrites.
- `CNAME` and `.nojekyll`: written by `staticDeployMarkerVitePlugins` (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🌐️site-html/🟦️.ts:31-35`).
- Verify: the artifact pipeline stages `dist/<mode>` through `buildViteArtifact` and `collectArtifactFiles` (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗂️files/🟦️.ts:31`). Confirm that `_headers`, `CNAME`, `.nojekyll`, `robots.txt` and `404.html` survive staging.

## Recommended verification (not run)

1. `repo run @teaching/staffage-site:build`, then list the release output and check for `_headers`, `CNAME`, `.nojekyll`, `404.html`, `favicon.svg`, `manifest.webmanifest`, `🖼️assets/` and the absence of `staffage_dwg.wasm`.
2. Serve the output with the production CSP in a headless browser and check the console for CSP violations on boot and on first draw.
3. Open a planar figure with a print profile and check for the `process is not defined` error (blocker 2).
4. Simulate a stale deploy (rename one chunk) and confirm that the lazy 3D and category loads fail without a reload (blocker 1).
