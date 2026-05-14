#!/usr/bin/env bun
/**
 * 🗄️ Cross-platform Neo4j MCP launcher; Bolt defaults to localhost. The graph database is **pinned** to the
 * technology positional (`semio` | `elements` | `coda` | `reuse`) so host `NEO4J_DATABASE` never overrides the wrong DB.
 */
import { spawnSync } from "node:child_process";

const [maybeTechnology, ...rest] = process.argv.slice(2);
const technology = maybeTechnology && !maybeTechnology.startsWith("-") ? maybeTechnology : undefined;
const args = technology ? [...rest] : process.argv.slice(2);
if (technology && !args.includes("--namespace")) {
  args.push("--namespace", technology);
}

const graphDatabase =
  technology && ["semio", "elements", "coda", "reuse"].includes(technology)
    ? technology
    : process.env.NEO4J_DATABASE || "semio";

const result = spawnSync("uvx", ["mcp-neo4j-cypher", ...args], {
  stdio: "inherit",
  env: {
    ...process.env,
    NEO4J_URI: process.env.NEO4J_URI || "bolt://localhost:7687",
    NEO4J_USERNAME: process.env.NEO4J_USERNAME || "neo4j",
    NEO4J_PASSWORD: process.env.NEO4J_PASSWORD || "password",
    NEO4J_DATABASE: graphDatabase,
    NEO4J_TELEMETRY: process.env.NEO4J_TELEMETRY || "false",
  },
});

process.exit(result.status ?? 1);
