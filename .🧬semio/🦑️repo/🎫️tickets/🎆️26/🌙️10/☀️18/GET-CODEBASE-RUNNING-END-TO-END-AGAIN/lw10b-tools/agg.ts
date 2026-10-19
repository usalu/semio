import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const P = "🧰️framework/🛍️products/";
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const strip = (s: string) => s.replace(/[\u{1F000}-\u{1FFFF}\u{2000}-\u{2BFF}\u{FE0F}\u{200D}]/gu, "");
const area = (p: string) => { const s = strip(p).split("/"); return s[0] === "framework" && s[1] === "products" ? `products/${s[2]}` : s[0] === "framework" ? `framework/${s[1]}/${s[2]}` : s[0] ?? p; };
const v = r.summary.violations as any[];
const byRule: Record<string, number> = {}; const edges: Record<string, number> = {};
for (const x of v) {
  const f = fam(x.rule.name); byRule[f] = (byRule[f] ?? 0) + 1;
  const k = `${area(x.from)} -> ${area(x.to)} | ${f}`; edges[k] = (edges[k] ?? 0) + 1;
}
console.log("total", v.length, "cruised", r.summary.totalCruised);
console.log(byRule);
console.log(Object.entries(edges).sort((a, b) => b[1] - a[1]).slice(0, 40).map(([k, n]) => `${n}\t${k}`).join("\n"));
