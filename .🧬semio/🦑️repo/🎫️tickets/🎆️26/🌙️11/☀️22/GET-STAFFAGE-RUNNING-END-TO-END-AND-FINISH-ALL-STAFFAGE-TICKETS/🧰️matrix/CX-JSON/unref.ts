/** 🔬️ Vertices of each POSITION accessor that no primitive indexes, and how index accessors of equal rebased content could be shared. unref.ts <file.glb> */
import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const j = JSON.parse(b.subarray(20, 20 + jl).toString());
const bin = b.subarray(20 + jl + 8);
const idx = (i: number): number[] => { const a = j.accessors[i]; const v = j.bufferViews[a.bufferView]; const out: number[] = []; for (let k = 0; k < a.count; k++) out.push(a.componentType === 5125 ? bin.readUInt32LE(v.byteOffset + 4 * k) : bin.readUInt16LE(v.byteOffset + 2 * k)); return out; };
const used = new Map<number, Uint8Array>();
const idxSeen = new Set<number>();
for (const m of j.meshes) for (const p of m.primitives) { const a = p.attributes.POSITION; if (!used.has(a)) used.set(a, new Uint8Array(j.accessors[a].count)); const u = used.get(a)!; if (!idxSeen.has(p.indices) || true) for (const i of idx(p.indices)) u[i] = 1; }
let tot = 0, unref = 0; for (const [a, u] of used) { tot += u.length; for (const x of u) if (!x) unref++; }
console.log({ positionAccessors: used.size, verts: tot, unref });
// exact-duplicate vertex positions+normals within a solid (lit) and index accessors by identical content
const size = (i: number) => j.accessors[i].count * (j.accessors[i].componentType === 5125 ? 4 : 2);
const byContent = new Map<string, number[]>();
for (const a of new Set(j.meshes.flatMap((m: any) => m.primitives.map((p: any) => p.indices as number)))) { const k = idx(a as number).join(","); byContent.set(k, [...(byContent.get(k) ?? []), a as number]); }
console.log({ indexAccessors: new Set(j.meshes.flatMap((m: any) => m.primitives.map((p: any) => p.indices))).size, identicalContentGroups: byContent.size });
