/** 📊️ What EXT_meshopt_compression (lossless, unfiltered) would take of the accessors of a saved GLB, with the reference encoder: meshopt.ts <file.glb> */
import { readFileSync } from "node:fs";
import { MeshoptEncoder } from "/Users/ueli/Documents/semio/node_modules/meshoptimizer/meshopt_encoder.module.js";
await MeshoptEncoder.ready;
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const json = JSON.parse(b.subarray(20, 20 + jl).toString()); const bin = b.subarray(20 + jl + 8);
const seen = new Set<number>(); const tot: Record<string, { raw: number; enc: number }> = {};
const size: Record<number, number> = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 }; const width: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4 };
const add = (k: string, raw: number, enc: number) => { const t = (tot[k] ??= { raw: 0, enc: 0 }); t.raw += raw; t.enc += enc; };
const data = (i: number) => { const a = json.accessors[i]; const v = json.bufferViews[a.bufferView]; const stride = v.byteStride ?? size[a.componentType] * width[a.type]; return { a, bytes: new Uint8Array(bin.buffer.slice(bin.byteOffset + v.byteOffset, bin.byteOffset + v.byteOffset + a.count * stride)), stride }; };
for (const m of json.meshes) for (const p of m.primitives) {
  for (const [name, i] of Object.entries(p.attributes as Record<string, number>)) { if (seen.has(i)) continue; seen.add(i); const { a, bytes, stride } = data(i); const enc = MeshoptEncoder.encodeVertexBuffer(bytes, a.count, stride); add(name, bytes.length, enc.length); }
  const i = p.indices as number; if (seen.has(i)) continue; seen.add(i); const { a, bytes } = data(i);
  if (p.mode === 4 && a.componentType === 5123) { const idx = new Uint32Array(a.count); const dv = new DataView(bytes.buffer, bytes.byteOffset); for (let k = 0; k < a.count; k++) idx[k] = dv.getUint16(k * 2, true); const enc = MeshoptEncoder.encodeIndexBuffer(new Uint8Array(idx.buffer), a.count, 4); add("tri-idx", bytes.length, enc.length); }
  else add("other-idx", bytes.length, bytes.length);
}
for (const [k, v] of Object.entries(tot)) console.log(k.padEnd(10), v.raw, "->", v.enc, (100 * v.enc / v.raw).toFixed(0) + "%");
const raw = Object.values(tot).reduce((s, v) => s + v.raw, 0), enc = Object.values(tot).reduce((s, v) => s + v.enc, 0); console.log("total", raw, "->", enc);
