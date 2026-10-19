// 🔬 Joins the 151 leaf findings with planner moves and checks collisions. usage: bun leaf-targets.ts <tsv> <inv.json>...
import {readFileSync, existsSync} from "node:fs";
const tsv = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map((l) => l.split("\t")[0]!).filter((p) => !p.includes(": "));
const moves = new Map<string, any>(), viol = new Map<string, any[]>();
for (const f of process.argv.slice(3)) { const j = JSON.parse(readFileSync(f, "utf8")); for (const m of j.moves) moves.set(m.from, m); for (const v of j.violations) (viol.get(v.path) ?? viol.set(v.path, []).get(v.path)!).push(v.code); }
let withMove = 0; const tos = new Map<string, string[]>(); const rows: any[] = [];
for (const p of tsv) { const m = moves.get(p); if (m) withMove++; rows.push({p, to: m?.to ?? null, exists: m ? existsSync(m.to) : null, viol: viol.get(p) ?? []}); if (m) (tos.get(m.to) ?? tos.set(m.to, []).get(m.to)!).push(p); }
console.log(JSON.stringify({total: tsv.length, withMove, dupTargets: [...tos].filter(([, v]) => v.length > 1).length, occupied: rows.filter((r) => r.exists).length}));
for (const r of rows) console.log(r.exists ? "OCCUPIED" : r.to ? "free    " : "NOMOVE  ", r.p.slice(-110), "=>", r.to ? r.to.slice(-60) : "-", r.viol.join(","));
