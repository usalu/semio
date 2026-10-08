# Release Deployment Contract Audit

Read-only follow-up performed on 2026-10-07 while the fresh component graph compiles. No deployment was attempted, and no new source defect requiring a patch was identified in this pass.

## Published Directory Contract

| Output Directory | Required Host | Authored Content |
| --- | --- | --- |
| `dist/pages/play` | `play.semio-tech.com` | App HTML, bundled JS/CSS, app-owned files, same-origin shard-worker copy |
| `dist/pages/map` | `map.assets.semio-tech.com` | `osm`, `vt`, `dem` map families |
| `dist/pages/media` | `media.assets.semio-tech.com` | `mesh`, `cad-assets`, `infinite-assets`, `🖼️assets` |
| `dist/pages/modules` | `modules.assets.semio-tech.com` | `🔌️plugin-modules`, `🧩️extension-modules`, support shim/font data and shard worker |

`playPageHost` supplies the CNAME declaration and origin-map host names from one authority. All pages emit `.nojekyll` and stable asset revalidation metadata; each satellite also emits wildcard CORS metadata. Publication checks that every completed page is strictly below 1,000,000,000 bytes. The final size now includes metadata, style rewrites, worker prelude and app-origin worker copy. That final-accounting repair passed the 18-test focused publication suite.

The app bundle has baked HTTPS CDN origins injected through `SEMIO_PLAY_PAGE_ORIGINS`. The worker constructor retains the app origin, while worker component/diagnostic imports and fetches resolve to satellites. `loadPluginModule` applies `publishedPageUrl` before bridge loading. Component support files copied into the modules page come from the explicit runtime closure, not all unrelated build directories.

## Repository Deployment Configuration

Inventoried provider configuration names (`hosting.json`, `netlify.toml`, `wrangler.toml/json/jsonc`, `vercel.json`, `firebase.json`) and `.github/workflows` outside generated/cache/ticket directories. There is no Play deployment-provider configuration or upload workflow selecting these four directories and hosts. The only current workflow is the unrelated architecture quiz deployment. The generic component-deployment contract validates owner-authored physical directory identities; it does not upload Play pages.

Consequently, the repository currently prepares four deployment artifacts and exposes verification gates. It does not establish an automatic Play upload step or prove that a hosting provider consumes `_headers`, CNAME or `.nojekyll`. All four directories must be published as one release generation to the declared hosts. Actual HTTPS, JavaScript/Wasm/font MIME, CORS, missing-file 404s and cache behavior must be verified at the provider boundary. This audit intentionally does not select a provider or copy the quiz's distinct hosting assumptions into Play.

## Fresh Build Gate

`build-fresh` is uncached and invokes the complete `build` graph with local/remote task cache bypass. It creates caller-owned Nx and Cargo compiler-storage directories and strips inherited `NX_TASK_*` and graph-reuse metadata. It preserves other developers' shared storage. The graph materializes release components before `catalog-release`, which refreshes the canonical descriptor projection before `prepare-release` and the site bundle/publication step.

The language-neutral freshness contract tests the isolation environment, Nx argument parsing with `yargs-parser`, and the catalog publication order. Package scripts call Nx; project targets call the respective `📜️script.ts`. Fresh build, release gate, routing contracts, browser capability, publication audit and worker-routing probe commands are present in generated launch configuration.

## Release Browser Gate

`test-release` is an uncached target whose only prerequisite is the browser runtime. It serves emitted `dist/pages` through four independent static listeners and never starts a development Vite server. The production Playwright fixture uses the exact reusable `installPublishedPlayRoutes` adapter to fulfill baked CDN requests from the corresponding satellite while retaining status, MIME, body and CORS responses. Missing files return 404, not application HTML. Browser contexts are isolated, and the script tears down its own listeners and supports cancellation.

The adapter has been proven with a real same-origin module worker performing baked CDN emoji-path fetches to all three satellites and a cross-origin dynamic import. All four requests returned 200, correct JSON/JavaScript MIME and `Access-Control-Allow-Origin: *`; actual console evidence is saved in `📓️worker-route-adapter-proof.md`.

