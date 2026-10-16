import { readFileSync } from "node:fs";
import { semanticDirectoryKindId, validateTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
const [taxPath, ...paths] = process.argv.slice(2);
const taxonomy = JSON.parse(readFileSync(taxPath!, "utf8"));
const problems = validateTaxonomy(taxonomy);
console.log("validateTaxonomy problems:", problems.length, problems.slice(0, 3).join(" | "));
for (const path of paths) {
  const ancestors: string[] = []; let parent: string | undefined; const trail: string[] = [];
  for (const part of path.split("/")) {
    const id = semanticDirectoryKindId(part, taxonomy, { parentKindId: parent, ancestorKindIds: [...ancestors].reverse() } as never);
    trail.push(`${part}=${id ?? "UNRESOLVED"}`);
    if (id) { if (parent) ancestors.push(parent); parent = id; }
  }
  console.log((trail.at(-1)!.endsWith("UNRESOLVED") ? "UNRESOLVED " : "ok ") + trail.slice(-3).join(" > "));
}
