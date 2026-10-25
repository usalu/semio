import { writeFileSync } from "node:fs";
import { policyPackageLanguagePurityBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism/🟦️.ts";
const b = policyPackageLanguagePurityBreaches("/Users/ueli/Documents/semio").filter(x => x.priority === "high");
const kinds: Record<string, number> = {};
for (const x of b) kinds[x.kind] = (kinds[x.kind] ?? 0) + 1;
console.log(b.length, JSON.stringify(kinds));
writeFileSync(process.argv[2]!, b.map(x => `${x.kind}\t${x.scope}\t${x.summary}`).join("\n"));
