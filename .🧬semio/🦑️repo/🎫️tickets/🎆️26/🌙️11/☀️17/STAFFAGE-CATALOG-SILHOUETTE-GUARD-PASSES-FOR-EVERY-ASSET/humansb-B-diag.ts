/** 🔬 Where the drawing and the volume of one asset disagree: worst mismatch clusters per view at one scale, in the world frame of the view (paper-view x, y in metres), with the drawn shape or volume solid nearest.
 *  bun humansb-B-diag.ts <asset> <scale> <view|-> <patch.json|-> [px py r] (in-memory patch like humans-try) */
import { readFileSync, writeFileSync } from "node:fs";
import { deflateSync } from "node:zlib";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundaryOf, distanceField, fillSmallHoles, extentOf, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, only0, patch, px, py, pr] = process.argv.slice(2) as [string, string, string | undefined, string | undefined, string | undefined, string | undefined, string | undefined];
const only = only0 === "-" ? undefined : only0;
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
if (patch && patch !== "-") {
  const find = (list: any[], id: string) => list.find((x) => x.id === id);
  for (const o of JSON.parse(readFileSync(patch, "utf8"))) {
    const shapes = (): any[] => (asset as any).representations[o.scale].geometry[o.view];
    const solids = (): any[] => (asset as any).volumes[o.scale].body;
    const [list, key] = o.op.startsWith("shape") ? [shapes(), "shape"] : [solids(), "solid"];
    if (o.op.endsWith("-add")) list.splice(list.indexOf(find(list, o.after)) + 1, 0, o[key]);
    else if (o.op.endsWith("-set")) Object.assign(find(list, o.id), o.set);
    else list[list.indexOf(find(list, o.id))] = o[key];
  }
}
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
  if (process.env.PNG !== undefined) {
    const [cx, cy, cr] = [Number(px ?? (box.minX + box.maxX) / 2), Number(py ?? (box.minY + box.maxY) / 2), Number(pr ?? Math.max(box.maxX - box.minX, box.maxY - box.minY) / 2)];
    const z = Math.max(1, Math.floor(800 / (2 * cr / cell)));
    const size = Math.floor((2 * cr) / cell);
    const W = size * z, raw = Buffer.alloc((W * 3 + 1) * W);
    const bdr = (arr: Uint8Array | boolean[], i: number, j: number) => arr[j * w + i];
    for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) {
      const i = Math.floor((cx - cr - box.minX) / cell) + x, j = Math.floor((cy + cr - box.minY) / cell) - y;
      let c = [255, 255, 255];
      if (i >= 0 && j >= 0 && i < w && j < h) {
        const a = drawn.cells[j * w + i], b = vol.cells[j * w + i];
        c = a && b ? [190, 190, 190] : a ? [235, 90, 90] : b ? [80, 120, 235] : [255, 255, 255];
        if (bdr(bd as never, i, j)) c = [120, 0, 0];
        if (bdr(bv as never, i, j)) c = [0, 0, 120];
      }
      for (let dy = 0; dy < z; dy++) for (let dx = 0; dx < z; dx++) { const o = (y * z + dy) * (W * 3 + 1) + 1 + (x * z + dx) * 3; raw[o] = c[0]!; raw[o + 1] = c[1]!; raw[o + 2] = c[2]!; }
    }
    const crcT = new Uint32Array(256).map((_, n) => { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; return c >>> 0; });
    const crc = (b: Buffer) => { let c = 0xffffffff; for (const x of b) c = crcT[(c ^ x) & 255]! ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
    const chunk = (t: string, d: Buffer) => { const l = Buffer.alloc(4); l.writeUInt32BE(d.length); const td = Buffer.concat([Buffer.from(t), d]); const c = Buffer.alloc(4); c.writeUInt32BE(crc(td)); return Buffer.concat([l, td, c]); };
    const ihdr = Buffer.alloc(13); ihdr.writeUInt32BE(W, 0); ihdr.writeUInt32BE(W, 4); ihdr[8] = 8; ihdr[9] = 2;
    writeFileSync(process.env.PNG, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]));
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
