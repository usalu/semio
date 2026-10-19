import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
console.log("totalCruised", r.summary.totalCruised, "violations", r.summary.violations.length, "error", r.summary.error, "warn", r.summary.warn);
const strip = (p: string) => p.replace(/[^\x00-\x7f\/]/gu, "");
const circ = r.summary.violations.filter((v: any) => v.rule.name === "no-circular" && /\/plugins\//.test(strip(v.from)) || (v.rule.name === "no-circular" && /s\/dev|s\/modules/.test(strip(v.from))));
for (const v of circ) console.log(strip(v.from).slice(-120), "|", (v.cycle ?? []).map((c: any) => strip(c.name).split("/").slice(-2).join("/")).join(" > "));
const byRule: Record<string, number> = {};
for (const v of r.summary.violations) byRule[v.rule.name.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1")] = (byRule[v.rule.name.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1")] ?? 0) + 1;
console.log(JSON.stringify(byRule));
