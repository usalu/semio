/** 🔬️ Upper bounds of binary savings in a saved GLB: rebased-index sharing per solid, exact-duplicate vertices per solid, unreferenced vertices. dup.ts <file.glb> */
import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const j = JSON.parse(b.subarray(20, 20 + jl).toString());
const bin = b.subarray(20 + jl + 8);
const W: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4 };
const read = (i: number): { n: number; v: number[]; stride: number; type: string; comp: number } => { const a = j.accessors[i]; const v = j.bufferViews[a.bufferView]; const w = W[a.type]!; const cs = a.componentType === 5126 || a.componentType === 5125 ? 4 : a.componentType === 5122 || a.componentType === 5123 ? 2 : 1; const stride = v.byteStride ?? cs * w; const out: number[] = []; const base = v.byteOffset + (a.byteOffset ?? 0); for (let k = 0; k < a.count; k++) for (let c = 0; c < w; c++) { const o = base + k * stride + c * cs; out.push(a.componentType === 5126 ? bin.readFloatLE(o) : a.componentType === 5125 ? bin.readUInt32LE(o) : a.componentType === 5123 ? bin.readUInt16LE(o) : a.componentType === 5120 ? bin.readInt8(o) : bin.readUInt8(o)); } return { n: a.count, v: out, stride, type: a.type, comp: a.componentType }; };
const seenAcc = new Set<number>(); let idxBytes = 0, posBytes = 0, nrmBytes = 0, otherBytes = 0;
const rebased = new Map<string, number>(); let rebasedAll = 0, rebasedUnique = 0; let dupVerts = 0, totalVerts = 0, unref = 0, dupBytes = 0;
const seenPrim = new Set<string>();
for (const m of j.meshes) for (const p of m.primitives) {
  const key = JSON.stringify([p.attributes, p.indices]); if (seenPrim.has(key)) continue; seenPrim.add(key);
  const ix = read(p.indices); if (!seenAcc.has(p.indices)) { seenAcc.add(p.indices); idxBytes += ix.n * (ix.comp === 5125 ? 4 : 2); }
  for (const [name, a] of Object.entries<number>(p.attributes)) if (!seenAcc.has(a)) { seenAcc.add(a); const r = j.accessors[a]; const v = j.bufferViews[r.bufferView]; const bytes = (v.byteStride ?? 12) * r.count; if (name === "POSITION") posBytes += bytes; else if (name === "NORMAL") nrmBytes += bytes; else otherBytes += bytes; }
  const attrs = Object.entries<number>(p.attributes).map(([k, a]) => [k, read(a)] as const);
  const n = attrs[0]![1].n; totalVerts += n;
  const ref = new Uint8Array(n); for (const i of ix.v) ref[i] = 1; for (const r of ref) if (!r) unref++;
  const solids = p.extras?.solids ?? [{ first: 0, count: ix.v.length }];
  for (const s of solids) {
    const sl = ix.v.slice(s.first, s.first + s.count); let lo = Infinity; for (const v of sl) lo = Math.min(lo, v);
    const k = sl.map((v) => v - lo).join(","); rebasedAll += s.count * 2; if (!rebased.has(k)) { rebased.set(k, s.count * 2); rebasedUnique += s.count * 2; }
    const seen = new Set<string>(); const verts = new Set(sl);
    for (const v of verts) { const kk = attrs.map(([, a]) => a.v.slice(v * W[a.type]!, v * W[a.type]! + W[a.type]!).join(",")).join("|"); if (seen.has(kk)) { dupVerts++; dupBytes += 16 * 1; } else seen.add(kk); }
  }
}
console.log({ idxBytes, posBytes, nrmBytes, otherBytes, rebasedAll, rebasedUnique, saveByIndexSharing: rebasedAll - rebasedUnique, totalVerts, unref, dupVerts });
