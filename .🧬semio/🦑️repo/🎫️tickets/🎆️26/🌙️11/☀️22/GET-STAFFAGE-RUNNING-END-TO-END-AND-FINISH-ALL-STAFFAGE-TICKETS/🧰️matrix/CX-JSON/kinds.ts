/** 📊️ Binary bytes of a saved GLB by accessor kind (unique accessors): kinds.ts <file.glb> */
import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const j = JSON.parse(b.subarray(20, 20 + jl).toString());
const W: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 }; const C: Record<number, number> = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 };
const used = new Map<number, string>();
for (const m of j.meshes) for (const p of m.primitives) { for (const [k, a] of Object.entries<number>(p.attributes)) used.set(a, k); used.set(p.indices, p.mode === 1 ? "idx-lines" : "idx-tri"); }
const t: Record<string, { n: number; bytes: number }> = {};
for (const [a, k] of used) { const x = j.accessors[a]; const v = j.bufferViews[x.bufferView]; const bytes = v.byteStride ? v.byteStride * x.count : x.count * C[x.componentType]! * W[x.type]!; const e = (t[k + "/" + x.componentType] ??= { n: 0, bytes: 0 }); e.n++; e.bytes += bytes; }
console.log(t);
