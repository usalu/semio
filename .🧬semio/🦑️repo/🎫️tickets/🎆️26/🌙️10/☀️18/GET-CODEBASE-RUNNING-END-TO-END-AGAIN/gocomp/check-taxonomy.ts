import {implementationLeafBasenameFinding, semanticDirectoryKindId, loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import {readdirSync, existsSync} from "node:fs";
const cli = "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli";
const t = loadCatalogTaxonomy();
const segs = cli.split("/");
let parent: string | undefined; const anc: string[] = [];
for (const s of segs) {
  const k = semanticDirectoryKindId(s, t, {parentKindId: parent, ancestorKindIds: [...anc]});
  console.log(s, "=>", k);
  if (k) { if (parent) anc.unshift(parent); parent = k; }
}
console.log("cli kind", parent, "ancestors", anc);
let bad = 0, n = 0;
const kinds = new Map<string, number>();
for (const e of readdirSync(`/Users/ueli/Documents/semio/${cli}`, {withFileTypes: true})) {
  if (!e.isDirectory()) continue;
  const f = `${cli}/${e.name}/🐹️.go`;
  if (!existsSync(`/Users/ueli/Documents/semio/${f}`)) continue;
  n++;
  const k = semanticDirectoryKindId(e.name, t, {parentKindId: parent, ancestorKindIds: [...anc]});
  kinds.set(String(k), (kinds.get(String(k)) ?? 0) + 1);
  if (k !== "members-of-members-of-members-of-modules") console.log("KIND", e.name, k);
  const finding = implementationLeafBasenameFinding(f, t);
  if (finding) { bad++; console.log("FINDING", e.name, JSON.stringify(finding)); }
}
console.log({dirsWithLeaf: n, findings: bad, kinds: Object.fromEntries(kinds)});
