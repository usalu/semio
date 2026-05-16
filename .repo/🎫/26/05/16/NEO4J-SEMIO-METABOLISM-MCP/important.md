# Neo4j `semio-metabolism`

- **MCP:** `neo4j-semio-metabolism` → `bun script.ts dev mcp neo4j semio metabolism` (argv segments joined with `-` for Bolt `NEO4J_DATABASE`; no per-server `env` override).
- **Generate:** `bun ./script.ts generate neo4j semio metabolism` or full `bun run generate` exports APOC `apoc.export.cypher.all` to `.repo/🛂/semio-metabolism.cypher` when the live graph exists.
- **Native setup:** `CREATE DATABASE \`semio-metabolism\` IF NOT EXISTS` (Enterprise-style multi-db); import loop loads `semio-metabolism.cypher` into the resolved default graph like other bundles.

Repo MCP (`search`, `ticket_close`) was not available in this session; ticket closed locally in `ticket.json`.
