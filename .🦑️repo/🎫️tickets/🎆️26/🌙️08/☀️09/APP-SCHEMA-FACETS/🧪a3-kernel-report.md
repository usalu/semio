# 🧪 A3 Kernel Report — App Schema Facets

Ticket `26/08/09/APP-SCHEMA-FACETS`, wave A3.

## Summary

Typed `DocumentApp::Presence` / `PresenceMutation` with `NoPresence` defaults; `PresencePeer.selection_json` replaced by `presence_pack: Option<Vec<u8>>`; framework schema module gained `AppSchemaDescriptor` / `AppSchemaRegistry` / `register_app_schema_descriptor` (artifact twin) plus empty-or-validate table-driven tests. Plugin apps intentionally untouched (A4/A5).

## Files edited

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🦀️component.rs`
- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`
- `🧰️framework/🔨️modules/🧬️schema/🟦️component.ts`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml`

## DocumentApp

- Added associated types beside config/draft: `Presence` + `PresenceMutation` (DocumentDsl + DocumentPack + Mutation/OpText/OpBinary).
- Added `NoPresence` / `NoPresenceMutation` mirroring `NoConfig` / `NoConfigMutation` (empty DSL/pack; extension `nopres`).
- Updated in-crate `DummyApp` / `TestApp` impls and exports.
- Associated-type defaults were not used (unstable on this toolchain).

## PresencePeer encoding

- Field: `selection_json: Option<String>` → `presence_pack: Option<Vec<u8>>`.
- Same flag bit `1 << 1` in `encode_presence_peer` / `decode_presence_peer`.
- Wire payload: length-prefixed **bytes** (`write_bytes` / `read_bytes`) instead of length-prefixed string.
- `ViewModel.presence_peers_json` name preserved; serde emits base64 under camelCase key `presencePack` (documented on `PresencePeer`). Hub remains a blind relay of pack bytes.
- Call sites updated under framework/os (hub wire tests, host/os `presence_peers_json` tests). No compose/ edits.

## Framework AppSchema

- Regions: `AppSchemaDescriptor`, `AppSchemaRegistry`, `GlobalAppSchemaCatalog` + kernel twin `KernelAppSchemaDescriptor` catalog in spr binary wire.
- Reuses shared `GRAPHQL_STATE_PREAMBLE` / `FacetLeaves` (no second GraphQL preamble).
- TS twin: `AppSchemaDescriptor` + `AppSchemaRegistry` in schema `component.ts`.
- Tests in existing module: `app_schema_registry_is_empty_or_every_registered_owner_validates` (empty OK; ready for A6) and placeholder-owner structure test.
- Artifact `CatalogIntegration` plugin registration stays behind feature `catalog-integration` (empty feature in A3). Plugin `[dev-dependencies]` removed for this wave so `cargo test -p semio-framework-schema` does not require A4/A5 Presence impls; restore after those land.

## Gate results

| Command | Result |
| --- | --- |
| `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-plugin` | **pass** |
| `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-os-kernel` | **pass** |
| `cargo test -p semio-framework-os-kernel presence_peer_binary` | **2 passed** |
| `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p semio-framework-schema` | **5 passed** (incl. AppSchema tests) |

## Notes for later waves

- A4/A5: set `type Presence` / `type PresenceMutation` (or `NoPresence`) on each plugin `DocumentApp`.
- A6: register all 39 owners via `register_app_schema_descriptor`; restore schema plugin `[dev-dependencies]` and wire `catalog-integration` feature deps to re-enable artifact catalog integration tests.
