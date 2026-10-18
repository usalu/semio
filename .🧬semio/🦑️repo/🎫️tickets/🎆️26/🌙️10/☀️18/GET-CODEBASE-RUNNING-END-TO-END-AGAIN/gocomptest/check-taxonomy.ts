import {implementationLeafBasenameFinding, semanticDirectoryKindId, loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import {readdirSync, existsSync} from "node:fs";
const cli = "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli";
const t = loadCatalogTaxonomy();
let parent: string | undefined; const anc: string[] = [];
for (const s of [...cli.split("/"), "🧪️tests"]) {
  const k = semanticDirectoryKindId(s, t, {parentKindId: parent, ancestorKindIds: [...anc]});
  if (k) { if (parent) anc.unshift(parent); parent = k; }
}
console.log("tests kind", parent);
const kinds = new Map<string, number>(); let n = 0, bad = 0;
for (const e of readdirSync(`/Users/ueli/Documents/semio/${cli}/🧪️tests`, {withFileTypes: true})) {
  if (!e.isDirectory()) continue;
  const f = `${cli}/🧪️tests/${e.name}/🐹️.go`;
  if (!existsSync(`/Users/ueli/Documents/semio/${f}`)) continue;
  n++;
  const k = semanticDirectoryKindId(e.name, t, {parentKindId: parent, ancestorKindIds: [...anc]});
  kinds.set(String(k), (kinds.get(String(k)) ?? 0) + 1);
  const finding = implementationLeafBasenameFinding(f, t);
  if (finding) { bad++; console.log("FINDING", e.name, JSON.stringify(finding)); }
}
console.log({dirs: n, findings: bad, kinds: Object.fromEntries(kinds)});
