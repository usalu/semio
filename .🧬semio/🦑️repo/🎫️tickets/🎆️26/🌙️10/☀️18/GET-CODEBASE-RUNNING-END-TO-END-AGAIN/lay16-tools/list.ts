import { readFileSync } from "node:fs";
const [file, rulePat] = [process.argv[2]!, new RegExp(process.argv[3] ?? ".")];
const r = JSON.parse(readFileSync(file, "utf8"));
const mods = new Map<string, any>(r.modules.map((m: any) => [m.source, m]));
for (const v of r.summary.violations) {
  if (v.rule.severity !== "error" || !rulePat.test(v.rule.name)) continue;
  const dep = mods.get(v.from)?.dependencies.find((d: any) => d.resolved === v.to);
  console.log([v.rule.name, v.from, v.to, dep?.module ?? "", (dep?.dependencyTypes ?? []).join(","), (v.cycle ?? []).map((c: any) => c.name).join(">")].join("\t"));
}
