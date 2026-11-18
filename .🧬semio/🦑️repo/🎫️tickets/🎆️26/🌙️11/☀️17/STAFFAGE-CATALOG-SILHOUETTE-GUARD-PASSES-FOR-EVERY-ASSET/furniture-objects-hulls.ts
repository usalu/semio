/** 🔷 Convex hull of the projection of volume solids into the three views (world metres): bun furniture-objects-hulls.ts <asset> <scale> <id-substring> [view] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, q, only, boneArg] = process.argv.slice(2) as [string, string, string, string | undefined, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale);
const hull = (pts: [number, number][]): [number, number][] => {
  const p = [...pts].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cross = (o: [number, number], a: [number, number], b: [number, number]) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const lo: [number, number][] = [];
  for (const x of p) { while (lo.length >= 2 && cross(lo[lo.length - 2]!, lo[lo.length - 1]!, x) <= 0) lo.pop(); lo.push(x); }
  const up: [number, number][] = [];
  for (const x of [...p].reverse()) { while (up.length >= 2 && cross(up[up.length - 2]!, up[up.length - 1]!, x) <= 0) up.pop(); up.push(x); }
  return [...lo.slice(0, -1), ...up.slice(0, -1)];
};
const views: Record<string, (x: number, y: number, z: number) => [number, number]> = { elevation: (x, y) => [x, y], front: (_x, y, z) => [z, y], plan: (x, _y, z) => [x, z] };
for (const s of solids) {
  if (!s.id.includes(q)) continue;
  for (const [v, f] of Object.entries(views)) {
    if (only !== undefined && only !== v) continue;
    const pts: [number, number][] = [];
    const fr = boneArg === undefined ? undefined : (state.bones as any)[boneArg].frame;
    for (let i = 0; i < s.positions.length; i += 3) {
      const [x, y, z] = [s.positions[i]!, s.positions[i + 1]!, s.positions[i + 2]!];
      if (fr !== undefined && v === "elevation") { const [dx, dy, dz] = [x - fr.o.x, y - fr.o.y, z - fr.o.z]; { const det = fr.u.x * fr.v.y - fr.v.x * fr.u.y; void dz; pts.push([(dx * fr.v.y - dy * fr.v.x) / det, (fr.u.x * dy - fr.u.y * dx) / det]); } } else pts.push(f(x, y, z));
    }
    const fmt = (h: [number, number][]) => JSON.stringify(h.map(([a, b]) => [+a.toFixed(4), +b.toFixed(4)]));
    if (process.env.SLABS === undefined) { console.log(s.id, v, fmt(hull(pts))); continue; }
    const tol = Number(process.env.SLABS);
    const keys = [...new Set(pts.map((p) => Math.round(p[0] / tol)))].sort((a, b) => a - b);
    const parts: string[] = [];
    for (let k = 0; k + 1 < keys.length; k++) parts.push(fmt(hull(pts.filter((p) => { const r = Math.round(p[0] / tol); return r === keys[k] || r === keys[k + 1]; }))));
    console.log(s.id, v, "[" + parts.join(",") + "]");
  }
}
