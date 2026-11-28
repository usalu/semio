import { readFileSync } from "node:fs";
const dir = process.argv[2]!, base = process.argv[3]!;
const parse = (file: string) => { const b = readFileSync(file); const jl = b.readUInt32LE(12); return { j: JSON.parse(b.subarray(20, 20 + jl).toString()), bin: b.subarray(20 + jl + 8), size: b.length }; };
const comp: any = { 5120: 1, 5122: 2, 5123: 2, 5126: 4 };
function stats(file: string) {
  const { j, bin } = parse(file);
  const grid = j.nodes.find((n: any) => n.name?.endsWith("-grid"))?.scale?.[0];
  const view = (i: number) => { const a = j.accessors[i], v = j.bufferViews[a.bufferView]; return { a, v, dv: new DataView(bin.buffer, bin.byteOffset + v.byteOffset, v.byteLength) }; };
  const pos = (i: number) => { const { a, v, dv } = view(i); const stride = v.byteStride ?? comp[a.componentType] * 3; const out = new Float64Array(a.count * 3); for (let e = 0; e < a.count; e++) for (let c = 0; c < 3; c++) out[e * 3 + c] = a.componentType === 5122 ? dv.getInt16(e * stride + c * 2, true) * (grid ?? 1) : dv.getFloat32(e * stride + c * 4, true); return out; };
  const idx = (i: number) => { const { a, dv } = view(i); const out = new Uint32Array(a.count); for (let e = 0; e < a.count; e++) out[e] = dv.getUint16(e * 2, true); return out; };
  let tri = 0, degenerate = 0, area0 = 0; const seen = new Set<string>(); let maxAbs = 0;
  const seenAcc = new Set<number>();
  for (const m of j.meshes) { if (m.name.includes("-structure-") || true) for (const p of m.primitives) { if (p.mode !== 4) continue; const key = p.attributes.POSITION + "/" + p.indices; if (seen.has(key)) continue; seen.add(key); const P = pos(p.attributes.POSITION), I = idx(p.indices); for (let t = 0; t < I.length; t += 3) { tri++; const a = I[t]!, b = I[t + 1]!, c = I[t + 2]!; const ux = P[3*b]!-P[3*a]!, uy = P[3*b+1]!-P[3*a+1]!, uz = P[3*b+2]!-P[3*a+2]!, vx = P[3*c]!-P[3*a]!, vy = P[3*c+1]!-P[3*a+1]!, vz = P[3*c+2]!-P[3*a+2]!; const n2 = (uy*vz-uz*vy)**2 + (uz*vx-ux*vz)**2 + (ux*vy-uy*vx)**2; if (n2 === 0) degenerate++; } } }
  return { grid, tri, degenerate };
}
for (const name of process.argv.slice(4)) { const a = stats(`${base}/${name}.glb`), b = stats(`${dir}/${name}.glb`); console.log("[DEBUG]", name, "float32:", JSON.stringify(a), "grid:", JSON.stringify(b), "step_m", b.grid, "worst_error_m", b.grid / 2, "size", parse(`${dir}/${name}.glb`).size); }
