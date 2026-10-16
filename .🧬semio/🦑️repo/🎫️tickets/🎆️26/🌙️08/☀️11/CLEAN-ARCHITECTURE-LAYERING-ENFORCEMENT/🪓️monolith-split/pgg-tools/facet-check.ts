import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { rustStringValue, rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const parent = process.argv[2]!;
const text = readFileSync(parent, "utf8");
const tokens = rustTokens(text);
const at = tokens.findIndex((token, index) => token.text === "rust" && tokens[index + 1]?.text === ":" && tokens[index + 2]?.text === "concat");
if (at < 0) throw new Error("no rust: concat!(...) facet in " + parent);
const paths: string[] = [];
for (let k = at + 3; k < tokens.length; k += 1) {
  if (tokens[k]!.text === "include_str" && tokens[k + 1]?.text === "!" && tokens[k + 2]?.text === "(") paths.push(rustStringValue(tokens[k + 3])!);
  if (tokens[k]!.text === "typescript") break;
}
const facet = paths.map((path) => readFileSync(resolve(dirname(parent), path), "utf8")).join("");
const tree = readModuleTreeSource(parent);
console.log(JSON.stringify({ files: paths.length, facetBytes: facet.length, treeBytes: tree.length, equal: facet === tree, facetLines: facet.split("\n").length - 1 }));
if (facet !== tree) process.exit(1);
