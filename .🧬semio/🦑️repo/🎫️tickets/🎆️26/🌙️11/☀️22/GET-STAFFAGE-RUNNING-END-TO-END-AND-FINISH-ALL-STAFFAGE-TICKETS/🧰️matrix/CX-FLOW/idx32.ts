/** 📊️ Index accessor widths of a saved GLB: idx32.ts <file.glb> */
import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const json = JSON.parse(b.subarray(20, 20 + jl).toString());
const seen = new Set<number>(); const t: Record<string, { n: number; bytes: number; maxV: number }> = {};
for (const m of json.meshes) for (const p of m.primitives) { if (seen.has(p.indices)) continue; seen.add(p.indices); const a = json.accessors[p.indices]; const k = `${a.componentType}/${p.mode}`; const e = (t[k] ??= { n: 0, bytes: 0, maxV: 0 }); e.n++; e.bytes += a.count * (a.componentType === 5125 ? 4 : 2); e.maxV = Math.max(e.maxV, json.accessors[p.attributes.POSITION].count); }
console.log(t);
