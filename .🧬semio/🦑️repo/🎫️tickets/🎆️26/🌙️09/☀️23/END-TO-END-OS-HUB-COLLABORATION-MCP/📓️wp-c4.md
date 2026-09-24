# WP-C4 — Hub Framework Port

Slice of session 9 / ticket `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP`, continuing canonical `26/09/18` (H2 / HT16 / DB3 / DB4 / G16).

## Verdict: PASS

Hub is instance #1 of `semio-framework-server`: `db::Database` sits behind `ServerInstance::Documents`, auth/directory contribute through `HubModules` / deciders / resolvers, bootstrap composes the framework router and merges it with the legacy hub surface.

## What landed

| Port | Implementation |
|---|---|
| `HubInstance::Documents` | `HubDocumentAuthority` over `Arc<db::Database>` |
| Handshake | `DocumentHandshake::ClientFirst`; welcome requires hello |
| Frames | Multi-frame welcome; `DocumentFrames { echo, relay }` on command path |
| Modules | `HubModules::Auth` → `HubSessionResolver`; `HubModules::Directory` → `HubDirectoryDecider` |
| Bootstrap | `compose_hub_server` → `.document_authority` + `.module(...)` + `framework_router.merge(hub router)` |
| HT16 | Single encode door projects wire `document_id` to client id `D`; ingress re-keys hello/advertise frontiers onto `v1:<spaceLen>:<docLen>:<space><document>` |

## Prior-report constraints honored

- **H2**: hub remains the real product; framework server is the generic instance surface it mounts.
- **HT16**: projection + mismatch refusal stay on the authority encode/ingress path (not dropped during the port).
- **DB3 / DB4 / G16**: authority is backend-agnostic via `db::Database` (sqlite feature exercised here; postgres/neo4j remain compile-time backends of the same type — no fs-only coupling in the document port).

## Tests (measured)

| Command | Result |
|---|---|
| `cargo check -p semio-hub --lib --features sqlite` | Finished |
| `cargo check -p semio-hub --tests --features sqlite` | Finished |
| fleet-mutex `cargo test -p semio-hub --features sqlite documents::tests` | **4 passed** (fixture, flat key, client-first, welcome + HT16 wire id) |
| fleet-mutex `cargo test -p semio-hub --features sqlite stores::tests::hub_` | **2 passed** (`hub_documents_port_is_document_authority`, `hub_directory_module_registers_directory_decider`) |
| `cargo test -p semio-framework-server --lib gateway::` | **24 passed** |

Language-agnostic fixture: `hub/documents/fixtures/hub-document-authority-v1` observations include `welcome-frontier-document-id-is-client-document-not-composite-key`.

## Honest gaps (not blocking this slice)

1. Legacy `/spaces/.../socket/v1` axum handlers still exist beside the merged framework router — dual surface until a later delete-the-legacy slice.
2. Full `semio-hub` suite / `os-hub:test*` not re-run here (filtered module tests only).
3. Live postgres/neo4j document sockets not re-probed in this slice (DB3/DB4 gaps on catalog publish remain coordinator/TC territory).

## Key paths

- `hub/documents/` — `HubDocumentAuthority`
- `hub/stores/` — `HubInstance`, `HubModules`, closed deciders/resolvers
- `hub/bootstrap/` — `compose_hub_server`
- `framework/.../server/.../gateway/` — `DocumentHandshake`, welcome hello, `DocumentFrames`
