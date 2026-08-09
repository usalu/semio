# A6 — App Schema Catalog Registration

## Done
- Appended `app_schema_descriptor` + `register_app_schema` to all **39** owner config schema leaves
- Added `register_all_plugin_app_schema_descriptors` under CatalogIntegration
- Restored `catalog-integration` feature + plugin `[dev-dependencies]` (32 crates)
- Tests: `app_schema_registry_registers_and_validates_all_thirty_nine_owners` (feature-gated)

## Gate
`DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p semio-framework-schema --features catalog-integration`
