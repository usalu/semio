# Neo4j `metabolism`

- **MCP:** `neo4j-metabolism` → `bun script.ts dev mcp neo4j metabolism` (Bolt database `metabolism`).
- **Generate:** `bun ./script.ts generate neo4j metabolism` or full `bun run generate` exports APOC `apoc.export.cypher.all` to `.repo/🛂/metabolism.cypher` when the live graph exists.
- **Native setup:** `CREATE DATABASE metabolism IF NOT EXISTS` on multi-db editions; import loop loads `metabolism.cypher` into the resolved default graph like other bundles.

Repo MCP (`search`, `ticket_close`) was not available in this session; ticket closed locally in `ticket.json`.
