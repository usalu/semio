import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const OS = "🧰️framework/🛍️products/💻️os/";
const v = (r.summary.violations as any[]).filter(x => x.from.startsWith("✏️s/🔌️plugins/") && x.to.startsWith("🧰️framework/"));
const bucket = (to: string) => to.startsWith(OS) ? "os" : to.startsWith("🧰️framework/🛍️products/🦑️repo/") ? "repo" : to.startsWith("🧰️framework/🔨️modules/") ? "modules" : "other";
const t: Record<string, Record<string, number>> = {};
for (const x of v) { const b = bucket(x.to); t[b] ??= {}; const f = fam(x.rule.name); t[b][f] = (t[b][f] ?? 0) + 1; t[b].total = (t[b].total ?? 0) + 1; }
console.log(JSON.stringify({ totalCruised: r.summary.totalCruised, allViolations: r.summary.violations.length, pluginToFramework: v.length, t }, null, 1));
const files: Record<string, Set<string>> = {};
for (const x of v) { const b = bucket(x.to); (files[b] ??= new Set()).add(x.from); }
console.log(Object.fromEntries(Object.entries(files).map(([k, s]) => [k, s.size])));
const all: Record<string, number> = {};
for (const x of r.summary.violations) { const f = fam(x.rule.name); all[f] = (all[f] ?? 0) + 1; }
console.log(all);
