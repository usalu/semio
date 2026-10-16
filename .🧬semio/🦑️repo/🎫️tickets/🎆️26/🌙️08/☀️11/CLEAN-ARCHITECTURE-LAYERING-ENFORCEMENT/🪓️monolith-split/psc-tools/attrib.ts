/** 🧭️ Attributes cruiser error lines added since the baseline to this slice: the importer is a file the codemod touched, or the target is a facade or package file it created. Usage: bun attrib.ts <before.txt> <after.txt> */
import { readFileSync, readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
const nfc = (s: string) => s.normalize("NFC");
const [a, b] = process.argv.slice(2);
const HERE = fileURLToPath(new URL("..", import.meta.url));
const touched = new Set<string>();
for (const n of readdirSync(HERE + "🗑️generated/psc-snap")) if (n.endsWith(".json")) for (const k of Object.keys(JSON.parse(readFileSync(HERE + "🗑️generated/psc-snap/" + n, "utf8")))) touched.add(nfc(k));
const plan = JSON.parse(readFileSync(HERE + "🗑️generated/psc-plan.json", "utf8"));
const facades = new Set<string>(plan.facades.map((f: any) => nfc(f.facade)));
const parse = (p: string) => { const m = new Set<string>(); for (const l of readFileSync(p, "utf8").split("\n")) { const x = l.match(/^  error ([^:]+): (.*) → (.*)$/u); if (x) m.add(nfc(x[1]) + "\t" + nfc(x[2]) + "\t" + nfc(x[3])); } return m; };
const A = parse(a), B = parse(b);
const mine: string[] = [];
for (const k of B) if (!A.has(k)) { const [r, f, t] = k.split("\t"); if (touched.has(f) || facades.has(f) || facades.has(t) || t.includes("📦️packages/🟦️typescript/") && /schema-ts|\/🧬️schema\/📦️packages/u.test(t)) mine.push(k); }
const byRule = new Map<string, number>();
for (const k of mine) byRule.set(k.split("\t")[0], (byRule.get(k.split("\t")[0]) ?? 0) + 1);
console.log("added lines attributable to touched importers / created facades:", mine.length, Object.fromEntries(byRule));
for (const k of mine.slice(0, 80)) { const [r, f, t] = k.split("\t"); console.log(r.slice(0, 50).padEnd(50), f.slice(-70), "->", t.slice(-70)); }
