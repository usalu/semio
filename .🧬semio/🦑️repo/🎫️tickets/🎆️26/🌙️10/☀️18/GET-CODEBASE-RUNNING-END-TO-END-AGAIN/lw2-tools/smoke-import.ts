import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
let ok = 0; const bad: string[] = [];
for (const f of files) {
  if (!/\.(ts|tsx)$/u.test(f)) continue;
  try { await import(resolve("/Users/ueli/Documents/semio", f)); ok++; }
  catch (e) { const m = String((e as Error).message).slice(0, 200); bad.push(f.slice(-70) + " :: " + m); }
}
console.log("loaded", ok, "failed", bad.length); for (const b of bad) console.log("  " + b);
