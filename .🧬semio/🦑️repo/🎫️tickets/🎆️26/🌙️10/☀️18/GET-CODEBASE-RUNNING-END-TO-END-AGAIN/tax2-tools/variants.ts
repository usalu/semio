// 🔬 Counts rejected scripts accepted after cumulative mechanical variants. usage: bun variants.ts <list.txt>
import {readFileSync} from "node:fs";
import {accepts, REPO} from "./lib.ts";
import {ecmaProgram} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🟨️ecma/📦️packages/🟦️typescript/🟦️.ts";
const paths = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const variants: [string, (s: string) => string][] = [
  ["throw-new", (s) => s.replace(/throw Error\(/g, "throw new Error(")],
  ["import-attributes", (s) => s.replace(/(from\s*"[^"]+")\s*with\s*\{[^}]*\}/g, "$1")],
];
const stats: Record<string, number> = {unparseable: 0};
let acc = 0;
for (const p of paths) {
  let s = readFileSync(`${REPO}/${p}`, "utf8");
  if (accepts(s)) continue;
  const unparse0 = !ecmaProgram(s);
  if (unparse0) stats.unparseable++;
  let hit = "";
  for (const [name, f] of variants) { s = f(s); if (accepts(s)) { hit = name; break; } }
  if (hit) { acc++; stats[hit] = (stats[hit] ?? 0) + 1; }
  else if (unparse0 && ecmaProgram(s)) stats["parseable-after-variants"] = (stats["parseable-after-variants"] ?? 0) + 1;
}
console.log({total: paths.length, accepted: acc, ...stats});
