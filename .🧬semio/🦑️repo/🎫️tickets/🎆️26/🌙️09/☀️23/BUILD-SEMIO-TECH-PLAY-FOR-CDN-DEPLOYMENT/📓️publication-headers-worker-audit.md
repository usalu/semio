# Publication Headers and Worker Audit

## Required Behavior

The app and its copied module worker are served from `play.semio-tech.com`. The map, media and modules satellites use their exact `*.assets.semio-tech.com` host declarations. `playPageHost` supplies both CNAME and runtime origin maps, so the authored host names agree. Every final page gets `.nojekyll`; satellites get `Access-Control-Allow-Origin: *`.

WebGPU requires a secure context; the published origins are HTTPS. It does not introduce a cross-origin-isolation requirement into this implementation. [Mozilla WebGPU API documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API).

The actual shard client accepts an optional heartbeat SAB. Its message heartbeats and worker liveness remain active without SAB, and no production caller constructs an SAB. The UI compute-count helper explicitly falls back to one worker when isolation is unavailable. COOP/COEP enable the optional shared-memory capability, but are not required for the current shard-worker transport. [HTML agent-cluster definition](https://html.spec.whatwg.org/multipage/webappapis.html#integration-with-the-javascript-agent-cluster-formalism) and [Mozilla crossOriginIsolated documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated).

The Worker constructor uses the same-origin copy with `type: module`. Its component bridge and diagnostic shim imports point to the modules satellite and need that satellite's CORS response. No production asset fetch opts into credentialed cross-origin requests. CORP is not required by the current unisolated document; no restrictive CSP is authored in the Play page configuration. Adding either a new CSP or COEP policy could change resource loading, so none has been added. [HTML worker construction and policy processing](https://html.spec.whatwg.org/multipage/workers.html#dom-worker).

A generic Vite iframe middleware exists and sets COOP/COEP when installed during dev/preview; the Play configuration does not install that plugin, and it does not emit production headers. The source-only audit cannot establish whether the chosen host interprets `_headers` or CNAME. In particular, the unrelated quiz's GitHub Pages workflow states its CNAME artifact is ignored. That does not identify Play's deployment provider. The root release harness verifies emitted metadata and simulated HTTP behavior; deployment-provider consumption must be verified during actual publication.

## Actual Worker Import Inventory

Invoked the real `shardWorkerSource()` generator and parsed its 39,831-byte result with third-party `es-module-lexer`. It contains exactly two imports:

1. Literal dynamic import of `../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js`.
2. Expression dynamic import of `moduleUrl` passed through the activation protocol.

The worker also gates boot on `WebAssembly.Suspending` and `WebAssembly.promising`; those are JavaScript engine capabilities, so adding response isolation headers cannot satisfy that guard. The root browser-capability probe measures them directly.

There are no static imports or other absolute/relative literal module imports in the generated shard worker. Publication rewrites the one relative diagnostic import onto the module satellite and preserves the activation expression. No speculative static-import transform has been added.

## Confirmed Final Byte Accounting Defect

Publication previously checked and returned directory sizes measured before CNAME, `.nojekyll`, CORS metadata, stylesheet rewriting, worker-prelude installation and same-origin worker copying. The actual published app page therefore exceeded its reported bytes. New neutral fixture cases plus third-party glob enumeration produced a real red baseline: 3 failed tests. Publication now measures the completed page directories, returns/logs those actual byte counts and enforces the strict limit after every mutation.

## Final Artifact Audit Input

`🔍️publication/📜️script.ts` is a retained ticket audit input. Invoke it with `bun nx exec --projects=workspace --excludeTaskDependencies -- bun <absolute-script-path> audit` after the fresh output appears. The project/dependency flags keep the audit within one command instead of traversing unrelated package dependencies. It verifies four host CNAMEs, `.nojekyll`, satellite CORS declarations, strict total byte budgets, app HTML/JavaScript, runtime module-closure directories and every declared bridge/Wasm, copied worker equality, literal static/dynamic imports, relative import-meta asset URLs and CSS/HTML references. It uses independent `es-module-lexer` parsing and provides progress/cancellation. Results are written to `📓️published-page-completeness.md`.

The final follow-up focused suite completed with 18 passing tests, including exact real-generator import inventory and independent final-byte accounting.

The isolated Nx audit was executed against the current partial `dist/pages` tree. It rejected the artifact with 11 failures (missing app/metadata/worker and incomplete canonical runtime closure), wrote `📓️published-page-completeness.md`, and emitted `[DEBUG] Publication audit failed`. The current tree has no application scripts. This proves rejection/report generation; it is not completed-release acceptance.
