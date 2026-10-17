/** 📊️ Skipped versus open edges per slice and per target owner. Usage: bun skipsumm.ts <scan.json> */
import { readFileSync } from "node:fs";
import { edgeSkip } from "./skip.ts";
const edges = JSON.parse(readFileSync(process.argv[2], "utf8"));
const sk = new Map<string, number[]>(); const open = new Map<string, number[]>();
for (const e of edges) { const r = edgeSkip(e); const m = r ? sk : open; const k = r ?? e.sourceOwner; const v = m.get(k) ?? [0, 0]; v[e.test ? 1 : 0]++; m.set(k, v); }
console.log("SKIPPED (prod, test)"); for (const [k, v] of sk) console.log(v.join("\t"), k);
let p = 0, t = 0; console.log("OPEN (prod, test)"); for (const [k, v] of [...open].sort((a, b) => b[1][0] - a[1][0])) { p += v[0]; t += v[1]; console.log(v.join("\t"), k.replace(/🧰️framework\/🔨️modules\//g, "F/M/").replace(/🧰️framework\/🛍️products\//g, "F/P/")); }
console.log("open total", p, t);
