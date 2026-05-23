import fs from "node:fs";

const idx = fs.readFileSync("semio/js/index.ts", "utf8");
const head = idx.split("export namespace WasmGraph")[0] ?? idx;
const existing = new Set();
for (const re of [/^\s*export type (\w+)/gm, /^\s*export interface (\w+)/gm, /^\s*export class (\w+)/gm]) {
  let m;
  while ((m = re.exec(head))) existing.add(m[1]);
}
const a = idx.indexOf("export namespace WasmGraph");
const b = idx.indexOf("//#endregion 🧷KitWasmHost");
const chunk = idx.slice(a, b);
const names = new Set();
for (const re of [/^\s*export type (\w+)/gm, /^\s*export interface (\w+)/gm]) {
  let m;
  while ((m = re.exec(chunk))) names.add(m[1]);
}
const lines = [...names]
  .filter((n) => !existing.has(n))
  .sort()
  .map((n) => `export type ${n} = WasmGraph.${n};`);
const ins = `//#region 🧷WasmGraphTypeAliases\n${lines.join("\n")}\n//#endregion 🧷WasmGraphTypeAliases\n`;
const marker = "//#endregion 🧷WasmGraphFlatReexports";
const i = idx.indexOf(marker);
if (i < 0) throw new Error("marker");
const i2 = i + marker.length + 1;
const out = idx.slice(0, i2) + "\n" + ins + idx.slice(i2);
fs.writeFileSync("semio/js/index.ts", out, "utf8");
console.log("inserted type aliases", lines.length);
