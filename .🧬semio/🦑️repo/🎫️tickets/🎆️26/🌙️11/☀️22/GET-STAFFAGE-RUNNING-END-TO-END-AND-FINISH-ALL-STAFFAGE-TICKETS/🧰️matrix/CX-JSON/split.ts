/** 🔬️ Model of splitting solids into primitives that share rebased index accessors: split.ts <file.glb> [costBytesPerSplit] */
import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const j = JSON.parse(b.subarray(20, 20 + jl).toString());
const bin = b.subarray(20 + jl + 8); const cost = Number(process.argv[3] ?? 600);
const idx = (i: number): number[] => { const a = j.accessors[i]; const v = j.bufferViews[a.bufferView]; const out: number[] = []; for (let k = 0; k < a.count; k++) out.push(a.componentType === 5125 ? bin.readUInt32LE(v.byteOffset + 4 * k) : bin.readUInt16LE(v.byteOffset + 2 * k)); return out; };
const seen = new Set<string>(); const content = new Map<string, number>(); const all: { key: string; bytes: number; prim: string }[] = [];
let curBytes = 0, tri = 0;
for (const m of j.meshes) for (const p of m.primitives) {
  const pk = JSON.stringify([p.attributes, p.indices]); if (seen.has(pk)) continue; seen.add(pk); if (p.mode === 1) continue;
  const ix = idx(p.indices); curBytes += ix.length * 2; tri += ix.length / 3;
  const solids = p.extras?.solids ?? [{ first: 0, count: ix.length }];
  for (const s of solids) { const sl = ix.slice(s.first, s.first + s.count); let lo = Infinity; for (const v of sl) lo = Math.min(lo, v); all.push({ key: sl.map((v) => v - lo).join(","), bytes: s.count * 2, prim: pk }); }
}
for (const T of [0, 500, 1000, 2000, 4000, 8000]) {
  const first = new Map<string, number>(); let kept = 0, splits = 0, byPrim = new Map<string, number>();
  for (const s of all) { if (s.bytes < T) { kept += s.bytes; continue; } splits++; if (!first.has(s.key)) { first.set(s.key, s.bytes); kept += s.bytes; } }
  console.log({ T, currentIdxBytes: curBytes, newIdxBytes: kept, solidsSplit: splits, netSaving: curBytes - kept - splits * cost, solidsTotal: all.length });
}
