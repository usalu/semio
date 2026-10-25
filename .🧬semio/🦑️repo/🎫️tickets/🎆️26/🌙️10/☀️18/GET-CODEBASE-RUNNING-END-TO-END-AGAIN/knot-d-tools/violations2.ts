import { readFileSync } from "node:fs";
const g = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const x of g.summary.violations as any[]) console.log(x.rule.name, (x.cycle ?? []).map((c: any) => c.name.replace("🧰️framework/", "F/")).join(" > "));
