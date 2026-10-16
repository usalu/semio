/** 🧮️ Diffs two cruiser err outputs as sets of (rule, from, to) lines, grouped by rule. Usage: bun errdiff.ts <before.txt> <after.txt> [rule-filter] */
import { readFileSync } from "node:fs";
const [a, b, filter] = process.argv.slice(2);
const parse = (p: string) => { const m = new Map<string, number>(); for (const l of readFileSync(p, "utf8").split("\n")) { const x = l.match(/^  error (.*)$/u); if (x) m.set(x[1].normalize("NFC"), (m.get(x[1].normalize("NFC")) ?? 0) + 1); } return m; };
const A = parse(a), B = parse(b);
const removed = new Map<string, string[]>(), added = new Map<string, string[]>();
for (const k of A.keys()) if (!B.has(k)) { const r = k.split(": ")[0]; (removed.get(r) ?? removed.set(r, []).get(r)!).push(k); }
for (const k of B.keys()) if (!A.has(k)) { const r = k.split(": ")[0]; (added.get(r) ?? added.set(r, []).get(r)!).push(k); }
const rules = new Set([...removed.keys(), ...added.keys()]);
for (const r of [...rules].sort()) {
  if (filter && !r.includes(filter)) continue;
  console.log(`${r}: removed ${removed.get(r)?.length ?? 0}, added ${added.get(r)?.length ?? 0}`);
}
if (filter) { for (const k of added.get([...rules].find(r => r.includes(filter)) ?? "") ?? []) console.log("  + " + k.slice(0, 300)); }
