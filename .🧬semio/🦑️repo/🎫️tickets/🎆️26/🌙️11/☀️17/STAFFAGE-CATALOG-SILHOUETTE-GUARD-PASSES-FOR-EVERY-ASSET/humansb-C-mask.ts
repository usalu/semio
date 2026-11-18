/** 🗺️ ASCII mask of drawn (D), volume (V), both (#) and neither (.) on a window of a view after the guard's hole fill. bun humansb-C-mask.ts <asset> <scale> <view> <x0> <x1> <y0> <y1> <stepMm> [patch.mjs] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { applyPatch } from "./humansb-C-patch.ts";
const [name, sc, view, x0, x1, y0, y1, step, patch] = process.argv.slice(2) as string[];
const scale = Number(sc);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
await applyPatch(asset, patch);
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
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
const st = Number(step) / 1000;
const bd = boundaryOf(dc, w, h), bv = boundaryOf(vc, w, h);
const toV = distanceField(bv, w, h), toD = distanceField(bd, w, h);
const lim = 2 * feature;
const bad = (b: ArrayLike<number>, f: ArrayLike<number>, k: number) => b[k] && Math.sqrt(f[k]!) * m.cell > lim;
console.log(`iou ${m.iou.toFixed(3)} h ${m.hausdorffM.toFixed(4)} cell ${m.cell}`);
for (let y = Number(y1); y > Number(y0); y -= st * 2) {
  let line = (y.toFixed(3) + " ").padStart(8);
  for (let x = Number(x0); x < Number(x1); x += st) {
    const i = Math.floor((x - m.box.minX) / m.cell), j = Math.floor((y - m.box.minY) / m.cell);
    const [a, b] = [dc[j * w + i], vc[j * w + i]];
    const k = j * w + i;
    line += bad(bd, toV, k) ? "d" : bad(bv, toD, k) ? "v" : a && b ? "#" : a ? "D" : b ? "V" : ".";
  }
  console.log(line);
}
{
  const out: { side: string; x: number; y: number; d: number }[] = [];
  for (const [side, from, field] of [["drawn-only", bd, toV], ["volume-only", bv, toD]] as const) {
    const list: { i: number; j: number; d: number }[] = [];
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * m.cell > lim) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * m.cell });
    list.sort((a, b) => b.d - a.d);
    const seen = new Set<number>();
    for (const c of list) {
      const key = Math.round(c.i / (0.1 / m.cell)) * 100000 + Math.round(c.j / (0.1 / m.cell));
      if (seen.has(key)) continue;
      seen.add(key);
      out.push({ side, x: +(m.box.minX + (c.i + 0.5) * m.cell).toFixed(3), y: +(m.box.minY + (c.j + 0.5) * m.cell).toFixed(3), d: +c.d.toFixed(3) });
    }
  }
  out.sort((a, b) => b.d - a.d);
  for (const c of out.slice(0, 10)) console.log(`  ${c.side} (${c.x}, ${c.y}) ${c.d}`);
}
if (process.env.BINS) {
  const bsz = Number(process.env.BINS);
  const bins = new Map<string, [number, number]>();
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    const k = j * w + i, a = drawn.cells[k], b = vol.cells[k];
    if (!!a === !!b) continue;
    const x = m.box.minX + (i + 0.5) * m.cell, y = m.box.minY + (j + 0.5) * m.cell;
    const key = `${(Math.floor(x / bsz) * bsz).toFixed(2)},${(Math.floor(y / bsz) * bsz).toFixed(2)}`;
    const e = bins.get(key) ?? [0, 0];
    e[a ? 0 : 1] += m.cell * m.cell;
    bins.set(key, e);
  }
  let td = 0, tv = 0, both = 0;
  for (let k = 0; k < w * h; k++) { if (drawn.cells[k] && vol.cells[k]) both += m.cell * m.cell; }
  for (const [, e] of bins) { td += e[0]; tv += e[1]; }
  console.log(`areas m2: both ${both.toFixed(4)} drawn-only ${td.toFixed(4)} volume-only ${tv.toFixed(4)} -> iou ${(both / (both + td + tv)).toFixed(4)}`);
  for (const [key, e] of [...bins].sort((a, b) => b[1][0] + b[1][1] - a[1][0] - a[1][1]).slice(0, 14)) console.log(`  bin ${key} drawn-only ${e[0].toFixed(4)} volume-only ${e[1].toFixed(4)}`);
}
