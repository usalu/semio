# Play Runtime and Deployment Coverage Audit

Read-only source audit on 2026-10-07. No tests, builds, browsers, servers or deployments were run. No runtime health claim follows from this report.

## Observed Structure

- Application: `🏢️semio-tech/🎡️play`, Nx project `@semio-tech/semio-tech-play`.
- Authored catalog: `🔨️modules/🧩️runtime/🔣️.json`, currently 148 variant rows.
- Unit target: `bun nx run @semio-tech/semio-tech-play:test`. Vitest includes source gates for runtime, brand, activation, assets, map serving and page publication.
- Browser target: `bun nx run @semio-tech/semio-tech-play:test-e2e`. Nx activates dev components and creates an isolated service generation. Consumer validates service ownership before Playwright runs.
- E2E config runs Chromium only, one worker, no full parallelism. Acceptance generates one overview test and one boot test for each catalog pane: 149 tests for the current catalog.
- Existing VS Code launch entries include Play dev, build and E2E commands.
- Build target depends on prepare-release and invokes `🔨️modules/📦️site/📜️script.ts build`. This prefetches map tiles, builds ship-mode React assets, then splits `dist/site` into `dist/pages` and removes the monolith.

## What Existing Acceptance Actually Verifies

`🧪️tests/🎭️acceptance/🟦️.ts` checks overview card count and accessible labels. Each pane deep link must report shell-ready, avoid page errors and refused inputs, have no pane error boundary, show its curated example label when a picker exists, publish a content witness, and return to the overview.

The paint witness asks retained 3D geometry, raster layers/assets, or DOM content for evidence. The pane passes if any window paints; it does not require every window to paint. Raster evidence requires a visible image layer with decoded dimensions at least eight pixels per edge. DOM evidence is nonempty text or at least ten elements.

Static pane-coverage tests enumerate plugin directories, descriptor editor apps, viewer role reachability and runtime dependency closure. These are valuable registry gates but do not execute viewer switching in the browser.

## Concrete Coverage Gaps

1. Browser acceptance does not mutate an artifact, inspect a resulting event or revision, exercise undo/redo, export/import, reload persistence, switch editor/viewer, change examples interactively, or test cancellation. Therefore its success alone cannot establish functional editor workflows end to end.
2. `significantConsoleErrors` suppresses all resource errors matching `40[0-9]`, despite its docstring describing resource 404 suppression. Actual 400/401/403/404 asset responses can disappear from the console gate.
3. The SPA fallback gate detects HTML responses only for its named asset extensions. It does not inspect failed request events or reject every non-success asset response. Extensionless asset URLs and asset types outside its regex are outside this gate.
4. Example label assertions are conditional on a picker being present. A missing picker skips that browser assertion; static defaults tests must establish whether the absence is expected.
5. The existing E2E server is a Vite dev service. It does not serve the freshly published four-host page output. Cross-origin asset relocation, CORS, worker delivery, and release artifact closure therefore need separate runtime validation.
6. Desktop Chromium is the only configured browser/device. This audit found no mobile, tablet, Firefox or WebKit acceptance configuration for Play.

## Publication Expectations

`🔨️modules/📦️site/📄pages/🟦️.ts` assigns pages under a 1,000,000,000-byte per-page budget:

| Page | Host | Content |
| --- | --- | --- |
| play | play.semio-tech.com | Application and remaining directories |
| map | map.assets.semio-tech.com | osm, vt, dem |
| media | media.assets.semio-tech.com | mesh, cad-assets, infinite-assets, framework asset directory |
| modules | modules.assets.semio-tech.com | Plugin and extension module routes |

Satellite pages are omitted when they have no assigned directories. Publication writes `CNAME` and `.nojekyll` to pages and `_headers` permitting cross-origin access to satellite pages. It rewrites stylesheet asset URLs, injects shard-worker fetch relocation, and retains a copy of the shard worker on the Play origin because browser workers cannot directly load from another host.

Only `.github/workflows/architecture-quiz.yml` exists in the checked workflows directory. No Play deployment workflow was found there. This is an observed absence of a repository workflow, not evidence that external hosting is unconfigured. The build produces reviewable static publication folders; external deployment mechanics remain unverified by this audit.

## Recommended Validation Order

Run the existing unit gates, then a fresh Nx release build with ticket-generated logs. Validate built publication folders and component inventories. Run all 149 existing acceptance cases against the owned dev service, then exercise the publication artifact with origin-aware local hosting or request routing. Add browser workflows that switch viewers and apply representative schema-valid changes per artifact family, with observable revision/output evidence. Preserve all unexpected resource failures in those gates.

These recommendations are inferred from the source coverage gaps. No concrete runtime failure was reproduced in this read-only audit.
