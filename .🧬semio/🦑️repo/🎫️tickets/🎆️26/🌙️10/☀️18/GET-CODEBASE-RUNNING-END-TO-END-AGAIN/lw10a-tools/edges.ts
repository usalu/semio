import { readFileSync, writeFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const closure = new Set(readFileSync(process.argv[3]!, "utf8").split("\n").filter(Boolean));
const src = "🧰️framework/🛍️products/🦑️repo/", tgt = "🧰️framework/🔨️modules/🏃️process/";
const rows = r.summary.violations.filter((v: any) => v.from.startsWith(src) && v.to.startsWith(tgt));
const files = new Set(rows.map((v: any) => v.from));
const inC = [...files].filter((f) => closure.has(f as string));
console.log("violations", rows.length, "files", files.size, "in closure", inC.length, "edges in closure", rows.filter((v: any) => closure.has(v.from)).length);
const tg: Record<string, number> = {};
for (const v of rows) tg[v.to] = (tg[v.to] ?? 0) + 1;
console.log("distinct targets", Object.keys(tg).length);
for (const [k, n] of Object.entries(tg).sort((a, b) => b[1] - a[1]).slice(0, 25)) console.log(n, k);
const dirs: Record<string, number> = {};
for (const f of files) { const d = (f as string).split("/").slice(0, 6).join("/"); dirs[d] = (dirs[d] ?? 0) + 1; }
console.log(Object.entries(dirs).sort((a, b) => b[1] - a[1]).slice(0, 20));
