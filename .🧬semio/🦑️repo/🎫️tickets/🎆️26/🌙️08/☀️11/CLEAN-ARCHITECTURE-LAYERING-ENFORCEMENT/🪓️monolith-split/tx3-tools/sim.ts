import { readFileSync, writeFileSync } from "node:fs";
const lib = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const { loadTaxonomy } = await import(`${lib}/🧹️normalization/🔣️taxonomy/🟦️.ts`);
const { matchDirectoryKind } = await import(`${lib}/🧹️normalization/🟦️.ts`);
const [, , taxPath, dirsPath, outPath] = process.argv;
const tax = loadTaxonomy({ repoRoot: "/Users/ueli/Documents/semio", taxonomyPath: taxPath });
const dirs = readFileSync(dirsPath!, "utf8").split("\n").filter(Boolean).sort((a, b) => a.split("/").length - b.split("/").length || (a < b ? -1 : 1));
const kindOf = new Map<string, string>();
const unres: string[] = [], ambig: Record<string, string[]> = {};
const chain = (p: string) => { const rows: string[] = []; let c = p.includes("/") ? p.slice(0, p.lastIndexOf("/")) : ""; while (c) { const k = kindOf.get(c); if (k) rows.push(k); c = c.includes("/") ? c.slice(0, c.lastIndexOf("/")) : ""; } return rows; };
const info: Record<string, { parent?: string; anc: string[] }> = {};
for (const p of dirs) {
  const name = p.slice(p.lastIndexOf("/") + 1);
  const parent = kindOf.get(p.includes("/") ? p.slice(0, p.lastIndexOf("/")) : "");
  const anc = chain(p);
  const r = matchDirectoryKind(name, tax, parent, anc);
  if (r.kind) kindOf.set(p, r.kind.id);
  else { if (r.ambiguous.length > 1) ambig[p] = [...r.ambiguous]; else unres.push(p); info[p] = { parent, anc }; }
}
console.log(`dirs=${dirs.length} unresolved=${unres.length} ambiguous=${Object.keys(ambig).length}`);
writeFileSync(outPath!, JSON.stringify({ unres, ambig, info, kinds: Object.fromEntries(kindOf) }));
