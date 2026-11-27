import { readFileSync } from "node:fs";
for (const file of process.argv.slice(2)) {
const b = readFileSync(file); const jl = b.readUInt32LE(12);
const j = JSON.parse(b.subarray(20, 20 + jl).toString()); const bin = b.subarray(20 + jl + 8);
const acc = j.accessors, views = j.bufferViews;
const f32 = (i: number) => { const a = acc[i], v = views[a.bufferView]; return new Float32Array(bin.buffer.slice(bin.byteOffset + v.byteOffset, bin.byteOffset + v.byteOffset + v.byteLength)); };
const idx = (i: number) => { const a = acc[i], v = views[a.bufferView]; const raw = bin.buffer.slice(bin.byteOffset + v.byteOffset, bin.byteOffset + v.byteOffset + v.byteLength); return a.componentType === 5123 ? new Uint16Array(raw) : new Uint32Array(raw); };
const seen = new Set<number>(); let total = 0, flat = 0, partial = 0, nprim = 0, nflat = 0, smoothish=0;
for (const m of j.meshes) for (const p of m.primitives) {
  if (p.mode !== 4 || p.attributes.NORMAL === undefined || seen.has(p.attributes.NORMAL)) continue; seen.add(p.attributes.NORMAL);
  const P = f32(p.attributes.POSITION), N = f32(p.attributes.NORMAL), I = idx(p.indices);
  const size = N.byteLength; total += size; nprim++;
  const used = new Uint8Array(P.length / 3); let ok = true;
  for (let t = 0; t < I.length && ok; t += 3) {
    const a = I[t]!, b2 = I[t + 1]!, c = I[t + 2]!;
    const ux = P[3*b2]!-P[3*a]!, uy = P[3*b2+1]!-P[3*a+1]!, uz = P[3*b2+2]!-P[3*a+2]!, vx = P[3*c]!-P[3*a]!, vy = P[3*c+1]!-P[3*a+1]!, vz = P[3*c+2]!-P[3*a+2]!;
    let nx = uy*vz-uz*vy, ny = uz*vx-ux*vz, nz = ux*vy-uy*vx; const l = Math.hypot(nx, ny, nz); if (l < 1e-12) continue; nx/=l; ny/=l; nz/=l;
    for (const v of [a, b2, c]) if (nx*N[3*v]!+ny*N[3*v+1]!+nz*N[3*v+2]! < 0.99999) { ok = false; break; }
  }
  if (ok) { flat += size; nflat++; }
}
console.log(file.split("/").pop(), "normal bytes", total, "flat-only prims", nflat, "/", nprim, "flat bytes", flat);
}
