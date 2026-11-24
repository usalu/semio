/** ✒️ The exact outline of body solids of a scale in one view of the default pose: the union of their projected triangles (the staffage polygon union), simplified to `--tol` millimetres (Douglas-Peucker, default 0.25) and rounded to 0.01 mm, as JSON rings in view metres (elevation x, y; front z, y; plan x, z). The numbers a repair script states for a drawn shape that must be the projection of its solid.
 *  bun outline.ts <slug|file.json> <scale> <view> <solid-id,solid-id…> [--tol 0.25] [--copy n: only the n-th copy of a repeated solid] */
import { parseArgs } from "node:util";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { unionPolygons } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { pieces, project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./lib.ts";

type P = { x: number; y: number };
const { values, positionals } = parseArgs({ allowPositionals: true, options: { tol: { type: "string" }, copy: { type: "string" } } });
const [name, scaleArg, view, ids] = positionals as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), tol = Number(values.tol ?? 0.25) / 1000;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const wanted = new Set(ids!.split(","));
const triangles: P[][] = [];
for (const solid of worldSolids(asset, state.bones as never, scale).filter((w) => wanted.has(w.id))) {
  const parts = values.copy === undefined ? [solid] : [pieces(solid)[Number(values.copy)]!];
  for (const w of parts) for (let t = 0; t < w.indices.length; t += 3) {
    const tri = [0, 1, 2].map((k) => { const i = w.indices[t + k]! * 3; return project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); });
    const a = (tri[1]!.x - tri[0]!.x) * (tri[2]!.y - tri[0]!.y) - (tri[2]!.x - tri[0]!.x) * (tri[1]!.y - tri[0]!.y);
    if (Math.abs(a) > 1e-12) triangles.push(a > 0 ? tri : [tri[0]!, tri[2]!, tri[1]!]);
  }
}
const simplify = (ring: P[]): P[] => {
  const keep = new Uint8Array(ring.length);
  let far = 0, best = -1;
  for (let i = 1; i < ring.length; i++) { const d = Math.hypot(ring[i]!.x - ring[0]!.x, ring[i]!.y - ring[0]!.y); if (d > best) { best = d; far = i; } }
  keep[0] = keep[far] = 1;
  const walk = (a: number, b: number): void => {
    const n = ring.length, [p, q] = [ring[a]!, ring[b % n]!];
    const l = Math.hypot(q.x - p.x, q.y - p.y) || 1;
    let [worst, at] = [0, -1];
    for (let i = a + 1; i < b; i++) { const r = ring[i % n]!; const d = Math.abs((q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x)) / l; if (d > worst) { worst = d; at = i; } }
    if (worst > tol) { keep[at % n] = 1; walk(a, at); walk(at, b); }
  };
  walk(0, far);
  walk(far, ring.length);
  return ring.filter((_, i) => keep[i]);
};
const rings = unionPolygons(triangles).map((r) => simplify(r as P[])).filter((r) => r.length >= 3);
console.log(`[DEBUG] ${asset.id} 1:${scale} ${view} ${ids}: ${triangles.length} triangles, ${rings.length} rings of ${rings.map((r) => r.length).join(", ")} points`);
console.log(JSON.stringify(rings.map((r) => r.map((p) => [Number(p.x.toFixed(5)), Number(p.y.toFixed(5))]))));
