/** 🏁 Selects a category-balanced top 50 from the finding rows and writes the table. */
import { readFileSync, writeFileSync } from "node:fs";
const base = process.argv[2];
const lines = readFileSync(`${base}/cache-audit-tools/rows-all.md`, "utf8").split("\n").filter((l) => l.startsWith("| "));
const cells = (l: string) => l.trim().replace(/^\|/, "").replace(/\|\s*$/, "").split(" | ").map((s) => s.trim());
const rows = lines.map((l) => { const c = cells(l); return { file: c[1] ?? "", target: c[2] ?? "", issue: c[3] ?? "", fix: c[4] ?? "", sev: c[5] ?? "" }; });
const cat = (r: any) => r.issue.startsWith("Cached on") ? "produces" : r.issue.startsWith("Output resolves") ? "outside" : r.issue.includes("not in inputs {env}") ? "env" : r.issue.startsWith("`cache: false` with declared") ? "candidate" : r.issue.startsWith("Cached target runs") ? "mutating" : r.issue.includes("cmd produces files") ? "cmdwrite" : "hand";
const by: Record<string, any[]> = {};
for (const r of rows) (by[cat(r)] ??= []).push(r);
const rustBuild = (r: any) => r.target === "build" && /🦀️rust/.test(r.file);
const sel: any[] = [...(by.hand ?? []).filter((r) => !r.target.includes("trusted-stdio")), ...(by.mutating ?? []).slice(0, 0), ...(by.env ?? []), ...(by.candidate ?? []), ...(by.outside ?? []).slice(0, 12), ...(by.cmdwrite ?? []).filter((r) => !/^test|\bverify/.test(r.target)).slice(0, 4), ...(by.produces ?? []).filter((r) => !rustBuild(r) && !/check/.test(r.target)).slice(0, 14)];
const seen = new Set<string>(); const top: any[] = [];
for (const r of sel) { const k = r.file + "|" + r.target + "|" + r.issue.slice(0, 40); if (seen.has(k) || top.length >= 50) continue; seen.add(k); top.push(r); }
const esc = (s: string) => s.replace(/\|/g, "/");
const out = top.map((r, i) => `| ${i + 1} | ${esc(r.file)} | ${esc(r.target)} | ${esc(r.issue)} | ${esc(r.fix)} | ${r.sev} |`);
writeFileSync(`${base}/cache-audit-tools/top50.md`, out.join("\n") + "\n");
console.log(JSON.stringify({ counts: Object.fromEntries(Object.entries(by).map(([k, v]) => [k, v.length])), selected: top.length, byCat: top.reduce((a: any, r) => ((a[cat(r)] = (a[cat(r)] ?? 0) + 1), a), {}) }));
