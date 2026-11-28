import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const j = JSON.parse(b.subarray(20, 20 + jl).toString());
let all = 0, small = 0, n = 0; const seen = new Set<number>();
for (const m of j.meshes) for (const p of m.primitives) { if (seen.has(p.indices)) continue; seen.add(p.indices); const c = j.accessors[j.meshes ? p.attributes.POSITION : 0].count; const a = j.accessors[p.indices]; all += a.count * 2; if (c <= 256) { small += a.count; n++; } }
console.log({ all, saveIfU8: small, accessors: n, of: seen.size });
