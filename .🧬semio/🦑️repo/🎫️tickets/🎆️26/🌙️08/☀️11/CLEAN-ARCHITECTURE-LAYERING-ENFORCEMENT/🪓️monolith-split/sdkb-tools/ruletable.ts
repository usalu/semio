/** 📋️ Per-rule totals and production counts (edges whose source is outside `🧪️tests`) from a full cruiser `err` output. Usage: bun ruletable.ts <err.txt> [rule-prefix...] */
import { readFileSync } from "node:fs";
const [file, ...prefixes] = process.argv.slice(2);
const lines = readFileSync(file, "utf8").split("\n");
const total = new Map<string, number>(), prod = new Map<string, number>();
const fam = (r: string) => r.replace(/^no-cross-package-relative-.*/, "no-cross-package-relative-*").replace(/^no-cross-plugin-.*/, "no-cross-plugin-*").replace(/^plugin-no-extension-or-artifact-.*/, "plugin-no-extension-or-artifact-*");
let all = 0, allProd = 0;
for (const l of lines) {
  const m = l.match(/^\s+(error|warn) ([^:]+): (\S+) → (.*)$/);
  if (!m) continue;
  const r = fam(m[2]);
  total.set(r, (total.get(r) ?? 0) + 1);
  all++;
  if (!m[3].includes("/🧪️tests/")) { prod.set(r, (prod.get(r) ?? 0) + 1); allProd++; }
}
console.log(lines.filter(l => /dependency violations/.test(l)).join(" "));
console.log("rules:", total.size, "| error lines:", all, "| prod:", allProd);
for (const [r, n] of [...total].sort((a, b) => b[1] - a[1])) if (!prefixes.length || prefixes.some(p => r.startsWith(p))) console.log(String(n).padStart(6), String(prod.get(r) ?? 0).padStart(6), r);
