# CDN Upload Checklist for Play Pages

Read-only research from code. No build, no git write, no source edit.

## 1. Where the pages land after `build`

- `build` (`🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts`, `BuildScript`) runs the release pipeline and then `publishPlayPageCohortV1(repoRoot, invocation, "play.semio-tech.com")` (`📄pages/📦️publication/🟦️.ts`).
- Scratch: the Vite bundle goes to `<sink>/site`, the split pages to `<sink>/pages`. `publishPlayPages` (`📄pages/🟦️.ts`) deletes `site` after splitting.
- Final location is the products cache store, not `dist`:
  - `repositoryCachePartitionRootDataV1(repoRoot,"products")` gives `<repo>/.cache/b/p/m` (policy `.cache/b/p`, mount dir `m`).
  - `playPagesArtifactCohortV1` gives `root = <productsRoot>/play/<mode>/pages`, resource `repository:play-pages:<mode>`.
  - `<mode>` is `release` for `SEMIO_BUILD_MODE=release`, else `prototype`.
- Result, release mode:

```
<repo>/.cache/b/p/m/play/release/pages/
  play/      index.html, bundled JS/CSS, same-origin shard-worker.js, CNAME, .nojekyll, _headers, 🧾️page.json
  map/       osm/, vt/, dem/ (those present), CNAME, .nojekyll, _headers, 🧾️page.json
  media/     mesh/, cad-assets/, infinite-assets/, 🖼️assets/, CNAME, .nojekyll, _headers, 🧾️page.json
  modules/   🔌️plugin-modules/ (incl. 🧾️catalog.json), 🧩️extension-modules/, vendor shims, shard-worker.js, CNAME, .nojekyll, _headers, 🧾️page.json
  cohort/    🔣️cohort.json (commit marker, lists the four manifest digests; not a deployable file)
```

- `dist/pages` is NOT written by the current code. `dist/` in play holds only `reports/` (plus receipt/service paths used by other targets). Older notes (`📓️2026-10-10-release-verification-path.md`, `📓️2026-11-17-release-verification.md`) still say `dist/pages`; they are stale.
- The store is a fixed disk image per partition. In this checkout `.cache/b/p` does not exist yet, so no finished cohort is on disk now.
- If `SEMIO_MACHINE_STATE_DIRECTORY` is set, `.cache/...` moves under that directory (`machinePathV1`).
- Page budget: each family must be below 1,000,000,000 bytes (`PLAY_PAGE_BUDGET_BYTES`), checked after final metadata.

## 2. Deploy checklist (family, local path, origin, headers)

| Family | Local path (release) | Origin (host, HTTPS, root path) | Required headers |
|---|---|---|---|
| play | `<repo>/.cache/b/p/m/play/release/pages/play/` | `https://play.semio-tech.com/` | `Cache-Control: no-cache` on `/*`. No CORS (worker is a same-origin copy). MIME by extension: `text/html`, `text/javascript` (.js), `text/css`, `application/wasm` (.wasm), `font/woff2`, `application/json`. No COOP/COEP/CORP/CSP authored. |
| map | `.../pages/map/` | `https://map.assets.semio-tech.com/` | `Cache-Control: no-cache`, `Access-Control-Allow-Origin: *`. Tiles: `image/png` (.png), `application/x-protobuf` (.pbf, not set by repo, set it at provider). |
| media | `.../pages/media/` | `https://media.assets.semio-tech.com/` | `Cache-Control: no-cache`, `Access-Control-Allow-Origin: *`. Per-extension MIME (`.glb`, `.woff2`, `.jpg`, `.3dm` etc.). |
| modules | `.../pages/modules/` | `https://modules.assets.semio-tech.com/` | `Cache-Control: no-cache`, `Access-Control-Allow-Origin: *`. `.js`/`.mjs` as `text/javascript`, `.wasm` as `application/wasm` (needed for streaming instantiation; the repo's own store maps it at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🗄️store/🟦️.ts:71`), `.json` as `application/json`. |

Cross-cutting rules (all four hosts):
- Each family is served at the root of its own host (the `CNAME` value). No sub-path deploy.
- No SPA fallback. A missing file must return 404, never `index.html`. Only `play` has an HTML entry.
- No COOP/COEP needed. SharedArrayBuffer is optional (`📓publication-headers-worker-audit.md`). Do not set `Cross-Origin-Embedder-Policy: require-corp` without CORP on every cross-origin asset.
- Stable URLs (no hashes); `no-cache` forces revalidation. If an older response was cached long-lived, purge at the provider.
- `_headers`, `CNAME`, `.nojekyll` are emitted per page. No provider is chosen; no `netlify.toml`, `wrangler.*`, `vercel.json`, `firebase.json`, `hosting.json` or upload workflow exists (`📓release-deployment-contract-audit.md`). If the provider ignores `_headers`, set the same values in provider config. Real HTTPS, MIME and CORS behaviour can only be verified after upload.
- Upload all four families in one generation, from the same build.

## 3. Commands a human runs

Build (release, from the ticket folder, or the project target):
```
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/native-play.sh site
repo run @semio-tech/semio-tech-play:build
```

Verify (prints "Play release verified" but not the path):
```
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/native-play.sh verify
```

Locate and inspect the finished output (no CLI prints the path; `withPublishedPlayPagesV1` is in-process only):
```
PAGES=/Users/ueli/Documents/semio/.cache/b/p/m/play/release/pages
ls "$PAGES"
for f in play map media modules; do echo "$f: $(cat "$PAGES/$f/CNAME")"; done
du -sk "$PAGES"/play "$PAGES"/map "$PAGES"/media "$PAGES"/modules
```

No upload or deploy script for the semio-tech CDN exists in the repo. Upload is manual and needs separate approval.

## 4. Reusable requirements from the sibling audit

`📓️static-hosting-audit.md` (MIT-Bestand demonstrator, 2026-11-17) confirms: root deployment on the CNAME host; no COOP/COEP; no SPA fallback; missing module JSON must 404. Known non-published behaviour: map tiles outside the bundled Switzerland z0-10 set 404; extension install/remove and `/semio-backbone?uri=` are dev-only middleware (they 404 on a static host); the hub service worker is inert (`VITE_S_HUB_URL` unset).

## 5. Open items

- Pick a provider; map `_headers` and `CNAME` to its config.
- Set `.pbf`, `.glb`, `.woff2` and similar MIME types at the provider, since the repo does not emit them.
- Run `verify` and `test-release` against the final `<mode>/pages` tree before upload.
- Purge caches if earlier responses were long-lived.
