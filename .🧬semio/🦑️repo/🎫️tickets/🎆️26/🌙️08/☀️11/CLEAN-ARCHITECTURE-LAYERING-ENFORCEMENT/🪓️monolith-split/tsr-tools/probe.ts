import { readModuleTreeSource, listModuleTree } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const [root, ...needles] = process.argv.slice(2);
const files = listModuleTree(root!);
const src = readModuleTreeSource(root!);
console.log("files", files.length, "bytes", src.length);
for (const n of needles) console.log(src.includes(n) ? "YES" : "NO ", n.slice(0, 100));
