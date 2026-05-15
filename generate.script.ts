#!/usr/bin/env bun
/**
 * 🧩 Aggregate workspace generation entrypoint (`bun run generate`). Neo4j `.repo/🛂/*.cypher` files come only from the live database via generate.neo4j.script.ts.
 */
import { spawnSync } from "node:child_process";
import { join } from "node:path";

//#region 🧭Constants
const REPO_ROOT = import.meta.dir;
const BUN = process.execPath;
const NEO4J_GENERATE_SCRIPT = join(REPO_ROOT, "generate.neo4j.script.ts");
const TECHNOLOGIES = ["semio", "elements", "coda", "reuse"] as const;
//#endregion 🧭Constants

//#region 🚀Entry
let successes = 0;
let failures = 0;
for (const technology of TECHNOLOGIES) {
  const result = spawnSync(BUN, [NEO4J_GENERATE_SCRIPT, technology], {
    stdio: "inherit",
    cwd: REPO_ROOT,
    env: { ...process.env, NEO4J_DATABASE: technology },
  });
  if (result.status === 0) {
    successes += 1;
  } else {
    failures += 1;
    console.error(`[generate] generate:neo4j (${technology}) exited with status ${result.status ?? "unknown"}.`);
  }
}

if (successes === 0) {
  console.error("[generate] no Neo4j database could be exported; fix Bolt reachability and APOC, then re-run `bun run generate`.");
  process.exit(1);
}

if (failures > 0) {
  console.error(
    `[generate] partial success (${successes} database(s) exported, ${failures} failed). Missing DBs are skipped until they exist on the server.`,
  );
}

console.log(`[generate] Neo4j Cypher export finished (${successes} ok, ${failures} skipped/failed) under .repo/🛂.`);
//#endregion 🚀Entry
