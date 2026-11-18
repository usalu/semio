/** 🔬 Where the drawing and the volume of one asset disagree: worst mismatch clusters per view at one scale, in the world frame of the view (paper-view x, y in metres), with the drawn shape or volume solid nearest.
 *  bun silhouette-diag.ts <asset> <scale> [view] */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundaryOf, distanceField, fillSmallHoles, extentOf, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

void readFileSync;
const [name, scaleArg, only, px, py, pr] = process.argv.slice(2) as [string, string, string | undefined, string | undefined, string | undefined, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const e = extentOf(solids);
console.log("extent min", e.min.map((x) => +x.toFixed(3)), "max", e.max.map((x) => +x.toFixed(3)), "dims", asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM, "feature", feature);
for (const view of VIEWPOINTS) {
  if (only !== undefined && only !== view) continue;
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
  const cell = m.cell;
  const box = m.box;
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  if (m.section) for (const g of m.visible) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = (4 * feature / cell) ** 2;
  const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, Math.round(holes)), w, h);
  const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, Math.round(holes)), w, h);
  const toVol = distanceField(bv, w, h);
  const toDrawn = distanceField(bd, w, h);
  const limit = 2 * feature;
  const clusters: { side: string; x: number; y: number; d: number }[] = [];
  const seen = new Set<number>();
  for (const [side, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const) {
    const list: { i: number; j: number; d: number }[] = [];
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > limit) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * cell });
    list.sort((a, b) => b.d - a.d);
    seen.clear();
    for (const c of list) {
      const key = Math.round(c.i / (0.15 / cell)) * 100000 + Math.round(c.j / (0.15 / cell));
      if (seen.has(key)) continue;
      seen.add(key);
      clusters.push({ side, x: +(box.minX + (c.i + 0.5) * cell).toFixed(3), y: +(box.minY + (c.j + 0.5) * cell).toFixed(3), d: +c.d.toFixed(3) });
      if (clusters.length > 400) break;
    }
  }
  if (px !== undefined && py !== undefined) {
    const r = Number(pr ?? 0.1);
    const step = Math.max(1, Math.round((2 * r) / cell / 70));
    for (let y = Number(py) + r; y > Number(py) - r; y -= step * cell * 2) {
      let line = "";
      for (let x = Number(px) - r; x < Number(px) + r; x += step * cell) {
        const i = Math.floor((x - box.minX) / cell);
        const j = Math.floor((y - box.minY) / cell);
        const [a, b] = [drawn.cells[j * w + i], vol.cells[j * w + i]];
        line += a && b ? "#" : a ? "D" : b ? "V" : ".";
      }
      console.log(line);
    }
  }
  console.log(`== ${view} 1:${scale} iou ${m.iou.toFixed(3)} hausdorff ${m.hausdorffM.toFixed(4)} limit ${limit} section ${m.section} drawn=${m.counts.drawn} vol=${m.counts.volume}`);
  const out = clusters.sort((a, b) => b.d - a.d).slice(0, 8);
  for (const c of out) {
    const near = m.filled.map((g, k) => ({ k, g })).slice(0, 0);
    void near;
    console.log(`  ${c.side} at (${c.x}, ${c.y}) distance ${c.d}`);
  }
}
void GATE;
