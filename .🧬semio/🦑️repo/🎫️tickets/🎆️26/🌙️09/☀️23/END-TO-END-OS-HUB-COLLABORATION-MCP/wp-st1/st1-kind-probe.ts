/** 🔎️ ST1 probe: how the taxonomy classifies plugin-level directory names (existing vs proposed family dirs). */
import { readdirSync } from "node:fs";
import { loadTaxonomy, semanticDirectoryKindId } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const taxonomy = loadTaxonomy();
const existing = readdirSync("/Users/ueli/Documents/semio/✏️s/🔌️plugins").filter((n) => !n.startsWith("."));
const proposed = process.argv.slice(2);
for (const name of [...existing, ...proposed]) {
  const kinds = ["plugins", undefined].map((parentKindId) => semanticDirectoryKindId(name, taxonomy, parentKindId ? { parentKindId } : {}));
  console.log(`${proposed.includes(name) ? "NEW " : "OLD "}${name} -> parent=plugins:${kinds[0]} global:${kinds[1]}`);
}
