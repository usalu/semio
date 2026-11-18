/** 🧭 Outer contour of the projected silhouette of one volume solid (raster of its triangles, boundary traced, simplified), in the world view frame or the local frame of a bone (elevation).
 *  bun machinery-contour.ts <asset> <scale> <view> <solid> <bone|-> [cell=0.002] [tol=0.002] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, solidId, bone, cellArg, tolArg] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string | undefined, string | undefined];
const scale = Number(scaleArg);
const cell = Number(cellArg ?? 0.002);
const tol = Number(tolArg ?? 0.002);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((s) => s.id === solidId);
type P = [number, number];
const tris: [P, P, P][] = [];
for (const s of solids) for (let t = 0; t < s.indices.length; t += 3) {
  const q = [0, 1, 2].map((k) => { const i = s.indices[t + k]!; const p = project(view, s.positions[i * 3]!, s.positions[i * 3 + 1]!, s.positions[i * 3 + 2]!); return [p.x, p.y] as P; });
  tris.push([q[0]!, q[1]!, q[2]!]);
}
const xs = tris.flat().map((p) => p[0]), ys = tris.flat().map((p) => p[1]);
const [x0, y0] = [Math.min(...xs) - 2 * cell, Math.min(...ys) - 2 * cell];
const [w, h] = [Math.ceil((Math.max(...xs) - x0) / cell) + 3, Math.ceil((Math.max(...ys) - y0) / cell) + 3];
const g = new Uint8Array(w * h);
for (const [a, b, c] of tris) {
  const [mx0, mx1] = [Math.min(a[0], b[0], c[0]), Math.max(a[0], b[0], c[0])];
  const [my0, my1] = [Math.min(a[1], b[1], c[1]), Math.max(a[1], b[1], c[1])];
  for (let j = Math.floor((my0 - y0) / cell); j <= Math.ceil((my1 - y0) / cell); j++) for (let i = Math.floor((mx0 - x0) / cell); i <= Math.ceil((mx1 - x0) / cell); i++) {
    const p: P = [x0 + (i + 0.5) * cell, y0 + (j + 0.5) * cell];
    const e = (u: P, v: P) => (v[0] - u[0]) * (p[1] - u[1]) - (v[1] - u[1]) * (p[0] - u[0]);
    const [d1, d2, d3] = [e(a, b), e(b, c), e(c, a)];
    const slack = cell * Math.max(1e-9, Math.hypot(b[0] - a[0], b[1] - a[1]));
    if ((d1 >= -slack && d2 >= -slack && d3 >= -slack) || (d1 <= slack && d2 <= slack && d3 <= slack)) g[j * w + i] = 1;
  }
}
const on = (i: number, j: number) => i >= 0 && j >= 0 && i < w && j < h && g[j * w + i] === 1;
let start: [number, number] | undefined;
for (let j = 0; j < h && start === undefined; j++) for (let i = 0; i < w; i++) if (on(i, j)) { start = [i, j]; break; }
const dirs: P[] = [[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [-1, -1], [0, -1], [1, -1]];
const loop: P[] = [];
let [ci, cj] = start!;
let back = 4;
for (let step = 0; step < 4 * w * h; step++) {
  loop.push([ci, cj]);
  let found = false;
  for (let k = 1; k <= 8; k++) {
    const dIdx = (back + k) % 8;
    const [ni, nj] = [ci + dirs[dIdx]![0], cj + dirs[dIdx]![1]];
    if (on(ni, nj)) { back = (dIdx + 4) % 8; [ci, cj] = [ni, nj]; found = true; break; }
  }
  if (!found || (ci === start![0] && cj === start![1] && loop.length > 2)) break;
}
let pts: P[] = loop.map(([i, j]) => [x0 + (i + 0.5) * cell, y0 + (j + 0.5) * cell]);
const cross = (o: P, a: P, b: P) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
function dp(l: P[], t: number): P[] {
  if (l.length < 3) return l;
  const [a, b] = [l[0]!, l[l.length - 1]!];
  let [best, idx] = [0, 0];
  for (let i = 1; i < l.length - 1; i++) { const d = Math.abs(cross(a, b, l[i]!)) / (Math.hypot(b[0] - a[0], b[1] - a[1]) || 1); if (d > best) [best, idx] = [d, i]; }
  return best > t ? [...dp(l.slice(0, idx + 1), t).slice(0, -1), ...dp(l.slice(idx), t)] : [a, b];
}
const half = Math.floor(pts.length / 2);
pts = [...dp(pts.slice(0, half + 1), tol).slice(0, -1), ...dp([...pts.slice(half), pts[0]!], tol).slice(0, -1)];
if (bone !== "-") {
  const f = state.bones[bone].frame;
  pts = pts.map(([x, y]) => { const [dx, dy] = [x - f.o.x, y - f.o.y]; return [dx * f.u.x + dy * f.u.y, dx * f.v.x + dy * f.v.y] as P; });
}
console.log(JSON.stringify(pts.map(([x, y]) => [+x.toFixed(4), +y.toFixed(4)])));
console.log("points", pts.length);
