/** 🔶 Emits shape ops for the convex hull of volume solids in a view, as rest-frame polygons of a bone (inverse of the bone's affine probe). bun humansb-C-hullshape.ts <asset> <scale> <view> <bone> <mode:add|replace> <afterOrId> <newId> <solidIds,...> [role] [z] [patches] [simplifyMm] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { applyPatch } from "./humansb-C-patch.ts";
const [name, sc, view, bone, mode, ref, newId, ids, role = "body", z = "5", patch, simp = "1"] = process.argv.slice(2);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
await applyPatch(asset, patch);
const probe = { kind: "poly", points: [[0, 0], [0.1, 0], [0, 0.1]], closed: true, id: "zz-probe", bone, role: "detail", z: 99, filled: true, tone: 0.5, region: "x" };
(asset as any).representations[sc].geometry[view].push(probe);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } }) as any;
const r = state.shapes[view].find((s: any) => s.shape.id === "zz-probe").rings[0];
const o = r[0], ex = { x: (r[1].x - o.x) / 0.1, y: (r[1].y - o.y) / 0.1 }, ey = { x: (r[2].x - o.x) / 0.1, y: (r[2].y - o.y) / 0.1 };
const det = ex.x * ey.y - ex.y * ey.x;
const inv = (p: { x: number; y: number }): [number, number] => { const dx = p.x - o.x, dy = p.y - o.y; return [+((dx * ey.y - dy * ey.x) / det).toFixed(5), +((-dx * ex.y + dy * ex.x) / det).toFixed(5)]; };
const want = new Set(ids.split(","));
const pts: { x: number; y: number }[] = [];
for (const w of worldSolids(asset, state.bones as never, Number(sc))) if (want.has(w.id)) for (let i = 0; i < w.positions.length; i += 3) pts.push(project(view as Viewpoint, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!));
if (pts.length === 0) throw new Error("no solids " + ids);
pts.sort((a, b) => a.x - b.x || a.y - b.y);
const cross = (a: any, b: any, c: any) => (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
const half = (list: any[]) => { const h: any[] = []; for (const p of list) { while (h.length >= 2 && cross(h[h.length - 2], h[h.length - 1], p) <= 0) h.pop(); h.push(p); } h.pop(); return h; };
let hull = [...half(pts), ...half([...pts].reverse())];
const tol = Number(simp) / 1000;
for (let again = true; again && hull.length > 4;) {
  again = false;
  let best = -1, bestD = Infinity;
  for (let i = 0; i < hull.length; i++) {
    const a = hull[(i + hull.length - 1) % hull.length], b = hull[i], c = hull[(i + 1) % hull.length];
    const d = Math.abs(cross(a, b, c)) / Math.hypot(c.x - a.x, c.y - a.y);
    if (d < bestD) { bestD = d; best = i; }
  }
  if (bestD < tol) { hull.splice(best, 1); again = true; }
}
const old = mode === "replace" ? (asset as any).representations[sc].geometry[view].find((s: any) => s.id === ref) : undefined;
const shape: any = { kind: "poly", points: hull.map(inv), closed: true, id: newId, bone, role, z: Number(z), filled: true, tone: 0.5, region: "bike-metal" };
if (old) for (const k of ["role", "z", "filled", "tone", "region", "material", "hatch"]) if (old[k] !== undefined) shape[k] = old[k]; else delete shape[k];
const opx = mode === "add" ? { op: "shape-add", scale: sc, view, after: ref, shape } : { op: "shape", scale: sc, view, id: ref, shape };
console.log(JSON.stringify(opx));
