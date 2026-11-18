/** 🧵 Refits (PROFILE=poly: refits the vertex widths of a closed polygon shape by horizontal slices, vertex i of the ring = point i; the second coordinate is scaled by volume over drawn half-width at the vertex height) the radii of a revolved profile shape (points [axial, radius], first and last on the axis) so the drawn half-width per height equals the volume half-width of the given solids; prints a shape-set ops entry. bun humansb-A2-fit.ts <asset> <scale> <view> <shapeId> <solidIds,..> [mix=1] [dir=1: axial grows with height, -1 against it] */
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
const ringAt = (y: number, list: [number, number, number, number][]) => {
  let lo = Infinity, hi = -Infinity;
  for (const [x0, y0, x1, y1] of list) if ((y0 - y) * (y1 - y) <= 0 && y0 !== y1) { const x = x0 + ((y - y0) / (y1 - y0)) * (x1 - x0); lo = Math.min(lo, x); hi = Math.max(hi, x); }
  return [lo, hi] as const;
};
const pp = shape.points as number[][];
if (ring.length !== pp.length) { console.error("ring length", ring.length, "points", pp.length); process.exit(1); }
const outp = pp.map(([u, w], i) => {
  const y = ring[i]!.y;
  const [vl, vh] = ringAt(y - 1e-6, segs), [dl, dh] = ringAt(y - 1e-6, rs);
  if (!isFinite(vl) || !isFinite(dl) || Math.abs(w!) < 0.003) return [u, w];
  const side = ring[i]!.x >= (dl + dh) / 2 ? 1 : -1;
  const want = side > 0 ? vh - (dl + dh) / 2 : (dl + dh) / 2 - vl;
  const have = side > 0 ? dh - (dl + dh) / 2 : (dl + dh) / 2 - dl;
  return [u, +(w! * (1 + mix * (want / have - 1))).toFixed(4)];
});
console.log(JSON.stringify([{ op: "shape-set", scale: sc, view, id: shapeId, set: { points: outp } }]));
