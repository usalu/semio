import { readFileSync, writeFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const vs = r.summary.violations as { from: string; to: string; rule: { name: string }; cycle?: unknown }[];
const cnt: Record<string, number> = {};
for (const v of vs) { const k = v.rule.name.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1"); cnt[k] = (cnt[k] ?? 0) + 1; }
console.log(cnt);
const ui = vs.filter((v) => v.rule.name === "ui-no-framework-packages");
writeFileSync(process.argv[3]!, JSON.stringify(ui.map((v) => ({ from: v.from, to: v.to })), null, 1));
const strip = (p: string) => p.replace(/[^\x00-\x7f]/gu, "");
const by: Record<string, number> = {};
for (const v of ui) { const s = v.to.split("/"); const k = strip(s.slice(0, 4).join("/")); by[k] = (by[k] ?? 0) + 1; }
console.log(Object.entries(by).sort((a, b) => b[1] - a[1]));
