// 🔎️ Checks that the split facet leaf `diff: FacetLeaves { rust: concat!(include_str!(..), ..) }` of a descriptor file equals the module-tree text of the split diff root. usage: bun facet-check.ts <descriptor file> <diff root file>
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { rustStringValue, rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const file = process.argv[2]!, root = process.argv[3]!;
const tokens = rustTokens(readFileSync(file, "utf8"));
const at = tokens.findIndex((t, i) => t.text === "diff" && tokens[i + 1]?.text === ":" && tokens[i + 2]?.text === "semio_framework_schema_registry");
if (at < 0) throw new Error("diff leaf anchor not found");
const rustAt = tokens.findIndex((t, i) => i > at && t.text === "rust" && tokens[i + 1]?.text === ":");
const paths: string[] = [];
for (let k = rustAt + 2; k < tokens.length; k += 1) {
  if (tokens[k]!.text === "include_str" && tokens[k + 1]?.text === "!" && tokens[k + 2]?.text === "(") paths.push(rustStringValue(tokens[k + 3])!);
  if (tokens[k]!.text === "typescript") break;
}
const facet = paths.map((p) => readFileSync(resolve(dirname(file), p), "utf8")).join("");
const tree = readModuleTreeSource(root);
console.log(JSON.stringify({ files: paths.length, facetBytes: facet.length, treeBytes: tree.length, equal: facet === tree }));
if (facet !== tree) process.exit(1);
