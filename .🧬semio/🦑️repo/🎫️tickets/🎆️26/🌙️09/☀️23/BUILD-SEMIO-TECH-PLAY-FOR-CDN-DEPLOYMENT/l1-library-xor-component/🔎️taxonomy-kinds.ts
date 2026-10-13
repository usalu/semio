/** 🔎️ Resolves the semantic directory kind of every segment of the given repository paths, parent by parent with the ancestor chain. */
import { semanticDirectoryKindId, loadTaxonomy } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const taxonomy = loadTaxonomy();
let unresolved = 0;
for (const path of process.argv.slice(2)) {
  const ancestors: string[] = [];
  const kinds = path.split("/").filter(Boolean).map((segment) => {
    const kind = semanticDirectoryKindId(segment, taxonomy, ancestors.length ? { parentKindId: ancestors[ancestors.length - 1], ancestorKindIds: [...ancestors].reverse() } : {});
    if (kind) ancestors.push(kind);
    return `${segment}=${kind ?? "?"}`;
  });
  if (kinds[kinds.length - 1]!.endsWith("=?")) unresolved++;
  console.log(kinds.slice(-3).join(" / "));
}
console.log(`[DEBUG] unresolved-leaves=${unresolved}`);
