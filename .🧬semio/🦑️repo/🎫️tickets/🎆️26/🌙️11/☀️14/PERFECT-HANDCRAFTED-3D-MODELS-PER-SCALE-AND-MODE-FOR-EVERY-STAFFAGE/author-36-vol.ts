import { openCtx } from "./kit-lib.ts";
import { scaleMeshes } from "./volumes-lib.ts";
const [spec, sc] = [process.argv[2]!, Number(process.argv[3] ?? 50)];
const c = openCtx(spec, sc);
const sm = scaleMeshes(c.m, sc, c.vol);
let tot = 0;
for (const m of sm.body) {
  let v = 0, mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
  const w = m.world, ix = m.indices;
  for (let i = 0; i < ix.length; i += 3) {
    const a = ix[i]! * 3, b = ix[i + 1]! * 3, d = ix[i + 2]! * 3;
    v += (w[a]! * (w[b + 1]! * w[d + 2]! - w[b + 2]! * w[d + 1]!) - w[a + 1]! * (w[b]! * w[d + 2]! - w[b + 2]! * w[d]!) + w[a + 2]! * (w[b]! * w[d + 1]! - w[b + 1]! * w[d]!)) / 6;
  }
  for (let i = 0; i < w.length; i += 3) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, w[i + k]!); mx[k] = Math.max(mx[k]!, w[i + k]!); }
  tot += Math.abs(v);
  let ar = 0; for (let i = 0; i < ix.length; i += 3) { const a = ix[i]! * 3, b = ix[i + 1]! * 3, d = ix[i + 2]! * 3; const ux = w[b]! - w[a]!, uy = w[b + 1]! - w[a + 1]!, uz = w[b + 2]! - w[a + 2]!, vx = w[d]! - w[a]!, vy = w[d + 1]! - w[a + 1]!, vz = w[d + 2]! - w[a + 2]!; ar += 0.5 * Math.hypot(uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx); }
  console.log(m.id.padEnd(26), "area", ar.toFixed(3).padStart(7), "vol", v.toFixed(3).padStart(8), "x", mn[0]!.toFixed(2), mx[0]!.toFixed(2), "y", mn[1]!.toFixed(2), mx[1]!.toFixed(2), "z", mn[2]!.toFixed(2), mx[2]!.toFixed(2));
}
console.log("total (sum, overlaps counted)", tot.toFixed(3));
