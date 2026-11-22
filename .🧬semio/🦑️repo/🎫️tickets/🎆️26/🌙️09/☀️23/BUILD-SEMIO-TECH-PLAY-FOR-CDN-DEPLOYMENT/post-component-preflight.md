# Post-component preflight: publication, verify, export, release E2E

Scope: everything after the component phase (Vite bundle, `publishPlayPageCohortV1`, `release-verify`, `test-release`, `release-export`). No `site` or `build` was started; all runs were read-only or prototype mode.

## Defects found and fixed (both would have failed the attempt after the compile)

1. **Every Play artifact reader was refused.** `request()` in `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/📦️publication/🟦️.ts` granted `maximumBytes = maximumOutputBytes + maximumManifestBytes` (8 GiB + 2 MiB). The closed reader request schema (`⚡️caching/📦️artifacts/👓️reading/🧬️schema/🔣️.json`) caps `maximumBytes` at 8589934592, so the bundle reader, the 82-member runtime asset group, the four family readers and the cohort reader all threw `Actual closed artifact reader custody required` (reproduced with a real invocation in prototype mode). Fixed: the grant is `maximumOutputBytes` (the publisher already counts the manifest inside it). The helper is now exported as `playPagesArtifactRequestV1`; new law "every Play artifact reader grant stays inside the closed reader request schema" in `📦️publication/🧪️tests/🟦️.ts`.
2. **`verify` could never pass.** `runVerifyScript` (`📦️site/🟦️.ts`) rendered the catalog with the neutral default publication, but the committed `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` is produced with `specificCatalogPublicationV1` (plugins plus Hub contribution): 10 of 13 files differed. Fixed: verify builds the same specific publication and reads `publication.outputRoot`. Re-rendered with the specific publication: 14 files, 0 differ.

Refactor to make the writer testable: the exclusive-lease family/cohort staging block of `publishPlayPageCohortV1` is now the exported `commitPlayPagesV1`; behaviour unchanged.

## Static checks (all green)

- Every module on the path loads under Bun (missing files, aliases, named exports): site, publication, export, pages, e2e, release server, map tiles, driver stages, cargo, registry projection, routing, owned execution, artifacts, playground, all `@semio-tech/*` aliases used. Scripts: `post-component-load-check.ts`, `post-component-dynamic-check.ts`.
- `nativeBuildPipelinePlanV1` accepts the `build` pipeline for the 73 play components (27 passes, 14 jobs); every job root and script exists (`post-component-plan.ts`).
- Publication input contract: 82 runtime asset sources (73 components, preview2 vendor, shard, fonts, 6 Canvas), unique roots, accepted by `readRepositoryArtifactGroupRequestsV1`, below the 128 member cap. Component owner `<crate>/Cargo.toml:browser`, vendor, shard, fonts and Canvas owners equal the markers on disk; `play:site` layout path equals the existing bundle output (`post-component-sources.ts`, `post-component-vite-layout.ts`).
- Module catalog schema: all 73 plugin ids match `PlayVariant`, directory names max 39 units, files max 160 units (`post-component-ids.ts`).
- Limits: bundle has 2296 files (cap 16384 per family), media 1479 files 171 MB, map 396 tiles, all families far below the 1 GB page budget. Map tile cache is complete for zoom 0..10 (198 tiles x 2, 0 missing), so `map-tiles` needs no network (`post-component-tiles.ts`).
- Path budget (256 units, `filesAt`): longest assembled file is `<scratch>/pages/media/🖼️assets/.../💥️cylindric-tambour_single-storey_collider.glb` = 248 units on this checkout. Only 8 units of slack: a checkout path longer than 8 characters than `/Users/ueli/Documents/semio` fails the publication after the compile. Not changed (limit is a framework protocol constant); keep the checkout path short, or shorten asset names (`post-component-path-budget.ts`).
- Playwright: version 1.64.1 matches installed `chromium-1248`; `test --list` loads the config and lists 173 tests from the repo root with the same resolution the release runner uses; `runOwnedCommand`, `withScriptInvocationSignalV1`, `scriptInvocationEnvironment` signatures match the e2e call.
- Mode: `build`, `release-verify`, `release-export`, `test-release` read `SEMIO_BUILD_MODE` and refuse it unset; set `SEMIO_BUILD_MODE=release` for `repo run` too (native-play.sh already does; noted in `cdn-export.md`).

## Executed

- Play units (vitest, `native-play.sh step ... repo ./📜️script.ts test`): 82/82, before and after the fixes (includes playfreshbuild, playmaptiles, playpages, playscopes, playactivation, playdevmounts, playgrid, panes).
- Bun: `playrelease` (13), `release-job-order`, `playpages` (19), publication laws (5, one new), export laws (2): 40 pass, 0 fail, 309 assertions; `native-play.sh routing` 13/13.
- Publication audit `🔍️publication/📜️script.ts verify-hashes`: all four synthetic digest cases behave. The full `audit` needs a real cohort.
- Prototype end-to-end smoke (`post-component-publish-harness.ts`, refuses release mode): synthetic site split by `publishPlayPages`, `commitPlayPagesV1` under the exclusive cohort lease, read back through `withPublishedPlayPagesV1` with the native-owner-command invocation, served by `servePublishedPlay`: 4 families, play index 200, missing file 404 without fallback, media 200 with `Access-Control-Allow-Origin: *`. `release-verify` against that cohort reports exactly one finding (the synthetic cohort has no module catalog) and the generated catalog now matches. The synthetic prototype cohort remains in `.cache/b/p/m/play/prototype/pages` (release untouched); the next prototype publish replaces it.

## CDN layout agreement (export vs `test-release`)

Both consume the same committed cohort (`withPublishedPlayPagesV1`), so the families are identical by construction:

| Item | Publisher writes | Export copies | test-release serves |
|---|---|---|---|
| Families | play, map, media, modules (empty satellites dropped) | `<out>/<family>/` | one 127.0.0.1 listener per family |
| Host | `CNAME` = `playPageHost(family, PLAY_HOST)` | `Family.host` read back from `CNAME` | origin key `https://playPageHost(name, PLAY_HOST)`, Playwright route regex `(map\|media\|modules)\.assets\.semio-tech\.com` |
| Headers | `_headers`: `Cache-Control: no-cache`, plus `Access-Control-Allow-Origin: *` on satellites | parsed into the descriptor | parsed by the server (cache, CORS) |
| Misc | `.nojekyll`, `🧾️page.json` | byte-for-byte with digest recheck | `.semio-artifact.json` marker also visible locally, never exported |
| Misses | no SPA fallback | rule in `🔣️cdn.json` | 404 |

Export requires a release cohort, an empty target, and removes its staging folder on any failure (unit-tested).

## Not verifiable without a cohort

Real module binding (`bindPlayRuntimeAssetsV1` over 73 compiled modules), `release-verify` module catalog findings on real files, the browser acceptance suite on GPU (`browser-capabilities` precondition), and a real export. Order after a successful build: `native-play.sh verify`, then `SEMIO_BUILD_MODE=release repo run @semio-tech/semio-tech-play:release-export`.
