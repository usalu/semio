import { readFileSync } from "node:fs";
const p = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const m: Record<string, number> = {};
for (const f of p.fixes) m[f.replacement] = (m[f.replacement] ?? 0) + 1;
console.log(Object.entries(m).sort((a,b)=>b[1]-a[1]));
