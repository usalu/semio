import { selectRustSourceV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const roots = process.argv.slice(2);
const walk = (d: string): string[] => readdirSync(d).flatMap(n => { const p = join(d, n); const s = statSync(p); return s.isDirectory() ? walk(p) : n.endsWith(".rs") ? [p] : []; });
for (const r of roots) for (const f of walk(r)) {
  try { selectRustSourceV1(readFileSync(f, "utf8"), ["feature=x"]); } catch (e: any) { console.log("ERR", f, String(e.message).slice(0, 120)); }
}
console.log("done");
