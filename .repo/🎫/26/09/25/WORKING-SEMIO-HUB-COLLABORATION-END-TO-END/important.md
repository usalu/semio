# 🌐 Hub Collaboration — Coordination Contract

This file is the single source of truth shared by all parallel agents of this ticket. Read it before touching code and re-read it when you are unsure. Keep it updated (append-only notes under `## Notes` with your agent role prefix).

## Architecture decisions (final)

1. `semio/client/lib/rs` (crate `semio`) is the **sole owner of domain logic**. The hub MUST NOT keep its own parallel domain model (the old per-entity `core.*` tables, `DomainCommand` enum, `EntityChange`, `FieldPatch`, … are removed). The hub embeds the native `semio` crate and keeps **one authoritative rs kit store per hub session** (same GraphQL control plane as the WASM worker and `semio-store`).
2. Collaboration = **operation replication**: clients send GraphQL mutation documents (operations) to the hub, the hub executes them in order on its authoritative store, assigns a monotonically increasing `version`, persists them, and broadcasts them. Other clients replay the same operation on their local (WASM) store. Every broadcast carries the resulting kit `hash`; on local/hub hash mismatch the client resyncs from `GET /sessions/{id}/kit` (+ `installProjection`). Operations that create entities MUST carry client-generated ids so replay is deterministic.
3. Persistence (Postgres, schema created by the hub at startup): persons, credentials, tokens, sessions, members, shares, operations log, periodic kit snapshots (for fast load + lookback), no per-entity tables.
4. Presence lives in the hub (in-memory per session, mirrored to `semio.person` for identity only) and is broadcast over the session websocket. AI agents (MCP) appear as participants with `kind: "agent"`.
5. The **semio MCP** for collaborative AI is served by the hub at `POST /mcp` (MCP Streamable HTTP, JSON responses, JSON-RPC 2.0), authenticated with a person bearer token. Tool calls execute operations on the authoritative session store exactly like a client, so every AI change is broadcast live to all collaborators. The local engine MCP (`semio/client/bin/engine`) keeps working for local kits.
6. Default ports: hub `8080` (`LISTEN_ADDR`), sketchpad dev `5173`, semio-store `4010`. Local dev DB: `postgres://semio:semio@localhost:5432/semio`.

## Hub Protocol v1 (all JSON bodies camelCase)

### Auth
- `POST /auth/register {name, email, password}` → `201 {token, person}`
- `POST /auth/login {email, password}` → `200 {token, person}`
- `GET /auth/me` (Bearer) → `{person}`
- `POST /auth/logout` (Bearer) → `204`
- `POST /auth/tokens {label}` (Bearer) → `201 {token, label, kind: "agent"}` — long-lived agent token for MCP clients; participants using it appear as `kind: "agent"`.
- `person = {id, name, email, color}`; tokens are opaque strings; `Authorization: Bearer <token>`; websocket uses `?token=<token>`.

### Sessions (a session = one shared kit)
- `GET /sessions` (Bearer) → `[{id, name, role, owner: {id, name}, version, hash, updatedAt, participantCount}]`
- `POST /sessions {name, kit?}` (Bearer) → `201 {id, name, role: "owner", version, hash}` — `kit` is a kit projection JSON (same format `installProjection` accepts); omitted → empty kit named `name`.
- `GET /sessions/{id}` → `{id, name, role, owner, version, hash, updatedAt, participantCount}`
- `DELETE /sessions/{id}` (owner) → `204`
- `GET /sessions/{id}/kit` (member or share token) → `{version, hash, kit}`
- `POST /sessions/{id}/operations {operationId, clientId, baseVersion, query, variables?}` (owner/editor) → `200 {version, hash, data}` | `4xx {error}`; idempotent on `operationId`.
- `POST /sessions/{id}/graphql {query, variables?}` (any member) → raw GraphQL response; read-only (mutations rejected with 400, use `/operations`).
- `GET /sessions/{id}/operations?after=<version>` → `[{version, operationId, clientId, personId, participantKind, query, variables, hash, createdAt}]`
- `GET /sessions/{id}/kit/at/{version}` → `{version, hash, kit}`
- `GET /sessions/{id}/members` → `[{person, role}]`; `POST /sessions/{id}/shares {role: "editor"|"viewer", label?}` (owner) → `201 {token, role}`; `GET /sessions/{id}/shares`; `DELETE /sessions/{id}/shares/{token}`; `POST /shares/{token}/join` (Bearer) → `{session, role}` (adds membership).
- `GET /sessions/{id}/presence` → `[participant]`
- Roles: `owner` > `editor` > `viewer`.

