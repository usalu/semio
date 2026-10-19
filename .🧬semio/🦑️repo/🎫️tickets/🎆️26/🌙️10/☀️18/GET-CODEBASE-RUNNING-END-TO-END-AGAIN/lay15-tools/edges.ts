import { readFileSync, readdirSync, writeFileSync } from "node:fs";
const dir = process.argv[2]!, out = process.argv[3]!;
const edges = new Map<string, any>();
for (const f of readdirSync(dir).filter(f => /^lay15b-.*\.json$/u.test(f) && !f.includes("merged") && !f.includes("plan") && !f.includes("revert") && !f.includes("edges"))) {
  const r = JSON.parse(readFileSync(dir + "/" + f, "utf8"));
  const mods = new Map<string, any>(r.modules.map((m: any) => [m.source, m]));
  for (const v of r.summary.violations) {
    if (v.rule.severity !== "error") continue;
    const dep = mods.get(v.from)?.dependencies.find((d: any) => d.resolved === v.to);
    const key = [v.rule.name, v.from, v.to, (v.cycle ?? []).map((c: any) => c.name).join(">")].join("|");
    edges.set(key, { rule: v.rule.name, from: v.from, to: v.to, specifier: dep?.module, depTypes: dep?.dependencyTypes, cycle: v.cycle?.map((c: any) => c.name) });
  }
}
writeFileSync(out, JSON.stringify([...edges.values()]));
console.log(edges.size);
