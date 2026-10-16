import { loadTaxonomy, semanticDirectoryKindId } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const taxonomy = loadTaxonomy();
const dirs = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean);
let bad = 0;
for (const dir of dirs) {
  const segs = dir.split("/");
  const chain: (string | null)[] = [];
  let parent: string | undefined;
  const ancestors: string[] = [];
  for (const seg of segs) {
    const id = semanticDirectoryKindId(seg, taxonomy, { parentKindId: parent, ancestorKindIds: [...ancestors].reverse() });
    chain.push(id);
    if (id) { if (parent) ancestors.push(parent); parent = id; }
  }
  const last = chain[chain.length - 1];
  if (!last) bad++;
  console.log((last ? "OK  " : "FAIL") + " " + segs.slice(-2).join("/") + " -> " + last + (last ? "" : "  chain=" + JSON.stringify(chain.slice(-4))));
}
console.log(bad === 0 ? "ALL RESOLVED" : bad + " UNRESOLVED");
