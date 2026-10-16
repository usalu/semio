import fs from "node:fs";
const j = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const rows = [];
const viol = new Set(j.summary.violations.filter(v => v.rule.name === "renderer-hosts-only-ui").map(v => v.from + "\u0000" + v.to));
for (const m of j.modules) for (const d of m.dependencies ?? []) if (viol.has(m.source + "\u0000" + d.resolved) && (d.rules ?? []).some(r => r.name === "renderer-hosts-only-ui")) rows.push({ from: m.source, to: d.resolved, spec: d.module, types: d.dependencyTypes });
const seen = new Set(); const out = [];
for (const r of rows) { const k = r.from + "|" + r.to + "|" + r.spec; if (!seen.has(k)) { seen.add(k); out.push(r); } }
fs.writeFileSync(process.argv[3], out.map(r => JSON.stringify(r)).join("\n") + "\n");
console.log("violations", viol.size, "rows", out.length);
