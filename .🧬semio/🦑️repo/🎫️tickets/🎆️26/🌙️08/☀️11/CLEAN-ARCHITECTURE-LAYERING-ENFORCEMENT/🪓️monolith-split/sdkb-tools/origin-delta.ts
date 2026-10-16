/** 🧭️ Rule x origin-area counts for two cruiser `err` files, with deltas. Usage: bun origin-delta.ts <before.txt> <after.txt> */
import { readFileSync } from "node:fs";
const fam = (r: string) => r.replace(/^no-cross-package-relative-.*/, "no-cross-package-relative-*").replace(/^no-cross-plugin-.*/, "no-cross-plugin-*").replace(/^plugin-no-extension-or-artifact-.*/, "plugin-no-extension-or-artifact-*");
const area = (f: string) => f.startsWith("✏️s/") ? "plugins" : f.startsWith("🧰️framework/📦️packages/🟦️typescript/") ? "sdk-package" : f.startsWith("🧰️framework/") ? "framework" : f.startsWith("🌎️hub/") ? "hub" : f.startsWith("♻️mit-bestand/") ? "mit-bestand" : "other";
const parse = (p: string) => { const m = new Map<string, number>(); for (const l of readFileSync(p, "utf8").split("\n")) { const x = l.match(/^\s+(error|warn) ([^:]+): (\S+) → (.*)$/); if (!x) continue; const k = `${fam(x[2])} | ${area(x[3])}`; m.set(k, (m.get(k) ?? 0) + 1); } return m; };
const [a, b] = [parse(process.argv[2]), parse(process.argv[3])];
const keys = [...new Set([...a.keys(), ...b.keys()])].sort();
for (const k of keys) { const x = a.get(k) ?? 0, y = b.get(k) ?? 0; if (x !== y) console.log(String(x).padStart(6), String(y).padStart(6), String(y - x).padStart(6), k); }
