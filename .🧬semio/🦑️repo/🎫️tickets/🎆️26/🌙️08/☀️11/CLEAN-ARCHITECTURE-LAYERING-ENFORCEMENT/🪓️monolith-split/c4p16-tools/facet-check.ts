// 🔎️ Checks that the descriptor's `diff: FacetLeaves { rust: concat!(include_str!(..), ..) }` equals the module-tree text of the split diff file. usage: bun facet-check.ts <descriptor file> <split diff file>
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { rustStringValue, rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const descriptor = process.argv[2]!, diffFile = process.argv[3]!;
const tokens = rustTokens(readFileSync(descriptor, "utf8"));
const diffAt = tokens.findIndex((token, index) => token.text === "diff" && tokens[index + 1]?.text === ":" && tokens[index + 2]?.text === "semio_framework_schema_registry");
if (diffAt < 0) throw new Error("no diff: FacetLeaves in " + descriptor);
const rustAt = tokens.findIndex((token, index) => index > diffAt && token.text === "rust" && tokens[index + 1]?.text === ":");
const paths: string[] = [];
for (let k = rustAt + 2; k < tokens.length; k += 1) {
  if (tokens[k]!.text === "include_str" && tokens[k + 1]?.text === "!" && tokens[k + 2]?.text === "(") paths.push(rustStringValue(tokens[k + 3])!);
  if (tokens[k]!.text === "typescript") break;
}
const facet = paths.map((path) => readFileSync(resolve(dirname(descriptor), path), "utf8")).join("");
const tree = readModuleTreeSource(diffFile);
console.log(JSON.stringify({ files: paths.length, facetBytes: facet.length, treeBytes: tree.length, equal: facet === tree, facetLines: facet.split("\n").length - 1 }));
if (facet !== tree) process.exit(1);
