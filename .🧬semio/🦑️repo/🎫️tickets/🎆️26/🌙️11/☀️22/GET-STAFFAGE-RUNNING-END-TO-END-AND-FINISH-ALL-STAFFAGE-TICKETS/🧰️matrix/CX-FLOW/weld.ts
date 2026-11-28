/** 📊️ Weld potential of the primitives of a saved GLB by mesh group: weld.ts <file.glb> */
import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const json = JSON.parse(b.subarray(20, 20 + jl).toString()); const bin = b.subarray(20 + jl + 8);
const acc = (i: number) => { const a = json.accessors[i]; const v = json.bufferViews[a.bufferView]; return { a, v }; };
const f32 = (i: number) => { const { a, v } = acc(i); return new Float32Array(bin.buffer.slice(bin.byteOffset + v.byteOffset, bin.byteOffset + v.byteOffset + a.count * 12)); };
const seen = new Set<number>(); const g: Record<string, { verts: number; uniq: number; normals: number; tri: number }> = {};
for (const m of json.meshes) { const k = m.name.replace(/^s\d+-/, "").replace(/\d+/g, "#").replace(/^(thermal|body|structure)-(flow-[a-z]+).*/, "$1 $2").replace(/^(thermal|body|structure)-.*/, "$1 other");
  for (const p of m.primitives) { if (p.mode !== 4 || seen.has(p.attributes.POSITION)) continue; seen.add(p.attributes.POSITION);
    const pos = f32(p.attributes.POSITION); const u = new Set<string>(); for (let i = 0; i < pos.length; i += 3) u.add(`${pos[i]},${pos[i+1]},${pos[i+2]}`);
    const e = (g[k] ??= { verts: 0, uniq: 0, normals: 0, tri: 0 }); e.verts += pos.length / 3; e.uniq += u.size; e.normals += p.attributes.NORMAL !== undefined ? 1 : 0; e.tri += json.accessors[p.indices].count / 3; } }
for (const [k, v] of Object.entries(g)) console.log(k.padEnd(26), v);
