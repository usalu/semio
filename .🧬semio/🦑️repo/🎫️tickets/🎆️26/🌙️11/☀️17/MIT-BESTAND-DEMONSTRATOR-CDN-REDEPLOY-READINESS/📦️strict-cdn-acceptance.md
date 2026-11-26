# CDN Release Acceptance

The static acceptance launcher starts a dedicated origin from the supplied Vite output. Its default port comes from the OS. It discovers and runs every configured English and German case with the Demonstrator's own Playwright installation. The 52 baseline cases include all 16 pane boots and interaction coverage such as editing, undo and redo. It also runs all 44 behavior executions on that same captured origin, including the four restored CAD and Process extension cases. Exact project and case rosters are verified before Chromium starts. The configurations discover 96 unique executions and share no cases. It publishes `🗑️generated/static-accepted.sha256` only after coverage and origin checks pass and the site manifests before and after the run match. It also checks that the test suite, fixtures, runner and brand source keep the same hashes. The launcher clears an earlier acceptance under the verification lock and stores evidence in a separate run directory. It checks the captured CNAME against the authored host, the empty .nojekyll marker, the application title and mount, and a local compiled entry script before and after Chromium.

From the workspace, use `repo tools exec --light python -- -I "$T/accept-site.py" "$OUT"`. Here `T` is this ticket and `OUT` is the output from the successful release site build. The Python entry point needs the repository's Node and Playwright installation and the host GPU for the map pane. The Bash entry point uses the last successful native site capture and can also specify a port.

Once the suite passes, use `repo tools exec --light python -- -I "$T/package-site.py" "$OUT" "$DESTINATION" "$T/🗑️generated/static-accepted.sha256"`. The packager checks release references and manifests, enforces the 1 GB published size for GitHub Pages including the checksum manifest, streams the archive through its verifier and publishes it in the destination. The Python origin and package entry points use the repository tool registry.

## Checks

- Project-local Playwright discovery found 26 baseline cases and 22 behavior cases in each of the two locale projects. The baseline includes eight pane boots per locale.
- The Playwright reporter fixture accepted the full roster, rejected a boot-only report and failed editing, and rejected skipped, expected-failure, missing and repeated cases, an agent-bridge request, and HTTP failures for WebAssembly and a vector tile. The static-origin fixture passed 288 requests covering WebAssembly, vector tiles and missing-path behavior, with all 288 statuses preserved in the access log.
- The native queue and delivery fixture passed. It checked that an overlapping verifier preserves the current owner's acceptance, an incomplete matrix and absent origin evidence clear stale acceptance, site changes during verification block acceptance, and site changes after acceptance block packaging. The accepted fixture packaged and passed archive verification.
- The historical archive `.cache/deploy/site-20261125T134801Z.tar.gz` independently verified its internal inventory of 1,815 files and 2,496,793,374 bytes. The corresponding current Vite output passed the local-reference scan. Those checks establish archive integrity and local-reference policy. The historical browser matrix recorded 10 of 16 strict passes.
- The historic Vite output contained 11 plugins and 13 extensions with receipts. Eight CAD and Process extensions were withheld from that output, with earlier contribution stalls recorded in the ticket. The restored extension modules add approximately 50.2 MB compressed, projecting a static delivery around 825.1 MB from the probe inventory.

## Compact static diagnostic

The full Vite rebuild waited for host admission at 4,096 MiB and then at 3,000 MiB. Neither request acquired a grant before it was cancelled. The read-only fallback used the production distribution compressor and loader rewrite on a separate copy of the historical Vite output. The compact copy has 1,814 files and 774,903,364 bytes. Its largest file is the 72,504,360-byte Puzzle wasm. The largest encoded guest core is the 65,126,311-byte Demonstrator core. The copy passed the local-reference check. The Playwright run uses a separate diagnostic manifest directory and a 2,048 MiB browser grant.

The English project passed 14 of 26 cases. Its overview, language and funding cases passed, including the live language switch. Koordinator, Aggregator, Energie, Verfolgen and Statik booted with encoded cores. The failures on these older guest bytes were as follows.

