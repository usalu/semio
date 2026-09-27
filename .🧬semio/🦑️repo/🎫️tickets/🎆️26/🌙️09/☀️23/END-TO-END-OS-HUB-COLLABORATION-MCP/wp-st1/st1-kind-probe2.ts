/** 🔎️ ST1 probe: classify stdio-root children and the proposed extensions axis. */
import { readdirSync } from "node:fs";
import { loadTaxonomy, semanticDirectoryKindId } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const taxonomy = loadTaxonomy();
for (const name of [...readdirSync("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio"), "🧩️extensions"]) console.log(name, "parent=stdio:", semanticDirectoryKindId(name, taxonomy, { parentKindId: "stdio" }));
for (const name of process.argv.slice(2)) console.log(name, "parent=plan:", semanticDirectoryKindId(name, taxonomy, { parentKindId: "plan" }));
