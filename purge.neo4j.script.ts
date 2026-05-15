#!/usr/bin/env bun
/** 🧽 Best-effort Neo4j legacy cleanup after schema reload (Bolt + `NEO4J_DATABASE`). */
import { spawnSync } from "node:child_process";

const database = process.env.NEO4J_DATABASE || "semio";
const uri = process.env.NEO4J_URI || "bolt://localhost:7687";
const user = process.env.NEO4J_USERNAME || "neo4j";
const password = process.env.NEO4J_PASSWORD || "password";

const result = spawnSync(
  "cypher-shell",
  ["-a", uri, "-u", user, "-p", password, "-d", database, "--format", "plain", "RETURN 1 AS ok;"],
  { stdio: "inherit" },
);

if (result.status !== 0) {
  console.warn("[purge.neo4j] cypher-shell unavailable or DB unreachable — skip.");
  process.exit(0);
}

console.log("[purge.neo4j] connectivity ok; no additional legacy mutations (noop).");
process.exit(0);