| Case | Observed failure |
| --- | --- |
| Generator boot and undo/redo | Empty preview at boot and busy compute status during history, with old preview debug messages. |
| Aussuchen and Bearbeiten boot | Rendered the required content and reported the old work-quote console error. |
| Aussuchen preview | Visible Preview window stayed at No selection. The staged descriptor has no interaction subscription for Preview, while current source subscribes it to rows. |
| Aussuchen and Bearbeiten undo/redo | Shell-outcome timeouts with work-quote and unsupported-backtrace console messages. |
| Energie run/cancel | A terminal-empty assertion trapped typed-operation progress in the staged run job. |
| Statik undo/redo | Initial automatic solve remained queued. |
| Koordinator and Aggregator undo/redo | The original test queried unscoped catalogue row IDs. The expected catalogue entries were present. |
| Verfolgen undo/redo | The test queried an unscoped Inspector row ID. The snapshot showed Positions 152 and a pending Note Shell Command. |

The fresh Demonstrator queue completed its host descriptor emitter, then waited on compiler image admission. At its first detailed wait, the scheduler forecast 14.0 GiB of writes with 9.9 GiB free, 3.8 GiB reserved by active work and a 2.9 GiB reserve. The active writer protected every remaining unit from reclaim. After that writer released, the Demonstrator captured all 992 source units and hit the component driver's independent physical admission guard before its first compiler unit. Run `wWRav8W` recorded 7.95 GiB free against a 10.28 GiB compile plan plus a 2 GiB reserve, a shortfall of 4.32 GiB. The source manifest scan excluded the Procedural PDF model and PDF mutations crates from the smaller Demonstrator component's normal dependency closure.

The full diagnostic ended with 28 passes, 24 failures, zero skipped cases and zero flaky cases in 61 minutes 43 seconds. Both locale projects completed the pane boots, with 10 of 16 strict passes. Later workers loaded the corrected test selectors and exposed retained command-authority errors for Koordinator, caller-loan errors for Aggregator, and a stalled Verfolgen publication. A final run needs fresh guests and one consistent test revision.

The diagnostic's original shared text stream produced invalid UTF-8 under concurrent HTTP requests. Its 3,147 recoverable request lines include 46 encoded core fetches, and show no HTTP failure or agent-bridge request. The corrected acceptance origin writes through a locked logging handler and provides complete evidence for the eventual final run.

## Remaining Asset and Extension Checks

The release browser support and Canvas asset owners republished Preview2 shims, the shard worker, fonts and runtime icon assets. SHA-256 comparison confirmed that the five frontend engine WASM payloads in the historical Vite output match the current staged products. Surface, Editor, Node Graph and Flow deliveries date from November 25, and the stored Puzzle frontend delivery dates from November 18.

The scoped opt-in Playwright configuration discovers `koordinator extensions` and `bearbeiten extensions` in both locales. The CAD case checks actual same-origin descriptors and the associated viewports for Spatial Shape, Building, Energy and Structure. Its fixture matches all four staged contribution and model definition IDs. The Process case installs a machine from each of the Concrete, Metal, Robotic and Wood workstation catalogues. The reporter fixture accepted the complete four-case set and rejected a missing case. Browser execution with the fresh release site remains pending.

## Production Vite Diagnostic

The regular production Vite target completed using the restored extension catalogue and the guests already staged at the start of the build. The captured output contains 2,103 files, 826,856,511 bytes and 32 encoded WebAssembly cores. Its largest asset is 72,504,360 bytes. The 21 extension-module directories include all eight restored CAD and Process extensions. This candidate is approximately 173 MB below the Pages published-site cap. The changed guest roots and the full acceptance suite remain prerequisites for a release receipt.

The Workshop behaviour now addresses the installed-machine and catalogue rows by their panel and window identities. The focused static check serves the actual production Vite output and runs the four CAD and Workshop cases in both configured languages.

The early run completed all four scoped executions. Both Koordinator cases stopped at the older CAD guest's active-example initialization. Both Workshop cases stopped before opening its panel and reported the older Demonstrator work quote and unsupported-backtrace messages. The origin recorded 328 requests, including four encoded guest fetches and successful descriptor responses for all eight restored extensions. It recorded zero HTTP failures and zero development bridge requests.

