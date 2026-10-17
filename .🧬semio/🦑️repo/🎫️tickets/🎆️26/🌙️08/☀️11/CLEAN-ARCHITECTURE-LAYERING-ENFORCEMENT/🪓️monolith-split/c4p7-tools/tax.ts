// 🏷️ c4p7: resolves directories (existing or hypothetical) through the live engine function semanticDirectoryKindId. usage: bun tax.ts <dir list file> [verbose]
import {readFileSync} from "node:fs";
import {loadTaxonomy, semanticDirectoryKindId} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const taxonomy = loadTaxonomy();
const dirs = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const kindOf = new Map<string, string | null>();
const resolve = (path: string): string | null => {
  if (kindOf.has(path)) return kindOf.get(path)!;
  const parts = path.split("/");
  const name = parts[parts.length - 1]!;
  const chain: string[] = [];
  for (let i = parts.length - 1; i >= 1; i -= 1) { const k = resolve(parts.slice(0, i).join("/")); if (k) chain.push(k); }
  const kind = semanticDirectoryKindId(name, taxonomy, {parentKindId: chain[0], ancestorKindIds: chain.slice(1)});
  kindOf.set(path, kind);
  return kind;
};
const unresolved: string[] = [];
for (const dir of dirs) { if (resolve(dir) === null) unresolved.push(dir); }
if (process.argv[3]) for (const d of dirs) console.log(d.split("/").slice(-2).join("/"), "=>", kindOf.get(d));
console.log(JSON.stringify({dirs: dirs.length, unresolved: unresolved.map(u => u.split("/").slice(-3).join("/"))}, null, 1));
