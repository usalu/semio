// [DEBUG] probe: walks a directory with the real taxonomy and reports directory-kind violations
import { readdirSync, statSync } from "node:fs";
import { join, dirname } from "node:path";
import { loadTaxonomy } from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔣️taxonomy/🟦️.ts";
import { canonicalDirectory } from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/📏️fixed-contracts/🟦️.ts";
import { ancestorDirectoryKindIds } from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/👥️mutation-projection/🟦️.ts";
const repoRoot = "/home/deck/semio";
const start = process.argv[2]!;
const taxonomy = loadTaxonomy({ repoRoot } as never);
const kinds = new Map<string, string>(), fixed = new Map<string, string>(), canon = new Map<string, string>();
const dirs: string[] = [];
const parts = start.split("/");
for (let i = 1; i <= parts.length; i++) dirs.push(parts.slice(0, i).join("/"));
function walk(rel: string): void {
  for (const name of readdirSync(join(repoRoot, rel)).sort()) {
    if (name === "node_modules" || name.startsWith(".")) continue;
    const p = rel + "/" + name;
    if (statSync(join(repoRoot, p)).isDirectory()) { dirs.push(p); walk(p); }
  }
}
walk(start);
let bad = 0;
for (const path of dirs) {
  const parentCanonical = canon.get(dirname(path)) ?? "";
  const parentCtx = kinds.get(dirname(path)) ?? fixed.get(dirname(path));
  const c = canonicalDirectory(path, parentCanonical, parentCtx, ancestorDirectoryKindIds(path, kinds), taxonomy);
  canon.set(path, c.path);
  if (c.kindId) kinds.set(path, c.kindId);
  if (c.fixedId) fixed.set(path, c.fixedId);
  for (const v of c.violations) { bad++; console.log("VIOLATION", v.id ?? v.code ?? "", path.replace(start, "~"), v.message ?? ""); }
}
console.log("dirs", dirs.length, "violations", bad);
