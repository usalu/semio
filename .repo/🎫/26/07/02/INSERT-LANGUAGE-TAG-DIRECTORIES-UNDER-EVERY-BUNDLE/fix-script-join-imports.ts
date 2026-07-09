#!/usr/bin/env bun
import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", "target", "dist", ".venv", ".git", ".cursor", ".repo"]);

function walk(d: string, out: string[] = []): string[] {
  for (const n of readdirSync(d)) {
    if (SKIP.has(n)) continue;
    const p = join(d, n);
    if (statSync(p).isDirectory()) walk(p, out);
    else if (n === "script.ts") out.push(p);
  }
  return out;
}

for (const file of walk(REPO)) {
  let c = readFileSync(file, "utf8");
  if (!c.includes("join(this.root")) continue;
  if (c.includes('from "node:path"') || c.includes("from 'node:path'")) continue;
  c = c.replace(/^(import .+\n)+/m, (m) => `${m}import { join } from "node:path";\n`);
  writeFileSync(file, c);
  console.log(file.replace(REPO + "/", ""));
}
