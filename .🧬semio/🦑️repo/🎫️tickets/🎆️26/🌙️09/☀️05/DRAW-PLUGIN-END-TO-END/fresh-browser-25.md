# Fresh Component Browser Fallback — 2026-11-25

`prepare-fresh-browser-25.ts` takes a completed standalone component, installed emitter, and a ticket-generated output directory. It hashes the component before and after each stage. It invokes the existing genuine `describe-linked-draw.ts` (component header, installed native emitter, strict JSON/Pack pair and Draw guest scenario validation), then `probe-linked-draw-browser.ts` (native transpilation, production module rewrites and Preview2 shims, host shim, and a Node JSPI descriptor comparison against the native semantic pair). Any mismatch, changed input, or nonzero tool exit stops the preparation. After parity it uses the production finalizer and bridge generator to produce delivery JSON and bridge from the fresh semantic pack. It does not produce compiler provenance or change the registered catalog.

Once the native link has finished and closed, from the repo root use:

```sh
T='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END'
repo tools exec --light bun -- "$T/prepare-fresh-browser-25.ts" "$T/🗑️generated/native25/wasm32-wasip2/debug/semio_hub_draw_component.wasm" '.cache/b/p/m/browser/prototype/tools/descriptor-emitter/darwin/semio-plugin-describe' "$T/🗑️generated/fresh-browser-25"
repo tools exec --light bun -- "$T/fresh-draw-proxy-25.mjs" "$T/🗑️generated/fresh-browser-25" 16064
```

Point the existing unchanged journey at `http://127.0.0.1:16064` with its normal locale and folder/PDF arguments. The proxy forwards the actual existing :6064 Vite shell and WebSocket upgrades, replacing only Draw's finalized descriptor, generated bridge, JS, core modules, host shim and the new materializer's vendor imports. Other paths go upstream. Requests for nonexistent mapped files fail closed. It logs each served asset. It does not modify assertions, the stable server, generated catalog, or a build receipt. This is an isolated transport fallback, not a registered publication or proof of native-build provenance. The upstream shell must remain running and its existing bridge must be compatible with the freshly materialized module.

Transport smoke used symlinks in generated storage to the *current already served* module and descriptor, not fresh bytes. Chromium reached OS ready through port 16064; proxy logs confirmed descriptor HEAD/GET, host shim, JS and all three Wasm cores were actually served. The native25 link did not yet exist at the time, so neither the new emitter/transpiler preparation nor the actual fresh browser journey has been tested. The owner must check logs, browser errors and all normal journey assertions, including folder/PDF, before claiming a result.

The fallback now serves every Draw module file from the verified native output directory, including the semantic Pack. A missing file returns HTTP 404. A transport regression fetched a current Pack and confirmed that a missing native compiler receipt did not fall through to the previous release's receipt on the upstream server.
