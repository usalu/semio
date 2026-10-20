/** 🧭 Feedback-arc ledger: Eades–Lin–Smyth ordering of every cyclic component and the edges that point upward against it. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { build, report } from "./sim.ts";
const plan = JSON.parse(readFileSync(resolve(import.meta.dir, process.argv[2] ?? "carves.json"), "utf8"));
const mode = (process.argv[3] ?? "runtime") as "runtime" | "all";
const es = build(plan).filter(e => mode === "all" || !e.test);
const { comps } = report(es, mode === "all" ? "all" : "runtime", false);
const w = new Map<string, number>();
for (const e of es) w.set(e.from + ">" + e.to, (w.get(e.from + ">" + e.to) ?? 0) + 1);
let totalFiles = 0;
for (const c of comps) {
  const set = new Set(c);
  const out = new Map<string, Map<string, number>>(), inn = new Map<string, Map<string, number>>();
  for (const [k, n] of w) { const [a, b] = k.split(">") as [string, string]; if (!set.has(a) || !set.has(b)) continue; (out.get(a) ?? out.set(a, new Map()).get(a)!).set(b, n); (inn.get(b) ?? inn.set(b, new Map()).get(b)!).set(a, n); }
  const remaining = new Set(c); const s1: string[] = [], s2: string[] = [];
  const outW = (v: string) => [...(out.get(v) ?? [])].filter(([x]) => remaining.has(x)).reduce((s, [, n]) => s + n, 0);
  const inW = (v: string) => [...(inn.get(v) ?? [])].filter(([x]) => remaining.has(x)).reduce((s, [, n]) => s + n, 0);
  while (remaining.size) {
    let moved = true;
    while (moved) { moved = false; for (const v of [...remaining]) { if (outW(v) === 0) { s2.unshift(v); remaining.delete(v); moved = true; } else if (inW(v) === 0) { s1.push(v); remaining.delete(v); moved = true; } } }
    if (!remaining.size) break;
    let best = "", bd = -Infinity; for (const v of remaining) { const d = outW(v) - inW(v); if (d > bd) { bd = d; best = v; } }
    s1.push(best); remaining.delete(best);
  }
  const order = [...s1, ...s2]; const pos = new Map(order.map((v, i) => [v, i]));
  console.log("\nCOMPONENT", c.length, "bottom->top order (lowest first):");
  [...order].reverse().forEach((v, i) => console.log(String(i).padStart(2), v));
  console.log("feedback edges (upward against the order):");
  const fb = es.filter(e => set.has(e.from) && set.has(e.to) && pos.get(e.from)! > pos.get(e.to)!);
  const by = new Map<string, Set<string>>();
  for (const e of fb) (by.get(e.from + " -> " + e.to) ?? by.set(e.from + " -> " + e.to, new Set()).get(e.from + " -> " + e.to)!).add(e.file);
  for (const [k, v] of [...by].sort((a, b) => b[1].size - a[1].size)) { console.log(String(v.size).padStart(4), "files", k); totalFiles += v.size; }
}
console.log("\nfeedback file-edges total", totalFiles);
