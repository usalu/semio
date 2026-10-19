import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const by: Record<string, number> = {};
for (const x of r.summary.violations) by[x.rule.name.replace(/^(no-cross-package-relative|no-cross-technology)-.+$/u, "$1")] = (by[x.rule.name.replace(/^(no-cross-package-relative|no-cross-technology)-.+$/u, "$1")] ?? 0) + 1;
console.log(by);
if (process.argv[3]) for (const x of r.summary.violations) if (x.rule.name === process.argv[3]) console.log(x.from.slice(0, 120), "->", String(x.to).slice(0, 120));
