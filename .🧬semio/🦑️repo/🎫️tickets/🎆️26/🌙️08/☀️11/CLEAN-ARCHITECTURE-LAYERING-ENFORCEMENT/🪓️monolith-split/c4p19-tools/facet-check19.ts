// 🔎️ Checks that each split facet leaf `rust: concat!(include_str!(..), ..)` equals the module-tree text of its split root. usage: bun facet-check19.ts <file> <anchor token before the rust: key>
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { rustStringValue, rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const file = process.argv[2]!, anchor = process.argv[3]!;
const tokens = rustTokens(readFileSync(file, "utf8"));
const at = tokens.findIndex((t) => t.text === anchor);
if (at < 0) throw new Error("anchor not found " + anchor);
const rustAt = tokens.findIndex((t, i) => i > at && t.text === "rust" && tokens[i + 1]?.text === ":");
const paths: string[] = [];
for (let k = rustAt + 2; k < tokens.length; k += 1) {
  if (tokens[k]!.text === "include_str" && tokens[k + 1]?.text === "!" && tokens[k + 2]?.text === "(") paths.push(rustStringValue(tokens[k + 3])!);
  if (tokens[k]!.text === "typescript") break;
}
const facet = paths.map((p) => readFileSync(resolve(dirname(file), p), "utf8")).join("");
const tree = readModuleTreeSource(file);
console.log(JSON.stringify({ files: paths.length, facetBytes: facet.length, treeBytes: tree.length, equal: facet === tree }));
if (facet !== tree) process.exit(1);
