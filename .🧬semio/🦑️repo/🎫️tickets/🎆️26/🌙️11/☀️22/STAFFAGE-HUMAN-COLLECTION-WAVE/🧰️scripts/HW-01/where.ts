/** 🗺️ Where drawing and volume disagree in one view of the default pose (the catalog-silhouette guard): the worst boundary points beyond
 *  two features (drawn-only, volume-only) and the 0.1 m bins with the most one-sided area.  bun where.ts <asset.json> <scale> <view> */
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [path, sc, view] = process.argv.slice(2) as [string, string, string];
const asset = (await Bun.file(path).json()).asset;
const scale = Number(sc);
const feature = featureOf(scale as never);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
const solids = worldSolids(asset, state.bones as never, scale as never).filter((w: { texture?: boolean }) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale as never, undefined, feature, Infinity, false);
const drawn = new Mask(m.box, m.cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
const vol = new Mask(m.box, m.cell);
for (const g of m.reference) vol.fill(g, true);
if (m.section) for (const g of m.visible) vol.fill(g, true);
vol.seal();
const [w, h] = [vol.width, vol.height];
const holes = Math.round((4 * feature / m.cell) ** 2);
const dc = fillSmallHoles(drawn.cells, w, h, holes), vc = fillSmallHoles(vol.cells, w, h, holes);
const bd = boundaryOf(dc, w, h), bv = boundaryOf(vc, w, h);
const toV = distanceField(bv, w, h), toD = distanceField(bd, w, h);
const lim = 2 * feature;
console.log(`iou ${m.iou.toFixed(3)} haus ${(m.hausdorffM * 1000).toFixed(0)} mm limit ${(lim * 1000).toFixed(0)} cell ${m.cell}`);
const out: { side: string; x: number; y: number; d: number }[] = [];
for (const [side, from, field] of [["drawn-only", bd, toV], ["volume-only", bv, toD]] as const) {
  const seen = new Set<string>();
  const list: { i: number; j: number; d: number }[] = [];
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * m.cell > lim) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * m.cell });
  list.sort((a, b) => b.d - a.d);
  for (const c of list) {
    const x = m.box.minX + (c.i + 0.5) * m.cell, y = m.box.minY + (c.j + 0.5) * m.cell;
    const key = `${Math.round(x / 0.05)},${Math.round(y / 0.05)}`;
    if (seen.has(key)) continue;
    seen.add(key);
    out.push({ side, x, y, d: c.d });
  }
}
out.sort((a, b) => b.d - a.d);
for (const c of out.slice(0, 12)) console.log(`  ${c.side.padEnd(11)} (${c.x.toFixed(3)}, ${c.y.toFixed(3)}) ${(c.d * 1000).toFixed(0)} mm`);
const bins = new Map<string, [number, number]>();
for (let k = 0; k < w * h; k++) {
  const a = dc[k], b = vc[k];
  if (!!a === !!b) continue;
  const i = k % w, j = Math.floor(k / w);
  const x = m.box.minX + (i + 0.5) * m.cell, y = m.box.minY + (j + 0.5) * m.cell;
  const key = `${(Math.floor(x / 0.1) * 0.1).toFixed(1)},${(Math.floor(y / 0.1) * 0.1).toFixed(1)}`;
  const e = bins.get(key) ?? [0, 0];
  e[a ? 0 : 1] += m.cell * m.cell;
  bins.set(key, e);
}
for (const [key, e] of [...bins].sort((p, q) => q[1][0] + q[1][1] - p[1][0] - p[1][1]).slice(0, 8)) console.log(`  bin ${key} drawn-only ${(e[0] * 1e4).toFixed(1)} cm2 volume-only ${(e[1] * 1e4).toFixed(1)} cm2`);
