/** 🧭️ Error lines whose target is a facade or package file created or extended by this slice (the new nodes), before versus after. Usage: bun attrib2.ts <before.txt> <after.txt> */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const nfc = (s: string) => s.normalize("NFC");
const [a, b] = process.argv.slice(2);
const HERE = fileURLToPath(new URL("..", import.meta.url));
const plan = JSON.parse(readFileSync(HERE + "🗑️generated/psc-plan.json", "utf8"));
const facades = new Set<string>(plan.facades.map((f: any) => nfc(f.facade)));
const parse = (p: string) => { const m = new Set<string>(); for (const l of readFileSync(p, "utf8").split("\n")) { const x = l.match(/^  error ([^:]+): (.*) → (.*)$/u); if (x) m.add(nfc(x[1]) + "\t" + nfc(x[2]) + "\t" + nfc(x[3])); } return m; };
const A = parse(a), B = parse(b);
const rows = [...B].filter(k => !A.has(k) && facades.has(k.split("\t")[2]));
const byRule = new Map<string, string[]>();
for (const k of rows) { const [r] = k.split("\t"); (byRule.get(r) ?? byRule.set(r, []).get(r)!).push(k); }
console.log("added error lines pointing at a created facade:", rows.length);
for (const [r, ks] of byRule) { console.log(`\n${r}: ${ks.length}`); for (const k of ks) { const [, f, t] = k.split("\t"); console.log("  ", f.slice(-95), "->", t.slice(-62)); } }
