import { readFileSync } from "node:fs";
const law = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts");
for (const f of process.argv.slice(2)) console.log(law.proseSitesV1(readFileSync(f, "utf8")), f.slice(-60));