### Websocket `GET /sessions/{id}/ws?token=<token>&clientId=<uuid>&client=<sketchpad|mcp|engine|...>`
Server → client (`type` discriminated):
- `welcome {self: participant, participants: [participant], version, hash}`
- `presence.joined {participant}` / `presence.left {participantId}` / `presence.updated {participant}`
- `operation {version, hash, operationId, clientId, participantId, personId, query, variables}` (sent to everybody, including the origin, so the origin can confirm)
- `error {message}`
Client → server:
- `presence {focus?: {app, designId?, typeId?}, selection?: {designId?, pieceIds, connectionIds}, cursor?: {x, y, space}}` (partial updates merge)
- `operation {operationId, baseVersion, query, variables?}` (same semantics as the POST)
- `ping` → `pong`
`participant = {id, personId, name, color, kind: "human"|"agent", client, focus?, selection?, cursor?, joinedAt}`; on socket close the participant is removed and `presence.left` broadcast.

### MCP `POST /mcp` (Bearer; `initialize`, `tools/list`, `tools/call`, `resources/list`, `resources/read`, `prompts/list`, `ping`)
Minimum tool set: `list_sessions`, `create_session`, `read_kit`, `list_designs`, `read_design`, `list_types`, `create_design`, `add_piece`, `connect_pieces`, `update_piece`, `remove_pieces`, `remove_connections`, `list_participants`, `get_history`, `run_query` (read-only GraphQL), `run_operation` (GraphQL mutation as an operation). Every mutating tool = one hub operation (broadcast). While an MCP client works on a session it is a presence participant `kind: "agent"`, `client: "mcp"`, removed after 120 s idle.

## Agent roles
- **H (hub)**: `semio/server/hub/**` (bin.rs, Cargo.toml, compose.yml, k8s.yaml, postgres/schema.sql).
- **M (mcp)**: engine MCP `semio/client/bin/engine/**`, root `pyproject.toml`/`uv.lock`, and the hub `mod mcp` region inside `semio/server/hub/bin.rs` (coordinate with H: H owns everything else in bin.rs).
- **F1 (frontend runtime)**: `@semio/js`, `@semio/react`, `@semio/sketchpad`, framework/ui fixes needed for the sketchpad to build, run and pass tests with all apps/plugins/artifacts.
- **F2 (collaboration client)**: hub client in `@semio/js` (region `🌐Hub`), re-exports in `@semio/react`, hub/presence/share/AI-connect UI in `@semio/sketchpad` (and `ui/react` only if a primitive is missing).
- Shared files (`semio/client/lib/js/index.ts`, `semio/client/lib/sketchpad/js/index.ts`, `.vscode/launch.json`): edit only your own regions, re-read before each edit, never revert others' changes.

## Rules for all agents
- Follow `/home/user/semio/AGENTS.md` (regions, concise code, emoji docstrings, no new files outside this ticket folder, extend existing test files, `[DEBUG] ` prefix for temp logs).
- NEVER run modifying git commands (commit/stash/checkout/reset/restore). The coordinator commits.
- Put scratch files/logs in this ticket folder only.
- Validate every claim by running it (tests, curl, Playwright, console logs).

## Notes
