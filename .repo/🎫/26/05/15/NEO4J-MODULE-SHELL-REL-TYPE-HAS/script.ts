#!/usr/bin/env bun
/** 🧩 Redirect: Neo4j migrations live in `repo/lib/neo4j-migrate/` — use `bun ./script.ts migrate neo4j` from the monorepo root. */
import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";

let dir = import.meta.dir;
let repoRoot = dir;
for (let i = 0; i < 32; i++) {
  if (existsSync(join(dir, "nx.json"))) {
    repoRoot = dir;
    break;
  }
  const parent = dirname(dir);
  if (parent === dir) break;
  dir = parent;
}
const migrateScript = join(repoRoot, "repo", "lib", "neo4j-migrate", "script.ts");
execFileSync(process.execPath, [migrateScript, ...process.argv.slice(2)], { stdio: "inherit", cwd: repoRoot });
