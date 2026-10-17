// 🧾 c4p22: the Rust facet of a schema descriptor (`rust: concat!(include_str!(..), ..)`) must equal the module tree text of the diff root (`readModuleTreeSource`). usage: bun facetcheck.ts <descriptor.rs> <diff-root.rs>
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const [descriptor, diffRoot] = process.argv.slice(2) as [string, string];
const text = readFileSync(descriptor, "utf8");
const line = text.split("\n").find((l) => l.includes("rust: concat!(") && l.includes("🔺️diff/🦀️.rs"));
if (!line) throw new Error("no diff rust concat! facet in " + descriptor);
const paths = [...line.matchAll(/include_str!\("([^"]+)"\)/g)].map((m) => m[1]!);
const facet = paths.map((p) => readFileSync(resolve(dirname(descriptor), p), "utf8")).join("");
const tree = readModuleTreeSource(diffRoot);
console.log(JSON.stringify({ files: paths.length, facetBytes: facet.length, treeBytes: tree.length, equal: facet === tree }));
if (facet !== tree) process.exit(1);
