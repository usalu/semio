# A6 — App Schema Catalog Registration

## Approach
Plugin→schema crate deps are cyclic (`schema` ← plugins ← `schema`), so A6 registers all **39** owners centrally in `semio-framework-schema` via `include_str!` of each owner's config/presence leaves — no per-plugin `register_app_schema` call sites and no `catalog-integration` plugin deps.

## Files
- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` — `register_all_app_schema_descriptors()` + table test
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml` — restored cycle-free (empty `catalog-integration` feature kept for parked artifact CatalogIntegration)

## Gate
`DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p semio-framework-schema --lib`
→ **5 passed**, including `app_schema_registry_registers_and_validates_all_thirty_nine_owners`.
