import { readFileSync } from "node:fs";
const d = Bun.JSONC.parse(readFileSync(".vscode/launch.json", "utf8")) as any;
const rows = d.configurations as any[];
const j = (r: any) => JSON.stringify(r);
console.log("ticket-path rows:", rows.filter(r => j(r).includes("🎫️tickets")).length);
console.log("nx exec --projects=workspace rows:", rows.filter(r => String(r.command).includes("nx exec") || String(r.command).includes('"nx" "exec"')).length);
const envKeys = new Map<string, number>();
for (const r of rows) for (const k of Object.keys(r.env ?? {})) envKeys.set(k, (envKeys.get(k) ?? 0) + 1);
console.log([...envKeys].sort((a, b) => b[1] - a[1]).slice(0, 14).map(([k, v]) => `${k}:${v}`).join("  "));
const first = new Map<string, number>();
for (const r of rows) { const m = /^(\S+?)(?:[A-Za-z]|$)/u.exec(String(r.name)); const p = [...String(r.name)].slice(0, 2).join(""); first.set(p, (first.get(p) ?? 0) + 1); }
console.log([...first].sort((a, b) => b[1] - a[1]).slice(0, 14).map(([k, v]) => `${k}:${v}`).join("  "));
console.log("rows w/o presentation:", rows.filter(r => !r.presentation).length, " rows w/ presentation.group:", rows.filter(r => r.presentation?.group).length);
console.log("compounds:", JSON.stringify(d.compounds).slice(0, 900));
console.log("Dashboard rows:", rows.filter(r => /dashboard/i.test(String(r.name))).map(r => r.name + " => " + r.command).slice(0, 12));
console.log("requests:", [...new Set(rows.map(r => r.request))]);
