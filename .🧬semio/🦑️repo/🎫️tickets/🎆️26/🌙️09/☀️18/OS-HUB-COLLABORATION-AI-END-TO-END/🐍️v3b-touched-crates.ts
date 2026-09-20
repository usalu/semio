#!/usr/bin/env bun
/** 📦️ The Cargo packages that own the files this slice edited, so the compile proof is scoped to them.
 *
 * Ownership is the nearest ancestor `📦️packages/🦀️rust/Cargo.toml` of each edited file — the same
 * `manifestFiles` relation `validateRustTaxonomyMounts` uses, since an artifact that ships as its own
 * crate mounts its subtree from its own `[lib]` and never from the plugin's.
 *
 * Usage: `bun 🐍️v3b-touched-crates.ts <file-list.txt>`
 */
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").map((line) => line.trim()).filter(Boolean);
if (files.length === 0) throw new Error("empty file list — the crate projection would silently report zero");

const crates = new Map<string, number>();
const orphans: string[] = [];
for (const file of files) {
  let dir = dirname(join(repoRoot, file));
  let manifest: string | null = null;
  while (dir.startsWith(repoRoot) && dir !== repoRoot) {
    const candidate = join(dir, "📦️packages/🦀️rust/Cargo.toml");
    if (existsSync(candidate)) {
      manifest = candidate;
      break;
    }
    dir = dirname(dir);
  }
  if (manifest === null) {
    orphans.push(file);
    continue;
  }
  const name = /^\s*name\s*=\s*"([^"]+)"/mu.exec(readFileSync(manifest, "utf8"))?.[1];
  if (!name) {
    orphans.push(file);
    continue;
  }
  crates.set(name, (crates.get(name) ?? 0) + 1);
}
console.log(`files=${files.length} crates=${crates.size} orphans=${orphans.length}`);
for (const [name, count] of [...crates].sort((a, b) => b[1] - a[1])) console.log(`${String(count).padStart(4)}  ${name}`);
for (const file of orphans) console.log(`orphan  ${file}`);
console.log("=== -p list ===");
console.log([...crates.keys()].map((name) => `-p ${name}`).join(" "));
