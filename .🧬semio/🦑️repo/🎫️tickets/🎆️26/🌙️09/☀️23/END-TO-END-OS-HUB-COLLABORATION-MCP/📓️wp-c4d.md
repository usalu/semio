# WP-C4d — Document Socket Actor Impersonation Fix

Slice: C4d. Ports: 7740-7749 / 6240-6249. Private cargo target: `.tmp-ticket/wp-c4d/target`.
Peers: O3b (policy/collab), C4c (hub suite), M10b (DEBUG logs — left alone).

## Verdict: PASS

## Status

| Item | Status |
|------|--------|
| 0. protocol-change.md for peers | **DONE** |
| 1. Schema: remove identity query params | **DONE** |
| 2. Gateway: derive actor/session from principal | **DONE** |
| 3. Hub authority: use principal + grant, reject without | **DONE** |
| 4. Rust sync client URL | **DONE** |
| 5. TS/browser clients | **DONE** |
| 6. Mock hub in kernel sync tests | **DONE** |
| 7. Negative + positive tests | **DONE** |
| 8. cargo kernel sync | **60 passed** |
| 9. gateway crate tests | **88 lib / 26 gateway::** |
| 10. hub document tests (mutex) | **9 passed** |
| 11. TS parity (C2b) | **7 + 9 passed**; server TS **14** |

## Design

- `Resolved.actor` from hub session resolver via `document_actor_id(secret_digest, true)`.
- `DocumentAuthority::bind_socket` + gateway `DocumentStreamQuery` (`surface`/`resume` only).
- Hub refuses Anonymous / missing actor grant; `submit_frame` rejects foreign envelope actors.
- Clients + mock hub no longer use `?actor=`.

## Evidence

| Command | Result |
|---|---|
| `cargo test -p semio-framework-os-kernel --lib --features sync,ureq sync` | 60 passed |
| `cargo test -p semio-framework-server --lib` | 88 passed |
| `cargo test -p semio-framework-server --lib gateway::` | 26 passed |
| hub mutex `cargo test -p semio-hub --lib --features sqlite documents::` | 9 passed |
| OS TS `test long -t "backbone parity"` | 7 passed |
| OS TS `test long -t "handleHubFrame\|rebootstrap\|outbox\|socket actor"` | 9 passed |
| server TS `test` | 14 passed |

Logs: `.tmp-ticket/wp-c4d/generated/`.

## Files changed

- framework server: policy `Resolved.actor`, gateway bind/query/handle, TS client `DocumentJoin`, closed-ports, instance tests
- hub: `document_actor_id`, `HubDocumentAuthority::bind_socket`, session resolver, bootstrap wrapper, bin-unit/two-client/hub-script URLs
- os: sync `hub_ws_url`, sync-unit mock hub, worker WS URL + session protocol, space-owner FakeHub protocol

## Honest gaps

1. Live two-browser collab e2e owned by O3b — not re-run here.
2. Full hub `test quick/long` owned by C4c — only `documents::` filtered suite run.
3. Share-token document sockets still need a resolver that sets `Resolved.actor` (session path is the collab path).
