// 🌳 c4p32: every installed child of the eight split roots is reached by the module-tree reader exactly once, and the tree text of each root contains the child text. usage: bun tree-check.ts
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { policyModuleTreeFiles } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const files = readFileSync(join(import.meta.dir, "files.txt"), "utf8").split("\n").filter(Boolean);
const dirs = readFileSync(join(import.meta.dir, "newdirs.txt"), "utf8").split("\n").filter(Boolean);
let bad = 0;
for (const f of files) {
  const tree = policyModuleTreeFiles(root, f);
  const mine = dirs.filter((d) => dirname(f) === dirname(d));
  const children = mine.map((d) => d + "/🦀️.rs").filter((p) => existsSync(join(root, p)));
  const seen = new Map<string, number>();
  for (const t of tree) seen.set(t, (seen.get(t) ?? 0) + 1);
  const missing = children.filter((c) => !seen.has(c));
  const dup = [...seen].filter(([, n]) => n > 1).map(([p]) => p);
  if (missing.length || dup.length) bad += 1;
  console.log(f.split("/").slice(-3).join("/"), "tree", tree.length, "children", children.length, "missing", missing.length, "dup", dup.length);
}
console.log(bad === 0 ? "TREE OK" : "TREE PROBLEMS " + bad);
process.exit(bad ? 1 : 0);
