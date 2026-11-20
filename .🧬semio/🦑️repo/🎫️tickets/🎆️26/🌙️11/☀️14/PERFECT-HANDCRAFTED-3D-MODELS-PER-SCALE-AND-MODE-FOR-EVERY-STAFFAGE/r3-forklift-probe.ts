/** 🔎 Prints the world rest bounding box and triangle count of chosen solids of the forklift candidate: `bun r3-forklift-probe.ts <scale> <list> [id-substring ...]`. */
import { readFileSync } from "node:fs";
import { localGeometry, qualityOfAsset, resolveList } from "./volumes-lib.ts";
import { frames } from "./volumes-base.ts";

const [scale, list, ...ids] = process.argv.slice(2);
const asset = JSON.parse(readFileSync("🗑️generated/r3-forklift/forklift-r3/🔣️.json", "utf8")).asset;
const f = frames(asset) as Record<string, { o: number[]; u: number[]; v: number[]; w: number[] }>;
const q = qualityOfAsset(asset);
for (const solid of resolveList(asset.volumes[scale!], list as never)) {
  if (ids.length && !ids.some((s) => solid.id.includes(s))) continue;
  const g = localGeometry(solid, q);
  const fr = f[solid.bone]!;
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < g.positions.length; i += 3) {
    for (let k = 0; k < 3; k++) {
      const w = fr.o[k]! + g.positions[i]! * fr.u[k]! + g.positions[i + 1]! * fr.v[k]! + g.positions[i + 2]! * fr.w[k]!;
      lo[k] = Math.min(lo[k]!, w); hi[k] = Math.max(hi[k]!, w);
    }
  }
  let area = 0;
  const P = g.positions, I = g.indices;
  for (let t = 0; t < I.length; t += 3) {
    const a = [P[I[t]! * 3]!, P[I[t]! * 3 + 1]!, P[I[t]! * 3 + 2]!], b = [P[I[t + 1]! * 3]!, P[I[t + 1]! * 3 + 1]!, P[I[t + 1]! * 3 + 2]!], c = [P[I[t + 2]! * 3]!, P[I[t + 2]! * 3 + 1]!, P[I[t + 2]! * 3 + 2]!];
    const u = [b[0]! - a[0]!, b[1]! - a[1]!, b[2]! - a[2]!], v = [c[0]! - a[0]!, c[1]! - a[1]!, c[2]! - a[2]!];
    area += 0.5 * Math.hypot(u[1]! * v[2]! - u[2]! * v[1]!, u[2]! * v[0]! - u[0]! * v[2]!, u[0]! * v[1]! - u[1]! * v[0]!);
  }
  console.log(`[DEBUG] ${solid.id.padEnd(22)} area ${area.toFixed(3)} region ${solid.region ?? "-"}`);
  console.log(`[DEBUG] ${solid.id.padEnd(22)} x ${lo[0]!.toFixed(3)}..${hi[0]!.toFixed(3)} y ${lo[1]!.toFixed(3)}..${hi[1]!.toFixed(3)} z ${lo[2]!.toFixed(3)}..${hi[2]!.toFixed(3)} tris ${g.indices.length / 3}`);
}
