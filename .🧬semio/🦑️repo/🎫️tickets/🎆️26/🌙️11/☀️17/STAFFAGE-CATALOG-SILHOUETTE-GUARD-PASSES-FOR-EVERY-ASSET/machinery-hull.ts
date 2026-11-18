/** 🔷 Convex hull of one volume solid projected into a view, simplified, optionally in the local frame of a bone (elevation only).
 *  bun machinery-hull.ts <asset> <scale> <view> <solid> [bone] [tol=0.004] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, solidId, bone, tolArg] = process.argv.slice(2) as [string, string, Viewpoint, string, string | undefined, string | undefined];
const scale = Number(scaleArg);
const tol = Number(tolArg ?? 0.004);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((s) => s.id === solidId || s.id.startsWith(solidId));
type P = [number, number];
const pts: P[] = [];
for (const s of solids) for (let i = 0; i < s.positions.length; i += 3) { const p = project(view, s.positions[i]!, s.positions[i + 1]!, s.positions[i + 2]!); pts.push([p.x, p.y]); }
pts.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
const cross = (o: P, a: P, b: P) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
const lo: P[] = [], up: P[] = [];
for (const p of pts) { while (lo.length >= 2 && cross(lo[lo.length - 2]!, lo[lo.length - 1]!, p) <= 0) lo.pop(); lo.push(p); }
for (const p of [...pts].reverse()) { while (up.length >= 2 && cross(up[up.length - 2]!, up[up.length - 1]!, p) <= 0) up.pop(); up.push(p); }
let hull = [...lo.slice(0, -1), ...up.slice(0, -1)];
function dp(l: P[], t: number): P[] {
  if (l.length < 3) return l;
  const [a, b] = [l[0]!, l[l.length - 1]!];
  let [best, idx] = [0, 0];
  for (let i = 1; i < l.length - 1; i++) { const d = Math.abs(cross(a, b, l[i]!)) / (Math.hypot(b[0] - a[0], b[1] - a[1]) || 1); if (d > best) [best, idx] = [d, i]; }
  return best > t ? [...dp(l.slice(0, idx + 1), t).slice(0, -1), ...dp(l.slice(idx), t)] : [a, b];
}
const half = Math.floor(hull.length / 2);
hull = [...dp(hull.slice(0, half + 1), tol).slice(0, -1), ...dp([...hull.slice(half), hull[0]!], tol).slice(0, -1)];
let out: P[] = hull;
if (bone !== undefined && bone !== "") {
  const f = state.bones[bone].frame;
  out = hull.map(([x, y]) => { const [dx, dy] = [x - f.o.x, y - f.o.y]; return [dx * f.u.x + dy * f.u.y, dx * f.v.x + dy * f.v.y] as P; });
}
console.log(JSON.stringify(out.map(([x, y]) => [+x.toFixed(4), +y.toFixed(4)])));
console.log("count", solids.length, "points", pts.length, "hull", out.length);
