# WP-C4 Hub Framework Port — Progress

## Design
- HubInstance::Documents = HubDocumentAuthority wrapping Arc<db::Database> (backend-agnostic: fs/sqlite/postgres/neo4j).
- DocumentHandshake::ClientFirst + multi-frame welcome + DocumentFrames { echo, relay } on framework DocumentAuthority.
- HubModules::Auth / Directory contribute HubSessionResolver and HubDirectoryDecider via dyn_enum_close.
- Bootstrap compose_hub_server wires document_authority + modules; merges framework router with hub router.
- HT16 document-id projection preserved in HubDocumentAuthority encode path (wire shows client document id; ingress re-keys to v1 composite).

## Status
- PASS. See ticket root `wp-c4.md`.

## Commands
- `cargo check -p semio-hub --lib --features sqlite` Finished (private target)
- `cargo check -p semio-hub --tests --features sqlite` Finished
- fleet-mutex hub: `cargo test -p semio-hub --features sqlite documents::tests` → 4 passed
- fleet-mutex hub: `cargo test -p semio-hub --features sqlite stores::tests::hub_` → 2 passed
- `cargo test -p semio-framework-server --lib gateway::` → 24 passed
