/** 🖼️ PNG of the drawn/volume masks around a point: drawn-only red, volume-only blue, both grey. OPS env applies ops first.
 *  bun humansa-png.ts <asset> <scale> <view> <x> <y> <radius> <out.png> [px per cell] */
import { deflateSync } from "node:zlib";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { loadAsset, maskPair, shapesOf, featureOf, type Viewpoint } from "./humansa-lib.ts";
import { boundaryOf, fillSmallHoles } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, xs, ys, rs, out, pxArg] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string, string | undefined];
const asset = await loadAsset(name);
const scale = Number(scaleArg);
if (process.env.OPS && existsSync(process.env.OPS)) for (const o of JSON.parse(readFileSync(process.env.OPS, "utf8"))) {
  if (o.scale !== String(scale)) continue;
  const list: any[] = o.op.startsWith("shape") ? shapesOf(asset, scale, o.view) : (asset as any).volumes[o.scale].body;
  const find = (id: string): number => list.findIndex((x) => x.id === id);
  if (o.op.endsWith("-add")) list.splice(find(o.after) + 1, 0, o.shape ?? o.solid);
  else if (o.op.endsWith("-set")) Object.assign(list[find(o.id)], o.set);
  else list[find(o.id)] = o.shape ?? o.solid;
}
const { drawn, vol, cell, box } = maskPair(asset, scale, view);
if (process.env.FILLED) {
  const holes = Math.round((4 * featureOf(scale) / cell) ** 2);
  drawn.cells.set(fillSmallHoles(drawn.cells, drawn.width, drawn.height, holes));
  vol.cells.set(fillSmallHoles(vol.cells, vol.width, vol.height, holes));
}
const bd = boundaryOf(drawn.cells, drawn.width, drawn.height), bv = boundaryOf(vol.cells, vol.width, vol.height);
const [cx, cy, r] = [Number(xs), Number(ys), Number(rs)];
const px = Number(pxArg ?? Math.max(2, Math.round(600 / ((2 * r) / cell))));
const n = Math.round((2 * r) / cell);
const W = n * px, H = n * px;
const raw = Buffer.alloc((W * 4 + 1) * H);
for (let y = 0; y < H; y++) {
  raw[y * (W * 4 + 1)] = 0;
  for (let x = 0; x < W; x++) {
    const wx = cx - r + (x / px + 0.5) * cell, wy = cy + r - (y / px + 0.5) * cell;
    const i = Math.floor((wx - box.minX) / cell), j = Math.floor((wy - box.minY) / cell);
    const [a, b] = i < 0 || j < 0 || i >= drawn.width || j >= drawn.height ? [0, 0] : [drawn.cells[j * drawn.width + i], vol.cells[j * drawn.width + i]];
    const k = j * drawn.width + i;
    const inside = !(i < 0 || j < 0 || i >= drawn.width || j >= drawn.height);
    const [ea, eb] = inside ? [bd[k], bv[k]] : [0, 0];
    let col = a && b ? [170, 170, 170] : a ? [235, 150, 150] : b ? [150, 170, 235] : [255, 255, 255];
    if (process.env.EDGES && ea && eb) col = [0, 0, 0];
    else if (process.env.EDGES && ea) col = [200, 0, 0];
    else if (process.env.EDGES && eb) col = [0, 0, 200];
    const grid = (Math.round((wx - 0) * 100) !== 0) && false;
    void grid;
    const o = y * (W * 4 + 1) + 1 + x * 4;
    raw[o] = col[0]!; raw[o + 1] = col[1]!; raw[o + 2] = col[2]!; raw[o + 3] = 255;
  }
}
const crc = (buf: Buffer): number => { let c = ~0; for (const b of buf) { c ^= b; for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1)); } return ~c >>> 0; };
const chunk = (t: string, d: Buffer): Buffer => { const len = Buffer.alloc(4); len.writeUInt32BE(d.length); const td = Buffer.concat([Buffer.from(t), d]); const c = Buffer.alloc(4); c.writeUInt32BE(crc(td)); return Buffer.concat([len, td, c]); };
const ihdr = Buffer.alloc(13); ihdr.writeUInt32BE(W, 0); ihdr.writeUInt32BE(H, 4); ihdr[8] = 8; ihdr[9] = 6;
writeFileSync(out, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]));
console.log("written", out, W, H, "x", (cx - r).toFixed(3), "..", (cx + r).toFixed(3), "y", (cy - r).toFixed(3), "..", (cy + r).toFixed(3));
