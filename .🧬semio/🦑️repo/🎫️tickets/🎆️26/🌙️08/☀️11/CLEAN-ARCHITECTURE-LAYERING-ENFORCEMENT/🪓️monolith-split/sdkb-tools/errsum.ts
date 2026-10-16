/** 📊️ Summarises a dependency-cruiser `err` output by rule family and by origin area. Usage: bun errsum.ts <err.txt> */
import { readFileSync } from "node:fs";
const lines = readFileSync(process.argv[2], "utf8").split("\n");
const fam = (r: string) => r.replace(/^no-cross-package-relative-.*/, "no-cross-package-relative-*").replace(/^no-cross-plugin-.*/, "no-cross-plugin-*").replace(/^plugin-no-extension-or-artifact-.*/, "plugin-no-extension-or-artifact-*");
const byRule = new Map<string, number>(), byRuleFrom = new Map<string, number>();
let total = 0;
for (const l of lines) {
  const m = l.match(/^\s+(error|warn) ([^:]+): (\S+) → (.*)$/);
  if (!m) continue;
  total++;
  const r = fam(m[2]);
  byRule.set(r, (byRule.get(r) ?? 0) + 1);
  const area = m[3].startsWith("✏️s/") ? "plugins" : m[3].startsWith("🧰️framework/📦️packages/🟦️typescript/") ? "sdk" : m[3].startsWith("🧰️framework/") ? "framework" : "other";
  const k = `${r} [from ${area}]`;
  byRuleFrom.set(k, (byRuleFrom.get(k) ?? 0) + 1);
}
console.log("total", total, "|", lines.filter(l => /dependency violations/.test(l)).join(" "));
for (const [k, v] of [...byRuleFrom].sort((a, b) => b[1] - a[1]).slice(0, 25)) console.log(String(v).padStart(7), k);
