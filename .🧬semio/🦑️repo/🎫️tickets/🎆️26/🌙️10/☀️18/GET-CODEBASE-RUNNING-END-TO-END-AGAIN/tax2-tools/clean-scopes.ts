// 🔬 For each leaf finding, finds the largest ancestor scope whose error violations are all on leaf-finding files. usage: bun clean-scopes.ts <tsv> <inv.json>...
import {readFileSync} from "node:fs";
const leaf = new Set(readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map((l) => l.split("\t")[0]!).filter((p) => !p.includes(": ")));
const viol: {path: string; code: string; severity: string}[] = [];
for (const f of process.argv.slice(3)) viol.push(...JSON.parse(readFileSync(f, "utf8")).violations);
const errs = viol.filter((v) => v.severity === "error" && !leaf.has(v.path));
const under = (p: string, s: string) => p === s || p.startsWith(s + "/");
const out = new Map<string, string[]>();
for (const p of leaf) {
  const parts = p.split("/"); let best = p;
  for (let n = parts.length - 1; n >= 1; n--) { const s = parts.slice(0, n).join("/"); if (errs.some((v) => under(v.path, s))) break; best = s; }
  (out.get(best) ?? out.set(best, []).get(best)!).push(p);
}
const merged = [...out].sort((a, b) => a[0].length - b[0].length);
console.log(merged.length);
for (const [s, ps] of merged) console.log(ps.length, s);