`test-release` is deliberately independent of `build-fresh`; it can verify an existing artifact. It does not itself require a newly completed generation, call an uploader or prevent a caller using a filtered Playwright run. Thus a successful full release gate must follow a successfully completed fresh build and the final completeness audit when claiming fresh redeployment readiness. An overview-only or filtered run does not establish the 149-test acceptance scope (overview plus 148 panes).

## Concrete Readiness Sequence

1. Wait for the `build-fresh` target to finish successfully; compile/descriptor errors must remain failures.
2. Run the full Play unit target after the canonical catalog is current.
3. Run `test-release-routing` and the browser-capability gate.
4. Run the ticket publication audit against the final four directories; it checks exact CNAMEs, metadata, byte budgets, every declared bridge/Wasm, worker equality and literal asset/module references.
5. Run the unfiltered `test-release` target against those exact emitted artifacts, covering all panes and editor/viewer states.
6. Preserve the verified artifact generation for the separately authorized upload of all four directories; verify real provider responses after publication.

The source contains development-only filesystem/backbone middleware. Those Vite server routes are not published as static services. Local-first curated examples must function through the release browser gate; any optional shared/server-backed feature requires its separately configured service and cannot be proven merely from these four static folders.

## Current Boundary

No final generated release has yet been inspected in this pass. The completeness input is ready in `🔍️publication/📜️script.ts`; its prior execution correctly rejected the incomplete tree. It will run again when final outputs are ready. No deploy, successful fresh build, full pane acceptance or real provider HTTP result is claimed by this report.

## Clean Publication and Obsolete Deliverables

The release graph does not merge old component deliverables into the new release. `🏃️process/📦️artifacts/🏗️native-build/🟦️.ts` collects the current Cargo final artifacts into a unique capture directory, validates that set and publishes through `stageArtifacts`. `🏃️process/📦️artifacts/📤️publication/🟦️.ts` validates ownership under an exclusive publication lease, compares the complete current manifest/file set, then replaces the entire destination with a unique staging tree. Its identical-content fast path rejects any extra child, so obsolete files also force replacement. Successful replacement deletes the previous tree; failure attempts rollback rather than exposing a partly copied tree.

JCO's low-level `transpilePluginComponentAsync` does not clear its caller's directory, but both actual release materializers supply `mkdtempSync` directories. `plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts` extracts a fresh descriptor from that freshly generated component, then stages only its current files using the Cargo manifest/profile ownership key. `plugin/🏗️build/📦️materialization/🟦️.ts` uses the same isolated staging/complete replacement pattern. Thus obsolete generated `.core*.wasm`, interfaces or descriptors in the previous owner directory are excluded, without deleting any shared build input.

The shared module parent may retain directories for other component owners. Play's `🧩️runtime/📦️assets/🟦️.ts` explicitly selects its catalog component IDs and browser support/font owners. `plugin/🌐️browser-bundle/📦️distribution/🟦️.ts` copies only each current ownership marker's declared files, rejects duplicate destinations and uses exclusive creates; it does not sweep/copy the parent tree. Retired directories outside that declared closure do not enter the deliverable.

`repo/⚡️caching/🌐️vite/🟦️.ts` bundles into a unique Vite directory with `--emptyOutDir`, collects the complete result and replaces `dist/site` through the same owned publisher. Finally `play/📦️site/📄pages/🟦️.ts` removes the prior `dist/pages`, moves only the current monolith entries, writes current metadata, rewrites the worker, copies its same-origin version, computes final byte totals and removes the monolith. No confirmed obsolete-generated-file survival defect was found in that release path. This is source evidence; final artifact inventory and browser acceptance remain separate gates. No live build directory was deleted during this audit.

## Stable Runtime URLs and Cache Readiness

The generated registry uses stable module directory/bridge filenames. The generated bridge adds actor/activation identity to component and host-shim URLs; it propagates `v` only when a caller already supplied one. `rewriteJcoComponentAssetUrls` likewise propagates optional `v` to extracted core Wasm. Production registry URLs carry no release hash/query, and the shard worker, vendor shims and descriptors use stable filenames. Descriptor hashes admit exact component identity but do not themselves replace URLs with content-addressed filenames.

