# 🔬️ Relocatable Demonstrator Bundle

Goal: the static bundle runs below any sub-path (`/sub/`), not only at a domain root.

## Design

One framework-owned runtime base route, in the existing published-page owner (`🧰️framework/🔨️modules/🖼️assets/🌐️published-page/🟦️.ts`):

- `runtimeBasePath(documentBaseUri, moduleUrl, production)` is pure. A document derives the base as the directory of its `document.baseURI`. A worker of a production build derives it from its own module URL, which lies one directory (`assets/`) below the bundle root. Development, a non-http(s) document and an opaque document yield `/`.
- `publishedPageUrl` keeps CDN origin precedence (`SEMIO_PLAY_PAGE_ORIGINS`). A root-relative path that no CDN page claims is placed below the base. A path already below the base is returned unchanged, so repeated relocation is idempotent. Relative paths, absolute URLs and `//host` URLs stay put.
- `relocatePublishedRequestUrl` and the installed global fetch use the same base, so Rust and wasm fetches of `/renderer-modules/*`, `/mesh`, `/cad-assets`, `/infinite-assets`, `/osm`, `/vt` are relocated without per-site code. The fetch patch is now installed when either an origin map or a non-root base exists.
- No route list is kept: every root-relative runtime path is by construction one of the bundle's routes.

At a domain root and under the dev server the base is `/`, so every function is the identity.

## Owners changed (non-fetch consumers)

| Owner | Change |
|---|---|
| `🎭️actor/🧵️shard-runtime` | `shardWorkerUrl()` relocates `/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js` |
| `🎠️kernel/📦️package-assets` | `packageAssetUrl` relocates `/renderer-modules/package-assets/...` |
| `wgpu/🎬️renderer-boot` | default renderer module route and every plugin `moduleUrl` (`/🔌️plugin-modules`, `/🧩️extension-modules`) relocated before resolution |
| `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts` | partner logos (`/♻️mit-bestand/🧺️demonstrator/🖼️asset/...`) relocated through one `demonstratorLogoUrl` |

Already relocating through `publishedPageUrl` (unchanged): mesh GLB loaders, tile templates (`/osm`, `/vt`), plugin module loader, the frame worker asset pump.

## Test

`♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorrelocatable/`:

- `🧬️schema/🔣️.json` and `🧫️cases.json`: the language-agnostic corpus (document base, input route, expected output, worker base derivation, owner outputs).
- `🟦️.ts` (bun): validates the corpus with Ajv; checks every case, plus the independent WHATWG oracle (`whatwg-url`: `"." + route` resolved against the document base); derives worker bases; checks the owners; installs the fetch patch against a fake global fetch; and serves a fixture site (one file per route) below `/sub/` over HTTP with a node server that answers 404 outside the mount, asserting each relocated route answers 200 with the right bytes below `/sub/` and the unrelocated root-absolute route answers 404.
- `SEMIO_DEMONSTRATOR_DIST=<dist>` serves a real built output in place of the fixture. No release site build was run (memory budget; one heavy process at a time).

## Not covered

- CSS `url()` and `<link>`/`<script>` in HTML are already relative (`base: "./"`).
- Story files (`/renderer-modules/wgpu` in `📖️stories`) are dev-only and not relocated.
- A worker derives its base from the module URL (`../` of the chunk), assuming Vite's default `assets/` chunk directory.
- The `/🧩️extension-modules/install` endpoint is still answered only by the dev server.

## Evidence

`bun test ♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorrelocatable/🟦️.ts`: 33 pass, 0 fail, 130 expectations. It was run with the repo-pinned bun binary directly, because `repo tools exec bun` sat in the shared queue for over 10 minutes without starting; it was stopped. The existing vitest suite `🧪️playpages` (CDN origin relocation) was not rerun; the origin-first precedence and the identity behaviour without a document are unchanged by construction.
