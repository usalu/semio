import { readModuleTreeSource } from "../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const [, , file, ...needles] = process.argv;
const t = readModuleTreeSource(file!).replace(/\s+/g, "");
for (const n of needles) console.log(t.includes(n.replace(/\s+/g, "")) ? "OK  " : "MISS", n.slice(0, 100));
