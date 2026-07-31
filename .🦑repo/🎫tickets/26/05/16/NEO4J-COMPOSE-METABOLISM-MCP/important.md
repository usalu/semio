# Neo4j extra graphs (generic)

- **Product graphs (code):** `compose`, `elements`, `coda`, `reuse` — `NEO4J_PRODUCT_GRAPH_DATABASE_SPECS` in `generate.neo4j.gen.ts`.
- **Extra Bolt graphs (env):** `NEO4J_EXTRA_GRAPH_DATABASES=comma,separated,names` — included in `bun run generate`, native `CREATE DATABASE`, `.repo/🛂/<name>.cypher` stubs, and devcontainer post-start reload.
- **MCP `neo4j-extra`:** set **`NEO4J_EXTRA_GRAPH_DATABASE`** to one Bolt graph name; server runs `… mcp neo4j` with `NEO4J_DATABASE` from that env.
