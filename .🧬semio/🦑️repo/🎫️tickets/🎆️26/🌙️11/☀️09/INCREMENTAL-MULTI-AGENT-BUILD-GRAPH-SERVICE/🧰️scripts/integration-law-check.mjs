import {readFileSync} from "node:fs";
import {languageLawFindingsV1,readLanguageLawSourcesV1} from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts";
const al=JSON.parse(readFileSync("/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json","utf8"));
const f=languageLawFindingsV1(readLanguageLawSourcesV1("/home/deck/semio"),al);
const all=f.length,mine=f.filter(x=>JSON.stringify(x).includes("🏗️build/🕸️graph/"));
console.log("total",all,"graph",mine.length,mine.map(x=>x.path.split("🕸️graph/")[1]).join(" ; "));
