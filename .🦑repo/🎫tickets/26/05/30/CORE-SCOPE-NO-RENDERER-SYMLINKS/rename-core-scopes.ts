#!/usr/bin/env bun
/** @emoji 🏷️ Rename @semio-tech/framework-platform → @semio-tech/framework-platform-core and @semio-tech/framework-playground → @semio-tech/framework-playground-core (skip renderer subpaths). */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../..");
const skipDir = new Set(["node_modules", "dist", "test-results", ".git", "target", ".repo"]);
const exts = /\.(ts|tsx|json|mjs|md)$/;

function walk(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    if (skipDir.has(name)) continue;
    const p = join(dir, name);
    try {
      const st = statSync(p);
      if (st.isDirectory()) out.push(...walk(p));
      else if (exts.test(name) && !name.endsWith("bun.lock")) out.push(p);
    } catch {
      /* ignore */
    }
  }
  return out;
}

function renameCoreScope(content: string, base: string, next: string): string {
  const esc = base.replace(/\//g, "\\/");
  return content.replace(new RegExp(`${esc}(?!(?:\\/renderer|\\/core))`, "g"), next);
}

let changed = 0;
for (const file of walk(root)) {
  const orig = readFileSync(file, "utf8");
  let next = renameCoreScope(orig, "@semio-tech/framework-platform", "@semio-tech/framework-platform-core");
  next = renameCoreScope(next, "@semio-tech/framework-playground", "@semio-tech/framework-playground-core");
  if (next !== orig) {
    writeFileSync(file, next);
    changed++;
  }
}
console.log(`[rename-core-scopes] updated ${changed} files`);
