import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const FM = "🧰️framework/🔨️modules/";
const v = (r.summary.violations as any[]).filter((x) => x.from.startsWith(OS) && !x.from.includes("/📺️renderer/"));
const byRule: Record<string, number> = {};
const byTarget: Record<string, number> = {};
const files = new Set<string>();
for (const x of v) {
  const n = x.rule.name.replace(/^(no-cross-package-relative)-.+$/u, "$1");
  byRule[n] = (byRule[n] ?? 0) + 1;
  if (x.to.startsWith(FM)) { const t = x.to.slice(FM.length).split("/")[0]!; byTarget[n + " -> fm:" + t] = (byTarget[n + " -> fm:" + t] ?? 0) + 1; files.add(x.from); }
  else if (!x.to.startsWith(OS)) { const k = n + " -> " + x.to.split("/").slice(0, 3).join("/"); byTarget[k] = (byTarget[k] ?? 0) + 1; }
}
console.log(JSON.stringify({ total: v.length, byRule, fmFiles: files.size }, null, 1));
console.log(Object.entries(byTarget).sort((a, b) => b[1] - a[1]).map(([k, c]) => c + " " + k).join("\n"));
