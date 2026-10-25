import { readFileSync, writeFileSync } from "node:fs";
import { languageLawFindingsV1, readLanguageLawSourcesV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts";
const allowlist = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json", "utf8"));
const findings = languageLawFindingsV1(readLanguageLawSourcesV1("/Users/ueli/Documents/semio"), allowlist);
writeFileSync(process.argv[2]!, JSON.stringify(findings, null, 1));
console.log(findings.length);
