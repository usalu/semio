/** 📏️ Per-shape and per-solid world extents in one view (paper x and y of that view) for shapes/solids whose id contains a filter.
 *  bun shape_ext.ts <slug> <pose> <scale> <view> <filter> */
import { join, resolve } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [slug, pose, sc, view, q] = process.argv.slice(2);
const asset = (await Bun.file(join(catalog, slug!, "🔣️.json")).json()).asset;
const state = evaluate(asset, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: Number(sc) } }) as any;
const f = (n: number): string => n.toFixed(4);
for (const s of state.shapes[view!]) {
  if (!String(s.shape.id).includes(q!)) continue;
  const pts = s.rings.flat();
  if (!pts.length) continue;
  console.log("D", String(s.shape.id).padEnd(28), "x", f(Math.min(...pts.map((p: any) => p.x))), f(Math.max(...pts.map((p: any) => p.x))), "y", f(Math.min(...pts.map((p: any) => p.y))), f(Math.max(...pts.map((p: any) => p.y))));
}
for (const w of worldSolids(asset, state.bones as never, Number(sc))) {
  if (!w.id.includes(q!)) continue;
  const xs: number[] = [], ys: number[] = [];
  for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as Viewpoint, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); xs.push(p.x); ys.push(p.y); }
  console.log("V", w.id.padEnd(28), "x", f(Math.min(...xs)), f(Math.max(...xs)), "y", f(Math.min(...ys)), f(Math.max(...ys)));
}
