import { readFileSync } from "node:fs";
const p = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const depth = Number(process.argv[3] ?? 9);
const m: Record<string, number> = {};
for (const f of process.argv[4] === "skipped" ? p.skipped : p.fixes) { const k = f.from.split("/").slice(0, depth).join("/"); m[k] = (m[k] ?? 0) + 1; }
for (const [k, n] of Object.entries(m).sort()) console.log(n, k);
