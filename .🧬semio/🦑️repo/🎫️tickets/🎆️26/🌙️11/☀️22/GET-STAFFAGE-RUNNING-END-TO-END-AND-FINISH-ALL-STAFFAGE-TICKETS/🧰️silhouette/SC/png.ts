/** 🖼️ PNG of the drawn and the volume mask of one view of a component as the guard rasterises them: drawn-only red, volume-only blue, both grey, the two boundaries dark.
 *  bun png.ts <id|file.json> <scale> <view> <out.png> - [x y radius] */
import { deflateSync } from "node:zlib";
import { writeFileSync } from "node:fs";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./lib.ts";

const [name, scaleArg, view, out, opsArg, xs, ys, rs] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
void opsArg;
const scale = Number(scaleArg), feature = featureOf(scale);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
const { cell, box } = m;
const drawn = new Mask(box, cell), vol = new Mask(box, cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
for (const g of m.reference) vol.fill(g, true);
if (m.section) for (const g of m.visible) vol.fill(g, true);
vol.seal();
const [w, h] = [vol.width, vol.height];
const holes = Math.round((4 * feature / cell) ** 2);
const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h), bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
const win = xs === undefined ? { x0: box.minX, x1: box.maxX, y0: box.minY, y1: box.maxY } : { x0: Number(xs) - Number(rs), x1: Number(xs) + Number(rs), y0: Number(ys) - Number(rs), y1: Number(ys) + Number(rs) };
const px = Math.max(1, Math.min(8, Math.round(900 / (Math.max(win.x1 - win.x0, win.y1 - win.y0) / cell))));
const W = Math.round((win.x1 - win.x0) / cell) * px, H = Math.round((win.y1 - win.y0) / cell) * px;
const raw = Buffer.alloc((W * 3 + 1) * H);
for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
  const wx = win.x0 + (x / px + 0.5) * cell, wy = win.y1 - (y / px + 0.5) * cell;
  const i = Math.floor((wx - box.minX) / cell), j = Math.floor((wy - box.minY) / cell);
  const inside = i >= 0 && j >= 0 && i < w && j < h, k = j * w + i;
  const [a, b] = inside ? [drawn.cells[k], vol.cells[k]] : [0, 0];
  let col = a && b ? [205, 205, 205] : a ? [240, 130, 130] : b ? [130, 150, 240] : [255, 255, 255];
  if (inside && bd[k] && bv[k]) col = [60, 60, 60]; else if (inside && bd[k]) col = [190, 0, 0]; else if (inside && bv[k]) col = [0, 0, 190];
  const o = y * (W * 3 + 1) + 1 + x * 3;
  raw[o] = col[0]!; raw[o + 1] = col[1]!; raw[o + 2] = col[2]!;
}
const crc = (buf: Buffer): number => { let c = ~0; for (const b of buf) { c ^= b; for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1)); } return ~c >>> 0; };
const chunk = (t: string, d: Buffer): Buffer => { const len = Buffer.alloc(4); len.writeUInt32BE(d.length); const td = Buffer.concat([Buffer.from(t), d]); const c = Buffer.alloc(4); c.writeUInt32BE(crc(td)); return Buffer.concat([len, td, c]); };
const ihdr = Buffer.alloc(13); ihdr.writeUInt32BE(W, 0); ihdr.writeUInt32BE(H, 4); ihdr[8] = 8; ihdr[9] = 2;
writeFileSync(out!, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]));
console.log(`${out} ${W}x${H} x ${win.x0.toFixed(3)}..${win.x1.toFixed(3)} y ${win.y0.toFixed(3)}..${win.y1.toFixed(3)} iou ${m.iou.toFixed(3)} H ${(m.hausdorffM * 1000).toFixed(1)} mm`);
