/** 🧵 Refits the radii of a revolved profile shape (points [axial, radius], first and last on the axis) so the drawn half-width per height equals the volume half-width of the given solids; prints a shape-set ops entry. bun humansb-A2-fit.ts <asset> <scale> <view> <shapeId> <solidIds,..> [mix=1] [dir=1: axial grows with height, -1 against it] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, shapeId, ids, mixArg, dirArg] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string | undefined, string | undefined];
const dir = Number(dirArg ?? 1);
const mix = Number(mixArg ?? 1);
const scale = Number(sc);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const want = new Set(ids.split(","));
const segs: [number, number, number, number][] = [];
for (const w of worldSolids(asset, state.bones as never, scale)) if (want.has(w.id)) for (let k = 0; k < w.indices.length; k += 3) {
  const p = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); });
  for (let j = 0; j < 3; j++) segs.push([p[j]!.x, p[j]!.y, p[(j + 1) % 3]!.x, p[(j + 1) % 3]!.y]);
}
const ring = (state.shapes as any)[view].find((x: any) => x.shape.id === shapeId).rings[0] as { x: number; y: number }[];
const shape = (asset as any).representations[sc].geometry[view].find((x: any) => x.id === shapeId);
const rs: [number, number, number, number][] = [];
for (let i = 0; i < ring.length; i++) { const p = ring[i]!, q = ring[(i + 1) % ring.length]!; rs.push([p.x, p.y, q.x, q.y]); }
const cx = ring.reduce((m, p) => m + p.x, 0) / ring.length, cy = ring.reduce((m, p) => m + p.y, 0) / ring.length;
let sxx = 0, sxy = 0, syy = 0;
for (const p of ring) { sxx += (p.x - cx) ** 2; sxy += (p.x - cx) * (p.y - cy); syy += (p.y - cy) ** 2; }
const th = 0.5 * Math.atan2(2 * sxy, sxx - syy);
let U: [number, number] = [Math.cos(th), Math.sin(th)];
if (U[1] < 0 || (Math.abs(U[1]) < 1e-9 && U[0] < 0)) U = [-U[0], -U[1]];
const Vv: [number, number] = [-U[1], U[0]];
const tOf = (x: number, y: number) => x * U[0] + y * U[1];
const wOf = (x: number, y: number) => x * Vv[0] + y * Vv[1];
const ext = (list: [number, number, number, number][], t: number) => {
  let lo = Infinity, hi = -Infinity;
  for (const [x0, y0, x1, y1] of list) {
    const [t0, t1] = [tOf(x0, y0), tOf(x1, y1)];
    if ((t0 - t) * (t1 - t) <= 0 && t0 !== t1) { const k = (t - t0) / (t1 - t0); const w = wOf(x0 + k * (x1 - x0), y0 + k * (y1 - y0)); lo = Math.min(lo, w); hi = Math.max(hi, w); }
  }
  return (hi - lo) / 2;
};
const ts = ring.map((p) => tOf(p.x, p.y));
const [tlo, thi] = [Math.min(...ts), Math.max(...ts)];
const pts = shape.points as number[][];
const [a0, a1] = [pts[0]![0]!, pts[pts.length - 1]![0]!];
const tAt = (a: number, d: number) => { const k = (a - a0) / (a1 - a0); return d > 0 ? tlo + k * (thi - tlo) : thi - k * (thi - tlo); };
const score = (d: number) => pts.slice(1, -1).reduce((m, [a, r]) => { const w = ext(rs, tAt(a!, d)); return m + (isFinite(w) ? Math.abs(w - r!) : 0); }, 0);
const best = dirArg === undefined ? (score(1) <= score(-1) ? 1 : -1) : dir;
console.error("dir", best, "score", score(1).toFixed(3), score(-1).toFixed(3), "U", U.map((x) => x.toFixed(3)).join(","));
const kept: number[][] = [];
pts.forEach(([a, r], i) => {
  if (i === 0 || i === pts.length - 1) return;
  const t = tAt(a!, best);
  const v = ext(segs, t), d = ext(rs, t);
  if (!isFinite(v) || v < 1e-3) return;
  kept.push([a!, !isFinite(d) || d < 1e-3 ? r! : +(r! * (1 + mix * (v / d - 1))).toFixed(4)]);
});
const out = [[kept[0]![0]!, 0], ...kept, [kept[kept.length - 1]![0]!, 0]];
console.log(JSON.stringify([{ op: "shape-set", scale: sc, view, id: shapeId, set: { points: out } }]));
