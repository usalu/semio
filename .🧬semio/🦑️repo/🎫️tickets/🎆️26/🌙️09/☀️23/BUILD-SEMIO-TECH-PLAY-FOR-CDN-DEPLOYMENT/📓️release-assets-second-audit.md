# Release Assets Second Audit

Read-only audit on 2026-10-07. No source/config changes.

## Confirmed source defect

The deployment resolver relocatePublishedRequestUrl parses absolute same-origin URLs and compares URL.pathname against literal emoji route keys. URL.pathname percent-encodes emojis. An inline Bun reproduction with /🔌️plugin-modules/a.wasm moved the relative string to https://modules.assets.semio-tech.com/🔌️plugin-modules/a.wasm but left the equivalent absolute URL at https://play.semio-tech.com/%F0%9F%94%8C%EF%B8%8Fplugin-modules/a.wasm. This affects Request and URL inputs because installPublishedPageFetch uses input.url/input.href. The copied shard-worker prelude implements the same comparison and shares the defect. Compare safely decoded pathname for route selection while preserving encoded request suffix.

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts and 🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/🟦️.ts.

## Diagnostic worker import gap

retainSameOriginShardWorker copies only shard-worker.js into play. armGuestRuntimeDiagnostics dynamically imports ../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js relative to that copy. The vendor remains on modules satellite. fetch interception does not redirect dynamic import. When diagnostics are armed this import can 404 and the catch silently disables guest diagnostics. Core loadActor imports its explicit moduleUrl and is not directly affected by this relative diagnostic import.

## Artifact limitations

Current dist/pages has 14,441 files under map and one file each under play, media and modules at audit time; no index.html or JavaScript to inspect. These are partial/stale artifacts while another agent builds. Zero missing relative references from this incomplete tree is not a release acceptance result. Existing map _headers only declares Access-Control-Allow-Origin: *. Publication source adds CNAME and .nojekyll to each page and CORS headers to satellites. Static audit cannot prove the CDN consumes _headers or the actual HTTP MIME/CORS behavior; root production harness must verify HTTP responses.
