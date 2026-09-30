/** ⚖️ W3-GLTF: the taxonomy inventory of each scope under the committed engine (`HEAD`, passed as the first argument) and
 * under the working engine, as finding counts per code, so an engine change is judged by its effect on every owner it touches. */
import { inventoryTaxonomy as current } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const [headEngine, ...named] = process.argv.slice(2);
const taxonomy = JSON.parse(await Bun.file("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json").text()) as { subsetDirectoryOverrides: Record<string, unknown> };
const scopes = [...Object.keys(taxonomy.subsetDirectoryOverrides), ...named];
const { inventoryTaxonomy: head } = (await import(headEngine!)) as { inventoryTaxonomy: typeof current };
const counts = (inventory: ReturnType<typeof current>): Map<string, number> => {
  const found = new Map<string, number>();
  for (const row of inventory.violations) found.set(row.code, (found.get(row.code) ?? 0) + 1);
  return found;
};
for (const scope of scopes) {
  const before = counts(head({ repoRoot: process.cwd(), scope })), after = counts(current({ repoRoot: process.cwd(), scope }));
  const codes = [...new Set([...before.keys(), ...after.keys()])].sort();
  const changed = codes.filter((code) => before.get(code) !== after.get(code)).map((code) => `${code} ${before.get(code) ?? 0}→${after.get(code) ?? 0}`);
  console.log(`${scope}\n  ${changed.length === 0 ? "unchanged" : changed.join("\n  ")}`);
}
