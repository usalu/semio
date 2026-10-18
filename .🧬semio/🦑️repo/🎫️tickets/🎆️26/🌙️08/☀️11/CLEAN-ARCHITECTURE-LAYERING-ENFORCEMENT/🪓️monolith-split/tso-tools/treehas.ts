import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
// usage: treehas.ts <root.rs> <needle>...
const [root, ...needles] = process.argv.slice(2);
const t = readModuleTreeSource(root.startsWith("/") ? root : "/Users/ueli/Documents/semio/" + root);
console.log("len", t.length);
for (const n of needles) { const i = t.indexOf(n); console.log(i >= 0 ? "FOUND  " : "MISSING", JSON.stringify(n.slice(0, 100)), i); }
