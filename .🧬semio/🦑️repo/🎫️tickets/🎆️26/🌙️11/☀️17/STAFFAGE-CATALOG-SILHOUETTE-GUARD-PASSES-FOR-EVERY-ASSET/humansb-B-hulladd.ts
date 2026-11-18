/** 🔶 Op that adds a flat polygon shape on a bone whose world outline is the convex hull of the projection of volume solids (rest-frame coordinates solved by damped Newton through the exact evaluator). bun humansb-B-hulladd.ts <asset> <scale> <view> <patch|-> <solidIds,..> <bone> <newId> <afterId> <out.json> [simplify mm]
 */
import { readFileSync, writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, patch, ids, bone, newId, after, out, simp] = process.argv.slice(2).filter((x) => x !== "--stroke") as string[];
const stroke = process.argv.includes("--stroke");
const scale = Number(sc);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const find = (list: any[], i: string) => list.find((x) => x.id === i);
const pre: any[] = patch !== "-" ? JSON.parse(readFileSync(patch!, "utf8")) : [];
for (const o of pre) {
  if (String(o.scale) !== sc) continue;
  if (o.op.startsWith("solid")) { Object.assign(find((asset as any).volumes[sc!].body, o.id), o.set); continue; }
  const list: any[] = (asset as any).representations[sc!].geometry[o.view];
  if (o.op === "shape-add") list.splice(list.indexOf(find(list, o.after)) + 1, 0, o.shape);
  else if (o.op === "shape-set") Object.assign(find(list, o.id), o.set);
  else list[list.indexOf(find(list, o.id))] = o.shape;
}
const evalA = (a: any) => evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never) as any;
const state = evalA(asset);
const want = new Set(ids!.split(","));
const pts: { x: number; y: number }[] = [];
for (const w of worldSolids(asset, state.bones as never, scale)) if (want.has(w.id)) for (let i = 0; i < w.positions.length; i += 3) pts.push(project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!));
pts.sort((a, b) => a.x - b.x || a.y - b.y);
const cross = (o: any, a: any, b: any) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
const half = (list: any[]) => { const h: any[] = []; for (const p of list) { while (h.length >= 2 && cross(h[h.length - 2], h[h.length - 1], p) <= 0) h.pop(); h.push(p); } h.pop(); return h; };
let hull: { x: number; y: number }[] = [...half(pts), ...half([...pts].reverse())];
const eps = Number(simp ?? 1) / 1000;
const rdpOpen = (p: any[], e: number): any[] => {
  if (p.length < 3) return p;
  const a = p[0], b = p[p.length - 1], dx = b.x - a.x, dy = b.y - a.y, l = Math.hypot(dx, dy) || 1;
  let best = -1, idx = -1;
  for (let i = 1; i < p.length - 1; i++) { const d = Math.abs((p[i].x - a.x) * dy - (p[i].y - a.y) * dx) / l; if (d > best) { best = d; idx = i; } }
  return best <= e ? [a, b] : [...rdpOpen(p.slice(0, idx + 1), e).slice(0, -1), ...rdpOpen(p.slice(idx), e)];
};
if (!stroke) hull = [...rdpOpen(hull.slice(0, Math.ceil(hull.length / 2) + 1), eps).slice(0, -1), ...rdpOpen([...hull.slice(Math.ceil(hull.length / 2)), hull[0]!], eps).slice(0, -1)];
if (stroke) {
  let best: [any, any] = [hull[0]!, hull[0]!], bd = -1;
  for (const a of hull) for (const b of hull) { const d = Math.hypot(a.x - b.x, a.y - b.y); if (d > bd) { bd = d; best = [a, b]; } }
  hull = [...best];
}
const shape: any = { kind: "poly", points: hull.map(() => [0, 0]), closed: !stroke, id: newId, bone, role: "body", z: 5, filled: !stroke, tone: 0.3 };
const geomList = (a: any) => a.representations[sc!].geometry[view!];
const listWith = (pts2: number[][]) => { const a = structuredClone(asset) as any; const l = geomList(a); l.splice(l.indexOf(find(l, after!)) + 1, 0, { ...shape, points: pts2 }); return a; };
const ringOf = (pts2: number[][]) => evalA(listWith(pts2)).shapes[view!].find((x: any) => x.shape.id === newId).rings[0] as { x: number; y: number }[];
let cur = hull.map((p) => [p.x, p.y]);
for (let it = 0; it < 14; it++) {
  const r0 = ringOf(cur), rx = ringOf(cur.map((p) => [p[0]! + 1e-3, p[1]!])), ry = ringOf(cur.map((p) => [p[0]!, p[1]! + 1e-3]));
  let worst = 0;
  cur = cur.map((p, k) => {
    const dx = hull[k]!.x - r0[k]!.x, dy = hull[k]!.y - r0[k]!.y;
    worst = Math.max(worst, Math.hypot(dx, dy));
    const a = (rx[k]!.x - r0[k]!.x) / 1e-3, c = (rx[k]!.y - r0[k]!.y) / 1e-3, b = (ry[k]!.x - r0[k]!.x) / 1e-3, d = (ry[k]!.y - r0[k]!.y) / 1e-3;
    const det = a * d - b * c;
    if (Math.abs(det) < 0.05) return p;
    const damp = it < 3 ? 0.5 : 0.7;
    return [p[0]! + (damp * (d * dx - b * dy)) / det, p[1]! + (damp * (-c * dx + a * dy)) / det];
  });
  if (worst < 3e-4) break;
}
const r5 = (v: number) => Math.round(v * 1e5) / 1e5;
writeFileSync(out!, JSON.stringify([...pre, { op: "shape-add", scale: sc, view, after, shape: { ...shape, points: cur.map((p) => [r5(p[0]!), r5(p[1]!)]) } }], null, 1));
console.log(`[DEBUG] hull ${hull.length} points added as ${newId}`);
