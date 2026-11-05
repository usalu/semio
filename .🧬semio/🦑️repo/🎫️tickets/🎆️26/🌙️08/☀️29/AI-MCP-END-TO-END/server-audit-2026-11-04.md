server audit: Runtime at framework OS modules/🌉️mcp. Rust package semio-framework-os-mcp produces semio-os-mcp; entrypoint 🏗️bootstrap/⌨️entrypoint/🦀️.rs delegates run_mcp_entrypoint(Vec::new(), Vec::new()). Protocol dispatcher 🧭️protocol/🎯️mcp-server/🦀️.rs; stdio 🚚️transport/🎯️stdio-transport/🦀️.rs. Gateway split into 23 Rust packages listed in module 🟦️.ts.

Observed blocker: .mcp.json passes --catalog ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated but directory is absent. takeMcpPluginCatalog consumes flag into SEMIO_PLUGIN_CATALOG; Rust configured_native_catalog_v1().expect and RegistryDiscovery expect may reject it. Runtime failure not executed. Staged debug binary absent; launcher supports asynchronous staging and warming handshake.

Schemas: module 🧬️schema/🔣️.json mirrors native schemas output. Canonical protocol test 🧪️tests/🚪️canonical/🟦️.ts uses adjacent language-neutral fixtures/schema plus official MCP SDK, Ajv and SQLite independent oracles. Tests actual installed frontend/native readiness, nonempty required tools, bounded requests/notifications, owned shutdown. Requires issued invocation and lease through owning root target.

Concrete verification targets via repo run @semio-tech/framework-os-mcp-rs:check, :test, :schema-mirror-check, :capability-audit-check, :installed-service-check. No builds/tests executed in read-only exploration.

Suggested assignment: Repair catalog publication/dependency and prove actual installed initialize/native readiness/tools/list plus tool call. Preserve schema-first oracles, avoid fallback fixture catalogs. README/Rust root docs contain stale architecture/distribution claims.
