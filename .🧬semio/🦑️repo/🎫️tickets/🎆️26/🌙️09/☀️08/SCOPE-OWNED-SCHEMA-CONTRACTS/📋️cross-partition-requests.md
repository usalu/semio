# Cross-partition request ledger

Coordinator routing of requests raised in worker reports. Status: `open` → `dispatched(<wave>)` → `done`.

| # | From | To (partition) | Request | Status |
|---|------|----------------|---------|--------|
| 1 | wp2-rust-schema-registry | every scope with named exports (plugins, hub, os, framework) | call `register_scope_schema_exports(ScopeSchemaExports{scope, exports})` beside `register_artifact_schema_descriptor`; scope id = module `$id` path form; export ids must not reuse artifact/snapshot/diff/mutations | open |
| 2 | wp2-rust-schema-registry | W2 tooling (root script) | `schema verify` cross-checks `🔣️schema-catalog.json` against `schema_export_catalog_entries()`; ascii format ids, map via `SCHEMA_FORMAT_TAXONOMY_KEYS` | open |
| 3 | wp2-rust-schema-registry | W2 tooling (root script/project/launch) | nx target for `🧰️framework/🔨️modules/🧬️schema/✅️draft07-oracle.test.ts`, part of `schema test` | open |
| 4 | wp2-rust-schema-registry | W2 tooling (root package.json) | declare `ajv` explicitly in devDependencies | open |
| 5 | wp2-rust-schema-registry | W5 os (🌉️mcp) | decide 2020-12 wire label vs draft-07 validator once tree is draft-07 | open |
| 6 | wp4-plugins-conventions | W10 framework modules (🖱️ui) | export `RetainedCommandLimits`, `RetainedCommandRoute`, `RetainedCommandRoutes` at `https://semio.tech/schema/framework/ui/schema.json#/$defs/…` | dispatched(W10 brief) |
