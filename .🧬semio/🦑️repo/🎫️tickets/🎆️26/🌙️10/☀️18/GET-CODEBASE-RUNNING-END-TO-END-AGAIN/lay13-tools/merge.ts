import { readFileSync, readdirSync, writeFileSync } from "node:fs";
const [dir, prefix, out] = process.argv.slice(2) as [string, string, string];
const seen = new Map<string, any>();
let cruised = 0;
for (const f of readdirSync(dir).filter(f => f.startsWith(prefix) && f.endsWith(".json") && !f.includes("merged") && !f.includes("plan") && !f.includes("revert") && !f.includes("edges"))) {
  const r = JSON.parse(readFileSync(dir + "/" + f, "utf8"));
  cruised += r.summary.totalCruised;
  for (const v of r.summary.violations) {
    const key = [v.rule.name, v.from, v.to, (v.cycle ?? []).map((c: any) => c.name).join(">")].join("|");
    if (v.rule.severity === "error" || v.rule.severity === undefined) seen.set(key, v);
  }
}
const vs = [...seen.values()];
writeFileSync(out, JSON.stringify({ cruised, violations: vs.map(v => ({ rule: v.rule.name, from: v.from, to: v.to, type: v.type, cycle: v.cycle?.map((c: any) => c.name) })) }));
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const byRule: Record<string, number> = {};
for (const v of vs) byRule[fam(v.rule.name)] = (byRule[fam(v.rule.name)] ?? 0) + 1;
console.log(vs.length, cruised, JSON.stringify(Object.entries(byRule).sort((a, b) => b[1] - a[1]), null, 0));
