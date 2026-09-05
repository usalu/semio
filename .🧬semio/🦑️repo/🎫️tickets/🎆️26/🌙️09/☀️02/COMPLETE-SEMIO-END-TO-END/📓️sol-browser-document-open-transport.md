# Browser Document Open Transport

Date: 2026-09-04

## Implemented boundary

The browser OS backbone owns every hub-bound document even when the optional Rust worker resolves. It performs the strict D1 sequence:

1. authenticated `DocumentOpenIntentV1` POST to the exact scoped open-plan issuer;
2. strict bounded `DocumentOpenPlanV1` validation;
3. receipt-only `DocumentPlanSocketGrantIntentV1` POST;
4. credential-free WebSocket dial with the returned SocketGrant subprotocol;
5. activation only after an exact authenticated `Session.actor` match.

The plan must equal a caller-supplied, locally verified installed target across every package field (`pluginId`, `packageId`, version, component SHA-256, component BLAKE3, descriptor-byte SHA-256), artifact field (kind, schema, pack-schema hash), and surface field (surface, app, window kind, role, renderer). Missing installed authority terminates before the first issuer request and clears the ShellHost readiness waiter. A mismatched Session clears pending/current actor state, emits a typed failure, and closes with policy violation.

Both issuer responses are streamed behind exact Content-Length and incremental 64 KiB ceilings, fatal UTF-8 decoding, and retained-byte wiping. Cancellation between issue and exchange prevents the exchange. Receipt/grant/session credentials are absent from URL, Hello, ordinary worker messages, and bounded diagnostics.

Browser artifact state, execution-owner state, and BroadcastChannel ownership now use the canonical UTF-8 scope key `v1:<space-bytes>:<document-bytes>:<space><document>`. Local documents use the disjoint `local:v1:<document-bytes>:<document>` namespace. A bare close/send cannot select between two hub spaces with the same document id, so it fails closed instead of crossing authority.

## Neutral contract and independent oracle

The adjacent strict JSON Schema and fixture at `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️browser-document-open-v1.*` define:

- Unicode authority scopes and their independent UTF-8-length-prefixed keys;
- the complete installed package/artifact/surface target;
- exact issuer/exchange/socket paths and authoritative Hello;
- 64 KiB response ceiling and credential-redaction fragments;
- twenty hostile vectors, including every installed-target equality class, wrong scope/schema/surface, max+1, cancellation, URL redaction, and Session mismatch.

The Bun oracle uses Ajv 2020, Node `Buffer.byteLength`, and Node URL behavior. It does not import the production target comparator or runtime-key helper. Current output is:

`browser-document-open-oracle: ajv=1 paths=3 installed-target=1 scope-keys=2 authority=1 exchange=1 websocket=1 rust-worker-bypass=denied hostile=20 bound=65536 redaction=6 passed`

## Runtime and law evidence

- Session `55814`: current server registered script terminal, exit 0. Eight exact default-feature issuer/ledger/exchange/consume laws completed; the final consume-revalidation law passed 1/1.
- Session `73369`: final-source canonical root-routed focused Nx packet, exit 0, one file, 6 passed and 230 skipped. This includes D1 ownership with a resolved Rust worker, same-document/two-space runtime isolation plus local discrimination, exact installed-target equality, issue/exchange/socket sequencing, Session-gated activation, max+1/cancellation/redaction, and missing-installed-target zero-effect rejection.
- Session `88223`: broad OS quick packet reached 235/236. All 40 backbone laws passed; the sole red was the unrelated concurrently moved workflow-pack fixture inventory (`expected >=5, received 0`). This is broad qualified evidence, not a full-suite pass.
- Session `32220`: independent oracle and production Vite/Chromium browser runtime were green. Runtime output was `chromium-worker=1 authenticated-open=1 receipt-exchange=1 credential-free-websocket=1 authoritative-tag7=1 pre-session-activation=0 matched-session-activation=1 fragment-cleared=1 passed`. The wrapper later exited 1 when its first server law returned Cargo status 101 during concurrent server edits.
- Sessions `64576` and `24870`: the exact server catalog law passed 1/1 both with the ordinary environment and with the registered server script's `RUST_MIN_STACK` environment. Registered sessions `91552` and `92200` had already passed the independent oracle, Chromium runtime and 6/6 browser laws before Cargo returned status 101 during concurrent native/plugin compilation. Those ambiguous tails are not counted.
- Session `3993`: the final uncontended registered target again passed the independent oracle, Chromium runtime, and all 6 browser laws. Its server-law preflight then stopped before selecting any server assertion because the current FEM dependency chain referenced the nonexistent doubled path `.../plugin/📦️📦️packages/🦀️rust/Cargo.toml`. The target therefore exited 1 and is recorded as a qualified browser boundary with an external workspace-manifest blocker, not a complete-wrapper pass.
- Server lane session `95998` and focused session `095981`, owned by the issuer/consume implementation, were independently reported green: eight exact server laws, six consume vectors, and stale descriptor/catalog-generation/directory/checkpoint revalidation before Welcome and on live authority ticks.

Earlier sessions `52675`, `35692`, `37124`, `2101`, and `8612` record superseded versions of the same boundary. They are not counted as final-source acceptance.

## Permanent ownership

The uncached target is `os-hub:browser-document-open-check`, declared in the hub `📋️project.json` and implemented only in the existing hub `📜️script.ts`. Launch seed entry `⚖️gate🌐️document-open-browser🌎️hub` uses the canonical root router:

`bun ./📜️script.ts nx run os-hub:browser-document-open-check --skip-nx-cache`

Session `85875` regenerated the plugin/launch catalog and exited 0. Session `72405` independently confirmed the generated catalog and launch bytes are fresh. Seed and generated launch both contain the same canonical root-routed browser command adjacent to the native gate.

After the final terminal was captured, the validated 3.7 GiB ticket-local `browser-document-open-sol-target` compiler tree was deleted. Sibling generated targets and concurrent reports were preserved.

## Explicit real-hub blocker

This is a synthetic-browser plus real-server-law boundary, not a real-hub browser success claim. Current production startup calls `linked_native_codec_bindings()`, whose implementation is still `Vec::new()`. Consequently configured trusted-catalog loading has no linked first-party codec binding, `artifact_authority` cannot become a usable verified open-target authority, and `open_plan_ready` remains false in an ordinary hub.

No fake binding, unsigned target, environment bypass, direct SocketGrant fallback, or production OIDC claim was added. A real browser-to-real-hub success journey remains blocked on the P0 first-party codec binding/catalog bootstrap. The browser side now fails closed until that producer supplies the complete verified installed target.
