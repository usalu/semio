import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const d = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const ROOT = "/Users/ueli/Documents/semio/";
let ok = 0; const bad: string[] = [];
for (const f of d.fixes) {
  try { const r = Bun.resolveSync(f.replacement, dirname(ROOT + f.from)); if (!r.startsWith(ROOT) && !r.includes("/Documents/semio")) bad.push(`outside ${f.replacement}`); else ok++; } catch (e) { bad.push(f.from + " -> " + f.replacement + " " + String(e).slice(0, 80)); }
}
console.log("resolved", ok, "bad", bad.length, bad.slice(0, 10));
