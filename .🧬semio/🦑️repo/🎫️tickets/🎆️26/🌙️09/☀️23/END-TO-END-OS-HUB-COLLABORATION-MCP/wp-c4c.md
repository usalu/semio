# WP-C4c — Hub Suite + Framework Document Socket Clients

Slice: C4c. Ports: 7770-7779. Private cargo target: `.tmp-ticket/wp-c4c/target`.
Continues C4b (legacy document socket deleted). Peers H1/C1 not reverted.

## Status

| Item | Status |
|------|--------|
| 1. Fresh `os-hub` build + `test quick` / `test long` | build PASS; `test quick` queued behind fleet hub mutex (o3b); first quick attempt failed on peer `IoFidelity::Lossless` (later cleared) |
| 2. Framework-ws two-client (sqlite) | **PASS** open+rejoin+late on `/scopes/.../document/ws` (port 7772) |
| 3. Other client harnesses on framework route | **PASS** — two-client e2e on framework route (C1+C4c); HAP/two-author have no legacy document socket; hub-script already on framework |
| 4. Postgres/Neo4j live two-client | **BLOCKED** — Docker daemon EOF (`Cannot connect to the Docker daemon`); no native install |

## Design (inherited)

- Framework path: `GET /scopes/{space}%2F{doc}/document/ws?actor=&surface=`
- Browser auth: `Sec-WebSocket-Protocol: semio.session.v1, <session.v1…>`
- Native auth: `Authorization: Bearer <session>`
- Legacy `/spaces/.../socket/v1` document socket removed by C4b
- Production loopback needs `OS_HUB_BIND=127.0.0.1` + `OS_HUB_ADMIN_SUBJECTS=credential.password.v1:<email>`

## Evidence (measured)

| Command | Result |
|---|---|
| `docker version --format '{{.Server.Version}}'` | FAIL — daemon not reachable |
| `cargo build -p semio-hub --bin os-hub --features sqlite` | PASS (15:32) |
| `cargo build -p semio-hub --bin os-hub --no-default-features --features sqlite,integration-fixtures,native-artifact-execution` | PASS (17:26), binary `.tmp-ticket/wp-c4c/target/debug/os-hub` |
| `bun ./script.ts test quick` (first) | FAIL compile `semio-s-artifact-vcs-vcs` `IoFidelity::Lossless` (peer mid-rename; later `cargo check -p` PASS) |
| `bun ./script.ts test quick` (retry) | queued behind hub mutex (c4c ticket in FIFO after o3b) |
| `c4c-open-only.ts` sqlite port 7772 | **PASS** healthz, sessions, two clients, rejoin, late joiner |
| harness audit legacy `/spaces/.../documents/.../socket/v1` (non-directory) | **0 hits** |
| two-client document e2e source | framework `/scopes/.../document/ws` + `semio.session.v1` (peer C1 refined) |

Captures: `generated/framework-ws-open-only.txt`, `generated/live-sqlite-pass.txt`, `generated/hub-build*.txt`, `generated/docker-version.txt`.

## Files changed

- `hub/tests/two-client-document` — framework document WS URL + session protocols (C4c initial; C1 refined actor-free query)
- `.tmp-ticket/wp-c4c/c4c-framework-ws-two-client.ts` — sqlite driver (edit path flaky under production spawn)
- `.tmp-ticket/wp-c4c/c4c-open-only.ts` — measured open+rejoin+late proof

## Honest gaps

1. Postgres/Neo4j live edit blocked on user (Docker Desktop hung; no native install).
2. Full `test quick`/`test long` counts still pending hub mutex turn after this report snapshot.
3. Full Commands/Ack edit exchange under production spawn timed out waiting for Ack; open+rejoin+late on framework surface is measured. Two-client vitest e2e remains the Commands proof path (peer C1 also driving it).
