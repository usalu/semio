import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const REPO = "🧰️framework/🛍️products/🦑️repo/";
const rend = OS + "🔨️modules/📺️renderer/";
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const out: Record<string, number> = {};
const files = new Set<string>();
const targets: Record<string, number> = {};
const rows: any[] = [];
for (const x of r.summary.violations as any[]) {
  if (!x.from.startsWith(OS) || x.from.startsWith(rend) || !x.to.startsWith(REPO)) continue;
  const f = fam(x.rule.name);
  out[f] = (out[f] ?? 0) + 1;
  files.add(x.from);
  targets[x.to] = (targets[x.to] ?? 0) + 1;
  rows.push({ from: x.from, to: x.to, rule: f });
}
console.log(JSON.stringify({ total: rows.length, byRule: out, files: files.size }, null, 1));
if (process.argv[3]) await Bun.write(process.argv[3], JSON.stringify({ rows, targets }, null, 1));
