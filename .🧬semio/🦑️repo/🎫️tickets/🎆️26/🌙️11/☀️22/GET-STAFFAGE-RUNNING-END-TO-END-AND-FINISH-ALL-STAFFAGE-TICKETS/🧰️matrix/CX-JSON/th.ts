import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const j = JSON.parse(b.subarray(20, 20 + jl).toString());
const seen = new Set<number>(); const rows: any[] = [];
for (const m of j.meshes) { let nb = 0, np = 0; for (const p of m.primitives) { const a = p.attributes.POSITION; if (!seen.has(a)) { seen.add(a); nb += j.accessors[a].count * 12; np++; } } if (nb) rows.push([m.name, nb, np, m.primitives.reduce((s: number, p: any) => s + (p.extras?.solids?.length ?? 0), 0), m.primitives[0].attributes.JOINTS_0 !== undefined]); }
rows.sort((a, b) => b[1] - a[1]); for (const r of rows.slice(0, 25)) console.log(r.join(" "));
