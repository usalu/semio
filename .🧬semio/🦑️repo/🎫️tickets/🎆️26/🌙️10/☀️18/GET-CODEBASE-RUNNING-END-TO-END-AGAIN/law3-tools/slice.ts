import { sliceModuleTreeSource } from "../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const [root, a, b] = process.argv.slice(2);
console.log(sliceModuleTreeSource(root.startsWith("/") ? root : process.cwd() + "/" + root, a, b));
