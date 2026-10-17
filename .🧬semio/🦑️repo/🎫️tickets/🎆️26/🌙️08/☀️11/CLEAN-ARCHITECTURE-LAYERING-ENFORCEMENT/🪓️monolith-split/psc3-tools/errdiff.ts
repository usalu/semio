/** 🧮️ Diffs two cruiser err outputs as sets of (rule, from, to) lines per rule. Keys are ASCII projections because the cruiser output has mojibake in emoji. Usage: bun errdiff.ts <before.txt> <after.txt> [rule-substring] */
import { readFileSync } from "node:fs";
const [a, b, filter] = process.argv.slice(2);
const ascii = (s: string) => s.replace(/[^\x20-\x7e]/gu, "");
const parse = (p: string) => { const m = new Map<string, string>(); for (const l of readFileSync(p, "utf8").split("\n")) { const x = l.match(/^  error (.*)$/u); if (x) m.set(ascii(x[1]), x[1]); } return m; };
const A = parse(a), B = parse(b);
const removed = new Map<string, string[]>(), added = new Map<string, string[]>();
for (const [k, v] of A) if (!B.has(k)) { const r = k.split(": ")[0]; (removed.get(r) ?? removed.set(r, []).get(r)!).push(v); }
for (const [k, v] of B) if (!A.has(k)) { const r = k.split(": ")[0]; (added.get(r) ?? added.set(r, []).get(r)!).push(v); }
const fam = (r: string) => r.startsWith("no-cross-package-relative-") ? "no-cross-package-relative-*" : r;
const sum = new Map<string, [number, number]>();
for (const [r, l] of removed) { const v = sum.get(fam(r)) ?? [0, 0]; v[0] += l.length; sum.set(fam(r), v); }
for (const [r, l] of added) { const v = sum.get(fam(r)) ?? [0, 0]; v[1] += l.length; sum.set(fam(r), v); }
for (const [r, v] of [...sum].sort()) console.log(`${r}: removed ${v[0]}, added ${v[1]}`);
if (filter) for (const [r, l] of added) if (r.includes(filter) || fam(r).includes(filter)) for (const k of l) console.log("  + " + k.slice(0, 320));
