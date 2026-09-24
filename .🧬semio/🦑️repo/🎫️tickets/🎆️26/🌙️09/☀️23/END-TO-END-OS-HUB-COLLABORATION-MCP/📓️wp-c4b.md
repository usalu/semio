# WP-C4b — Delete Legacy Hub Document Surface

Slice: C4b. Ports: 7760-7769. Private cargo target: `.tmp-ticket/wp-c4b/target`.
Continues C4 (framework port). Peers H1/TC5 not reverted.

## Status

| Item | Status |
|------|--------|
| 1. Delete legacy `/spaces/.../socket/v1` handlers | **PASS** (route + `document_ws_v1`/`handle_ws`/`handle_client_frame` removed) |
| 2. Clients on framework surface only | **PASS** (worker, sync, two-client, collab filter, hub-script) |
| 3. `test quick` + `test long` | **BLOCKED** — hub fleet-mutex held by `h1` since 14:27 (32+ min) stuck in `prebuild_lock_exclusive` while running wasm `trusted-catalog-bootstrap` under the **hub** mutex |
| 4. Live Postgres two-client edit | **BLOCKED** — needs fresh `os-hub` build (mutex) + docker postgres image pull failed (`unexpected end of JSON input`) |
| 5. Live Neo4j two-client edit | **BLOCKED** — same |

## Design (landed)

- Framework path: `GET /scopes/{space}%2F{doc}/document/ws?actor=&surface=`
- Browser auth: `Sec-WebSocket-Protocol: semio.session.v1, <session.v1…>`
- Native auth: `Authorization: Bearer <session>`
- `PolicyEngine::set_authenticated_template("authenticated")` wired from HubAuthModule manifest
- Open-plan + document socket-grants HTTP retained for execution-target / actor_id (not WS auth)
- Directory sockets unchanged

## Evidence (measured)

| Command | Result |
|---|---|
| `cargo check -p semio-hub --lib --features sqlite` | Finished |
| `cargo check -p semio-hub --tests --features sqlite` | Finished |
| `cargo check -p semio-framework-server --lib` | Finished |
| `cargo check -p semio-framework-os --lib` | Finished |
| `bun … script.ts test quick` via hub mutex | Queued behind h1; not started |
| docker compose db4 | Image pull EOF |

Probe ready: `.tmp-ticket/wp-c4b/c4b-framework-ws-two-client.ts` (ports 7760+).

## Files changed

- `framework/.../server/.../gateway` — session protocol credential + document WS protocols + authenticated template wire
- `framework/.../server/.../policy` — `authenticated` auto-grant
- `hub/bootstrap` — deleted legacy document WS surface; restored `compose_hub_server` + shared helpers
- `hub/documents` — module doc
- `hub/tests/bin-unit` — framework URL patterns + `document_socket_request`
- `hub/packages/rust/script.ts` — document WS opens
- `os/store/worker`, `os/store/sync` (+ unit), `os/dev/collaboration` test, `hub/two-client-document`

## Honest gaps

1. Full suite + live DB not executed this slice (hub mutex deadlock on peer H1; docker pull broken).
2. Document socket-grant HTTP remains for open-plan exchange; WS no longer consumes grants.
3. Stale pre-C4b hub binaries must not be used for live proof — rebuild after mutex frees.
