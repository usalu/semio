import { inventoryTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";
const scope = process.argv[2]!;
const inv = inventoryTaxonomy({ repoRoot: "/Users/ueli/Documents/semio", scope });
const kinds = new Map<string, number>();
for (const e of inv.entries as any[]) kinds.set(e.nodeKind, (kinds.get(e.nodeKind) ?? 0) + 1);
console.log([...kinds].map(([k, n]) => `${k}:${n}`).join(" "));
for (const e of inv.entries as any[]) {
  if (e.nodeKind === "file") continue;
  console.log(JSON.stringify({ p: String(e.sourcePath).split("/").slice(-2).join("/"), k: e.fileKind, o: e.ownerId, s: e.semanticStem, v: (e.violations ?? []).map((x: any) => x.code) }));
}
console.log("violations", inv.violations.length);