Before this repair, the publisher emitted only satellite CORS metadata and no app `_headers`; it supplied no cache policy for stable runtime paths. The local acceptance server explicitly sent `no-store`, so its fresh context could not establish real CDN/browser cache freshness. A neutral emitted-header regression now requires `Cache-Control: no-cache` on every host, so compliant HTTP caches must revalidate stable app/worker/module/Wasm/media/map paths before reuse. The existing third-party `http-cache-semantics` oracle compares long-lived immutable caching against the emitted revalidation policy. The parent changed the acceptance server to read the emitted cache policy faithfully and reported its neutral HTTP regression green (14 tests, 63 assertions), following an actual baseline that served no-store instead of declared no-cache. That parent evidence is distinct from this agent's 19-test publication result.

`no-cache` permits retaining bytes with validators while requiring origin validation. It cannot retroactively revoke an already cached response whose earlier policy was long-lived immutable; actual provider cache purge and a browser reload/fresh context remain separate deployment actions if old responses were cached that way. An already running realm's ESM module map is also retained until reload. No provider policy or purge endpoint exists in the inspected repository configuration, so this audit does not claim either purge was performed.

## Stable Asset Cache Regression

A language-neutral `cache` fixture in Play's publication corpus describes all four hosts, stable representative files, previous long-immutable response headers and the required `no-cache` policy. The production publication test parses each emitted `_headers` and uses existing `http-cache-semantics` to prove the previous policy permits immediate reuse while the current policy requires revalidation; satellite CORS is still present. The regression actually failed before repair (1 failed, 18 passed; app `_headers` absent), then passed after writing revalidation metadata to every host (19 passed). Logs: `🗑️generated/publication-cache-red.log` and `publication-cache-green.log`.

The same fixture now seeds obsolete files in every previous page and a retired fifth page. The independent `glob` inventory verifies none survives republishing. The final rerun including these clean-republication assertions passed: 19 tests, exit 0; its separate log is `🗑️generated/publication-cache-clean-green.log`. The completeness auditor now requires `Cache-Control: no-cache` on all four final pages and checks literal references passed through `__semioVersionedComponentAssetUrl`, in addition to native `new URL` asset references. Provider metadata consumption and actual provider cache invalidation are unverified deployment operations.

## Diagnostic Path Portability

The retained actual Nx exec probe under a literal directory with spaces passed with exact POSIX and Windows-style workspace strings and an emoji argument. Existing third-party string-argv independently parsed the installed serializer's form to the same argv. The pinned Nx implementation wraps argv elements in double quotes but does not escape embedded double quotes, explaining the earlier inline-code failure; our diagnostic commands use filenames and plain arguments. No ordinary spaced-path launch defect was demonstrated. Native Windows execution remains unverified, rather than inferred from the macOS path-string proof. Full retained runtime evidence is in `📓️diagnostic-argument-path-audit.md`.

## Descriptor and Shipped Core Digest Gate

The descriptor finalizer's existing relation is exact: `finalizePluginDescriptor` writes `hashes.coreWasmSha256`; release materialization supplies the SHA256 of `${componentBase}.core.wasm`. The final-page auditor now requires every selected component directory to contain a descriptor with that valid digest and exactly one primary `.core.wasm`, streams the shipped bytes through native SHA256, and rejects any mismatch. Additional `.core2.wasm` files are checked through literal asset references; this field describes the primary core specifically, not an invented aggregate of all cores.

A strict neutral corpus describes valid, mismatched, missing-core and invalid-digest synthetic cases. The real presence-only baseline accepted the mismatched byte file and failed the fixture (exit 1). After adding the digest relation, all four cases passed (exit 0), independently compared against existing third-party `@noble/hashes` SHA256. Actual `[DEBUG]` logs record each outcome and independent digest in `🗑️generated/descriptor-digest-red.log` and `descriptor-digest-green.log`. Those synthetic files are test inputs, not production component validity claims.

The verifier is registered in both launch configs as `🔎️diagnostic🏢️semio-tech🎡️play🔏️digests`, `4_gate` / `11.09`. The final inventory's digest count and byte proofs will be produced only after the coordinating build declares output complete. No partially populated publication was scanned, and no runtime/source schema was edited.
