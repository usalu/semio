import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const filt = process.argv[3] ?? "";
const by: Record<string, number> = {};
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
for (const v of r.summary.violations) { if (!v.from.includes(filt)) continue; by[fam(v.rule.name)] = (by[fam(v.rule.name)] ?? 0) + 1; }
console.log("cruised", r.summary.totalCruised, "violations", r.summary.violations.length, JSON.stringify(by));
if (process.argv[4] === "list") for (const v of r.summary.violations) if (v.from.includes(filt)) console.log(fam(v.rule.name).slice(0, 36).padEnd(36), v.from.slice(-70), "->", v.to.slice(-90));
