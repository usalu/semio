#!/usr/bin/env bun
/** 🧽 Purge entrypoint: `neo4j` → `purge.neo4j.script.ts`. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = import.meta.dir;
const sub = process.argv[2];

if (sub === "neo4j") {
  execFileSync("bun", [join(root, "purge.neo4j.script.ts")], { cwd: root, stdio: "inherit" });
  process.exit(0);
}

console.error("[purge] usage: bun ./purge.script.ts neo4j");
process.exit(1);
