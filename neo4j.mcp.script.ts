#!/usr/bin/env bun
/**
 * 🗄️ Cross-platform Neo4j MCP launcher with local Bolt defaults.
 */
import { spawnSync } from "node:child_process";

const [maybeDatabase, ...rest] = process.argv.slice(2);
const database = maybeDatabase && !maybeDatabase.startsWith("-") ? maybeDatabase : process.env.NEO4J_DATABASE || "neo4j";
const args = maybeDatabase && !maybeDatabase.startsWith("-") ? rest : process.argv.slice(2);

const result = spawnSync("uvx", ["mcp-neo4j-cypher", ...args], {
  stdio: "inherit",
  env: {
    ...process.env,
    NEO4J_URI: process.env.NEO4J_URI || "bolt://localhost:7687",
    NEO4J_USERNAME: process.env.NEO4J_USERNAME || "neo4j",
    NEO4J_PASSWORD: process.env.NEO4J_PASSWORD || "password",
    NEO4J_DATABASE: database,
    NEO4J_TELEMETRY: process.env.NEO4J_TELEMETRY || "false",
  },
});

process.exit(result.status ?? 1);
