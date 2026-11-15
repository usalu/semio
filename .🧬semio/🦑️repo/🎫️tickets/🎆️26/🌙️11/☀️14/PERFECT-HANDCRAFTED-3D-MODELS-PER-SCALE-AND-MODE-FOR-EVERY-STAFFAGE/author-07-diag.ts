import { entryOf, loadSource, merged, featureM, poseOf } from "./volumes-lib.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids, referenceOf, measureView, Mask, distanceField, VIEWPOINTS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const [spec, scaleArg, viewArg, limitArg] = process.argv.slice(2);
const scale = Number(scaleArg ?? 50);
const { entry, asset: raw } = entryOf(spec!);
const asset: any = merged(raw, loadSource(entry)!);
const feature = featureM(scale, asset);
const limit = limitArg ? Number(limitArg) : 2 * feature;
const pose = poseOf(asset);
const state: any = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale);
const rep = asset.representations[String(scale)];
for (const view of VIEWPOINTS) {
  if (viewArg && viewArg !== "all" && viewArg !== view) continue;
  const m = measureView(asset, state, solids as never, view, scale, undefined, feature);
  const { box, cell } = m;
  const volume = new Mask(box, cell);
  for (const g of m.reference) volume.fill(g);
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const W = volume.width, H = volume.height;
  const dv = distanceField(drawn.cells, W, H);
  const dd = distanceField(volume.cells, W, H);
  console.log(`== ${view} iou=${m.iou.toFixed(3)} H=${(m.hausdorffM*1000).toFixed(1)}mm limit=${(limit*1000).toFixed(1)}mm cell=${(cell*1000).toFixed(2)}mm box=[${box.minX.toFixed(2)},${box.maxX.toFixed(2)}]x[${box.minY.toFixed(2)},${box.maxY.toFixed(2)}]`);
  const lim2 = (limit / cell) ** 2;
  const blame = (name: string, mask: Mask, field: Float64Array) => {
    let n = 0, x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9;
    for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) if (mask.cells[j * W + i] && field[j * W + i]! > lim2) { n++; const x = box.minX + (i + 0.5) * cell, y = box.minY + (j + 0.5) * cell; x0 = Math.min(x0, x); x1 = Math.max(x1, x); y0 = Math.min(y0, y); y1 = Math.max(y1, y); }
    return n ? { name, n, bbox: [x0, x1, y0, y1].map((v) => +v.toFixed(3)) } : null;
  };
  const far3d: any[] = [];
  for (const s of solids) {
    const g = referenceOf(view, [s as never], undefined);
    const mk = new Mask(box, cell);
    for (const r of g.cut) mk.fill(r);
    const b = blame("3D " + s.id, mk, dv);
    if (b) far3d.push(b);
  }
  const farDrawn: any[] = [];
  for (const s of state.shapes[view] as any[]) {
    if (!["body", "detail", "glazing"].includes(s.shape.role)) continue;
    const mk = new Mask(box, cell);
    const rings = s.rings;
    if (s.closed && s.shape.filled !== false) mk.fill(rings);
    else continue;
    const b = blame("2D " + s.shape.id + " (" + s.shape.bone + ")", mk, dd);
    if (b) farDrawn.push(b);
  }
  const totalOut = blame("DRAWN-ALL", drawn, dd);
  const totalVol = blame("VOLUME-ALL", volume, dv);
  for (const b of [totalOut, totalVol].filter(Boolean)) console.log("  ", JSON.stringify(b));
  for (const b of far3d.sort((a, b) => b.n - a.n).slice(0, 12)) console.log("   ", JSON.stringify(b));
  for (const b of farDrawn.sort((a, b) => b.n - a.n).slice(0, 12)) console.log("   ", JSON.stringify(b));
}
