/** 🎯️ What covers a point of one view: the solids whose projected triangles hold it (with the area of the triangle) and the drawn shapes whose rings hold it or pass within a cell.
 *  bun who.ts <id|file.json> <scale> <view> <x> <y> */
import { project } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { edgeDistance, inside, scene } from "./lib.ts";
import { fileOf } from "./plants.ts";
const [name, scaleArg, view, xs, ys] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const sc = scene(asset, Number(scaleArg), view!);
const p = { x: Number(xs), y: Number(ys) };
for (const w of sc.solids as any[]) {
  const q: { x: number; y: number }[] = [];
  for (let i = 0; i < w.positions.length; i += 3) q.push(project(view as never, w.positions[i], w.positions[i + 1], w.positions[i + 2]));
  for (let t = 0; t < w.indices.length; t += 3) {
    const tri = [q[w.indices[t]]!, q[w.indices[t + 1]]!, q[w.indices[t + 2]]!];
    if (inside(p, [tri])) { const area = Math.abs((tri[1]!.x - tri[0]!.x) * (tri[2]!.y - tri[0]!.y) - (tri[2]!.x - tri[0]!.x) * (tri[1]!.y - tri[0]!.y)) / 2; console.log(`solid ${w.id}@${w.bone}${w.slender ? " slender" : ""} triangle ${t / 3} area ${(area * 1e6).toFixed(2)} mm2 ${tri.map((v) => `(${v.x.toFixed(5)},${v.y.toFixed(5)})`).join(" ")}`); }
  }
}
for (const s of sc.evaluated as any[]) { const d = edgeDistance(p, s.rings, s.closed); const hit = s.closed && inside(p, s.rings); if (hit || d < 2 * sc.cell) console.log(`shape ${s.shape.id}@${s.shape.bone} ${s.closed ? "closed" : "open"}${hit ? " holds it" : ""}, edge ${(d * 1000).toFixed(3)} mm, ${s.rings.length} rings, held by rings ${s.rings.map((r: any, i: number) => (inside(p, [r]) ? i : -1)).filter((i: number) => i >= 0).join(",")}; nearest edge ${(() => { let best = ""; let bd = Infinity; for (const r of s.rings) for (let i = 0; i < r.length; i++) { const d = edgeDistance(p, [[r[i], r[(i + 1) % r.length]]], false); if (d < bd) { bd = d; best = `(${r[i].x.toFixed(5)},${r[i].y.toFixed(5)}) (${r[(i + 1) % r.length].x.toFixed(5)},${r[(i + 1) % r.length].y.toFixed(5)})`; } } return best; })()}`); }
