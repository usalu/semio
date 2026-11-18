/** 🔍 Greedy search on the guard Hausdorff of one view of an asset that draws plates as extruded or revolved solids: the worst boundary cluster picks the nearby solid-drawn shapes, whose profile is scaled along and across (small steps) while the Hausdorff distance of the view falls.
 *  bun vegetation-hsearch.ts <asset> <scale> <view> <out.json> [iterations] */
import { writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, out, itersArg] = process.argv.slice(2) as string[];
const scale = Number(scaleArg);
const iterations = Number(itersArg ?? 12);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const limit = 2 * feature;
const geo = (asset.representations as any)[String(scale)].geometry[view] as any[];
const originals = geo.map((s) => JSON.parse(JSON.stringify(s)));
const scalePath = (d: string, su: number, sv: number): string => {
  let k = 0;
  return d.replace(/[MLCQSTHVZmlcqsthvz]|-?\d+\.?\d*(?:e-?\d+)?/g, (tok) => {
    if (/[A-Za-z]/.test(tok)) { k = 0; return tok; }
    const v = Number(tok) * (k % 2 === 0 ? su : sv);
    k++;
    return String(+v.toFixed(5));
  });
};
const shaped = (o: any, su: number, sv: number): any => {
  const c = JSON.parse(JSON.stringify(o));
  if (c.kind === "poly") c.points = c.points.map(([x, y]: number[]) => [x * su, y * sv]);
  else if (c.kind === "path") c.d = scalePath(c.d, su, sv);
  return c;
};
const state0 = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state0.bones as never, scale).filter((s) => s.texture !== true);
const params = new Map<number, { su: number; sv: number }>();
const stateOf = (): any => evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const apply = (): void => {
  originals.forEach((o, i) => { const p = params.get(i); geo[i] = p === undefined ? o : shaped(o, p.su, p.sv); });
};
const measure = (): { h: number; iou: number; clusters: { x: number; y: number; d: number }[] } => {
  apply();
  const state = stateOf();
  const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
  const drawn = new Mask(m.box, m.cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(m.box, m.cell);
  for (const g of m.reference) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = Math.round((4 * feature / m.cell) ** 2);
  const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h);
  const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
  const [toVol, toDrawn] = [distanceField(bv, w, h), distanceField(bd, w, h)];
  const clusters: { x: number; y: number; d: number }[] = [];
  for (const [from, field] of [[bd, toVol], [bv, toDrawn]] as const) {
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i]) { const d = Math.sqrt(field[j * w + i]!) * m.cell; if (d > limit * 0.9) clusters.push({ x: m.box.minX + (i + 0.5) * m.cell, y: m.box.minY + (j + 0.5) * m.cell, d }); }
  }
  clusters.sort((a, b) => b.d - a.d);
  return { h: m.hausdorffM, iou: m.iou, clusters };
};
let cur = measure();
const start = cur.h;
console.log(`[DEBUG] start H ${cur.h.toFixed(4)} limit ${limit} clusters ${cur.clusters.length}`);
for (let it = 0; it < iterations && cur.h > limit * 0.9; it++) {
  const worst = cur.clusters[0];
  if (worst === undefined) break;
  const state = stateOf();
  const near = (state.shapes[view] as any[]).map((s, i) => ({ s, i })).filter(({ s }) => {
    if (s.shape.solid === undefined || s.shape.filled !== true || (s.shape.kind !== "poly" && s.shape.kind !== "path")) return false;
    const pts = s.rings.flat();
    if (pts.length === 0) return false;
    const [x0, x1, y0, y1] = [Math.min(...pts.map((q: any) => q.x)), Math.max(...pts.map((q: any) => q.x)), Math.min(...pts.map((q: any) => q.y)), Math.max(...pts.map((q: any) => q.y))];
    const r = 0.5 * worst.d + 4 * feature;
    return x0 - r <= worst.x && worst.x <= x1 + r && y0 - r <= worst.y && worst.y <= y1 + r;
  });
  let best = { h: cur.h, i: -1, su: 1, sv: 1 };
  for (const { i } of near) {
    const base = params.get(i) ?? { su: 1, sv: 1 };
    for (const [du, dv] of [[1, 0.94], [1, 0.97], [1, 1.03], [1, 1.06], [0.97, 1], [1.03, 1], [0.94, 0.94], [1.06, 1.06]] as const) {
      params.set(i, { su: base.su * du, sv: base.sv * dv });
      const m = measure();
      if (m.h < best.h - 1e-5) best = { h: m.h, i, su: base.su * du, sv: base.sv * dv };
    }
    params.set(i, base);
  }
  if (best.i < 0) { console.log("[DEBUG] no improving step at", worst.x.toFixed(3), worst.y.toFixed(3)); break; }
  params.set(best.i, { su: best.su, sv: best.sv });
  cur = measure();
  console.log(`[DEBUG] iteration ${it}: H ${cur.h.toFixed(4)} (shape ${originals[best.i].id} su ${best.su.toFixed(3)} sv ${best.sv.toFixed(3)})`);
}
originals.forEach((o, i) => { geo[i] = o; });
const result = [...params.entries()].map(([i, p]) => ({ scale, view, id: originals[i].id, bone: originals[i].bone, su: +p.su.toFixed(4), sv: +p.sv.toFixed(4), iou0: 0, iou1: 1 }));
writeFileSync(out, JSON.stringify(result));
console.log(`[DEBUG] hsearch ${name} 1:${scale} ${view}: H ${start.toFixed(4)} -> ${cur.h.toFixed(4)} with ${result.length} shapes changed`);
