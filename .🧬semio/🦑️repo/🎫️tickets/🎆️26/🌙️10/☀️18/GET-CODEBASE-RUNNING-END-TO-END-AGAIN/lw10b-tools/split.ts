import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const clo = new Set(readFileSync(process.argv[3]!, "utf8").split("\n").filter(Boolean));
let inC = 0, out = 0; const t: Record<string, number> = {};
for (const x of r.summary.violations) { if (!x.rule.startsWith) {} const n = x.rule.name; if (!n.startsWith("no-cross-package-relative-")) continue; if (clo.has(x.from)) { inC++; const k = n.slice(26); t[k] = (t[k] ?? 0) + 1; } else out++; }
console.log({ inC, out, t });
