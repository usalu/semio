# Browser Document Open Transport

Date: 2026-09-04

## Stable boundary

The React OS backbone worker no longer requests a document SocketGrant with an empty legacy body. Every hub document dial now performs the authenticated D1 sequence `DocumentOpenIntentV1 -> DocumentOpenPlanV1 -> DocumentPlanSocketGrantIntentV1 -> SocketGrantReceiptV1`, then uses only the selected surface and the existing SocketGrant subprotocol to connect the credential-free WebSocket.

The worker accepts the plan only when its exact space, document, requested surface, React renderer, artifact schema, and pack-schema hash agree with the opening request. Both JSON responses are streamed behind a 64 KiB Content-Length and incremental-read ceiling with fatal UTF-8 decoding and retained-byte wiping. The plan receipt is present only in the authenticated exchange body; it is absent from the socket URL, protocols, Hello, worker messages, and diagnostics. Cancellation after plan validation prevents the exchange, and the existing reconnect owner obtains a fresh plan and grant for every dial.

The grant actor is pending authority, not readiness. A successful WebSocket upgrade and Welcome do not publish `socket-actor` and cannot activate plugin attachment. Only an exact authenticated `Session.actor` match publishes readiness. A mismatch clears the pending/current actor, leaves readiness false, emits a typed terminal failure, and closes the socket with policy violation. ShellHost rejects its readiness waiter on that failure, so plugin attachment cannot race ahead of Session.

## Language-neutral contract

`🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️browser-document-open-v1.schema.json` and its adjacent JSON fixture describe the exact Unicode paths, plan, receipt exchange, SocketGrant, authoritative Hello fields, 64 KiB response limit, forbidden credential fragments, and eight hostile outcomes. The schema is strict at every public object, including the catalog/package/artifact/surface/grant, checkpoint frontier, and exactly one session/share revalidation generation.

The independent Bun oracle uses Ajv 2020 plus Node URL/Buffer primitives and does not import the production plan parser. It proves three encoded paths, plan authority, the typed receipt-only exchange, credential-free socket/Hello material, all eight hostile vectors, and six forbidden credential fragments.

## Runtime evidence

- Session `9548`: registered Nx `@semio-tech/framework-os:test-quick` focused packet, exit 0, one file, 3 passed and 230 skipped. The laws cover exact D1 sequencing, matching-Session readiness, mismatched-Session terminal clearing, max+1 rejection, cancellation before exchange, and redaction.
- Session `37124`: real headless Chromium loaded the production Vite worker and crossed the production private browser relay to an independent authenticated contract authority. Runtime output was `chromium-worker=1 authenticated-open=1 receipt-exchange=1 credential-free-websocket=1 authoritative-tag7=1 matched-session-activation=1 fragment-cleared=1 passed`. The later Nx stage was externally blocked by a transient duplicate renderer project path; no browser assertion failed.
- Server lane session `95998`, owned independently by the issuer/consume implementation: registered server gate exit 0 with eight exact Rust laws and six independent consume vectors. Focused session `095981` proved descriptor/catalog-generation/directory/checkpoint revalidation before Welcome and during live authority.
- Final registered browser gate session `35692`: active at report draft time after Ajv/independent oracle `hostile=8`, real Chromium runtime, and focused worker 3/3 all passed; the ticket-local Cargo target is compiling the eight exact server laws.

Earlier runtime reds are retained as diagnostic history, not passes. Session `8612` exposed an initialization-order request before proof installation. Session `2101` then localized expiry of the intentionally 15-second proof and 30-second fixture plan during cold Vite graph construction. The permanent runtime now loads the browser graph before beginning the actual bounded proof/plan epoch on the same loopback relay binding; it does not widen production deadlines.

## Ownership and qualifications

The uncached permanent gate is `os-hub:browser-document-open-check`, owned by the existing hub `📜️script.ts`, declared in the hub `📋️project.json`, and launch-seeded as `⚖️gate🌐️document-open-browser🌎️hub` at order `411.1095`. Generated launch freshness and the final registered terminal remain to be recorded after session `35692` completes.

This boundary proves the browser D1 transport and the current server issuer/consume subset. It does not claim native/WGPU document opening, production OIDC, initial artifact creation, or release-wide readiness. Ordinary usable document opening remains contingent on the hub publishing a nonempty verified openable catalog and the remaining master-plan prerequisites.
