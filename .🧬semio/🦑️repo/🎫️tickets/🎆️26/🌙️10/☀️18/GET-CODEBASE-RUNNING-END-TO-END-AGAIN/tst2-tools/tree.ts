import { readModuleTreeSource } from "../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readFileSync } from "node:fs";
const [root, ...needles] = process.argv.slice(2);
const text = readModuleTreeSource(root.startsWith("/") ? root : process.cwd() + "/" + root);
console.log("treeBytes", text.length);
for (const n of needles) {
  const count = text.split(n).length - 1;
  console.log(count, JSON.stringify(n.slice(0, 120)));
}
