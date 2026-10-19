import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const p = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
let bad = 0, n = 0;
for (const f of p.fixes) { n++; try { const r = Bun.resolveSync(f.replacement, dirname(resolve(f.from))); if (!r) throw 0; } catch { bad++; console.log("UNRESOLVED", f.from, f.replacement); } }
console.log("checked", n, "unresolved", bad);