All 32 real production module representations matched their staged receipts, decoded core SHA-256 values, distribution file inventories and binding rewrites. The package preflight accepted the site's local references. The release acceptance entrypoint also requires all module confirmations before and after browser testing. The independent 96-case fixture rejects incomplete behavior discovery, missing or failed behavior results, and accepts only after both suites pass. A direct release invocation on the early site refused its missing module receipts without starting Chromium or issuing an accepted manifest.

## Confirming Retained Extensions

The authored semantic descriptors matched all 21 independently staged extensions. Eighteen matched current stage-cache records through their exact transpile inputs, optimized inventories and semantic probes. Flow BIM and Dictionary had lost their ephemeral transpile records. Their retained raw Wasm and semantic descriptor hashes match their original receipts, and their certified core hashes and byte lengths match the audit, the independent earlier archive and the compact site.

[Earlier core custody audit](🔬️optimized-browser-core-custody.md)

The remaining Sourcing Windows proof replayed the retained linked Wasm through the native transpiler and release Binaryen. The two small adapter cores matched the published binding's embedded bytes. The external core matched the staged and served copies at 24,310,299 bytes and SHA-256 `b97103cc6567f1a7a600e8036e7bdc7746dc7d52e66ff5a9c8cc06512b614f54`. The successful production probe `b46aee77508d55b4390f3bc6db55e4c19c7b367b4b762eea5cde7fceeae0a51b` matches the same binding, shim, external core and authored descriptor. Its key and persisted output were validated. All 21 extensions now have confirmations. The 11 primary roots remain on the fresh build queue.

## Factory Core Transport

The site distributor also supports the newer async instantiation factory, which delegates core compilation to the shared browser worker. The delivered factory names the encoded core explicitly. Receipt verification reconstructs the binding and, for any compressed factory, requires the served worker to equal the current generated gzip-aware worker. A fixture with an actual factory call rejects missing and stale workers and rejects a deployed binding that still requests the raw filename. The canonical release browser-support target republished the current worker and Preview2 support. The real staged worker passed that verifier.

The retained extensions export the worker's public actor methods through their original monolithic bridges. Chromium activated and closed all 21 through the current shard worker without a failure. A freshly transpiled Windows factory compiled three gzip cores and returned the exact historical descriptor using the operation declared by its WIT. The full browser scenarios on rebuilt guests remain the functional release gate. [Current verification](📊️release-verification-current.md)

## Per-Owner Probes

The private origin can serve a focused Playwright case as soon as its guest is confirmed. It uses the production site's selected artifact list and copier to materialize the chosen gzip modules, current shard worker and Preview2 support. Selected module directories are sealed so a missing guest file returns 404 without falling back to a stale captured file. The retained frontend serves the remaining paths, and the run records actual encoded core fetches for each selected guest. Only the full fresh Vite run can issue release acceptance.

The HTTP fixture verified the captured page, substituted actor, query-bearing requests and encoded bytes, and proved that selected stale fallback returns 404. A real Concrete overlay also reproduced the previously certified receipt, decoded core, rewritten binding and current shard. The focused Demo+Process browser run is waiting for their fresh queue confirmations. [Focused runner](probe-owner-site.py)

## Current authored delivery roster

The canonical GIS runtime declaration requires Stdio in its declared transitive closure. The canonical catalogue and runtime fixture now include exactly 33 actors, comprising 12 plugins and 21 extensions. The release gate takes all IDs and deployment directories from this fixture. A controlled synthetic site with all 33 receipts passed the 96-case manifest/archive workflow and failed correctly when live fixture custody added Stdio to an older 32-receipt case. Existing acceptance specs still discover 52 baseline and 44 behavior executions, including all 16 pane boots in the baseline.

Fresh Verfolgen raw diagnostics passed four real English cases on a private static overlay. The raw component and checked descriptor pair were verified independently in the Rust emitter and browser factory. The [GIS diagnostic report](🔬️gis-fresh-descriptor-and-browser.md) carries the exact custody, browser and transfer observations. Its original paired compiler run did not seal. The final gate requires the normal certified browser receipt, the canonical current 33 hosted module set and all 96 actual browser outcomes.

The final native launcher now packages the accepted site in the same run. The package step re-reads the accepted manifest, independently checks every archive member, and publishes the verified archive under the workspace deployment cache.
