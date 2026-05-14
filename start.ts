#!/usr/bin/env bun
/** 🟢 IDE session hook: verify workspace deps; extend here for long-running local services. */
import { existsSync } from "node:fs";
import { join } from "node:path";

const root = import.meta.dir;
process.chdir(root);

if (!existsSync(join(root, "node_modules", "nx", "package.json"))) {
  console.log("[start] node_modules incomplete — run `bun install` and `bun setup.ts` (or platform setup script).");
  process.exit(0);
}

console.log("[start] Workspace session ready (use `bun dev.*.ts` or `bun nx run …` for dev servers).");
