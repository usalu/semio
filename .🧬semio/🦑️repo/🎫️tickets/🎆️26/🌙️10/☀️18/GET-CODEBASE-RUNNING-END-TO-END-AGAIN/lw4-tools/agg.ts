import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const v = (r.summary.violations as any[]).filter((x) => x.from.startsWith("🌎️hub/"));
const byRule: Record<string, number> = {}, byTo: Record<string, number> = {}, files = new Set<string>();
for (const x of v) {
  const f = fam(x.rule.name);
  byRule[f] = (byRule[f] ?? 0) + 1;
  files.add(x.from);
  const t = x.to.replace(/^(🧰️framework\/[^/]+\/[^/]+\/[^/]+|✏️s\/[^/]+\/[^/]+|[^/]+\/[^/]+)\/.*/u, "$1");
  byTo[`${f} -> ${t}`] = (byTo[`${f} -> ${t}`] ?? 0) + 1;
}
console.log(JSON.stringify({ totalCruised: r.summary.totalCruised, total: v.length, files: files.size, byRule, byTo }, null, 1));
