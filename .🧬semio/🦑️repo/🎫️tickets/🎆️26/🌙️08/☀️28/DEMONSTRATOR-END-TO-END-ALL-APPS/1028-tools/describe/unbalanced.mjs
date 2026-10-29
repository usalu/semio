import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const { rustTokens, rustTokenPairs } = await import(process.argv[2]);
const walk = d => readdirSync(d).flatMap(n => { const p = join(d, n); if (n === "target" || n === "node_modules") return []; return statSync(p).isDirectory() ? walk(p) : p.endsWith(".rs") ? [p] : []; });
for (const f of walk(process.argv[3])) {
  try {
    const t = rustTokens(readFileSync(f, "utf8")), p = rustTokenPairs(t);
    const bad = t.findIndex((x, i) => ["(", ")", "[", "]", "{", "}"].includes(x.text) && !p.has(i));
    if (bad >= 0) console.log("UNBALANCED", f, "token", t[bad].text, "at", t[bad].start);
  } catch (e) { console.log("ERR", f, e.message); }
}
