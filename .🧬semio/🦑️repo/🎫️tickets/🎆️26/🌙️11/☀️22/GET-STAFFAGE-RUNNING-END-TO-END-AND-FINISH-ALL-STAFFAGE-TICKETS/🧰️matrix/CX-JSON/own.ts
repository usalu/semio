/** 📊️ Bytes by mesh group (first owner of an accessor) of a saved GLB: own.ts <file.glb> */
import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const json = JSON.parse(b.subarray(20, 20 + jl).toString());
const size: Record<number, number> = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 };
const width: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };
const bytesOf = (i: number): number => { const a = json.accessors[i]; const v = json.bufferViews[a.bufferView]; return v.byteStride ? v.byteStride * a.count : a.count * size[a.componentType]! * width[a.type]!; };
const seen = new Set<number>(); const g: Record<string, { bytes: number; tri: number; meshes: number; verts: number }> = {};
const grp = (n: string) => n.replace(/^s\d+-/, "").replace(/\d+/g, "#").replace(/^(thermal|body|structure)-(flow-[a-z]+).*/, "$1 $2").replace(/^(thermal|body|structure)-.*/, "$1 other");
for (const m of json.meshes) { const k = grp(m.name); const e = (g[k] ??= { bytes: 0, tri: 0, meshes: 0, verts: 0 }); e.meshes++;
  for (const p of m.primitives) { for (const a of [...Object.values(p.attributes as Record<string, number>), p.indices as number]) if (!seen.has(a)) { seen.add(a); e.bytes += bytesOf(a); }
    if (p.mode === 4) e.tri += json.accessors[p.indices].count / 3; e.verts += json.accessors[p.attributes.POSITION].count; } }
for (const [k, v] of Object.entries(g).sort((a, b) => b[1].bytes - a[1].bytes)) console.log(k.padEnd(28), String(v.bytes).padStart(9), "B", "meshes", v.meshes, "tri", Math.round(v.tri), "verts", v.verts);
console.log("json", jl, "total accessor bytes", [...seen].reduce((s, a) => s + bytesOf(a), 0));
