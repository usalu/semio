# Semio MCP Validation Audit — 2026-11-04

Read-only source audit. Commands ran through repo tools exec --light bun. No implementation files edited.

## Setup and Launch

Root .mcp.json semio entry launches repo dev mcp stdio os with generated specific catalog ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated, folder .🧬semio/🔗space/os-mcp, and scopes workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write. Baseline development Rust MCP entrypoint is ✏️s/🧑‍💻dev/💡️services/🌉️mcp/⌨️entrypoint/🦀️.rs. It installs no service or credential protocols. Root and os AGENTS.md read.

## Existing Validation

Project @semio-tech/framework-os-mcp is owned by 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📋️project.json. Its test target depends on framework-os-mcp-rs build and runs Vitest legacy SDK conformance, modern raw JSON-RPC, hygiene, end-to-end surface, binary resolution, catalog, capability description, and authenticated hub tests. Config includes only mcp/🧪️tests/*/🟦️.ts. Deep Bun tests below stdio are therefore outside that target.

Exact source-only Bun entrypoints, relative to MCP root 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp:

- 📡️stdio/📋️protocol/🧪️tests/🟦️.ts: bounded wire/snapshot JSON corpus, independent Ajv, SQLite JSON, fast-json-stable-stringify oracles.
- 📡️stdio/🚪️gateway/🧪️tests/🟦️.ts: JSON corpus reply/journey replays, official SDK schema validation, real peer process transition and staging cancellation.
- 📡️stdio/🎫️session/🧪️tests/🟦️.ts: session test entrypoint.
- 📡️stdio/🏗️staging/🧪️tests/🟦️.ts: source-only staging target entrypoint.

These are run through repo-owned testing execution, or repo tools exec --light bun -- test <exact path> when parent authorizes the direct source probe.

## Real Process Targets

- @semio-tech/framework-os-mcp:test-native-staging has no build prerequisites and invokes the staging Bun tests.
- @semio-tech/framework-os-mcp:test runs compiled real-process Vitest suite; depends on @semio-tech/framework-os-mcp-rs:build.
- @semio-tech/framework-os-mcp:test-canonical-stdio depends on Rust gateway build and repo-mcp install; launches the installed executable with dev mcp stdio os (semio only), verifies installed frontend identity before/after, real server/discover readiness beyond warming, required capabilities_search and action_prepare, SDK envelopes, and owned shutdown. Policy bound 600000 ms. It creates outputs through SEMIO_TEST_ARTIFACT_DIR, which must be ticket 🗑️generated. Entrypoint 🧪️tests/🚪️canonical/🟦️.ts and matching 🧬️schema/🔣️.json and 🧫️fixtures/🔣️.json.
- @semio-tech/s-dev-composition:mcp-client-e2e drives BOTH .mcp.json servers, including repo MCP. Avoid for strict semio-only scope. Its specific pins use note editor.addBlock / duplicateSelection and wfc bitmap inference.

## Oracle Availability

Installed node_modules/@modelcontextprotocol/sdk/package.json reports 1.32.1; MCP package devDependency is ^1.30.0. SDK Client real handshake is used by legacy-conformance. SDK InitializeResultSchema, ListToolsResultSchema, JSONRPCErrorSchema validate warm gateway output. SDK JSONRPCMessageSchema validates canonical captured protocol. Ajv and SQLite supply independent fixture oracles. Existing package description and suite labels mention 1.30.0 but installed version is newer.

## Scope and Risks

No tests executed in this read-only audit. Run source protocol/gateway/staging tests first, then managed Rust build, real-process conformance, and installed canonical probe. A successful warm initialize/tools empty response alone does not prove native gateway readiness; canonical probe deliberately requires server/discover non-warming result. Existing end-to-end title says 28-tool gateway surface and may need current expected-surface review if functionality changes.
